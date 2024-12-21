use day6_2024_part1::PuzzleSolver;

#[test]
fn test_example_input() {
    let puzzle_solver = PuzzleSolver::new();
    let solution = puzzle_solver
        .solve(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        )
        .expect("puzzle should be solved");

    assert_eq!(solution, 41);
}
