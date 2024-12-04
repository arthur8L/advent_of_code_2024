use regex::Regex;
use util::AdventInput;

const VALID_MUL_CHARS: [char; 16] = [
    'm', 'u', 'l', '(', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ',', ')',
];

fn main() {
    let input = AdventInput::read("./day3/input".into()).unwrap();

    // could have done it with String too
    part1(input.clone());
    part2(input);
}

fn part1(input: AdventInput) {
    // you can use Regex::captures_iter for getting answers simply but have O(m * n^2)
    let re = Regex::new(r"mul\((?<lhs>\d{1,3}),(?<rhs>\d{1,3})\)$").unwrap();
    let mul_val = input
        .chars()
        .fold((0, vec![]), |mut accu, c| {
            if !VALID_MUL_CHARS.contains(&c) {
                return (accu.0, vec![]);
            }
            accu.1.push(c);
            if c != ')' {
                return accu;
            }
            let istr = accu.1.iter().collect::<String>();
            if !re.is_match(&istr) {
                return (accu.0, vec![]);
            }
            let caps = re.captures(&istr).unwrap();
            (
                accu.0
                    + caps.name("lhs").unwrap().as_str().parse::<u64>().unwrap()
                        * caps.name("rhs").unwrap().as_str().parse::<u64>().unwrap(),
                vec![],
            )
        })
        .0;

    println!("Part 1 Answer: {:?}", mul_val);
}

fn part2(input: AdventInput) {
    // you can use Regex::captures_iter for getting answers simply but have O(m * n^2)
    let con_re = Regex::new(r"(?<con>do\(\)|don't\(\))$").unwrap();
    let mul_re = Regex::new(r"mul\((?<lhs>\d{1,3}),(?<rhs>\d{1,3})\)$").unwrap();
    let mul_val = input
        .chars()
        .enumerate()
        .fold((0, true, vec![]), |mut accu, (idx, c)| {
            if !VALID_MUL_CHARS.contains(&c) {
                return (accu.0, accu.1, vec![]);
            }
            accu.2.push(c);
            if c != ')' {
                return accu;
            }

            if let Some(c) = con_re.captures(&input[(idx as i32 - 7) as usize..=idx]) {
                accu.1 = c.name("con").unwrap().as_str() == "do()"
            }

            let mul_str = accu.2.iter().collect::<String>();
            if !mul_re.is_match(&mul_str) {
                return (accu.0, accu.1, vec![]);
            }

            if !accu.1 {
                return (accu.0, accu.1, vec![]);
            }

            let mul_caps = mul_re.captures(&mul_str).unwrap();
            (
                accu.0
                    + mul_caps
                        .name("lhs")
                        .unwrap()
                        .as_str()
                        .parse::<u64>()
                        .unwrap()
                        * mul_caps
                            .name("rhs")
                            .unwrap()
                            .as_str()
                            .parse::<u64>()
                            .unwrap(),
                accu.1,
                vec![],
            )
        })
        .0;

    println!("Part 2 Answer: {:?}", mul_val);
}
