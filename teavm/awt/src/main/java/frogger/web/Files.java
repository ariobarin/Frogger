package frogger.web;

import java.io.ByteArrayInputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.io.UncheckedIOException;
import java.nio.charset.StandardCharsets;
import java.util.zip.ZipEntry;
import java.util.zip.ZipInputStream;
import org.teavm.jso.JSBody;
import org.teavm.jso.typedarrays.Int8Array;

// The game's files, in TeaVM's in-memory file system: its folder of PNGs, which the page fetches as a
// zip, and the scores file, which the page keeps in its local storage
public final class Files {
    private static final String SCORES = "scores.txt"; // ScoreManager's file
    private static String savedScores;

    private Files() {
    }

    // Before the game starts: the files go where the game reads them, as on the desktop
    public static void mount() {
        try (ZipInputStream zip = new ZipInputStream(new ByteArrayInputStream(assets().copyToJavaArray()))) {
            for (ZipEntry entry; (entry = zip.getNextEntry()) != null; ) {
                File file = new File(entry.getName());
                if (entry.isDirectory()) {
                    file.mkdirs();
                    continue;
                }
                file.getParentFile().mkdirs();
                try (OutputStream out = new FileOutputStream(file)) {
                    zip.transferTo(out);
                }
            }
            savedScores = loadScores();
            if (savedScores != null) {
                try (OutputStream out = new FileOutputStream(SCORES)) {
                    out.write(savedScores.getBytes(StandardCharsets.UTF_8));
                }
            }
        } catch (IOException e) {
            throw new UncheckedIOException(e);
        }
    }

    // A file, whatever the case of its name, as Windows (where Frogger was made) finds it: the game asks
    // for Background/Intro.png, which is intro.png. Null if there's none.
    public static File find(File file) {
        if (file.exists()) {
            return file;
        }
        File parent = file.getParentFile();
        File folder = parent == null ? null : find(parent);
        File[] entries = folder == null ? null : folder.listFiles();
        if (entries != null) {
            for (File entry : entries) {
                if (entry.getName().equalsIgnoreCase(file.getName())) {
                    return entry;
                }
            }
        }
        return null;
    }

    // The game saves its scores after the name dialog; the page keeps a copy for the next visit
    public static void keepScores() {
        File file = new File(SCORES);
        if (!file.exists()) {
            return;
        }
        try (InputStream in = new FileInputStream(file)) {
            String scores = new String(in.readAllBytes(), StandardCharsets.UTF_8);
            if (!scores.equals(savedScores)) {
                savedScores = scores;
                storeScores(scores);
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    @JSBody(script = "return froggerPage.assets;")
    private static native Int8Array assets();

    @JSBody(script = "return froggerPage.loadScores();")
    private static native String loadScores();

    @JSBody(params = "text", script = "froggerPage.saveScores(text);")
    private static native void storeScores(String text);
}
