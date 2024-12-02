use std::io;
use itertools::Itertools;

fn main() {
    let input = parse_input();
    println!("input: {input:?}");

    let part1_res = calc_part1(&input);
    println!("part1 res {part1_res:?}");

    let part2_res = calc_part2(&input);
    println!("part2 res {part2_res:?}");
}

fn parse_input() -> (Vec<u64>, Vec<u64>) {
    io::stdin()
        .lines()
        .map(|line| {
            let line_val = line
                .expect("Unable to read line");
            let (first_str, second_str) = line_val
                .split_once("   ")
                .expect("Unable to split line");

            (
                first_str.parse::<u64>().unwrap(),
                second_str.parse::<u64>().unwrap(),
            )
        })
        .unzip()
}

fn calc_part1(input: &(Vec<u64>, Vec<u64>)) -> u64 {
    let list1 = &mut input.0.clone();
    let list2 = &mut input.1.clone();

    list1.sort_unstable();
    list2.sort_unstable();

    list1.iter().zip(list2)
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn calc_part2(input: &(Vec<u64>, Vec<u64>)) -> u64 {
    let list1 = &input.0;
    let list2_freq = input.1.iter().counts();

    list1.iter()
        .map(|n| {
            let freq = list2_freq.get(n).unwrap_or_else({|| &0} );

            n * *freq as u64
        })
        .sum()
}
