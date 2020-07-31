use skiplist::run::KVpair;


pub struct DiskRun<K, V> {
    pub fd:         isize,
    pub page_size:  isize,
    pub min_key:    isize,
    pub max_key:    isize,
    pub map:        KVpair<K, V>,

    capacity:       usize,
    filename:       String,
    level:          isize,
    fence_pointers: Vec<K>,
    imax_fp:        usize,
    run_id:         usize,
    bf_fp:          f64,
}