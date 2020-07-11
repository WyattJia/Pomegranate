use std::cmp;
use std::iter;
use std::mem;
use std::cmp::Ordering;

use crate::run::Run;
use crate::run::KVpair;
use crate::helpers::LevelGenerator;
use crate::helpers::GeoLevelGenerator;
use crate::node::Node;


pub struct SkipList<K, V> {

    pub head: Option<Box<Node<K, V>>>,
    pub tail: Option<Box<Node<K, V>>>,
    pub current_max_level: isize,
    // how high the node reaches, this should be euqal to be the vector length.
    pub max_level: isize,
    pub min: Option<K>,
    pub max: Option<K>,
    pub min_key: Option<K>,
    pub max_key: Option<K>,
    pub n: i64,
    pub max_size: usize,
    level_gen: GeoLevelGenerator,
}

impl<K, V> Run<K, V> for SkipList<K, V>
where
K: cmp::Ord,
{
    #[inline]
    fn new() -> Self {
           let maxLevel = 12;
           let level_gen = GeoLevelGenerator::new(16, 1.0 / 2.0);
           SkipList {
               head: Some(Box::new(Node::head(level_gen.total()))),
               tail: Some(Box::new(Node::head(level_gen.total()))),
               current_max_level: 1,
               max_level: maxLevel,
               min: None,
               max: None,
               min_key: None,
               max_key: None,
               n: 0,
               max_size: 0,
               level_gen,
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
            let mut lvl = self.level_gen.total();
            let mut node: *mut Node<K, V> = mem::transmute_copy(&self.head);
            let mut existing_node: Option<*mut Node<K, V>> = None;
            let mut prev_nodes: Vec<*mut Node<K, V>> =
                Vec::with_capacity(self.level_gen.total());

            
            while lvl > 0 {
                lvl -= 1;
                // 递减遍历 level 变量
                // 当 existing_node 变量不为空
                if let Some(existing_node) = existing_node {

                    while let Some(next) = (*node).forwards[lvl] {
                        if next == existing_node {
                            prev_nodes.push(node);
                            break;
                        } else {
                            node = next;
                            continue;
                        }
                    }
                } else {
                    while let Some(next) = (*node).forwards[lvl] {
                        if let Some(ref next_key) = (*next).key {
                            match next_key.cmp(&key) {
                                Ordering::Less => {
                                    node = next;
                                    continue;
                                }
                                Ordering::Equal => {
                                    existing_node = Some(next);
                                    prev_nodes.push(node);
                                    break;
                                }
                                Ordering::Greater => {
                                    prev_nodes.push(node);
                                    break;
                                }
                            }
                        }
                    }
                    // We have not yet found the node, and there are no further
                    // nodes at this level, so the return node (if present) is
                    // between `node` and tail.
                    if (*node).forwards[lvl].is_none() {
                        prev_nodes.push(node);
                        continue;
                    }
                }
            }

            // At this point, `existing_node` contains a reference to the node
            // with the same key if it was found, otherwise it is None.
            if let Some(existing_node) = existing_node {
                mem::replace(&mut (*existing_node).value, Some(value));
            } else {
                let mut new_node =
                    Box::new(Node::new(key, value, self.level_gen.random()));
                let new_node_ptr: *mut Node<K, V> = mem::transmute_copy(&new_node);

                for (lvl, &prev_node) in prev_nodes.iter().rev().enumerate() {
                    if lvl <= new_node.max_level {
                        new_node.forwards[lvl] = (*prev_node).forwards[lvl];
                        (*prev_node).forwards[lvl] = Some(new_node_ptr);

                        if lvl == 0 {
                            new_node.prev = Some(prev_node);
                            if let Some(next) = new_node.forwards[lvl] {
                                (*next).prev = Some(new_node_ptr);
                            }
                            new_node.links_len[lvl] = 1;
                        } else {
                            let length = self
                                .link_length(prev_node, Some(new_node_ptr), lvl)
                                .unwrap();
                            new_node.links_len[lvl] = (*prev_node).links_len[lvl] - length + 1;
                            (*prev_node).links_len[lvl] = length;
                        }
                    } else {
                        (*prev_node).links_len[lvl] += 1;
                    }
                }

                // Move the ownerships around, inserting the new node.
                let prev_node = (*new_node_ptr).prev.unwrap();
                let tmp = mem::replace(&mut (*prev_node).next, Some(new_node));
                if let Some(ref mut node) = (*prev_node).next {
                    node.next = tmp;
                }
                self.n += 1;
            }
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

    fn link_length(&self, start: *mut Node<K, V>, end: Option<*mut Node<K, V>>, lvl: usize, ) -> Result<usize, bool> {
        
        unsafe {
            let mut length = 0;
            let mut node = start;
            if lvl == 0 {
                while Some(node) != end {
                    length += 1;
                    // Since the head node is not a node proper, the link
                    // between it and the next node (on level 0) is actual 0
                    // hence the offset here.
                    if (*node).is_header() {
                        length -= 1;
                    }
                    match (*node).forwards[lvl] {
                        Some(ptr) => node = ptr,
                        None => break,
                    }
                }
            } else {
                while Some(node) != end {
                    length += (*node).links_len[lvl - 1];
                    match (*node).forwards[lvl - 1] {
                        Some(ptr) => node = ptr,
                        None => break,
                    }
                }
            }
            // Check that we actually have calculated the length to the end node
            // we want.
            if let Some(end) = end {
                if node != end {
                    return Err(false);
                }
            }
            Ok(length)
        }
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
