// Renders frames of the Java version from a script, headless and deterministic, to compare with the
// TeaVM build (FroggerFramesWeb runs the same scripts; see teavm/README.md).
// Usage (from the Frogger repo root): javac -d out src/*.java teavm/parity/jvm/FroggerFrames.java &&
//   java -Djava.awt.headless=true -cp out FroggerFrames <script> <out dir>
import java.awt.event.*;
import java.awt.image.BufferedImage;
import java.io.*;
import java.lang.reflect.Field;
import java.nio.file.*;
import java.util.*;
import javax.imageio.ImageIO;

public class FroggerFrames {
    public static void main(String[] args) throws Exception {
        List<String> script = Files.readAllLines(Paths.get(args[0]));
        File out = new File(args[1]);
        out.mkdirs();

        long seed = 0;
        int steps = 0;
        for (String line : script) {
            if (line.startsWith("seed ")) seed = Long.parseLong(line.substring(5).trim());
        }
        Field random = Util.class.getDeclaredField("RANDOM");
        random.setAccessible(true);
        ((Random) random.get(null)).setSeed(seed);

        FroggerPanel panel = new FroggerPanel();
        Field timer = FroggerPanel.class.getDeclaredField("timer");
        timer.setAccessible(true);
        ((javax.swing.Timer) timer.get(panel)).stop();
        panel.setSize(800, 800);

        BufferedImage image = new BufferedImage(800, 800, BufferedImage.TYPE_INT_RGB);
        java.awt.Graphics g0 = image.getGraphics();
        g0.setColor(new java.awt.Color(238, 238, 238));
        g0.fillRect(0, 0, 800, 800);
        g0.dispose();
        paint(panel, image);

        for (String line : script) {
            String[] w = line.trim().split("\\s+");
            switch (w[0]) {
                case "step" -> {
                    for (int i = 0; i < Integer.parseInt(w[1]); i++) {
                        steps++;
                        try {
                            panel.step();
                        } catch (java.awt.HeadlessException e) {
                            System.out.println("game over dialog at step " + steps);
                            System.exit(0);
                        }
                        paint(panel, image);
                    }
                }
                case "key" -> {
                    int code = Integer.parseInt(w[1]);
                    KeyEvent e = new KeyEvent(panel, w[2].equals("down") ? KeyEvent.KEY_PRESSED : KeyEvent.KEY_RELEASED, 0, 0, code, KeyEvent.CHAR_UNDEFINED);
                    if (w[2].equals("down")) panel.keyPressed(e); else panel.keyReleased(e);
                }
                case "move" -> {
                    MouseEvent e = new MouseEvent(panel, MouseEvent.MOUSE_MOVED, 0, 0, Integer.parseInt(w[1]), Integer.parseInt(w[2]), 0, false);
                    for (MouseMotionListener l : panel.getMouseMotionListeners()) l.mouseMoved(e);
                    paint(panel, image);
                }
                case "press" -> {
                    MouseEvent e = new MouseEvent(panel, MouseEvent.MOUSE_PRESSED, 0, 0, Integer.parseInt(w[1]), Integer.parseInt(w[2]), 1, false);
                    panel.mousePressed(e);
                }
                case "lives" -> {
                    Field lives = FroggerPanel.class.getDeclaredField("lifeCounter");
                    lives.setAccessible(true);
                    ((LifeCounter) lives.get(panel)).reset();
                }
                case "shot" -> ImageIO.write(image, "png", new File(out, w[1] + ".png"));
                default -> {}
            }
        }
        System.exit(0);
    }

    static void paint(FroggerPanel panel, BufferedImage image) {
        java.awt.Graphics g = image.getGraphics();
        panel.paint(g);
        g.dispose();
    }
}
