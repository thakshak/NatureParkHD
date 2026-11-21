mod audio;
mod game;
mod hd_sprites;

use audio::AudioEngine;
use game::{Game, GameMode, MAX_COLS, MAX_VISIBLE_ROWS, MIN_COLS, MIN_VISIBLE_ROWS, ScreenState};
use hd_sprites::{generate_hd_block_textures, load_hd_sprite};
use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

const TICK_SEC: f32 = 0.050;

struct CharacterAssets {
    bunny_face: Texture2D,
    bunny_eye: [Texture2D; 4],
    bunny_tear: Texture2D,
    bunny_mouth: [Texture2D; 3],
    bunny_hand: [Texture2D; 3],

    du_face: Texture2D,
    du_eye: [Texture2D; 4],
    du_tear: [Texture2D; 2],
    du_mouth: [Texture2D; 3],
    du_hand: [Texture2D; 2],

    fenny_face: Texture2D,
    fenny_body: Texture2D,
    fenny_eye: [Texture2D; 4],
    fenny_tear: Texture2D,
    fenny_face2: Texture2D,
    fenny_tongue: Texture2D,
    fenny_face3: Texture2D,
    fenny_hand: [Texture2D; 2],

    pan_face: Texture2D,
    pan_eye: [Texture2D; 3],
    pan_tear: [Texture2D; 2],
    pan_mouth: [Texture2D; 4],
    pan_hand: [Texture2D; 2],
}

struct Assets {
    intro_buble: Texture2D,
    intro_flower1: Texture2D,
    intro_flower3: Texture2D,
    intro_title: Texture2D,
    intro_bigstar: Texture2D,
    intro_midstar: Texture2D,
    intro_smallstar: Texture2D,
    baguni: Texture2D,
    cloud: Texture2D,

    blocks: [Texture2D; 5],
    special: [Texture2D; 4],
    explosion: [Texture2D; 2],

    main_wave: Texture2D,
    main_boat: Texture2D,
    wheel: Texture2D,
    combo: Texture2D,
    combo_star: Texture2D,
    combo_digits: [Texture2D; 10],
    chars: CharacterAssets,
}

