use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::mem;
use std::borrow::Borrow;

use crate::node::Node;
use crate::run::Run;

use crate::helpers::{GeoLevelGenerator, LevelGenerator};


pub struct SkipList<K, V> {
    pub header: Box<Node<K, V>>,
    pub length: usize,
    pub level_gen: GeoLevelGenerator,
}

impl Run for SkipList<K, V>
{

    fn get_min(&mut self) -> Option<K>{

    }

    fn get_max(&mut self) -> Option<K> {}

    fn insert_key(&mut self, key: K, value: V){

    }

    fn delete_key(&mut self, key: K) {

    }

    fn lookup(&mut self, key: K, found: bool) -> Option<V> {}
    fn num_elements(&mut self) -> usize {}
    fn set_size(&mut self, size: usize) {}
    fn get_all(&mut self) -> Vec<Option<Node<K, V>>>{}
    fn get_all_in_range(&mut self, key1: K, key2: K) -> Vec<Option<Node<K, V>>>{}
}

