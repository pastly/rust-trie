use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[macro_use]
extern crate serde;
extern crate serde_cbor;

#[derive(Serialize, Deserialize, Debug)]
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

    fn keys<'a>(&'a self) -> TrieKeyIter<'a, K, V> {
        TrieKeyIter {
            iter: self.iter_impl(&[]),
        }
    }

    fn values<'a>(&'a self) -> TrieValueIter<'a, K, V> {
        TrieValueIter {
            iter: self.iter_impl(&[]),
        }
    }

    fn iter<'a>(&'a self) -> TrieIter<'a, K, V> {
        self.iter_impl(&[])
    }

    fn iter_impl<'a>(&'a self, keys_above: &[&'a K]) -> TrieIter<'a, K, V> {
        TrieIter {
            inner: self,
            child_iters: None,
            current: 0,
            did_self: false,
            keys_above: keys_above.to_vec(),
        }
    }
}

#[derive(Debug)]
struct TrieKeyIter<'a, K, V>
where
    K: Eq + Hash + Debug + Clone,
    V: Debug + Clone,
{
    iter: TrieIter<'a, K, V>,
}

impl<'a, K, V> Iterator for TrieKeyIter<'a, K, V>
where
    K: Eq + Hash + Debug + Clone,
    V: Debug + Clone,
{
    type Item = Vec<&'a K>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(n) => Some(n.0),
        }
    }
}

#[derive(Debug)]
struct TrieValueIter<'a, K, V>
where
    K: Eq + Hash + Debug + Clone,
    V: Debug + Clone,
{
    iter: TrieIter<'a, K, V>,
}

impl<'a, K, V> Iterator for TrieValueIter<'a, K, V>
where
    K: Eq + Hash + Debug + Clone,
    V: Debug + Clone,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(n) => Some(n.1),
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
    keys_above: Vec<&'a K>,
}

