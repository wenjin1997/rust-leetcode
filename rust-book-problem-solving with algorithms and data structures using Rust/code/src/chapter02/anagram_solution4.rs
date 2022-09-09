// 2.4.4 计数和比较法

fn anagram_solution4(s1: &str, s2: &str) -> bool {
    // base case
    if s1.len() != s2.len() { return false; }

    // 两个数组，用来记录字符串中 26 个英文字母出现的频次
    let mut c1 = [0; 26];
    let mut c2 = [0; 26];

    for c in s1.chars() {
        let index: usize = (c as u8 - 'a' as u8) as usize;
        c1[index] += 1;
    }

    for c in s2.chars() {
        let index: usize = (c as u8 - 'a' as u8) as usize;
        c2[index] += 1;
    }

    // 比较 c1 和 c2 中的数字是否都相等
    for i in 0..26 {
        let index = i as usize;
        if c1[index] != c2[index] { return false; }
    }

    true
}

#[test]
fn main() {
    let s1 = "rust";
    let s2 = "trus";
    let result: bool = anagram_solution4(s1, s2);
    println!("s1 and s2 is anagram: {}", result);
}