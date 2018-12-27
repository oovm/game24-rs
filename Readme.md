# Game24 Solver

## Basic

```rust
use game24::basic::{solve, solve_all};

#[test]
fn find_one() {
    println!("{}", solve(&[1, 2, 3, 4], 24).unwrap())
}

// (1 + 2 + 3) × 4

#[test]
fn find_all() {
    for v in solve_all(&[1, 2, 3, 4], 24) {
        println!("{}", v)
    }
}

// (1 + 2 + 3) × 4
// (1 + 3) × (2 + 4)
// 1 × 2 × 3 × 4
// 2 ÷ 1 × 3 × 4
```


## Todo list

- Support unary op like `!`
- Time Constraint
- Search all solutions for a given deck
- Cli/Web application
- Meta programming instead of big function map
