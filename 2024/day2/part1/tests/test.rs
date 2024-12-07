use day2_2024_part1::PuzzleSolver;

#[test]
fn test_example_input() {
    let puzzle_solver = PuzzleSolver::new();
    let solution = puzzle_solver
        .solve(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        )
        .expect("puzzle should be solved");

    assert_eq!(solution, 2);
}
