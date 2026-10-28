// Score.java and ScoreManager.java. The browser keeps the scores file in the page's local storage
// (see web/index.html); its text is the same as scores.txt: a name line, then a score line.

#[derive(Clone)]
pub struct Score {
    pub name: String,
    pub score: i32,
}

#[derive(Clone)]
pub struct ScoreManager {
    high_scores: Vec<Score>,
}

impl ScoreManager {
    pub fn new(saved: &str) -> ScoreManager {
        ScoreManager { high_scores: load_scores(saved) }
    }

    pub fn get_high_scores(&self) -> &[Score] {
        &self.high_scores
    }

    pub fn get_highest_score(&self) -> i32 {
        // If the high scores are empty, return 0
        self.high_scores.first().map_or(0, |s| s.score)
    }

    pub fn add_score(&mut self, score: Score) {
        // If the high scores are empty, add the score and return
        if self.high_scores.is_empty() {
            self.high_scores.push(score);
            return;
        }

        // If the score already exists, keep the highest score
        if let Some(i) = self.high_scores.iter().position(|s| s.name == score.name) {
            if self.high_scores[i].score <= score.score {
                self.high_scores.remove(i);
                self.high_scores.push(score);
            }
        } else {
            // If the score doesn't exist, add it
            self.high_scores.push(score);
        }

        // Sort the high scores
        sort(&mut self.high_scores);
    }

    // The scores file's text, for the page to save
    pub fn save_scores(&self) -> String {
        self.high_scores.iter().map(|s| format!("{}\n{}\n", s.name, s.score)).collect()
    }
}

fn load_scores(saved: &str) -> Vec<Score> {
    let mut scores = Vec::new();
    let mut lines = saved.lines();
    while let Some(name) = lines.next() {
        let Some(points) = lines.next() else { break };
        // the Java version gives up on the rest of the file at a line that isn't a number
        let Ok(points) = points.trim().parse() else { break };
        scores.push(Score { name: name.to_string(), score: points });
    }

    // Sort the high scores
    sort(&mut scores);
    scores
}

// Collections.sort with Score.compareTo: highest first, ties keep their order
fn sort(scores: &mut [Score]) {
    scores.sort_by(|a, b| b.score.cmp(&a.score));
}
