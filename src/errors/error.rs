use derive_more::Display;

#[derive(Debug, Display)]
pub(crate) enum AppError {
    #[display(fmt = "Internal Database Error: {}", source)]
    DbError { source: diesel::result::Error },
}


// convert diesel errors into AppError
impl From<diesel::result::Error> for AppError {
    fn from(f: diesel::result::Error) -> Self {
        AppError::DbError{ source: f }
    }
}