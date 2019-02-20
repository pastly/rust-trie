#[derive(Debug)]
struct Trie<T>
where T: std::fmt::Debug + std::clone::Clone
{
    val: T,
    children: Vec<Self>,
}

impl<T> Trie<T>
where T: std::fmt::Debug + std::clone::Clone
{
    fn new(val: T) -> Self {
        Self{val: val, children: vec![]}
    }
    fn insert(&mut self, val: T) {
        self.children.push(Self::new(val))
    }

    fn insert_recursive(&mut self, vals: &[T]) {
        if vals.len() < 1 {
            return;
        }
        let (first, remaining) = vals.split_first().unwrap();
        let mut new = Trie::new(first.clone());
        new.insert_recursive(remaining);
        self.children.push(new)
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;
    #[test]
    fn foo() {
        let mut t = Trie::new("A");
        t.insert_recursive(&["B", "C", "D"]);
        println!("{:?}", t);
        assert!(false);
    }
}
