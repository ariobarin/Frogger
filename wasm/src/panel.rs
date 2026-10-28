// FroggerPanel.java: the screens, levels, game logic and drawing

use crate::frog::Frog;
use crate::gfx::{Color, Graphics};
use crate::goal::Goal;
use crate::hud::{self, Button, LifeCounter, Terrain, TimerBar};
use crate::objects::{self, Alligator, Car, Log, Turtle};
use crate::random::Random;
use crate::scores::{Score, ScoreManager};
use crate::util::{self, Art};
use crate::assets;

pub const INTRO: i32 = 0;
pub const GAME: i32 = 1;
pub const LEADERBOARD: i32 = 2;
pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 800;
pub const FPS: i32 = 60;

// java.awt.event.KeyEvent codes
pub const VK_LEFT: usize = 37;
pub const VK_UP: usize = 38;
pub const VK_RIGHT: usize = 39;
pub const VK_DOWN: usize = 40;
pub const KEY_LAST: usize = 401;

#[derive(Clone)]
pub struct FroggerPanel {
    screen: i32,
    score: i32,
    highest_score: i32, // Highest score in the leaderboard
    level: i32,         // Current level
    life_counter: LifeCounter,
    timer_bar: TimerBar,

    // Keys pressed - so holding down a key doesn't register multiple times
    keys: Vec<bool>,
    key_pressed: Vec<bool>,

    frog: Frog,

    cars: Vec<Car>,
    logs: Vec<Log>,
    turtles: Vec<Turtle>,
    alligators: Vec<Alligator>,
    goals: Vec<Goal>,
    grasses: Vec<Terrain>,
    water: Terrain,

    score_manager: ScoreManager,
    play_button: Button,
    leaderboard_button: Button,

    random: Random,
    // Java shows a modal "Game Over! Enter your name:" dialog in the middle of a step; here the step
    // waits for the page's answer (see lib.rs) and then carries on
    pub asking_name: bool,
}

impl FroggerPanel {
    pub fn new(seed: u64, saved_scores: &str) -> FroggerPanel {
        // Score manager, timer bar, and life counter
        let score_manager = ScoreManager::new(saved_scores);
        FroggerPanel {
            screen: INTRO,
            score: 0,
            // Get the highest score for the "Hi-Score" text
            highest_score: score_manager.get_highest_score(),
            level: 1,
            life_counter: LifeCounter::new(700, 760),
            timer_bar: TimerBar::new(10, 760, 400, 30, 10.0, FPS),
            keys: vec![false; KEY_LAST + 1],
            key_pressed: vec![false; KEY_LAST + 1],
            frog: Frog::new(VK_RIGHT, VK_UP, VK_LEFT, VK_DOWN),
            cars: Vec::new(),
            logs: Vec::new(),
            turtles: Vec::new(),
            alligators: Vec::new(),
            goals: Vec::new(),
            grasses: Vec::new(),
            water: Terrain::new(0, 0, WIDTH, HEIGHT / 2, hud::WATER),
            score_manager,
            play_button: Button::new("Play", 300, 252, 200, 50, Color::rgba(0, 0, 0, 196)),
            leaderboard_button: Button::new("Leaderboard", 250, 302, 300, 50, Color::rgba(0, 0, 0, 196)),
            random: Random::new(seed),
            asking_name: false,
        }
    }

    fn init_base(&mut self) {
        self.frog.reset();
        self.cars = Vec::new();
        self.logs = Vec::new();
        self.turtles = Vec::new();
        self.alligators = Vec::new();
        self.goals = Vec::new();
        self.water = Terrain::new(0, 0, WIDTH, HEIGHT / 2, hud::WATER);
        self.grasses = Vec::new();

        // Goals
        self.create_goals(75, 100, 5, 150);

        // Terrain
        self.grasses.push(Terrain::new(0, HEIGHT / 2, WIDTH, 50, hud::GRASS));
        self.grasses.push(Terrain::new(0, HEIGHT - 100, WIDTH, 50, hud::GRASS));
    }

