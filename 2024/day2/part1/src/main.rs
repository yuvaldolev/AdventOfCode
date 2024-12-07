use advent_of_code::PuzzleInputDownloader;
use day2_2024_part1::PuzzleSolver;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let puzzle_input_downloader = PuzzleInputDownloader::new()?;
    let puzzle_input = puzzle_input_downloader.download(2024, 2)?;

    let puzzle_solver = PuzzleSolver::new();
    let solution = puzzle_solver.solve(&puzzle_input)?;

    println!("Number of safe reports: {solution}");

    Ok(())
}
