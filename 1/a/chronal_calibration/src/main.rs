use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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
    match input {
        Ok(freqs) => {
            let answer: i32 = freqs.into_iter().sum();
            println!("Answer: {}", answer);
        }
        Err(_err) => {}
    }
}
