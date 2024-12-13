use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use util::AdventInput;

fn main() {
    let raw = AdventInput::read("./day11/input".into()).unwrap();
    let input = raw
        .split_ascii_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    part1(&input);
    part2(&input);
}

fn part1(input: &[usize]) {
    let stones = (0..25).fold(input.to_vec(), |a, _| {
        a.iter()
            .flat_map(|v| {
                if *v == 0 {
                    return Vec::from([1]);
                }

                let zeroes = get_zeroes(v);
                if zeroes % 2 == 0 {
                    return Vec::from([
                        v / 10_usize.pow((zeroes / 2) as u32),
                        v % 10_usize.pow((zeroes / 2) as u32),
                    ]);
                }
                Vec::from([v * 2024])
            })
            .collect::<Vec<usize>>()
    });

    println!("Part 1: {:?}", stones.len());
}

fn part2(input: &[usize]) {
    let stone_count = input.iter().map(|v| get_that_stone(*v, 75)).sum::<usize>();

    println!("Part 2: {:?}", stone_count);
}

fn get_zeroes(v: &usize) -> usize {
    (0..)
        .take_while(|idx| *v >= 10_usize.pow(*idx as u32))
        .count()
}

fn get_zeroes_log10(v: &usize) -> usize {
    (*v as f32).log10() as usize
}

fn get_that_stone(mut stone: usize, blinks: usize) -> usize {
    let mut count = 1;
    // println!("{:?}", blinks);
    (0..blinks).for_each(|idx| {
        if stone == 0 {
            stone = 1;
            return;
        }
        let digits = get_zeroes_log10(&stone) + 1;
        if digits % 2 == 0 {
            let divisor = 10_usize.pow((digits / 2) as u32);
            let (lhs, rhs) = (stone / divisor, stone % divisor);
            stone = lhs;
            count += get_that_stone(rhs, blinks - idx - 1);
            return;
        }

        stone *= 2024;
    });

    count
}

#[allow(dead_code)]
fn part2_first_attempt(input: &[usize]) {
    const TOTAL_BLINKS: usize = 25;
    let mut s_map: HashMap<usize, (Vec<Vec<usize>>, usize)> = HashMap::new();
    let mut found_stones = Vec::new();

    let remain_stones = (0..TOTAL_BLINKS).fold(input.to_vec(), |a, idx| {
        let s = a
            .iter()
            .flat_map(|v| {
                if *v < 10 {
                    check_stone_map(&mut s_map, &mut found_stones, *v, TOTAL_BLINKS - idx - 1);
                    return Vec::from([]);
                }
                let zeroes = get_zeroes(v);
                if zeroes % 2 == 0 {
                    let (lhs, rhs) = (
                        v / 10_usize.pow((zeroes / 2) as u32),
                        v % 10_usize.pow((zeroes / 2) as u32),
                    );
                    if lhs < 10 {
                        check_stone_map(&mut s_map, &mut found_stones, lhs, TOTAL_BLINKS - idx - 1);
                        check_stone_map(&mut s_map, &mut found_stones, rhs, TOTAL_BLINKS - idx - 1);
                        return Vec::from([]);
                    }

                    if rhs < 10 {
                        check_stone_map(&mut s_map, &mut found_stones, rhs, TOTAL_BLINKS - idx - 1);
                        return Vec::from([lhs]);
                    }
                    return Vec::from([lhs, rhs]);
                }
                Vec::from([v * 2024])
            })
            .collect::<Vec<usize>>();

        s_map.iter_mut().for_each(|(_, v)| {
            if v.1 == 0 {
                return;
            }
            let ns = blink(&v.0[v.0.len() - 1]);
            v.0.push(ns);
            v.1 -= 1;
        });
        s
    });

    let logged_stones = found_stones
        .iter()
        .map(|(v, c)| s_map.get(v).unwrap().0[*c].len())
        .sum::<usize>();

    println!("Part 2: {:?}", remain_stones.len() + logged_stones);
}

fn blink(s: &[usize]) -> Vec<usize> {
    s.iter()
        .flat_map(|v| {
            if *v == 0 {
                return Vec::from([1]);
            }
            let zeroes = get_zeroes(v);
            if zeroes % 2 == 0 {
                return Vec::from([
                    v / 10_usize.pow((zeroes / 2) as u32),
                    v % 10_usize.pow((zeroes / 2) as u32),
                ]);
            }
            Vec::from([v * 2024])
        })
        .collect::<Vec<usize>>()
}

fn check_stone_map(
    map: &mut HashMap<usize, (Vec<Vec<usize>>, usize)>,
    found_vec: &mut Vec<(usize, usize)>,
    v: usize,
    blink: usize,
) {
    if let Vacant(e) = map.entry(v) {
        e.insert((vec![vec![v]], blink));
    } else {
        found_vec.push((v, blink));
    }
}
