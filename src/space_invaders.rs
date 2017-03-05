extern crate rand;

use rand::Rng;
use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq)]
pub enum BodyDirection {
    Left,
    Right,
}

#[derive(Debug)]
pub struct SpaceInvaders {
    pub head: Point,
    pub body: Vec<Point>,
    pub bullets: Vec<Point>,
    pub eggs: Vec<Point>,
    pub body_direction: BodyDirection,
    pub width: u32,
    pub height: u32,
    max_bullets: u32,
    count_bullets: u32,
}

impl SpaceInvaders {
    pub fn new_with_bounds(width: u32, height: u32) -> SpaceInvaders {
        SpaceInvaders {
            head: Point((width / 2) as i64, (height - 1) as i64),
            body: (10..20).map(|i| Point(i as i64, 0)).collect(),
            bullets: Vec::new(),
            eggs: Vec::new(),
            body_direction: BodyDirection::Right,
            width: width,
            height: height,
            max_bullets: 200,
            count_bullets: 0,
        }
    }

    // pub fn set_max_bullets(&mut self, max: u32) {
    //     self.max_bullets = max;
    // }

    pub fn gen_egg(&mut self) {
        let index = rand::thread_rng().gen_range(0, self.body.len());
        let p = self.body[index];
        self.eggs.push(p + Point(0, 1));
    }

    pub fn move_on(&mut self) -> Result<(), &'static str> {
        self.del_collides();

        self.update_body();
        self.update_bullets();
        self.update_eggs();

        if self.body.is_empty() {
            Err("You win!!!")
        } else if self.count_bullets >= self.max_bullets {
            Err("You lose!!! No bullets left...")
        } else if self.eggs.contains(&self.head) {
            Err("You lose!!! You were shoot...")
        }
        else {
            Ok(())
        }
    }

    pub fn head_left(&mut self) {
        if self.head.0 > 0 {
            self.head += Point(-1, 0);
        }
    }

    pub fn head_right(&mut self) {
        if self.head.0 < (self.width as i64 - 1) {
            self.head += Point(1, 0);
        }
    }

    fn body_right(&mut self) {
        // let mut tmp_body = HashSet::new();
        // for i in &self.body {
        //     tmp_body.push(Point(i.0 + 1, 0));
        //     if i.0 == ((self.width - 2) as i64) {
        //         self.body_direction = BodyDirection::Left;
        //     }
        // }
        // self.body = tmp_body;
        let tmp_body = self.body.clone();
        self.body = tmp_body.iter().map(|p| {
            if p.0 == (self.width as i64 - 2) {
                self.body_direction = BodyDirection::Left;
            }
            *p + Point(1, 0)
        }).collect();
    }

    fn body_left(&mut self) {
        let tmp_body = self.body.clone();
        self.body = tmp_body.iter().map(|p| {
            if p.0 == 1 {
                self.body_direction = BodyDirection::Right;
            }
            *p + Point(-1, 0)
        }).collect();
    }

    fn update_body(&mut self) {
        let r = rand::thread_rng().gen_range(1, 3);
        for _ in 0..r {
            match self.body_direction {
                BodyDirection::Right => self.body_right(),
                BodyDirection::Left => self.body_left(),
            }
        }
    }

    fn update_bullets(&mut self) {
        self.bullets = self.bullets.iter().filter(|p| p.1 >= 0).map(|p| {
            *p + Point(0, -2)
        }).collect();
    }

    fn update_eggs(&mut self) {
        self.eggs = self.eggs.iter().filter(|p| p.1 < (self.height as i64)).map(|p| {
            *p + Point(0, 1)
        }).collect();
    }

    pub fn shoot(&mut self) {
        let bullet = self.head + Point(0, -1);
        if !self.bullets.contains(&bullet) {
            self.bullets.push(self.head + Point(0, -1));
            self.count_bullets += 1;
        }
    }

    fn del_collides(&mut self) {
        let tmp_eggs = self.eggs.clone();

        self.body = self.body.iter().filter(|p| {
            !self.bullets.contains(*p)
        }).cloned().collect();
        self.eggs = self.eggs.iter().filter(|p| {
            !(self.bullets.contains(*p) || self.bullets.contains(&Point(p.0, p.1 + 1)))
        }).cloned().collect();
        self.bullets = self.bullets.iter().filter(|p| {
            !(tmp_eggs.contains(*p) || tmp_eggs.contains(&Point(p.0, p.1 - 1)))
        }).cloned().collect();
        // self.bullets = self.bullets.difference(&tmp_eggs).cloned().collect::<HashSet<_>>();
    }
}