    // Unused, as in the Java version: startGame begins with level three's layout
    #[allow(dead_code)]
    fn init_level_one(&mut self) {
        self.init_base();

        // Floating objects
        self.create_logs(150, 200, 2, false, 2);
        self.create_turtles(200, 100, 5, true, 4);
        self.create_logs(250, 200, 5, false, 2);
        self.create_logs(300, 150, 3, false, 3);
        self.create_turtles(350, 150, 5, true, 4);

        // Cars
        self.create_cars(450, 125, 5, true, objects::WHITE, 3);
        self.create_cars(500, 50, 4, false, objects::RED, 3);
        self.create_cars(550, 50, 6, true, objects::PINK, 3);
        self.create_cars(600, 50, 3, false, objects::GREEN, 3);
        self.create_cars(650, 50, 4, true, objects::YELLOW, 3);
    }

    fn init_level_two(&mut self) {
        self.init_base();

        // Floating objects
        self.create_alligators(150, 200, 2, false, 2);
        self.create_turtles(200, 100, 5, true, 4);
        self.create_logs(250, 200, 7, false, 2);
        self.create_logs(300, 150, 3, false, 3);
        self.create_turtles(350, 150, 5, true, 4);

        // Cars
        self.create_cars(450, 125, 6, true, objects::WHITE, 3);
        self.create_cars(500, 50, 3, false, objects::RED, 2);
        self.create_cars(550, 50, 5, true, objects::PINK, 4);
        self.create_cars(600, 50, 2, false, objects::GREEN, 3);
        self.create_cars(650, 50, 4, true, objects::YELLOW, 3);
    }

    fn init_level_three(&mut self) {
        self.init_base();

        // Floating objects
        self.create_alligators(150, 200, 3, false, 3);
        self.create_turtles(200, 100, 5, true, 3);
        self.create_logs(250, 200, 6, false, 1);
        self.create_logs(300, 150, 4, false, 3);
        self.create_turtles(350, 150, 2, true, 3);

        // Cars
        self.create_cars(450, 125, 5, true, objects::WHITE, 3);
        self.create_cars(500, 50, 3, false, objects::RED, 3);
        self.create_cars(550, 50, 6, true, objects::PINK, 3);
        self.create_cars(600, 50, 2, false, objects::GREEN, 3);
        self.create_cars(650, 50, 4, true, objects::YELLOW, 3);
    }

    fn next_level(&mut self) {
        self.level += 1;
        self.frog.reset();
        match self.level {
            2 => self.init_level_two(),
            3 => self.init_level_three(),
            // Game completed - only 3 levels to encourage fast gameplay amirite
            _ => self.screen = LEADERBOARD,
        }
    }

    // Helper method to calculate starting positions
    fn calculate_positions(&mut self, width: i32, count: i32) -> Vec<i32> {
        let gap = 800 / count;
        let offset = (util::random_double(&mut self.random, 0.0, 1.0) * (gap - width) as f64) as i32;
        (0..count).map(|i| gap * i + offset).collect()
    }

    fn create_cars(&mut self, y: i32, width: i32, speed: i32, left: bool, color: usize, count: i32) {
        for x in self.calculate_positions(width, count) {
            self.cars.push(Car::new(x, y, width, speed, left, color));
        }
    }

    fn create_logs(&mut self, y: i32, width: i32, speed: i32, left: bool, count: i32) {
        for x in self.calculate_positions(width, count) {
            self.logs.push(Log::new(x, y, width, speed, left));
        }
    }

    fn create_turtles(&mut self, y: i32, width: i32, speed: i32, left: bool, count: i32) {
        let positions = self.calculate_positions(width, count);
        for (i, x) in positions.into_iter().enumerate() {
            let breathing = i as i32 == count - 1; // Only the last turtle breathes
            self.turtles.push(Turtle::new(x, y, width, speed, left, breathing, &mut self.random));
        }
    }

    fn create_alligators(&mut self, y: i32, width: i32, speed: i32, left: bool, count: i32) {
        let positions = self.calculate_positions(width, count);
        for (i, x) in positions.into_iter().enumerate() {
            if i == 0 {
                // First object is an alligator
                self.alligators.push(Alligator::new(x, y, width, speed, left));
            } else {
                // Rest are logs
                self.logs.push(Log::new(x, y, width, speed, left));
            }
        }
    }

    fn create_goals(&mut self, x: i32, y: i32, count: i32, gap: i32) {
        for i in 0..count {
            self.goals.push(Goal::new(gap * i + x, y));
        }
    }

