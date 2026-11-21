use macroquad::prelude::*;
use std::fs;
use std::path::Path;

const BLOCK_RES: usize = 64;

fn blend(dst: [u8; 4], src: [u8; 4]) -> [u8; 4] {
    let a = src[3] as f32 / 255.0;
    let inv = 1.0 - a;
    [
        (src[0] as f32 * a + dst[0] as f32 * inv).round() as u8,
        (src[1] as f32 * a + dst[1] as f32 * inv).round() as u8,
        (src[2] as f32 * a + dst[2] as f32 * inv).round() as u8,
        (src[3] as f32 + dst[3] as f32 * inv).min(255.0).round() as u8,
    ]
}

fn rounded_box_sdf(px: f32, py: f32, half: f32, radius: f32) -> f32 {
    let qx = px.abs() - (half - radius);
    let qy = py.abs() - (half - radius);
    let out = (qx.max(0.0).powi(2) + qy.max(0.0).powi(2)).sqrt();
    let inside = qx.max(qy).min(0.0);
    out + inside - radius
}

fn make_gem_block(base: [u8; 3], light: [u8; 3], dark: [u8; 3], emblem: u8) -> Image {
    let n = BLOCK_RES;
    let mut bytes = vec![0u8; n * n * 4];
    let center = (n as f32 - 1.0) * 0.5;

    for y in 0..n {
        for x in 0..n {
            let fx = x as f32 - center;
            let fy = y as f32 - center;
            let dist = rounded_box_sdf(fx, fy, center - 2.0, 12.0);
            let idx = (y * n + x) * 4;
            if dist > 1.0 {
                continue;
            }
            let edge_alpha = (1.0 - dist).clamp(0.0, 1.0);

            // Vertical + diagonal gradient
            let t = ((fx + fy) / (n as f32 * 1.4) + 0.5).clamp(0.0, 1.0);
            let mut r = base[0] as f32 * (1.0 - t * 0.35) + dark[0] as f32 * (t * 0.35);
            let mut g = base[1] as f32 * (1.0 - t * 0.35) + dark[1] as f32 * (t * 0.35);
            let mut b = base[2] as f32 * (1.0 - t * 0.35) + dark[2] as f32 * (t * 0.35);

            // Outer dark rim
            if dist > -3.0 {
                r *= 0.65;
                g *= 0.65;
                b *= 0.65;
            } else if dist > -6.5 {
                // Inner bright bevel
                r = r * 0.5 + light[0] as f32 * 0.5;
                g = g * 0.5 + light[1] as f32 * 0.5;
                b = b * 0.5 + light[2] as f32 * 0.5;
            }

            // Glossy top-left highlight
            let hx = (fx + 10.0) / 18.0;
            let hy = (fy + 14.0) / 10.0;
            let hdist = (hx * hx + hy * hy).sqrt();
            if hdist < 1.0 && dist < -4.0 {
                let shine = (1.0 - hdist) * 0.55;
                r = r * (1.0 - shine) + 255.0 * shine;
                g = g * (1.0 - shine) + 255.0 * shine;
                b = b * (1.0 - shine) + 255.0 * shine;
            }

            let mut col = [
                r.clamp(0.0, 255.0) as u8,
                g.clamp(0.0, 255.0) as u8,
                b.clamp(0.0, 255.0) as u8,
                (edge_alpha * 255.0) as u8,
            ];

            // Subtle inner emblem to aid colorblind distinction
            let r_center = (fx * fx + fy * fy).sqrt();
            match emblem {
                1 => {
                    // Blue: water drop ring
                    if (r_center - 10.0).abs() < 2.2 {
                        col = blend(col, [220, 245, 255, 130]);
                    }
                }
                2 => {
                    // Green: leaf diamond
                    if (fx.abs() + fy.abs() - 12.0).abs() < 2.2 {
                        col = blend(col, [230, 255, 220, 130]);
                    }
                }
                3 => {
                    // Purple: inner gem square
                    if fx.abs().max(fy.abs()) < 9.0 && fx.abs().max(fy.abs()) > 5.5 {
                        col = blend(col, [245, 220, 255, 130]);
                    }
                }
                4 => {
                    // Red: bright core dot
                    if r_center < 7.5 {
                        col = blend(col, [255, 220, 220, 130]);
                    }
                }
                5 => {
                    // Yellow: cross sparkle
                    if (fx.abs() < 2.2 && fy.abs() < 11.0) || (fy.abs() < 2.2 && fx.abs() < 11.0) {
                        col = blend(col, [255, 255, 230, 150]);
                    }
                }
                _ => {}
            }

            bytes[idx..idx + 4].copy_from_slice(&col);
        }
    }

    Image {
        bytes,
        width: n as u16,
        height: n as u16,
    }
}

