use std::cmp;

use super::{block::Block, command::CommandConfiguration, history_table::HistoryTable};

pub type CommandCallback = fn(&CommandConfiguration, &[u8], usize, &HistoryTable) -> Option<Block>;

pub fn direct_copy(
    cmd_config: &CommandConfiguration,
    source: &[u8],
    index: usize,
    _history: &HistoryTable,
) -> Option<Block> {
    if source.is_empty() {
        return None;
    }
    let num_bytes_consumed = source.len();
    let mut data = build_command_bytes(cmd_config, num_bytes_consumed);
    let mut arguments = source.to_owned();
    data.append(&mut arguments);
    let block = Block::new(index, num_bytes_consumed, data).set_debug_message("direct copy");
    Some(block)
}

pub fn byte_fill(
    cmd_config: &CommandConfiguration,
    source: &[u8],
    index: usize,
    _history: &HistoryTable,
) -> Option<Block> {
    let &first_byte = source.get(0)?;
    let mut num_bytes_consumed = 0;

    for i in 0..source.len() {
        if first_byte != source[i] {
            break;
        }
        num_bytes_consumed += 1;
    }

    let mut data = build_command_bytes(cmd_config, num_bytes_consumed);
    data.push(first_byte);
    if num_bytes_consumed <= data.len() {
        return None;
    }
    let block = Block::new(index, num_bytes_consumed, data).set_debug_message("byte fill");
    Some(block)
}

pub fn word_fill(
    cmd_config: &CommandConfiguration,
    source: &[u8],
    index: usize,
    _history: &HistoryTable,
) -> Option<Block> {
    let &first_byte = source.get(0)?;
    let &second_byte = source.get(1)?;
    let mut num_bytes_consumed = 0;

    for i in 0..source.len() {
        if i % 2 == 0 && source[i] != first_byte {
            break;
        }
        if i % 2 == 1 && source[i] != second_byte {
            break;
        }
        num_bytes_consumed += 1;
    }

    let mut data = build_command_bytes(cmd_config, num_bytes_consumed);
    data.push(first_byte);
    data.push(second_byte);
    if num_bytes_consumed <= data.len() {
        return None;
    }
    let block = Block::new(index, num_bytes_consumed, data).set_debug_message("word fill");
    Some(block)
}

pub fn increasing_fill(
    cmd_config: &CommandConfiguration,
    source: &[u8],
    index: usize,
    _history: &HistoryTable,
) -> Option<Block> {
    let &first_byte = source.get(0)?;
    let mut num_bytes_consumed = 0;
    let mut next_byte = first_byte;
    for i in 0..source.len() {
        // TODO: should this be able to handle overflows???
        if next_byte != source[i] {
            break;
        }
        num_bytes_consumed += 1;
        if next_byte == 0xFF {
            break;
        }
        next_byte += 1;
    }

    let mut data = build_command_bytes(cmd_config, num_bytes_consumed);
    data.push(first_byte);
    if num_bytes_consumed <= data.len() {
        return None;
    }
    let block = Block::new(index, num_bytes_consumed, data).set_debug_message("increasing fill");
    Some(block)
}

pub fn repeat_le(
    cmd_config: &CommandConfiguration,
    source: &[u8],
    index: usize,
    history: &HistoryTable,
) -> Option<Block> {
    let repeat_info = history.find_longest_repeat(source, 0)?;

    let num_bytes_consumed = repeat_info.size;
    if num_bytes_consumed == 0 {
        return None;
    }

    let mut data = build_command_bytes(cmd_config, num_bytes_consumed);
    data.append(&mut transform_into_bytes_le(repeat_info.start_index));
    if num_bytes_consumed <= data.len() {
        return None;
    }
    let block = Block::new(index, num_bytes_consumed, data).set_debug_message("repeat le");
    Some(block)
}

pub fn xor_repeat_le(
    cmd_config: &CommandConfiguration,
    source: &[u8],
    index: usize,
    history: &HistoryTable,
) -> Option<Block> {
    let repeat_info = history.find_longest_repeat_xor(source, 0)?;

    let num_bytes_consumed = repeat_info.size;
    if num_bytes_consumed == 0 {
        return None;
    }

    let mut data = build_command_bytes(cmd_config, num_bytes_consumed);
    data.append(&mut transform_into_bytes_le(repeat_info.start_index));
    if num_bytes_consumed <= data.len() {
        return None;
    }
    let block = Block::new(index, num_bytes_consumed, data).set_debug_message("xor repeat le");
    Some(block)
}

pub fn negative_repeat(
    cmd_config: &CommandConfiguration,
    source: &[u8],
    index: usize,
    history: &HistoryTable,
) -> Option<Block> {
    let lower_bound = index - cmp::min(255, index);
    let repeat_info = history.find_longest_repeat(source, lower_bound)?;

    let num_bytes_consumed = repeat_info.size;
    if num_bytes_consumed == 0 {
        return None;
    }

    let mut data = build_command_bytes(cmd_config, num_bytes_consumed);
    data.push((index - repeat_info.start_index) as u8);
    if num_bytes_consumed <= data.len() {
        return None;
    }
    let block = Block::new(index, num_bytes_consumed, data).set_debug_message("negative repeat");
    Some(block)
}

