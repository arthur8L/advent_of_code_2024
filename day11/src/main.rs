use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use util::AdventInput;

// FAILED ON THIS ONE. OVERLY RELIED ON ITERATOR BC IT LOOKED FUN SHOULD HAVE APPROACHED MORE FUNDAMENTALLY
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

fn get_zeroes(v: &usize) -> usize {
    (0..)
        .take_while(|idx| *v >= 10_usize.pow(*idx as u32))
        .count()
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
