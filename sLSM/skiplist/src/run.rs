use std::cmp;
use std::borrow::Borrow;

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

pub struct Iter<'a, K: 'a, V: 'a> {
    start: *const Node<K, V>,
    end: *const Node<K, V>,
    size: usize,
    _lifetime_k: PhantomData<&'a K>,
    _lifetime_v: PhantomData<&'a V>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)>{
        unsafe {
            if self.start == self.end {
                return None;
            }
            if let Some(next) = (*self.start).forwards[0] {
                self.start = next;
                if self.size > 0 {
                    self.size -= 1;
                }
                return Some((
                    (*self.start).key.as_ref().unwrap(),
                    (*self.start).value.as_ref().unwrap(),
                ))
            }
        }

    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

pub trait Run<K, V> {

    fn new() -> Self;
    fn get_min(&mut self) -> Option<K>;
    fn get_max(&mut self) -> Option<K>;
    fn insert_key(&mut self, key: K, value: V);
    fn delete_key<Q: ?Sized>(&mut self, key: &Q) -> Option<V> where K: Borrow<Q>, Q:Ord;
    fn lookup<Q: ?Sized>(&self, key: &Q, found: bool) -> Option<&V> where K: Borrow<Q>, Q:Ord;
    fn num_elements(&self) -> i64;
    fn set_size(&mut self, size: usize);
    fn get_all(&mut self) -> Vec<Option<Node<K, V>>>;
    fn get_all_in_range(&mut self, key1: K, key2: K) -> Vec<Option<Node<K,V>>>;
    fn range<Q>(&self, min: Bound<&Q>, max: Bound<&Q>) -> Iter<K, V> where K: Borrow<Q>, Q: Ord;
    fn link_length(&self, start: *mut Node<K, V>, end: Option<*mut Node<K, V>>, lvl: usize, ) -> Result<usize, bool>; 
} 