pub fn negative_xor_repeat(
    cmd_config: &CommandConfiguration,
    source: &[u8],
    index: usize,
    history: &HistoryTable,
) -> Option<Block> {
    let lower_bound = index - cmp::min(255, index);
    let repeat_info = history.find_longest_repeat_xor(source, lower_bound)?;

    let num_bytes_consumed = repeat_info.size;
    if num_bytes_consumed == 0 {
        return None;
    }

    let mut data = build_command_bytes(cmd_config, num_bytes_consumed);
    data.push((index - repeat_info.start_index) as u8);
    if num_bytes_consumed <= data.len() {
        return None;
    }
    let block =
        Block::new(index, num_bytes_consumed, data).set_debug_message("negative xor repeat");
    Some(block)
}

fn transform_into_bytes_le(val: usize) -> Vec<u8> {
    let first = val as u8;
    let second = (val >> 8) as u8;
    vec![first, second]
}

fn build_command_bytes(cmd_config: &CommandConfiguration, num_bytes_consumed: usize) -> Vec<u8> {
    let cmd = cmd_config.cmd_num;
    let cmd_size = cmd_config.cmd_size;
    let extended_threshold = 2_usize.pow(8 - cmd_size as u32);
    let size = num_bytes_consumed - 1;
    let is_extended = size >= extended_threshold || cmd_config.is_extended_only;

    let shift_width = 8 - cmd_size;
    let extended_mask = {
        if is_extended {
            0xFF << shift_width
        } else {
            0
        }
    };
    let cmd_mask = {
        if is_extended {
            cmd << (shift_width - cmd_size)
        } else {
            cmd << shift_width
        }
    };
    let size_mask = {
        if is_extended {
            (size >> 8) as u8
        } else {
            size as u8
        }
    };
    let mut command_bytes = vec![];
    command_bytes.push(bit_or(extended_mask, cmd_mask, size_mask));
    if is_extended {
        command_bytes.push((size & 0xFF) as u8);
    }
    command_bytes
}

