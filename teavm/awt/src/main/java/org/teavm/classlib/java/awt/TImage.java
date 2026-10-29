package org.teavm.classlib.java.awt;

import org.teavm.classlib.java.awt.image.TImageObserver;
import org.teavm.classlib.java.awt.image.TImageProducer;

// java.awt.Image
public abstract class TImage {
    public abstract TImageProducer getSource();

    public abstract int getWidth(TImageObserver observer);

    public abstract int getHeight(TImageObserver observer);

    // Not in Java: the picture as ARGB pixels, row by row, for frogger.web.Raster to draw
    public abstract int[] pixels();
}