impl Assets {
    async fn load() -> Self {
        let (blocks, special, explosion) = generate_hd_block_textures();
        Self {
            intro_buble: load_hd_sprite("assets/intro_buble.png").await,
            intro_flower1: load_hd_sprite("assets/intro_flower1.png").await,
            intro_flower3: load_hd_sprite("assets/intro_flower3.png").await,
            intro_title: load_hd_sprite("assets/intro_title.png").await,
            intro_bigstar: load_hd_sprite("assets/intro_bigstar.png").await,
            intro_midstar: load_hd_sprite("assets/intro_midstar.png").await,
            intro_smallstar: load_hd_sprite("assets/intro_smallstar.png").await,
            baguni: load_hd_sprite("assets/baguni.png").await,
            cloud: load_hd_sprite("assets/cloud.png").await,
            blocks,
            special,
            explosion,
            main_wave: load_hd_sprite("assets/main_wave.png").await,
            main_boat: load_hd_sprite("assets/main_boat.png").await,
            wheel: load_hd_sprite("assets/wheel.png").await,
            combo: load_hd_sprite("assets/combo.png").await,
            combo_star: load_hd_sprite("assets/combo_star.png").await,
            combo_digits: [
                load_hd_sprite("assets/combo_0.png").await,
                load_hd_sprite("assets/combo_1.png").await,
                load_hd_sprite("assets/combo_2.png").await,
                load_hd_sprite("assets/combo_3.png").await,
                load_hd_sprite("assets/combo_4.png").await,
                load_hd_sprite("assets/combo_5.png").await,
                load_hd_sprite("assets/combo_6.png").await,
                load_hd_sprite("assets/combo_7.png").await,
                load_hd_sprite("assets/combo_8.png").await,
                load_hd_sprite("assets/combo_9.png").await,
            ],
            chars: CharacterAssets {
                bunny_face: load_hd_sprite("assets/bunny_face.png").await,
                bunny_eye: [
                    load_hd_sprite("assets/bunny_eye_01.png").await,
                    load_hd_sprite("assets/bunny_eye_02.png").await,
                    load_hd_sprite("assets/bunny_eye_04.png").await,
                    load_hd_sprite("assets/bunny_eye_03.png").await,
                ],
                bunny_tear: load_hd_sprite("assets/bunny_tear_01.png").await,
                bunny_mouth: [
                    load_hd_sprite("assets/bunny_mouth_01.png").await,
                    load_hd_sprite("assets/bunny_mouth_02.png").await,
                    load_hd_sprite("assets/bunny_mouth_03_04.png").await,
                ],
                bunny_hand: [
                    load_hd_sprite("assets/bunny_hand_left.png").await,
                    load_hd_sprite("assets/bunny_hand_right.png").await,
                    load_hd_sprite("assets/bunny_hand_intro.png").await,
                ],
                du_face: load_hd_sprite("assets/du_face.png").await,
                du_eye: [
                    load_hd_sprite("assets/du_eye_01.png").await,
                    load_hd_sprite("assets/du_eye_02.png").await,
                    load_hd_sprite("assets/du_eye_04.png").await,
                    load_hd_sprite("assets/du_eye_03.png").await,
                ],
                du_tear: [
                    load_hd_sprite("assets/du_tear_01.png").await,
                    load_hd_sprite("assets/du_tear_02.png").await,
                ],
                du_mouth: [
                    load_hd_sprite("assets/du_mouth_01.png").await,
                    load_hd_sprite("assets/du_mouth_02.png").await,
                    load_hd_sprite("assets/du_mouth_03.png").await,
                ],
                du_hand: [
                    load_hd_sprite("assets/du_hand_left.png").await,
                    load_hd_sprite("assets/du_hand_right.png").await,
                ],
                fenny_face: load_hd_sprite("assets/fenny_face_01.png").await,
                fenny_body: load_hd_sprite("assets/fenny_body.png").await,
                fenny_eye: [
                    load_hd_sprite("assets/fenny_eye_01.png").await,
                    load_hd_sprite("assets/fenny_eye_02.png").await,
                    load_hd_sprite("assets/fenny_eye_04.png").await,
                    load_hd_sprite("assets/fenny_eye_03.png").await,
                ],
                fenny_tear: load_hd_sprite("assets/fenny_tear_01.png").await,
                fenny_face2: load_hd_sprite("assets/fenny_face_02.png").await,
                fenny_tongue: load_hd_sprite("assets/fenny_tongue.png").await,
                fenny_face3: load_hd_sprite("assets/fenny_face_03.png").await,
                fenny_hand: [
                    load_hd_sprite("assets/fenny_hand_left.png").await,
                    load_hd_sprite("assets/fenny_hand_right.png").await,
                ],
                pan_face: load_hd_sprite("assets/pan_face.png").await,
                pan_eye: [
                    load_hd_sprite("assets/pan_eye_02.png").await,
                    load_hd_sprite("assets/pan_eye_04.png").await,
                    load_hd_sprite("assets/pan_eye_03.png").await,
                ],
                pan_tear: [
                    load_hd_sprite("assets/pan_tear_01.png").await,
                    load_hd_sprite("assets/pan_tear_02.png").await,
                ],
                pan_mouth: [
                    load_hd_sprite("assets/pan_mouth_01.png").await,
                    load_hd_sprite("assets/pan_mouth_02.png").await,
                    load_hd_sprite("assets/pan_mouth_04.png").await,
                    load_hd_sprite("assets/pan_mouth_03.png").await,
                ],
                pan_hand: [
                    load_hd_sprite("assets/pan_hand_left.png").await,
                    load_hd_sprite("assets/pan_hand_right.png").await,
                ],
            },
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Nature Park HD".to_owned(),
        window_width: 960,
        window_height: 720,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let assets = Assets::load().await;
    let mut audio = AudioEngine::new();

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u32)
        .unwrap_or(54321);
    let mut game = Game::new(seed);
    let mut accum = 0.0_f32;

    loop {
        if game.should_exit {
            break;
        }

        if is_key_pressed(KeyCode::M) {
            audio.muted = !audio.muted;
            if audio.muted {
                audio.stop();
            }
        }

        handle_input(&mut game);

        accum += get_frame_time().min(0.25);
        while accum >= TICK_SEC {
            accum -= TICK_SEC;
            game.tick();
        }

        for snd in game.pending_sounds.drain(..) {
            audio.play(snd);
        }

        clear_background(c24(0x13273D));
        draw_screen(&game, &assets);
        next_frame().await;
    }
}

fn handle_input(game: &mut Game) {
    match game.state {
        ScreenState::Intro => {
            if get_last_key_pressed().is_some() || is_mouse_button_pressed(MouseButton::Left) {
                game.open_menu();
            }
        }
        ScreenState::Menu => {
            let items = game.menu_items();
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                game.menu_index = if game.menu_index == 0 {
                    items.len() - 1
                } else {
                    game.menu_index - 1
                };
            } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                game.menu_index = (game.menu_index + 1) % items.len();
            } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                game.select_menu_item();
            } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Right) {
                let items = game.menu_items();
                if let Some(choice) = items.get(game.menu_index) {
                    if choice.starts_with("Mode:") || choice.starts_with("Blind Next:") {
                        game.select_menu_item();
                    }
                }
            } else if is_key_pressed(KeyCode::Escape) {
                if game.game_in_progress {
                    game.continue_game();
                } else {
                    game.should_exit = true;
                }
            }
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                let sw = screen_width();
                for (i, _) in items.iter().enumerate() {
                    let y = 175.0 + (i as f32) * 54.0;
                    if mx >= sw * 0.5 - 200.0
                        && mx <= sw * 0.5 + 200.0
                        && my >= y - 34.0
                        && my <= y + 12.0
                    {
                        game.menu_index = i;
                        game.select_menu_item();
                        break;
                    }
                }
            }
        }
        ScreenState::GridSettings => {
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                game.grid_setting_row = if game.grid_setting_row == 0 {
                    2
                } else {
                    game.grid_setting_row - 1
                };
            } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                game.grid_setting_row = (game.grid_setting_row + 1) % 3;
            } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                match game.grid_setting_row {
                    0 => game.adjust_cols(-1),
                    1 => game.adjust_rows(-1),
                    _ => {}
                }
            } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                match game.grid_setting_row {
                    0 => game.adjust_cols(1),
                    1 => game.adjust_rows(1),
                    _ => {}
                }
            } else if is_key_pressed(KeyCode::Enter)
                || is_key_pressed(KeyCode::Space)
                || is_key_pressed(KeyCode::Escape)
            {
                if game.grid_setting_row == 2 || is_key_pressed(KeyCode::Escape) {
                    game.state = ScreenState::Menu;
                }
            }
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                let sw = screen_width();
                let cx = sw * 0.5;
                // Row 0: Columns (y = 240), Row 1: Rows (y = 316), Row 2: Done (y = 392)
                for row_idx in 0..2 {
                    let y = 240.0 + (row_idx as f32) * 76.0;
                    if my >= y - 38.0 && my <= y + 16.0 {
                        game.grid_setting_row = row_idx;
                        // '-' button: [cx - 35 .. cx + 10], '+' button: [cx + 75 .. cx + 120]
                        if mx >= cx - 35.0 && mx <= cx + 10.0 {
                            if row_idx == 0 {
                                game.adjust_cols(-1);
                            } else {
                                game.adjust_rows(-1);
                            }
                        } else if mx >= cx + 75.0 && mx <= cx + 120.0 {
                            if row_idx == 0 {
                                game.adjust_cols(1);
                            } else {
                                game.adjust_rows(1);
                            }
                        }
                    }
                }
                let done_y = 240.0 + 2.0 * 76.0;
                if mx >= cx - 250.0 && mx <= cx + 250.0 && my >= done_y - 38.0 && my <= done_y + 16.0 {
                    game.state = ScreenState::Menu;
                }
            }
        }
        ScreenState::HighScores | ScreenState::Instructions => {
            if is_key_pressed(KeyCode::Escape)
                || is_key_pressed(KeyCode::Enter)
                || is_key_pressed(KeyCode::Space)
            {
                game.state = ScreenState::Menu;
            }
        }
        ScreenState::Ending => {
            if get_last_key_pressed().is_some() {
                game.open_menu();
            }
        }
        ScreenState::Paused => {
            if is_key_pressed(KeyCode::P)
                || is_key_pressed(KeyCode::Enter)
                || is_key_pressed(KeyCode::Space)
            {
                game.toggle_pause();
            } else if is_key_pressed(KeyCode::Escape) {
                game.open_menu();
            } else if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                let sw = screen_width();
                let sh = screen_height();
                if mx >= sw - 236.0 && mx <= sw - 132.0 && my >= 10.0 && my <= 46.0 {
                    game.open_menu();
                } else if mx >= sw - 124.0 && mx <= sw - 20.0 && my >= 10.0 && my <= 46.0 {
                    game.toggle_pause();
                } else if my >= sh * 0.5 + 10.0 && my <= sh * 0.5 + 58.0 {
                    if mx >= sw * 0.5 - 165.0 && mx <= sw * 0.5 - 15.0 {
                        game.toggle_pause();
                    } else if mx >= sw * 0.5 + 15.0 && mx <= sw * 0.5 + 175.0 {
                        game.open_menu();
                    }
                }
            }
        }
        ScreenState::Falling => {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                let sw = screen_width();
                if mx >= sw - 236.0 && mx <= sw - 132.0 && my >= 10.0 && my <= 46.0 {
                    game.open_menu();
                    return;
                }
                if mx >= sw - 124.0 && mx <= sw - 20.0 && my >= 10.0 && my <= 46.0 {
                    game.toggle_pause();
                    return;
                }
            }
            if is_key_pressed(KeyCode::P) {
                game.toggle_pause();
            } else if is_key_pressed(KeyCode::Escape) {
                game.open_menu();
            } else if is_key_pressed(KeyCode::Left)
                || is_key_pressed(KeyCode::A)
                || is_key_pressed(KeyCode::Key4)
            {
                game.move_left();
            } else if is_key_pressed(KeyCode::Right)
                || is_key_pressed(KeyCode::D)
                || is_key_pressed(KeyCode::Key6)
            {
                game.move_right();
            } else if is_key_pressed(KeyCode::Up)
                || is_key_pressed(KeyCode::W)
                || is_key_pressed(KeyCode::Key2)
                || is_key_pressed(KeyCode::Z)
            {
                game.rotate_piece();
            } else if is_key_pressed(KeyCode::Down)
                || is_key_pressed(KeyCode::S)
                || is_key_pressed(KeyCode::Space)
                || is_key_pressed(KeyCode::Key5)
                || is_key_pressed(KeyCode::Key8)
            {
                game.fast_drop();
            } else if is_key_pressed(KeyCode::C)
                || is_key_pressed(KeyCode::LeftShift)
                || is_key_pressed(KeyCode::RightShift)
            {
                game.hold_current_piece();
            }
        }
        _ => {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                let sw = screen_width();
                if mx >= sw - 236.0 && mx <= sw - 132.0 && my >= 10.0 && my <= 46.0 {
                    game.open_menu();
                    return;
                }
                if mx >= sw - 124.0 && mx <= sw - 20.0 && my >= 10.0 && my <= 46.0 {
                    game.toggle_pause();
                    return;
                }
            }
            if is_key_pressed(KeyCode::P) {
                game.toggle_pause();
            } else if is_key_pressed(KeyCode::Escape) {
                game.open_menu();
            }
        }
    }
}

