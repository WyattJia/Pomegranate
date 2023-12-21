use std::borrow::Borrow;
use std::cmp;
use std::cmp::Ordering;
use std::default;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem;
use std::ops::Drop;
use std::ops::Bound;
use std::ops::Bound::Included;

use crate::helpers::GeoLevelGenerator;
use crate::helpers::LevelGenerator;
use crate::node::Node;
use crate::run::Iter;
use crate::run::KVpair;
use crate::run::Run;

const MAX_LEVEL: usize = 16;

struct SkipList<K, V> {
    head: Box<Node<K, V>>,
    current_level: usize,
    max_level: usize,
    length: usize,
    // min: Option<K>,
    // max: Option<K>,
    // min_key: Option<K>,
    // ax_key: Option<K>,
    // n: i64,
    // ax_size: usize,
    level_gen: GeoLevelGenerator,
}

// impl<K, V> Run<K, V> for SkipList<K, V>
//     where
//         K: cmp::Ord,
// {
//     #[inline]
//     fn new() -> Self {
//         let level_gen = GeoLevelGenerator::new(16, 1.0 / 2.0);
//         SkipList {
//             head: Some(Box::new(Node::head(level_gen.total()))),
//             tail: Some(Box::new(Node::head(level_gen.total()))),
//             current_max_level: 1,
//             max_level: 12,
//             min: None,
//             max: None,
//             min_key: None,
//             max_key: None,
//             n: 0,
//             max_size: 0,
//             level_gen,
//         }
//     }
//
//     fn get_min(&mut self) -> Option<K> {
//         // self.head
//         unsafe {
//             let header: Node<K, V> = mem::transmute_copy(&self.head);
//             return header.key
//         }
//     }
//
//     fn get_max(&mut self) -> Option<K> {
//         unsafe {
//             let max: Node<K, V> = mem::transmute_copy(&self.get_last());
//             return max.key
//         }
//     }
//
//     fn insert_key(&mut self, key: K, value: V) {
//         unsafe {
//             let mut lvl = self.level_gen.total();
//             let mut node: *mut Node<K, V> = mem::transmute_copy(&self.head);
//             let mut existing_node: Option<*mut Node<K, V>> = None;
//             let mut prev_nodes: Vec<*mut Node<K, V>> = Vec::with_capacity(self.level_gen.total());
//
//             while lvl > 0 {
//                 lvl -= 1;
//                 if let Some(existing_node) = existing_node {
//                     while let Some(next) = (*node).forwards[lvl] {
//                         if next == existing_node {
//                             prev_nodes.push(node);
//                             break;
//                         } else {
//                             node = next;
//                             continue;
//                         }
//                     }
//                 } else {
//                     while let Some(next) = (*node).forwards[lvl] {
//                         if let Some(ref next_key) = (*next).key {
//                             match next_key.cmp(&key) {
//                                 Ordering::Less => {
//                                     node = next;
//                                     continue;
//                                 }
//                                 Ordering::Equal => {
//                                     existing_node = Some(next);
//                                     prev_nodes.push(node);
//                                     break;
//                                 }
//                                 Ordering::Greater => {
//                                     prev_nodes.push(node);
//                                     break;
//                                 }
//                             }
//                         }
//                     }
//                     if (*node).forwards[lvl].is_none() {
//                         prev_nodes.push(node);
//                         continue;
//                     }
//                 }
//             }
//
//             if let Some(existing_node) = existing_node {
//                 let _ = mem::replace(&mut (*existing_node).value, Some(value));
//             } else {
//                 let mut new_node = Box::new(Node::new(key, value, self.level_gen.random()));
//                 let new_node_ptr: *mut Node<K, V> = mem::transmute_copy(&new_node);
//
//                 for (lvl, &prev_node) in prev_nodes.iter().rev().enumerate() {
//                     if lvl <= new_node.max_level {
//                         new_node.forwards[lvl] = (*prev_node).forwards[lvl];
//                         (*prev_node).forwards[lvl] = Some(new_node_ptr);
//
//                         if lvl == 0 {
//                             new_node.prev = Some(prev_node);
//                             if let Some(next) = new_node.forwards[lvl] {
//                                 (*next).prev = Some(new_node_ptr);
//                             }
//                             new_node.links_len[lvl] = 1;
//                         } else {
//                             let length = self
//                                 .link_length(prev_node, Some(new_node_ptr), lvl)
//                                 .unwrap();
//                             new_node.links_len[lvl] = (*prev_node).links_len[lvl] - length + 1;
//                             (*prev_node).links_len[lvl] = length;
//                         }
//                     } else {
//                         (*prev_node).links_len[lvl] += 1;
//                     }
//                 }
//
//                 let prev_node = (*new_node_ptr).prev.unwrap();
//                 let tmp = mem::replace(&mut (*prev_node).next, Some(new_node));
//                 if let Some(ref mut node) = (*prev_node).next {
//                     node.next = tmp;
//                 }
//                 self.n += 1;
//             }
//         }
//     }
//
//     fn delete_key<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
//         where
//             K: Borrow<Q>,
//             Q: Ord,
//     {
//         if self.n == 0 {
//             return None;
//         }
//
//         unsafe {
//             let mut node: *mut Node<K, V> = mem::transmute_copy(&self.head);
//             let mut return_node: Option<*mut Node<K, V>> = None;
//             let mut prev_nodes: Vec<*mut Node<K, V>> = Vec::with_capacity(self.level_gen.total());
//
//             let mut lvl = self.level_gen.total();
//             while lvl > 0 {
//                 lvl -= 1;
//
//                 if let Some(return_node) = return_node {
//                     while let Some(next) = (*node).forwards[lvl] {
//                         if next == return_node {
//                             prev_nodes.push(node);
//                             break;
//                         } else {
//                             node = next;
//                         }
//                     }
//                 } else {
//                     if (*node).forwards[lvl].is_none() {
//                         prev_nodes.push(node);
//                         continue;
//                     }
//                     while let Some(next) = (*node).forwards[lvl] {
//                         if let Some(ref next_key) = (*next).key {
//                             match next_key.borrow().cmp(key) {
//                                 Ordering::Less => {
//                                     node = next;
//                                     continue;
//                                 }
//                                 Ordering::Equal => {
//                                     return_node = Some(next);
//                                     prev_nodes.push(node);
//                                     break;
//                                 }
//                                 Ordering::Greater => {
//                                     prev_nodes.push(node);
//                                     break;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//
//             if let Some(return_node) = return_node {
//                 for (lvl, &prev_node) in prev_nodes.iter().rev().enumerate() {
//                     if (*prev_node).forwards[lvl] == Some(return_node) {
//                         (*prev_node).forwards[lvl] = (*return_node).forwards[lvl];
//                         (*prev_node).links_len[lvl] += (*return_node).links_len[lvl] - 1;
//                     } else {
//                         (*prev_node).links_len[lvl] -= 1;
//                     }
//                 }
//                 if let Some(next_node) = (*return_node).forwards[0] {
//                     (*next_node).prev = (*return_node).prev;
//                 }
//                 self.n -= 1;
//                 Some(
//                     mem::replace(
//                         &mut (*(*return_node).prev.unwrap()).next,
//                         mem::replace(&mut (*return_node).next, None),
//                     )
//                         .unwrap()
//                         .into_inner()
//                         .unwrap()
//                         .1,
//                 )
//             } else {
//                 None
//             }
//         }
//     }
//
//     fn lookup<Q: ?Sized>(&self, key: &Q) -> Option<&V>
//         where
//             K: Borrow<Q>,
//             Q: Ord,
//     {
//         unsafe {
//             let mut node: *const Node<K, V> = mem::transmute_copy(&self.head);
//             let mut lvl = self.level_gen.total();
//             while lvl > 0 {
//                 lvl -= 1;
//
//                 while let Some(next) = (*node).forwards[lvl] {
//                     if let Some(ref next_key) = (*next).key {
//                         match next_key.borrow().cmp(key) {
//                             Ordering::Less => {
//                                 node = next;
//                                 continue;
//                             }
//                             Ordering::Equal => {
//                                 // &mut found = true;
//                                 return (*next).value.as_ref();
//                             }
//                             Ordering::Greater => break,
//                         }
//                     }
//                 }
//             }
//             None
//         }
//     }
//
//     fn find_key<Q: ?Sized>(&self, key: &Q) -> *const Node<K, V>
//         where
//             K: Borrow<Q>,
//             Q: Ord,
//     {
//         unsafe {
//             let mut node: *const Node<K, V> = mem::transmute_copy(&self.head);
//
//             let mut lvl = self.level_gen.total();
//             while lvl > 0 {
//                 lvl -= 1;
//
//                 while let Some(next) = (*node).forwards[lvl] {
//                     if let Some(ref next_key) = (*next).key {
//                         match next_key.borrow().cmp(key) {
//                             Ordering::Less => node = next,
//                             Ordering::Equal => return next,
//                             Ordering::Greater => break,
//                         }
//                     } else {
//                         panic!("Encountered a value-less node.");
//                     }
//                 }
//             }
//             node
//         }
//     }
//
//     fn num_elements(&self) -> i64 {
//         return self.n;
//     }
//     fn set_size(&mut self, size: usize) {
//         self.max_size = size;
//     }
//     fn get_last(&self) -> *const Node<K, V> {
//         unsafe {
//             let mut node: *const Node<K, V> = mem::transmute_copy(&self.head);
//
//             let mut lvl = self.level_gen.total();
//             while lvl > 0 {
//                 lvl -= 1;
//
//                 while let Some(next) = (*node).forwards[lvl] {
//                     node = next;
//                 }
//             }
//             node
//         }
//     }
//
//     fn get_all(&mut self) -> Vec<KVpair<K, V>> {
//         unsafe {
//             let mut all: Vec<KVpair<K, V>> = Vec::with_capacity(self.level_gen.total());
//
//             let mut node: *mut Node<K, V> = mem::transmute(&self.head);
//
//             let mut lvl = self.level_gen.total();
//
//             while lvl > 0 {
//                 lvl -= 1;
//
//                 while let Some(next) = (*node).forwards[lvl] {
//                     let node_key   = mem::transmute_copy(&(*node).key);
//                     let node_value = mem::transmute_copy(&(*node).value);
//                     let kv = KVpair {
//                         key:   node_key,
//                         value: node_value,
//                     };
//                     all.push(kv);
//                     node = next;
//                 }
//             }
//             all
//         }
//     }
//
//     fn get_all_in_range(&mut self, key1: K, key2: K) -> Vec<KVpair<K, V>> {
//         unsafe {
//             let mut all: Vec<KVpair<K, V>> = Vec::with_capacity(self.level_gen.total());
//
//             for (k, v) in self.range(Included(&key1), Included(&key2)) {
//                 let node_key   = mem::transmute_copy(&k);
//                 let node_value = mem::transmute_copy(&v);
//                 let kv = KVpair {
//                     key:   node_key,
//                     value: node_value,
//                 };
//                 all.push(kv);
//             }
//             all
//         }
//     }
//
//     fn range<Q>(&self, min: Bound<&Q>, max: Bound<&Q>) -> Iter<K, V>
//         where
//             K: Borrow<Q>,
//             Q: Ord,
//     {
//         unsafe {
//             let start = match min {
//                 Bound::Included(min) => {
//                     let mut node = self.find_key(min);
//                     if let Some(ref key) = (*node).key {
//                         if key.borrow() == min {
//                             node = (*node).prev.unwrap();
//                         }
//                     }
//                     node
//                 }
//                 Bound::Excluded(min) => self.find_key(min),
//                 Bound::Unbounded => mem::transmute_copy(&self.head),
//             };
//             let end = match max {
//                 Bound::Included(max) => self.find_key(max),
//                 Bound::Excluded(max) => {
//                     let mut node = self.find_key(max);
//                     if let Some(ref key) = (*node).key {
//                         if key.borrow() == max {
//                             node = (*node).prev.unwrap();
//                         }
//                     }
//                     node
//                 }
//                 Bound::Unbounded => self.get_last(),
//             };
//             match self.link_length(
//                 start as *mut Node<K, V>,
//                 Some(end as *mut Node<K, V>),
//                 cmp::min((*start).max_level, (*end).max_level) + 1,
//             ) {
//                 Err(_) => Iter {
//                     start,
//                     end: start,
//                     size: 0,
//                     _lifetime_k: PhantomData,
//                     _lifetime_v: PhantomData,
//                 },
//                 Ok(l) => Iter {
//                     start,
//                     end,
//                     size: l,
//                     _lifetime_k: PhantomData,
//                     _lifetime_v: PhantomData,
//                 },
//             }
//         }
//     }
//
//     fn link_length(
//         &self,
//         start: *mut Node<K, V>,
//         end: Option<*mut Node<K, V>>,
//         lvl: usize,
//     ) -> Result<usize, bool> {
//         unsafe {
//             let mut length = 0;
//             let mut node = start;
//             if lvl == 0 {
//                 while Some(node) != end {
//                     length += 1;
//                     if (*node).is_header() {
//                         length -= 1;
//                     }
//                     match (*node).forwards[lvl] {
//                         Some(ptr) => node = ptr,
//                         None => break,
//                     }
//                 }
//             } else {
//                 while Some(node) != end {
//                     length += (*node).links_len[lvl - 1];
//                     match (*node).forwards[lvl - 1] {
//                         Some(ptr) => node = ptr,
//                         None => break,
//                     }
//                 }
//             }
//             if let Some(end) = end {
//                 if node != end {
//                     return Err(false);
//                 }
//             }
//             Ok(length)
//         }
//     }
//
//     fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
//         where
//             K: Borrow<Q>,
//             Q: Ord,
//     {
//         unsafe {
//             let mut node: *mut Node<K, V> = mem::transmute_copy(&self.head);
//
//             let mut lvl = self.level_gen.total();
//             while lvl > 0 {
//                 lvl -= 1;
//
//                 while let Some(next) = (*node).forwards[lvl] {
//                     if let Some(ref next_key) = (*next).key {
//                         match next_key.borrow().cmp(key) {
//                             Ordering::Less => {
//                                 node = next;
//                                 continue;
//                             }
//                             Ordering::Equal => {
//                                 return true;
//                             }
//                             Ordering::Greater => {
//                                 break;
//                             }
//                         }
//                     }
//                 }
//             }
//             false
//         }
//     }
// }

