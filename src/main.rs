use std::{io, net::{IpAddr, SocketAddr}};

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn health() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "healthy",
            "service": env!("CARGO_PKG_NAME"),
            "version": env!("CARGO_PKG_VERSION"),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })),
    )
}

fn app() -> Router {
    Router::new()
        .route("/api/v1/health", get(health))
        .layer(TraceLayer::new_for_http())
}

fn parse_bind_address(host: &str, port: &str) -> Result<SocketAddr, io::Error> {
    let ip = host.parse::<IpAddr>().map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("HOST must be a valid IP address: {error}"),
        )
    })?;
    let port = port.parse::<u16>().map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("PORT must be an integer between 1 and 65535: {error}"),
        )
    })?;

    if port == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "PORT must be an integer between 1 and 65535",
        ));
    }

    Ok(SocketAddr::new(ip, port))
}

fn bind_address_from_env() -> Result<SocketAddr, io::Error> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_owned());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    parse_bind_address(&host, &port)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(error) = tokio::signal::ctrl_c().await {
            tracing::error!(%error, "failed to install Ctrl+C handler");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(error) => {
                tracing::error!(%error, "failed to install SIGTERM handler");
                std::future::pending::<()>().await;
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("shutdown signal received");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let address = bind_address_from_env()?;
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(address = %listener.local_addr()?, "server listening");

    axum::serve(listener, app())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_handler_returns_ok() {
        let response = health().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn parses_valid_bind_addresses() {
        assert_eq!(
            parse_bind_address("127.0.0.1", "9000").expect("address should parse"),
            "127.0.0.1:9000".parse::<SocketAddr>().unwrap()
        );
    }

    #[test]
    fn rejects_invalid_bind_configuration() {
        assert!(parse_bind_address("localhost", "8080").is_err());
        assert!(parse_bind_address("127.0.0.1", "0").is_err());
        assert!(parse_bind_address("127.0.0.1", "not-a-port").is_err());
    }
}
