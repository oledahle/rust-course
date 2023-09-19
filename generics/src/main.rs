use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Mul;

fn main() {
    println!("Hello, world!");
}

struct Holder<T> {
    val: T,
}

impl<T> Holder<T> {
    pub fn new(val: T) -> Self {
        Holder { val }
    }
}


fn combine_frequencies<T: Eq + Hash> (
    mut acc: HashMap<T, usize>,
    freqs: HashMap<T, usize>,
) -> HashMap<T, usize> {
    for (c, count) in freqs {
        *acc.entry(c).or_insert(0) += count;
    }
    acc
}

pub trait FileHandler {}

#[derive(PartialEq, Debug)]
struct WindowsFileHandler;
impl FileHandler for WindowsFileHandler {}

#[derive(PartialEq, Debug)]
struct OSXFileHandler;
impl FileHandler for OSXFileHandler {}

pub fn vectorOfHandlers<T : FileHandler>(a : T, b : T) -> Vec<T> {
    vec![a, b]
}

pub fn square<T : std::ops::Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holder_with_i64() {
        assert_eq!(Holder::new(24i64).val, 24i64);
    }

    #[test]
    fn holder_with_string() {
        assert_eq!(Holder::new("rust").val, "rust");
    }

    #[test]
    fn combine_two_int_hashes() {
        let mut hash1  = HashMap::new();
        hash1.insert(1, 1);
        hash1.insert(2,1);

        let mut hash2  = HashMap::new();
        hash2.insert(1, 1);
        hash2.insert(2,1);

        let hm_comb = combine_frequencies(hash1, hash2);
        assert_eq!(*hm_comb.get(&1).unwrap(), 2);
    }

    #[test]
    fn test_combine_frequencies_generic() {
        let mut hm1 = HashMap::new();
        hm1.insert('a', 2);

        let mut hm2 = HashMap::new();
        hm2.insert('a', 2);

        let hm_comb = combine_frequencies(hm1, hm2);
        assert_eq!(*hm_comb.get(&'a').unwrap(), 4);
    }
    #[test]
    fn vector_of_osxhandlers() {
        let  o1 = OSXFileHandler{};
        let  o2 =  OSXFileHandler{};
        let myvec = vectorOfHandlers(o1, o2);
        assert_eq!(myvec.len(), 2);
        assert_eq!(myvec[0], OSXFileHandler);
        assert_eq!(myvec[1], OSXFileHandler);
    }

    #[test]
    fn square_of_types() {
        let two : i32 = 2;
        let two64 : u64 = 2;
        assert_eq!(square(two,two), 4);
        assert_eq!(square(two64,two64), 4);
        assert!(square(4.4,4.4) < 20.0);
    }
}