pub fn run_tests() {
    println!("Tests running");
}

#[test]
fn add_two() {
    assert_eq!(super::add_two(3), 5);
}
