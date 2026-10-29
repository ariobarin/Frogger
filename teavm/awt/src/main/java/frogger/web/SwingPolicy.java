package frogger.web;

import java.util.function.Predicate;
import org.teavm.extension.spi.substitution.SimpleSubstitutionPolicy;
import org.teavm.extension.spi.substitution.SubstitutionSink;

// Tells TeaVM to use this project's javax.swing and javax.imageio classes (org.teavm.classlib.javax.*,
// with a T in front of each name), the way it maps java.* onto its own class library. AWT needs no rule:
// its classes here are already where TeaVM looks for java.*.
public class SwingPolicy extends SimpleSubstitutionPolicy {
    @Override
    public void contribute(SubstitutionSink sink) {
        Predicate<String> classes = inPackage("javax.swing", true).or(inPackage("javax.imageio", true));
        sink.selectClasses(classes).dontFallbackWhenNoSubstitution();
        sink.selectClasses(classes).packagePrefix("org.teavm.classlib.").simpleNamePrefix("T");
    }
}
