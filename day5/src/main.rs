use std::{cmp::Ordering, collections::HashMap};

use util::AdventInput;

fn main() {
    let b = AdventInput::read("./day5/input".into()).unwrap();
    let inputs = b.lines();
    let mut input = inputs.split_inclusive(|l| l.is_empty());
    let (rule, input) = (input.next().unwrap(), input.next().unwrap());
    let mut rule_map = HashMap::new();
    rule.iter().for_each(|v| {
        if v.is_empty() {
            return;
        }
        let mut rs = v.split("|");
        let (l, r) = (
            rs.next().unwrap().parse::<usize>().unwrap(),
            rs.next().unwrap().parse::<usize>().unwrap(),
        );

        rule_map.entry(l).or_insert(vec![]).push(r);
    });

    part1(rule_map.clone(), input);
    part2(rule_map, input);
}

fn part1(rule_map: HashMap<usize, Vec<usize>>, input: &[&str]) {
    let ans = input.iter().fold(0_usize, |a, v| {
        let ps = v
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let ps_len = ps.len();
        for (idx, p) in ps.iter().enumerate() {
            if idx == ps_len - 1 {
                continue;
            }

            if !rule_map
                .get(p)
                .is_some_and(|e| ps[..idx].iter().any(|v| e.contains(v)))
                && ps[idx + 1..]
                    .iter()
                    .all(|v| !rule_map.get(v).is_some_and(|m| m.contains(p)))
            {
                continue;
            }
            return a;
        }
        a + ps[ps.len() / 2]
    });

    println!("Part 1 Answer: {:?}", ans);
}

fn part2(rule_map: HashMap<usize, Vec<usize>>, input: &[&str]) {
    let ans = input.iter().fold(0_usize, |a, v| {
        let mut ps = v
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let ps_len = ps.len();

        for (idx, p) in ps.iter().enumerate() {
            if idx == ps_len - 1 {
                return a;
            }

            if !rule_map
                .get(p)
                .is_some_and(|e| ps[..idx].iter().any(|v| e.contains(v)))
                && ps[idx + 1..]
                    .iter()
                    .all(|v| !rule_map.get(v).is_some_and(|m| m.contains(p)))
            {
                continue;
            }

            ps.sort_by(|a, b| {
                if rule_map.get(b).is_some_and(|e| e.contains(a)) {
                    Ordering::Greater
                } else if rule_map.get(a).is_some_and(|e| e.contains(b)) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });

            break;
        }

        a + ps[ps.len() / 2]
    });

    println!("Part 2 Answer: {:?}", ans);
}
