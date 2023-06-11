pub const FOLDER_NAME: &str = "./render";
pub const IMAGE_WIDTH: u16 = 1920;
pub const IMAGE_HEIGHT: u16 = 1080;

pub const ASPECT_RATIO: f32 = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;
pub const VIEWPORT_HEIGHT: f32 = 2.0;
pub const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
pub const FOCAL_LENGTH: f32 = 1.0;

pub const SAMPLES_PER_PIXEL : u16 = 16;
pub const MAX_BOUNCES : u16 = 32;

pub const QUICK_RENDER : bool= false;
pub const SEED : u64 = 41253;
