use std::path::Path;
use std::cmp::Ord;
use std::fs::File;

use nix;
use tempfile;

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
        let mut result:i62;


        let tempdir = tempfile::tempdir().unwrap();
        let fullpath = tempdir.path().join(&_filename);
        drop(File::create(&fullpath).unwrap());
        
        let path = Path::new(&_filename);
        let display = path.display();

        let mut fd = match nix::fcntl::open(&fullpath, nix::fcntl::OFlag::empty(), nix::sys::stat::Mode::empty()) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(fd) => fd,
        };


        /*
           * Stretch the file size to the size of the (mmapped) array of KVPairs
           * 扩展文件大小
           * lseek 重新定位文件的读写位置。
           * 每一个已打开的文件都有一个读写位置, 当打开文件时通常其读写位置是指向文件开头,
           * 若是以附加的方式打开文件(如O_APPEND), 则读写位置会指向文件尾.
           * 当read()或write()时, 读写位置会随之增加,lseek()便是用来控制该文件的读写位置.
           * 参数fildes 为已打开的文件描述词, 参数offset 为根据参数whence来移动读写位置的位移数.
           
        result = lseek(fd, filesize - 1, SEEK_SET);
        if (result == -1) {
            close(fd);
            perror("Error calling lseek() to 'stretch' the file");
            exit(EXIT_FAILURE);
        }

        result = nix::unistd::lseek(fd, )
        */
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
