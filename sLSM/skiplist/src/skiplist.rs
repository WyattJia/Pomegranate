use std::cmp;
use std::iter;
use std::mem;

use crate::run::Run;
use crate::run::KVpair;
use crate::helpers::LevelGenerator;
use crate::helpers::GeoLevelGenerator;
use crate::node::Node;


pub struct SkipList<K, V> {

    pub head: Option<Box<Node<K, V>>>,
    pub tail: Option<Box<Node<K, V>>>,
    pub current_max_level: isize,
    pub max_level: isize,
    pub min: Option<K>,
    pub max: Option<K>,
    pub min_key: Option<K>,
    pub max_key: Option<K>,
    pub n: i64,
    pub max_size: usize,
}

impl<K, V> Run<K, V> for SkipList<K, V>
where
K: cmp::Ord,
{
    #[inline]
    fn new() -> Self {
           let maxLevel = 12;
           let lg = GeoLevelGenerator::new(16, 1.0 / 2.0);
           SkipList {
               head: Some(Box::new(Node::head(lg.total()))),
               tail: Some(Box::new(Node::head(lg.total()))),
               current_max_level: 1,
               max_level: maxLevel,
               min: None,
               max: None,
               min_key: None,
               max_key: None,
               n: 0,
               max_size: 0,
           }
    }


    fn get_min(&mut self) -> Option<K>{
        return self.min
    }

    fn get_max(&mut self) -> Option<K> {
        return self.max
    }

    fn insert_key(&mut self, key: K, value: V){

        unsafe {
            let mut node: *mut Node<K, V> = mem::transmute_copy(&self.head);
            let mut current_node: Option<*mut Node<K, V>> = None;
            let mut prev_nodes: Vec<*mut Node<K, V>> = 
            Vec::with_capacity(level_gen.total());

            let mut lvl = level_gen.total();

            while lvl > 0 {
              lvl -= 1;

              if let Some(current_node) = current_node{
                  while 
              }
            }

        }
        // if Some(key) > self.max {
        //     self.max = Some(key);
        // } else if Some(key) < self.min {
        //     self.min = Some(key);
        // }
        // let mut updated = iter::repeat(None).take(self.max_level as usize + 1 ).collect();

        // let mut level = &mut self.current_max_level as isize;

        // loop {
        //     level -= 1;
        //     if level > 0  {
        //         while let Some(key) > (*current_node).forwards[level] {
        //             current_node = (*current_node).forward[level];
        //         }
        //         updated[level] = current_node;
        //     }
        // }

        // let mut current_node = *(current_node).forwards[1];

        // let levels = cmp::max(1, (&self.max_level as f64).log2().floor() as usize);
        // let level_gen = GeoLevelGenerator::new(levels, 1.0 / 2.0);


        // if *(current_node).key == key {
        //     *(current_node).value = value;
        // } else {
        //     let insert_level = level_gen.total();
        //     if insert_level > self.current_max_level as usize && insert_level < (self.max_level - 1) {
        //         let mut lv = self.current_max_level + 1;
        //         loop {
        //             lv += 1;
        //             if lv <= insert_level {
        //                 updated[lv] = &mut self.head
        //             }
        //             // mem trans insert_level to self.current_max_level
        //             let &mut self.current_max_level = insert_level;
        //         }
        //     }

        //     let current_node = Node::new(key, value);

        //     let mut level = 1;
        //     loop {
        //         &mut level += 1;
        //         if level <= &mut self.current_max_level {
        //             current_node.forwards[&mut level] = updated[&mut level].forwards[&mut level];

        //             updated[&mut level].forwards[&mut level] = current_node;

        //         }
        //     }
        //     &mut self.n += 1;
        // }
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
            None
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
            let key = node.key;
            let value = node.value;
            let kv = KVpair{key, value};

                (*all).push(kv);

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
            let key = node.key;
            let value = node.value;
            let kv = KVpair { key, value };
            (*all).push(kv);
            node = node.forwards[1];
        }

        return &mut all;


    }

    

    
}

impl<K, V> Drop for SkipList<K, V>{
    fn drop(&mut self){
        println!("Dropping...");
    }
}

impl<K, V> SkipList<K, V>
where
K: cmp::Ord,
{
    #[inline]
    fn is_empty(&mut self) -> bool {
        return &mut self.head.forwards[1] == &mut self.tail
    }

    #[inline]
    fn elt_in (&mut self, key: K) -> bool {
        return self.lookup(key)
    }
}
