extern crate rand;

use rand::Rng;
use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point(pub i64, pub i64);

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

pub enum Direction {
    Left,
    Right,
    Down,
}

pub struct Tetris {
    now_shape: Vec<Point>,
    bottom: Vec<Point>,
    width: u32,
    height: u32,
    offset: Point,
    score: u32,
    now_index: usize,
    next_index: usize,
}

const SHAPES: [[Point; 4]; 7] = [[Point(-1, 0), Point(0, 0), Point(0, 1), Point(1, 0)], // -|-
                                 [Point(-1, 1), Point(0, 1), Point(0, 0), Point(1, 0)], // Z
                                 [Point(-1, 0), Point(0, 1), Point(0, 0), Point(1, 1)], // S
                                 [Point(-1, 0), Point(0, 0), Point(1, 0), Point(2, 0)], // --
                                 [Point(-1, 1), Point(-1, 0), Point(0, 0), Point(1, 0)], // L
                                 [Point(-1, 0), Point(0, 0), Point(1, 0), Point(1, 1)], // _|
                                 [Point(0, 1), Point(0, 0), Point(1, 1), Point(1, 0)] /* O */];

fn rand_index() -> usize {
    rand::thread_rng().gen_range(0, 7)
}

impl Tetris {
    pub fn new_with_bounds(width: u32, height: u32) -> Tetris {
        Tetris {
            now_shape: SHAPES[0].iter().cloned().collect::<Vec<Point>>(),
            bottom: Vec::new(),
            width: width,
            height: height,
            offset: Point(width as i64 / 4, 0),
            score: 0,
            now_index: 0,
            next_index: rand_index(),
        }
    }

    pub fn reset(&mut self) {
        self.bottom.clear();
        self.score = 0;
        self.offset = Point(self.width as i64 / 4, 0);
    }

    pub fn bottom(&self) -> &Vec<Point> {
        &self.bottom
    }

    pub fn now_shape(&self) -> Vec<Point> {
        self.now_shape
            .iter()
            .map(|p| *p + self.offset)
            .collect()
    }

    pub fn next_shape(&self) -> [Point; 4] {
        SHAPES[self.next_index]
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn move_on(&mut self) -> Result<(), &'static str> {
        // if self.score >= 2000 {
        //     return Err("You win! Score >= 2000...");
        // }
        if self.move_collides(Direction::Down) {
            self.push_bottom();
            self.del_full_lines();
            self.update_shape();
            if self.game_over() {
                Err("You lose! Game over...")
            } else {
                Ok(())
            }
        } else {
            self.down_once();
            Ok(())
        }

    }

    // pub fn ghost_shape(&mut self) -> Vec<Point> {
    //     let mut dt = 0;
    //
    //     loop {
    //         let collides: bool = self.now_shape.iter().any(|p| {
    //             let next_p = *p + self.offset + Point(0, dt + 1);
    //             next_p.1 >= (self.height as i64) || self.bottom.contains(&next_p)
    //         });
    //
    //         if collides { break; }
    //         dt += 1;
    //     }
    //
    //     self.now_shape.iter().map(|p| {
    //         *p + self.offset + Point(0, dt)
    //     }).collect()
    // }

    fn rotated_shape(&self) -> Vec<Point> {
        self.now_shape
            .iter()
            .map(|p| Point(p.1, -p.0))
            .collect()
    }

    pub fn rotate(&mut self) {
        if self.now_index != 6 {
            let tmp_shape = self.rotated_shape();
            let rotate_collides: bool = tmp_shape.iter().any(|p| {
                let next_p = self.offset + *p;
                next_p.0 < 0 || next_p.0 >= (self.width as i64 / 2) ||
                next_p.1 >= (self.height as i64) || self.bottom.contains(&next_p)
            });
            if !rotate_collides {
                self.now_shape = tmp_shape;
            }
        }
    }

    fn move_collides(&self, direction: Direction) -> bool {
        let mut tmp_offset = self.offset;
        match direction {
            Direction::Down => tmp_offset += Point(0, 1),
            Direction::Left => tmp_offset += Point(-1, 0),
            Direction::Right => tmp_offset += Point(1, 0),
        }
        self.now_shape.iter().any(|p| {
            let next_p = tmp_offset + *p;
            next_p.0 < 0 || next_p.0 >= (self.width as i64 / 2) ||
            next_p.1 >= (self.height as i64) || self.bottom.contains(&next_p)
        })
    }

    pub fn down_once(&mut self) {
        if !self.move_collides(Direction::Down) {
            self.offset += Point(0, 1);
        }
    }

    pub fn right_once(&mut self) {
        if !self.move_collides(Direction::Right) {
            self.offset += Point(1, 0);
        }
    }

    pub fn left_once(&mut self) {
        if !self.move_collides(Direction::Left) {
            self.offset += Point(-1, 0);
        }
    }

    pub fn down_immediately(&mut self) {
        // loop {
        //     if self.move_collides(Direction::Down) {
        //         break;
        //     }
        //     self.offset += Point(0, 1);
        // }

        while !self.move_collides(Direction::Down) {
            self.offset += Point(0, 1);
        }
    }

    fn push_bottom(&mut self) {
        for p in &self.now_shape {
            self.bottom.push(*p + self.offset);
        }
    }

    fn update_shape(&mut self) {
        self.now_shape.clear();
        for p in &SHAPES[self.next_index] {
            self.now_shape.push(*p);
        }
        self.offset = Point(self.width as i64 / 4, 0);
        self.now_index = self.next_index;

        // let i = rand::thread_rng().gen_range(0, 7usize);
        self.next_index = rand_index();
    }

    fn del_full_lines(&mut self) {
        let full_lines = self.full_lines();
        self.add_score(full_lines.len());

        for line in full_lines {
            self.bottom = self.bottom
                .iter()
                .filter(|p| p.1 != line)
                .map(|p| if p.1 < line { *p + Point(0, 1) } else { *p })
                .collect();
        }
    }

    fn full_lines(&self) -> Vec<i64> {
        let mut full_lines = Vec::new();

        for y in 0..(self.height as i64) {
            let is_full = (0..(self.width as i64 / 2)).all(|x| self.bottom.contains(&Point(x, y)));
            if is_full {
                full_lines.push(y);
            }

            // let mut full = true;
            // for x in 0..(self.width as i64 / 2) {
            //     if !self.bottom.contains(&Point(x, y)) {
            //         full = false;
            //     }
            // }
            // if full {
            //     full_lines.push(y);
            // }
        }

        full_lines
    }

    fn add_score(&mut self, lines_num: usize) {
        match lines_num {
            0 => {}
            1 => self.score += 100,
            2 => self.score += 300,
            3 => self.score += 500,
            4 => self.score += 800,
            _ => panic!("full_lines > 4 !!!"),
        }
    }

    fn game_over(&self) -> bool {
        for p in &self.bottom {
            if p.1 <= 0 {
                return true;
            }
        }
        false
    }
}
// fn main() {
//     let t = Tetris::new_with_bounds(600, 400);
// }
