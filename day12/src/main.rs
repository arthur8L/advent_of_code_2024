use std::collections::{hash_map::Entry::Vacant, HashMap};
use util::AdventInput;

fn main() {
    let raw = AdventInput::read("./day12/sample".into()).unwrap();
    let input = raw.as_char_vec();

    part1(&input);
}

fn part1(input: &[Vec<char>]) {
    let (x_len, y_len) = (input[0].len(), input.len());
    const DIR: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut p_map: HashMap<char, (usize, usize)> = HashMap::new();

    input.iter().enumerate().for_each(|(r_idx, r)| {
        r.iter().enumerate().for_each(|(c_idx, p)| {
            let c = DIR
                .iter()
                .filter(|(x, y)| {
                    let (nx, ny) = (c_idx as isize + x, r_idx as isize + y);
                    nx < 0
                        || ny < 0
                        || nx >= x_len as isize
                        || ny >= y_len as isize
                        || input[ny as usize][nx as usize] != *p
                })
                .count();

            if let Vacant(e) = p_map.entry(*p) {
                e.insert((1, c));
            } else {
                let pm = p_map.get_mut(p).unwrap();
                pm.0 += 1;
                pm.1 += c;
            }
        })
    });
    let price = p_map.values().map(|v| v.0 * v.1).sum::<usize>();
    println!("Part 1: {:?}", price)
}

#[allow(dead_code)]
fn part1_v1(input: &[Vec<char>]) {
    let x_len = input[0].len();
    let mut plot_map: HashMap<char, (usize, Vec<Vec<usize>>)> = HashMap::new();
    let mut prev_plot: Option<(usize, &char)> = None;

    input.iter().enumerate().for_each(|(r_idx, r)| {
        r.iter().enumerate().for_each(|(c_idx, p)| {
            if prev_plot.is_some_and(|pp| pp.1 == p) && c_idx < x_len - 1 {
                return;
            }
            match prev_plot {
                Some(pp) if c_idx < x_len - 1 || (c_idx == x_len - 1 && pp.1 == p) => {
                    update_plot_map(&mut plot_map, *pp.1, r_idx, pp.0, c_idx);
                }
                Some(pp) if c_idx == x_len - 1 && pp.1 != p => {
                    update_plot_map(&mut plot_map, *pp.1, r_idx, pp.0, c_idx);
                    update_plot_map(&mut plot_map, *p, r_idx, c_idx, c_idx + 1);
                }
                _ => {}
            }
            prev_plot = if c_idx == x_len - 1 {
                None
            } else {
                Some((c_idx, p))
            };
        });
    });

    let price = plot_map
        .values()
        .map(|(x, p)| {
            let p_len = p.len();
            let p_val = p.iter().enumerate().fold((0, 0), |mut a, (r_idx, r)| {
                let c_chunks = r.chunks(2);
                let c_len = c_chunks.len();
                let p_c = c_chunks.map(|c| c[1] - c[0]).sum::<usize>();
                if r_idx == 0 {
                    a.0 += p_c * if p_len == 1 { 2 } else { 1 } + c_len * 2;
                } else {
                    let prev = &p[r_idx - 1];
                    let prev_c = prev.chunks(2).map(|c| c[1] - c[0]).sum::<usize>();
                    let or = check_plot_overwrap(prev, r);
                    a.0 += (p_c - or)
                        + (prev_c - or)
                        + c_len * 2
                        + if p_len - 1 == r_idx { p_c } else { 0 };
                }

                a.1 += p_c;

                a
            });

            println!("{:?} {:?}", (x, p_val), p);
            p_val.0 * p_val.1
        })
        .sum::<usize>();

    // println!("{:?}", plot_map);

    println!("Part 1: {:?}", price);
}

fn update_plot_map(
    map: &mut HashMap<char, (usize, Vec<Vec<usize>>)>,
    plot: char,
    row: usize,
    start: usize,
    end: usize,
) {
    let e = map.entry(plot).or_insert((row, vec![]));
    let p_r_len = e.1.len();
    if p_r_len + 1 == row - e.0 {
        e.1[p_r_len - 1].push(start);
        e.1[p_r_len - 1].push(end);
    } else {
        e.1.push(vec![start, end])
    }
}

fn check_plot_overwrap(top: &[usize], bottom: &[usize]) -> usize {
    top.chunks(2)
        .map(|t| {
            let c_range = t[0]..t[1];
            bottom
                .chunks(2)
                .filter(|v| c_range.contains(&v[0]) || c_range.contains(&v[1]))
                .map(|b| {
                    if c_range.contains(&b[0]) && c_range.contains(&b[1]) {
                        return b[1] - b[0];
                    }

                    if c_range.contains(&b[0]) {
                        return t[1] - b[0];
                    }

                    b[1] - t[0]
                })
                .sum::<usize>()
        })
        .sum()
}
