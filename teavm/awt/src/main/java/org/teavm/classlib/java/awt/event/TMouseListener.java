package org.teavm.classlib.java.awt.event;

import java.util.EventListener;

// java.awt.event.MouseListener
public interface TMouseListener extends EventListener {
    void mouseClicked(TMouseEvent e);

    void mousePressed(TMouseEvent e);

    void mouseReleased(TMouseEvent e);

    void mouseEntered(TMouseEvent e);

    void mouseExited(TMouseEvent e);
}
