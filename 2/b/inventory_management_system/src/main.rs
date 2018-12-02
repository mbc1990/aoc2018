extern crate packed_simd;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
use packed_simd::u8x32;


fn read_input(fname: String) -> Result<Vec<String>, &'static str> {
    let mut ret = Vec::new();
    let input = File::open(fname).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        let my_line = line.unwrap();
        ret.push(my_line);
    }
    return Ok(ret);
}

// Whether the string has any letters that appear exactly two time
fn has_exactly_twice(input: String) -> bool {
    let mut char_counts = HashMap::new();
    for c in input.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    for key in char_counts.keys() {
        let count = char_counts.get(key).unwrap();
        if *count == 2 {
            return true;
        }
    }
    return false;
}

// Whether the string has any letters that appear exactly two time
fn has_exactly_thrice(input: String) -> bool {
    let mut char_counts = HashMap::new();
    for c in input.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    for key in char_counts.keys() {
        let count = char_counts.get(key).unwrap();
        if *count == 3 {
            return true;
        }
    }
    return false;
}

// Same performance as non-SIMD, probably something is wrong
fn differ_by_exactly_one_at_same_index_simd(s1: String, s2: String) -> bool {
    let s1b = s1.as_bytes();
    let s2b = s2.as_bytes();

    let gotta_go_fast_s1 = u8x32::new(s1b[0], s1b[1], s1b[2], s1b[3], s1b[4], s1b[5], s1b[6], s1b[7], s1b[8], s1b[9],  s1b[10],  s1b[11],  s1b[12],  s1b[13],  s1b[14],  s1b[15],  s1b[16],  s1b[17],  s1b[18],  s1b[19],  s1b[20],  s1b[21],  s1b[22],  s1b[23],  s1b[24],  s1b[25], 0, 0, 0, 0, 0, 0);
    let gotta_go_fast_s2 = u8x32::new(s2b[0], s2b[1], s2b[2], s2b[3], s2b[4], s2b[5], s2b[6], s2b[7], s2b[8], s2b[9],  s2b[10],  s2b[11],  s2b[12],  s2b[13],  s2b[14],  s2b[15],  s2b[16],  s2b[17],  s2b[18],  s2b[19],  s2b[20],  s2b[21],  s2b[22],  s2b[23],  s2b[24],  s2b[25], 0, 0, 0, 0, 0, 0);

    let xored = gotta_go_fast_s1 ^ gotta_go_fast_s2;

    let mut num_diff = 0;
    for i in 0..26 {
        unsafe {
            if xored.extract_unchecked(i) != 0 {
                num_diff += 1;
            }
        }
    }

    if num_diff == 1 {
        return true;
    }

    return false;
}

fn differ_by_exactly_one_at_same_index(s1: String, s2: String) -> bool {
    let s1b = s1.as_bytes();
    let s2b = s2.as_bytes();

    let mut num_diff = 0;
    for i in 0..s1.len() {
        let xored = s1b[i] ^ s2b[i];
        if xored != 0 {
            num_diff += 1;
        }
    }
    if num_diff == 1 {
        return true;
    }

    return false;
}

fn main() {
    let mut my_map = HashMap::new();
    my_map.insert("blah", 3);

    let input = read_input("input.txt".parse().unwrap());
    let mut relevant_box_ids = Vec::new();
    match input {
        Ok(box_ids) => {
            for box_id in box_ids {
                if has_exactly_twice(box_id.clone()) {
                    relevant_box_ids.push(box_id.clone());
                }
                if has_exactly_thrice(box_id.clone()) {
                    relevant_box_ids.push(box_id.clone());
                }
            }
        }
        Err(_err) => {}
    }

    // println!("Box ids: {:?}", relevant_box_ids);
    for s1 in relevant_box_ids.clone() {
       for s2 in relevant_box_ids.clone()  {
             // if differ_by_exactly_one_at_same_index_simd(s1.clone(), s2.clone()) {
             if differ_by_exactly_one_at_same_index(s1.clone(), s2.clone()) {

                println!("Found boxes!");
                println!("{}", s1);
                println!("{}", s2);
            }
        }
    }
    // TODO: Which of these box IDs differ by exactly one character at the same index?

}
