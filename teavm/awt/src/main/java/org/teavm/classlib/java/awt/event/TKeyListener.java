package org.teavm.classlib.java.awt.event;

import java.util.EventListener;

// java.awt.event.KeyListener
public interface TKeyListener extends EventListener {
    void keyTyped(TKeyEvent e);

    void keyPressed(TKeyEvent e);

    void keyReleased(TKeyEvent e);
}
