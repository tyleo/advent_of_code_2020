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

pub fn problem_11_1() -> String {
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
                let top = reference_grid.get(i.wrapping_sub(1));
                let (tl, t, tr) = (
                    top.and_then(|row| row.get(j.wrapping_sub(1)).copied())
                        .unwrap_or(Cell::Floor),
                    top.and_then(|row| row.get(j).copied())
                        .unwrap_or(Cell::Floor),
                    top.and_then(|row| row.get(j + 1).copied())
                        .unwrap_or(Cell::Floor),
                );

                let center = reference_grid.get(i);
                let (cl, c, cr) = (
                    center
                        .and_then(|row| row.get(j.wrapping_sub(1)).copied())
                        .unwrap_or(Cell::Floor),
                    center
                        .and_then(|row| row.get(j).copied())
                        .unwrap_or(Cell::Floor),
                    center
                        .and_then(|row| row.get(j + 1).copied())
                        .unwrap_or(Cell::Floor),
                );

                let bottom = reference_grid.get(i + 1);
                let (bl, b, br) = (
                    bottom
                        .and_then(|row| row.get(j.wrapping_sub(1)).copied())
                        .unwrap_or(Cell::Floor),
                    bottom
                        .and_then(|row| row.get(j).copied())
                        .unwrap_or(Cell::Floor),
                    bottom
                        .and_then(|row| row.get(j + 1).copied())
                        .unwrap_or(Cell::Floor),
                );

                let full_count = [tl, t, tr, cl, cr, bl, b, br]
                    .iter()
                    .copied()
                    .filter(|i| *i == Cell::Full)
                    .count();

                updating_grid[i][j] = match c {
                    Cell::Empty => {
                        if full_count == 0 {
                            Cell::Full
                        } else {
                            Cell::Empty
                        }
                    }
                    Cell::Floor => Cell::Floor,
                    Cell::Full => {
                        if full_count >= 4 {
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
