/*
 * ScoreManager.java
 * Ario Barin Ostovary
 * Manages loading, saving, and updating high scores.
 */

import java.io.*;
import java.util.*;

public class ScoreManager {
    private ArrayList<Score> highScores;
    // the browser build keeps scores in its own writable storage (see web/index.html)
    private final String scoreFile = System.getProperty("frogger.scores", "scores.txt");

    public ScoreManager() {
        highScores = loadScores();
    }

    public ArrayList<Score> getHighScores() {
        return highScores;
    }

    public int getHighestScore() {
        // If the high scores are empty, return 0
        if (highScores.isEmpty()) {
            return 0;
        }
        return highScores.get(0).getScore();
    }

    public void addScore(Score score) {
        // If the high scores are empty, add the score and return
        if (highScores.isEmpty()) {
            highScores.add(score);
            return;
        }
        
        // Check if the score already exists
        Score existingScore = null;
        for (Score sc : highScores) {
            if (sc.getName().equals(score.getName())) {
                existingScore = sc;
                break;
            }
        }
        
        // If the score already exists, keep the highest score
        if (existingScore != null) {
            if (existingScore.getScore() <= score.getScore()) {
                highScores.remove(existingScore);
                highScores.add(score);
            }
        } else {
            // If the score doesn't exist, add it
            highScores.add(score);
        }
        
        // Sort the high scores
        Collections.sort(highScores);
    }

    public void saveScores() {
        try {
            PrintWriter out = new PrintWriter(new File(scoreFile));
            for (Score sc : highScores) {
                out.println(sc.twoLine());
            }
            out.close();
        } catch (Exception e) {
        }
    }

    private ArrayList<Score> loadScores() {
        ArrayList<Score> scores = new ArrayList<>();
        try {
            File file = new File(scoreFile);
            if (file.exists()) {
                // Read the scores from the file
                Scanner input = new Scanner(file);
                while (input.hasNextLine()) {
                    String name = input.nextLine();
                    if (input.hasNextLine()) {
                        int points = Integer.parseInt(input.nextLine());
                        scores.add(new Score(name, points));
                    }
                }
                input.close();
            }
        } catch (Exception e) {
            System.out.println("bruh cant load scores");
        }

        // Sort the high scores
        Collections.sort(scores);

        return scores;
    }
}