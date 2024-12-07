use day2_2024_common::InputProcessor;
use itertools::Itertools;

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
                Self::is_safe_report(report.iter())
                    || (0..report.len()).any(|skip_index| {
                        Self::is_safe_report(
                            report
                                .iter()
                                .take(skip_index)
                                .chain(report.iter().skip(skip_index + 1)),
                        )
                    })
            })
            .count();

        Ok(safe_reports)
    }

    fn is_safe_report<'a, T>(report: T) -> bool
    where
        T: Iterator<Item = &'a u32> + Clone,
    {
        report
            .clone()
            .tuple_windows()
            .all(|(&x, &y)| (x < y) && (y - x <= 3))
            || report
                .tuple_windows()
                .all(|(&x, &y)| (x > y) && (x - y <= 3))
    }
}