impl<K:Ord + Copy + Debug, V: Debug> SkipList<K, V>
{
    fn new(max_level: usize) -> Self{
        SkipList {
            head: Box::new(Node::head(max_level)),
            current_level: 1,
            max_level,
            length: 0,
            level_gen: GeoLevelGenerator::new(max_level, 1.0 / 2.0),
        }
    }

    fn search(&self, key: K) -> Option<&V> {
        let mut node = &self.head;
        for i in (0..self.current_level).rev() {
            while let Some(ref next) = node.forwards[i] {
                if next.key < key {
                    node = next;
                } else {
                    break;
                }
            }
        }
        if let Some(ref next) = node.forwards[0] {
            if next.key == key {
                return Some(&next.value);
            }
        }
        None
    }

    fn delete(&mut self, key: K) -> bool {
        let mut update = Vec::with_capacity(self.max_level);
        let mut x = &mut self.head as *mut _;

        // 查找每一层中要删除节点的位置
        for i in (0..self.current_level).rev() {
            unsafe {
                while let Some(ref mut next) = (*x).forwards[i] {
                    if next.key < key {
                        x = next as *mut _;
                    } else {
                        break;
                    }
                }
            }
            update.push(x);
        }

        let mut deleted = false;
        unsafe {
            // 检查并删除节点
            if let Some(ref mut next) = (*x).forwards[0] {
                if next.key == key {
                    for i in 0..self.current_level {
                        if let Some(ref mut next_node) = (*update[i]).forwards[i] {
                            if next_node.key != key {
                                break;
                            }
                            (*update[i]).forwards[i] = next_node.forwards[i].take();
                        }
                    }
                    deleted = true;
                    self.length -= 1;
                }
            }
        }

        // 降低当前层级（如果需要）
        while self.current_level > 1 && self.head.forwards[self.current_level - 1].is_none() {
            self.current_level -= 1;
        }

        deleted
    }

