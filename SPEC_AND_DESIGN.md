# Nature Park HD — Game Specification & Design Document

A complete, language-agnostic specification for rebuilding **Nature Park HD** (based on the classic Nokia Series 40 J2ME game `com.nokia.mid.appl.np`, extended with HD rendering, customizable grid sizes, new game modes, and skill-ceiling mechanics).

---

## 1. High-Level Overview

Nature Park is a falling-block match-3 puzzle game. Players steer and color-cycle 1–3 block pieces falling down a vertical well, matching **3 or more identical colored blocks** horizontally, vertically, or diagonally. Clearing blocks triggers gravity collapses, chain combos, a Fever multiplier, special power-up blocks, and level progression featuring 4 animated boat-riding characters (**Bunny**, **Du**, **Fenny**, **Pan**).

---

## 2. Coordinate System & Grid Model

### 2.1 Customizable Dimensions
- **Columns (`cols`)**: `5..=12` (default: `6`)
- **Visible Rows (`visible_rows`)**: `8..=16` (default: `10`)
- **Buffer Rows (`BUFFER_ROWS`)**: `3` hidden rows (`0, 1, 2`) above the visible playfield for piece spawning and overflow detection.
- **Total Rows (`total_rows`)**: `visible_rows + 3` (default: `13`, indexed `0..total_rows - 1` top-to-bottom).

### 2.2 Cell Values (`board[col][row]`)
| Value | Meaning |
| :--- | :--- |
| `0` | Empty cell |
| `1..5` | Normal colored blocks (`1=Blue`, `2=Green`, `3=Purple`, `4=Red`, `5=Yellow`) |
| `6` | Special: **Lightning** (clears entire column) |
| `7` | Special: **Bomb** (`block_n`, clears `3×3` area) |
| `8` | Special: **Star** (clears all blocks matching the tile directly below it) |
| `9` | Hazard: **Stone** (unmatchable; only cleared by Lightning/Bomb/Star; triggers character cry/shake) |
| `100` | Explosion animation frame 1 |
| `101` | Explosion animation frame 2 (transitions to `0` on next explosion tick) |

---

## 3. Piece Definitions & Spawning

### 3.1 Piece Shapes (`SHAPES[0..9][3][3]`)
Every piece occupies a `3×3` local matrix `[row][col]`:

- **Type `0` (Vertical 3)**: `(0,1), (1,1), (2,1)`
- **Type `1` (Horizontal 3)**: `(2,0), (2,1), (2,2)`
- **Type `2` (L-Shape)**: `(1,1), (2,0), (2,1)`
- **Type `3` (J-Shape)**: `(1,0), (2,0), (2,1)`
- **Type `4` (Horizontal 2)**: `(2,0), (2,1)`
- **Type `5` (Vertical 2)**: `(1,1), (2,1)`
- **Type `6` (Lightning)**: `(2,1)` — tile `6`
- **Type `7` (Bomb)**: `(2,1)` — tile `7`
- **Type `8` (Star)**: `(2,1)` — tile `8`

### 3.2 Spawn Position
- `spawn_col = max(0, (cols - 3) / 2)`
- `piece_pos = [spawn_col, 0]`

### 3.3 Piece Generation & Throttling
1. Roll `ptype = rand(0..9)`.
2. If `ptype` is `6, 7, 8` (special block), check three throttle counters `special_throttle[0..3]` with thresholds `15`, `12`, `10`. If none have reached their threshold, increment them sequentially and re-roll `ptype`; otherwise reset the triggered counter to `0` and accept the special piece.
3. If `ptype < 6`, check `stone_counter >= max(4, (10 - min(level, 9)) * 2)`:
   - If reached: reset `stone_counter = 0` and fill the piece with **Stone (`9`)** blocks.
   - Otherwise: increment `stone_counter` and assign each occupied cell an independent random color `1..5`.

### 3.4 Color Cycling (Rotation)
Rotation cycles the colors within the piece's cells without changing its geometry:
- `ptype <= 3` (3-block pieces): `(c0, c1, c2) -> (c1, c2, c0)`
- `ptype == 4 || ptype == 5` (2-block pieces): `(c0, c1) -> (c1, c0)`
- `ptype >= 6` (1-block specials): no-op.

---

## 4. Simulation Loop & Timing

### 4.1 Fixed Tick & Step Dividers
- **Base tick**: `50 ms` (`20 Hz`), incrementing `tick_counter` (`0..100` wrap).
- Each state advances only when `should_step(tick_counter)` is true:

