// #![feature(plugin)]
// #![plugin(clippy)]

extern crate piston_window;
extern crate find_folder;
extern crate rand;

mod space_invaders;
mod consts;

use piston_window::*;
use space_invaders::*;
use consts::*;

#[derive(PartialEq)]
enum GameState {
    Init,
    Playing,
    Paused,
    GameOver,
}

struct App {
    space_invaders: SpaceInvaders,
    game_state: GameState,
    message: &'static str,
    cache: Glyphs,
    // duration: f64,
}

impl App {
    fn handle_press(&mut self, key: &keyboard::Key) {
        match *key {
            Key::Up if self.game_state == GameState::Playing => {
                self.space_invaders.shoot();
                self.space_invaders.gen_egg();
            },
            Key::Down if self.game_state == GameState::Playing => {
                self.space_invaders.gen_egg();
            },
            Key::Left if self.game_state == GameState::Playing => {
                self.space_invaders.head_left();
            },
            Key::Right if self.game_state == GameState::Playing => {
                self.space_invaders.head_right();
            },
            Key::P if self.game_state == GameState::Playing => {
                self.game_state = GameState::Paused;
            },
            Key::R if self.game_state == GameState::Paused => {
                self.game_state = GameState::Playing;
            },
            Key::N => {
                self.space_invaders = SpaceInvaders::new_with_bounds(self.space_invaders.width, self.space_invaders.height);
                self.game_state = GameState::Playing;
            },
            _ => {},
        }
    }

    fn handle_draw(&mut self, c: &Context, g: &mut G2d) {
        if self.game_state == GameState::GameOver {
            clear(color::WHITE, g);
            text(NEW_RED, FONT_SIZE, self.message,
                &mut self.cache, c.transform.trans(LEFT_MARGIN, TOP_MARGIN), g);
        } else {
            clear(color::grey(0.6), g);

            ellipse(color::BLACK, CELL_SQUARE,
                c.transform.trans(self.space_invaders.head.0 as f64 * CELL_SIZE,
                self.space_invaders.head.1 as f64 * CELL_SIZE), g);

            for p in &self.space_invaders.body {
                rectangle(color::BLACK, CELL_SQUARE,
                    c.transform.trans(p.0 as f64 * CELL_SIZE, p.1 as f64 * CELL_SIZE), g);
            }

            for p in &self.space_invaders.eggs {
                rectangle(NEW_RED, CELL_SQUARE,
                    c.transform.trans(p.0 as f64 * CELL_SIZE, p.1 as f64 * CELL_SIZE), g);
            }

            for p in &self.space_invaders.bullets {
                ellipse(NEW_BLUE, CELL_SQUARE,
                    c.transform.trans(p.0 as f64 * CELL_SIZE, p.1 as f64 * CELL_SIZE), g);
            }
        }

        if self.game_state != GameState::Playing {
            // clear(color::WHITE, g);
            text(color::BLACK, FONT_SIZE, "Press `N` to begin a New game",
                &mut self.cache, c.transform.trans(LEFT_MARGIN, TOP_MARGIN + TEXT_HEIGHT * 3.0), g);
            text(color::BLACK, FONT_SIZE, "Press `P` to Pause",
                &mut self.cache, c.transform.trans(LEFT_MARGIN, TOP_MARGIN + TEXT_HEIGHT * 4.0), g);
            text(color::BLACK, FONT_SIZE, "Press `R` to Resume",
                &mut self.cache, c.transform.trans(LEFT_MARGIN, TOP_MARGIN + TEXT_HEIGHT * 5.0), g);
            text(color::BLACK, FONT_SIZE, "Press `ESC` to Exit",
                &mut self.cache, c.transform.trans(LEFT_MARGIN, TOP_MARGIN + TEXT_HEIGHT * 6.0), g);
        }
    }

    fn handle_update(&mut self, _args: &UpdateArgs) {
        if self.game_state == GameState::Playing {
            // self.duration += _args.dt;
            // println!("{:?}", self.duration);
            match self.space_invaders.move_on() {
                Ok(_) => {},
                Err(message) => {
                    self.game_state = GameState::GameOver;
                    self.message = message;
                },
            }
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("my_snake", [WINDOW_WIDTH, WINDOW_HEIGHT])
                                    .exit_on_esc(true)
                                    // .vsync(true)
                                    .build()
                                    .unwrap_or_else(|e| {
                                        panic!("Faild build PistonWindow: {:?}", e);
                                    });
    window.set_ups(WINDOW_UPS);

    // println!("{:?}", window.size());
    let assets = find_folder::Search::KidsThenParents(3, 5)
                    .for_folder("assets")
                    .unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Bold.ttf");

    let mut app = App {
        space_invaders: SpaceInvaders::new_with_bounds(WINDOW_WIDTH / (CELL_SIZE as u32), WINDOW_HEIGHT / (CELL_SIZE as u32)),
        game_state: GameState::Init,
        message: "no message",
        cache: Glyphs::new(&font_path, window.factory.clone()).unwrap(),
        // duration: 0.0,
    };

    while let Some(e) = window.next() {
        e.update(|args| {
            // println!("{:?}", args);
            app.handle_update(args);
        });

        window.draw_2d(&e, |c, g| {
            app.handle_draw(&c, g);
        });

        e.press(|button| {
            if let Button::Keyboard(key) = button {
                app.handle_press(&key);
            }
        });
    }
}
