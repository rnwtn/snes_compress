pub struct Block {
    pub index: usize,
    pub num_bytes_consumed: usize,
    pub data: Vec<u8>,
    pub debug_message: Option<String>,
}

impl Block {
    pub fn new(index: usize, num_bytes_consumed: usize, data: Vec<u8>) -> Self {
        Block {
            index,
            num_bytes_consumed,
            data,
            debug_message: None,
        }
    }

    pub fn set_debug_message(mut self, message: &str) -> Self {
        self.debug_message = Some(message.to_owned());
        self
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_better(&self, other: &Block) -> bool {
        if self.num_bytes_consumed > other.num_bytes_consumed {
            return true
        }
        if self.num_bytes_consumed == other.num_bytes_consumed {
            return self.len() < other.len();
        }
        return false
    }

    pub fn collect(self) -> Vec<u8> {
        self.data
    }
}
