use std::iter;

use skiplist::run::KVpair;


macro_rules! leftchild {
  (x) => {
    2 * x + 1
  };
}

macro_rules! rightchild {
  (x) => {
    2 * x + 2
  };
}

macro_rules! parent{
    // `()` indicates that the macro takes no argument.
    (x) => {
        // The macro will expand into the contents of this block.
        (x - 1) / 2
    };
}


#[derive(Debug, Clone)]
pub struct KVIntPairT<K, V> {
  pub kvpair: KVpair<K, V>,
  pub i: isize,
}

// pub struct DiskLevel<K, V> {
// 
// 
// }



#[derive(Debug, Clone)]
pub struct StaticHeap<K, V> {
    pub size: isize,
    pub arr: Vec<KVIntPairT<K, V>>,
    pub max: Option<*mut KVIntPairT(<K, V>,isize), 
}


impl <K, V> StaticHeap<K, V> {

    pub fn new(sz: usize, mx: KVIntPairT<K, V>) -> Self {

        StaticHeap {
            size: 0,
            arr: iter::repeat(None).take(sz).collect(),
            arr: Vec::with_capacity(sz);
            // arr: iter::repeat(mx).take(sz).collect(),
            max: mx,
        }


    }
    fn push(&mut self, blob: KVpairT<K, V>) {
        let mut i: isize = self.size;
        i += 1;
        while (i > 0 && blob < arr[parent!(i)]){
            arr[i] = arr[parent!(i)];
  
            i = parent!(i);
        };
  
        arr[i] = blob;
  
    }

    fn heapify(&mut self, i: isize) {} 
    fn pop(&mut self) -> KVIntPairT<K, V> {

    }
}