impl<'a, K, V> Iterator for TrieIter<'a, K, V>
where
    K: Eq + Hash + Debug + Clone,
    V: Debug + Clone,
{
    type Item = (Vec<&'a K>, V);

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
                let child_keys_iter = self.inner.children.keys();
                // Turn iter of Tries into iter of TrieIters
                let child_iters = child_keys_iter.map(|k| {
                    self.keys_above.push(k);
                    let i = self.inner.children[k].iter_impl(&self.keys_above);
                    self.keys_above.pop();
                    i
                });
                // Collect and store
                let v = child_iters.collect::<Vec<TrieIter<'a, K, V>>>();
                self.child_iters = Some(v);
            }
            // Now that we are done storing iters for our children, we should return our own value,
            // if any.
            if self.inner.val.is_some() {
                return Some((self.keys_above.clone(), self.inner.val.clone().unwrap()));
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
    use serde_cbor;

    #[test]
    fn single_key_value() {
        let mut t: Trie<i32, i32> = Trie::new(None);
        t.insert(&[1, 2, 3], 123);
        assert_eq!(t.fetch(&[1, 2, 3]), Some(123));
    }

    #[test]
    fn single_key_value_no_invalid() {
        let mut t: Trie<i32, i32> = Trie::new(None);
        t.insert(&[1, 2, 3], 123);
        assert_eq!(t.fetch(&[1, 2, 4]), None);
        assert_eq!(t.fetch(&[1, 2]), None);
        assert_eq!(t.fetch(&[2]), None);
    }

    #[test]
    fn single_key_value_some_root() {
        let mut t: Trie<i32, i32> = Trie::new(Some(0));
        t.insert(&[1, 2, 3], 123);
        assert_eq!(t.fetch(&[0, 1, 2, 3]), None);
        assert_eq!(t.fetch(&[1, 2, 3]), Some(123));
    }

    fn iter_test_data() -> Trie<i32, i32> {
        let mut t: Trie<i32, i32> = Trie::new(None);
        t.insert(&[1], 1);
        t.insert(&[1, 1], 11);
        t.insert(&[1, 2], 12);
        t.insert(&[1, 2, 1], 121);
        t.insert(&[1, 2, 2], 122);
        t.insert(&[1, 3, 1, 1, 1], 13111);
        t
    }

    #[test]
    fn iter_order() {
        let t = iter_test_data();
        let items = t.iter().collect::<Vec<_>>();
        let pos_1 = items.iter().position(|k| k == &(vec![&1i32], 1i32)).unwrap();
        let pos_11 = items.iter().position(|k| k == &(vec![&1i32, &1], 11i32)).unwrap();
        let pos_12 = items.iter().position(|k| k == &(vec![&1i32, &2], 12i32)).unwrap();
        let pos_121 = items.iter().position(|k| k == &(vec![&1i32, &2, &1], 121i32)).unwrap();
        let pos_122 = items.iter().position(|k| k == &(vec![&1i32, &2, &2], 122i32)).unwrap();
        let pos_13111 = items.iter().position(|k| k == &(vec![&1i32, &3, &1, &1, &1], 13111i32)).unwrap();
        assert!(pos_1 == 0);
        assert!(pos_11 > pos_1);
        assert!(pos_12 > pos_1);
        assert!(pos_121 > pos_12);
        assert!(pos_122 > pos_12);
        assert!(pos_13111 > pos_1);
    }

    #[test]
    fn iter_key_order() {
        let t = iter_test_data();
        let keys = t.keys().collect::<Vec<_>>();
        let pos_1 = keys.iter().position(|k| k == &vec![&1i32]).unwrap();
        let pos_11 = keys.iter().position(|k| k == &vec![&1i32, &1]).unwrap();
        let pos_12 = keys.iter().position(|k| k == &vec![&1i32, &2]).unwrap();
        let pos_121 = keys.iter().position(|k| k == &vec![&1i32, &2, &1]).unwrap();
        let pos_122 = keys.iter().position(|k| k == &vec![&1i32, &2, &2]).unwrap();
        let pos_13111 = keys.iter().position(|k| k == &vec![&1i32, &3, &1, &1, &1]).unwrap();
        assert!(pos_1 == 0);
        assert!(pos_11 > pos_1);
        assert!(pos_12 > pos_1);
        assert!(pos_121 > pos_12);
        assert!(pos_122 > pos_12);
        assert!(pos_13111 > pos_1);
    }

    #[test]
    fn iter_value_order() {
        let t = iter_test_data();
        let vals = t.values().collect::<Vec<_>>();
        let pos_1 = vals.iter().position(|v| v == &1i32).unwrap();
        let pos_11 = vals.iter().position(|v| v == &11i32).unwrap();
        let pos_12 = vals.iter().position(|v| v == &12i32).unwrap();
        let pos_121 = vals.iter().position(|v| v == &121i32).unwrap();
        let pos_122 = vals.iter().position(|v| v == &122i32).unwrap();
        let pos_13111 = vals.iter().position(|v| v == &13111i32).unwrap();
        assert!(pos_1 == 0);
        assert!(pos_11 > pos_1);
        assert!(pos_12 > pos_1);
        assert!(pos_121 > pos_12);
        assert!(pos_122 > pos_12);
        assert!(pos_13111 > pos_1);
    }

    #[test]
    /// assert that serde still can't tell the difference between None and ()
    fn serialize_none_vs_unit() {
        let mut t: Trie<&str, ()> = Trie::new(None);
        t.insert(&["yes_exist"], ());
        let encoded: Vec<u8> = serde_cbor::to_vec(&t).unwrap();
        let out: Trie<&str, ()> = serde_cbor::de::from_slice(&encoded).unwrap();
        assert_eq!(out.fetch(&["yes_exist"]), out.fetch(&["no_exist"]));
    }

    #[test]
    fn serialize_simple() {
        let mut t: Trie<i32, i32> = Trie::new(None);
        t.insert(&[1, 1], 11);
        t.insert(&[2, 1, 1], 211);
        let encoded: Vec<u8> = serde_cbor::to_vec(&t).unwrap();
        let out: Trie<i32, i32> = serde_cbor::de::from_slice(&encoded).unwrap();
        assert_eq!(out.fetch(&[1, 1]), Some(11));
        assert_eq!(out.fetch(&[2, 1, 1]), Some(211));
    }
}
