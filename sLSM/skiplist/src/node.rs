use std::fmt;
use std::iter;
use std::ops;
// use std::mem;

pub struct Node<K, V> {
    pub key: Option<K>,
    pub value: Option<V>,
    pub max_level: usize,
    // forwards: vector of links to next node at the respective level.
    // this vector must be of length `self.level + 1`.
    // links[0] stores a pointer to the same node as next.
    pub forwards: Vec<Option<*mut Node<K, V>>>,
    pub prev: Option<*mut Node<K, V>>,
    pub next: Option<Box<Node<K, V>>>,
    pub links_len: Vec<usize>,
}

impl<K, V> Node<K, V> {
    pub fn head(max_level: usize) -> Self {
        Node {
            key: None,
            value: None,
            next: None,
            prev: None,
            max_level,
            forwards: iter::repeat(None).take(max_level).collect(),
            links_len: iter::repeat(0).take(max_level).collect(),
        }
    }

    pub fn new(key: K, value: V, max_level: usize) -> Self {
        Node {
            key: Some(key),
            value: Some(value),
            max_level: max_level,
            next: None,
            prev: None,
            forwards: iter::repeat(None).take(max_level + 1).collect(),
            links_len: iter::repeat(0).take(max_level).collect(),
        }
    }

    pub fn is_header(&self) -> bool {
        self.prev.is_none()
    }

    pub fn into_inner(self) -> Option<(K, V)> {
        if self.key.is_some() {
            Some((self.key.unwrap(), self.value.unwrap()))
        } else {
            None
        }
    }
}

impl<K, V> fmt::Display for Node<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        // let node: *mut Node<K, V> = mem::transmute_copy((s)))
        println!("Dropping self...")
    }
}
