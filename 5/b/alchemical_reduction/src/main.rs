use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

fn main() {
    // let input = read_input("small_input.txt".parse().unwrap()).unwrap();
    let input = read_input("input.txt".parse().unwrap()).unwrap();
    let first_line = input.get(0).unwrap();

    let to_try_removing = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    // let to_try_removing = vec!["c"];
    let mut shortest_yet = 99999999;
    // This is what we'll continually update
    for bad_type in to_try_removing {
        let mut current = first_line.clone();

        let mut iter_counter = 0;
        loop {
            let mut did_change = false;
            let mut next_input = Vec::new();
            let chars: Vec<_> = current.chars().collect();

            let mut idx = 0;
            while idx < chars.len() {
                let cur = chars.get(idx).unwrap();
                // Skip the bad type we're testing
                if cur.to_lowercase().to_string() == bad_type {
                    idx += 1;
                    did_change = true;
                } else {
                    // Only try to do the substitution if there are following character(s)
                    if idx < chars.len() - 1 {
                        let cur = chars.get(idx).unwrap();
                        let next = chars.get(idx + 1).unwrap();

                        // If the "type" (letter) is the same, but the "polarity" (capitalization) is different, we remove the letters
                        if cur.to_lowercase().to_string() == next.to_lowercase().to_string() && cur.to_string() != next.to_string() {
                            did_change = true;
                            idx += 2;
                        } else {
                            // Keeping the character, add it to the next string and increment by 1
                            next_input.push(chars.get(idx).unwrap().clone());
                            idx += 1;
                        }
                    } else {
                        next_input.push(chars.get(idx).unwrap().clone());
                        idx += 1;
                    }
                }
            }
            current = next_input.into_iter().collect();
            if !did_change {
                break;
            }
            println!("Iteration: {:?} Bad type: {:?}", iter_counter, bad_type);
            iter_counter += 1;
        }
        println!("Ended up with string of length {:?}", current.len());
        if current.len() < shortest_yet {
            shortest_yet = current.len();
        }
    }
    println!("Shortest string found len: {:?}", shortest_yet);
}