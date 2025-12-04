//! A two-dimensional array using a flat internal representation.
//!
//! This is a row major implementation, consecutive elements across the x
//! dimension are next to each other, whereas columns are strided.
//!
//! `x` represents variation in row elements (which column the value is in),
//! whereas `y` represents a change in column elements (which row is it in). The
//! grid may be indexed with a tuple using `get_from2d((x,y))` , for example:
//!
//! - get_from2d((5, 0)) returns the sixth element of the first row. It can also be
//!   interpreted as the the element at column 5 and row 0.
//!
//! - get_from2d((1, 5)) returns the second element of the sixth row. In other words, the
//!   element at column 1 and row 5.
//!
//! # Indexing
//!
//! Implements the Index trait, so the grid may be read by a tuple inside
//! square brackets. Example:
//!
//! ```
//! use aoc_tools::grid::Grid;
//! let mut grid = Grid::new(5, 5, 0u8);
//! let v = grid.get_mut(2, 2);
//! *v = 100;
//! assert_eq!(grid[(2,2)], 100);
//! ```
//!
//! ## Beware
//!
//! If no inferring is made, the Default type for tuples in rust is i32.
//!
//! # Panics
//!
//! Panics if the indexing inside square brackets is done with negative values.

// use std::{convert::TryInto, fmt::Debug, ops::Index};

// use super::base2d::Base2d;

use crate::pair::Pair;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    flat: Vec<T>,
    pub len_x: usize,
    pub len_y: usize,
}

impl<T: Clone> Grid<T> {
    /// creates a new grid with all the elements having the `init`ial value
    pub fn new(len_x: usize, len_y: usize, init: T) -> Grid<T> {
        Grid {
            flat: vec![init; len_x * len_y],
            len_x,
            len_y,
        }
    }

    /// If vector `v` is larger than `len_x` * `len_y`, the extra elements are truncated.
    ///
    /// # Panics
    ///
    /// - The input vector `v` must have at least `len_x` * `len_y` length. Otherwise the
    ///   program may panic while trying to access the elements of the inner vector;
    pub fn from_vec(len_x: usize, len_y: usize, mut v: Vec<T>) -> Grid<T> {
        debug_assert!(v.len() >= len_x * len_y);
        v.truncate(len_x * len_y);

        Grid { flat: v, len_x, len_y }
    }

    /// The input iterator `iter` must have at least as many elements as `len_x` *
    /// `len_y`, otherwise the function panics. If it has more elements, the remaining are
    /// ignored.
    ///
    /// # Panics
    ///
    /// - If `iter` does not have enough elements to fill the grid.
    pub fn from_iter<I>(len_x: usize, len_y: usize, iter: I) -> Grid<T>
    where
        I: IntoIterator<Item = T>,
    {
        let size = len_x * len_y;
        let flat = iter.into_iter().take(size).collect::<Vec<T>>();

        debug_assert!(flat.len() == size);
        Grid::from_vec(len_x, len_y, flat)
    }

    /// Creates a new grid from a iterator and adds a border to it. Also receives as
    /// parameters, the neutral element to be used as a `border` and the border
    /// `thickness` as rows or columns.
    ///
    /// Iterator must be finite, if it does not have enough items to complete the last
    /// line, the remaining elements will be completed with the border value.
    ///
    /// # Assumes
    ///
    /// - Iterator is not infinite.
    pub fn with_borders<I>(len_x: usize, border: T, thickness: usize, iter: I) -> Grid<T>
    where
        I: IntoIterator<Item = T>,
        T: Copy,
    {
        let mut iter = iter.into_iter().peekable();
        let mut flat = Vec::<T>::new();
        let len = len_x + 2;
        flat.extend(vec![border; len * thickness]); // upper border
        while iter.peek().is_some() {
            (0..thickness).for_each(|_| flat.push(border)); // left border
            flat.extend(iter.by_ref().take(len_x));
            (0..thickness).for_each(|_| flat.push(border)); // right border
        }
        flat.extend(vec![border; (1 + thickness) * len - flat.len() % len]); // complete last line and add lower border

        Grid::from_vec(len, flat.len() / len, flat)
    }
}

