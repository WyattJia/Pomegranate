use std::cmp;
use std::iter;
use std::cmp::Ordering;
use std::fmt;
use std::mem;
use std::ops::Add;
use std::borrow::Borrow;

use crate::run::Run;
use crate::helpers::{GeoLevelGenerator, LevelGenerator};


pub trait LevelGenerator {
    fn total(&mut self) -> usize;

    fn random(&mut self) -> usize;
}

pub struct GeoLevelGenerator {
    total: usize,
    p: f64,
    rng: SmallRng,
}

impl GeoLevelGenerator {
    pub fn new(total: usize, p: f64) -> Self{
        if total == 0 {
            panic!("total can not be zero.");
        }
        if p <= 0.0 || p >= 1.0 {
            panic!("p value must in between in (0, 1)");
        }
        geo_level_gen {
            total,
            p,
            rng: SmallRng::from_rng(thread_rng()).unwrap(),
        }
    }
}

impl LevelGenerator for GeoLevelGenerator {
    fn total(&mut self) -> usize {
        *self.total
    }

    fn random(&mut self) -> usize {
        let mut h = 0;
        let mut x = &self.p;
        let f = 1.0 - self.rng.gen::<f64>();
        while x > f && &h + 1 < self.total {
            h += 1;
            x *= &self.p
        }
        h
    }
}

pub struct Node<K, V> {
    key: Option<K>,
    value: Option<V>,
    max_level: usize,
    forwards: Vec<Option<*mut Node<K, V>>>
}


impl <K, V> Node<K, V> {


    fn head(&mut max_level: usize) -> Self {
        Node {
            key: None,
            value: None,

            max_level,
            forwards: iter::repeat(None).take(max_level).collect(),
        }
    }

