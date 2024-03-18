use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::ops::AddAssign;

#[derive(Clone, Default, Debug)]
pub struct ValuePair<T: AddAssign + Default + Clone> {
    pub first: T,
    pub second: T,
}

impl<T: AddAssign + Default + Clone> ValuePair<T> {
    pub fn new(first: T, second: T) -> Self {
        ValuePair { first, second }
    }
}

impl<T: AddAssign + Default + Clone> AddAssign for ValuePair<T> {
    fn add_assign(&mut self, other: Self) {
        self.first += other.first;
        self.second += other.second;
    }
}

#[derive(Clone, Default, Debug)]
pub struct DynamicLimitedSizeMap<K, V>
where
    K: Eq + Hash + Clone,
    V: AddAssign + Default + Clone,
{
    max_size: usize,
    keys_order: VecDeque<K>,
    map: HashMap<K, V>,
}

impl<K, V> DynamicLimitedSizeMap<K, ValuePair<V>>
where
    K: Eq + Hash + Clone,
    V: AddAssign + Default + Clone + From<u32>,
{
    pub fn new(max_size: usize) -> Self {
        DynamicLimitedSizeMap {
            max_size,
            keys_order: VecDeque::with_capacity(max_size),
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: K, increment1: V, increment2: V) {
        if !self.map.contains_key(&key) && self.map.len() == self.max_size {
            if let Some(oldest_key) = self.keys_order.pop_front() {
                self.map.remove(&oldest_key);
            }
        }

        let entry = self.map.entry(key.clone()).or_insert_with(|| ValuePair {
            first: 1.into(),
            second: 1.into(),
        });

        entry.first += increment1;
        entry.second += increment2;

        if !self.keys_order.contains(&key) {
            self.keys_order.push_back(key);
        }
    }

    pub fn get(&self, key: &K) -> Option<&ValuePair<V>> {
        self.map.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut ValuePair<V>> {
        self.map.get_mut(key)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<K, ValuePair<V>> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<K, ValuePair<V>> {
        self.map.iter_mut()
    }
}

#[derive(Clone, Default, Debug)]
pub struct LimitedSizeMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Default + Clone,
{
    max_size: usize,
    keys_order: VecDeque<K>,
    map: HashMap<K, V>,
}

impl<K, V> LimitedSizeMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Default + Clone,
{
    pub fn new(max_size: usize) -> Self {
        LimitedSizeMap {
            max_size,
            keys_order: VecDeque::with_capacity(max_size),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if !self.map.contains_key(&key) && self.map.len() == self.max_size {
            if let Some(oldest_key) = self.keys_order.pop_front() {
                self.map.remove(&oldest_key);
            }
        }

        self.map.insert(key.clone(), value);

        if !self.keys_order.contains(&key) {
            self.keys_order.push_back(key);
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.map.get_mut(key)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<K, V> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<K, V> {
        self.map.iter_mut()
    }
}
