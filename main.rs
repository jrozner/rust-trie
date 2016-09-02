use std::collections::HashMap;
use std::collections::hash_map::Keys;
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

    for item in trie.iter() {
        println!("{}", item);
    }
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

    fn iter(&self) -> Iter {
        Iter{
            stack: vec![StatePair{trie: self, keys: self.children.keys()}],
            current_word: Vec::new(),
        }
    }
}

pub struct Iter {
    stack: Vec<StatePair>,
    current_word: Vec<char>,
}

struct StatePair {
    trie: Trie,
    keys: Keys,
}

impl Iterator for Iter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            if let Some(current) = self.stack.last() {
                if let Some(next_key) = current.keys.next() {
                    self.current_word.push(next_key);
                    let next = current.trie.children.get(next_key).unwrap();
                    self.stack.push(StatePair{trie: next, keys: next.children.keys()});
                    if current.boundary == true {
                        return Some(self.current_word.iter().cloned().collect::<String>());
                    }
                    continue;
                } else {
                    self.stack.pop();
                    self.current_word.pop();
                }
            } else {
                return None;
            }
        }
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
