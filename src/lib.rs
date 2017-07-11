// TODO: Index of bottom row with cells is 1-based. Gross, fix.
// TODO: Maybe cut down on `Vec.clone()`s
// TODO: Un-stub level/locking ticks
// TODO: Get rid of any dead code
// TODO: Use different ticks for gravity/locking/clearing
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

#[macro_use]
mod macros;
mod models;

use std::cmp::{max, min};
use std::mem;
use std::path::Path;

use graphics::{ Context, Text, Transformed, image, clear, rectangle };
use graphics::character::CharacterCache;
use graphics::rectangle::{ Border, Rectangle };
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event_loop::{ Events, EventLoop, EventSettings };
use piston::input::{ Button, RenderEvent, PressEvent, Input };
use piston::input::keyboard::Key;
use piston::window::WindowSettings;

use models::{ Direction, Grid, Movement, Tetrimino, Tetriminos };


const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const GRAY: [f32; 4] = [0.4, 0.4, 0.4, 1.0];
const BLACKISH: [f32; 4] = [0.05, 0.05, 0.05, 1.0];

const CELL_SIZE: f64 = 40.0;

#[derive(Eq, PartialEq)]
pub enum States {
    Falling,
    Clearing,
    Locking,
}


pub struct Game {
    grid: Grid,
    tetriminos: Tetriminos,
    active: Tetrimino,
    peeked: Tetrimino,
    state: States,
    score: u32,
    level: u8,
    fall_ticks: u8,
    lock_ticks: u8,
    clear_ticks: u8,
    lines: u32,

    img: Texture,
    cache: GlyphCache<'static>,
}


impl Game {
    fn on_move(&mut self, movement: Movement) {
        match self.state {
            States::Falling | States::Locking => {
                let next = self.active.peek(&movement);
                if !self.grid.is_legal(&next) {
                    return;
                }
                match movement {
                    Movement::Rotate => self.active.rotate(&self.grid),
                    Movement::Shift(direction) => self.active.shift(direction, &self.grid),
                };
                let has_landed = self.grid.has_landed(&self.active);
                if has_landed {
                    self.state = States::Locking;
                } else {
                    if self.state == States::Locking {
                        self.reset_lock_ticks();
                    }
                    self.state = States::Falling;
                }
            },
            _ => {},
        }
    }

