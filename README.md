# snes-compress

A compression library I made. Only supports LZ5 right now, but I tried to code it so implementing the others should be trivial. Planning on compiling this to a DLL so it can be used by other languages besides Rust. Everything's still very WIP; needs more testing.

CLI:

1. Install rust
2. Copy bytes you want to (de)compress to a file.
3. Run like

```
Usage:
  cargo run -- [option] "<input_file>" "<output_file>"

Options:
  -d: Decompress
  -c: Compress
```