    fn new(key: K, value: V, &mut max_level: usize) -> Self {
        Node{
            key: Some(key),
            value: Some(value),
            max_level,
            forwards: iter::repeat(None).take(max_level + 1).collect()
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
        self.forwards.drop()
    }
}

pub struct KVpair<K, V> {
    pub key: Option<K>,
    pub value: Option<V>
}

// todo impl KVpair compare struct
impl <K, V> Eq for KVpair<K, V>
where
    K: cmp::Eq,
    V: cmp::Eq,
{
}




pub struct SkipList<K, V> {
    pub head: Option<*mut Node<K, V>>,
    pub tail: Option<*mut Node<K, V>>,
    pub current_max_level: isize,
    pub max_level: isize,
    pub min: Option<K>,
    pub max: Option<K>,
    pub min_key: K,
    pub max_key: K,
    pub n: i64,
    pub max_size: usize,

}

impl<K, V> Run for SkipList<K, V>
    where
        K: cmp::Ord,
{
    #[inline]
    fn new() -> Self {
        K: Cmp::Ord;
        min_key = 0;
        max_key = 0;
        let maxlevel = 12;
        SkipList {
            head: Node::new(min_key),
            tail: Node::new(max_key),
            current_max_level: 1,
            max_level: maxlevel,
            min: None,
            max: None,
            min_key: K,
            max_key: K,
            n: 0,
            max_size: None,
        }
    }


    fn get_min(&mut self) -> Option<K>{
        return &mut self.min
    }

    fn get_max(&mut self) -> Option<K> {
        return self.max
    }

    fn insert_key(&mut self, key: K, value: V){

        if key > self.max {
            self.max = key;
        } else if key < self.min {
            self.min = key;
        }
        let mut updated = iter::repeat(None).take(&mut self.max_level + 1).collect();
        let mut current_node = self.head;

        let mut level = &mut self.current_max_level;

        loop {
            level -= 1;
            if level > 0  {
                while (*current_node).forwards[level] < key {
                    current_node = (*current_node).forward[level];
                }
                updated[level] = current_node;
            }
        }

        let mut current_node = current_node.forwards[1];

        let levels = cmp::max(1, (&self.max_level as f64).log2.floor() as usize);
        let level_gen = GeoLevelGenerator::new(levels, 1.0 / 2.0);


        if *(current_node).key == key {
            *(current_node).value = value;
        } else {
            let insert_level = level_gen.total();
            if insert_level > &mut self.current_max_level && insert_level < &mut self.max_level - 1 {
                let mut lv = &mut self.current_max_level + 1;
                loop {
                    lv += 1;
                    if lv <= insert_level {
                        updated[lv] = &mut self.head
                    }
                    &mut self.current_max_level = insert_level;
                }
            }

            let current_node = Node::new(key, value);

            let mut level = 1;
            loop {
                &mut level += 1;
                if level <= &mut self.current_max_level {
                   current_node.forwards[&mut level] = updated[&mut level].forwards[&mut level];

                   updated[&mut level].forwards[&mut level] = current_node;

                }
            }
            &mut self.n += 1;
        }
    }

    fn delete_key(&mut self, key: K) {

        let mut updated = iter::repeat(None).take(&mut self.max_level + 1).collect();
        let mut current_node = &mut self.head;

        let mut level = &mut self.current_max_level;
        loop {
            level -= 1;
            if level >= 1{
                while current_node.forwards[level].key < key {
                    *(current_node) = *(current_node).forwards[level];
                }
                updated[level] = current_node;
            }
        }
        current_node = current_node.forwards[1];

        if current_node.key == key {
            let mut level = 1;
            loop {
                level += 1;
                if level <= &mut self.current_max_level{
                    if updated[&mut level].forwards[&mut level] != current_node{
                        break;
                    }
                    updated[&mut level].forwards[&mut level] = current_node.forwards[&mut level];
                }
                drop(current_node);
                while &mut self.current_max_level > 1 && &mut self.head.forward[&mut self.current_max_level] == None {
                    &mut self.current_max_level -= 1;
                }
            }
        }

        &mut self.n -= 1;
    }

    fn lookup(&mut self, key: K, mut found: bool) -> Option<V> {
        let current_node = self.head;
        let mut level = &mut self.current_max_level;
        loop {
            level -= 1;

            while current_node.forwards[level].key < key {
                current_node = current_node.forwards[level];
            }
        }

        current_node = current_node.forwards[1];

        return if current_node.key = key {
            found = true;
            current_node.value
        } else {
            V(None)
        }


    }
    fn num_elements(&mut self) -> usize {
        return &mut self.n
    }
    fn set_size(&mut self, size: usize) {
        *self.max_size = size;
    }
    fn get_all(&mut self) -> Vec<Option<Node<K, V>>>{
        let mut all: Vec<KVpair<K, V>> = Vec::new();

        let node = *self.head.forwards[1];
        
        while node != &mut self.tail {
            let kv = KVpair(node.key, node.value)

            *all.push(kv);
            
            node = node.forwards[1];
        }
        return all 

    }
    fn get_all_in_range(&mut self, key1: K, key2: K) -> Vec<Option<Node<K, V>>>{

        if key1 > self.max || key2 < self.min {
            let null_vec: Vec<KVpair<K, V>> = Vec::new();
            return null_vec;
        }

        let mut all: Vec<KVpair<K, V>> = Vec::new();

        let mut node = self.head.forwards[1];

        while node.key < key2 {
            node = node.forwards[1];
        }

        while node.key < key2 {
            let kv = KVpair<node.key, node.value>;
            all.push(kv);
            node = node.forwards[1];
        }

        return &mut all;


    }

    fn is_empty(&mut self) -> bool {
        return &mut self.head.forwards[1] == &mut self.tail
    }

    // todo modify elt_in to inline method.
    fn elt_in (&mut key: K) -> bool {
        return self::lookup(key)
    }
}

impl<K, V> Drop for SkipList<K, V>{
    fn drop(&mut self){
        println!("Dropping...");
    }
}


