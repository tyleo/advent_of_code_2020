use crate::util;

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    dir: i32,
}

impl Ship {
    pub fn new() -> Self {
        Self { x: 0, y: 0, dir: 0 }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn n(&mut self, val: i32) {
        self.y += val;
    }

    pub fn s(&mut self, val: i32) {
        self.y -= val;
    }

    pub fn e(&mut self, val: i32) {
        self.x += val;
    }

    pub fn w(&mut self, val: i32) {
        self.x -= val;
    }

    pub fn l(&mut self, val: i32) {
        self.dir = (self.dir + val) % 360
    }

    pub fn r(&mut self, val: i32) {
        self.dir = if self.dir - val < 0 {
            self.dir - val + 360
        } else {
            self.dir - val
        }
    }

    pub fn f(&mut self, val: i32) {
        match self.dir {
            0 => self.e(val),
            90 => self.n(val),
            180 => self.w(val),
            270 => self.s(val),
            _ => {}
        }
    }
}

pub fn problem_12_1() -> String {
    let file = util::read("input/problem_12_input.txt");
    let actions = file.lines().map(|i| {
        (
            i.chars().nth(0).unwrap(),
            i.chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .unwrap(),
        )
    });
    let mut ship = Ship::new();
    for (action, value) in actions {
        match action {
            'N' => ship.n(value),
            'S' => ship.s(value),
            'E' => ship.e(value),
            'W' => ship.w(value),
            'L' => ship.l(value),
            'R' => ship.r(value),
            'F' => ship.f(value),
            _ => {}
        }
    }

    (ship.get_x().abs() + ship.get_y().abs()).to_string()
}