fn make_lightning_block() -> Image {
    let mut img = make_gem_block([45, 110, 245], [170, 225, 255], [20, 45, 130], 0);
    let n = BLOCK_RES;
    let c = (n as f32 - 1.0) * 0.5;
    for y in 0..n {
        for x in 0..n {
            let fx = x as f32 - c;
            let fy = y as f32 - c;
            // Two triangles forming a crisp lightning bolt
            let top_bolt = fy >= -18.0 && fy <= 2.0 && fx >= (-4.0 - (fy + 18.0) * 0.45) && fx <= (8.0 - (fy + 18.0) * 0.45);
            let bot_bolt = fy >= -2.0 && fy <= 18.0 && fx >= (-2.0 - (fy + 2.0) * 0.45) && fx <= (10.0 - (fy + 2.0) * 0.45);
            if top_bolt || bot_bolt {
                let idx = (y * n + x) * 4;
                let cur = [img.bytes[idx], img.bytes[idx + 1], img.bytes[idx + 2], img.bytes[idx + 3]];
                let blended = blend(cur, [255, 240, 80, 255]);
                img.bytes[idx..idx + 4].copy_from_slice(&blended);
            }
        }
    }
    img
}

fn make_bomb_block() -> Image {
    let mut img = make_gem_block([55, 58, 68], [130, 135, 150], [22, 24, 30], 0);
    let n = BLOCK_RES;
    let c = (n as f32 - 1.0) * 0.5;
    for y in 0..n {
        for x in 0..n {
            let fx = x as f32 - c;
            let fy = y as f32 - c;
            let idx = (y * n + x) * 4;
            let cur = [img.bytes[idx], img.bytes[idx + 1], img.bytes[idx + 2], img.bytes[idx + 3]];
            // Bomb sphere + spark
            let d = (fx * fx + (fy - 3.0) * (fy - 3.0)).sqrt();
            if d < 14.0 {
                let shade = if d < 6.0 && fx < 0.0 && fy < 2.0 {
                    [110, 115, 130, 255]
                } else {
                    [20, 22, 26, 255]
                };
                let b = blend(cur, shade);
                img.bytes[idx..idx + 4].copy_from_slice(&b);
            } else if ((fx - 6.0).powi(2) + (fy + 14.0).powi(2)).sqrt() < 5.5 {
                let b = blend(cur, [255, 160, 40, 255]);
                img.bytes[idx..idx + 4].copy_from_slice(&b);
            }
        }
    }
    img
}

fn make_star_block() -> Image {
    let mut img = make_gem_block([245, 175, 25], [255, 240, 150], [165, 90, 10], 0);
    let n = BLOCK_RES;
    let c = (n as f32 - 1.0) * 0.5;
    for y in 0..n {
        for x in 0..n {
            let fx = x as f32 - c;
            let fy = y as f32 - c;
            let r = (fx * fx + fy * fy).sqrt();
            let ang = fy.atan2(fx) - std::f32::consts::FRAC_PI_2;
            let star_r = 10.5 + 6.5 * (5.0 * ang).cos();
            if r <= star_r {
                let idx = (y * n + x) * 4;
                let cur = [img.bytes[idx], img.bytes[idx + 1], img.bytes[idx + 2], img.bytes[idx + 3]];
                let b = blend(cur, [255, 252, 200, 255]);
                img.bytes[idx..idx + 4].copy_from_slice(&b);
            }
        }
    }
    img
}

fn make_stone_block() -> Image {
    let mut img = make_gem_block([128, 132, 138], [185, 190, 198], [72, 76, 82], 0);
    let n = BLOCK_RES;
    let c = (n as f32 - 1.0) * 0.5;
    for y in 0..n {
        for x in 0..n {
            let fx = x as f32 - c;
            let fy = y as f32 - c;
            // Cracks
            let crack = (fx - fy * 0.6).abs() < 1.3 && fy > -12.0 && fy < 14.0
                || (fx + fy * 0.8 - 4.0).abs() < 1.1 && fx > -10.0 && fx < 12.0;
            if crack {
                let idx = (y * n + x) * 4;
                let cur = [img.bytes[idx], img.bytes[idx + 1], img.bytes[idx + 2], img.bytes[idx + 3]];
                let b = blend(cur, [45, 48, 52, 210]);
                img.bytes[idx..idx + 4].copy_from_slice(&b);
            }
        }
    }
    img
}

