/// count duplicates
use std::collections::HashMap;
use std::error::Error;
use std::io::{Read};

type Err = Box<dyn Error>;

fn main() -> Result<(), Err> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    println!("checksum: {}", checksum(&input)?);
    println!("diffed: {}", closest_strings(&input)?);
    Ok(())
}

fn checksum(input: &str) -> Result<i32, Err> {
    let mut chars = HashMap::new();
    let (mut twos, mut threes) = (0, 0);
    for line in input.lines() {
        chars.clear();
        for chr in line.chars() {
            *chars.entry(chr).or_insert(0) += 1;
        }

        let (mut two, mut three) = (false, false);
        for val in chars.values() {
            match val {
                2 => two = true,
                3 => three = true,
                _ => {}
            }
        }
        if two {
            twos += 1;
        }
        if three {
            threes += 1;
        }
    }
    Ok(twos * threes)
}


fn closest_strings(input: &str) -> Result<String, Err> {
    let mut seen: Vec<&str> = vec![];
    for line in input.lines() {
        for ln in &seen {
            let mut diff = 0;
            let mut diff_idx = 100;
            for (idx, (left, right)) in ln.chars().zip(line.chars()).enumerate() {
                if left != right {
                    diff += 1;
                    diff_idx = idx;
                }
            }
            if diff == 1 {
                assert!(diff_idx != 100);
                let mut actual = String::from(&line[..diff_idx]);
                actual += &line[diff_idx + 1..];
                return Ok(actual);
            }
        }
        seen.push(line);
    }

    Err("Nothing Found".into())
}
