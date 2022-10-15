use std::cmp;

use super::block::Block;
use super::command::{Command, CommandConfiguration};
use super::command_callbacks as commands;
use super::history_table::HistoryTable;
use super::CompressionType;
use commands::CommandCallback;

pub struct CompressionStrategy {
    commands: Vec<Command>,
    fallback_command: Option<Command>,
    cmd_size: usize,
    max_block_size: usize,
}

impl CompressionStrategy {
    pub fn get_best_block(
        &self,
        source: &[u8],
        start_index: usize,
        history_table: &HistoryTable,
    ) -> Option<Block> {
        self.commands.iter().fold(None, |best_block, command| {
            let source = get_source_slice(source, start_index, self.max_block_size);
            if let Some(current_block) = command.call(source, start_index, history_table) {
                match &best_block {
                    Some(best) => {
                        if current_block.ratio() < best.ratio() {
                            return Some(current_block);
                        }
                    }
                    None => return Some(current_block),
                }
            }
            best_block
        })
    }

    pub fn get_fallback_blocks(
        &self,
        source: &[u8],
        start_index: usize,
        end_index: usize,
        history_table: &HistoryTable,
    ) -> Option<Vec<Block>> {
        if self.fallback_command.is_none() {
            return None;
        }
        let max_size = end_index - start_index;
        let mut blocks = Vec::new();
        let num_iterations = (max_size as f32 / self.max_block_size as f32).ceil() as usize;
        for i in 0..num_iterations {
            let source = get_source_slice(source, start_index, max_size);
            let fallback_command = self.fallback_command.as_ref().unwrap();
            blocks.push(fallback_command.call(source, start_index, history_table)?);
        }
        Some(blocks)
    }

    fn new(cmd_size: usize, max_block_size: usize) -> Self {
        CompressionStrategy {
            commands: Vec::new(),
            fallback_command: None,
            cmd_size,
            max_block_size,
        }
    }

    fn insert_command(
        mut self,
        cmd_num: u8,
        is_extended_only: bool,
        callback: CommandCallback,
    ) -> Self {
        let cmd_size = self.cmd_size;
        let max_block_size = self.max_block_size;
        let command_config =
            CommandConfiguration::new(cmd_num, cmd_size, max_block_size, is_extended_only);
        let cmd = Command::new(command_config, callback);
        self.commands.push(cmd);
        self
    }

    fn set_fallback_command(
        mut self,
        cmd_num: u8,
        is_extended_only: bool,
        callback: CommandCallback,
    ) -> Self {
        let cmd_size = self.cmd_size;
        let max_block_size = self.max_block_size;
        let command_config =
            CommandConfiguration::new(cmd_num, cmd_size, max_block_size, is_extended_only);
        let cmd = Command::new(command_config, callback);
        self.fallback_command = Some(cmd);
        self
    }
}

pub fn get_compression_strategy(compression_type: CompressionType) -> Option<CompressionStrategy> {
    match compression_type {
        CompressionType::LZ5 => Some(lz5_compression_strategy()),
    }
}

// https://github.com/bonimy/MushROMs/blob/master/doc/LC_LZ5%20Compression%20Format.md
fn lz5_compression_strategy() -> CompressionStrategy {
    let cmd_size = 3;
    let max_block_size = 1024;
    CompressionStrategy::new(cmd_size, max_block_size)
        .set_fallback_command(0b000, false, commands::direct_copy)
        .insert_command(0b001, false, commands::byte_fill)
        .insert_command(0b010, false, commands::word_fill)
        .insert_command(0b011, false, commands::increasing_fill)
        .insert_command(0b100, false, commands::repeat_le)
        .insert_command(0b101, false, commands::xor_repeat_le)
        .insert_command(0b110, false, commands::negative_repeat)
        .insert_command(0b111, true, commands::negative_xor_repeat)
}

fn get_source_slice(source: &[u8], start_index: usize, max_size: usize) -> &[u8] {
    let upper_bound = cmp::min(start_index + max_size, source.len());
    &source[start_index..upper_bound]
}
