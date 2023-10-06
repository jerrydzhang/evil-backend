#[cfg(test)]
mod test {
    use crate::errors::error::AppError;

    // Test convertion of diesel errors into AppError
    #[test]
    fn error_test() {
        assert_eq!(AppError::DbError{source: diesel::result::Error::NotFound}.to_string(), "Internal Database Error: Record not found");
        assert_eq!(AppError::DbError{source: diesel::result::Error::QueryBuilderError("test".into())}.to_string(), "Internal Database Error: test");
    }
}
