use std::cmp::PartialEq;
use std::cmp::Ordering;
use std::borrow::Borrow;
use std::ops::Bound;
use std::marker::PhantomData;

use crate::node::Node;

// Key-value pair struct start
pub struct KVpair<K, V> {
    pub key: Option<K>,
    pub value: Option<V>
}

// impl <K, V> Iterator for KVpair<K, V> {
//     fn iter(){
//         println!("impl iter.")
//     }
// }


impl <K: Ord, V: Ord> Ord for KVpair<K, V> 
{
    #[inline]
    fn cmp(&self, other: &KVpair<K, V>) -> Ordering {
        self.key.cmp(&other.key)
    }
}

// impl <K, V> Default for KVpair<K, V> 
// where 
// {
// 
// }

impl <K, V> PartialEq for KVpair<K, V>
where
K: PartialEq,
V: PartialEq,
{
    #[inline]
    fn eq(&self, other: &KVpair<K, V>) -> bool {
        self.key == other.key && self.value == other.value
    }

    #[inline]
    fn ne(&self, other: &KVpair<K, V>) -> bool {
        self.key != other.key && self.key != other.key
    }
}

impl<K: Eq, V: Eq> Eq for KVpair<K, V> {}

impl <K, V> PartialOrd for KVpair<K, V>
where
K: PartialOrd + PartialEq,
V: PartialOrd + PartialEq,
{
    #[inline]
    fn partial_cmp(&self, other: &KVpair<K, V>) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }

    #[inline]
    fn gt(&self, other: &KVpair<K, V>) -> bool {
        self.key > other.key
    }

    #[inline]
    fn ge(&self, other: &KVpair<K, V>) -> bool {
        self.key >= other.key
    }

    #[inline]
    fn lt(&self, other: &KVpair<K, V>) -> bool {
       self.key < other.key
    }

    #[inline]
    fn le(&self, other: &KVpair<K, V>) -> bool {
        self.key <= other.key
    }
}
// Key value pair struct end



// Run Iterator 
pub struct Iter<'a, K: 'a, V: 'a> {
    start: *const Node<K, V>,
    end: *const Node<K, V>,
    size: usize,
    _lifetime_k: PhantomData<&'a K>,
    _lifetime_v: PhantomData<&'a V>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

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
                        ));
            }
            None
        }

    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}
// Run Iterator end

pub trait Run<K, V> {

    fn new() -> Self;
    fn get_min(&mut self) -> Option<K>;
    fn get_max(&mut self) -> Option<K>;
    fn insert_key(&mut self, key: K, value: V);
    fn delete_key<Q: ?Sized>(&mut self, key: &Q) -> Option<V> where K: Borrow<Q>, Q:Ord;
    fn lookup<Q: ?Sized>(&self, key: &Q, found: bool) -> Option<&V> where K: Borrow<Q>, Q:Ord;
    fn find_key<Q: ?Sized>(&self, key: &Q) -> *const Node<K, V> where K: Borrow<Q>, Q:Ord;
    fn num_elements(&self) -> i64;
    fn set_size(&mut self, size: usize);
    fn get_last(&self) -> *const Node<K, V>;
    fn get_all(&mut self) -> Vec<Option<Node<K, V>>>;
    fn get_all_in_range(&mut self, key1: K, key2: K) -> Vec<Option<Node<K,V>>>;
    fn range<Q>(&self, min: Bound<&Q>, max: Bound<&Q>) -> Iter<K, V> where K: Borrow<Q>, Q: Ord;
    fn link_length(&self, start: *mut Node<K, V>, end: Option<*mut Node<K, V>>, lvl: usize, ) -> Result<usize, bool>; 
    fn contains_key<Q: ?Sized>(&self, key:&Q) -> bool where K: Borrow<Q>, Q: Ord;
} 
