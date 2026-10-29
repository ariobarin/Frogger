package org.teavm.classlib.java.awt;

import frogger.web.Page;
import java.util.ArrayList;
import java.util.List;
import org.teavm.classlib.java.awt.event.TKeyListener;
import org.teavm.classlib.java.awt.event.TMouseListener;
import org.teavm.classlib.java.awt.event.TMouseMotionListener;

// java.awt.Component: size, listeners, focus and repaint, for components in the page's one window
public abstract class TComponent {
    private int width;
    private int height;
    private TDimension preferredSize;
    private boolean focusable = true;
    TContainer parent;
    protected boolean shown; // a window on screen
    private final List<TKeyListener> keyListeners = new ArrayList<>();
    private final List<TMouseListener> mouseListeners = new ArrayList<>();
    private final List<TMouseMotionListener> mouseMotionListeners = new ArrayList<>();

    public int getWidth() {
        return width;
    }

    public int getHeight() {
        return height;
    }

    public void setSize(int width, int height) {
        this.width = width;
        this.height = height;
    }

    public TDimension getPreferredSize() {
        return preferredSize != null ? preferredSize : new TDimension(width, height);
    }

    public void setPreferredSize(TDimension preferredSize) {
        this.preferredSize = preferredSize;
    }

    public boolean isFocusable() {
        return focusable;
    }

    public void setFocusable(boolean focusable) {
        this.focusable = focusable;
    }

    public boolean isShowing() {
        return parent != null ? parent.isShowing() : shown;
    }

    // As in Swing, only a component on screen can take the focus. A window gives it to its first
    // focusable component when it opens.
    public void requestFocus() {
        if (isShowing() && focusable) {
            Page.focus(this);
        }
    }

    public void repaint() {
        if (isShowing()) {
            Page.repaint();
        }
    }

    public void paint(TGraphics g) {
    }

    public void addKeyListener(TKeyListener listener) {
        if (listener != null) {
            keyListeners.add(listener);
        }
    }

    public void addMouseListener(TMouseListener listener) {
        if (listener != null) {
            mouseListeners.add(listener);
        }
    }

    public void addMouseMotionListener(TMouseMotionListener listener) {
        if (listener != null) {
            mouseMotionListeners.add(listener);
        }
    }

    public TKeyListener[] getKeyListeners() {
        return keyListeners.toArray(new TKeyListener[0]);
    }

    public TMouseListener[] getMouseListeners() {
        return mouseListeners.toArray(new TMouseListener[0]);
    }

    public TMouseMotionListener[] getMouseMotionListeners() {
        return mouseMotionListeners.toArray(new TMouseMotionListener[0]);
    }
}
