use game24::basic::{solve, solve_all};

#[test]
fn find_one() {
    println!("{}", solve(&[1, 2, 3, 4], 24).unwrap())
}

#[test]
fn find_all() {
    for v in solve_all(&[1, 2, 3, 4], 24) {
        println!("{}", v)
    }
}
