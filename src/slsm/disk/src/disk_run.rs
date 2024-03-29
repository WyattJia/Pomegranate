use std::fmt;
use std::convert::TryInto;
use std::fs::remove_file;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::iter;
use std::mem;
use std::os::unix::prelude::AsRawFd;
use std::path::Path;
use std::ptr;
use core::ffi;

use libc;
use nix;
use tempfile;
use nix::unistd::close;

use skiplist::run::KVpair;

pub struct DiskRun<K, V> {
    pub fd: isize,
    pub page_size: isize,
    pub min_key: Option<KVpair<K, V>>,
    pub max_key: Option<KVpair<K, V>>,
    pub map: Vec<KVpair<K, V>>,

    capacity: usize,
    filename: String,
    level: isize,
    fence_pointers: Vec<Option<KVpair<K, V>>>,
    imax_fp: usize,
    run_id: usize,
    bf_fp: f64,
}


impl<K, V> DiskRun<K, V> {
    fn new(capacity: usize, page_size: usize, level: isize, run_id: isize, bf_fp: f32) -> Self {
        let size = 1024 * 1024;
        let _filename = "C_".to_owned() + &level.to_string() + "_" + &run_id.to_string() + ".txt";
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&_filename)
            .expect("unable to open file.");

        let mut result: i64;

        // allocate space in the file first.
        f.seek(SeekFrom::Start(size as u64)).unwrap();
        f.write_all(&[0]).unwrap();
        f.seek(SeekFrom::Start(0)).unwrap();

        unsafe {
            let c_void_map = libc::mmap(
                ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                f.as_raw_fd(),
                0,
            );

            if c_void_map == libc::MAP_FAILED {
                panic!("Could not access data from memory mapped file.")
            };


            let mut map = Vec::new();
            let mut kv: &mut KVpair<K, V> = &mut *(c_void_map as *mut KVpair<K, V>);

            map.push(kv);
            

            let tempdir = tempfile::tempdir().unwrap();
            let fullpath = tempdir.path().join(&_filename);
            drop(File::create(&fullpath).unwrap());

            let path = Path::new(&_filename);
            let display = path.display();

            let fd = match nix::fcntl::open(
                &fullpath,
                nix::fcntl::OFlag::empty(),
                nix::sys::stat::Mode::empty(),
            ) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(fd) => fd,
            };

