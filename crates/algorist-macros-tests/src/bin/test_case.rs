use algorist_macros::test_case;

#[test_case]
pub fn single_test_case() {
    let (a, b): (i32, i32) = scan.pair();
    writeln!(w, "Sum: {}", a + b);
}

pub fn main() {
    single_test_case();
}