    fn check_death(&self) -> bool {
        let frog = &self.frog;
        if frog.is_dead() {
            return false; // Don't check for death if the frog is already dying
        }

        // Check if frog is off screen
        if frog.off_screen() {
            return true;
        }

        // Check if the frog is riding a log
        if self.logs.iter().any(|l| frog.is_riding_log(l)) {
            return false;
        }

        // Check if the frog is riding a turtle
        if self.turtles.iter().any(|t| frog.is_riding_turtle(t)) {
            return false;
        }

        // Check if the frog is being eaten by an alligator or is riding the alligator
        for a in &self.alligators {
            if frog.is_being_eaten(a) {
                return true;
            }
            if frog.is_riding_alligator(a) {
                return false;
            }
        }

        // Check if the frog is colliding with a car
        if self.cars.iter().any(|c| frog.is_colliding(c.base.get_rect())) {
            return true;
        }

        // Check if the frog is over the time limit
        if self.timer_bar.is_over_time_limit() {
            return true;
        }

        // Check if the frog is colliding with water
        frog.is_colliding(self.water.get_rect()) && !frog.in_air()
    }

    // The game step, up to the game over dialog. Returns false when the step has to wait for a name.
    pub fn step(&mut self) -> bool {
        // like Java's switch, the GAME case runs to its end even if it changes the screen
        let game = self.screen == GAME;
        if game {
            // Update time spent
            self.timer_bar.update();

            // Check for goal
            for i in 0..self.goals.len() {
                if !self.frog.is_colliding(self.goals[i].get_rect()) {
                    continue;
                }
                if self.goals[i].is_filled() {
                    self.frog.die();
                    break;
                }

                // Fill the goal
                if self.goals[i].fill() {
                    self.frog.reset();

                    // Add score based on time remaining
                    self.add_score(((self.timer_bar.get_time_limit() - self.timer_bar.get_time_spent()) * 50.0) as i32);

                    // Add score based on flies
                    if self.goals[i].fly() {
                        self.add_score(100);
                    }

                    // Go to the next level if all goals are filled
                    if self.goals.iter().all(|g| g.is_filled()) {
                        self.next_level();
                    }

                    // Reset the timer after a goal is filled
                    self.timer_bar.reset();
                }
            }

            // Check for death
            if self.check_death() {
                self.frog.die();
                self.timer_bar.reset();
                self.life_counter.lose_life();
            }

            // Game over - show input dialog for name and add score to leaderboard
            if self.life_counter.is_game_over() && !self.frog.is_dead() {
                self.asking_name = true;
                return false;
            }
        }
        self.finish_step(game);
        true
    }

    // The answer to the game over dialog (None for Cancel), then the rest of the step
    pub fn name_entered(&mut self, name: Option<&str>) {
        if let Some(name) = name.filter(|n| !n.is_empty()) {
            let mut chars = name.chars();
            let first: String = chars.next().into_iter().flat_map(char::to_uppercase).collect();
            let title_case_name = first + &chars.as_str().to_lowercase();
            self.score_manager.add_score(Score { name: title_case_name, score: self.score });
        }
        self.screen = LEADERBOARD;
        self.asking_name = false;
        self.finish_step(true);
    }

    pub fn scores_text(&self) -> String {
        self.score_manager.save_scores()
    }

    // The rest of the step: the GAME case moves everything, then every screen resets the pressed keys
    fn finish_step(&mut self, game: bool) {
        if game {
            // Move cars
            for c in &mut self.cars {
                c.base.move_object();
            }

            // Reset frog momentum if the frog is not in the air
            if !self.frog.in_air() {
                self.frog.reset_momentum();
            }

            // Move logs
            for l in &mut self.logs {
                l.base.move_object();
                if self.frog.is_riding_log(l) {
                    self.frog.slide(l.base.get_velocity());
                }
            }
            // Move turtles
            for t in &mut self.turtles {
                t.move_object();
                if self.frog.is_riding_turtle(t) {
                    self.frog.slide(t.base.get_velocity());
                }
            }
            // Move alligators
            for a in &mut self.alligators {
                a.move_object();
                if self.frog.is_riding_alligator(a) {
                    self.frog.slide(a.base.get_velocity());
                }
            }

            // Move frog
            self.frog.move_with_keys(&self.key_pressed);

            // Update goals
            for goal in &mut self.goals {
                goal.update(&mut self.random);
            }
        }

        // Reset the key pressed array
        self.key_pressed.fill(false);
    }

    pub fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    pub fn key_released(&mut self, key: usize) {
        if key <= KEY_LAST {
            self.keys[key] = false;
            self.key_pressed[key] = false;
        }
    }

