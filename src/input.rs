//! Utilities to read input files.
//!
//! Notes:
//! - All file helpers open files under the "inputs/" directory. Pass filenames relative
//!   to that directory (e.g. "day01.txt");
//! - Iterators returned from these helpers may hide I/O or parse errors depending on the
//!   function. See each function's doc for the exact semantics.
//! - Aside for the `lines` iterator, all other iterators use dynamic dispatch for the
//!   return type;
use std::{fs::File, io, io::prelude::*, str::FromStr};

//------------------------------
// Read whole content into memory
//------------------------------

/// Reads the whole file, parsing each line into `T` and returning a `Vec<T>`.
///
/// User must assume that newline and CRLF bytes are not maintained at the end of the
/// line.
///
/// Parsing failures are skipped in the current implementation.
pub fn lines_to_vec<T>(filename: &str) -> io::Result<Vec<T>>
where
    T: FromStr,
{
    Ok(buf_reader(filename)?
        .lines()
        .map_while(Result::ok)
        .flat_map(|s| s.parse())
        .collect())
}

/// Splits the contents of the file at `split_bit` and parses each section into `T`,
/// returning a `Vec<T>`.
///
/// Invalid UTF-8 chunks and parse failures are currently skipped.
///
/// Notes:
/// - Newline, CRLF and other special control bytes are preserved.
pub fn split_to_vec<T>(filename: &str, split_bit: u8) -> io::Result<Vec<T>>
where
    T: FromStr,
{
    Ok(buf_reader(filename)?
        .split(split_bit)
        .flatten()
        .filter(|v| !v.is_empty())
        .flat_map(String::from_utf8)
        .flat_map(|s| s.parse())
        .collect())
}

//------------------------------
// Iterators
//------------------------------

/// Returns an Iterator over the lines of a file.
///
/// The iterator yields `io::Result<String>` for each line. Each `String` produced does
/// not include the trailing newline byte(s) (LF or CRLF).
pub fn lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    Ok(buf_reader(filename)?.lines())
}

/// Returns an Iterator over lines parsed into type `T`.
///
/// Behaviour:
/// - The outer `io::Result` represents only the result of opening the file.
/// - The returned iterator currently ignores I/O errors encountered while iterating
///   (stops iteration) and also ignores parse failures (skips lines that fail
///   `T::from_str`). Consider using `lines_parsed_explicit` if you need parse error
///   information.
pub fn lines_parsed<T>(filename: &str) -> io::Result<impl Iterator<Item = T>>
where
    T: FromStr,
{
    Ok(buf_reader(filename)?
        .lines()
        .map_while(Result::ok)
        .flat_map(|s| s.parse::<T>()))
}

/// Variant of `lines_parsed` that exposes parse failures.
///
/// Returns an iterator yielding `Result<T, <T as FromStr>::Err>` so callers can also
/// handle parse errors explicitly.
pub fn lines_parsed_explicit<T>(filename: &str) -> io::Result<impl Iterator<Item = Result<T, <T as FromStr>::Err>>>
where
    T: FromStr,
{
    Ok(buf_reader(filename)?
        .lines()
        .map_while(Result::ok)
        .map(|s| s.parse::<T>()))
}

/// Returns an Iterator over substrings of a file, using `split_bit` as the delimiter. The
/// file is read as raw bytes and split on the given byte.
///
/// Behaviour:
/// - Each split chunk is converted with `String::from_utf8`. Chunks that are not valid
///   UTF-8 are currently skipped (they are filtered out).
/// - Empty chunks are filtered out.
pub fn split(filename: &str, split_bit: u8) -> io::Result<impl Iterator<Item = String>> {
    Ok(buf_reader(filename)?
        .split(split_bit)
        .flatten()
        .filter(|v| !v.is_empty())
        .flat_map(String::from_utf8))
}

/// Returns an Iterator over a file, where the content is split at `split_bit` and each
/// piece is parsed into `T`.
///
/// Behaviour:
/// - Parsing failures are currently skipped (pieces that fail `T::from_str` are ignored).
///   If you need parse errors surfaced, use `split_parsed_explicit`.
pub fn split_parsed<T>(filename: &str, split_bit: u8) -> io::Result<impl Iterator<Item = T>>
where
    T: FromStr,
{
    Ok(split(filename, split_bit)?.flat_map(|s| s.parse()))
}

/// Like `split_parsed` but yields `Result<T, <T as FromStr>::Err>` so callers can handle
/// parse failures explicitly.
pub fn split_parsed_explicit<T>(
    filename: &str,
    split_bit: u8,
) -> io::Result<impl Iterator<Item = Result<T, <T as FromStr>::Err>>>
where
    T: FromStr,
{
    Ok(split(filename, split_bit)?.map(|s| s.parse()))
}

//------------------------------
// Helpers
//------------------------------

/// Open `inputs/<filename>`.
fn open_file(filename: &str) -> Result<File, io::Error> {
    File::open("inputs/".to_string() + filename)
}

/// Returns a buffered reader for the file.
fn buf_reader(filename: &str) -> io::Result<io::BufReader<File>> {
    let file = open_file(filename)?;
    Ok(io::BufReader::new(file))
}

//------------------------------
// Tests
//------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines() {
        let mut lines = lines("test").unwrap();

        assert_eq!(
            lines.next().unwrap().unwrap(),
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed"
        );

        assert_eq!(
            lines.last().unwrap().unwrap(),
            "si aliquod aeternum et infinitum impendere malum nobis."
        );
    }

    #[test]
    fn test_split_strings() {
        let mut split = split("test", b',').unwrap();

        // first line
        assert_eq!(split.next().unwrap(), "Lorem ipsum dolor sit amet");
        // second line and keeping leading blank space
        assert_eq!(split.next().unwrap(), " consectetur adipiscing elit");
        // final line and keeping newline character
        assert_eq!(
            split.last().unwrap(),
            "\nsi aliquod aeternum et infinitum impendere malum nobis."
        );
    }

    #[test]
    fn test_lines_parsed() {
        //todo
    }

    #[test]
    fn test_split_parsed() {
        //todo
    }

    #[test]
    fn test_lines_to_vec() {
        //todo
    }

    #[test]
    fn test_split_to_vec() {
        //todo
    }
}
