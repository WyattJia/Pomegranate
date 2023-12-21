use std::fmt;

pub struct Node<K, V> {
    pub(crate) key:   K,
    pub(crate) value: V,
    // forwards: vector of links to next node at the respective level.
    // this vector must be of length `self.level + 1`.
    // links[0] stores a pointer to the same node as next.
    pub(crate) forwards: Vec<Option<Box<Node<K, V>>>>,
    prev: Option<Box<Node<K, V>>>,
    next: Option<Box<Node<K, V>>>,
    links_len: usize,
}

impl<K, V> Node<K, V> {
    pub(crate) fn new(key: K, value: V, level: usize) -> Self {
        Node {
            key,
            value,
            prev: None,
            next: None,
            forwards: vec![None; level],
            links_len: level,
        }
    }

    pub fn head(level: usize) -> Self {
        Node {
            key:   Default::default(),
            value: Default::default(),
            next: None,
            prev: None,
            forwards: vec![None; level],
            links_len: level,
        }
    }

    pub fn is_header(&self) -> bool {
        //Judge based on some specific conditions, such as checking whether key and value are default values
        // The specific implementation here depends on the types of K and V
        // For example, if both K and V implement PartialEq + Default, it can be judged like this:
        self.key == Default::default() && self.value == Default::default()

    }

    pub fn into_inner(self) -> (K, V) {
        (self.key, self.value)
    }
}

impl<K, V> fmt::Display for Node<K, V>
    where
        K: fmt::Display,
        V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node(Key: {}, Value: {})", self.key, self.value)
    }
}