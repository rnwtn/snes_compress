# snes_compress

A compression library for old games.

## Supported Formats

- [ ] LZ1
- [ ] LZ19
- [ ] LZ2
- [ ] LZ3
- [x] LZ5
- [ ] RLE1
- [ ] RLE2


## Usage
### Rust Library:
Add dependency
`https://crates.io/crates/snes_compress`
```
use snes_compress::{CompressionType, errors::{DecompressionError, CompressionError}};

fn decompress_data(compressed_data: &[u8]) -> Result<Vec<u8>, DecompressionError> {
    snes_compress::decompress(compressed_data, CompressionType::LZ5)
}

fn compress_data(decompressed_data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    snes_compress::compress(decompressed_data, CompressionType::LZ5)
}
```

### Shared Library:
TODO: Add wrapper project to compile this to so (linux), dll (windows), and dylib (mac)

### Binary:
`cargo install snes_compress`

```
Usage:
  snes_compress [option] "<input_file>" "<output_file>"

Options:
  -d: Decompress
  -c: Compress
```