    fn on_press(&mut self, e: &Input) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up => self.on_move(Movement::Rotate),
                Key::Down => self.on_move(Movement::Shift(Direction::Down)),
                Key::Left => self.on_move(Movement::Shift(Direction::Left)),
                Key::Right => self.on_move(Movement::Shift(Direction::Right)),
                _ => {},
            }
        }
    }

    fn on_update(&mut self) {
        match self.state {
            States::Locking => {
                let ticks = self.lock_ticks;
                if ticks > 0 {
                    self.lock_ticks -= 1;
                } else {
                    let mut other = self.tetriminos.next().unwrap();
                    let peeked = self.tetriminos.peek();
                    mem::swap(&mut other, &mut self.active);
                    self.grid.lock(other);
                    self.peeked = peeked;
                    self.state = States::Clearing;
                    self.reset_lock_ticks();
                    self.reset_fall_ticks();
                }
            },
            States::Clearing => {
                let ticks = self.clear_ticks;
                let num_full_rows = self.grid.get_full_rows().len();
                if num_full_rows == 0 {
                    self.state = States::Falling;
                } else if ticks > 0 {
                    self.clear_ticks -= 1;
                } else {
                    let cleared = self.grid.clear_full_rows();
                    self.update_score(cleared);
                    self.lines += cleared;
                    self.state = States::Falling;
                    self.update_level();
                    self.reset_clear_ticks();
                }
            },
            States::Falling => {
                let ticks = self.fall_ticks;
                if ticks > 0 {
                    self.fall_ticks -= 1;
                } else {
                    self.on_move(Movement::Shift(Direction::Down));
                    self.reset_fall_ticks();
                }
            },
        }
    }

    fn draw_well(&mut self, c: &Context, gl: &mut GlGraphics) {
        let full_rows = self.grid.get_full_rows();
        let active_blocks = self.active.blocks();
        let base_blocks = self.grid.blocks();
        let blocks = active_blocks.iter()
            .chain(base_blocks.iter())
            .filter(|&block| {
                if self.state == States::Clearing {
                    if self.clear_ticks % 8 < 4 {
                        return !full_rows.contains(&block.y);
                    }
                }
                true
            });
        let height = self.grid.height;
        let shade = &self.img;

        rectangle(BLACKISH, [50.0, 0.0, 400.0, 800.0], c.transform, gl);

        for block in blocks {
            let x_cell = block.x as f64;
            let y_cell = height as f64 - block.y as f64;
            let x_pos = 50.0f64 + (x_cell * CELL_SIZE);
            let y_pos = 0.0f64 + (y_cell * CELL_SIZE);
            let color = block.color.clone();

            rectangle(color, [x_pos, y_pos, CELL_SIZE, CELL_SIZE], c.transform, gl);
            image(shade, c.transform.trans(x_pos, y_pos), gl);
        }
    }

    fn draw_preview(&mut self, c: &Context, gl: &mut GlGraphics) {
        let peeked_blocks = self.peeked.blocks();
        let shade = &self.img;

        let preview_rect = Rectangle::new(BLACKISH).border(
            Border {
                color: WHITE,
                radius: 3.0,
            });
        preview_rect.draw([500.0, 550.0, 240.0, 200.0], &c.draw_state,
                           c.transform, gl);

        for block in &peeked_blocks {
            let x_cell= (block.x - 2) as f64;
            let y_cell = 21.0 - block.y as f64;
            let x_pos = 500.0f64 + (x_cell * CELL_SIZE);
            let y_pos = 590.0f64 + (y_cell * CELL_SIZE);
            let color = block.color.clone();

            rectangle(color, [x_pos, y_pos, CELL_SIZE, CELL_SIZE], c.transform, gl);
            image(shade, c.transform.trans(x_pos, y_pos), gl);
        }
    }


    fn draw_textbox(&mut self, label: &str, val: &str, x: f64, y: f64,
                    c: &Context, gl: &mut GlGraphics) {

        let ref mut font = self.cache;
        let val_rect = Rectangle::new(BLACKISH).border(
            Border {
                color: WHITE,
                radius: 3.0,
            });
        val_rect.draw([x, y, 200.0, 80.0], &c.draw_state,
                      c.transform, gl);

        let val_text = Text::new_color(WHITE, 40);
        let val_width = font.width(40, &val);
        let val_x_off = (200.0 - val_width) / 2.0;
        let val_y_off = 60.0;
        let text_trans = c.transform.trans(x + val_x_off, y + val_y_off);
        val_text.draw(&val, font, &c.draw_state, text_trans, gl);

        let label_x = x + 50.0;
        let label_y = y - 15.0;
        let label_rect = Rectangle::new(BLACKISH).border(
            Border {
                color: WHITE,
                radius: 1.0,
            });
        label_rect.draw([label_x, label_y, 100.0, 30.0], &c.draw_state,
                        c.transform, gl);

        let label_text = Text::new_color(WHITE, 20);
        let label_width = font.width(20, &label);
        let label_x_off = (100.0 - label_width) / 2.0;
        let label_y_off = 25.0;
        let label_trans = c.transform.trans(label_x + label_x_off,
                                            label_y + label_y_off);
        label_text.draw(&label, font, &c.draw_state, label_trans, gl);
    }


    fn draw_score(&mut self, c: &Context, gl: &mut GlGraphics) {
        let score = format!("{:0>6}", self.score);
        self.draw_textbox("SCORE", &score, 520.0, 50.0, c, gl);
    }


    fn draw_lines(&mut self, c: &Context, gl: &mut GlGraphics) {
        let lines = format!("{:0>4}", self.lines);
        self.draw_textbox("LINES", &lines, 520.0, 200.0, c, gl);
    }

    fn draw_level(&mut self, c: &Context, gl: &mut GlGraphics) {
        let level = format!("{:0>2}", self.level);
        self.draw_textbox("LEVEL", &level, 520.0, 350.0, c, gl);
    }


    fn on_render(&mut self, e: &Input, gl: &mut GlGraphics) {
        let args = e.render_args().unwrap();

        gl.draw(args.viewport(), |c, gl| {
            clear(GRAY, gl);

            self.draw_well(&c, gl);
            self.draw_preview(&c, gl);
            self.draw_score(&c, gl);
            self.draw_lines(&c, gl);
            self.draw_level(&c, gl);
        });
    }

    fn update_level(&mut self) {
        let lines = self.lines;
        self.level = max(self.level, min(lines / 10, 20) as u8);
    }

    fn update_score(&mut self, num_rows_cleared: u32) {
        let l = (self.level + 1) as u32;
        match num_rows_cleared {
            1 => { self.score += 40u32 * l; },
            2 => { self.score += 100u32 * l; },
            3 => { self.score += 300u32 * l; },
            4 => { self.score += 1200u32 * l; },
            _ => {},
        }
    }

    fn reset_fall_ticks(&mut self) {
        self.fall_ticks = match self.level {
            0 => 53,
            1 => 49,
            2 => 45,
            3 => 41,
            4 => 37,
            5 => 33,
            6 => 28,
            7 => 22,
            8 => 17,
            9 => 11,
            10 => 10,
            11 => 9,
            12 => 8,
            13 => 7,
            14 => 6,
            15 => 6,
            16 => 5,
            17 => 5,
            18 => 4,
            19 => 4,
            20 => 3,
            _ => panic!("illegal level"),
        }
    }

    fn reset_lock_ticks(&mut self) {
        self.lock_ticks = 10;
    }

    fn reset_clear_ticks(&mut self) {
        self.clear_ticks = 93;
    }

    pub fn run(start_level: u8) {
        let opengl = OpenGL::V3_2;
        let font_path = Path::new("assets/Verdana.ttf");
        let mut window: Window = WindowSettings::new(
            "tetris",
            [800, 800])
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let mut tetriminos = Tetriminos::init();
        let active = tetriminos.next().unwrap();
        let peeked = tetriminos.peek();
        let mut game = Game {
            grid: Grid::new(20, 10),
            tetriminos,
            active,
            peeked,
            level: start_level,
            fall_ticks: 53,
            lock_ticks: 10,
            clear_ticks: 48,
            score: 0,
            lines: 0,
            state: States::Falling,

            img: Texture::from_path("assets/shade.png").unwrap(),
            cache: GlyphCache::new(font_path).unwrap(),
        };
        let ref mut gl = GlGraphics::new(opengl);

        let mut settings = EventSettings::new();
        settings.set_ups(60);
        settings.set_max_fps(60);
        let mut events = Events::new(settings);
        while let Some(e) = events.next(&mut window) {
            match e {
                Input::Render(_) => game.on_render(&e, gl),
                Input::Press(_) => game.on_press(&e),
                Input::Update(_) => game.on_update(),
                _ => {},
            }
        }
    }
}
