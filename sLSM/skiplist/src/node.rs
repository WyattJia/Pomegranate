use std::iter;
use std::fmt;
use std::ops;
// use std::mem;


pub struct Node<K, V> {
    key: Option<K>,
    value: Option<V>,
    max_level: usize,
    forwards: Vec<Option<*mut Node<K, V>>>
}


impl <K, V> Node<K, V> {


    pub fn head(&mut max_level: usize) -> Self {
        Node {
            key: None,
            value: None,

            max_level,
            forwards: iter::repeat(None).take(max_level).collect(),
        }
    }

    pub fn new(key: K, value: V, &mut max_level: usize) -> Self {
        Node{
            key: Some(key),
            value: Some(value),
            max_level: max_level,
            forwards: iter::repeat(None).take(max_level + 1).collect()
        }
    }

    pub fn get(self) -> Option<(K, V)> {
        if self.key.is_some() {
            Some((self.key.unwrap(), self.value.unwrap()))
        } else {
            None
        }
    }

    pub fn is_header(&self) -> bool {
        self.prev.is_none
    }
}

impl <K, V> fmt::Display for Node<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
{
    pub fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let (&Some(ref k), &Some(ref v)) = (&self.key, &self.value) {
            write!(f, "({}, {})", k, v)
        } else {
            Ok(())
        }
    }

}

impl<K, V> ops::Drop for Node<K, V> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            // let node: *mut Node<K, V> = mem::transmute_copy((s)))
            println!("Dropping self...")
        }
    }
}