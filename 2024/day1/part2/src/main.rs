use advent_of_code::PuzzleInputDownloader;
use day1_2024_part2::PuzzleSolver;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let puzzle_input_downloader = PuzzleInputDownloader::new()?;
    let puzzle_input = puzzle_input_downloader.download(2024, 1)?;

    let puzzle_solver = PuzzleSolver::new();
    let solution = puzzle_solver.solve(&puzzle_input)?;

    println!("Similarity score: {solution}");

    Ok(())
}