fn c24(rgb: u32) -> Color {
    Color::from_rgba(
        ((rgb >> 16) & 0xFF) as u8,
        ((rgb >> 8) & 0xFF) as u8,
        (rgb & 0xFF) as u8,
        255,
    )
}

fn draw_tex_scaled(tex: &Texture2D, x: f32, y: f32, scale: f32) {
    // Textures are 4x EPX-upscaled from original, so 1 original px = 0.25 * tex.width()
    let w = (tex.width() / 4.0) * scale;
    let h = (tex.height() / 4.0) * scale;
    draw_texture_ex(
        tex,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(w, h)),
            ..Default::default()
        },
    );
}

fn draw_character_hd(
    assets: &Assets,
    kind: usize,
    x: f32,
    y: f32,
    scale: f32,
    eye: usize,
    tear: usize,
    mouth: usize,
    intro_hand: usize,
    with_buble: bool,
    with_wave: bool,
    bob: f32,
) {
    let ch = &assets.chars;
    let p = |dx: f32, dy: f32| (x + dx * scale, y + dy * scale);
    match kind {
        0 => {
            draw_tex_scaled(&assets.main_boat, x, y + 8.0 * scale, scale);
            if !with_buble {
                draw_tex_scaled(&assets.intro_buble, x - 8.0 * scale, y - 13.0 * scale + bob * scale, scale);
            }
            draw_tex_scaled(&ch.bunny_face, x, y, scale);
            match eye {
                0 => {
                    let (ex, ey) = p(18.0, 29.0);
                    draw_tex_scaled(&ch.bunny_eye[0], ex, ey, scale);
                }
                1 => {
                    let (ex, ey) = p(16.0, 31.0);
                    draw_tex_scaled(&ch.bunny_eye[1], ex, ey, scale);
                }
                2 => {
                    let (ex, ey) = p(15.0, 29.0);
                    draw_tex_scaled(&ch.bunny_eye[2], ex, ey, scale);
                    if tear < 2 {
                        let (tx, ty) = p(16.0, 33.0);
                        draw_tex_scaled(&ch.bunny_tear, tx, ty, scale);
                        if tear == 1 {
                            let (tx2, ty2) = p(16.0, 37.0);
                            draw_tex_scaled(&ch.bunny_tear, tx2, ty2, scale);
                        }
                    }
                }
                _ => {
                    let (ex, ey) = p(14.0, 25.0);
                    draw_tex_scaled(&ch.bunny_eye[3], ex, ey, scale);
                }
            }
            let m_idx = if mouth == 3 { 2 } else { mouth.min(2) };
            let (mx, my) = p(22.0, 39.0);
            draw_tex_scaled(&ch.bunny_mouth[m_idx], mx, my, scale);
            let (wx, wy) = p(18.0, 47.0);
            draw_tex_scaled(&assets.wheel, wx, wy, scale);
            let (rx, ry) = p(33.0, 46.0);
            draw_tex_scaled(&ch.bunny_hand[1], rx, ry, scale);
            if intro_hand == 0 {
                let (lx, ly) = p(15.0, 46.0);
                draw_tex_scaled(&ch.bunny_hand[0], lx, ly, scale);
            } else if intro_hand == 1 {
                let (lx, ly) = p(0.0, 28.0);
                draw_tex_scaled(&ch.bunny_hand[2], lx, ly, scale);
            }
            if with_wave {
                draw_tex_scaled(&assets.main_wave, x, y + 69.0 * scale, scale);
            }
        }
        1 => {
            draw_tex_scaled(&assets.main_boat, x - 7.0 * scale, y - 2.0 * scale, scale);
            draw_tex_scaled(&ch.du_face, x, y, scale);
            match eye {
                0 => {
                    let (ex, ey) = p(2.0, 15.0);
                    draw_tex_scaled(&ch.du_eye[0], ex, ey, scale);
                }
                1 => {
                    let (ex, ey) = p(2.0, 16.0);
                    draw_tex_scaled(&ch.du_eye[1], ex, ey, scale);
                }
                2 => {
                    let (ex, ey) = p(2.0, 18.0);
                    draw_tex_scaled(&ch.du_eye[2], ex, ey, scale);
                    if tear < 2 {
                        let (tx, ty) = p(2.0, 19.0);
                        draw_tex_scaled(&ch.du_tear[tear], tx, ty, scale);
                    }
                }
                _ => {
                    let (ex, ey) = p(1.0, 7.0);
                    draw_tex_scaled(&ch.du_eye[3], ex, ey, scale);
                }
            }
            let m_idx = if mouth >= 2 { 2 } else { mouth };
            let (dx, dy) = match m_idx {
                0 => (6.0, 27.0),
                1 => (7.0, 24.0),
                _ => (7.0, 22.0),
            };
            let (mx, my) = p(dx, dy);
            draw_tex_scaled(&ch.du_mouth[m_idx], mx, my, scale);
            let (wx, wy) = p(11.0, 38.0);
            draw_tex_scaled(&assets.wheel, wx, wy, scale);
            let (lx, ly) = p(7.0, 37.0);
            draw_tex_scaled(&ch.du_hand[0], lx, ly, scale);
            let (rx, ry) = p(23.0, 37.0);
            draw_tex_scaled(&ch.du_hand[1], rx, ry, scale);
            if with_wave {
                draw_tex_scaled(&assets.main_wave, x - 7.0 * scale, y + 60.0 * scale, scale);
            }
        }
        2 => {
            draw_tex_scaled(&assets.main_boat, x - 3.0 * scale, y - 1.0 * scale, scale);
            let (bx, by) = p(16.0, 36.0);
            draw_tex_scaled(&ch.fenny_body, bx, by, scale);
            draw_tex_scaled(&ch.fenny_face, x, y, scale);
            match mouth {
                1 => {
                    let (tx, ty) = p(21.0, 38.0);
                    draw_tex_scaled(&ch.fenny_tongue, tx, ty, scale);
                    let (fx, fy) = p(12.0, 21.0);
                    draw_tex_scaled(&ch.fenny_face2, fx, fy, scale);
                }
                0 => {
                    let (fx, fy) = p(12.0, 21.0);
                    draw_tex_scaled(&ch.fenny_face2, fx, fy, scale);
                }
                _ => {
                    let (fx, fy) = p(12.0, 21.0);
                    draw_tex_scaled(&ch.fenny_face3, fx, fy, scale);
                }
            }
            match eye {
                0 => {
                    let (ex, ey) = p(11.0, 5.0);
                    draw_tex_scaled(&ch.fenny_eye[0], ex, ey, scale);
                }
                1 => {
                    let (ex, ey) = p(13.0, 4.0);
                    draw_tex_scaled(&ch.fenny_eye[1], ex, ey, scale);
                }
                2 => {
                    let (ex, ey) = p(13.0, 6.0);
                    draw_tex_scaled(&ch.fenny_eye[2], ex, ey, scale);
                    if tear < 2 {
                        let (tx, ty) = p(12.0, 17.0);
                        draw_tex_scaled(&ch.fenny_tear, tx, ty, scale);
                        if tear == 1 {
                            let (tx2, ty2) = p(12.0, 20.0);
                            draw_tex_scaled(&ch.fenny_tear, tx2, ty2, scale);
                        }
                    }
                }
                _ => {
                    let (ex, ey) = p(11.0, 5.0);
                    draw_tex_scaled(&ch.fenny_eye[3], ex, ey, scale);
                }
            }
            let (wx, wy) = p(15.0, 39.0);
            draw_tex_scaled(&assets.wheel, wx, wy, scale);
            let (lx, ly) = p(9.0, 38.0);
            draw_tex_scaled(&ch.fenny_hand[0], lx, ly, scale);
            let (rx, ry) = p(31.0, 38.0);
            draw_tex_scaled(&ch.fenny_hand[1], rx, ry, scale);
            if with_wave {
                draw_tex_scaled(&assets.main_wave, x - 3.0 * scale, y + 60.0 * scale, scale);
            }
        }
        _ => {
            draw_tex_scaled(&assets.main_boat, x - 5.0 * scale, y - 2.0 * scale, scale);
            draw_tex_scaled(&ch.pan_face, x, y, scale);
            let (dx, dy) = match mouth.min(3) {
                0 => (18.0, 33.0),
                1 => (17.0, 30.0),
                2 => (16.0, 32.0),
                _ => (19.0, 33.0),
            };
            let (mx, my) = p(dx, dy);
            draw_tex_scaled(&ch.pan_mouth[mouth.min(3)], mx, my, scale);
            match eye {
                3 => {
                    let (ex, ey) = p(12.0, 18.0);
                    draw_tex_scaled(&ch.pan_eye[0], ex, ey, scale);
                }
                1 => {
                    let (ex, ey) = p(13.0, 19.0);
                    draw_tex_scaled(&ch.pan_eye[1], ex, ey, scale);
                }
                2 => {
                    let (ex, ey) = p(13.0, 19.0);
                    draw_tex_scaled(&ch.pan_eye[2], ex, ey, scale);
                    if tear < 2 {
                        let (tx, ty) = p(13.0, 23.0);
                        draw_tex_scaled(&ch.pan_tear[tear], tx, ty, scale);
                    }
                }
                _ => {}
            }
            let (wx, wy) = p(13.0, 37.0);
            draw_tex_scaled(&assets.wheel, wx, wy, scale);
            let (lx, ly) = p(9.0, 36.0);
            draw_tex_scaled(&ch.pan_hand[0], lx, ly, scale);
            let (rx, ry) = p(28.0, 36.0);
            draw_tex_scaled(&ch.pan_hand[1], rx, ry, scale);
            if with_wave {
                draw_tex_scaled(&assets.main_wave, x - 5.0 * scale, y + 60.0 * scale, scale);
            }
        }
    }
}

