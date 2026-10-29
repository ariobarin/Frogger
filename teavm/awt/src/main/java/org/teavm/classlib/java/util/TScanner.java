package org.teavm.classlib.java.util;

import java.io.Closeable;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.util.NoSuchElementException;

// java.util.Scanner, which TeaVM doesn't have: line by line only, as ScoreManager reads its file
public final class TScanner implements Closeable {
    private final String text;
    private int position;
    private boolean closed;

    public TScanner(File source) throws FileNotFoundException {
        this(new FileInputStream(source));
    }

    public TScanner(InputStream source) {
        try {
            text = new String(source.readAllBytes(), StandardCharsets.UTF_8);
        } catch (IOException e) {
            throw new IllegalStateException(e);
        }
    }

    public boolean hasNextLine() {
        ensureOpen();
        return position < text.length();
    }

    // The text up to the next line break (\r\n, \n, \r,  ,   or \u0085), or the rest
    public String nextLine() {
        if (!hasNextLine()) {
            throw new NoSuchElementException("No line found");
        }
        int end = position;
        while (end < text.length() && "\r\n  \u0085".indexOf(text.charAt(end)) < 0) {
            end++;
        }
        String line = text.substring(position, end);
        position = end;
        if (position < text.length()) {
            position += text.startsWith("\r\n", position) ? 2 : 1;
        }
        return line;
    }

    @Override
    public void close() {
        closed = true;
    }

    private void ensureOpen() {
        if (closed) {
            throw new IllegalStateException("Scanner closed");
        }
    }
}
