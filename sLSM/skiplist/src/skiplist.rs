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
}

#[inline]
pub fn with_capacity(capacity: usize) -> Self {

}