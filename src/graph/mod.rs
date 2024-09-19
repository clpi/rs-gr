use petgraph::{
    adj::{EdgeIndex, NodeIndex},
    algo::{
        astar, connected_components, greedy_feedback_arc_set, has_path_connecting,
        min_spanning_tree,
    },
    csr::IndexType,
    graph::{Edge, Graph, Node},
    prelude::StableUnGraph,
    stable_graph::{StableDiGraph, StableGraph},
    visit::{Bfs, Data, Dfs, EdgeIndexable},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    time::{Instant, SystemTime, SystemTimeError},
};
use std::{error::Error as StdError, fmt::Debug, marker::PhantomData};
use time::ext::InstantExt;

#[derive(Serialize, Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppGraph<K, V>
where
    Self: Serialize + Clone + Default + Debug + 'static,
    K: Clone + Default + Debug + 'static + Deserialize<'static> + Serialize,
    V: Serialize + Clone + Debug + Default + 'static + Deserialize<'static>,
{
    pub graph: StableDiGraph<K, V>,
    pub data: StableUnGraph<usize, BTreeMap<String, BTreeSet<String>>>,
    pub meta: BTreeMap<usize, BTreeMap<String, BTreeSet<String>>>,
    pub path: PathBuf,
}

impl<K, V> Default for AppGraph<K, V>
where
    Self: Serialize + Clone + Default + Debug + 'static,
    K: Clone + Default + Debug + 'static + Deserialize<'static> + Serialize,
    V: Serialize + Clone + Debug + Default + 'static + Deserialize<'static>,
{
    fn default() -> Self {
        Self {
            graph: StableDiGraph::new(),
            data: StableUnGraph::default(),
            meta: BTreeMap::new(),
            path: dirs::desktop_dir().unwrap_or_default().join("graph.bin"),
        }
    }
}
impl<K, V> AppGraph<K, V>
where
    Self: Serialize + Clone + Default + Debug + Deserialize<'static> + 'static,
    K: Clone + Default + Debug + 'static + Deserialize<'static> + Serialize,
    V: Serialize + Clone + Debug + Default + 'static + Deserialize<'static>,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, n: K) -> usize {
        let n = self.graph.add_node(n);
        let ni = &n.index();
        let mut created_map: BTreeMap<usize, BTreeSet<String>> = BTreeMap::new();
        created_map
            .insert(
                ni.clone() as u32,
                BTreeSet::from_iter(vec![format!("{:?}", SystemTime::now())]),
            )
            .unwrap_or_default();
        n.index()
    }

    /// from a to b
    pub fn add_edge(&mut self, a: usize, b: usize, v: V) -> usize {
        let e = self.graph.add_edge(a, b, v.clone());
        if let Some(btm) = self.meta.get_mut(&a) {
            if let Some(bt) = btm.get_mut("edges".into()) {
                bt.append(&mut BTreeSet::from_iter(vec![
                    "a".into(),
                    "b".into(),
                    format!("{:?}", v.clone()),
                ]));
            } else {
                btm.insert(
                    "edges".into(),
                    BTreeSet::from_iter(vec!["a".into(), "b".into(), format!("{:?}", v)]),
                );
            }
        } else {
            self.meta.insert(
                a.index(),
                BTreeMap::from_iter(vec![(
                    "edges".into(),
                    BTreeSet::from_iter(vec!["a".into(), "b".into(), format!("{:?}", v)]),
                )]),
            );
        }
        e.index()
    }

    pub fn get_node(&self, n: usize) -> Option<&Node<K>> {
        self.graph.node_weight(n)
    }
    pub fn get_edge(&self, e: usize) -> Option<&Edge<V>> {
        self.graph.edge_weight(e)
    }

    pub fn get_node_mut(&mut self, n: usize) -> Option<&mut Node<K>> {
        self.graph.node_weight_mut(n)
    }
    pub fn get_edge_mut(&mut self, e: usize) -> Option<&mut Edge<V>> {
        self.graph.edge_weight_mut(e)
    }
    pub fn get_node_index(&self, n: usize) -> Option<NodeIndex> {
        self.graph.node_bound(n)
    }
    pub fn get_edge_index(&self, e: usize) -> Option<EdgeIndex> {
        self.graph.edge_bound(e)
    }

    pub fn get_node_data(&self, n: usize) -> Option<&BTreeMap<String, BTreeSet<String>>>
    where
        K: Clone + Default + Debug + Deserialize<'static> + Serialize,
    {
        self.meta.get(&n)
    }
    pub fn ser(&self) -> bincode::Result<String> {
        let s = bincode::serialize(&self)?;
        Ok(String::from_utf8(s).unwrap_or_default())
    }

    pub async fn write_file(&self) -> tokio::io::Result<()> {
        let s = self.ser().expect("failed to serialize");
        tokio::fs::write(&self.path, s).await
    }

    pub async fn deser(s: &'static [u8]) -> tokio::io::Result<Self> {
        let r: Self = bincode::deserialize(s).expect("failed to deserialize");
        return Ok(r);
    }

    pub fn set_path(&mut self, p: PathBuf) {
        self.path = p;
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
    pub fn get_meta(&self, ni: usize) -> Option<&BTreeMap<String, BTreeSet<String>>>
    where
        K: Clone + Default + Debug + Deserialize<'static> + Serialize,
    {
        self.meta.get(&ni)
    }
    pub fn set_meta(&mut self, ni: usize, k: String, v: BTreeSet<String>) {
        if let Some(btm) = self.meta.get_mut(&ni) {
            btm.insert(k, v);
        } else {
            self.meta.insert(ni, BTreeMap::from_iter(vec![(k, v)]));
        }
    }

    pub async fn read_file(&self) -> tokio::io::Result<Self> {
        let s = tokio::fs::read(&self.path).await?;
        Self::deser(&s).await
    }
}
