use std::iter;
use std::fmt;


pub struct Node<K, V> {
    key: Option<K>,
    value: Option<V>,
    level: usize,
    next: Option<Box<Node<K, V>>>,
    prev: Option<*mut Node<k, V>>,
    links: Vev<Option<*mut Node<K, V>>>,
    links_len: Vec<usize>,
}


impl <K, V> Node<K, V> {
    fn header(max_level: usize) -> Self{
        Node{
            key: None,
            value: None,
            level: max_level - 1,
            next: None,
            prev: None,
            links: iter::repeat(None).take(max_level).collect(),
            links_len: iter::repeat(0).take(max_level).collect(),
        }
    }

    fn new(key: K, value: V, level: usize) -> Self {
        Node{
            key: Some(key),
            value: Some(value),
            level,
            next: None,
            prev: None,
            links: iter::repeat(None).take(level + 1).collect(),
            links_len: iter::repeat(0).take(level + 1).collect(),
        }
    }

    fn get(self) -> Option<(K, V)> {
        if self.key.is_some() {
            Some((self.key.unwrap(), self.value.unwrap()))
        } else {
            None
        }
    }

    fn is_header(&self) -> bool {
        self.prev.is_none
    }
}

impl <K, V> fmt::Display for Node<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
{
    // todo: need test this method.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let (&Some(ref k), &Some(ref v)) = (&self.key, &self.value) {
            write!(f, "({}, {})", k, v)
        } else {
            Ok(())
        }
    }
}