impl<T> Grid<T> {
    //------------------------------
    // Getters for single elements
    //------------------------------

    /// returns the value at position x,y.
    ///
    /// # Panics
    ///
    /// Panics if either index is out of bounds.
    pub fn get(&self, x: usize, y: usize) -> &T {
        // &self.flat[self.index(x, y)]
        self.flat.get(self.index(x, y)).unwrap()
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let i = self.index(x, y); // must have an aux variable coz mutable borrow
        &mut self.flat[i]
    }

    /// returns the value at position `idx` from the backend `Vec`
    /// If using the (x, y) coordinates, it is the same element as (idx % self.len_x, idx
    /// / self.len_y)
    ///
    /// # Panics
    ///
    /// Panics if either index is out of bounds.
    pub fn get_flat(&self, idx: usize) -> &T {
        &self.flat[idx]
    }

    pub fn get_flat_mut(&mut self, idx: usize) -> &mut T {
        &mut self.flat[idx]
    }

    //------------------------------
    // Getters for multiple elements
    //------------------------------

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.flat.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.flat.iter_mut()
    }

    // returns an array slice for a line of the grid
    pub fn row(&self, y: usize) -> &[T] {
        &self.flat[self.index(0, y)..=self.index(self.len_x - 1, y)]
    }

    pub fn row_mut(&mut self, y: usize) -> &mut [T] {
        let idx0 = self.index(0, y);
        let idx1 = self.index(self.len_x - 1, y);
        &mut self.flat[idx0..=idx1]
    }

    pub fn iter_col(&self, x: usize) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator {
        self.flat.iter().skip(x).step_by(self.len_x)
    }

    pub fn iter_col_mut(&mut self, x: usize) -> impl DoubleEndedIterator<Item = &mut T> + ExactSizeIterator {
        self.flat.iter_mut().skip(x).step_by(self.len_x)
    }

    //------------------------------
    // Helpers
    //------------------------------

    /// returns the total size of the array (len_x * len_y)
    pub fn size(&self) -> usize {
        self.flat.len()
    }

    //------------------------------
    // Private
    //------------------------------

    /// returns the index for acessing the `flat` array from the coordinates `x`
    /// and `y`.
    fn index(&self, x: usize, y: usize) -> usize {
        self.len_x * y + x
    }

    pub fn xy_index(&self, idx: usize) -> (usize, usize) {
        (idx % self.len_x, idx / self.len_x)
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, item: &T) -> Option<(usize, usize)> {
        let (idx, _) = self.iter().enumerate().find(|&(_, x)| x == item)?;
        Some(self.xy_index(idx))
    }
}

//------------------------------
// Indexing
//------------------------------

/// Uses a tuple for indexing.
impl<T, V> Index<(V, V)> for Grid<T>
where
    V: Into<usize>,
{
    type Output = T;

    fn index(&self, index: (V, V)) -> &Self::Output {
        self.get(index.0.into(), index.1.into())
    }
}

impl<T, V> IndexMut<(V, V)> for Grid<T>
where
    V: Into<usize>,
{
    fn index_mut(&mut self, index: (V, V)) -> &mut Self::Output {
        self.get_mut(index.0.into(), index.1.into())
    }
}

/// Uses a `Pair` for indexing.
impl<T, U> Index<Pair<U>> for Grid<T>
where
    U: Into<usize>,
{
    type Output = T;

    fn index(&self, index: Pair<U>) -> &Self::Output {
        self.get(index.x.into(), index.y.into())
    }
}

impl<T, U> IndexMut<Pair<U>> for Grid<T>
where
    U: Into<usize>,
{
    fn index_mut(&mut self, index: Pair<U>) -> &mut Self::Output {
        self.get_mut(index.x.into(), index.y.into())
    }
}