    fn insert(&mut self, key: K, value: V) { // -> Option<V> {
        // 创建一个临时向量，用于存储每层中小于新节点键值的最后一个节点的引用
        let mut update = vec![None; self.max_level];
        let mut x = &mut self.head as *mut _;

        // 从最高层开始遍历，寻找插入点
        for i in (0..self.current_level).rev() {
            unsafe {
                // 向前移动，直到找到小于新节点键值的节点
                while let Some(ref mut next) = (*x).forwards[i] {
                    if next.key < key {
                        x = next as *mut _;
                    } else {
                        break;
                    }
                }
            }
            update[i] = Some(x); // 在此层级中记住位置
        }

        // 随机决定新节点的层级
        let new_level = self.random_level();

        // 如果新节点层级超过当前层级，需要更新 update 数组和 current_level
        if new_level > self.current_level {
            for i in self.current_level..new_level {
                update[i] = Some(&mut *self.head);
            }
            self.current_level = new_level;
        }

        // 创建新节点
        let mut new_node = Box::new(Node::new(key, value, new_level));

        // 更新指针
        for i in 0..new_level {
            // 此处不再使用裸指针进行操作
            new_node.forwards[i] = if let Some(ptr) = update[i] {
                unsafe { (*ptr).forwards[i].take() }
            } else {
                None
            };
            if let Some(ptr) = update[i] {
                unsafe { (*ptr).forwards[i] = Some(new_node.clone()); }
            }
        }

        // 增加跳表的长度
        self.length += 1;
    }

    fn random_level(&self) -> usize {
        let mut level = 1;
        while rand::random::<bool>() && level < self.max_level {
            level += 1;
        }
        level
    }
}


    // #[inline]
    // fn is_empty(&self) -> bool {
    //     self.n == 0
    // }

    // #[inline]
    // fn elt_in(&mut self, key: K) -> bool {
    //     self.contains_key(&key)
    // }

impl<K, V> Drop for SkipList<K, V> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let node: *mut Node<K, V> = mem::transmute_copy(&self.head);

            while let Some(ref mut next) = (*node).next {
                let _ = mem::replace(&mut (*node).next, mem::replace(&mut next.next, None));
            }
        }
    }
}

impl<K: Ord, V> default::Default for SkipList<K, V> {
    fn default() -> SkipList<K, V> {
        SkipList::new()
    }
}