            DiskRun {
                fd: fd as isize,
                filename: _filename,
                min_key: None,
                max_key: None,
                map,
                capacity,
                page_size: page_size as isize,
                level,
                fence_pointers: iter::repeat(None).take(level.try_into().unwrap()).collect(),
                imax_fp: 0,
                run_id: 0,
                bf_fp: 0.0,
            }
        }
    }

    #[inline]
    fn set_capacity(&mut self, new_capacity: usize) {
        self.capacity = new_capacity;
    }

    #[inline]
    fn get_capacity(self) -> usize {
        return self.capacity;
    }

    fn write_data(&mut self, run: &mut Vec<KVpair<K, V>>, offset: usize, len: usize) {
        // todo cover self.map to String
        unsafe {
            ptr::copy_nonoverlapping(
                &self.map as *const Vec<KVpair<K, V>>,
                run,
                &self.filename.len() + offset,
            );
        }
        self.capacity = len
    }

    fn construct_index(&mut self) {
        self.fence_pointers.reverse();

        let mut max_fp = 0;
        let mut j: usize = 0;
        while j < self.capacity {
            j += 1;
            // todo init bloom
            // self.bf.
            if j % (self.page_size as usize) == 0 {
                self.fence_pointers.push(self.map.get(j).unwrap());
                max_fp += 1;
            }
        }

        if max_fp >= 0 {
            
            self.fence_pointers.resize(max_fp as usize + 1, None);
        }

       self.min_key = Some(self.map[0]);
       self.max_key = Some(self.map[self.capacity - 1]);
    }

    fn binary_search(&mut self, offset: usize, n: usize, key: KVpair<K, V>, mut found: bool) -> usize {
        let min = offset;

        while n == 0 {
            found = true;
            return offset;
        }

        let mut min = offset;
        let mut max = offset + n - 1;
        let mut middle = (min + max) >> 1;
        while min <= max {
            if key > self.map[middle].unwrap() {
                min = middle + 1;
            } else if key == self.map[middle].unwrap() {
                found = true;
                return middle;
            } else {
                max = middle - 1;
                middle = (min + max) >> 1;
            }
        }

        return min;
    }

    fn get_flanking_fp(&mut self, key: KVpair<K, V>, start: &mut usize, end: &mut usize) {
        if self.imax_fp == 0 {
            start = &(0 as usize);
            end = &(self.capacity as usize);
            end = &(self.capacity as usize);
        } else if key < self.fence_pointers[1].unwrap(){
            // todo: impl Ord for K
            start = &(0 as usize);
            end = &(self.page_size as usize);
        } else if key >= self.fence_pointers[self.imax_fp].unwrap(){
            start = &(self.imax_fp * self.page_size as usize);
            end = &(self.capacity);
        } else {
            let mut min:usize = 0;
            let mut max:usize = self.imax_fp;
            while min < max {
                let middle: usize = (min + max) >> 1;

                if key > self.fence_pointers[middle]{
                    if key < self.fence_pointers[middle + 1].unwrap() {
                        start = &(middle * self.page_size as usize);
                        end = &((middle + 1) * self.page_size as usize);
                        return
                    }
                    min = middle + 1;
                }
                else if key < self.fence_pointers[middle] {
                    if key >= self.fence_pointers[middle - 1].unwrap(){
                        start = &((middle - 1) * self.page_size as usize);
                        end = &(middle * self.page_size as usize);
                        return
                    }
                    max = middle - 1;
                }

                else {
                    start = &(middle * self.page_size as usize);
                    end = start;
                    return; 
                }
            }
        }
    }

    fn get_index(&mut self, key: KVpair<K, V>, found: &bool) -> usize {
        let mut start: usize = 0;
        let mut end: usize = 0;
        self.get_flanking_fp(*key, &mut start, &mut end);
        let mut ret: usize = self.binary_search(start, end-start, key, *found);
        ret

    }

    fn lookup(&mut self, key: KVpair<K, V>, found: &bool) {
        let mut idx: usize = self.get_index(key, found);
        let ret: V = self.map[idx].unwrap();
        if Some(ret) == true{
            return ret 
        }

    }

    fn range(&mut self, key1: KVpair<K, V>, key2: KVpair<K, V>, i1: &usize, i2: &usize) {

        let mut i1: usize = 0;
        let mut i2: usize = 0;
        let mut found: bool;
        // todo impl PartialOrd for KVpair
        if key1 > self.max_key.unwrap() || key2 < self.min_key.unwrap() {
            return
        }
        if key1 >= self.min_key.unwrap() {
            found = true;
            i1 = self.get_index(key1, &found);
        }
        if key2 > (self.max_key.unwrap()) {
            i2 = self.capacity;
            return
        } else {
            found = false;
            i2 = self.get_index(key2, &found)
        }

    }


    fn do_map(&mut self) {


        let size = 1024 * 1024;
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.filename)
            .expect("unable to open file.");

        let filesize:usize = self.capacity * mem::size_of::<KVpair<K, V>>();

        let tempdir = tempfile::tempdir().unwrap();
        let fullpath = tempdir.path().join(self.filename);

        let path = Path::new(&self.filename);
        let display = path.display();

        let fd = match nix::fcntl::open(
                &fullpath,
                nix::fcntl::OFlag::empty(),
                nix::sys::stat::Mode::empty(),
            ) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(fd) => fd,
            };



         unsafe {
            let c_void_map = libc::mmap(
                ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                f.as_raw_fd(),
                0,
            );

            if c_void_map == libc::MAP_FAILED {
                panic!("Could not access data from memory mapped file.")
            };


            let mut map = Vec::new();
            let mut kv: &mut KVpair<K, V> = &mut *(c_void_map as *mut KVpair<K, V>);

            map.push(kv);

            let tempdir = tempfile::tempdir().unwrap();
            let fullpath = tempdir.path().join(self.filename);
            drop(File::create(&fullpath).unwrap());

            let path = Path::new(&self.filename);
            let display = path.display();

            let fd = match nix::fcntl::open(
                &fullpath,
                nix::fcntl::OFlag::empty(),
                nix::sys::stat::Mode::empty(),
            ) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(fd) => fd,
            };            
    };

}

    fn do_unmap(&mut self) {
        let filesize:usize = self.capacity * mem::size_of::<KVpair<K, V>>();

        unsafe {
            if libc::munmap(&(self.map) as *mut libc::c_void, 0) == -1 {
            panic!("Error unmmapping the file.");
            }

            close(self.fd as i32).unwrap(); 
            }
        self.fd = -5;
    }

    fn double_size(&mut self) {}
}

impl<K, V> Drop for DiskRun<K, V> {
    #[inline]
    fn drop(&mut self) {
        nix::unistd::fsync(*&self.fd as i32).expect("TODO: panic message");
        &self.do_unmap();

        if let Err(e) = remove_file(&self.filename) {
            panic!("failed to remove file, maybe file race? {}", e);
        };
    }
}




impl<K, V> fmt::Display for DiskRun<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // if let (&Some(ref K), &Some(ref V)) = (&self.min_key, &self.max_key) {
            write!(f, "({}, {})", &self.min_key, &self.max_key)
        // } else {
            // Ok(())
        // }
    }
}
