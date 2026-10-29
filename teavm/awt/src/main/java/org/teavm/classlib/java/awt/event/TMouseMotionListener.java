package org.teavm.classlib.java.awt.event;

import java.util.EventListener;

// java.awt.event.MouseMotionListener
public interface TMouseMotionListener extends EventListener {
    void mouseDragged(TMouseEvent e);

    void mouseMoved(TMouseEvent e);
}
