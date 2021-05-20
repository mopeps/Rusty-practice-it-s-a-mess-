use std::collections::HashMap;

fn main() {
    let mut v = vec![1,1,2,2,2,2,3];
    println!("Hello, world! {:?}",mean_and_mode(&mut v));
    let s: &str = "hola";
    println!("{}",pig_latin(&s));
}

fn is_sorted(v: &Vec<i32>) -> bool {
    let mut res = true;
    let mut iter = 0;
    while res && iter < v.len() - 1 {
        if v[iter] > v[iter+1] {
            res = false;
        }
        iter += 1;
    }
    res
}


fn mean_and_mode(v:&mut Vec<i32>) -> (i32,i32) {
    let mut average = 0;
    let mut mode: (i32,i32) = (0,0);
    if  !is_sorted(&v)  {
        v.sort();
    }        
    average = v[v.len() / 2];

    let mut counter = HashMap::new();
    for elem in v {
        let count = counter.entry(elem).or_insert(1);
        *count += 1;
    }
    for elem in counter.iter() {
        if elem.1 > &mode.1 {
            mode.0 = **elem.0;
            mode.1 = *elem.1;
        }
    }
    (average,mode.0)
}

const VOWELS: &str = "aeiou";

fn pig_latin(s:&str) -> String {
  let consonant_pos = first_consonant_pos(&s) + 1;
  let (consonant_part,rest_of_word) = s.split_at(consonant_pos);
  let (non_consonant_part,consonant) = consonant_part.split_at(consonant_part.len() - 1);
  if consonant_pos == s.len() {
      return format!("{}hay",s);
  }
  format!("{}{}{}ay",non_consonant_part,rest_of_word,consonant)

}


fn first_consonant_pos(word: &str) -> usize {
    if word.is_empty() {
        return 0
    }
    let mut pos = 0;

    for i in 0..word.len() - 1  {
        let cur = word.chars().nth(i).unwrap();
        if !VOWELS.contains(cur) {
            return pos;
        }
        pos += 1;
    }
    pos
}
