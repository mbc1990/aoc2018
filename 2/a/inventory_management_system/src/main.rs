
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

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

fn main() {

    let mut my_map  = HashMap::new();
    my_map.insert("blah", 3);

    let input = read_input("input.txt".parse().unwrap());
    match input {
        Ok(box_ids) => {
            let mut total_twice = 0;
            let mut total_thrice = 0;
            for box_id in box_ids {
                if has_exactly_twice(box_id.clone()) {
                    total_twice  += 1;
                }
                if has_exactly_thrice(box_id.clone()) {
                    total_thrice += 1;
                }
            }
            println!("twice: {:?}", total_twice);
            println!("thrice: {:?}", total_thrice);
            println!("Product: {:?}", total_twice * total_thrice);
        }
        Err(_err) => {}
    }
}
