use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    pub sub: i64,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

pub fn create_token(
    user_id: i64,
    email: &str,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = now + Duration::days(7);

    let claims = Claims {
        sub: user_id,
        email: email.to_owned(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_round_trip_preserves_identity_claims() {
        let token = create_token(42, "researcher@example.com", "test-secret-only")
            .expect("token should encode");
        let claims = verify_token(&token, "test-secret-only").expect("token should verify");

        assert_eq!(claims.sub, 42);
        assert_eq!(claims.email, "researcher@example.com");
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn verification_fails_with_wrong_secret() {
        let token = create_token(42, "researcher@example.com", "correct-secret")
            .expect("token should encode");

        assert!(verify_token(&token, "wrong-secret").is_err());
    }
}
