use std::convert::TryInto;
use std::fs::remove_file;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::iter;
use std::mem;
use std::collections::HashMap;
use std::os::unix::prelude::AsRawFd;
use std::path::Path;
use std::ptr;

use libc;
use nix;
use tempfile;

use skiplist::run::KVpair;

pub struct DiskRun<'a, K: 'a, V: 'a> {
    pub fd: isize,
    pub page_size: isize,
    pub min_key: isize,
    pub max_key: isize,
    pub map: Vec<KVpair<K, V>>,

    capacity: usize,
    filename: String,
    level: isize,
    fence_pointers: Vec<Option<*mut K>>,
    imax_fp: usize,
    run_id: usize,
    bf_fp: f64,
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

impl<'a, K, V> DiskRun<'a, K, V> {
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

            // todo: fix insert Some(key) err.
            map.push(*kv);
            
            // todo get this fucking transmute_copy away.

            /* Todo: review mmap usage.
            map = (KVPair<K, V>*) mmap(0, filesize, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);

            todo: update size of

            todo: optimize unsafe code block

            todo: return self

            */

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
                min_key: 0,
                max_key: 0,
                map: map,
                capacity: capacity,
                page_size: page_size as isize,
                level: level,
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

    fn write_data(&mut self, run: &mut KVpair<K, V>, offset: usize, len: usize) {
        // todo cover self.map to String
        unsafe {
            ptr::copy_nonoverlapping(
                &self.map as *const KVpair<K, V>,
                run,
                &self.filename.len() + offset,
            );
        }
        self.capacity = len
    }

    fn construct_index(&mut self) {
        /*
                // construct fence pointers and write BF
                //        _fencePointers.resize(0);
                _fencePointers.reserve(_capacity / pageSize);
                _iMaxFP = -1; // TODO IS THIS SAFE?
                for (int j = 0; j < _capacity; j++) {
                    bf.add((K*) &map[j].key, sizeof(K));
                    if (j % pageSize == 0){
                        _fencePointers.push_back(map[j].key);
                        _iMaxFP++;
                    }
                }
                if (_iMaxFP >= 0){
                    _fencePointers.resize(_iMaxFP + 1);
                }

                minKey = map[0].key;
                maxKey = map[_capacity - 1].key;
        */
        // self.fence_pointers.reverse(self.capacity / self.page_size as usize);
        self.fence_pointers.reverse();

        let mut max_fp = -1;
        let mut j: usize = 0;
        while j < self.capacity {
            j += 1;
            // todo init bloom
            // self.bf.
            if j % (self.page_size as usize) == 0 {
                self.fence_pointers.push(self.map.get(j).key);
                max_fp += 1;
            }
        }

        if max_fp >= 0 {
            self.fence_pointers.resize(max_fp as usize + 1, None);
        }

        // todo change map type to hash or vec , check cpp's type
        self.min_key = map[0].key;
        self.max_key = map[self.capacity - 1].key;
    }

    fn binary_search(&mut self, offset: usize, n: usize, key: &K, found: bool) -> usize {
        let min = offset;

        while n == 0 {
            found = true;
            return offset;
        }

        let mut min = offset;
        let mut max = offset + n - 1;
        let mut middle = (min + max) >> 1;
        while (min <= max) {
            if (key > map[middle].key) {
                min = middle + 1;
            } else if (key == map[middle].key) {
                found = true;
                return middle;
            } else {
                max = middle - 1;
                middle = (min + max) >> 1;
            }
        }

        return min;
    }

    fn get_flanking_fp(&mut self, start: &usize, end: &usize) {}

    fn get_index(&mut self, key: &K, found: &bool) -> usize {
        let i: usize = 1;
        i
    }

    fn lookup(&self, key: &K, found: &bool) -> Option<&V> {
        None
    }

    fn range(&self, key1: &K, key2: &K, i1: &usize, i2: &usize) {}

    fn print_elts(&self) {
        let mut j: usize = 0;
        while j < self.capacity {
            j += 1
        }
    }

    fn do_map(&mut self) {}

    fn do_unmap(&mut self) {}

    fn double_size(&mut self) {}
}

impl<K, V> Drop for DiskRun<K, V> {
    #[inline]
    fn drop(&mut self) {
        nix::unistd::fsync(*&self.fd as i32);
        &self.do_unmap();

        if let Err(e) = remove_file(&self.filename) {
            panic!("failed to remove file, maybe file race? {}", e);
        };
    }
}
