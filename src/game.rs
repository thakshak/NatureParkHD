use crate::audio::SoundId;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

pub const MIN_COLS: usize = 5;
pub const MAX_COLS: usize = 12;
pub const MIN_VISIBLE_ROWS: usize = 8;
pub const MAX_VISIBLE_ROWS: usize = 16;
pub const BUFFER_ROWS: usize = 3;
const HIGHSCORE_FILE: &str = "highscores_v2.json";

pub const SHAPES: [[[u8; 3]; 3]; 9] = [
    [[0, 1, 0], [0, 1, 0], [0, 1, 0]], // 0: vertical 3
    [[0, 0, 0], [0, 0, 0], [1, 1, 1]], // 1: horizontal 3
    [[0, 0, 0], [0, 1, 0], [1, 1, 0]], // 2: L-shape
    [[0, 0, 0], [1, 0, 0], [1, 1, 0]], // 3: J-shape
    [[0, 0, 0], [0, 0, 0], [1, 1, 0]], // 4: horizontal 2
    [[0, 0, 0], [0, 1, 0], [0, 1, 0]], // 5: vertical 2
    [[0, 0, 0], [0, 0, 0], [0, 1, 0]], // 6: lightning
    [[0, 0, 0], [0, 0, 0], [0, 1, 0]], // 7: bomb (block_n)
    [[0, 0, 0], [0, 0, 0], [0, 1, 0]], // 8: star
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameMode {
    Classic,
    Endless,
    RisingTide,
    Blitz,
}

impl GameMode {
    pub fn label(self) -> &'static str {
        match self {
            GameMode::Classic => "Classic (10 Lvls)",
            GameMode::Endless => "Endless",
            GameMode::RisingTide => "Rising Tide",
            GameMode::Blitz => "2-Min Blitz",
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            GameMode::Classic => "classic",
            GameMode::Endless => "endless",
            GameMode::RisingTide => "rising_tide",
            GameMode::Blitz => "blitz",
        }
    }

    pub fn next(self) -> Self {
        match self {
            GameMode::Classic => GameMode::Endless,
            GameMode::Endless => GameMode::RisingTide,
            GameMode::RisingTide => GameMode::Blitz,
            GameMode::Blitz => GameMode::Classic,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScreenState {
    Intro,
    Menu,
    GridSettings,
    HighScores,
    Instructions,
    LevelExit,
    LevelEnter,
    SpawnFirst,
    SpawnNext,
    Falling,
    Collapse,
    Evaluate,
    Exploding,
    StoneShake,
    GameOver,
    AllCleared,
    Ending,
    Paused,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct HighScoreEntry {
    pub score: u32,
    pub level: u32,
    pub year: String,
    pub month: String,
    pub day: String,
}

impl Default for HighScoreEntry {
    fn default() -> Self {
        Self {
            score: 0,
            level: 1,
            year: "----".to_owned(),
            month: "--".to_owned(),
            day: "--".to_owned(),
        }
    }
}

pub struct Game {
    pub state: ScreenState,
    pub saved_state: ScreenState,
    pub tick_counter: u32,
    pub sub_tick: i32,

    // Mode & options
    pub mode: GameMode,
    pub blind_next: bool,

    // Customizable grid dimensions
    pub cols: usize,
    pub visible_rows: usize,
    pub total_rows: usize,

    // Board & pieces
    pub board: Vec<Vec<u8>>,
    pub match_mask: Vec<Vec<u8>>,
    pub piece: [[[u8; 3]; 3]; 2],
    pub piece_colors: [[u8; 3]; 2],
    pub piece_type: [usize; 2],
    pub piece_pos: [i32; 2],

    // Hold & skill-ceiling mechanics
    pub hold_piece: Option<([[u8; 3]; 3], [u8; 3], usize)>,
    pub hold_used_this_turn: bool,
    pub fever_meter: f32,
    pub tide_counter: u32,
    pub tide_interval: u32,
    pub blitz_ticks_remaining: i32,

    // Progression & scoring
    pub score: u32,
    pub high_score: u32,
    pub level: u32,
    pub combo: u32,
    pub cleared_in_step: u32,
    pub match_found: bool,
    pub just_landed: bool,

    // Spawner throttling
    pub special_throttle: [u32; 3],
    pub stone_counter: u32,
    pub stone_flag: u8,

    // Character facial expression
    pub char_eye: usize,
    pub char_tear: usize,
    pub char_mouth: usize,
    pub char_hand: usize,
    pub char_intro: usize,

    // High scores & UI
    pub high_scores_by_grid: HashMap<String, [HighScoreEntry; 5]>,
    pub high_scores: [HighScoreEntry; 5],
    pub game_in_progress: bool,
    pub menu_index: usize,
    pub grid_setting_row: usize,
    pub pause_choice: usize,

    pub pending_sounds: Vec<SoundId>,
    pub should_exit: bool,
    rng_state: u32,
}

impl Game {
    pub fn new(seed: u32) -> Self {
        let cols = 6;
        let visible_rows = 10;
        let total_rows = visible_rows + BUFFER_ROWS;
        let mut g = Self {
            state: ScreenState::Intro,
            saved_state: ScreenState::LevelEnter,
            tick_counter: 0,
            sub_tick: 0,
            mode: GameMode::Classic,
            blind_next: false,
            cols,
            visible_rows,
            total_rows,
            board: vec![vec![0; total_rows]; cols],
            match_mask: vec![vec![0; total_rows]; cols],
            piece: [[[0; 3]; 3]; 2],
            piece_colors: [[0; 3]; 2],
            piece_type: [0; 2],
            piece_pos: [((cols as i32) - 3).max(0) / 2, 0],
            hold_piece: None,
            hold_used_this_turn: false,
            fever_meter: 0.0,
            tide_counter: 0,
            tide_interval: 8,
            blitz_ticks_remaining: 2400,
            score: 0,
            high_score: 0,
            level: 1,
            combo: 0,
            cleared_in_step: 0,
            match_found: false,
            just_landed: true,
            special_throttle: [0; 3],
            stone_counter: 0,
            stone_flag: 0,
            char_eye: 0,
            char_tear: 0,
            char_mouth: 2,
            char_hand: 2,
            char_intro: 0,
            high_scores_by_grid: HashMap::new(),
            high_scores: Default::default(),
            game_in_progress: false,
            menu_index: 0,
            grid_setting_row: 0,
            pause_choice: 0,
            pending_sounds: vec![SoundId::Intro],
            should_exit: false,
            rng_state: if seed == 0 { 0xCAFE_BABE } else { seed },
        };
        g.load_high_scores();
        g.high_score = g.high_scores[0].score;
        g
    }

    pub fn fever_multiplier(&self) -> u32 {
        if self.fever_meter >= 0.85 {
            4
        } else if self.fever_meter >= 0.55 {
            3
        } else if self.fever_meter >= 0.25 {
            2
        } else {
            1
        }
    }

    pub fn score_key(&self) -> String {
        format!("{}x{}:{}", self.cols, self.visible_rows, self.mode.key())
    }

    fn sync_high_scores_for_key(&mut self) {
        self.high_scores = self
            .high_scores_by_grid
            .get(&self.score_key())
            .cloned()
            .unwrap_or_default();
        self.high_score = self.high_scores[0].score;
    }

    pub fn set_grid_size(&mut self, cols: usize, visible_rows: usize) {
        let prev_state = self.state;
        self.cols = cols.clamp(MIN_COLS, MAX_COLS);
        self.visible_rows = visible_rows.clamp(MIN_VISIBLE_ROWS, MAX_VISIBLE_ROWS);
        self.total_rows = self.visible_rows + BUFFER_ROWS;
        self.game_in_progress = false;
        self.sync_high_scores_for_key();
        self.reset_game_state();
        self.state = prev_state;
    }

    pub fn adjust_cols(&mut self, delta: i32) {
        let next = (self.cols as i32 + delta).clamp(MIN_COLS as i32, MAX_COLS as i32) as usize;
        if next != self.cols {
            self.set_grid_size(next, self.visible_rows);
        }
    }

    pub fn adjust_rows(&mut self, delta: i32) {
        let next = (self.visible_rows as i32 + delta)
            .clamp(MIN_VISIBLE_ROWS as i32, MAX_VISIBLE_ROWS as i32) as usize;
        if next != self.visible_rows {
            self.set_grid_size(self.cols, next);
        }
    }

    fn rand_mod(&mut self, n: u32) -> u32 {
        let mut x = self.rng_state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng_state = x;
        x % n
    }

    fn load_high_scores(&mut self) {
        if let Ok(data) = fs::read_to_string(HIGHSCORE_FILE) {
            if let Ok(map) = serde_json::from_str::<HashMap<String, [HighScoreEntry; 5]>>(&data) {
                for (k, v) in map {
                    let norm = if k.contains(':') {
                        k
                    } else {
                        format!("{k}:classic")
                    };
                    self.high_scores_by_grid.insert(norm, v);
                }
            } else if let Ok(legacy) = serde_json::from_str::<[HighScoreEntry; 5]>(&data) {
                self.high_scores_by_grid
                    .insert("6x10:classic".to_owned(), legacy);
            }
        }
        self.sync_high_scores_for_key();
    }

    fn save_high_scores(&mut self) {
        self.high_scores_by_grid
            .insert(self.score_key(), self.high_scores.clone());
        if let Ok(json) = serde_json::to_string_pretty(&self.high_scores_by_grid) {
            let _ = fs::write(HIGHSCORE_FILE, json);
        }
    }

    pub fn menu_items(&self) -> Vec<String> {
        let mut items = Vec::new();
        if self.game_in_progress {
            items.push("Continue".to_owned());
        }
        items.push("New game".to_owned());
        items.push(format!("Mode: {}", self.mode.label()));
        items.push(format!("Grid: {} x {}", self.cols, self.visible_rows));
        items.push(format!(
            "Blind Next: {}",
            if self.blind_next { "ON" } else { "OFF" }
        ));
        items.push("High scores".to_owned());
        items.push("Instructions".to_owned());
        items.push("Exit".to_owned());
        items
    }

    pub fn toggle_pause(&mut self) {
        if self.state == ScreenState::Paused {
            self.state = self.saved_state;
        } else if matches!(
            self.state,
            ScreenState::LevelExit
                | ScreenState::LevelEnter
                | ScreenState::SpawnFirst
                | ScreenState::SpawnNext
                | ScreenState::Falling
                | ScreenState::Collapse
                | ScreenState::Evaluate
                | ScreenState::Exploding
                | ScreenState::StoneShake
        ) {
            self.saved_state = self.state;
            self.state = ScreenState::Paused;
        }
    }

    pub fn open_menu(&mut self) {
        if self.state == ScreenState::Paused {
            self.game_in_progress = true;
            self.record_high_score();
        } else if matches!(
            self.state,
            ScreenState::LevelExit
                | ScreenState::LevelEnter
                | ScreenState::SpawnFirst
                | ScreenState::SpawnNext
                | ScreenState::Falling
                | ScreenState::Collapse
                | ScreenState::Evaluate
                | ScreenState::Exploding
                | ScreenState::StoneShake
        ) {
            self.saved_state = self.state;
            self.game_in_progress = true;
            self.record_high_score();
        }
        self.state = ScreenState::Menu;
        self.menu_index = 0;
    }

    pub fn start_new_game(&mut self) {
        self.reset_game_state();
        self.pending_sounds.push(SoundId::Theme);
        self.state = ScreenState::LevelEnter;
    }

    pub fn continue_game(&mut self) {
        self.tick_counter = 0;
        self.sub_tick = 0;
        self.pause_choice = 0;
        self.game_in_progress = false;
        self.state = if self.saved_state == ScreenState::Intro {
            ScreenState::LevelEnter
        } else {
            self.saved_state
        };
    }

    pub fn select_menu_item(&mut self) {
        let items = self.menu_items();
        let Some(choice) = items.get(self.menu_index) else {
            return;
        };
        if choice == "Continue" {
            self.continue_game();
        } else if choice == "New game" {
            self.start_new_game();
        } else if choice.starts_with("Mode:") {
            self.mode = self.mode.next();
            self.game_in_progress = false;
            self.sync_high_scores_for_key();
        } else if choice.starts_with("Grid:") {
            self.grid_setting_row = 0;
            self.state = ScreenState::GridSettings;
        } else if choice.starts_with("Blind Next:") {
            self.blind_next = !self.blind_next;
        } else if choice == "High scores" {
            self.state = ScreenState::HighScores;
        } else if choice == "Instructions" {
            self.state = ScreenState::Instructions;
        } else if choice == "Exit" {
            self.should_exit = true;
        }
    }

    fn spawn_col(&self) -> i32 {
        ((self.cols as i32) - 3).max(0) / 2
    }

    fn reset_game_state(&mut self) {
        self.just_landed = true;
        self.sub_tick = 0;
        self.piece_pos = [self.spawn_col(), 0];
        self.match_found = false;
        self.combo = 0;
        self.special_throttle = [0; 3];
        self.cleared_in_step = 0;
        self.pause_choice = 0;
        self.stone_flag = 0;
        self.stone_counter = 0;
        self.hold_piece = None;
        self.hold_used_this_turn = false;
        self.fever_meter = 0.0;
        self.tide_counter = 0;
        self.tide_interval = 8;
        self.blitz_ticks_remaining = 2400; // 120 seconds at 50ms/tick
        self.board = vec![vec![0; self.total_rows]; self.cols];
        self.match_mask = vec![vec![0; self.total_rows]; self.cols];
        self.piece_type = [0; 2];
        self.high_score = self.high_scores[0].score;
        self.tick_counter = 0;
        self.char_eye = 0;
        self.char_tear = 0;
        self.char_mouth = 2;
        self.char_hand = 2;
        self.char_intro = 0;
        self.state = ScreenState::LevelEnter;
        self.saved_state = ScreenState::LevelEnter;
        self.score = 0;
        self.level = 1;
        self.game_in_progress = false;
        self.piece_colors = [[0; 3]; 2];
        self.piece = [[[0; 3]; 3]; 2];
    }

    fn should_step(&self) -> bool {
        let b = self.tick_counter;
        match self.state {
            ScreenState::Intro
            | ScreenState::StoneShake
            | ScreenState::Exploding
            | ScreenState::AllCleared => b % 7 == 0,
            ScreenState::SpawnFirst | ScreenState::GameOver => b % 9 == 0,
            ScreenState::SpawnNext | ScreenState::Falling => {
                let div = if self.mode == GameMode::Endless {
                    // 50ms (1 tick) faster per level: 21 (1050ms) down to 1 (50ms)
                    22u32.saturating_sub(self.level).max(1)
                } else {
                    match self.level {
                        1 => 21,
                        2 => 19,
                        3 => 15,
                        4 => 11,
                        5 => 9,
                        6 => 7,
                        7 => 5,
                        8 => 3,
                        _ => 1,
                    }
                };
                b % div == 0
            }
            ScreenState::Collapse => true,
            ScreenState::Evaluate => b % 3 == 0,
            ScreenState::LevelExit | ScreenState::LevelEnter => b % 5 == 0,
            ScreenState::Ending => b % 2 == 0,
            _ => true,
        }
    }

    pub fn tick(&mut self) {
        if matches!(
            self.state,
            ScreenState::Menu
                | ScreenState::GridSettings
                | ScreenState::HighScores
                | ScreenState::Instructions
                | ScreenState::Paused
        ) {
            return;
        }

        // Fever decay & Blitz countdown during active play
        if matches!(
            self.state,
            ScreenState::Falling | ScreenState::Collapse | ScreenState::Evaluate | ScreenState::Exploding
        ) {
            if self.fever_meter > 0.0 {
                self.fever_meter = (self.fever_meter - 0.0025).max(0.0);
            }
            if self.mode == GameMode::Blitz {
                self.blitz_ticks_remaining -= 1;
                if self.blitz_ticks_remaining <= 0 {
                    self.high_score = self.score.max(self.high_score);
                    self.record_high_score();
                    self.game_in_progress = false;
                    self.sub_tick = 0;
                    self.state = ScreenState::GameOver;
                    return;
                }
            }
        }

        if self.should_step() {
            match self.state {
                ScreenState::Intro => {
                    if self.sub_tick > 8 {
                        self.state = ScreenState::Menu;
                        self.menu_index = 0;
                    } else {
                        self.sub_tick += 1;
                    }
                }
                ScreenState::LevelExit => {
                    self.sub_tick += 1;
                    if self.sub_tick >= 10 {
                        self.level += 1;
                        self.sub_tick = 0;
                        self.state = ScreenState::LevelEnter;
                    }
                }
                ScreenState::LevelEnter => {
                    self.sub_tick += 1;
                    if self.sub_tick >= 10 {
                        self.sub_tick = 0;
                        self.state = if self.level == 1 {
                            ScreenState::SpawnFirst
                        } else {
                            ScreenState::SpawnNext
                        };
                    }
                }
                ScreenState::SpawnFirst => {
                    self.generate_piece(0);
                    self.step_spawn();
                }
                ScreenState::SpawnNext => {
                    if self.mode == GameMode::RisingTide {
                        self.tide_counter += 1;
                        if self.tide_counter >= self.tide_interval {
                            self.tide_counter = 0;
                            self.push_rising_tide_row();
                        }
                    }
                    self.step_spawn();
                }
                ScreenState::Falling => {
                    let max_piece_row = (self.total_rows as i32) - 3;
                    if self.piece_pos[1] < max_piece_row {
                        if self.check_collision(2) == 0 {
                            let prev_row = self.piece_pos[1];
                            self.piece_pos[1] += 1;
                            self.place_piece_on_board(self.piece_pos[0], prev_row);
                        } else {
                            self.just_landed = true;
                            self.state = ScreenState::Collapse;
                            self.combo = 0;
                        }
                    } else {
                        self.state = ScreenState::Collapse;
                        self.combo = 0;
                    }
                }
                ScreenState::Collapse => {
                    self.step_collapse();
                }
                ScreenState::Evaluate => {
                    self.step_evaluate();
                }
                ScreenState::Exploding => {
                    if self.sub_tick < 3 {
                        for c in 0..self.cols {
                            for r in 2..self.total_rows {
                                if self.board[c][r] == 100 {
                                    self.board[c][r] = 101;
                                } else if self.board[c][r] == 101 {
                                    self.board[c][r] = 0;
                                }
                            }
                        }
                        self.sub_tick += 1;
                        self.char_tear = 1;
                        self.char_eye = 1;
                        self.char_intro = 0;
                        self.char_hand = 3;
                        self.char_mouth = 2;
                    } else {
                        self.char_intro = 0;
                        self.char_tear = 0;
                        self.char_eye = 0;
                        self.char_mouth = 2;
                        self.char_hand = 2;
                        self.sub_tick = 0;
                        self.match_found = false;
                        self.state = ScreenState::Collapse;

                        let mult = self.fever_multiplier();
                        if matches!(self.piece_type[0], 6 | 7 | 8) {
                            self.piece_type[0] = 0;
                            self.score += 10 * mult;
                        } else if self.combo > 0 {
                            let c1 = self.combo - 1;
                            self.score += (c1 * c1 * 2 + self.cleared_in_step) * mult;
                        }
                        if self.mode == GameMode::Blitz && self.combo > 1 {
                            self.blitz_ticks_remaining += 40; // +2 seconds per chain
                        }
                        self.fever_meter = (self.fever_meter + 0.22 + 0.08 * self.combo as f32).min(1.0);
                        if self.score > self.high_score {
                            self.high_score = self.score;
                        }
                    }
                }
                ScreenState::StoneShake => match self.sub_tick {
                    0..=2 => {
                        self.char_tear = 2;
                        self.char_eye = 2;
                        self.char_mouth = self.sub_tick as usize;
                        self.char_hand = 3;
                        self.sub_tick += 1;
                    }
                    _ => {
                        self.char_intro = 0;
                        self.char_tear = 0;
                        self.char_eye = 0;
                        self.char_mouth = 2;
                        self.char_hand = 2;
                        self.sub_tick = 0;
                        self.match_found = false;
                        self.piece_pos = [self.spawn_col(), 0];
                        self.state = ScreenState::SpawnNext;
                    }
                },
                ScreenState::GameOver => {
                    if self.sub_tick == 0 {
                        self.pending_sounds.push(SoundId::GameOver);
                    }
                    if self.sub_tick >= 9 {
                        self.level = 1;
                        self.open_menu();
                    } else {
                        self.sub_tick += 1;
                    }
                }
                ScreenState::AllCleared => {
                    if self.sub_tick >= 9 {
                        self.sub_tick = 0;
                        self.state = ScreenState::Ending;
                    } else {
                        self.sub_tick += 1;
                    }
                }
                ScreenState::Ending => {
                    self.sub_tick += 1;
                    if self.sub_tick >= 60 {
                        self.reset_game_state();
                        self.state = ScreenState::Menu;
                    }
                }
                _ => {}
            }
        }

        self.tick_counter = if self.tick_counter >= 100 {
            0
        } else {
            self.tick_counter + 1
        };
    }

    pub fn push_rising_tide_row(&mut self) {
        for c in 0..self.cols {
            for r in 0..self.total_rows - 1 {
                self.board[c][r] = self.board[c][r + 1];
            }
            let val = if (c + self.level as usize + self.tide_counter as usize) % self.cols == 0 {
                9 // stone
            } else {
                (self.rand_mod(5) + 1) as u8
            };
            self.board[c][self.total_rows - 1] = val;
        }
    }

    pub fn ghost_row(&self) -> i32 {
        let max_piece_row = (self.total_rows as i32) - 3;
        let mut r = self.piece_pos[1];
        while r < max_piece_row {
            let mut hit = false;
            for c_off in 0..3 {
                // Find lowest occupied row in this column of the piece
                for r_off in (0..3).rev() {
                    if self.piece[0][r_off][c_off] > 0 {
                        let nc = self.piece_pos[0] + c_off as i32;
                        let nr = r + r_off as i32 + 1;
                        if nc >= 0
                            && (nc as usize) < self.cols
                            && nr >= 0
                            && (nr as usize) < self.total_rows
                            && self.board[nc as usize][nr as usize] > 0
                        {
                            hit = true;
                        }
                        break;
                    }
                }
            }
            if hit {
                break;
            }
            r += 1;
        }
        r
    }

    pub fn hold_current_piece(&mut self) {
        if self.state != ScreenState::Falling || self.hold_used_this_turn || self.piece_pos[1] <= 0 {
            return;
        }
        self.hold_used_this_turn = true;
        // Clear current piece from board
        for r in 0..3 {
            for c in 0..3 {
                if self.piece[0][r][c] > 0 && self.piece_pos[0] + c as i32 >= 0 {
                    let cc = (self.piece_pos[0] + c as i32) as usize;
                    let rr = (self.piece_pos[1] + r as i32) as usize;
                    if cc < self.cols && rr < self.total_rows {
                        self.board[cc][rr] = 0;
                    }
                }
            }
        }
        let cur = (self.piece[0], self.piece_colors[0], self.piece_type[0]);
        if let Some((hp, hcols, htype)) = self.hold_piece.replace(cur) {
            self.piece[0] = hp;
            self.piece_colors[0] = hcols;
            self.piece_type[0] = htype;
        } else {
            self.piece[0] = self.piece[1];
            self.piece_colors[0] = self.piece_colors[1];
            self.piece_type[0] = self.piece_type[1];
            self.generate_piece(1);
        }
        // Spawn at row 1 so the bottom cell (row 3) is immediately visible
        self.piece_pos = [self.spawn_col(), 1];
        self.place_piece_on_board(self.piece_pos[0], self.piece_pos[1]);
    }

    fn step_spawn(&mut self) {
        self.hold_used_this_turn = false;
        self.char_eye = 0;
        self.char_tear = 0;
        self.char_mouth = 2;
        self.char_hand = 2;
        self.char_intro = 0;

        for r in 0..6.min(self.total_rows) {
            if self.char_eye == 3 {
                break;
            }
            for c in 0..self.cols {
                if self.board[c][r] > 0 {
                    self.char_eye = 3;
                    self.char_tear = 3;
                    self.char_mouth = 2;
                    self.char_hand = 2;
                    break;
                }
            }
        }

        if self.state == ScreenState::SpawnNext {
            self.piece_type[0] = self.piece_type[1];
            self.piece[0] = self.piece[1];
            self.piece_colors[0] = self.piece_colors[1];
        }
        self.generate_piece(1);
        self.piece_pos = [self.spawn_col(), 0];
        self.place_piece_on_board(self.piece_pos[0], self.piece_pos[1]);
        self.state = ScreenState::Falling;
    }

    fn step_collapse(&mut self) {
        for c in (0..self.cols).rev() {
            for r in (0..self.total_rows).rev() {
                if self.board[c][r] > 0 {
                    let mut empty_below = 0;
                    for k in (r + 1)..self.total_rows {
                        if self.board[c][k] == 0 {
                            empty_below += 1;
                        }
                    }
                    if empty_below > 0 {
                        let val = self.board[c][r];
                        self.board[c][r] = 0;
                        self.board[c][r + empty_below] = val;
                        if val == 7 || val == 8 {
                            self.piece_pos[1] = (r + empty_below) as i32;
                        }
                    }
                }
            }
        }
        if self.match_found {
            self.char_tear = 1;
            self.char_eye = 1;
            self.char_intro = 0;
            self.char_hand = 3;
            self.char_mouth = 2;
        }
        self.just_landed = false;
        self.state = ScreenState::Evaluate;
    }

    fn step_evaluate(&mut self) {
        self.cleared_in_step = 0;
        let ptype = self.piece_type[0];
        if ptype == 6 {
            let col = (self.piece_pos[0] + 1) as usize;
            if col < self.cols {
                for r in 0..self.total_rows {
                    if self.board[col][r] > 0 {
                        self.board[col][r] = 100;
                        self.cleared_in_step += 1;
                    }
                }
            }
            self.match_found = true;
        } else if ptype == 7 {
            let mut bomb_pos = None;
            let pref_col = (self.piece_pos[0] + 1).clamp(0, self.cols as i32 - 1) as usize;
            for r in 0..self.total_rows {
                if self.board[pref_col][r] == 7 {
                    bomb_pos = Some((pref_col as i32, r as i32));
                    break;
                }
            }
            if bomb_pos.is_none() {
                'outer_b: for c in 0..self.cols {
                    for r in 0..self.total_rows {
                        if self.board[c][r] == 7 {
                            bomb_pos = Some((c as i32, r as i32));
                            break 'outer_b;
                        }
                    }
                }
            }
            if let Some((bc, br)) = bomb_pos {
                for c in bc - 1..=bc + 1 {
                    for r in br - 1..=br + 1 {
                        if c >= 0
                            && (c as usize) < self.cols
                            && r >= 0
                            && (r as usize) < self.total_rows
                            && self.board[c as usize][r as usize] > 0
                        {
                            self.board[c as usize][r as usize] = 100;
                            self.cleared_in_step += 1;
                        }
                    }
                }
            }
            self.match_found = true;
        } else {
            let mut target_below = 16u8;
            let mut star_self = 0u8;
            if ptype == 8 {
                self.combo = 0;
                let pref_col = (self.piece_pos[0] + 1).clamp(0, self.cols as i32 - 1) as usize;
                let mut star_pos = None;
                for r in 0..self.total_rows {
                    if self.board[pref_col][r] == 8 {
                        star_pos = Some((pref_col, r));
                        break;
                    }
                }
                if let Some((col, row)) = star_pos {
                    star_self = self.board[col][row];
                    if row + 1 < self.total_rows {
                        target_below = self.board[col][row + 1];
                    }
                }
            }
            for c in (0..self.cols).rev() {
                for r in (0..self.total_rows).rev() {
                    let val = self.board[c][r];
                    if val > 0 {
                        if ptype == 8 {
                            if val == target_below || val == star_self {
                                self.board[c][r] = 100;
                                self.cleared_in_step += 1;
                                self.match_found = true;
                            }
                        } else if val != 9 && val != 10 && val < 100 {
                            self.check_matches_from(c, r, val);
                        }
                    }
                }
            }
        }

        if self.match_found {
            for r in 0..self.total_rows {
                for c in 0..self.cols {
                    if self.match_mask[c][r] == 1 {
                        self.match_mask[c][r] = 0;
                        self.board[c][r] = 100;
                        self.cleared_in_step += 1;
                    }
                }
            }
            self.char_tear = 1;
            self.char_eye = 1;
            self.char_intro = 0;
            self.char_hand = 3;
            self.char_mouth = 2;

            if self.piece_type[0] <= 5 {
                self.combo += 1;
            }
            self.sub_tick = 0;
            if self.combo > 1 {
                self.match_found = false;
                self.pending_sounds.push(SoundId::Combo);
            } else if matches!(self.piece_type[0], 6 | 7 | 8) {
                self.pending_sounds.push(SoundId::Special);
            } else {
                self.pending_sounds.push(SoundId::Clear);
            }
            self.state = ScreenState::Exploding;
        } else {
            self.combo = 0;
            if self.check_collision(3) == 1 {
                self.high_score = self.score;
                self.record_high_score();
                self.score = 0;
                self.game_in_progress = false;
                self.sub_tick = 0;
                self.state = ScreenState::GameOver;
            } else if self.piece[0][2][1] == 9 || self.piece[0][2][1] == 10 {
                self.sub_tick = 0;
                self.state = ScreenState::StoneShake;
            } else {
                self.piece_pos = [self.spawn_col(), 0];
                self.state = ScreenState::SpawnNext;
                self.check_level_up();
            }
        }
    }

    fn check_level_up(&mut self) {
        if 30 * self.level * self.level <= self.score {
            if self.mode == GameMode::Classic {
                if self.level <= 10 {
                    self.sub_tick = 0;
                    self.combo = 0;
                    self.pending_sounds.push(SoundId::LevelUp);
                    if self.level + 1 >= 11 {
                        self.level += 1;
                        self.high_score = self.score;
                        self.record_high_score();
                        self.score = 0;
                        self.level = 1;
                        self.state = ScreenState::AllCleared;
                    } else {
                        self.state = ScreenState::LevelExit;
                    }
                }
            } else {
                self.sub_tick = 0;
                self.combo = 0;
                self.pending_sounds.push(SoundId::LevelUp);
                self.tide_interval = (8u32.saturating_sub(self.level / 2)).max(4);
                self.state = ScreenState::LevelExit;
            }
        }
    }

    fn check_matches_from(&mut self, col: usize, row: usize, target: u8) {
        let max_len = self.cols.max(self.total_rows);
        let mut pts = vec![[0usize; 2]; max_len];
        let mut len = 0usize;

        let push_or_reset = |c: usize,
                             r: usize,
                             len: &mut usize,
                             pts: &mut Vec<[usize; 2]>,
                             m: &mut Vec<Vec<u8>>,
                             f: &mut bool| {
            if self.board[c][r] == target {
                pts[*len] = [c, r];
                *len += 1;
                if *len >= 3 {
                    *f = true;
                    for p in &pts[..*len] {
                        m[p[0]][p[1]] = 1;
                    }
                }
            } else {
                *len = 0;
            }
        };

        for c in 0..self.cols {
            push_or_reset(c, row, &mut len, &mut pts, &mut self.match_mask, &mut self.match_found);
        }
        len = 0;
        for r in 0..self.total_rows {
            push_or_reset(col, r, &mut len, &mut pts, &mut self.match_mask, &mut self.match_found);
        }
        len = 0;
        let (sc, sr) = if col >= row { (col - row, 0) } else { (0, row - col) };
        for d in 0..max_len {
            if sc + d < self.cols && sr + d < self.total_rows {
                push_or_reset(sc + d, sr + d, &mut len, &mut pts, &mut self.match_mask, &mut self.match_found);
            }
        }
        len = 0;
        let right = self.cols - 1;
        let (sc2, sr2) = if right - col >= row {
            (col + row, 0)
        } else {
            (right, row - (right - col))
        };
        for d in 0..max_len {
            if sc2 >= d && sr2 + d < self.total_rows {
                push_or_reset(sc2 - d, sr2 + d, &mut len, &mut pts, &mut self.match_mask, &mut self.match_found);
            }
        }
    }

    fn generate_piece(&mut self, slot: usize) {
        loop {
            let ptype = self.rand_mod(9) as usize;
            self.piece_type[slot] = ptype;
            if ptype == 6 || ptype == 7 || ptype == 8 {
                if self.special_throttle[0] >= 15 {
                    self.special_throttle[0] = 0;
                    break;
                }
                self.special_throttle[0] += 1;
                if self.special_throttle[1] >= 12 {
                    self.special_throttle[1] = 0;
                    break;
                }
                self.special_throttle[1] += 1;
                if self.special_throttle[2] >= 10 {
                    self.special_throttle[2] = 0;
                    break;
                }
                self.special_throttle[2] += 1;
            } else {
                let threshold = if self.mode == GameMode::Endless {
                    24u32.saturating_sub(self.level).max(6)
                } else {
                    (10u32.saturating_sub(self.level.min(9)) * 2).max(4)
                };
                if self.stone_counter >= threshold {
                    self.stone_counter = 0;
                    self.stone_flag = 9;
                } else {
                    self.stone_flag = 0;
                    self.stone_counter += 1;
                }
                break;
            }
        }

        let ptype = self.piece_type[slot];
        if ptype < 6 {
            for i in 0..3 {
                self.piece_colors[slot][i] = if self.stone_flag == 9 {
                    9
                } else {
                    (self.rand_mod(5) + 1) as u8
                };
            }
        } else {
            self.piece_colors[slot][0] = ptype as u8;
        }

        let mut cidx = 0;
        for r in 0..3 {
            for c in 0..3 {
                if SHAPES[ptype][r][c] == 1 {
                    self.piece[slot][r][c] = self.piece_colors[slot][cidx];
                    cidx += 1;
                } else {
                    self.piece[slot][r][c] = 0;
                }
            }
        }
    }

    fn check_collision(&self, dir: u8) -> u8 {
        match dir {
            1 => {
                for r in 0..3 {
                    if self.piece[0][2][2] > 0 {
                        let nc = self.piece_pos[0] + 3;
                        let nr = self.piece_pos[1] + r as i32;
                        if self.piece[0][r][2] > 0
                            && nc >= 0
                            && (nc as usize) < self.cols
                            && nr >= 0
                            && (nr as usize) < self.total_rows
                            && self.board[nc as usize][nr as usize] > 0
                        {
                            return 1;
                        }
                    } else {
                        let nc = self.piece_pos[0] + 2;
                        let nr = self.piece_pos[1] + r as i32;
                        if self.piece[0][r][1] > 0
                            && nc >= 0
                            && (nc as usize) < self.cols
                            && nr >= 0
                            && (nr as usize) < self.total_rows
                            && self.board[nc as usize][nr as usize] > 0
                        {
                            return 1;
                        }
                    }
                }
            }
            0 => {
                for r in 0..3 {
                    if self.piece[0][2][0] > 0 {
                        let nc = self.piece_pos[0] - 1;
                        let nr = self.piece_pos[1] + r as i32;
                        if self.piece[0][r][0] > 0
                            && self.piece_pos[0] > 0
                            && (nc as usize) < self.cols
                            && nr >= 0
                            && (nr as usize) < self.total_rows
                            && self.board[nc as usize][nr as usize] > 0
                        {
                            return 1;
                        }
                    } else {
                        let nc = self.piece_pos[0];
                        let nr = self.piece_pos[1] + r as i32;
                        if self.piece[0][r][1] > 0
                            && self.piece_pos[0] > -1
                            && nc >= 0
                            && (nc as usize) < self.cols
                            && nr >= 0
                            && (nr as usize) < self.total_rows
                            && self.board[nc as usize][nr as usize] > 0
                        {
                            return 1;
                        }
                    }
                }
            }
            2 => {
                for c in 0..3 {
                    let nc = self.piece_pos[0] + c as i32;
                    let nr = self.piece_pos[1] + 3;
                    if self.piece[0][2][c] > 0
                        && nc >= 0
                        && (nc as usize) < self.cols
                        && nr >= 0
                        && (nr as usize) < self.total_rows
                        && self.board[nc as usize][nr as usize] > 0
                    {
                        return 1;
                    }
                }
            }
            3 => {
                for c in 0..self.cols {
                    if self.board[c][1] > 0 || self.board[c][2] > 0 {
                        return 1;
                    }
                }
            }
            _ => {}
        }
        0
    }

    fn place_piece_on_board(&mut self, old_col: i32, old_row: i32) {
        for r in 0..3 {
            for c in 0..3 {
                if self.piece[0][r][c] > 0 && old_col + c as i32 >= 0 {
                    let cc = (old_col + c as i32) as usize;
                    let rr = (old_row + r as i32) as usize;
                    if cc < self.cols && rr < self.total_rows {
                        self.board[cc][rr] = 0;
                    }
                }
            }
        }
        for r in 0..3 {
            for c in 0..3 {
                if self.piece[0][r][c] > 0 && self.piece_pos[0] + c as i32 >= 0 {
                    let cc = (self.piece_pos[0] + c as i32) as usize;
                    let rr = (self.piece_pos[1] + r as i32) as usize;
                    if cc < self.cols && rr < self.total_rows {
                        self.board[cc][rr] = self.piece[0][r][c];
                    }
                }
            }
        }
    }

    pub fn move_left(&mut self) {
        if self.state != ScreenState::Falling || self.piece_pos[1] <= 0 {
            return;
        }
        let prev_col = self.piece_pos[0];
        let prev_row = self.piece_pos[1];
        if self.piece[0][2][0] == 0 {
            if self.piece_pos[0] > -1 && self.check_collision(0) == 0 {
                self.piece_pos[0] -= 1;
            }
        } else if self.piece_pos[0] > 0 && self.check_collision(0) == 0 {
            self.piece_pos[0] -= 1;
        }
        self.place_piece_on_board(prev_col, prev_row);
    }

    pub fn move_right(&mut self) {
        if self.state != ScreenState::Falling || self.piece_pos[1] <= 0 {
            return;
        }
        let prev_col = self.piece_pos[0];
        let prev_row = self.piece_pos[1];
        let max_c_narrow = (self.cols as i32) - 2;
        let max_c_wide = (self.cols as i32) - 3;
        if self.piece[0][2][2] == 0 {
            if self.piece_pos[0] < max_c_narrow && self.check_collision(1) == 0 {
                self.piece_pos[0] += 1;
            }
        } else if self.piece_pos[0] < max_c_wide && self.check_collision(1) == 0 {
            self.piece_pos[0] += 1;
        }
        self.place_piece_on_board(prev_col, prev_row);
    }

    pub fn rotate_piece(&mut self) {
        if self.state != ScreenState::Falling || self.piece_pos[1] <= 0 {
            return;
        }
        let prev_col = self.piece_pos[0];
        let prev_row = self.piece_pos[1];
        let ptype = self.piece_type[0];
        if ptype <= 3 {
            let t = self.piece_colors[0][0];
            self.piece_colors[0][0] = self.piece_colors[0][1];
            self.piece_colors[0][1] = self.piece_colors[0][2];
            self.piece_colors[0][2] = t;
        } else if ptype == 4 || ptype == 5 {
            self.piece_colors[0].swap(0, 1);
        }
        if ptype <= 5 {
            let mut idx = 0;
            for r in 0..3 {
                for c in 0..3 {
                    if SHAPES[ptype][r][c] == 1 {
                        self.piece[0][r][c] = self.piece_colors[0][idx];
                        idx += 1;
                    } else {
                        self.piece[0][r][c] = 0;
                    }
                }
            }
        }
        self.place_piece_on_board(prev_col, prev_row);
    }

    pub fn fast_drop(&mut self) {
        if self.state != ScreenState::Falling || self.piece_pos[1] <= 0 {
            return;
        }
        self.state = ScreenState::Collapse;
        self.combo = 0;
        self.just_landed = true;
    }

    fn record_high_score(&mut self) {
        if self.score > 0 {
            let now = Local::now();
            let year = now.format("%Y").to_string();
            let month = now.format("%m").to_string();
            let day = now.format("%d").to_string();
            for i in 0..5 {
                if self.score > self.high_scores[i].score {
                    for j in (i + 1..5).rev() {
                        self.high_scores[j] = self.high_scores[j - 1].clone();
                    }
                    self.high_scores[i] = HighScoreEntry {
                        score: self.score,
                        level: self.level,
                        year,
                        month,
                        day,
                    };
                    break;
                } else if self.score == self.high_scores[i].score {
                    break;
                }
            }
        }
        self.save_high_scores();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_three_and_score() {
        let mut g = Game::new(42);
        g.reset_game_state();
        g.board[0][12] = 1;
        g.board[1][12] = 1;
        g.board[2][12] = 1;
        g.piece_type[0] = 0;
        g.step_evaluate();
        assert_eq!(g.state, ScreenState::Exploding);
        assert_eq!(g.cleared_in_step, 3);
    }

    #[test]
    fn custom_grid_size_10x14() {
        let mut g = Game::new(99);
        g.set_grid_size(10, 14);
        assert_eq!(g.cols, 10);
        assert_eq!(g.visible_rows, 14);
        assert_eq!(g.total_rows, 17);
        g.board[7][16] = 2;
        g.board[8][16] = 2;
        g.board[9][16] = 2;
        g.piece_type[0] = 0;
        g.step_evaluate();
        assert_eq!(g.state, ScreenState::Exploding);
        assert_eq!(g.cleared_in_step, 3);
    }

    #[test]
    fn hold_and_ghost_work() {
        let mut g = Game::new(5);
        g.reset_game_state();
        g.generate_piece(0);
        g.generate_piece(1);
        g.piece_pos = [2, 2];
        g.state = ScreenState::Falling;
        let orig_type = g.piece_type[0];
        let next_type = g.piece_type[1];
        let ghost = g.ghost_row();
        assert!(ghost >= 2);
        g.hold_current_piece();
        assert!(g.hold_piece.is_some());
        assert_eq!(g.hold_piece.unwrap().2, orig_type);
        assert_eq!(g.piece_type[0], next_type);
        assert!(g.hold_used_this_turn);
        assert_eq!(g.piece_pos[1], 1);
        g.rotate_piece();
    }

    #[test]
    fn endless_continues_past_level_10() {
        let mut g = Game::new(11);
        g.mode = GameMode::Endless;
        g.reset_game_state();
        g.level = 10;
        g.score = 3500;
        g.check_level_up();
        assert_eq!(g.state, ScreenState::LevelExit);
    }

    #[test]
    fn endless_difficulty_ramps_50ms_per_level() {
        let mut g = Game::new(12);
        g.mode = GameMode::Endless;
        g.reset_game_state();
        g.state = ScreenState::Falling;

        g.level = 1;
        g.tick_counter = 21;
        assert!(g.should_step());

        g.level = 2;
        g.tick_counter = 20;
        assert!(g.should_step());

        g.level = 3;
        g.tick_counter = 19;
        assert!(g.should_step());
    }

    #[test]
    fn high_scores_isolated_per_grid_and_mode() {
        let mut g = Game::new(17);
        g.high_scores_by_grid.clear();
        g.mode = GameMode::Classic;
        g.set_grid_size(6, 10);
        g.score = 150;
        g.record_high_score();
        assert_eq!(g.high_scores[0].score, 150);

        g.mode = GameMode::Endless;
        g.sync_high_scores_for_key();
        assert_eq!(g.high_scores[0].score, 0);

        g.set_grid_size(8, 12);
        assert_eq!(g.high_scores[0].score, 0);
        assert_eq!(g.high_score, 0);

        g.mode = GameMode::Classic;
        g.set_grid_size(6, 10);
        assert_eq!(g.high_scores[0].score, 150);
    }

    #[test]
    fn bomb_clears_3x3() {
        let mut g = Game::new(9);
        g.reset_game_state();
        g.piece_type[0] = 7;
        // Normal landing: piece_pos[1] is 9 (top of 3x3), so bomb at row_off=2 is at (2, 11)
        g.piece_pos = [1, 9];
        g.board[2][11] = 7;
        g.board[1][11] = 2;
        g.board[2][12] = 3;
        g.board[3][12] = 4;
        g.step_collapse();
        g.step_evaluate();
        assert_eq!(g.state, ScreenState::Exploding);
        assert_eq!(g.cleared_in_step, 4);
        assert_eq!(g.board[2][11], 100);
    }
}
