use std::io;

fn main() {
    let input = parse_input();
    println!("input: {input:?}");

    let part1_res = calc_part1(&input);
    println!("part1 res {part1_res:?}");

    let part2_res = calc_part2(&input);
    println!("part2 res {part2_res:?}");
}

fn parse_input() -> Vec<Vec<char>> {
    io::stdin()
        .lines()
        .map(|line| line.expect("read line error").chars().collect())
        .collect()
}

fn calc_part1(input: &Vec<Vec<char>>) -> usize {
    let rows = input.len();
    let cols = input[0].len();
    let mut res = 0;
    let word = &['X', 'M', 'A', 'S'];
    const DIFFS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for r in 0..rows {
        for c in 0..cols {
            'diff: for (dr, dc) in DIFFS {
                let mut new_r = r;
                let mut new_c = c;

                if input[new_r][new_c] != word[0] {
                    continue 'diff;
                }

                for ch_i in 1..word.len() {
                    match (new_r.checked_add_signed(dr), new_c.checked_add_signed(dc)) {
                        (Some(new_r_val), Some(new_c_val)) => {
                            (new_r, new_c) = (new_r_val, new_c_val)
                        }
                        _ => continue 'diff,
                    }

                    if new_r >= rows || new_c >= cols {
                        continue 'diff;
                    }

                    if input[new_r][new_c] != word[ch_i] {
                        continue 'diff;
                    }
                }

                res += 1;
            }
        }
    }

    res
}
fn calc_part2(input: &Vec<Vec<char>>) -> usize {
    let rows = input.len();
    let cols = input[0].len();
    let mut res = 0;

    for r in 0..rows {
        for c in 0..cols {
            if input[r][c] == 'A' {
                if [[(-1, -1), (1, 1)], [(-1, 1), (1, -1)]]
                    .iter()
                    .all(|diffs| {
                        let near_chars: Vec<_> = diffs
                            .iter()
                            .map(|diff| {
                                get_ch_from_input(input, (r as isize + diff.0, c as isize + diff.1))
                            })
                            .collect();

                        near_chars == [Some('M'), Some('S')] || near_chars == [Some('S'), Some('M')]
                    })
                {
                    res += 1
                }
            }
        }
    }

    res
}

fn get_ch_from_input(input: &Vec<Vec<char>>, (r, c): (isize, isize)) -> Option<char> {
    if r < 0 || r >= input.len() as isize || c < 0 || c >= input[0].len() as isize {
        return None;
    }

    Some(input[r as usize][c as usize])
}
