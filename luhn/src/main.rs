use std::str::Chars;

pub fn luhn(cc_number: &str) -> bool {
    let mut arr: Vec<u32> = vec![];
    for x in cc_number.chars().rev() {
        if x.is_whitespace() {
            continue;
        }
        if !x.is_numeric() {
            return false;
        }
        match x.to_string().parse::<u32>() {
            Ok(i) => arr.push(i),
            _ => return false,
        }
    }
    if arr.len() < 2 {
        return false;
    }
    let mut i = 1;
    let mut sum = 0;
    for mut x in arr {
        if i % 2 == 0 {
            sum += trim_num(x * 2)
        } else {
            sum += x;
        }
        i += 1;
    }
    return sum % 10 == 0;
}

fn trim_num(mut i: u32) -> u32 {
    if i < 10 {
        return i;
    }
    let mut y: u32 = 0;
    while i > 0 {
        y += i % 10;
        i = i / 10;
    }
    return trim_num(y);
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
