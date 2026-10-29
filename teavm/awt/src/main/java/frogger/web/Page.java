package frogger.web;

import java.awt.Color;
import java.awt.Component;
import java.awt.Container;
import java.awt.Graphics;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.awt.event.KeyEvent;
import java.awt.event.KeyListener;
import java.awt.event.MouseEvent;
import java.awt.event.MouseListener;
import java.awt.event.MouseMotionListener;
import java.awt.image.BufferedImage;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.IntBuffer;
import java.util.ArrayList;
import java.util.List;
import javax.swing.JFrame;
import javax.swing.Timer;
import org.teavm.jso.typedarrays.Uint8ClampedArray;

// What the window system does for the game, done on the web page (web/index.html): it shows the window,
// ticks the timers on the page's animation frames, paints, and passes on the keyboard and mouse. It
// works through the AWT and Swing API, which the classes in org/teavm/classlib provide here. The page
// reaches it through FroggerWeb's exported functions.
public final class Page {
    private static final List<Ticking> timers = new ArrayList<>();
    private static JFrame window;
    private static BufferedImage screen; // the window's picture, which Swing paints over without clearing
    private static int[] argb;
    private static ByteBuffer rgba;
    private static IntBuffer rgbaPixels;
    private static Component focus;
    private static Component pressed;
    private static boolean moved;
    private static boolean repaint;
    private static double last = Double.NaN;

    // A running timer and the time it is owed, in ms
    private static final class Ticking {
        final Timer timer;
        double clock;

        Ticking(Timer timer) {
            this.timer = timer;
        }
    }

    private Page() {
    }

    // A window opened (JFrame.setVisible). As in Swing, it starts out grey, its first focusable
    // component gets the focus, and it paints.
    public static void open(Object frame) {
        window = (JFrame) frame;
        int width = window.getWidth();
        int height = window.getHeight();
        screen = new BufferedImage(width, height, BufferedImage.TYPE_INT_RGB);
        Graphics g = screen.getGraphics();
        g.setColor(new Color(238, 238, 238));
        g.fillRect(0, 0, width, height);
        g.dispose();
        argb = new int[width * height];
        rgba = ByteBuffer.allocateDirect(width * height * 4).order(ByteOrder.LITTLE_ENDIAN);
        rgbaPixels = rgba.asIntBuffer();
        focus = firstFocusable(window);
        repaint = true;
    }

    private static Component firstFocusable(Component component) {
        if (component instanceof Container container) {
            for (Component child : container.getComponents()) {
                Component found = firstFocusable(child);
                if (found != null) {
                    return found;
                }
            }
        }
        return component.isFocusable() && !(component instanceof JFrame) ? component : null;
    }

    public static void focus(Object component) {
        focus = (Component) component;
    }

    public static void repaint() {
        repaint = true;
    }

    public static void addTimer(Object timer) {
        timers.add(new Ticking((Timer) timer));
    }

    public static void removeTimer(Object timer) {
        timers.removeIf(ticking -> ticking.timer == timer);
    }

    // Runs an event handler as Swing's event thread does: an exception is printed and the game goes on
    private static void run(Runnable handler) {
        try {
            handler.run();
        } catch (Throwable e) {
            e.printStackTrace();
        }
    }

    // An animation frame at `now` ms: the timer ticks owed, then a paint if one was asked for. True when
    // the picture changed (pixels() has it).
    public static boolean frame(double now) {
        double elapsed = Double.isNaN(last) ? 0 : now - last;
        last = now;
        for (Ticking ticking : timers.toArray(new Ticking[0])) {
            tick(ticking, elapsed);
        }
        Files.keepScores();
        if (!repaint || window == null) {
            return false;
        }
        repaint = false;
        Graphics g = screen.getGraphics();
        run(() -> window.paint(g));
        g.dispose();
        return true;
    }

