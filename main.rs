use std::collections::HashMap;
use std::str::Chars;

fn main() {
    let trie: &mut Trie = &mut Trie::new();
    trie.insert(String::from("/usr/share"));
    trie.insert(String::from("/etc"));

    println!("{:?}", trie.prefix(String::from("/usr/share/dict/words")));
    println!("{:?}", trie.prefix(String::from("doc")));
    println!("{:?}", trie.prefix(String::from("/usr/sha")));
    println!("{:?}", trie.prefix(String::from("/usr/shat/stuff")));
    println!("{:?}", trie.prefix(String::from("/etc")));
    println!("{:?}", trie.prefix(String::from("/etc/")));

    println!("");

    println!("{:?}", trie.contains(String::from("/usr/share/dict/words")));
    println!("{:?}", trie.contains(String::from("doc")));
    println!("{:?}", trie.contains(String::from("/usr/share")));
    println!("{:?}", trie.contains(String::from("/usr/shat/stuff")));
    println!("{:?}", trie.contains(String::from("/etc")));
    println!("{:?}", trie.contains(String::from("/etc/")));
}

pub struct Trie {
    children: HashMap<char, Trie>,
    boundary: bool
}

impl Trie {
    fn new() -> Trie {
        Trie{children: HashMap::new(), boundary: false}
    }

    fn insert(&mut self, string: String) {
        insert(self, string.chars())
    }

    fn prefix(&self, string: String) -> bool {
        lookup(self, string.chars(), true)
    }

    fn contains(&self, string: String) -> bool {
        lookup(self, string.chars(), false)
    }
}

fn insert(trie: &mut Trie, mut iter: Chars) {
    if let Some(c) = iter.next() {
        match trie.children.contains_key(&c) {
            true => insert(trie.children.get_mut(&c).unwrap(), iter),
            false => {
                trie.children.insert(c, Trie::new());
                insert(trie.children.get_mut(&c).unwrap(), iter);
            }
        }
    } else {
        trie.boundary = true;
    }
}

fn lookup(trie: &Trie, mut iter: Chars, prefix: bool) -> bool {
    if let Some(c) = iter.next() {
        match trie.children.contains_key(&c) {
            true => return lookup(trie.children.get(&c).unwrap(), iter, prefix),
            false => return if prefix {trie.boundary} else {false}
        }
    }

    return trie.boundary;
}
