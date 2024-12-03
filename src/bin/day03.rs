use std::io;
use std::io::Read;
use regex::Regex;

fn main() {
    let input = parse_input();
    println!("input: {input:?}");

    let part1_res = calc_part1(&input);
    println!("part1 res {part1_res:?}");

    let part2_res = calc_part2(&input);
    println!("part2 res {part2_res:?}");
}

fn parse_input() -> String {
    let mut buf = "".to_string();
    io::stdin()
        .read_to_string(&mut buf).expect("can't read input");
    
    buf
}

fn calc_part1(input: &String) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c|{
            let a = c[1].parse::<u64>().unwrap();
            let b = c[2].parse::<u64>().unwrap();

            a * b
        })
        .sum()
}
fn calc_part2(input: &String) -> u64 {
    let re = Regex::new(r"(?:mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    let mut is_enabled = true;
    let mut res = 0;
    for c in re.captures_iter(input) {
        match &c[0] {
            "do()" => {
                is_enabled = true;
            }

            "don't()" => {
                is_enabled = false;
            }

            _ => {
                if is_enabled {
                    let a = c[1].parse::<u64>().unwrap();
                    let b = c[2].parse::<u64>().unwrap();

                    res += a * b
                }
            }
        }
    }

    res
}
