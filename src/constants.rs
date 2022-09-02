pub const SNAKE_INIT_POS: (i16, i16) = (5, 5);
pub const FRUIT_INIT_POS: (i16, i16) = (15, 15);
pub const FIRST_WALL_INIT_POS: (i16, i16) = (10, 10);
pub const WALLS_SIZE: i16 = 8;

pub const PIXEL_SIZE: (i16, i16) = (20, 20);
pub const SIZE_IN_PIXELS: (i16, i16) = (20, 20);

pub const DEFAULT_FPS: i16 = 8;

pub const DEFAULT_ACCEL: i16 = 1;

pub const SCREEN_SIZE: (f32, f32) = (
    (PIXEL_SIZE.0 * SIZE_IN_PIXELS.0) as f32,
    (PIXEL_SIZE.1 * SIZE_IN_PIXELS.1) as f32
);

pub const NUMBER_OF_WALLS_ALLOWED_TO_DESTROY: i16 = 3;
pub const NUMBER_OF_APPLES_TO_EAT_FOR_POWER: i16 = 5;