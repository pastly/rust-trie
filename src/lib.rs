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
}
