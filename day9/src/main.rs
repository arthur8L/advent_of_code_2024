use util::AdventInput;

fn main() {
    let raw = AdventInput::read("./day9/input".into()).unwrap();
    let input = raw.chars().collect::<Vec<char>>();
    part1(&input);
    part2(&input);
}

fn part1(input: &[char]) {
    let mut result = 0_usize;
    let (mut v_idx, mut b_idx) = (0, input.len() - 1);

    let mut bf_c = input[b_idx].to_digit(10).unwrap() as usize;

    for (idx, v) in input.iter().enumerate() {
        // u can just chunk it
        if idx % 2 != 0 {
            continue;
        }
        if idx >= b_idx {
            if bf_c > 0 {
                (v_idx..v_idx + bf_c).for_each(|iidx| {
                    result += iidx * b_idx / 2;
                });
            }
            break;
        }

        let (f, fs) = (
            v.to_digit(10).unwrap() as usize,
            input[idx + 1].to_digit(10).unwrap() as usize,
        );

        (v_idx..v_idx + f + fs).for_each(|iidx| {
            if iidx < v_idx + f {
                result += iidx * idx / 2;
            } else if b_idx >= idx + 2 && bf_c != 0 {
                bf_c -= 1;
                result += iidx * b_idx / 2;
                if bf_c == 0 && b_idx != idx + 2 {
                    b_idx -= 2;
                    bf_c = input[b_idx].to_digit(10).unwrap() as usize;
                }
            }
        });

        v_idx += f + fs;
    }

    println!("Part 1: {:?}", result);
}

fn part2(input: &[char]) {
    let mut m = input
        .chunks(2)
        .enumerate()
        .map(|(idx, v)| {
            (
                Some(v[0].to_digit(10).unwrap() as usize),
                v[0].to_digit(10).unwrap() as usize,
                Vec::<usize>::with_capacity(if idx != input.len() / 2 {
                    v[1].to_digit(10).unwrap() as usize
                } else {
                    0
                }),
            )
        })
        .collect::<Vec<(Option<usize>, usize, Vec<usize>)>>();
    let mut b_idx = m.len() - 1;
    loop {
        let b_item = m[b_idx].0.unwrap();

        let fittable = m
            .iter()
            .enumerate()
            .find(|(idx, v)| *idx < b_idx && (v.2.capacity() - v.2.len()) >= b_item);

        if let Some((idx, _)) = fittable {
            (0..b_item).for_each(|_| m[idx].2.push(b_idx));
            m[b_idx].0 = None;
        }

        if b_idx == 0 {
            break;
        }

        b_idx -= 1;
    }

    let result = m
        .iter()
        .enumerate()
        .fold((0, 0), |(v_idx, mut t), (idx, (f, dv, fs))| {
            let mut vi = v_idx;
            if let Some(v) = f {
                (0..*v).for_each(|_| {
                    t += vi * idx;
                    vi += 1;
                })
            } else {
                vi += dv;
            }

            fs.iter().for_each(|v| {
                t += v * vi;
                vi += 1;
            });
            (v_idx + dv + fs.capacity(), t)
        })
        .1;

    println!("Part 2: {:?}", result);
}
