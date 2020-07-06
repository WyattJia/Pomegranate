use std::cmp;

use crate::node::Node;

pub struct KVpair<K, V> {
    pub key: Option<K>,
    pub value: Option<V>
}

// todo impl KVpair compare struct
impl <K, V> cmp::PartialEq for KVpair<K, V>
where
K: cmp::PartialEq,
V: cmp::PartialEq,
{
    fn eq(&self, other: Self) -> bool {
        self.key == other.key &&  self.value && other.value
    }
}

impl <K, V> cmp::PartialOrd for KVpair<K, V>
where 
K: cmp::PartialOrd,
V: cmp::PartialOrd,
{
    fn gt(&self, &other: Self) -> bool {
        self.key > other.key
    }
}

pub trait Run<K, V> {
    // placeholder type

    fn new() -> Self;
    fn get_min(&mut self) -> Option<K>;
    fn get_max(&mut self) -> Option<K>;
    fn insert_key(&mut self, key: K, value: V);
    fn delete_key(&mut self, key: K);
    fn lookup(&mut self, key: K, found: bool) -> Option<V>;
    fn num_elements(&mut self) -> usize;
    fn set_size(&mut self, size: usize);
    fn get_all(&mut self) -> Vec<Option<Node<K, V>>>;
    fn get_all_in_range(&mut self, key1: K, key2: K) -> Vec<Option<Node<K,V>>>;
} 
