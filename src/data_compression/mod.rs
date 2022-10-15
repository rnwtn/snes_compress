mod compression;
mod decompression;

use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum CompressionType {
    LZ5,
}

impl fmt::Display for CompressionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub use compression::compress;
pub use decompression::decompress;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_and_decompress_simple() {
        let source = vec![0xA, 0xA, 0xA, 0xA, 1, 2, 3, 4, 3, 2, 1, 0xB];
        let compressed = compress(&source, CompressionType::LZ5).unwrap();
        let decompressed = decompress(&compressed, CompressionType::LZ5).unwrap();
        assert_eq!(decompressed, source);
        assert!(compressed.len() < decompressed.len());
    }
}
