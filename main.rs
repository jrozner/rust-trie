use std::collections::HashMap;
use std::str::Chars;

fn main() {
    let trie: &mut Trie = &mut Trie::new();
    insert(trie, String::from("/usr/share"));
    insert(trie, String::from("/etc"));

    println!("{:?}", prefix(trie, String::from("/usr/share/dict/words")));
    println!("{:?}", prefix(trie, String::from("doc")));
    println!("{:?}", prefix(trie, String::from("/usr/sha")));
    println!("{:?}", prefix(trie, String::from("/etc")));
    println!("{:?}", prefix(trie, String::from("/etc/")));
}

pub struct Trie {
    children: HashMap<char, Trie>,
    boundary: bool
}

impl Trie {
    fn new() -> Trie {
        Trie{children: HashMap::new(), boundary: false}
    }
}

fn insert(trie: &mut Trie, string: String) {
    _insert(trie, string.chars())
}

fn prefix(trie: &Trie, string: String) -> bool {
    _prefix(trie, string.chars())
}

fn _insert(trie: &mut Trie, mut iter: Chars) {
    if let Some(c) = iter.next() {
        match trie.children.contains_key(&c) {
            true => _insert(trie.children.get_mut(&c).unwrap(), iter),
            false => {
                let t = Trie::new();
                trie.children.insert(c, t);
                _insert(trie.children.get_mut(&c).unwrap(), iter);
            }
        }
    } else {
        trie.boundary = true;
    }
}

fn _prefix(trie: &Trie, mut iter: Chars) -> bool {
    if let Some(c) = iter.next() {
        match trie.children.contains_key(&c) {
            true => return _prefix(trie.children.get(&c).unwrap(), iter),
            false => return trie.boundary
        }
    }

    return trie.boundary;
}
