package org.teavm.classlib.java.awt;

import org.teavm.classlib.java.awt.image.TBufferedImage;
import org.teavm.classlib.java.awt.image.TFilteredImageSource;
import org.teavm.classlib.java.awt.image.TImageObserver;
import org.teavm.classlib.java.awt.image.TImageProducer;

// java.awt.Toolkit: makes images from image producers (Util.drawImage's transparency filter)
public class TToolkit {
    private static final TToolkit DEFAULT = new TToolkit();

    public static TToolkit getDefaultToolkit() {
        return DEFAULT;
    }

    public TImage createImage(TImageProducer producer) {
        return new Produced(producer);
    }

    // An image whose pixels come from a buffered image, maybe through a filter
    private static class Produced extends TImage {
        private final TImageProducer producer;

        Produced(TImageProducer producer) {
            this.producer = producer;
        }

        private TBufferedImage image() {
            return producer instanceof TFilteredImageSource filtered ? filtered.image()
                    : ((TBufferedImage.Source) producer).image();
        }

        @Override
        public TImageProducer getSource() {
            return producer;
        }

        @Override
        public int getWidth(TImageObserver observer) {
            return image().getWidth();
        }

        @Override
        public int getHeight(TImageObserver observer) {
            return image().getHeight();
        }

        @Override
        public int[] pixels() {
            return producer instanceof TFilteredImageSource filtered ? filtered.pixels() : image().pixels();
        }
    }
}
