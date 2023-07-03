use std::fs;

use glob::glob;
use proc_macro::{TokenStream, TokenTree, Punct, Spacing, Group, Delimiter, Literal};

/// Includes a file as a reference to a byte array via a glob pattern.
/// 
/// # Examples
/// 
/// Assume there is a file `file.123.txt` with contents `hello world`:
/// 
/// 
/// ```rust
/// use include_glob::include_glob_bytes;
/// 
/// static FILE: &[u8] = include_glob_bytes!("file.*.txt");
/// assert_eq!(FILE, b"hello world");
/// ```
#[proc_macro]
pub fn include_glob_bytes(input: TokenStream) -> TokenStream {
    bytes_to_token_stream(include_glob_inner(input))
}

/// Includes a UTF-8 encoded file as a string via a glob pattern.
/// 
/// # Examples
/// 
/// Assume there is a file `file.123.txt` with contents `hello world`:
/// 
/// 
/// ```rust
/// use include_glob::include_glob_str;
/// 
/// static FILE: &str = include_glob_str!("file.*.txt");
/// assert_eq!(FILE, "hello world");
/// ```
#[proc_macro]
pub fn include_glob_str(input: TokenStream) -> TokenStream {
    str_to_token_stream(include_glob_inner(input))
}

fn include_glob_inner(input: TokenStream) -> Vec<u8> {
    let tokens = input.into_iter().collect::<Vec<TokenTree>>();

    match tokens.as_slice() {
        [TokenTree::Literal(lit)] => {
            let pattern = lit.to_string();

            if !pattern.starts_with('"') || !pattern.ends_with('"') {
                panic!("this macro only accepts a string argument")
            }

            let pattern: &str = &pattern[1..(pattern.len() - 1)];

            let mut files = match glob(pattern) {
                Ok(files) => files,
                Err(e) => panic!("invalid glob pattern: {}", e),
            };

            let path = match files.next() {
                Some(file) => {
                    match file {
                        Ok(file) => file,
                        Err(e) => panic!("couldn't read {path}: {e}", path = e.path().display()),
                    }
                },
                None => panic!("no file found that matches pattern {pattern}"),
            };

            // only one file should match the pattern so builds are deterministic
            if files.next().is_some() {
                panic!("pattern is valid for multiple files");
            }

            let bytes: Vec<u8> = match fs::read(&path) {
                Ok(bytes) => bytes,
                Err(e) => panic!("couldn't read {path}: {e}", path = path.display()),
            };

            bytes
        },
        _ => panic!("input needs to be a string"),
    }
}

fn bytes_to_token_stream(bytes: Vec<u8>) -> TokenStream {
    let mut bytes_ts: Vec<TokenTree> = Vec::with_capacity(bytes.len() * 2);

    for byte in bytes {
        bytes_ts.push(TokenTree::Literal(Literal::u8_unsuffixed(byte)));
        bytes_ts.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
    }

    let res: [TokenTree; 2] = [
        TokenTree::Punct(Punct::new('&', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Bracket, TokenStream::from_iter(bytes_ts))),
    ];

    TokenStream::from_iter(res)
}

fn str_to_token_stream(bytes: Vec<u8>) -> TokenStream {
    let string: String = match String::from_utf8(bytes) {
        Ok(string) => string,
        Err(e) => panic!("file is not valid utf8: {e}"),
    };

    TokenStream::from(TokenTree::Literal(Literal::string(&string)))
}