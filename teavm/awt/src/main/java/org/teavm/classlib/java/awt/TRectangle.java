package org.teavm.classlib.java.awt;

// java.awt.Rectangle
public class TRectangle {
    public int x;
    public int y;
    public int width;
    public int height;

    public TRectangle() {
    }

    public TRectangle(int width, int height) {
        this(0, 0, width, height);
    }

    public TRectangle(int x, int y, int width, int height) {
        this.x = x;
        this.y = y;
        this.width = width;
        this.height = height;
    }

    public boolean contains(TPoint p) {
        return contains(p.x, p.y);
    }

    // Inside means x <= px < x + width and y <= py < y + height
    public boolean contains(int px, int py) {
        return width >= 0 && height >= 0 && px >= x && py >= y && px - x < width && py - y < height;
    }
}
