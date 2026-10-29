package org.teavm.classlib.java.awt.image;

import org.teavm.classlib.java.awt.TGraphics;
import org.teavm.classlib.java.awt.TGraphics2D;
import org.teavm.classlib.java.awt.TImage;

// java.awt.image.BufferedImage, kept as ARGB pixels whatever its type
public class TBufferedImage extends TImage {
    public static final int TYPE_INT_RGB = 1;
    public static final int TYPE_INT_ARGB = 2;
    public static final int TYPE_3BYTE_BGR = 5;
    public static final int TYPE_4BYTE_ABGR = 6;

    private final int width;
    private final int height;
    private final int type;
    private final int[] pixels;
    private final Source source = new Source();

    // The last filtered copy (Util.drawImage filters every sprite each time it draws it)
    private Class<?> filteredWith;
    private int[] filtered;

    public TBufferedImage(int width, int height, int type) {
        this(width, height, type, new int[width * height]);
        if (type == TYPE_INT_RGB || type == TYPE_3BYTE_BGR) {
            java.util.Arrays.fill(pixels, 0xff000000);
        }
    }

    // Not in Java: an image around ARGB pixels (javax.imageio.ImageIO's)
    TBufferedImage(int width, int height, int type, int[] pixels) {
        if (width <= 0 || height <= 0) {
            throw new IllegalArgumentException("Width (" + width + ") and height (" + height + ") cannot be <= 0");
        }
        this.width = width;
        this.height = height;
        this.type = type;
        this.pixels = pixels;
    }

    public int getWidth() {
        return width;
    }

    public int getHeight() {
        return height;
    }

    public int getType() {
        return type;
    }

    @Override
    public int getWidth(TImageObserver observer) {
        return width;
    }

    @Override
    public int getHeight(TImageObserver observer) {
        return height;
    }

    public int getRGB(int x, int y) {
        check(x, y);
        return pixels[y * width + x];
    }

    public int[] getRGB(int startX, int startY, int w, int h, int[] rgbArray, int offset, int scansize) {
        check(startX, startY);
        check(startX + w - 1, startY + h - 1);
        int[] out = rgbArray != null ? rgbArray : new int[offset + h * scansize];
        for (int row = 0; row < h; row++) {
            System.arraycopy(pixels, (startY + row) * width + startX, out, offset + row * scansize, w);
        }
        return out;
    }

    public void setRGB(int startX, int startY, int w, int h, int[] rgbArray, int offset, int scansize) {
        check(startX, startY);
        check(startX + w - 1, startY + h - 1);
        for (int row = 0; row < h; row++) {
            for (int col = 0; col < w; col++) {
                setRGB(startX + col, startY + row, rgbArray[offset + row * scansize + col]);
            }
        }
    }

    public TGraphics getGraphics() {
        return createGraphics();
    }

    public TGraphics2D createGraphics() {
        filteredWith = null;
        return new TImageGraphics(this);
    }

    public void setRGB(int x, int y, int rgb) {
        check(x, y);
        boolean opaque = type == TYPE_INT_RGB || type == TYPE_3BYTE_BGR;
        pixels[y * width + x] = opaque ? 0xff000000 | rgb : rgb;
        filteredWith = null;
    }

    private void check(int x, int y) {
        if (x < 0 || y < 0 || x >= width || y >= height) {
            throw new ArrayIndexOutOfBoundsException("Coordinate out of bounds!");
        }
    }

    // A copy here, not a view as in Java; nothing draws into the game's images once they are loaded
    public TBufferedImage getSubimage(int x, int y, int w, int h) {
        if (x < 0 || y < 0 || w <= 0 || h <= 0 || x + w > width || y + h > height) {
            throw new IllegalArgumentException("Subimage (" + x + ", " + y + ", " + w + ", " + h + ") is outside the image");
        }
        int[] copy = new int[w * h];
        for (int row = 0; row < h; row++) {
            System.arraycopy(pixels, (y + row) * width + x, copy, row * w, w);
        }
        return new TBufferedImage(w, h, type, copy);
    }

    @Override
    public TImageProducer getSource() {
        return source;
    }

    @Override
    public int[] pixels() {
        return pixels;
    }

    // The pixels through an RGBImageFilter. Kept for the next filter of the same class: the game's one
    // filter (black becomes see-through) gives the same pixels every time.
    int[] filteredBy(TRGBImageFilter filter) {
        if (filteredWith != filter.getClass()) {
            filtered = new int[pixels.length];
            for (int y = 0; y < height; y++) {
                for (int x = 0; x < width; x++) {
                    filtered[y * width + x] = filter.filterRGB(x, y, pixels[y * width + x]);
                }
            }
            filteredWith = filter.getClass();
        }
        return filtered;
    }

    // What getSource() gives: this image's pixels
    public final class Source implements TImageProducer {
        public TBufferedImage image() {
            return TBufferedImage.this;
        }
    }
}
