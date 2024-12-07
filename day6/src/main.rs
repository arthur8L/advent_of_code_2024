use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use util::AdventInput;

fn main() {
    let input = AdventInput::read("./day6/input".into()).unwrap();
    let rows = input.lines();

    let guard_mov: HashMap<usize, (isize, isize)> =
        HashMap::from([(0, (0, -1)), (90, (1, 0)), (180, (0, 1)), (270, (-1, 0))]);
    let guard_pos = get_guard_pos(&rows);

    part1(guard_pos, &guard_mov, &rows);
    part2(guard_pos, &guard_mov, &rows);
}

fn part1(mut gp: (usize, usize), gm: &HashMap<usize, (isize, isize)>, lines: &[&str]) {
    let grid = lines
        .iter()
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (x_len, y_len) = (grid[0].len(), grid.len());

    let mut paths = HashSet::from([gp]);
    let mut g_dir = 0_usize;
    loop {
        let m = gm.get(&g_dir).unwrap();
        let (nx, ny) = (gp.0 as isize + m.0, gp.1 as isize + m.1);
        if nx < 0 || ny < 0 || nx > (x_len as isize - 1) || ny > (y_len as isize - 1) {
            break;
        }
        if grid[ny as usize][nx as usize] == '#' {
            g_dir = (g_dir + 90) % 360;
            continue;
        }
        gp = (nx as usize, ny as usize);
        if !paths.contains(&gp) {
            paths.insert(gp);
        }
    }

    println!("Part 1 Answer: {:?}", paths.len());
}

fn part2(mut gp: (usize, usize), gm: &HashMap<usize, (isize, isize)>, lines: &[&str]) {
    let ogp = gp;
    let mut obs = lines
        .iter()
        .enumerate()
        .flat_map(|(r_idx, r)| {
            r.chars().enumerate().filter_map(move |(c_idx, c)| {
                if c != '#' {
                    return None;
                }
                Some((c_idx, r_idx))
            })
        })
        .collect::<Vec<(usize, usize)>>();

    let grid = lines
        .iter()
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (x_len, y_len) = (grid[0].len(), grid.len());

    let mut nblocks = HashSet::new();
    let mut checked_b = HashSet::new();
    let mut g_dir = 0_usize;
    //O(n^2)
    loop {
        let m = gm.get(&g_dir).unwrap();
        let (x_r, y_r) = get_move_rg(gp, m, x_len, y_len);

        let b = obs
            .iter()
            .filter(|bb| x_r.contains(&bb.0) && y_r.contains(&bb.1))
            .cloned()
            .collect::<Vec<(usize, usize)>>();

        let closest_b = if (m.0 + m.1) > 0 {
            if b.is_empty() {
                (
                    if m.0 == 0 {
                        gp.0 as isize
                    } else {
                        x_len as isize
                    },
                    if m.1 == 0 {
                        gp.1 as isize
                    } else {
                        y_len as isize
                    },
                )
            } else {
                (b[0].0 as isize, b[0].1 as isize)
            }
        } else if b.is_empty() {
            (
                if m.0 == 0 { gp.0 as isize } else { -1 },
                if m.1 == 0 { gp.1 as isize } else { -1 },
            )
        } else {
            let v = b[b.len() - 1];
            (v.0 as isize, v.1 as isize)
        };

        if m.0 == 0 {
            1..(closest_b.1 - gp.1 as isize).abs()
        } else {
            1..(closest_b.0 - gp.0 as isize).abs()
        }
        .for_each(|idx| {
            let (nx, ny) = (gp.0 as isize + (idx * m.0), gp.1 as isize + (idx * m.1));
            if ogp.0 == nx as usize && ogp.1 == ny as usize {
                return;
            }
            if checked_b.contains(&(nx, ny)) {
                return;
            }
            checked_b.insert((nx, ny));

            let temp_idx = obs
                .iter()
                .take_while(|v| nx > v.0 as isize || ny > v.1 as isize)
                .count();

            obs.insert(temp_idx, (nx as usize, ny as usize));

            if got_trapped(gp, g_dir, gm, &obs, (x_len, y_len)) {
                nblocks.insert((nx, ny));
            }
            obs.remove(temp_idx);
        });

        if b.is_empty() {
            break;
        }

        g_dir = (g_dir + 90) % 360;

        gp = ((closest_b.0 - m.0) as usize, (closest_b.1 - m.1) as usize);
    }
    //1809 1812 1342

    println!("Part 2 Answer: {:?}", nblocks.len());
}

fn got_trapped(
    mut gp: (usize, usize),
    mut g_dir: usize,
    gm: &HashMap<usize, (isize, isize)>,
    obs: &[(usize, usize)],
    limit: (usize, usize),
) -> bool {
    let mut collided = HashSet::new();
    loop {
        let m = gm.get(&g_dir).unwrap();
        let (x_r, y_r) = get_move_rg(gp, m, limit.0, limit.1);
        let b = obs
            .iter()
            .filter(|bb| x_r.contains(&bb.0) && y_r.contains(&bb.1))
            .collect::<Vec<&(usize, usize)>>();

        if b.is_empty() {
            break;
        }

        let closest_b = if (m.0 + m.1) > 0 {
            b[0]
        } else {
            b[b.len() - 1]
        };

        if collided.contains(&(g_dir, closest_b.0, closest_b.1)) {
            return true;
        }

        collided.insert((g_dir, closest_b.0, closest_b.1));

        g_dir = (g_dir + 90) % 360;

        gp = (
            (closest_b.0 as isize - m.0) as usize,
            (closest_b.1 as isize - m.1) as usize,
        );
    }
    false
}

fn get_move_rg(
    gp: (usize, usize),
    m: &(isize, isize),
    x_len: usize,
    y_len: usize,
) -> (Range<usize>, Range<usize>) {
    (
        match m.0 {
            1.. => (gp.0 + 1)..x_len,
            ..=-1 => 0..gp.0,
            _ => gp.0..(gp.0 + 1),
        },
        match m.1 {
            1.. => (gp.1 + 1)..y_len,
            ..=-1 => 0..gp.1,
            _ => gp.1..(gp.1 + 1),
        },
    )
}

fn get_guard_pos(input: &[&str]) -> (usize, usize) {
    input
        .iter()
        .enumerate()
        .find_map(|(r_idx, r)| {
            let r_v = r.chars().collect::<Vec<char>>();
            if !r_v.contains(&'^') {
                return None;
            };
            Some((r_v.iter().take_while(|v| **v != '^').count(), r_idx))
        })
        .unwrap()
}