| State | Tick Divider | Notes |
| :--- | :--- | :--- |
| `Intro` | `% 7` | 9 frames (`sub_tick = 0..8`), then transitions to `Menu` |
| `LevelExit` / `LevelEnter` | `% 5` | 10 frames each for boat exit/enter animation |
| `SpawnFirst` / `GameOver` | `% 9` | `GameOver` blinks 9 frames before returning to `Menu` |
| `SpawnNext` / `Falling` | Level-dependent | Classic/Tide/Blitz: L1:`21`, L2:`19`, L3:`15`, L4:`11`, L5:`9`, L6:`7`, L7:`5`, L8:`3`, L9+:`1`. Endless: `max(1, 22 - level)` (`50ms` faster per level) |
| `Collapse` | `% 1` | Immediate gravity collapse |
| `Evaluate` | `% 3` | Match/special evaluation |
| `Exploding` | `% 7` | 3 frames (`100 -> 101 -> 0`) |
| `StoneShake` | `% 7` | 3 frames of character crying animation |
| `AllCleared` | `% 7` | 9 frames blinking banner -> `Ending` |
| `Ending` | `% 2` | 60 frames celebration -> `Menu` |

### 4.2 State Machine
```
Intro -> Menu -> LevelEnter -> SpawnFirst -> Falling
                                  ^             |
                                  |             v
                            SpawnNext <- Evaluate <- Collapse
                                  |         |           ^
                                  |         v           |
                                  |     Exploding ------+
                                  |
                                  +-> LevelExit -> (level + 1) -> LevelEnter
                                  +-> GameOver -> Menu
                                  +-> AllCleared -> Ending -> Menu
```

---

## 5. Core Gameplay Rules

### 5.1 Movement, Collision & Fast Drop
- **Move Left / Right**: Only allowed while `state == Falling` and `piece_pos[1] > 0`. Checks left/right bounds (allowing `-1` or `cols - 2` when the outer column of the `3×3` piece matrix is empty) and `board` occupancy.
- **Ghost Row**: Computed by scanning downward from `piece_pos[1]` until the lowest occupied cell in any column of the `3×3` piece touches a non-zero cell or `total_rows - 1`.
- **Fast Drop**: Immediately transitions `Falling -> Collapse`, letting `step_collapse()` drop all unsupported blocks to rest.
- **Hold (`C` / `Shift`)**: Swaps the active piece with `hold_piece` once per drop (`hold_used_this_turn` resets on `step_spawn`).

### 5.2 Gravity Collapse (`step_collapse`)
For every column `0..cols` and row from bottom to top, any non-zero cell falls into the lowest contiguous empty cells below it. If a Bomb (`7`) or Star (`8`) moves, `piece_pos[1]` tracks its landed row.

