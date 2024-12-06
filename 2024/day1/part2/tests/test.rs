use day1_part2_2024::PuzzleSolver;

#[test]
fn test_example_input() {
    let puzzle_solver = PuzzleSolver::new();
    let solution = puzzle_solver
        .solve(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        )
        .expect("puzzle should be solved");

    assert_eq!(solution, 31);
}
