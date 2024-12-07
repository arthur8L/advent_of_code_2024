use util::AdventInput;

fn main() {
    let input = AdventInput::read("./day7/input".into()).unwrap();
    let inputs = input
        .lines()
        .iter()
        .map(|r| {
            let mut v = r.split(":");
            (
                v.next().unwrap().parse::<u64>().unwrap(),
                v.next()
                    .unwrap()
                    .split_whitespace()
                    .map(|iv| iv.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .collect::<Vec<(u64, Vec<u64>)>>();

    part1(&inputs);
    part2(&inputs);
}

fn part1(input: &[(u64, Vec<u64>)]) {
    let result = input.iter().fold(0, |acc, (r, i)| {
        if !b_check(*r, i[0], &i[1..]) {
            return acc;
        }
        acc + r
    });

    println!("Part 1 Answer: {:?}", result);
}

fn part2(input: &[(u64, Vec<u64>)]) {
    let result = input.iter().fold(0, |acc, (r, i)| {
        if !b_check_v2(*r, i[0], &i[1..]) {
            return acc;
        }
        acc + r
    });

    println!("Part 2 Answer: {:?}", result);
}
//206663758039735 (too low)

fn b_check(r: u64, acc: u64, i: &[u64]) -> bool {
    let (add, mul) = (acc + i[0], acc * i[0]);
    if i.len() == 1 {
        return r == add || r == mul;
    }
    add <= r && b_check(r, add, &i[1..]) || mul <= r && b_check(r, mul, &i[1..])
}

fn b_check_v2(r: u64, acc: u64, i: &[u64]) -> bool {
    let (add, mul, cc) = (acc + i[0], acc * i[0], acc * get_pow10(i[0]) + i[0]);
    if i.len() == 1 {
        return r == add || r == mul || r == cc;
    }

    add <= r && b_check_v2(r, add, &i[1..])
        || mul <= r && b_check_v2(r, mul, &i[1..])
        || cc <= r && b_check_v2(r, cc, &i[1..])
}

fn get_pow10(v: u64) -> u64 {
    10_u64.pow((0_u32..).take_while(|idx| v >= 10_u64.pow(*idx)).count() as u32)
}
