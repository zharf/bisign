#[derive(Debug)]
pub enum BISignError {
    HashMismatch { signed: String, real: String },
    UknownBISignVersion(u32),
    IOError(std::io::Error),
}

impl From<std::io::Error> for BISignError {
    fn from(e: std::io::Error) -> BISignError {
        BISignError::IOError(e)
    }
}