extern crate ggez;
extern crate rand;

const SNAKE_INIT_POS: (i16, i16) = (5, 5);
const FRUIT_INIT_POS: (i16, i16) = (10, 10);

const PIXEL_SIZE: (i16, i16) = (20, 20);
const SIZE_IN_PIXELS: (i16, i16) = (20, 20);

const DEFAULT_FPS: i16 = 8;

const DEFAULT_ACCEL: i16 = 1;

const SCREEN_SIZE: (f32, f32) = (
    (PIXEL_SIZE.0 * SIZE_IN_PIXELS.0) as f32,
    (PIXEL_SIZE.1 * SIZE_IN_PIXELS.1) as f32
);

use ggez::{event, Context, ContextBuilder, GameResult, graphics};
use rand::Rng;

struct Game {
    snake: Snake,
    fruit: Fruit,
    walls: Vec<Wall>,
}

impl Game {
    pub fn new() -> Self {
        let mut walls = Vec::<Wall>::new();
        walls.push(Wall::new(10, 10, 3));
        Self {
            snake: Snake::new(SNAKE_INIT_POS.0, SNAKE_INIT_POS.1),
            fruit: Fruit::new(FRUIT_INIT_POS.0, FRUIT_INIT_POS.1),
            walls: walls,
        }
    }

    fn opposite(&self, direction: Direction) -> bool {
        (self.snake.direction == Direction::Up && direction == Direction::Down) 
        || (self.snake.direction == Direction::Down && direction == Direction::Up)
        || (self.snake.direction == Direction::Left && direction == Direction::Right)
        || (self.snake.direction == Direction::Right && direction == Direction::Left)        
    }
}

impl event::EventHandler for Game {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        ggez::graphics::clear(ctx, (0, 0, 0).into());

        self.fruit.draw(ctx)?;
        self.snake.draw(ctx)?;
        for wall in &self.walls {
            (*wall).draw(ctx);
        }

