use std::cmp;

use crate::node::Node;

use crate::helpers::{GeoLevelGenerator, LevelGenerator};


pub struct SkipList<K, V> {
    header: Box<Node<K, V>>,
    length: usize,
    level_gen: GeoLevelGenerator,
}

impl <K, V> SkipList<K, V>
    where
        K: cmp::Ord,
{
    #[inline]
    pub fn new() -> Self {
        let lg = GeoLevelGenerator::new(16, 1.0 / 2.0);
        SkipList {
            header: Box::new(Node::header(lg.total())),
            length: 0,
            level_gen: lg,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {}

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        unsafe {}
    }

    pub fn delete_key(&mut self, key: K) -> Self {

    }

    pub fn loopup(&mut self, key: K, found: bool) -> Option<V> {

    }

    pub fn get_all(&mut self) -> Vec<Option<K, V>> {

    }

    pub fn get_all_in_range(&mut self, key1: K, key2: K) -> Vec<Option<K, V>> {

    }

}

impl <K, V> SkipList<K, V> {
    #[inline]
    pub fn clear(&mut self) {
        unsafe {

        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        // self.length
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }


    #[inline]
    pub fn front(&self) -> Option<(&K, &V)> {

    }

    #[inline]
    pub fn elt_in(&self, key: K) -> bool {

    }

    #[inline]
    pub fn get_min(&self) -> Option<&K> {

    }

    #[inline]
    pub fn get_max(&self) -> Option<&K> {

    }

    #[inline]
    pub fn get_size_bytes(&self) -> usize {

    }

}