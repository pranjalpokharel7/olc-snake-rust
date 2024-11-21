use std::time::Duration;

pub const FRAME_DURATION: Duration = Duration::from_millis(200);

// TODO: changing these values don't really change anything because we use a static str for optimization
pub const SCREEN_WIDTH: usize = 100;
pub const SCREEN_HEIGHT: usize = 40;

pub const SCREEN_STRING: &'static str = "====================================================================================================\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
|                                                                                                  |\n\
====================================================================================================\n
";
