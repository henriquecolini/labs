use std::collections::{
    btree_map::{Iter, Keys, Values},
    BTreeMap,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table<T> {
    m: BTreeMap<usize, T>,
    inc: usize,
}

impl<T> Default for Table<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl<T> Table<T> {
    pub fn new() -> Self {
        Self {
            m: Default::default(),
            inc: Default::default(),
        }
    }
    pub fn insert(&mut self, value: T) -> usize {
        let key = self.inc;
        self.m.insert(key, value);
        self.inc += 1;
        key
    }
    pub fn insert_unique(&mut self, t: T) -> usize
    where
        T: PartialEq,
    {
        match self.iter().find(|(_, r)| r == &&t) {
            Some((i, _)) => *i,
            None => self.insert(t),
        }
    }
    pub fn retain(&mut self, f: impl FnMut(&usize, &mut T) -> bool) {
        self.m.retain(f);
    }
    pub fn get(&self, k: usize) -> Option<&T> {
        self.m.get(&k)
    }
    pub fn iter(&self) -> Iter<'_, usize, T> {
        self.m.iter()
    }
    pub fn keys(&self) -> Keys<'_, usize, T> {
        self.m.keys()
    }
    pub fn values(&self) -> Values<'_, usize, T> {
        self.m.values()
    }
    pub fn contains(&self, needle: &T) -> bool
    where
        T: PartialEq,
    {
        self.m.values().any(|v| v == needle)
    }
}
