use std::fmt::Debug;
use std::clone::Clone;

#[derive(Debug)]
struct Trie<T>
where T: Debug + Clone + Trieable<T>
{
    val: T,
    children: Vec<Self>,
}

impl<T> Trie<T>
where T: Debug + Clone + Trieable<T>
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

trait Trieable<T>
{
    fn trie_split(&self) -> Vec<T>;
    fn trie_join(vals: &[T]) -> T;
}

impl Trieable<String> for String
{
    fn trie_split(&self) -> Vec<Self> {
        self.chars().map(|c| c.to_string()).collect()
    }

    fn trie_join(vals: &[String]) -> String {
        vals.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;
    use super::Trieable;
    #[test]
    fn foo() {
        let mut t = Trie::new("A".to_string());
        t.insert_recursive(&["B".to_string(), "C".to_string(), "D".to_string()]);
        println!("{:?}", t);
        assert!(true);
    }

    #[test]
    fn bar() {
        let s = String::from("ABC");
        println!("{:?}", s.trie_split());
        let t = String::trie_join(&["D".to_string(), "E".to_string(), "F".to_string()]);
        println!("{:?}", t);
        assert!(false);
    }
}
