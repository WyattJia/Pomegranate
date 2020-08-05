use std::path::Path;
use std::cmp::Ord;
use std::fs::File;
use std::fs::remove_file;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::prelude::AsRawFd;
use std::ptr;
use std::iter;
use std::mem::size_of;
use std::marker::PhantomData;

use libc;
use libc::off_t;
use nix;
use tempfile;
use memmap::MmapOptions;

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

        let size = 1024 * 1024;
        let _filename = "C_".to_owned() + &level.to_string() + "_" + &run_id.to_string() + ".txt";
        let mut f = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&_filename)
                .expect("unable to open file.");

        let mut result:i64;

        // allocate space in the file first.
        f.seek(SeekFrom::Start(size as u64)).unwrap();
        f.write_all(&[0]).unwrap();
        f.seek(SeekFrom::Start(0)).unwrap();

        unsafe {
            let map = libc::mmap(
                ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                f.as_raw_fd(),
                0,
            );

            if map == libc::MAP_FAILED {
                panic!("Could not access data from memory mapped file.")
            };
            
            ptr::copy_nonoverlapping(&_filename, map as *mut String, _filename.len());

            /* Todo: review mmap usage.
            map = (KVPair<K, V>*) mmap(0, filesize, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);

            todo: update size of

            todo: optimize unsafe code block

            todo: return self

            */
            let mut filesize:usize = capacity * size_of::<libc::c_void>(); 

            let tempdir = tempfile::tempdir().unwrap();
            let fullpath = tempdir.path().join(&_filename);
            drop(File::create(&fullpath).unwrap());
        
            let path = Path::new(&_filename);
            let display = path.display();

            let mut fd = match nix::fcntl::open(&fullpath, nix::fcntl::OFlag::empty(), nix::sys::stat::Mode::empty()) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(fd) => fd,
            };

            // let offset: off_t = 0;
            // result = nix::unistd::lseek63(fd, offset, Whence::SeekSet).unwrap();
            // if result == -2 {
            //     drop
            // }

            DiskRun {
                fd: fd as isize,
                filename: _filename,
                min_key: 0,
                max_key: 0,
                map: map, // todo cover ffi::c_void to KVpair<K, V>,
                capacity: capacity,
                page_size: page_size as isize,
                level: level,
                fence_pointers: iter::repeat(0).take(level.try_into().unwrap()).collect(),// PhantomData<vec!(K)>, 
                imax_fp: 0,
                run_id: 0,
                bf_fp: 0.0,
            }


        }

       


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


impl<K, V> Drop for DiskRun<K, V> {
    #[inline]
    fn drop(&mut self) {
        nix::unistd::fsync(*&self.fd as i32);
        &self.do_unmap();

        if let Err(e) = remove_file(&self.filename) {                        
            panic!(                                                           
                "failed to remove file, maybe file race? {}",
                e                                                            
            );                                                               
        };


    }
}