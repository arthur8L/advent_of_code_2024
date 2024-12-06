use util::AdventInput;

fn main() -> std::io::Result<()> {
    let input = AdventInput::read("./day2/input".into())?;

    part1(input.clone());
    part2(input);

    Ok(())
}

fn part1(input: AdventInput) {
    let valid_counts = input
        .lines()
        .iter()
        .map(|v| {
            v.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|r| {
            if r[0] == r[1] || r[0].abs_diff(r[1]) > 3 {
                return false;
            }
            let full_len = r.len();
            let multiplier = if r[0] < r[1] { -1 } else { 1 };
            r.iter()
                .enumerate()
                .take_while(|(idx, v)| {
                    if *idx == full_len - 1 {
                        return true;
                    }
                    (1..=3).contains(&((*v - r[idx + 1]) * multiplier))
                })
                .count()
                == full_len
        })
        .count();

    println!("Part 1 Answer:{}", valid_counts);
}

fn part2(input: AdventInput) {
    let valid_counts = input
        .lines()
        .iter()
        .map(|v| {
            v.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|r| {
            let full_len = r.len();
            let mut multiplier = if r[0] < r[1] { -1 } else { 1 };
            let mut pass = true;
            r.iter().enumerate().all(|(idx, v)| {
                if idx == full_len - 1 {
                    return true;
                }
                let diff = *v - r[idx + 1];
                let check = (1..=3).contains(&(diff * multiplier));
                if pass && !check {
                    pass = false;
                    if idx == 1 && r[0] > r[idx + 1] && multiplier < 0 {
                        multiplier *= -1;
                    }
                    return true;
                }
                check
            })
        })
        .count();

    println!("Part 2 Answer:{}", valid_counts);
}
