use include_glob::{include_glob_bytes, include_glob_str};

#[test]
fn read_bytes() {
    let hello_world: &[u8] = include_glob_bytes!("tests/data/hello.*.txt");

    assert_eq!(hello_world, b"hello world");
}

#[test]
fn read_str() {
    let hello_world: &str = include_glob_str!("tests/data/hello.*.txt");

    assert_eq!(hello_world, "hello world");
}

// #[test]
// #[should_panic(expected = "pattern is valid for multiple files")]
// fn read_multiple() {
//     let file: &[u8] = include_glob_bytes!("tests/data/file.*.txt");
// }
