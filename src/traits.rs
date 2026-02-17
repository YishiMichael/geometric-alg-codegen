use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Additive identity.
pub trait Zero {
    fn zero() -> Self;
}

/// Multiplicative identity.
pub trait One {
    fn one() -> Self;
}

// Grade selection. -> From
// pub trait Select<T> {
//     fn select(self) -> T;
// }

/// Flip signs by `g % 2 = 1`
pub trait GradeInvolution {
    type Output;
    fn grade_involution(self) -> Self::Output;
}

/// Flip signs by `g % 4 = 2, 3`
pub trait Reverse {
    type Output;
    fn reverse(self) -> Self::Output;
}

/// Flip signs by `g % 4 = 1, 2`
pub trait Conjugate {
    type Output;
    fn conjugate(self) -> Self::Output;
}

/// `geometric_product(a, dual(a)) = I`
pub trait Dual {
    type Output;
    fn dual(self) -> Self::Output;
}

/// `geometric_product(inverse_dual(a), a) = I`
pub trait Undual {
    type Output;
    fn undual(self) -> Self::Output;
}

/// `scalar_product(reverse(A), A)`
pub trait NormSquared {
    type Output;
    fn norm_squared(self) -> Self::Output;
}

/// `sqrt(abs(norm_squared(A)))`
pub trait Norm {
    type Output;
    fn norm(self) -> Self::Output;
}

/// `reverse(A) / norm_squared(A)`
pub trait Inverse {
    type Output;
    fn inverse(self) -> Self::Output;
}

/// `A / norm(A)`
pub trait Normalized {
    type Output;
    fn normalized(self) -> Self::Output;
}

/// In-place normalization.
pub trait Normalize {
    fn normalize(&mut self);
}

/// The geometric product.
pub trait GeometricProduct<T> {
    type Output;
    fn geometric_product(self, other: T) -> Self::Output;
}

/// Geometric product grade-filtered by `c = 0`
pub trait ScalarProduct<T> {
    type Output;
    fn scalar_product(self, other: T) -> Self::Output;
}

/// Geometric product grade-filtered by `c = b - a`
pub trait LeftInnerProduct<T> {
    type Output;
    fn left_inner_product(self, other: T) -> Self::Output;
}

/// Geometric product grade-filtered by `c = a - b`
pub trait RightInnerProduct<T> {
    type Output;
    fn right_inner_product(self, other: T) -> Self::Output;
}

/// Geometric product grade-filtered by `c = |a - b|`
pub trait InnerProduct<T> {
    type Output;
    fn inner_product(self, other: T) -> Self::Output;
}

/// Geometric product grade-filtered by `c = a + b`
pub trait OuterProduct<T> {
    type Output;
    fn outer_product(self, other: T) -> Self::Output;
}

/// `inverse_dual(outer_product(dual(a), dual(b)))`
pub trait RegressiveProduct<T> {
    type Output;
    fn regressive_product(self, other: T) -> Self::Output;
}

/// `(geometric_product(A, B) - geometric_product(B, A)) / 2`
pub trait Commutator<T> {
    type Output;
    fn commutator(self, other: T) -> Self::Output;
}

/// `(geometric_product(A, B) + geometric_product(B, A)) / 2`
pub trait Anticommutator<T> {
    type Output;
    fn anticommutator(self, other: T) -> Self::Output;
}

/// `geometric_product(geometric_product(A, B), reverse(A))`
pub trait Transform<T> {
    type Output;
    fn transform(self, other: T) -> Self::Output;
}

/// `left_inner(left_inner(A, B), reverse(B))`
pub trait Project<T> {
    type Output;
    fn project(self, other: T) -> Self::Output;
}

/// `right_inner(outer(A, B), reverse(B))`
pub trait Reject<T> {
    type Output;
    fn reject(self, other: T) -> Self::Output;
}
