use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use crate::utils::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum CoinFetchError {
    #[error("{0}")]
    ValidationError(String),
    #[error("{0}")]
    NotFoundError(String),
    #[error("Failed to fetch result from CoinGecko")]
    GeckoError(#[from] reqwest::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error)
}

impl std::fmt::Debug for CoinFetchError {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self,f)
    }
}

impl ResponseError for CoinFetchError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::GeckoError(_) |
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFoundError(_) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(
            ErrorResponse {
                code: self.status_code().as_u16(),
                message: self.to_string(),
            }
        )
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

pub struct StoreTokenError(pub sqlx::Error);

impl std::error::Error for StoreTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl std::fmt::Debug for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl std::fmt::Display for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database failure was encountered while trying to store Market Data."
        )
    }
}