/*
 * FroggerWeb.java
 * Frogger in a web page: TeaVM compiles this, the game in ../src and the AWT and Swing stand-ins in
 * org/teavm/classlib to WebAssembly (see pom.xml). It puts the game's files where it looks for them,
 * then starts it as the desktop does. The page (web/index.html) drives it through the exported functions.
 */

import frogger.web.Files;
import frogger.web.Page;
import org.teavm.jso.JSExport;
import org.teavm.jso.typedarrays.Uint8ClampedArray;

public class FroggerWeb {
    public static void main(String[] args) {
        Files.mount();
        Frogger.main(args);
    }

    @JSExport
    public static boolean frame(double now) {
        return Page.frame(now);
    }

    @JSExport
    public static Uint8ClampedArray pixels() {
        return Page.pixels();
    }

    @JSExport
    public static void key(int code, boolean down) {
        Page.key(code, down);
    }

    @JSExport
    public static void mouseMove(int x, int y, boolean buttonDown) {
        Page.mouseMove(x, y, buttonDown);
    }

    @JSExport
    public static void mouseDown(int x, int y) {
        Page.mouseDown(x, y);
    }

    @JSExport
    public static void mouseUp(int x, int y) {
        Page.mouseUp(x, y);
    }
}
