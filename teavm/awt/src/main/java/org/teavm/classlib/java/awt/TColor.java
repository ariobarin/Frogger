package org.teavm.classlib.java.awt;

// java.awt.Color. It replaces TeaVM's own Color, which has no getRGB.
public class TColor {
    public static final TColor BLACK = new TColor(0, 0, 0);
    public static final TColor WHITE = new TColor(255, 255, 255);

    private final int value; // ARGB

    public TColor(int r, int g, int b) {
        this(r, g, b, 255);
    }

    public TColor(int r, int g, int b, int a) {
        String bad = (a < 0 || a > 255 ? " Alpha" : "") + (r < 0 || r > 255 ? " Red" : "")
                + (g < 0 || g > 255 ? " Green" : "") + (b < 0 || b > 255 ? " Blue" : "");
        if (!bad.isEmpty()) {
            throw new IllegalArgumentException("Color parameter outside of expected range:" + bad);
        }
        value = a << 24 | r << 16 | g << 8 | b;
    }

    public TColor(int rgb) {
        value = 0xff000000 | rgb;
    }

    public TColor(int rgba, boolean hasAlpha) {
        value = hasAlpha ? rgba : 0xff000000 | rgba;
    }

    public int getRed() {
        return value >> 16 & 0xff;
    }

    public int getGreen() {
        return value >> 8 & 0xff;
    }

    public int getBlue() {
        return value & 0xff;
    }

    public int getAlpha() {
        return value >>> 24;
    }

    public int getRGB() {
        return value;
    }

    @Override
    public boolean equals(Object other) {
        return other instanceof TColor color && color.value == value;
    }

    @Override
    public int hashCode() {
        return value;
    }
}
