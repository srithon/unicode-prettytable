pub trait Stringifiable<'a>
where
    Self: AsRef<str>,
    Self: 'a,
    &'a Self: AsRef<str>
{ }

impl <'a, X> Stringifiable<'a> for X
where
    X: AsRef<str>,
    X: 'a,
    &'a X: AsRef<str>
{ }

pub trait IntoStringIterator1D<'a> : IntoIterator
where
    Self::Item: Stringifiable<'a>
{ }

impl <'a, X> IntoStringIterator1D<'a> for X
where
    X: IntoIterator,
    X::Item: Stringifiable<'a>
{ }

pub trait IntoStringIterator2D<'a> : IntoIterator
where
    Self::Item: IntoStringIterator1D<'a>,
    <<Self as IntoIterator>::Item as IntoIterator>::Item: Stringifiable<'a>
{ }

impl <'a, T> IntoStringIterator2D<'a> for T
where
    T: IntoIterator,
    Self::Item: IntoStringIterator1D<'a>,
    <<Self as IntoIterator>::Item as IntoIterator>::Item: Stringifiable<'a>
{ }

