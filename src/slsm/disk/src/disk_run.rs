use std::fs::File;
use std::cmp::Ord;

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

// impl <K: Ord, V: Ord> for DiskRun<K, V>
// {
//     fn compareKVs(&self, a: &KVpair<K, V>, b: &KVpair<K, V>) -> isize {
//         /*
//          * Compare KV pairs.
//          */
//         return 10
//     }
// 
// 
// }

impl <K, V> DiskRun<K, V>
{
    fn new(capacity: usize, page_size: usize, level: isize, run_id: isize, bf_fp: f32) -> Self {
        let _filename = "C_".to_owned() + &level.to_string() + "_" + &run_id.to_string() + ".txt";

        let mut result:i64;

        let fd =     
        let mut file = File::open("foo.txt")?;

    }

    fn set_capacity(&mut self, new_capacity: usize) {
        &mut self.capacity = new_capacity;
    }

    fn get_capacity(self) -> usize {
        return self.capacity
    }

    fn write_data(&mut self, run: KVpair<K, V>, offset: usize, len: usize){
    }

    fn construct_index(&mut self){

    }

    fn binary_search(&mut self, offset: usize, n: usize, key: &K, found: &bool) -> usize {
        let mut min = offset;
        min
    }

    fn get_flanking_fp(&mut self, start: &usize, end: &usize){

    }

    fn get_index(&mut self, key: &K, found: &bool) -> usize {

        let i:usize = 1;
        i
    }

    fn lookup(&self, key: &K, found: &bool) -> V {

        V
    }

    fn range(&self, key1: &K, key2: &K, i1: &usize, i2: &usize){

    }

    fn print_elts(&self){

    }

    fn do_map(&mut self) {

    }

    fn do_unmap(&mut self) {

    }

    fn double_size(&mut self) {

    }
}