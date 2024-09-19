use std::{fmt::Debug, collections::BTreeMap, hash::Hash};
use async_trait::async_trait;
use std::task::Context;
pub trait Cacheable<K, V>
where 
    K: Eq + Hash,
    V: Clone,
{
    fn get(&self, key: &K) -> Option<&V>;
    fn has(&self, key: &K) -> bool;
    fn set(&mut self, key: K, value: V) -> Option<V>;
    fn delete(&mut self, key: &K) -> Option<V>;
    fn clear(&mut self) -> Option<V>;
}
pub struct Cache<K, V>
where 
    K: Eq + Hash + Debug + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + Debug + 'static,
{
    pub cache: BTreeMap<K, V>,

}

impl<K, V> Cacheable<K, V> for Cache<K, V>
where 
    K: Eq + Hash + Debug + Clone + Ord + Send + Sync + 'static,
    V: Clone + Send + Sync + Debug + 'static,
{
    fn get(&self, key: &K) -> Option<&V> {
        self.cache.get(key)
    }
    fn clear(&mut self) -> Option<V> {
        self.cache.clear();
        None
    }
    fn delete(&mut self, key: &K) -> Option<V> {
        self.cache.remove(key)
    }
    fn set(&mut self, key: K, value: V) -> Option<V> {
        self.cache.insert(key, value)
    }
    fn has(&self, key: &K) -> bool {
        self.cache.contains_key(key)
    }
}
