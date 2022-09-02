extern crate ggez;
extern crate rand;

const SNAKE_INIT_POS: (i16, i16) = (5, 5);
const FRUIT_INIT_POS: (i16, i16) = (15, 15);
const FIRST_WALL_INIT_POS: (i16, i16) = (10, 10);
const WALLS_SIZE: i16 = 8;

const PIXEL_SIZE: (i16, i16) = (20, 20);
const SIZE_IN_PIXELS: (i16, i16) = (20, 20);

const DEFAULT_FPS: i16 = 8;

const DEFAULT_ACCEL: i16 = 1;

const SCREEN_SIZE: (f32, f32) = (
    (PIXEL_SIZE.0 * SIZE_IN_PIXELS.0) as f32,
    (PIXEL_SIZE.1 * SIZE_IN_PIXELS.1) as f32
);

const NUMBER_OF_WALLS_ALLOWED_TO_DESTROY: i16 = 3;
const NUMBER_OF_APPLES_TO_EAT_FOR_POWER: i16 = 5;

use ggez::{event, Context, ContextBuilder, GameResult, graphics};
use rand::Rng;

struct Game {
    snake: Snake,
    fruit: Fruit,
    walls: Vec<Wall>,
    score: i16,
    destroyed_walls: i16,
}

impl Game {
    pub fn new() -> Self {
        let mut walls = Vec::<Wall>::new();
        walls.push(Wall::new(FIRST_WALL_INIT_POS.0, FIRST_WALL_INIT_POS.1, WALLS_SIZE));
        Self {
            snake: Snake::new(SNAKE_INIT_POS.0, SNAKE_INIT_POS.1),
            fruit: Fruit::new(FRUIT_INIT_POS.0, FRUIT_INIT_POS.1),
            walls: walls,
            score: 0,
            destroyed_walls: 0,
        }
    }

    pub fn add_wall(&mut self) {
        let mut rng = rand::thread_rng();

        let rand_x = rng.gen_range(0..SIZE_IN_PIXELS.0);
        let rand_y = rng.gen_range(0..SIZE_IN_PIXELS.1);

        let x = rand_x as i16;
        let y = rand_y as i16;

        self.walls.push(Wall::new(x, y, WALLS_SIZE));
    }

    fn opposite(&self, direction: Direction) -> bool {
        (self.snake.direction == Direction::Up && direction == Direction::Down) 
        || (self.snake.direction == Direction::Down && direction == Direction::Up)
        || (self.snake.direction == Direction::Left && direction == Direction::Right)
        || (self.snake.direction == Direction::Right && direction == Direction::Left)        
    }

    fn draw_score(&self, ctx: &mut Context) -> GameResult<()> {

        let draw_mode = graphics::DrawMode::Fill(graphics::FillOptions::default());
        let outline = graphics::Text::new(format!("Score : {}", self.score));
        graphics::draw(ctx, &outline, graphics::DrawParam::default())?;
        
        Ok(())
    }

    fn draw_superpower_left(&self, ctx: &mut Context) -> GameResult<()> {

        let draw_mode = graphics::DrawMode::Fill(graphics::FillOptions::default());
        let outline = graphics::Text::new(format!("Score : {} Walls left to destroy : {}", 
        self.score,
        NUMBER_OF_WALLS_ALLOWED_TO_DESTROY - self.destroyed_walls));
        graphics::draw(ctx, &outline, graphics::DrawParam::default())?;
        
        Ok(())
    }

    fn draw_apples_for_superpower_left(&self, ctx: &mut Context) -> GameResult<()> {

        let draw_mode = graphics::DrawMode::Fill(graphics::FillOptions::default());
        let outline = graphics::Text::new(format!("Score : {} Apples for superspower left : {}",
        self.score,
        NUMBER_OF_APPLES_TO_EAT_FOR_POWER - self.score % NUMBER_OF_APPLES_TO_EAT_FOR_POWER));
        graphics::draw(ctx, &outline, graphics::DrawParam::default())?;
        
        Ok(())
    }

    pub fn remove_wall(&mut self, position: Position) {
        self.walls.retain(|wall| !(wall.contains_position(position)));
        self.destroyed_walls += 1;
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

        // self.draw_score(ctx);

        if self.snake.is_empowered() {
            self.draw_superpower_left(ctx);
        } else {
            self.draw_apples_for_superpower_left(ctx);
        }

        ggez::graphics::present(ctx);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, DEFAULT_FPS as u32) {
            self.snake.update(&self.fruit, &self.walls, self.score)?;

            match self.snake.state {
                Some(SnakeAction::AteFruit) => {
                    self.score += 1;
                    self.add_wall();
                    self.fruit.regenerate_outside_walls(&self.walls);
                    if self.score % NUMBER_OF_APPLES_TO_EAT_FOR_POWER == 0 {
                        self.snake.empower();
                    } else {
                        self.snake.remove_power();
                    }
                },
                Some(SnakeAction::SelfCollision) => {
                    self.score = 0;
                    self.snake.reset();
                    self.walls.clear();
                },
                Some(SnakeAction::WallCollision) => {
                    if self.snake.is_empowered() {
                        self.remove_wall(self.snake.get_head());
                        if self.destroyed_walls == NUMBER_OF_WALLS_ALLOWED_TO_DESTROY {
                            self.snake.remove_power();
                            self.destroyed_walls = 0;
                        }
                    } else {
                        self.score = 0;
                        self.snake.reset();
                        self.walls.clear();
                    }
                },
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

    pub fn position_in_walls(position: Position, walls: &Vec<Wall>) -> bool {
        for segment in walls {
            if (*segment).contains_position(position) {
                return true;
            }
        }
        false
    }

    pub fn any_position_in_walls(positions: &Vec<Position>, walls: &Vec<Wall>) -> bool {
        for position in positions {
            if Wall::position_in_walls(*position, walls) {
                return true;
            }
        }
        false
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
    WallCollision,
}

struct Snake {
    head: Position,
    body: Vec<Position>, // first new line
    direction: Direction,
    state: Option<SnakeAction>,
    empowered: bool,
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
            state: None,
            empowered: false,
        }
    }

    fn update(&mut self, fruit: &Fruit, walls: &Vec<Wall>, score: i16) -> GameResult<()> {
        let new_head = Position::new_by_direction(self.head.x, self.head.y, self.direction);
        self.body.insert(0, self.head);
        self.head = new_head;

        if self.head == fruit.pos {
            self.state = Some(SnakeAction::AteFruit)
        } else if self.self_collision() {
            self.state = Some(SnakeAction::SelfCollision)
        } else if Wall::position_in_walls(self.head, walls) || Wall::any_position_in_walls(&self.body, walls) {
            self.state = Some(SnakeAction::WallCollision)
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
        )];
        self.empowered = false;
    }

    fn self_collision(&self) -> bool {
        for segment in &self.body {
            if self.head == *segment {
                return true;
            }
        }

        false
    }

    fn is_empowered(&self) -> bool {
        self.empowered
    }

    fn get_head(&self) -> Position {
        self.head
    }

    fn remove_power(&mut self) {
        self.empowered = false;
    }

    fn empower(&mut self) {
        self.empowered = true;
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

    pub fn regenerate_outside_walls(&mut self, walls: &Vec<Wall>) {
        self.regenerate();
        while Wall::position_in_walls(self.pos, walls) {
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
