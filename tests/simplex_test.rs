use rustimizer::*;

#[test]
fn test_simple_maximization_problem() {
    // 最大化問題:
    // maximize: 3x1 + 2x2
    // subject to:
    //   x1 + x2 <= 4
    //   2x1 + x2 <= 5
    //   x1, x2 >= 0

    let c = vec![3.0, 2.0]; // 目的関数の係数
    let a = vec![vec![1.0, 1.0], vec![2.0, 1.0]]; // 制約行列
    let b = vec![4.0, 5.0]; // 制約の右辺

    let problem = LinearProgram::new(c, a, b, OptimizationType::Maximize);
    let solver = SimplexSolver::new();
    let result = solver.solve(&problem).unwrap();

    // 最適解は x1 = 1, x2 = 3 で目的関数値は 9
    assert!((result.objective_value - 9.0).abs() < 1e-6);
    assert!((result.solution[0] - 1.0).abs() < 1e-6);
    assert!((result.solution[1] - 3.0).abs() < 1e-6);
}

#[test]
fn test_simple_minimization_problem_positive_rhs() {
    // まず正の右辺値の簡単な最小化問題をテスト
    // minimize: x1 + x2
    // subject to:
    //   x1 + x2 <= 10
    //   x1, x2 >= 0

    let c = vec![1.0, 1.0];
    let a = vec![vec![1.0, 1.0]];
    let b = vec![10.0];

    let problem = LinearProgram::new(c, a, b, OptimizationType::Minimize);
    let solver = SimplexSolver::new();
    let result = solver.solve(&problem).unwrap();

    // 最適解は x1 = 0, x2 = 0 で目的関数値は 0
    assert!((result.objective_value - 0.0).abs() < 1e-6);
    assert!((result.solution[0] - 0.0).abs() < 1e-6);
    assert!((result.solution[1] - 0.0).abs() < 1e-6);
}

