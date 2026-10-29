package org.teavm.classlib.java.awt.image;

import org.teavm.classlib.java.awt.TImage;

// java.awt.image.ImageObserver. Images here are always ready, so nothing calls it.
public interface TImageObserver {
    boolean imageUpdate(TImage img, int infoflags, int x, int y, int width, int height);
}
