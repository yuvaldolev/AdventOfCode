use day4_2024_common::InputParser;

pub struct PuzzleSolver {
    input_parser: InputParser,
    needle: Vec<char>,
}

impl PuzzleSolver {
    pub fn new() -> Self {
        Self {
            input_parser: InputParser::new(),
            needle: vec!['X', 'M', 'A', 'S'],
        }
    }

    pub fn solve(&self, input: &str) -> usize {
        let puzzle = self.input_parser.parse(input);

        let mut occurrences: usize = 0;

        for (y, line) in puzzle.iter().enumerate() {
            for (x, character) in line.iter().enumerate() {
                if self.needle[0] != *character {
                    continue;
                }

                if self.search_horizontal(&puzzle, x, y, 1) {
                    occurrences += 1;
                }

                // search_horizontal(puzzle, x, y, -1);
                // search_vertical(puzzle, x, y, 1);
                // search_vertical(puzzle, x, y, -1);
                // search_diagonal(puzzle, x, y, 1);
                // search_diagonal(puzzle, x, y, -1);
            }
        }

        occurrences
    }

    fn search_horizontal(&self, puzzle: &[Vec<char>], x: usize, y: usize, step: isize) -> bool {
        if ((x as isize) + (step * (self.needle.len() as isize))) as usize > puzzle[0].len() {
            return false;
        }

        true
    }
}
