use snake::elements::{Snake, Fruit, Wall, SnakeAction, Direction, Position};

#[test]
fn test_snake_state_update_ate_fruit() {
    let fruit = Fruit::new(15, 15);
    let walls = Vec::<Wall>::new();
    let mut snake = Snake::new(14, 15);
    
    snake.update(&fruit, &walls);

    assert_eq!(snake.get_state(), Some(SnakeAction::AteFruit));
}

#[test]
fn test_snake_state_update_wall_collision() {
    let fruit = Fruit::new(5, 5);
    let walls = vec![Wall::new(15, 15, 1)];
    let mut snake = Snake::new(14, 15);

    snake.update(&fruit, &walls);

    assert_eq!(snake.get_state(), Some(SnakeAction::WallCollision));
}

#[test]
fn test_new_by_direction_generator() {
    let x: i16 = 5;
    let y: i16 = 5;

    let left_position = Position::new_by_direction(x, y, Direction::Left);
    let right_position = Position::new_by_direction(x, y, Direction::Right);
    let down_position = Position::new_by_direction(x, y, Direction::Down);
    let up_position = Position::new_by_direction(x, y, Direction::Up);

    assert_eq!(left_position, Position::new(x - 1, y));
    assert_eq!(right_position, Position::new(x + 1, y));
    assert_eq!(down_position, Position::new(x, y + 1));
    assert_eq!(up_position, Position::new(x, y - 1));
}

#[test]
fn test_position_in_walls() {
    let walls = vec![Wall::new(15, 15, 3)];
    let position_in = Position::new(15, 15);
    let position_out = Position::new(20, 20);

    assert!(Wall::position_in_walls(position_in, &walls));
    assert!(!Wall::position_in_walls(position_out, &walls));
}

#[test]
fn test_any_position_in_walls() {
    let walls = vec![Wall::new(15, 15, 3)];
    let position_in = Position::new(15, 15);
    let position_out1 = Position::new(20, 20);
    let position_out2 = Position::new(25, 25);

    let positions_in = vec![position_in, position_out1];
    let positions_out = vec![position_out1, position_out2];

    assert!(Wall::any_position_in_walls(&positions_in, &walls));
    assert!(!Wall::any_position_in_walls(&positions_out, &walls));
}