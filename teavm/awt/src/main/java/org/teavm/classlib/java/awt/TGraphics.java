package org.teavm.classlib.java.awt;

import org.teavm.classlib.java.awt.image.TImageObserver;

// java.awt.Graphics: the calls Frogger makes. frogger.web.Raster draws them.
public abstract class TGraphics {
    public abstract TColor getColor();

    public abstract void setColor(TColor color);

    public abstract void fillRect(int x, int y, int width, int height);

    public abstract void drawRect(int x, int y, int width, int height);

    public abstract boolean drawImage(TImage img, int x, int y, int width, int height, TImageObserver observer);

    public abstract void dispose();
}
