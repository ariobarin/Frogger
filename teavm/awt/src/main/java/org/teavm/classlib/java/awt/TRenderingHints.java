package org.teavm.classlib.java.awt;

// java.awt.RenderingHints. Images are always scaled nearest neighbour, Java2D's default and the game's choice.
public class TRenderingHints {
    public static final Key KEY_INTERPOLATION = new Key(5);
    public static final Object VALUE_INTERPOLATION_NEAREST_NEIGHBOR = new Object();
    public static final Object VALUE_INTERPOLATION_BILINEAR = new Object();
    public static final Object VALUE_INTERPOLATION_BICUBIC = new Object();

    public static class Key {
        private final int key;

        protected Key(int key) {
            this.key = key;
        }

        protected final int intKey() {
            return key;
        }
    }
}
