# include_glob!

embed files into an executable via glob patterns

## Features
* zero cost - behaves exactly the same as the native `include_bytes!` and `include_str!` macros behind the scenes.
* very simple - less than 100 lines of code.
* minimal dependencies - despite being a procedural macro, it only depends on `glob`

## Usage

In the command line run:

```
cargo add include_glob
```

## Examples

Assume there is a file `file.123.txt` with contents `hello world`:

```
use include_glob::include_glob_bytes;

static FILE: &[u8] = include_glob_bytes!("file.*.txt");
assert_eq!(FILE, b"hello world");
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
