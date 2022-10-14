use std::collections::{BTreeMap, VecDeque};
pub struct RadixTrie<T> {
    root: BTreeMap<u8, NodeTrie<T>>,
}
impl<T> Default for RadixTrie<T> {
    fn default() -> Self {
        Self::new()
    }
}
struct NodeTrie<T> {
    key: Vec<u8>,
    children: Option<BTreeMap<u8, NodeTrie<T>>>,
    cnt: u8,
    value: Option<T>,
}
impl<T> RadixTrie<T> {
    pub fn new() -> Self {
        RadixTrie { root: BTreeMap::new() }
    }
    pub fn insert(&mut self, key: &[u8], value: T) {
        if key.is_empty() {
            return;
        }
        match self.root.get_mut(&key[0]) {
            Some(next) => {
                next.insert(key, Some(value));
            }
            None => {
                self.root.insert(
                    key[0],
                    NodeTrie::new(key, Some(value), Some(BTreeMap::new()), 0),
                );
            }
        }
    }
    pub fn lookup(&self, key: &[u8]) -> Option<&T> {
        if !key.is_empty() {
            if let Some(next) = self.root.get(&key[0]) {
                return next.lookup(key);
            }
        }
        None
    }
    pub fn lookup_mut(&mut self, key: &[u8]) -> Option<&mut T> {
        if !key.is_empty() {
            if let Some(next) = self.root.get_mut(&key[0]) {
                return next.lookup_mut(key);
            }
        }
        None
    }
    pub fn remove(&mut self, key: &[u8]) {
        if key.is_empty() {
            return;
        }
        if let Some(next) = self.root.get_mut(&key[0]) {
            match next.remove(key) {
                Action::Delete(idx) => {
                    self.root.remove(&idx);
                }
                Action::Nothing => {}
                Action::Merge => {}
            }
        }
    }
    pub fn iter<'a>(&'a self, prefix: &'a [u8]) -> RadixTrieIter<'a, T> {
        let mut queue = VecDeque::new();
        if prefix.is_empty() {
            for node in self.root.values() {
                queue.push_back((Vec::new(), node));
            }
        } else {
            let mut inner_prefix = prefix;
            let mut node = self.root.get(&inner_prefix[0]);
            while let Some(next_node) = node {
                if inner_prefix.len() <= next_node.key.len() && next_node.key.starts_with(inner_prefix) {
                    queue.push_back((prefix[..prefix.len() - inner_prefix.len()].to_vec(), next_node));
                    break;
                } else if inner_prefix.starts_with(&next_node.key) {
                    inner_prefix = &inner_prefix[next_node.key.len()..];
                    node = next_node.children.as_ref().unwrap().get(&inner_prefix[0]);
                } else {
                    break;
                }
            }
        }
        RadixTrieIter { queue }
    }
}
impl<T> NodeTrie<T> {
    fn new(key: &[u8], value: Option<T>, children: Option<BTreeMap<u8, NodeTrie<T>>>, cnt: u8) -> Self {
        NodeTrie {
            key: Vec::from(key),
            value,
            children,
            cnt,
        }
    }
    fn insert(&mut self, key: &[u8], value: Option<T>) {
        let idx = common_prefix(key, &self.key);
        if idx < key.len() {
            if idx < self.key.len() {
                let node = NodeTrie::new(&self.key[idx..], self.value.take(), self.children.take(), self.cnt);
                self.cnt = 1;
                self.children = Some(BTreeMap::new());
                unsafe {
                    self.children.as_mut().unwrap_unchecked().insert(self.key[idx], node);
                }
                self.key.drain(idx..);
            }
            if let Some(next) = unsafe { self.children.as_mut().unwrap_unchecked().get_mut(&key[idx]) } {
                next.insert(&key[idx..], value);
            } else {
                self.cnt += 1;
                unsafe {
                    self.children.as_mut().unwrap_unchecked().insert(
                        key[idx],
                        NodeTrie::new(&key[idx..], value, Some(BTreeMap::new()), 0),
                    );
                }
            }
        } else if idx < self.key.len() {
            let node = NodeTrie::new(&self.key[idx..], self.value.take(), self.children.take(), self.cnt);
            self.cnt = 1;
            self.children = Some(BTreeMap::new());
            unsafe {
                self.children.as_mut().unwrap_unchecked().insert(self.key[idx], node);
            }
            self.value = value;
            self.key.drain(idx..);
        } else {
            self.value = value;
        }
    }
    fn lookup(&self, key: &[u8]) -> Option<&T> {
        if !key.is_empty() {
            let idx = common_prefix(&self.key, key);
            if key.len() == self.key.len() && key.len() == idx {
                return self.value.as_ref();
            }
            if idx == self.key.len() {
                if let Some(next) = unsafe { self.children.as_ref().unwrap_unchecked().get(&key[idx]) } {
                    return next.lookup(&key[idx..]);
                }
            }
        }
        None
    }
    fn lookup_mut(&mut self, key: &[u8]) -> Option<&mut T> {
        if !key.is_empty() {
            let idx = common_prefix(&self.key, key);
            if key.len() == self.key.len() && key.len() == idx {
                return self.value.as_mut();
            }
            if idx == self.key.len() {
                if let Some(next) = unsafe { self.children.as_mut().unwrap_unchecked().get_mut(&key[idx]) } {
                    return next.lookup_mut(&key[idx..]);
                }
            }
        }
        None
    }
    fn remove(&mut self, key: &[u8]) -> Action {
        if !key.is_empty() {
            let idx = common_prefix(key, &self.key);
            if key.len() == self.key.len() && key.len() == idx {
                self.value.take();
                if self.cnt == 0 {
                    return Action::Delete(self.key[0]);
                }
                if self.cnt == 1 {
                    self.merge();
                    return Action::Merge;
                }
                return Action::Nothing;
            }
            if idx == self.key.len() {
                if let Some(next) = unsafe { self.children.as_mut().unwrap_unchecked().get_mut(&key[idx]) } {
                    match next.remove(&key[idx..]) {
                        Action::Delete(idx) => {
                            unsafe {
                                self.children.as_mut().unwrap_unchecked().remove(&idx);
                            }
                            self.cnt -= 1;
                            if self.cnt == 0 && self.value.is_none() {
                                return Action::Delete(self.key[0]);
                            }
                            if self.cnt == 1 && self.value.is_none() {
                                self.merge();
                                return Action::Merge;
                            }
                            return Action::Nothing;
                        }
                        Action::Merge => {
                            if self.cnt == 1 && self.value.is_none() {
                                self.merge();
                                return Action::Merge;
                            }
                            return Action::Nothing;
                        }
                        Action::Nothing => {
                            return Action::Nothing;
                        }
                    }
                }
            }
            return Action::Nothing;
        }
        Action::Nothing
    }
    fn merge(&mut self) {
        let (&key, _) = unsafe {
            self.children
                .as_mut()
                .unwrap_unchecked()
                .iter()
                .next()
                .unwrap_unchecked()
        };
        let mut child = unsafe {
            self.children
                .as_mut()
                .unwrap_unchecked()
                .remove(&key)
                .unwrap_unchecked()
        };
        self.key.extend(&*child.key);
        self.value = child.value.take();
        self.cnt = child.cnt;
        self.children = child.children.take();
    }
}
pub struct RadixTrieIter<'a, T> {
    queue: VecDeque<(Vec<u8>, &'a NodeTrie<T>)>,
}
impl<'a, T> Iterator for RadixTrieIter<'a, T> {
    type Item = (Vec<u8>, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        while !self.queue.is_empty() {
            let (ref mut key, node) = self.queue.pop_front().unwrap();
            key.extend_from_slice(&node.key);
            for node in node.children.as_ref().unwrap().values() {
                self.queue.push_back((key.to_vec(), node));
            }
            if node.value.is_some() {
                return Some((key.clone(), node.value.as_ref().unwrap()));
            }
        }
        None
    }
}
fn common_prefix(a: &[u8], b: &[u8]) -> usize {
    let mut idx = 0;
    for (first, second) in a.iter().zip(b.iter()) {
        if first != second {
            break;
        }
        idx += 1;
    }
    idx
}
enum Action {
    Nothing,
    Delete(u8),
    Merge,
}
