use crate::util;

pub fn problem_3_2() -> String {
    let text = util::read("input/problem_3_input.txt");
    let slope = text.lines().enumerate().collect::<Vec<_>>();

    fn count_collisions<'a, T: IntoIterator<Item = &'a (usize, &'a str)>>(
        slope: T,
        right: usize,
        down: usize,
    ) -> usize {
        slope
            .into_iter()
            .map(|(y, row)| {
                row.chars().cycle().nth((y / down) * right).unwrap() == '#' && y % down == 0
            })
            .filter(|i| *i)
            .count()
    }

    (count_collisions(&slope, 1, 1)
        * count_collisions(&slope, 3, 1)
        * count_collisions(&slope, 5, 1)
        * count_collisions(&slope, 7, 1)
        * count_collisions(&slope, 1, 2))
    .to_string()
}
