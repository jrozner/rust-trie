use std::collections::HashMap;
use std::str::Chars;

fn main() {
    let trie: &mut Trie = &mut Trie::new();
    trie.insert("/usr/share");
    trie.insert("/etc");

    println!("{:?}", trie.prefix("/usr/share/dict/words"));
    println!("{:?}", trie.prefix("doc"));
    println!("{:?}", trie.prefix("/usr/sha"));
    println!("{:?}", trie.prefix("/usr/shat/stuff"));
    println!("{:?}", trie.prefix("/etc"));
    println!("{:?}", trie.prefix("/etc/"));

    println!("");

    println!("{:?}", trie.contains("/usr/share/dict/words"));
    println!("{:?}", trie.contains("doc"));
    println!("{:?}", trie.contains("/usr/share"));
    println!("{:?}", trie.contains("/usr/shat/stuff"));
    println!("{:?}", trie.contains("/etc"));
    println!("{:?}", trie.contains("/etc/"));
}

pub struct Trie {
    children: HashMap<char, Trie>,
    boundary: bool
}

impl Trie {
    fn new() -> Trie {
        Trie{children: HashMap::new(), boundary: false}
    }

    fn insert<S: Into<String>>(&mut self, string: S) {
        insert(self, string.into().chars())
    }

    fn prefix<S: Into<String>>(&self, string: S) -> bool {
        lookup(self, string.into().chars(), true)
    }

    fn contains<S: Into<String>>(&self, string: S) -> bool {
        lookup(self, string.into().chars(), false)
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
