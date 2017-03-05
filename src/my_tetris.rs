// #![feature(plugin)]
// #![plugin(clippy)]

extern crate piston_window;
extern crate find_folder;
extern crate rand;
// extern crate ears;

mod tetris;
mod consts;

use piston_window::*;
use tetris::*;
use consts::*;
// use ears::{Sound, Music, AudioController};

#[derive(PartialEq)]
enum GameState {
    Init,
    Playing,
    Paused,
    GameOver,
}

struct App {
    tetris: Tetris,
    game_state: GameState,
    message: &'static str,
    cache: Glyphs, 
    // background_music: Music, // shoot_sound: Sound,
}

impl App {
    fn handle_press(&mut self, key: &keyboard::Key) {
        match *key {
            Key::Up if self.game_state == GameState::Playing => {
                self.tetris.rotate();
            }
            Key::Down if self.game_state == GameState::Playing => {
                self.tetris.down_immediately();
            }
            Key::Left if self.game_state == GameState::Playing => {
                self.tetris.left_once();
            }
            Key::Right if self.game_state == GameState::Playing => {
                self.tetris.right_once();
            }
            Key::P if self.game_state == GameState::Playing => {
                self.game_state = GameState::Paused;
                // self.background_music.pause();
            }
            Key::R if self.game_state == GameState::Paused => {
                self.game_state = GameState::Playing;
                // self.background_music.play();
            }
            Key::N => {
                self.tetris.reset();
                self.game_state = GameState::Playing;
                // self.background_music.stop();
                // self.background_music.play();
            }
            _ => {}
        }
    }

    fn handle_draw(&mut self, c: &Context, g: &mut G2d) {
        if self.game_state == GameState::GameOver {
            clear(color::WHITE, g);
            text(NEW_RED,
                 FONT_SIZE,
                 self.message,
                 &mut self.cache,
                 c.transform.trans(LEFT_MARGIN, TOP_MARGIN),
                 g);
        } else {
            clear(color::grey(0.6), g);

            // for p in &self.tetris.ghost_shape() {
            //     ellipse(color::grey(0.8), CELL_SQUARE,
            //         c.transform.trans(p.0 as f64 * CELL_SIZE, p.1 as f64 * CELL_SIZE), g);
            // }

            text(color::BLACK,
                 FONT_SIZE,
                 &format!("Score: {}", self.tetris.score()),
                 &mut self.cache,
                 c.transform.trans((self.tetris.width() / 2) as f64 * CELL_SIZE + LEFT_MARGIN,
                                   TOP_MARGIN),
                 g);

            for p in &self.tetris.next_shape() {
                rectangle(NEW_RED,
                          CELL_SQUARE,
                          c.transform.trans((p.0 as f64 + (self.tetris.width() / 4 * 3) as f64) *
                                            CELL_SIZE,
                                            p.1 as f64 * CELL_SIZE + TOP_MARGIN +
                                            FONT_SIZE as f64),
                          g);
            }

            rectangle([0.5, 0.0, 0.5, 0.5],
                      [0.0,
                       0.0,
                       (self.tetris.width() / 2) as f64 * CELL_SIZE,
                       self.tetris.height() as f64 * CELL_SIZE],
                      c.transform,
                      g);

            for p in &self.tetris.now_shape() {
                rectangle(NEW_BLUE,
                          CELL_SQUARE,
                          c.transform.trans(p.0 as f64 * CELL_SIZE, p.1 as f64 * CELL_SIZE),
                          g);
            }

            for p in self.tetris.bottom() {
                rectangle(color::BLACK,
                          CELL_SQUARE,
                          c.transform.trans(p.0 as f64 * CELL_SIZE, p.1 as f64 * CELL_SIZE),
                          g);
            }
        }

        if self.game_state != GameState::Playing {
            // clear(color::WHITE, g);
            text(color::BLACK,
                 FONT_SIZE,
                 "Press `N` to begin a New game",
                 &mut self.cache,
                 c.transform.trans(LEFT_MARGIN, TOP_MARGIN + TEXT_HEIGHT * 3.0),
                 g);
            text(color::BLACK,
                 FONT_SIZE,
                 "Press `P` to Pause",
                 &mut self.cache,
                 c.transform.trans(LEFT_MARGIN, TOP_MARGIN + TEXT_HEIGHT * 4.0),
                 g);
            text(color::BLACK,
                 FONT_SIZE,
                 "Press `R` to Resume",
                 &mut self.cache,
                 c.transform.trans(LEFT_MARGIN, TOP_MARGIN + TEXT_HEIGHT * 5.0),
                 g);
            text(color::BLACK,
                 FONT_SIZE,
                 "Press `ESC` to Exit",
                 &mut self.cache,
                 c.transform.trans(LEFT_MARGIN, TOP_MARGIN + TEXT_HEIGHT * 6.0),
                 g);
        }
    }

    fn handle_update(&mut self, _args: &UpdateArgs) {
        if self.game_state == GameState::Playing {
            match self.tetris.move_on() {
                Ok(_) => {}
                Err(message) => {
                    self.game_state = GameState::GameOver;
                    self.message = message;
                    // self.background_music.stop();
                }
            }
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("my_tetris", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_ups(WINDOW_UPS);

    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Bold.ttf");

    let mut app = App {
        tetris: Tetris::new_with_bounds(WINDOW_WIDTH / (CELL_SIZE as u32),
                                        WINDOW_HEIGHT / (CELL_SIZE as u32)),
        game_state: GameState::Init,
        message: "no message",
        cache: Glyphs::new(&font_path, window.factory.clone()).unwrap(), 
        // background_music: Music::new("assets/sound/korobeiniki.ogg").unwrap(), /* shoot_sound: Sound::new("assets/sound/korobeiniki.ogg").unwrap(), */
    };
    // app.background_music.set_looping(true);
    // app.background_music.set_volume(0.7);

    while let Some(e) = window.next() {
        e.press(|button| if let Button::Keyboard(k) = button {
            app.handle_press(&k);
        });

        window.draw_2d(&e, |c, g| { app.handle_draw(&c, g); });

        e.update(|args| { app.handle_update(args); });
    }
}
