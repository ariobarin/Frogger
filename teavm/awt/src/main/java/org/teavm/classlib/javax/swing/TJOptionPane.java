package org.teavm.classlib.javax.swing;

import org.teavm.jso.JSBody;

// javax.swing.JOptionPane. The browser's prompt is modal too: the game waits for the answer.
public class TJOptionPane {
    public static String showInputDialog(Object message) {
        return prompt(String.valueOf(message));
    }

    @JSBody(params = "message", script = "return prompt(message);")
    private static native String prompt(String message);
}
