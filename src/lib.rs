use counter::Counter;
/// ! A collection of anagram utility functions
/// ! looc era smarganA
use std::{collections::HashSet, str::from_utf8};

fn factorial(n: u128) -> u128 {
  if n <= 1 {
    1
  } else {
    n * factorial(n - 1)
  }
}

/// Count the number of anagrams that can be formed from a word
pub fn count(word: &str) -> u128 {
  let mut unique = HashSet::new();
  let count: usize = word.chars().count();

  for c in word.chars() {
    unique.insert(c);
  }

  if unique.len() == count {
    factorial(count as u128)
  } else {
    let mut seen = HashSet::new();

    let mut divisor: u128 = 1;

    let char_counts = word.chars().collect::<Counter<_>>();

    for c in word.chars() {
      if !seen.contains(&c) {
        divisor *= char_counts[&c] as u128;
        seen.insert(c);
      }
    }

    factorial(count as u128) / divisor
  }
}

/// Count the number of occurences of an anagram in a word
pub fn occurences(word: &str, input: &str) -> u128 {
  let len_word = word.chars().count();
  let len_input = input.chars().count();

  // Check if all counts are zero
  let is_zero = |count: &[i64]| {
    for val in count.iter() {
      if *val != 0 {
        return false;
      }
    }
    true
  };

  let mut count: [i64; 256 as usize] = [0; 256 as usize];

  for val in 0..len_input {
    count[word.as_bytes()[val] as usize] += 1;
  }

  for val in 0..len_input {
    count[input.as_bytes()[val] as usize] -= 1;
  }

  let mut result: u128 = 0;
  result += is_zero(&count) as u128;

  for i in len_input..len_word {
    // add last character
    count[word.as_bytes()[i] as usize] += 1;

    // remove first character
    count[word.as_bytes()[i - len_input] as usize] -= 1;

    result += is_zero(&count) as u128;
  }
  result
}

/// Check if a word is an anagram of another word
pub fn is_anagram(left: &str, right: &str) -> bool {
  if left.chars().count() != right.chars().count() {
    false
  } else {
    let mut count: i128 = 0;

    for c in left.chars() {
      count += c as i128;
    }

    for c in right.chars() {
      count -= c as i128;
    }

    count == 0
  }
}

/// Get the next lexicographically greater anagram
/// (or find the next lexicographically greater permutation of the given string)
/// This function will return either the next greater permutation or the given
/// word if it is already the greatest permutation
/// Examples:
/// "abc" -> "acb"
/// "cba" -> "cba"
pub fn get_next(word: &str) -> String {
  let mut i = word.chars().count() - 1;

  // find the first char smaller than the char next to it
  while i > 0 {
    if word.as_bytes()[i] > word.as_bytes()[i - 1] {
      break;
    }
    i -= 1;
  }

  // word is already the greatest permutation, so return it
  if i == 0 {
    return word.to_string();
  }

  let x = word.as_bytes()[i - 1];
  let mut smallest = i;

  // find the smallest char on the right side of i-1'th char
  // that's greater than word[i - 1]
  let mut j = i + 1;
  while j < word.chars().count() {
    if word.as_bytes()[j] > x && word.as_bytes()[j] < word.as_bytes()[smallest] {
      smallest = j;
    }
    j += 1;
  }

  let mut word_as_bytes: Vec<u8> = word.to_string().into_bytes();

  // swap smallest with word[i - 1]
  let t = word_as_bytes[smallest];
  word_as_bytes[smallest] = word_as_bytes[i - 1];
  word_as_bytes[i - 1] = t;

  // sort right half
  let mut right_half: Vec<u8> = Vec::new();

  for val in i..word.chars().count() {
    right_half.push(word_as_bytes[val]);
  }

  right_half.sort();

  // merge back and return as a String
  let mut ret: Vec<u8> = Vec::new();

  for val in 0..i {
    ret.push(word_as_bytes[val]);
  }

  for val in right_half {
    ret.push(val);
  }

  from_utf8(&ret).unwrap().to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_factorial() {
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2 * 2), 24);
    assert_eq!(factorial(14), 87178291200);
    assert_eq!(factorial(12), 479001600);
  }

  #[test]
  fn test_anagram_count() {
    assert_eq!(count("at"), 2);
    assert_eq!(count("ordeals"), 5040);
    assert_eq!(
      count("abcdefghijklmnopqrstuvwxyz"),
      403291461126605635584000000
    );
    assert_eq!(count("abcdefghijklmabcdefghijklm"), 49229914688306352000000);
    assert_eq!(count("abcdABCDabcd"), 29937600);
  }

  #[test]
  fn test_is_anagram() {
    assert_eq!(is_anagram("hello", "olleh"), true);
    assert_eq!(is_anagram("hello", "ooo"), false);
    assert_eq!(is_anagram("helicopter", "copterheli"), true);
    assert_eq!(is_anagram("hacker", "hackes"), false);
  }

  #[test]
  fn test_occurences() {
    assert_eq!(occurences("forxxorfxdofr", "for"), 3);
    assert_eq!(occurences("hellohelloleh", "hel"), 3);
    assert_eq!(occurences("oofooflolhi", "oo"), 2);
    assert_eq!(occurences("rustiscool", "st"), 1);
    assert_eq!(occurences("thegrandopeningscenerywasgreat", "grand"), 1);
    assert_eq!(occurences("anagrams", "smargana"), 1);
  }

  #[test]
  fn test_get_next() {
    assert_eq!(get_next("abc"), "acb");
    assert_eq!(get_next("bac"), "bca");
    assert_eq!(get_next("aaa"), "aaa");
    assert_eq!(get_next("cba"), "cba");
    assert_eq!(get_next("218765"), "251678");
    assert_eq!(get_next("1234"), "1243");
    assert_eq!(get_next("4321"), "4321");
    assert_eq!(get_next("534976"), "536479");
  }
}
