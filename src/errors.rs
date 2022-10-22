use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct DecompressionErrorInfo {
    source: Vec<u8>,
    result: Vec<u8>,
}

impl DecompressionErrorInfo {
    pub fn new(source: &[u8], result: &[u8]) -> Self {
        DecompressionErrorInfo {
            source: source.to_owned(),
            result: result.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DecompressionErrorKind {
    UnsupportedFormat,
    InvalidCommand,
    IndexOutOfBounds,
}

#[derive(Error, Debug, PartialEq)]
#[error("Decompression Error")]
pub struct DecompressionError {
    error_info: DecompressionErrorInfo,
    kind: DecompressionErrorKind,
}

impl DecompressionError {
    pub fn new(kind: DecompressionErrorKind, error_info: DecompressionErrorInfo) -> Self {
        DecompressionError { error_info, kind }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum CompressionError {
    #[error("Compression type \"{compression_type}\" is not supported.")]
    UnsupportedFormat { compression_type: String },
    #[error("Compression failed. All data SHOULD be compressible for any format. This is a problem with the library.")]
    CompressionFailed,
}
