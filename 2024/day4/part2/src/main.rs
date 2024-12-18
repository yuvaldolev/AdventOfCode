use advent_of_code::PuzzleInputDownloader;
use day4_2024_part2::PuzzleSolver;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let puzzle_input_downloader = PuzzleInputDownloader::new()?;
    let puzzle_input = puzzle_input_downloader.download(2024, 4)?;

    let puzzle_solver = PuzzleSolver::new();
    let solution = puzzle_solver.solve(&puzzle_input);

    println!("Result: {solution}");

    Ok(())
}
