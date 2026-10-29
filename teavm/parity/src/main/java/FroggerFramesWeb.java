// FroggerFrames (teavm/parity/jvm) for the TeaVM build: runs a parity script through the game and
// prints a CRC-32 of each shot's RGB bytes, to compare with the Java version's frames (teavm/parity/frames.py does).
// A script that is just "probe" runs ScaleProbe instead. The page (teavm/parity/web/index.html) passes the script.
import frogger.web.Files;
import java.awt.Color;
import java.awt.Graphics;
import java.awt.event.KeyEvent;
import java.awt.event.MouseEvent;
import java.awt.event.MouseMotionListener;
import java.awt.image.BufferedImage;
import java.lang.reflect.Field;
import java.util.Random;
import java.util.zip.CRC32;
import org.teavm.jso.JSBody;

public class FroggerFramesWeb {
    public static void main(String[] args) throws Exception {
        Files.mount();
        String[] script = script().split("\n");
        if (script[0].trim().equals("probe")) {
            ScaleProbe.main(args);
            return;
        }

        long seed = 0;
        int steps = 0;
        for (String line : script) {
            if (line.startsWith("seed ")) seed = Long.parseLong(line.substring(5).trim());
        }
        Field random = Util.class.getDeclaredField("RANDOM");
        random.setAccessible(true);
        ((Random) random.get(null)).setSeed(seed);

        FroggerPanel panel = new FroggerPanel(); // its timer never ticks: no page frames run it here
        panel.setSize(800, 800);

        BufferedImage image = new BufferedImage(800, 800, BufferedImage.TYPE_INT_RGB);
        Graphics g0 = image.getGraphics();
        g0.setColor(new Color(238, 238, 238));
        g0.fillRect(0, 0, 800, 800);
        g0.dispose();
        paint(panel, image);

        for (String line : script) {
            String[] w = line.trim().split("\\s+");
            switch (w[0]) {
                case "step" -> {
                    for (int i = 0; i < Integer.parseInt(w[1]); i++) {
                        steps++;
                        panel.step();
                        if (dialogShown()) {
                            System.out.println("game over dialog at step " + steps);
                            return;
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
                case "shot" -> System.out.println("shot " + w[1] + " " + crc(image));
                default -> {}
            }
        }
    }

    static void paint(FroggerPanel panel, BufferedImage image) {
        Graphics g = image.getGraphics();
        panel.paint(g);
        g.dispose();
    }

    // CRC-32 of the picture's bytes as RGB, row by row (what frames.py reads from the Java version's PNGs)
    static long crc(BufferedImage image) {
        int[] argb = image.getRGB(0, 0, 800, 800, null, 0, 800);
        byte[] rgb = new byte[argb.length * 3];
        for (int i = 0; i < argb.length; i++) {
            rgb[i * 3] = (byte) (argb[i] >> 16);
            rgb[i * 3 + 1] = (byte) (argb[i] >> 8);
            rgb[i * 3 + 2] = (byte) argb[i];
        }
        CRC32 crc = new CRC32();
        crc.update(rgb);
        return crc.getValue();
    }

    @JSBody(script = "return froggerScript;")
    private static native String script();

    // The game over dialog (the page's prompt notes it). The Java version's script runs stop there.
    @JSBody(script = "return !!window.froggerDialog;")
    private static native boolean dialogShown();
}
