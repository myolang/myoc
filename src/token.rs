use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct ByteIndex(pub usize);

impl Add for ByteIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ByteIndex(self.0 + rhs.0)
    }
}

impl Sub for ByteIndex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ByteIndex(self.0 - rhs.0)
    }
}

pub type Span = (ByteIndex, ByteIndex);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Ident {
    pub value: String,
    pub start: ByteIndex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct NumberLiteral {
    pub value: usize,
    pub span: (ByteIndex, ByteIndex),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct StringLiteral {
    pub value: String,
    pub span: (ByteIndex, ByteIndex),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct UniverseLiteral {
    pub level: usize,
    pub start: ByteIndex,
    /// This is true if the literal is a `Prop`
    /// and false if the literal is a `Set`.
    pub erasable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct EnumKw {
    pub level: usize,
    pub start: ByteIndex,
    /// This is true if the literal is a `prop`
    /// and false if the literal is a `set`.
    pub erasable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct VconIndexLiteral {
    pub index: usize,
    pub start: ByteIndex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct ReturnArityLiteral {
    pub arity: usize,
    pub start: ByteIndex,
}

pub use crate::cst::Token;
