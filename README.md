## anagram

[![crates.io](https://shields.io/crates/v/anagram.svg)](https://crates.io/crates/anagram)
![Build and Test](https://github.com/terror/anagram/actions/workflows/rust.yml/badge.svg)

A collection of anagram utility functions

## Examples

```rust
use anagram::{count, get_next, is_anagram, occurences};

fn main() {
  // count how many anagrams can be formed from a given word
  let anagram_count = count("ordeals");
  assert_eq!(anagram_count, 5040);

  // count the number of occurences of an anagram in a given word
  let occur = occurences("helloworldhello", "ll");
  assert_eq!(occur, 2);

  // check if a word is an anagram of another word
  let ok = is_anagram("rustiscool", "oolcsistru");
  assert_eq!(ok, true);

  // get the next lexicographically greater anagram
  let next = get_next("abcdefg");
  assert_eq!(next, "abcdegf");

  // get all anagrams of a word
  let mut word: String = String::from("abc");
  for _ in 0..count(&word) {
    // get next anagram
    word = get_next(&word);
    println!("{}", word);
  }
}
```
