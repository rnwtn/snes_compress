use std::collections::HashMap;

use super::command_callbacks as commands;
use super::CompressionType;
use commands::CommandCallback;

type CommandMap = HashMap<u8, CommandCallback>;

pub struct DecompressionStrategy {
    command_map: CommandMap,
    ext_command_map: CommandMap,
}

impl DecompressionStrategy {
    pub fn get_command_callback(&self, cmd_bits: u8, is_extended: bool) -> Option<CommandCallback> {
        let temp = if is_extended && self.ext_command_map.contains_key(&cmd_bits) {
            self.ext_command_map.get(&cmd_bits)
        } else {
            self.command_map.get(&cmd_bits)
        };

        match temp {
            Some(&callback) => Some(callback),
            None => None,
        }
    }

    fn new() -> DecompressionStrategy {
        DecompressionStrategy {
            command_map: CommandMap::new(),
            ext_command_map: CommandMap::new(),
        }
    }

    fn insert_command(
        mut self,
        command: u8,
        is_extended_only: bool,
        callback: CommandCallback,
    ) -> Self {
        if is_extended_only {
            self.ext_command_map.insert(command, callback);
        } else {
            self.command_map.insert(command, callback);
        }
        self
    }
}

pub fn get_decompression_strategy(
    compression_type: CompressionType,
) -> Option<DecompressionStrategy> {
    match compression_type {
        CompressionType::LZ5 => Some(lz5_decomp_strategy()),
    }
}

// https://github.com/bonimy/MushROMs/blob/master/doc/LC_LZ5%20Compression%20Format.md
fn lz5_decomp_strategy() -> DecompressionStrategy {
    DecompressionStrategy::new()
        .insert_command(0b000, false, commands::direct_copy)
        .insert_command(0b001, false, commands::byte_fill)
        .insert_command(0b010, false, commands::word_fill)
        .insert_command(0b011, false, commands::increasing_fill)
        .insert_command(0b100, false, commands::repeat_le)
        .insert_command(0b101, false, commands::xor_repeat_le)
        .insert_command(0b110, false, commands::negative_repeat)
        .insert_command(0b111, true, commands::negative_xor_repeat_le)
}
