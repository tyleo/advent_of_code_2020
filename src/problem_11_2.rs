use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::util;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Cell {
    Empty,
    Full,
    Floor,
}

fn hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

pub fn problem_11_2() -> String {
    let grid: Vec<Vec<_>> = util::read("input/problem_11_input.txt")
        .lines()
        .map(|i| {
            i.chars()
                .filter_map(|j| match j {
                    'L' => Some(Cell::Empty),
                    '.' => Some(Cell::Floor),
                    '#' => Some(Cell::Full),
                    _ => None,
                })
                .collect()
        })
        .collect();

    let mut grids = (grid.clone(), grid);
    loop {
        grids = (grids.1, grids.0);
        let (updating_grid, reference_grid) = &mut grids;

        for i in 0..updating_grid.len() {
            for j in 0..updating_grid[i].len() {
                let (mut tl, mut t, mut tr) = (Cell::Floor, Cell::Floor, Cell::Floor);
                for k in 1..i + 1 {
                    let top = &reference_grid[i - k];
                    if let Cell::Floor = tl {
                        tl = top.get(j.wrapping_sub(k)).copied().unwrap_or(Cell::Floor)
                    }
                    if let Cell::Floor = t {
                        t = top.get(j).copied().unwrap_or(Cell::Floor)
                    }
                    if let Cell::Floor = tr {
                        tr = top.get(j + k).copied().unwrap_or(Cell::Floor)
                    }
                }

                let center = &reference_grid[i];

                let mut cl = Cell::Floor;
                for k in (0..j).rev() {
                    if let Cell::Floor = cl {
                        cl = center.get(k).copied().unwrap_or(Cell::Floor)
                    }
                }

                let mut cr = Cell::Floor;
                for k in j + 1..updating_grid[i].len() {
                    if let Cell::Floor = cr {
                        cr = center.get(k).copied().unwrap_or(Cell::Floor)
                    }
                }

                let (mut bl, mut b, mut br) = (Cell::Floor, Cell::Floor, Cell::Floor);
                for mut k in i + 1..updating_grid.len() {
                    k = k - i;
                    let bot = &reference_grid[i + k];
                    if let Cell::Floor = bl {
                        bl = bot.get(j.wrapping_sub(k)).copied().unwrap_or(Cell::Floor)
                    }
                    if let Cell::Floor = b {
                        b = bot.get(j).copied().unwrap_or(Cell::Floor)
                    }
                    if let Cell::Floor = br {
                        br = bot.get(j + k).copied().unwrap_or(Cell::Floor)
                    }
                }

                let full_count = [tl, t, tr, cl, cr, bl, b, br]
                    .iter()
                    .copied()
                    .filter(|i| *i == Cell::Full)
                    .count();

                updating_grid[i][j] = match reference_grid[i][j] {
                    Cell::Empty => {
                        if full_count == 0 {
                            Cell::Full
                        } else {
                            Cell::Empty
                        }
                    }
                    Cell::Floor => Cell::Floor,
                    Cell::Full => {
                        if full_count >= 5 {
                            Cell::Empty
                        } else {
                            Cell::Full
                        }
                    }
                };
            }
        }

        if hash(&updating_grid) == hash(&reference_grid) {
            break;
        }
    }

    grids
        .0
        .iter()
        .fold(0, |prev, curr| {
            prev + curr.iter().fold(0, |prev, curr| {
                prev + if *curr == Cell::Full { 1 } else { 0 }
            })
        })
        .to_string()
}
