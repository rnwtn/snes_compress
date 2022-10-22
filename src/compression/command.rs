use super::{block::Block, command_callbacks::CommandCallback, history_table::HistoryTable};

pub struct CommandConfiguration {
    pub cmd_num: u8,
    pub cmd_size: usize,
    pub max_block_size: usize,
    pub is_extended_only: bool,
}

impl CommandConfiguration {
    pub fn new(cmd_num: u8, cmd_size: usize, max_block_size: usize, is_extended_only: bool) -> Self {
        CommandConfiguration {
            cmd_num,
            cmd_size,
            max_block_size,
            is_extended_only,
        }
    }
}

pub struct Command {
    config: CommandConfiguration,
    callback: CommandCallback,
}

impl Command {
    pub fn new(config: CommandConfiguration, callback: CommandCallback) -> Self {
        Command { config, callback }
    }

    pub fn call(&self, source: &[u8], index: usize, history_table: &HistoryTable) -> Option<Block> {
        (self.callback)(&self.config, source, index, history_table)
    }
}
