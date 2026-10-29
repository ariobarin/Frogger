package frogger.web;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.DataInputStream;
import java.io.IOException;
import java.util.zip.DataFormatException;
import java.util.zip.Inflater;

// Reads PNGs like Frogger's (8 bits per channel, RGBA or RGB, not interlaced) as javax.imageio does:
// the stored values, with no gamma or colour profile correction
public final class Png {
    public final int width;
    public final int height;
    public final boolean alpha;
    public final int[] pixels; // ARGB, row by row

    private static final int IHDR = 0x49484452;
    private static final int IDAT = 0x49444154;
    private static final int IEND = 0x49454e44;

    private Png(int width, int height, boolean alpha, int[] pixels) {
        this.width = width;
        this.height = height;
        this.alpha = alpha;
        this.pixels = pixels;
    }

    public static Png decode(byte[] data) throws IOException {
        DataInputStream in = new DataInputStream(new ByteArrayInputStream(data));
        if (in.readLong() != 0x89504e470d0a1a0aL) {
            throw new IOException("Not a PNG");
        }
        int width = 0;
        int height = 0;
        int channels = 0;
        ByteArrayOutputStream compressed = new ByteArrayOutputStream();
        while (true) {
            byte[] body = new byte[in.readInt()];
            int type = in.readInt();
            in.readFully(body);
            in.readInt(); // CRC
            if (type == IHDR) {
                DataInputStream header = new DataInputStream(new ByteArrayInputStream(body));
                width = header.readInt();
                height = header.readInt();
                int depth = header.readUnsignedByte();
                int colorType = header.readUnsignedByte();
                header.skipBytes(2);
                int interlace = header.readUnsignedByte();
                if (depth != 8 || (colorType != 2 && colorType != 6) || interlace != 0) {
                    throw new IOException("Only 8-bit RGB and RGBA PNGs without interlacing are read here");
                }
                channels = colorType == 6 ? 4 : 3;
            } else if (type == IDAT) {
                compressed.write(body);
            } else if (type == IEND) {
                break;
            }
        }

        int stride = width * channels;
        byte[] raw = inflate(compressed.toByteArray(), height * (stride + 1));
        int[] pixels = new int[width * height];
        byte[] previous = new byte[stride];
        byte[] row = new byte[stride];
        for (int y = 0; y < height; y++) {
            System.arraycopy(raw, y * (stride + 1) + 1, row, 0, stride);
            unfilter(raw[y * (stride + 1)], row, previous, channels);
            for (int x = 0; x < width; x++) {
                int i = x * channels;
                int alpha = channels == 4 ? row[i + 3] & 0xff : 255;
                pixels[y * width + x] = alpha << 24 | (row[i] & 0xff) << 16 | (row[i + 1] & 0xff) << 8 | row[i + 2] & 0xff;
            }
            byte[] done = previous;
            previous = row;
            row = done;
        }
        return new Png(width, height, channels == 4, pixels);
    }

    private static byte[] inflate(byte[] compressed, int length) throws IOException {
        byte[] raw = new byte[length];
        Inflater inflater = new Inflater();
        inflater.setInput(compressed);
        try {
            int done = 0;
            while (done < length && !inflater.finished()) {
                int n = inflater.inflate(raw, done, length - done);
                if (n == 0 && (inflater.needsInput() || inflater.needsDictionary())) {
                    break;
                }
                done += n;
            }
            if (done < length) {
                throw new IOException("PNG data ends early");
            }
        } catch (DataFormatException e) {
            throw new IOException(e);
        } finally {
            inflater.end();
        }
        return raw;
    }

    // Undoes a row's filter (PNG spec section 9), in place; `up` is the row above, already undone
    private static void unfilter(int filter, byte[] row, byte[] up, int bpp) throws IOException {
        for (int i = 0; i < row.length; i++) {
            int left = i >= bpp ? row[i - bpp] & 0xff : 0;
            int above = up[i] & 0xff;
            int corner = i >= bpp ? up[i - bpp] & 0xff : 0;
            int value = row[i] & 0xff;
            switch (filter) {
                case 0 -> {
                }
                case 1 -> value += left;
                case 2 -> value += above;
                case 3 -> value += (left + above) >> 1;
                case 4 -> value += paeth(left, above, corner);
                default -> throw new IOException("Bad PNG filter " + filter);
            }
            row[i] = (byte) value;
        }
    }

    private static int paeth(int left, int above, int corner) {
        int p = left + above - corner;
        int toLeft = Math.abs(p - left);
        int toAbove = Math.abs(p - above);
        int toCorner = Math.abs(p - corner);
        if (toLeft <= toAbove && toLeft <= toCorner) {
            return left;
        }
        return toAbove <= toCorner ? above : corner;
    }
}
