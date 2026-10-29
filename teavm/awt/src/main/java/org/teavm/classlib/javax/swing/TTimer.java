package org.teavm.classlib.javax.swing;

import frogger.web.Page;
import java.util.ArrayList;
import java.util.List;
import org.teavm.classlib.java.awt.event.TActionListener;

// javax.swing.Timer. The page's animation frames tick it (frogger.web.Page.frame).
public class TTimer {
    private final List<TActionListener> listeners = new ArrayList<>();
    private int delay;
    private boolean running;

    public TTimer(int delay, TActionListener listener) {
        this.delay = delay;
        if (listener != null) {
            listeners.add(listener);
        }
    }

    public void addActionListener(TActionListener listener) {
        if (listener != null) {
            listeners.add(listener);
        }
    }

    public TActionListener[] getActionListeners() {
        return listeners.toArray(new TActionListener[0]);
    }

    public int getDelay() {
        return delay;
    }

    public void setDelay(int delay) {
        this.delay = delay;
    }

    public boolean isRunning() {
        return running;
    }

    public void start() {
        if (!running) {
            running = true;
            Page.addTimer(this);
        }
    }

    public void stop() {
        if (running) {
            running = false;
            Page.removeTimer(this);
        }
    }
}