### 5.3 Match & Special Evaluation (`step_evaluate`)
- **Lightning (`ptype == 6`)**: Marks every occupied cell in column `piece_pos[0] + 1` as `100`.
- **Bomb (`ptype == 7`)**: Marks every occupied cell in the `3×3` neighborhood around `(piece_pos[0] + 1, piece_pos[1])` as `100`.
- **Star (`ptype == 8`)**: Marks the Star itself and all cells on the board matching `board[piece_pos[0] + 1][piece_pos[1] + 1]` as `100`.
- **Normal (`ptype <= 5`)**: Scans horizontal, vertical, and both diagonals (`\` and `/`) for runs of `>= 3` identical cells (`1..5`, excluding `9`/`10`/`100`). All matched cells become `100`.

### 5.4 Scoring, Combos & Fever Multiplier
- **Fever Meter (`0.0..=1.0`)**:
  - Decays by `0.0025` per `50ms` tick during active play.
  - Increases by `0.22 + 0.08 * combo` on each clear.
  - Multiplier: `<0.25 -> 1×`, `<0.55 -> 2×`, `<0.85 -> 3×`, `>=0.85 -> 4×`.
- **Score Gain**:
  - Special clear (`6, 7, 8`): `10 * fever_multiplier`
  - Normal clear: `((combo - 1)^2 * 2 + cleared_count) * fever_multiplier`
- **Overflow / Game Over**: If no match occurs and any cell in buffer rows `1` or `2` is non-zero (`board[c][1] > 0 || board[c][2] > 0`), trigger `GameOver` and persist top-5 high scores.

---

## 6. Game Modes & Level Progression

- **Level Up Threshold**: `score >= 30 * level * level`.
- **Modes**:
  1. **Classic (`Classic`)**: Ends at Level 10 (`level + 1 >= 11` triggers `AllCleared` + `Ending`).
  2. **Endless (`Endless`)**: Unlimited levels past Level 10 with a gentler difficulty curve—fall speed increases by `1` tick (`50 ms`) per level (`max(1, 22 - level)`) and stone frequency ramps slowly (`max(6, 24 - level)`).
  3. **Rising Tide (`RisingTide`)**: Unlimited levels + every `tide_interval` drops (`max(4, 8 - level / 2)`), shifts the entire board up 1 row and inserts a new bottom row of random colors + 1 stone block.
  4. **2-Min Blitz (`Blitz`)**: Starts with `2400` ticks (`120s`). Each combo chain (`combo > 1`) adds `+40` ticks (`+2s`). Reaching `0` ends the run and records high score.
- **Blind Next (`blind_next`)**: Optional toggle that hides the `NEXT` piece preview (`?`).

---

## 7. Character Animation State Machine

Four characters cycle by `(level - 1) % 4` (classic mapping: `1,5,9 -> Bunny`, `2,6,10 -> Du`, `3,7 -> Fenny`, `4,8 -> Pan`):
- **Normal**: `eye=0, tear=0, mouth=2, hand=2`
- **Danger** (any block in rows `0..6`): `eye=3, tear=3`
- **Happy / Match** (`Exploding` / `match_found`): `eye=1, tear=1, hand=3`
- **Stone Shake** (`StoneShake`): `eye=2, tear=2, mouth=0..2, hand=3`

---

## 8. Audio Specification (Nokia OTT `FORMAT_TONE`)

The original JAR embeds 8 Nokia Smart Messaging **OTT** bit-streams (`com.nokia.mid.appl.np.a`, indices `0..7`):
- `0`: Theme, `1`: Combo, `2`: Clear, `3`: Special, `4`: Level Up, `5`: Game Over, `6`: Unused, `7`: Intro.
- **Bit-stream format**:
  1. `8` bits command length, `7` bits command ID (`RingingTone`), `8` bits `Sound` header (`23` bits total).
  2. `3` bits `song_type` (if `1`, read `4` bits `title_len` and skip `title_len * 8` bits), `8` bits `pattern_count`.
  3. Per pattern: `3` bits header, `2` bits pattern ID, `4` bits loop count, `8` bits `instr_count`.
  4. Instructions (`3` bits ID): `1`=Note (`4b note, 3b dur, 2b spec`), `2`=Scale (`2b`), `3`=Style (`2b`), `4`=Tempo (`5b` index into standard Nokia BPM table `25..900`), `5`=Volume (`4b`).
- Synthesize as a monophonic square wave (`44.1 kHz`, `88%` note gate).

---

## 9. Visual & HD Sprite Generation

- **Window**: Default `960×720` (resizable), `FilterMode::Linear`.
- **HD Blocks (`64×64` PNGs)**: Generated procedurally using a rounded-box SDF (`radius=12`), diagonal gradient, dark outer rim, bright inner bevel, top-left specular highlight, and distinct geometric emblems for colorblind accessibility.
- **HD Character & Scene Sprites**: Original PNGs upscaled `4×` using **2 passes of Scale2x (EPX)** so outlines remain crisp without blocky stair-stepping.
- **Dynamic Playfield Sizing**: `tile_size = clamp(floor(min(avail_w / cols, avail_h / visible_rows)), 24, 64)` centered in the left pane; right pane (`300 px`) hosts `NEXT`, `HOLD (C)`, `FEVER` bar, and the animated character boat.

---

## 10. Controls & Persistence

| Input | Action |
| :--- | :--- |
| `Left` / `Right` / `A` / `D` / `4` / `6` | Move piece left / right |
| `Up` / `W` / `Z` / `2` | Cycle piece colors |
| `Down` / `S` / `Space` / `5` / `8` | Fast drop |
| `C` / `Shift` | Hold / swap piece |
| `P` / HUD `PAUSE` | Pause / Resume |
| `Esc` / HUD `MENU` | Quit to Menu |
| `M` | Mute / Unmute audio |

- **High Scores**: Top 5 entries (`{ score, level, year, month, day }`) stored separately per `(grid size, mode)` (`"{cols}x{visible_rows}:{mode}"`) in JSON (`highscores_v2.json`).
