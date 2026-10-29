# Frogger in the browser, with TeaVM

The game's Java in `../src` is compiled as it is to WebAssembly by [TeaVM](https://teavm.org). Only the
parts of Java's desktop library that it uses are replaced, with small versions that draw on a web page's
canvas.

- `awt/`: AWT, Swing and ImageIO, the classes and methods Frogger calls.
  - `Graphics` draws on an image with the arithmetic Java2D's software drawing gives. How it scales
    sprites is written from Java's measured behaviour (`parity/jvm/ScaleProbe.java`), not from OpenJDK's
    source.
  - `javax.swing.Timer` ticks on the page's animation frames. The game asks for 1000 / 60 ms, which
    integer division makes 16, and gets 60 ticks a second, one per frame on a 60 Hz screen.
  - Keyboard and mouse events come from the page and go to the game's listeners as Swing sends them.
  - `JOptionPane.showInputDialog` is the browser's prompt, which waits for the answer as Java's dialog does.
  - `ImageIO.read` reads the PNGs from TeaVM's in-memory file system, finding names whatever their case,
    as Windows does (the game asks for `Intro.png`; the file is `intro.png`).
  - `java.util.Scanner`, which TeaVM lacks, and `PrintWriter(File)`, read and save the scores file. The
    page keeps it in local storage between visits.
- `frogger/`: the game and its page. The page fetches the PNGs as a zip of `src/assets` and puts them in
  the file system before the game starts.
- `parity/`: checks the build against the Java version, frame by frame.

## Build

Needs a JDK (21 or newer) and Maven:

```
cd teavm
mvn package
```

The page is in `frogger/target/site`. Serve that folder over HTTP and open `index.html`.

## Check it against the Java version

A script plays the game one line at a time: `seed N`, `step N`, `key CODE down|up`, `move X Y`,
`press X Y`, `lives` (back to full lives) or `shot NAME`. `parity/scripts/game.txt` goes from the title
screen into a game; the `wasm` branch's bot (`cargo run --example bot`) writes longer ones.

Render the Java version's frames with `parity/jvm/FroggerFrames.java` (its header says how), then build
the harness and compare. Both read the repo's `scores.txt`, whose top score shows in every game frame:

```
mvn -P parity package
cd parity
uv run frames.py scripts/game.txt <folder of the Java version's frames>
uv run frames.py probe <ScaleProbe's output>
```
