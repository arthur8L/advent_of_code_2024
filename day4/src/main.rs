use util::AdventInput;

fn main() {
    let input = AdventInput::read("./day4/input".into()).unwrap();

    part1(input.clone());
    part2(input);
}

fn part1(input: AdventInput) {
    const VALID_XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
    const XMAS_DIRS: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let rows = input.lines();

    let (row_range, col_range) = ((0..rows.len() as i32), (0..rows[0].len() as i32));
    let xmas_c = rows.iter().enumerate().fold(0, |accu, (r_idx, r)| {
        accu + r
            .chars()
            .enumerate()
            .filter_map(|(c_idx, c)| {
                if c != 'X' {
                    return None;
                }
                let mut xmas_c = 0;
                XMAS_DIRS.iter().for_each(|dir| {
                    if (1..=3).all(|v| {
                        let (x_dir, y_dir) = (c_idx as i32 + dir.0 * v, r_idx as i32 + dir.1 * v);
                        if !col_range.contains(&x_dir) || !row_range.contains(&y_dir) {
                            return false;
                        }
                        let Some(xmas_char) = rows[y_dir as usize].chars().nth(x_dir as usize)
                        else {
                            return false;
                        };
                        VALID_XMAS[v as usize] == xmas_char
                    }) {
                        return;
                    }
                    xmas_c += 1;
                });
                Some(xmas_c)
            })
            .sum::<i32>()
    });
    println!("Part1 Answer: {:?}", xmas_c);
}

fn part2(input: AdventInput) {
    const X_MAX_DIR: [((i32, i32), (i32, i32)); 2] = [((-1, -1), (1, 1)), ((1, -1), (-1, 1))];
    let rows = input.lines();

    let x_mas_c = rows.iter().enumerate().fold(0, |accu, (r_idx, r)| {
        accu + r
            .chars()
            .enumerate()
            .filter_map(|(c_idx, c)| {
                if c != 'A' {
                    return None;
                }
                if X_MAX_DIR.iter().all(|d| {
                    let (x1_dir, y1_dir) = (c_idx as i32 + d.0 .0, r_idx as i32 + d.0 .1);
                    if x1_dir < 0 || y1_dir < 0 {
                        return false;
                    }
                    let Some(i_c) = rows
                        .get(y1_dir as usize)
                        .and_then(|r_s| r_s.chars().nth(x1_dir as usize))
                    else {
                        return false;
                    };

                    if i_c != 'M' && i_c != 'S' {
                        return false;
                    }
                    let (x2_dir, y2_dir) = (c_idx as i32 + d.1 .0, r_idx as i32 + d.1 .1);
                    if x2_dir < 0 || y2_dir < 0 {
                        return false;
                    }

                    let Some(r_c) = rows
                        .get(y2_dir as usize)
                        .and_then(|r_s| r_s.chars().nth(x2_dir as usize))
                    else {
                        return false;
                    };

                    r_c == if i_c == 'M' { 'S' } else { 'M' }
                }) {
                    return None;
                }
                Some(c)
            })
            .count()
    });
    println!("Part2 Answer: {:?}", x_mas_c);
}
