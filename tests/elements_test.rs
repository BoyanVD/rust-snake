use snake::elements::{Snake, Fruit, Wall, SnakeAction};

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