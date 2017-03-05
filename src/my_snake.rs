// #![feature(plugin)]
// #![plugin(clippy)]

extern crate piston_window;
extern crate find_folder;
extern crate rand;

mod snake;
mod consts;

use piston_window::*;
use snake::*;
use consts::*;

#[derive(PartialEq)]
enum GameState {
    Init,
    Playing,
    Paused,
    GameOver,
}

struct App {
    snake: Snake,
    game_state: GameState,
    message: &'static str,
    cache: Glyphs,
}

impl App {
    fn handle_press(&mut self, key: &keyboard::Key) {
        match *key {
            Key::Up if self.game_state == GameState::Playing => {
                if self.snake.direction != Direction::Down {
                    self.snake.direction = Direction::Up;
                }
            },
            Key::Down if self.game_state == GameState::Playing => {
                if self.snake.direction != Direction::Up {
                    self.snake.direction = Direction::Down;
                }
            },
            Key::Left if self.game_state == GameState::Playing => {
                if self.snake.direction != Direction::Right {
                    self.snake.direction = Direction::Left;
                }
            },
            Key::Right if self.game_state == GameState::Playing => {
                if self.snake.direction != Direction::Left {
                    self.snake.direction = Direction::Right;
                }
            },
            Key::P if self.game_state == GameState::Playing => {
                self.game_state = GameState::Paused;
            },
            Key::R if self.game_state == GameState::Paused => {
                self.game_state = GameState::Playing;
            },
            Key::N => {
                self.snake = Snake::new_with_bounds(self.snake.width, self.snake.height);
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

            ellipse(NEW_BLUE, CELL_SQUARE,
                c.transform.trans(self.snake.egg.0 as f64 * CELL_SIZE,
                self.snake.egg.1 as f64 * CELL_SIZE), g);

            ellipse(NEW_RED, CELL_SQUARE,
                c.transform.trans(self.snake.head.0 as f64 * CELL_SIZE,
                self.snake.head.1 as f64 * CELL_SIZE), g);

            for p in &self.snake.body {
                rectangle(color::BLACK, CELL_SQUARE,
                    c.transform.trans(p.0 as f64 * CELL_SIZE, p.1 as f64 * CELL_SIZE), g);
            }
        }

        if self.game_state != GameState::Playing {
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
            match self.snake.move_on() {
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
        snake: Snake::new_with_bounds(WINDOW_WIDTH / (CELL_SIZE as u32), WINDOW_HEIGHT / (CELL_SIZE as u32)),
        game_state: GameState::Init,
        message: "no message",
        cache: Glyphs::new(&font_path, window.factory.clone()).unwrap(),
    };

    while let Some(e) = window.next() {
        e.update(|args| {
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
