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

use std::mem;

use graphics::{ Context, Transformed, image, clear, rectangle };
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use piston::event_loop::{ Events, EventLoop, EventSettings };
use piston::input::{ Button, RenderEvent, PressEvent, Input };
use piston::input::keyboard::Key;
use piston::window::WindowSettings;

use models::{ Direction, Grid, Movement, Tetrimino, Tetriminos };


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
    state: States,
    level: u8,
    ticks: u8,

    img: Texture,
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
                        self.reset_ticks();
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
        let ticks = self.ticks;
        if ticks > 0 {
            self.ticks -= 1;
        } else {
            match self.state {
                States::Locking => {
                    let mut other = self.tetriminos.next().unwrap();
                    mem::swap(&mut other, &mut self.active);
                    self.grid.lock(other);
                    self.state = States::Clearing;
                },
                States::Clearing => {
                    self.grid.clear_full_rows();
                    self.state = States::Falling;
                },
                _ => {},
            }
            self.on_move(Movement::Shift(Direction::Down));
            self.reset_ticks();
        }
    }

    fn draw_well(&mut self, c: &Context, gl: &mut GlGraphics) {
        const BLACKISH: [f32; 4] = [0.05, 0.05, 0.05, 1.0];
        const CELL_SIZE: f64 = 40.0;

        let active_blocks = self.active.blocks();
        let base_blocks = self.grid.blocks();
        let blocks = active_blocks.iter().chain(base_blocks.iter());
        let height = self.grid.height;
        let shade = &self.img;

        rectangle(BLACKISH, [0.0, 0.0, 400.0, 800.0], c.transform, gl);

        for block in blocks {
            let x_cell = block.x as f64;
            let y_cell = height as f64 - block.y as f64;
            let x_pos = 0.0f64 + (x_cell * CELL_SIZE);
            let y_pos = 0.0f64 + (y_cell * CELL_SIZE);
            let color = block.color.clone();

            rectangle(color, [x_pos, y_pos, CELL_SIZE, CELL_SIZE], c.transform, gl);
            image(shade, c.transform.trans(x_pos, y_pos), gl);
        }
    }

    fn on_render(&mut self, e: &Input, gl: &mut GlGraphics) {
        const GRAY: [f32; 4] = [0.4, 0.4, 0.4, 1.0];

        let args = e.render_args().unwrap();

        gl.draw(args.viewport(), |c, gl| {
            clear(GRAY, gl);

            self.draw_well(&c, gl);
        });
    }

    fn reset_ticks(&mut self) {
        self.ticks = 23;
    }

    pub fn run() {
        let opengl = OpenGL::V3_2;
        let mut window: Window = WindowSettings::new(
            "tetris",
            [1000, 1000])
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let mut tetriminos = Tetriminos::init();
        let active = tetriminos.next().unwrap();
        let mut game = Game {
            grid: Grid::new(20, 10),
            tetriminos,
            active,
            level: 1,
            ticks: 23,
            state: States::Falling,

            img: Texture::from_path("assets/shade.png").unwrap(),
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
