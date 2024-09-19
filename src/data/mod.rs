pub mod cache;
pub mod memory;

use crate::fs::FileStore;
use async_trait::async_trait;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::error::Error as StdError;
use std::ptr::addr_of_mut;

pub enum DbStore {
    File(FileStore),
}

pub trait Store: Send + Sync + 'static {
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>>;
}
pub struct Db {
    store: DbStore,
}

pub trait Model: Send + Sync + 'static {}

#[async_trait::async_trait]
pub trait Adapter: Send + Sync + 'static {
    async fn load_policy(&mut self, model: &mut dyn Model) -> Result<(), Box<dyn StdError>>;

    async fn remove_policy(&mut self, model: &mut dyn Model) -> Result<(), Box<dyn StdError>>;
}

