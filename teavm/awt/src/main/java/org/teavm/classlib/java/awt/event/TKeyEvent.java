package org.teavm.classlib.java.awt.event;

import java.util.EventObject;
import org.teavm.classlib.java.awt.TComponent;

// java.awt.event.KeyEvent. Key codes are Java's VK_ codes, which match the browser's keyCode.
public class TKeyEvent extends EventObject {
    public static final int KEY_FIRST = 400;
    public static final int KEY_TYPED = 400;
    public static final int KEY_PRESSED = 401;
    public static final int KEY_RELEASED = 402;
    public static final int KEY_LAST = 402;
    public static final int VK_LEFT = 37;
    public static final int VK_UP = 38;
    public static final int VK_RIGHT = 39;
    public static final int VK_DOWN = 40;
    public static final char CHAR_UNDEFINED = 0xffff;

    private final int id;
    private final int keyCode;
    private final char keyChar;

    public TKeyEvent(TComponent source, int id, long when, int modifiers, int keyCode, char keyChar) {
        super(source);
        this.id = id;
        this.keyCode = keyCode;
        this.keyChar = keyChar;
    }

    public int getID() {
        return id;
    }

    public int getKeyCode() {
        return keyCode;
    }

    public char getKeyChar() {
        return keyChar;
    }
}
