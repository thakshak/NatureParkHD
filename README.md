# Nature Park HD

A Rust HD resurrection of the classic Nokia Series 40 falling-block match-3 puzzle game (**Nature Park**), rebuilt with procedural `64×64` colorblind-friendly sprites, `4×` Scale2x-smoothed character animations, authentic Nokia OTT buzzer audio, customizable grid sizes, and modern puzzle modes.

---

## Features

- **4 Game Modes:**
  - **Classic (10 Lvls):** Clear 10 escalating levels (`30 × level²` pts) to reach the celebration ending.
  - **Endless:** Unlimited levels with a smooth difficulty curve (`50 ms` faster per level, gradual stone ramp).
  - **Rising Tide:** New rows of mixed blocks and stones push up from the bottom every few drops.
  - **2-Min Blitz:** 120-second time attack where combo chains add `+2s`.
- **Customizable Grid Sizes:** Adjust **Columns (`5–12`)** and **Rows (`8–16`)** directly from the menu.
- **Separate Leaderboards:** Top-5 high scores are saved locally per `(grid size, mode)` pair (`highscores_v2.json`).
- **Skill-Ceiling Mechanics:**
  - **Ghost Drop Preview** showing where the active piece will land.
  - **Hold Slot (`C` / `Shift`)** to bank one piece per drop.
  - **Combo Fever Meter (`1×–4×`)** rewarding rapid chains.
  - **Blind Next** toggle to hide the `NEXT` preview.
- **Colorblind-Friendly HD Blocks:** Each of the 5 block colors features a distinct geometric inner emblem alongside **Lightning**, **Bomb**, **Star**, and **Stone** tiles.
- **Authentic Nokia OTT Audio:** Pure-Rust bit-stream parser and square-wave synthesizer rendering all 8 original melodies and sound effects.

---

## Controls

| Key / Input | Action |
| :--- | :--- |
| `Left` / `Right` or `A` / `D` or `4` / `6` | Move piece left / right |
| `Up` or `W` or `Z` or `2` | Cycle piece colors |
| `Down` or `S` or `Space` or `5` / `8` | Fast drop |
| `C` or `Shift` | Hold / swap piece |
| `P` or HUD **PAUSE** | Pause / Resume |
| `Esc` or HUD **MENU** | Quit to Menu |
| `M` | Mute / Unmute audio |

---

## Build & Run

Requires **Rust** (`cargo`):

```bash
# Run unit tests
cargo test

# Build and run in release mode
cargo run --release
```

---

## Documentation

- [SPEC_AND_DESIGN.md](SPEC_AND_DESIGN.md) — Full language-agnostic game specification & architecture.
- [playground.md](playground.md) — Prompt adaptation for `playground.google` (**Pixel Meadow**).
