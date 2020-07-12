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
    #[inline]
    fn eq(&self, other: &KVpair<K, V>) -> bool {
        self.K == other.K &&  self.V && other.V
    }
}


impl <K, V> cmp::PartialOrd for KVpair<K, V>
where 
K: cmp::PartialOrd,
V: cmp::PartialOrd,
{

    fn partial_cmp(&self, other: &KVpair<K, V>) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn gt(&self, other: &KVpair<K, V>) -> bool {
        self.K > other.K
    }
}

pub trait Run<K, V> {
    // placeholder type

    fn new() -> Self;
    fn get_min(&mut self) -> Option<K>;
    fn get_max(&mut self) -> Option<K>;
    fn insert_key(&mut self, key: K, value: V);
    fn delete_key<Q: ?Sized>(&mut self, key: &Q) -> Option<V>;
    fn lookup(&mut self, key: K, found: bool) -> Option<V>;
    fn num_elements(&mut self) -> usize;
    fn set_size(&mut self, size: usize);
    fn get_all(&mut self) -> Vec<Option<Node<K, V>>>;
    fn get_all_in_range(&mut self, key1: K, key2: K) -> Vec<Option<Node<K,V>>>;
    fn link_length(&self, start: *mut Node<K, V>, end: Option<*mut Node<K, V>>, lvl: usize, ) -> Result<usize, bool>; 
} 
