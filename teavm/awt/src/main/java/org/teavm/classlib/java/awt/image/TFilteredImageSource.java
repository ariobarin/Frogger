package org.teavm.classlib.java.awt.image;

// java.awt.image.FilteredImageSource: a buffered image's pixels through an RGBImageFilter
public class TFilteredImageSource implements TImageProducer {
    private final TImageProducer source;
    private final TImageFilter filter;

    public TFilteredImageSource(TImageProducer source, TImageFilter filter) {
        this.source = source;
        this.filter = filter;
    }

    public TBufferedImage image() {
        return ((TBufferedImage.Source) source).image();
    }

    public int[] pixels() {
        return image().filteredBy((TRGBImageFilter) filter);
    }
}
