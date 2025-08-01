# rustimizer

Rustで実装された線形計画問題の最適化ソルバーライブラリです。シンプレックス法と二段階法を使用して、様々な制約条件を持つ線形計画問題を解くことができます。

## 特徴

- **シンプレックス法**による線形計画問題の求解
- **二段階法**による複雑な制約条件への対応
- **混合制約**のサポート（<=、>=、= 制約）
- **負の右辺値**を持つ制約の自動処理
- **実行不可能性**と**非有界性**の自動検出
- **最大化**と**最小化**両方の目的関数に対応

## インストール

`Cargo.toml`に以下を追加してください：

```toml
[dependencies]
rustimizer = "0.1.0"
```

## 使用方法

### 基本的な使い方

```rust
use rustimizer::*;

// 最大化問題の例:
// maximize: 3x1 + 2x2
// subject to:
//   x1 + x2 <= 4
//   2x1 + x2 <= 5
//   x1, x2 >= 0

let c = vec![3.0, 2.0]; // 目的関数の係数
let a = vec![
    vec![1.0, 1.0],     // 第1制約の係数
    vec![2.0, 1.0],     // 第2制約の係数
];
let b = vec![4.0, 5.0]; // 制約の右辺

let problem = LinearProgram::new(c, a, b, OptimizationType::Maximize);
let solver = SimplexSolver::new();
let result = solver.solve(&problem).unwrap();

println!("最適解: {:?}", result.solution);      // [1.0, 3.0]
println!("目的関数値: {}", result.objective_value); // 9.0
```

### 制約タイプを指定した問題

```rust
use rustimizer::*;

// 混合制約の例:
// maximize: 2x1 + x2
// subject to:
//   x1 + x2 <= 4     (<=制約)
//   x1 - x2 >= 0     (>=制約)
//   x1 + 2x2 = 6     (=制約)
//   x1, x2 >= 0

let c = vec![2.0, 1.0];
let a = vec![
    vec![1.0, 1.0],   // x1 + x2 <= 4
    vec![1.0, -1.0],  // x1 - x2 >= 0
    vec![1.0, 2.0],   // x1 + 2x2 = 6
];
let b = vec![4.0, 0.0, 6.0];
let constraint_types = vec![
    ConstraintType::LessOrEqual,
    ConstraintType::GreaterOrEqual,
    ConstraintType::Equal,
];

let problem = LinearProgram::new_with_constraints(
    c, a, b, constraint_types, OptimizationType::Maximize
);
let solver = SimplexSolver::new();
let result = solver.solve(&problem).unwrap();

println!("最適解: {:?}", result.solution);
println!("目的関数値: {}", result.objective_value);
```

### エラーハンドリング

```rust
use rustimizer::*;

let solver = SimplexSolver::new();
match solver.solve(&problem) {
    Ok(result) => {
        println!("最適解が見つかりました:");
        println!("解: {:?}", result.solution);
        println!("目的関数値: {}", result.objective_value);
    }
    Err(SimplexError::Infeasible) => {
        println!("この問題は実行不可能です");
    }
    Err(SimplexError::Unbounded) => {
        println!("この問題は非有界です");
    }
}
```

## API リファレンス

### `LinearProgram`

線形計画問題を表現する構造体です。

#### コンストラクタ

- `new(c, a, b, optimization_type)` - すべて <= 制約として扱う
- `new_with_constraints(c, a, b, constraint_types, optimization_type)` - 制約タイプを指定

#### パラメータ

- `c: Vec<f64>` - 目的関数の係数ベクトル
- `a: Vec<Vec<f64>>` - 制約行列（各行が1つの制約）
- `b: Vec<f64>` - 制約の右辺値ベクトル
- `constraint_types: Vec<ConstraintType>` - 各制約のタイプ
- `optimization_type: OptimizationType` - 最適化の種類（最大化/最小化）

### `ConstraintType`

制約の種類を表す列挙型です。

- `LessOrEqual` - <= 制約
- `GreaterOrEqual` - >= 制約  
- `Equal` - = 制約

### `OptimizationType`

最適化の種類を表す列挙型です。

- `Maximize` - 最大化問題
- `Minimize` - 最小化問題

### `SimplexSolver`

シンプレックス法を実装したソルバーです。

#### メソッド

- `new()` - 新しいソルバーインスタンスを作成
- `solve(&self, problem: &LinearProgram)` - 線形計画問題を解く

### `SimplexResult`

最適化の結果を表す構造体です。

- `objective_value: f64` - 最適な目的関数値
- `solution: Vec<f64>` - 最適解のベクトル

### `SimplexError`

エラーの種類を表す列挙型です。

- `Infeasible` - 実行不可能（解が存在しない）
- `Unbounded` - 非有界（目的関数値が無限に改善される）

## 実装されているアルゴリズム

1. **シンプレックス法** - 基本的な線形計画問題の求解
2. **二段階法** - 複雑な制約条件（>=、= 制約）への対応
3. **ピボット操作** - タブローの更新
4. **最小比率テスト** - 非有界性の検出
5. **実行可能性判定** - Phase1での人工変数による判定

## 制限事項

- 変数は非負制約（x >= 0）が自動的に適用されます
- 大規模問題に対するパフォーマンス最適化は未実装
- 数値的安定性の改善余地があります

## テスト

```bash
cargo test
```

全てのテストケースが含まれており、様々な問題パターンで動作を確認できます。

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。