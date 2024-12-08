use std::collections::{HashMap, HashSet};

use util::AdventInput;

fn main() {
    let r_input = AdventInput::read("./day8/input".into()).unwrap();
    let r_lines = r_input.lines();

    let dim = (r_lines[0].chars().count(), r_lines.len());
    let a_map = get_antenna(&r_lines);
    part1(&a_map, dim);
    part2(&a_map, dim);
}

fn part1(loc: &HashMap<char, Vec<(usize, usize)>>, dim: (usize, usize)) {
    let mut set = HashSet::new();
    let (x_range, y_range) = ((0_isize..dim.0 as isize), (0_isize..dim.1 as isize));

    for s in loc.values() {
        s.iter().enumerate().for_each(|(idx, l)| {
            if idx == 0 {
                return;
            }
            for v in &s[..idx] {
                let (diff_x, diff_y) = (v.0 as isize - l.0 as isize, v.1 as isize - l.1 as isize);
                let (n1_x, n1_y) = (v.0 as isize + diff_x, v.1 as isize + diff_y);
                if x_range.contains(&n1_x)
                    && y_range.contains(&n1_y)
                    && !set.contains(&(n1_x, n1_y))
                {
                    set.insert((n1_x, n1_y));
                }

                let (n2_x, n2_y) = (l.0 as isize - diff_x, l.1 as isize - diff_y);
                if x_range.contains(&n2_x)
                    && y_range.contains(&n2_y)
                    && !set.contains(&(n2_x, n2_y))
                {
                    set.insert((n2_x, n2_y));
                }
            }
        })
    }
    println!("Part 1: {:?}", set.len());
}

fn part2(loc: &HashMap<char, Vec<(usize, usize)>>, dim: (usize, usize)) {
    let mut set = HashSet::new();
    let (x_range, y_range) = ((0..dim.0 as isize), (0..dim.1 as isize));

    for s in loc.values() {
        s.iter().enumerate().for_each(|(idx, l)| {
            if !set.contains(&(l.0 as isize, l.1 as isize)) {
                set.insert((l.0 as isize, l.1 as isize));
            }
            if idx == 0 {
                return;
            }
            for v in &s[..idx] {
                let (diff_x, diff_y) = (v.0 as isize - l.0 as isize, v.1 as isize - l.1 as isize);

                let mut iidx = 1;
                loop {
                    let (n1_x, n1_y) = (v.0 as isize + iidx * diff_x, v.1 as isize + iidx * diff_y);
                    if !x_range.contains(&n1_x) || !y_range.contains(&n1_y) {
                        break;
                    }
                    if !set.contains(&(n1_x, n1_y)) {
                        set.insert((n1_x, n1_y));
                    }
                    iidx += 1;
                }

                iidx = 1;
                loop {
                    let (n2_x, n2_y) = (l.0 as isize - iidx * diff_x, l.1 as isize - iidx * diff_y);
                    if !x_range.contains(&n2_x) || !y_range.contains(&n2_y) {
                        break;
                    }
                    if !set.contains(&(n2_x, n2_y)) {
                        set.insert((n2_x, n2_y));
                    }
                    iidx += 1;
                }
            }
        })
    }

    println!("Part 2: {:?}", set.len());
}

fn get_antenna(lines: &[&str]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut a_map = HashMap::new();
    lines.iter().enumerate().for_each(|(r_idx, r)| {
        r.chars().enumerate().for_each(|(c_idx, v)| {
            if v == '.' {
                return;
            }
            (*a_map.entry(v).or_insert(vec![])).push((c_idx, r_idx));
        })
    });
    a_map
}
