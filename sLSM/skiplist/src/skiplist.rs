use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::mem;
use std::borrow::Borrow;

use crate::node::Node;
use crate::run::Run;

use crate::helpers::{GeoLevelGenerator, LevelGenerator};


pub struct SkipList<K, V> {
    pub header: Node<K, V>,
    pub min: K,
    pub max: K,
    pub current_level: isize,
    pub max_level: isize,
    pub p: f64,
    pub n: i64
}

impl<K, V> Run for SkipList<K, V>
where
    K: cmp::Ord,
{
    #[inline]
    fn new() -> Self {
        min_key = 0;
        max_key = 0;
        SkipList {
            header: Box::new(Node::head),
            min: (Node::new(min_key)),
            p: 0.5,
            n: 0,
            current_level: 0,
            max: (Node::new(max_key)),
            max_level: 0
        }
    }

    fn get_min(&mut self) -> Option<K>{

    }

    fn get_max(&mut self) -> Option<K> {}

    fn insert_key(&mut self, key: K, value: V){

        let mut node: *mut Node<K, V> = mem::transmute_copy(&self.header);
        let mut existing_node: Option<*mut Node<K, V>> = None;
        // let mut prev_nodes: Vec<*mut Node<K, V>> =
        if key > self.max {
            self.max = key;
        } else if key < self.min {
            self.min = key;
        }

        let mut updated = Vec::with_capacity(max_level + 1);
        let mut current_node = self.header;

        let mut i = &self.current_level;
        while i >= 0 {
            // 循环体
            i = i - 1;

        }

        let levels = cmp::max(1, (&self.max_level as f64).log2.floor() as usize);
        let level_gen = GeoLevelGenerator::new(levels, 1.0 / 2.0);

        let current_node = current_node.forward[1];
        if current_node.key == key {
            current_node.value = value;
        } else {
            let insert_level = level_gen.total();

            if insert_level > &self.current_level && insert_level < &self.max_level - 1 {
                let lv = &self.current_level + 1;
            }
        }

    }

    fn delete_key(&mut self, key: K) {

    }

    fn lookup(&mut self, key: K, found: bool) -> Option<V> {}
    fn num_elements(&mut self) -> usize {}
    fn set_size(&mut self, size: usize) {}
    fn get_all(&mut self) -> Vec<Option<Node<K, V>>>{}
    fn get_all_in_range(&mut self, key1: K, key2: K) -> Vec<Option<Node<K, V>>>{}
}

impl<K, V> Drop for SkipList<K, V>{
    fn drop(&mut self){
        println!("Dropping...");
    }
}


