#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        println!(
            "Construct a HighScores struct, given the scores: {:?}",
            scores
        );
        HighScores {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        // Return all the scores as a slice
        println!("Return all the scores as a slice");
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        // Return the latest (last) score
        println!("Return the latest (last) score");
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        // Return the highest score
        println!("Return the highest score");
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        // Return 3 highest scores in the self.scores vector
        println!("Return 3 highest scores");
        let mut top_three = self.scores.clone();
        top_three.sort_by(|a, b| b.cmp(a));
        top_three.truncate(3);
        top_three
    }
}
