use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Input file does not exist: {0}")]
    MissingFile(String),
    #[error("Input must be PDF: {0}")]
    InvalidExtension(String),
}

pub struct ValidationService;

impl ValidationService {
    pub fn validate_pdf(path: &str) -> Result<(), ValidationError> {
        let p = Path::new(path);
        if !p.exists() {
            return Err(ValidationError::MissingFile(path.to_owned()));
        }
        if p.extension().map(|ext| ext.to_string_lossy().to_ascii_lowercase()) != Some("pdf".to_string()) {
            return Err(ValidationError::InvalidExtension(path.to_owned()));
        }
        Ok(())
    }
}