fn draw_tile_hd(assets: &Assets, val: u8, x: f32, y: f32, size: f32) {
    let tex = if val == 100 {
        &assets.explosion[0]
    } else if val == 101 {
        &assets.explosion[1]
    } else if (6..=10).contains(&val) {
        &assets.special[(val - 6) as usize]
    } else if (1..=5).contains(&val) {
        &assets.blocks[(val - 1) as usize]
    } else {
        return;
    };
    draw_texture_ex(
        tex,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(size, size)),
            ..Default::default()
        },
    );
}

fn draw_hud_and_board_hd(game: &Game, assets: &Assets) {
    let sw = screen_width();
    let sh = screen_height();

    // Top HUD bar
    let hud_h = 56.0;
    draw_rectangle(0.0, 0.0, sw, hud_h, c24(0xFFB800));
    draw_line(0.0, hud_h, sw, hud_h, 3.0, c24(0xB84A00));
    draw_text("SCORE", 24.0, 37.0, 28.0, c24(0x2B1900));
    draw_rectangle(125.0, 10.0, 140.0, 36.0, c24(0x0B1638));
    draw_rectangle_lines(125.0, 10.0, 140.0, 36.0, 2.0, c24(0x5B3300));
    draw_text(&format!("{:05}", game.score), 144.0, 37.0, 30.0, WHITE);

    draw_text("HIGH", 295.0, 37.0, 28.0, c24(0x2B1900));
    draw_rectangle(375.0, 10.0, 140.0, 36.0, c24(0x0B1638));
    draw_rectangle_lines(375.0, 10.0, 140.0, 36.0, 2.0, c24(0x5B3300));
    draw_text(&format!("{:05}", game.high_score), 394.0, 37.0, 30.0, c24(0xFFE066));

    let mode_hud = match game.mode {
        GameMode::Blitz => {
            let secs = (game.blitz_ticks_remaining.max(0) + 19) / 20;
            format!("LVL {}  |  {:02}:{:02}", game.level, secs / 60, secs % 60)
        }
        GameMode::RisingTide => {
            let left = game.tide_interval.saturating_sub(game.tide_counter);
            format!("LVL {}  |  Tide: {}", game.level, left)
        }
        _ => format!("LVL {}  |  {}x{}", game.level, game.cols, game.visible_rows),
    };
    draw_text(&mode_hud, 535.0, 37.0, 22.0, c24(0x2B1900));

    // Quit to Menu & Pause buttons in top-right HUD
    let menu_x = sw - 236.0;
    draw_rectangle(menu_x, 10.0, 104.0, 36.0, c24(0x4A3A28));
    draw_rectangle_lines(menu_x, 10.0, 104.0, 36.0, 2.0, c24(0xFFE066));
    let mtw = measure_text("MENU", None, 22, 1.0).width;
    draw_text("MENU", menu_x + (104.0 - mtw) * 0.5, 35.0, 22.0, WHITE);

    let btn_x = sw - 124.0;
    let btn_col = if game.state == ScreenState::Paused {
        c24(0x2A6B38)
    } else {
        c24(0x5B3300)
    };
    draw_rectangle(btn_x, 10.0, 104.0, 36.0, btn_col);
    draw_rectangle_lines(btn_x, 10.0, 104.0, 36.0, 2.0, c24(0xFFE066));
    let btn_label = if game.state == ScreenState::Paused {
        "RESUME"
    } else {
        "PAUSE"
    };
    let btw = measure_text(btn_label, None, 22, 1.0).width;
    draw_text(btn_label, btn_x + (104.0 - btw) * 0.5, 35.0, 22.0, WHITE);

    // Dynamic playfield sizing to fit any custom grid
    let side_panel_w = 300.0;
    let margin = 24.0;
    let avail_w = (sw - side_panel_w - margin * 2.0).max(200.0);
    let avail_h = (sh - hud_h - margin * 2.0).max(240.0);
    let tile_size = (avail_w / game.cols as f32)
        .min(avail_h / game.visible_rows as f32)
        .floor()
        .clamp(24.0, 64.0);

    let board_w = tile_size * game.cols as f32;
    let board_h = tile_size * game.visible_rows as f32;
    let bx = margin + (avail_w - board_w) * 0.5;
    let by = hud_h + margin + (avail_h - board_h) * 0.5;

    // Playfield backdrop & subtle grid
    draw_rectangle(bx - 6.0, by - 6.0, board_w + 12.0, board_h + 12.0, c24(0x2E5C38));
    draw_rectangle(bx, by, board_w, board_h, c24(0xE8F4DC));
    for c in 0..game.cols {
        for r in 0..game.visible_rows {
            if (c + r) % 2 == 1 {
                draw_rectangle(
                    bx + c as f32 * tile_size,
                    by + r as f32 * tile_size,
                    tile_size,
                    tile_size,
                    c24(0xDFECD2),
                );
            }
        }
    }

    // Ghost drop preview
    if game.state == ScreenState::Falling && game.piece_pos[1] > 0 {
        let gr = game.ghost_row();
        if gr > game.piece_pos[1] {
            for r_off in 0..3 {
                for c_off in 0..3 {
                    let val = game.piece[0][r_off][c_off];
                    if val > 0 {
                        let bc = game.piece_pos[0] + c_off as i32;
                        let br = gr + r_off as i32;
                        if bc >= 0 && (bc as usize) < game.cols && br >= 3 && (br as usize) < game.total_rows {
                            let gx = bx + bc as f32 * tile_size;
                            let gy = by + (br - 3) as f32 * tile_size;
                            draw_rectangle(gx + 2.0, gy + 2.0, tile_size - 4.0, tile_size - 4.0, Color::from_rgba(255, 255, 255, 55));
                            draw_rectangle_lines(gx + 2.0, gy + 2.0, tile_size - 4.0, tile_size - 4.0, 2.0, Color::from_rgba(40, 80, 50, 150));
                        }
                    }
                }
            }
        }
    }

    // Board tiles (rows 3..total_rows)
    for r in 3..game.total_rows {
        let vr = r - 3;
        for c in 0..game.cols {
            let val = game.board[c][r];
            if val > 0 {
                draw_tile_hd(
                    assets,
                    val,
                    bx + c as f32 * tile_size,
                    by + vr as f32 * tile_size,
                    tile_size,
                );
            }
        }
    }
    draw_rectangle_lines(bx - 3.0, by - 3.0, board_w + 6.0, board_h + 6.0, 4.0, c24(0x1E3D24));

    // Combo banner
    if game.combo > 1 {
        let cx = bx + board_w * 0.5;
        let cy = by + board_h * 0.45;
        draw_tex_scaled(&assets.combo, cx - 60.0, cy - 36.0, 3.0);
        draw_tex_scaled(&assets.combo_star, cx - 76.0, cy - 10.0, 3.0);
        draw_tex_scaled(&assets.combo_star, cx + 62.0, cy - 18.0, 3.0);
        let cval = (game.combo - 1) as usize;
        if cval < 10 {
            draw_tex_scaled(&assets.combo_digits[cval], cx - 12.0, cy + 12.0, 3.0);
        } else {
            draw_tex_scaled(&assets.combo_digits[(cval / 10).min(9)], cx - 26.0, cy + 12.0, 3.0);
            draw_tex_scaled(&assets.combo_digits[cval % 10], cx + 4.0, cy + 12.0, 3.0);
        }
    }

    // Right panel: Next Piece + Hold Piece + Fever Bar + Character in Boat
    let rx = sw - side_panel_w;
    draw_rectangle(rx, hud_h, side_panel_w, sh - hud_h, c24(0x78C8F0));

    let box_sz = 114.0;
    let mini_tile = 32.0;
    let next_box_x = rx + 22.0;
    let next_box_y = hud_h + 40.0;
    draw_text("NEXT", next_box_x, hud_h + 30.0, 22.0, c24(0x163858));
    draw_rectangle(next_box_x, next_box_y, box_sz, box_sz, c24(0xFFFFAB));
    draw_rectangle_lines(next_box_x, next_box_y, box_sz, box_sz, 3.0, c24(0xC29828));

    if game.blind_next {
        draw_text("?", next_box_x + 46.0, next_box_y + 72.0, 52.0, c24(0xC29828));
    } else {
        for r in 0..3 {
            for c in 0..3 {
                let val = game.piece[1][r][c];
                if val > 0 {
                    draw_tile_hd(
                        assets,
                        val,
                        next_box_x + 9.0 + c as f32 * mini_tile,
                        next_box_y + 9.0 + r as f32 * mini_tile,
                        mini_tile,
                    );
                }
            }
        }
    }

    let hold_box_x = rx + 156.0;
    draw_text("HOLD (C)", hold_box_x, hud_h + 30.0, 22.0, c24(0x163858));
    draw_rectangle(hold_box_x, next_box_y, box_sz, box_sz, c24(0xFFFFAB));
    draw_rectangle_lines(hold_box_x, next_box_y, box_sz, box_sz, 3.0, c24(0xC29828));
    if let Some((hp, _, _)) = game.hold_piece {
        for r in 0..3 {
            for c in 0..3 {
                let val = hp[r][c];
                if val > 0 {
                    draw_tile_hd(
                        assets,
                        val,
                        hold_box_x + 9.0 + c as f32 * mini_tile,
                        next_box_y + 9.0 + r as f32 * mini_tile,
                        mini_tile,
                    );
                }
            }
        }
    }

    // Combo Fever Meter
    let fy = next_box_y + box_sz + 18.0;
    let mult = game.fever_multiplier();
    draw_text(
        &format!("FEVER  {}x", mult),
        next_box_x,
        fy + 16.0,
        20.0,
        if mult > 1 { c24(0xB82000) } else { c24(0x163858) },
    );
    draw_rectangle(next_box_x, fy + 24.0, 248.0, 16.0, c24(0x163858));
    let bar_col = match mult {
        4 => c24(0xFF3838),
        3 => c24(0xFF9020),
        2 => c24(0xFFE030),
        _ => c24(0x55B870),
    };
    draw_rectangle(next_box_x + 2.0, fy + 26.0, 244.0 * game.fever_meter, 12.0, bar_col);

    // Animated character in boat
    let kind = match game.level {
        1 | 5 | 9 => 0,
        2 | 6 | 10 => 1,
        3 | 7 => 2,
        _ => 3,
    };
    let cscale = 4.2;
    let mut cx = rx + 40.0;
    let mut cy = sh - 320.0;
    let bob = (game.tick_counter % 2) as f32;
    if game.state == ScreenState::LevelExit {
        cy += (game.sub_tick * 10) as f32 * cscale;
    } else if game.state == ScreenState::LevelEnter {
        let off = (10 - game.sub_tick).max(0) as f32 * 24.0;
        cx += off;
    } else {
        cy -= bob * 3.0;
    }
    draw_character_hd(
        assets,
        kind,
        cx,
        cy,
        cscale,
        game.char_eye,
        game.char_tear,
        game.char_mouth,
        game.char_intro,
        true,
        true,
        bob,
    );
}

