package parity;

import org.teavm.extension.spi.reflection.SimpleReflectionPolicy;

// The two fields FroggerFramesWeb reaches by reflection, as FroggerFrames does: the game's random
// numbers (to seed them) and its lives (scripts give the bot more)
public class Reflection extends SimpleReflectionPolicy {
    @Override
    protected void setup() {
        selectClass("Util").reflectableFields(named("RANDOM"));
        selectClass("FroggerPanel").reflectableFields(named("lifeCounter"));
    }
}
