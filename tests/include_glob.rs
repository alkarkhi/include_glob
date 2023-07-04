use include_glob::{include_glob_bytes, include_glob_str, glob};

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

#[test]
fn glob() {
    let file_name = glob!("tests/data/hello.*.txt");

    assert_eq!(file_name, "hello.123.txt");

    let file_data = include_bytes!(concat!("data/", glob!("tests/data/hello.*.txt")));

    assert_eq!(file_data, b"hello world");
}

// #[test]
// #[should_panic(expected = "pattern is valid for multiple files")]
// fn read_multiple() {
//     let file: &[u8] = include_glob_bytes!("tests/data/file.*.txt");
// }
