//! Simple helpers to parse strings into `Vector`s or `Iterator`s
use std::str::FromStr;

//------------------------------
// Vectors
//------------------------------

/// parses an `input` where each line is an entry into a `Vec`.
pub fn lines_into_vec<T: FromStr>(text: &str) -> Vec<T> {
    text.lines().flat_map(str::parse::<T>).collect()
}

/// parses an `input` into a `Vec<T>`. Entries in the string slice are separated by the `split_at` characters.
pub fn split_into_vec<T>(input: &str, split_at: &str) -> Vec<T>
where
    T: FromStr,
{
    input.split(split_at).flat_map(str::parse::<T>).collect()
}

//------------------------------
// Iterators
//------------------------------

/// returns an iterator over parsed values of an `input` string slice where the entries are separated by a new line.
pub fn lines<'a, T>(input: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr + 'a,
{
    input.lines().flat_map(str::parse::<T>)
}

/// returns an iterator over parsed values of an `input` string slice where the entries are separated by a new line.
// Disabling clippy lint because writing the function this way yields a more concise return type
#[allow(clippy::map_flatten)]
pub fn lines_cloneable<'a, T>(input: &'a str) -> impl Iterator<Item = T> + Clone + 'a
where
    T: FromStr + Clone + 'a,
{
    input.lines().map(str::parse::<T>).flatten()
}

/// returns an iterator over parsed values of an `input` string where the entries are separated by the `split_at`
/// characters.
pub fn split<'a, T>(input: &'a str, split_at: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr + 'a,
{
    input.split(split_at).flat_map(str::parse::<T>)
}
