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

                occurrences += (-1isize..=1isize)
                    .flat_map(|step_x| (-1isize..=1isize).map(move |step_y| (step_x, step_y)))
                    .filter(|(step_x, step_y)| self.search(&puzzle, x, y, *step_x, *step_y))
                    .count();
            }
        }

        occurrences
    }

    fn search(
        &self,
        puzzle: &[Vec<char>],
        x: usize,
        y: usize,
        step_x: isize,
        step_y: isize,
    ) -> bool {
        let mut current_x = x as isize;
        let mut current_y = y as isize;

        for needle_character in self.needle.iter() {
            if (current_x < 0) || ((current_x as usize) >= puzzle[0].len()) {
                return false;
            }

            if (current_y < 0) || ((current_y as usize) >= puzzle.len()) {
                return false;
            }

            if *needle_character != puzzle[current_y as usize][current_x as usize] {
                return false;
            }

            current_x = current_x + (1 * step_x);
            current_y = current_y + (1 * step_y);
        }

        true
    }
}
