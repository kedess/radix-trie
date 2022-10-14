use radix_trie::RadixTrie;
use std::time::{Duration, Instant};
const KX: u32 = 123456789;
const KY: u32 = 362436069;
const KZ: u32 = 521288629;
const KW: u32 = 88675123;
pub struct Rand {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}
impl Rand {
    pub fn new(seed: u32) -> Rand {
        Rand {
            x: KX ^ seed,
            y: KY ^ seed,
            z: KZ,
            w: KW,
        }
    }
    pub fn rand(&mut self) -> u32 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        return self.w;
    }
    pub fn shuffle<T>(&mut self, a: &mut [T]) {
        if a.len() == 0 {
            return;
        }
        let mut i = a.len() - 1;
        while i > 0 {
            let j = (self.rand() as usize) % (i + 1);
            a.swap(i, j);
            i -= 1;
        }
    }
    pub fn rand_range(&mut self, a: i32, b: i32) -> i32 {
        let m = (b - a + 1) as u32;
        return a + (self.rand() % m) as i32;
    }
    pub fn rand_float(&mut self) -> f64 {
        (self.rand() as f64) / (<u32>::max_value() as f64)
    }
}
#[test]
fn test_radix_trie_1() {
    RadixTrie::<i32>::new();
    let mut radix_trie = RadixTrie::<u32>::new();
    radix_trie.insert("abab".as_bytes(), 1);
    radix_trie.insert("abab".as_bytes(), 11);
    radix_trie.insert("abc".as_bytes(), 2);
    radix_trie.insert("abccc".as_bytes(), 3);
    radix_trie.insert("ddvbn".as_bytes(), 4);
    radix_trie.insert("gg".as_bytes(), 5);
    radix_trie.insert("g".as_bytes(), 6);
    assert_eq!(radix_trie.lookup("abab".as_bytes()), Some(&11));
    assert_eq!(radix_trie.lookup("ababa".as_bytes()), None);
    assert_eq!(radix_trie.lookup("abccc".as_bytes()), Some(&3));
    assert_eq!(radix_trie.lookup("abcc".as_bytes()), None);
    assert_eq!(radix_trie.lookup("abc".as_bytes()), Some(&2));
    assert_eq!(radix_trie.lookup("ddvbn".as_bytes()), Some(&4));
    assert_eq!(radix_trie.lookup("ddvbnn".as_bytes()), None);
    assert_eq!(radix_trie.lookup("gg".as_bytes()), Some(&5));
    assert_eq!(radix_trie.lookup("g".as_bytes()), Some(&6));
    assert_eq!(radix_trie.lookup("ggg".as_bytes()), None);
    let mut radix_trie = RadixTrie::<u32>::new();
    radix_trie.insert("dear".as_bytes(), 1);
    radix_trie.insert("sweetie".as_bytes(), 2);
    radix_trie.insert("angel".as_bytes(), 3);
    radix_trie.insert("dream".as_bytes(), 4);
    radix_trie.insert("baby".as_bytes(), 5);
    assert_eq!(radix_trie.lookup("dear".as_bytes()), Some(&1));
    assert_eq!(radix_trie.lookup("sweetie".as_bytes()), Some(&2));
    assert_eq!(radix_trie.lookup("angel".as_bytes()), Some(&3));
    assert_eq!(radix_trie.lookup("dream".as_bytes()), Some(&4));
    assert_eq!(radix_trie.lookup("baby".as_bytes()), Some(&5));
    assert_eq!(radix_trie.lookup("avens".as_bytes()), None);
    let mut radix_trie = RadixTrie::<u32>::new();
    radix_trie.insert("abd".as_bytes(), 1);
    radix_trie.insert("bcd".as_bytes(), 2);
    radix_trie.insert("d".as_bytes(), 3);
    assert_eq!(radix_trie.lookup("bcd".as_bytes()), Some(&2));
    assert_eq!(radix_trie.lookup("d".as_bytes()), Some(&3));
    let mut radix_trie = RadixTrie::<u32>::new();
    radix_trie.insert("a b".as_bytes(), 1);
    assert_eq!(radix_trie.lookup("a b".as_bytes()), Some(&1));
    let mut radix_trie = RadixTrie::<u32>::new();
    radix_trie.insert("a".as_bytes(), 1);
    radix_trie.insert("aa".as_bytes(), 2);
    radix_trie.insert("aaa".as_bytes(), 3);
    radix_trie.insert("aaaa".as_bytes(), 4);
    assert_eq!(radix_trie.lookup("aaa".as_bytes()), Some(&3));
    assert_eq!(radix_trie.lookup("".as_bytes()), None);
    let radix_trie = RadixTrie::<u32>::new();
    assert_eq!(radix_trie.lookup("aaa".as_bytes()), None);
}
#[test]
fn test_radix_trie_2() {
    let mut radix_trie = RadixTrie::<bool>::new();
    let text = "Daytrip took it to 10
I spent some days in the trap and got my money up (Yeah)
Bust it down, the Rollie, now she tryna fuck (Yeah, ah)
And now she feeding all up on me just like I'm a drug (Yeah)
I remember back when y'all wouldn't show me love (Yeah, ah)
Jaguar truck on the freeway (Woah)
You can catch me ballin' on the replay (Woah, oh)
Saint Laurent jacket, it's 3K (Woah)
I been with the same gang since Pre-K' (Woah, oh)
Get the money, stack it with my niggas
I remember trappin' with my niggas
Now we overseas in a villa
Goin' back, we won't consider
I remember days we was starving
Still finessin', we was juggin', we was robbing
I was coming off the bench, I wasn't starting
Now I'm out here ballin' like I'm Harden
And I make sure my day ones straight, stay with my gang
You ain't fuckin' with me, you can stay out my way
Stay in your lane, fuck your campaign, yeah
Last year, I was sleepin' in a basement
Now the foreign lookin' like a spaceship
Bust down the gang with the bag, yeah
All the money on me, holographic
I spent some days in the trap and got my money up (Yeah)
Bust it down, the Rollie, now she tryna fuck (Yeah, ah)
And now she feeding all up on me just like I'm a drug (Yeah)
I remember back when y'all wouldn't show me love (Yeah, ah)
Jaguar truck on the freeway (Woah)
You can catch me ballin' on the replay (Woah, oh)
Saint Laurent jacket, it's 3K (Woah)
I been with the same gang since Pre-K' (Woah, oh)
We was down and we was out, and bitches looked at us stupid
It was hating ass niggas, so I kept me a Ruger
Sending shots to his body, make him sing like Ozuna
They was running out they mouth, but they respected the shooter
We was down and we was out, and bitches looked at us stupid
It was hating ass niggas, so I kept me a Ruger
Sending shots to his body, make him sing like Ozuna
They was running out they mouth, but they respected the shooter, yeah
Blocked blocks, sent shots
Threw that shit in reverse, then we bending through your block
Blocked blocks, sent shots
Threw that shit in reverse, then we bending through your block
Watch his body Milly Rock, yeah";
    for line in text.split_whitespace() {
        radix_trie.insert(line.as_bytes(), true);
    }
    assert_eq!(radix_trie.lookup("aaa".as_bytes()), None);
    assert_eq!(radix_trie.lookup("shots".as_bytes()), Some(&true));
    assert_eq!(radix_trie.lookup("hating".as_bytes()), Some(&true));
    assert_eq!(radix_trie.lookup("holographic".as_bytes()), Some(&true));
    assert_eq!(radix_trie.lookup("Laurent".as_bytes()), Some(&true));
    assert_eq!(radix_trie.lookup("Daytrip".as_bytes()), Some(&true));
    assert_eq!(radix_trie.lookup("Daytris".as_bytes()), None);
    assert_eq!(radix_trie.lookup("bitches".as_bytes()), Some(&true));
    assert_eq!(radix_trie.lookup("yeah".as_bytes()), Some(&true));
    assert_eq!(radix_trie.lookup("I".as_bytes()), Some(&true));
}
#[test]
fn test_radix_trie_3() {
    for _ in 0..10 {
        let mut radix_trie = RadixTrie::<u32>::new();
        for i in 0..10000 {
            let key = format!("key-{}", i);
            radix_trie.insert(key.as_bytes(), i);
        }
        for i in 0..10000 {
            let key = format!("key-{}", i);
            assert_eq!(radix_trie.lookup(key.as_bytes()), Some(&i));
        }
    }
}
#[test]
fn test_radix_trie_4() {
    let mut radix_trie = RadixTrie::<u32>::new();
    radix_trie.insert("abcd".as_bytes(), 1);
    radix_trie.insert("abce".as_bytes(), 2);
    radix_trie.insert("abt".as_bytes(), 3);
    radix_trie.insert("ab".as_bytes(), 4);
    assert_eq!(radix_trie.lookup("ab".as_bytes()), Some(&4));
}
#[test]
fn test_radix_trie_5() {
    let mut radix_trie = RadixTrie::<u32>::new();
    radix_trie.insert("abcd".as_bytes(), 1);
    radix_trie.insert("bd".as_bytes(), 2);
    assert_eq!(radix_trie.lookup("abcd".as_bytes()), Some(&1));
    assert_eq!(radix_trie.lookup("bd".as_bytes()), Some(&2));
    assert_eq!(radix_trie.lookup("ab".as_bytes()), None);
    assert_eq!(radix_trie.lookup("cd".as_bytes()), None);
    assert_eq!(radix_trie.lookup("bcd".as_bytes()), None);
}
#[test]
fn test_radix_trie_6() {
    let mut radix_trie = RadixTrie::<u32>::new();
    radix_trie.insert("aaaaa".as_bytes(), 1);
    radix_trie.insert("aaaa".as_bytes(), 2);
    radix_trie.insert("aaa".as_bytes(), 3);
    radix_trie.remove("aaaaa".as_bytes());
    radix_trie.remove("aaaa".as_bytes());
    radix_trie.remove("aaa".as_bytes());
    assert_eq!(radix_trie.lookup("aaaaa".as_bytes()), None);
    assert_eq!(radix_trie.lookup("aaaa".as_bytes()), None);
    assert_eq!(radix_trie.lookup("aaa".as_bytes()), None);
}
#[test]
fn test_radix_trie_7() {
    let symbols = Vec::from("abcdefghijklmnopqrstuvwxyz");
    const SZ: usize = 100_000;
    let mut rng = Rand::new(0);
    let mut radix_trie = RadixTrie::<bool>::new();
    for _ in 0..10 {
        let mut keys = vec![];
        for _ in 0..SZ {
            let mut key = String::new();
            for _ in 0..rng.rand_range(1, 20) {
                key.push(symbols[rng.rand_range(1, symbols.len() as i32 - 1) as usize] as char);
            }
            keys.push(key);
        }
        for key in &keys {
            radix_trie.insert(key.as_bytes(), true);
        }
        for _ in 0..1000 {
            let idx = rng.rand_range(1, (SZ - 1) as i32);
            assert_eq!(radix_trie.lookup(keys[idx as usize].as_bytes()), Some(&true));
        }
        for key in &keys {
            radix_trie.remove(key.as_bytes());
        }
        for _ in 0..1000 {
            let idx = rng.rand_range(1, (SZ - 1) as i32);
            assert_eq!(radix_trie.lookup(keys[idx as usize].as_bytes()), None);
        }
    }
}
#[test]
fn test_radix_trie_8() {
    struct St {
        name: String,
        _value: i32,
    }
    let mut radix_trie = RadixTrie::<Box<St>>::new();
    radix_trie.insert(
        "id_1".as_bytes(),
        Box::new(St {
            name: "John".to_string(),
            _value: 17,
        }),
    );
    radix_trie.insert(
        "id_2".as_bytes(),
        Box::new(St {
            name: "Katy".to_string(),
            _value: 23,
        }),
    );

    assert_eq!(radix_trie.lookup("id_1".as_bytes()).unwrap().name, "John");
    assert_eq!(radix_trie.lookup("id_2".as_bytes()).unwrap().name, "Katy");
}
#[test]
fn test_radix_trie_9() {
    let mut radix_trie = RadixTrie::<u32>::new();
    radix_trie.insert("a".as_bytes(), 1);
    radix_trie.insert("ab".as_bytes(), 2);
    radix_trie.insert("abc".as_bytes(), 3);
    assert_eq!(radix_trie.lookup("a".as_bytes()), Some(&1));
    assert_eq!(radix_trie.lookup("ab".as_bytes()), Some(&2));
    assert_eq!(radix_trie.lookup("abc".as_bytes()), Some(&3));
    radix_trie.remove("a".as_bytes());
    assert_eq!(radix_trie.lookup("ab".as_bytes()), Some(&2));
    assert_eq!(radix_trie.lookup("abc".as_bytes()), Some(&3));
    radix_trie.remove("ab".as_bytes());
    assert_eq!(radix_trie.lookup("abc".as_bytes()), Some(&3));
    radix_trie.remove("abc".as_bytes());
    assert_eq!(radix_trie.lookup("abc".as_bytes()), None);
}
#[test]
fn test_radix_trie_10() {
    let mut radix_trie = RadixTrie::<u32>::new();
    radix_trie.insert("你好".as_bytes(), 1);
    radix_trie.insert("好搞笑".as_bytes(), 2);
    assert_eq!(radix_trie.lookup("你好".as_bytes()), Some(&1));
    assert_eq!(radix_trie.lookup("好搞笑".as_bytes()), Some(&2));
}
#[test]
fn test_radix_trie_11() {
    let mut radix_trie = RadixTrie::<()>::new();
    radix_trie.insert("a".as_bytes(), ());
    radix_trie.insert("ab".as_bytes(), ());
    radix_trie.insert("abc".as_bytes(), ());
    let keys: Vec<String> = radix_trie
        .iter("a".as_bytes())
        .map(|(key, _)| String::from_utf8(key).unwrap())
        .collect();
    assert_eq!(keys, vec!["a".to_string(), "ab".to_string(), "abc".to_string()]);
}
#[test]
fn test_radix_trie_12() {
    let mut radix_trie = RadixTrie::<()>::new();
    radix_trie.insert("aaa".as_bytes(), ());
    radix_trie.insert("aaab".as_bytes(), ());
    radix_trie.insert("aaabc".as_bytes(), ());
    let keys: Vec<String> = radix_trie
        .iter("aa".as_bytes())
        .map(|(key, _)| String::from_utf8(key).unwrap())
        .collect();
    assert_eq!(keys, vec!["aaa".to_string(), "aaab".to_string(), "aaabc".to_string()]);
}
#[test]
fn test_radix_trie_13() {
    let mut radix_trie = RadixTrie::<()>::new();
    radix_trie.insert("aaa".as_bytes(), ());
    radix_trie.insert("aaab".as_bytes(), ());
    radix_trie.insert("aaabc".as_bytes(), ());
    let keys: Vec<String> = radix_trie
        .iter("aaab".as_bytes())
        .map(|(key, _)| String::from_utf8(key).unwrap())
        .collect();
    assert_eq!(keys, vec!["aaab".to_string(), "aaabc".to_string()]);
}
#[test]
fn test_radix_trie_14() {
    let mut radix_trie = RadixTrie::<()>::new();
    radix_trie.insert("aaa".as_bytes(), ());
    radix_trie.insert("aaab".as_bytes(), ());
    radix_trie.insert("aaac".as_bytes(), ());
    radix_trie.insert("aaad".as_bytes(), ());
    let keys: Vec<String> = radix_trie
        .iter("aaa".as_bytes())
        .map(|(key, _)| String::from_utf8(key).unwrap())
        .collect();
    assert_eq!(
        keys,
        vec![
            "aaa".to_string(),
            "aaab".to_string(),
            "aaac".to_string(),
            "aaad".to_string()
        ]
    );
    radix_trie.remove("aaa".as_bytes());
    let keys: Vec<String> = radix_trie
        .iter("aaa".as_bytes())
        .map(|(key, _)| String::from_utf8(key).unwrap())
        .collect();
    assert_eq!(keys, vec!["aaab".to_string(), "aaac".to_string(), "aaad".to_string()]);
}
#[test]
fn test_radix_trie_15() {
    let mut radix_trie = RadixTrie::<()>::new();
    radix_trie.insert("aaa".as_bytes(), ());
    radix_trie.insert("aaab".as_bytes(), ());
    radix_trie.insert("aaac".as_bytes(), ());
    radix_trie.insert("aaad".as_bytes(), ());
    let keys: Vec<String> = radix_trie
        .iter("acc".as_bytes())
        .map(|(key, _)| String::from_utf8(key).unwrap())
        .collect();
    assert_eq!(keys, Vec::<String>::new());
}
#[test]
fn test_radix_trie_16() {
    let mut radix_trie = RadixTrie::<()>::new();
    radix_trie.insert("kare".as_bytes(), ());
    radix_trie.insert("kanojo".as_bytes(), ());
    radix_trie.insert("karetachi".as_bytes(), ());
    radix_trie.insert("sakura".as_bytes(), ());
    radix_trie.insert("korosu".as_bytes(), ());
    let keys: Vec<String> = radix_trie
        .iter("kar".as_bytes())
        .map(|(key, _)| String::from_utf8(key).unwrap())
        .collect();
    assert_eq!(keys, vec!["kare".to_string(), "karetachi".to_string()]);
}
#[test]
fn test_radix_trie_17() {
    let mut radix_trie = RadixTrie::<u32>::new();
    radix_trie.insert("Евгений".as_bytes(), 1);
    radix_trie.insert("Дамир".as_bytes(), 2);
    radix_trie.insert("Дмитрий".as_bytes(), 3);
    let values: Vec<u32> = radix_trie.iter("Д".as_bytes()).map(|(_, value)| *value).collect();
    assert_eq!(values, vec![2, 3]);
}
#[test]
//#[ignore]
fn test_radix_trie_18() {
    let symbols = Vec::from("abcdefghijklmnopqrstuvwxyz");
    const SZ: usize = 10_000_000;
    let mut rng = Rand::new(0);
    let mut radix_trie = RadixTrie::<bool>::new();
    let start = Instant::now();
    for _ in 0..SZ {
        let mut key = String::new();
        for _ in 0..40 {
            key.push(symbols[rng.rand_range(1, symbols.len() as i32 - 1) as usize] as char);
        }
        radix_trie.insert(key.as_bytes(), true);
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    std::thread::sleep(Duration::from_secs(5));
}
#[test]
#[ignore]
fn test_radix_trie_19() {
    let symbols = Vec::from("abcdefghijklmnopqrstuvwxyz");
    const SZ: usize = 10_000_000;
    let mut rng = Rand::new(0);
    let mut map = std::collections::HashMap::new();
    let start = Instant::now();
    for _ in 0..SZ {
        let mut key = String::new();
        for _ in 0..40 {
            key.push(symbols[rng.rand_range(1, symbols.len() as i32 - 1) as usize] as char);
        }
        map.insert(key, true);
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    std::thread::sleep(Duration::from_secs(5));
}
#[test]
#[ignore]
fn test_radix_trie_20() {
    let symbols = Vec::from("abcdefghijklmnopqrstuvwxyz");
    const SZ: usize = 10_000_000;
    let mut rng = Rand::new(0);
    let mut map = std::collections::BTreeMap::new();
    let start = Instant::now();
    for _ in 0..SZ {
        let mut key = String::new();
        for _ in 0..40 {
            key.push(symbols[rng.rand_range(1, symbols.len() as i32 - 1) as usize] as char);
        }
        map.insert(key, true);
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    std::thread::sleep(Duration::from_secs(5));
}

#[test]
fn test_radix_trie_21() {
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

    let item = radix_trie.lookup_mut("rabbit".as_bytes()).unwrap();
    *item = 4;
    assert_eq!(radix_trie.lookup("rabbit".as_bytes()), Some(&4));
}
