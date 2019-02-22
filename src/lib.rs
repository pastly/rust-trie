use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
struct Trie<K, V>
where
    K: Eq + Hash + Debug + Clone,
    V: Debug + Clone,
{
    val: Option<V>,
    children: HashMap<K, Trie<K, V>>,
}

impl<K, V> Trie<K, V>
where
    K: Eq + Hash + Debug + Clone,
    V: Debug + Clone,
{
    fn new(val: Option<V>) -> Self {
        Self {
            val,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, keys: &[K], val: V) {
        if keys.is_empty() {
            assert!(
                self.val.is_none(),
                "Tried to insert into Trie where value already exists"
            );
            self.val = Some(val);
            return;
        }
        assert!(!keys.is_empty());
        let (first, remaining) = keys.split_first().unwrap();
        if self.children.contains_key(first) {
            self.children.get_mut(first).unwrap().insert(remaining, val);
        } else {
            let mut new = Trie::new(None);
            new.insert(remaining, val);
            self.children.insert(first.clone(), new);
        }
    }

    fn fetch(&self, keys: &[K]) -> Option<V> {
        if keys.is_empty() {
            return self.val.clone();
        }
        assert!(!keys.is_empty());
        let (first, remaining) = keys.split_first().unwrap();
        if self.children.contains_key(first) {
            self.children[first].fetch(remaining)
        } else {
            None
        }
    }

    fn iter<'a>(&'a self) -> TrieIter<'a, K, V> {
        TrieIter {
            inner: self,
            child_iters: None,
            current: 0,
            did_self: false,
        }
    }
}

#[derive(Debug)]
struct TrieIter<'a, K, V>
where
    K: Eq + Hash + Debug + Clone,
    V: Debug + Clone,
{
    inner: &'a Trie<K, V>,
    child_iters: Option<Vec<Self>>,
    current: usize,
    did_self: bool,
}

impl<'a, K, V> Iterator for TrieIter<'a, K, V>
where
    K: Eq + Hash + Debug + Clone,
    V: Debug + Clone,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        // If we haven't done ourself yet, then we need to build up a vector of iters from our
        // children, and also return our own value, if we have one.
        if !self.did_self {
            // Make sure we only come in here once
            self.did_self = true;
            // If we have children, then we need to build a vector of their iters
            if !self.inner.children.is_empty() {
                assert!(self.child_iters.is_none());
                assert!(self.current == 0);
                // Get all children Tries
                let children_iter = self.inner.children.values();
                // Turn iter of Tries into iter of TrieIters
                let child_iters = children_iter.map(|c| c.iter());
                // Collect and store
                let v = child_iters.collect::<Vec<TrieIter<'a, K, V>>>();
                self.child_iters = Some(v);
            }
            // Now that we are done storing iters for our children, we should return our own value,
            // if any.
            if self.inner.val.is_some() {
                return Some(self.inner.val.clone().unwrap());
            }
        }
        assert!(self.did_self);
        // We must have done ourself, so if we didn't collect some child iters, we must not have
        // any children and are done
        if self.child_iters.is_none() {
            return None;
        }
        // Otherwise, we have children and need to return values from them.
        loop {
            // Get the next value from the current child
            let n = self.child_iters.as_mut().unwrap()[self.current].next();
            // And return it if it exists
            if n.is_some() {
                return n;
            }
            // If the current child has no more values, then go to the next child
            if n.is_none() {
                self.current += 1;
                // If moving to the next child pushes us past our last child, then we are
                // completely done
                if self.current >= self.child_iters.as_ref().unwrap().len() {
                    return None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;
    #[test]
    fn foo() {
        let mut t: Trie<u8, bool> = Trie::new(None);
        t.insert(&[1, 2, 3], true);
        t.insert(&[1, 2, 4], true);
        println!("{:?}", t);
        let v1 = t.fetch(&[1]);
        println!("{:?}", v1);
        let v2 = t.fetch(&[1, 2, 3]);
        println!("{:?}", v2);
        assert!(false);
    }

    #[test]
    fn bar() {
        let mut t: Trie<u16, String> = Trie::new(None);
        t.insert(&[1], "A".to_string());
        t.insert(&[2], "B".to_string());
        t.insert(&[3], "C".to_string());
        t.insert(&[2, 21, 211, 2111], "D".to_string());
        //println!("{:?}", t);
        for item in t.iter() {
            println!("ITEM: {:?}", item);
        }
        assert!(false);
    }

    #[test]
    fn baz() {
        let mut t: Trie<&str, &str> = Trie::new(None);
        t.insert(&["a", "b"], "ab");
        t.insert(&["a", "c"], "ac");
        t.insert(&["a", "c", "d"], "acd");
        t.insert(&["a"], "a");
        t.insert(&["1", "2", "3"], "123");
        for item in t.iter() {
            println!("ITEM: {:?}", item);
        }
        assert!(false);
    }
}
