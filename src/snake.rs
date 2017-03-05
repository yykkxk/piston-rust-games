extern crate rand;

use rand::Rng;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(pub i64, pub i64);

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
//
// impl AddAssign<Point> for Point {
//     fn add_assign(&mut self, rhs: Point) {
//         self.0 += rhs.0;
//         self.1 += rhs.1;
//     }
// }

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Snake {
    pub head: Point,
    pub body: Vec<Point>,
    pub egg: Point,
    pub direction: Direction,
    pub width: u32,
    pub height: u32,
}

impl Snake {
    pub fn new_with_bounds(width: u32, height: u32) -> Snake {
        Snake {
            head: Point(10, 10),
            body: Vec::new(),
            egg: Point((width as i64) / 2, (height as i64) / 2),
            direction: Direction::Down,
            width: width,
            height: height,
        }
    }

    pub fn move_on(&mut self) -> Result<(), &'static str> {
        if self.next_head() == self.egg {
            self.grow_once();
        } else {
            self.move_once();
        }

        if self.hit_tail() {
            Err("HitTail...")
        } else if self.hit_wall() {
            Err("HitWall...")
        } else {
            Ok(())
        }
    }

    fn hit_tail(&self) -> bool {
        self.body.len() > 3 && self.body.contains(&self.head)
    }

    fn hit_wall(&self) -> bool {
        let Point(x, y) = self.head;
        x < 0 || y < 0 || x >= (self.width as i64) || y >= (self.height as i64)
    }

    fn next_head(&mut self) -> Point {
        match self.direction {
            Direction::Down => self.head + Point(0, 1),
            Direction::Up => self.head + Point(0, -1),
            Direction::Left => self.head + Point(-1, 0),
            Direction::Right => self.head + Point(1, 0),
        }
    }

    fn next_egg(&self) -> Point {
        let mut new_egg: Point;
        let mut x: u32;
        let mut y: u32;

        loop {
            x = rand::thread_rng().gen_range(0, self.width);
            y = rand::thread_rng().gen_range(0, self.height);
            new_egg = Point(x as i64, y as i64);
            if new_egg != self.head && !self.body.contains(&new_egg) {
                return new_egg;
            }
        }
    }

    fn grow_once(&mut self) {
        self.body.push(self.head);
        self.head = self.egg;

        self.egg = self.next_egg();
    }

    fn move_once(&mut self) {
        self.body.push(self.head);
        self.head = self.next_head();

        self.body.remove(0);
    }
}
