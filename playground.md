# Pixel Meadow — Prompt for `playground.google`

Copy and paste the prompt below into **Playground by Google** (`playground.google`) to generate **Pixel Meadow** with original art, responsive mobile/desktop controls, and all core mechanics.

---

## Prompt

Create an original, cozy, high-resolution 2D falling-block match-3 puzzle game called **Pixel Meadow** ("Drop. Match. Sail Away.").

### Visuals & Theme
- **Setting:** A bright, charming nature/riverbank theme. On the right side (or top strip in portrait mobile layout), an original cute woodland critter in a wooden rowboat bobs gently on animated water waves, reacting to gameplay (happy/cheering on combos, nervous when blocks stack near the top, shocked when heavy stone blocks land). Every few levels, a new original woodland critter (e.g., Bunny, Duckling, Fox, Panda) sails in.
- **Blocks (Colorblind-Friendly):** Render crisp, glossy rounded blocks in 5 colors, each with a distinct inner geometric emblem so players don't rely on color alone:
  1. **Blue** (water ring)
  2. **Green** (diamond leaf)
  3. **Purple** (inner square)
  4. **Red** (core dot)
  5. **Yellow** (cross sparkle)
- **Special & Hazard Blocks:**
  - **Lightning:** Clears the entire column it lands in.
  - **Bomb:** Clears a 3×3 area around where it lands.
  - **Star:** Clears itself and every block on the board matching the color directly beneath it.
  - **Stone:** Unmatchable gray cracked block (cleared only by Lightning, Bomb, or Star) that shakes the boat on impact.

### Core Gameplay
- **Grid:** Default `6 columns × 10 visible rows` (plus 3 hidden spawn rows at the top), with a **Grid Size** setting allowing `5–12` columns and `8–16` rows.
- **Pieces:** Falling 1-to-3 block pieces (vertical 3, horizontal 3, L-shape, J-shape, vertical 2, horizontal 2, or single special blocks).
- **Controls:**
  - Move Left / Right (`ArrowLeft`/`ArrowRight` or `A`/`D`, plus mobile touch swipe/buttons).
  - Rotate / Cycle Colors (`ArrowUp` or `W` or tap): cycles the colors within the piece in place.
  - Fast Drop (`ArrowDown` or `Space`, plus swipe down).
  - **Hold Piece** (`C` or `Shift`, plus on-screen Hold button): bank the current piece in the `HOLD` slot once per drop and swap it back later.
  - **Pause** (`P` or HUD Pause button), **Quit to Menu** (`Esc` or HUD Menu button), **Mute Audio** (`M` or HUD Mute button).
- **Matching & Combos:**
  - When a piece lands, unsupported blocks settle downward, then any 3 or more matching colored blocks in a row **horizontally, vertically, or diagonally** explode with a 2-frame burst animation and collapse columns downward, chaining into combos.
  - **Ghost Drop Preview:** Show a subtle translucent landing outline on the board directly below the active piece.
  - **Combo Fever Meter:** Clearing blocks fills a decaying Fever bar (`1×`, `2×`, `3×`, `4×` score multiplier).
  - **Score Formula:** Special clear = `10 × Fever`; Normal clear = `((combo - 1)² × 2 + cleared_blocks) × Fever`.

### Game Modes & Progression
1. **Classic (10 Levels):** Level up at `30 × level²` points. Clearing Level 10 triggers an "All Cleared!" celebration ending.
2. **Endless:** Unlimited levels with a gentle difficulty curve—fall speed increases smoothly by `50 ms` per level (starting at `1050 ms`/row down to `50 ms`/row) and stone frequency ramps slowly.
3. **Rising Tide:** Unlimited levels where every 8 drops (down to 4 at higher levels), a new row of mixed blocks + 1 stone pushes up from the bottom.
4. **2-Min Blitz:** 120-second countdown timer; combo chains add `+2` seconds.
- **Blind Next Toggle:** Optional menu toggle to hide the `NEXT` piece preview (`?`).
- **Leaderboards:** Save top-5 high scores locally, stored separately for each `(grid size, mode)` combination.

### Audio & Polish
- Synthesize upbeat retro chiptune/square-wave sound effects for piece drop, line clear, combo chains, special blocks, level up, and game over, with an instant Mute toggle.
- Make the layout **fully responsive** for both desktop/landscape and mobile portrait screens (clean touch controls on mobile, no overlapping HUD).