#[test]
fn test_simple_minimization_problem() {
    // 最小化問題を簡略化してテスト
    // minimize: x1 + x2
    // subject to:
    //   x1 + x2 >= 2  => -x1 - x2 <= -2
    //   x1, x2 >= 0

    let c = vec![1.0, 1.0]; // 目的関数の係数
    let a = vec![vec![-1.0, -1.0]]; // 制約行列（>= を <= に変換）
    let b = vec![-2.0]; // 制約の右辺

    let problem = LinearProgram::new(c, a, b, OptimizationType::Minimize);
    let solver = SimplexSolver::new();
    let result = solver.solve(&problem);

    match result {
        Ok(res) => {
            println!(
                "負の右辺値の最小化問題 - Objective: {}, Solution: {:?}",
                res.objective_value, res.solution
            );
            // 最適解は x1 + x2 = 2 を満たす任意の非負の点
            // 例: x1 = 0, x2 = 2 または x1 = 1, x2 = 1
            assert!((res.objective_value - 2.0).abs() < 1e-6);
            assert!((res.solution[0] + res.solution[1] - 2.0).abs() < 1e-6);
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_unbounded_problem() {
    // 非有界問題:
    // maximize: x1 + x2
    // subject to:
    //   -x1 + x2 <= 1
    //   x1, x2 >= 0

    let c = vec![1.0, 1.0];
    let a = vec![vec![-1.0, 1.0]];
    let b = vec![1.0];

    let problem = LinearProgram::new(c, a, b, OptimizationType::Maximize);
    let solver = SimplexSolver::new();
    let result = solver.solve(&problem);

    assert!(matches!(result, Err(SimplexError::Unbounded)));
}

#[test]
fn test_infeasible_problem() {
    // 実行不可能問題:
    // maximize: x1 + x2
    // subject to:
    //   x1 + x2 <= 1
    //   x1 + x2 >= 2
    //   x1, x2 >= 0

    let c = vec![1.0, 1.0];
    let a = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
    let b = vec![1.0, 2.0];
    let constraint_types = vec![ConstraintType::LessOrEqual, ConstraintType::GreaterOrEqual];

    let problem =
        LinearProgram::new_with_constraints(c, a, b, constraint_types, OptimizationType::Maximize);
    let solver = SimplexSolver::new();
    let result = solver.solve(&problem);

    assert!(matches!(result, Err(SimplexError::Infeasible)));
}

#[test]
fn test_greater_or_equal_constraint() {
    // 2段階法が必要な >= 制約を含む問題
    // minimize: x1 + x2
    // subject to:
    //   x1 + x2 >= 2
    //   x1, x2 >= 0

    let c = vec![1.0, 1.0];
    let a = vec![vec![1.0, 1.0]];
    let b = vec![2.0];
    let constraint_types = vec![ConstraintType::GreaterOrEqual];

    let problem =
        LinearProgram::new_with_constraints(c, a, b, constraint_types, OptimizationType::Minimize);
    let solver = SimplexSolver::new();
    let result = solver.solve(&problem);

    match result {
        Ok(res) => {
            println!(
                ">=制約テスト - Objective: {}, Solution: {:?}",
                res.objective_value, res.solution
            );
            // 最適解は x1 + x2 = 2 を満たす任意の非負の点
            // 例: x1 = 0, x2 = 2 または x1 = 2, x2 = 0
            assert!((res.objective_value - 2.0).abs() < 1e-6);
            assert!((res.solution[0] + res.solution[1] - 2.0).abs() < 1e-6);
        }
        Err(e) => panic!(">=制約テストでエラー: {:?}", e),
    }
}

#[test]
fn test_equality_constraint() {
    // 等式制約を含む問題
    // minimize: x1 + 2*x2
    // subject to:
    //   x1 + x2 = 3
    //   x1, x2 >= 0

    let c = vec![1.0, 2.0];
    let a = vec![vec![1.0, 1.0]];
    let b = vec![3.0];
    let constraint_types = vec![ConstraintType::Equal];

    let problem =
        LinearProgram::new_with_constraints(c, a, b, constraint_types, OptimizationType::Minimize);
    let solver = SimplexSolver::new();
    let result = solver.solve(&problem);

    match result {
        Ok(res) => {
            println!(
                "等式制約テスト - Objective: {}, Solution: {:?}",
                res.objective_value, res.solution
            );
            // 最適解は x1 = 3, x2 = 0 で目的関数値は 3
            assert!((res.objective_value - 3.0).abs() < 1e-6);
            assert!((res.solution[0] - 3.0).abs() < 1e-6);
            assert!((res.solution[1] - 0.0).abs() < 1e-6);
        }
        Err(e) => panic!("等式制約テストでエラー: {:?}", e),
    }
}

#[test]
fn test_mixed_constraints() {
    // 混合制約を含む問題
    // maximize: 2*x1 + x2
    // subject to:
    //   x1 + x2 <= 4     (<=制約)
    //   x1 - x2 >= 0     (>=制約)
    //   x1 + 2*x2 = 6    (=制約)
    //   x1, x2 >= 0

    let c = vec![2.0, 1.0];
    let a = vec![vec![1.0, 1.0], vec![1.0, -1.0], vec![1.0, 2.0]];
    let b = vec![4.0, 0.0, 6.0];
    let constraint_types = vec![
        ConstraintType::LessOrEqual,
        ConstraintType::GreaterOrEqual,
        ConstraintType::Equal,
    ];

    let problem =
        LinearProgram::new_with_constraints(c, a, b, constraint_types, OptimizationType::Maximize);
    let solver = SimplexSolver::new();
    let result = solver.solve(&problem).unwrap();

    // 等式制約 x1 + 2*x2 = 6 と x1 - x2 >= 0 から
    // x1 = 6 - 2*x2, x1 >= x2 なので 6 - 2*x2 >= x2 -> x2 <= 2
    // x1 + x2 <= 4 から (6 - 2*x2) + x2 <= 4 -> x2 >= 2
    // よって x2 = 2, x1 = 2 で目的関数値は 6
    assert!((result.objective_value - 6.0).abs() < 1e-6);
    assert!((result.solution[0] - 2.0).abs() < 1e-6);
    assert!((result.solution[1] - 2.0).abs() < 1e-6);
}
