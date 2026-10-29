package org.teavm.classlib.java.awt.event;

import java.util.EventObject;

// java.awt.event.ActionEvent
public class TActionEvent extends EventObject {
    public static final int ACTION_PERFORMED = 1001;

    private final int id;
    private final String command;

    public TActionEvent(Object source, int id, String command) {
        super(source);
        this.id = id;
        this.command = command;
    }

    public int getID() {
        return id;
    }

    public String getActionCommand() {
        return command;
    }
}
