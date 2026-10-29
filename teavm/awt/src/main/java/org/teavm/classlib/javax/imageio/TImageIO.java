package org.teavm.classlib.javax.imageio;

import frogger.web.Files;
import frogger.web.Png;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import org.teavm.classlib.java.awt.image.TBufferedImage;

// javax.imageio.ImageIO: reads the game's PNGs from the page's file system
public class TImageIO {
    public static TBufferedImage read(File input) throws IOException {
        if (input == null) {
            throw new IllegalArgumentException("input == null!");
        }
        File file = Files.find(input);
        if (file == null) {
            throw new IOException("Can't read input file!");
        }
        try (InputStream in = new FileInputStream(file)) {
            return read(in);
        }
    }

    public static TBufferedImage read(InputStream input) throws IOException {
        if (input == null) {
            throw new IllegalArgumentException("input == null!");
        }
        Png png = Png.decode(input.readAllBytes());
        int type = png.alpha ? TBufferedImage.TYPE_4BYTE_ABGR : TBufferedImage.TYPE_3BYTE_BGR;
        TBufferedImage image = new TBufferedImage(png.width, png.height, type);
        image.setRGB(0, 0, png.width, png.height, png.pixels, 0, png.width);
        return image;
    }
}
