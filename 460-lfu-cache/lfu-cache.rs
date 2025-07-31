use std::collections::{HashMap, HashSet, VecDeque};

struct LFUCache {
    capacity: usize,
    min_freq: i32,
    key_val: HashMap<i32, i32>,
    key_freq: HashMap<i32, i32>,
    freq_keys: HashMap<i32, VecDeque<i32>>,
    key_pos: HashMap<i32, usize>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            min_freq: 0,
            key_val: HashMap::new(),
            key_freq: HashMap::new(),
            freq_keys: HashMap::new(),
            key_pos: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if !self.key_val.contains_key(&key) {
            return -1;
        }
        self.update_freq(key);
        self.key_val[&key]
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        if self.key_val.contains_key(&key) {
            self.key_val.insert(key, value);
            self.update_freq(key);
            return;
        }

        if self.key_val.len() == self.capacity {
            if let Some(keys) = self.freq_keys.get_mut(&self.min_freq) {
                if let Some(evict_key) = keys.pop_front() {
                    self.key_val.remove(&evict_key);
                    self.key_freq.remove(&evict_key);
                }
            }
        }
        self.key_val.insert(key, value);
        self.key_freq.insert(key, 1);
        self.freq_keys.entry(1).or_default().push_back(key);
        self.min_freq = 1;
    }

    fn update_freq(&mut self, key: i32) {
        let freq = self.key_freq[&key];
        self.key_freq.insert(key, freq + 1);
        if let Some(keys) = self.freq_keys.get_mut(&freq) {
            if let Some(pos) = keys.iter().position(|&x| x == key) {
                keys.remove(pos);
            }
            if keys.is_empty() {
                self.freq_keys.remove(&freq);
                if self.min_freq == freq {
                    self.min_freq += 1;
                }
            }
        }
        self.freq_keys.entry(freq + 1).or_default().push_back(key);
    }
}