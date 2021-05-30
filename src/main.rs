use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use std::collections::VecDeque;
use std::time::Duration;

const BOX_SIZE: i32 = 512;
const GRID_WIDTH: i32 = 32;
const GRID_HEIGHT: i32 = 32;
const CELL_SIZE: i32 = 16;

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Noop,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Cell {
    is_food: bool,
    is_part_of_snake: bool,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Snake {}

impl Cell {
    const fn new() -> Self {
        Self {
            is_food: false,
            is_part_of_snake: false,
            direction: Direction::Noop,
        }
    }

    fn set_is_food(&mut self, state: bool) {
        self.is_food = state;
    }

    fn set_is_part_of_snake(&mut self, state: bool) {
        self.is_part_of_snake = state;
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}

#[derive(Debug, Clone)]
struct Game {
    board: Vec<Vec<Cell>>,
    snake: VecDeque<(usize, usize)>,
    current_direction: Direction,
    food: (usize, usize),
}

impl Game {
    fn new() -> Self {
        let mut board = vec![vec![Cell::new(); GRID_WIDTH as usize]; GRID_HEIGHT as usize];
        let (random_food_x, random_food_y) = Self::get_food_location();
        board[random_food_x][random_food_y].set_is_food(true);
        println!("Food Coords: ({}, {})", random_food_x, random_food_y);

        let mut rng = thread_rng();
        let snake_start_x = rng.gen_range(3..GRID_WIDTH - 3) as usize;
        let snake_start_y = rng.gen_range(3..GRID_HEIGHT - 3) as usize;

        println!("Snake Coords: ({}, {})", snake_start_x, snake_start_y);
        let mut snake = VecDeque::new();
        for i in 0..3 {
            let start_x = snake_start_x + i;
            board[start_x][snake_start_y].set_is_part_of_snake(true);
            board[start_x][snake_start_y].set_direction(Direction::Left);
            snake.push_back((start_x, snake_start_y));
        }

        Self {
            board,
            snake,
            current_direction: Direction::Left,
            food: (random_food_x, random_food_y),
        }
    }

    fn get_food_location() -> (usize, usize) {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..GRID_WIDTH) as usize;
        let y = rng.gen_range(0..GRID_HEIGHT) as usize;

        (x, y)
    }

    fn render(&self, canvas: &mut WindowCanvas, color: Color) -> Result<(), String> {
        canvas.clear();
        canvas.set_draw_color(Color::GREY);
        let (window_width, window_height) = canvas.output_size()?;
        let window_rect = Rect::new(0, 0, window_width, window_height);
        canvas.fill_rect(window_rect)?;
        let blue_box = Rect::from_center(
            (window_width as i32 / 2, window_height as i32 / 2),
            BOX_SIZE as u32,
            BOX_SIZE as u32,
        );
        canvas.set_draw_color(color);
        canvas.fill_rect(blue_box)?;
        self.draw_board(canvas, blue_box.top_left())?;
        canvas.present();

        Ok(())
    }

    fn draw_board(&self, canvas: &mut WindowCanvas, offset: Point) -> Result<(), String> {
        for row in 0..GRID_HEIGHT {
            for col in 0..GRID_WIDTH {
                canvas.set_draw_color(Color::BLACK);
                let cell = &self.board[row as usize][col as usize];
                let cell_rect = Rect::new(
                    offset.x() + (row * CELL_SIZE),
                    offset.y() + (col * CELL_SIZE),
                    CELL_SIZE as u32,
                    CELL_SIZE as u32,
                );

                if cell.is_food {
                    canvas.set_draw_color(Color::RED);
                    canvas.fill_rect(cell_rect)?;
                } else if cell.is_part_of_snake {
                    canvas.set_draw_color(Color::GREEN);
                    canvas.fill_rect(cell_rect)?;
                } else {
                    canvas.draw_rect(cell_rect)?;
                }
            }
        }
        Ok(())
    }

    fn move_snake(&mut self, direction: Direction) {
        self.current_direction = direction;
    }

    fn calculate_new_head(&self, direction: &Direction) -> (usize, usize) {
        let (head_x, head_y) = self.snake.front().unwrap();
        match direction {
            Direction::Up => {
                let new_y = if *head_y > 0 { *head_y - 1 } else { 0 };
                (*head_x, new_y)
            }
            Direction::Down => {
                let new_y = if *head_y + 1 < GRID_HEIGHT as usize {
                    *head_y + 1
                } else {
                    GRID_HEIGHT as usize - 1
                };
                (*head_x, new_y)
            }
            Direction::Left => {
                let new_x = if *head_x > 0 { *head_x - 1 } else { 0 };
                (new_x, *head_y)
            }
            Direction::Right => {
                let new_x = if *head_x + 1 < GRID_WIDTH as usize {
                    *head_x + 1
                } else {
                    GRID_WIDTH as usize
                };
                (new_x, *head_y)
            }
            _ => (*head_x, *head_y),
        }
    }

    fn handle_events(&mut self, event_pump: &mut EventPump) -> Result<bool, String> {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    return Ok(true);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    println!("Up");
                    if self.current_direction != Direction::Down {
                        self.move_snake(Direction::Up);
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    println!("Down");
                    if self.current_direction != Direction::Up {
                        self.move_snake(Direction::Down);
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    println!("Left");
                    if self.current_direction != Direction::Right {
                        self.move_snake(Direction::Left);
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    println!("Right");
                    if self.current_direction != Direction::Left {
                        self.move_snake(Direction::Right);
                    }
                }

                _ => {}
            }
        }

        Ok(false)
    }

    fn update(&mut self) {
        println!("Update...");
        let (new_x, new_y) = self.calculate_new_head(&self.current_direction);
        let (tail_x, tail_y) = self.snake.pop_back().unwrap();
        self.board[tail_x][tail_y].set_is_part_of_snake(false);

        self.board[new_x][new_y].set_is_part_of_snake(true);
        self.snake.push_front((new_x, new_y));

        // Check for collision with food and grow snake if it found food.
        let (head_x, head_y) = self.snake.front().unwrap();
        if self.food.0 == *head_x && self.food.1 == *head_y {
            println!("Got Food...");
            let (new_food_x, new_food_y) = Self::get_food_location();
            self.board[self.food.0][self.food.1].set_is_food(false);
            self.board[new_food_x][new_food_y].set_is_food(true);
            self.food = (new_food_x, new_food_y);
            self.grow_snake();
        }
    }

    fn grow_snake(&mut self) {
        let (new_head_x, new_head_y) = self.calculate_new_head(&self.current_direction);
        self.board[new_head_x][new_head_y].set_is_part_of_snake(true);
        self.snake.push_back((new_head_x, new_head_y));
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let mut game = Game::new();

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // Handle events
        let end = game.handle_events(&mut event_pump)?;
        if end {
            break 'running;
        }

        // Update
        game.update();

        // Render
        // game.render(&mut canvas, Color::RGB(i, 64, 255 - i))?;
        game.render(&mut canvas, Color::RGB(0, 100, 200))?;

        // Time management!
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        ::std::thread::sleep(Duration::new(0, 500_000_000));
    }

    Ok(())
}
