/*!

`snes_compress` provides functions for decompressing and recompressing snes data.

## Supported Formats

- [ ] LZ1
- [ ] LZ19
- [ ] LZ2
- [ ] LZ3
- [x] LZ5
- [ ] RLE1
- [ ] RLE2

## Example

```
use snes_compress::{CompressionType, errors::{DecompressionError, CompressionError}};

fn decompress_data(compressed_data: &[u8]) -> Result<Vec<u8>, DecompressionError> {
    snes_compress::decompress(compressed_data, CompressionType::LZ5)
}

fn compress_data(decompressed_data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    snes_compress::compress(decompressed_data, CompressionType::LZ5)
}
```

*/
use core::fmt;

mod compression;
mod decompression;

pub mod errors;
pub use compression::compress;
pub use decompression::decompress;


#[derive(Debug, Clone, Copy)]
pub enum CompressionType {
    LZ5,
}

impl fmt::Display for CompressionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
