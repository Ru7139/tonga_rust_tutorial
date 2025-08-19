#[test]
fn test_hello_world() {
    println!("Hello world")
}

#[test]
#[rustfmt::skip]
fn test_func_in_func() {
    test_hello_world();
    fn another_func() { println!("this is another func"); }
    another_func();
}
