use util::AdventInput;

fn main() -> std::io::Result<()> {
    let input = AdventInput::read("day1/input".into())?;
    part1(input.clone());
    part2(input);
    Ok(())
}

fn part1(input: AdventInput) {
    let mut columns = input.split_column_as_i64();
    columns.0.sort();
    columns.1.sort();
    println!(
        "Part1 Answer: {}",
        columns
            .0
            .iter()
            .zip(columns.1.iter())
            .fold(0, |accu, (l, r)| accu + (l - r).abs())
    );
}

fn part2(input: AdventInput) {
    let mut map = std::collections::HashMap::with_capacity(input.len());
    let columns = input.split_column_as_i64();
    columns.0.iter().for_each(|v| {
        let count = columns.1.iter().filter(|cv| **cv == *v).count();
        if count == 0 {
            return;
        }
        *map.entry(*v).or_insert(0) += *v * count as i64;
    });
    println!("Part2 Answer: {}", map.into_values().sum::<i64>());
}
