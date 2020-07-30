use std::iter;
use std::collections::BinaryHeap;

use skiplist::run::KVpair;



#[derive(Debug, Clone)]
pub struct KVIntPairT<K, V> {
  pub kvpair: KVpair<K, V>,
  pub i: isize,
}

// pub struct DiskLevel<K, V> {
// 
// 
// }


