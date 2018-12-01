use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashSet;

fn read_input(fname: String) -> Result<Vec<i32>, &'static str> {
    let mut ret = Vec::new();
    let input = File::open(fname).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        let my_line = line.unwrap();
        let sign = &my_line[0..1];
        let val = &my_line[1..my_line.len()];
        let number = val.parse::<i32>().unwrap();
        if sign == "+" {
            ret.push(number);
        } else {
            ret.push(number * -1);
        }
    }
    return Ok(ret);
}


fn main() {
    let input = read_input("input.txt".parse().unwrap());
    if let Ok(freqs) = input {
        let mut seen_freqs = HashSet::new();
        let mut cur_freq = 0;
        loop {
            let inner_freqs = freqs.clone();
            for freq_change in inner_freqs.into_iter() {
                println!("Current frequency: {}", cur_freq);
                if seen_freqs.contains(&cur_freq) {
                    println!("Duplicate frequency found: {}", cur_freq);
                    return;
                }
                seen_freqs.insert(cur_freq.clone());
                cur_freq += freq_change;
            }
        }
    }
}