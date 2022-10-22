mod command_callbacks;
mod stragies;

use self::{command_callbacks::CommandCallback, stragies::DecompressionStrategy};

use super::CompressionType;
use crate::errors::{DecompressionError, DecompressionErrorInfo, DecompressionErrorKind};

type DecompResult<T> = core::result::Result<T, DecompressionErrorKind>;

struct CommandOutcome {
    is_terminated: bool,
    num_bytes_consumed: usize,
}

impl CommandOutcome {
    fn new(is_terminated: bool, num_bytes_consumed: usize) -> Self {
        CommandOutcome {
            is_terminated,
            num_bytes_consumed,
        }
    }
}

pub fn decompress(
    source: &[u8],
    compression_type: CompressionType,
) -> Result<Vec<u8>, DecompressionError> {
    let mut buffer: Vec<u8> = Vec::new();
    let strategy = get_decompression_strategy(compression_type)
        .map_err(|kind| build_error(source, &buffer, kind))?;
    let mut i = 0;
    while i < source.len() {
        let command_outcome = process_next(&source[i..], &mut buffer, &strategy)
            .map_err(|kind| build_error(source, &buffer, kind))?;
        if command_outcome.is_terminated {
            break;
        } else {
            i += command_outcome.num_bytes_consumed;
        }
    }
    Ok(buffer)
}

fn process_next(
    source: &[u8],
    buffer: &mut Vec<u8>,
    strategy: &DecompressionStrategy,
) -> DecompResult<CommandOutcome> {
    let first_byte = source[0];
    if first_byte == 0xFF {
        Ok(CommandOutcome::new(true, 1))
    } else {
        let is_extended_cmd = is_extended_cmd(first_byte);
        let cmd_bits = get_command_bits(first_byte, is_extended_cmd);
        let cmd_size = get_command_size(source, is_extended_cmd)?;
        let source_offset = get_command_source_offset(source, is_extended_cmd)?;
        let cmd_callback = get_command_callback(is_extended_cmd, cmd_bits, &strategy)?;
        let num_skip = cmd_callback(source_offset, buffer, cmd_size)?;

        if is_extended_cmd {
            Ok(CommandOutcome::new(false, num_skip + 2))
        } else {
            Ok(CommandOutcome::new(false, num_skip + 1))
        }
    }
}

fn get_command_callback(
    is_extended: bool,
    cmd_bits: u8,
    strategy: &DecompressionStrategy,
) -> DecompResult<CommandCallback> {
    let callback = strategy
        .get_command_callback(cmd_bits, is_extended)
        .ok_or(DecompressionErrorKind::InvalidCommand)?;
    Ok(callback)
}

fn is_extended_cmd(byte: u8) -> bool {
    get_command_bits(byte, false) == 0b111
}

fn get_command_bits(byte: u8, extended: bool) -> u8 {
    if extended {
        (byte & 0b00011100) >> 2
    } else {
        (byte & 0b11100000) >> 5
    }
}

fn get_command_size(source: &[u8], extended: bool) -> DecompResult<usize> {
    let required_size = 1_usize + extended as usize;
    if source.len() < required_size {
        Err(DecompressionErrorKind::IndexOutOfBounds)
    } else {
        let cmd_size = if extended {
            let first_byte: usize = (source[0] & 0b00000011).into();
            let second_byte: usize = (source[1]).into();
            (first_byte << 8) | second_byte
        } else {
            (source[0] & 0b00011111).into()
        };
        Ok(cmd_size + 1)
    }
}

fn get_command_source_offset(source: &[u8], extended: bool) -> DecompResult<&[u8]> {
    let offset = if extended { 2 } else { 1 };
    if offset > source.len() {
        Err(DecompressionErrorKind::IndexOutOfBounds)
    } else {
        let source_offset = &source[offset..];
        Ok(source_offset)
    }
}

fn get_decompression_strategy(
    compression_type: CompressionType,
) -> DecompResult<DecompressionStrategy> {
    stragies::get_decompression_strategy(compression_type)
        .ok_or(DecompressionErrorKind::UnsupportedFormat)
}

fn build_error(
    source: &[u8],
    buffer: &Vec<u8>,
    kind: DecompressionErrorKind,
) -> DecompressionError {
    let error_info = DecompressionErrorInfo::new(source, &buffer);
    DecompressionError::new(kind, error_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_terminate() {
        let result = decompress(&[0xFF], CompressionType::LZ5).unwrap();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn direct_copy_works() {
        let first_byte = 0b00000010;
        let source = vec![first_byte, 0xA1, 0xA2, 0xA3, 0xFF];
        match decompress(&source, CompressionType::LZ5) {
            Ok(decompressed) => assert_eq!(decompressed, vec![0xA1, 0xA2, 0xA3]),
            Err(_) => panic!("Direct copy failed"),
        };
    }

    #[test]
    fn byte_fill_works() {
        let first_byte = 0b00100010;
        let source = vec![first_byte, 0xA1, 0xFF];
        match decompress(&source, CompressionType::LZ5) {
            Ok(decompressed) => assert_eq!(decompressed, vec![0xA1, 0xA1, 0xA1]),
            Err(_) => panic!("Byte fill failed"),
        };
    }

    #[test]
    fn word_fill_works() {
        let first_byte = 0b01000011;
        let source = vec![first_byte, 0xAA, 0xBB];
        match decompress(&source, CompressionType::LZ5) {
            Ok(decompressed) => assert_eq!(decompressed, vec![0xAA, 0xBB, 0xAA, 0xBB]),
            Err(_) => panic!("Word fill failed"),
        };
    }

    #[test]
    fn sigma_fill_works() {
        let first_byte = 0b01100011;
        let source = vec![first_byte, 0x01];
        match decompress(&source, CompressionType::LZ5) {
            Ok(decompressed) => assert_eq!(decompressed, vec![0x01, 0x02, 0x03, 0x04]),
            Err(_) => panic!("Word fill failed"),
        };
    }
}
