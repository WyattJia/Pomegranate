// use std::collections::BinaryHeap;
// use std::iter;

use skiplist::run::KVpair;
use crate::disk_run::DiskRun;

#[derive(Debug, Clone)]
pub struct KVIntPairT<K, V> {
    pub kvpair: KVpair<K, V>,
    pub i: isize,
}

pub struct DiskLevel<K, V> {
    tombstone:      isize,
    KVINTPAIRTMAX:  isize,
    KVPAIRMAX:      isize,

    pub level:      isize,
    pub page_size:  usize,
    pub run_size:   usize,
    pub run_nums:   usize,
    pub active_run: usize,
    pub merge_size: usize,
    pub bf_fp:      f64,
    pub runs:       Vec<DiskRun<K, V>>,
    
}
