mod block;
mod command;
mod command_callbacks;
mod history_table;
mod strategies;

use self::block::Block;

use super::CompressionType;
use crate::errors::CompressionError;
use history_table::HistoryTable;
use strategies::CompressionStrategy;

pub fn compress(
    source: &[u8],
    compression_type: CompressionType,
) -> Result<Vec<u8>, CompressionError> {
    let strategy = strategies::get_compression_strategy(compression_type).ok_or(
        CompressionError::UnsupportedFormat {
            compression_type: compression_type.to_string(),
        },
    )?;

    let mut history_table = HistoryTable::new(source);
    let mut buffer: Vec<u8> = vec![];
    let mut last_block_end_index = 0;
    let mut i = 0;
    while i < source.len() {
        if i > 0 {
            let first = source[i - 1];
            let second = source[i];
            history_table.insert(first, second, i - 1);
        }

        if i < last_block_end_index {
            i += 1;
            continue;
        }

        if let Some(block) = strategy.get_best_block(source, i, &history_table) {
            if i > last_block_end_index {
                let fallback_blocks =
                    get_fallback_blocks(&strategy, source, last_block_end_index, i, &history_table)?;
                for fallback_block in fallback_blocks {
                    buffer.append(&mut fallback_block.collect());
                }
            }
            last_block_end_index = i + block.num_bytes_consumed;
            buffer.append(&mut block.collect());
        }
        i += 1;
    }
    if i > last_block_end_index {
        let fallback_blocks =
            get_fallback_blocks(&strategy, source, last_block_end_index, i, &history_table)?;
        for fallback_block in fallback_blocks {
            buffer.append(&mut fallback_block.collect());
        }
    }
    buffer.push(0xFF);
    Ok(buffer)
}

fn get_fallback_blocks(
    strategy: &CompressionStrategy,
    source: &[u8],
    start_index: usize,
    end_index: usize,
    history_table: &HistoryTable,
) -> Result<Vec<Block>, CompressionError> {
    strategy
        .get_fallback_blocks(source, start_index, end_index, history_table)
        .ok_or(CompressionError::CompressionFailed)
}
