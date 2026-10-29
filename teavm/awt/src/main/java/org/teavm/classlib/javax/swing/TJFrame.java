package org.teavm.classlib.javax.swing;

import frogger.web.Page;
import org.teavm.classlib.java.awt.TComponent;
import org.teavm.classlib.java.awt.TContainer;
import org.teavm.classlib.java.awt.TDimension;

// javax.swing.JFrame: the page's canvas is the window, without a frame around it
public class TJFrame extends TContainer {
    public static final int EXIT_ON_CLOSE = 3;

    private String title;
    private int closeOperation;

    public TJFrame() {
        this("");
    }

    public TJFrame(String title) {
        this.title = title;
    }

    public String getTitle() {
        return title;
    }

    public void setTitle(String title) {
        this.title = title;
    }

    public void setDefaultCloseOperation(int operation) {
        closeOperation = operation;
    }

    // Sizes the window to its content's preferred size
    public void pack() {
        int width = 0;
        int height = 0;
        for (TComponent component : getComponents()) {
            TDimension size = component.getPreferredSize();
            component.setSize(size.width, size.height);
            width = Math.max(width, size.width);
            height = Math.max(height, size.height);
        }
        setSize(width, height);
    }

    public void setVisible(boolean visible) {
        if (visible && !shown) {
            shown = true;
            Page.open(this);
        }
    }
}