fn make_explosion(radius: f32, core: [u8; 4], outer: [u8; 4]) -> Image {
    let n = BLOCK_RES;
    let mut bytes = vec![0u8; n * n * 4];
    let c = (n as f32 - 1.0) * 0.5;
    for y in 0..n {
        for x in 0..n {
            let fx = x as f32 - c;
            let fy = y as f32 - c;
            let r = (fx * fx + fy * fy).sqrt();
            let ang = fy.atan2(fx);
            let spike = radius * (0.72 + 0.28 * (8.0 * ang).cos());
            if r <= spike {
                let idx = (y * n + x) * 4;
                let col = if r <= spike * 0.48 { core } else { outer };
                bytes[idx..idx + 4].copy_from_slice(&col);
            }
        }
    }
    Image {
        bytes,
        width: n as u16,
        height: n as u16,
    }
}

/// 2x EPX / Scale2x pixel-art upscaler (smooths diagonal edges while keeping crisp lines).
fn scale2x(src: &Image) -> Image {
    let w = src.width as usize;
    let h = src.height as usize;
    let nw = w * 2;
    let nh = h * 2;
    let mut out = vec![0u8; nw * nh * 4];

    let px = |x: isize, y: isize| -> [u8; 4] {
        let cx = x.clamp(0, w as isize - 1) as usize;
        let cy = y.clamp(0, h as isize - 1) as usize;
        let i = (cy * w + cx) * 4;
        [src.bytes[i], src.bytes[i + 1], src.bytes[i + 2], src.bytes[i + 3]]
    };

    for y in 0..h as isize {
        for x in 0..w as isize {
            let p = px(x, y);
            let a = px(x, y - 1);
            let b = px(x + 1, y);
            let c = px(x - 1, y);
            let d = px(x, y + 1);

            let (e0, e1, e2, e3) = if a != d && c != b {
                (
                    if c == a { a } else { p },
                    if a == b { b } else { p },
                    if c == d { c } else { p },
                    if d == b { d } else { p },
                )
            } else {
                (p, p, p, p)
            };

            let ux = (x as usize) * 2;
            let uy = (y as usize) * 2;
            let i0 = (uy * nw + ux) * 4;
            let i1 = (uy * nw + ux + 1) * 4;
            let i2 = ((uy + 1) * nw + ux) * 4;
            let i3 = ((uy + 1) * nw + ux + 1) * 4;
            out[i0..i0 + 4].copy_from_slice(&e0);
            out[i1..i1 + 4].copy_from_slice(&e1);
            out[i2..i2 + 4].copy_from_slice(&e2);
            out[i3..i3 + 4].copy_from_slice(&e3);
        }
    }

    Image {
        bytes: out,
        width: nw as u16,
        height: nh as u16,
    }
}

pub async fn load_hd_sprite(path: &str) -> Texture2D {
    let raw = load_image(path)
        .await
        .unwrap_or_else(|e| panic!("Failed to load {path}: {e}"));
    let up = scale2x(&scale2x(&raw));
    let tex = Texture2D::from_image(&up);
    tex.set_filter(FilterMode::Linear);
    tex
}

pub fn generate_hd_block_textures() -> ([Texture2D; 5], [Texture2D; 4], [Texture2D; 2]) {
    let hd_dir = Path::new("assets_hd");
    if !hd_dir.exists() {
        let _ = fs::create_dir_all(hd_dir);
    }

    let blocks_img = [
        ("block_blue.png", make_gem_block([42, 130, 245], [160, 215, 255], [18, 65, 165], 1)),
        ("block_green.png", make_gem_block([48, 195, 85], [170, 248, 185], [20, 110, 42], 2)),
        ("block_purple.png", make_gem_block([165, 65, 230], [225, 175, 255], [95, 25, 150], 3)),
        ("block_red.png", make_gem_block([235, 58, 62], [255, 175, 175], [145, 20, 25], 4)),
        ("block_yellow.png", make_gem_block([245, 200, 32], [255, 248, 165], [175, 120, 10], 5)),
    ];
    let special_img = [
        ("block_lightning.png", make_lightning_block()),
        ("block_n.png", make_bomb_block()),
        ("block_star.png", make_star_block()),
        ("block_stone.png", make_stone_block()),
    ];
    let explosion_img = [
        ("block_explosion1.png", make_explosion(22.0, [255, 255, 220, 255], [255, 155, 30, 235])),
        ("block_explosion2.png", make_explosion(30.0, [255, 255, 255, 230], [255, 95, 20, 190])),
    ];

    let to_tex = |(name, img): (&str, Image)| -> Texture2D {
        img.export_png(&hd_dir.join(name).to_string_lossy());
        let t = Texture2D::from_image(&img);
        t.set_filter(FilterMode::Linear);
        t
    };

    let blocks = blocks_img.map(to_tex);
    let special = special_img.map(to_tex);
    let explosion = explosion_img.map(to_tex);
    (blocks, special, explosion)
}