fn bit_or(a: u8, b: u8, c: u8) -> u8 {
    a | b | c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_copy_works() {
        let cmd_config = CommandConfiguration::new(0b000, 3, 1024, false);
        let source = &[0xAA, 0xAB, 0xAC, 0xAD];
        let history_table = HistoryTable::new(source);
        let block = direct_copy(&cmd_config, source, 0, &history_table).unwrap();
        assert_eq!(block.index, 0);
        assert_eq!(block.num_bytes_consumed, 4);
        assert_eq!(block.data, vec![0x03, 0xAA, 0xAb, 0xAC, 0xAD]);
    }

    #[test]
    fn direct_copy_returns_none_if_source_is_empty() {
        let cmd_config = CommandConfiguration::new(0b000, 3, 1024, false);
        let source = &[];
        let history_table = HistoryTable::new(source);
        let block = direct_copy(&cmd_config, source, 0, &history_table);
        assert!(block.is_none());
    }

    #[test]
    fn byte_fill_works() {
        let cmd_config = CommandConfiguration::new(0b001, 3, 1024, false);
        let source = &[0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA];
        let history_table = HistoryTable::new(source);
        let block = byte_fill(&cmd_config, source, 0, &history_table).unwrap();
        assert_eq!(block.index, 0);
        assert_eq!(block.num_bytes_consumed, 7);
        assert_eq!(block.data, vec![0x26, 0xAA]);
    }

    #[test]
    fn byte_fill_returns_none_if_source_is_empty() {
        let cmd_config = CommandConfiguration::new(0b001, 3, 1024, false);
        let source = &[];
        let history_table = HistoryTable::new(source);
        let block = byte_fill(&cmd_config, source, 0, &history_table);
        assert!(block.is_none());
    }

    #[test]
    fn word_fill_works() {
        let cmd_config = CommandConfiguration::new(0b010, 3, 1024, false);
        let source = &[0xAA, 0xBB, 0xAA, 0xBB, 0xAA, 0xBB, 0xAA];
        let history_table = HistoryTable::new(source);
        let block = word_fill(&cmd_config, source, 0, &history_table).unwrap();
        assert_eq!(block.index, 0);
        assert_eq!(block.num_bytes_consumed, 7);
        assert_eq!(block.data, vec![0x46, 0xAA, 0xBB]);
    }

    #[test]
    fn word_fill_returns_none_if_source_is_one_byte() {
        let cmd_config = CommandConfiguration::new(0b010, 3, 1024, false);
        let source = &[0xAA];
        let history_table = HistoryTable::new(source);
        let block = word_fill(&cmd_config, source, 0, &history_table);
        assert!(block.is_none());
    }

    #[test]
    fn word_fill_returns_none_if_source_is_empty() {
        let cmd_config = CommandConfiguration::new(0b010, 3, 1024, false);
        let source = &[];
        let history_table = HistoryTable::new(source);
        let block = word_fill(&cmd_config, source, 0, &history_table);
        assert!(block.is_none());
    }

    #[test]
    fn increasing_fill_works() {
        let cmd_config = CommandConfiguration::new(0b011, 3, 1024, false);
        let source = &[0xAA, 0xAB, 0xAC, 0xAD, 0xAE, 0xAF, 0xB0];
        let history_table = HistoryTable::new(source);
        let block = increasing_fill(&cmd_config, source, 0, &history_table).unwrap();
        assert_eq!(block.index, 0);
        assert_eq!(block.num_bytes_consumed, 7);
        assert_eq!(block.data, vec![0x66, 0xAA]);
    }

    #[test]
    fn increasing_fill_returns_none_if_source_is_empty() {
        let cmd_config = CommandConfiguration::new(0b011, 3, 1024, false);
        let source = &[];
        let history_table = HistoryTable::new(source);
        let block = increasing_fill(&cmd_config, source, 0, &history_table);
        assert!(block.is_none());
    }

    #[test]
    fn repeat_le_works() {
        let cmd_config = CommandConfiguration::new(0b100, 3, 1024, false);
        let source = b"ASDF_APPLEAPPLE";
        let mut history_table = HistoryTable::new(source);
        history_table.insert(b'A', b'P', 5);
        let block = repeat_le(&cmd_config, &source[10..], 10, &history_table).unwrap();
        assert_eq!(block.index, 10);
        assert_eq!(block.num_bytes_consumed, 5);
        assert_eq!(block.data, vec![0x84, 0x05, 0x00]);
    }

    #[test]
    fn repeat_le_returns_none_if_source_is_empty() {
        let cmd_config = CommandConfiguration::new(0b100, 3, 1024, false);
        let source = &[];
        let history_table = HistoryTable::new(source);
        let block = repeat_le(&cmd_config, source, 0, &history_table);
        assert!(block.is_none());
    }

    #[test]
    fn xor_repeat_le_works() {
        let cmd_config = CommandConfiguration::new(0b101, 3, 1024, false);
        let mut source: Vec<u8> = b"ASDF_APPLE".iter().map(|&x| x).collect();
        source.append(&mut b"APPLE".iter().map(|&x| x ^ 0xFF).collect());
        let mut history_table = HistoryTable::new(&source);
        history_table.insert(b'A', b'P', 5);
        let block = xor_repeat_le(&cmd_config, &source[10..], 10, &history_table).unwrap();
        assert_eq!(block.index, 10);
        assert_eq!(block.num_bytes_consumed, 5);
        assert_eq!(block.data, vec![0xA4, 0x05, 0x00]);
    }

    #[test]
    fn xor_repeat_le_returns_none_if_source_is_empty() {
        let cmd_config = CommandConfiguration::new(0b101, 3, 1024, false);
        let source = &[];
        let history_table = HistoryTable::new(source);
        let block = xor_repeat_le(&cmd_config, source, 0, &history_table);
        assert!(block.is_none());
    }

    #[test]
    fn negative_repeat_works() {
        let cmd_config = CommandConfiguration::new(0b110, 3, 1024, false);
        let source = b"ASDF_ASDF_APPLEAPPLE";
        let mut history_table = HistoryTable::new(source);
        history_table.insert(b'A', b'P', 10);
        let block = negative_repeat(&cmd_config, &source[15..], 15, &history_table).unwrap();
        assert_eq!(block.index, 15);
        assert_eq!(block.num_bytes_consumed, 5);
        assert_eq!(block.data, vec![0xC4, 0x05]);
    }

    #[test]
    fn negative_repeat_returns_none_if_source_is_empty() {
        let cmd_config = CommandConfiguration::new(0b110, 3, 1024, false);
        let source = &[];
        let history_table = HistoryTable::new(source);
        let block = negative_repeat(&cmd_config, source, 0, &history_table);
        assert!(block.is_none());
    }

    #[test]
    fn negative_xor_repeat_le_works() {
        let cmd_config = CommandConfiguration::new(0b111, 3, 1024, true);
        let mut source: Vec<u8> = b"ASDF_APPLE".iter().map(|&x| x).collect();
        source.append(&mut b"APPLE".iter().map(|&x| x ^ 0xFF).collect());
        let mut history_table = HistoryTable::new(&source);
        history_table.insert(b'A', b'P', 5);
        let block = xor_repeat_le(&cmd_config, &source[10..], 10, &history_table).unwrap();
        assert_eq!(block.index, 10);
        assert_eq!(block.num_bytes_consumed, 5);
        assert_eq!(block.data, vec![0xFC, 0x04, 0x05, 0x00]);
    }

    #[test]
    fn negative_xor_repeat_le_returns_none_if_source_is_empty() {
        let cmd_config = CommandConfiguration::new(0b111, 3, 1024, true);
        let source = &[];
        let history_table = HistoryTable::new(source);
        let block = xor_repeat_le(&cmd_config, source, 0, &history_table);
        assert!(block.is_none());
    }
}
