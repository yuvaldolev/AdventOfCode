use day5_2024_part1::PuzzleSolver;

#[test]
fn test_example_input() {
    let puzzle_solver = PuzzleSolver::new();
    let solution = puzzle_solver
        .solve(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        )
        .expect("puzzle should be solved");

    assert_eq!(solution, 143);
}
