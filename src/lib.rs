use std::array::TryFromSliceError;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
    ParseUtf8(Utf8Error),
    General(String),
    PatternMatch,
    EmptyIterator,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Error::ParseFloat(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::ParseUtf8(e)
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(_e: std::convert::Infallible) -> Self {
        panic!("Infallible error should never occur")
    }
}

impl From<TryFromSliceError> for Error {
    fn from(e: TryFromSliceError) -> Self {
        Error::General(e.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => f.write_fmt(format_args!("Io: {}", e)),
            Error::ParseInt(e) => f.write_fmt(format_args!("Parse: {}", e)),
            Error::ParseFloat(e) => f.write_fmt(format_args!("Parse: {}", e)),
            Error::ParseUtf8(e) => f.write_fmt(format_args!("Parse: {}", e)),
            Error::General(s) => f.write_fmt(format_args!("General: {}", s)),
            Error::PatternMatch => f.write_str("Pattern mismatch"),
            Error::EmptyIterator => f.write_str("Empty iterator"),
        }
    }
}

impl std::error::Error for Error {}

pub fn read_lines(file: &str) -> Result<Vec<String>> {
    let path = Path::new(file);
    let io = File::open(path)?;
    let br = BufReader::new(io);
    Ok(br.lines().collect::<std::io::Result<Vec<String>>>()?)
}

#[derive(Clone,Copy,Default,Eq, PartialEq)]
pub struct Bitmap64(u64);

impl Bitmap64 {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    #[inline]
    pub fn is_set(&self, value: usize) -> bool {
        self.0 & (1<<value) != 0
    }

    #[inline]
    pub fn set_mut(&mut self, value: usize) {
        self.0 |= 1 << value;
    }

    #[inline]
    pub fn unset_mut(&mut self, value: usize) {
        self.0 &= !(1 << value);
    }

    #[inline]
    pub fn set(&self, value: usize) -> Self {
        let mut result = *self;
        result.set_mut(value);
        result
    }

    #[inline]
    pub fn unset(&self, value: usize) -> Self {
        let mut result = *self;
        result.unset_mut(value);
        result
    }

    #[inline]
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    #[inline]
    pub fn and(&self, other: &Bitmap64) -> Self {
        Bitmap64(self.0 & other.0)
    }

    #[inline]
    pub fn and_not(&self, other: &Bitmap64) -> Self {
        Bitmap64(self.0 & !other.0)
    }

    #[inline]
    pub fn or(&self, other: &Bitmap64) -> Self {
        Bitmap64(self.0 | other.0)
    }

    #[inline]
    pub fn iter(&self) -> Bitmap64Iter {
        Bitmap64Iter(self.0)
    }
}

impl Debug for Bitmap64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Bitmap64({:064b}, {})", self.0, self.0))
    }
}

impl From<u64> for Bitmap64 {
    #[inline]
    fn from(mask: u64) -> Self {
        Bitmap64(mask)
    }
}

impl FromIterator<usize> for Bitmap64 {
    fn from_iter<T: IntoIterator<Item=usize>>(iter: T) -> Self {
        let mut bitmap = Bitmap64(0);
        for elem in iter {
            assert!(elem <= 64);
            bitmap.set_mut(elem);
        }
        bitmap
    }
}

impl IntoIterator for Bitmap64 {
    type Item = usize;
    type IntoIter = Bitmap64Iter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Bitmap64Iter(u64);

impl Iterator for Bitmap64Iter {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let tz = self.0.trailing_zeros();
            self.0 &= !(1<<tz);
            Some(tz as usize)
        }
    }
}