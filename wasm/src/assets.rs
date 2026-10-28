// The game's images, built into the .wasm file. The page decodes them (the browser already has a PNG
// decoder) and hands the pixels back through lib.rs before the game starts.

macro_rules! assets {
    ($($name:ident = $path:literal,)*) => {
        pub const PNGS: &[&[u8]] = &[$(include_bytes!(concat!("../../src/assets/", $path)),)*];
        assets!(@index 0usize, $($name,)*);
    };
    (@index $i:expr, $name:ident, $($rest:ident,)*) => {
        #[allow(dead_code)]
        pub const $name: usize = $i;
        assets!(@index $i + 1usize, $($rest,)*);
    };
    (@index $i:expr,) => {};
}

assets! {
    INTRO = "Background/intro.png",
    BG = "Background/BG.png",
    FONT_RED = "Fonts/red.png",
    FONT_WHITE = "Fonts/white.png",
    FONT_YELLOW = "Fonts/yellow.png",
    FONT_FROGGER = "Fonts/frogger.png",
    FROG_00 = "Frog/00.png",
    FROG_01 = "Frog/01.png",
    FROG_10 = "Frog/10.png",
    FROG_11 = "Frog/11.png",
    FROG_20 = "Frog/20.png",
    FROG_21 = "Frog/21.png",
    FROG_30 = "Frog/30.png",
    FROG_31 = "Frog/31.png",
    DEATH_0 = "Death/0.png",
    DEATH_1 = "Death/1.png",
    DEATH_2 = "Death/2.png",
    DEATH_3 = "Death/3.png",
    DEATH_4 = "Death/4.png",
    DEATH_5 = "Death/5.png",
    DEATH_6 = "Death/6.png",
    CAR_0 = "Cars/Car0.png",
    CAR_1 = "Cars/Car1.png",
    CAR_2 = "Cars/Car2.png",
    CAR_3 = "Cars/Car3.png",
    CAR_4 = "Cars/Car4.png",
    LOG_LEFT = "Log/Left.png",
    LOG_MIDDLE = "Log/Middle.png",
    LOG_RIGHT = "Log/Right.png",
    TURTLE_0 = "Turtle/Turtle0.png",
    TURTLE_1 = "Turtle/Turtle1.png",
    TURTLE_2 = "Turtle/Turtle2.png",
    TURTLE_3 = "Turtle/Turtle3.png",
    TURTLE_4 = "Turtle/Turtle4.png",
    ALLIGATOR_0 = "Alligator/0.png",
    ALLIGATOR_1 = "Alligator/1.png",
    GOAL_DANGER_0 = "Goal/Danger0.png",
    GOAL_DANGER_1 = "Goal/Danger1.png",
    GOAL_FLY = "Goal/Fly.png",
    GOAL_FILLED = "Goal/Filled.png",
    GOAL = "Goal/Goal.png",
    GRASS = "Terrain/Grass.png",
    LIFE = "Life/0.png",
}
