use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use base::language::Language;

pub struct ExtractLanguage(pub Language);

impl <S> FromRequestParts<S> for ExtractLanguage
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) ->  Result<Self, Self::Rejection> {
        match parts.extensions.get::<Language>() {
            Some(language) => Ok(Self(*language)),
            None => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
