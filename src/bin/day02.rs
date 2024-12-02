use std::io;

fn main() {
    let input = parse_input();
    println!("input: {input:?}");

    let part1_res = calc_part1(&input);
    println!("part1 res {part1_res:?}");

    let part2_res = calc_part2(&input);
    println!("part2 res {part2_res:?}");
}

fn parse_input() -> Vec<Vec<i64>> {
    io::stdin()
        .lines()
        .map(|line| {
            line.expect("read line error")
                .split_whitespace()
                .map(|strs| strs.parse::<i64>().expect("parse error"))
                .collect()
        })
        .collect()
}

fn calc_part1(input: &Vec<Vec<i64>>) -> i64 {
    input.iter().filter(|&levels| is_safe(levels)).count() as i64
}
fn calc_part2(input: &Vec<Vec<i64>>) -> i64 {
    input
        .iter()
        .filter(|&levels| {
            (0..levels.len()).any(|i| is_safe(
                &[&levels[..i], &levels[i+1..]].concat())
            )
        })
        .count() as i64
}

fn is_safe(levels: &[i64]) -> bool {
    let direction = (levels[0] - levels[1]).signum();
    levels.windows(2).all(|w| {
        let diff = w[0] - w[1];
        diff.signum() == direction && (1..=3).contains(&diff.abs())
    })
}
