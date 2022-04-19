use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: usize);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);

}
#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern "C" {
    fn rnd(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[wasm_bindgen]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}

#[derive(PartialEq, Clone, Copy)]
pub struct SnakeCell(usize);

struct Snake {
    pub body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(span_index: usize, size: usize) -> Snake {
        let mut body = vec![];
        for i in 0..size {
            body.push(SnakeCell(span_index + i))
        }
        Snake {
            body,
            direction: Direction::Down,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: Option<usize>,
    status: Option<GameStatus>,
    points: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_spawn_idx: usize) -> World {
        let size = width * width;
        let snake = Snake::new(snake_spawn_idx, 3);

        World {
            width,
            size,
            reward_cell: World::gen_reward_cell(size, &snake.body),
            snake,
            next_cell: None,
            status: None,
            points: 0,
        }
    }
    fn gen_reward_cell(max: usize, snake_body: &[SnakeCell]) -> Option<usize> {
        let mut reward_cell;
        loop {
            reward_cell = rnd(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                return Some(reward_cell);
            }
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);
        if self.snake.body.len() > 1 && self.snake.body[1].0 == next_cell.0 {
            return;
        }
        self.next_cell = Some(next_cell);
        self.snake.direction = direction
    }

    pub fn get_points(&self) -> usize {
        self.points
    }

    pub fn get_snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn get_snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn get_reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Played);
    }

    pub fn get_games_status(self) -> Option<GameStatus> {
        self.status
    }
    pub fn get_games_status_text(self) -> String {
        match self.status {
            Some(GameStatus::Won) => String::from("Won"),
            Some(GameStatus::Lost) => String::from("Lost"),
            Some(GameStatus::Played) => String::from("Playing"),
            None => String::from("No status"),
        }
    }

    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Played) => {
                for i in (1..self.get_snake_length()).rev() {
                    self.snake.body[i] = SnakeCell(self.snake.body[i - 1].0)
                }

                match self.next_cell {
                    Some(next_cell) => {
                        self.snake.body[0] = next_cell;
                        self.next_cell = None
                    }
                    None => self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction),
                };

                if self.snake.body[1..self.get_snake_length()].contains(&self.snake.body[0]) {
                    return self.status = Some(GameStatus::Lost);
                }

                if self.reward_cell == Some(self.snake_head_idx()) {
                    if self.get_snake_length() < self.size {
                        self.points += 1;
                        self.reward_cell = World::gen_reward_cell(self.size, &self.snake.body);
                    } else {
                        self.reward_cell = None;
                        self.status = Some(GameStatus::Won);
                    }
                    self.snake.body.push(SnakeCell(self.snake_head_idx()));
                }
            }
            Some(_) => {}
            None => {}
        }
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

        match direction {
            Direction::Right => SnakeCell((row * self.width) + (snake_idx + 1) % self.width),
            Direction::Left => SnakeCell((row * self.width) + (snake_idx - 1) % self.width),
            Direction::Up => SnakeCell((snake_idx - self.width) % self.size),
            Direction::Down => SnakeCell((snake_idx + self.width) % self.size),
        }
    }
}
