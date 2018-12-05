use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::prelude::v1::Vec;

#[derive(Clone)]
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

// TODO: Implement this function
fn claims_overlap(claim_a: Claim, claim_b: Claim) -> bool {
    let mut fabric = [[0; 1000]; 1000];

    // Populate claim_a
    let start_x = claim_a.offset_x + 1;
    let start_y = claim_a.offset_y + 1;
    for i in start_x..start_x+claim_a.width {
        for k in start_y..start_y+claim_a.height {
            let exists = fabric[i as usize][k as usize];
            fabric[i as usize][k as usize] += 1;
        }
    }

    let mut overlaps = false;
    // Find out if claim b overlaps
    let start_x = claim_b.offset_x + 1;
    let start_y = claim_b.offset_y + 1;
    for i in start_x..start_x+claim_b.width {
        for k in start_y..start_y+claim_b.height {
            let exists = fabric[i as usize][k as usize];
            if exists == 1 {
                overlaps = true;
                break;
            }
        }
    }
    return overlaps;
}

// TODO: Find claim that doesn't overlap
fn main() {

    let mut total_overlap = 0;

    let mut fabric = [[0; 1000]; 1000];

    // let claims = read_input("input.txt".parse().unwrap()).unwrap();
    let claims_a = read_input("input.txt".parse().unwrap()).unwrap();
    let claims_b = claims_a.clone();
    let mut counter = 0;
    for claim_a in claims_a {
        println!("Processing claim {:?}", counter);
        counter += 1;
        let mut is_valid = true;
        for claim_b in claims_b.clone() {

            // Don't compare a claim against itself
            if claim_a.id == claim_b.id {
                continue;
            }
            if  claims_overlap(claim_a.clone(), claim_b.clone()) {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            println!("Found valid claim: {:?}", claim_a.id);
            return;
        }
    }

    /*
    let mut valid = Vec::new();
    let mut possible = claims.clone();

    // TODO: This doesn't work because a rectangle will rule itself out on subsequent runs
    while valid.len() != 1 {
        valid = Vec::new();
        println!("Valid size: {}", valid.len());
        for claim in possible.clone() {
            // Fill in matrix
            let start_x = claim.offset_x + 1;
            let start_y = claim.offset_y + 1;
            let mut is_valid = true;
            for i in start_x..start_x+claim.width {
                for k in start_y..start_y+claim.height {
                    let exists = fabric[i as usize][k as usize];
                    println!("Found arr val: {}", exists);
                    if exists >= 1 {
                        is_valid = false;
                    }
                    fabric[i as usize][k as usize] += 1;
                }
            }

            // No conflicts yet
            if is_valid {
                println!("Valid rect found");
                valid.push(claim);
            } else {
                println!("Rect invalid");
            }
        }
        possible = valid.clone();
        println!("Valid size: {}", valid.len());
    }
    */

    println!("Done");

    // while valid.len != 1
        // for each claim in possible
            // fill in matrix
            // if still valid
                // add to valid
        // possible = valid
        // valid = new vector
    /*
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
    */
}