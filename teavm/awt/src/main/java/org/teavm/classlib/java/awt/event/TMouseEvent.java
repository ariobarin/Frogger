package org.teavm.classlib.java.awt.event;

import java.util.EventObject;
import org.teavm.classlib.java.awt.TComponent;

// java.awt.event.MouseEvent
public class TMouseEvent extends EventObject {
    public static final int MOUSE_CLICKED = 500;
    public static final int MOUSE_PRESSED = 501;
    public static final int MOUSE_RELEASED = 502;
    public static final int MOUSE_MOVED = 503;
    public static final int MOUSE_ENTERED = 504;
    public static final int MOUSE_EXITED = 505;
    public static final int MOUSE_DRAGGED = 506;

    private final int id;
    private final int x;
    private final int y;
    private final int clickCount;

    public TMouseEvent(TComponent source, int id, long when, int modifiers, int x, int y, int clickCount,
            boolean popupTrigger) {
        super(source);
        this.id = id;
        this.x = x;
        this.y = y;
        this.clickCount = clickCount;
    }

    public int getID() {
        return id;
    }

    public int getX() {
        return x;
    }

    public int getY() {
        return y;
    }

    public int getClickCount() {
        return clickCount;
    }
}
