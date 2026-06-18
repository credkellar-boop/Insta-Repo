#[derive(Debug)]
pub enum ScanError {
    IoError(std::io::Error),
    ApiError(String),
    HeuristicParseError,
}

impl From<std::io::Error> for ScanError {
    fn from(err: std::io::Error) -> Self { ScanError::IoError(err) }
}
