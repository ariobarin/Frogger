// Measures which source pixel Java picks for each screen pixel when Util.drawImage scales a sprite, the
// way the game draws them (sprites have transparent pixels). Prints one line per case:
// "<source size> <drawn size> <x>: <source column for screen x = 0..799, or -1>"; examples/probe_check.rs
// compares the port's sampling against it.
// Usage (from the Frogger repo root): javac -d out src/*.java wasm/parity/ScaleProbe.java && java -Djava.awt.headless=true -cp out ScaleProbe > probe.txt
import java.awt.Graphics;
import java.awt.image.BufferedImage;

public class ScaleProbe {
    public static void main(String[] args) {
        int[] sources = {8, 9, 14, 15, 16, 17, 24, 27, 32, 49, 296};
        int[] sizes = {20, 24, 30, 40, 42, 50, 60, 100, 125, 150, 200, 300};
        int[] offsets = {50, 3, 0, -7, -23, -60, 760};
        for (int sw : sources) {
            // row 0 numbers its columns (never black); row 1 is see-through, as every sprite has some
            BufferedImage src = new BufferedImage(sw, 2, BufferedImage.TYPE_INT_ARGB);
            for (int x = 0; x < sw; x++) {
                src.setRGB(x, 0, 0xff000000 | (x + 1));
                src.setRGB(x, 1, 0);
            }
            for (int dw : sizes) {
                for (int at : offsets) {
                    BufferedImage dst = new BufferedImage(800, 2, BufferedImage.TYPE_INT_RGB);
                    Graphics g = dst.getGraphics();
                    Util.drawImage(g, src, at, 0, dw, 2); // 2 rows tall, so screen row 0 shows source row 0
                    g.dispose();
                    StringBuilder line = new StringBuilder(sw + " " + dw + " " + at + ":");
                    for (int x = 0; x < 800; x++) {
                        int v = dst.getRGB(x, 0) & 0xffffff;
                        line.append(' ').append(v == 0 ? -1 : v - 1);
                    }
                    System.out.println(line);
                }
            }
        }
        System.exit(0);
    }
}
