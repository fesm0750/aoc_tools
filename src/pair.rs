//! A helper struct for representing 2d values, i.e: coordinates, indexes, points on a
//! grid or plane, etc.
use std::{
    cmp,
    convert::{TryFrom, TryInto},
    error::Error,
    fmt::Debug,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
    str::FromStr,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pair<U> {
    pub x: U,
    pub y: U,
}

impl<U: Copy> Pair<U> {
    /// Constructs a new Pair
    pub fn new(x: U, y: U) -> Pair<U> {
        Pair { x, y }
    }

    /// Constructs a new Pair from a tuple
    pub fn from_tuple(t: (U, U)) -> Pair<U> {
        Pair { x: t.0, y: t.1 }
    }

    /// Returns a tuple `(x, y)`.
    /// In some situations a tuple is a more handy alternative.
    pub fn tuple(&self) -> (U, U) {
        (self.x, self.y)
    }
}

//--------------------------------------------------------------------
// Distance between two points
//--------------------------------------------------------------------

impl<F> Pair<F>
where
    F: Into<f64> + Copy,
{
    pub fn distance(&self, rhs: &Self) -> f64 {
        let a = self.x.into() - rhs.x.into();
        let b = self.y.into() - rhs.y.into();
        f64::sqrt(a * a + b * b)
    }
}

impl<U> Pair<U>
where
    U: Add<Output = U> + Sub<Output = U> + Ord + Copy,
{
    pub fn distance_manhattan(&self, rhs: &Self) -> U {
        cmp::max(self.x, rhs.x) - cmp::min(self.x, rhs.x) + cmp::max(self.y, rhs.y) - cmp::min(self.y, rhs.y)
    }
}

impl<U> Pair<U>
where
    U: Add<Output = U> + Sub<Output = U> + Mul<Output = U> + Ord + Copy,
{
    pub fn distance_squared(&self, rhs: &Self) -> U {
        let a = cmp::max(self.x, rhs.x) - cmp::min(self.x, rhs.x);
        let b = cmp::max(self.y, rhs.y) - cmp::min(self.y, rhs.y);
        a * a + b * b
    }
}

//--------------------------------------------------------------------
// Arithmetic Operations
//--------------------------------------------------------------------

impl<U> Add for Pair<U>
where
    U: Add<Output = U>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<U: AddAssign> AddAssign for Pair<U> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<U> Sub for Pair<U>
where
    U: Sub<Output = U>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<U: SubAssign> SubAssign for Pair<U> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/// Multiplication by scalar.
impl<U> Mul<U> for Pair<U>
where
    U: Mul<Output = U> + Copy,
{
    type Output = Self;

    fn mul(self, scalar: U) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

//--------------------------------------------------------------------
// Conversion traits
//--------------------------------------------------------------------

/// Defines how to convert a tuple (U, U) to a Point<T>,`U` must implement `TryInto<T>`.
/// An user case would be to convert a tuple of `usize` to a Base2d of a smaller unsigned
/// or an integer.
///
///  If the conversion is Infallible, the method `from_tuple` may be a more handy
/// alternative.
impl<U, T> TryFrom<(U, U)> for Pair<T>
where
    U: TryInto<T>,
    <U as TryInto<T>>::Error: std::error::Error + 'static,
{
    type Error = Box<dyn Error>;

    fn try_from(item: (U, U)) -> Result<Self, Self::Error> {
        Ok(Pair {
            x: item.0.try_into()?,
            y: item.1.try_into()?,
        })
    }
}

impl<U> FromStr for Pair<U>
where
    U: FromStr + Copy,
    <U as FromStr>::Err: std::error::Error + 'static,
{
    type Err = Box<dyn Error>;

    /// string needs to have two values separated by comma (','), no blank space allowed.
    /// Examples: "15,21" or "A,B".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        let x = iter
            .next()
            .ok_or("Could not parse the number before the comma.")?
            .parse::<U>()?;
        let y = iter
            .next()
            .ok_or("Could not parse the number after the comma.")?
            .parse::<U>()?;
        Ok(Pair::new(x, y))
    }
}
