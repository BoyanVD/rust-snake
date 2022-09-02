extern crate ggez;
extern crate rand;

use crate::elements::{Snake, Fruit, Direction, SnakeAction, Wall, Position};
use crate::constants;

use ggez::{event, Context, ContextBuilder, GameResult, graphics};
use rand::Rng;

pub struct Game {
    snake: Snake,
    fruit: Fruit,
    walls: Vec<Wall>,
    score: i16,
    destroyed_walls: i16,
}

impl Game {
    pub fn new() -> Self {
        let mut walls = Vec::<Wall>::new();
        walls.push(Wall::new(constants::FIRST_WALL_INIT_POS.0, constants::FIRST_WALL_INIT_POS.1, constants::WALLS_SIZE));
        Self {
            snake: Snake::new(constants::SNAKE_INIT_POS.0, constants::SNAKE_INIT_POS.1),
            fruit: Fruit::new(constants::FRUIT_INIT_POS.0, constants::FRUIT_INIT_POS.1),
            walls: walls,
            score: 0,
            destroyed_walls: 0,
        }
    }

    fn add_wall(&mut self) {
        let mut rng = rand::thread_rng();

        let rand_x = rng.gen_range(0..constants::SIZE_IN_PIXELS.0);
        let rand_y = rng.gen_range(0..constants::SIZE_IN_PIXELS.1);

        let x = rand_x as i16;
        let y = rand_y as i16;

        self.walls.push(Wall::new(x, y, constants::WALLS_SIZE));
    }

    fn opposite(&self, direction: Direction) -> bool {
        (self.snake.get_direction() == Direction::Up && direction == Direction::Down) 
        || (self.snake.get_direction() == Direction::Down && direction == Direction::Up)
        || (self.snake.get_direction() == Direction::Left && direction == Direction::Right)
        || (self.snake.get_direction() == Direction::Right && direction == Direction::Left)        
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
        constants::NUMBER_OF_WALLS_ALLOWED_TO_DESTROY - self.destroyed_walls));
        graphics::draw(ctx, &outline, graphics::DrawParam::default())?;
        
        Ok(())
    }

    fn draw_apples_for_superpower_left(&self, ctx: &mut Context) -> GameResult<()> {

        let draw_mode = graphics::DrawMode::Fill(graphics::FillOptions::default());
        let outline = graphics::Text::new(format!("Score : {} Apples for superspower left : {}",
        self.score,
        constants::NUMBER_OF_APPLES_TO_EAT_FOR_POWER - self.score % constants::NUMBER_OF_APPLES_TO_EAT_FOR_POWER));
        graphics::draw(ctx, &outline, graphics::DrawParam::default())?;
        
        Ok(())
    }

    fn remove_wall(&mut self, position: Position) {
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

        if self.snake.is_empowered() {
            self.draw_superpower_left(ctx);
        } else {
            self.draw_apples_for_superpower_left(ctx);
        }

        ggez::graphics::present(ctx);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, constants::DEFAULT_FPS as u32) {
            self.snake.update(&self.fruit, &self.walls)?;

            match self.snake.get_state() {
                Some(SnakeAction::AteFruit) => {
                    self.score += 1;
                    self.add_wall();
                    self.fruit.regenerate_outside_walls(&self.walls);
                    if self.score % constants::NUMBER_OF_APPLES_TO_EAT_FOR_POWER == 0 {
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
                        if self.destroyed_walls == constants::NUMBER_OF_WALLS_ALLOWED_TO_DESTROY {
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

        if let Some(direction) = Direction::from_keycode(keycode) {
            if !self.opposite(direction) {
                self.snake.set_direction(direction);
            }
        }
    }
}