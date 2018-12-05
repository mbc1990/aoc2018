use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

struct Claim {
    pub id: String,
    pub offset_x: i32,
    pub offset_y: i32,
    pub width: i32,
    pub height: i32
}

fn parse_claim(line: String) -> Claim {
    // #1 @ 108,350: 22x29
    let split = line.split(" ");
    let by_spaces = split.collect::<Vec<&str>>();

    // Parse ID
    let id = by_spaces[0];

    // Parse rect starting position
    let ul_corner = by_spaces[2];
    let len = ul_corner.len();
    let ul_corner_no_colon = &ul_corner[..len-1];
    let ul_split = ul_corner_no_colon.split(",");
    let ul_split_by_comma = ul_split.collect::<Vec<&str>>();
    let offset_x: i32 = ul_split_by_comma[0].parse().unwrap();
    let offset_y: i32 = ul_split_by_comma[1].parse().unwrap();

    // Parse width and height
    let size = by_spaces[3];
    let size_spl = size.split("x");
    let size_spl_x = size_spl.collect::<Vec<&str>>();
    let width: i32 = size_spl_x[0].parse().unwrap();
    let height: i32 = size_spl_x[1].parse().unwrap();

    return Claim{
        id: id.to_string(),
        offset_x: offset_x,
        offset_y: offset_y,
        width: width,
        height: height,
    };
}

fn read_input(fname: String) -> Result<Vec<Claim>, &'static str> {
    let mut ret = Vec::new();
    let input = File::open(fname).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        let my_line = line.unwrap();

        let claim = parse_claim(my_line);
        ret.push(claim);
    }
    return Ok(ret);
}

fn main() {

    let mut total_overlap = 0;

    let mut fabric = [[0; 1000]; 1000];

    // let claims = read_input("input.txt".parse().unwrap()).unwrap();
    let claims = read_input("input.txt".parse().unwrap()).unwrap();
    for claim in claims {
        let start_x = claim.offset_x + 1;
        let start_y = claim.offset_y + 1;
        for i in start_x..start_x+claim.width {
            for k in start_y..start_y+claim.height {
                let exists = fabric[i as usize][k as usize];
                if exists == 1 {
                    total_overlap += 1;
                }
                fabric[i as usize][k as usize] += 1;
            }
        }
    }
    println!("Total overlapping: {}", total_overlap);
}