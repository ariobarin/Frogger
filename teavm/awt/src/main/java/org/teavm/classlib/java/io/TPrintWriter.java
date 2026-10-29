package org.teavm.classlib.java.io;

import java.io.BufferedWriter;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.OutputStream;
import java.io.OutputStreamWriter;
import java.io.Writer;
import java.util.Locale;

// java.io.PrintWriter. It replaces TeaVM's own, which can't open a file (ScoreManager saves with
// new PrintWriter(file)). Like Java's, it never throws IOException; checkError() tells.
public class TPrintWriter extends Writer {
    protected Writer out;
    private final boolean autoFlush;
    private boolean error;

    public TPrintWriter(Writer out) {
        this(out, false);
    }

    public TPrintWriter(Writer out, boolean autoFlush) {
        this.out = out;
        this.autoFlush = autoFlush;
    }

    public TPrintWriter(OutputStream out) {
        this(out, false);
    }

    public TPrintWriter(OutputStream out, boolean autoFlush) {
        this(new BufferedWriter(new OutputStreamWriter(out)), autoFlush);
    }

    public TPrintWriter(String fileName) throws FileNotFoundException {
        this(new FileOutputStream(fileName));
    }

    public TPrintWriter(File file) throws FileNotFoundException {
        this(new FileOutputStream(file));
    }

    public boolean checkError() {
        if (out != null) {
            flush();
        }
        return error;
    }

    protected void setError() {
        error = true;
    }

    protected void clearError() {
        error = false;
    }

    @Override
    public void flush() {
        if (out == null) {
            return;
        }
        try {
            out.flush();
        } catch (IOException e) {
            error = true;
        }
    }

    @Override
    public void close() {
        if (out == null) {
            return;
        }
        try {
            out.close();
        } catch (IOException e) {
            error = true;
        }
        out = null;
    }

    @Override
    public void write(char[] buffer, int offset, int length) {
        if (out == null) {
            error = true;
            return;
        }
        try {
            out.write(buffer, offset, length);
        } catch (IOException e) {
            error = true;
        }
    }

    @Override
    public void write(char[] buffer) {
        write(buffer, 0, buffer.length);
    }

    @Override
    public void write(int c) {
        write(new char[] { (char) c }, 0, 1);
    }

    @Override
    public void write(String text, int offset, int length) {
        write(text.substring(offset, offset + length).toCharArray());
    }

    @Override
    public void write(String text) {
        write(text.toCharArray());
    }

    public void print(String text) {
        write(String.valueOf(text));
    }

    public void print(Object value) {
        print(String.valueOf(value));
    }

    public void print(char[] chars) {
        write(chars);
    }

    public void print(char c) {
        write(c);
    }

    public void print(boolean value) {
        print(String.valueOf(value));
    }

    public void print(int value) {
        print(String.valueOf(value));
    }

    public void print(long value) {
        print(String.valueOf(value));
    }

    public void print(float value) {
        print(String.valueOf(value));
    }

    public void print(double value) {
        print(String.valueOf(value));
    }

    public void println() {
        print(System.lineSeparator());
        if (autoFlush) {
            flush();
        }
    }

    public void println(String text) {
        print(text);
        println();
    }

    public void println(Object value) {
        println(String.valueOf(value));
    }

    public void println(char[] chars) {
        print(chars);
        println();
    }

    public void println(char c) {
        println(String.valueOf(c));
    }

    public void println(boolean value) {
        println(String.valueOf(value));
    }

    public void println(int value) {
        println(String.valueOf(value));
    }

    public void println(long value) {
        println(String.valueOf(value));
    }

    public void println(float value) {
        println(String.valueOf(value));
    }

    public void println(double value) {
        println(String.valueOf(value));
    }

    public TPrintWriter format(String format, Object... args) {
        return format(Locale.getDefault(), format, args);
    }

    public TPrintWriter format(Locale locale, String format, Object... args) {
        print(String.format(locale, format, args));
        if (autoFlush) {
            flush();
        }
        return this;
    }

    public TPrintWriter printf(String format, Object... args) {
        return format(format, args);
    }

    public TPrintWriter printf(Locale locale, String format, Object... args) {
        return format(locale, format, args);
    }

    @Override
    public TPrintWriter append(char c) {
        write(c);
        return this;
    }

    @Override
    public TPrintWriter append(CharSequence text) {
        print(String.valueOf(text));
        return this;
    }

    @Override
    public TPrintWriter append(CharSequence text, int start, int end) {
        return append(text == null ? "null".subSequence(start, end) : text.subSequence(start, end));
    }
}
