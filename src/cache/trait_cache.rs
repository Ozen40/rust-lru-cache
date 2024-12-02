use std::collections::{HashMap, VecDeque};

pub struct Cache<K, V>
{
    size: usize,
    cache_content: HashMap<K, V>,
    cache_order: VecDeque<K>,
}

pub trait TraitCache<K ,V>
{
    fn put(&mut self, key: K, value: V);

    fn get(&mut self, key: K) -> Option<&V>;

    fn move_key_end_cache(&mut self, key: &K);
}