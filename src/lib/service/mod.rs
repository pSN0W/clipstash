use crate::{DataError,ClipError};

#[derive(Debug,thiserror::Error)]
pub enum ServiceError {
    #[error("Clip Error : {0}")]
    Clip(#[from] ClipError),

    #[error("Database Error : {0}")]
    Data(DataError),

    #[error("Service not found")]
    NotFound,

    #[error("Permission not met : {0}")]
    Permission(String)
}

impl From<DataError> for ServiceError {
    fn from(data_error: DataError) -> Self {
        match data_error{
            DataError::DataBase(d) => {
                match d {
                    sqlx::Error::RowNotFound => Self::NotFound,
                    other => Self::Data(DataError::DataBase(other))
                }
            }
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        match err{
            sqlx::Error::RowNotFound => Self::NotFound,
            other => Self::Data(DataError::DataBase(other))
        }
    }
}