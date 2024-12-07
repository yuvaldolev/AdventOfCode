use day2_2024_common::InputProcessor;

pub struct PuzzleSolver {
    input_processor: InputProcessor,
}

impl PuzzleSolver {
    pub fn new() -> Self {
        Self {
            input_processor: InputProcessor::new(),
        }
    }

    pub fn solve(&self, input: &str) -> day2_2024_common::Result<usize> {
        let reports = self.input_processor.process(input)?;
        let safe_reports = reports
            .iter()
            .filter(|&report| {
                (report
                    .windows(2)
                    .filter(|&levels| (levels[0] <= levels[1]) || (levels[0] - levels[1] > 3))
                    .count()
                    == 0)
                    || (report
                        .windows(2)
                        .filter(|&levels| (levels[0] >= levels[1]) || (levels[1] - levels[0] > 3))
                        .count()
                        == 0)
            })
            .count();

        Ok(safe_reports)
    }
}
