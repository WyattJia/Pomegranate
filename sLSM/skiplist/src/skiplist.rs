use std::cmp;
use std::iter;
use std::mem;
use std::marker::PhantomData;
use std::ops::Bound;
use std::borrow::Borrow;
use std::cmp::Ordering;

use crate::run::Run;
use crate::run::KVpair;
use crate::run::Iter;
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
                    if (*node).forwards[lvl].is_none() {
                        prev_nodes.push(node);
                        continue;
                    }
                }
            }

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

                let prev_node = (*new_node_ptr).prev.unwrap();
                let tmp = mem::replace(&mut (*prev_node).next, Some(new_node));
                if let Some(ref mut node) = (*prev_node).next {
                    node.next = tmp;
                }
                self.n += 1;
            }
        }
    }

    fn delete_key<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where 
    K: Borrow<Q>,
    Q: Ord,
    {
        if self.n == 0 {
            return None;
        }

        unsafe {
            let mut node: *mut Node<K, V> = mem::transmute_copy(&self.head);
            let mut return_node: Option<*mut Node<K, V>> = None;
            let mut prev_nodes: Vec<*mut Node<K, V>> =
                Vec::with_capacity(self.level_gen.total());

            let mut lvl = self.level_gen.total();
            while lvl > 0 {
                lvl -= 1;

                if let Some(return_node) = return_node {
                    while let Some(next) = (*node).forwards[lvl] {
                        if next == return_node {
                            prev_nodes.push(node);
                            break;
                        } else {
                            node = next;
                        }
                    }
                } else {
                    if (*node).forwards[lvl].is_none() {
                        prev_nodes.push(node);
                        continue;
                    }
                    while let Some(next) = (*node).forwards[lvl] {
                        if let Some(ref next_key) = (*next).key {
                            match next_key.borrow().cmp(key) {
                                Ordering::Less => {
                                    node = next;
                                    continue;
                                }
                                Ordering::Equal => {
                                    return_node = Some(next);
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
                }
            }

            if let Some(return_node) = return_node {
                for (lvl, &prev_node) in prev_nodes.iter().rev().enumerate() {
                    if (*prev_node).forwards[lvl] == Some(return_node) {
                        (*prev_node).forwards[lvl] = (*return_node).forwards[lvl];
                        (*prev_node).links_len[lvl] += (*return_node).links_len[lvl] - 1;
                    } else {
                        (*prev_node).links_len[lvl] -= 1;
                    }
                }
                if let Some(next_node) = (*return_node).forwards[0] {
                    (*next_node).prev = (*return_node).prev;
                }
                self.n -= 1;
                Some(
                    mem::replace(
                        &mut (*(*return_node).prev.unwrap()).next,
                        mem::replace(&mut (*return_node).next, None),
                    )
                    .unwrap()
                    .into_inner()
                    .unwrap()
                    .1,
                )
            } else {
                None
            }
        }
    }

    fn find_key<Q: ?Sized>(&self, key: &Q) -> *const Node<K, V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        unsafe {
            let mut node: *const Node<K, V> = mem::transmute_copy(&self.head);

            let mut lvl = self.level_gen.total();
            while lvl > 0 {
                lvl -= 1;

                while let Some(next) = (*node).forwards[lvl] {
                    if let Some(ref next_key) = (*next).key {
                        match next_key.borrow().cmp(key) {
                            Ordering::Less => node = next,
                            Ordering::Equal => return next,
                            Ordering::Greater => break,
                        }
                    } else {
                        panic!("Encountered a value-less node.");
                    }
                }
            }
            node
        }
    }


    fn lookup<Q: ?Sized>(&self, key: &Q, mut found: bool) -> Option<&V> 
    where 
        K: Borrow<Q>,
        Q: Ord,
    {
        unsafe {
            let mut node: *const Node<K, V> = mem::transmute_copy(&self.head);
            let mut lvl = self.level_gen.total();
            while lvl > 0 {
                lvl -= 1;

                while let Some(next) = (*node).forwards[lvl] {
                    if let Some(ref next_key) = (*next).key {
                        match next_key.borrow().cmp(key) {
                            Ordering::Less => {
                                node = next;
                                continue;
                            }
                            Ordering::Equal => {
                                // &mut found = true;
                                return (*next).value.as_ref();
                            }
                            Ordering::Greater => break,
                        }
                    }
                }
            }
            None
        }
    }


    fn num_elements(&self) -> i64 {
        return self.n
    }
    fn set_size(&mut self, size: usize) {
        self.max_size = size;
    }
    fn get_all(&mut self) -> Vec<*mut Option<Node<K, V>>>{
        let mut all: Vec<*mut Node<K, V>> = 
                     Vec::with_capacity(self.level_gen.total());

        let mut node: *mut Node<K, V> = mem::transmute(&self.head);

        let mut lvl = self.level_gen.total();
        
        while lvl > 0 {
            lvl -= 1;

            while let Some(next) = (*node).forwards[lvl] {
                all.push(node);
                node = next;
            }
        }

        all
        // let node = &mut self.head.forwards[1];

        // while node != &mut self.tail {
        //     let key = node.key;
        //     let value = node.value;
        //     let kv = KVpair{key, value};

        //         (*all).push(kv);

        //     node = node.forwards[1];
        // }
        // return all 

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

    fn range<Q>(&self, min: Bound<&Q>, max: Bound<&Q>) -> Iter<K, V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        unsafe {
            let start = match min {
                Bound::Included(min) => {
                    let mut node = self.find_key(min);
                    if let Some(ref key) = (*node).key {
                        if key.borrow() == min {
                            node = (*node).prev.unwrap();
                        }
                    }
                    node
                }
                Bound::Excluded(min) => self.find_key(min),
                Bound::Unbounded => mem::transmute_copy(&self.head),
            };
            let end = match max {
                Bound::Included(max) => self.find_key(max),
                Bound::Excluded(max) => {
                    let mut node = self.find_key(max);
                    if let Some(ref key) = (*node).key {
                        if key.borrow() == max {
                            node = (*node).prev.unwrap();
                        }
                    }
                    node
                }
                Bound::Unbounded => self.get_last(),
            };
            match self.link_length(
                start as *mut Node<K, V>,
                Some(end as *mut Node<K, V>),
                cmp::min((*start).max_level, (*end).max_level) + 1,
            ) {
                Err(_) => Iter {
                    start,
                    end: start,
                    size: 0,
                    _lifetime_k: PhantomData,
                    _lifetime_v: PhantomData,
                },
                Ok(l) => Iter {
                    start,
                    end,
                    size: l,
                    _lifetime_k: PhantomData,
                    _lifetime_v: PhantomData,
                },
            }
        }
    }

    fn get_last(&self) -> *const Node<K, V> {
        unsafe {
            let mut node: *const Node<K, V> = mem::transmute_copy(&self.head);

            let mut lvl = self.level_gen.total();
            while lvl > 0 {
                lvl -= 1;

                while let Some(next) = (*node).forwards[lvl] {
                    node = next;
                }
            }
            node
        }
    }

    fn link_length(&self, start: *mut Node<K, V>, end: Option<*mut Node<K, V>>, lvl: usize, ) -> Result<usize, bool> {
        
        unsafe {
            let mut length = 0;
            let mut node = start;
            if lvl == 0 {
                while Some(node) != end {
                    length += 1;
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
            if let Some(end) = end {
                if node != end {
                    return Err(false);
                }
            }
            Ok(length)
        }
    }
    
    fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool 
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        unsafe {
            let mut node: *mut Node<K, V> = mem::transmute_copy(&self.head);

            let mut lvl = self.level_gen.total();
            while lvl > 0 {
                lvl -= 1;

                while let Some(next) = (*node).forwards[lvl] {
                    if let Some(ref next_key) = (*next).key {
                        match next_key.borrow().cmp(key) {
                            Ordering::Less => {
                                node = next;
                                continue;
                            }
                            Ordering::Equal => {
                                return true;
                            }
                            Ordering::Greater => {
                                break;
                            }
                        }

                    }
                }
            }
            false
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
        self.contains_key(&key)
    }
}
