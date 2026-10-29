package org.teavm.classlib.java.awt;

import java.util.ArrayList;
import java.util.List;

// java.awt.Container. Its components fill it, as one component in a JFrame's content pane does.
public class TContainer extends TComponent {
    private final List<TComponent> components = new ArrayList<>();

    public TComponent add(TComponent component) {
        components.add(component);
        component.parent = this;
        return component;
    }

    public TComponent[] getComponents() {
        return components.toArray(new TComponent[0]);
    }

    // The innermost component at a point (in this container's coordinates, which its components share)
    public TComponent getComponentAt(int x, int y) {
        if (x < 0 || y < 0 || x >= getWidth() || y >= getHeight()) {
            return null;
        }
        for (int i = components.size() - 1; i >= 0; i--) {
            TComponent component = components.get(i);
            if (x < component.getWidth() && y < component.getHeight()) {
                return component instanceof TContainer container ? container.getComponentAt(x, y) : component;
            }
        }
        return this;
    }

    @Override
    public void paint(TGraphics g) {
        for (TComponent component : components) {
            component.paint(g);
        }
    }
}