    pub fn key_pressed(&mut self, key: usize) {
        if key <= KEY_LAST && !self.keys[key] {
            self.keys[key] = true;

            // This will reset until the key is released - on press
            self.key_pressed[key] = true;
        }
    }

    // Mouse hover - for the buttons
    pub fn mouse_moved(&mut self, x: i32, y: i32) {
        let play = self.play_button.is_hovered(x, y);
        let leaderboard = self.leaderboard_button.is_hovered(x, y);
        self.play_button.set_hovered(play);
        self.leaderboard_button.set_hovered(leaderboard);
    }

    pub fn mouse_pressed(&mut self, x: i32, y: i32) {
        // Start the game if the play button is clicked
        if self.screen == INTRO {
            if self.play_button.is_hovered(x, y) {
                self.start_game();
            } else if self.leaderboard_button.is_hovered(x, y) {
                self.screen = LEADERBOARD;
            }
        } else if self.screen == LEADERBOARD {
            // Go back to the intro screen if the leaderboard button is clicked
            self.screen = INTRO;
        }
    }

    pub fn paint(&self, g: &mut Graphics, art: &Art) {
        match self.screen {
            INTRO => {
                // Draw the intro screen
                util::draw_image(g, art, assets::INTRO, 0, 0, WIDTH, HEIGHT);
                util::write_centered_text(g, art, "Frogger", WIDTH / 2, 90, 60, util::FROGGER_FONT);

                // Draw custom buttons
                self.play_button.draw(g, art);
                self.leaderboard_button.draw(g, art);
            }
            GAME => {
                // Draw the game background - water and road
                util::draw_image(g, art, assets::BG, 0, 0, WIDTH, HEIGHT);

                for t in &self.grasses {
                    t.draw(g, art);
                }
                for c in &self.cars {
                    c.draw(g, art);
                }
                for l in &self.logs {
                    l.draw(g, art);
                }
                for t in &self.turtles {
                    t.draw(g, art);
                }
                for a in &self.alligators {
                    a.draw(g, art);
                }
                for goal in &self.goals {
                    goal.draw(g, art);
                }

                self.frog.draw(g, art);
                self.timer_bar.draw(g);
                self.life_counter.draw(g, art);

                // Draw score
                util::write_text(g, art, "Score ", 30, 15, 30, util::WHITE_FONT);
                util::write_text(g, art, &self.score.to_string(), 30 * 7, 15, 30, util::RED_FONT);

                // Draw hi-score
                util::write_text(g, art, "Hi-Score ", WIDTH / 2 - 50, 15, 30, util::WHITE_FONT);
                util::write_text(g, art, &self.highest_score.to_string(), WIDTH / 2 + 9 * 30 - 50, 15, 30, util::RED_FONT);
            }
            _ => {
                // Draw the leaderboard
                g.fill_rect(Color::BLACK, 0, 0, WIDTH, HEIGHT);
                util::write_centered_text(g, art, "Hi-Scores", WIDTH / 2, 100, 60, util::YELLOW_FONT);

                // Go through the high scores and display them - going down the screen
                let mut y = 160;
                for s in self.score_manager.get_high_scores().iter().take(15) {
                    util::write_right_text(g, art, &format!("{} -", s.name), WIDTH / 2 + 24, y, 24, util::WHITE_FONT);
                    util::write_text(g, art, &format!(" {}", s.score), WIDTH / 2 + 24, y, 24, util::RED_FONT);
                    y += 30;
                }

                // Draw the play again button
                util::write_centered_text(g, art, "Click to Play Again", WIDTH / 2, HEIGHT - 50, 24, util::YELLOW_FONT);
            }
        }
    }

    // For the frame by frame comparison with the Java version (examples/frames.rs), which plays long
    // runs without reaching the game over dialog
    #[doc(hidden)]
    pub fn reset_lives(&mut self) {
        self.life_counter.reset();
    }

    // For the test bot that plays those runs (examples/bot.rs): screen, level, score, frog dead / idle, frog y
    #[doc(hidden)]
    pub fn debug_state(&self) -> (i32, i32, i32, bool, bool, i32) {
        (self.screen, self.level, self.score, self.frog.is_dead(), self.frog.is_idle(), self.frog.y())
    }

    fn start_game(&mut self) {
        self.screen = GAME;
        self.score = 0;
        self.highest_score = self.score_manager.get_highest_score();
        self.level = 1;
        self.life_counter.reset();
        self.init_level_three(); // TEST
    }
}
