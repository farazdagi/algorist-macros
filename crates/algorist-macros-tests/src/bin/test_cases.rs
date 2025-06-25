use algorist_macros::test_cases;

#[test_cases]
pub fn multiple_test_cases() {
    let (a, b): (i32, i32) = scan.pair();
    writeln!(w, "Sum: {}", a + b);
}

pub fn main() {
    multiple_test_cases();
}
