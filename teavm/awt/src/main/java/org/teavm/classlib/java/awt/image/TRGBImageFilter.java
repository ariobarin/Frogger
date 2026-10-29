package org.teavm.classlib.java.awt.image;

// java.awt.image.RGBImageFilter: changes each ARGB pixel on its own
public abstract class TRGBImageFilter extends TImageFilter {
    public abstract int filterRGB(int x, int y, int rgb);
}
