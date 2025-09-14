use axum::{
    extract::{FromRequestParts},
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use crate::auth::{verify_jwt, Claims};

pub struct AuthUser(pub Claims);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract Bearer token
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| (StatusCode::UNAUTHORIZED, "Missing Authorization header".into()))?;

        // Verify JWT
        let claims = verify_jwt(bearer.token())
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token".into()))?;

        Ok(AuthUser(claims))
    }
}
