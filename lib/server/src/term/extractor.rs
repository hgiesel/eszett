use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use dao::term::term::Term;

pub struct ExtractTerm(pub Term);

impl <S> FromRequestParts<S> for ExtractTerm
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) ->  Result<Self, Self::Rejection> {
        let term = parts.uri.path().trim_start_matches('/');
        Ok(ExtractTerm(term.into()))
    }
}
