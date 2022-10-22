use std::collections::HashMap;

type WordMap = HashMap<u8, HashMap<u8, Vec<usize>>>;

pub struct RepeatInformation {
    pub start_index: usize,
    pub size: usize,
}

impl RepeatInformation {
    fn new(start_index: usize, size: usize) -> Self {
        RepeatInformation { start_index, size }
    }
}

pub struct HistoryTable<'a> {
    source: &'a [u8],
    map: WordMap,
}

impl HistoryTable<'_> {
    pub fn new(source: &[u8]) -> HistoryTable {
        HistoryTable {
            map: HashMap::new(),
            source,
        }
    }

    pub fn insert(&mut self, first: u8, second: u8, index: usize) {
        let internal_map = match self.map.get_mut(&first) {
            Some(internal_map) => internal_map,
            None => {
                let internal_map = HashMap::new();
                self.map.insert(first, internal_map);
                self.map.get_mut(&first).unwrap()
            }
        };
        let index_collection = match internal_map.get_mut(&second) {
            Some(index_collection) => index_collection,
            None => {
                let index_collection = Vec::new();
                internal_map.insert(second, index_collection);
                internal_map.get_mut(&second).unwrap()
            }
        };
        index_collection.push(index);
    }

    pub fn find_longest_repeat(
        &self,
        source: &[u8],
        lower_bound: usize,
    ) -> Option<RepeatInformation> {
        let &first_byte = source.get(0)?;
        let &second_byte = source.get(1)?;
        let indices = self.get_indices(first_byte, second_byte, lower_bound)?;

        let mut best_index = 0;
        let mut best_size = 0;
        indices.iter().for_each(|x| {
            let mut iter_index = *x;
            let mut compare_index = 0;
            let mut num_same = 0;
            loop {
                let a = self.source.get(iter_index);
                let b = source.get(compare_index);
                if a.is_none() || b.is_none() || a != b {
                    if num_same > best_size {
                        best_index = *x;
                        best_size = num_same;
                    }
                    break;
                }
                num_same += 1;
                iter_index += 1;
                compare_index += 1;
            }
        });
        if best_size == 0 {
            None
        } else {
            Some(RepeatInformation::new(best_index, best_size))
        }
    }

    pub fn find_longest_repeat_xor(
        &self,
        source: &[u8],
        lower_bound: usize,
    ) -> Option<RepeatInformation> {
        let &first_byte = source.get(0)?;
        let &second_byte = source.get(1)?;
        let first_byte = first_byte ^ 0xFF;
        let second_byte = second_byte ^ 0xFF;
        let indices = self.get_indices(first_byte, second_byte, lower_bound)?;

        let mut best_index = 0;
        let mut best_size = 0;
        indices.iter().for_each(|x| {
            let mut iter_index = *x;
            let mut compare_index = 0;
            let mut num_same = 0;
            loop {
                let a = self.source.get(iter_index);
                let b = source.get(compare_index);

                if a.is_none() || b.is_none() {
                    if num_same > best_size {
                        best_index = *x;
                        best_size = num_same;
                    }
                    break;
                }

                let a = a.unwrap() ^ 0xFF;
                let b = b.unwrap().to_owned();
                if a != b {
                    if num_same > best_size {
                        best_index = *x;
                        best_size = num_same;
                    }
                    break;
                }

                num_same += 1;
                iter_index += 1;
                compare_index += 1;
            }
        });
        if best_size == 0 {
            None
        } else {
            Some(RepeatInformation::new(best_index, best_size))
        }
    }

    fn get_indices(&self, first: u8, second: u8, lower_bound: usize) -> Option<Vec<usize>> {
        let internal_map = self.map.get(&first)?;
        let indices = internal_map.get(&second)?;
        if indices.is_empty() {
            None
        } else {
            let indices = indices
                .iter()
                .filter(|&&x| x >= lower_bound)
                .map(|&x| x)
                .collect();
            Some(indices)
        }
    }
}
