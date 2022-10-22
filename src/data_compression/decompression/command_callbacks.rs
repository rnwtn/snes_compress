use super::DecompResult;
use crate::errors::DecompressionErrorKind;

pub type CommandCallback = fn(&[u8], &mut Vec<u8>, usize) -> DecompResult<usize>;

pub fn direct_copy(
    source: &[u8],
    buffer: &mut Vec<u8>,
    cmd_size: usize,
) -> DecompResult<usize> {
    for &byte in source.iter().take(cmd_size) {
        buffer.push(byte);
    }
    Ok(cmd_size)
}

pub fn byte_fill(
    source: &[u8],
    buffer: &mut Vec<u8>,
    cmd_size: usize,
) -> DecompResult<usize> {
    let byte = try_get_byte(source, 0)?;
    for _ in 0..cmd_size {
        buffer.push(byte);
    }
    Ok(1)
}

pub fn word_fill(
    source: &[u8],
    buffer: &mut Vec<u8>,
    cmd_size: usize,
) -> DecompResult<usize> {
    let byte1: u8 = try_get_byte(source, 0)?;
    let byte2: u8 = try_get_byte(source, 1)?;
    for i in 0..cmd_size {
        let to_push = if i % 2 == 0 { byte1 } else { byte2 };
        buffer.push(to_push);
    }
    Ok(2)
}

pub fn increasing_fill(
    source: &[u8],
    buffer: &mut Vec<u8>,
    cmd_size: usize,
) -> DecompResult<usize> {
    let mut byte: u8 = try_get_byte(source, 0)?;
    for _ in 0..cmd_size {
        buffer.push(byte);
        byte += 1
    }
    Ok(1)
}

pub fn repeat_be(
    source: &[u8],
    buffer: &mut Vec<u8>,
    cmd_size: usize,
) -> DecompResult<usize> {
    let byte1: usize = try_get_byte(source, 0)?;
    let byte2: usize = try_get_byte(source, 1)?;
    let mut offset = (byte1 << 8) | byte2;
    for _ in 0..cmd_size {
        buffer.push(buffer[offset]);
        offset += 1;
    }
    Ok(2)
}

pub fn repeat_le(
    source: &[u8],
    buffer: &mut Vec<u8>,
    cmd_size: usize,
) -> DecompResult<usize> {
    let byte1: usize = try_get_byte(source, 1)?;
    let byte2: usize = try_get_byte(source, 0)?;
    let mut offset = (byte1 << 8) | byte2;
    for _ in 0..cmd_size {
        buffer.push(buffer[offset]);
        offset += 1;
    }
    Ok(2)
}

pub fn xor_repeat_be(
    source: &[u8],
    buffer: &mut Vec<u8>,
    cmd_size: usize,
) -> DecompResult<usize> {
    let byte1: usize = try_get_byte(source, 0)?;
    let byte2: usize = try_get_byte(source, 1)?;
    let mut offset = (byte1 << 8) | byte2;
    for _ in 0..cmd_size {
        buffer.push(buffer[offset] ^ 0xFF);
        offset += 1;
    }
    Ok(2)
}

pub fn xor_repeat_le(
    source: &[u8],
    buffer: &mut Vec<u8>,
    cmd_size: usize,
) -> DecompResult<usize> {
    let byte1: usize = try_get_byte(source, 1)?;
    let byte2: usize = try_get_byte(source, 0)?;
    let mut offset = (byte1 << 8) | byte2;
    for _ in 0..cmd_size {
        buffer.push(buffer[offset] ^ 0xFF);
        offset += 1;
    }
    Ok(2)
}

pub fn negative_repeat(
    source: &[u8],
    buffer: &mut Vec<u8>,
    cmd_size: usize,
) -> DecompResult<usize> {
    let byte: usize = try_get_byte(source, 0)?;
    let mut offset = buffer.len() - byte;
    for _ in 0..cmd_size {
        buffer.push(buffer[offset]);
        offset += 1;
    }
    Ok(1)
}

pub fn negative_xor_repeat_le(
    source: &[u8],
    buffer: &mut Vec<u8>,
    cmd_size: usize,
) -> DecompResult<usize> {
    let byte: usize = try_get_byte(source, 0)?;
    let mut offset = buffer.len() - byte;
    for _ in 0..cmd_size {
        buffer.push(buffer[offset] ^ 0xFF);
        offset += 1;
    }
    Ok(1)
}

fn try_get_byte<T: std::convert::From<u8>>(source: &[u8], idx: usize) -> DecompResult<T> {
    if idx < source.len() {
        Ok(source[idx].into())
    } else {
        Err(DecompressionErrorKind::IndexOutOfBounds)
    }
}