    // Fires a timer's ticks owed after `elapsed` more ms. The game asks for 1000 / FPS ms, which integer
    // division makes 16 for its 60; that delay ticks 60 times a second, once per frame on a 60 Hz screen.
    // The count is rounded to the nearest tick so frame timing jitter doesn't double or skip one.
    private static void tick(Ticking ticking, double elapsed) {
        Timer timer = ticking.timer;
        double period = timer.getDelay() == 1000 / 60 ? 1000.0 / 60 : Math.max(timer.getDelay(), 1);
        // after a pause (a hidden tab, the name dialog) carry on rather than catch up
        ticking.clock = elapsed > 250 ? 0 : ticking.clock + elapsed;
        int ticks = (int) Math.max(0, Math.floor((ticking.clock + period / 4) / period));
        ticking.clock -= ticks * period;
        if (ticks > 4) {
            ticks = 4;
            ticking.clock = 0;
        }
        for (int i = 0; i < ticks && timer.isRunning(); i++) {
            ActionEvent event = new ActionEvent(timer, ActionEvent.ACTION_PERFORMED, null);
            for (ActionListener listener : timer.getActionListeners()) {
                run(() -> listener.actionPerformed(event));
            }
        }
    }

    // The picture as canvas ImageData bytes (RGBA), in memory the page can read
    public static Uint8ClampedArray pixels() {
        int width = screen.getWidth();
        screen.getRGB(0, 0, width, screen.getHeight(), argb, 0, width);
        for (int i = 0; i < argb.length; i++) {
            int p = argb[i];
            rgbaPixels.put(i, p & 0xff00ff00 | p >> 16 & 0xff | (p & 0xff) << 16);
        }
        return Uint8ClampedArray.fromJavaBuffer(rgba);
    }

    // A key went down or up (a java.awt.event.KeyEvent code); the focused component's listeners hear it
    public static void key(int code, boolean down) {
        Component target = focus;
        if (target == null) {
            return;
        }
        int id = down ? KeyEvent.KEY_PRESSED : KeyEvent.KEY_RELEASED;
        KeyEvent event = new KeyEvent(target, id, System.currentTimeMillis(), 0, code, KeyEvent.CHAR_UNDEFINED);
        for (KeyListener listener : target.getKeyListeners()) {
            run(() -> {
                if (down) {
                    listener.keyPressed(event);
                } else {
                    listener.keyReleased(event);
                }
            });
        }
    }

    // The mouse, at x, y in the window. As in Swing, a move with a button held is a drag and goes to the
    // component the press went to, as does the release, and a release with no move since the press is
    // also a click.
    public static void mouseMove(int x, int y, boolean buttonDown) {
        moved = true;
        Component target = buttonDown && pressed != null ? pressed : under(x, y);
        mouse(target, buttonDown ? MouseEvent.MOUSE_DRAGGED : MouseEvent.MOUSE_MOVED, x, y);
    }

    public static void mouseDown(int x, int y) {
        pressed = under(x, y);
        moved = false;
        mouse(pressed, MouseEvent.MOUSE_PRESSED, x, y);
    }

    public static void mouseUp(int x, int y) {
        Component target = pressed != null ? pressed : under(x, y);
        mouse(target, MouseEvent.MOUSE_RELEASED, x, y);
        if (!moved) {
            mouse(target, MouseEvent.MOUSE_CLICKED, x, y);
        }
        pressed = null;
    }

    private static Component under(int x, int y) {
        return window == null ? null : window.getComponentAt(x, y);
    }

    private static void mouse(Component target, int id, int x, int y) {
        if (target == null) {
            return;
        }
        MouseEvent event = new MouseEvent(target, id, System.currentTimeMillis(), 0, x, y, 1, false);
        for (MouseMotionListener listener : target.getMouseMotionListeners()) {
            if (id == MouseEvent.MOUSE_MOVED) {
                run(() -> listener.mouseMoved(event));
            } else if (id == MouseEvent.MOUSE_DRAGGED) {
                run(() -> listener.mouseDragged(event));
            }
        }
        for (MouseListener listener : target.getMouseListeners()) {
            if (id == MouseEvent.MOUSE_PRESSED) {
                run(() -> listener.mousePressed(event));
            } else if (id == MouseEvent.MOUSE_RELEASED) {
                run(() -> listener.mouseReleased(event));
            } else if (id == MouseEvent.MOUSE_CLICKED) {
                run(() -> listener.mouseClicked(event));
            }
        }
    }
}
