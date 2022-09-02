extern crate ggez;
extern crate rand;

use ggez::{event, Context, ContextBuilder, GameResult, graphics};
use rand::Rng;

use crate::constants::{DEFAULT_ACCEL, PIXEL_SIZE, SIZE_IN_PIXELS};

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
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
pub struct Position {
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

pub struct Wall {
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

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {

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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SnakeAction {
    SelfCollision,
    AteFruit,
    WallCollision,
}

pub struct Snake {
    head: Position,
    body: Vec<Position>,
    direction: Direction,
    state: Option<SnakeAction>,
    empowered: bool,
}

impl Snake {
    pub fn new(x: i16, y:i16) -> Self {
        let direction = Direction::Right;

        Self {
            head: Position::new(x, y),
            body: Vec::<Position>::new(),
            direction: direction,
            state: None,
            empowered: false,
        }
    }

    pub fn update(&mut self, fruit: &Fruit, walls: &Vec<Wall>) -> GameResult<()> {
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

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {

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

    pub fn reset(&mut self) {
        self.body = Vec::<Position>::new();
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

    pub fn is_empowered(&self) -> bool {
        self.empowered
    }

    pub fn get_head(&self) -> Position {
        self.head
    }

    pub fn remove_power(&mut self) {
        self.empowered = false;
    }

    pub fn empower(&mut self) {
        self.empowered = true;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn get_state(&self) -> Option<SnakeAction> {
        self.state
    }
}

pub struct Fruit {
    pos: Position,
}

impl Fruit {
    pub fn new(x: i16, y: i16) -> Self {
        Self {
            pos: Position::new(x, y),
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {

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