use std::collections::HashMap;

fn count_chars(val: String) -> Vec<i32> {
    let mut char_count = vec![0;26];
    for c in val.chars() {
        let i = (c as u32) - 97;
        if i <= 26 {
            char_count[i as usize] += 1;
        }
    }
    char_count
}

fn can_construct(ransom_note: String, magazine: String) -> bool {
    let rchars = count_chars(ransom_note);
    let mchars = count_chars(magazine);
    for (i,j) in rchars.iter().zip(mchars.iter()) {
        if i > j {
            return false;
        }
    }
    true
}

fn can_construct_v2(ransom_note: String, magazine: String) -> bool {
    let mut r_chars = HashMap::new();
    let mut m_chars: HashMap<char, i32> = HashMap::new();
    for c in ransom_note.chars() {
        let count = r_chars.entry(c).or_insert(0);
        m_chars.entry(c).or_insert(0);
        *count += 1;
    }

    for c in magazine.chars() {
        let count = m_chars.entry(c).or_insert(0);
        *count += 1;
    }

    for (k, v) in m_chars.iter() {
        if *v < *r_chars.entry(*k).or_insert(0) {
            return false;
        }
    }

    true
}

fn main() {
    println!("Using Vectors");
    println!("{:?}", can_construct(String::from("a"),String::from("b")));
    println!("{:?}", can_construct(String::from("aa"),String::from("ab")));
    println!("{:?}", can_construct(String::from("aa"),String::from("aab")));

    println!("Using HashMap");
    println!("{:?}", can_construct_v2(String::from("a"),String::from("b")));
    println!("{:?}", can_construct_v2(String::from("aa"),String::from("ab")));
    println!("{:?}", can_construct_v2(String::from("aa"),String::from("aab")));
}
