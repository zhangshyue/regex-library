/// Initialize a Bufreader from a string
/// # Examples
/// ```rust
/// use std::io::{Cursor, BufRead};
/// use crate::extraction::cursor_from_string;
/// let x = String::from("line1\nline2");
/// let mut x_reader: Cursor<&[u8]> = cursor_from_string!(x);
/// let mut string_buf = String::new();
/// x_reader.read_line(&mut string_buf);
/// assert_eq!(string_buf, String::from("line1\n"));
/// ```
#[macro_export]
macro_rules! cursor_from_string {
    ($n: expr) => {
        ::std::io::Cursor::<&[u8]>::new($n.as_bytes().into())
    };
}

/// Helper macro for splitting a string and optionally getting
/// an index from it, just syntactic sugar
///
/// # Examples
/// ```rust
/// use crate::extraction::split_string;
/// let my_string: String = String::from("Hello, world!");
///
/// let split_string = split_string!(my_string, ", ", 1);
/// assert_eq!(split_string, "world!");
/// ```
#[macro_export]
macro_rules! split_string {
    ($n: expr, $a: expr) => {
        $n.split($a).collect::<Vec<&str>>()
    };
    ($n: expr, $a: expr, $o: tt) => {
        *split_string!($n, $a).get($o).unwrap()
    };
}
