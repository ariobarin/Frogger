# Frogger

A from-scratch remake of the classic 1981 arcade game Frogger, written in plain
Java (Swing + AWT) with no external libraries. Guide a frog across a busy road
and a hazard-filled river to reach the safety of five home goals before the
clock or your lives run out.

This was a personal project to rebuild a well-known arcade game entirely from
first principles: every sprite, sound, animation, collision rule, and UI screen
is implemented by hand.

## What it does

The frog starts at the bottom of an 800x800 window. The lower half is a road
with lanes of cars moving in both directions at different speeds. The upper
half is a river with logs, diving turtles, and open-mouthed alligators drifting
across it. The frog must hop up, across the river (riding the floating objects),
and land in one of five goal slots at the top. Fill all five goals and the level
ends. Get hit by a car, fall in the water, get eaten by an alligator, drift off
the edge, or run out of time, and the frog dies and a life is lost. Three lives
and three levels (each faster and more crowded than the last) stand between the
player and a cleared run.

## Key features

- Three hand-tuned levels with progressively denser traffic and trickier rivers.
- Five distinct obstacle types: cars (white, red, pink, green, yellow), floating
  logs, breathing turtles that submerge and resurface, and alligators whose
  open mouths are lethal but whose backs are safe to ride.
- Eased jump animation with a two-deep input buffer so quick directional taps
  queue up cleanly instead of being dropped.
- Circle-based collision detection that is intentionally forgiving, plus a
  jump-in-air state that grants brief invulnerability while the frog is hopping.
- Riding physics: when the frog stands on a log, turtle, or alligator, it
  inherits that object's velocity and drifts with it (and off the screen if
  you are not careful).
- Scoring tied to time remaining and to catching bonus flies in the goals, with
  a persistent local leaderboard (stored in `scores.txt`) that survives restarts.
- Animated intro screen, in-game HUD (score, hi-score, lives, timer bar), death
  animation, and a leaderboard screen, all driven by a single 60 FPS game loop.
- Custom button components with mouse-hover highlighting for the menus.

## Tech stack

- Java (Swing for the window, AWT for rendering and input).
- `javax.swing.Timer` driving a fixed-step 60 FPS loop.
- Hand-drawn PNG sprite assets (frog directions, cars, terrain, deaths) loaded
  via `BufferedImage`.
- No build tool is required: plain `.java` sources in `src/` compiled with `javac`.

## How to run

You need a JDK installed (the project is plain Java SE with no dependencies).

1. Compile the sources from the repo root:

   ```
   javac -d bin src/*.java
   ```

2. Run the game (the entry point is `Frogger.main`):

   ```
   java -cp bin Frogger
   ```

The window opens on the intro screen. Click **Play** to start, or
**Leaderboard** to view the high scores.

### Controls

- Arrow keys move the frog one hop (up, down, left, right).
- Mouse clicks navigate the menus.
- Reaching a goal or losing all lives ends the run and (on game over) prompts
  for a name for the leaderboard.

## Project layout

```
src/            Java sources (one class per file)
  Frogger.java          Window and main() entry point
  FroggerPanel.java     Game loop, screen routing, level setup, drawing
  Frog.java             Frog movement, jumping, momentum, collisions, death
  MovingObject.java     Base class for all moving hazards
  Car.java, Log.java, Turtle.java, Alligator.java
  Goal.java, Terrain.java, Score.java, ScoreManager.java
  LifeCounter.java, TimerBar.java, Button.java
  Util.java             Shared image loading and text/sprite drawing helpers
  SoundEffect.java
  assets/               Sprite PNGs, fonts, and sounds (organized by category)
bin/            Compiled .class files
scores.txt      Persistent leaderboard data (player names and scores)
```

## Notes

- All assets live under `src/assets/` and are loaded by relative path at runtime
  (`src/assets/...`), so the game must be launched from the repo root for the
  sprites and sounds to be found.
- The leaderboard is plain text: entries are appended on game over and re-read
  on startup, so editing `scores.txt` directly is a quick way to seed scores.

## In the browser

`wasm/` is the same game ported line by line to Rust and compiled to WebAssembly, with no Java runtime:
the page (`wasm/web/index.html`) loads one ~120 KB file, `frogger.wasm`, which has the images built in.
It runs at the game's own 60 frames a second and draws every frame exactly as Java2D does, down to how
it scales the sprites. This is the version [ariobarin.com/frogger](https://ariobarin.com/frogger) embeds.

```bash
cd wasm
cargo build --release --target wasm32-unknown-unknown
```

To check that the port still matches the Java version, `wasm/parity/` renders the same scripted games
in both and compares the frames pixel for pixel; `cargo run --release --example bot` writes a script
that plays through the levels. `wasm/parity/ScaleProbe.java` measures how Java picks source pixels
when it scales a sprite, and `cargo run --release --example probe_check -- <its output>` checks the
port against it.

```bash
javac -d wasm/parity/out src/*.java wasm/parity/FroggerFrames.java
java -Djava.awt.headless=true -cp wasm/parity/out FroggerFrames <script> java-frames
cargo run --release --manifest-path wasm/Cargo.toml --example frames -- <script> rust-frames scores.txt
uv run --with pillow python wasm/parity/compare.py java-frames rust-frames
```

## License

This project is released under the [MIT License](LICENSE).
