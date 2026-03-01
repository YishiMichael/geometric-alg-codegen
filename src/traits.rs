pub use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Additive identity.
pub trait Zero {
    fn zero() -> Self;
}

/// Multiplicative identity.
pub trait One {
    fn one() -> Self;
}

/// The grade involution operation.
///
/// Flip signs of blades with grade `g` where `g % 2 = 1`.
pub trait Involute {
    type Output;
    fn involute(self) -> Self::Output;
}

/// The reverse operation.
///
/// Flip signs of blades with grade `g` where `g % 4 = 2, 3`.
pub trait Reverse {
    type Output;
    fn reverse(self) -> Self::Output;
}

/// The Clifford conjugation operation.
///
/// Flip signs of blades with grade `g` where `g % 4 = 1, 2`.
pub trait Conjugate {
    type Output;
    fn conjugate(self) -> Self::Output;
}

/// The Hodge dual operation.
///
/// Satisfies `geometric_product(e, dual(e)) = I` for any blade `e`.
pub trait Dual {
    type Output;
    fn dual(self) -> Self::Output;
}

/// The Hodge undual operation (inverse of dual).
///
/// Satisfies `geometric_product(undual(e), e) = I` for any blade `e`.
pub trait Undual {
    type Output;
    fn undual(self) -> Self::Output;
}

/// The norm squared operation.
///
/// Equivalent to `scalar_product(A, reverse(A))`.
pub trait NormSquared {
    type Output;
    fn norm_squared(self) -> Self::Output;
}

/// The norm operation.
///
/// Equivalent to `sqrt(abs(norm_squared(A)))`.
pub trait Norm {
    type Output;
    fn norm(self) -> Self::Output;
}

/// The normalization operation.
///
/// Equivalent to `A / norm(A)`.
pub trait Normalized {
    type Output;
    fn normalized(self) -> Self::Output;
}

/// The in-place normalization operation.
pub trait Normalize {
    fn normalize(&mut self);
}

/// The inverse operation.
///
/// Satisfies `geometric_product(inverse(A), A) = geometric_product(A, inverse(A)) = 1`.
pub trait Inverse {
    type Output;
    fn inverse(self) -> Self::Output;
}

/// The geometric product operation.
pub trait GeometricProduct<T> {
    type Output;
    fn geometric_product(self, other: T) -> Self::Output;
}

/// The scalar product operation.
///
/// Geometric product grade-filtered by `c = 0`.
pub trait ScalarProduct<T> {
    type Output;
    fn scalar_product(self, other: T) -> Self::Output;
}

/// The left inner product operation. Also called left contraction.
///
/// Geometric product grade-filtered by `c = b - a`.
pub trait LeftInnerProduct<T> {
    type Output;
    fn left_inner_product(self, other: T) -> Self::Output;
}

/// The right inner product operation. Also called right contraction.
///
/// Geometric product grade-filtered by `c = a - b`.
pub trait RightInnerProduct<T> {
    type Output;
    fn right_inner_product(self, other: T) -> Self::Output;
}

/// The inner product operation.
///
/// Geometric product grade-filtered by `c = |a - b|`.
pub trait InnerProduct<T> {
    type Output;
    fn inner_product(self, other: T) -> Self::Output;
}

/// The outer product operation.
///
/// Geometric product grade-filtered by `c = a + b`.
pub trait OuterProduct<T> {
    type Output;
    fn outer_product(self, other: T) -> Self::Output;
}

/// The regressive product operation.
///
/// Equivalent to `undual(outer_product(dual(a), dual(b)))`.
pub trait RegressiveProduct<T> {
    type Output;
    fn regressive_product(self, other: T) -> Self::Output;
}

/// The commutator product operation.
///
/// Equivalent to `(geometric_product(A, B) - geometric_product(B, A)) / 2`.
pub trait Commutator<T> {
    type Output;
    fn commutator(self, other: T) -> Self::Output;
}

/// The anticommutator product operation.
///
/// Equivalent to `(geometric_product(A, B) + geometric_product(B, A)) / 2`.
pub trait Anticommutator<T> {
    type Output;
    fn anticommutator(self, other: T) -> Self::Output;
}

/// Transform `A` by versor `B`. Also called sandwich product.
///
/// Equivalent to `geometric_product(geometric_product(involute(B), involute(A)), reverse(B))`
///
/// Note, the actual definition is `geometric_product(geometric_product(involute(B), involute(A)), inverse(B))`.
/// You may want to scale the result by `1 / norm_squared(B)` if `norm_squared(B) != 1`.
pub trait Transform<T> {
    type Output;
    fn transform(self, other: T) -> Self::Output;
}

/// Orthogonal projection of `A` into versor `B`.
///
/// Equivalent to `left_inner_product(left_inner_product(A, B), reverse(B))`.
///
/// Note, the actual definition is `left_inner_product(left_inner_product(A, B), inverse(B))`.
/// You may want to scale the result by `1 / norm_squared(B)` if `norm_squared(B) != 1`.
pub trait Project<T> {
    type Output;
    fn project(self, other: T) -> Self::Output;
}

/// Orthogonal rejection of `A` from versor `B`.
///
/// Equivalent to `right_inner_product(outer_product(A, B), reverse(B))`.
///
/// Note, the actual definition is `right_inner_product(outer_product(A, B), inverse(B))`.
/// You may want to scale the result by `1 / norm_squared(B)` if `norm_squared(B) != 1`.
pub trait Reject<T> {
    type Output;
    fn reject(self, other: T) -> Self::Output;
}
