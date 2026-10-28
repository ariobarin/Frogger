/*
 * Frogger.java
 * Ario Barin Ostovary
 * This runs the game
 */

import javax.swing.*;

public class Frogger extends JFrame {
    FroggerPanel game = new FroggerPanel();

    public Frogger() {
        super("Frogger");
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        add(game);
        // in the browser (CheerpJ, see web/index.html) the game fills the page, so no window frame
        if (System.getProperty("frogger.web") != null) {
            setUndecorated(true);
            setLocation(0, 0);
        }
        pack();
        setVisible(true);
    }

    public static void main(String[] arguments) {
        // Run the game
        new Frogger();
    }
}