fn draw_screen(game: &Game, assets: &Assets) {
    let sw = screen_width();
    let sh = screen_height();

    match game.state {
        ScreenState::Intro => {
            draw_rectangle(0.0, 0.0, sw, sh, c24(0x240055));
            let s = (sw / 128.0).min(sh / 128.0);
            let ox = (sw - 128.0 * s) * 0.5;
            let oy = (sh - 128.0 * s) * 0.5;
            draw_tex_scaled(&assets.cloud, ox + 7.0 * s, oy + 75.0 * s, s);
            draw_tex_scaled(&assets.intro_bigstar, ox + 36.0 * s, oy + 7.0 * s, s);
            draw_tex_scaled(&assets.intro_bigstar, ox + 86.0 * s, oy + 34.0 * s, s);
            draw_tex_scaled(&assets.intro_midstar, ox + 29.0 * s, oy + 40.0 * s, s);
            draw_tex_scaled(&assets.intro_smallstar, ox + 12.0 * s, oy + 37.0 * s, s);
            let hand = if game.sub_tick == 3 { 0 } else { 1 };
            draw_character_hd(
                assets,
                0,
                ox + 38.0 * s,
                oy + (44.0 + (game.sub_tick % 2) as f32) * s,
                s,
                1,
                2,
                2,
                hand,
                false,
                false,
                (game.sub_tick % 2) as f32,
            );
            draw_tex_scaled(&assets.intro_title, ox + 16.0 * s, oy + 7.0 * s, s);
            draw_tex_scaled(&assets.intro_flower1, ox + 37.0 * s, oy + 16.0 * s, s);
            draw_text("Press any key to start", sw * 0.5 - 135.0, sh - 28.0, 26.0, LIGHTGRAY);
        }
        ScreenState::Ending => {
            draw_rectangle(0.0, 0.0, sw, sh, c24(0x00B6FF));
            let s = (sw / 128.0).min(sh / 128.0);
            let ox = (sw - 128.0 * s) * 0.5;
            let oy = (sh - 128.0 * s) * 0.5;
            let dy = (game.sub_tick * 2) as f32 * s;
            draw_tex_scaled(&assets.cloud, ox, oy + 88.0 * s, s);
            draw_tex_scaled(&assets.intro_flower3, ox + 8.0 * s, oy + 79.0 * s + dy, s);
            draw_tex_scaled(&assets.intro_flower3, ox + 67.0 * s, oy + 79.0 * s + dy, s);
            let (eye, mouth, bob) = if game.sub_tick % 10 <= 5 {
                (0, 1, 0.0)
            } else {
                (1, 2, 1.0)
            };
            draw_character_hd(assets, 0, ox + 12.0 * s, oy + 31.0 * s + dy, s, eye, 2, 2, 0, false, false, bob);
            draw_character_hd(assets, 2, ox + 64.0 * s, oy + 41.0 * s + dy, s, eye, 2, mouth, 0, false, false, 0.0);
            draw_tex_scaled(&assets.baguni, ox + 20.0 * s, oy + 90.0 * s + dy, s);
        }
        ScreenState::GameOver | ScreenState::AllCleared => {
            draw_hud_and_board_hd(game, assets);
            if game.sub_tick % 2 == 1 || game.sub_tick == 0 || game.sub_tick == 8 {
                let msg = if game.state == ScreenState::GameOver {
                    "GAME OVER"
                } else {
                    "ALL CLEARED!"
                };
                draw_rectangle(sw * 0.5 - 160.0, sh * 0.5 - 48.0, 320.0, 96.0, Color::from_rgba(0, 0, 0, 210));
                draw_rectangle_lines(sw * 0.5 - 160.0, sh * 0.5 - 48.0, 320.0, 96.0, 3.0, c24(0xFFE066));
                let tw = measure_text(msg, None, 44, 1.0).width;
                draw_text(msg, sw * 0.5 - tw * 0.5, sh * 0.5 + 14.0, 44.0, WHITE);
            }
        }
        ScreenState::Menu => {
            draw_rectangle(0.0, 0.0, sw, sh, c24(0x102848));
            let title = "Nature Park HD";
            let tw = measure_text(title, None, 50, 1.0).width;
            draw_text(title, sw * 0.5 - tw * 0.5, 92.0, 50.0, c24(0xFFE066));

            for (i, label) in game.menu_items().iter().enumerate() {
                let y = 175.0 + (i as f32) * 54.0;
                if i == game.menu_index {
                    draw_rectangle(sw * 0.5 - 200.0, y - 34.0, 400.0, 46.0, c24(0x2A6B38));
                    draw_rectangle_lines(sw * 0.5 - 200.0, y - 34.0, 400.0, 46.0, 2.0, c24(0x88E898));
                }
                let lw = measure_text(label, None, 30, 1.0).width;
                draw_text(label, sw * 0.5 - lw * 0.5, y, 30.0, WHITE);
            }
            draw_text("Click or Arrows + Enter  |  M: Mute  |  Esc: Exit", 24.0, sh - 24.0, 22.0, LIGHTGRAY);
        }
        ScreenState::GridSettings => {
            draw_rectangle(0.0, 0.0, sw, sh, c24(0x102848));
            let title = "Customize Grid Size";
            let tw = measure_text(title, None, 44, 1.0).width;
            draw_text(title, sw * 0.5 - tw * 0.5, 110.0, 44.0, c24(0xFFE066));

            let cx = sw * 0.5;
            for (i, (label, val, min_v, max_v)) in [
                ("Columns", game.cols, MIN_COLS, MAX_COLS),
                ("Rows", game.visible_rows, MIN_VISIBLE_ROWS, MAX_VISIBLE_ROWS),
            ]
            .iter()
            .enumerate()
            {
                let y = 240.0 + (i as f32) * 76.0;
                if i == game.grid_setting_row {
                    draw_rectangle(cx - 250.0, y - 38.0, 500.0, 54.0, c24(0x2A6B38));
                    draw_rectangle_lines(cx - 250.0, y - 38.0, 500.0, 54.0, 2.0, c24(0x88E898));
                }
                draw_text(label, cx - 220.0, y, 32.0, WHITE);

                // Clickable '-' button
                draw_rectangle(cx - 35.0, y - 32.0, 45.0, 42.0, c24(0x4A3A28));
                draw_rectangle_lines(cx - 35.0, y - 32.0, 45.0, 42.0, 2.0, c24(0xFFE066));
                draw_text("-", cx - 19.0, y - 1.0, 34.0, WHITE);

                let vstr = format!("{}", val);
                let vw = measure_text(&vstr, None, 32, 1.0).width;
                draw_text(&vstr, cx + 42.0 - vw * 0.5, y, 32.0, c24(0xFFE066));

                // Clickable '+' button
                draw_rectangle(cx + 75.0, y - 32.0, 45.0, 42.0, c24(0x4A3A28));
                draw_rectangle_lines(cx + 75.0, y - 32.0, 45.0, 42.0, 2.0, c24(0xFFE066));
                draw_text("+", cx + 88.0, y - 1.0, 34.0, WHITE);

                draw_text(&format!("({}..{})", min_v, max_v), cx + 140.0, y, 24.0, LIGHTGRAY);
            }

            let done_y = 240.0 + 2.0 * 76.0;
            if game.grid_setting_row == 2 {
                draw_rectangle(cx - 250.0, done_y - 38.0, 500.0, 54.0, c24(0x2A6B38));
                draw_rectangle_lines(cx - 250.0, done_y - 38.0, 500.0, 54.0, 2.0, c24(0x88E898));
            }
            let done_txt = "Done (Back to Menu)";
            let dw = measure_text(done_txt, None, 32, 1.0).width;
            draw_text(done_txt, cx - dw * 0.5, done_y, 32.0, WHITE);

            draw_text("Click -/+ or use Left/Right  |  Enter/Esc to return", 24.0, sh - 24.0, 22.0, LIGHTGRAY);
        }
        ScreenState::HighScores => {
            draw_rectangle(0.0, 0.0, sw, sh, c24(0x102848));
            let hs_title = format!(
                "High Scores — {} ({}x{})",
                game.mode.label(),
                game.cols,
                game.visible_rows
            );
            let htw = measure_text(&hs_title, None, 36, 1.0).width;
            draw_text(&hs_title, sw * 0.5 - htw * 0.5, 100.0, 36.0, c24(0xFFE066));
            for (i, entry) in game.high_scores.iter().enumerate() {
                let y = 190.0 + (i as f32) * 58.0;
                let line = format!(
                    "{}.  {}/{}/{}   —   {} pts (Lvl {})",
                    i + 1,
                    entry.day,
                    entry.month,
                    entry.year,
                    entry.score,
                    entry.level
                );
                draw_text(&line, sw * 0.5 - 240.0, y, 30.0, WHITE);
            }
            draw_text("Press Enter or Esc to go back", 24.0, sh - 24.0, 22.0, LIGHTGRAY);
        }
        ScreenState::Instructions => {
            draw_rectangle(0.0, 0.0, sw, sh, c24(0x102848));
            draw_text("Instructions", sw * 0.5 - 125.0, 100.0, 44.0, c24(0xFFE066));
            let lines = [
                "Match 3 or more blocks of the same color",
                "horizontally, vertically, or diagonally.",
                "",
                "Left / Right (or A / D): Move piece",
                "Up / W / Z: Rotate colors",
                "Down / Space: Fast drop",
                "C / Shift: Hold piece",
                "P: Pause  |  Esc: Menu  |  M: Mute",
                "Modes: Classic, Endless, Rising Tide, 2-Min Blitz",
            ];
            for (i, line) in lines.iter().enumerate() {
                draw_text(line, 120.0, 180.0 + (i as f32) * 44.0, 28.0, LIGHTGRAY);
            }
            draw_text("Press Enter or Esc to go back", 24.0, sh - 24.0, 22.0, LIGHTGRAY);
        }
        ScreenState::Paused => {
            draw_hud_and_board_hd(game, assets);
            draw_rectangle(0.0, 56.0, sw, sh - 56.0, Color::from_rgba(0, 0, 0, 165));
            draw_rectangle(
                sw * 0.5 - 190.0,
                sh * 0.5 - 80.0,
                380.0,
                160.0,
                c24(0x102848),
            );
            draw_rectangle_lines(
                sw * 0.5 - 190.0,
                sh * 0.5 - 80.0,
                380.0,
                160.0,
                3.0,
                c24(0xFFE066),
            );
            let pw = measure_text("PAUSED", None, 40, 1.0).width;
            draw_text("PAUSED", sw * 0.5 - pw * 0.5, sh * 0.5 - 24.0, 40.0, c24(0xFFE066));

            draw_rectangle(sw * 0.5 - 165.0, sh * 0.5 + 10.0, 150.0, 48.0, c24(0x2A6B38));
            draw_rectangle_lines(sw * 0.5 - 165.0, sh * 0.5 + 10.0, 150.0, 48.0, 2.0, c24(0x88E898));
            draw_text("Resume (P)", sw * 0.5 - 144.0, sh * 0.5 + 42.0, 24.0, WHITE);

            draw_rectangle(sw * 0.5 + 15.0, sh * 0.5 + 10.0, 160.0, 48.0, c24(0x4A3A28));
            draw_rectangle_lines(sw * 0.5 + 15.0, sh * 0.5 + 10.0, 160.0, 48.0, 2.0, c24(0xFFE066));
            draw_text("Quit to Menu", sw * 0.5 + 28.0, sh * 0.5 + 42.0, 24.0, WHITE);
        }
        _ => {
            draw_hud_and_board_hd(game, assets);
        }
    }
}
