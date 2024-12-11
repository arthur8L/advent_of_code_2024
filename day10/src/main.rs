use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use util::AdventInput;

const MOVEMENT: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let raw = AdventInput::read("./day10/input".into()).unwrap();

    let inputs = raw
        .lines()
        .iter()
        .map(|r| r.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let th = inputs
        .iter()
        .enumerate()
        .flat_map(|(r_idx, r)| {
            r.iter().enumerate().filter_map(move |(c_idx, c)| {
                if *c == '0' {
                    Some((c_idx, r_idx))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<(usize, usize)>>();

    part1(&inputs, &th);
    part2(&inputs, &th);
}

fn part1(inputs: &[Vec<char>], th: &[(usize, usize)]) {
    let (x_len, y_len) = (inputs[0].len(), inputs.len());
    let score = th
        .par_iter()
        .map(|t| {
            (1..=9)
                .fold(vec![(t.0, t.1)], |a, idx| {
                    let mut na = vec![];
                    a.iter().for_each(|v| {
                        MOVEMENT.iter().for_each(|d| {
                            let (nx, ny) = (v.0 as isize + d.0, v.1 as isize + d.1);
                            if nx < 0
                                || ny < 0
                                || nx >= x_len as isize
                                || ny >= y_len as isize
                                || na.contains(&(nx as usize, ny as usize))
                                || inputs[ny as usize][nx as usize].to_digit(10).unwrap() != idx
                            {
                                return;
                            }

                            na.push((nx as usize, ny as usize))
                        })
                    });
                    na
                })
                .len()
        })
        .sum::<usize>();

    println!("Part 1: {:?}", score);
}

fn part2(inputs: &[Vec<char>], th: &[(usize, usize)]) {
    let (x_len, y_len) = (inputs[0].len(), inputs.len());
    let score = th
        .par_iter()
        .map(|t| {
            (1..=9)
                .fold(vec![(t.0, t.1)], |a, idx| {
                    let mut na = vec![];
                    a.iter().for_each(|v| {
                        MOVEMENT.iter().for_each(|d| {
                            let (nx, ny) = (v.0 as isize + d.0, v.1 as isize + d.1);
                            if nx < 0
                                || ny < 0
                                || nx >= x_len as isize
                                || ny >= y_len as isize
                                || inputs[ny as usize][nx as usize].to_digit(10).unwrap() != idx
                            {
                                return;
                            }

                            na.push((nx as usize, ny as usize))
                        })
                    });
                    na
                })
                .len()
        })
        .sum::<usize>();

    println!("Part 2: {:?}", score);
}
