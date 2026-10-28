/*
 * Util.java
 * Ario Barin Ostovary
 * This class contains utility methods for graphics and random number generation.
 */

import java.awt.*;
import java.awt.image.*;
import java.io.*;
import java.util.*;
import javax.imageio.*;

public class Util {
    private static final Random RANDOM = new Random();

    public static final int RED_FONT = 0, WHITE_FONT = 1, YELLOW_FONT = 2, FROGGER_FONT = 3;

    // Images
    public static final BufferedImage[] FULL_FONT_IMAGES = new BufferedImage[4];
    public static final BufferedImage[][] FONT_IMAGES = new BufferedImage[4][128];

    // Characters in the fonts in order from left to right
    public static final String DEFAULT_FONT_CHARACTERS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-";
    public static final String FROGGER_FONT_CHARACTERS = "FROGE";

    // Preload the fonts
    static {
        loadFonts();
    }

    private static void loadFonts() {
        // Load the red font
        FULL_FONT_IMAGES[RED_FONT] = loadImage("src/assets/Fonts/red.png");
        for (char c : DEFAULT_FONT_CHARACTERS.toCharArray()) {
            FONT_IMAGES[RED_FONT][c] = getFontImage(RED_FONT, c);
        }

        // Load the white font
        FULL_FONT_IMAGES[WHITE_FONT] = loadImage("src/assets/Fonts/white.png");
        for (char c : DEFAULT_FONT_CHARACTERS.toCharArray()) {
            FONT_IMAGES[WHITE_FONT][c] = getFontImage(WHITE_FONT, c);
        }

        // Load the yellow font
        FULL_FONT_IMAGES[YELLOW_FONT] = loadImage("src/assets/Fonts/yellow.png");
        for (char c : DEFAULT_FONT_CHARACTERS.toCharArray()) {
            FONT_IMAGES[YELLOW_FONT][c] = getFontImage(YELLOW_FONT, c);
        }

        // Load the frogger font
        FULL_FONT_IMAGES[FROGGER_FONT] = loadImage("src/assets/Fonts/frogger.png");
        for (char c : FROGGER_FONT_CHARACTERS.toCharArray()) {
            FONT_IMAGES[FROGGER_FONT][c] = getFontImage(FROGGER_FONT, c);
        }
    }

    // Generate a random integer between min and max
    public static int randomInt(int min, int max) {
        return RANDOM.nextInt(max - min + 1) + min;
    }

    // Generate a random double between min and max
    public static double randomDouble(double min, double max) {
        return min + (max - min) * RANDOM.nextDouble();
    }

    // Load an image from a path. The browser build packs the images into the jar, so look on the
    // classpath first (see web/index.html); on the desktop they are plain files
    public static BufferedImage loadImage(String path) {
        try {
            InputStream packed = Util.class.getResourceAsStream("/" + path);
            if (packed != null) {
                return ImageIO.read(packed);
            }
            return ImageIO.read(new File(path));
        } catch (IOException e) {
            throw new RuntimeException("Could not load image at path: " + path, e);
        }
    }

    // Draw an image with transparency - black pixels are transparent (0, 0, 0)
    public static void drawImage(Graphics g, BufferedImage img, int x, int y, int width, int height) {
        // Create a filter to make black pixels transparent
        ImageFilter filter = new RGBImageFilter() {
            private final int transparentColor = Color.BLACK.getRGB() | 0xFF000000;

            @Override
            public final int filterRGB(int x, int y, int rgb) {
                if ((rgb | 0xFF000000) == transparentColor) {
                    return 0x00FFFFFF & rgb; // Make black pixels transparent
                }
                return rgb; // Keep other pixels unchanged
            }
        };

        // Apply the transparency filter
        ImageProducer ip = new FilteredImageSource(img.getSource(), filter);
        Image transparentImg = Toolkit.getDefaultToolkit().createImage(ip);

        // Draw the image with nearest neighbor interpolation - no blurring
        Graphics2D g2d = (Graphics2D) g;
        g2d.setRenderingHint(RenderingHints.KEY_INTERPOLATION, RenderingHints.VALUE_INTERPOLATION_NEAREST_NEIGHBOR);
        g2d.drawImage(transparentImg, x, y, width, height, null);
    }

    // Get the image of a character from the font
    private static BufferedImage getFontImage(int color, char c) {
        BufferedImage fontImage = FULL_FONT_IMAGES[color];
        String characters = color == FROGGER_FONT ? "FROGE" : "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-";

        int charIndex = characters.indexOf(c);
        if (charIndex == -1) {
            System.out.println("Character not found: " + c);
            return new BufferedImage(1, fontImage.getHeight(), BufferedImage.TYPE_INT_ARGB);
        }

        int charWidth = fontImage.getWidth() / characters.length();
        int x = charIndex * charWidth;
        int y = 0;
        int width = charWidth;
        int height = fontImage.getHeight();

        return fontImage.getSubimage(x, y, width, height);
    }

    // Write text right-aligned to the x position
    public static void writeRightText(Graphics g, String text, int x, int y, int height, int color) {
        int textWidth = text.length() * height;
        writeText(g, text, x - textWidth, y, textWidth, height, color);
    }

    // Write text centered to the x position
    public static void writeCenteredText(Graphics g, String text, int center_x, int center_y, int height, int color) {
        int textWidth = text.length() * height;
        int x = center_x - textWidth / 2;
        writeText(g, text, x, center_y - height / 2, textWidth, height, color);
    }

    // Write text to the screen
    public static void writeText(Graphics g, String text, int x, int y, int height, int color) {
        writeText(g, text, x, y, text.length() * height, height, color);
    }

    // Write text to the screen
    public static void writeText(Graphics g, String text, int x, int y, int width, int height, int color) {
        int charWidth = width / text.length();
        for (int i = 0; i < text.length(); i++) {
            char c = text.charAt(i);
            if (c == ' ') {
                continue;
            }

            // Convert to uppercase - the font only has uppercase characters
            c = Character.toUpperCase(c);

            // Get the image of the character
            BufferedImage fontImage = FONT_IMAGES[color][c];

            // Draw the character
            drawImage(g, fontImage, x + i * charWidth, y, charWidth, height);
        }
    }
}
