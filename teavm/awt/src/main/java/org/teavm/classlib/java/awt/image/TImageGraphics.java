package org.teavm.classlib.java.awt.image;

import org.teavm.classlib.java.awt.TColor;
import org.teavm.classlib.java.awt.TGraphics2D;
import org.teavm.classlib.java.awt.TImage;
import org.teavm.classlib.java.awt.TRenderingHints;

// Not in Java by this name: what BufferedImage.getGraphics() gives (Java's sun.java2d.SunGraphics2D). It
// draws the three calls Frogger makes (drawImage, fillRect, drawRect) onto an opaque image's pixels,
// with the arithmetic Java2D's software drawing gives.
public class TImageGraphics extends TGraphics2D {
    private final int width;
    private final int height;
    private final int[] pixels;
    private final int[] columns;
    private final int[] rows;
    private TColor color = TColor.BLACK;

    TImageGraphics(TBufferedImage image) {
        width = image.getWidth();
        height = image.getHeight();
        pixels = image.pixels();
        columns = new int[width];
        rows = new int[height];
    }

    @Override
    public TColor getColor() {
        return color;
    }

    @Override
    public void setColor(TColor color) {
        if (color != null) {
            this.color = color;
        }
    }

    @Override
    public void setRenderingHint(TRenderingHints.Key key, Object value) {
    }

    @Override
    public void dispose() {
    }

    // The image scaled to width x height, nearest neighbour, composited over what is there
    @Override
    public boolean drawImage(TImage img, int x, int y, int width, int height, TImageObserver observer) {
        if (img == null || width <= 0 || height <= 0) {
            return true;
        }
        int sourceWidth = img.getWidth(null);
        int[] source = img.pixels();
        int left = Math.max(x, 0);
        int top = Math.max(y, 0);
        int right = axis(x, width, sourceWidth, this.width, columns);
        int bottom = axis(y, height, img.getHeight(null), this.height, rows);
        for (int dy = top; dy < bottom; dy++) {
            int sy = rows[dy - top];
            if (sy < 0) {
                continue;
            }
            int from = sy * sourceWidth;
            int to = dy * this.width;
            for (int dx = left; dx < right; dx++) {
                int sx = columns[dx - left];
                if (sx < 0) {
                    continue;
                }
                int pixel = source[from + sx];
                int alpha = pixel >>> 24;
                if (alpha == 255) {
                    pixels[to + dx] = pixel;
                } else if (alpha != 0) {
                    pixels[to + dx] = blend(pixel, alpha, pixels[to + dx]);
                }
            }
        }
        return true;
    }

    // One axis of a drawImage: for each screen pixel from max(at, 0) on, the source pixel it shows (-1 for
    // none) goes in map, and the end of the run comes back. As measured from Java (../parity/ScaleProbe.java),
    // it follows a source position in fixed point with 32 fraction bits. That starts under the centre of
    // the first pixel on screen and moves by source / size per pixel, both truncated. An image drawn at
    // its own size is copied.
    static int axis(int at, int size, int source, int screen, int[] map) {
        int first = Math.max(at, 0);
        int last = Math.min(at + size, screen);
        if (size == source) {
            for (int i = first; i < last; i++) {
                map[i - first] = i - at;
            }
            return last;
        }
        double ratio = (double) source / size;
        double unit = 4294967296.0; // 2^32
        long position = (long) ((first + 0.5 - at) * ratio * unit);
        long step = (long) (ratio * unit);
        for (int i = first; i < last; i++) {
            int pixel = (int) (position >> 32);
            map[i - first] = pixel < source ? pixel : -1;
            position += step;
        }
        return last;
    }

    @Override
    public void fillRect(int x, int y, int width, int height) {
        int alpha = color.getAlpha();
        if (alpha == 0) {
            return;
        }
        int pixel = 0xff000000 | color.getRGB();
        int left = Math.max(x, 0);
        int right = Math.min(x + width, this.width);
        for (int dy = Math.max(y, 0); dy < Math.min(y + height, this.height); dy++) {
            for (int dx = left; dx < right; dx++) {
                int i = dy * this.width + dx;
                pixels[i] = alpha == 255 ? pixel : blend(pixel, alpha, pixels[i]);
            }
        }
    }

    // A one pixel outline around x..x + width, y..y + height
    @Override
    public void drawRect(int x, int y, int width, int height) {
        if (width < 0 || height < 0) {
            return;
        }
        fillRect(x, y, width + 1, 1);
        fillRect(x, y + height, width + 1, 1);
        fillRect(x, y + 1, 1, height - 1);
        fillRect(x + width, y + 1, 1, height - 1);
    }

    // Source over an opaque pixel: each channel is src * alpha + dst * (255 - alpha), with each product
    // divided by 255 and rounded to the nearest
    private static int blend(int src, int alpha, int dst) {
        int rest = 255 - alpha;
        int red = mul(alpha, src >> 16 & 0xff) + mul(rest, dst >> 16 & 0xff);
        int green = mul(alpha, src >> 8 & 0xff) + mul(rest, dst >> 8 & 0xff);
        int blue = mul(alpha, src & 0xff) + mul(rest, dst & 0xff);
        return 0xff000000 | red << 16 | green << 8 | blue;
    }

    private static int mul(int a, int b) {
        return (a * b + 127) / 255;
    }
}
