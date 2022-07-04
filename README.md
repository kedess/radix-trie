## radix-trie
<hr/>

Radix Trie (compressed prefix tree) is a data structure which is a memory-optimized implementation of the prefix tree.

The algorithmic complexity for all operations (insert, delete, search) is <b>O(n * log(k))</b>, where <b>n</b> is the string length, <b>k</b> is the size of the alphabet.

### Usage:

Insert, delete, search

```rust
use radix_trie::RadixTrie;

fn main() {
    let mut radix_trie = RadixTrie::<u32>::new();

    radix_trie.insert("dog".as_bytes(), 1);
    radix_trie.insert("elephant".as_bytes(), 2);
    radix_trie.insert("rabbit".as_bytes(), 3);

    assert_eq!(radix_trie.lookup("dog".as_bytes()), Some(&1));
    assert_eq!(radix_trie.lookup("elephant".as_bytes()), Some(&2));

    radix_trie.remove("dog".as_bytes());
    radix_trie.remove("elephant".as_bytes());

    assert_eq!(radix_trie.lookup("dog".as_bytes()), None);
    assert_eq!(radix_trie.lookup("elephant".as_bytes()), None);
}
```

Traversal

```rust
use radix_trie::RadixTrie;

fn main() {
    let mut radix_trie = RadixTrie::<()>::new();
    radix_trie.insert("kare".as_bytes(), ());
    radix_trie.insert("kanojo".as_bytes(), ());
    radix_trie.insert("karetachi".as_bytes(), ());
    radix_trie.insert("sakura".as_bytes(), ());
    radix_trie.insert("korosu".as_bytes(), ());

    let keys: Vec<String> = radix_trie.iter("kar".as_bytes()).map(|(key,_)| String::from_utf8(key).unwrap()).collect();
    assert_eq!(keys, vec!["kare".to_string(), "karetachi".to_string()]);
}
```

### Cargo.toml
```bash
[dependencies]
radix-trie = {git = "https://github.com/kedess/radix-trie.git", branch="master"} // github
radix-trie = {git = "https://gitflic.ru/project/kedess/radix-trie.git", branch="master"} // gitflic
```