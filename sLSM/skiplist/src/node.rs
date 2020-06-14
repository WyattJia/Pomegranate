use std::iter;
use std::fmt;


pub struct Node<K, V> {
    key: Option<K>,
    value: Option<V>,
    max_level: usize,
    forward: Option<*mut Node<K, V>>,
}


impl <K, V> Node<K, V> {

    fn new(key: K, value: V) -> Self {
        Node{
            key: Some(key),
            value: Some(value),
            max_level: 0,
            forward: None,
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let (&Some(ref k), &Some(ref v)) = (&self.key, &self.value) {
            write!(f, "({}, {})", k, v)
        } else {
            Ok(())
        }
    }

    fn drop(&mut self){
        println!("Dropping Node...")
    }
}