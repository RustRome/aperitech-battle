extern crate cfg_if;
extern crate wasm_bindgen;

extern crate js_sys;

extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::Window;

use js_sys::Math;

use std::collections::LinkedList;

mod utils;

use cfg_if::cfg_if;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

const GRID_SIZE: u32 = 400;
const SQUARE_SIZE: u32 = 10;

#[wasm_bindgen]
pub struct Game {
    context: web_sys::CanvasRenderingContext2d,
    food: Food,
    snake: Snake,
}

#[wasm_bindgen]
impl Game {
    pub fn new(elem: String) -> Game {
        let window: Window = web_sys::window().expect("Cannot get a window");
        let document = window.document().expect("Cannot get a document");
        let canvas = document.get_element_by_id(&elem).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let food = Food::random();
        let snake = Snake::new();
        let game = Game {
            context,
            food,
            snake,
        };

        game.render();

        game
    }

    fn render(&self) {
        self.food.render(&self.context);
        self.snake.render(&self.context);
    }
    pub fn tick(&mut self, key: i32) {
        let direction = match key {
            37 => Direction::Left,
            38 => Direction::Up,
            39 => Direction::Right,
            40 => Direction::Down,
            _ => self.snake.direction.clone(),
        };

        let direction = match self.snake.direction.opposite(&direction) {
            true => self.snake.direction.clone(),
            false => direction,
        };
        self.snake.tick(direction);

        match self.detect_collision(self.snake.head()) {
            Collision::None => {
                let tail = self.snake.shrink_tail();
                clear_rect(&self.context, &tail);
            }
            Collision::Food => {
                clear_rect(&self.context, &self.food.location);
                self.food = Food::random();
            }
            Collision::Snake => {
                alert(&format!("You Bit Yourself!"));

                self.snake
                    .body
                    .iter()
                    .for_each(|e| clear_rect(&self.context, e));
                self.snake = Snake::new();
            }
        }

        self.render();
    }

    fn detect_collision(&self, head: &(u32, u32)) -> Collision {
        let self_hitting = self
            .snake
            .body
            .iter()
            .skip(1)
            .any(|segment| head == segment);

        if self_hitting {
            return Collision::Snake;
        }

        if &self.food.location == head {
            return Collision::Food;
        }
        Collision::None
    }
}

struct Food {
    location: (u32, u32),
}

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self, other: &Direction) -> bool {
        self == &Direction::Up && other == &Direction::Down
            || self == &Direction::Down && other == &Direction::Up
            || self == &Direction::Left && other == &Direction::Right
            || self == &Direction::Right && other == &Direction::Left
    }
}

impl Food {
    fn random() -> Food {
        Food {
            location: (random_coord(), random_coord()),
        }
    }

    fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        let style = ctx.fill_style();
        ctx.set_fill_style(&"red".into());
        ctx.fill_rect(
            self.location.0 as f64,
            self.location.1 as f64,
            SQUARE_SIZE as f64,
            SQUARE_SIZE as f64,
        );
        ctx.set_fill_style(&style);
    }
}

struct Snake {
    direction: Direction,
    body: LinkedList<(u32, u32)>,
}

impl Snake {
    pub fn new() -> Snake {
        let mut body = LinkedList::new();
        body.push_back(((GRID_SIZE / 2), (GRID_SIZE / 2)));
        body.push_back(((GRID_SIZE / 2), ((GRID_SIZE / 2) + 1)));
        body.push_back(((GRID_SIZE / 2), ((GRID_SIZE / 2) + 2)));
        Snake {
            body: body,
            direction: Direction::Up,
        }
    }

    fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        let style = ctx.fill_style();
        ctx.set_fill_style(&"green".into());

        self.body.iter().for_each(|e| {
            draw_rect(ctx, e);
        });

        ctx.set_fill_style(&style);
    }

    fn head(&self) -> &(u32, u32) {
        self.body.front().unwrap()
    }
    fn tick(&mut self, direction: Direction) {
        self.direction = direction;

        let new_head = {
            let head = self.body.front().unwrap();
            match self.direction {
                Direction::Up => {
                    if head.1 == 0 {
                        (head.0 % GRID_SIZE, GRID_SIZE)
                    } else {
                        (head.0 % GRID_SIZE, (head.1 - SQUARE_SIZE) % GRID_SIZE)
                    }
                }
                Direction::Down => (head.0 % GRID_SIZE, (head.1 + SQUARE_SIZE) % GRID_SIZE),
                Direction::Left => {
                    if head.0 == 0 {
                        (GRID_SIZE, head.1 % GRID_SIZE)
                    } else {
                        ((head.0 - SQUARE_SIZE) % GRID_SIZE, head.1 % GRID_SIZE)
                    }
                }
                Direction::Right => ((head.0 + SQUARE_SIZE) % GRID_SIZE, head.1 % GRID_SIZE),
            }
        };

        self.body.push_front(new_head);
    }

    pub fn shrink_tail(&mut self) -> (u32, u32) {
        self.body.pop_back().unwrap()
    }
}

fn clear_rect(ctx: &web_sys::CanvasRenderingContext2d, elem: &(u32, u32)) {
    ctx.clear_rect(
        elem.0 as f64,
        elem.1 as f64,
        SQUARE_SIZE as f64,
        SQUARE_SIZE as f64,
    );
}

fn draw_rect(ctx: &web_sys::CanvasRenderingContext2d, elem: &(u32, u32)) {
    ctx.fill_rect(
        elem.0 as f64,
        elem.1 as f64,
        SQUARE_SIZE as f64,
        SQUARE_SIZE as f64,
    );
}
fn random_coord() -> u32 {
    return (Math::floor((Math::random() * GRID_SIZE as f64) / SQUARE_SIZE as f64)
        * SQUARE_SIZE as f64) as u32;
}
enum Collision {
    Snake,
    Food,
    None,
}
