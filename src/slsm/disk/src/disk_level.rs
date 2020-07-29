use std::iter;

use skiplist::run::KVpair;


macro_rules! leftchild {
  ($x:expr) => {
    2 * $x + 1
  };
}

macro_rules! rightchild {
  ($x:expr) => {
    2 * $x + 2
  };
}

macro_rules! parent{
    // `()` indicates that the macro takes no argument.
    ($x:expr) => {
        // The macro will expand into the contents of this block.
        ($x - 1) / 2
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
    pub max: Option<*mut KVIntPairT<K, V>>
}


impl <K, V> StaticHeap<K, V> {

    pub fn new(sz: usize, mx: KVIntPairT<K, V>) -> Self {

        StaticHeap {
            size: 0,
            arr: iter::repeat(None).take(sz).collect(),
            // arr: iter::repeat(mx).take(sz).collect(),
            max: Some(*mut mx),
        }


    }
    fn push(&mut self, blob: KVpair<K, V>) {
        let mut i: isize = self.size;
        i += 1;
        while i > 0 && blob <= &mut self.arr[parent!(i)]{
            // impl range arr, or use while range like sk range.
            &mut self.arr[i] = &mut self.arr[parent!(i)];
  
            i = parent!(i);
        };
  
        &mut self.arr[i] = blob;
  
    }

    fn heapify(&mut self, i: isize) {
      let mut smallest = (leftchild!(i) < &self.size && &self.arr[leftchild!(i)] < &self.arr[i)] ? leftchild!(i) : i;
      if (rightchild!(i) < &self.size && &self.arr[rightchild!(i)] < &self.arr[smallest]) {

      smallest = rightchild!(i);
      }
      if(smallest != i) {
        let tmp: KVIntPairT = &self.arr[i];
        &mut self.arr[i] = &mut self.arr[smallest];
        &mut self.arr[smallest] = tmp;

        heapify(smallest);
      } 

    } 
    fn pop(&mut self) -> KVIntPairT<K, V> {
        let ret: KVIntPairT = &mut self.arr[0];
        &mut self.arr[0] = &mut self.arr[--size];
        &self.heapify(0);
        ret
    }
}