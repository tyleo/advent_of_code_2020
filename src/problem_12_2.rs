use crate::util;

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl Ship {
    pub fn new(waypoint_x: i32, waypoint_y: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            waypoint_x,
            waypoint_y,
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn n(&mut self, val: i32) {
        self.waypoint_y += val;
    }

    pub fn s(&mut self, val: i32) {
        self.waypoint_y -= val;
    }

    pub fn e(&mut self, val: i32) {
        self.waypoint_x += val;
    }

    pub fn w(&mut self, val: i32) {
        self.waypoint_x -= val;
    }

    pub fn l(&mut self, val: i32) {
        let (x, y) = match val {
            90 => (-self.waypoint_y, self.waypoint_x),
            180 => (-self.waypoint_x, -self.waypoint_y),
            270 => (self.waypoint_y, -self.waypoint_x),
            _ => (0, 0),
        };

        self.waypoint_x = x;
        self.waypoint_y = y;
    }

    pub fn r(&mut self, val: i32) {
        let (x, y) = match val {
            90 => (self.waypoint_y, -self.waypoint_x),
            180 => (-self.waypoint_x, -self.waypoint_y),
            270 => (-self.waypoint_y, self.waypoint_x),
            _ => (0, 0),
        };

        self.waypoint_x = x;
        self.waypoint_y = y;
    }

    pub fn f(&mut self, val: i32) {
        let x_speed = self.waypoint_x;
        let y_speed = self.waypoint_y;
        self.x += x_speed * val;
        self.y += y_speed * val;
    }
}

pub fn problem_12_2() -> String {
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
    let mut ship = Ship::new(10, 1);
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
