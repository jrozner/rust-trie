use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::str::Chars;

fn main() {
    let trie: &mut Trie = &mut Trie::new();
    trie.insert("/usr/share");
    trie.insert("/etc");
    trie.insert("/herp/derp");
    trie.insert("/usr/lib");
    trie.insert("/etc/openvpn");
    println!("Trie: {:?}", trie);

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

    println!("things:");
    for item in trie.iter() {
        println!("{}", item);
    }
}

#[derive(Debug)]
pub struct Trie {
    children: HashMap<char, Trie>,
    boundary: bool,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            children: HashMap::new(),
            boundary: false,
        }
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
        let mut keys: Vec<char> = self.children.keys().cloned().collect();
        keys.sort_by(|a, b| b.cmp(a));
        Iter {
            stack: vec![StatePair {
                            trie: self,
                            keys: keys,
                        }],
            current_word: Vec::new(),
        }
    }
}

pub struct Iter<'a> {
    stack: Vec<StatePair<'a>>,
    current_word: Vec<char>,
}

#[derive(Clone)]
struct StatePair<'a> {
    trie: &'a Trie,
    keys: Vec<char>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut direction = true;
        loop {
            if let None = self.stack.last() {
                return None;
            }

            let current = self.stack.last().unwrap().clone();
            let boundary = current.trie.boundary;

            let ret = if boundary == true {
                Some(self.current_word.iter().cloned().collect::<String>())
            } else {
                None
            };

            let next_key = {
                let current = self.stack.last_mut().unwrap();
                current.keys.pop()
            };

            let next_direction = match next_key {
                Some(key) => {
                    self.current_word.push(key);
                    let next = current.trie.children.get(&key).unwrap();

                    let mut next_keys: Vec<char> = next.children.keys().cloned().collect();
                    next_keys.sort_by(|a, b| b.cmp(a));

                    self.stack.push(StatePair {
                        trie: next,
                        keys: next_keys,
                    });
                    true
                }
                None => {
                    self.stack.pop();
                    self.current_word.pop();
                    false
                }
            };

            if let Some(ret) = ret {
                if direction {
                    return Some(ret);
                }
            }

            direction = next_direction;

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
            false => return if prefix { trie.boundary } else { false },
        }
    }

    return trie.boundary;
}
