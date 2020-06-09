use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::mem;
use std::borrow:Borrow

use crate::node::Node;

use crate::helpers::{GeoLevelGenerator, LevelGenerator};


pub struct SkipList<K, V> {
    header: Box<Node<K, V>>,
    length: usize,
    level_gen: GeoLevelGenerator,
}

impl <K, V> SkipList<K, V>
where
K: cmp::Ord,
{
    #[inline]
    pub fn new() -> Self {
        let lg = GeoLevelGenerator::new(16, 1.0 / 2.0);
        SkipList {
            header: Box::new(Node::header(lg.total())),
            length: 0,
            level_gen: lg,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let levels = cmp::max(1, (capacity as f64).log2().floor() as usize);
        let levelgen = GeoLevelGenerator::new(levels, 1.0 / 2.0);
        SkipList {
            header: Box::new(SkipList::head(level_gen.total())),
            length: 0,
            level_gen:levelgen,
        }

    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        unsafe {
            let mut node: *mut SkipList<K, V> = mem::transmute_copy(&self.header);
            let mut existing_node: Option<*mut SkipList<K, V>> = None;
            let mut prev_nodes: Vec<*mut SkipList<K, V>> =
                Vec::with_capacity(self.level_gen.total());

            let mut level = self.level_gen.total();
            while level > 0  {
                level -= 1;
                if let Some(existing_node) = existing_node {
                    while let Some(next) = (*node).links[level]{
                        if next == existing_node {
                            prev_nodes.push(node);
                            break;
                        } else {
                            node = next;
                            continue
                        }
                    }
                } else {
                    while let Some(next) = (*node).links[level] {
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
                                    break
                                }
                            }
                        }
                    }
                    if (*node).links[level].is_none() {
                        prev_nodes.push(node);
                        continue;
                    }
                }
            }
            if let Some(existing_node) = existing_node {
                mem::replace(&mut (*existing_node).value, Some(value))
            } else {
                let mut new_node =
                    Box::new(SkipList::new(key, value, self.level_gen.random()));
                let new_node_ptr: *mut SkipList<K, V> = mem::transmute_copy(&new_node);

                for (level, &prev_node) in prev_nodes.iter().rev().enumerate() {
                    if level <= new_node.level {
                        new_node.links[level] = (*prev_node).links[level];
                        (*prev_node).links[level] = Some(new_node_ptr);

                        if level == 0 {
                            new_node.prev = Some(prev_node);
                            if let Some(next) = new_node.links[level] {
                                (*next).prev = Some(new_node_ptr);
                            }
                            new_node.links_len[level] = 1;
                        } else {
                            let len = self.
                                link_length(prev_node, Some(new_node_ptr), level)
                                .unwrap();
                            new_node.links_len[level] = (*prev_node).links_len[level] - len + 1;
                            (*prev_node).links_len(level) = len;
                        }
                    } else {
                        (*prev_node).links_len[level] += 1;
                    }
                }

                let prev_node = (*new_node_ptr).prev.unwrap();
                let tmp = mem::replace(&mut (prev_node).next, Some(new_node));
                if let Some(ref mut node) = (*prev_node).next {
                    node.next = tmp;
                }
                self.length += 1;
                None
            }
        }
    }

    pub fn delete<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
        where 
        K: Borrow<Q>,
        Q: Ord,
        {
            if self.length == 0 {
                return None;

            }

            unsafe {
                let mut node: *mut Node<K, V> =  mem::transmute_copy(&self.header);
                let mut return_node: Option<*mut Node<K, V>> = None;
                let mut prev_nodes: Vec<*mut Node<K, V>> = 
                    Vec::with_capacity(self.level_gen.total());

                let mut level = self.level_gen.total();
                while level > 0 {
                    level -=  1;
                    if let Some(return_node) = return_node {
                        while let Some(next) = (*node).links[level] {
                            if next == return_node {
                                prev_nodes.push(node);
                                break;
                            } else {
                                node = next;
                            }
                        }
                    } else {
                        if (*node).links[level].is_none() {
                            prev_nodes.push(node);
                            continue;
                        }
                        while let Some(next) = (*node).links[level] {
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
                    for (level, &prev_node) in prev_nodes.iter().rev().enumerate() {
                        if (*prev_node).links[level] == Some(return_node) {
                            (*prev_node).links[level] = (*return_node).links[level];
                            (*prev_node).links_len[level] += (*return_node).links_len[level] - 1;
                        }else {
                            (*prev_node).links_len[level] -= 1;
                        }

                    }

                    if let Some(next_node) = (*return_node).links[0] {

                        (*next_node).prev = (*return_node).prev;
                    }
                    self.length -= 1;
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

                }
                else {
                    None
                }

            }

        }

    pub fn delete_key(&mut self, key: K) -> Self {

    }

    pub fn loopup(&mut self, key: K, found: bool) -> Option<V> {

    }

    pub fn get_all(&mut self) -> Vec<Option<K, V>> {

    }

    pub fn get_all_in_range(&mut self, key1: K, key2: K) -> Vec<Option<K, V>> {

    }

}

impl <K, V> SkipList<K, V> {
    #[inline]
    pub fn clear(&mut self) {
        unsafe {

        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        // self.length
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }


    #[inline]
    pub fn front(&self) -> Option<(&K, &V)> {

    }

    #[inline]
    pub fn elt_in(&self, key: K) -> bool {

    }

    #[inline]
    pub fn get_min(&self) -> Option<&K> {

    }

    #[inline]
    pub fn get_max(&self) -> Option<&K> {

    }

    #[inline]
    pub fn get_size_bytes(&self) -> usize {

    }

}