        ggez::graphics::present(ctx);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, DEFAULT_FPS as u32) {
            self.snake.update(&self.fruit)?;

            match self.snake.state {
                Some(SnakeAction::AteFruit) => self.fruit.regenerate_outside_walls(&self.walls),
                Some(SnakeAction::SelfCollision) => self.snake.reset(),
                _ => (),
            }
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: ggez::event::KeyCode,
        _keymod: ggez::event::KeyMods,
        _repeat: bool,
    ) {
        // if keycode == ggez::event::Escape {
        //     // Game::gameover(ctx);
        // }

        if let Some(direction) = Direction::from_keycode(keycode) {
            if !self.opposite(direction) {
                self.snake.direction = direction;
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_keycode(key: event::KeyCode) -> Option<Direction> {
        match key {
            event::KeyCode::Up => Some(Direction::Up),
            event::KeyCode::Down => Some(Direction::Down),
            event::KeyCode::Left => Some(Direction::Left),
            event::KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }

    pub fn random_direction() -> Direction {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..5);
        let mut result = Direction::Up;

        if n <= 1 {
            result = Direction::Left;
        } else if n <= 2 {
            result = Direction::Right;
        } else if n <= 3 {
            result = Direction::Down;
        }

        result
    }
}

#[derive(PartialEq, Copy, Clone)]
struct Position {
    x: i16,
    y:i16,
}

impl Position {
    pub fn new(x: i16, y: i16) -> Self {
        Self {x, y}
    }

    pub fn new_by_direction(x: i16, y: i16, direction: Direction) -> Self {
        let accel = DEFAULT_ACCEL;

        let (mut x, mut  y) = match direction {
            Direction::Up => (x, y - accel),
            Direction::Down => (x, y + accel),
            Direction::Left => (x - accel, y),
            Direction::Right => (x + accel, y),
        };

        if x < 0 {
            x = PIXEL_SIZE.0 - 1;
        } else if x > PIXEL_SIZE.0 - 1 {
            x = 0;
        }

        if y < 0 {
            y = PIXEL_SIZE.1 - 1;
        } else if y > PIXEL_SIZE.1 - 1 {
            y = 0;
        }
        
        Self {x, y}
    }
}

impl From<Position> for graphics::Rect {
    fn from(pos: Position) -> Self {
        graphics::Rect::new_i32(
            (pos.x * PIXEL_SIZE.0).into(),
            (pos.y * PIXEL_SIZE.1).into(),
            (PIXEL_SIZE.0 - 1).into(),
            (PIXEL_SIZE.1 - 1).into(),
        )
    }
}

struct Wall {
    blocks: Vec<Position>,
}

impl Wall {
    pub fn new(x: i16, y: i16, size: i16) -> Self {
        let mut blocks = Vec::<Position>::new();
        blocks.push(Position::new(x, y));
        let mut x_counter = x;
        let mut y_counter = y;
        for i in 1..size {
            let direction = Direction::random_direction();
            let position = Position::new_by_direction(x_counter, y_counter, direction);
            blocks.push(position);
            x_counter = position.x;
            y_counter = position.y;
        }

        Wall{blocks: blocks}
    }

    pub fn contains_position(&self, position: Position) -> bool {
        self.blocks.contains(&position)
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // let rect = graphics::Rect::new_i32(
        //     (self.head.x).into(),
        //     (self.head.y).into(),
        //     PIXEL_SIZE.0.into(),
        //     PIXEL_SIZE.1.into(),
        // );

        let draw_mode = graphics::DrawMode::Fill(graphics::FillOptions::default());
        let green = graphics::Color::from_rgb(176,224,230);

        for segment in &self.blocks {
            let outline = graphics::MeshBuilder::new()
                .rectangle(draw_mode, (*segment).into(), green)
                .unwrap()
                .build(ctx)
                .unwrap();
            graphics::draw(ctx, &outline, graphics::DrawParam::default())?;
        }
        
        Ok(())
    }
}

enum SnakeAction {
    SelfCollision,
    AteFruit,
}

struct Snake {
    head: Position,
    body: Vec<Position>, // first new line
    direction: Direction,
    state: Option<SnakeAction>,
}

impl Snake {
    pub fn new(x: i16, y:i16) -> Self {
        let direction = Direction::Right;
        let mut body = Vec::<Position>::new();
        body.push(Position::new_by_direction(x, y, direction));

        Self {
            head: Position::new(x, y),
            body: body,
            direction: direction,
            state: None
        }
    }

    fn update(&mut self, fruit: &Fruit) -> GameResult<()> {
        let new_head = Position::new_by_direction(self.head.x, self.head.y, self.direction);
        self.body.insert(0, self.head);
        self.head = new_head;

        if self.head == fruit.pos {
            self.state = Some(SnakeAction::AteFruit)
        } else if self.self_collision() {
            self.state = Some(SnakeAction::SelfCollision)
        } else {
            self.body.pop();
            self.state = None;
        }

        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // let rect = graphics::Rect::new_i32(
        //     (self.head.x).into(),
        //     (self.head.y).into(),
        //     PIXEL_SIZE.0.into(),
        //     PIXEL_SIZE.1.into(),
        // );

        let draw_mode = graphics::DrawMode::Fill(graphics::FillOptions::default());
        let green = graphics::Color::from_rgb(124, 252, 0);
        let outline = graphics::MeshBuilder::new()
            .rectangle(draw_mode, self.head.into(), green)
            .unwrap()
            .build(ctx)
            .unwrap();

        graphics::draw(ctx, &outline, graphics::DrawParam::default())?;

        for segment in &self.body {
            let outline = graphics::MeshBuilder::new()
                .rectangle(draw_mode, (*segment).into(), green)
                .unwrap()
                .build(ctx)
                .unwrap();
            graphics::draw(ctx, &outline, graphics::DrawParam::default())?;
        }
        
        Ok(())
    }

    fn reset(&mut self) {
        self.body = vec![Position::new_by_direction(
            self.head.x,
            self.head.y,
            self.direction
        )]
    }

    fn self_collision(&self) -> bool {
        for segment in &self.body {
            if self.head == *segment {
                return true;
            }
        }

        false
    }
}

struct Fruit {
    pos: Position,
}

impl Fruit {
    pub fn new(x: i16, y: i16) -> Self {
        Self {
            pos: Position::new(x, y),
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // let rect = graphics::Rect::new_i32(
        //     (self.pos.x).into(),
        //     (self.pos.y).into(),
        //     (PIXEL_SIZE.0 - 1).into(),
        //     (PIXEL_SIZE.1 - 1).into(),
        // );

        let draw_mode = graphics::DrawMode::Fill(graphics::FillOptions::default());
        let red = graphics::Color::from_rgb(255, 0, 0);
        let outline = graphics::MeshBuilder::new().
            rectangle(draw_mode, self.pos.into(), red).
            unwrap().
            build(ctx).
            unwrap();

        graphics::draw(ctx, &outline, graphics::DrawParam::default())?;
        Ok(())
    }

    fn regenerate(&mut self) {
        let mut rng = rand::thread_rng();

        let rand_x = rng.gen_range(0..SIZE_IN_PIXELS.0);
        let rand_y = rng.gen_range(0..SIZE_IN_PIXELS.1);

        let x = rand_x as i16;
        let y = rand_y as i16;

        self.pos = Position::new(x, y)
    }

    fn position_in_walls(&mut self, walls: &Vec<Wall>) -> bool {
        for segment in walls {
            if (*segment).contains_position(self.pos) {
                return true;
            }
        }
        false
    }

    pub fn regenerate_outside_walls(&mut self, walls: &Vec<Wall>) {
        self.regenerate();
        while self.position_in_walls(walls) {
            self.regenerate();
        }
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = ContextBuilder::new("snake", "BoyanVD")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("Failed to build context !");

    let game = Game::new();
    
    event::run(ctx, events_loop, game)
}
