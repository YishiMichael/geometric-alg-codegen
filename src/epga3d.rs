// Automatically generated
use crate::traits::*;

pub struct Scalar {
    s: f32,
}

pub struct Vector {
    e0: f32,
    e1: f32,
    e2: f32,
    e3: f32,
}

pub struct Bivector {
    e01: f32,
    e02: f32,
    e12: f32,
    e03: f32,
    e31: f32,
    e23: f32,
}

pub struct Trivector {
    e021: f32,
    e013: f32,
    e032: f32,
    e123: f32,
}

pub struct FourVector {
    e0123: f32,
}

pub struct Null;

pub struct OddMultivector {
    e0: f32,
    e1: f32,
    e2: f32,
    e021: f32,
    e3: f32,
    e013: f32,
    e032: f32,
    e123: f32,
}

pub struct EvenMultivector {
    s: f32,
    e01: f32,
    e02: f32,
    e12: f32,
    e03: f32,
    e31: f32,
    e23: f32,
    e0123: f32,
}

pub struct Multivector {
    s: f32,
    e0: f32,
    e1: f32,
    e01: f32,
    e2: f32,
    e02: f32,
    e12: f32,
    e021: f32,
    e3: f32,
    e03: f32,
    e31: f32,
    e013: f32,
    e23: f32,
    e032: f32,
    e123: f32,
    e0123: f32,
}

impl GradeInvolution for Scalar {
    type Output = Scalar;
    fn grade_involution(self) -> Scalar {
        Scalar {
            s: self.s,
        }
    }
}

impl GradeInvolution for Vector {
    type Output = Vector;
    fn grade_involution(self) -> Vector {
        Vector {
            e0: self.e0,
            e1: -self.e1,
            e2: -self.e2,
            e3: -self.e3,
        }
    }
}

impl GradeInvolution for Bivector {
    type Output = Bivector;
    fn grade_involution(self) -> Bivector {
        Bivector {
            e01: -self.e01,
            e02: -self.e02,
            e12: self.e12,
            e03: -self.e03,
            e31: self.e31,
            e23: self.e23,
        }
    }
}

impl GradeInvolution for Trivector {
    type Output = Trivector;
    fn grade_involution(self) -> Trivector {
        Trivector {
            e021: self.e021,
            e013: self.e013,
            e032: self.e032,
            e123: -self.e123,
        }
    }
}

impl GradeInvolution for FourVector {
    type Output = FourVector;
    fn grade_involution(self) -> FourVector {
        FourVector {
            e0123: -self.e0123,
        }
    }
}

impl GradeInvolution for Null {
    type Output = Null;
    fn grade_involution(self) -> Null {
        Null
    }
}

impl GradeInvolution for OddMultivector {
    type Output = OddMultivector;
    fn grade_involution(self) -> OddMultivector {
        OddMultivector {
            e0: self.e0,
            e1: -self.e1,
            e2: -self.e2,
            e021: self.e021,
            e3: -self.e3,
            e013: self.e013,
            e032: self.e032,
            e123: -self.e123,
        }
    }
}

impl GradeInvolution for EvenMultivector {
    type Output = EvenMultivector;
    fn grade_involution(self) -> EvenMultivector {
        EvenMultivector {
            s: self.s,
            e01: -self.e01,
            e02: -self.e02,
            e12: self.e12,
            e03: -self.e03,
            e31: self.e31,
            e23: self.e23,
            e0123: -self.e0123,
        }
    }
}

impl GradeInvolution for Multivector {
    type Output = Multivector;
    fn grade_involution(self) -> Multivector {
        Multivector {
            s: self.s,
            e0: self.e0,
            e1: -self.e1,
            e01: -self.e01,
            e2: -self.e2,
            e02: -self.e02,
            e12: self.e12,
            e021: self.e021,
            e3: -self.e3,
            e03: -self.e03,
            e31: self.e31,
            e013: self.e013,
            e23: self.e23,
            e032: self.e032,
            e123: -self.e123,
            e0123: -self.e0123,
        }
    }
}

impl Reverse for Scalar {
    type Output = Scalar;
    fn reverse(self) -> Scalar {
        Scalar {
            s: self.s,
        }
    }
}

impl Reverse for Vector {
    type Output = Vector;
    fn reverse(self) -> Vector {
        Vector {
            e0: -self.e0,
            e1: self.e1,
            e2: self.e2,
            e3: self.e3,
        }
    }
}

impl Reverse for Bivector {
    type Output = Bivector;
    fn reverse(self) -> Bivector {
        Bivector {
            e01: self.e01,
            e02: self.e02,
            e12: -self.e12,
            e03: self.e03,
            e31: -self.e31,
            e23: -self.e23,
        }
    }
}

impl Reverse for Trivector {
    type Output = Trivector;
    fn reverse(self) -> Trivector {
        Trivector {
            e021: self.e021,
            e013: self.e013,
            e032: self.e032,
            e123: -self.e123,
        }
    }
}

impl Reverse for FourVector {
    type Output = FourVector;
    fn reverse(self) -> FourVector {
        FourVector {
            e0123: -self.e0123,
        }
    }
}

impl Reverse for Null {
    type Output = Null;
    fn reverse(self) -> Null {
        Null
    }
}

impl Reverse for OddMultivector {
    type Output = OddMultivector;
    fn reverse(self) -> OddMultivector {
        OddMultivector {
            e0: -self.e0,
            e1: self.e1,
            e2: self.e2,
            e021: self.e021,
            e3: self.e3,
            e013: self.e013,
            e032: self.e032,
            e123: -self.e123,
        }
    }
}

impl Reverse for EvenMultivector {
    type Output = EvenMultivector;
    fn reverse(self) -> EvenMultivector {
        EvenMultivector {
            s: self.s,
            e01: self.e01,
            e02: self.e02,
            e12: -self.e12,
            e03: self.e03,
            e31: -self.e31,
            e23: -self.e23,
            e0123: -self.e0123,
        }
    }
}

impl Reverse for Multivector {
    type Output = Multivector;
    fn reverse(self) -> Multivector {
        Multivector {
            s: self.s,
            e0: -self.e0,
            e1: self.e1,
            e01: self.e01,
            e2: self.e2,
            e02: self.e02,
            e12: -self.e12,
            e021: self.e021,
            e3: self.e3,
            e03: self.e03,
            e31: -self.e31,
            e013: self.e013,
            e23: -self.e23,
            e032: self.e032,
            e123: -self.e123,
            e0123: -self.e0123,
        }
    }
}

impl Conjugate for Scalar {
    type Output = Scalar;
    fn conjugate(self) -> Scalar {
        Scalar {
            s: self.s,
        }
    }
}

impl Conjugate for Vector {
    type Output = Vector;
    fn conjugate(self) -> Vector {
        Vector {
            e0: self.e0,
            e1: -self.e1,
            e2: -self.e2,
            e3: -self.e3,
        }
    }
}

impl Conjugate for Bivector {
    type Output = Bivector;
    fn conjugate(self) -> Bivector {
        Bivector {
            e01: self.e01,
            e02: self.e02,
            e12: -self.e12,
            e03: self.e03,
            e31: -self.e31,
            e23: -self.e23,
        }
    }
}

impl Conjugate for Trivector {
    type Output = Trivector;
    fn conjugate(self) -> Trivector {
        Trivector {
            e021: -self.e021,
            e013: -self.e013,
            e032: -self.e032,
            e123: self.e123,
        }
    }
}

impl Conjugate for FourVector {
    type Output = FourVector;
    fn conjugate(self) -> FourVector {
        FourVector {
            e0123: -self.e0123,
        }
    }
}

impl Conjugate for Null {
    type Output = Null;
    fn conjugate(self) -> Null {
        Null
    }
}

impl Conjugate for OddMultivector {
    type Output = OddMultivector;
    fn conjugate(self) -> OddMultivector {
        OddMultivector {
            e0: self.e0,
            e1: -self.e1,
            e2: -self.e2,
            e021: -self.e021,
            e3: -self.e3,
            e013: -self.e013,
            e032: -self.e032,
            e123: self.e123,
        }
    }
}

impl Conjugate for EvenMultivector {
    type Output = EvenMultivector;
    fn conjugate(self) -> EvenMultivector {
        EvenMultivector {
            s: self.s,
            e01: self.e01,
            e02: self.e02,
            e12: -self.e12,
            e03: self.e03,
            e31: -self.e31,
            e23: -self.e23,
            e0123: -self.e0123,
        }
    }
}

impl Conjugate for Multivector {
    type Output = Multivector;
    fn conjugate(self) -> Multivector {
        Multivector {
            s: self.s,
            e0: self.e0,
            e1: -self.e1,
            e01: self.e01,
            e2: -self.e2,
            e02: self.e02,
            e12: -self.e12,
            e021: -self.e021,
            e3: -self.e3,
            e03: self.e03,
            e31: -self.e31,
            e013: -self.e013,
            e23: -self.e23,
            e032: -self.e032,
            e123: self.e123,
            e0123: -self.e0123,
        }
    }
}

impl Dual for Scalar {
    type Output = FourVector;
    fn dual(self) -> FourVector {
        FourVector {
            e0123: self.s,
        }
    }
}

impl Dual for Vector {
    type Output = Trivector;
    fn dual(self) -> Trivector {
        Trivector {
            e021: self.e3,
            e013: -self.e2,
            e032: self.e1,
            e123: self.e0,
        }
    }
}

impl Dual for Bivector {
    type Output = Bivector;
    fn dual(self) -> Bivector {
        Bivector {
            e01: self.e23,
            e02: -self.e31,
            e12: -self.e03,
            e03: self.e12,
            e31: self.e02,
            e23: -self.e01,
        }
    }
}

impl Dual for Trivector {
    type Output = Vector;
    fn dual(self) -> Vector {
        Vector {
            e0: self.e123,
            e1: self.e032,
            e2: -self.e013,
            e3: self.e021,
        }
    }
}

impl Dual for FourVector {
    type Output = Scalar;
    fn dual(self) -> Scalar {
        Scalar {
            s: -self.e0123,
        }
    }
}

impl Dual for Null {
    type Output = Null;
    fn dual(self) -> Null {
        Null
    }
}

impl Dual for OddMultivector {
    type Output = OddMultivector;
    fn dual(self) -> OddMultivector {
        OddMultivector {
            e0: self.e123,
            e1: self.e032,
            e2: -self.e013,
            e021: self.e3,
            e3: self.e021,
            e013: -self.e2,
            e032: self.e1,
            e123: self.e0,
        }
    }
}

impl Dual for EvenMultivector {
    type Output = EvenMultivector;
    fn dual(self) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0123,
            e01: self.e23,
            e02: -self.e31,
            e12: -self.e03,
            e03: self.e12,
            e31: self.e02,
            e23: -self.e01,
            e0123: self.s,
        }
    }
}

impl Dual for Multivector {
    type Output = Multivector;
    fn dual(self) -> Multivector {
        Multivector {
            s: -self.e0123,
            e0: self.e123,
            e1: self.e032,
            e01: self.e23,
            e2: -self.e013,
            e02: -self.e31,
            e12: -self.e03,
            e021: self.e3,
            e3: self.e021,
            e03: self.e12,
            e31: self.e02,
            e013: -self.e2,
            e23: -self.e01,
            e032: self.e1,
            e123: self.e0,
            e0123: self.s,
        }
    }
}

impl Undual for Scalar {
    type Output = FourVector;
    fn undual(self) -> FourVector {
        FourVector {
            e0123: self.s,
        }
    }
}

impl Undual for Vector {
    type Output = Trivector;
    fn undual(self) -> Trivector {
        Trivector {
            e021: -self.e3,
            e013: self.e2,
            e032: -self.e1,
            e123: -self.e0,
        }
    }
}

impl Undual for Bivector {
    type Output = Bivector;
    fn undual(self) -> Bivector {
        Bivector {
            e01: self.e23,
            e02: -self.e31,
            e12: -self.e03,
            e03: self.e12,
            e31: self.e02,
            e23: -self.e01,
        }
    }
}

impl Undual for Trivector {
    type Output = Vector;
    fn undual(self) -> Vector {
        Vector {
            e0: -self.e123,
            e1: -self.e032,
            e2: self.e013,
            e3: -self.e021,
        }
    }
}

impl Undual for FourVector {
    type Output = Scalar;
    fn undual(self) -> Scalar {
        Scalar {
            s: -self.e0123,
        }
    }
}

impl Undual for Null {
    type Output = Null;
    fn undual(self) -> Null {
        Null
    }
}

impl Undual for OddMultivector {
    type Output = OddMultivector;
    fn undual(self) -> OddMultivector {
        OddMultivector {
            e0: -self.e123,
            e1: -self.e032,
            e2: self.e013,
            e021: -self.e3,
            e3: -self.e021,
            e013: self.e2,
            e032: -self.e1,
            e123: -self.e0,
        }
    }
}

impl Undual for EvenMultivector {
    type Output = EvenMultivector;
    fn undual(self) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0123,
            e01: self.e23,
            e02: -self.e31,
            e12: -self.e03,
            e03: self.e12,
            e31: self.e02,
            e23: -self.e01,
            e0123: self.s,
        }
    }
}

impl Undual for Multivector {
    type Output = Multivector;
    fn undual(self) -> Multivector {
        Multivector {
            s: -self.e0123,
            e0: -self.e123,
            e1: -self.e032,
            e01: self.e23,
            e2: self.e013,
            e02: -self.e31,
            e12: -self.e03,
            e021: -self.e3,
            e3: -self.e021,
            e03: self.e12,
            e31: self.e02,
            e013: self.e2,
            e23: -self.e01,
            e032: -self.e1,
            e123: -self.e0,
            e0123: self.s,
        }
    }
}

impl NormSquared for Scalar {
    type Output = f32;
    fn norm_squared(self) -> f32 {
        self.s * self.s
    }
}

impl NormSquared for Vector {
    type Output = f32;
    fn norm_squared(self) -> f32 {
        -self.e0 * self.e0 + self.e1 * self.e1 + self.e2 * self.e2 + self.e3 * self.e3
    }
}

impl NormSquared for Bivector {
    type Output = f32;
    fn norm_squared(self) -> f32 {
        -self.e01 * self.e01 - self.e02 * self.e02 + self.e12 * self.e12 - self.e03 * self.e03 + self.e31 * self.e31 + self.e23 * self.e23
    }
}

impl NormSquared for Trivector {
    type Output = f32;
    fn norm_squared(self) -> f32 {
        -self.e021 * self.e021 - self.e013 * self.e013 - self.e032 * self.e032 + self.e123 * self.e123
    }
}

impl NormSquared for FourVector {
    type Output = f32;
    fn norm_squared(self) -> f32 {
        -self.e0123 * self.e0123
    }
}

impl NormSquared for Null {
    type Output = f32;
    fn norm_squared(self) -> f32 {
        0.0
    }
}

impl NormSquared for OddMultivector {
    type Output = f32;
    fn norm_squared(self) -> f32 {
        -self.e0 * self.e0 + self.e1 * self.e1 + self.e2 * self.e2 - self.e021 * self.e021 + self.e3 * self.e3 - self.e013 * self.e013 - self.e032 * self.e032 + self.e123 * self.e123
    }
}

impl NormSquared for EvenMultivector {
    type Output = f32;
    fn norm_squared(self) -> f32 {
        self.s * self.s - self.e01 * self.e01 - self.e02 * self.e02 + self.e12 * self.e12 - self.e03 * self.e03 + self.e31 * self.e31 + self.e23 * self.e23 - self.e0123 * self.e0123
    }
}

impl NormSquared for Multivector {
    type Output = f32;
    fn norm_squared(self) -> f32 {
        self.s * self.s - self.e0 * self.e0 + self.e1 * self.e1 - self.e01 * self.e01 + self.e2 * self.e2 - self.e02 * self.e02 + self.e12 * self.e12 - self.e021 * self.e021 + self.e3 * self.e3 - self.e03 * self.e03 + self.e31 * self.e31 - self.e013 * self.e013 + self.e23 * self.e23 - self.e032 * self.e032 + self.e123 * self.e123 - self.e0123 * self.e0123
    }
}

impl GeometricProduct<Scalar> for Scalar {
    type Output = EvenMultivector;
    fn geometric_product(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl GeometricProduct<Vector> for Scalar {
    type Output = OddMultivector;
    fn geometric_product(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e2: self.s * other.e2,
            e021: 0.0,
            e3: self.s * other.e3,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl GeometricProduct<Bivector> for Scalar {
    type Output = EvenMultivector;
    fn geometric_product(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.s * other.e01,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e23: self.s * other.e23,
            e0123: 0.0,
        }
    }
}

impl GeometricProduct<Trivector> for Scalar {
    type Output = OddMultivector;
    fn geometric_product(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: self.s * other.e021,
            e3: 0.0,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl GeometricProduct<FourVector> for Scalar {
    type Output = EvenMultivector;
    fn geometric_product(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: self.s * other.e0123,
        }
    }
}

impl GeometricProduct<Null> for Scalar {
    type Output = Null;
    fn geometric_product(self, other: Null) -> Null {
        Null
    }
}

impl GeometricProduct<OddMultivector> for Scalar {
    type Output = OddMultivector;
    fn geometric_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e2: self.s * other.e2,
            e021: self.s * other.e021,
            e3: self.s * other.e3,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl GeometricProduct<EvenMultivector> for Scalar {
    type Output = EvenMultivector;
    fn geometric_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: self.s * other.e01,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e23: self.s * other.e23,
            e0123: self.s * other.e0123,
        }
    }
}

impl GeometricProduct<Multivector> for Scalar {
    type Output = Multivector;
    fn geometric_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e01: self.s * other.e01,
            e2: self.s * other.e2,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e021: self.s * other.e021,
            e3: self.s * other.e3,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e013: self.s * other.e013,
            e23: self.s * other.e23,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
            e0123: self.s * other.e0123,
        }
    }
}

impl GeometricProduct<Scalar> for Vector {
    type Output = OddMultivector;
    fn geometric_product(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e2: self.e2 * other.s,
            e021: 0.0,
            e3: self.e3 * other.s,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl GeometricProduct<Vector> for Vector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e31: -self.e1 * other.e3 + self.e3 * other.e1,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
            e0123: 0.0,
        }
    }
}

impl GeometricProduct<Bivector> for Vector {
    type Output = OddMultivector;
    fn geometric_product(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        }
    }
}

impl GeometricProduct<Trivector> for Vector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl GeometricProduct<FourVector> for Vector {
    type Output = OddMultivector;
    fn geometric_product(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: self.e3 * other.e0123,
            e3: 0.0,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
        }
    }
}

impl GeometricProduct<Null> for Vector {
    type Output = Null;
    fn geometric_product(self, other: Null) -> Null {
        Null
    }
}

impl GeometricProduct<OddMultivector> for Vector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e01: self.e0 * other.e1 - self.e1 * other.e0 - self.e2 * other.e021 + self.e3 * other.e013,
            e02: self.e0 * other.e2 + self.e1 * other.e021 - self.e2 * other.e0 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e1 * other.e2 - self.e2 * other.e1 + self.e3 * other.e123,
            e03: self.e0 * other.e3 - self.e1 * other.e013 + self.e2 * other.e032 - self.e3 * other.e0,
            e31: self.e0 * other.e013 - self.e1 * other.e3 + self.e2 * other.e123 + self.e3 * other.e1,
            e23: self.e0 * other.e032 + self.e1 * other.e123 + self.e2 * other.e3 - self.e3 * other.e2,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl GeometricProduct<EvenMultivector> for Vector {
    type Output = OddMultivector;
    fn geometric_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s - self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 + self.e1 * other.s - self.e2 * other.e12 + self.e3 * other.e31,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 + self.e2 * other.s - self.e3 * other.e23,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01 + self.e3 * other.e0123,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 + self.e3 * other.s,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e2 * other.e0123 + self.e3 * other.e01,
            e032: -self.e0 * other.e23 + self.e1 * other.e0123 + self.e2 * other.e03 - self.e3 * other.e02,
            e123: -self.e0 * other.e0123 + self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        }
    }
}

impl GeometricProduct<Multivector> for Vector {
    type Output = Multivector;
    fn geometric_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e0: self.e0 * other.s - self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 + self.e1 * other.s - self.e2 * other.e12 + self.e3 * other.e31,
            e01: self.e0 * other.e1 - self.e1 * other.e0 - self.e2 * other.e021 + self.e3 * other.e013,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 + self.e2 * other.s - self.e3 * other.e23,
            e02: self.e0 * other.e2 + self.e1 * other.e021 - self.e2 * other.e0 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e1 * other.e2 - self.e2 * other.e1 + self.e3 * other.e123,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01 + self.e3 * other.e0123,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 + self.e3 * other.s,
            e03: self.e0 * other.e3 - self.e1 * other.e013 + self.e2 * other.e032 - self.e3 * other.e0,
            e31: self.e0 * other.e013 - self.e1 * other.e3 + self.e2 * other.e123 + self.e3 * other.e1,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e2 * other.e0123 + self.e3 * other.e01,
            e23: self.e0 * other.e032 + self.e1 * other.e123 + self.e2 * other.e3 - self.e3 * other.e2,
            e032: -self.e0 * other.e23 + self.e1 * other.e0123 + self.e2 * other.e03 - self.e3 * other.e02,
            e123: -self.e0 * other.e0123 + self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl GeometricProduct<Scalar> for Bivector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e01 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e23: self.e23 * other.s,
            e0123: 0.0,
        }
    }
}

impl GeometricProduct<Vector> for Bivector {
    type Output = OddMultivector;
    fn geometric_product(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e013: self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e032: -self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        }
    }
}

impl GeometricProduct<Bivector> for Bivector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e01: -self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03,
            e02: self.e01 * other.e12 - self.e12 * other.e01 - self.e03 * other.e23 + self.e23 * other.e03,
            e12: self.e01 * other.e02 - self.e02 * other.e01 + self.e31 * other.e23 - self.e23 * other.e31,
            e03: -self.e01 * other.e31 + self.e02 * other.e23 + self.e31 * other.e01 - self.e23 * other.e02,
            e31: -self.e01 * other.e03 - self.e12 * other.e23 + self.e03 * other.e01 + self.e23 * other.e12,
            e23: self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12,
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl GeometricProduct<Trivector> for Bivector {
    type Output = OddMultivector;
    fn geometric_product(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e021: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e013: -self.e02 * other.e123 - self.e12 * other.e032 + self.e23 * other.e021,
            e032: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021,
            e123: -self.e01 * other.e032 - self.e02 * other.e013 - self.e03 * other.e021,
        }
    }
}

impl GeometricProduct<FourVector> for Bivector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e23 * other.e0123,
            e02: -self.e31 * other.e0123,
            e12: self.e03 * other.e0123,
            e03: -self.e12 * other.e0123,
            e31: self.e02 * other.e0123,
            e23: self.e01 * other.e0123,
            e0123: 0.0,
        }
    }
}

impl GeometricProduct<Null> for Bivector {
    type Output = Null;
    fn geometric_product(self, other: Null) -> Null {
        Null
    }
}

impl GeometricProduct<OddMultivector> for Bivector {
    type Output = OddMultivector;
    fn geometric_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e12 * other.e021 + self.e03 * other.e3 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e01 * other.e0 + self.e02 * other.e021 + self.e12 * other.e2 - self.e03 * other.e013 - self.e31 * other.e3 - self.e23 * other.e123,
            e2: -self.e01 * other.e021 + self.e02 * other.e0 - self.e12 * other.e1 + self.e03 * other.e032 - self.e31 * other.e123 + self.e23 * other.e3,
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0 - self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e013: self.e01 * other.e3 - self.e02 * other.e123 - self.e12 * other.e032 - self.e03 * other.e1 - self.e31 * other.e0 + self.e23 * other.e021,
            e032: -self.e01 * other.e123 - self.e02 * other.e3 + self.e12 * other.e013 + self.e03 * other.e2 - self.e31 * other.e021 - self.e23 * other.e0,
            e123: -self.e01 * other.e032 - self.e02 * other.e013 + self.e12 * other.e3 - self.e03 * other.e021 + self.e31 * other.e2 + self.e23 * other.e1,
        }
    }
}

impl GeometricProduct<EvenMultivector> for Bivector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e01: self.e01 * other.s - self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03 - self.e23 * other.e0123,
            e02: self.e01 * other.e12 + self.e02 * other.s - self.e12 * other.e01 - self.e03 * other.e23 - self.e31 * other.e0123 + self.e23 * other.e03,
            e12: self.e01 * other.e02 - self.e02 * other.e01 + self.e12 * other.s + self.e03 * other.e0123 + self.e31 * other.e23 - self.e23 * other.e31,
            e03: -self.e01 * other.e31 + self.e02 * other.e23 - self.e12 * other.e0123 + self.e03 * other.s + self.e31 * other.e01 - self.e23 * other.e02,
            e31: -self.e01 * other.e03 + self.e02 * other.e0123 - self.e12 * other.e23 + self.e03 * other.e01 + self.e31 * other.s + self.e23 * other.e12,
            e23: self.e01 * other.e0123 + self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12 + self.e23 * other.s,
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl GeometricProduct<Multivector> for Bivector {
    type Output = Multivector;
    fn geometric_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e12 * other.e021 + self.e03 * other.e3 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e01 * other.e0 + self.e02 * other.e021 + self.e12 * other.e2 - self.e03 * other.e013 - self.e31 * other.e3 - self.e23 * other.e123,
            e01: self.e01 * other.s - self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03 - self.e23 * other.e0123,
            e2: -self.e01 * other.e021 + self.e02 * other.e0 - self.e12 * other.e1 + self.e03 * other.e032 - self.e31 * other.e123 + self.e23 * other.e3,
            e02: self.e01 * other.e12 + self.e02 * other.s - self.e12 * other.e01 - self.e03 * other.e23 - self.e31 * other.e0123 + self.e23 * other.e03,
            e12: self.e01 * other.e02 - self.e02 * other.e01 + self.e12 * other.s + self.e03 * other.e0123 + self.e31 * other.e23 - self.e23 * other.e31,
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0 - self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e03: -self.e01 * other.e31 + self.e02 * other.e23 - self.e12 * other.e0123 + self.e03 * other.s + self.e31 * other.e01 - self.e23 * other.e02,
            e31: -self.e01 * other.e03 + self.e02 * other.e0123 - self.e12 * other.e23 + self.e03 * other.e01 + self.e31 * other.s + self.e23 * other.e12,
            e013: self.e01 * other.e3 - self.e02 * other.e123 - self.e12 * other.e032 - self.e03 * other.e1 - self.e31 * other.e0 + self.e23 * other.e021,
            e23: self.e01 * other.e0123 + self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12 + self.e23 * other.s,
            e032: -self.e01 * other.e123 - self.e02 * other.e3 + self.e12 * other.e013 + self.e03 * other.e2 - self.e31 * other.e021 - self.e23 * other.e0,
            e123: -self.e01 * other.e032 - self.e02 * other.e013 + self.e12 * other.e3 - self.e03 * other.e021 + self.e31 * other.e2 + self.e23 * other.e1,
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl GeometricProduct<Scalar> for Trivector {
    type Output = OddMultivector;
    fn geometric_product(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: self.e021 * other.s,
            e3: 0.0,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl GeometricProduct<Vector> for Trivector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl GeometricProduct<Bivector> for Trivector {
    type Output = OddMultivector;
    fn geometric_product(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e2: -self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e021: self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e013: -self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e032: self.e021 * other.e31 - self.e013 * other.e12 + self.e123 * other.e01,
            e123: self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
        }
    }
}

impl GeometricProduct<Trivector> for Trivector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: self.e032 * other.e123 - self.e123 * other.e032,
            e02: self.e013 * other.e123 - self.e123 * other.e013,
            e12: -self.e013 * other.e032 + self.e032 * other.e013,
            e03: self.e021 * other.e123 - self.e123 * other.e021,
            e31: self.e021 * other.e032 - self.e032 * other.e021,
            e23: -self.e021 * other.e013 + self.e013 * other.e021,
            e0123: 0.0,
        }
    }
}

impl GeometricProduct<FourVector> for Trivector {
    type Output = OddMultivector;
    fn geometric_product(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e2: -self.e013 * other.e0123,
            e021: 0.0,
            e3: -self.e021 * other.e0123,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl GeometricProduct<Null> for Trivector {
    type Output = Null;
    fn geometric_product(self, other: Null) -> Null {
        Null
    }
}

impl GeometricProduct<OddMultivector> for Trivector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: -self.e021 * other.e2 + self.e013 * other.e3 + self.e032 * other.e123 - self.e123 * other.e032,
            e02: self.e021 * other.e1 + self.e013 * other.e123 - self.e032 * other.e3 - self.e123 * other.e013,
            e12: self.e021 * other.e0 - self.e013 * other.e032 + self.e032 * other.e013 + self.e123 * other.e3,
            e03: self.e021 * other.e123 - self.e013 * other.e1 + self.e032 * other.e2 - self.e123 * other.e021,
            e31: self.e021 * other.e032 + self.e013 * other.e0 - self.e032 * other.e021 + self.e123 * other.e2,
            e23: -self.e021 * other.e013 + self.e013 * other.e021 + self.e032 * other.e0 + self.e123 * other.e1,
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl GeometricProduct<EvenMultivector> for Trivector {
    type Output = OddMultivector;
    fn geometric_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23 + self.e123 * other.e0123,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e032 * other.e0123 - self.e123 * other.e23,
            e2: -self.e021 * other.e01 - self.e013 * other.e0123 + self.e032 * other.e03 - self.e123 * other.e31,
            e021: self.e021 * other.s + self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: -self.e021 * other.e0123 + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e013: -self.e021 * other.e23 + self.e013 * other.s + self.e032 * other.e12 + self.e123 * other.e02,
            e032: self.e021 * other.e31 - self.e013 * other.e12 + self.e032 * other.s + self.e123 * other.e01,
            e123: self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01 + self.e123 * other.s,
        }
    }
}

impl GeometricProduct<Multivector> for Trivector {
    type Output = Multivector;
    fn geometric_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23 + self.e123 * other.e0123,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e032 * other.e0123 - self.e123 * other.e23,
            e01: -self.e021 * other.e2 + self.e013 * other.e3 + self.e032 * other.e123 - self.e123 * other.e032,
            e2: -self.e021 * other.e01 - self.e013 * other.e0123 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.e021 * other.e1 + self.e013 * other.e123 - self.e032 * other.e3 - self.e123 * other.e013,
            e12: self.e021 * other.e0 - self.e013 * other.e032 + self.e032 * other.e013 + self.e123 * other.e3,
            e021: self.e021 * other.s + self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: -self.e021 * other.e0123 + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: self.e021 * other.e123 - self.e013 * other.e1 + self.e032 * other.e2 - self.e123 * other.e021,
            e31: self.e021 * other.e032 + self.e013 * other.e0 - self.e032 * other.e021 + self.e123 * other.e2,
            e013: -self.e021 * other.e23 + self.e013 * other.s + self.e032 * other.e12 + self.e123 * other.e02,
            e23: -self.e021 * other.e013 + self.e013 * other.e021 + self.e032 * other.e0 + self.e123 * other.e1,
            e032: self.e021 * other.e31 - self.e013 * other.e12 + self.e032 * other.s + self.e123 * other.e01,
            e123: self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01 + self.e123 * other.s,
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl GeometricProduct<Scalar> for FourVector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: self.e0123 * other.s,
        }
    }
}

impl GeometricProduct<Vector> for FourVector {
    type Output = OddMultivector;
    fn geometric_product(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: -self.e0123 * other.e3,
            e3: 0.0,
            e013: -self.e0123 * other.e2,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
        }
    }
}

impl GeometricProduct<Bivector> for FourVector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e0123 * other.e23,
            e02: -self.e0123 * other.e31,
            e12: self.e0123 * other.e03,
            e03: -self.e0123 * other.e12,
            e31: self.e0123 * other.e02,
            e23: self.e0123 * other.e01,
            e0123: 0.0,
        }
    }
}

impl GeometricProduct<Trivector> for FourVector {
    type Output = OddMultivector;
    fn geometric_product(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e2: self.e0123 * other.e013,
            e021: 0.0,
            e3: self.e0123 * other.e021,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl GeometricProduct<FourVector> for FourVector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0123 * other.e0123,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl GeometricProduct<Null> for FourVector {
    type Output = Null;
    fn geometric_product(self, other: Null) -> Null {
        Null
    }
}

impl GeometricProduct<OddMultivector> for FourVector {
    type Output = OddMultivector;
    fn geometric_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e2: self.e0123 * other.e013,
            e021: -self.e0123 * other.e3,
            e3: self.e0123 * other.e021,
            e013: -self.e0123 * other.e2,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
        }
    }
}

impl GeometricProduct<EvenMultivector> for FourVector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0123 * other.e0123,
            e01: -self.e0123 * other.e23,
            e02: -self.e0123 * other.e31,
            e12: self.e0123 * other.e03,
            e03: -self.e0123 * other.e12,
            e31: self.e0123 * other.e02,
            e23: self.e0123 * other.e01,
            e0123: self.e0123 * other.s,
        }
    }
}

impl GeometricProduct<Multivector> for FourVector {
    type Output = Multivector;
    fn geometric_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0123 * other.e0123,
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e01: -self.e0123 * other.e23,
            e2: self.e0123 * other.e013,
            e02: -self.e0123 * other.e31,
            e12: self.e0123 * other.e03,
            e021: -self.e0123 * other.e3,
            e3: self.e0123 * other.e021,
            e03: -self.e0123 * other.e12,
            e31: self.e0123 * other.e02,
            e013: -self.e0123 * other.e2,
            e23: self.e0123 * other.e01,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
            e0123: self.e0123 * other.s,
        }
    }
}

impl GeometricProduct<Scalar> for Null {
    type Output = Null;
    fn geometric_product(self, other: Scalar) -> Null {
        Null
    }
}

impl GeometricProduct<Vector> for Null {
    type Output = Null;
    fn geometric_product(self, other: Vector) -> Null {
        Null
    }
}

impl GeometricProduct<Bivector> for Null {
    type Output = Null;
    fn geometric_product(self, other: Bivector) -> Null {
        Null
    }
}

impl GeometricProduct<Trivector> for Null {
    type Output = Null;
    fn geometric_product(self, other: Trivector) -> Null {
        Null
    }
}

impl GeometricProduct<FourVector> for Null {
    type Output = Null;
    fn geometric_product(self, other: FourVector) -> Null {
        Null
    }
}

impl GeometricProduct<Null> for Null {
    type Output = Null;
    fn geometric_product(self, other: Null) -> Null {
        Null
    }
}

impl GeometricProduct<OddMultivector> for Null {
    type Output = Null;
    fn geometric_product(self, other: OddMultivector) -> Null {
        Null
    }
}

impl GeometricProduct<EvenMultivector> for Null {
    type Output = Null;
    fn geometric_product(self, other: EvenMultivector) -> Null {
        Null
    }
}

impl GeometricProduct<Multivector> for Null {
    type Output = Null;
    fn geometric_product(self, other: Multivector) -> Null {
        Null
    }
}

impl GeometricProduct<Scalar> for OddMultivector {
    type Output = OddMultivector;
    fn geometric_product(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e2: self.e2 * other.s,
            e021: self.e021 * other.s,
            e3: self.e3 * other.s,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl GeometricProduct<Vector> for OddMultivector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e01: self.e0 * other.e1 - self.e1 * other.e0 - self.e021 * other.e2 + self.e013 * other.e3,
            e02: self.e0 * other.e2 - self.e2 * other.e0 + self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e1 * other.e2 - self.e2 * other.e1 + self.e021 * other.e0 + self.e123 * other.e3,
            e03: self.e0 * other.e3 - self.e3 * other.e0 - self.e013 * other.e1 + self.e032 * other.e2,
            e31: -self.e1 * other.e3 + self.e3 * other.e1 + self.e013 * other.e0 + self.e123 * other.e2,
            e23: self.e2 * other.e3 - self.e3 * other.e2 + self.e032 * other.e0 + self.e123 * other.e1,
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl GeometricProduct<Bivector> for OddMultivector {
    type Output = OddMultivector;
    fn geometric_product(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e1 * other.e01 - self.e2 * other.e02 + self.e021 * other.e12 - self.e3 * other.e03 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e021 * other.e02 + self.e3 * other.e31 - self.e013 * other.e03 - self.e123 * other.e23,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e021 * other.e01 - self.e3 * other.e23 + self.e032 * other.e03 - self.e123 * other.e31,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01 + self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 - self.e021 * other.e23 + self.e3 * other.e01 + self.e032 * other.e12 + self.e123 * other.e02,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 + self.e021 * other.e31 - self.e3 * other.e02 - self.e013 * other.e12 + self.e123 * other.e01,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e021 * other.e03 + self.e3 * other.e12 + self.e013 * other.e02 + self.e032 * other.e01,
        }
    }
}

impl GeometricProduct<Trivector> for OddMultivector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: -self.e2 * other.e021 + self.e3 * other.e013 + self.e032 * other.e123 - self.e123 * other.e032,
            e02: self.e1 * other.e021 - self.e3 * other.e032 + self.e013 * other.e123 - self.e123 * other.e013,
            e12: self.e0 * other.e021 + self.e3 * other.e123 - self.e013 * other.e032 + self.e032 * other.e013,
            e03: -self.e1 * other.e013 + self.e2 * other.e032 + self.e021 * other.e123 - self.e123 * other.e021,
            e31: self.e0 * other.e013 + self.e2 * other.e123 + self.e021 * other.e032 - self.e032 * other.e021,
            e23: self.e0 * other.e032 + self.e1 * other.e123 - self.e021 * other.e013 + self.e013 * other.e021,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl GeometricProduct<FourVector> for OddMultivector {
    type Output = OddMultivector;
    fn geometric_product(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e2: -self.e013 * other.e0123,
            e021: self.e3 * other.e0123,
            e3: -self.e021 * other.e0123,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
        }
    }
}

impl GeometricProduct<Null> for OddMultivector {
    type Output = Null;
    fn geometric_product(self, other: Null) -> Null {
        Null
    }
}

impl GeometricProduct<OddMultivector> for OddMultivector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: self.e0 * other.e1 - self.e1 * other.e0 - self.e2 * other.e021 - self.e021 * other.e2 + self.e3 * other.e013 + self.e013 * other.e3 + self.e032 * other.e123 - self.e123 * other.e032,
            e02: self.e0 * other.e2 + self.e1 * other.e021 - self.e2 * other.e0 + self.e021 * other.e1 - self.e3 * other.e032 + self.e013 * other.e123 - self.e032 * other.e3 - self.e123 * other.e013,
            e12: self.e0 * other.e021 + self.e1 * other.e2 - self.e2 * other.e1 + self.e021 * other.e0 + self.e3 * other.e123 - self.e013 * other.e032 + self.e032 * other.e013 + self.e123 * other.e3,
            e03: self.e0 * other.e3 - self.e1 * other.e013 + self.e2 * other.e032 + self.e021 * other.e123 - self.e3 * other.e0 - self.e013 * other.e1 + self.e032 * other.e2 - self.e123 * other.e021,
            e31: self.e0 * other.e013 - self.e1 * other.e3 + self.e2 * other.e123 + self.e021 * other.e032 + self.e3 * other.e1 + self.e013 * other.e0 - self.e032 * other.e021 + self.e123 * other.e2,
            e23: self.e0 * other.e032 + self.e1 * other.e123 + self.e2 * other.e3 - self.e021 * other.e013 - self.e3 * other.e2 + self.e013 * other.e021 + self.e032 * other.e0 + self.e123 * other.e1,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 - self.e021 * other.e3 + self.e3 * other.e021 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl GeometricProduct<EvenMultivector> for OddMultivector {
    type Output = OddMultivector;
    fn geometric_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s - self.e1 * other.e01 - self.e2 * other.e02 + self.e021 * other.e12 - self.e3 * other.e03 + self.e013 * other.e31 + self.e032 * other.e23 + self.e123 * other.e0123,
            e1: -self.e0 * other.e01 + self.e1 * other.s - self.e2 * other.e12 + self.e021 * other.e02 + self.e3 * other.e31 - self.e013 * other.e03 - self.e032 * other.e0123 - self.e123 * other.e23,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 + self.e2 * other.s - self.e021 * other.e01 - self.e3 * other.e23 - self.e013 * other.e0123 + self.e032 * other.e03 - self.e123 * other.e31,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01 + self.e021 * other.s + self.e3 * other.e0123 + self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 - self.e021 * other.e0123 + self.e3 * other.s + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e2 * other.e0123 - self.e021 * other.e23 + self.e3 * other.e01 + self.e013 * other.s + self.e032 * other.e12 + self.e123 * other.e02,
            e032: -self.e0 * other.e23 + self.e1 * other.e0123 + self.e2 * other.e03 + self.e021 * other.e31 - self.e3 * other.e02 - self.e013 * other.e12 + self.e032 * other.s + self.e123 * other.e01,
            e123: -self.e0 * other.e0123 + self.e1 * other.e23 + self.e2 * other.e31 + self.e021 * other.e03 + self.e3 * other.e12 + self.e013 * other.e02 + self.e032 * other.e01 + self.e123 * other.s,
        }
    }
}

impl GeometricProduct<Multivector> for OddMultivector {
    type Output = Multivector;
    fn geometric_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.e0 * other.s - self.e1 * other.e01 - self.e2 * other.e02 + self.e021 * other.e12 - self.e3 * other.e03 + self.e013 * other.e31 + self.e032 * other.e23 + self.e123 * other.e0123,
            e1: -self.e0 * other.e01 + self.e1 * other.s - self.e2 * other.e12 + self.e021 * other.e02 + self.e3 * other.e31 - self.e013 * other.e03 - self.e032 * other.e0123 - self.e123 * other.e23,
            e01: self.e0 * other.e1 - self.e1 * other.e0 - self.e2 * other.e021 - self.e021 * other.e2 + self.e3 * other.e013 + self.e013 * other.e3 + self.e032 * other.e123 - self.e123 * other.e032,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 + self.e2 * other.s - self.e021 * other.e01 - self.e3 * other.e23 - self.e013 * other.e0123 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.e0 * other.e2 + self.e1 * other.e021 - self.e2 * other.e0 + self.e021 * other.e1 - self.e3 * other.e032 + self.e013 * other.e123 - self.e032 * other.e3 - self.e123 * other.e013,
            e12: self.e0 * other.e021 + self.e1 * other.e2 - self.e2 * other.e1 + self.e021 * other.e0 + self.e3 * other.e123 - self.e013 * other.e032 + self.e032 * other.e013 + self.e123 * other.e3,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01 + self.e021 * other.s + self.e3 * other.e0123 + self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 - self.e021 * other.e0123 + self.e3 * other.s + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: self.e0 * other.e3 - self.e1 * other.e013 + self.e2 * other.e032 + self.e021 * other.e123 - self.e3 * other.e0 - self.e013 * other.e1 + self.e032 * other.e2 - self.e123 * other.e021,
            e31: self.e0 * other.e013 - self.e1 * other.e3 + self.e2 * other.e123 + self.e021 * other.e032 + self.e3 * other.e1 + self.e013 * other.e0 - self.e032 * other.e021 + self.e123 * other.e2,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e2 * other.e0123 - self.e021 * other.e23 + self.e3 * other.e01 + self.e013 * other.s + self.e032 * other.e12 + self.e123 * other.e02,
            e23: self.e0 * other.e032 + self.e1 * other.e123 + self.e2 * other.e3 - self.e021 * other.e013 - self.e3 * other.e2 + self.e013 * other.e021 + self.e032 * other.e0 + self.e123 * other.e1,
            e032: -self.e0 * other.e23 + self.e1 * other.e0123 + self.e2 * other.e03 + self.e021 * other.e31 - self.e3 * other.e02 - self.e013 * other.e12 + self.e032 * other.s + self.e123 * other.e01,
            e123: -self.e0 * other.e0123 + self.e1 * other.e23 + self.e2 * other.e31 + self.e021 * other.e03 + self.e3 * other.e12 + self.e013 * other.e02 + self.e032 * other.e01 + self.e123 * other.s,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 - self.e021 * other.e3 + self.e3 * other.e021 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl GeometricProduct<Scalar> for EvenMultivector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: self.e01 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e23: self.e23 * other.s,
            e0123: self.e0123 * other.s,
        }
    }
}

impl GeometricProduct<Vector> for EvenMultivector {
    type Output = OddMultivector;
    fn geometric_product(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0 + self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.s * other.e1 + self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e2: self.s * other.e2 + self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0 - self.e0123 * other.e3,
            e3: self.s * other.e3 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e013: self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0 - self.e0123 * other.e2,
            e032: -self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0 - self.e0123 * other.e1,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1 + self.e0123 * other.e0,
        }
    }
}

impl GeometricProduct<Bivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e01: self.s * other.e01 - self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03 - self.e0123 * other.e23,
            e02: self.s * other.e02 + self.e01 * other.e12 - self.e12 * other.e01 - self.e03 * other.e23 + self.e23 * other.e03 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e01 * other.e02 - self.e02 * other.e01 + self.e31 * other.e23 - self.e23 * other.e31 + self.e0123 * other.e03,
            e03: self.s * other.e03 - self.e01 * other.e31 + self.e02 * other.e23 + self.e31 * other.e01 - self.e23 * other.e02 - self.e0123 * other.e12,
            e31: self.s * other.e31 - self.e01 * other.e03 - self.e12 * other.e23 + self.e03 * other.e01 + self.e23 * other.e12 + self.e0123 * other.e02,
            e23: self.s * other.e23 + self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12 + self.e0123 * other.e01,
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl GeometricProduct<Trivector> for EvenMultivector {
    type Output = OddMultivector;
    fn geometric_product(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032 - self.e0123 * other.e123,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123 + self.e0123 * other.e032,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123 + self.e0123 * other.e013,
            e021: self.s * other.e021 - self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e0123 * other.e021,
            e013: self.s * other.e013 - self.e02 * other.e123 - self.e12 * other.e032 + self.e23 * other.e021,
            e032: self.s * other.e032 - self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021,
            e123: self.s * other.e123 - self.e01 * other.e032 - self.e02 * other.e013 - self.e03 * other.e021,
        }
    }
}

impl GeometricProduct<FourVector> for EvenMultivector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0123 * other.e0123,
            e01: -self.e23 * other.e0123,
            e02: -self.e31 * other.e0123,
            e12: self.e03 * other.e0123,
            e03: -self.e12 * other.e0123,
            e31: self.e02 * other.e0123,
            e23: self.e01 * other.e0123,
            e0123: self.s * other.e0123,
        }
    }
}

impl GeometricProduct<Null> for EvenMultivector {
    type Output = Null;
    fn geometric_product(self, other: Null) -> Null {
        Null
    }
}

impl GeometricProduct<OddMultivector> for EvenMultivector {
    type Output = OddMultivector;
    fn geometric_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0 + self.e01 * other.e1 + self.e02 * other.e2 + self.e12 * other.e021 + self.e03 * other.e3 + self.e31 * other.e013 + self.e23 * other.e032 - self.e0123 * other.e123,
            e1: self.s * other.e1 + self.e01 * other.e0 + self.e02 * other.e021 + self.e12 * other.e2 - self.e03 * other.e013 - self.e31 * other.e3 - self.e23 * other.e123 + self.e0123 * other.e032,
            e2: self.s * other.e2 - self.e01 * other.e021 + self.e02 * other.e0 - self.e12 * other.e1 + self.e03 * other.e032 - self.e31 * other.e123 + self.e23 * other.e3 + self.e0123 * other.e013,
            e021: self.s * other.e021 - self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0 - self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013 - self.e0123 * other.e3,
            e3: self.s * other.e3 + self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2 + self.e0123 * other.e021,
            e013: self.s * other.e013 + self.e01 * other.e3 - self.e02 * other.e123 - self.e12 * other.e032 - self.e03 * other.e1 - self.e31 * other.e0 + self.e23 * other.e021 - self.e0123 * other.e2,
            e032: self.s * other.e032 - self.e01 * other.e123 - self.e02 * other.e3 + self.e12 * other.e013 + self.e03 * other.e2 - self.e31 * other.e021 - self.e23 * other.e0 - self.e0123 * other.e1,
            e123: self.s * other.e123 - self.e01 * other.e032 - self.e02 * other.e013 + self.e12 * other.e3 - self.e03 * other.e021 + self.e31 * other.e2 + self.e23 * other.e1 + self.e0123 * other.e0,
        }
    }
}

impl GeometricProduct<EvenMultivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn geometric_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e01: self.s * other.e01 + self.e01 * other.s - self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03 - self.e23 * other.e0123 - self.e0123 * other.e23,
            e02: self.s * other.e02 + self.e01 * other.e12 + self.e02 * other.s - self.e12 * other.e01 - self.e03 * other.e23 - self.e31 * other.e0123 + self.e23 * other.e03 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e01 * other.e02 - self.e02 * other.e01 + self.e12 * other.s + self.e03 * other.e0123 + self.e31 * other.e23 - self.e23 * other.e31 + self.e0123 * other.e03,
            e03: self.s * other.e03 - self.e01 * other.e31 + self.e02 * other.e23 - self.e12 * other.e0123 + self.e03 * other.s + self.e31 * other.e01 - self.e23 * other.e02 - self.e0123 * other.e12,
            e31: self.s * other.e31 - self.e01 * other.e03 + self.e02 * other.e0123 - self.e12 * other.e23 + self.e03 * other.e01 + self.e31 * other.s + self.e23 * other.e12 + self.e0123 * other.e02,
            e23: self.s * other.e23 + self.e01 * other.e0123 + self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12 + self.e23 * other.s + self.e0123 * other.e01,
            e0123: self.s * other.e0123 + self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01 + self.e0123 * other.s,
        }
    }
}

impl GeometricProduct<Multivector> for EvenMultivector {
    type Output = Multivector;
    fn geometric_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e0: self.s * other.e0 + self.e01 * other.e1 + self.e02 * other.e2 + self.e12 * other.e021 + self.e03 * other.e3 + self.e31 * other.e013 + self.e23 * other.e032 - self.e0123 * other.e123,
            e1: self.s * other.e1 + self.e01 * other.e0 + self.e02 * other.e021 + self.e12 * other.e2 - self.e03 * other.e013 - self.e31 * other.e3 - self.e23 * other.e123 + self.e0123 * other.e032,
            e01: self.s * other.e01 + self.e01 * other.s - self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03 - self.e23 * other.e0123 - self.e0123 * other.e23,
            e2: self.s * other.e2 - self.e01 * other.e021 + self.e02 * other.e0 - self.e12 * other.e1 + self.e03 * other.e032 - self.e31 * other.e123 + self.e23 * other.e3 + self.e0123 * other.e013,
            e02: self.s * other.e02 + self.e01 * other.e12 + self.e02 * other.s - self.e12 * other.e01 - self.e03 * other.e23 - self.e31 * other.e0123 + self.e23 * other.e03 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e01 * other.e02 - self.e02 * other.e01 + self.e12 * other.s + self.e03 * other.e0123 + self.e31 * other.e23 - self.e23 * other.e31 + self.e0123 * other.e03,
            e021: self.s * other.e021 - self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0 - self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013 - self.e0123 * other.e3,
            e3: self.s * other.e3 + self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2 + self.e0123 * other.e021,
            e03: self.s * other.e03 - self.e01 * other.e31 + self.e02 * other.e23 - self.e12 * other.e0123 + self.e03 * other.s + self.e31 * other.e01 - self.e23 * other.e02 - self.e0123 * other.e12,
            e31: self.s * other.e31 - self.e01 * other.e03 + self.e02 * other.e0123 - self.e12 * other.e23 + self.e03 * other.e01 + self.e31 * other.s + self.e23 * other.e12 + self.e0123 * other.e02,
            e013: self.s * other.e013 + self.e01 * other.e3 - self.e02 * other.e123 - self.e12 * other.e032 - self.e03 * other.e1 - self.e31 * other.e0 + self.e23 * other.e021 - self.e0123 * other.e2,
            e23: self.s * other.e23 + self.e01 * other.e0123 + self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12 + self.e23 * other.s + self.e0123 * other.e01,
            e032: self.s * other.e032 - self.e01 * other.e123 - self.e02 * other.e3 + self.e12 * other.e013 + self.e03 * other.e2 - self.e31 * other.e021 - self.e23 * other.e0 - self.e0123 * other.e1,
            e123: self.s * other.e123 - self.e01 * other.e032 - self.e02 * other.e013 + self.e12 * other.e3 - self.e03 * other.e021 + self.e31 * other.e2 + self.e23 * other.e1 + self.e0123 * other.e0,
            e0123: self.s * other.e0123 + self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01 + self.e0123 * other.s,
        }
    }
}

impl GeometricProduct<Scalar> for Multivector {
    type Output = Multivector;
    fn geometric_product(self, other: Scalar) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e01: self.e01 * other.s,
            e2: self.e2 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e021: self.e021 * other.s,
            e3: self.e3 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e013: self.e013 * other.s,
            e23: self.e23 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
            e0123: self.e0123 * other.s,
        }
    }
}

impl GeometricProduct<Vector> for Multivector {
    type Output = Multivector;
    fn geometric_product(self, other: Vector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e0: self.s * other.e0 + self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.s * other.e1 + self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e01: self.e0 * other.e1 - self.e1 * other.e0 - self.e021 * other.e2 + self.e013 * other.e3,
            e2: self.s * other.e2 + self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e02: self.e0 * other.e2 - self.e2 * other.e0 + self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e1 * other.e2 - self.e2 * other.e1 + self.e021 * other.e0 + self.e123 * other.e3,
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0 - self.e0123 * other.e3,
            e3: self.s * other.e3 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e03: self.e0 * other.e3 - self.e3 * other.e0 - self.e013 * other.e1 + self.e032 * other.e2,
            e31: -self.e1 * other.e3 + self.e3 * other.e1 + self.e013 * other.e0 + self.e123 * other.e2,
            e013: self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0 - self.e0123 * other.e2,
            e23: self.e2 * other.e3 - self.e3 * other.e2 + self.e032 * other.e0 + self.e123 * other.e1,
            e032: -self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0 - self.e0123 * other.e1,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1 + self.e0123 * other.e0,
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl GeometricProduct<Bivector> for Multivector {
    type Output = Multivector;
    fn geometric_product(self, other: Bivector) -> Multivector {
        Multivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e0: -self.e1 * other.e01 - self.e2 * other.e02 + self.e021 * other.e12 - self.e3 * other.e03 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e021 * other.e02 + self.e3 * other.e31 - self.e013 * other.e03 - self.e123 * other.e23,
            e01: self.s * other.e01 - self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03 - self.e0123 * other.e23,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e021 * other.e01 - self.e3 * other.e23 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.s * other.e02 + self.e01 * other.e12 - self.e12 * other.e01 - self.e03 * other.e23 + self.e23 * other.e03 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e01 * other.e02 - self.e02 * other.e01 + self.e31 * other.e23 - self.e23 * other.e31 + self.e0123 * other.e03,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01 + self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: self.s * other.e03 - self.e01 * other.e31 + self.e02 * other.e23 + self.e31 * other.e01 - self.e23 * other.e02 - self.e0123 * other.e12,
            e31: self.s * other.e31 - self.e01 * other.e03 - self.e12 * other.e23 + self.e03 * other.e01 + self.e23 * other.e12 + self.e0123 * other.e02,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 - self.e021 * other.e23 + self.e3 * other.e01 + self.e032 * other.e12 + self.e123 * other.e02,
            e23: self.s * other.e23 + self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12 + self.e0123 * other.e01,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 + self.e021 * other.e31 - self.e3 * other.e02 - self.e013 * other.e12 + self.e123 * other.e01,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e021 * other.e03 + self.e3 * other.e12 + self.e013 * other.e02 + self.e032 * other.e01,
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl GeometricProduct<Trivector> for Multivector {
    type Output = Multivector;
    fn geometric_product(self, other: Trivector) -> Multivector {
        Multivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032 - self.e0123 * other.e123,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123 + self.e0123 * other.e032,
            e01: -self.e2 * other.e021 + self.e3 * other.e013 + self.e032 * other.e123 - self.e123 * other.e032,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123 + self.e0123 * other.e013,
            e02: self.e1 * other.e021 - self.e3 * other.e032 + self.e013 * other.e123 - self.e123 * other.e013,
            e12: self.e0 * other.e021 + self.e3 * other.e123 - self.e013 * other.e032 + self.e032 * other.e013,
            e021: self.s * other.e021 - self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e0123 * other.e021,
            e03: -self.e1 * other.e013 + self.e2 * other.e032 + self.e021 * other.e123 - self.e123 * other.e021,
            e31: self.e0 * other.e013 + self.e2 * other.e123 + self.e021 * other.e032 - self.e032 * other.e021,
            e013: self.s * other.e013 - self.e02 * other.e123 - self.e12 * other.e032 + self.e23 * other.e021,
            e23: self.e0 * other.e032 + self.e1 * other.e123 - self.e021 * other.e013 + self.e013 * other.e021,
            e032: self.s * other.e032 - self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021,
            e123: self.s * other.e123 - self.e01 * other.e032 - self.e02 * other.e013 - self.e03 * other.e021,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl GeometricProduct<FourVector> for Multivector {
    type Output = Multivector;
    fn geometric_product(self, other: FourVector) -> Multivector {
        Multivector {
            s: -self.e0123 * other.e0123,
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e01: -self.e23 * other.e0123,
            e2: -self.e013 * other.e0123,
            e02: -self.e31 * other.e0123,
            e12: self.e03 * other.e0123,
            e021: self.e3 * other.e0123,
            e3: -self.e021 * other.e0123,
            e03: -self.e12 * other.e0123,
            e31: self.e02 * other.e0123,
            e013: self.e2 * other.e0123,
            e23: self.e01 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
            e0123: self.s * other.e0123,
        }
    }
}

impl GeometricProduct<Null> for Multivector {
    type Output = Null;
    fn geometric_product(self, other: Null) -> Null {
        Null
    }
}

impl GeometricProduct<OddMultivector> for Multivector {
    type Output = Multivector;
    fn geometric_product(self, other: OddMultivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.s * other.e0 + self.e01 * other.e1 + self.e02 * other.e2 + self.e12 * other.e021 + self.e03 * other.e3 + self.e31 * other.e013 + self.e23 * other.e032 - self.e0123 * other.e123,
            e1: self.s * other.e1 + self.e01 * other.e0 + self.e02 * other.e021 + self.e12 * other.e2 - self.e03 * other.e013 - self.e31 * other.e3 - self.e23 * other.e123 + self.e0123 * other.e032,
            e01: self.e0 * other.e1 - self.e1 * other.e0 - self.e2 * other.e021 - self.e021 * other.e2 + self.e3 * other.e013 + self.e013 * other.e3 + self.e032 * other.e123 - self.e123 * other.e032,
            e2: self.s * other.e2 - self.e01 * other.e021 + self.e02 * other.e0 - self.e12 * other.e1 + self.e03 * other.e032 - self.e31 * other.e123 + self.e23 * other.e3 + self.e0123 * other.e013,
            e02: self.e0 * other.e2 + self.e1 * other.e021 - self.e2 * other.e0 + self.e021 * other.e1 - self.e3 * other.e032 + self.e013 * other.e123 - self.e032 * other.e3 - self.e123 * other.e013,
            e12: self.e0 * other.e021 + self.e1 * other.e2 - self.e2 * other.e1 + self.e021 * other.e0 + self.e3 * other.e123 - self.e013 * other.e032 + self.e032 * other.e013 + self.e123 * other.e3,
            e021: self.s * other.e021 - self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0 - self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013 - self.e0123 * other.e3,
            e3: self.s * other.e3 + self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2 + self.e0123 * other.e021,
            e03: self.e0 * other.e3 - self.e1 * other.e013 + self.e2 * other.e032 + self.e021 * other.e123 - self.e3 * other.e0 - self.e013 * other.e1 + self.e032 * other.e2 - self.e123 * other.e021,
            e31: self.e0 * other.e013 - self.e1 * other.e3 + self.e2 * other.e123 + self.e021 * other.e032 + self.e3 * other.e1 + self.e013 * other.e0 - self.e032 * other.e021 + self.e123 * other.e2,
            e013: self.s * other.e013 + self.e01 * other.e3 - self.e02 * other.e123 - self.e12 * other.e032 - self.e03 * other.e1 - self.e31 * other.e0 + self.e23 * other.e021 - self.e0123 * other.e2,
            e23: self.e0 * other.e032 + self.e1 * other.e123 + self.e2 * other.e3 - self.e021 * other.e013 - self.e3 * other.e2 + self.e013 * other.e021 + self.e032 * other.e0 + self.e123 * other.e1,
            e032: self.s * other.e032 - self.e01 * other.e123 - self.e02 * other.e3 + self.e12 * other.e013 + self.e03 * other.e2 - self.e31 * other.e021 - self.e23 * other.e0 - self.e0123 * other.e1,
            e123: self.s * other.e123 - self.e01 * other.e032 - self.e02 * other.e013 + self.e12 * other.e3 - self.e03 * other.e021 + self.e31 * other.e2 + self.e23 * other.e1 + self.e0123 * other.e0,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 - self.e021 * other.e3 + self.e3 * other.e021 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl GeometricProduct<EvenMultivector> for Multivector {
    type Output = Multivector;
    fn geometric_product(self, other: EvenMultivector) -> Multivector {
        Multivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e0: self.e0 * other.s - self.e1 * other.e01 - self.e2 * other.e02 + self.e021 * other.e12 - self.e3 * other.e03 + self.e013 * other.e31 + self.e032 * other.e23 + self.e123 * other.e0123,
            e1: -self.e0 * other.e01 + self.e1 * other.s - self.e2 * other.e12 + self.e021 * other.e02 + self.e3 * other.e31 - self.e013 * other.e03 - self.e032 * other.e0123 - self.e123 * other.e23,
            e01: self.s * other.e01 + self.e01 * other.s - self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03 - self.e23 * other.e0123 - self.e0123 * other.e23,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 + self.e2 * other.s - self.e021 * other.e01 - self.e3 * other.e23 - self.e013 * other.e0123 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.s * other.e02 + self.e01 * other.e12 + self.e02 * other.s - self.e12 * other.e01 - self.e03 * other.e23 - self.e31 * other.e0123 + self.e23 * other.e03 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e01 * other.e02 - self.e02 * other.e01 + self.e12 * other.s + self.e03 * other.e0123 + self.e31 * other.e23 - self.e23 * other.e31 + self.e0123 * other.e03,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01 + self.e021 * other.s + self.e3 * other.e0123 + self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 - self.e021 * other.e0123 + self.e3 * other.s + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: self.s * other.e03 - self.e01 * other.e31 + self.e02 * other.e23 - self.e12 * other.e0123 + self.e03 * other.s + self.e31 * other.e01 - self.e23 * other.e02 - self.e0123 * other.e12,
            e31: self.s * other.e31 - self.e01 * other.e03 + self.e02 * other.e0123 - self.e12 * other.e23 + self.e03 * other.e01 + self.e31 * other.s + self.e23 * other.e12 + self.e0123 * other.e02,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e2 * other.e0123 - self.e021 * other.e23 + self.e3 * other.e01 + self.e013 * other.s + self.e032 * other.e12 + self.e123 * other.e02,
            e23: self.s * other.e23 + self.e01 * other.e0123 + self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12 + self.e23 * other.s + self.e0123 * other.e01,
            e032: -self.e0 * other.e23 + self.e1 * other.e0123 + self.e2 * other.e03 + self.e021 * other.e31 - self.e3 * other.e02 - self.e013 * other.e12 + self.e032 * other.s + self.e123 * other.e01,
            e123: -self.e0 * other.e0123 + self.e1 * other.e23 + self.e2 * other.e31 + self.e021 * other.e03 + self.e3 * other.e12 + self.e013 * other.e02 + self.e032 * other.e01 + self.e123 * other.s,
            e0123: self.s * other.e0123 + self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01 + self.e0123 * other.s,
        }
    }
}

impl GeometricProduct<Multivector> for Multivector {
    type Output = Multivector;
    fn geometric_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s - self.e0 * other.e0 + self.e1 * other.e1 + self.e01 * other.e01 + self.e2 * other.e2 + self.e02 * other.e02 - self.e12 * other.e12 + self.e021 * other.e021 + self.e3 * other.e3 + self.e03 * other.e03 - self.e31 * other.e31 + self.e013 * other.e013 - self.e23 * other.e23 + self.e032 * other.e032 - self.e123 * other.e123 - self.e0123 * other.e0123,
            e0: self.s * other.e0 + self.e0 * other.s - self.e1 * other.e01 + self.e01 * other.e1 - self.e2 * other.e02 + self.e02 * other.e2 + self.e12 * other.e021 + self.e021 * other.e12 - self.e3 * other.e03 + self.e03 * other.e3 + self.e31 * other.e013 + self.e013 * other.e31 + self.e23 * other.e032 + self.e032 * other.e23 + self.e123 * other.e0123 - self.e0123 * other.e123,
            e1: self.s * other.e1 - self.e0 * other.e01 + self.e1 * other.s + self.e01 * other.e0 - self.e2 * other.e12 + self.e02 * other.e021 + self.e12 * other.e2 + self.e021 * other.e02 + self.e3 * other.e31 - self.e03 * other.e013 - self.e31 * other.e3 - self.e013 * other.e03 - self.e23 * other.e123 - self.e032 * other.e0123 - self.e123 * other.e23 + self.e0123 * other.e032,
            e01: self.s * other.e01 + self.e0 * other.e1 - self.e1 * other.e0 + self.e01 * other.s - self.e2 * other.e021 - self.e02 * other.e12 + self.e12 * other.e02 - self.e021 * other.e2 + self.e3 * other.e013 + self.e03 * other.e31 - self.e31 * other.e03 + self.e013 * other.e3 - self.e23 * other.e0123 + self.e032 * other.e123 - self.e123 * other.e032 - self.e0123 * other.e23,
            e2: self.s * other.e2 - self.e0 * other.e02 + self.e1 * other.e12 - self.e01 * other.e021 + self.e2 * other.s + self.e02 * other.e0 - self.e12 * other.e1 - self.e021 * other.e01 - self.e3 * other.e23 + self.e03 * other.e032 - self.e31 * other.e123 - self.e013 * other.e0123 + self.e23 * other.e3 + self.e032 * other.e03 - self.e123 * other.e31 + self.e0123 * other.e013,
            e02: self.s * other.e02 + self.e0 * other.e2 + self.e1 * other.e021 + self.e01 * other.e12 - self.e2 * other.e0 + self.e02 * other.s - self.e12 * other.e01 + self.e021 * other.e1 - self.e3 * other.e032 - self.e03 * other.e23 - self.e31 * other.e0123 + self.e013 * other.e123 + self.e23 * other.e03 - self.e032 * other.e3 - self.e123 * other.e013 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e0 * other.e021 + self.e1 * other.e2 + self.e01 * other.e02 - self.e2 * other.e1 - self.e02 * other.e01 + self.e12 * other.s + self.e021 * other.e0 + self.e3 * other.e123 + self.e03 * other.e0123 + self.e31 * other.e23 - self.e013 * other.e032 - self.e23 * other.e31 + self.e032 * other.e013 + self.e123 * other.e3 + self.e0123 * other.e03,
            e021: self.s * other.e021 - self.e0 * other.e12 + self.e1 * other.e02 - self.e01 * other.e2 - self.e2 * other.e01 + self.e02 * other.e1 - self.e12 * other.e0 + self.e021 * other.s + self.e3 * other.e0123 - self.e03 * other.e123 + self.e31 * other.e032 + self.e013 * other.e23 - self.e23 * other.e013 - self.e032 * other.e31 + self.e123 * other.e03 - self.e0123 * other.e3,
            e3: self.s * other.e3 - self.e0 * other.e03 - self.e1 * other.e31 + self.e01 * other.e013 + self.e2 * other.e23 - self.e02 * other.e032 - self.e12 * other.e123 - self.e021 * other.e0123 + self.e3 * other.s + self.e03 * other.e0 + self.e31 * other.e1 + self.e013 * other.e01 - self.e23 * other.e2 - self.e032 * other.e02 - self.e123 * other.e12 + self.e0123 * other.e021,
            e03: self.s * other.e03 + self.e0 * other.e3 - self.e1 * other.e013 - self.e01 * other.e31 + self.e2 * other.e032 + self.e02 * other.e23 - self.e12 * other.e0123 + self.e021 * other.e123 - self.e3 * other.e0 + self.e03 * other.s + self.e31 * other.e01 - self.e013 * other.e1 - self.e23 * other.e02 + self.e032 * other.e2 - self.e123 * other.e021 - self.e0123 * other.e12,
            e31: self.s * other.e31 + self.e0 * other.e013 - self.e1 * other.e3 - self.e01 * other.e03 + self.e2 * other.e123 + self.e02 * other.e0123 - self.e12 * other.e23 + self.e021 * other.e032 + self.e3 * other.e1 + self.e03 * other.e01 + self.e31 * other.s + self.e013 * other.e0 + self.e23 * other.e12 - self.e032 * other.e021 + self.e123 * other.e2 + self.e0123 * other.e02,
            e013: self.s * other.e013 - self.e0 * other.e31 - self.e1 * other.e03 + self.e01 * other.e3 + self.e2 * other.e0123 - self.e02 * other.e123 - self.e12 * other.e032 - self.e021 * other.e23 + self.e3 * other.e01 - self.e03 * other.e1 - self.e31 * other.e0 + self.e013 * other.s + self.e23 * other.e021 + self.e032 * other.e12 + self.e123 * other.e02 - self.e0123 * other.e2,
            e23: self.s * other.e23 + self.e0 * other.e032 + self.e1 * other.e123 + self.e01 * other.e0123 + self.e2 * other.e3 + self.e02 * other.e03 + self.e12 * other.e31 - self.e021 * other.e013 - self.e3 * other.e2 - self.e03 * other.e02 - self.e31 * other.e12 + self.e013 * other.e021 + self.e23 * other.s + self.e032 * other.e0 + self.e123 * other.e1 + self.e0123 * other.e01,
            e032: self.s * other.e032 - self.e0 * other.e23 + self.e1 * other.e0123 - self.e01 * other.e123 + self.e2 * other.e03 - self.e02 * other.e3 + self.e12 * other.e013 + self.e021 * other.e31 - self.e3 * other.e02 + self.e03 * other.e2 - self.e31 * other.e021 - self.e013 * other.e12 - self.e23 * other.e0 + self.e032 * other.s + self.e123 * other.e01 - self.e0123 * other.e1,
            e123: self.s * other.e123 - self.e0 * other.e0123 + self.e1 * other.e23 - self.e01 * other.e032 + self.e2 * other.e31 - self.e02 * other.e013 + self.e12 * other.e3 + self.e021 * other.e03 + self.e3 * other.e12 - self.e03 * other.e021 + self.e31 * other.e2 + self.e013 * other.e02 + self.e23 * other.e1 + self.e032 * other.e01 + self.e123 * other.s + self.e0123 * other.e0,
            e0123: self.s * other.e0123 + self.e0 * other.e123 + self.e1 * other.e032 + self.e01 * other.e23 + self.e2 * other.e013 + self.e02 * other.e31 + self.e12 * other.e03 - self.e021 * other.e3 + self.e3 * other.e021 + self.e03 * other.e12 + self.e31 * other.e02 - self.e013 * other.e2 + self.e23 * other.e01 - self.e032 * other.e1 - self.e123 * other.e0 + self.e0123 * other.s,
        }
    }
}

impl ScalarProduct<Scalar> for Scalar {
    type Output = f32;
    fn scalar_product(self, other: Scalar) -> f32 {
        self.s * other.s
    }
}

impl ScalarProduct<Vector> for Scalar {
    type Output = f32;
    fn scalar_product(self, other: Vector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Bivector> for Scalar {
    type Output = f32;
    fn scalar_product(self, other: Bivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Trivector> for Scalar {
    type Output = f32;
    fn scalar_product(self, other: Trivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<FourVector> for Scalar {
    type Output = f32;
    fn scalar_product(self, other: FourVector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Null> for Scalar {
    type Output = f32;
    fn scalar_product(self, other: Null) -> f32 {
        0.0
    }
}

impl ScalarProduct<OddMultivector> for Scalar {
    type Output = f32;
    fn scalar_product(self, other: OddMultivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<EvenMultivector> for Scalar {
    type Output = f32;
    fn scalar_product(self, other: EvenMultivector) -> f32 {
        self.s * other.s
    }
}

impl ScalarProduct<Multivector> for Scalar {
    type Output = f32;
    fn scalar_product(self, other: Multivector) -> f32 {
        self.s * other.s
    }
}

impl ScalarProduct<Scalar> for Vector {
    type Output = f32;
    fn scalar_product(self, other: Scalar) -> f32 {
        0.0
    }
}

impl ScalarProduct<Vector> for Vector {
    type Output = f32;
    fn scalar_product(self, other: Vector) -> f32 {
        -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3
    }
}

impl ScalarProduct<Bivector> for Vector {
    type Output = f32;
    fn scalar_product(self, other: Bivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Trivector> for Vector {
    type Output = f32;
    fn scalar_product(self, other: Trivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<FourVector> for Vector {
    type Output = f32;
    fn scalar_product(self, other: FourVector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Null> for Vector {
    type Output = f32;
    fn scalar_product(self, other: Null) -> f32 {
        0.0
    }
}

impl ScalarProduct<OddMultivector> for Vector {
    type Output = f32;
    fn scalar_product(self, other: OddMultivector) -> f32 {
        -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3
    }
}

impl ScalarProduct<EvenMultivector> for Vector {
    type Output = f32;
    fn scalar_product(self, other: EvenMultivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Multivector> for Vector {
    type Output = f32;
    fn scalar_product(self, other: Multivector) -> f32 {
        -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3
    }
}

impl ScalarProduct<Scalar> for Bivector {
    type Output = f32;
    fn scalar_product(self, other: Scalar) -> f32 {
        0.0
    }
}

impl ScalarProduct<Vector> for Bivector {
    type Output = f32;
    fn scalar_product(self, other: Vector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Bivector> for Bivector {
    type Output = f32;
    fn scalar_product(self, other: Bivector) -> f32 {
        self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23
    }
}

impl ScalarProduct<Trivector> for Bivector {
    type Output = f32;
    fn scalar_product(self, other: Trivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<FourVector> for Bivector {
    type Output = f32;
    fn scalar_product(self, other: FourVector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Null> for Bivector {
    type Output = f32;
    fn scalar_product(self, other: Null) -> f32 {
        0.0
    }
}

impl ScalarProduct<OddMultivector> for Bivector {
    type Output = f32;
    fn scalar_product(self, other: OddMultivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<EvenMultivector> for Bivector {
    type Output = f32;
    fn scalar_product(self, other: EvenMultivector) -> f32 {
        self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23
    }
}

impl ScalarProduct<Multivector> for Bivector {
    type Output = f32;
    fn scalar_product(self, other: Multivector) -> f32 {
        self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23
    }
}

impl ScalarProduct<Scalar> for Trivector {
    type Output = f32;
    fn scalar_product(self, other: Scalar) -> f32 {
        0.0
    }
}

impl ScalarProduct<Vector> for Trivector {
    type Output = f32;
    fn scalar_product(self, other: Vector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Bivector> for Trivector {
    type Output = f32;
    fn scalar_product(self, other: Bivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Trivector> for Trivector {
    type Output = f32;
    fn scalar_product(self, other: Trivector) -> f32 {
        self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123
    }
}

impl ScalarProduct<FourVector> for Trivector {
    type Output = f32;
    fn scalar_product(self, other: FourVector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Null> for Trivector {
    type Output = f32;
    fn scalar_product(self, other: Null) -> f32 {
        0.0
    }
}

impl ScalarProduct<OddMultivector> for Trivector {
    type Output = f32;
    fn scalar_product(self, other: OddMultivector) -> f32 {
        self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123
    }
}

impl ScalarProduct<EvenMultivector> for Trivector {
    type Output = f32;
    fn scalar_product(self, other: EvenMultivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Multivector> for Trivector {
    type Output = f32;
    fn scalar_product(self, other: Multivector) -> f32 {
        self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123
    }
}

impl ScalarProduct<Scalar> for FourVector {
    type Output = f32;
    fn scalar_product(self, other: Scalar) -> f32 {
        0.0
    }
}

impl ScalarProduct<Vector> for FourVector {
    type Output = f32;
    fn scalar_product(self, other: Vector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Bivector> for FourVector {
    type Output = f32;
    fn scalar_product(self, other: Bivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Trivector> for FourVector {
    type Output = f32;
    fn scalar_product(self, other: Trivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<FourVector> for FourVector {
    type Output = f32;
    fn scalar_product(self, other: FourVector) -> f32 {
        -self.e0123 * other.e0123
    }
}

impl ScalarProduct<Null> for FourVector {
    type Output = f32;
    fn scalar_product(self, other: Null) -> f32 {
        0.0
    }
}

impl ScalarProduct<OddMultivector> for FourVector {
    type Output = f32;
    fn scalar_product(self, other: OddMultivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<EvenMultivector> for FourVector {
    type Output = f32;
    fn scalar_product(self, other: EvenMultivector) -> f32 {
        -self.e0123 * other.e0123
    }
}

impl ScalarProduct<Multivector> for FourVector {
    type Output = f32;
    fn scalar_product(self, other: Multivector) -> f32 {
        -self.e0123 * other.e0123
    }
}

impl ScalarProduct<Scalar> for Null {
    type Output = f32;
    fn scalar_product(self, other: Scalar) -> f32 {
        0.0
    }
}

impl ScalarProduct<Vector> for Null {
    type Output = f32;
    fn scalar_product(self, other: Vector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Bivector> for Null {
    type Output = f32;
    fn scalar_product(self, other: Bivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Trivector> for Null {
    type Output = f32;
    fn scalar_product(self, other: Trivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<FourVector> for Null {
    type Output = f32;
    fn scalar_product(self, other: FourVector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Null> for Null {
    type Output = f32;
    fn scalar_product(self, other: Null) -> f32 {
        0.0
    }
}

impl ScalarProduct<OddMultivector> for Null {
    type Output = f32;
    fn scalar_product(self, other: OddMultivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<EvenMultivector> for Null {
    type Output = f32;
    fn scalar_product(self, other: EvenMultivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Multivector> for Null {
    type Output = f32;
    fn scalar_product(self, other: Multivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Scalar> for OddMultivector {
    type Output = f32;
    fn scalar_product(self, other: Scalar) -> f32 {
        0.0
    }
}

impl ScalarProduct<Vector> for OddMultivector {
    type Output = f32;
    fn scalar_product(self, other: Vector) -> f32 {
        -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3
    }
}

impl ScalarProduct<Bivector> for OddMultivector {
    type Output = f32;
    fn scalar_product(self, other: Bivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Trivector> for OddMultivector {
    type Output = f32;
    fn scalar_product(self, other: Trivector) -> f32 {
        self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123
    }
}

impl ScalarProduct<FourVector> for OddMultivector {
    type Output = f32;
    fn scalar_product(self, other: FourVector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Null> for OddMultivector {
    type Output = f32;
    fn scalar_product(self, other: Null) -> f32 {
        0.0
    }
}

impl ScalarProduct<OddMultivector> for OddMultivector {
    type Output = f32;
    fn scalar_product(self, other: OddMultivector) -> f32 {
        -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123
    }
}

impl ScalarProduct<EvenMultivector> for OddMultivector {
    type Output = f32;
    fn scalar_product(self, other: EvenMultivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Multivector> for OddMultivector {
    type Output = f32;
    fn scalar_product(self, other: Multivector) -> f32 {
        -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123
    }
}

impl ScalarProduct<Scalar> for EvenMultivector {
    type Output = f32;
    fn scalar_product(self, other: Scalar) -> f32 {
        self.s * other.s
    }
}

impl ScalarProduct<Vector> for EvenMultivector {
    type Output = f32;
    fn scalar_product(self, other: Vector) -> f32 {
        0.0
    }
}

impl ScalarProduct<Bivector> for EvenMultivector {
    type Output = f32;
    fn scalar_product(self, other: Bivector) -> f32 {
        self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23
    }
}

impl ScalarProduct<Trivector> for EvenMultivector {
    type Output = f32;
    fn scalar_product(self, other: Trivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<FourVector> for EvenMultivector {
    type Output = f32;
    fn scalar_product(self, other: FourVector) -> f32 {
        -self.e0123 * other.e0123
    }
}

impl ScalarProduct<Null> for EvenMultivector {
    type Output = f32;
    fn scalar_product(self, other: Null) -> f32 {
        0.0
    }
}

impl ScalarProduct<OddMultivector> for EvenMultivector {
    type Output = f32;
    fn scalar_product(self, other: OddMultivector) -> f32 {
        0.0
    }
}

impl ScalarProduct<EvenMultivector> for EvenMultivector {
    type Output = f32;
    fn scalar_product(self, other: EvenMultivector) -> f32 {
        self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123
    }
}

impl ScalarProduct<Multivector> for EvenMultivector {
    type Output = f32;
    fn scalar_product(self, other: Multivector) -> f32 {
        self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123
    }
}

impl ScalarProduct<Scalar> for Multivector {
    type Output = f32;
    fn scalar_product(self, other: Scalar) -> f32 {
        self.s * other.s
    }
}

impl ScalarProduct<Vector> for Multivector {
    type Output = f32;
    fn scalar_product(self, other: Vector) -> f32 {
        -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3
    }
}

impl ScalarProduct<Bivector> for Multivector {
    type Output = f32;
    fn scalar_product(self, other: Bivector) -> f32 {
        self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23
    }
}

impl ScalarProduct<Trivector> for Multivector {
    type Output = f32;
    fn scalar_product(self, other: Trivector) -> f32 {
        self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123
    }
}

impl ScalarProduct<FourVector> for Multivector {
    type Output = f32;
    fn scalar_product(self, other: FourVector) -> f32 {
        -self.e0123 * other.e0123
    }
}

impl ScalarProduct<Null> for Multivector {
    type Output = f32;
    fn scalar_product(self, other: Null) -> f32 {
        0.0
    }
}

impl ScalarProduct<OddMultivector> for Multivector {
    type Output = f32;
    fn scalar_product(self, other: OddMultivector) -> f32 {
        -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123
    }
}

impl ScalarProduct<EvenMultivector> for Multivector {
    type Output = f32;
    fn scalar_product(self, other: EvenMultivector) -> f32 {
        self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123
    }
}

impl ScalarProduct<Multivector> for Multivector {
    type Output = f32;
    fn scalar_product(self, other: Multivector) -> f32 {
        self.s * other.s - self.e0 * other.e0 + self.e1 * other.e1 + self.e01 * other.e01 + self.e2 * other.e2 + self.e02 * other.e02 - self.e12 * other.e12 + self.e021 * other.e021 + self.e3 * other.e3 + self.e03 * other.e03 - self.e31 * other.e31 + self.e013 * other.e013 - self.e23 * other.e23 + self.e032 * other.e032 - self.e123 * other.e123 - self.e0123 * other.e0123
    }
}

impl LeftInnerProduct<Scalar> for Scalar {
    type Output = Scalar;
    fn left_inner_product(self, other: Scalar) -> Scalar {
        Scalar {
            s: self.s * other.s,
        }
    }
}

impl LeftInnerProduct<Vector> for Scalar {
    type Output = Vector;
    fn left_inner_product(self, other: Vector) -> Vector {
        Vector {
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e2: self.s * other.e2,
            e3: self.s * other.e3,
        }
    }
}

impl LeftInnerProduct<Bivector> for Scalar {
    type Output = Bivector;
    fn left_inner_product(self, other: Bivector) -> Bivector {
        Bivector {
            e01: self.s * other.e01,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e23: self.s * other.e23,
        }
    }
}

impl LeftInnerProduct<Trivector> for Scalar {
    type Output = Trivector;
    fn left_inner_product(self, other: Trivector) -> Trivector {
        Trivector {
            e021: self.s * other.e021,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl LeftInnerProduct<FourVector> for Scalar {
    type Output = FourVector;
    fn left_inner_product(self, other: FourVector) -> FourVector {
        FourVector {
            e0123: self.s * other.e0123,
        }
    }
}

impl LeftInnerProduct<Null> for Scalar {
    type Output = Null;
    fn left_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl LeftInnerProduct<OddMultivector> for Scalar {
    type Output = OddMultivector;
    fn left_inner_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e2: self.s * other.e2,
            e021: self.s * other.e021,
            e3: self.s * other.e3,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl LeftInnerProduct<EvenMultivector> for Scalar {
    type Output = EvenMultivector;
    fn left_inner_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: self.s * other.e01,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e23: self.s * other.e23,
            e0123: self.s * other.e0123,
        }
    }
}

impl LeftInnerProduct<Multivector> for Scalar {
    type Output = Multivector;
    fn left_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e01: self.s * other.e01,
            e2: self.s * other.e2,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e021: self.s * other.e021,
            e3: self.s * other.e3,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e013: self.s * other.e013,
            e23: self.s * other.e23,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
            e0123: self.s * other.e0123,
        }
    }
}

impl LeftInnerProduct<Scalar> for Vector {
    type Output = Null;
    fn left_inner_product(self, other: Scalar) -> Null {
        Null
    }
}

impl LeftInnerProduct<Vector> for Vector {
    type Output = Scalar;
    fn left_inner_product(self, other: Vector) -> Scalar {
        Scalar {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
        }
    }
}

impl LeftInnerProduct<Bivector> for Vector {
    type Output = Vector;
    fn left_inner_product(self, other: Bivector) -> Vector {
        Vector {
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23,
        }
    }
}

impl LeftInnerProduct<Trivector> for Vector {
    type Output = Bivector;
    fn left_inner_product(self, other: Trivector) -> Bivector {
        Bivector {
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
        }
    }
}

impl LeftInnerProduct<FourVector> for Vector {
    type Output = Trivector;
    fn left_inner_product(self, other: FourVector) -> Trivector {
        Trivector {
            e021: self.e3 * other.e0123,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
        }
    }
}

impl LeftInnerProduct<Null> for Vector {
    type Output = Null;
    fn left_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl LeftInnerProduct<OddMultivector> for Vector {
    type Output = EvenMultivector;
    fn left_inner_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<EvenMultivector> for Vector {
    type Output = OddMultivector;
    fn left_inner_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23,
            e021: self.e3 * other.e0123,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
        }
    }
}

impl LeftInnerProduct<Multivector> for Vector {
    type Output = Multivector;
    fn left_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e021: self.e3 * other.e0123,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e013: self.e2 * other.e0123,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<Scalar> for Bivector {
    type Output = Null;
    fn left_inner_product(self, other: Scalar) -> Null {
        Null
    }
}

impl LeftInnerProduct<Vector> for Bivector {
    type Output = Null;
    fn left_inner_product(self, other: Vector) -> Null {
        Null
    }
}

impl LeftInnerProduct<Bivector> for Bivector {
    type Output = Scalar;
    fn left_inner_product(self, other: Bivector) -> Scalar {
        Scalar {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
        }
    }
}

impl LeftInnerProduct<Trivector> for Bivector {
    type Output = Vector;
    fn left_inner_product(self, other: Trivector) -> Vector {
        Vector {
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
        }
    }
}

impl LeftInnerProduct<FourVector> for Bivector {
    type Output = Bivector;
    fn left_inner_product(self, other: FourVector) -> Bivector {
        Bivector {
            e01: -self.e23 * other.e0123,
            e02: -self.e31 * other.e0123,
            e12: self.e03 * other.e0123,
            e03: -self.e12 * other.e0123,
            e31: self.e02 * other.e0123,
            e23: self.e01 * other.e0123,
        }
    }
}

impl LeftInnerProduct<Null> for Bivector {
    type Output = Null;
    fn left_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl LeftInnerProduct<OddMultivector> for Bivector {
    type Output = OddMultivector;
    fn left_inner_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e021: 0.0,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl LeftInnerProduct<EvenMultivector> for Bivector {
    type Output = EvenMultivector;
    fn left_inner_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e01: -self.e23 * other.e0123,
            e02: -self.e31 * other.e0123,
            e12: self.e03 * other.e0123,
            e03: -self.e12 * other.e0123,
            e31: self.e02 * other.e0123,
            e23: self.e01 * other.e0123,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<Multivector> for Bivector {
    type Output = Multivector;
    fn left_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e01: -self.e23 * other.e0123,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e02: -self.e31 * other.e0123,
            e12: self.e03 * other.e0123,
            e021: 0.0,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e03: -self.e12 * other.e0123,
            e31: self.e02 * other.e0123,
            e013: 0.0,
            e23: self.e01 * other.e0123,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<Scalar> for Trivector {
    type Output = Null;
    fn left_inner_product(self, other: Scalar) -> Null {
        Null
    }
}

impl LeftInnerProduct<Vector> for Trivector {
    type Output = Null;
    fn left_inner_product(self, other: Vector) -> Null {
        Null
    }
}

impl LeftInnerProduct<Bivector> for Trivector {
    type Output = Null;
    fn left_inner_product(self, other: Bivector) -> Null {
        Null
    }
}

impl LeftInnerProduct<Trivector> for Trivector {
    type Output = Scalar;
    fn left_inner_product(self, other: Trivector) -> Scalar {
        Scalar {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
        }
    }
}

impl LeftInnerProduct<FourVector> for Trivector {
    type Output = Vector;
    fn left_inner_product(self, other: FourVector) -> Vector {
        Vector {
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e2: -self.e013 * other.e0123,
            e3: -self.e021 * other.e0123,
        }
    }
}

impl LeftInnerProduct<Null> for Trivector {
    type Output = Null;
    fn left_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl LeftInnerProduct<OddMultivector> for Trivector {
    type Output = EvenMultivector;
    fn left_inner_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<EvenMultivector> for Trivector {
    type Output = OddMultivector;
    fn left_inner_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e2: -self.e013 * other.e0123,
            e021: 0.0,
            e3: -self.e021 * other.e0123,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl LeftInnerProduct<Multivector> for Trivector {
    type Output = Multivector;
    fn left_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e01: 0.0,
            e2: -self.e013 * other.e0123,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: -self.e021 * other.e0123,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<Scalar> for FourVector {
    type Output = Null;
    fn left_inner_product(self, other: Scalar) -> Null {
        Null
    }
}

impl LeftInnerProduct<Vector> for FourVector {
    type Output = Null;
    fn left_inner_product(self, other: Vector) -> Null {
        Null
    }
}

impl LeftInnerProduct<Bivector> for FourVector {
    type Output = Null;
    fn left_inner_product(self, other: Bivector) -> Null {
        Null
    }
}

impl LeftInnerProduct<Trivector> for FourVector {
    type Output = Null;
    fn left_inner_product(self, other: Trivector) -> Null {
        Null
    }
}

impl LeftInnerProduct<FourVector> for FourVector {
    type Output = Scalar;
    fn left_inner_product(self, other: FourVector) -> Scalar {
        Scalar {
            s: -self.e0123 * other.e0123,
        }
    }
}

impl LeftInnerProduct<Null> for FourVector {
    type Output = Null;
    fn left_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl LeftInnerProduct<OddMultivector> for FourVector {
    type Output = OddMultivector;
    fn left_inner_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl LeftInnerProduct<EvenMultivector> for FourVector {
    type Output = EvenMultivector;
    fn left_inner_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0123 * other.e0123,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<Multivector> for FourVector {
    type Output = Multivector;
    fn left_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0123 * other.e0123,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<Scalar> for Null {
    type Output = Null;
    fn left_inner_product(self, other: Scalar) -> Null {
        Null
    }
}

impl LeftInnerProduct<Vector> for Null {
    type Output = Null;
    fn left_inner_product(self, other: Vector) -> Null {
        Null
    }
}

impl LeftInnerProduct<Bivector> for Null {
    type Output = Null;
    fn left_inner_product(self, other: Bivector) -> Null {
        Null
    }
}

impl LeftInnerProduct<Trivector> for Null {
    type Output = Null;
    fn left_inner_product(self, other: Trivector) -> Null {
        Null
    }
}

impl LeftInnerProduct<FourVector> for Null {
    type Output = Null;
    fn left_inner_product(self, other: FourVector) -> Null {
        Null
    }
}

impl LeftInnerProduct<Null> for Null {
    type Output = Null;
    fn left_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl LeftInnerProduct<OddMultivector> for Null {
    type Output = Null;
    fn left_inner_product(self, other: OddMultivector) -> Null {
        Null
    }
}

impl LeftInnerProduct<EvenMultivector> for Null {
    type Output = Null;
    fn left_inner_product(self, other: EvenMultivector) -> Null {
        Null
    }
}

impl LeftInnerProduct<Multivector> for Null {
    type Output = Null;
    fn left_inner_product(self, other: Multivector) -> Null {
        Null
    }
}

impl LeftInnerProduct<Scalar> for OddMultivector {
    type Output = OddMultivector;
    fn left_inner_product(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl LeftInnerProduct<Vector> for OddMultivector {
    type Output = EvenMultivector;
    fn left_inner_product(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<Bivector> for OddMultivector {
    type Output = OddMultivector;
    fn left_inner_product(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23,
            e021: 0.0,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl LeftInnerProduct<Trivector> for OddMultivector {
    type Output = EvenMultivector;
    fn left_inner_product(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<FourVector> for OddMultivector {
    type Output = OddMultivector;
    fn left_inner_product(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e2: -self.e013 * other.e0123,
            e021: self.e3 * other.e0123,
            e3: -self.e021 * other.e0123,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
        }
    }
}

impl LeftInnerProduct<Null> for OddMultivector {
    type Output = Null;
    fn left_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl LeftInnerProduct<OddMultivector> for OddMultivector {
    type Output = EvenMultivector;
    fn left_inner_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<EvenMultivector> for OddMultivector {
    type Output = OddMultivector;
    fn left_inner_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03 + self.e123 * other.e0123,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31 - self.e032 * other.e0123,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23 - self.e013 * other.e0123,
            e021: self.e3 * other.e0123,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 - self.e021 * other.e0123,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
        }
    }
}

impl LeftInnerProduct<Multivector> for OddMultivector {
    type Output = Multivector;
    fn left_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03 + self.e123 * other.e0123,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31 - self.e032 * other.e0123,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23 - self.e013 * other.e0123,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e021: self.e3 * other.e0123,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 - self.e021 * other.e0123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e013: self.e2 * other.e0123,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<Scalar> for EvenMultivector {
    type Output = EvenMultivector;
    fn left_inner_product(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<Vector> for EvenMultivector {
    type Output = OddMultivector;
    fn left_inner_product(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e2: self.s * other.e2,
            e021: 0.0,
            e3: self.s * other.e3,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl LeftInnerProduct<Bivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn left_inner_product(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e01: self.s * other.e01,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e23: self.s * other.e23,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<Trivector> for EvenMultivector {
    type Output = OddMultivector;
    fn left_inner_product(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e021: self.s * other.e021,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl LeftInnerProduct<FourVector> for EvenMultivector {
    type Output = EvenMultivector;
    fn left_inner_product(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0123 * other.e0123,
            e01: -self.e23 * other.e0123,
            e02: -self.e31 * other.e0123,
            e12: self.e03 * other.e0123,
            e03: -self.e12 * other.e0123,
            e31: self.e02 * other.e0123,
            e23: self.e01 * other.e0123,
            e0123: self.s * other.e0123,
        }
    }
}

impl LeftInnerProduct<Null> for EvenMultivector {
    type Output = Null;
    fn left_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl LeftInnerProduct<OddMultivector> for EvenMultivector {
    type Output = OddMultivector;
    fn left_inner_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0 + self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.s * other.e1 + self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e2: self.s * other.e2 - self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e021: self.s * other.e021,
            e3: self.s * other.e3 + self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl LeftInnerProduct<EvenMultivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn left_inner_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e01: self.s * other.e01 - self.e23 * other.e0123,
            e02: self.s * other.e02 - self.e31 * other.e0123,
            e12: self.s * other.e12 + self.e03 * other.e0123,
            e03: self.s * other.e03 - self.e12 * other.e0123,
            e31: self.s * other.e31 + self.e02 * other.e0123,
            e23: self.s * other.e23 + self.e01 * other.e0123,
            e0123: self.s * other.e0123,
        }
    }
}

impl LeftInnerProduct<Multivector> for EvenMultivector {
    type Output = Multivector;
    fn left_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e0: self.s * other.e0 + self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.s * other.e1 + self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e01: self.s * other.e01 - self.e23 * other.e0123,
            e2: self.s * other.e2 - self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e02: self.s * other.e02 - self.e31 * other.e0123,
            e12: self.s * other.e12 + self.e03 * other.e0123,
            e021: self.s * other.e021,
            e3: self.s * other.e3 + self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e03: self.s * other.e03 - self.e12 * other.e0123,
            e31: self.s * other.e31 + self.e02 * other.e0123,
            e013: self.s * other.e013,
            e23: self.s * other.e23 + self.e01 * other.e0123,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
            e0123: self.s * other.e0123,
        }
    }
}

impl LeftInnerProduct<Scalar> for Multivector {
    type Output = Multivector;
    fn left_inner_product(self, other: Scalar) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<Vector> for Multivector {
    type Output = Multivector;
    fn left_inner_product(self, other: Vector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e01: 0.0,
            e2: self.s * other.e2,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: self.s * other.e3,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<Bivector> for Multivector {
    type Output = Multivector;
    fn left_inner_product(self, other: Bivector) -> Multivector {
        Multivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31,
            e01: self.s * other.e01,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e021: 0.0,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e013: 0.0,
            e23: self.s * other.e23,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<Trivector> for Multivector {
    type Output = Multivector;
    fn left_inner_product(self, other: Trivector) -> Multivector {
        Multivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e021: self.s * other.e021,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e013: self.s * other.e013,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<FourVector> for Multivector {
    type Output = Multivector;
    fn left_inner_product(self, other: FourVector) -> Multivector {
        Multivector {
            s: -self.e0123 * other.e0123,
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e01: -self.e23 * other.e0123,
            e2: -self.e013 * other.e0123,
            e02: -self.e31 * other.e0123,
            e12: self.e03 * other.e0123,
            e021: self.e3 * other.e0123,
            e3: -self.e021 * other.e0123,
            e03: -self.e12 * other.e0123,
            e31: self.e02 * other.e0123,
            e013: self.e2 * other.e0123,
            e23: self.e01 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
            e0123: self.s * other.e0123,
        }
    }
}

impl LeftInnerProduct<Null> for Multivector {
    type Output = Null;
    fn left_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl LeftInnerProduct<OddMultivector> for Multivector {
    type Output = Multivector;
    fn left_inner_product(self, other: OddMultivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.s * other.e0 + self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.s * other.e1 + self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e2: self.s * other.e2 - self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e021: self.s * other.e021,
            e3: self.s * other.e3 + self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e013: self.s * other.e013,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
            e0123: 0.0,
        }
    }
}

impl LeftInnerProduct<EvenMultivector> for Multivector {
    type Output = Multivector;
    fn left_inner_product(self, other: EvenMultivector) -> Multivector {
        Multivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03 + self.e123 * other.e0123,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31 - self.e032 * other.e0123,
            e01: self.s * other.e01 - self.e23 * other.e0123,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23 - self.e013 * other.e0123,
            e02: self.s * other.e02 - self.e31 * other.e0123,
            e12: self.s * other.e12 + self.e03 * other.e0123,
            e021: self.e3 * other.e0123,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 - self.e021 * other.e0123,
            e03: self.s * other.e03 - self.e12 * other.e0123,
            e31: self.s * other.e31 + self.e02 * other.e0123,
            e013: self.e2 * other.e0123,
            e23: self.s * other.e23 + self.e01 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
            e0123: self.s * other.e0123,
        }
    }
}

impl LeftInnerProduct<Multivector> for Multivector {
    type Output = Multivector;
    fn left_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s - self.e0 * other.e0 + self.e1 * other.e1 + self.e01 * other.e01 + self.e2 * other.e2 + self.e02 * other.e02 - self.e12 * other.e12 + self.e021 * other.e021 + self.e3 * other.e3 + self.e03 * other.e03 - self.e31 * other.e31 + self.e013 * other.e013 - self.e23 * other.e23 + self.e032 * other.e032 - self.e123 * other.e123 - self.e0123 * other.e0123,
            e0: self.s * other.e0 - self.e1 * other.e01 - self.e2 * other.e02 + self.e12 * other.e021 - self.e3 * other.e03 + self.e31 * other.e013 + self.e23 * other.e032 + self.e123 * other.e0123,
            e1: self.s * other.e1 - self.e0 * other.e01 - self.e2 * other.e12 + self.e02 * other.e021 + self.e3 * other.e31 - self.e03 * other.e013 - self.e23 * other.e123 - self.e032 * other.e0123,
            e01: self.s * other.e01 - self.e2 * other.e021 + self.e3 * other.e013 - self.e23 * other.e0123,
            e2: self.s * other.e2 - self.e0 * other.e02 + self.e1 * other.e12 - self.e01 * other.e021 - self.e3 * other.e23 + self.e03 * other.e032 - self.e31 * other.e123 - self.e013 * other.e0123,
            e02: self.s * other.e02 + self.e1 * other.e021 - self.e3 * other.e032 - self.e31 * other.e0123,
            e12: self.s * other.e12 + self.e0 * other.e021 + self.e3 * other.e123 + self.e03 * other.e0123,
            e021: self.s * other.e021 + self.e3 * other.e0123,
            e3: self.s * other.e3 - self.e0 * other.e03 - self.e1 * other.e31 + self.e01 * other.e013 + self.e2 * other.e23 - self.e02 * other.e032 - self.e12 * other.e123 - self.e021 * other.e0123,
            e03: self.s * other.e03 - self.e1 * other.e013 + self.e2 * other.e032 - self.e12 * other.e0123,
            e31: self.s * other.e31 + self.e0 * other.e013 + self.e2 * other.e123 + self.e02 * other.e0123,
            e013: self.s * other.e013 + self.e2 * other.e0123,
            e23: self.s * other.e23 + self.e0 * other.e032 + self.e1 * other.e123 + self.e01 * other.e0123,
            e032: self.s * other.e032 + self.e1 * other.e0123,
            e123: self.s * other.e123 - self.e0 * other.e0123,
            e0123: self.s * other.e0123,
        }
    }
}

impl RightInnerProduct<Scalar> for Scalar {
    type Output = Scalar;
    fn right_inner_product(self, other: Scalar) -> Scalar {
        Scalar {
            s: self.s * other.s,
        }
    }
}

impl RightInnerProduct<Vector> for Scalar {
    type Output = Null;
    fn right_inner_product(self, other: Vector) -> Null {
        Null
    }
}

impl RightInnerProduct<Bivector> for Scalar {
    type Output = Null;
    fn right_inner_product(self, other: Bivector) -> Null {
        Null
    }
}

impl RightInnerProduct<Trivector> for Scalar {
    type Output = Null;
    fn right_inner_product(self, other: Trivector) -> Null {
        Null
    }
}

impl RightInnerProduct<FourVector> for Scalar {
    type Output = Null;
    fn right_inner_product(self, other: FourVector) -> Null {
        Null
    }
}

impl RightInnerProduct<Null> for Scalar {
    type Output = Null;
    fn right_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl RightInnerProduct<OddMultivector> for Scalar {
    type Output = OddMultivector;
    fn right_inner_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl RightInnerProduct<EvenMultivector> for Scalar {
    type Output = EvenMultivector;
    fn right_inner_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<Multivector> for Scalar {
    type Output = Multivector;
    fn right_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<Scalar> for Vector {
    type Output = Vector;
    fn right_inner_product(self, other: Scalar) -> Vector {
        Vector {
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e2: self.e2 * other.s,
            e3: self.e3 * other.s,
        }
    }
}

impl RightInnerProduct<Vector> for Vector {
    type Output = Scalar;
    fn right_inner_product(self, other: Vector) -> Scalar {
        Scalar {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
        }
    }
}

impl RightInnerProduct<Bivector> for Vector {
    type Output = Null;
    fn right_inner_product(self, other: Bivector) -> Null {
        Null
    }
}

impl RightInnerProduct<Trivector> for Vector {
    type Output = Null;
    fn right_inner_product(self, other: Trivector) -> Null {
        Null
    }
}

impl RightInnerProduct<FourVector> for Vector {
    type Output = Null;
    fn right_inner_product(self, other: FourVector) -> Null {
        Null
    }
}

impl RightInnerProduct<Null> for Vector {
    type Output = Null;
    fn right_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl RightInnerProduct<OddMultivector> for Vector {
    type Output = EvenMultivector;
    fn right_inner_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<EvenMultivector> for Vector {
    type Output = OddMultivector;
    fn right_inner_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e2: self.e2 * other.s,
            e021: 0.0,
            e3: self.e3 * other.s,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl RightInnerProduct<Multivector> for Vector {
    type Output = Multivector;
    fn right_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e01: 0.0,
            e2: self.e2 * other.s,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: self.e3 * other.s,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<Scalar> for Bivector {
    type Output = Bivector;
    fn right_inner_product(self, other: Scalar) -> Bivector {
        Bivector {
            e01: self.e01 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e23: self.e23 * other.s,
        }
    }
}

impl RightInnerProduct<Vector> for Bivector {
    type Output = Vector;
    fn right_inner_product(self, other: Vector) -> Vector {
        Vector {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
        }
    }
}

impl RightInnerProduct<Bivector> for Bivector {
    type Output = Scalar;
    fn right_inner_product(self, other: Bivector) -> Scalar {
        Scalar {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
        }
    }
}

impl RightInnerProduct<Trivector> for Bivector {
    type Output = Null;
    fn right_inner_product(self, other: Trivector) -> Null {
        Null
    }
}

impl RightInnerProduct<FourVector> for Bivector {
    type Output = Null;
    fn right_inner_product(self, other: FourVector) -> Null {
        Null
    }
}

impl RightInnerProduct<Null> for Bivector {
    type Output = Null;
    fn right_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl RightInnerProduct<OddMultivector> for Bivector {
    type Output = OddMultivector;
    fn right_inner_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e021: 0.0,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl RightInnerProduct<EvenMultivector> for Bivector {
    type Output = EvenMultivector;
    fn right_inner_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e01: self.e01 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e23: self.e23 * other.s,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<Multivector> for Bivector {
    type Output = Multivector;
    fn right_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e01: self.e01 * other.s,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e021: 0.0,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e013: 0.0,
            e23: self.e23 * other.s,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<Scalar> for Trivector {
    type Output = Trivector;
    fn right_inner_product(self, other: Scalar) -> Trivector {
        Trivector {
            e021: self.e021 * other.s,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl RightInnerProduct<Vector> for Trivector {
    type Output = Bivector;
    fn right_inner_product(self, other: Vector) -> Bivector {
        Bivector {
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
        }
    }
}

impl RightInnerProduct<Bivector> for Trivector {
    type Output = Vector;
    fn right_inner_product(self, other: Bivector) -> Vector {
        Vector {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e2: -self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e3: self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
        }
    }
}

impl RightInnerProduct<Trivector> for Trivector {
    type Output = Scalar;
    fn right_inner_product(self, other: Trivector) -> Scalar {
        Scalar {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
        }
    }
}

impl RightInnerProduct<FourVector> for Trivector {
    type Output = Null;
    fn right_inner_product(self, other: FourVector) -> Null {
        Null
    }
}

impl RightInnerProduct<Null> for Trivector {
    type Output = Null;
    fn right_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl RightInnerProduct<OddMultivector> for Trivector {
    type Output = EvenMultivector;
    fn right_inner_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<EvenMultivector> for Trivector {
    type Output = OddMultivector;
    fn right_inner_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e2: -self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e021: self.e021 * other.s,
            e3: self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl RightInnerProduct<Multivector> for Trivector {
    type Output = Multivector;
    fn right_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e2: -self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e021: self.e021 * other.s,
            e3: self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e013: self.e013 * other.s,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<Scalar> for FourVector {
    type Output = FourVector;
    fn right_inner_product(self, other: Scalar) -> FourVector {
        FourVector {
            e0123: self.e0123 * other.s,
        }
    }
}

impl RightInnerProduct<Vector> for FourVector {
    type Output = Trivector;
    fn right_inner_product(self, other: Vector) -> Trivector {
        Trivector {
            e021: -self.e0123 * other.e3,
            e013: -self.e0123 * other.e2,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
        }
    }
}

impl RightInnerProduct<Bivector> for FourVector {
    type Output = Bivector;
    fn right_inner_product(self, other: Bivector) -> Bivector {
        Bivector {
            e01: -self.e0123 * other.e23,
            e02: -self.e0123 * other.e31,
            e12: self.e0123 * other.e03,
            e03: -self.e0123 * other.e12,
            e31: self.e0123 * other.e02,
            e23: self.e0123 * other.e01,
        }
    }
}

impl RightInnerProduct<Trivector> for FourVector {
    type Output = Vector;
    fn right_inner_product(self, other: Trivector) -> Vector {
        Vector {
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e2: self.e0123 * other.e013,
            e3: self.e0123 * other.e021,
        }
    }
}

impl RightInnerProduct<FourVector> for FourVector {
    type Output = Scalar;
    fn right_inner_product(self, other: FourVector) -> Scalar {
        Scalar {
            s: -self.e0123 * other.e0123,
        }
    }
}

impl RightInnerProduct<Null> for FourVector {
    type Output = Null;
    fn right_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl RightInnerProduct<OddMultivector> for FourVector {
    type Output = OddMultivector;
    fn right_inner_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e2: self.e0123 * other.e013,
            e021: -self.e0123 * other.e3,
            e3: self.e0123 * other.e021,
            e013: -self.e0123 * other.e2,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
        }
    }
}

impl RightInnerProduct<EvenMultivector> for FourVector {
    type Output = EvenMultivector;
    fn right_inner_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0123 * other.e0123,
            e01: -self.e0123 * other.e23,
            e02: -self.e0123 * other.e31,
            e12: self.e0123 * other.e03,
            e03: -self.e0123 * other.e12,
            e31: self.e0123 * other.e02,
            e23: self.e0123 * other.e01,
            e0123: self.e0123 * other.s,
        }
    }
}

impl RightInnerProduct<Multivector> for FourVector {
    type Output = Multivector;
    fn right_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0123 * other.e0123,
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e01: -self.e0123 * other.e23,
            e2: self.e0123 * other.e013,
            e02: -self.e0123 * other.e31,
            e12: self.e0123 * other.e03,
            e021: -self.e0123 * other.e3,
            e3: self.e0123 * other.e021,
            e03: -self.e0123 * other.e12,
            e31: self.e0123 * other.e02,
            e013: -self.e0123 * other.e2,
            e23: self.e0123 * other.e01,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
            e0123: self.e0123 * other.s,
        }
    }
}

impl RightInnerProduct<Scalar> for Null {
    type Output = Null;
    fn right_inner_product(self, other: Scalar) -> Null {
        Null
    }
}

impl RightInnerProduct<Vector> for Null {
    type Output = Null;
    fn right_inner_product(self, other: Vector) -> Null {
        Null
    }
}

impl RightInnerProduct<Bivector> for Null {
    type Output = Null;
    fn right_inner_product(self, other: Bivector) -> Null {
        Null
    }
}

impl RightInnerProduct<Trivector> for Null {
    type Output = Null;
    fn right_inner_product(self, other: Trivector) -> Null {
        Null
    }
}

impl RightInnerProduct<FourVector> for Null {
    type Output = Null;
    fn right_inner_product(self, other: FourVector) -> Null {
        Null
    }
}

impl RightInnerProduct<Null> for Null {
    type Output = Null;
    fn right_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl RightInnerProduct<OddMultivector> for Null {
    type Output = Null;
    fn right_inner_product(self, other: OddMultivector) -> Null {
        Null
    }
}

impl RightInnerProduct<EvenMultivector> for Null {
    type Output = Null;
    fn right_inner_product(self, other: EvenMultivector) -> Null {
        Null
    }
}

impl RightInnerProduct<Multivector> for Null {
    type Output = Null;
    fn right_inner_product(self, other: Multivector) -> Null {
        Null
    }
}

impl RightInnerProduct<Scalar> for OddMultivector {
    type Output = OddMultivector;
    fn right_inner_product(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e2: self.e2 * other.s,
            e021: self.e021 * other.s,
            e3: self.e3 * other.s,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl RightInnerProduct<Vector> for OddMultivector {
    type Output = EvenMultivector;
    fn right_inner_product(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<Bivector> for OddMultivector {
    type Output = OddMultivector;
    fn right_inner_product(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e2: -self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e021: 0.0,
            e3: self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl RightInnerProduct<Trivector> for OddMultivector {
    type Output = EvenMultivector;
    fn right_inner_product(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<FourVector> for OddMultivector {
    type Output = OddMultivector;
    fn right_inner_product(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl RightInnerProduct<Null> for OddMultivector {
    type Output = Null;
    fn right_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl RightInnerProduct<OddMultivector> for OddMultivector {
    type Output = EvenMultivector;
    fn right_inner_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<EvenMultivector> for OddMultivector {
    type Output = OddMultivector;
    fn right_inner_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s + self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e1 * other.s + self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e2: self.e2 * other.s - self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e021: self.e021 * other.s,
            e3: self.e3 * other.s + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl RightInnerProduct<Multivector> for OddMultivector {
    type Output = Multivector;
    fn right_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.e0 * other.s + self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e1 * other.s + self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e2: self.e2 * other.s - self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e021: self.e021 * other.s,
            e3: self.e3 * other.s + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e013: self.e013 * other.s,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<Scalar> for EvenMultivector {
    type Output = EvenMultivector;
    fn right_inner_product(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: self.e01 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e23: self.e23 * other.s,
            e0123: self.e0123 * other.s,
        }
    }
}

impl RightInnerProduct<Vector> for EvenMultivector {
    type Output = OddMultivector;
    fn right_inner_product(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e021: -self.e0123 * other.e3,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e013: -self.e0123 * other.e2,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
        }
    }
}

impl RightInnerProduct<Bivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn right_inner_product(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e01: -self.e0123 * other.e23,
            e02: -self.e0123 * other.e31,
            e12: self.e0123 * other.e03,
            e03: -self.e0123 * other.e12,
            e31: self.e0123 * other.e02,
            e23: self.e0123 * other.e01,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<Trivector> for EvenMultivector {
    type Output = OddMultivector;
    fn right_inner_product(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e2: self.e0123 * other.e013,
            e021: 0.0,
            e3: self.e0123 * other.e021,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl RightInnerProduct<FourVector> for EvenMultivector {
    type Output = EvenMultivector;
    fn right_inner_product(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0123 * other.e0123,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<Null> for EvenMultivector {
    type Output = Null;
    fn right_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl RightInnerProduct<OddMultivector> for EvenMultivector {
    type Output = OddMultivector;
    fn right_inner_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3 - self.e0123 * other.e123,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3 + self.e0123 * other.e032,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3 + self.e0123 * other.e013,
            e021: -self.e0123 * other.e3,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2 + self.e0123 * other.e021,
            e013: -self.e0123 * other.e2,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
        }
    }
}

impl RightInnerProduct<EvenMultivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn right_inner_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e01: self.e01 * other.s - self.e0123 * other.e23,
            e02: self.e02 * other.s - self.e0123 * other.e31,
            e12: self.e12 * other.s + self.e0123 * other.e03,
            e03: self.e03 * other.s - self.e0123 * other.e12,
            e31: self.e31 * other.s + self.e0123 * other.e02,
            e23: self.e23 * other.s + self.e0123 * other.e01,
            e0123: self.e0123 * other.s,
        }
    }
}

impl RightInnerProduct<Multivector> for EvenMultivector {
    type Output = Multivector;
    fn right_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3 - self.e0123 * other.e123,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3 + self.e0123 * other.e032,
            e01: self.e01 * other.s - self.e0123 * other.e23,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3 + self.e0123 * other.e013,
            e02: self.e02 * other.s - self.e0123 * other.e31,
            e12: self.e12 * other.s + self.e0123 * other.e03,
            e021: -self.e0123 * other.e3,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2 + self.e0123 * other.e021,
            e03: self.e03 * other.s - self.e0123 * other.e12,
            e31: self.e31 * other.s + self.e0123 * other.e02,
            e013: -self.e0123 * other.e2,
            e23: self.e23 * other.s + self.e0123 * other.e01,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
            e0123: self.e0123 * other.s,
        }
    }
}

impl RightInnerProduct<Scalar> for Multivector {
    type Output = Multivector;
    fn right_inner_product(self, other: Scalar) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e01: self.e01 * other.s,
            e2: self.e2 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e021: self.e021 * other.s,
            e3: self.e3 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e013: self.e013 * other.s,
            e23: self.e23 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
            e0123: self.e0123 * other.s,
        }
    }
}

impl RightInnerProduct<Vector> for Multivector {
    type Output = Multivector;
    fn right_inner_product(self, other: Vector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e021: -self.e0123 * other.e3,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e013: -self.e0123 * other.e2,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<Bivector> for Multivector {
    type Output = Multivector;
    fn right_inner_product(self, other: Bivector) -> Multivector {
        Multivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e01: -self.e0123 * other.e23,
            e2: -self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: -self.e0123 * other.e31,
            e12: self.e0123 * other.e03,
            e021: 0.0,
            e3: self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: -self.e0123 * other.e12,
            e31: self.e0123 * other.e02,
            e013: 0.0,
            e23: self.e0123 * other.e01,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<Trivector> for Multivector {
    type Output = Multivector;
    fn right_inner_product(self, other: Trivector) -> Multivector {
        Multivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e01: 0.0,
            e2: self.e0123 * other.e013,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: self.e0123 * other.e021,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<FourVector> for Multivector {
    type Output = Multivector;
    fn right_inner_product(self, other: FourVector) -> Multivector {
        Multivector {
            s: -self.e0123 * other.e0123,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<Null> for Multivector {
    type Output = Null;
    fn right_inner_product(self, other: Null) -> Null {
        Null
    }
}

impl RightInnerProduct<OddMultivector> for Multivector {
    type Output = Multivector;
    fn right_inner_product(self, other: OddMultivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3 - self.e0123 * other.e123,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3 + self.e0123 * other.e032,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3 + self.e0123 * other.e013,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e021: -self.e0123 * other.e3,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2 + self.e0123 * other.e021,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e013: -self.e0123 * other.e2,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
            e0123: 0.0,
        }
    }
}

impl RightInnerProduct<EvenMultivector> for Multivector {
    type Output = Multivector;
    fn right_inner_product(self, other: EvenMultivector) -> Multivector {
        Multivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e0: self.e0 * other.s + self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e1 * other.s + self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e01: self.e01 * other.s - self.e0123 * other.e23,
            e2: self.e2 * other.s - self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.e02 * other.s - self.e0123 * other.e31,
            e12: self.e12 * other.s + self.e0123 * other.e03,
            e021: self.e021 * other.s,
            e3: self.e3 * other.s + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: self.e03 * other.s - self.e0123 * other.e12,
            e31: self.e31 * other.s + self.e0123 * other.e02,
            e013: self.e013 * other.s,
            e23: self.e23 * other.s + self.e0123 * other.e01,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
            e0123: self.e0123 * other.s,
        }
    }
}

impl RightInnerProduct<Multivector> for Multivector {
    type Output = Multivector;
    fn right_inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s - self.e0 * other.e0 + self.e1 * other.e1 + self.e01 * other.e01 + self.e2 * other.e2 + self.e02 * other.e02 - self.e12 * other.e12 + self.e021 * other.e021 + self.e3 * other.e3 + self.e03 * other.e03 - self.e31 * other.e31 + self.e013 * other.e013 - self.e23 * other.e23 + self.e032 * other.e032 - self.e123 * other.e123 - self.e0123 * other.e0123,
            e0: self.e0 * other.s + self.e01 * other.e1 + self.e02 * other.e2 + self.e021 * other.e12 + self.e03 * other.e3 + self.e013 * other.e31 + self.e032 * other.e23 - self.e0123 * other.e123,
            e1: self.e1 * other.s + self.e01 * other.e0 + self.e12 * other.e2 + self.e021 * other.e02 - self.e31 * other.e3 - self.e013 * other.e03 - self.e123 * other.e23 + self.e0123 * other.e032,
            e01: self.e01 * other.s - self.e021 * other.e2 + self.e013 * other.e3 - self.e0123 * other.e23,
            e2: self.e2 * other.s + self.e02 * other.e0 - self.e12 * other.e1 - self.e021 * other.e01 + self.e23 * other.e3 + self.e032 * other.e03 - self.e123 * other.e31 + self.e0123 * other.e013,
            e02: self.e02 * other.s + self.e021 * other.e1 - self.e032 * other.e3 - self.e0123 * other.e31,
            e12: self.e12 * other.s + self.e021 * other.e0 + self.e123 * other.e3 + self.e0123 * other.e03,
            e021: self.e021 * other.s - self.e0123 * other.e3,
            e3: self.e3 * other.s + self.e03 * other.e0 + self.e31 * other.e1 + self.e013 * other.e01 - self.e23 * other.e2 - self.e032 * other.e02 - self.e123 * other.e12 + self.e0123 * other.e021,
            e03: self.e03 * other.s - self.e013 * other.e1 + self.e032 * other.e2 - self.e0123 * other.e12,
            e31: self.e31 * other.s + self.e013 * other.e0 + self.e123 * other.e2 + self.e0123 * other.e02,
            e013: self.e013 * other.s - self.e0123 * other.e2,
            e23: self.e23 * other.s + self.e032 * other.e0 + self.e123 * other.e1 + self.e0123 * other.e01,
            e032: self.e032 * other.s - self.e0123 * other.e1,
            e123: self.e123 * other.s + self.e0123 * other.e0,
            e0123: self.e0123 * other.s,
        }
    }
}

impl InnerProduct<Scalar> for Scalar {
    type Output = Scalar;
    fn inner_product(self, other: Scalar) -> Scalar {
        Scalar {
            s: self.s * other.s,
        }
    }
}

impl InnerProduct<Vector> for Scalar {
    type Output = Vector;
    fn inner_product(self, other: Vector) -> Vector {
        Vector {
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e2: self.s * other.e2,
            e3: self.s * other.e3,
        }
    }
}

impl InnerProduct<Bivector> for Scalar {
    type Output = Bivector;
    fn inner_product(self, other: Bivector) -> Bivector {
        Bivector {
            e01: self.s * other.e01,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e23: self.s * other.e23,
        }
    }
}

impl InnerProduct<Trivector> for Scalar {
    type Output = Trivector;
    fn inner_product(self, other: Trivector) -> Trivector {
        Trivector {
            e021: self.s * other.e021,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl InnerProduct<FourVector> for Scalar {
    type Output = FourVector;
    fn inner_product(self, other: FourVector) -> FourVector {
        FourVector {
            e0123: self.s * other.e0123,
        }
    }
}

impl InnerProduct<Null> for Scalar {
    type Output = Null;
    fn inner_product(self, other: Null) -> Null {
        Null
    }
}

impl InnerProduct<OddMultivector> for Scalar {
    type Output = OddMultivector;
    fn inner_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e2: self.s * other.e2,
            e021: self.s * other.e021,
            e3: self.s * other.e3,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl InnerProduct<EvenMultivector> for Scalar {
    type Output = EvenMultivector;
    fn inner_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: self.s * other.e01,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e23: self.s * other.e23,
            e0123: self.s * other.e0123,
        }
    }
}

impl InnerProduct<Multivector> for Scalar {
    type Output = Multivector;
    fn inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e01: self.s * other.e01,
            e2: self.s * other.e2,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e021: self.s * other.e021,
            e3: self.s * other.e3,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e013: self.s * other.e013,
            e23: self.s * other.e23,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
            e0123: self.s * other.e0123,
        }
    }
}

impl InnerProduct<Scalar> for Vector {
    type Output = Vector;
    fn inner_product(self, other: Scalar) -> Vector {
        Vector {
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e2: self.e2 * other.s,
            e3: self.e3 * other.s,
        }
    }
}

impl InnerProduct<Vector> for Vector {
    type Output = Scalar;
    fn inner_product(self, other: Vector) -> Scalar {
        Scalar {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
        }
    }
}

impl InnerProduct<Bivector> for Vector {
    type Output = Vector;
    fn inner_product(self, other: Bivector) -> Vector {
        Vector {
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23,
        }
    }
}

impl InnerProduct<Trivector> for Vector {
    type Output = Bivector;
    fn inner_product(self, other: Trivector) -> Bivector {
        Bivector {
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
        }
    }
}

impl InnerProduct<FourVector> for Vector {
    type Output = Trivector;
    fn inner_product(self, other: FourVector) -> Trivector {
        Trivector {
            e021: self.e3 * other.e0123,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
        }
    }
}

impl InnerProduct<Null> for Vector {
    type Output = Null;
    fn inner_product(self, other: Null) -> Null {
        Null
    }
}

impl InnerProduct<OddMultivector> for Vector {
    type Output = EvenMultivector;
    fn inner_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<EvenMultivector> for Vector {
    type Output = OddMultivector;
    fn inner_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s - self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 + self.e1 * other.s - self.e2 * other.e12 + self.e3 * other.e31,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 + self.e2 * other.s - self.e3 * other.e23,
            e021: self.e3 * other.e0123,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 + self.e3 * other.s,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
        }
    }
}

impl InnerProduct<Multivector> for Vector {
    type Output = Multivector;
    fn inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e0: self.e0 * other.s - self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 + self.e1 * other.s - self.e2 * other.e12 + self.e3 * other.e31,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 + self.e2 * other.s - self.e3 * other.e23,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e021: self.e3 * other.e0123,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 + self.e3 * other.s,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e013: self.e2 * other.e0123,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<Scalar> for Bivector {
    type Output = Bivector;
    fn inner_product(self, other: Scalar) -> Bivector {
        Bivector {
            e01: self.e01 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e23: self.e23 * other.s,
        }
    }
}

impl InnerProduct<Vector> for Bivector {
    type Output = Vector;
    fn inner_product(self, other: Vector) -> Vector {
        Vector {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
        }
    }
}

impl InnerProduct<Bivector> for Bivector {
    type Output = Scalar;
    fn inner_product(self, other: Bivector) -> Scalar {
        Scalar {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
        }
    }
}

impl InnerProduct<Trivector> for Bivector {
    type Output = Vector;
    fn inner_product(self, other: Trivector) -> Vector {
        Vector {
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
        }
    }
}

impl InnerProduct<FourVector> for Bivector {
    type Output = Bivector;
    fn inner_product(self, other: FourVector) -> Bivector {
        Bivector {
            e01: -self.e23 * other.e0123,
            e02: -self.e31 * other.e0123,
            e12: self.e03 * other.e0123,
            e03: -self.e12 * other.e0123,
            e31: self.e02 * other.e0123,
            e23: self.e01 * other.e0123,
        }
    }
}

impl InnerProduct<Null> for Bivector {
    type Output = Null;
    fn inner_product(self, other: Null) -> Null {
        Null
    }
}

impl InnerProduct<OddMultivector> for Bivector {
    type Output = OddMultivector;
    fn inner_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e12 * other.e021 + self.e03 * other.e3 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e01 * other.e0 + self.e02 * other.e021 + self.e12 * other.e2 - self.e03 * other.e013 - self.e31 * other.e3 - self.e23 * other.e123,
            e2: -self.e01 * other.e021 + self.e02 * other.e0 - self.e12 * other.e1 + self.e03 * other.e032 - self.e31 * other.e123 + self.e23 * other.e3,
            e021: 0.0,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl InnerProduct<EvenMultivector> for Bivector {
    type Output = EvenMultivector;
    fn inner_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e01: self.e01 * other.s - self.e23 * other.e0123,
            e02: self.e02 * other.s - self.e31 * other.e0123,
            e12: self.e12 * other.s + self.e03 * other.e0123,
            e03: -self.e12 * other.e0123 + self.e03 * other.s,
            e31: self.e02 * other.e0123 + self.e31 * other.s,
            e23: self.e01 * other.e0123 + self.e23 * other.s,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<Multivector> for Bivector {
    type Output = Multivector;
    fn inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e12 * other.e021 + self.e03 * other.e3 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e01 * other.e0 + self.e02 * other.e021 + self.e12 * other.e2 - self.e03 * other.e013 - self.e31 * other.e3 - self.e23 * other.e123,
            e01: self.e01 * other.s - self.e23 * other.e0123,
            e2: -self.e01 * other.e021 + self.e02 * other.e0 - self.e12 * other.e1 + self.e03 * other.e032 - self.e31 * other.e123 + self.e23 * other.e3,
            e02: self.e02 * other.s - self.e31 * other.e0123,
            e12: self.e12 * other.s + self.e03 * other.e0123,
            e021: 0.0,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e03: -self.e12 * other.e0123 + self.e03 * other.s,
            e31: self.e02 * other.e0123 + self.e31 * other.s,
            e013: 0.0,
            e23: self.e01 * other.e0123 + self.e23 * other.s,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<Scalar> for Trivector {
    type Output = Trivector;
    fn inner_product(self, other: Scalar) -> Trivector {
        Trivector {
            e021: self.e021 * other.s,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl InnerProduct<Vector> for Trivector {
    type Output = Bivector;
    fn inner_product(self, other: Vector) -> Bivector {
        Bivector {
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
        }
    }
}

impl InnerProduct<Bivector> for Trivector {
    type Output = Vector;
    fn inner_product(self, other: Bivector) -> Vector {
        Vector {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e2: -self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e3: self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
        }
    }
}

impl InnerProduct<Trivector> for Trivector {
    type Output = Scalar;
    fn inner_product(self, other: Trivector) -> Scalar {
        Scalar {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
        }
    }
}

impl InnerProduct<FourVector> for Trivector {
    type Output = Vector;
    fn inner_product(self, other: FourVector) -> Vector {
        Vector {
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e2: -self.e013 * other.e0123,
            e3: -self.e021 * other.e0123,
        }
    }
}

impl InnerProduct<Null> for Trivector {
    type Output = Null;
    fn inner_product(self, other: Null) -> Null {
        Null
    }
}

impl InnerProduct<OddMultivector> for Trivector {
    type Output = EvenMultivector;
    fn inner_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<EvenMultivector> for Trivector {
    type Output = OddMultivector;
    fn inner_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23 + self.e123 * other.e0123,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e032 * other.e0123 - self.e123 * other.e23,
            e2: -self.e021 * other.e01 - self.e013 * other.e0123 + self.e032 * other.e03 - self.e123 * other.e31,
            e021: self.e021 * other.s,
            e3: -self.e021 * other.e0123 + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl InnerProduct<Multivector> for Trivector {
    type Output = Multivector;
    fn inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23 + self.e123 * other.e0123,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e032 * other.e0123 - self.e123 * other.e23,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e2: -self.e021 * other.e01 - self.e013 * other.e0123 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e021: self.e021 * other.s,
            e3: -self.e021 * other.e0123 + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e013: self.e013 * other.s,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<Scalar> for FourVector {
    type Output = FourVector;
    fn inner_product(self, other: Scalar) -> FourVector {
        FourVector {
            e0123: self.e0123 * other.s,
        }
    }
}

impl InnerProduct<Vector> for FourVector {
    type Output = Trivector;
    fn inner_product(self, other: Vector) -> Trivector {
        Trivector {
            e021: -self.e0123 * other.e3,
            e013: -self.e0123 * other.e2,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
        }
    }
}

impl InnerProduct<Bivector> for FourVector {
    type Output = Bivector;
    fn inner_product(self, other: Bivector) -> Bivector {
        Bivector {
            e01: -self.e0123 * other.e23,
            e02: -self.e0123 * other.e31,
            e12: self.e0123 * other.e03,
            e03: -self.e0123 * other.e12,
            e31: self.e0123 * other.e02,
            e23: self.e0123 * other.e01,
        }
    }
}

impl InnerProduct<Trivector> for FourVector {
    type Output = Vector;
    fn inner_product(self, other: Trivector) -> Vector {
        Vector {
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e2: self.e0123 * other.e013,
            e3: self.e0123 * other.e021,
        }
    }
}

impl InnerProduct<FourVector> for FourVector {
    type Output = Scalar;
    fn inner_product(self, other: FourVector) -> Scalar {
        Scalar {
            s: -self.e0123 * other.e0123,
        }
    }
}

impl InnerProduct<Null> for FourVector {
    type Output = Null;
    fn inner_product(self, other: Null) -> Null {
        Null
    }
}

impl InnerProduct<OddMultivector> for FourVector {
    type Output = OddMultivector;
    fn inner_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e2: self.e0123 * other.e013,
            e021: -self.e0123 * other.e3,
            e3: self.e0123 * other.e021,
            e013: -self.e0123 * other.e2,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
        }
    }
}

impl InnerProduct<EvenMultivector> for FourVector {
    type Output = EvenMultivector;
    fn inner_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0123 * other.e0123,
            e01: -self.e0123 * other.e23,
            e02: -self.e0123 * other.e31,
            e12: self.e0123 * other.e03,
            e03: -self.e0123 * other.e12,
            e31: self.e0123 * other.e02,
            e23: self.e0123 * other.e01,
            e0123: self.e0123 * other.s,
        }
    }
}

impl InnerProduct<Multivector> for FourVector {
    type Output = Multivector;
    fn inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0123 * other.e0123,
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e01: -self.e0123 * other.e23,
            e2: self.e0123 * other.e013,
            e02: -self.e0123 * other.e31,
            e12: self.e0123 * other.e03,
            e021: -self.e0123 * other.e3,
            e3: self.e0123 * other.e021,
            e03: -self.e0123 * other.e12,
            e31: self.e0123 * other.e02,
            e013: -self.e0123 * other.e2,
            e23: self.e0123 * other.e01,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
            e0123: self.e0123 * other.s,
        }
    }
}

impl InnerProduct<Scalar> for Null {
    type Output = Null;
    fn inner_product(self, other: Scalar) -> Null {
        Null
    }
}

impl InnerProduct<Vector> for Null {
    type Output = Null;
    fn inner_product(self, other: Vector) -> Null {
        Null
    }
}

impl InnerProduct<Bivector> for Null {
    type Output = Null;
    fn inner_product(self, other: Bivector) -> Null {
        Null
    }
}

impl InnerProduct<Trivector> for Null {
    type Output = Null;
    fn inner_product(self, other: Trivector) -> Null {
        Null
    }
}

impl InnerProduct<FourVector> for Null {
    type Output = Null;
    fn inner_product(self, other: FourVector) -> Null {
        Null
    }
}

impl InnerProduct<Null> for Null {
    type Output = Null;
    fn inner_product(self, other: Null) -> Null {
        Null
    }
}

impl InnerProduct<OddMultivector> for Null {
    type Output = Null;
    fn inner_product(self, other: OddMultivector) -> Null {
        Null
    }
}

impl InnerProduct<EvenMultivector> for Null {
    type Output = Null;
    fn inner_product(self, other: EvenMultivector) -> Null {
        Null
    }
}

impl InnerProduct<Multivector> for Null {
    type Output = Null;
    fn inner_product(self, other: Multivector) -> Null {
        Null
    }
}

impl InnerProduct<Scalar> for OddMultivector {
    type Output = OddMultivector;
    fn inner_product(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e2: self.e2 * other.s,
            e021: self.e021 * other.s,
            e3: self.e3 * other.s,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl InnerProduct<Vector> for OddMultivector {
    type Output = EvenMultivector;
    fn inner_product(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<Bivector> for OddMultivector {
    type Output = OddMultivector;
    fn inner_product(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e1 * other.e01 - self.e2 * other.e02 + self.e021 * other.e12 - self.e3 * other.e03 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e021 * other.e02 + self.e3 * other.e31 - self.e013 * other.e03 - self.e123 * other.e23,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e021 * other.e01 - self.e3 * other.e23 + self.e032 * other.e03 - self.e123 * other.e31,
            e021: 0.0,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl InnerProduct<Trivector> for OddMultivector {
    type Output = EvenMultivector;
    fn inner_product(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<FourVector> for OddMultivector {
    type Output = OddMultivector;
    fn inner_product(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e2: -self.e013 * other.e0123,
            e021: self.e3 * other.e0123,
            e3: -self.e021 * other.e0123,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
        }
    }
}

impl InnerProduct<Null> for OddMultivector {
    type Output = Null;
    fn inner_product(self, other: Null) -> Null {
        Null
    }
}

impl InnerProduct<OddMultivector> for OddMultivector {
    type Output = EvenMultivector;
    fn inner_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: -self.e2 * other.e021 - self.e021 * other.e2 + self.e3 * other.e013 + self.e013 * other.e3,
            e02: self.e1 * other.e021 + self.e021 * other.e1 - self.e3 * other.e032 - self.e032 * other.e3,
            e12: self.e0 * other.e021 + self.e021 * other.e0 + self.e3 * other.e123 + self.e123 * other.e3,
            e03: -self.e1 * other.e013 + self.e2 * other.e032 - self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e0 * other.e013 + self.e2 * other.e123 + self.e013 * other.e0 + self.e123 * other.e2,
            e23: self.e0 * other.e032 + self.e1 * other.e123 + self.e032 * other.e0 + self.e123 * other.e1,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<EvenMultivector> for OddMultivector {
    type Output = OddMultivector;
    fn inner_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s - self.e1 * other.e01 - self.e2 * other.e02 + self.e021 * other.e12 - self.e3 * other.e03 + self.e013 * other.e31 + self.e032 * other.e23 + self.e123 * other.e0123,
            e1: -self.e0 * other.e01 + self.e1 * other.s - self.e2 * other.e12 + self.e021 * other.e02 + self.e3 * other.e31 - self.e013 * other.e03 - self.e032 * other.e0123 - self.e123 * other.e23,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 + self.e2 * other.s - self.e021 * other.e01 - self.e3 * other.e23 - self.e013 * other.e0123 + self.e032 * other.e03 - self.e123 * other.e31,
            e021: self.e021 * other.s + self.e3 * other.e0123,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 - self.e021 * other.e0123 + self.e3 * other.s + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e013: self.e2 * other.e0123 + self.e013 * other.s,
            e032: self.e1 * other.e0123 + self.e032 * other.s,
            e123: -self.e0 * other.e0123 + self.e123 * other.s,
        }
    }
}

impl InnerProduct<Multivector> for OddMultivector {
    type Output = Multivector;
    fn inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.e0 * other.s - self.e1 * other.e01 - self.e2 * other.e02 + self.e021 * other.e12 - self.e3 * other.e03 + self.e013 * other.e31 + self.e032 * other.e23 + self.e123 * other.e0123,
            e1: -self.e0 * other.e01 + self.e1 * other.s - self.e2 * other.e12 + self.e021 * other.e02 + self.e3 * other.e31 - self.e013 * other.e03 - self.e032 * other.e0123 - self.e123 * other.e23,
            e01: -self.e2 * other.e021 - self.e021 * other.e2 + self.e3 * other.e013 + self.e013 * other.e3,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 + self.e2 * other.s - self.e021 * other.e01 - self.e3 * other.e23 - self.e013 * other.e0123 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.e1 * other.e021 + self.e021 * other.e1 - self.e3 * other.e032 - self.e032 * other.e3,
            e12: self.e0 * other.e021 + self.e021 * other.e0 + self.e3 * other.e123 + self.e123 * other.e3,
            e021: self.e021 * other.s + self.e3 * other.e0123,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 - self.e021 * other.e0123 + self.e3 * other.s + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: -self.e1 * other.e013 + self.e2 * other.e032 - self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e0 * other.e013 + self.e2 * other.e123 + self.e013 * other.e0 + self.e123 * other.e2,
            e013: self.e2 * other.e0123 + self.e013 * other.s,
            e23: self.e0 * other.e032 + self.e1 * other.e123 + self.e032 * other.e0 + self.e123 * other.e1,
            e032: self.e1 * other.e0123 + self.e032 * other.s,
            e123: -self.e0 * other.e0123 + self.e123 * other.s,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<Scalar> for EvenMultivector {
    type Output = EvenMultivector;
    fn inner_product(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: self.e01 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e23: self.e23 * other.s,
            e0123: self.e0123 * other.s,
        }
    }
}

impl InnerProduct<Vector> for EvenMultivector {
    type Output = OddMultivector;
    fn inner_product(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0 + self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.s * other.e1 + self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e2: self.s * other.e2 + self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e021: -self.e0123 * other.e3,
            e3: self.s * other.e3 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e013: -self.e0123 * other.e2,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
        }
    }
}

impl InnerProduct<Bivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn inner_product(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e01: self.s * other.e01 - self.e0123 * other.e23,
            e02: self.s * other.e02 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e0123 * other.e03,
            e03: self.s * other.e03 - self.e0123 * other.e12,
            e31: self.s * other.e31 + self.e0123 * other.e02,
            e23: self.s * other.e23 + self.e0123 * other.e01,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<Trivector> for EvenMultivector {
    type Output = OddMultivector;
    fn inner_product(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032 - self.e0123 * other.e123,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123 + self.e0123 * other.e032,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123 + self.e0123 * other.e013,
            e021: self.s * other.e021,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e0123 * other.e021,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl InnerProduct<FourVector> for EvenMultivector {
    type Output = EvenMultivector;
    fn inner_product(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0123 * other.e0123,
            e01: -self.e23 * other.e0123,
            e02: -self.e31 * other.e0123,
            e12: self.e03 * other.e0123,
            e03: -self.e12 * other.e0123,
            e31: self.e02 * other.e0123,
            e23: self.e01 * other.e0123,
            e0123: self.s * other.e0123,
        }
    }
}

impl InnerProduct<Null> for EvenMultivector {
    type Output = Null;
    fn inner_product(self, other: Null) -> Null {
        Null
    }
}

impl InnerProduct<OddMultivector> for EvenMultivector {
    type Output = OddMultivector;
    fn inner_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0 + self.e01 * other.e1 + self.e02 * other.e2 + self.e12 * other.e021 + self.e03 * other.e3 + self.e31 * other.e013 + self.e23 * other.e032 - self.e0123 * other.e123,
            e1: self.s * other.e1 + self.e01 * other.e0 + self.e02 * other.e021 + self.e12 * other.e2 - self.e03 * other.e013 - self.e31 * other.e3 - self.e23 * other.e123 + self.e0123 * other.e032,
            e2: self.s * other.e2 - self.e01 * other.e021 + self.e02 * other.e0 - self.e12 * other.e1 + self.e03 * other.e032 - self.e31 * other.e123 + self.e23 * other.e3 + self.e0123 * other.e013,
            e021: self.s * other.e021 - self.e0123 * other.e3,
            e3: self.s * other.e3 + self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2 + self.e0123 * other.e021,
            e013: self.s * other.e013 - self.e0123 * other.e2,
            e032: self.s * other.e032 - self.e0123 * other.e1,
            e123: self.s * other.e123 + self.e0123 * other.e0,
        }
    }
}

impl InnerProduct<EvenMultivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn inner_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e01: self.s * other.e01 + self.e01 * other.s - self.e23 * other.e0123 - self.e0123 * other.e23,
            e02: self.s * other.e02 + self.e02 * other.s - self.e31 * other.e0123 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e12 * other.s + self.e03 * other.e0123 + self.e0123 * other.e03,
            e03: self.s * other.e03 - self.e12 * other.e0123 + self.e03 * other.s - self.e0123 * other.e12,
            e31: self.s * other.e31 + self.e02 * other.e0123 + self.e31 * other.s + self.e0123 * other.e02,
            e23: self.s * other.e23 + self.e01 * other.e0123 + self.e23 * other.s + self.e0123 * other.e01,
            e0123: self.s * other.e0123 + self.e0123 * other.s,
        }
    }
}

impl InnerProduct<Multivector> for EvenMultivector {
    type Output = Multivector;
    fn inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e0: self.s * other.e0 + self.e01 * other.e1 + self.e02 * other.e2 + self.e12 * other.e021 + self.e03 * other.e3 + self.e31 * other.e013 + self.e23 * other.e032 - self.e0123 * other.e123,
            e1: self.s * other.e1 + self.e01 * other.e0 + self.e02 * other.e021 + self.e12 * other.e2 - self.e03 * other.e013 - self.e31 * other.e3 - self.e23 * other.e123 + self.e0123 * other.e032,
            e01: self.s * other.e01 + self.e01 * other.s - self.e23 * other.e0123 - self.e0123 * other.e23,
            e2: self.s * other.e2 - self.e01 * other.e021 + self.e02 * other.e0 - self.e12 * other.e1 + self.e03 * other.e032 - self.e31 * other.e123 + self.e23 * other.e3 + self.e0123 * other.e013,
            e02: self.s * other.e02 + self.e02 * other.s - self.e31 * other.e0123 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e12 * other.s + self.e03 * other.e0123 + self.e0123 * other.e03,
            e021: self.s * other.e021 - self.e0123 * other.e3,
            e3: self.s * other.e3 + self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2 + self.e0123 * other.e021,
            e03: self.s * other.e03 - self.e12 * other.e0123 + self.e03 * other.s - self.e0123 * other.e12,
            e31: self.s * other.e31 + self.e02 * other.e0123 + self.e31 * other.s + self.e0123 * other.e02,
            e013: self.s * other.e013 - self.e0123 * other.e2,
            e23: self.s * other.e23 + self.e01 * other.e0123 + self.e23 * other.s + self.e0123 * other.e01,
            e032: self.s * other.e032 - self.e0123 * other.e1,
            e123: self.s * other.e123 + self.e0123 * other.e0,
            e0123: self.s * other.e0123 + self.e0123 * other.s,
        }
    }
}

impl InnerProduct<Scalar> for Multivector {
    type Output = Multivector;
    fn inner_product(self, other: Scalar) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e01: self.e01 * other.s,
            e2: self.e2 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e021: self.e021 * other.s,
            e3: self.e3 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e013: self.e013 * other.s,
            e23: self.e23 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
            e0123: self.e0123 * other.s,
        }
    }
}

impl InnerProduct<Vector> for Multivector {
    type Output = Multivector;
    fn inner_product(self, other: Vector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e0: self.s * other.e0 + self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.s * other.e1 + self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e2: self.s * other.e2 + self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e021: -self.e0123 * other.e3,
            e3: self.s * other.e3 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e013: -self.e0123 * other.e2,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<Bivector> for Multivector {
    type Output = Multivector;
    fn inner_product(self, other: Bivector) -> Multivector {
        Multivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e0: -self.e1 * other.e01 - self.e2 * other.e02 + self.e021 * other.e12 - self.e3 * other.e03 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e021 * other.e02 + self.e3 * other.e31 - self.e013 * other.e03 - self.e123 * other.e23,
            e01: self.s * other.e01 - self.e0123 * other.e23,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e021 * other.e01 - self.e3 * other.e23 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.s * other.e02 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e0123 * other.e03,
            e021: 0.0,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: self.s * other.e03 - self.e0123 * other.e12,
            e31: self.s * other.e31 + self.e0123 * other.e02,
            e013: 0.0,
            e23: self.s * other.e23 + self.e0123 * other.e01,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<Trivector> for Multivector {
    type Output = Multivector;
    fn inner_product(self, other: Trivector) -> Multivector {
        Multivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032 - self.e0123 * other.e123,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123 + self.e0123 * other.e032,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123 + self.e0123 * other.e013,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e021: self.s * other.e021,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e0123 * other.e021,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e013: self.s * other.e013,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<FourVector> for Multivector {
    type Output = Multivector;
    fn inner_product(self, other: FourVector) -> Multivector {
        Multivector {
            s: -self.e0123 * other.e0123,
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e01: -self.e23 * other.e0123,
            e2: -self.e013 * other.e0123,
            e02: -self.e31 * other.e0123,
            e12: self.e03 * other.e0123,
            e021: self.e3 * other.e0123,
            e3: -self.e021 * other.e0123,
            e03: -self.e12 * other.e0123,
            e31: self.e02 * other.e0123,
            e013: self.e2 * other.e0123,
            e23: self.e01 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
            e0123: self.s * other.e0123,
        }
    }
}

impl InnerProduct<Null> for Multivector {
    type Output = Null;
    fn inner_product(self, other: Null) -> Null {
        Null
    }
}

impl InnerProduct<OddMultivector> for Multivector {
    type Output = Multivector;
    fn inner_product(self, other: OddMultivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.s * other.e0 + self.e01 * other.e1 + self.e02 * other.e2 + self.e12 * other.e021 + self.e03 * other.e3 + self.e31 * other.e013 + self.e23 * other.e032 - self.e0123 * other.e123,
            e1: self.s * other.e1 + self.e01 * other.e0 + self.e02 * other.e021 + self.e12 * other.e2 - self.e03 * other.e013 - self.e31 * other.e3 - self.e23 * other.e123 + self.e0123 * other.e032,
            e01: -self.e2 * other.e021 - self.e021 * other.e2 + self.e3 * other.e013 + self.e013 * other.e3,
            e2: self.s * other.e2 - self.e01 * other.e021 + self.e02 * other.e0 - self.e12 * other.e1 + self.e03 * other.e032 - self.e31 * other.e123 + self.e23 * other.e3 + self.e0123 * other.e013,
            e02: self.e1 * other.e021 + self.e021 * other.e1 - self.e3 * other.e032 - self.e032 * other.e3,
            e12: self.e0 * other.e021 + self.e021 * other.e0 + self.e3 * other.e123 + self.e123 * other.e3,
            e021: self.s * other.e021 - self.e0123 * other.e3,
            e3: self.s * other.e3 + self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2 + self.e0123 * other.e021,
            e03: -self.e1 * other.e013 + self.e2 * other.e032 - self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e0 * other.e013 + self.e2 * other.e123 + self.e013 * other.e0 + self.e123 * other.e2,
            e013: self.s * other.e013 - self.e0123 * other.e2,
            e23: self.e0 * other.e032 + self.e1 * other.e123 + self.e032 * other.e0 + self.e123 * other.e1,
            e032: self.s * other.e032 - self.e0123 * other.e1,
            e123: self.s * other.e123 + self.e0123 * other.e0,
            e0123: 0.0,
        }
    }
}

impl InnerProduct<EvenMultivector> for Multivector {
    type Output = Multivector;
    fn inner_product(self, other: EvenMultivector) -> Multivector {
        Multivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e0: self.e0 * other.s - self.e1 * other.e01 - self.e2 * other.e02 + self.e021 * other.e12 - self.e3 * other.e03 + self.e013 * other.e31 + self.e032 * other.e23 + self.e123 * other.e0123,
            e1: -self.e0 * other.e01 + self.e1 * other.s - self.e2 * other.e12 + self.e021 * other.e02 + self.e3 * other.e31 - self.e013 * other.e03 - self.e032 * other.e0123 - self.e123 * other.e23,
            e01: self.s * other.e01 + self.e01 * other.s - self.e23 * other.e0123 - self.e0123 * other.e23,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 + self.e2 * other.s - self.e021 * other.e01 - self.e3 * other.e23 - self.e013 * other.e0123 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.s * other.e02 + self.e02 * other.s - self.e31 * other.e0123 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e12 * other.s + self.e03 * other.e0123 + self.e0123 * other.e03,
            e021: self.e021 * other.s + self.e3 * other.e0123,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 - self.e021 * other.e0123 + self.e3 * other.s + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: self.s * other.e03 - self.e12 * other.e0123 + self.e03 * other.s - self.e0123 * other.e12,
            e31: self.s * other.e31 + self.e02 * other.e0123 + self.e31 * other.s + self.e0123 * other.e02,
            e013: self.e2 * other.e0123 + self.e013 * other.s,
            e23: self.s * other.e23 + self.e01 * other.e0123 + self.e23 * other.s + self.e0123 * other.e01,
            e032: self.e1 * other.e0123 + self.e032 * other.s,
            e123: -self.e0 * other.e0123 + self.e123 * other.s,
            e0123: self.s * other.e0123 + self.e0123 * other.s,
        }
    }
}

impl InnerProduct<Multivector> for Multivector {
    type Output = Multivector;
    fn inner_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s - self.e0 * other.e0 + self.e1 * other.e1 + self.e01 * other.e01 + self.e2 * other.e2 + self.e02 * other.e02 - self.e12 * other.e12 + self.e021 * other.e021 + self.e3 * other.e3 + self.e03 * other.e03 - self.e31 * other.e31 + self.e013 * other.e013 - self.e23 * other.e23 + self.e032 * other.e032 - self.e123 * other.e123 - self.e0123 * other.e0123,
            e0: self.s * other.e0 + self.e0 * other.s - self.e1 * other.e01 + self.e01 * other.e1 - self.e2 * other.e02 + self.e02 * other.e2 + self.e12 * other.e021 + self.e021 * other.e12 - self.e3 * other.e03 + self.e03 * other.e3 + self.e31 * other.e013 + self.e013 * other.e31 + self.e23 * other.e032 + self.e032 * other.e23 + self.e123 * other.e0123 - self.e0123 * other.e123,
            e1: self.s * other.e1 - self.e0 * other.e01 + self.e1 * other.s + self.e01 * other.e0 - self.e2 * other.e12 + self.e02 * other.e021 + self.e12 * other.e2 + self.e021 * other.e02 + self.e3 * other.e31 - self.e03 * other.e013 - self.e31 * other.e3 - self.e013 * other.e03 - self.e23 * other.e123 - self.e032 * other.e0123 - self.e123 * other.e23 + self.e0123 * other.e032,
            e01: self.s * other.e01 + self.e01 * other.s - self.e2 * other.e021 - self.e021 * other.e2 + self.e3 * other.e013 + self.e013 * other.e3 - self.e23 * other.e0123 - self.e0123 * other.e23,
            e2: self.s * other.e2 - self.e0 * other.e02 + self.e1 * other.e12 - self.e01 * other.e021 + self.e2 * other.s + self.e02 * other.e0 - self.e12 * other.e1 - self.e021 * other.e01 - self.e3 * other.e23 + self.e03 * other.e032 - self.e31 * other.e123 - self.e013 * other.e0123 + self.e23 * other.e3 + self.e032 * other.e03 - self.e123 * other.e31 + self.e0123 * other.e013,
            e02: self.s * other.e02 + self.e1 * other.e021 + self.e02 * other.s + self.e021 * other.e1 - self.e3 * other.e032 - self.e31 * other.e0123 - self.e032 * other.e3 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e0 * other.e021 + self.e12 * other.s + self.e021 * other.e0 + self.e3 * other.e123 + self.e03 * other.e0123 + self.e123 * other.e3 + self.e0123 * other.e03,
            e021: self.s * other.e021 + self.e021 * other.s + self.e3 * other.e0123 - self.e0123 * other.e3,
            e3: self.s * other.e3 - self.e0 * other.e03 - self.e1 * other.e31 + self.e01 * other.e013 + self.e2 * other.e23 - self.e02 * other.e032 - self.e12 * other.e123 - self.e021 * other.e0123 + self.e3 * other.s + self.e03 * other.e0 + self.e31 * other.e1 + self.e013 * other.e01 - self.e23 * other.e2 - self.e032 * other.e02 - self.e123 * other.e12 + self.e0123 * other.e021,
            e03: self.s * other.e03 - self.e1 * other.e013 + self.e2 * other.e032 - self.e12 * other.e0123 + self.e03 * other.s - self.e013 * other.e1 + self.e032 * other.e2 - self.e0123 * other.e12,
            e31: self.s * other.e31 + self.e0 * other.e013 + self.e2 * other.e123 + self.e02 * other.e0123 + self.e31 * other.s + self.e013 * other.e0 + self.e123 * other.e2 + self.e0123 * other.e02,
            e013: self.s * other.e013 + self.e2 * other.e0123 + self.e013 * other.s - self.e0123 * other.e2,
            e23: self.s * other.e23 + self.e0 * other.e032 + self.e1 * other.e123 + self.e01 * other.e0123 + self.e23 * other.s + self.e032 * other.e0 + self.e123 * other.e1 + self.e0123 * other.e01,
            e032: self.s * other.e032 + self.e1 * other.e0123 + self.e032 * other.s - self.e0123 * other.e1,
            e123: self.s * other.e123 - self.e0 * other.e0123 + self.e123 * other.s + self.e0123 * other.e0,
            e0123: self.s * other.e0123 + self.e0123 * other.s,
        }
    }
}

impl OuterProduct<Scalar> for Scalar {
    type Output = Scalar;
    fn outer_product(self, other: Scalar) -> Scalar {
        Scalar {
            s: self.s * other.s,
        }
    }
}

impl OuterProduct<Vector> for Scalar {
    type Output = Vector;
    fn outer_product(self, other: Vector) -> Vector {
        Vector {
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e2: self.s * other.e2,
            e3: self.s * other.e3,
        }
    }
}

impl OuterProduct<Bivector> for Scalar {
    type Output = Bivector;
    fn outer_product(self, other: Bivector) -> Bivector {
        Bivector {
            e01: self.s * other.e01,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e23: self.s * other.e23,
        }
    }
}

impl OuterProduct<Trivector> for Scalar {
    type Output = Trivector;
    fn outer_product(self, other: Trivector) -> Trivector {
        Trivector {
            e021: self.s * other.e021,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl OuterProduct<FourVector> for Scalar {
    type Output = FourVector;
    fn outer_product(self, other: FourVector) -> FourVector {
        FourVector {
            e0123: self.s * other.e0123,
        }
    }
}

impl OuterProduct<Null> for Scalar {
    type Output = Null;
    fn outer_product(self, other: Null) -> Null {
        Null
    }
}

impl OuterProduct<OddMultivector> for Scalar {
    type Output = OddMultivector;
    fn outer_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e2: self.s * other.e2,
            e021: self.s * other.e021,
            e3: self.s * other.e3,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl OuterProduct<EvenMultivector> for Scalar {
    type Output = EvenMultivector;
    fn outer_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: self.s * other.e01,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e23: self.s * other.e23,
            e0123: self.s * other.e0123,
        }
    }
}

impl OuterProduct<Multivector> for Scalar {
    type Output = Multivector;
    fn outer_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e01: self.s * other.e01,
            e2: self.s * other.e2,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e021: self.s * other.e021,
            e3: self.s * other.e3,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e013: self.s * other.e013,
            e23: self.s * other.e23,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
            e0123: self.s * other.e0123,
        }
    }
}

impl OuterProduct<Scalar> for Vector {
    type Output = Vector;
    fn outer_product(self, other: Scalar) -> Vector {
        Vector {
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e2: self.e2 * other.s,
            e3: self.e3 * other.s,
        }
    }
}

impl OuterProduct<Vector> for Vector {
    type Output = Bivector;
    fn outer_product(self, other: Vector) -> Bivector {
        Bivector {
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e31: -self.e1 * other.e3 + self.e3 * other.e1,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
        }
    }
}

impl OuterProduct<Bivector> for Vector {
    type Output = Trivector;
    fn outer_product(self, other: Bivector) -> Trivector {
        Trivector {
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        }
    }
}

impl OuterProduct<Trivector> for Vector {
    type Output = FourVector;
    fn outer_product(self, other: Trivector) -> FourVector {
        FourVector {
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl OuterProduct<FourVector> for Vector {
    type Output = Null;
    fn outer_product(self, other: FourVector) -> Null {
        Null
    }
}

impl OuterProduct<Null> for Vector {
    type Output = Null;
    fn outer_product(self, other: Null) -> Null {
        Null
    }
}

impl OuterProduct<OddMultivector> for Vector {
    type Output = EvenMultivector;
    fn outer_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e31: -self.e1 * other.e3 + self.e3 * other.e1,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl OuterProduct<EvenMultivector> for Vector {
    type Output = OddMultivector;
    fn outer_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e2: self.e2 * other.s,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01,
            e3: self.e3 * other.s,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        }
    }
}

impl OuterProduct<Multivector> for Vector {
    type Output = Multivector;
    fn outer_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e2: self.e2 * other.s,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01,
            e3: self.e3 * other.s,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e31: -self.e1 * other.e3 + self.e3 * other.e1,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl OuterProduct<Scalar> for Bivector {
    type Output = Bivector;
    fn outer_product(self, other: Scalar) -> Bivector {
        Bivector {
            e01: self.e01 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e23: self.e23 * other.s,
        }
    }
}

impl OuterProduct<Vector> for Bivector {
    type Output = Trivector;
    fn outer_product(self, other: Vector) -> Trivector {
        Trivector {
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e013: self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e032: -self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        }
    }
}

impl OuterProduct<Bivector> for Bivector {
    type Output = FourVector;
    fn outer_product(self, other: Bivector) -> FourVector {
        FourVector {
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl OuterProduct<Trivector> for Bivector {
    type Output = Null;
    fn outer_product(self, other: Trivector) -> Null {
        Null
    }
}

impl OuterProduct<FourVector> for Bivector {
    type Output = Null;
    fn outer_product(self, other: FourVector) -> Null {
        Null
    }
}

impl OuterProduct<Null> for Bivector {
    type Output = Null;
    fn outer_product(self, other: Null) -> Null {
        Null
    }
}

impl OuterProduct<OddMultivector> for Bivector {
    type Output = OddMultivector;
    fn outer_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: 0.0,
            e013: self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e032: -self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        }
    }
}

impl OuterProduct<EvenMultivector> for Bivector {
    type Output = EvenMultivector;
    fn outer_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e01 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e23: self.e23 * other.s,
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl OuterProduct<Multivector> for Bivector {
    type Output = Multivector;
    fn outer_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: 0.0,
            e1: 0.0,
            e01: self.e01 * other.s,
            e2: 0.0,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: 0.0,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e013: self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e23: self.e23 * other.s,
            e032: -self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl OuterProduct<Scalar> for Trivector {
    type Output = Trivector;
    fn outer_product(self, other: Scalar) -> Trivector {
        Trivector {
            e021: self.e021 * other.s,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl OuterProduct<Vector> for Trivector {
    type Output = FourVector;
    fn outer_product(self, other: Vector) -> FourVector {
        FourVector {
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl OuterProduct<Bivector> for Trivector {
    type Output = Null;
    fn outer_product(self, other: Bivector) -> Null {
        Null
    }
}

impl OuterProduct<Trivector> for Trivector {
    type Output = Null;
    fn outer_product(self, other: Trivector) -> Null {
        Null
    }
}

impl OuterProduct<FourVector> for Trivector {
    type Output = Null;
    fn outer_product(self, other: FourVector) -> Null {
        Null
    }
}

impl OuterProduct<Null> for Trivector {
    type Output = Null;
    fn outer_product(self, other: Null) -> Null {
        Null
    }
}

impl OuterProduct<OddMultivector> for Trivector {
    type Output = EvenMultivector;
    fn outer_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl OuterProduct<EvenMultivector> for Trivector {
    type Output = OddMultivector;
    fn outer_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: self.e021 * other.s,
            e3: 0.0,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl OuterProduct<Multivector> for Trivector {
    type Output = Multivector;
    fn outer_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: self.e021 * other.s,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: self.e013 * other.s,
            e23: 0.0,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl OuterProduct<Scalar> for FourVector {
    type Output = FourVector;
    fn outer_product(self, other: Scalar) -> FourVector {
        FourVector {
            e0123: self.e0123 * other.s,
        }
    }
}

impl OuterProduct<Vector> for FourVector {
    type Output = Null;
    fn outer_product(self, other: Vector) -> Null {
        Null
    }
}

impl OuterProduct<Bivector> for FourVector {
    type Output = Null;
    fn outer_product(self, other: Bivector) -> Null {
        Null
    }
}

impl OuterProduct<Trivector> for FourVector {
    type Output = Null;
    fn outer_product(self, other: Trivector) -> Null {
        Null
    }
}

impl OuterProduct<FourVector> for FourVector {
    type Output = Null;
    fn outer_product(self, other: FourVector) -> Null {
        Null
    }
}

impl OuterProduct<Null> for FourVector {
    type Output = Null;
    fn outer_product(self, other: Null) -> Null {
        Null
    }
}

impl OuterProduct<OddMultivector> for FourVector {
    type Output = OddMultivector;
    fn outer_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl OuterProduct<EvenMultivector> for FourVector {
    type Output = EvenMultivector;
    fn outer_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: self.e0123 * other.s,
        }
    }
}

impl OuterProduct<Multivector> for FourVector {
    type Output = Multivector;
    fn outer_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: self.e0123 * other.s,
        }
    }
}

impl OuterProduct<Scalar> for Null {
    type Output = Null;
    fn outer_product(self, other: Scalar) -> Null {
        Null
    }
}

impl OuterProduct<Vector> for Null {
    type Output = Null;
    fn outer_product(self, other: Vector) -> Null {
        Null
    }
}

impl OuterProduct<Bivector> for Null {
    type Output = Null;
    fn outer_product(self, other: Bivector) -> Null {
        Null
    }
}

impl OuterProduct<Trivector> for Null {
    type Output = Null;
    fn outer_product(self, other: Trivector) -> Null {
        Null
    }
}

impl OuterProduct<FourVector> for Null {
    type Output = Null;
    fn outer_product(self, other: FourVector) -> Null {
        Null
    }
}

impl OuterProduct<Null> for Null {
    type Output = Null;
    fn outer_product(self, other: Null) -> Null {
        Null
    }
}

impl OuterProduct<OddMultivector> for Null {
    type Output = Null;
    fn outer_product(self, other: OddMultivector) -> Null {
        Null
    }
}

impl OuterProduct<EvenMultivector> for Null {
    type Output = Null;
    fn outer_product(self, other: EvenMultivector) -> Null {
        Null
    }
}

impl OuterProduct<Multivector> for Null {
    type Output = Null;
    fn outer_product(self, other: Multivector) -> Null {
        Null
    }
}

impl OuterProduct<Scalar> for OddMultivector {
    type Output = OddMultivector;
    fn outer_product(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e2: self.e2 * other.s,
            e021: self.e021 * other.s,
            e3: self.e3 * other.s,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl OuterProduct<Vector> for OddMultivector {
    type Output = EvenMultivector;
    fn outer_product(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e31: -self.e1 * other.e3 + self.e3 * other.e1,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl OuterProduct<Bivector> for OddMultivector {
    type Output = OddMultivector;
    fn outer_product(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01,
            e3: 0.0,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        }
    }
}

impl OuterProduct<Trivector> for OddMultivector {
    type Output = EvenMultivector;
    fn outer_product(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl OuterProduct<FourVector> for OddMultivector {
    type Output = OddMultivector;
    fn outer_product(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl OuterProduct<Null> for OddMultivector {
    type Output = Null;
    fn outer_product(self, other: Null) -> Null {
        Null
    }
}

impl OuterProduct<OddMultivector> for OddMultivector {
    type Output = EvenMultivector;
    fn outer_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e31: -self.e1 * other.e3 + self.e3 * other.e1,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 - self.e021 * other.e3 + self.e3 * other.e021 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl OuterProduct<EvenMultivector> for OddMultivector {
    type Output = OddMultivector;
    fn outer_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e2: self.e2 * other.s,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01 + self.e021 * other.s,
            e3: self.e3 * other.s,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01 + self.e013 * other.s,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02 + self.e032 * other.s,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12 + self.e123 * other.s,
        }
    }
}

impl OuterProduct<Multivector> for OddMultivector {
    type Output = Multivector;
    fn outer_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e2: self.e2 * other.s,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01 + self.e021 * other.s,
            e3: self.e3 * other.s,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e31: -self.e1 * other.e3 + self.e3 * other.e1,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01 + self.e013 * other.s,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02 + self.e032 * other.s,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12 + self.e123 * other.s,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 - self.e021 * other.e3 + self.e3 * other.e021 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl OuterProduct<Scalar> for EvenMultivector {
    type Output = EvenMultivector;
    fn outer_product(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: self.e01 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e23: self.e23 * other.s,
            e0123: self.e0123 * other.s,
        }
    }
}

impl OuterProduct<Vector> for EvenMultivector {
    type Output = OddMultivector;
    fn outer_product(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e2: self.s * other.e2,
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: self.s * other.e3,
            e013: self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e032: -self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        }
    }
}

impl OuterProduct<Bivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn outer_product(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.s * other.e01,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e23: self.s * other.e23,
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl OuterProduct<Trivector> for EvenMultivector {
    type Output = OddMultivector;
    fn outer_product(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: self.s * other.e021,
            e3: 0.0,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl OuterProduct<FourVector> for EvenMultivector {
    type Output = EvenMultivector;
    fn outer_product(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: self.s * other.e0123,
        }
    }
}

impl OuterProduct<Null> for EvenMultivector {
    type Output = Null;
    fn outer_product(self, other: Null) -> Null {
        Null
    }
}

impl OuterProduct<OddMultivector> for EvenMultivector {
    type Output = OddMultivector;
    fn outer_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e2: self.s * other.e2,
            e021: self.s * other.e021 - self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: self.s * other.e3,
            e013: self.s * other.e013 + self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e032: self.s * other.e032 - self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.s * other.e123 + self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        }
    }
}

impl OuterProduct<EvenMultivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn outer_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: self.s * other.e01 + self.e01 * other.s,
            e02: self.s * other.e02 + self.e02 * other.s,
            e12: self.s * other.e12 + self.e12 * other.s,
            e03: self.s * other.e03 + self.e03 * other.s,
            e31: self.s * other.e31 + self.e31 * other.s,
            e23: self.s * other.e23 + self.e23 * other.s,
            e0123: self.s * other.e0123 + self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01 + self.e0123 * other.s,
        }
    }
}

impl OuterProduct<Multivector> for EvenMultivector {
    type Output = Multivector;
    fn outer_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e01: self.s * other.e01 + self.e01 * other.s,
            e2: self.s * other.e2,
            e02: self.s * other.e02 + self.e02 * other.s,
            e12: self.s * other.e12 + self.e12 * other.s,
            e021: self.s * other.e021 - self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: self.s * other.e3,
            e03: self.s * other.e03 + self.e03 * other.s,
            e31: self.s * other.e31 + self.e31 * other.s,
            e013: self.s * other.e013 + self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e23: self.s * other.e23 + self.e23 * other.s,
            e032: self.s * other.e032 - self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.s * other.e123 + self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
            e0123: self.s * other.e0123 + self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01 + self.e0123 * other.s,
        }
    }
}

impl OuterProduct<Scalar> for Multivector {
    type Output = Multivector;
    fn outer_product(self, other: Scalar) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e01: self.e01 * other.s,
            e2: self.e2 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e021: self.e021 * other.s,
            e3: self.e3 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e013: self.e013 * other.s,
            e23: self.e23 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
            e0123: self.e0123 * other.s,
        }
    }
}

impl OuterProduct<Vector> for Multivector {
    type Output = Multivector;
    fn outer_product(self, other: Vector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e2: self.s * other.e2,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: self.s * other.e3,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e31: -self.e1 * other.e3 + self.e3 * other.e1,
            e013: self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
            e032: -self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl OuterProduct<Bivector> for Multivector {
    type Output = Multivector;
    fn outer_product(self, other: Bivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: 0.0,
            e1: 0.0,
            e01: self.s * other.e01,
            e2: 0.0,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01,
            e3: 0.0,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01,
            e23: self.s * other.e23,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl OuterProduct<Trivector> for Multivector {
    type Output = Multivector;
    fn outer_product(self, other: Trivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: self.s * other.e021,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: self.s * other.e013,
            e23: 0.0,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl OuterProduct<FourVector> for Multivector {
    type Output = Multivector;
    fn outer_product(self, other: FourVector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: self.s * other.e0123,
        }
    }
}

impl OuterProduct<Null> for Multivector {
    type Output = Null;
    fn outer_product(self, other: Null) -> Null {
        Null
    }
}

impl OuterProduct<OddMultivector> for Multivector {
    type Output = Multivector;
    fn outer_product(self, other: OddMultivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e2: self.s * other.e2,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e021: self.s * other.e021 - self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: self.s * other.e3,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e31: -self.e1 * other.e3 + self.e3 * other.e1,
            e013: self.s * other.e013 + self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
            e032: self.s * other.e032 - self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.s * other.e123 + self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 - self.e021 * other.e3 + self.e3 * other.e021 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl OuterProduct<EvenMultivector> for Multivector {
    type Output = Multivector;
    fn outer_product(self, other: EvenMultivector) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e01: self.s * other.e01 + self.e01 * other.s,
            e2: self.e2 * other.s,
            e02: self.s * other.e02 + self.e02 * other.s,
            e12: self.s * other.e12 + self.e12 * other.s,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01 + self.e021 * other.s,
            e3: self.e3 * other.s,
            e03: self.s * other.e03 + self.e03 * other.s,
            e31: self.s * other.e31 + self.e31 * other.s,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01 + self.e013 * other.s,
            e23: self.s * other.e23 + self.e23 * other.s,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02 + self.e032 * other.s,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12 + self.e123 * other.s,
            e0123: self.s * other.e0123 + self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01 + self.e0123 * other.s,
        }
    }
}

impl OuterProduct<Multivector> for Multivector {
    type Output = Multivector;
    fn outer_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: self.s * other.e0 + self.e0 * other.s,
            e1: self.s * other.e1 + self.e1 * other.s,
            e01: self.s * other.e01 + self.e0 * other.e1 - self.e1 * other.e0 + self.e01 * other.s,
            e2: self.s * other.e2 + self.e2 * other.s,
            e02: self.s * other.e02 + self.e0 * other.e2 - self.e2 * other.e0 + self.e02 * other.s,
            e12: self.s * other.e12 + self.e1 * other.e2 - self.e2 * other.e1 + self.e12 * other.s,
            e021: self.s * other.e021 - self.e0 * other.e12 + self.e1 * other.e02 - self.e01 * other.e2 - self.e2 * other.e01 + self.e02 * other.e1 - self.e12 * other.e0 + self.e021 * other.s,
            e3: self.s * other.e3 + self.e3 * other.s,
            e03: self.s * other.e03 + self.e0 * other.e3 - self.e3 * other.e0 + self.e03 * other.s,
            e31: self.s * other.e31 - self.e1 * other.e3 + self.e3 * other.e1 + self.e31 * other.s,
            e013: self.s * other.e013 - self.e0 * other.e31 - self.e1 * other.e03 + self.e01 * other.e3 + self.e3 * other.e01 - self.e03 * other.e1 - self.e31 * other.e0 + self.e013 * other.s,
            e23: self.s * other.e23 + self.e2 * other.e3 - self.e3 * other.e2 + self.e23 * other.s,
            e032: self.s * other.e032 - self.e0 * other.e23 + self.e2 * other.e03 - self.e02 * other.e3 - self.e3 * other.e02 + self.e03 * other.e2 - self.e23 * other.e0 + self.e032 * other.s,
            e123: self.s * other.e123 + self.e1 * other.e23 + self.e2 * other.e31 + self.e12 * other.e3 + self.e3 * other.e12 + self.e31 * other.e2 + self.e23 * other.e1 + self.e123 * other.s,
            e0123: self.s * other.e0123 + self.e0 * other.e123 + self.e1 * other.e032 + self.e01 * other.e23 + self.e2 * other.e013 + self.e02 * other.e31 + self.e12 * other.e03 - self.e021 * other.e3 + self.e3 * other.e021 + self.e03 * other.e12 + self.e31 * other.e02 - self.e013 * other.e2 + self.e23 * other.e01 - self.e032 * other.e1 - self.e123 * other.e0 + self.e0123 * other.s,
        }
    }
}

impl RegressiveProduct<Scalar> for Scalar {
    type Output = Null;
    fn regressive_product(self, other: Scalar) -> Null {
        Null
    }
}

impl RegressiveProduct<Vector> for Scalar {
    type Output = Null;
    fn regressive_product(self, other: Vector) -> Null {
        Null
    }
}

impl RegressiveProduct<Bivector> for Scalar {
    type Output = Null;
    fn regressive_product(self, other: Bivector) -> Null {
        Null
    }
}

impl RegressiveProduct<Trivector> for Scalar {
    type Output = Null;
    fn regressive_product(self, other: Trivector) -> Null {
        Null
    }
}

impl RegressiveProduct<FourVector> for Scalar {
    type Output = Scalar;
    fn regressive_product(self, other: FourVector) -> Scalar {
        Scalar {
            s: self.s * other.e0123,
        }
    }
}

impl RegressiveProduct<Null> for Scalar {
    type Output = Null;
    fn regressive_product(self, other: Null) -> Null {
        Null
    }
}

impl RegressiveProduct<OddMultivector> for Scalar {
    type Output = OddMultivector;
    fn regressive_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl RegressiveProduct<EvenMultivector> for Scalar {
    type Output = EvenMultivector;
    fn regressive_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0123,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<Multivector> for Scalar {
    type Output = Multivector;
    fn regressive_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.e0123,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<Scalar> for Vector {
    type Output = Null;
    fn regressive_product(self, other: Scalar) -> Null {
        Null
    }
}

impl RegressiveProduct<Vector> for Vector {
    type Output = Null;
    fn regressive_product(self, other: Vector) -> Null {
        Null
    }
}

impl RegressiveProduct<Bivector> for Vector {
    type Output = Null;
    fn regressive_product(self, other: Bivector) -> Null {
        Null
    }
}

impl RegressiveProduct<Trivector> for Vector {
    type Output = Scalar;
    fn regressive_product(self, other: Trivector) -> Scalar {
        Scalar {
            s: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl RegressiveProduct<FourVector> for Vector {
    type Output = Vector;
    fn regressive_product(self, other: FourVector) -> Vector {
        Vector {
            e0: -self.e0 * other.e0123,
            e1: -self.e1 * other.e0123,
            e2: self.e2 * other.e0123,
            e3: -self.e3 * other.e0123,
        }
    }
}

impl RegressiveProduct<Null> for Vector {
    type Output = Null;
    fn regressive_product(self, other: Null) -> Null {
        Null
    }
}

impl RegressiveProduct<OddMultivector> for Vector {
    type Output = EvenMultivector;
    fn regressive_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<EvenMultivector> for Vector {
    type Output = OddMultivector;
    fn regressive_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e0123,
            e1: -self.e1 * other.e0123,
            e2: self.e2 * other.e0123,
            e021: 0.0,
            e3: -self.e3 * other.e0123,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl RegressiveProduct<Multivector> for Vector {
    type Output = Multivector;
    fn regressive_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
            e0: -self.e0 * other.e0123,
            e1: -self.e1 * other.e0123,
            e01: 0.0,
            e2: self.e2 * other.e0123,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: -self.e3 * other.e0123,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<Scalar> for Bivector {
    type Output = Null;
    fn regressive_product(self, other: Scalar) -> Null {
        Null
    }
}

impl RegressiveProduct<Vector> for Bivector {
    type Output = Null;
    fn regressive_product(self, other: Vector) -> Null {
        Null
    }
}

impl RegressiveProduct<Bivector> for Bivector {
    type Output = Scalar;
    fn regressive_product(self, other: Bivector) -> Scalar {
        Scalar {
            s: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl RegressiveProduct<Trivector> for Bivector {
    type Output = Vector;
    fn regressive_product(self, other: Trivector) -> Vector {
        Vector {
            e0: self.e01 * other.e032 + self.e02 * other.e013 + self.e03 * other.e021,
            e1: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021,
            e2: self.e02 * other.e123 + self.e12 * other.e032 - self.e23 * other.e021,
            e3: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
        }
    }
}

impl RegressiveProduct<FourVector> for Bivector {
    type Output = Bivector;
    fn regressive_product(self, other: FourVector) -> Bivector {
        Bivector {
            e01: -self.e01 * other.e0123,
            e02: self.e02 * other.e0123,
            e12: self.e12 * other.e0123,
            e03: -self.e03 * other.e0123,
            e31: -self.e31 * other.e0123,
            e23: self.e23 * other.e0123,
        }
    }
}

impl RegressiveProduct<Null> for Bivector {
    type Output = Null;
    fn regressive_product(self, other: Null) -> Null {
        Null
    }
}

impl RegressiveProduct<OddMultivector> for Bivector {
    type Output = OddMultivector;
    fn regressive_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e01 * other.e032 + self.e02 * other.e013 + self.e03 * other.e021,
            e1: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021,
            e2: self.e02 * other.e123 + self.e12 * other.e032 - self.e23 * other.e021,
            e021: 0.0,
            e3: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl RegressiveProduct<EvenMultivector> for Bivector {
    type Output = EvenMultivector;
    fn regressive_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
            e01: -self.e01 * other.e0123,
            e02: self.e02 * other.e0123,
            e12: self.e12 * other.e0123,
            e03: -self.e03 * other.e0123,
            e31: -self.e31 * other.e0123,
            e23: self.e23 * other.e0123,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<Multivector> for Bivector {
    type Output = Multivector;
    fn regressive_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
            e0: self.e01 * other.e032 + self.e02 * other.e013 + self.e03 * other.e021,
            e1: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021,
            e01: -self.e01 * other.e0123,
            e2: self.e02 * other.e123 + self.e12 * other.e032 - self.e23 * other.e021,
            e02: self.e02 * other.e0123,
            e12: self.e12 * other.e0123,
            e021: 0.0,
            e3: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
            e03: -self.e03 * other.e0123,
            e31: -self.e31 * other.e0123,
            e013: 0.0,
            e23: self.e23 * other.e0123,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<Scalar> for Trivector {
    type Output = Null;
    fn regressive_product(self, other: Scalar) -> Null {
        Null
    }
}

impl RegressiveProduct<Vector> for Trivector {
    type Output = Scalar;
    fn regressive_product(self, other: Vector) -> Scalar {
        Scalar {
            s: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl RegressiveProduct<Bivector> for Trivector {
    type Output = Vector;
    fn regressive_product(self, other: Bivector) -> Vector {
        Vector {
            e0: self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
            e1: -self.e021 * other.e31 + self.e013 * other.e12 - self.e123 * other.e01,
            e2: -self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e3: -self.e013 * other.e23 + self.e032 * other.e31 - self.e123 * other.e03,
        }
    }
}

impl RegressiveProduct<Trivector> for Trivector {
    type Output = Bivector;
    fn regressive_product(self, other: Trivector) -> Bivector {
        Bivector {
            e01: self.e021 * other.e013 - self.e013 * other.e021,
            e02: self.e021 * other.e032 - self.e032 * other.e021,
            e12: -self.e021 * other.e123 + self.e123 * other.e021,
            e03: self.e013 * other.e032 - self.e032 * other.e013,
            e31: self.e013 * other.e123 - self.e123 * other.e013,
            e23: -self.e032 * other.e123 + self.e123 * other.e032,
        }
    }
}

impl RegressiveProduct<FourVector> for Trivector {
    type Output = Trivector;
    fn regressive_product(self, other: FourVector) -> Trivector {
        Trivector {
            e021: self.e021 * other.e0123,
            e013: -self.e013 * other.e0123,
            e032: self.e032 * other.e0123,
            e123: self.e123 * other.e0123,
        }
    }
}

impl RegressiveProduct<Null> for Trivector {
    type Output = Null;
    fn regressive_product(self, other: Null) -> Null {
        Null
    }
}

impl RegressiveProduct<OddMultivector> for Trivector {
    type Output = EvenMultivector;
    fn regressive_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
            e01: self.e021 * other.e013 - self.e013 * other.e021,
            e02: self.e021 * other.e032 - self.e032 * other.e021,
            e12: -self.e021 * other.e123 + self.e123 * other.e021,
            e03: self.e013 * other.e032 - self.e032 * other.e013,
            e31: self.e013 * other.e123 - self.e123 * other.e013,
            e23: -self.e032 * other.e123 + self.e123 * other.e032,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<EvenMultivector> for Trivector {
    type Output = OddMultivector;
    fn regressive_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
            e1: -self.e021 * other.e31 + self.e013 * other.e12 - self.e123 * other.e01,
            e2: -self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e021: self.e021 * other.e0123,
            e3: -self.e013 * other.e23 + self.e032 * other.e31 - self.e123 * other.e03,
            e013: -self.e013 * other.e0123,
            e032: self.e032 * other.e0123,
            e123: self.e123 * other.e0123,
        }
    }
}

impl RegressiveProduct<Multivector> for Trivector {
    type Output = Multivector;
    fn regressive_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
            e0: self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
            e1: -self.e021 * other.e31 + self.e013 * other.e12 - self.e123 * other.e01,
            e01: self.e021 * other.e013 - self.e013 * other.e021,
            e2: -self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e02: self.e021 * other.e032 - self.e032 * other.e021,
            e12: -self.e021 * other.e123 + self.e123 * other.e021,
            e021: self.e021 * other.e0123,
            e3: -self.e013 * other.e23 + self.e032 * other.e31 - self.e123 * other.e03,
            e03: self.e013 * other.e032 - self.e032 * other.e013,
            e31: self.e013 * other.e123 - self.e123 * other.e013,
            e013: -self.e013 * other.e0123,
            e23: -self.e032 * other.e123 + self.e123 * other.e032,
            e032: self.e032 * other.e0123,
            e123: self.e123 * other.e0123,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<Scalar> for FourVector {
    type Output = Scalar;
    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar {
            s: self.e0123 * other.s,
        }
    }
}

impl RegressiveProduct<Vector> for FourVector {
    type Output = Vector;
    fn regressive_product(self, other: Vector) -> Vector {
        Vector {
            e0: -self.e0123 * other.e0,
            e1: -self.e0123 * other.e1,
            e2: self.e0123 * other.e2,
            e3: -self.e0123 * other.e3,
        }
    }
}

impl RegressiveProduct<Bivector> for FourVector {
    type Output = Bivector;
    fn regressive_product(self, other: Bivector) -> Bivector {
        Bivector {
            e01: -self.e0123 * other.e01,
            e02: self.e0123 * other.e02,
            e12: self.e0123 * other.e12,
            e03: -self.e0123 * other.e03,
            e31: -self.e0123 * other.e31,
            e23: self.e0123 * other.e23,
        }
    }
}

impl RegressiveProduct<Trivector> for FourVector {
    type Output = Trivector;
    fn regressive_product(self, other: Trivector) -> Trivector {
        Trivector {
            e021: self.e0123 * other.e021,
            e013: -self.e0123 * other.e013,
            e032: self.e0123 * other.e032,
            e123: self.e0123 * other.e123,
        }
    }
}

impl RegressiveProduct<FourVector> for FourVector {
    type Output = FourVector;
    fn regressive_product(self, other: FourVector) -> FourVector {
        FourVector {
            e0123: -self.e0123 * other.e0123,
        }
    }
}

impl RegressiveProduct<Null> for FourVector {
    type Output = Null;
    fn regressive_product(self, other: Null) -> Null {
        Null
    }
}

impl RegressiveProduct<OddMultivector> for FourVector {
    type Output = OddMultivector;
    fn regressive_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0123 * other.e0,
            e1: -self.e0123 * other.e1,
            e2: self.e0123 * other.e2,
            e021: self.e0123 * other.e021,
            e3: -self.e0123 * other.e3,
            e013: -self.e0123 * other.e013,
            e032: self.e0123 * other.e032,
            e123: self.e0123 * other.e123,
        }
    }
}

impl RegressiveProduct<EvenMultivector> for FourVector {
    type Output = EvenMultivector;
    fn regressive_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e0123 * other.s,
            e01: -self.e0123 * other.e01,
            e02: self.e0123 * other.e02,
            e12: self.e0123 * other.e12,
            e03: -self.e0123 * other.e03,
            e31: -self.e0123 * other.e31,
            e23: self.e0123 * other.e23,
            e0123: -self.e0123 * other.e0123,
        }
    }
}

impl RegressiveProduct<Multivector> for FourVector {
    type Output = Multivector;
    fn regressive_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.e0123 * other.s,
            e0: -self.e0123 * other.e0,
            e1: -self.e0123 * other.e1,
            e01: -self.e0123 * other.e01,
            e2: self.e0123 * other.e2,
            e02: self.e0123 * other.e02,
            e12: self.e0123 * other.e12,
            e021: self.e0123 * other.e021,
            e3: -self.e0123 * other.e3,
            e03: -self.e0123 * other.e03,
            e31: -self.e0123 * other.e31,
            e013: -self.e0123 * other.e013,
            e23: self.e0123 * other.e23,
            e032: self.e0123 * other.e032,
            e123: self.e0123 * other.e123,
            e0123: -self.e0123 * other.e0123,
        }
    }
}

impl RegressiveProduct<Scalar> for Null {
    type Output = Null;
    fn regressive_product(self, other: Scalar) -> Null {
        Null
    }
}

impl RegressiveProduct<Vector> for Null {
    type Output = Null;
    fn regressive_product(self, other: Vector) -> Null {
        Null
    }
}

impl RegressiveProduct<Bivector> for Null {
    type Output = Null;
    fn regressive_product(self, other: Bivector) -> Null {
        Null
    }
}

impl RegressiveProduct<Trivector> for Null {
    type Output = Null;
    fn regressive_product(self, other: Trivector) -> Null {
        Null
    }
}

impl RegressiveProduct<FourVector> for Null {
    type Output = Null;
    fn regressive_product(self, other: FourVector) -> Null {
        Null
    }
}

impl RegressiveProduct<Null> for Null {
    type Output = Null;
    fn regressive_product(self, other: Null) -> Null {
        Null
    }
}

impl RegressiveProduct<OddMultivector> for Null {
    type Output = Null;
    fn regressive_product(self, other: OddMultivector) -> Null {
        Null
    }
}

impl RegressiveProduct<EvenMultivector> for Null {
    type Output = Null;
    fn regressive_product(self, other: EvenMultivector) -> Null {
        Null
    }
}

impl RegressiveProduct<Multivector> for Null {
    type Output = Null;
    fn regressive_product(self, other: Multivector) -> Null {
        Null
    }
}

impl RegressiveProduct<Scalar> for OddMultivector {
    type Output = OddMultivector;
    fn regressive_product(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl RegressiveProduct<Vector> for OddMultivector {
    type Output = EvenMultivector;
    fn regressive_product(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<Bivector> for OddMultivector {
    type Output = OddMultivector;
    fn regressive_product(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
            e1: -self.e021 * other.e31 + self.e013 * other.e12 - self.e123 * other.e01,
            e2: -self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e021: 0.0,
            e3: -self.e013 * other.e23 + self.e032 * other.e31 - self.e123 * other.e03,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl RegressiveProduct<Trivector> for OddMultivector {
    type Output = EvenMultivector;
    fn regressive_product(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
            e01: self.e021 * other.e013 - self.e013 * other.e021,
            e02: self.e021 * other.e032 - self.e032 * other.e021,
            e12: -self.e021 * other.e123 + self.e123 * other.e021,
            e03: self.e013 * other.e032 - self.e032 * other.e013,
            e31: self.e013 * other.e123 - self.e123 * other.e013,
            e23: -self.e032 * other.e123 + self.e123 * other.e032,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<FourVector> for OddMultivector {
    type Output = OddMultivector;
    fn regressive_product(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e0123,
            e1: -self.e1 * other.e0123,
            e2: self.e2 * other.e0123,
            e021: self.e021 * other.e0123,
            e3: -self.e3 * other.e0123,
            e013: -self.e013 * other.e0123,
            e032: self.e032 * other.e0123,
            e123: self.e123 * other.e0123,
        }
    }
}

impl RegressiveProduct<Null> for OddMultivector {
    type Output = Null;
    fn regressive_product(self, other: Null) -> Null {
        Null
    }
}

impl RegressiveProduct<OddMultivector> for OddMultivector {
    type Output = EvenMultivector;
    fn regressive_product(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 - self.e021 * other.e3 + self.e3 * other.e021 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
            e01: self.e021 * other.e013 - self.e013 * other.e021,
            e02: self.e021 * other.e032 - self.e032 * other.e021,
            e12: -self.e021 * other.e123 + self.e123 * other.e021,
            e03: self.e013 * other.e032 - self.e032 * other.e013,
            e31: self.e013 * other.e123 - self.e123 * other.e013,
            e23: -self.e032 * other.e123 + self.e123 * other.e032,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<EvenMultivector> for OddMultivector {
    type Output = OddMultivector;
    fn regressive_product(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e0123 + self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
            e1: -self.e1 * other.e0123 - self.e021 * other.e31 + self.e013 * other.e12 - self.e123 * other.e01,
            e2: self.e2 * other.e0123 - self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e021: self.e021 * other.e0123,
            e3: -self.e3 * other.e0123 - self.e013 * other.e23 + self.e032 * other.e31 - self.e123 * other.e03,
            e013: -self.e013 * other.e0123,
            e032: self.e032 * other.e0123,
            e123: self.e123 * other.e0123,
        }
    }
}

impl RegressiveProduct<Multivector> for OddMultivector {
    type Output = Multivector;
    fn regressive_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 - self.e021 * other.e3 + self.e3 * other.e021 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
            e0: -self.e0 * other.e0123 + self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
            e1: -self.e1 * other.e0123 - self.e021 * other.e31 + self.e013 * other.e12 - self.e123 * other.e01,
            e01: self.e021 * other.e013 - self.e013 * other.e021,
            e2: self.e2 * other.e0123 - self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e02: self.e021 * other.e032 - self.e032 * other.e021,
            e12: -self.e021 * other.e123 + self.e123 * other.e021,
            e021: self.e021 * other.e0123,
            e3: -self.e3 * other.e0123 - self.e013 * other.e23 + self.e032 * other.e31 - self.e123 * other.e03,
            e03: self.e013 * other.e032 - self.e032 * other.e013,
            e31: self.e013 * other.e123 - self.e123 * other.e013,
            e013: -self.e013 * other.e0123,
            e23: -self.e032 * other.e123 + self.e123 * other.e032,
            e032: self.e032 * other.e0123,
            e123: self.e123 * other.e0123,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<Scalar> for EvenMultivector {
    type Output = EvenMultivector;
    fn regressive_product(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: self.e0123 * other.s,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<Vector> for EvenMultivector {
    type Output = OddMultivector;
    fn regressive_product(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0123 * other.e0,
            e1: -self.e0123 * other.e1,
            e2: self.e0123 * other.e2,
            e021: 0.0,
            e3: -self.e0123 * other.e3,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl RegressiveProduct<Bivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn regressive_product(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
            e01: -self.e0123 * other.e01,
            e02: self.e0123 * other.e02,
            e12: self.e0123 * other.e12,
            e03: -self.e0123 * other.e03,
            e31: -self.e0123 * other.e31,
            e23: self.e0123 * other.e23,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<Trivector> for EvenMultivector {
    type Output = OddMultivector;
    fn regressive_product(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: self.e01 * other.e032 + self.e02 * other.e013 + self.e03 * other.e021,
            e1: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021,
            e2: self.e02 * other.e123 + self.e12 * other.e032 - self.e23 * other.e021,
            e021: self.e0123 * other.e021,
            e3: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
            e013: -self.e0123 * other.e013,
            e032: self.e0123 * other.e032,
            e123: self.e0123 * other.e123,
        }
    }
}

impl RegressiveProduct<FourVector> for EvenMultivector {
    type Output = EvenMultivector;
    fn regressive_product(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0123,
            e01: -self.e01 * other.e0123,
            e02: self.e02 * other.e0123,
            e12: self.e12 * other.e0123,
            e03: -self.e03 * other.e0123,
            e31: -self.e31 * other.e0123,
            e23: self.e23 * other.e0123,
            e0123: -self.e0123 * other.e0123,
        }
    }
}

impl RegressiveProduct<Null> for EvenMultivector {
    type Output = Null;
    fn regressive_product(self, other: Null) -> Null {
        Null
    }
}

impl RegressiveProduct<OddMultivector> for EvenMultivector {
    type Output = OddMultivector;
    fn regressive_product(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e01 * other.e032 + self.e02 * other.e013 + self.e03 * other.e021 - self.e0123 * other.e0,
            e1: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021 - self.e0123 * other.e1,
            e2: self.e02 * other.e123 + self.e12 * other.e032 - self.e23 * other.e021 + self.e0123 * other.e2,
            e021: self.e0123 * other.e021,
            e3: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013 - self.e0123 * other.e3,
            e013: -self.e0123 * other.e013,
            e032: self.e0123 * other.e032,
            e123: self.e0123 * other.e123,
        }
    }
}

impl RegressiveProduct<EvenMultivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn regressive_product(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0123 + self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01 + self.e0123 * other.s,
            e01: -self.e01 * other.e0123 - self.e0123 * other.e01,
            e02: self.e02 * other.e0123 + self.e0123 * other.e02,
            e12: self.e12 * other.e0123 + self.e0123 * other.e12,
            e03: -self.e03 * other.e0123 - self.e0123 * other.e03,
            e31: -self.e31 * other.e0123 - self.e0123 * other.e31,
            e23: self.e23 * other.e0123 + self.e0123 * other.e23,
            e0123: -self.e0123 * other.e0123,
        }
    }
}

impl RegressiveProduct<Multivector> for EvenMultivector {
    type Output = Multivector;
    fn regressive_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.e0123 + self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01 + self.e0123 * other.s,
            e0: self.e01 * other.e032 + self.e02 * other.e013 + self.e03 * other.e021 - self.e0123 * other.e0,
            e1: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021 - self.e0123 * other.e1,
            e01: -self.e01 * other.e0123 - self.e0123 * other.e01,
            e2: self.e02 * other.e123 + self.e12 * other.e032 - self.e23 * other.e021 + self.e0123 * other.e2,
            e02: self.e02 * other.e0123 + self.e0123 * other.e02,
            e12: self.e12 * other.e0123 + self.e0123 * other.e12,
            e021: self.e0123 * other.e021,
            e3: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013 - self.e0123 * other.e3,
            e03: -self.e03 * other.e0123 - self.e0123 * other.e03,
            e31: -self.e31 * other.e0123 - self.e0123 * other.e31,
            e013: -self.e0123 * other.e013,
            e23: self.e23 * other.e0123 + self.e0123 * other.e23,
            e032: self.e0123 * other.e032,
            e123: self.e0123 * other.e123,
            e0123: -self.e0123 * other.e0123,
        }
    }
}

impl RegressiveProduct<Scalar> for Multivector {
    type Output = Multivector;
    fn regressive_product(self, other: Scalar) -> Multivector {
        Multivector {
            s: self.e0123 * other.s,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<Vector> for Multivector {
    type Output = Multivector;
    fn regressive_product(self, other: Vector) -> Multivector {
        Multivector {
            s: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
            e0: -self.e0123 * other.e0,
            e1: -self.e0123 * other.e1,
            e01: 0.0,
            e2: self.e0123 * other.e2,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: -self.e0123 * other.e3,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<Bivector> for Multivector {
    type Output = Multivector;
    fn regressive_product(self, other: Bivector) -> Multivector {
        Multivector {
            s: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
            e0: self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
            e1: -self.e021 * other.e31 + self.e013 * other.e12 - self.e123 * other.e01,
            e01: -self.e0123 * other.e01,
            e2: -self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e02: self.e0123 * other.e02,
            e12: self.e0123 * other.e12,
            e021: 0.0,
            e3: -self.e013 * other.e23 + self.e032 * other.e31 - self.e123 * other.e03,
            e03: -self.e0123 * other.e03,
            e31: -self.e0123 * other.e31,
            e013: 0.0,
            e23: self.e0123 * other.e23,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<Trivector> for Multivector {
    type Output = Multivector;
    fn regressive_product(self, other: Trivector) -> Multivector {
        Multivector {
            s: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
            e0: self.e01 * other.e032 + self.e02 * other.e013 + self.e03 * other.e021,
            e1: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021,
            e01: self.e021 * other.e013 - self.e013 * other.e021,
            e2: self.e02 * other.e123 + self.e12 * other.e032 - self.e23 * other.e021,
            e02: self.e021 * other.e032 - self.e032 * other.e021,
            e12: -self.e021 * other.e123 + self.e123 * other.e021,
            e021: self.e0123 * other.e021,
            e3: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
            e03: self.e013 * other.e032 - self.e032 * other.e013,
            e31: self.e013 * other.e123 - self.e123 * other.e013,
            e013: -self.e0123 * other.e013,
            e23: -self.e032 * other.e123 + self.e123 * other.e032,
            e032: self.e0123 * other.e032,
            e123: self.e0123 * other.e123,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<FourVector> for Multivector {
    type Output = Multivector;
    fn regressive_product(self, other: FourVector) -> Multivector {
        Multivector {
            s: self.s * other.e0123,
            e0: -self.e0 * other.e0123,
            e1: -self.e1 * other.e0123,
            e01: -self.e01 * other.e0123,
            e2: self.e2 * other.e0123,
            e02: self.e02 * other.e0123,
            e12: self.e12 * other.e0123,
            e021: self.e021 * other.e0123,
            e3: -self.e3 * other.e0123,
            e03: -self.e03 * other.e0123,
            e31: -self.e31 * other.e0123,
            e013: -self.e013 * other.e0123,
            e23: self.e23 * other.e0123,
            e032: self.e032 * other.e0123,
            e123: self.e123 * other.e0123,
            e0123: -self.e0123 * other.e0123,
        }
    }
}

impl RegressiveProduct<Null> for Multivector {
    type Output = Null;
    fn regressive_product(self, other: Null) -> Null {
        Null
    }
}

impl RegressiveProduct<OddMultivector> for Multivector {
    type Output = Multivector;
    fn regressive_product(self, other: OddMultivector) -> Multivector {
        Multivector {
            s: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 - self.e021 * other.e3 + self.e3 * other.e021 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
            e0: self.e01 * other.e032 + self.e02 * other.e013 + self.e03 * other.e021 - self.e0123 * other.e0,
            e1: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021 - self.e0123 * other.e1,
            e01: self.e021 * other.e013 - self.e013 * other.e021,
            e2: self.e02 * other.e123 + self.e12 * other.e032 - self.e23 * other.e021 + self.e0123 * other.e2,
            e02: self.e021 * other.e032 - self.e032 * other.e021,
            e12: -self.e021 * other.e123 + self.e123 * other.e021,
            e021: self.e0123 * other.e021,
            e3: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013 - self.e0123 * other.e3,
            e03: self.e013 * other.e032 - self.e032 * other.e013,
            e31: self.e013 * other.e123 - self.e123 * other.e013,
            e013: -self.e0123 * other.e013,
            e23: -self.e032 * other.e123 + self.e123 * other.e032,
            e032: self.e0123 * other.e032,
            e123: self.e0123 * other.e123,
            e0123: 0.0,
        }
    }
}

impl RegressiveProduct<EvenMultivector> for Multivector {
    type Output = Multivector;
    fn regressive_product(self, other: EvenMultivector) -> Multivector {
        Multivector {
            s: self.s * other.e0123 + self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01 + self.e0123 * other.s,
            e0: -self.e0 * other.e0123 + self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
            e1: -self.e1 * other.e0123 - self.e021 * other.e31 + self.e013 * other.e12 - self.e123 * other.e01,
            e01: -self.e01 * other.e0123 - self.e0123 * other.e01,
            e2: self.e2 * other.e0123 - self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e02: self.e02 * other.e0123 + self.e0123 * other.e02,
            e12: self.e12 * other.e0123 + self.e0123 * other.e12,
            e021: self.e021 * other.e0123,
            e3: -self.e3 * other.e0123 - self.e013 * other.e23 + self.e032 * other.e31 - self.e123 * other.e03,
            e03: -self.e03 * other.e0123 - self.e0123 * other.e03,
            e31: -self.e31 * other.e0123 - self.e0123 * other.e31,
            e013: -self.e013 * other.e0123,
            e23: self.e23 * other.e0123 + self.e0123 * other.e23,
            e032: self.e032 * other.e0123,
            e123: self.e123 * other.e0123,
            e0123: -self.e0123 * other.e0123,
        }
    }
}

impl RegressiveProduct<Multivector> for Multivector {
    type Output = Multivector;
    fn regressive_product(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.e0123 + self.e0 * other.e123 + self.e1 * other.e032 + self.e01 * other.e23 + self.e2 * other.e013 + self.e02 * other.e31 + self.e12 * other.e03 - self.e021 * other.e3 + self.e3 * other.e021 + self.e03 * other.e12 + self.e31 * other.e02 - self.e013 * other.e2 + self.e23 * other.e01 - self.e032 * other.e1 - self.e123 * other.e0 + self.e0123 * other.s,
            e0: -self.e0 * other.e0123 + self.e01 * other.e032 + self.e02 * other.e013 + self.e021 * other.e03 + self.e03 * other.e021 + self.e013 * other.e02 + self.e032 * other.e01 - self.e0123 * other.e0,
            e1: -self.e1 * other.e0123 - self.e01 * other.e123 + self.e12 * other.e013 - self.e021 * other.e31 - self.e31 * other.e021 + self.e013 * other.e12 - self.e123 * other.e01 - self.e0123 * other.e1,
            e01: -self.e01 * other.e0123 + self.e021 * other.e013 - self.e013 * other.e021 - self.e0123 * other.e01,
            e2: self.e2 * other.e0123 + self.e02 * other.e123 + self.e12 * other.e032 - self.e021 * other.e23 - self.e23 * other.e021 + self.e032 * other.e12 + self.e123 * other.e02 + self.e0123 * other.e2,
            e02: self.e02 * other.e0123 + self.e021 * other.e032 - self.e032 * other.e021 + self.e0123 * other.e02,
            e12: self.e12 * other.e0123 - self.e021 * other.e123 + self.e123 * other.e021 + self.e0123 * other.e12,
            e021: self.e021 * other.e0123 + self.e0123 * other.e021,
            e3: -self.e3 * other.e0123 - self.e03 * other.e123 + self.e31 * other.e032 - self.e013 * other.e23 - self.e23 * other.e013 + self.e032 * other.e31 - self.e123 * other.e03 - self.e0123 * other.e3,
            e03: -self.e03 * other.e0123 + self.e013 * other.e032 - self.e032 * other.e013 - self.e0123 * other.e03,
            e31: -self.e31 * other.e0123 + self.e013 * other.e123 - self.e123 * other.e013 - self.e0123 * other.e31,
            e013: -self.e013 * other.e0123 - self.e0123 * other.e013,
            e23: self.e23 * other.e0123 - self.e032 * other.e123 + self.e123 * other.e032 + self.e0123 * other.e23,
            e032: self.e032 * other.e0123 + self.e0123 * other.e032,
            e123: self.e123 * other.e0123 + self.e0123 * other.e123,
            e0123: -self.e0123 * other.e0123,
        }
    }
}

impl Commutator<Scalar> for Scalar {
    type Output = EvenMultivector;
    fn commutator(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Vector> for Scalar {
    type Output = OddMultivector;
    fn commutator(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Commutator<Bivector> for Scalar {
    type Output = EvenMultivector;
    fn commutator(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Trivector> for Scalar {
    type Output = OddMultivector;
    fn commutator(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Commutator<FourVector> for Scalar {
    type Output = EvenMultivector;
    fn commutator(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Null> for Scalar {
    type Output = Null;
    fn commutator(self, other: Null) -> Null {
        Null
    }
}

impl Commutator<OddMultivector> for Scalar {
    type Output = OddMultivector;
    fn commutator(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Commutator<EvenMultivector> for Scalar {
    type Output = EvenMultivector;
    fn commutator(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Multivector> for Scalar {
    type Output = Multivector;
    fn commutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Scalar> for Vector {
    type Output = OddMultivector;
    fn commutator(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Commutator<Vector> for Vector {
    type Output = EvenMultivector;
    fn commutator(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e31: -self.e1 * other.e3 + self.e3 * other.e1,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
            e0123: 0.0,
        }
    }
}

impl Commutator<Bivector> for Vector {
    type Output = OddMultivector;
    fn commutator(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23,
            e021: 0.0,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Commutator<Trivector> for Vector {
    type Output = EvenMultivector;
    fn commutator(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl Commutator<FourVector> for Vector {
    type Output = OddMultivector;
    fn commutator(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: self.e3 * other.e0123,
            e3: 0.0,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
        }
    }
}

impl Commutator<Null> for Vector {
    type Output = Null;
    fn commutator(self, other: Null) -> Null {
        Null
    }
}

impl Commutator<OddMultivector> for Vector {
    type Output = EvenMultivector;
    fn commutator(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e31: -self.e1 * other.e3 + self.e3 * other.e1,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl Commutator<EvenMultivector> for Vector {
    type Output = OddMultivector;
    fn commutator(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23,
            e021: self.e3 * other.e0123,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
        }
    }
}

impl Commutator<Multivector> for Vector {
    type Output = Multivector;
    fn commutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31,
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e021: self.e3 * other.e0123,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e31: -self.e1 * other.e3 + self.e3 * other.e1,
            e013: self.e2 * other.e0123,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl Commutator<Scalar> for Bivector {
    type Output = EvenMultivector;
    fn commutator(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Vector> for Bivector {
    type Output = OddMultivector;
    fn commutator(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e021: 0.0,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Commutator<Bivector> for Bivector {
    type Output = EvenMultivector;
    fn commutator(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03,
            e02: self.e01 * other.e12 - self.e12 * other.e01 - self.e03 * other.e23 + self.e23 * other.e03,
            e12: self.e01 * other.e02 - self.e02 * other.e01 + self.e31 * other.e23 - self.e23 * other.e31,
            e03: -self.e01 * other.e31 + self.e02 * other.e23 + self.e31 * other.e01 - self.e23 * other.e02,
            e31: -self.e01 * other.e03 - self.e12 * other.e23 + self.e03 * other.e01 + self.e23 * other.e12,
            e23: self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12,
            e0123: 0.0,
        }
    }
}

impl Commutator<Trivector> for Bivector {
    type Output = OddMultivector;
    fn commutator(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
            e3: 0.0,
            e013: -self.e02 * other.e123 - self.e12 * other.e032 + self.e23 * other.e021,
            e032: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021,
            e123: -self.e01 * other.e032 - self.e02 * other.e013 - self.e03 * other.e021,
        }
    }
}

impl Commutator<FourVector> for Bivector {
    type Output = EvenMultivector;
    fn commutator(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Null> for Bivector {
    type Output = Null;
    fn commutator(self, other: Null) -> Null {
        Null
    }
}

impl Commutator<OddMultivector> for Bivector {
    type Output = OddMultivector;
    fn commutator(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e021: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e013: -self.e02 * other.e123 - self.e12 * other.e032 + self.e23 * other.e021,
            e032: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021,
            e123: -self.e01 * other.e032 - self.e02 * other.e013 - self.e03 * other.e021,
        }
    }
}

impl Commutator<EvenMultivector> for Bivector {
    type Output = EvenMultivector;
    fn commutator(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03,
            e02: self.e01 * other.e12 - self.e12 * other.e01 - self.e03 * other.e23 + self.e23 * other.e03,
            e12: self.e01 * other.e02 - self.e02 * other.e01 + self.e31 * other.e23 - self.e23 * other.e31,
            e03: -self.e01 * other.e31 + self.e02 * other.e23 + self.e31 * other.e01 - self.e23 * other.e02,
            e31: -self.e01 * other.e03 - self.e12 * other.e23 + self.e03 * other.e01 + self.e23 * other.e12,
            e23: self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12,
            e0123: 0.0,
        }
    }
}

impl Commutator<Multivector> for Bivector {
    type Output = Multivector;
    fn commutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e01: -self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e02: self.e01 * other.e12 - self.e12 * other.e01 - self.e03 * other.e23 + self.e23 * other.e03,
            e12: self.e01 * other.e02 - self.e02 * other.e01 + self.e31 * other.e23 - self.e23 * other.e31,
            e021: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e03: -self.e01 * other.e31 + self.e02 * other.e23 + self.e31 * other.e01 - self.e23 * other.e02,
            e31: -self.e01 * other.e03 - self.e12 * other.e23 + self.e03 * other.e01 + self.e23 * other.e12,
            e013: -self.e02 * other.e123 - self.e12 * other.e032 + self.e23 * other.e021,
            e23: self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12,
            e032: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021,
            e123: -self.e01 * other.e032 - self.e02 * other.e013 - self.e03 * other.e021,
            e0123: 0.0,
        }
    }
}

impl Commutator<Scalar> for Trivector {
    type Output = OddMultivector;
    fn commutator(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Commutator<Vector> for Trivector {
    type Output = EvenMultivector;
    fn commutator(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl Commutator<Bivector> for Trivector {
    type Output = OddMultivector;
    fn commutator(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: 0.0,
            e013: -self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e032: self.e021 * other.e31 - self.e013 * other.e12 + self.e123 * other.e01,
            e123: self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
        }
    }
}

impl Commutator<Trivector> for Trivector {
    type Output = EvenMultivector;
    fn commutator(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e032 * other.e123 - self.e123 * other.e032,
            e02: self.e013 * other.e123 - self.e123 * other.e013,
            e12: -self.e013 * other.e032 + self.e032 * other.e013,
            e03: self.e021 * other.e123 - self.e123 * other.e021,
            e31: self.e021 * other.e032 - self.e032 * other.e021,
            e23: -self.e021 * other.e013 + self.e013 * other.e021,
            e0123: 0.0,
        }
    }
}

impl Commutator<FourVector> for Trivector {
    type Output = OddMultivector;
    fn commutator(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e2: -self.e013 * other.e0123,
            e021: 0.0,
            e3: -self.e021 * other.e0123,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Commutator<Null> for Trivector {
    type Output = Null;
    fn commutator(self, other: Null) -> Null {
        Null
    }
}

impl Commutator<OddMultivector> for Trivector {
    type Output = EvenMultivector;
    fn commutator(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e032 * other.e123 - self.e123 * other.e032,
            e02: self.e013 * other.e123 - self.e123 * other.e013,
            e12: -self.e013 * other.e032 + self.e032 * other.e013,
            e03: self.e021 * other.e123 - self.e123 * other.e021,
            e31: self.e021 * other.e032 - self.e032 * other.e021,
            e23: -self.e021 * other.e013 + self.e013 * other.e021,
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl Commutator<EvenMultivector> for Trivector {
    type Output = OddMultivector;
    fn commutator(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e2: -self.e013 * other.e0123,
            e021: self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: -self.e021 * other.e0123,
            e013: -self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e032: self.e021 * other.e31 - self.e013 * other.e12 + self.e123 * other.e01,
            e123: self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
        }
    }
}

impl Commutator<Multivector> for Trivector {
    type Output = Multivector;
    fn commutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e01: self.e032 * other.e123 - self.e123 * other.e032,
            e2: -self.e013 * other.e0123,
            e02: self.e013 * other.e123 - self.e123 * other.e013,
            e12: -self.e013 * other.e032 + self.e032 * other.e013,
            e021: self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: -self.e021 * other.e0123,
            e03: self.e021 * other.e123 - self.e123 * other.e021,
            e31: self.e021 * other.e032 - self.e032 * other.e021,
            e013: -self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e23: -self.e021 * other.e013 + self.e013 * other.e021,
            e032: self.e021 * other.e31 - self.e013 * other.e12 + self.e123 * other.e01,
            e123: self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl Commutator<Scalar> for FourVector {
    type Output = EvenMultivector;
    fn commutator(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Vector> for FourVector {
    type Output = OddMultivector;
    fn commutator(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: -self.e0123 * other.e3,
            e3: 0.0,
            e013: -self.e0123 * other.e2,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
        }
    }
}

impl Commutator<Bivector> for FourVector {
    type Output = EvenMultivector;
    fn commutator(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Trivector> for FourVector {
    type Output = OddMultivector;
    fn commutator(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e2: self.e0123 * other.e013,
            e021: 0.0,
            e3: self.e0123 * other.e021,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Commutator<FourVector> for FourVector {
    type Output = EvenMultivector;
    fn commutator(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Null> for FourVector {
    type Output = Null;
    fn commutator(self, other: Null) -> Null {
        Null
    }
}

impl Commutator<OddMultivector> for FourVector {
    type Output = OddMultivector;
    fn commutator(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e2: self.e0123 * other.e013,
            e021: -self.e0123 * other.e3,
            e3: self.e0123 * other.e021,
            e013: -self.e0123 * other.e2,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
        }
    }
}

impl Commutator<EvenMultivector> for FourVector {
    type Output = EvenMultivector;
    fn commutator(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Multivector> for FourVector {
    type Output = Multivector;
    fn commutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e01: 0.0,
            e2: self.e0123 * other.e013,
            e02: 0.0,
            e12: 0.0,
            e021: -self.e0123 * other.e3,
            e3: self.e0123 * other.e021,
            e03: 0.0,
            e31: 0.0,
            e013: -self.e0123 * other.e2,
            e23: 0.0,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Scalar> for Null {
    type Output = Null;
    fn commutator(self, other: Scalar) -> Null {
        Null
    }
}

impl Commutator<Vector> for Null {
    type Output = Null;
    fn commutator(self, other: Vector) -> Null {
        Null
    }
}

impl Commutator<Bivector> for Null {
    type Output = Null;
    fn commutator(self, other: Bivector) -> Null {
        Null
    }
}

impl Commutator<Trivector> for Null {
    type Output = Null;
    fn commutator(self, other: Trivector) -> Null {
        Null
    }
}

impl Commutator<FourVector> for Null {
    type Output = Null;
    fn commutator(self, other: FourVector) -> Null {
        Null
    }
}

impl Commutator<Null> for Null {
    type Output = Null;
    fn commutator(self, other: Null) -> Null {
        Null
    }
}

impl Commutator<OddMultivector> for Null {
    type Output = Null;
    fn commutator(self, other: OddMultivector) -> Null {
        Null
    }
}

impl Commutator<EvenMultivector> for Null {
    type Output = Null;
    fn commutator(self, other: EvenMultivector) -> Null {
        Null
    }
}

impl Commutator<Multivector> for Null {
    type Output = Null;
    fn commutator(self, other: Multivector) -> Null {
        Null
    }
}

impl Commutator<Scalar> for OddMultivector {
    type Output = OddMultivector;
    fn commutator(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Commutator<Vector> for OddMultivector {
    type Output = EvenMultivector;
    fn commutator(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e31: -self.e1 * other.e3 + self.e3 * other.e1,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl Commutator<Bivector> for OddMultivector {
    type Output = OddMultivector;
    fn commutator(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23,
            e021: self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23,
            e013: -self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e032: self.e021 * other.e31 - self.e013 * other.e12 + self.e123 * other.e01,
            e123: self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
        }
    }
}

impl Commutator<Trivector> for OddMultivector {
    type Output = EvenMultivector;
    fn commutator(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e032 * other.e123 - self.e123 * other.e032,
            e02: self.e013 * other.e123 - self.e123 * other.e013,
            e12: -self.e013 * other.e032 + self.e032 * other.e013,
            e03: self.e021 * other.e123 - self.e123 * other.e021,
            e31: self.e021 * other.e032 - self.e032 * other.e021,
            e23: -self.e021 * other.e013 + self.e013 * other.e021,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl Commutator<FourVector> for OddMultivector {
    type Output = OddMultivector;
    fn commutator(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e2: -self.e013 * other.e0123,
            e021: self.e3 * other.e0123,
            e3: -self.e021 * other.e0123,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
        }
    }
}

impl Commutator<Null> for OddMultivector {
    type Output = Null;
    fn commutator(self, other: Null) -> Null {
        Null
    }
}

impl Commutator<OddMultivector> for OddMultivector {
    type Output = EvenMultivector;
    fn commutator(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e0 * other.e1 - self.e1 * other.e0 + self.e032 * other.e123 - self.e123 * other.e032,
            e02: self.e0 * other.e2 - self.e2 * other.e0 + self.e013 * other.e123 - self.e123 * other.e013,
            e12: self.e1 * other.e2 - self.e2 * other.e1 - self.e013 * other.e032 + self.e032 * other.e013,
            e03: self.e0 * other.e3 + self.e021 * other.e123 - self.e3 * other.e0 - self.e123 * other.e021,
            e31: -self.e1 * other.e3 + self.e021 * other.e032 + self.e3 * other.e1 - self.e032 * other.e021,
            e23: self.e2 * other.e3 - self.e021 * other.e013 - self.e3 * other.e2 + self.e013 * other.e021,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 - self.e021 * other.e3 + self.e3 * other.e021 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl Commutator<EvenMultivector> for OddMultivector {
    type Output = OddMultivector;
    fn commutator(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03 + self.e123 * other.e0123,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31 - self.e032 * other.e0123,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23 - self.e013 * other.e0123,
            e021: self.e3 * other.e0123 + self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 - self.e021 * other.e0123,
            e013: self.e2 * other.e0123 - self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e032: self.e1 * other.e0123 + self.e021 * other.e31 - self.e013 * other.e12 + self.e123 * other.e01,
            e123: -self.e0 * other.e0123 + self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
        }
    }
}

impl Commutator<Multivector> for OddMultivector {
    type Output = Multivector;
    fn commutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03 + self.e123 * other.e0123,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31 - self.e032 * other.e0123,
            e01: self.e0 * other.e1 - self.e1 * other.e0 + self.e032 * other.e123 - self.e123 * other.e032,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23 - self.e013 * other.e0123,
            e02: self.e0 * other.e2 - self.e2 * other.e0 + self.e013 * other.e123 - self.e123 * other.e013,
            e12: self.e1 * other.e2 - self.e2 * other.e1 - self.e013 * other.e032 + self.e032 * other.e013,
            e021: self.e3 * other.e0123 + self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 - self.e021 * other.e0123,
            e03: self.e0 * other.e3 + self.e021 * other.e123 - self.e3 * other.e0 - self.e123 * other.e021,
            e31: -self.e1 * other.e3 + self.e021 * other.e032 + self.e3 * other.e1 - self.e032 * other.e021,
            e013: self.e2 * other.e0123 - self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e23: self.e2 * other.e3 - self.e021 * other.e013 - self.e3 * other.e2 + self.e013 * other.e021,
            e032: self.e1 * other.e0123 + self.e021 * other.e31 - self.e013 * other.e12 + self.e123 * other.e01,
            e123: -self.e0 * other.e0123 + self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 - self.e021 * other.e3 + self.e3 * other.e021 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl Commutator<Scalar> for EvenMultivector {
    type Output = EvenMultivector;
    fn commutator(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Vector> for EvenMultivector {
    type Output = OddMultivector;
    fn commutator(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e021: -self.e0123 * other.e3,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e013: -self.e0123 * other.e2,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
        }
    }
}

impl Commutator<Bivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn commutator(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03,
            e02: self.e01 * other.e12 - self.e12 * other.e01 - self.e03 * other.e23 + self.e23 * other.e03,
            e12: self.e01 * other.e02 - self.e02 * other.e01 + self.e31 * other.e23 - self.e23 * other.e31,
            e03: -self.e01 * other.e31 + self.e02 * other.e23 + self.e31 * other.e01 - self.e23 * other.e02,
            e31: -self.e01 * other.e03 - self.e12 * other.e23 + self.e03 * other.e01 + self.e23 * other.e12,
            e23: self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12,
            e0123: 0.0,
        }
    }
}

impl Commutator<Trivector> for EvenMultivector {
    type Output = OddMultivector;
    fn commutator(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e2: self.e0123 * other.e013,
            e021: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
            e3: self.e0123 * other.e021,
            e013: -self.e02 * other.e123 - self.e12 * other.e032 + self.e23 * other.e021,
            e032: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021,
            e123: -self.e01 * other.e032 - self.e02 * other.e013 - self.e03 * other.e021,
        }
    }
}

impl Commutator<FourVector> for EvenMultivector {
    type Output = EvenMultivector;
    fn commutator(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Null> for EvenMultivector {
    type Output = Null;
    fn commutator(self, other: Null) -> Null {
        Null
    }
}

impl Commutator<OddMultivector> for EvenMultivector {
    type Output = OddMultivector;
    fn commutator(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3 - self.e0123 * other.e123,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3 + self.e0123 * other.e032,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3 + self.e0123 * other.e013,
            e021: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013 - self.e0123 * other.e3,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2 + self.e0123 * other.e021,
            e013: -self.e02 * other.e123 - self.e12 * other.e032 + self.e23 * other.e021 - self.e0123 * other.e2,
            e032: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021 - self.e0123 * other.e1,
            e123: -self.e01 * other.e032 - self.e02 * other.e013 - self.e03 * other.e021 + self.e0123 * other.e0,
        }
    }
}

impl Commutator<EvenMultivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn commutator(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03,
            e02: self.e01 * other.e12 - self.e12 * other.e01 - self.e03 * other.e23 + self.e23 * other.e03,
            e12: self.e01 * other.e02 - self.e02 * other.e01 + self.e31 * other.e23 - self.e23 * other.e31,
            e03: -self.e01 * other.e31 + self.e02 * other.e23 + self.e31 * other.e01 - self.e23 * other.e02,
            e31: -self.e01 * other.e03 - self.e12 * other.e23 + self.e03 * other.e01 + self.e23 * other.e12,
            e23: self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12,
            e0123: 0.0,
        }
    }
}

impl Commutator<Multivector> for EvenMultivector {
    type Output = Multivector;
    fn commutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3 - self.e0123 * other.e123,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3 + self.e0123 * other.e032,
            e01: -self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3 + self.e0123 * other.e013,
            e02: self.e01 * other.e12 - self.e12 * other.e01 - self.e03 * other.e23 + self.e23 * other.e03,
            e12: self.e01 * other.e02 - self.e02 * other.e01 + self.e31 * other.e23 - self.e23 * other.e31,
            e021: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013 - self.e0123 * other.e3,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2 + self.e0123 * other.e021,
            e03: -self.e01 * other.e31 + self.e02 * other.e23 + self.e31 * other.e01 - self.e23 * other.e02,
            e31: -self.e01 * other.e03 - self.e12 * other.e23 + self.e03 * other.e01 + self.e23 * other.e12,
            e013: -self.e02 * other.e123 - self.e12 * other.e032 + self.e23 * other.e021 - self.e0123 * other.e2,
            e23: self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12,
            e032: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021 - self.e0123 * other.e1,
            e123: -self.e01 * other.e032 - self.e02 * other.e013 - self.e03 * other.e021 + self.e0123 * other.e0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Scalar> for Multivector {
    type Output = Multivector;
    fn commutator(self, other: Scalar) -> Multivector {
        Multivector {
            s: 0.0,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl Commutator<Vector> for Multivector {
    type Output = Multivector;
    fn commutator(self, other: Vector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3,
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e021: -self.e0123 * other.e3,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e31: -self.e1 * other.e3 + self.e3 * other.e1,
            e013: -self.e0123 * other.e2,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
            e032: -self.e0123 * other.e1,
            e123: self.e0123 * other.e0,
            e0123: -self.e021 * other.e3 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl Commutator<Bivector> for Multivector {
    type Output = Multivector;
    fn commutator(self, other: Bivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31,
            e01: -self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23,
            e02: self.e01 * other.e12 - self.e12 * other.e01 - self.e03 * other.e23 + self.e23 * other.e03,
            e12: self.e01 * other.e02 - self.e02 * other.e01 + self.e31 * other.e23 - self.e23 * other.e31,
            e021: self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23,
            e03: -self.e01 * other.e31 + self.e02 * other.e23 + self.e31 * other.e01 - self.e23 * other.e02,
            e31: -self.e01 * other.e03 - self.e12 * other.e23 + self.e03 * other.e01 + self.e23 * other.e12,
            e013: -self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e23: self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12,
            e032: self.e021 * other.e31 - self.e013 * other.e12 + self.e123 * other.e01,
            e123: self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
            e0123: 0.0,
        }
    }
}

impl Commutator<Trivector> for Multivector {
    type Output = Multivector;
    fn commutator(self, other: Trivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: -self.e0123 * other.e123,
            e1: self.e0123 * other.e032,
            e01: self.e032 * other.e123 - self.e123 * other.e032,
            e2: self.e0123 * other.e013,
            e02: self.e013 * other.e123 - self.e123 * other.e013,
            e12: -self.e013 * other.e032 + self.e032 * other.e013,
            e021: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013,
            e3: self.e0123 * other.e021,
            e03: self.e021 * other.e123 - self.e123 * other.e021,
            e31: self.e021 * other.e032 - self.e032 * other.e021,
            e013: -self.e02 * other.e123 - self.e12 * other.e032 + self.e23 * other.e021,
            e23: -self.e021 * other.e013 + self.e013 * other.e021,
            e032: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021,
            e123: -self.e01 * other.e032 - self.e02 * other.e013 - self.e03 * other.e021,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl Commutator<FourVector> for Multivector {
    type Output = Multivector;
    fn commutator(self, other: FourVector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: self.e123 * other.e0123,
            e1: -self.e032 * other.e0123,
            e01: 0.0,
            e2: -self.e013 * other.e0123,
            e02: 0.0,
            e12: 0.0,
            e021: self.e3 * other.e0123,
            e3: -self.e021 * other.e0123,
            e03: 0.0,
            e31: 0.0,
            e013: self.e2 * other.e0123,
            e23: 0.0,
            e032: self.e1 * other.e0123,
            e123: -self.e0 * other.e0123,
            e0123: 0.0,
        }
    }
}

impl Commutator<Null> for Multivector {
    type Output = Null;
    fn commutator(self, other: Null) -> Null {
        Null
    }
}

impl Commutator<OddMultivector> for Multivector {
    type Output = Multivector;
    fn commutator(self, other: OddMultivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3 - self.e0123 * other.e123,
            e1: self.e01 * other.e0 + self.e12 * other.e2 - self.e31 * other.e3 + self.e0123 * other.e032,
            e01: self.e0 * other.e1 - self.e1 * other.e0 + self.e032 * other.e123 - self.e123 * other.e032,
            e2: self.e02 * other.e0 - self.e12 * other.e1 + self.e23 * other.e3 + self.e0123 * other.e013,
            e02: self.e0 * other.e2 - self.e2 * other.e0 + self.e013 * other.e123 - self.e123 * other.e013,
            e12: self.e1 * other.e2 - self.e2 * other.e1 - self.e013 * other.e032 + self.e032 * other.e013,
            e021: -self.e03 * other.e123 + self.e31 * other.e032 - self.e23 * other.e013 - self.e0123 * other.e3,
            e3: self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2 + self.e0123 * other.e021,
            e03: self.e0 * other.e3 + self.e021 * other.e123 - self.e3 * other.e0 - self.e123 * other.e021,
            e31: -self.e1 * other.e3 + self.e021 * other.e032 + self.e3 * other.e1 - self.e032 * other.e021,
            e013: -self.e02 * other.e123 - self.e12 * other.e032 + self.e23 * other.e021 - self.e0123 * other.e2,
            e23: self.e2 * other.e3 - self.e021 * other.e013 - self.e3 * other.e2 + self.e013 * other.e021,
            e032: -self.e01 * other.e123 + self.e12 * other.e013 - self.e31 * other.e021 - self.e0123 * other.e1,
            e123: -self.e01 * other.e032 - self.e02 * other.e013 - self.e03 * other.e021 + self.e0123 * other.e0,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 - self.e021 * other.e3 + self.e3 * other.e021 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl Commutator<EvenMultivector> for Multivector {
    type Output = Multivector;
    fn commutator(self, other: EvenMultivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: -self.e1 * other.e01 - self.e2 * other.e02 - self.e3 * other.e03 + self.e123 * other.e0123,
            e1: -self.e0 * other.e01 - self.e2 * other.e12 + self.e3 * other.e31 - self.e032 * other.e0123,
            e01: -self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 - self.e3 * other.e23 - self.e013 * other.e0123,
            e02: self.e01 * other.e12 - self.e12 * other.e01 - self.e03 * other.e23 + self.e23 * other.e03,
            e12: self.e01 * other.e02 - self.e02 * other.e01 + self.e31 * other.e23 - self.e23 * other.e31,
            e021: self.e3 * other.e0123 + self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 - self.e021 * other.e0123,
            e03: -self.e01 * other.e31 + self.e02 * other.e23 + self.e31 * other.e01 - self.e23 * other.e02,
            e31: -self.e01 * other.e03 - self.e12 * other.e23 + self.e03 * other.e01 + self.e23 * other.e12,
            e013: self.e2 * other.e0123 - self.e021 * other.e23 + self.e032 * other.e12 + self.e123 * other.e02,
            e23: self.e02 * other.e03 + self.e12 * other.e31 - self.e03 * other.e02 - self.e31 * other.e12,
            e032: self.e1 * other.e0123 + self.e021 * other.e31 - self.e013 * other.e12 + self.e123 * other.e01,
            e123: -self.e0 * other.e0123 + self.e021 * other.e03 + self.e013 * other.e02 + self.e032 * other.e01,
            e0123: 0.0,
        }
    }
}

impl Commutator<Multivector> for Multivector {
    type Output = Multivector;
    fn commutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: -self.e1 * other.e01 + self.e01 * other.e1 - self.e2 * other.e02 + self.e02 * other.e2 - self.e3 * other.e03 + self.e03 * other.e3 + self.e123 * other.e0123 - self.e0123 * other.e123,
            e1: -self.e0 * other.e01 + self.e01 * other.e0 - self.e2 * other.e12 + self.e12 * other.e2 + self.e3 * other.e31 - self.e31 * other.e3 - self.e032 * other.e0123 + self.e0123 * other.e032,
            e01: self.e0 * other.e1 - self.e1 * other.e0 - self.e02 * other.e12 + self.e12 * other.e02 + self.e03 * other.e31 - self.e31 * other.e03 + self.e032 * other.e123 - self.e123 * other.e032,
            e2: -self.e0 * other.e02 + self.e1 * other.e12 + self.e02 * other.e0 - self.e12 * other.e1 - self.e3 * other.e23 - self.e013 * other.e0123 + self.e23 * other.e3 + self.e0123 * other.e013,
            e02: self.e0 * other.e2 + self.e01 * other.e12 - self.e2 * other.e0 - self.e12 * other.e01 - self.e03 * other.e23 + self.e013 * other.e123 + self.e23 * other.e03 - self.e123 * other.e013,
            e12: self.e1 * other.e2 + self.e01 * other.e02 - self.e2 * other.e1 - self.e02 * other.e01 + self.e31 * other.e23 - self.e013 * other.e032 - self.e23 * other.e31 + self.e032 * other.e013,
            e021: self.e3 * other.e0123 - self.e03 * other.e123 + self.e31 * other.e032 + self.e013 * other.e23 - self.e23 * other.e013 - self.e032 * other.e31 + self.e123 * other.e03 - self.e0123 * other.e3,
            e3: -self.e0 * other.e03 - self.e1 * other.e31 + self.e2 * other.e23 - self.e021 * other.e0123 + self.e03 * other.e0 + self.e31 * other.e1 - self.e23 * other.e2 + self.e0123 * other.e021,
            e03: self.e0 * other.e3 - self.e01 * other.e31 + self.e02 * other.e23 + self.e021 * other.e123 - self.e3 * other.e0 + self.e31 * other.e01 - self.e23 * other.e02 - self.e123 * other.e021,
            e31: -self.e1 * other.e3 - self.e01 * other.e03 - self.e12 * other.e23 + self.e021 * other.e032 + self.e3 * other.e1 + self.e03 * other.e01 + self.e23 * other.e12 - self.e032 * other.e021,
            e013: self.e2 * other.e0123 - self.e02 * other.e123 - self.e12 * other.e032 - self.e021 * other.e23 + self.e23 * other.e021 + self.e032 * other.e12 + self.e123 * other.e02 - self.e0123 * other.e2,
            e23: self.e2 * other.e3 + self.e02 * other.e03 + self.e12 * other.e31 - self.e021 * other.e013 - self.e3 * other.e2 - self.e03 * other.e02 - self.e31 * other.e12 + self.e013 * other.e021,
            e032: self.e1 * other.e0123 - self.e01 * other.e123 + self.e12 * other.e013 + self.e021 * other.e31 - self.e31 * other.e021 - self.e013 * other.e12 + self.e123 * other.e01 - self.e0123 * other.e1,
            e123: -self.e0 * other.e0123 - self.e01 * other.e032 - self.e02 * other.e013 + self.e021 * other.e03 - self.e03 * other.e021 + self.e013 * other.e02 + self.e032 * other.e01 + self.e0123 * other.e0,
            e0123: self.e0 * other.e123 + self.e1 * other.e032 + self.e2 * other.e013 - self.e021 * other.e3 + self.e3 * other.e021 - self.e013 * other.e2 - self.e032 * other.e1 - self.e123 * other.e0,
        }
    }
}

impl Anticommutator<Scalar> for Scalar {
    type Output = EvenMultivector;
    fn anticommutator(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<Vector> for Scalar {
    type Output = OddMultivector;
    fn anticommutator(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e2: self.s * other.e2,
            e021: 0.0,
            e3: self.s * other.e3,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Anticommutator<Bivector> for Scalar {
    type Output = EvenMultivector;
    fn anticommutator(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.s * other.e01,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e23: self.s * other.e23,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<Trivector> for Scalar {
    type Output = OddMultivector;
    fn anticommutator(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: self.s * other.e021,
            e3: 0.0,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl Anticommutator<FourVector> for Scalar {
    type Output = EvenMultivector;
    fn anticommutator(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: self.s * other.e0123,
        }
    }
}

impl Anticommutator<Null> for Scalar {
    type Output = Null;
    fn anticommutator(self, other: Null) -> Null {
        Null
    }
}

impl Anticommutator<OddMultivector> for Scalar {
    type Output = OddMultivector;
    fn anticommutator(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e2: self.s * other.e2,
            e021: self.s * other.e021,
            e3: self.s * other.e3,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl Anticommutator<EvenMultivector> for Scalar {
    type Output = EvenMultivector;
    fn anticommutator(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: self.s * other.e01,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e23: self.s * other.e23,
            e0123: self.s * other.e0123,
        }
    }
}

impl Anticommutator<Multivector> for Scalar {
    type Output = Multivector;
    fn anticommutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e01: self.s * other.e01,
            e2: self.s * other.e2,
            e02: self.s * other.e02,
            e12: self.s * other.e12,
            e021: self.s * other.e021,
            e3: self.s * other.e3,
            e03: self.s * other.e03,
            e31: self.s * other.e31,
            e013: self.s * other.e013,
            e23: self.s * other.e23,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
            e0123: self.s * other.e0123,
        }
    }
}

impl Anticommutator<Scalar> for Vector {
    type Output = OddMultivector;
    fn anticommutator(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e2: self.e2 * other.s,
            e021: 0.0,
            e3: self.e3 * other.s,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Anticommutator<Vector> for Vector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<Bivector> for Vector {
    type Output = OddMultivector;
    fn anticommutator(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01,
            e3: 0.0,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        }
    }
}

impl Anticommutator<Trivector> for Vector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<FourVector> for Vector {
    type Output = OddMultivector;
    fn anticommutator(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Anticommutator<Null> for Vector {
    type Output = Null;
    fn anticommutator(self, other: Null) -> Null {
        Null
    }
}

impl Anticommutator<OddMultivector> for Vector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<EvenMultivector> for Vector {
    type Output = OddMultivector;
    fn anticommutator(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e2: self.e2 * other.s,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01,
            e3: self.e3 * other.s,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        }
    }
}

impl Anticommutator<Multivector> for Vector {
    type Output = Multivector;
    fn anticommutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e2: self.e2 * other.s,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01,
            e3: self.e3 * other.s,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<Scalar> for Bivector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e01 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e23: self.e23 * other.s,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<Vector> for Bivector {
    type Output = OddMultivector;
    fn anticommutator(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: 0.0,
            e013: self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e032: -self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        }
    }
}

impl Anticommutator<Bivector> for Bivector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl Anticommutator<Trivector> for Bivector {
    type Output = OddMultivector;
    fn anticommutator(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e021: 0.0,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Anticommutator<FourVector> for Bivector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e23 * other.e0123,
            e02: -self.e31 * other.e0123,
            e12: self.e03 * other.e0123,
            e03: -self.e12 * other.e0123,
            e31: self.e02 * other.e0123,
            e23: self.e01 * other.e0123,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<Null> for Bivector {
    type Output = Null;
    fn anticommutator(self, other: Null) -> Null {
        Null
    }
}

impl Anticommutator<OddMultivector> for Bivector {
    type Output = OddMultivector;
    fn anticommutator(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e013: self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e032: -self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        }
    }
}

impl Anticommutator<EvenMultivector> for Bivector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e01: self.e01 * other.s - self.e23 * other.e0123,
            e02: self.e02 * other.s - self.e31 * other.e0123,
            e12: self.e12 * other.s + self.e03 * other.e0123,
            e03: -self.e12 * other.e0123 + self.e03 * other.s,
            e31: self.e02 * other.e0123 + self.e31 * other.s,
            e23: self.e01 * other.e0123 + self.e23 * other.s,
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl Anticommutator<Multivector> for Bivector {
    type Output = Multivector;
    fn anticommutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e01: self.e01 * other.s - self.e23 * other.e0123,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e02: self.e02 * other.s - self.e31 * other.e0123,
            e12: self.e12 * other.s + self.e03 * other.e0123,
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e03: -self.e12 * other.e0123 + self.e03 * other.s,
            e31: self.e02 * other.e0123 + self.e31 * other.s,
            e013: self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e23: self.e01 * other.e0123 + self.e23 * other.s,
            e032: -self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl Anticommutator<Scalar> for Trivector {
    type Output = OddMultivector;
    fn anticommutator(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: self.e021 * other.s,
            e3: 0.0,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl Anticommutator<Vector> for Trivector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<Bivector> for Trivector {
    type Output = OddMultivector;
    fn anticommutator(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e2: -self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e021: 0.0,
            e3: self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Anticommutator<Trivector> for Trivector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<FourVector> for Trivector {
    type Output = OddMultivector;
    fn anticommutator(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Anticommutator<Null> for Trivector {
    type Output = Null;
    fn anticommutator(self, other: Null) -> Null {
        Null
    }
}

impl Anticommutator<OddMultivector> for Trivector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<EvenMultivector> for Trivector {
    type Output = OddMultivector;
    fn anticommutator(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e2: -self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e021: self.e021 * other.s,
            e3: self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl Anticommutator<Multivector> for Trivector {
    type Output = Multivector;
    fn anticommutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e2: -self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e021: self.e021 * other.s,
            e3: self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e013: self.e013 * other.s,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<Scalar> for FourVector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: self.e0123 * other.s,
        }
    }
}

impl Anticommutator<Vector> for FourVector {
    type Output = OddMultivector;
    fn anticommutator(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Anticommutator<Bivector> for FourVector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e0123 * other.e23,
            e02: -self.e0123 * other.e31,
            e12: self.e0123 * other.e03,
            e03: -self.e0123 * other.e12,
            e31: self.e0123 * other.e02,
            e23: self.e0123 * other.e01,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<Trivector> for FourVector {
    type Output = OddMultivector;
    fn anticommutator(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Anticommutator<FourVector> for FourVector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0123 * other.e0123,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<Null> for FourVector {
    type Output = Null;
    fn anticommutator(self, other: Null) -> Null {
        Null
    }
}

impl Anticommutator<OddMultivector> for FourVector {
    type Output = OddMultivector;
    fn anticommutator(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Anticommutator<EvenMultivector> for FourVector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0123 * other.e0123,
            e01: -self.e0123 * other.e23,
            e02: -self.e0123 * other.e31,
            e12: self.e0123 * other.e03,
            e03: -self.e0123 * other.e12,
            e31: self.e0123 * other.e02,
            e23: self.e0123 * other.e01,
            e0123: self.e0123 * other.s,
        }
    }
}

impl Anticommutator<Multivector> for FourVector {
    type Output = Multivector;
    fn anticommutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0123 * other.e0123,
            e0: 0.0,
            e1: 0.0,
            e01: -self.e0123 * other.e23,
            e2: 0.0,
            e02: -self.e0123 * other.e31,
            e12: self.e0123 * other.e03,
            e021: 0.0,
            e3: 0.0,
            e03: -self.e0123 * other.e12,
            e31: self.e0123 * other.e02,
            e013: 0.0,
            e23: self.e0123 * other.e01,
            e032: 0.0,
            e123: 0.0,
            e0123: self.e0123 * other.s,
        }
    }
}

impl Anticommutator<Scalar> for Null {
    type Output = Null;
    fn anticommutator(self, other: Scalar) -> Null {
        Null
    }
}

impl Anticommutator<Vector> for Null {
    type Output = Null;
    fn anticommutator(self, other: Vector) -> Null {
        Null
    }
}

impl Anticommutator<Bivector> for Null {
    type Output = Null;
    fn anticommutator(self, other: Bivector) -> Null {
        Null
    }
}

impl Anticommutator<Trivector> for Null {
    type Output = Null;
    fn anticommutator(self, other: Trivector) -> Null {
        Null
    }
}

impl Anticommutator<FourVector> for Null {
    type Output = Null;
    fn anticommutator(self, other: FourVector) -> Null {
        Null
    }
}

impl Anticommutator<Null> for Null {
    type Output = Null;
    fn anticommutator(self, other: Null) -> Null {
        Null
    }
}

impl Anticommutator<OddMultivector> for Null {
    type Output = Null;
    fn anticommutator(self, other: OddMultivector) -> Null {
        Null
    }
}

impl Anticommutator<EvenMultivector> for Null {
    type Output = Null;
    fn anticommutator(self, other: EvenMultivector) -> Null {
        Null
    }
}

impl Anticommutator<Multivector> for Null {
    type Output = Null;
    fn anticommutator(self, other: Multivector) -> Null {
        Null
    }
}

impl Anticommutator<Scalar> for OddMultivector {
    type Output = OddMultivector;
    fn anticommutator(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e2: self.e2 * other.s,
            e021: self.e021 * other.s,
            e3: self.e3 * other.s,
            e013: self.e013 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
        }
    }
}

impl Anticommutator<Vector> for OddMultivector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<Bivector> for OddMultivector {
    type Output = OddMultivector;
    fn anticommutator(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e2: -self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01,
            e3: self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        }
    }
}

impl Anticommutator<Trivector> for OddMultivector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<FourVector> for OddMultivector {
    type Output = OddMultivector;
    fn anticommutator(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Anticommutator<Null> for OddMultivector {
    type Output = Null;
    fn anticommutator(self, other: Null) -> Null {
        Null
    }
}

impl Anticommutator<OddMultivector> for OddMultivector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e01: -self.e2 * other.e021 - self.e021 * other.e2 + self.e3 * other.e013 + self.e013 * other.e3,
            e02: self.e1 * other.e021 + self.e021 * other.e1 - self.e3 * other.e032 - self.e032 * other.e3,
            e12: self.e0 * other.e021 + self.e021 * other.e0 + self.e3 * other.e123 + self.e123 * other.e3,
            e03: -self.e1 * other.e013 + self.e2 * other.e032 - self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e0 * other.e013 + self.e2 * other.e123 + self.e013 * other.e0 + self.e123 * other.e2,
            e23: self.e0 * other.e032 + self.e1 * other.e123 + self.e032 * other.e0 + self.e123 * other.e1,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<EvenMultivector> for OddMultivector {
    type Output = OddMultivector;
    fn anticommutator(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s + self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e1 * other.s + self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e2: self.e2 * other.s - self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01 + self.e021 * other.s,
            e3: self.e3 * other.s + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01 + self.e013 * other.s,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02 + self.e032 * other.s,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12 + self.e123 * other.s,
        }
    }
}

impl Anticommutator<Multivector> for OddMultivector {
    type Output = Multivector;
    fn anticommutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.e0 * other.s + self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e1 * other.s + self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e01: -self.e2 * other.e021 - self.e021 * other.e2 + self.e3 * other.e013 + self.e013 * other.e3,
            e2: self.e2 * other.s - self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.e1 * other.e021 + self.e021 * other.e1 - self.e3 * other.e032 - self.e032 * other.e3,
            e12: self.e0 * other.e021 + self.e021 * other.e0 + self.e3 * other.e123 + self.e123 * other.e3,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01 + self.e021 * other.s,
            e3: self.e3 * other.s + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: -self.e1 * other.e013 + self.e2 * other.e032 - self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e0 * other.e013 + self.e2 * other.e123 + self.e013 * other.e0 + self.e123 * other.e2,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01 + self.e013 * other.s,
            e23: self.e0 * other.e032 + self.e1 * other.e123 + self.e032 * other.e0 + self.e123 * other.e1,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02 + self.e032 * other.s,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12 + self.e123 * other.s,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<Scalar> for EvenMultivector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s,
            e01: self.e01 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e23: self.e23 * other.s,
            e0123: self.e0123 * other.s,
        }
    }
}

impl Anticommutator<Vector> for EvenMultivector {
    type Output = OddMultivector;
    fn anticommutator(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e2: self.s * other.e2,
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: self.s * other.e3,
            e013: self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e032: -self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        }
    }
}

impl Anticommutator<Bivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e01: self.s * other.e01 - self.e0123 * other.e23,
            e02: self.s * other.e02 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e0123 * other.e03,
            e03: self.s * other.e03 - self.e0123 * other.e12,
            e31: self.s * other.e31 + self.e0123 * other.e02,
            e23: self.s * other.e23 + self.e0123 * other.e01,
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl Anticommutator<Trivector> for EvenMultivector {
    type Output = OddMultivector;
    fn anticommutator(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e021: self.s * other.e021,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e013: self.s * other.e013,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
        }
    }
}

impl Anticommutator<FourVector> for EvenMultivector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e0123 * other.e0123,
            e01: -self.e23 * other.e0123,
            e02: -self.e31 * other.e0123,
            e12: self.e03 * other.e0123,
            e03: -self.e12 * other.e0123,
            e31: self.e02 * other.e0123,
            e23: self.e01 * other.e0123,
            e0123: self.s * other.e0123,
        }
    }
}

impl Anticommutator<Null> for EvenMultivector {
    type Output = Null;
    fn anticommutator(self, other: Null) -> Null {
        Null
    }
}

impl Anticommutator<OddMultivector> for EvenMultivector {
    type Output = OddMultivector;
    fn anticommutator(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.s * other.e0 + self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.s * other.e1 + self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e2: self.s * other.e2 - self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e021: self.s * other.e021 - self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: self.s * other.e3 + self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e013: self.s * other.e013 + self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e032: self.s * other.e032 - self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.s * other.e123 + self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        }
    }
}

impl Anticommutator<EvenMultivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn anticommutator(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e01: self.s * other.e01 + self.e01 * other.s - self.e23 * other.e0123 - self.e0123 * other.e23,
            e02: self.s * other.e02 + self.e02 * other.s - self.e31 * other.e0123 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e12 * other.s + self.e03 * other.e0123 + self.e0123 * other.e03,
            e03: self.s * other.e03 - self.e12 * other.e0123 + self.e03 * other.s - self.e0123 * other.e12,
            e31: self.s * other.e31 + self.e02 * other.e0123 + self.e31 * other.s + self.e0123 * other.e02,
            e23: self.s * other.e23 + self.e01 * other.e0123 + self.e23 * other.s + self.e0123 * other.e01,
            e0123: self.s * other.e0123 + self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01 + self.e0123 * other.s,
        }
    }
}

impl Anticommutator<Multivector> for EvenMultivector {
    type Output = Multivector;
    fn anticommutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e0: self.s * other.e0 + self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.s * other.e1 + self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e01: self.s * other.e01 + self.e01 * other.s - self.e23 * other.e0123 - self.e0123 * other.e23,
            e2: self.s * other.e2 - self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e02: self.s * other.e02 + self.e02 * other.s - self.e31 * other.e0123 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e12 * other.s + self.e03 * other.e0123 + self.e0123 * other.e03,
            e021: self.s * other.e021 - self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: self.s * other.e3 + self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e03: self.s * other.e03 - self.e12 * other.e0123 + self.e03 * other.s - self.e0123 * other.e12,
            e31: self.s * other.e31 + self.e02 * other.e0123 + self.e31 * other.s + self.e0123 * other.e02,
            e013: self.s * other.e013 + self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e23: self.s * other.e23 + self.e01 * other.e0123 + self.e23 * other.s + self.e0123 * other.e01,
            e032: self.s * other.e032 - self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.s * other.e123 + self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
            e0123: self.s * other.e0123 + self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01 + self.e0123 * other.s,
        }
    }
}

impl Anticommutator<Scalar> for Multivector {
    type Output = Multivector;
    fn anticommutator(self, other: Scalar) -> Multivector {
        Multivector {
            s: self.s * other.s,
            e0: self.e0 * other.s,
            e1: self.e1 * other.s,
            e01: self.e01 * other.s,
            e2: self.e2 * other.s,
            e02: self.e02 * other.s,
            e12: self.e12 * other.s,
            e021: self.e021 * other.s,
            e3: self.e3 * other.s,
            e03: self.e03 * other.s,
            e31: self.e31 * other.s,
            e013: self.e013 * other.s,
            e23: self.e23 * other.s,
            e032: self.e032 * other.s,
            e123: self.e123 * other.s,
            e0123: self.e0123 * other.s,
        }
    }
}

impl Anticommutator<Vector> for Multivector {
    type Output = Multivector;
    fn anticommutator(self, other: Vector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3,
            e0: self.s * other.e0,
            e1: self.s * other.e1,
            e01: -self.e021 * other.e2 + self.e013 * other.e3,
            e2: self.s * other.e2,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e12: self.e021 * other.e0 + self.e123 * other.e3,
            e021: -self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: self.s * other.e3,
            e03: -self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e013 * other.e0 + self.e123 * other.e2,
            e013: self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e23: self.e032 * other.e0 + self.e123 * other.e1,
            e032: -self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<Bivector> for Multivector {
    type Output = Multivector;
    fn anticommutator(self, other: Bivector) -> Multivector {
        Multivector {
            s: self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23,
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e01: self.s * other.e01 - self.e0123 * other.e23,
            e2: -self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.s * other.e02 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e0123 * other.e03,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01,
            e3: self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: self.s * other.e03 - self.e0123 * other.e12,
            e31: self.s * other.e31 + self.e0123 * other.e02,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01,
            e23: self.s * other.e23 + self.e0123 * other.e01,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl Anticommutator<Trivector> for Multivector {
    type Output = Multivector;
    fn anticommutator(self, other: Trivector) -> Multivector {
        Multivector {
            s: self.e021 * other.e021 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e01: -self.e2 * other.e021 + self.e3 * other.e013,
            e2: -self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e12: self.e0 * other.e021 + self.e3 * other.e123,
            e021: self.s * other.e021,
            e3: self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032,
            e31: self.e0 * other.e013 + self.e2 * other.e123,
            e013: self.s * other.e013,
            e23: self.e0 * other.e032 + self.e1 * other.e123,
            e032: self.s * other.e032,
            e123: self.s * other.e123,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<FourVector> for Multivector {
    type Output = Multivector;
    fn anticommutator(self, other: FourVector) -> Multivector {
        Multivector {
            s: -self.e0123 * other.e0123,
            e0: 0.0,
            e1: 0.0,
            e01: -self.e23 * other.e0123,
            e2: 0.0,
            e02: -self.e31 * other.e0123,
            e12: self.e03 * other.e0123,
            e021: 0.0,
            e3: 0.0,
            e03: -self.e12 * other.e0123,
            e31: self.e02 * other.e0123,
            e013: 0.0,
            e23: self.e01 * other.e0123,
            e032: 0.0,
            e123: 0.0,
            e0123: self.s * other.e0123,
        }
    }
}

impl Anticommutator<Null> for Multivector {
    type Output = Null;
    fn anticommutator(self, other: Null) -> Null {
        Null
    }
}

impl Anticommutator<OddMultivector> for Multivector {
    type Output = Multivector;
    fn anticommutator(self, other: OddMultivector) -> Multivector {
        Multivector {
            s: -self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2 + self.e021 * other.e021 + self.e3 * other.e3 + self.e013 * other.e013 + self.e032 * other.e032 - self.e123 * other.e123,
            e0: self.s * other.e0 + self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: self.s * other.e1 + self.e02 * other.e021 - self.e03 * other.e013 - self.e23 * other.e123,
            e01: -self.e2 * other.e021 - self.e021 * other.e2 + self.e3 * other.e013 + self.e013 * other.e3,
            e2: self.s * other.e2 - self.e01 * other.e021 + self.e03 * other.e032 - self.e31 * other.e123,
            e02: self.e1 * other.e021 + self.e021 * other.e1 - self.e3 * other.e032 - self.e032 * other.e3,
            e12: self.e0 * other.e021 + self.e021 * other.e0 + self.e3 * other.e123 + self.e123 * other.e3,
            e021: self.s * other.e021 - self.e01 * other.e2 + self.e02 * other.e1 - self.e12 * other.e0,
            e3: self.s * other.e3 + self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123,
            e03: -self.e1 * other.e013 + self.e2 * other.e032 - self.e013 * other.e1 + self.e032 * other.e2,
            e31: self.e0 * other.e013 + self.e2 * other.e123 + self.e013 * other.e0 + self.e123 * other.e2,
            e013: self.s * other.e013 + self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e23: self.e0 * other.e032 + self.e1 * other.e123 + self.e032 * other.e0 + self.e123 * other.e1,
            e032: self.s * other.e032 - self.e02 * other.e3 + self.e03 * other.e2 - self.e23 * other.e0,
            e123: self.s * other.e123 + self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
            e0123: 0.0,
        }
    }
}

impl Anticommutator<EvenMultivector> for Multivector {
    type Output = Multivector;
    fn anticommutator(self, other: EvenMultivector) -> Multivector {
        Multivector {
            s: self.s * other.s + self.e01 * other.e01 + self.e02 * other.e02 - self.e12 * other.e12 + self.e03 * other.e03 - self.e31 * other.e31 - self.e23 * other.e23 - self.e0123 * other.e0123,
            e0: self.e0 * other.s + self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: self.e1 * other.s + self.e021 * other.e02 - self.e013 * other.e03 - self.e123 * other.e23,
            e01: self.s * other.e01 + self.e01 * other.s - self.e23 * other.e0123 - self.e0123 * other.e23,
            e2: self.e2 * other.s - self.e021 * other.e01 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.s * other.e02 + self.e02 * other.s - self.e31 * other.e0123 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e12 * other.s + self.e03 * other.e0123 + self.e0123 * other.e03,
            e021: -self.e0 * other.e12 + self.e1 * other.e02 - self.e2 * other.e01 + self.e021 * other.s,
            e3: self.e3 * other.s + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: self.s * other.e03 - self.e12 * other.e0123 + self.e03 * other.s - self.e0123 * other.e12,
            e31: self.s * other.e31 + self.e02 * other.e0123 + self.e31 * other.s + self.e0123 * other.e02,
            e013: -self.e0 * other.e31 - self.e1 * other.e03 + self.e3 * other.e01 + self.e013 * other.s,
            e23: self.s * other.e23 + self.e01 * other.e0123 + self.e23 * other.s + self.e0123 * other.e01,
            e032: -self.e0 * other.e23 + self.e2 * other.e03 - self.e3 * other.e02 + self.e032 * other.s,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12 + self.e123 * other.s,
            e0123: self.s * other.e0123 + self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01 + self.e0123 * other.s,
        }
    }
}

impl Anticommutator<Multivector> for Multivector {
    type Output = Multivector;
    fn anticommutator(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s - self.e0 * other.e0 + self.e1 * other.e1 + self.e01 * other.e01 + self.e2 * other.e2 + self.e02 * other.e02 - self.e12 * other.e12 + self.e021 * other.e021 + self.e3 * other.e3 + self.e03 * other.e03 - self.e31 * other.e31 + self.e013 * other.e013 - self.e23 * other.e23 + self.e032 * other.e032 - self.e123 * other.e123 - self.e0123 * other.e0123,
            e0: self.s * other.e0 + self.e0 * other.s + self.e12 * other.e021 + self.e021 * other.e12 + self.e31 * other.e013 + self.e013 * other.e31 + self.e23 * other.e032 + self.e032 * other.e23,
            e1: self.s * other.e1 + self.e1 * other.s + self.e02 * other.e021 + self.e021 * other.e02 - self.e03 * other.e013 - self.e013 * other.e03 - self.e23 * other.e123 - self.e123 * other.e23,
            e01: self.s * other.e01 + self.e01 * other.s - self.e2 * other.e021 - self.e021 * other.e2 + self.e3 * other.e013 + self.e013 * other.e3 - self.e23 * other.e0123 - self.e0123 * other.e23,
            e2: self.s * other.e2 - self.e01 * other.e021 + self.e2 * other.s - self.e021 * other.e01 + self.e03 * other.e032 - self.e31 * other.e123 + self.e032 * other.e03 - self.e123 * other.e31,
            e02: self.s * other.e02 + self.e1 * other.e021 + self.e02 * other.s + self.e021 * other.e1 - self.e3 * other.e032 - self.e31 * other.e0123 - self.e032 * other.e3 - self.e0123 * other.e31,
            e12: self.s * other.e12 + self.e0 * other.e021 + self.e12 * other.s + self.e021 * other.e0 + self.e3 * other.e123 + self.e03 * other.e0123 + self.e123 * other.e3 + self.e0123 * other.e03,
            e021: self.s * other.e021 - self.e0 * other.e12 + self.e1 * other.e02 - self.e01 * other.e2 - self.e2 * other.e01 + self.e02 * other.e1 - self.e12 * other.e0 + self.e021 * other.s,
            e3: self.s * other.e3 + self.e01 * other.e013 - self.e02 * other.e032 - self.e12 * other.e123 + self.e3 * other.s + self.e013 * other.e01 - self.e032 * other.e02 - self.e123 * other.e12,
            e03: self.s * other.e03 - self.e1 * other.e013 + self.e2 * other.e032 - self.e12 * other.e0123 + self.e03 * other.s - self.e013 * other.e1 + self.e032 * other.e2 - self.e0123 * other.e12,
            e31: self.s * other.e31 + self.e0 * other.e013 + self.e2 * other.e123 + self.e02 * other.e0123 + self.e31 * other.s + self.e013 * other.e0 + self.e123 * other.e2 + self.e0123 * other.e02,
            e013: self.s * other.e013 - self.e0 * other.e31 - self.e1 * other.e03 + self.e01 * other.e3 + self.e3 * other.e01 - self.e03 * other.e1 - self.e31 * other.e0 + self.e013 * other.s,
            e23: self.s * other.e23 + self.e0 * other.e032 + self.e1 * other.e123 + self.e01 * other.e0123 + self.e23 * other.s + self.e032 * other.e0 + self.e123 * other.e1 + self.e0123 * other.e01,
            e032: self.s * other.e032 - self.e0 * other.e23 + self.e2 * other.e03 - self.e02 * other.e3 - self.e3 * other.e02 + self.e03 * other.e2 - self.e23 * other.e0 + self.e032 * other.s,
            e123: self.s * other.e123 + self.e1 * other.e23 + self.e2 * other.e31 + self.e12 * other.e3 + self.e3 * other.e12 + self.e31 * other.e2 + self.e23 * other.e1 + self.e123 * other.s,
            e0123: self.s * other.e0123 + self.e01 * other.e23 + self.e02 * other.e31 + self.e12 * other.e03 + self.e03 * other.e12 + self.e31 * other.e02 + self.e23 * other.e01 + self.e0123 * other.s,
        }
    }
}

impl Transform<Scalar> for Scalar {
    type Output = EvenMultivector;
    fn transform(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s * other.s,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Transform<Vector> for Scalar {
    type Output = EvenMultivector;
    fn transform(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e3 * other.e3,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Transform<Bivector> for Scalar {
    type Output = EvenMultivector;
    fn transform(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: -2.0 * self.s * other.e01 * other.e23 - 2.0 * self.s * other.e02 * other.e31 - 2.0 * self.s * other.e12 * other.e03,
        }
    }
}

impl Transform<Trivector> for Scalar {
    type Output = EvenMultivector;
    fn transform(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e021 * other.e021 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Transform<FourVector> for Scalar {
    type Output = EvenMultivector;
    fn transform(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0123 * other.e0123,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Transform<Null> for Scalar {
    type Output = Null;
    fn transform(self, other: Null) -> Null {
        Null
    }
}

impl Transform<OddMultivector> for Scalar {
    type Output = EvenMultivector;
    fn transform(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: -2.0 * self.s * other.e0 * other.e123 - 2.0 * self.s * other.e1 * other.e032 - 2.0 * self.s * other.e2 * other.e013 - 2.0 * self.s * other.e021 * other.e3,
        }
    }
}

impl Transform<EvenMultivector> for Scalar {
    type Output = EvenMultivector;
    fn transform(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s * other.s + self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23 + self.s * other.e0123 * other.e0123,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 2.0 * self.s * other.s * other.e0123 - 2.0 * self.s * other.e01 * other.e23 - 2.0 * self.s * other.e02 * other.e31 - 2.0 * self.s * other.e12 * other.e03,
        }
    }
}

impl Transform<Multivector> for Scalar {
    type Output = Multivector;
    fn transform(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01 + self.s * other.e2 * other.e2 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e013 * other.e013 + self.s * other.e23 * other.e23 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 + self.s * other.e0123 * other.e0123,
            e0: 2.0 * self.s * other.s * other.e0 + 2.0 * self.s * other.e1 * other.e01 + 2.0 * self.s * other.e2 * other.e02 - 2.0 * self.s * other.e12 * other.e021 + 2.0 * self.s * other.e3 * other.e03 - 2.0 * self.s * other.e31 * other.e013 - 2.0 * self.s * other.e23 * other.e032 + 2.0 * self.s * other.e123 * other.e0123,
            e1: 2.0 * self.s * other.s * other.e1 - 2.0 * self.s * other.e0 * other.e01 + 2.0 * self.s * other.e2 * other.e12 + 2.0 * self.s * other.e02 * other.e021 - 2.0 * self.s * other.e3 * other.e31 - 2.0 * self.s * other.e03 * other.e013 + 2.0 * self.s * other.e23 * other.e123 + 2.0 * self.s * other.e032 * other.e0123,
            e01: 0.0,
            e2: 2.0 * self.s * other.s * other.e2 - 2.0 * self.s * other.e0 * other.e02 - 2.0 * self.s * other.e1 * other.e12 - 2.0 * self.s * other.e01 * other.e021 + 2.0 * self.s * other.e3 * other.e23 + 2.0 * self.s * other.e03 * other.e032 + 2.0 * self.s * other.e31 * other.e123 + 2.0 * self.s * other.e013 * other.e0123,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 2.0 * self.s * other.s * other.e3 - 2.0 * self.s * other.e0 * other.e03 + 2.0 * self.s * other.e1 * other.e31 + 2.0 * self.s * other.e01 * other.e013 - 2.0 * self.s * other.e2 * other.e23 - 2.0 * self.s * other.e02 * other.e032 + 2.0 * self.s * other.e12 * other.e123 + 2.0 * self.s * other.e021 * other.e0123,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 2.0 * self.s * other.s * other.e0123 - 2.0 * self.s * other.e0 * other.e123 - 2.0 * self.s * other.e1 * other.e032 - 2.0 * self.s * other.e01 * other.e23 - 2.0 * self.s * other.e2 * other.e013 - 2.0 * self.s * other.e02 * other.e31 - 2.0 * self.s * other.e12 * other.e03 - 2.0 * self.s * other.e021 * other.e3,
        }
    }
}

impl Transform<Scalar> for Vector {
    type Output = OddMultivector;
    fn transform(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s * other.s,
            e1: self.e1 * other.s * other.s,
            e2: self.e2 * other.s * other.s,
            e021: 0.0,
            e3: self.e3 * other.s * other.s,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Transform<Vector> for Vector {
    type Output = OddMultivector;
    fn transform(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 - self.e0 * other.e2 * other.e2 - self.e0 * other.e3 * other.e3 + 2.0 * self.e1 * other.e0 * other.e1 + 2.0 * self.e2 * other.e0 * other.e2 + 2.0 * self.e3 * other.e0 * other.e3,
            e1: 2.0 * self.e0 * other.e0 * other.e1 - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1 - self.e1 * other.e2 * other.e2 - self.e1 * other.e3 * other.e3 + 2.0 * self.e2 * other.e1 * other.e2 + 2.0 * self.e3 * other.e1 * other.e3,
            e2: 2.0 * self.e0 * other.e0 * other.e2 + 2.0 * self.e1 * other.e1 * other.e2 - self.e2 * other.e0 * other.e0 - self.e2 * other.e1 * other.e1 + self.e2 * other.e2 * other.e2 - self.e2 * other.e3 * other.e3 + 2.0 * self.e3 * other.e2 * other.e3,
            e021: 0.0,
            e3: 2.0 * self.e0 * other.e0 * other.e3 + 2.0 * self.e1 * other.e1 * other.e3 + 2.0 * self.e2 * other.e2 * other.e3 - self.e3 * other.e0 * other.e0 - self.e3 * other.e1 * other.e1 - self.e3 * other.e2 * other.e2 + self.e3 * other.e3 * other.e3,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Transform<Bivector> for Vector {
    type Output = OddMultivector;
    fn transform(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.e01 * other.e01 + self.e0 * other.e02 * other.e02 + self.e0 * other.e12 * other.e12 + self.e0 * other.e03 * other.e03 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 - 2.0 * self.e1 * other.e02 * other.e12 + 2.0 * self.e1 * other.e03 * other.e31 + 2.0 * self.e2 * other.e01 * other.e12 - 2.0 * self.e2 * other.e03 * other.e23 - 2.0 * self.e3 * other.e01 * other.e31 + 2.0 * self.e3 * other.e02 * other.e23,
            e1: -2.0 * self.e0 * other.e02 * other.e12 + 2.0 * self.e0 * other.e03 * other.e31 - self.e1 * other.e01 * other.e01 + self.e1 * other.e02 * other.e02 - self.e1 * other.e12 * other.e12 + self.e1 * other.e03 * other.e03 - self.e1 * other.e31 * other.e31 + self.e1 * other.e23 * other.e23 - 2.0 * self.e2 * other.e01 * other.e02 + 2.0 * self.e2 * other.e31 * other.e23 - 2.0 * self.e3 * other.e01 * other.e03 + 2.0 * self.e3 * other.e12 * other.e23,
            e2: 2.0 * self.e0 * other.e01 * other.e12 - 2.0 * self.e0 * other.e03 * other.e23 - 2.0 * self.e1 * other.e01 * other.e02 + 2.0 * self.e1 * other.e31 * other.e23 + self.e2 * other.e01 * other.e01 - self.e2 * other.e02 * other.e02 - self.e2 * other.e12 * other.e12 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e2 * other.e23 * other.e23 - 2.0 * self.e3 * other.e02 * other.e03 + 2.0 * self.e3 * other.e12 * other.e31,
            e021: 0.0,
            e3: -2.0 * self.e0 * other.e01 * other.e31 + 2.0 * self.e0 * other.e02 * other.e23 - 2.0 * self.e1 * other.e01 * other.e03 + 2.0 * self.e1 * other.e12 * other.e23 - 2.0 * self.e2 * other.e02 * other.e03 + 2.0 * self.e2 * other.e12 * other.e31 + self.e3 * other.e01 * other.e01 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12 - self.e3 * other.e03 * other.e03 - self.e3 * other.e31 * other.e31 - self.e3 * other.e23 * other.e23,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Transform<Trivector> for Vector {
    type Output = OddMultivector;
    fn transform(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e021 * other.e021 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 - self.e0 * other.e123 * other.e123 - 2.0 * self.e1 * other.e032 * other.e123 - 2.0 * self.e2 * other.e013 * other.e123 - 2.0 * self.e3 * other.e021 * other.e123,
            e1: -2.0 * self.e0 * other.e032 * other.e123 + self.e1 * other.e021 * other.e021 + self.e1 * other.e013 * other.e013 - self.e1 * other.e032 * other.e032 + self.e1 * other.e123 * other.e123 - 2.0 * self.e2 * other.e013 * other.e032 - 2.0 * self.e3 * other.e021 * other.e032,
            e2: -2.0 * self.e0 * other.e013 * other.e123 - 2.0 * self.e1 * other.e013 * other.e032 + self.e2 * other.e021 * other.e021 - self.e2 * other.e013 * other.e013 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 - 2.0 * self.e3 * other.e021 * other.e013,
            e021: 0.0,
            e3: -2.0 * self.e0 * other.e021 * other.e123 - 2.0 * self.e1 * other.e021 * other.e032 - 2.0 * self.e2 * other.e021 * other.e013 - self.e3 * other.e021 * other.e021 + self.e3 * other.e013 * other.e013 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Transform<FourVector> for Vector {
    type Output = OddMultivector;
    fn transform(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.e0123 * other.e0123,
            e1: -self.e1 * other.e0123 * other.e0123,
            e2: -self.e2 * other.e0123 * other.e0123,
            e021: 0.0,
            e3: -self.e3 * other.e0123 * other.e0123,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Transform<Null> for Vector {
    type Output = Null;
    fn transform(self, other: Null) -> Null {
        Null
    }
}

impl Transform<OddMultivector> for Vector {
    type Output = OddMultivector;
    fn transform(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 - self.e0 * other.e2 * other.e2 - self.e0 * other.e021 * other.e021 - self.e0 * other.e3 * other.e3 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 - self.e0 * other.e123 * other.e123 + 2.0 * self.e1 * other.e0 * other.e1 + 2.0 * self.e1 * other.e2 * other.e021 - 2.0 * self.e1 * other.e3 * other.e013 - 2.0 * self.e1 * other.e032 * other.e123 + 2.0 * self.e2 * other.e0 * other.e2 - 2.0 * self.e2 * other.e1 * other.e021 + 2.0 * self.e2 * other.e3 * other.e032 - 2.0 * self.e2 * other.e013 * other.e123 + 2.0 * self.e3 * other.e0 * other.e3 + 2.0 * self.e3 * other.e1 * other.e013 - 2.0 * self.e3 * other.e2 * other.e032 - 2.0 * self.e3 * other.e021 * other.e123,
            e1: 2.0 * self.e0 * other.e0 * other.e1 - 2.0 * self.e0 * other.e2 * other.e021 + 2.0 * self.e0 * other.e3 * other.e013 - 2.0 * self.e0 * other.e032 * other.e123 - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1 - self.e1 * other.e2 * other.e2 + self.e1 * other.e021 * other.e021 - self.e1 * other.e3 * other.e3 + self.e1 * other.e013 * other.e013 - self.e1 * other.e032 * other.e032 + self.e1 * other.e123 * other.e123 + 2.0 * self.e2 * other.e0 * other.e021 + 2.0 * self.e2 * other.e1 * other.e2 - 2.0 * self.e2 * other.e3 * other.e123 - 2.0 * self.e2 * other.e013 * other.e032 - 2.0 * self.e3 * other.e0 * other.e013 + 2.0 * self.e3 * other.e1 * other.e3 + 2.0 * self.e3 * other.e2 * other.e123 - 2.0 * self.e3 * other.e021 * other.e032,
            e2: 2.0 * self.e0 * other.e0 * other.e2 + 2.0 * self.e0 * other.e1 * other.e021 - 2.0 * self.e0 * other.e3 * other.e032 - 2.0 * self.e0 * other.e013 * other.e123 - 2.0 * self.e1 * other.e0 * other.e021 + 2.0 * self.e1 * other.e1 * other.e2 + 2.0 * self.e1 * other.e3 * other.e123 - 2.0 * self.e1 * other.e013 * other.e032 - self.e2 * other.e0 * other.e0 - self.e2 * other.e1 * other.e1 + self.e2 * other.e2 * other.e2 + self.e2 * other.e021 * other.e021 - self.e2 * other.e3 * other.e3 - self.e2 * other.e013 * other.e013 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 + 2.0 * self.e3 * other.e0 * other.e032 - 2.0 * self.e3 * other.e1 * other.e123 + 2.0 * self.e3 * other.e2 * other.e3 - 2.0 * self.e3 * other.e021 * other.e013,
            e021: 0.0,
            e3: 2.0 * self.e0 * other.e0 * other.e3 - 2.0 * self.e0 * other.e1 * other.e013 + 2.0 * self.e0 * other.e2 * other.e032 - 2.0 * self.e0 * other.e021 * other.e123 + 2.0 * self.e1 * other.e0 * other.e013 + 2.0 * self.e1 * other.e1 * other.e3 - 2.0 * self.e1 * other.e2 * other.e123 - 2.0 * self.e1 * other.e021 * other.e032 - 2.0 * self.e2 * other.e0 * other.e032 + 2.0 * self.e2 * other.e1 * other.e123 + 2.0 * self.e2 * other.e2 * other.e3 - 2.0 * self.e2 * other.e021 * other.e013 - self.e3 * other.e0 * other.e0 - self.e3 * other.e1 * other.e1 - self.e3 * other.e2 * other.e2 - self.e3 * other.e021 * other.e021 + self.e3 * other.e3 * other.e3 + self.e3 * other.e013 * other.e013 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Transform<EvenMultivector> for Vector {
    type Output = OddMultivector;
    fn transform(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s * other.s + self.e0 * other.e01 * other.e01 + self.e0 * other.e02 * other.e02 + self.e0 * other.e12 * other.e12 + self.e0 * other.e03 * other.e03 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 + self.e0 * other.e0123 * other.e0123 + 2.0 * self.e1 * other.s * other.e01 - 2.0 * self.e1 * other.e02 * other.e12 + 2.0 * self.e1 * other.e03 * other.e31 + 2.0 * self.e1 * other.e23 * other.e0123 + 2.0 * self.e2 * other.s * other.e02 + 2.0 * self.e2 * other.e01 * other.e12 - 2.0 * self.e2 * other.e03 * other.e23 + 2.0 * self.e2 * other.e31 * other.e0123 + 2.0 * self.e3 * other.s * other.e03 - 2.0 * self.e3 * other.e01 * other.e31 + 2.0 * self.e3 * other.e02 * other.e23 + 2.0 * self.e3 * other.e12 * other.e0123,
            e1: -2.0 * self.e0 * other.s * other.e01 - 2.0 * self.e0 * other.e02 * other.e12 + 2.0 * self.e0 * other.e03 * other.e31 - 2.0 * self.e0 * other.e23 * other.e0123 + self.e1 * other.s * other.s - self.e1 * other.e01 * other.e01 + self.e1 * other.e02 * other.e02 - self.e1 * other.e12 * other.e12 + self.e1 * other.e03 * other.e03 - self.e1 * other.e31 * other.e31 + self.e1 * other.e23 * other.e23 - self.e1 * other.e0123 * other.e0123 + 2.0 * self.e2 * other.s * other.e12 - 2.0 * self.e2 * other.e01 * other.e02 + 2.0 * self.e2 * other.e03 * other.e0123 + 2.0 * self.e2 * other.e31 * other.e23 - 2.0 * self.e3 * other.s * other.e31 - 2.0 * self.e3 * other.e01 * other.e03 - 2.0 * self.e3 * other.e02 * other.e0123 + 2.0 * self.e3 * other.e12 * other.e23,
            e2: -2.0 * self.e0 * other.s * other.e02 + 2.0 * self.e0 * other.e01 * other.e12 - 2.0 * self.e0 * other.e03 * other.e23 - 2.0 * self.e0 * other.e31 * other.e0123 - 2.0 * self.e1 * other.s * other.e12 - 2.0 * self.e1 * other.e01 * other.e02 - 2.0 * self.e1 * other.e03 * other.e0123 + 2.0 * self.e1 * other.e31 * other.e23 + self.e2 * other.s * other.s + self.e2 * other.e01 * other.e01 - self.e2 * other.e02 * other.e02 - self.e2 * other.e12 * other.e12 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e2 * other.e23 * other.e23 - self.e2 * other.e0123 * other.e0123 + 2.0 * self.e3 * other.s * other.e23 + 2.0 * self.e3 * other.e01 * other.e0123 - 2.0 * self.e3 * other.e02 * other.e03 + 2.0 * self.e3 * other.e12 * other.e31,
            e021: 0.0,
            e3: -2.0 * self.e0 * other.s * other.e03 - 2.0 * self.e0 * other.e01 * other.e31 + 2.0 * self.e0 * other.e02 * other.e23 - 2.0 * self.e0 * other.e12 * other.e0123 + 2.0 * self.e1 * other.s * other.e31 - 2.0 * self.e1 * other.e01 * other.e03 + 2.0 * self.e1 * other.e02 * other.e0123 + 2.0 * self.e1 * other.e12 * other.e23 - 2.0 * self.e2 * other.s * other.e23 - 2.0 * self.e2 * other.e01 * other.e0123 - 2.0 * self.e2 * other.e02 * other.e03 + 2.0 * self.e2 * other.e12 * other.e31 + self.e3 * other.s * other.s + self.e3 * other.e01 * other.e01 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12 - self.e3 * other.e03 * other.e03 - self.e3 * other.e31 * other.e31 - self.e3 * other.e23 * other.e23 - self.e3 * other.e0123 * other.e0123,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Transform<Multivector> for Vector {
    type Output = Multivector;
    fn transform(self, other: Multivector) -> Multivector {
        Multivector {
            s: 2.0 * self.e0 * other.s * other.e0 - 2.0 * self.e0 * other.e1 * other.e01 - 2.0 * self.e0 * other.e2 * other.e02 - 2.0 * self.e0 * other.e12 * other.e021 - 2.0 * self.e0 * other.e3 * other.e03 - 2.0 * self.e0 * other.e31 * other.e013 - 2.0 * self.e0 * other.e23 * other.e032 - 2.0 * self.e0 * other.e123 * other.e0123 + 2.0 * self.e1 * other.s * other.e1 + 2.0 * self.e1 * other.e0 * other.e01 - 2.0 * self.e1 * other.e2 * other.e12 + 2.0 * self.e1 * other.e02 * other.e021 + 2.0 * self.e1 * other.e3 * other.e31 - 2.0 * self.e1 * other.e03 * other.e013 + 2.0 * self.e1 * other.e23 * other.e123 - 2.0 * self.e1 * other.e032 * other.e0123 + 2.0 * self.e2 * other.s * other.e2 + 2.0 * self.e2 * other.e0 * other.e02 + 2.0 * self.e2 * other.e1 * other.e12 - 2.0 * self.e2 * other.e01 * other.e021 - 2.0 * self.e2 * other.e3 * other.e23 + 2.0 * self.e2 * other.e03 * other.e032 + 2.0 * self.e2 * other.e31 * other.e123 - 2.0 * self.e2 * other.e013 * other.e0123 + 2.0 * self.e3 * other.s * other.e3 + 2.0 * self.e3 * other.e0 * other.e03 - 2.0 * self.e3 * other.e1 * other.e31 + 2.0 * self.e3 * other.e01 * other.e013 + 2.0 * self.e3 * other.e2 * other.e23 - 2.0 * self.e3 * other.e02 * other.e032 + 2.0 * self.e3 * other.e12 * other.e123 - 2.0 * self.e3 * other.e021 * other.e0123,
            e0: self.e0 * other.s * other.s - self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 + self.e0 * other.e01 * other.e01 - self.e0 * other.e2 * other.e2 + self.e0 * other.e02 * other.e02 + self.e0 * other.e12 * other.e12 - self.e0 * other.e021 * other.e021 - self.e0 * other.e3 * other.e3 + self.e0 * other.e03 * other.e03 + self.e0 * other.e31 * other.e31 - self.e0 * other.e013 * other.e013 + self.e0 * other.e23 * other.e23 - self.e0 * other.e032 * other.e032 - self.e0 * other.e123 * other.e123 + self.e0 * other.e0123 * other.e0123 + 2.0 * self.e1 * other.s * other.e01 + 2.0 * self.e1 * other.e0 * other.e1 + 2.0 * self.e1 * other.e2 * other.e021 - 2.0 * self.e1 * other.e02 * other.e12 - 2.0 * self.e1 * other.e3 * other.e013 + 2.0 * self.e1 * other.e03 * other.e31 + 2.0 * self.e1 * other.e23 * other.e0123 - 2.0 * self.e1 * other.e032 * other.e123 + 2.0 * self.e2 * other.s * other.e02 + 2.0 * self.e2 * other.e0 * other.e2 - 2.0 * self.e2 * other.e1 * other.e021 + 2.0 * self.e2 * other.e01 * other.e12 + 2.0 * self.e2 * other.e3 * other.e032 - 2.0 * self.e2 * other.e03 * other.e23 + 2.0 * self.e2 * other.e31 * other.e0123 - 2.0 * self.e2 * other.e013 * other.e123 + 2.0 * self.e3 * other.s * other.e03 + 2.0 * self.e3 * other.e0 * other.e3 + 2.0 * self.e3 * other.e1 * other.e013 - 2.0 * self.e3 * other.e01 * other.e31 - 2.0 * self.e3 * other.e2 * other.e032 + 2.0 * self.e3 * other.e02 * other.e23 + 2.0 * self.e3 * other.e12 * other.e0123 - 2.0 * self.e3 * other.e021 * other.e123,
            e1: -2.0 * self.e0 * other.s * other.e01 + 2.0 * self.e0 * other.e0 * other.e1 - 2.0 * self.e0 * other.e2 * other.e021 - 2.0 * self.e0 * other.e02 * other.e12 + 2.0 * self.e0 * other.e3 * other.e013 + 2.0 * self.e0 * other.e03 * other.e31 - 2.0 * self.e0 * other.e23 * other.e0123 - 2.0 * self.e0 * other.e032 * other.e123 + self.e1 * other.s * other.s - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1 - self.e1 * other.e01 * other.e01 - self.e1 * other.e2 * other.e2 + self.e1 * other.e02 * other.e02 - self.e1 * other.e12 * other.e12 + self.e1 * other.e021 * other.e021 - self.e1 * other.e3 * other.e3 + self.e1 * other.e03 * other.e03 - self.e1 * other.e31 * other.e31 + self.e1 * other.e013 * other.e013 + self.e1 * other.e23 * other.e23 - self.e1 * other.e032 * other.e032 + self.e1 * other.e123 * other.e123 - self.e1 * other.e0123 * other.e0123 + 2.0 * self.e2 * other.s * other.e12 + 2.0 * self.e2 * other.e0 * other.e021 + 2.0 * self.e2 * other.e1 * other.e2 - 2.0 * self.e2 * other.e01 * other.e02 - 2.0 * self.e2 * other.e3 * other.e123 + 2.0 * self.e2 * other.e03 * other.e0123 + 2.0 * self.e2 * other.e31 * other.e23 - 2.0 * self.e2 * other.e013 * other.e032 - 2.0 * self.e3 * other.s * other.e31 - 2.0 * self.e3 * other.e0 * other.e013 + 2.0 * self.e3 * other.e1 * other.e3 - 2.0 * self.e3 * other.e01 * other.e03 + 2.0 * self.e3 * other.e2 * other.e123 - 2.0 * self.e3 * other.e02 * other.e0123 + 2.0 * self.e3 * other.e12 * other.e23 - 2.0 * self.e3 * other.e021 * other.e032,
            e01: 0.0,
            e2: -2.0 * self.e0 * other.s * other.e02 + 2.0 * self.e0 * other.e0 * other.e2 + 2.0 * self.e0 * other.e1 * other.e021 + 2.0 * self.e0 * other.e01 * other.e12 - 2.0 * self.e0 * other.e3 * other.e032 - 2.0 * self.e0 * other.e03 * other.e23 - 2.0 * self.e0 * other.e31 * other.e0123 - 2.0 * self.e0 * other.e013 * other.e123 - 2.0 * self.e1 * other.s * other.e12 - 2.0 * self.e1 * other.e0 * other.e021 + 2.0 * self.e1 * other.e1 * other.e2 - 2.0 * self.e1 * other.e01 * other.e02 + 2.0 * self.e1 * other.e3 * other.e123 - 2.0 * self.e1 * other.e03 * other.e0123 + 2.0 * self.e1 * other.e31 * other.e23 - 2.0 * self.e1 * other.e013 * other.e032 + self.e2 * other.s * other.s - self.e2 * other.e0 * other.e0 - self.e2 * other.e1 * other.e1 + self.e2 * other.e01 * other.e01 + self.e2 * other.e2 * other.e2 - self.e2 * other.e02 * other.e02 - self.e2 * other.e12 * other.e12 + self.e2 * other.e021 * other.e021 - self.e2 * other.e3 * other.e3 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e2 * other.e013 * other.e013 - self.e2 * other.e23 * other.e23 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 - self.e2 * other.e0123 * other.e0123 + 2.0 * self.e3 * other.s * other.e23 + 2.0 * self.e3 * other.e0 * other.e032 - 2.0 * self.e3 * other.e1 * other.e123 + 2.0 * self.e3 * other.e01 * other.e0123 + 2.0 * self.e3 * other.e2 * other.e3 - 2.0 * self.e3 * other.e02 * other.e03 + 2.0 * self.e3 * other.e12 * other.e31 - 2.0 * self.e3 * other.e021 * other.e013,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: -2.0 * self.e0 * other.s * other.e03 + 2.0 * self.e0 * other.e0 * other.e3 - 2.0 * self.e0 * other.e1 * other.e013 - 2.0 * self.e0 * other.e01 * other.e31 + 2.0 * self.e0 * other.e2 * other.e032 + 2.0 * self.e0 * other.e02 * other.e23 - 2.0 * self.e0 * other.e12 * other.e0123 - 2.0 * self.e0 * other.e021 * other.e123 + 2.0 * self.e1 * other.s * other.e31 + 2.0 * self.e1 * other.e0 * other.e013 + 2.0 * self.e1 * other.e1 * other.e3 - 2.0 * self.e1 * other.e01 * other.e03 - 2.0 * self.e1 * other.e2 * other.e123 + 2.0 * self.e1 * other.e02 * other.e0123 + 2.0 * self.e1 * other.e12 * other.e23 - 2.0 * self.e1 * other.e021 * other.e032 - 2.0 * self.e2 * other.s * other.e23 - 2.0 * self.e2 * other.e0 * other.e032 + 2.0 * self.e2 * other.e1 * other.e123 - 2.0 * self.e2 * other.e01 * other.e0123 + 2.0 * self.e2 * other.e2 * other.e3 - 2.0 * self.e2 * other.e02 * other.e03 + 2.0 * self.e2 * other.e12 * other.e31 - 2.0 * self.e2 * other.e021 * other.e013 + self.e3 * other.s * other.s - self.e3 * other.e0 * other.e0 - self.e3 * other.e1 * other.e1 + self.e3 * other.e01 * other.e01 - self.e3 * other.e2 * other.e2 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12 - self.e3 * other.e021 * other.e021 + self.e3 * other.e3 * other.e3 - self.e3 * other.e03 * other.e03 - self.e3 * other.e31 * other.e31 + self.e3 * other.e013 * other.e013 - self.e3 * other.e23 * other.e23 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123 - self.e3 * other.e0123 * other.e0123,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: -2.0 * self.e0 * other.s * other.e123 - 2.0 * self.e0 * other.e0 * other.e0123 + 2.0 * self.e0 * other.e1 * other.e23 - 2.0 * self.e0 * other.e01 * other.e032 + 2.0 * self.e0 * other.e2 * other.e31 - 2.0 * self.e0 * other.e02 * other.e013 + 2.0 * self.e0 * other.e12 * other.e3 - 2.0 * self.e0 * other.e021 * other.e03 - 2.0 * self.e1 * other.s * other.e032 - 2.0 * self.e1 * other.e0 * other.e23 + 2.0 * self.e1 * other.e1 * other.e0123 - 2.0 * self.e1 * other.e01 * other.e123 + 2.0 * self.e1 * other.e2 * other.e03 - 2.0 * self.e1 * other.e02 * other.e3 + 2.0 * self.e1 * other.e12 * other.e013 - 2.0 * self.e1 * other.e021 * other.e31 - 2.0 * self.e2 * other.s * other.e013 - 2.0 * self.e2 * other.e0 * other.e31 - 2.0 * self.e2 * other.e1 * other.e03 + 2.0 * self.e2 * other.e01 * other.e3 + 2.0 * self.e2 * other.e2 * other.e0123 - 2.0 * self.e2 * other.e02 * other.e123 - 2.0 * self.e2 * other.e12 * other.e032 + 2.0 * self.e2 * other.e021 * other.e23 - 2.0 * self.e3 * other.s * other.e021 - 2.0 * self.e3 * other.e0 * other.e12 + 2.0 * self.e3 * other.e1 * other.e02 - 2.0 * self.e3 * other.e01 * other.e2 + 2.0 * self.e3 * other.e3 * other.e0123 - 2.0 * self.e3 * other.e03 * other.e123 + 2.0 * self.e3 * other.e31 * other.e032 - 2.0 * self.e3 * other.e013 * other.e23,
        }
    }
}

impl Transform<Scalar> for Bivector {
    type Output = EvenMultivector;
    fn transform(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e01 * other.s * other.s,
            e02: self.e02 * other.s * other.s,
            e12: self.e12 * other.s * other.s,
            e03: self.e03 * other.s * other.s,
            e31: self.e31 * other.s * other.s,
            e23: self.e23 * other.s * other.s,
            e0123: 0.0,
        }
    }
}

impl Transform<Vector> for Bivector {
    type Output = EvenMultivector;
    fn transform(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1 + self.e01 * other.e2 * other.e2 + self.e01 * other.e3 * other.e3 - 2.0 * self.e02 * other.e1 * other.e2 + 2.0 * self.e12 * other.e0 * other.e2 - 2.0 * self.e03 * other.e1 * other.e3 - 2.0 * self.e31 * other.e0 * other.e3,
            e02: -2.0 * self.e01 * other.e1 * other.e2 + self.e02 * other.e0 * other.e0 + self.e02 * other.e1 * other.e1 - self.e02 * other.e2 * other.e2 + self.e02 * other.e3 * other.e3 - 2.0 * self.e12 * other.e0 * other.e1 - 2.0 * self.e03 * other.e2 * other.e3 + 2.0 * self.e23 * other.e0 * other.e3,
            e12: 2.0 * self.e01 * other.e0 * other.e2 - 2.0 * self.e02 * other.e0 * other.e1 + self.e12 * other.e0 * other.e0 - self.e12 * other.e1 * other.e1 - self.e12 * other.e2 * other.e2 + self.e12 * other.e3 * other.e3 + 2.0 * self.e31 * other.e2 * other.e3 + 2.0 * self.e23 * other.e1 * other.e3,
            e03: -2.0 * self.e01 * other.e1 * other.e3 - 2.0 * self.e02 * other.e2 * other.e3 + self.e03 * other.e0 * other.e0 + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 - self.e03 * other.e3 * other.e3 + 2.0 * self.e31 * other.e0 * other.e1 - 2.0 * self.e23 * other.e0 * other.e2,
            e31: -2.0 * self.e01 * other.e0 * other.e3 + 2.0 * self.e12 * other.e2 * other.e3 + 2.0 * self.e03 * other.e0 * other.e1 + self.e31 * other.e0 * other.e0 - self.e31 * other.e1 * other.e1 + self.e31 * other.e2 * other.e2 - self.e31 * other.e3 * other.e3 + 2.0 * self.e23 * other.e1 * other.e2,
            e23: 2.0 * self.e02 * other.e0 * other.e3 + 2.0 * self.e12 * other.e1 * other.e3 - 2.0 * self.e03 * other.e0 * other.e2 + 2.0 * self.e31 * other.e1 * other.e2 + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1 - self.e23 * other.e2 * other.e2 - self.e23 * other.e3 * other.e3,
            e0123: 0.0,
        }
    }
}

impl Transform<Bivector> for Bivector {
    type Output = EvenMultivector;
    fn transform(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e01 * other.e01 * other.e01 + self.e01 * other.e02 * other.e02 - self.e01 * other.e12 * other.e12 + self.e01 * other.e03 * other.e03 - self.e01 * other.e31 * other.e31 + self.e01 * other.e23 * other.e23 - 2.0 * self.e02 * other.e01 * other.e02 + 2.0 * self.e02 * other.e31 * other.e23 + 2.0 * self.e12 * other.e01 * other.e12 + 2.0 * self.e12 * other.e03 * other.e23 - 2.0 * self.e03 * other.e01 * other.e03 + 2.0 * self.e03 * other.e12 * other.e23 + 2.0 * self.e31 * other.e01 * other.e31 + 2.0 * self.e31 * other.e02 * other.e23 + 2.0 * self.e23 * other.e01 * other.e23 - 2.0 * self.e23 * other.e02 * other.e31 - 2.0 * self.e23 * other.e12 * other.e03,
            e02: -2.0 * self.e01 * other.e01 * other.e02 + 2.0 * self.e01 * other.e31 * other.e23 + self.e02 * other.e01 * other.e01 - self.e02 * other.e02 * other.e02 - self.e02 * other.e12 * other.e12 + self.e02 * other.e03 * other.e03 + self.e02 * other.e31 * other.e31 - self.e02 * other.e23 * other.e23 + 2.0 * self.e12 * other.e02 * other.e12 + 2.0 * self.e12 * other.e03 * other.e31 - 2.0 * self.e03 * other.e02 * other.e03 + 2.0 * self.e03 * other.e12 * other.e31 - 2.0 * self.e31 * other.e01 * other.e23 + 2.0 * self.e31 * other.e02 * other.e31 - 2.0 * self.e31 * other.e12 * other.e03 + 2.0 * self.e23 * other.e01 * other.e31 + 2.0 * self.e23 * other.e02 * other.e23,
            e12: 2.0 * self.e01 * other.e01 * other.e12 + 2.0 * self.e01 * other.e03 * other.e23 + 2.0 * self.e02 * other.e02 * other.e12 + 2.0 * self.e02 * other.e03 * other.e31 - self.e12 * other.e01 * other.e01 - self.e12 * other.e02 * other.e02 + self.e12 * other.e12 * other.e12 + self.e12 * other.e03 * other.e03 - self.e12 * other.e31 * other.e31 - self.e12 * other.e23 * other.e23 - 2.0 * self.e03 * other.e01 * other.e23 - 2.0 * self.e03 * other.e02 * other.e31 + 2.0 * self.e03 * other.e12 * other.e03 + 2.0 * self.e31 * other.e02 * other.e03 + 2.0 * self.e31 * other.e12 * other.e31 + 2.0 * self.e23 * other.e01 * other.e03 + 2.0 * self.e23 * other.e12 * other.e23,
            e03: -2.0 * self.e01 * other.e01 * other.e03 + 2.0 * self.e01 * other.e12 * other.e23 - 2.0 * self.e02 * other.e02 * other.e03 + 2.0 * self.e02 * other.e12 * other.e31 - 2.0 * self.e12 * other.e01 * other.e23 - 2.0 * self.e12 * other.e02 * other.e31 + 2.0 * self.e12 * other.e12 * other.e03 + self.e03 * other.e01 * other.e01 + self.e03 * other.e02 * other.e02 + self.e03 * other.e12 * other.e12 - self.e03 * other.e03 * other.e03 - self.e03 * other.e31 * other.e31 - self.e03 * other.e23 * other.e23 + 2.0 * self.e31 * other.e02 * other.e12 + 2.0 * self.e31 * other.e03 * other.e31 + 2.0 * self.e23 * other.e01 * other.e12 + 2.0 * self.e23 * other.e03 * other.e23,
            e31: 2.0 * self.e01 * other.e01 * other.e31 + 2.0 * self.e01 * other.e02 * other.e23 - 2.0 * self.e02 * other.e01 * other.e23 + 2.0 * self.e02 * other.e02 * other.e31 - 2.0 * self.e02 * other.e12 * other.e03 + 2.0 * self.e12 * other.e02 * other.e03 + 2.0 * self.e12 * other.e12 * other.e31 + 2.0 * self.e03 * other.e02 * other.e12 + 2.0 * self.e03 * other.e03 * other.e31 - self.e31 * other.e01 * other.e01 + self.e31 * other.e02 * other.e02 - self.e31 * other.e12 * other.e12 - self.e31 * other.e03 * other.e03 + self.e31 * other.e31 * other.e31 - self.e31 * other.e23 * other.e23 + 2.0 * self.e23 * other.e01 * other.e02 + 2.0 * self.e23 * other.e31 * other.e23,
            e23: 2.0 * self.e01 * other.e01 * other.e23 - 2.0 * self.e01 * other.e02 * other.e31 - 2.0 * self.e01 * other.e12 * other.e03 + 2.0 * self.e02 * other.e01 * other.e31 + 2.0 * self.e02 * other.e02 * other.e23 + 2.0 * self.e12 * other.e01 * other.e03 + 2.0 * self.e12 * other.e12 * other.e23 + 2.0 * self.e03 * other.e01 * other.e12 + 2.0 * self.e03 * other.e03 * other.e23 + 2.0 * self.e31 * other.e01 * other.e02 + 2.0 * self.e31 * other.e31 * other.e23 + self.e23 * other.e01 * other.e01 - self.e23 * other.e02 * other.e02 - self.e23 * other.e12 * other.e12 - self.e23 * other.e03 * other.e03 - self.e23 * other.e31 * other.e31 + self.e23 * other.e23 * other.e23,
            e0123: 0.0,
        }
    }
}

impl Transform<Trivector> for Bivector {
    type Output = EvenMultivector;
    fn transform(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e01 * other.e021 * other.e021 - self.e01 * other.e013 * other.e013 + self.e01 * other.e032 * other.e032 - self.e01 * other.e123 * other.e123 + 2.0 * self.e02 * other.e013 * other.e032 + 2.0 * self.e12 * other.e013 * other.e123 + 2.0 * self.e03 * other.e021 * other.e032 - 2.0 * self.e31 * other.e021 * other.e123,
            e02: 2.0 * self.e01 * other.e013 * other.e032 - self.e02 * other.e021 * other.e021 + self.e02 * other.e013 * other.e013 - self.e02 * other.e032 * other.e032 - self.e02 * other.e123 * other.e123 - 2.0 * self.e12 * other.e032 * other.e123 + 2.0 * self.e03 * other.e021 * other.e013 + 2.0 * self.e23 * other.e021 * other.e123,
            e12: 2.0 * self.e01 * other.e013 * other.e123 - 2.0 * self.e02 * other.e032 * other.e123 + self.e12 * other.e021 * other.e021 - self.e12 * other.e013 * other.e013 - self.e12 * other.e032 * other.e032 + self.e12 * other.e123 * other.e123 + 2.0 * self.e31 * other.e021 * other.e013 + 2.0 * self.e23 * other.e021 * other.e032,
            e03: 2.0 * self.e01 * other.e021 * other.e032 + 2.0 * self.e02 * other.e021 * other.e013 + self.e03 * other.e021 * other.e021 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 - self.e03 * other.e123 * other.e123 + 2.0 * self.e31 * other.e032 * other.e123 - 2.0 * self.e23 * other.e013 * other.e123,
            e31: -2.0 * self.e01 * other.e021 * other.e123 + 2.0 * self.e12 * other.e021 * other.e013 + 2.0 * self.e03 * other.e032 * other.e123 - self.e31 * other.e021 * other.e021 + self.e31 * other.e013 * other.e013 - self.e31 * other.e032 * other.e032 + self.e31 * other.e123 * other.e123 + 2.0 * self.e23 * other.e013 * other.e032,
            e23: 2.0 * self.e02 * other.e021 * other.e123 + 2.0 * self.e12 * other.e021 * other.e032 - 2.0 * self.e03 * other.e013 * other.e123 + 2.0 * self.e31 * other.e013 * other.e032 - self.e23 * other.e021 * other.e021 - self.e23 * other.e013 * other.e013 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123,
            e0123: 0.0,
        }
    }
}

impl Transform<FourVector> for Bivector {
    type Output = EvenMultivector;
    fn transform(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: -self.e01 * other.e0123 * other.e0123,
            e02: -self.e02 * other.e0123 * other.e0123,
            e12: self.e12 * other.e0123 * other.e0123,
            e03: -self.e03 * other.e0123 * other.e0123,
            e31: self.e31 * other.e0123 * other.e0123,
            e23: self.e23 * other.e0123 * other.e0123,
            e0123: 0.0,
        }
    }
}

impl Transform<Null> for Bivector {
    type Output = Null;
    fn transform(self, other: Null) -> Null {
        Null
    }
}

impl Transform<OddMultivector> for Bivector {
    type Output = EvenMultivector;
    fn transform(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1 + self.e01 * other.e2 * other.e2 - self.e01 * other.e021 * other.e021 + self.e01 * other.e3 * other.e3 - self.e01 * other.e013 * other.e013 + self.e01 * other.e032 * other.e032 - self.e01 * other.e123 * other.e123 - 2.0 * self.e02 * other.e0 * other.e021 - 2.0 * self.e02 * other.e1 * other.e2 + 2.0 * self.e02 * other.e3 * other.e123 + 2.0 * self.e02 * other.e013 * other.e032 + 2.0 * self.e12 * other.e0 * other.e2 + 2.0 * self.e12 * other.e1 * other.e021 + 2.0 * self.e12 * other.e3 * other.e032 + 2.0 * self.e12 * other.e013 * other.e123 + 2.0 * self.e03 * other.e0 * other.e013 - 2.0 * self.e03 * other.e1 * other.e3 - 2.0 * self.e03 * other.e2 * other.e123 + 2.0 * self.e03 * other.e021 * other.e032 - 2.0 * self.e31 * other.e0 * other.e3 + 2.0 * self.e31 * other.e1 * other.e013 + 2.0 * self.e31 * other.e2 * other.e032 - 2.0 * self.e31 * other.e021 * other.e123 + 2.0 * self.e23 * other.e0 * other.e123 + 2.0 * self.e23 * other.e1 * other.e032 - 2.0 * self.e23 * other.e2 * other.e013 - 2.0 * self.e23 * other.e021 * other.e3,
            e02: 2.0 * self.e01 * other.e0 * other.e021 - 2.0 * self.e01 * other.e1 * other.e2 - 2.0 * self.e01 * other.e3 * other.e123 + 2.0 * self.e01 * other.e013 * other.e032 + self.e02 * other.e0 * other.e0 + self.e02 * other.e1 * other.e1 - self.e02 * other.e2 * other.e2 - self.e02 * other.e021 * other.e021 + self.e02 * other.e3 * other.e3 + self.e02 * other.e013 * other.e013 - self.e02 * other.e032 * other.e032 - self.e02 * other.e123 * other.e123 - 2.0 * self.e12 * other.e0 * other.e1 + 2.0 * self.e12 * other.e2 * other.e021 + 2.0 * self.e12 * other.e3 * other.e013 - 2.0 * self.e12 * other.e032 * other.e123 - 2.0 * self.e03 * other.e0 * other.e032 + 2.0 * self.e03 * other.e1 * other.e123 - 2.0 * self.e03 * other.e2 * other.e3 + 2.0 * self.e03 * other.e021 * other.e013 + 2.0 * self.e31 * other.e0 * other.e123 - 2.0 * self.e31 * other.e1 * other.e032 + 2.0 * self.e31 * other.e2 * other.e013 - 2.0 * self.e31 * other.e021 * other.e3 + 2.0 * self.e23 * other.e0 * other.e3 + 2.0 * self.e23 * other.e1 * other.e013 + 2.0 * self.e23 * other.e2 * other.e032 + 2.0 * self.e23 * other.e021 * other.e123,
            e12: 2.0 * self.e01 * other.e0 * other.e2 - 2.0 * self.e01 * other.e1 * other.e021 - 2.0 * self.e01 * other.e3 * other.e032 + 2.0 * self.e01 * other.e013 * other.e123 - 2.0 * self.e02 * other.e0 * other.e1 - 2.0 * self.e02 * other.e2 * other.e021 - 2.0 * self.e02 * other.e3 * other.e013 - 2.0 * self.e02 * other.e032 * other.e123 + self.e12 * other.e0 * other.e0 - self.e12 * other.e1 * other.e1 - self.e12 * other.e2 * other.e2 + self.e12 * other.e021 * other.e021 + self.e12 * other.e3 * other.e3 - self.e12 * other.e013 * other.e013 - self.e12 * other.e032 * other.e032 + self.e12 * other.e123 * other.e123 - 2.0 * self.e03 * other.e0 * other.e123 + 2.0 * self.e03 * other.e1 * other.e032 + 2.0 * self.e03 * other.e2 * other.e013 - 2.0 * self.e03 * other.e021 * other.e3 + 2.0 * self.e31 * other.e0 * other.e032 + 2.0 * self.e31 * other.e1 * other.e123 + 2.0 * self.e31 * other.e2 * other.e3 + 2.0 * self.e31 * other.e021 * other.e013 - 2.0 * self.e23 * other.e0 * other.e013 + 2.0 * self.e23 * other.e1 * other.e3 - 2.0 * self.e23 * other.e2 * other.e123 + 2.0 * self.e23 * other.e021 * other.e032,
            e03: -2.0 * self.e01 * other.e0 * other.e013 - 2.0 * self.e01 * other.e1 * other.e3 + 2.0 * self.e01 * other.e2 * other.e123 + 2.0 * self.e01 * other.e021 * other.e032 + 2.0 * self.e02 * other.e0 * other.e032 - 2.0 * self.e02 * other.e1 * other.e123 - 2.0 * self.e02 * other.e2 * other.e3 + 2.0 * self.e02 * other.e021 * other.e013 + 2.0 * self.e12 * other.e0 * other.e123 - 2.0 * self.e12 * other.e1 * other.e032 - 2.0 * self.e12 * other.e2 * other.e013 + 2.0 * self.e12 * other.e021 * other.e3 + self.e03 * other.e0 * other.e0 + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 + self.e03 * other.e021 * other.e021 - self.e03 * other.e3 * other.e3 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 - self.e03 * other.e123 * other.e123 + 2.0 * self.e31 * other.e0 * other.e1 + 2.0 * self.e31 * other.e2 * other.e021 + 2.0 * self.e31 * other.e3 * other.e013 + 2.0 * self.e31 * other.e032 * other.e123 - 2.0 * self.e23 * other.e0 * other.e2 + 2.0 * self.e23 * other.e1 * other.e021 + 2.0 * self.e23 * other.e3 * other.e032 - 2.0 * self.e23 * other.e013 * other.e123,
            e31: -2.0 * self.e01 * other.e0 * other.e3 - 2.0 * self.e01 * other.e1 * other.e013 - 2.0 * self.e01 * other.e2 * other.e032 - 2.0 * self.e01 * other.e021 * other.e123 - 2.0 * self.e02 * other.e0 * other.e123 + 2.0 * self.e02 * other.e1 * other.e032 - 2.0 * self.e02 * other.e2 * other.e013 + 2.0 * self.e02 * other.e021 * other.e3 - 2.0 * self.e12 * other.e0 * other.e032 - 2.0 * self.e12 * other.e1 * other.e123 + 2.0 * self.e12 * other.e2 * other.e3 + 2.0 * self.e12 * other.e021 * other.e013 + 2.0 * self.e03 * other.e0 * other.e1 - 2.0 * self.e03 * other.e2 * other.e021 - 2.0 * self.e03 * other.e3 * other.e013 + 2.0 * self.e03 * other.e032 * other.e123 + self.e31 * other.e0 * other.e0 - self.e31 * other.e1 * other.e1 + self.e31 * other.e2 * other.e2 - self.e31 * other.e021 * other.e021 - self.e31 * other.e3 * other.e3 + self.e31 * other.e013 * other.e013 - self.e31 * other.e032 * other.e032 + self.e31 * other.e123 * other.e123 + 2.0 * self.e23 * other.e0 * other.e021 + 2.0 * self.e23 * other.e1 * other.e2 + 2.0 * self.e23 * other.e3 * other.e123 + 2.0 * self.e23 * other.e013 * other.e032,
            e23: -2.0 * self.e01 * other.e0 * other.e123 - 2.0 * self.e01 * other.e1 * other.e032 + 2.0 * self.e01 * other.e2 * other.e013 + 2.0 * self.e01 * other.e021 * other.e3 + 2.0 * self.e02 * other.e0 * other.e3 - 2.0 * self.e02 * other.e1 * other.e013 - 2.0 * self.e02 * other.e2 * other.e032 + 2.0 * self.e02 * other.e021 * other.e123 + 2.0 * self.e12 * other.e0 * other.e013 + 2.0 * self.e12 * other.e1 * other.e3 + 2.0 * self.e12 * other.e2 * other.e123 + 2.0 * self.e12 * other.e021 * other.e032 - 2.0 * self.e03 * other.e0 * other.e2 - 2.0 * self.e03 * other.e1 * other.e021 - 2.0 * self.e03 * other.e3 * other.e032 - 2.0 * self.e03 * other.e013 * other.e123 - 2.0 * self.e31 * other.e0 * other.e021 + 2.0 * self.e31 * other.e1 * other.e2 - 2.0 * self.e31 * other.e3 * other.e123 + 2.0 * self.e31 * other.e013 * other.e032 + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1 - self.e23 * other.e2 * other.e2 - self.e23 * other.e021 * other.e021 - self.e23 * other.e3 * other.e3 - self.e23 * other.e013 * other.e013 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123,
            e0123: 0.0,
        }
    }
}

impl Transform<EvenMultivector> for Bivector {
    type Output = EvenMultivector;
    fn transform(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: self.e01 * other.s * other.s - self.e01 * other.e01 * other.e01 + self.e01 * other.e02 * other.e02 - self.e01 * other.e12 * other.e12 + self.e01 * other.e03 * other.e03 - self.e01 * other.e31 * other.e31 + self.e01 * other.e23 * other.e23 - self.e01 * other.e0123 * other.e0123 + 2.0 * self.e02 * other.s * other.e12 - 2.0 * self.e02 * other.e01 * other.e02 + 2.0 * self.e02 * other.e03 * other.e0123 + 2.0 * self.e02 * other.e31 * other.e23 - 2.0 * self.e12 * other.s * other.e02 + 2.0 * self.e12 * other.e01 * other.e12 + 2.0 * self.e12 * other.e03 * other.e23 + 2.0 * self.e12 * other.e31 * other.e0123 - 2.0 * self.e03 * other.s * other.e31 - 2.0 * self.e03 * other.e01 * other.e03 - 2.0 * self.e03 * other.e02 * other.e0123 + 2.0 * self.e03 * other.e12 * other.e23 + 2.0 * self.e31 * other.s * other.e03 + 2.0 * self.e31 * other.e01 * other.e31 + 2.0 * self.e31 * other.e02 * other.e23 - 2.0 * self.e31 * other.e12 * other.e0123 - 2.0 * self.e23 * other.s * other.e0123 + 2.0 * self.e23 * other.e01 * other.e23 - 2.0 * self.e23 * other.e02 * other.e31 - 2.0 * self.e23 * other.e12 * other.e03,
            e02: -2.0 * self.e01 * other.s * other.e12 - 2.0 * self.e01 * other.e01 * other.e02 - 2.0 * self.e01 * other.e03 * other.e0123 + 2.0 * self.e01 * other.e31 * other.e23 + self.e02 * other.s * other.s + self.e02 * other.e01 * other.e01 - self.e02 * other.e02 * other.e02 - self.e02 * other.e12 * other.e12 + self.e02 * other.e03 * other.e03 + self.e02 * other.e31 * other.e31 - self.e02 * other.e23 * other.e23 - self.e02 * other.e0123 * other.e0123 + 2.0 * self.e12 * other.s * other.e01 + 2.0 * self.e12 * other.e02 * other.e12 + 2.0 * self.e12 * other.e03 * other.e31 - 2.0 * self.e12 * other.e23 * other.e0123 + 2.0 * self.e03 * other.s * other.e23 + 2.0 * self.e03 * other.e01 * other.e0123 - 2.0 * self.e03 * other.e02 * other.e03 + 2.0 * self.e03 * other.e12 * other.e31 - 2.0 * self.e31 * other.s * other.e0123 - 2.0 * self.e31 * other.e01 * other.e23 + 2.0 * self.e31 * other.e02 * other.e31 - 2.0 * self.e31 * other.e12 * other.e03 - 2.0 * self.e23 * other.s * other.e03 + 2.0 * self.e23 * other.e01 * other.e31 + 2.0 * self.e23 * other.e02 * other.e23 + 2.0 * self.e23 * other.e12 * other.e0123,
            e12: 2.0 * self.e01 * other.s * other.e02 + 2.0 * self.e01 * other.e01 * other.e12 + 2.0 * self.e01 * other.e03 * other.e23 - 2.0 * self.e01 * other.e31 * other.e0123 - 2.0 * self.e02 * other.s * other.e01 + 2.0 * self.e02 * other.e02 * other.e12 + 2.0 * self.e02 * other.e03 * other.e31 + 2.0 * self.e02 * other.e23 * other.e0123 + self.e12 * other.s * other.s - self.e12 * other.e01 * other.e01 - self.e12 * other.e02 * other.e02 + self.e12 * other.e12 * other.e12 + self.e12 * other.e03 * other.e03 - self.e12 * other.e31 * other.e31 - self.e12 * other.e23 * other.e23 + self.e12 * other.e0123 * other.e0123 - 2.0 * self.e03 * other.s * other.e0123 - 2.0 * self.e03 * other.e01 * other.e23 - 2.0 * self.e03 * other.e02 * other.e31 + 2.0 * self.e03 * other.e12 * other.e03 - 2.0 * self.e31 * other.s * other.e23 + 2.0 * self.e31 * other.e01 * other.e0123 + 2.0 * self.e31 * other.e02 * other.e03 + 2.0 * self.e31 * other.e12 * other.e31 + 2.0 * self.e23 * other.s * other.e31 + 2.0 * self.e23 * other.e01 * other.e03 - 2.0 * self.e23 * other.e02 * other.e0123 + 2.0 * self.e23 * other.e12 * other.e23,
            e03: 2.0 * self.e01 * other.s * other.e31 - 2.0 * self.e01 * other.e01 * other.e03 + 2.0 * self.e01 * other.e02 * other.e0123 + 2.0 * self.e01 * other.e12 * other.e23 - 2.0 * self.e02 * other.s * other.e23 - 2.0 * self.e02 * other.e01 * other.e0123 - 2.0 * self.e02 * other.e02 * other.e03 + 2.0 * self.e02 * other.e12 * other.e31 - 2.0 * self.e12 * other.s * other.e0123 - 2.0 * self.e12 * other.e01 * other.e23 - 2.0 * self.e12 * other.e02 * other.e31 + 2.0 * self.e12 * other.e12 * other.e03 + self.e03 * other.s * other.s + self.e03 * other.e01 * other.e01 + self.e03 * other.e02 * other.e02 + self.e03 * other.e12 * other.e12 - self.e03 * other.e03 * other.e03 - self.e03 * other.e31 * other.e31 - self.e03 * other.e23 * other.e23 - self.e03 * other.e0123 * other.e0123 - 2.0 * self.e31 * other.s * other.e01 + 2.0 * self.e31 * other.e02 * other.e12 + 2.0 * self.e31 * other.e03 * other.e31 + 2.0 * self.e31 * other.e23 * other.e0123 + 2.0 * self.e23 * other.s * other.e02 + 2.0 * self.e23 * other.e01 * other.e12 + 2.0 * self.e23 * other.e03 * other.e23 - 2.0 * self.e23 * other.e31 * other.e0123,
            e31: -2.0 * self.e01 * other.s * other.e03 + 2.0 * self.e01 * other.e01 * other.e31 + 2.0 * self.e01 * other.e02 * other.e23 + 2.0 * self.e01 * other.e12 * other.e0123 - 2.0 * self.e02 * other.s * other.e0123 - 2.0 * self.e02 * other.e01 * other.e23 + 2.0 * self.e02 * other.e02 * other.e31 - 2.0 * self.e02 * other.e12 * other.e03 + 2.0 * self.e12 * other.s * other.e23 - 2.0 * self.e12 * other.e01 * other.e0123 + 2.0 * self.e12 * other.e02 * other.e03 + 2.0 * self.e12 * other.e12 * other.e31 + 2.0 * self.e03 * other.s * other.e01 + 2.0 * self.e03 * other.e02 * other.e12 + 2.0 * self.e03 * other.e03 * other.e31 - 2.0 * self.e03 * other.e23 * other.e0123 + self.e31 * other.s * other.s - self.e31 * other.e01 * other.e01 + self.e31 * other.e02 * other.e02 - self.e31 * other.e12 * other.e12 - self.e31 * other.e03 * other.e03 + self.e31 * other.e31 * other.e31 - self.e31 * other.e23 * other.e23 + self.e31 * other.e0123 * other.e0123 - 2.0 * self.e23 * other.s * other.e12 + 2.0 * self.e23 * other.e01 * other.e02 + 2.0 * self.e23 * other.e03 * other.e0123 + 2.0 * self.e23 * other.e31 * other.e23,
            e23: -2.0 * self.e01 * other.s * other.e0123 + 2.0 * self.e01 * other.e01 * other.e23 - 2.0 * self.e01 * other.e02 * other.e31 - 2.0 * self.e01 * other.e12 * other.e03 + 2.0 * self.e02 * other.s * other.e03 + 2.0 * self.e02 * other.e01 * other.e31 + 2.0 * self.e02 * other.e02 * other.e23 - 2.0 * self.e02 * other.e12 * other.e0123 - 2.0 * self.e12 * other.s * other.e31 + 2.0 * self.e12 * other.e01 * other.e03 + 2.0 * self.e12 * other.e02 * other.e0123 + 2.0 * self.e12 * other.e12 * other.e23 - 2.0 * self.e03 * other.s * other.e02 + 2.0 * self.e03 * other.e01 * other.e12 + 2.0 * self.e03 * other.e03 * other.e23 + 2.0 * self.e03 * other.e31 * other.e0123 + 2.0 * self.e31 * other.s * other.e12 + 2.0 * self.e31 * other.e01 * other.e02 - 2.0 * self.e31 * other.e03 * other.e0123 + 2.0 * self.e31 * other.e31 * other.e23 + self.e23 * other.s * other.s + self.e23 * other.e01 * other.e01 - self.e23 * other.e02 * other.e02 - self.e23 * other.e12 * other.e12 - self.e23 * other.e03 * other.e03 - self.e23 * other.e31 * other.e31 + self.e23 * other.e23 * other.e23 + self.e23 * other.e0123 * other.e0123,
            e0123: 0.0,
        }
    }
}

impl Transform<Multivector> for Bivector {
    type Output = Multivector;
    fn transform(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: 0.0,
            e1: 0.0,
            e01: self.e01 * other.s * other.s + self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1 - self.e01 * other.e01 * other.e01 + self.e01 * other.e2 * other.e2 + self.e01 * other.e02 * other.e02 - self.e01 * other.e12 * other.e12 - self.e01 * other.e021 * other.e021 + self.e01 * other.e3 * other.e3 + self.e01 * other.e03 * other.e03 - self.e01 * other.e31 * other.e31 - self.e01 * other.e013 * other.e013 + self.e01 * other.e23 * other.e23 + self.e01 * other.e032 * other.e032 - self.e01 * other.e123 * other.e123 - self.e01 * other.e0123 * other.e0123 + 2.0 * self.e02 * other.s * other.e12 - 2.0 * self.e02 * other.e0 * other.e021 - 2.0 * self.e02 * other.e1 * other.e2 - 2.0 * self.e02 * other.e01 * other.e02 + 2.0 * self.e02 * other.e3 * other.e123 + 2.0 * self.e02 * other.e03 * other.e0123 + 2.0 * self.e02 * other.e31 * other.e23 + 2.0 * self.e02 * other.e013 * other.e032 - 2.0 * self.e12 * other.s * other.e02 + 2.0 * self.e12 * other.e0 * other.e2 + 2.0 * self.e12 * other.e1 * other.e021 + 2.0 * self.e12 * other.e01 * other.e12 + 2.0 * self.e12 * other.e3 * other.e032 + 2.0 * self.e12 * other.e03 * other.e23 + 2.0 * self.e12 * other.e31 * other.e0123 + 2.0 * self.e12 * other.e013 * other.e123 - 2.0 * self.e03 * other.s * other.e31 + 2.0 * self.e03 * other.e0 * other.e013 - 2.0 * self.e03 * other.e1 * other.e3 - 2.0 * self.e03 * other.e01 * other.e03 - 2.0 * self.e03 * other.e2 * other.e123 - 2.0 * self.e03 * other.e02 * other.e0123 + 2.0 * self.e03 * other.e12 * other.e23 + 2.0 * self.e03 * other.e021 * other.e032 + 2.0 * self.e31 * other.s * other.e03 - 2.0 * self.e31 * other.e0 * other.e3 + 2.0 * self.e31 * other.e1 * other.e013 + 2.0 * self.e31 * other.e01 * other.e31 + 2.0 * self.e31 * other.e2 * other.e032 + 2.0 * self.e31 * other.e02 * other.e23 - 2.0 * self.e31 * other.e12 * other.e0123 - 2.0 * self.e31 * other.e021 * other.e123 - 2.0 * self.e23 * other.s * other.e0123 + 2.0 * self.e23 * other.e0 * other.e123 + 2.0 * self.e23 * other.e1 * other.e032 + 2.0 * self.e23 * other.e01 * other.e23 - 2.0 * self.e23 * other.e2 * other.e013 - 2.0 * self.e23 * other.e02 * other.e31 - 2.0 * self.e23 * other.e12 * other.e03 - 2.0 * self.e23 * other.e021 * other.e3,
            e2: 0.0,
            e02: -2.0 * self.e01 * other.s * other.e12 + 2.0 * self.e01 * other.e0 * other.e021 - 2.0 * self.e01 * other.e1 * other.e2 - 2.0 * self.e01 * other.e01 * other.e02 - 2.0 * self.e01 * other.e3 * other.e123 - 2.0 * self.e01 * other.e03 * other.e0123 + 2.0 * self.e01 * other.e31 * other.e23 + 2.0 * self.e01 * other.e013 * other.e032 + self.e02 * other.s * other.s + self.e02 * other.e0 * other.e0 + self.e02 * other.e1 * other.e1 + self.e02 * other.e01 * other.e01 - self.e02 * other.e2 * other.e2 - self.e02 * other.e02 * other.e02 - self.e02 * other.e12 * other.e12 - self.e02 * other.e021 * other.e021 + self.e02 * other.e3 * other.e3 + self.e02 * other.e03 * other.e03 + self.e02 * other.e31 * other.e31 + self.e02 * other.e013 * other.e013 - self.e02 * other.e23 * other.e23 - self.e02 * other.e032 * other.e032 - self.e02 * other.e123 * other.e123 - self.e02 * other.e0123 * other.e0123 + 2.0 * self.e12 * other.s * other.e01 - 2.0 * self.e12 * other.e0 * other.e1 + 2.0 * self.e12 * other.e2 * other.e021 + 2.0 * self.e12 * other.e02 * other.e12 + 2.0 * self.e12 * other.e3 * other.e013 + 2.0 * self.e12 * other.e03 * other.e31 - 2.0 * self.e12 * other.e23 * other.e0123 - 2.0 * self.e12 * other.e032 * other.e123 + 2.0 * self.e03 * other.s * other.e23 - 2.0 * self.e03 * other.e0 * other.e032 + 2.0 * self.e03 * other.e1 * other.e123 + 2.0 * self.e03 * other.e01 * other.e0123 - 2.0 * self.e03 * other.e2 * other.e3 - 2.0 * self.e03 * other.e02 * other.e03 + 2.0 * self.e03 * other.e12 * other.e31 + 2.0 * self.e03 * other.e021 * other.e013 - 2.0 * self.e31 * other.s * other.e0123 + 2.0 * self.e31 * other.e0 * other.e123 - 2.0 * self.e31 * other.e1 * other.e032 - 2.0 * self.e31 * other.e01 * other.e23 + 2.0 * self.e31 * other.e2 * other.e013 + 2.0 * self.e31 * other.e02 * other.e31 - 2.0 * self.e31 * other.e12 * other.e03 - 2.0 * self.e31 * other.e021 * other.e3 - 2.0 * self.e23 * other.s * other.e03 + 2.0 * self.e23 * other.e0 * other.e3 + 2.0 * self.e23 * other.e1 * other.e013 + 2.0 * self.e23 * other.e01 * other.e31 + 2.0 * self.e23 * other.e2 * other.e032 + 2.0 * self.e23 * other.e02 * other.e23 + 2.0 * self.e23 * other.e12 * other.e0123 + 2.0 * self.e23 * other.e021 * other.e123,
            e12: 2.0 * self.e01 * other.s * other.e02 + 2.0 * self.e01 * other.e0 * other.e2 - 2.0 * self.e01 * other.e1 * other.e021 + 2.0 * self.e01 * other.e01 * other.e12 - 2.0 * self.e01 * other.e3 * other.e032 + 2.0 * self.e01 * other.e03 * other.e23 - 2.0 * self.e01 * other.e31 * other.e0123 + 2.0 * self.e01 * other.e013 * other.e123 - 2.0 * self.e02 * other.s * other.e01 - 2.0 * self.e02 * other.e0 * other.e1 - 2.0 * self.e02 * other.e2 * other.e021 + 2.0 * self.e02 * other.e02 * other.e12 - 2.0 * self.e02 * other.e3 * other.e013 + 2.0 * self.e02 * other.e03 * other.e31 + 2.0 * self.e02 * other.e23 * other.e0123 - 2.0 * self.e02 * other.e032 * other.e123 + self.e12 * other.s * other.s + self.e12 * other.e0 * other.e0 - self.e12 * other.e1 * other.e1 - self.e12 * other.e01 * other.e01 - self.e12 * other.e2 * other.e2 - self.e12 * other.e02 * other.e02 + self.e12 * other.e12 * other.e12 + self.e12 * other.e021 * other.e021 + self.e12 * other.e3 * other.e3 + self.e12 * other.e03 * other.e03 - self.e12 * other.e31 * other.e31 - self.e12 * other.e013 * other.e013 - self.e12 * other.e23 * other.e23 - self.e12 * other.e032 * other.e032 + self.e12 * other.e123 * other.e123 + self.e12 * other.e0123 * other.e0123 - 2.0 * self.e03 * other.s * other.e0123 - 2.0 * self.e03 * other.e0 * other.e123 + 2.0 * self.e03 * other.e1 * other.e032 - 2.0 * self.e03 * other.e01 * other.e23 + 2.0 * self.e03 * other.e2 * other.e013 - 2.0 * self.e03 * other.e02 * other.e31 + 2.0 * self.e03 * other.e12 * other.e03 - 2.0 * self.e03 * other.e021 * other.e3 - 2.0 * self.e31 * other.s * other.e23 + 2.0 * self.e31 * other.e0 * other.e032 + 2.0 * self.e31 * other.e1 * other.e123 + 2.0 * self.e31 * other.e01 * other.e0123 + 2.0 * self.e31 * other.e2 * other.e3 + 2.0 * self.e31 * other.e02 * other.e03 + 2.0 * self.e31 * other.e12 * other.e31 + 2.0 * self.e31 * other.e021 * other.e013 + 2.0 * self.e23 * other.s * other.e31 - 2.0 * self.e23 * other.e0 * other.e013 + 2.0 * self.e23 * other.e1 * other.e3 + 2.0 * self.e23 * other.e01 * other.e03 - 2.0 * self.e23 * other.e2 * other.e123 - 2.0 * self.e23 * other.e02 * other.e0123 + 2.0 * self.e23 * other.e12 * other.e23 + 2.0 * self.e23 * other.e021 * other.e032,
            e021: -2.0 * self.e01 * other.s * other.e2 + 2.0 * self.e01 * other.e0 * other.e02 - 2.0 * self.e01 * other.e1 * other.e12 - 2.0 * self.e01 * other.e01 * other.e021 - 2.0 * self.e01 * other.e3 * other.e23 - 2.0 * self.e01 * other.e03 * other.e032 + 2.0 * self.e01 * other.e31 * other.e123 + 2.0 * self.e01 * other.e013 * other.e0123 + 2.0 * self.e02 * other.s * other.e1 - 2.0 * self.e02 * other.e0 * other.e01 - 2.0 * self.e02 * other.e2 * other.e12 - 2.0 * self.e02 * other.e02 * other.e021 - 2.0 * self.e02 * other.e3 * other.e31 - 2.0 * self.e02 * other.e03 * other.e013 - 2.0 * self.e02 * other.e23 * other.e123 - 2.0 * self.e02 * other.e032 * other.e0123 - 2.0 * self.e12 * other.s * other.e0 + 2.0 * self.e12 * other.e1 * other.e01 + 2.0 * self.e12 * other.e2 * other.e02 + 2.0 * self.e12 * other.e12 * other.e021 - 2.0 * self.e12 * other.e3 * other.e03 - 2.0 * self.e12 * other.e31 * other.e013 - 2.0 * self.e12 * other.e23 * other.e032 - 2.0 * self.e12 * other.e123 * other.e0123 + 2.0 * self.e03 * other.s * other.e123 - 2.0 * self.e03 * other.e0 * other.e0123 + 2.0 * self.e03 * other.e1 * other.e23 + 2.0 * self.e03 * other.e01 * other.e032 + 2.0 * self.e03 * other.e2 * other.e31 + 2.0 * self.e03 * other.e02 * other.e013 - 2.0 * self.e03 * other.e12 * other.e3 - 2.0 * self.e03 * other.e021 * other.e03 - 2.0 * self.e31 * other.s * other.e032 + 2.0 * self.e31 * other.e0 * other.e23 - 2.0 * self.e31 * other.e1 * other.e0123 - 2.0 * self.e31 * other.e01 * other.e123 - 2.0 * self.e31 * other.e2 * other.e03 - 2.0 * self.e31 * other.e02 * other.e3 + 2.0 * self.e31 * other.e12 * other.e013 + 2.0 * self.e31 * other.e021 * other.e31 + 2.0 * self.e23 * other.s * other.e013 - 2.0 * self.e23 * other.e0 * other.e31 - 2.0 * self.e23 * other.e1 * other.e03 - 2.0 * self.e23 * other.e01 * other.e3 + 2.0 * self.e23 * other.e2 * other.e0123 + 2.0 * self.e23 * other.e02 * other.e123 + 2.0 * self.e23 * other.e12 * other.e032 + 2.0 * self.e23 * other.e021 * other.e23,
            e3: 0.0,
            e03: 2.0 * self.e01 * other.s * other.e31 - 2.0 * self.e01 * other.e0 * other.e013 - 2.0 * self.e01 * other.e1 * other.e3 - 2.0 * self.e01 * other.e01 * other.e03 + 2.0 * self.e01 * other.e2 * other.e123 + 2.0 * self.e01 * other.e02 * other.e0123 + 2.0 * self.e01 * other.e12 * other.e23 + 2.0 * self.e01 * other.e021 * other.e032 - 2.0 * self.e02 * other.s * other.e23 + 2.0 * self.e02 * other.e0 * other.e032 - 2.0 * self.e02 * other.e1 * other.e123 - 2.0 * self.e02 * other.e01 * other.e0123 - 2.0 * self.e02 * other.e2 * other.e3 - 2.0 * self.e02 * other.e02 * other.e03 + 2.0 * self.e02 * other.e12 * other.e31 + 2.0 * self.e02 * other.e021 * other.e013 - 2.0 * self.e12 * other.s * other.e0123 + 2.0 * self.e12 * other.e0 * other.e123 - 2.0 * self.e12 * other.e1 * other.e032 - 2.0 * self.e12 * other.e01 * other.e23 - 2.0 * self.e12 * other.e2 * other.e013 - 2.0 * self.e12 * other.e02 * other.e31 + 2.0 * self.e12 * other.e12 * other.e03 + 2.0 * self.e12 * other.e021 * other.e3 + self.e03 * other.s * other.s + self.e03 * other.e0 * other.e0 + self.e03 * other.e1 * other.e1 + self.e03 * other.e01 * other.e01 + self.e03 * other.e2 * other.e2 + self.e03 * other.e02 * other.e02 + self.e03 * other.e12 * other.e12 + self.e03 * other.e021 * other.e021 - self.e03 * other.e3 * other.e3 - self.e03 * other.e03 * other.e03 - self.e03 * other.e31 * other.e31 - self.e03 * other.e013 * other.e013 - self.e03 * other.e23 * other.e23 - self.e03 * other.e032 * other.e032 - self.e03 * other.e123 * other.e123 - self.e03 * other.e0123 * other.e0123 - 2.0 * self.e31 * other.s * other.e01 + 2.0 * self.e31 * other.e0 * other.e1 + 2.0 * self.e31 * other.e2 * other.e021 + 2.0 * self.e31 * other.e02 * other.e12 + 2.0 * self.e31 * other.e3 * other.e013 + 2.0 * self.e31 * other.e03 * other.e31 + 2.0 * self.e31 * other.e23 * other.e0123 + 2.0 * self.e31 * other.e032 * other.e123 + 2.0 * self.e23 * other.s * other.e02 - 2.0 * self.e23 * other.e0 * other.e2 + 2.0 * self.e23 * other.e1 * other.e021 + 2.0 * self.e23 * other.e01 * other.e12 + 2.0 * self.e23 * other.e3 * other.e032 + 2.0 * self.e23 * other.e03 * other.e23 - 2.0 * self.e23 * other.e31 * other.e0123 - 2.0 * self.e23 * other.e013 * other.e123,
            e31: -2.0 * self.e01 * other.s * other.e03 - 2.0 * self.e01 * other.e0 * other.e3 - 2.0 * self.e01 * other.e1 * other.e013 + 2.0 * self.e01 * other.e01 * other.e31 - 2.0 * self.e01 * other.e2 * other.e032 + 2.0 * self.e01 * other.e02 * other.e23 + 2.0 * self.e01 * other.e12 * other.e0123 - 2.0 * self.e01 * other.e021 * other.e123 - 2.0 * self.e02 * other.s * other.e0123 - 2.0 * self.e02 * other.e0 * other.e123 + 2.0 * self.e02 * other.e1 * other.e032 - 2.0 * self.e02 * other.e01 * other.e23 - 2.0 * self.e02 * other.e2 * other.e013 + 2.0 * self.e02 * other.e02 * other.e31 - 2.0 * self.e02 * other.e12 * other.e03 + 2.0 * self.e02 * other.e021 * other.e3 + 2.0 * self.e12 * other.s * other.e23 - 2.0 * self.e12 * other.e0 * other.e032 - 2.0 * self.e12 * other.e1 * other.e123 - 2.0 * self.e12 * other.e01 * other.e0123 + 2.0 * self.e12 * other.e2 * other.e3 + 2.0 * self.e12 * other.e02 * other.e03 + 2.0 * self.e12 * other.e12 * other.e31 + 2.0 * self.e12 * other.e021 * other.e013 + 2.0 * self.e03 * other.s * other.e01 + 2.0 * self.e03 * other.e0 * other.e1 - 2.0 * self.e03 * other.e2 * other.e021 + 2.0 * self.e03 * other.e02 * other.e12 - 2.0 * self.e03 * other.e3 * other.e013 + 2.0 * self.e03 * other.e03 * other.e31 - 2.0 * self.e03 * other.e23 * other.e0123 + 2.0 * self.e03 * other.e032 * other.e123 + self.e31 * other.s * other.s + self.e31 * other.e0 * other.e0 - self.e31 * other.e1 * other.e1 - self.e31 * other.e01 * other.e01 + self.e31 * other.e2 * other.e2 + self.e31 * other.e02 * other.e02 - self.e31 * other.e12 * other.e12 - self.e31 * other.e021 * other.e021 - self.e31 * other.e3 * other.e3 - self.e31 * other.e03 * other.e03 + self.e31 * other.e31 * other.e31 + self.e31 * other.e013 * other.e013 - self.e31 * other.e23 * other.e23 - self.e31 * other.e032 * other.e032 + self.e31 * other.e123 * other.e123 + self.e31 * other.e0123 * other.e0123 - 2.0 * self.e23 * other.s * other.e12 + 2.0 * self.e23 * other.e0 * other.e021 + 2.0 * self.e23 * other.e1 * other.e2 + 2.0 * self.e23 * other.e01 * other.e02 + 2.0 * self.e23 * other.e3 * other.e123 + 2.0 * self.e23 * other.e03 * other.e0123 + 2.0 * self.e23 * other.e31 * other.e23 + 2.0 * self.e23 * other.e013 * other.e032,
            e013: 2.0 * self.e01 * other.s * other.e3 - 2.0 * self.e01 * other.e0 * other.e03 - 2.0 * self.e01 * other.e1 * other.e31 - 2.0 * self.e01 * other.e01 * other.e013 - 2.0 * self.e01 * other.e2 * other.e23 - 2.0 * self.e01 * other.e02 * other.e032 - 2.0 * self.e01 * other.e12 * other.e123 - 2.0 * self.e01 * other.e021 * other.e0123 + 2.0 * self.e02 * other.s * other.e123 - 2.0 * self.e02 * other.e0 * other.e0123 + 2.0 * self.e02 * other.e1 * other.e23 + 2.0 * self.e02 * other.e01 * other.e032 - 2.0 * self.e02 * other.e2 * other.e31 - 2.0 * self.e02 * other.e02 * other.e013 + 2.0 * self.e02 * other.e12 * other.e3 + 2.0 * self.e02 * other.e021 * other.e03 + 2.0 * self.e12 * other.s * other.e032 - 2.0 * self.e12 * other.e0 * other.e23 + 2.0 * self.e12 * other.e1 * other.e0123 + 2.0 * self.e12 * other.e01 * other.e123 - 2.0 * self.e12 * other.e2 * other.e03 - 2.0 * self.e12 * other.e02 * other.e3 + 2.0 * self.e12 * other.e12 * other.e013 + 2.0 * self.e12 * other.e021 * other.e31 - 2.0 * self.e03 * other.s * other.e1 + 2.0 * self.e03 * other.e0 * other.e01 - 2.0 * self.e03 * other.e2 * other.e12 - 2.0 * self.e03 * other.e02 * other.e021 - 2.0 * self.e03 * other.e3 * other.e31 - 2.0 * self.e03 * other.e03 * other.e013 + 2.0 * self.e03 * other.e23 * other.e123 + 2.0 * self.e03 * other.e032 * other.e0123 - 2.0 * self.e31 * other.s * other.e0 + 2.0 * self.e31 * other.e1 * other.e01 - 2.0 * self.e31 * other.e2 * other.e02 - 2.0 * self.e31 * other.e12 * other.e021 + 2.0 * self.e31 * other.e3 * other.e03 + 2.0 * self.e31 * other.e31 * other.e013 - 2.0 * self.e31 * other.e23 * other.e032 - 2.0 * self.e31 * other.e123 * other.e0123 - 2.0 * self.e23 * other.s * other.e021 + 2.0 * self.e23 * other.e0 * other.e12 - 2.0 * self.e23 * other.e1 * other.e02 - 2.0 * self.e23 * other.e01 * other.e2 - 2.0 * self.e23 * other.e3 * other.e0123 - 2.0 * self.e23 * other.e03 * other.e123 + 2.0 * self.e23 * other.e31 * other.e032 + 2.0 * self.e23 * other.e013 * other.e23,
            e23: -2.0 * self.e01 * other.s * other.e0123 - 2.0 * self.e01 * other.e0 * other.e123 - 2.0 * self.e01 * other.e1 * other.e032 + 2.0 * self.e01 * other.e01 * other.e23 + 2.0 * self.e01 * other.e2 * other.e013 - 2.0 * self.e01 * other.e02 * other.e31 - 2.0 * self.e01 * other.e12 * other.e03 + 2.0 * self.e01 * other.e021 * other.e3 + 2.0 * self.e02 * other.s * other.e03 + 2.0 * self.e02 * other.e0 * other.e3 - 2.0 * self.e02 * other.e1 * other.e013 + 2.0 * self.e02 * other.e01 * other.e31 - 2.0 * self.e02 * other.e2 * other.e032 + 2.0 * self.e02 * other.e02 * other.e23 - 2.0 * self.e02 * other.e12 * other.e0123 + 2.0 * self.e02 * other.e021 * other.e123 - 2.0 * self.e12 * other.s * other.e31 + 2.0 * self.e12 * other.e0 * other.e013 + 2.0 * self.e12 * other.e1 * other.e3 + 2.0 * self.e12 * other.e01 * other.e03 + 2.0 * self.e12 * other.e2 * other.e123 + 2.0 * self.e12 * other.e02 * other.e0123 + 2.0 * self.e12 * other.e12 * other.e23 + 2.0 * self.e12 * other.e021 * other.e032 - 2.0 * self.e03 * other.s * other.e02 - 2.0 * self.e03 * other.e0 * other.e2 - 2.0 * self.e03 * other.e1 * other.e021 + 2.0 * self.e03 * other.e01 * other.e12 - 2.0 * self.e03 * other.e3 * other.e032 + 2.0 * self.e03 * other.e03 * other.e23 + 2.0 * self.e03 * other.e31 * other.e0123 - 2.0 * self.e03 * other.e013 * other.e123 + 2.0 * self.e31 * other.s * other.e12 - 2.0 * self.e31 * other.e0 * other.e021 + 2.0 * self.e31 * other.e1 * other.e2 + 2.0 * self.e31 * other.e01 * other.e02 - 2.0 * self.e31 * other.e3 * other.e123 - 2.0 * self.e31 * other.e03 * other.e0123 + 2.0 * self.e31 * other.e31 * other.e23 + 2.0 * self.e31 * other.e013 * other.e032 + self.e23 * other.s * other.s + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1 + self.e23 * other.e01 * other.e01 - self.e23 * other.e2 * other.e2 - self.e23 * other.e02 * other.e02 - self.e23 * other.e12 * other.e12 - self.e23 * other.e021 * other.e021 - self.e23 * other.e3 * other.e3 - self.e23 * other.e03 * other.e03 - self.e23 * other.e31 * other.e31 - self.e23 * other.e013 * other.e013 + self.e23 * other.e23 * other.e23 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123 + self.e23 * other.e0123 * other.e0123,
            e032: 2.0 * self.e01 * other.s * other.e123 - 2.0 * self.e01 * other.e0 * other.e0123 - 2.0 * self.e01 * other.e1 * other.e23 - 2.0 * self.e01 * other.e01 * other.e032 + 2.0 * self.e01 * other.e2 * other.e31 + 2.0 * self.e01 * other.e02 * other.e013 + 2.0 * self.e01 * other.e12 * other.e3 + 2.0 * self.e01 * other.e021 * other.e03 - 2.0 * self.e02 * other.s * other.e3 + 2.0 * self.e02 * other.e0 * other.e03 - 2.0 * self.e02 * other.e1 * other.e31 - 2.0 * self.e02 * other.e01 * other.e013 - 2.0 * self.e02 * other.e2 * other.e23 - 2.0 * self.e02 * other.e02 * other.e032 + 2.0 * self.e02 * other.e12 * other.e123 + 2.0 * self.e02 * other.e021 * other.e0123 - 2.0 * self.e12 * other.s * other.e013 + 2.0 * self.e12 * other.e0 * other.e31 - 2.0 * self.e12 * other.e1 * other.e03 - 2.0 * self.e12 * other.e01 * other.e3 - 2.0 * self.e12 * other.e2 * other.e0123 - 2.0 * self.e12 * other.e02 * other.e123 + 2.0 * self.e12 * other.e12 * other.e032 + 2.0 * self.e12 * other.e021 * other.e23 + 2.0 * self.e03 * other.s * other.e2 - 2.0 * self.e03 * other.e0 * other.e02 - 2.0 * self.e03 * other.e1 * other.e12 - 2.0 * self.e03 * other.e01 * other.e021 - 2.0 * self.e03 * other.e3 * other.e23 - 2.0 * self.e03 * other.e03 * other.e032 - 2.0 * self.e03 * other.e31 * other.e123 - 2.0 * self.e03 * other.e013 * other.e0123 + 2.0 * self.e31 * other.s * other.e021 - 2.0 * self.e31 * other.e0 * other.e12 - 2.0 * self.e31 * other.e1 * other.e02 - 2.0 * self.e31 * other.e01 * other.e2 + 2.0 * self.e31 * other.e3 * other.e0123 + 2.0 * self.e31 * other.e03 * other.e123 + 2.0 * self.e31 * other.e31 * other.e032 + 2.0 * self.e31 * other.e013 * other.e23 - 2.0 * self.e23 * other.s * other.e0 - 2.0 * self.e23 * other.e1 * other.e01 + 2.0 * self.e23 * other.e2 * other.e02 - 2.0 * self.e23 * other.e12 * other.e021 + 2.0 * self.e23 * other.e3 * other.e03 - 2.0 * self.e23 * other.e31 * other.e013 + 2.0 * self.e23 * other.e23 * other.e032 - 2.0 * self.e23 * other.e123 * other.e0123,
            e123: -2.0 * self.e01 * other.s * other.e032 - 2.0 * self.e01 * other.e0 * other.e23 - 2.0 * self.e01 * other.e1 * other.e0123 + 2.0 * self.e01 * other.e01 * other.e123 - 2.0 * self.e01 * other.e2 * other.e03 + 2.0 * self.e01 * other.e02 * other.e3 + 2.0 * self.e01 * other.e12 * other.e013 - 2.0 * self.e01 * other.e021 * other.e31 - 2.0 * self.e02 * other.s * other.e013 - 2.0 * self.e02 * other.e0 * other.e31 + 2.0 * self.e02 * other.e1 * other.e03 - 2.0 * self.e02 * other.e01 * other.e3 - 2.0 * self.e02 * other.e2 * other.e0123 + 2.0 * self.e02 * other.e02 * other.e123 - 2.0 * self.e02 * other.e12 * other.e032 + 2.0 * self.e02 * other.e021 * other.e23 + 2.0 * self.e12 * other.s * other.e3 - 2.0 * self.e12 * other.e0 * other.e03 - 2.0 * self.e12 * other.e1 * other.e31 - 2.0 * self.e12 * other.e01 * other.e013 + 2.0 * self.e12 * other.e2 * other.e23 + 2.0 * self.e12 * other.e02 * other.e032 + 2.0 * self.e12 * other.e12 * other.e123 + 2.0 * self.e12 * other.e021 * other.e0123 - 2.0 * self.e03 * other.s * other.e021 - 2.0 * self.e03 * other.e0 * other.e12 - 2.0 * self.e03 * other.e1 * other.e02 + 2.0 * self.e03 * other.e01 * other.e2 - 2.0 * self.e03 * other.e3 * other.e0123 + 2.0 * self.e03 * other.e03 * other.e123 + 2.0 * self.e03 * other.e31 * other.e032 - 2.0 * self.e03 * other.e013 * other.e23 + 2.0 * self.e31 * other.s * other.e2 - 2.0 * self.e31 * other.e0 * other.e02 + 2.0 * self.e31 * other.e1 * other.e12 + 2.0 * self.e31 * other.e01 * other.e021 - 2.0 * self.e31 * other.e3 * other.e23 - 2.0 * self.e31 * other.e03 * other.e032 + 2.0 * self.e31 * other.e31 * other.e123 + 2.0 * self.e31 * other.e013 * other.e0123 + 2.0 * self.e23 * other.s * other.e1 - 2.0 * self.e23 * other.e0 * other.e01 - 2.0 * self.e23 * other.e2 * other.e12 - 2.0 * self.e23 * other.e02 * other.e021 + 2.0 * self.e23 * other.e3 * other.e31 + 2.0 * self.e23 * other.e03 * other.e013 + 2.0 * self.e23 * other.e23 * other.e123 + 2.0 * self.e23 * other.e032 * other.e0123,
            e0123: 0.0,
        }
    }
}

impl Transform<Scalar> for Trivector {
    type Output = OddMultivector;
    fn transform(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: self.e021 * other.s * other.s,
            e3: 0.0,
            e013: self.e013 * other.s * other.s,
            e032: self.e032 * other.s * other.s,
            e123: self.e123 * other.s * other.s,
        }
    }
}

impl Transform<Vector> for Trivector {
    type Output = OddMultivector;
    fn transform(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: -self.e021 * other.e0 * other.e0 + self.e021 * other.e1 * other.e1 + self.e021 * other.e2 * other.e2 - self.e021 * other.e3 * other.e3 - 2.0 * self.e013 * other.e2 * other.e3 - 2.0 * self.e032 * other.e1 * other.e3 - 2.0 * self.e123 * other.e0 * other.e3,
            e3: 0.0,
            e013: -2.0 * self.e021 * other.e2 * other.e3 - self.e013 * other.e0 * other.e0 + self.e013 * other.e1 * other.e1 - self.e013 * other.e2 * other.e2 + self.e013 * other.e3 * other.e3 - 2.0 * self.e032 * other.e1 * other.e2 - 2.0 * self.e123 * other.e0 * other.e2,
            e032: -2.0 * self.e021 * other.e1 * other.e3 - 2.0 * self.e013 * other.e1 * other.e2 - self.e032 * other.e0 * other.e0 - self.e032 * other.e1 * other.e1 + self.e032 * other.e2 * other.e2 + self.e032 * other.e3 * other.e3 - 2.0 * self.e123 * other.e0 * other.e1,
            e123: -2.0 * self.e021 * other.e0 * other.e3 - 2.0 * self.e013 * other.e0 * other.e2 - 2.0 * self.e032 * other.e0 * other.e1 - self.e123 * other.e0 * other.e0 + self.e123 * other.e1 * other.e1 + self.e123 * other.e2 * other.e2 + self.e123 * other.e3 * other.e3,
        }
    }
}

impl Transform<Bivector> for Trivector {
    type Output = OddMultivector;
    fn transform(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: -self.e021 * other.e01 * other.e01 - self.e021 * other.e02 * other.e02 + self.e021 * other.e12 * other.e12 + self.e021 * other.e03 * other.e03 - self.e021 * other.e31 * other.e31 - self.e021 * other.e23 * other.e23 + 2.0 * self.e013 * other.e02 * other.e03 + 2.0 * self.e013 * other.e12 * other.e31 + 2.0 * self.e032 * other.e01 * other.e03 + 2.0 * self.e032 * other.e12 * other.e23 - 2.0 * self.e123 * other.e01 * other.e31 + 2.0 * self.e123 * other.e02 * other.e23,
            e3: 0.0,
            e013: 2.0 * self.e021 * other.e02 * other.e03 + 2.0 * self.e021 * other.e12 * other.e31 - self.e013 * other.e01 * other.e01 + self.e013 * other.e02 * other.e02 - self.e013 * other.e12 * other.e12 - self.e013 * other.e03 * other.e03 + self.e013 * other.e31 * other.e31 - self.e013 * other.e23 * other.e23 + 2.0 * self.e032 * other.e01 * other.e02 + 2.0 * self.e032 * other.e31 * other.e23 + 2.0 * self.e123 * other.e01 * other.e12 - 2.0 * self.e123 * other.e03 * other.e23,
            e032: 2.0 * self.e021 * other.e01 * other.e03 + 2.0 * self.e021 * other.e12 * other.e23 + 2.0 * self.e013 * other.e01 * other.e02 + 2.0 * self.e013 * other.e31 * other.e23 + self.e032 * other.e01 * other.e01 - self.e032 * other.e02 * other.e02 - self.e032 * other.e12 * other.e12 - self.e032 * other.e03 * other.e03 - self.e032 * other.e31 * other.e31 + self.e032 * other.e23 * other.e23 - 2.0 * self.e123 * other.e02 * other.e12 + 2.0 * self.e123 * other.e03 * other.e31,
            e123: -2.0 * self.e021 * other.e01 * other.e31 + 2.0 * self.e021 * other.e02 * other.e23 + 2.0 * self.e013 * other.e01 * other.e12 - 2.0 * self.e013 * other.e03 * other.e23 - 2.0 * self.e032 * other.e02 * other.e12 + 2.0 * self.e032 * other.e03 * other.e31 - self.e123 * other.e01 * other.e01 - self.e123 * other.e02 * other.e02 + self.e123 * other.e12 * other.e12 - self.e123 * other.e03 * other.e03 + self.e123 * other.e31 * other.e31 + self.e123 * other.e23 * other.e23,
        }
    }
}

impl Transform<Trivector> for Trivector {
    type Output = OddMultivector;
    fn transform(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: -self.e021 * other.e021 * other.e021 + self.e021 * other.e013 * other.e013 + self.e021 * other.e032 * other.e032 - self.e021 * other.e123 * other.e123 - 2.0 * self.e013 * other.e021 * other.e013 - 2.0 * self.e032 * other.e021 * other.e032 + 2.0 * self.e123 * other.e021 * other.e123,
            e3: 0.0,
            e013: -2.0 * self.e021 * other.e021 * other.e013 + self.e013 * other.e021 * other.e021 - self.e013 * other.e013 * other.e013 + self.e013 * other.e032 * other.e032 - self.e013 * other.e123 * other.e123 - 2.0 * self.e032 * other.e013 * other.e032 + 2.0 * self.e123 * other.e013 * other.e123,
            e032: -2.0 * self.e021 * other.e021 * other.e032 - 2.0 * self.e013 * other.e013 * other.e032 + self.e032 * other.e021 * other.e021 + self.e032 * other.e013 * other.e013 - self.e032 * other.e032 * other.e032 - self.e032 * other.e123 * other.e123 + 2.0 * self.e123 * other.e032 * other.e123,
            e123: 2.0 * self.e021 * other.e021 * other.e123 + 2.0 * self.e013 * other.e013 * other.e123 + 2.0 * self.e032 * other.e032 * other.e123 - self.e123 * other.e021 * other.e021 - self.e123 * other.e013 * other.e013 - self.e123 * other.e032 * other.e032 + self.e123 * other.e123 * other.e123,
        }
    }
}

impl Transform<FourVector> for Trivector {
    type Output = OddMultivector;
    fn transform(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: self.e021 * other.e0123 * other.e0123,
            e3: 0.0,
            e013: self.e013 * other.e0123 * other.e0123,
            e032: self.e032 * other.e0123 * other.e0123,
            e123: -self.e123 * other.e0123 * other.e0123,
        }
    }
}

impl Transform<Null> for Trivector {
    type Output = Null;
    fn transform(self, other: Null) -> Null {
        Null
    }
}

impl Transform<OddMultivector> for Trivector {
    type Output = OddMultivector;
    fn transform(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: -self.e021 * other.e0 * other.e0 + self.e021 * other.e1 * other.e1 + self.e021 * other.e2 * other.e2 - self.e021 * other.e021 * other.e021 - self.e021 * other.e3 * other.e3 + self.e021 * other.e013 * other.e013 + self.e021 * other.e032 * other.e032 - self.e021 * other.e123 * other.e123 - 2.0 * self.e013 * other.e0 * other.e032 - 2.0 * self.e013 * other.e1 * other.e123 - 2.0 * self.e013 * other.e2 * other.e3 - 2.0 * self.e013 * other.e021 * other.e013 + 2.0 * self.e032 * other.e0 * other.e013 - 2.0 * self.e032 * other.e1 * other.e3 + 2.0 * self.e032 * other.e2 * other.e123 - 2.0 * self.e032 * other.e021 * other.e032 - 2.0 * self.e123 * other.e0 * other.e3 + 2.0 * self.e123 * other.e1 * other.e013 - 2.0 * self.e123 * other.e2 * other.e032 + 2.0 * self.e123 * other.e021 * other.e123,
            e3: 0.0,
            e013: 2.0 * self.e021 * other.e0 * other.e032 + 2.0 * self.e021 * other.e1 * other.e123 - 2.0 * self.e021 * other.e2 * other.e3 - 2.0 * self.e021 * other.e021 * other.e013 - self.e013 * other.e0 * other.e0 + self.e013 * other.e1 * other.e1 - self.e013 * other.e2 * other.e2 + self.e013 * other.e021 * other.e021 + self.e013 * other.e3 * other.e3 - self.e013 * other.e013 * other.e013 + self.e013 * other.e032 * other.e032 - self.e013 * other.e123 * other.e123 - 2.0 * self.e032 * other.e0 * other.e021 - 2.0 * self.e032 * other.e1 * other.e2 - 2.0 * self.e032 * other.e3 * other.e123 - 2.0 * self.e032 * other.e013 * other.e032 - 2.0 * self.e123 * other.e0 * other.e2 - 2.0 * self.e123 * other.e1 * other.e021 + 2.0 * self.e123 * other.e3 * other.e032 + 2.0 * self.e123 * other.e013 * other.e123,
            e032: -2.0 * self.e021 * other.e0 * other.e013 - 2.0 * self.e021 * other.e1 * other.e3 - 2.0 * self.e021 * other.e2 * other.e123 - 2.0 * self.e021 * other.e021 * other.e032 + 2.0 * self.e013 * other.e0 * other.e021 - 2.0 * self.e013 * other.e1 * other.e2 + 2.0 * self.e013 * other.e3 * other.e123 - 2.0 * self.e013 * other.e013 * other.e032 - self.e032 * other.e0 * other.e0 - self.e032 * other.e1 * other.e1 + self.e032 * other.e2 * other.e2 + self.e032 * other.e021 * other.e021 + self.e032 * other.e3 * other.e3 + self.e032 * other.e013 * other.e013 - self.e032 * other.e032 * other.e032 - self.e032 * other.e123 * other.e123 - 2.0 * self.e123 * other.e0 * other.e1 + 2.0 * self.e123 * other.e2 * other.e021 - 2.0 * self.e123 * other.e3 * other.e013 + 2.0 * self.e123 * other.e032 * other.e123,
            e123: -2.0 * self.e021 * other.e0 * other.e3 - 2.0 * self.e021 * other.e1 * other.e013 + 2.0 * self.e021 * other.e2 * other.e032 + 2.0 * self.e021 * other.e021 * other.e123 - 2.0 * self.e013 * other.e0 * other.e2 + 2.0 * self.e013 * other.e1 * other.e021 - 2.0 * self.e013 * other.e3 * other.e032 + 2.0 * self.e013 * other.e013 * other.e123 - 2.0 * self.e032 * other.e0 * other.e1 - 2.0 * self.e032 * other.e2 * other.e021 + 2.0 * self.e032 * other.e3 * other.e013 + 2.0 * self.e032 * other.e032 * other.e123 - self.e123 * other.e0 * other.e0 + self.e123 * other.e1 * other.e1 + self.e123 * other.e2 * other.e2 - self.e123 * other.e021 * other.e021 + self.e123 * other.e3 * other.e3 - self.e123 * other.e013 * other.e013 - self.e123 * other.e032 * other.e032 + self.e123 * other.e123 * other.e123,
        }
    }
}

impl Transform<EvenMultivector> for Trivector {
    type Output = OddMultivector;
    fn transform(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: self.e021 * other.s * other.s - self.e021 * other.e01 * other.e01 - self.e021 * other.e02 * other.e02 + self.e021 * other.e12 * other.e12 + self.e021 * other.e03 * other.e03 - self.e021 * other.e31 * other.e31 - self.e021 * other.e23 * other.e23 + self.e021 * other.e0123 * other.e0123 - 2.0 * self.e013 * other.s * other.e23 + 2.0 * self.e013 * other.e01 * other.e0123 + 2.0 * self.e013 * other.e02 * other.e03 + 2.0 * self.e013 * other.e12 * other.e31 + 2.0 * self.e032 * other.s * other.e31 + 2.0 * self.e032 * other.e01 * other.e03 - 2.0 * self.e032 * other.e02 * other.e0123 + 2.0 * self.e032 * other.e12 * other.e23 - 2.0 * self.e123 * other.s * other.e03 - 2.0 * self.e123 * other.e01 * other.e31 + 2.0 * self.e123 * other.e02 * other.e23 - 2.0 * self.e123 * other.e12 * other.e0123,
            e3: 0.0,
            e013: 2.0 * self.e021 * other.s * other.e23 - 2.0 * self.e021 * other.e01 * other.e0123 + 2.0 * self.e021 * other.e02 * other.e03 + 2.0 * self.e021 * other.e12 * other.e31 + self.e013 * other.s * other.s - self.e013 * other.e01 * other.e01 + self.e013 * other.e02 * other.e02 - self.e013 * other.e12 * other.e12 - self.e013 * other.e03 * other.e03 + self.e013 * other.e31 * other.e31 - self.e013 * other.e23 * other.e23 + self.e013 * other.e0123 * other.e0123 - 2.0 * self.e032 * other.s * other.e12 + 2.0 * self.e032 * other.e01 * other.e02 + 2.0 * self.e032 * other.e03 * other.e0123 + 2.0 * self.e032 * other.e31 * other.e23 - 2.0 * self.e123 * other.s * other.e02 + 2.0 * self.e123 * other.e01 * other.e12 - 2.0 * self.e123 * other.e03 * other.e23 - 2.0 * self.e123 * other.e31 * other.e0123,
            e032: -2.0 * self.e021 * other.s * other.e31 + 2.0 * self.e021 * other.e01 * other.e03 + 2.0 * self.e021 * other.e02 * other.e0123 + 2.0 * self.e021 * other.e12 * other.e23 + 2.0 * self.e013 * other.s * other.e12 + 2.0 * self.e013 * other.e01 * other.e02 - 2.0 * self.e013 * other.e03 * other.e0123 + 2.0 * self.e013 * other.e31 * other.e23 + self.e032 * other.s * other.s + self.e032 * other.e01 * other.e01 - self.e032 * other.e02 * other.e02 - self.e032 * other.e12 * other.e12 - self.e032 * other.e03 * other.e03 - self.e032 * other.e31 * other.e31 + self.e032 * other.e23 * other.e23 + self.e032 * other.e0123 * other.e0123 - 2.0 * self.e123 * other.s * other.e01 - 2.0 * self.e123 * other.e02 * other.e12 + 2.0 * self.e123 * other.e03 * other.e31 - 2.0 * self.e123 * other.e23 * other.e0123,
            e123: 2.0 * self.e021 * other.s * other.e03 - 2.0 * self.e021 * other.e01 * other.e31 + 2.0 * self.e021 * other.e02 * other.e23 + 2.0 * self.e021 * other.e12 * other.e0123 + 2.0 * self.e013 * other.s * other.e02 + 2.0 * self.e013 * other.e01 * other.e12 - 2.0 * self.e013 * other.e03 * other.e23 + 2.0 * self.e013 * other.e31 * other.e0123 + 2.0 * self.e032 * other.s * other.e01 - 2.0 * self.e032 * other.e02 * other.e12 + 2.0 * self.e032 * other.e03 * other.e31 + 2.0 * self.e032 * other.e23 * other.e0123 + self.e123 * other.s * other.s - self.e123 * other.e01 * other.e01 - self.e123 * other.e02 * other.e02 + self.e123 * other.e12 * other.e12 - self.e123 * other.e03 * other.e03 + self.e123 * other.e31 * other.e31 + self.e123 * other.e23 * other.e23 - self.e123 * other.e0123 * other.e0123,
        }
    }
}

impl Transform<Multivector> for Trivector {
    type Output = Multivector;
    fn transform(self, other: Multivector) -> Multivector {
        Multivector {
            s: 0.0,
            e0: 0.0,
            e1: 0.0,
            e01: -2.0 * self.e021 * other.s * other.e2 - 2.0 * self.e021 * other.e0 * other.e02 + 2.0 * self.e021 * other.e1 * other.e12 - 2.0 * self.e021 * other.e01 * other.e021 + 2.0 * self.e021 * other.e3 * other.e23 - 2.0 * self.e021 * other.e03 * other.e032 + 2.0 * self.e021 * other.e31 * other.e123 - 2.0 * self.e021 * other.e013 * other.e0123 + 2.0 * self.e013 * other.s * other.e3 + 2.0 * self.e013 * other.e0 * other.e03 + 2.0 * self.e013 * other.e1 * other.e31 - 2.0 * self.e013 * other.e01 * other.e013 + 2.0 * self.e013 * other.e2 * other.e23 - 2.0 * self.e013 * other.e02 * other.e032 - 2.0 * self.e013 * other.e12 * other.e123 + 2.0 * self.e013 * other.e021 * other.e0123 - 2.0 * self.e032 * other.s * other.e123 - 2.0 * self.e032 * other.e0 * other.e0123 + 2.0 * self.e032 * other.e1 * other.e23 - 2.0 * self.e032 * other.e01 * other.e032 - 2.0 * self.e032 * other.e2 * other.e31 + 2.0 * self.e032 * other.e02 * other.e013 - 2.0 * self.e032 * other.e12 * other.e3 + 2.0 * self.e032 * other.e021 * other.e03 + 2.0 * self.e123 * other.s * other.e032 + 2.0 * self.e123 * other.e0 * other.e23 - 2.0 * self.e123 * other.e1 * other.e0123 + 2.0 * self.e123 * other.e01 * other.e123 + 2.0 * self.e123 * other.e2 * other.e03 - 2.0 * self.e123 * other.e02 * other.e3 + 2.0 * self.e123 * other.e12 * other.e013 - 2.0 * self.e123 * other.e021 * other.e31,
            e2: 0.0,
            e02: 2.0 * self.e021 * other.s * other.e1 + 2.0 * self.e021 * other.e0 * other.e01 + 2.0 * self.e021 * other.e2 * other.e12 - 2.0 * self.e021 * other.e02 * other.e021 + 2.0 * self.e021 * other.e3 * other.e31 - 2.0 * self.e021 * other.e03 * other.e013 - 2.0 * self.e021 * other.e23 * other.e123 + 2.0 * self.e021 * other.e032 * other.e0123 - 2.0 * self.e013 * other.s * other.e123 - 2.0 * self.e013 * other.e0 * other.e0123 - 2.0 * self.e013 * other.e1 * other.e23 + 2.0 * self.e013 * other.e01 * other.e032 + 2.0 * self.e013 * other.e2 * other.e31 - 2.0 * self.e013 * other.e02 * other.e013 - 2.0 * self.e013 * other.e12 * other.e3 + 2.0 * self.e013 * other.e021 * other.e03 - 2.0 * self.e032 * other.s * other.e3 - 2.0 * self.e032 * other.e0 * other.e03 + 2.0 * self.e032 * other.e1 * other.e31 - 2.0 * self.e032 * other.e01 * other.e013 + 2.0 * self.e032 * other.e2 * other.e23 - 2.0 * self.e032 * other.e02 * other.e032 + 2.0 * self.e032 * other.e12 * other.e123 - 2.0 * self.e032 * other.e021 * other.e0123 + 2.0 * self.e123 * other.s * other.e013 + 2.0 * self.e123 * other.e0 * other.e31 - 2.0 * self.e123 * other.e1 * other.e03 + 2.0 * self.e123 * other.e01 * other.e3 - 2.0 * self.e123 * other.e2 * other.e0123 + 2.0 * self.e123 * other.e02 * other.e123 - 2.0 * self.e123 * other.e12 * other.e032 + 2.0 * self.e123 * other.e021 * other.e23,
            e12: -2.0 * self.e021 * other.s * other.e0 - 2.0 * self.e021 * other.e1 * other.e01 - 2.0 * self.e021 * other.e2 * other.e02 + 2.0 * self.e021 * other.e12 * other.e021 + 2.0 * self.e021 * other.e3 * other.e03 - 2.0 * self.e021 * other.e31 * other.e013 - 2.0 * self.e021 * other.e23 * other.e032 + 2.0 * self.e021 * other.e123 * other.e0123 - 2.0 * self.e013 * other.s * other.e032 + 2.0 * self.e013 * other.e0 * other.e23 + 2.0 * self.e013 * other.e1 * other.e0123 + 2.0 * self.e013 * other.e01 * other.e123 + 2.0 * self.e013 * other.e2 * other.e03 + 2.0 * self.e013 * other.e02 * other.e3 + 2.0 * self.e013 * other.e12 * other.e013 + 2.0 * self.e013 * other.e021 * other.e31 + 2.0 * self.e032 * other.s * other.e013 - 2.0 * self.e032 * other.e0 * other.e31 + 2.0 * self.e032 * other.e1 * other.e03 + 2.0 * self.e032 * other.e01 * other.e3 - 2.0 * self.e032 * other.e2 * other.e0123 - 2.0 * self.e032 * other.e02 * other.e123 + 2.0 * self.e032 * other.e12 * other.e032 + 2.0 * self.e032 * other.e021 * other.e23 + 2.0 * self.e123 * other.s * other.e3 + 2.0 * self.e123 * other.e0 * other.e03 + 2.0 * self.e123 * other.e1 * other.e31 - 2.0 * self.e123 * other.e01 * other.e013 - 2.0 * self.e123 * other.e2 * other.e23 + 2.0 * self.e123 * other.e02 * other.e032 + 2.0 * self.e123 * other.e12 * other.e123 - 2.0 * self.e123 * other.e021 * other.e0123,
            e021: self.e021 * other.s * other.s - self.e021 * other.e0 * other.e0 + self.e021 * other.e1 * other.e1 - self.e021 * other.e01 * other.e01 + self.e021 * other.e2 * other.e2 - self.e021 * other.e02 * other.e02 + self.e021 * other.e12 * other.e12 - self.e021 * other.e021 * other.e021 - self.e021 * other.e3 * other.e3 + self.e021 * other.e03 * other.e03 - self.e021 * other.e31 * other.e31 + self.e021 * other.e013 * other.e013 - self.e021 * other.e23 * other.e23 + self.e021 * other.e032 * other.e032 - self.e021 * other.e123 * other.e123 + self.e021 * other.e0123 * other.e0123 - 2.0 * self.e013 * other.s * other.e23 - 2.0 * self.e013 * other.e0 * other.e032 - 2.0 * self.e013 * other.e1 * other.e123 + 2.0 * self.e013 * other.e01 * other.e0123 - 2.0 * self.e013 * other.e2 * other.e3 + 2.0 * self.e013 * other.e02 * other.e03 + 2.0 * self.e013 * other.e12 * other.e31 - 2.0 * self.e013 * other.e021 * other.e013 + 2.0 * self.e032 * other.s * other.e31 + 2.0 * self.e032 * other.e0 * other.e013 - 2.0 * self.e032 * other.e1 * other.e3 + 2.0 * self.e032 * other.e01 * other.e03 + 2.0 * self.e032 * other.e2 * other.e123 - 2.0 * self.e032 * other.e02 * other.e0123 + 2.0 * self.e032 * other.e12 * other.e23 - 2.0 * self.e032 * other.e021 * other.e032 - 2.0 * self.e123 * other.s * other.e03 - 2.0 * self.e123 * other.e0 * other.e3 + 2.0 * self.e123 * other.e1 * other.e013 - 2.0 * self.e123 * other.e01 * other.e31 - 2.0 * self.e123 * other.e2 * other.e032 + 2.0 * self.e123 * other.e02 * other.e23 - 2.0 * self.e123 * other.e12 * other.e0123 + 2.0 * self.e123 * other.e021 * other.e123,
            e3: 0.0,
            e03: -2.0 * self.e021 * other.s * other.e123 - 2.0 * self.e021 * other.e0 * other.e0123 - 2.0 * self.e021 * other.e1 * other.e23 + 2.0 * self.e021 * other.e01 * other.e032 - 2.0 * self.e021 * other.e2 * other.e31 + 2.0 * self.e021 * other.e02 * other.e013 + 2.0 * self.e021 * other.e12 * other.e3 - 2.0 * self.e021 * other.e021 * other.e03 - 2.0 * self.e013 * other.s * other.e1 - 2.0 * self.e013 * other.e0 * other.e01 + 2.0 * self.e013 * other.e2 * other.e12 - 2.0 * self.e013 * other.e02 * other.e021 + 2.0 * self.e013 * other.e3 * other.e31 - 2.0 * self.e013 * other.e03 * other.e013 + 2.0 * self.e013 * other.e23 * other.e123 - 2.0 * self.e013 * other.e032 * other.e0123 + 2.0 * self.e032 * other.s * other.e2 + 2.0 * self.e032 * other.e0 * other.e02 + 2.0 * self.e032 * other.e1 * other.e12 - 2.0 * self.e032 * other.e01 * other.e021 + 2.0 * self.e032 * other.e3 * other.e23 - 2.0 * self.e032 * other.e03 * other.e032 - 2.0 * self.e032 * other.e31 * other.e123 + 2.0 * self.e032 * other.e013 * other.e0123 + 2.0 * self.e123 * other.s * other.e021 + 2.0 * self.e123 * other.e0 * other.e12 + 2.0 * self.e123 * other.e1 * other.e02 - 2.0 * self.e123 * other.e01 * other.e2 - 2.0 * self.e123 * other.e3 * other.e0123 + 2.0 * self.e123 * other.e03 * other.e123 + 2.0 * self.e123 * other.e31 * other.e032 - 2.0 * self.e123 * other.e013 * other.e23,
            e31: 2.0 * self.e021 * other.s * other.e032 - 2.0 * self.e021 * other.e0 * other.e23 - 2.0 * self.e021 * other.e1 * other.e0123 - 2.0 * self.e021 * other.e01 * other.e123 + 2.0 * self.e021 * other.e2 * other.e03 + 2.0 * self.e021 * other.e02 * other.e3 + 2.0 * self.e021 * other.e12 * other.e013 + 2.0 * self.e021 * other.e021 * other.e31 - 2.0 * self.e013 * other.s * other.e0 - 2.0 * self.e013 * other.e1 * other.e01 + 2.0 * self.e013 * other.e2 * other.e02 - 2.0 * self.e013 * other.e12 * other.e021 - 2.0 * self.e013 * other.e3 * other.e03 + 2.0 * self.e013 * other.e31 * other.e013 - 2.0 * self.e013 * other.e23 * other.e032 + 2.0 * self.e013 * other.e123 * other.e0123 - 2.0 * self.e032 * other.s * other.e021 + 2.0 * self.e032 * other.e0 * other.e12 + 2.0 * self.e032 * other.e1 * other.e02 + 2.0 * self.e032 * other.e01 * other.e2 + 2.0 * self.e032 * other.e3 * other.e0123 + 2.0 * self.e032 * other.e03 * other.e123 + 2.0 * self.e032 * other.e31 * other.e032 + 2.0 * self.e032 * other.e013 * other.e23 + 2.0 * self.e123 * other.s * other.e2 + 2.0 * self.e123 * other.e0 * other.e02 - 2.0 * self.e123 * other.e1 * other.e12 + 2.0 * self.e123 * other.e01 * other.e021 + 2.0 * self.e123 * other.e3 * other.e23 - 2.0 * self.e123 * other.e03 * other.e032 + 2.0 * self.e123 * other.e31 * other.e123 - 2.0 * self.e123 * other.e013 * other.e0123,
            e013: 2.0 * self.e021 * other.s * other.e23 + 2.0 * self.e021 * other.e0 * other.e032 + 2.0 * self.e021 * other.e1 * other.e123 - 2.0 * self.e021 * other.e01 * other.e0123 - 2.0 * self.e021 * other.e2 * other.e3 + 2.0 * self.e021 * other.e02 * other.e03 + 2.0 * self.e021 * other.e12 * other.e31 - 2.0 * self.e021 * other.e021 * other.e013 + self.e013 * other.s * other.s - self.e013 * other.e0 * other.e0 + self.e013 * other.e1 * other.e1 - self.e013 * other.e01 * other.e01 - self.e013 * other.e2 * other.e2 + self.e013 * other.e02 * other.e02 - self.e013 * other.e12 * other.e12 + self.e013 * other.e021 * other.e021 + self.e013 * other.e3 * other.e3 - self.e013 * other.e03 * other.e03 + self.e013 * other.e31 * other.e31 - self.e013 * other.e013 * other.e013 - self.e013 * other.e23 * other.e23 + self.e013 * other.e032 * other.e032 - self.e013 * other.e123 * other.e123 + self.e013 * other.e0123 * other.e0123 - 2.0 * self.e032 * other.s * other.e12 - 2.0 * self.e032 * other.e0 * other.e021 - 2.0 * self.e032 * other.e1 * other.e2 + 2.0 * self.e032 * other.e01 * other.e02 - 2.0 * self.e032 * other.e3 * other.e123 + 2.0 * self.e032 * other.e03 * other.e0123 + 2.0 * self.e032 * other.e31 * other.e23 - 2.0 * self.e032 * other.e013 * other.e032 - 2.0 * self.e123 * other.s * other.e02 - 2.0 * self.e123 * other.e0 * other.e2 - 2.0 * self.e123 * other.e1 * other.e021 + 2.0 * self.e123 * other.e01 * other.e12 + 2.0 * self.e123 * other.e3 * other.e032 - 2.0 * self.e123 * other.e03 * other.e23 - 2.0 * self.e123 * other.e31 * other.e0123 + 2.0 * self.e123 * other.e013 * other.e123,
            e23: -2.0 * self.e021 * other.s * other.e013 + 2.0 * self.e021 * other.e0 * other.e31 + 2.0 * self.e021 * other.e1 * other.e03 + 2.0 * self.e021 * other.e01 * other.e3 + 2.0 * self.e021 * other.e2 * other.e0123 + 2.0 * self.e021 * other.e02 * other.e123 + 2.0 * self.e021 * other.e12 * other.e032 + 2.0 * self.e021 * other.e021 * other.e23 + 2.0 * self.e013 * other.s * other.e021 - 2.0 * self.e013 * other.e0 * other.e12 + 2.0 * self.e013 * other.e1 * other.e02 + 2.0 * self.e013 * other.e01 * other.e2 - 2.0 * self.e013 * other.e3 * other.e0123 - 2.0 * self.e013 * other.e03 * other.e123 + 2.0 * self.e013 * other.e31 * other.e032 + 2.0 * self.e013 * other.e013 * other.e23 - 2.0 * self.e032 * other.s * other.e0 + 2.0 * self.e032 * other.e1 * other.e01 - 2.0 * self.e032 * other.e2 * other.e02 - 2.0 * self.e032 * other.e12 * other.e021 - 2.0 * self.e032 * other.e3 * other.e03 - 2.0 * self.e032 * other.e31 * other.e013 + 2.0 * self.e032 * other.e23 * other.e032 + 2.0 * self.e032 * other.e123 * other.e0123 + 2.0 * self.e123 * other.s * other.e1 + 2.0 * self.e123 * other.e0 * other.e01 + 2.0 * self.e123 * other.e2 * other.e12 - 2.0 * self.e123 * other.e02 * other.e021 - 2.0 * self.e123 * other.e3 * other.e31 + 2.0 * self.e123 * other.e03 * other.e013 + 2.0 * self.e123 * other.e23 * other.e123 - 2.0 * self.e123 * other.e032 * other.e0123,
            e032: -2.0 * self.e021 * other.s * other.e31 - 2.0 * self.e021 * other.e0 * other.e013 - 2.0 * self.e021 * other.e1 * other.e3 + 2.0 * self.e021 * other.e01 * other.e03 - 2.0 * self.e021 * other.e2 * other.e123 + 2.0 * self.e021 * other.e02 * other.e0123 + 2.0 * self.e021 * other.e12 * other.e23 - 2.0 * self.e021 * other.e021 * other.e032 + 2.0 * self.e013 * other.s * other.e12 + 2.0 * self.e013 * other.e0 * other.e021 - 2.0 * self.e013 * other.e1 * other.e2 + 2.0 * self.e013 * other.e01 * other.e02 + 2.0 * self.e013 * other.e3 * other.e123 - 2.0 * self.e013 * other.e03 * other.e0123 + 2.0 * self.e013 * other.e31 * other.e23 - 2.0 * self.e013 * other.e013 * other.e032 + self.e032 * other.s * other.s - self.e032 * other.e0 * other.e0 - self.e032 * other.e1 * other.e1 + self.e032 * other.e01 * other.e01 + self.e032 * other.e2 * other.e2 - self.e032 * other.e02 * other.e02 - self.e032 * other.e12 * other.e12 + self.e032 * other.e021 * other.e021 + self.e032 * other.e3 * other.e3 - self.e032 * other.e03 * other.e03 - self.e032 * other.e31 * other.e31 + self.e032 * other.e013 * other.e013 + self.e032 * other.e23 * other.e23 - self.e032 * other.e032 * other.e032 - self.e032 * other.e123 * other.e123 + self.e032 * other.e0123 * other.e0123 - 2.0 * self.e123 * other.s * other.e01 - 2.0 * self.e123 * other.e0 * other.e1 + 2.0 * self.e123 * other.e2 * other.e021 - 2.0 * self.e123 * other.e02 * other.e12 - 2.0 * self.e123 * other.e3 * other.e013 + 2.0 * self.e123 * other.e03 * other.e31 - 2.0 * self.e123 * other.e23 * other.e0123 + 2.0 * self.e123 * other.e032 * other.e123,
            e123: 2.0 * self.e021 * other.s * other.e03 - 2.0 * self.e021 * other.e0 * other.e3 - 2.0 * self.e021 * other.e1 * other.e013 - 2.0 * self.e021 * other.e01 * other.e31 + 2.0 * self.e021 * other.e2 * other.e032 + 2.0 * self.e021 * other.e02 * other.e23 + 2.0 * self.e021 * other.e12 * other.e0123 + 2.0 * self.e021 * other.e021 * other.e123 + 2.0 * self.e013 * other.s * other.e02 - 2.0 * self.e013 * other.e0 * other.e2 + 2.0 * self.e013 * other.e1 * other.e021 + 2.0 * self.e013 * other.e01 * other.e12 - 2.0 * self.e013 * other.e3 * other.e032 - 2.0 * self.e013 * other.e03 * other.e23 + 2.0 * self.e013 * other.e31 * other.e0123 + 2.0 * self.e013 * other.e013 * other.e123 + 2.0 * self.e032 * other.s * other.e01 - 2.0 * self.e032 * other.e0 * other.e1 - 2.0 * self.e032 * other.e2 * other.e021 - 2.0 * self.e032 * other.e02 * other.e12 + 2.0 * self.e032 * other.e3 * other.e013 + 2.0 * self.e032 * other.e03 * other.e31 + 2.0 * self.e032 * other.e23 * other.e0123 + 2.0 * self.e032 * other.e032 * other.e123 + self.e123 * other.s * other.s - self.e123 * other.e0 * other.e0 + self.e123 * other.e1 * other.e1 - self.e123 * other.e01 * other.e01 + self.e123 * other.e2 * other.e2 - self.e123 * other.e02 * other.e02 + self.e123 * other.e12 * other.e12 - self.e123 * other.e021 * other.e021 + self.e123 * other.e3 * other.e3 - self.e123 * other.e03 * other.e03 + self.e123 * other.e31 * other.e31 - self.e123 * other.e013 * other.e013 + self.e123 * other.e23 * other.e23 - self.e123 * other.e032 * other.e032 + self.e123 * other.e123 * other.e123 - self.e123 * other.e0123 * other.e0123,
            e0123: 0.0,
        }
    }
}

impl Transform<Scalar> for FourVector {
    type Output = EvenMultivector;
    fn transform(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: self.e0123 * other.s * other.s,
        }
    }
}

impl Transform<Vector> for FourVector {
    type Output = EvenMultivector;
    fn transform(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: self.e0123 * other.e0 * other.e0 - self.e0123 * other.e1 * other.e1 - self.e0123 * other.e2 * other.e2 - self.e0123 * other.e3 * other.e3,
        }
    }
}

impl Transform<Bivector> for FourVector {
    type Output = EvenMultivector;
    fn transform(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: -2.0 * self.e0123 * other.e01 * other.e23 - 2.0 * self.e0123 * other.e02 * other.e31 - 2.0 * self.e0123 * other.e12 * other.e03,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: -self.e0123 * other.e01 * other.e01 - self.e0123 * other.e02 * other.e02 + self.e0123 * other.e12 * other.e12 - self.e0123 * other.e03 * other.e03 + self.e0123 * other.e31 * other.e31 + self.e0123 * other.e23 * other.e23,
        }
    }
}

impl Transform<Trivector> for FourVector {
    type Output = EvenMultivector;
    fn transform(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: self.e0123 * other.e021 * other.e021 + self.e0123 * other.e013 * other.e013 + self.e0123 * other.e032 * other.e032 - self.e0123 * other.e123 * other.e123,
        }
    }
}

impl Transform<FourVector> for FourVector {
    type Output = EvenMultivector;
    fn transform(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: -self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Transform<Null> for FourVector {
    type Output = Null;
    fn transform(self, other: Null) -> Null {
        Null
    }
}

impl Transform<OddMultivector> for FourVector {
    type Output = EvenMultivector;
    fn transform(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 2.0 * self.e0123 * other.e0 * other.e123 + 2.0 * self.e0123 * other.e1 * other.e032 + 2.0 * self.e0123 * other.e2 * other.e013 + 2.0 * self.e0123 * other.e021 * other.e3,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: self.e0123 * other.e0 * other.e0 - self.e0123 * other.e1 * other.e1 - self.e0123 * other.e2 * other.e2 + self.e0123 * other.e021 * other.e021 - self.e0123 * other.e3 * other.e3 + self.e0123 * other.e013 * other.e013 + self.e0123 * other.e032 * other.e032 - self.e0123 * other.e123 * other.e123,
        }
    }
}

impl Transform<EvenMultivector> for FourVector {
    type Output = EvenMultivector;
    fn transform(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 2.0 * self.e0123 * other.s * other.e0123 - 2.0 * self.e0123 * other.e01 * other.e23 - 2.0 * self.e0123 * other.e02 * other.e31 - 2.0 * self.e0123 * other.e12 * other.e03,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: self.e0123 * other.s * other.s - self.e0123 * other.e01 * other.e01 - self.e0123 * other.e02 * other.e02 + self.e0123 * other.e12 * other.e12 - self.e0123 * other.e03 * other.e03 + self.e0123 * other.e31 * other.e31 + self.e0123 * other.e23 * other.e23 - self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Transform<Multivector> for FourVector {
    type Output = Multivector;
    fn transform(self, other: Multivector) -> Multivector {
        Multivector {
            s: 2.0 * self.e0123 * other.s * other.e0123 + 2.0 * self.e0123 * other.e0 * other.e123 + 2.0 * self.e0123 * other.e1 * other.e032 - 2.0 * self.e0123 * other.e01 * other.e23 + 2.0 * self.e0123 * other.e2 * other.e013 - 2.0 * self.e0123 * other.e02 * other.e31 - 2.0 * self.e0123 * other.e12 * other.e03 + 2.0 * self.e0123 * other.e021 * other.e3,
            e0: 2.0 * self.e0123 * other.s * other.e123 - 2.0 * self.e0123 * other.e0 * other.e0123 - 2.0 * self.e0123 * other.e1 * other.e23 - 2.0 * self.e0123 * other.e01 * other.e032 - 2.0 * self.e0123 * other.e2 * other.e31 - 2.0 * self.e0123 * other.e02 * other.e013 - 2.0 * self.e0123 * other.e12 * other.e3 - 2.0 * self.e0123 * other.e021 * other.e03,
            e1: 2.0 * self.e0123 * other.s * other.e032 + 2.0 * self.e0123 * other.e0 * other.e23 + 2.0 * self.e0123 * other.e1 * other.e0123 - 2.0 * self.e0123 * other.e01 * other.e123 - 2.0 * self.e0123 * other.e2 * other.e03 + 2.0 * self.e0123 * other.e02 * other.e3 + 2.0 * self.e0123 * other.e12 * other.e013 - 2.0 * self.e0123 * other.e021 * other.e31,
            e01: 0.0,
            e2: 2.0 * self.e0123 * other.s * other.e013 + 2.0 * self.e0123 * other.e0 * other.e31 + 2.0 * self.e0123 * other.e1 * other.e03 - 2.0 * self.e0123 * other.e01 * other.e3 + 2.0 * self.e0123 * other.e2 * other.e0123 - 2.0 * self.e0123 * other.e02 * other.e123 - 2.0 * self.e0123 * other.e12 * other.e032 + 2.0 * self.e0123 * other.e021 * other.e23,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 2.0 * self.e0123 * other.s * other.e021 + 2.0 * self.e0123 * other.e0 * other.e12 - 2.0 * self.e0123 * other.e1 * other.e02 + 2.0 * self.e0123 * other.e01 * other.e2 + 2.0 * self.e0123 * other.e3 * other.e0123 - 2.0 * self.e0123 * other.e03 * other.e123 + 2.0 * self.e0123 * other.e31 * other.e032 - 2.0 * self.e0123 * other.e013 * other.e23,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: self.e0123 * other.s * other.s + self.e0123 * other.e0 * other.e0 - self.e0123 * other.e1 * other.e1 - self.e0123 * other.e01 * other.e01 - self.e0123 * other.e2 * other.e2 - self.e0123 * other.e02 * other.e02 + self.e0123 * other.e12 * other.e12 + self.e0123 * other.e021 * other.e021 - self.e0123 * other.e3 * other.e3 - self.e0123 * other.e03 * other.e03 + self.e0123 * other.e31 * other.e31 + self.e0123 * other.e013 * other.e013 + self.e0123 * other.e23 * other.e23 + self.e0123 * other.e032 * other.e032 - self.e0123 * other.e123 * other.e123 - self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Transform<Scalar> for Null {
    type Output = Null;
    fn transform(self, other: Scalar) -> Null {
        Null
    }
}

impl Transform<Vector> for Null {
    type Output = Null;
    fn transform(self, other: Vector) -> Null {
        Null
    }
}

impl Transform<Bivector> for Null {
    type Output = Null;
    fn transform(self, other: Bivector) -> Null {
        Null
    }
}

impl Transform<Trivector> for Null {
    type Output = Null;
    fn transform(self, other: Trivector) -> Null {
        Null
    }
}

impl Transform<FourVector> for Null {
    type Output = Null;
    fn transform(self, other: FourVector) -> Null {
        Null
    }
}

impl Transform<Null> for Null {
    type Output = Null;
    fn transform(self, other: Null) -> Null {
        Null
    }
}

impl Transform<OddMultivector> for Null {
    type Output = Null;
    fn transform(self, other: OddMultivector) -> Null {
        Null
    }
}

impl Transform<EvenMultivector> for Null {
    type Output = Null;
    fn transform(self, other: EvenMultivector) -> Null {
        Null
    }
}

impl Transform<Multivector> for Null {
    type Output = Null;
    fn transform(self, other: Multivector) -> Null {
        Null
    }
}

impl Transform<Scalar> for OddMultivector {
    type Output = OddMultivector;
    fn transform(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s * other.s,
            e1: self.e1 * other.s * other.s,
            e2: self.e2 * other.s * other.s,
            e021: self.e021 * other.s * other.s,
            e3: self.e3 * other.s * other.s,
            e013: self.e013 * other.s * other.s,
            e032: self.e032 * other.s * other.s,
            e123: self.e123 * other.s * other.s,
        }
    }
}

impl Transform<Vector> for OddMultivector {
    type Output = OddMultivector;
    fn transform(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 - self.e0 * other.e2 * other.e2 - self.e0 * other.e3 * other.e3 + 2.0 * self.e1 * other.e0 * other.e1 + 2.0 * self.e2 * other.e0 * other.e2 + 2.0 * self.e3 * other.e0 * other.e3,
            e1: 2.0 * self.e0 * other.e0 * other.e1 - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1 - self.e1 * other.e2 * other.e2 - self.e1 * other.e3 * other.e3 + 2.0 * self.e2 * other.e1 * other.e2 + 2.0 * self.e3 * other.e1 * other.e3,
            e2: 2.0 * self.e0 * other.e0 * other.e2 + 2.0 * self.e1 * other.e1 * other.e2 - self.e2 * other.e0 * other.e0 - self.e2 * other.e1 * other.e1 + self.e2 * other.e2 * other.e2 - self.e2 * other.e3 * other.e3 + 2.0 * self.e3 * other.e2 * other.e3,
            e021: -self.e021 * other.e0 * other.e0 + self.e021 * other.e1 * other.e1 + self.e021 * other.e2 * other.e2 - self.e021 * other.e3 * other.e3 - 2.0 * self.e013 * other.e2 * other.e3 - 2.0 * self.e032 * other.e1 * other.e3 - 2.0 * self.e123 * other.e0 * other.e3,
            e3: 2.0 * self.e0 * other.e0 * other.e3 + 2.0 * self.e1 * other.e1 * other.e3 + 2.0 * self.e2 * other.e2 * other.e3 - self.e3 * other.e0 * other.e0 - self.e3 * other.e1 * other.e1 - self.e3 * other.e2 * other.e2 + self.e3 * other.e3 * other.e3,
            e013: -2.0 * self.e021 * other.e2 * other.e3 - self.e013 * other.e0 * other.e0 + self.e013 * other.e1 * other.e1 - self.e013 * other.e2 * other.e2 + self.e013 * other.e3 * other.e3 - 2.0 * self.e032 * other.e1 * other.e2 - 2.0 * self.e123 * other.e0 * other.e2,
            e032: -2.0 * self.e021 * other.e1 * other.e3 - 2.0 * self.e013 * other.e1 * other.e2 - self.e032 * other.e0 * other.e0 - self.e032 * other.e1 * other.e1 + self.e032 * other.e2 * other.e2 + self.e032 * other.e3 * other.e3 - 2.0 * self.e123 * other.e0 * other.e1,
            e123: -2.0 * self.e021 * other.e0 * other.e3 - 2.0 * self.e013 * other.e0 * other.e2 - 2.0 * self.e032 * other.e0 * other.e1 - self.e123 * other.e0 * other.e0 + self.e123 * other.e1 * other.e1 + self.e123 * other.e2 * other.e2 + self.e123 * other.e3 * other.e3,
        }
    }
}

impl Transform<Bivector> for OddMultivector {
    type Output = OddMultivector;
    fn transform(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.e01 * other.e01 + self.e0 * other.e02 * other.e02 + self.e0 * other.e12 * other.e12 + self.e0 * other.e03 * other.e03 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 - 2.0 * self.e1 * other.e02 * other.e12 + 2.0 * self.e1 * other.e03 * other.e31 + 2.0 * self.e2 * other.e01 * other.e12 - 2.0 * self.e2 * other.e03 * other.e23 - 2.0 * self.e3 * other.e01 * other.e31 + 2.0 * self.e3 * other.e02 * other.e23,
            e1: -2.0 * self.e0 * other.e02 * other.e12 + 2.0 * self.e0 * other.e03 * other.e31 - self.e1 * other.e01 * other.e01 + self.e1 * other.e02 * other.e02 - self.e1 * other.e12 * other.e12 + self.e1 * other.e03 * other.e03 - self.e1 * other.e31 * other.e31 + self.e1 * other.e23 * other.e23 - 2.0 * self.e2 * other.e01 * other.e02 + 2.0 * self.e2 * other.e31 * other.e23 - 2.0 * self.e3 * other.e01 * other.e03 + 2.0 * self.e3 * other.e12 * other.e23,
            e2: 2.0 * self.e0 * other.e01 * other.e12 - 2.0 * self.e0 * other.e03 * other.e23 - 2.0 * self.e1 * other.e01 * other.e02 + 2.0 * self.e1 * other.e31 * other.e23 + self.e2 * other.e01 * other.e01 - self.e2 * other.e02 * other.e02 - self.e2 * other.e12 * other.e12 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e2 * other.e23 * other.e23 - 2.0 * self.e3 * other.e02 * other.e03 + 2.0 * self.e3 * other.e12 * other.e31,
            e021: -self.e021 * other.e01 * other.e01 - self.e021 * other.e02 * other.e02 + self.e021 * other.e12 * other.e12 + self.e021 * other.e03 * other.e03 - self.e021 * other.e31 * other.e31 - self.e021 * other.e23 * other.e23 + 2.0 * self.e013 * other.e02 * other.e03 + 2.0 * self.e013 * other.e12 * other.e31 + 2.0 * self.e032 * other.e01 * other.e03 + 2.0 * self.e032 * other.e12 * other.e23 - 2.0 * self.e123 * other.e01 * other.e31 + 2.0 * self.e123 * other.e02 * other.e23,
            e3: -2.0 * self.e0 * other.e01 * other.e31 + 2.0 * self.e0 * other.e02 * other.e23 - 2.0 * self.e1 * other.e01 * other.e03 + 2.0 * self.e1 * other.e12 * other.e23 - 2.0 * self.e2 * other.e02 * other.e03 + 2.0 * self.e2 * other.e12 * other.e31 + self.e3 * other.e01 * other.e01 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12 - self.e3 * other.e03 * other.e03 - self.e3 * other.e31 * other.e31 - self.e3 * other.e23 * other.e23,
            e013: 2.0 * self.e021 * other.e02 * other.e03 + 2.0 * self.e021 * other.e12 * other.e31 - self.e013 * other.e01 * other.e01 + self.e013 * other.e02 * other.e02 - self.e013 * other.e12 * other.e12 - self.e013 * other.e03 * other.e03 + self.e013 * other.e31 * other.e31 - self.e013 * other.e23 * other.e23 + 2.0 * self.e032 * other.e01 * other.e02 + 2.0 * self.e032 * other.e31 * other.e23 + 2.0 * self.e123 * other.e01 * other.e12 - 2.0 * self.e123 * other.e03 * other.e23,
            e032: 2.0 * self.e021 * other.e01 * other.e03 + 2.0 * self.e021 * other.e12 * other.e23 + 2.0 * self.e013 * other.e01 * other.e02 + 2.0 * self.e013 * other.e31 * other.e23 + self.e032 * other.e01 * other.e01 - self.e032 * other.e02 * other.e02 - self.e032 * other.e12 * other.e12 - self.e032 * other.e03 * other.e03 - self.e032 * other.e31 * other.e31 + self.e032 * other.e23 * other.e23 - 2.0 * self.e123 * other.e02 * other.e12 + 2.0 * self.e123 * other.e03 * other.e31,
            e123: -2.0 * self.e021 * other.e01 * other.e31 + 2.0 * self.e021 * other.e02 * other.e23 + 2.0 * self.e013 * other.e01 * other.e12 - 2.0 * self.e013 * other.e03 * other.e23 - 2.0 * self.e032 * other.e02 * other.e12 + 2.0 * self.e032 * other.e03 * other.e31 - self.e123 * other.e01 * other.e01 - self.e123 * other.e02 * other.e02 + self.e123 * other.e12 * other.e12 - self.e123 * other.e03 * other.e03 + self.e123 * other.e31 * other.e31 + self.e123 * other.e23 * other.e23,
        }
    }
}

impl Transform<Trivector> for OddMultivector {
    type Output = OddMultivector;
    fn transform(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e021 * other.e021 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 - self.e0 * other.e123 * other.e123 - 2.0 * self.e1 * other.e032 * other.e123 - 2.0 * self.e2 * other.e013 * other.e123 - 2.0 * self.e3 * other.e021 * other.e123,
            e1: -2.0 * self.e0 * other.e032 * other.e123 + self.e1 * other.e021 * other.e021 + self.e1 * other.e013 * other.e013 - self.e1 * other.e032 * other.e032 + self.e1 * other.e123 * other.e123 - 2.0 * self.e2 * other.e013 * other.e032 - 2.0 * self.e3 * other.e021 * other.e032,
            e2: -2.0 * self.e0 * other.e013 * other.e123 - 2.0 * self.e1 * other.e013 * other.e032 + self.e2 * other.e021 * other.e021 - self.e2 * other.e013 * other.e013 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 - 2.0 * self.e3 * other.e021 * other.e013,
            e021: -self.e021 * other.e021 * other.e021 + self.e021 * other.e013 * other.e013 + self.e021 * other.e032 * other.e032 - self.e021 * other.e123 * other.e123 - 2.0 * self.e013 * other.e021 * other.e013 - 2.0 * self.e032 * other.e021 * other.e032 + 2.0 * self.e123 * other.e021 * other.e123,
            e3: -2.0 * self.e0 * other.e021 * other.e123 - 2.0 * self.e1 * other.e021 * other.e032 - 2.0 * self.e2 * other.e021 * other.e013 - self.e3 * other.e021 * other.e021 + self.e3 * other.e013 * other.e013 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123,
            e013: -2.0 * self.e021 * other.e021 * other.e013 + self.e013 * other.e021 * other.e021 - self.e013 * other.e013 * other.e013 + self.e013 * other.e032 * other.e032 - self.e013 * other.e123 * other.e123 - 2.0 * self.e032 * other.e013 * other.e032 + 2.0 * self.e123 * other.e013 * other.e123,
            e032: -2.0 * self.e021 * other.e021 * other.e032 - 2.0 * self.e013 * other.e013 * other.e032 + self.e032 * other.e021 * other.e021 + self.e032 * other.e013 * other.e013 - self.e032 * other.e032 * other.e032 - self.e032 * other.e123 * other.e123 + 2.0 * self.e123 * other.e032 * other.e123,
            e123: 2.0 * self.e021 * other.e021 * other.e123 + 2.0 * self.e013 * other.e013 * other.e123 + 2.0 * self.e032 * other.e032 * other.e123 - self.e123 * other.e021 * other.e021 - self.e123 * other.e013 * other.e013 - self.e123 * other.e032 * other.e032 + self.e123 * other.e123 * other.e123,
        }
    }
}

impl Transform<FourVector> for OddMultivector {
    type Output = OddMultivector;
    fn transform(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.e0123 * other.e0123,
            e1: -self.e1 * other.e0123 * other.e0123,
            e2: -self.e2 * other.e0123 * other.e0123,
            e021: self.e021 * other.e0123 * other.e0123,
            e3: -self.e3 * other.e0123 * other.e0123,
            e013: self.e013 * other.e0123 * other.e0123,
            e032: self.e032 * other.e0123 * other.e0123,
            e123: -self.e123 * other.e0123 * other.e0123,
        }
    }
}

impl Transform<Null> for OddMultivector {
    type Output = Null;
    fn transform(self, other: Null) -> Null {
        Null
    }
}

impl Transform<OddMultivector> for OddMultivector {
    type Output = OddMultivector;
    fn transform(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 - self.e0 * other.e2 * other.e2 - self.e0 * other.e021 * other.e021 - self.e0 * other.e3 * other.e3 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 - self.e0 * other.e123 * other.e123 + 2.0 * self.e1 * other.e0 * other.e1 + 2.0 * self.e1 * other.e2 * other.e021 - 2.0 * self.e1 * other.e3 * other.e013 - 2.0 * self.e1 * other.e032 * other.e123 + 2.0 * self.e2 * other.e0 * other.e2 - 2.0 * self.e2 * other.e1 * other.e021 + 2.0 * self.e2 * other.e3 * other.e032 - 2.0 * self.e2 * other.e013 * other.e123 + 2.0 * self.e3 * other.e0 * other.e3 + 2.0 * self.e3 * other.e1 * other.e013 - 2.0 * self.e3 * other.e2 * other.e032 - 2.0 * self.e3 * other.e021 * other.e123,
            e1: 2.0 * self.e0 * other.e0 * other.e1 - 2.0 * self.e0 * other.e2 * other.e021 + 2.0 * self.e0 * other.e3 * other.e013 - 2.0 * self.e0 * other.e032 * other.e123 - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1 - self.e1 * other.e2 * other.e2 + self.e1 * other.e021 * other.e021 - self.e1 * other.e3 * other.e3 + self.e1 * other.e013 * other.e013 - self.e1 * other.e032 * other.e032 + self.e1 * other.e123 * other.e123 + 2.0 * self.e2 * other.e0 * other.e021 + 2.0 * self.e2 * other.e1 * other.e2 - 2.0 * self.e2 * other.e3 * other.e123 - 2.0 * self.e2 * other.e013 * other.e032 - 2.0 * self.e3 * other.e0 * other.e013 + 2.0 * self.e3 * other.e1 * other.e3 + 2.0 * self.e3 * other.e2 * other.e123 - 2.0 * self.e3 * other.e021 * other.e032,
            e2: 2.0 * self.e0 * other.e0 * other.e2 + 2.0 * self.e0 * other.e1 * other.e021 - 2.0 * self.e0 * other.e3 * other.e032 - 2.0 * self.e0 * other.e013 * other.e123 - 2.0 * self.e1 * other.e0 * other.e021 + 2.0 * self.e1 * other.e1 * other.e2 + 2.0 * self.e1 * other.e3 * other.e123 - 2.0 * self.e1 * other.e013 * other.e032 - self.e2 * other.e0 * other.e0 - self.e2 * other.e1 * other.e1 + self.e2 * other.e2 * other.e2 + self.e2 * other.e021 * other.e021 - self.e2 * other.e3 * other.e3 - self.e2 * other.e013 * other.e013 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 + 2.0 * self.e3 * other.e0 * other.e032 - 2.0 * self.e3 * other.e1 * other.e123 + 2.0 * self.e3 * other.e2 * other.e3 - 2.0 * self.e3 * other.e021 * other.e013,
            e021: -self.e021 * other.e0 * other.e0 + self.e021 * other.e1 * other.e1 + self.e021 * other.e2 * other.e2 - self.e021 * other.e021 * other.e021 - self.e021 * other.e3 * other.e3 + self.e021 * other.e013 * other.e013 + self.e021 * other.e032 * other.e032 - self.e021 * other.e123 * other.e123 - 2.0 * self.e013 * other.e0 * other.e032 - 2.0 * self.e013 * other.e1 * other.e123 - 2.0 * self.e013 * other.e2 * other.e3 - 2.0 * self.e013 * other.e021 * other.e013 + 2.0 * self.e032 * other.e0 * other.e013 - 2.0 * self.e032 * other.e1 * other.e3 + 2.0 * self.e032 * other.e2 * other.e123 - 2.0 * self.e032 * other.e021 * other.e032 - 2.0 * self.e123 * other.e0 * other.e3 + 2.0 * self.e123 * other.e1 * other.e013 - 2.0 * self.e123 * other.e2 * other.e032 + 2.0 * self.e123 * other.e021 * other.e123,
            e3: 2.0 * self.e0 * other.e0 * other.e3 - 2.0 * self.e0 * other.e1 * other.e013 + 2.0 * self.e0 * other.e2 * other.e032 - 2.0 * self.e0 * other.e021 * other.e123 + 2.0 * self.e1 * other.e0 * other.e013 + 2.0 * self.e1 * other.e1 * other.e3 - 2.0 * self.e1 * other.e2 * other.e123 - 2.0 * self.e1 * other.e021 * other.e032 - 2.0 * self.e2 * other.e0 * other.e032 + 2.0 * self.e2 * other.e1 * other.e123 + 2.0 * self.e2 * other.e2 * other.e3 - 2.0 * self.e2 * other.e021 * other.e013 - self.e3 * other.e0 * other.e0 - self.e3 * other.e1 * other.e1 - self.e3 * other.e2 * other.e2 - self.e3 * other.e021 * other.e021 + self.e3 * other.e3 * other.e3 + self.e3 * other.e013 * other.e013 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123,
            e013: 2.0 * self.e021 * other.e0 * other.e032 + 2.0 * self.e021 * other.e1 * other.e123 - 2.0 * self.e021 * other.e2 * other.e3 - 2.0 * self.e021 * other.e021 * other.e013 - self.e013 * other.e0 * other.e0 + self.e013 * other.e1 * other.e1 - self.e013 * other.e2 * other.e2 + self.e013 * other.e021 * other.e021 + self.e013 * other.e3 * other.e3 - self.e013 * other.e013 * other.e013 + self.e013 * other.e032 * other.e032 - self.e013 * other.e123 * other.e123 - 2.0 * self.e032 * other.e0 * other.e021 - 2.0 * self.e032 * other.e1 * other.e2 - 2.0 * self.e032 * other.e3 * other.e123 - 2.0 * self.e032 * other.e013 * other.e032 - 2.0 * self.e123 * other.e0 * other.e2 - 2.0 * self.e123 * other.e1 * other.e021 + 2.0 * self.e123 * other.e3 * other.e032 + 2.0 * self.e123 * other.e013 * other.e123,
            e032: -2.0 * self.e021 * other.e0 * other.e013 - 2.0 * self.e021 * other.e1 * other.e3 - 2.0 * self.e021 * other.e2 * other.e123 - 2.0 * self.e021 * other.e021 * other.e032 + 2.0 * self.e013 * other.e0 * other.e021 - 2.0 * self.e013 * other.e1 * other.e2 + 2.0 * self.e013 * other.e3 * other.e123 - 2.0 * self.e013 * other.e013 * other.e032 - self.e032 * other.e0 * other.e0 - self.e032 * other.e1 * other.e1 + self.e032 * other.e2 * other.e2 + self.e032 * other.e021 * other.e021 + self.e032 * other.e3 * other.e3 + self.e032 * other.e013 * other.e013 - self.e032 * other.e032 * other.e032 - self.e032 * other.e123 * other.e123 - 2.0 * self.e123 * other.e0 * other.e1 + 2.0 * self.e123 * other.e2 * other.e021 - 2.0 * self.e123 * other.e3 * other.e013 + 2.0 * self.e123 * other.e032 * other.e123,
            e123: -2.0 * self.e021 * other.e0 * other.e3 - 2.0 * self.e021 * other.e1 * other.e013 + 2.0 * self.e021 * other.e2 * other.e032 + 2.0 * self.e021 * other.e021 * other.e123 - 2.0 * self.e013 * other.e0 * other.e2 + 2.0 * self.e013 * other.e1 * other.e021 - 2.0 * self.e013 * other.e3 * other.e032 + 2.0 * self.e013 * other.e013 * other.e123 - 2.0 * self.e032 * other.e0 * other.e1 - 2.0 * self.e032 * other.e2 * other.e021 + 2.0 * self.e032 * other.e3 * other.e013 + 2.0 * self.e032 * other.e032 * other.e123 - self.e123 * other.e0 * other.e0 + self.e123 * other.e1 * other.e1 + self.e123 * other.e2 * other.e2 - self.e123 * other.e021 * other.e021 + self.e123 * other.e3 * other.e3 - self.e123 * other.e013 * other.e013 - self.e123 * other.e032 * other.e032 + self.e123 * other.e123 * other.e123,
        }
    }
}

impl Transform<EvenMultivector> for OddMultivector {
    type Output = OddMultivector;
    fn transform(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s * other.s + self.e0 * other.e01 * other.e01 + self.e0 * other.e02 * other.e02 + self.e0 * other.e12 * other.e12 + self.e0 * other.e03 * other.e03 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 + self.e0 * other.e0123 * other.e0123 + 2.0 * self.e1 * other.s * other.e01 - 2.0 * self.e1 * other.e02 * other.e12 + 2.0 * self.e1 * other.e03 * other.e31 + 2.0 * self.e1 * other.e23 * other.e0123 + 2.0 * self.e2 * other.s * other.e02 + 2.0 * self.e2 * other.e01 * other.e12 - 2.0 * self.e2 * other.e03 * other.e23 + 2.0 * self.e2 * other.e31 * other.e0123 + 2.0 * self.e3 * other.s * other.e03 - 2.0 * self.e3 * other.e01 * other.e31 + 2.0 * self.e3 * other.e02 * other.e23 + 2.0 * self.e3 * other.e12 * other.e0123,
            e1: -2.0 * self.e0 * other.s * other.e01 - 2.0 * self.e0 * other.e02 * other.e12 + 2.0 * self.e0 * other.e03 * other.e31 - 2.0 * self.e0 * other.e23 * other.e0123 + self.e1 * other.s * other.s - self.e1 * other.e01 * other.e01 + self.e1 * other.e02 * other.e02 - self.e1 * other.e12 * other.e12 + self.e1 * other.e03 * other.e03 - self.e1 * other.e31 * other.e31 + self.e1 * other.e23 * other.e23 - self.e1 * other.e0123 * other.e0123 + 2.0 * self.e2 * other.s * other.e12 - 2.0 * self.e2 * other.e01 * other.e02 + 2.0 * self.e2 * other.e03 * other.e0123 + 2.0 * self.e2 * other.e31 * other.e23 - 2.0 * self.e3 * other.s * other.e31 - 2.0 * self.e3 * other.e01 * other.e03 - 2.0 * self.e3 * other.e02 * other.e0123 + 2.0 * self.e3 * other.e12 * other.e23,
            e2: -2.0 * self.e0 * other.s * other.e02 + 2.0 * self.e0 * other.e01 * other.e12 - 2.0 * self.e0 * other.e03 * other.e23 - 2.0 * self.e0 * other.e31 * other.e0123 - 2.0 * self.e1 * other.s * other.e12 - 2.0 * self.e1 * other.e01 * other.e02 - 2.0 * self.e1 * other.e03 * other.e0123 + 2.0 * self.e1 * other.e31 * other.e23 + self.e2 * other.s * other.s + self.e2 * other.e01 * other.e01 - self.e2 * other.e02 * other.e02 - self.e2 * other.e12 * other.e12 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e2 * other.e23 * other.e23 - self.e2 * other.e0123 * other.e0123 + 2.0 * self.e3 * other.s * other.e23 + 2.0 * self.e3 * other.e01 * other.e0123 - 2.0 * self.e3 * other.e02 * other.e03 + 2.0 * self.e3 * other.e12 * other.e31,
            e021: self.e021 * other.s * other.s - self.e021 * other.e01 * other.e01 - self.e021 * other.e02 * other.e02 + self.e021 * other.e12 * other.e12 + self.e021 * other.e03 * other.e03 - self.e021 * other.e31 * other.e31 - self.e021 * other.e23 * other.e23 + self.e021 * other.e0123 * other.e0123 - 2.0 * self.e013 * other.s * other.e23 + 2.0 * self.e013 * other.e01 * other.e0123 + 2.0 * self.e013 * other.e02 * other.e03 + 2.0 * self.e013 * other.e12 * other.e31 + 2.0 * self.e032 * other.s * other.e31 + 2.0 * self.e032 * other.e01 * other.e03 - 2.0 * self.e032 * other.e02 * other.e0123 + 2.0 * self.e032 * other.e12 * other.e23 - 2.0 * self.e123 * other.s * other.e03 - 2.0 * self.e123 * other.e01 * other.e31 + 2.0 * self.e123 * other.e02 * other.e23 - 2.0 * self.e123 * other.e12 * other.e0123,
            e3: -2.0 * self.e0 * other.s * other.e03 - 2.0 * self.e0 * other.e01 * other.e31 + 2.0 * self.e0 * other.e02 * other.e23 - 2.0 * self.e0 * other.e12 * other.e0123 + 2.0 * self.e1 * other.s * other.e31 - 2.0 * self.e1 * other.e01 * other.e03 + 2.0 * self.e1 * other.e02 * other.e0123 + 2.0 * self.e1 * other.e12 * other.e23 - 2.0 * self.e2 * other.s * other.e23 - 2.0 * self.e2 * other.e01 * other.e0123 - 2.0 * self.e2 * other.e02 * other.e03 + 2.0 * self.e2 * other.e12 * other.e31 + self.e3 * other.s * other.s + self.e3 * other.e01 * other.e01 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12 - self.e3 * other.e03 * other.e03 - self.e3 * other.e31 * other.e31 - self.e3 * other.e23 * other.e23 - self.e3 * other.e0123 * other.e0123,
            e013: 2.0 * self.e021 * other.s * other.e23 - 2.0 * self.e021 * other.e01 * other.e0123 + 2.0 * self.e021 * other.e02 * other.e03 + 2.0 * self.e021 * other.e12 * other.e31 + self.e013 * other.s * other.s - self.e013 * other.e01 * other.e01 + self.e013 * other.e02 * other.e02 - self.e013 * other.e12 * other.e12 - self.e013 * other.e03 * other.e03 + self.e013 * other.e31 * other.e31 - self.e013 * other.e23 * other.e23 + self.e013 * other.e0123 * other.e0123 - 2.0 * self.e032 * other.s * other.e12 + 2.0 * self.e032 * other.e01 * other.e02 + 2.0 * self.e032 * other.e03 * other.e0123 + 2.0 * self.e032 * other.e31 * other.e23 - 2.0 * self.e123 * other.s * other.e02 + 2.0 * self.e123 * other.e01 * other.e12 - 2.0 * self.e123 * other.e03 * other.e23 - 2.0 * self.e123 * other.e31 * other.e0123,
            e032: -2.0 * self.e021 * other.s * other.e31 + 2.0 * self.e021 * other.e01 * other.e03 + 2.0 * self.e021 * other.e02 * other.e0123 + 2.0 * self.e021 * other.e12 * other.e23 + 2.0 * self.e013 * other.s * other.e12 + 2.0 * self.e013 * other.e01 * other.e02 - 2.0 * self.e013 * other.e03 * other.e0123 + 2.0 * self.e013 * other.e31 * other.e23 + self.e032 * other.s * other.s + self.e032 * other.e01 * other.e01 - self.e032 * other.e02 * other.e02 - self.e032 * other.e12 * other.e12 - self.e032 * other.e03 * other.e03 - self.e032 * other.e31 * other.e31 + self.e032 * other.e23 * other.e23 + self.e032 * other.e0123 * other.e0123 - 2.0 * self.e123 * other.s * other.e01 - 2.0 * self.e123 * other.e02 * other.e12 + 2.0 * self.e123 * other.e03 * other.e31 - 2.0 * self.e123 * other.e23 * other.e0123,
            e123: 2.0 * self.e021 * other.s * other.e03 - 2.0 * self.e021 * other.e01 * other.e31 + 2.0 * self.e021 * other.e02 * other.e23 + 2.0 * self.e021 * other.e12 * other.e0123 + 2.0 * self.e013 * other.s * other.e02 + 2.0 * self.e013 * other.e01 * other.e12 - 2.0 * self.e013 * other.e03 * other.e23 + 2.0 * self.e013 * other.e31 * other.e0123 + 2.0 * self.e032 * other.s * other.e01 - 2.0 * self.e032 * other.e02 * other.e12 + 2.0 * self.e032 * other.e03 * other.e31 + 2.0 * self.e032 * other.e23 * other.e0123 + self.e123 * other.s * other.s - self.e123 * other.e01 * other.e01 - self.e123 * other.e02 * other.e02 + self.e123 * other.e12 * other.e12 - self.e123 * other.e03 * other.e03 + self.e123 * other.e31 * other.e31 + self.e123 * other.e23 * other.e23 - self.e123 * other.e0123 * other.e0123,
        }
    }
}

impl Transform<Multivector> for OddMultivector {
    type Output = Multivector;
    fn transform(self, other: Multivector) -> Multivector {
        Multivector {
            s: 2.0 * self.e0 * other.s * other.e0 - 2.0 * self.e0 * other.e1 * other.e01 - 2.0 * self.e0 * other.e2 * other.e02 - 2.0 * self.e0 * other.e12 * other.e021 - 2.0 * self.e0 * other.e3 * other.e03 - 2.0 * self.e0 * other.e31 * other.e013 - 2.0 * self.e0 * other.e23 * other.e032 - 2.0 * self.e0 * other.e123 * other.e0123 + 2.0 * self.e1 * other.s * other.e1 + 2.0 * self.e1 * other.e0 * other.e01 - 2.0 * self.e1 * other.e2 * other.e12 + 2.0 * self.e1 * other.e02 * other.e021 + 2.0 * self.e1 * other.e3 * other.e31 - 2.0 * self.e1 * other.e03 * other.e013 + 2.0 * self.e1 * other.e23 * other.e123 - 2.0 * self.e1 * other.e032 * other.e0123 + 2.0 * self.e2 * other.s * other.e2 + 2.0 * self.e2 * other.e0 * other.e02 + 2.0 * self.e2 * other.e1 * other.e12 - 2.0 * self.e2 * other.e01 * other.e021 - 2.0 * self.e2 * other.e3 * other.e23 + 2.0 * self.e2 * other.e03 * other.e032 + 2.0 * self.e2 * other.e31 * other.e123 - 2.0 * self.e2 * other.e013 * other.e0123 + 2.0 * self.e3 * other.s * other.e3 + 2.0 * self.e3 * other.e0 * other.e03 - 2.0 * self.e3 * other.e1 * other.e31 + 2.0 * self.e3 * other.e01 * other.e013 + 2.0 * self.e3 * other.e2 * other.e23 - 2.0 * self.e3 * other.e02 * other.e032 + 2.0 * self.e3 * other.e12 * other.e123 - 2.0 * self.e3 * other.e021 * other.e0123,
            e0: self.e0 * other.s * other.s - self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 + self.e0 * other.e01 * other.e01 - self.e0 * other.e2 * other.e2 + self.e0 * other.e02 * other.e02 + self.e0 * other.e12 * other.e12 - self.e0 * other.e021 * other.e021 - self.e0 * other.e3 * other.e3 + self.e0 * other.e03 * other.e03 + self.e0 * other.e31 * other.e31 - self.e0 * other.e013 * other.e013 + self.e0 * other.e23 * other.e23 - self.e0 * other.e032 * other.e032 - self.e0 * other.e123 * other.e123 + self.e0 * other.e0123 * other.e0123 + 2.0 * self.e1 * other.s * other.e01 + 2.0 * self.e1 * other.e0 * other.e1 + 2.0 * self.e1 * other.e2 * other.e021 - 2.0 * self.e1 * other.e02 * other.e12 - 2.0 * self.e1 * other.e3 * other.e013 + 2.0 * self.e1 * other.e03 * other.e31 + 2.0 * self.e1 * other.e23 * other.e0123 - 2.0 * self.e1 * other.e032 * other.e123 + 2.0 * self.e2 * other.s * other.e02 + 2.0 * self.e2 * other.e0 * other.e2 - 2.0 * self.e2 * other.e1 * other.e021 + 2.0 * self.e2 * other.e01 * other.e12 + 2.0 * self.e2 * other.e3 * other.e032 - 2.0 * self.e2 * other.e03 * other.e23 + 2.0 * self.e2 * other.e31 * other.e0123 - 2.0 * self.e2 * other.e013 * other.e123 + 2.0 * self.e3 * other.s * other.e03 + 2.0 * self.e3 * other.e0 * other.e3 + 2.0 * self.e3 * other.e1 * other.e013 - 2.0 * self.e3 * other.e01 * other.e31 - 2.0 * self.e3 * other.e2 * other.e032 + 2.0 * self.e3 * other.e02 * other.e23 + 2.0 * self.e3 * other.e12 * other.e0123 - 2.0 * self.e3 * other.e021 * other.e123,
            e1: -2.0 * self.e0 * other.s * other.e01 + 2.0 * self.e0 * other.e0 * other.e1 - 2.0 * self.e0 * other.e2 * other.e021 - 2.0 * self.e0 * other.e02 * other.e12 + 2.0 * self.e0 * other.e3 * other.e013 + 2.0 * self.e0 * other.e03 * other.e31 - 2.0 * self.e0 * other.e23 * other.e0123 - 2.0 * self.e0 * other.e032 * other.e123 + self.e1 * other.s * other.s - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1 - self.e1 * other.e01 * other.e01 - self.e1 * other.e2 * other.e2 + self.e1 * other.e02 * other.e02 - self.e1 * other.e12 * other.e12 + self.e1 * other.e021 * other.e021 - self.e1 * other.e3 * other.e3 + self.e1 * other.e03 * other.e03 - self.e1 * other.e31 * other.e31 + self.e1 * other.e013 * other.e013 + self.e1 * other.e23 * other.e23 - self.e1 * other.e032 * other.e032 + self.e1 * other.e123 * other.e123 - self.e1 * other.e0123 * other.e0123 + 2.0 * self.e2 * other.s * other.e12 + 2.0 * self.e2 * other.e0 * other.e021 + 2.0 * self.e2 * other.e1 * other.e2 - 2.0 * self.e2 * other.e01 * other.e02 - 2.0 * self.e2 * other.e3 * other.e123 + 2.0 * self.e2 * other.e03 * other.e0123 + 2.0 * self.e2 * other.e31 * other.e23 - 2.0 * self.e2 * other.e013 * other.e032 - 2.0 * self.e3 * other.s * other.e31 - 2.0 * self.e3 * other.e0 * other.e013 + 2.0 * self.e3 * other.e1 * other.e3 - 2.0 * self.e3 * other.e01 * other.e03 + 2.0 * self.e3 * other.e2 * other.e123 - 2.0 * self.e3 * other.e02 * other.e0123 + 2.0 * self.e3 * other.e12 * other.e23 - 2.0 * self.e3 * other.e021 * other.e032,
            e01: -2.0 * self.e021 * other.s * other.e2 - 2.0 * self.e021 * other.e0 * other.e02 + 2.0 * self.e021 * other.e1 * other.e12 - 2.0 * self.e021 * other.e01 * other.e021 + 2.0 * self.e021 * other.e3 * other.e23 - 2.0 * self.e021 * other.e03 * other.e032 + 2.0 * self.e021 * other.e31 * other.e123 - 2.0 * self.e021 * other.e013 * other.e0123 + 2.0 * self.e013 * other.s * other.e3 + 2.0 * self.e013 * other.e0 * other.e03 + 2.0 * self.e013 * other.e1 * other.e31 - 2.0 * self.e013 * other.e01 * other.e013 + 2.0 * self.e013 * other.e2 * other.e23 - 2.0 * self.e013 * other.e02 * other.e032 - 2.0 * self.e013 * other.e12 * other.e123 + 2.0 * self.e013 * other.e021 * other.e0123 - 2.0 * self.e032 * other.s * other.e123 - 2.0 * self.e032 * other.e0 * other.e0123 + 2.0 * self.e032 * other.e1 * other.e23 - 2.0 * self.e032 * other.e01 * other.e032 - 2.0 * self.e032 * other.e2 * other.e31 + 2.0 * self.e032 * other.e02 * other.e013 - 2.0 * self.e032 * other.e12 * other.e3 + 2.0 * self.e032 * other.e021 * other.e03 + 2.0 * self.e123 * other.s * other.e032 + 2.0 * self.e123 * other.e0 * other.e23 - 2.0 * self.e123 * other.e1 * other.e0123 + 2.0 * self.e123 * other.e01 * other.e123 + 2.0 * self.e123 * other.e2 * other.e03 - 2.0 * self.e123 * other.e02 * other.e3 + 2.0 * self.e123 * other.e12 * other.e013 - 2.0 * self.e123 * other.e021 * other.e31,
            e2: -2.0 * self.e0 * other.s * other.e02 + 2.0 * self.e0 * other.e0 * other.e2 + 2.0 * self.e0 * other.e1 * other.e021 + 2.0 * self.e0 * other.e01 * other.e12 - 2.0 * self.e0 * other.e3 * other.e032 - 2.0 * self.e0 * other.e03 * other.e23 - 2.0 * self.e0 * other.e31 * other.e0123 - 2.0 * self.e0 * other.e013 * other.e123 - 2.0 * self.e1 * other.s * other.e12 - 2.0 * self.e1 * other.e0 * other.e021 + 2.0 * self.e1 * other.e1 * other.e2 - 2.0 * self.e1 * other.e01 * other.e02 + 2.0 * self.e1 * other.e3 * other.e123 - 2.0 * self.e1 * other.e03 * other.e0123 + 2.0 * self.e1 * other.e31 * other.e23 - 2.0 * self.e1 * other.e013 * other.e032 + self.e2 * other.s * other.s - self.e2 * other.e0 * other.e0 - self.e2 * other.e1 * other.e1 + self.e2 * other.e01 * other.e01 + self.e2 * other.e2 * other.e2 - self.e2 * other.e02 * other.e02 - self.e2 * other.e12 * other.e12 + self.e2 * other.e021 * other.e021 - self.e2 * other.e3 * other.e3 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e2 * other.e013 * other.e013 - self.e2 * other.e23 * other.e23 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 - self.e2 * other.e0123 * other.e0123 + 2.0 * self.e3 * other.s * other.e23 + 2.0 * self.e3 * other.e0 * other.e032 - 2.0 * self.e3 * other.e1 * other.e123 + 2.0 * self.e3 * other.e01 * other.e0123 + 2.0 * self.e3 * other.e2 * other.e3 - 2.0 * self.e3 * other.e02 * other.e03 + 2.0 * self.e3 * other.e12 * other.e31 - 2.0 * self.e3 * other.e021 * other.e013,
            e02: 2.0 * self.e021 * other.s * other.e1 + 2.0 * self.e021 * other.e0 * other.e01 + 2.0 * self.e021 * other.e2 * other.e12 - 2.0 * self.e021 * other.e02 * other.e021 + 2.0 * self.e021 * other.e3 * other.e31 - 2.0 * self.e021 * other.e03 * other.e013 - 2.0 * self.e021 * other.e23 * other.e123 + 2.0 * self.e021 * other.e032 * other.e0123 - 2.0 * self.e013 * other.s * other.e123 - 2.0 * self.e013 * other.e0 * other.e0123 - 2.0 * self.e013 * other.e1 * other.e23 + 2.0 * self.e013 * other.e01 * other.e032 + 2.0 * self.e013 * other.e2 * other.e31 - 2.0 * self.e013 * other.e02 * other.e013 - 2.0 * self.e013 * other.e12 * other.e3 + 2.0 * self.e013 * other.e021 * other.e03 - 2.0 * self.e032 * other.s * other.e3 - 2.0 * self.e032 * other.e0 * other.e03 + 2.0 * self.e032 * other.e1 * other.e31 - 2.0 * self.e032 * other.e01 * other.e013 + 2.0 * self.e032 * other.e2 * other.e23 - 2.0 * self.e032 * other.e02 * other.e032 + 2.0 * self.e032 * other.e12 * other.e123 - 2.0 * self.e032 * other.e021 * other.e0123 + 2.0 * self.e123 * other.s * other.e013 + 2.0 * self.e123 * other.e0 * other.e31 - 2.0 * self.e123 * other.e1 * other.e03 + 2.0 * self.e123 * other.e01 * other.e3 - 2.0 * self.e123 * other.e2 * other.e0123 + 2.0 * self.e123 * other.e02 * other.e123 - 2.0 * self.e123 * other.e12 * other.e032 + 2.0 * self.e123 * other.e021 * other.e23,
            e12: -2.0 * self.e021 * other.s * other.e0 - 2.0 * self.e021 * other.e1 * other.e01 - 2.0 * self.e021 * other.e2 * other.e02 + 2.0 * self.e021 * other.e12 * other.e021 + 2.0 * self.e021 * other.e3 * other.e03 - 2.0 * self.e021 * other.e31 * other.e013 - 2.0 * self.e021 * other.e23 * other.e032 + 2.0 * self.e021 * other.e123 * other.e0123 - 2.0 * self.e013 * other.s * other.e032 + 2.0 * self.e013 * other.e0 * other.e23 + 2.0 * self.e013 * other.e1 * other.e0123 + 2.0 * self.e013 * other.e01 * other.e123 + 2.0 * self.e013 * other.e2 * other.e03 + 2.0 * self.e013 * other.e02 * other.e3 + 2.0 * self.e013 * other.e12 * other.e013 + 2.0 * self.e013 * other.e021 * other.e31 + 2.0 * self.e032 * other.s * other.e013 - 2.0 * self.e032 * other.e0 * other.e31 + 2.0 * self.e032 * other.e1 * other.e03 + 2.0 * self.e032 * other.e01 * other.e3 - 2.0 * self.e032 * other.e2 * other.e0123 - 2.0 * self.e032 * other.e02 * other.e123 + 2.0 * self.e032 * other.e12 * other.e032 + 2.0 * self.e032 * other.e021 * other.e23 + 2.0 * self.e123 * other.s * other.e3 + 2.0 * self.e123 * other.e0 * other.e03 + 2.0 * self.e123 * other.e1 * other.e31 - 2.0 * self.e123 * other.e01 * other.e013 - 2.0 * self.e123 * other.e2 * other.e23 + 2.0 * self.e123 * other.e02 * other.e032 + 2.0 * self.e123 * other.e12 * other.e123 - 2.0 * self.e123 * other.e021 * other.e0123,
            e021: self.e021 * other.s * other.s - self.e021 * other.e0 * other.e0 + self.e021 * other.e1 * other.e1 - self.e021 * other.e01 * other.e01 + self.e021 * other.e2 * other.e2 - self.e021 * other.e02 * other.e02 + self.e021 * other.e12 * other.e12 - self.e021 * other.e021 * other.e021 - self.e021 * other.e3 * other.e3 + self.e021 * other.e03 * other.e03 - self.e021 * other.e31 * other.e31 + self.e021 * other.e013 * other.e013 - self.e021 * other.e23 * other.e23 + self.e021 * other.e032 * other.e032 - self.e021 * other.e123 * other.e123 + self.e021 * other.e0123 * other.e0123 - 2.0 * self.e013 * other.s * other.e23 - 2.0 * self.e013 * other.e0 * other.e032 - 2.0 * self.e013 * other.e1 * other.e123 + 2.0 * self.e013 * other.e01 * other.e0123 - 2.0 * self.e013 * other.e2 * other.e3 + 2.0 * self.e013 * other.e02 * other.e03 + 2.0 * self.e013 * other.e12 * other.e31 - 2.0 * self.e013 * other.e021 * other.e013 + 2.0 * self.e032 * other.s * other.e31 + 2.0 * self.e032 * other.e0 * other.e013 - 2.0 * self.e032 * other.e1 * other.e3 + 2.0 * self.e032 * other.e01 * other.e03 + 2.0 * self.e032 * other.e2 * other.e123 - 2.0 * self.e032 * other.e02 * other.e0123 + 2.0 * self.e032 * other.e12 * other.e23 - 2.0 * self.e032 * other.e021 * other.e032 - 2.0 * self.e123 * other.s * other.e03 - 2.0 * self.e123 * other.e0 * other.e3 + 2.0 * self.e123 * other.e1 * other.e013 - 2.0 * self.e123 * other.e01 * other.e31 - 2.0 * self.e123 * other.e2 * other.e032 + 2.0 * self.e123 * other.e02 * other.e23 - 2.0 * self.e123 * other.e12 * other.e0123 + 2.0 * self.e123 * other.e021 * other.e123,
            e3: -2.0 * self.e0 * other.s * other.e03 + 2.0 * self.e0 * other.e0 * other.e3 - 2.0 * self.e0 * other.e1 * other.e013 - 2.0 * self.e0 * other.e01 * other.e31 + 2.0 * self.e0 * other.e2 * other.e032 + 2.0 * self.e0 * other.e02 * other.e23 - 2.0 * self.e0 * other.e12 * other.e0123 - 2.0 * self.e0 * other.e021 * other.e123 + 2.0 * self.e1 * other.s * other.e31 + 2.0 * self.e1 * other.e0 * other.e013 + 2.0 * self.e1 * other.e1 * other.e3 - 2.0 * self.e1 * other.e01 * other.e03 - 2.0 * self.e1 * other.e2 * other.e123 + 2.0 * self.e1 * other.e02 * other.e0123 + 2.0 * self.e1 * other.e12 * other.e23 - 2.0 * self.e1 * other.e021 * other.e032 - 2.0 * self.e2 * other.s * other.e23 - 2.0 * self.e2 * other.e0 * other.e032 + 2.0 * self.e2 * other.e1 * other.e123 - 2.0 * self.e2 * other.e01 * other.e0123 + 2.0 * self.e2 * other.e2 * other.e3 - 2.0 * self.e2 * other.e02 * other.e03 + 2.0 * self.e2 * other.e12 * other.e31 - 2.0 * self.e2 * other.e021 * other.e013 + self.e3 * other.s * other.s - self.e3 * other.e0 * other.e0 - self.e3 * other.e1 * other.e1 + self.e3 * other.e01 * other.e01 - self.e3 * other.e2 * other.e2 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12 - self.e3 * other.e021 * other.e021 + self.e3 * other.e3 * other.e3 - self.e3 * other.e03 * other.e03 - self.e3 * other.e31 * other.e31 + self.e3 * other.e013 * other.e013 - self.e3 * other.e23 * other.e23 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123 - self.e3 * other.e0123 * other.e0123,
            e03: -2.0 * self.e021 * other.s * other.e123 - 2.0 * self.e021 * other.e0 * other.e0123 - 2.0 * self.e021 * other.e1 * other.e23 + 2.0 * self.e021 * other.e01 * other.e032 - 2.0 * self.e021 * other.e2 * other.e31 + 2.0 * self.e021 * other.e02 * other.e013 + 2.0 * self.e021 * other.e12 * other.e3 - 2.0 * self.e021 * other.e021 * other.e03 - 2.0 * self.e013 * other.s * other.e1 - 2.0 * self.e013 * other.e0 * other.e01 + 2.0 * self.e013 * other.e2 * other.e12 - 2.0 * self.e013 * other.e02 * other.e021 + 2.0 * self.e013 * other.e3 * other.e31 - 2.0 * self.e013 * other.e03 * other.e013 + 2.0 * self.e013 * other.e23 * other.e123 - 2.0 * self.e013 * other.e032 * other.e0123 + 2.0 * self.e032 * other.s * other.e2 + 2.0 * self.e032 * other.e0 * other.e02 + 2.0 * self.e032 * other.e1 * other.e12 - 2.0 * self.e032 * other.e01 * other.e021 + 2.0 * self.e032 * other.e3 * other.e23 - 2.0 * self.e032 * other.e03 * other.e032 - 2.0 * self.e032 * other.e31 * other.e123 + 2.0 * self.e032 * other.e013 * other.e0123 + 2.0 * self.e123 * other.s * other.e021 + 2.0 * self.e123 * other.e0 * other.e12 + 2.0 * self.e123 * other.e1 * other.e02 - 2.0 * self.e123 * other.e01 * other.e2 - 2.0 * self.e123 * other.e3 * other.e0123 + 2.0 * self.e123 * other.e03 * other.e123 + 2.0 * self.e123 * other.e31 * other.e032 - 2.0 * self.e123 * other.e013 * other.e23,
            e31: 2.0 * self.e021 * other.s * other.e032 - 2.0 * self.e021 * other.e0 * other.e23 - 2.0 * self.e021 * other.e1 * other.e0123 - 2.0 * self.e021 * other.e01 * other.e123 + 2.0 * self.e021 * other.e2 * other.e03 + 2.0 * self.e021 * other.e02 * other.e3 + 2.0 * self.e021 * other.e12 * other.e013 + 2.0 * self.e021 * other.e021 * other.e31 - 2.0 * self.e013 * other.s * other.e0 - 2.0 * self.e013 * other.e1 * other.e01 + 2.0 * self.e013 * other.e2 * other.e02 - 2.0 * self.e013 * other.e12 * other.e021 - 2.0 * self.e013 * other.e3 * other.e03 + 2.0 * self.e013 * other.e31 * other.e013 - 2.0 * self.e013 * other.e23 * other.e032 + 2.0 * self.e013 * other.e123 * other.e0123 - 2.0 * self.e032 * other.s * other.e021 + 2.0 * self.e032 * other.e0 * other.e12 + 2.0 * self.e032 * other.e1 * other.e02 + 2.0 * self.e032 * other.e01 * other.e2 + 2.0 * self.e032 * other.e3 * other.e0123 + 2.0 * self.e032 * other.e03 * other.e123 + 2.0 * self.e032 * other.e31 * other.e032 + 2.0 * self.e032 * other.e013 * other.e23 + 2.0 * self.e123 * other.s * other.e2 + 2.0 * self.e123 * other.e0 * other.e02 - 2.0 * self.e123 * other.e1 * other.e12 + 2.0 * self.e123 * other.e01 * other.e021 + 2.0 * self.e123 * other.e3 * other.e23 - 2.0 * self.e123 * other.e03 * other.e032 + 2.0 * self.e123 * other.e31 * other.e123 - 2.0 * self.e123 * other.e013 * other.e0123,
            e013: 2.0 * self.e021 * other.s * other.e23 + 2.0 * self.e021 * other.e0 * other.e032 + 2.0 * self.e021 * other.e1 * other.e123 - 2.0 * self.e021 * other.e01 * other.e0123 - 2.0 * self.e021 * other.e2 * other.e3 + 2.0 * self.e021 * other.e02 * other.e03 + 2.0 * self.e021 * other.e12 * other.e31 - 2.0 * self.e021 * other.e021 * other.e013 + self.e013 * other.s * other.s - self.e013 * other.e0 * other.e0 + self.e013 * other.e1 * other.e1 - self.e013 * other.e01 * other.e01 - self.e013 * other.e2 * other.e2 + self.e013 * other.e02 * other.e02 - self.e013 * other.e12 * other.e12 + self.e013 * other.e021 * other.e021 + self.e013 * other.e3 * other.e3 - self.e013 * other.e03 * other.e03 + self.e013 * other.e31 * other.e31 - self.e013 * other.e013 * other.e013 - self.e013 * other.e23 * other.e23 + self.e013 * other.e032 * other.e032 - self.e013 * other.e123 * other.e123 + self.e013 * other.e0123 * other.e0123 - 2.0 * self.e032 * other.s * other.e12 - 2.0 * self.e032 * other.e0 * other.e021 - 2.0 * self.e032 * other.e1 * other.e2 + 2.0 * self.e032 * other.e01 * other.e02 - 2.0 * self.e032 * other.e3 * other.e123 + 2.0 * self.e032 * other.e03 * other.e0123 + 2.0 * self.e032 * other.e31 * other.e23 - 2.0 * self.e032 * other.e013 * other.e032 - 2.0 * self.e123 * other.s * other.e02 - 2.0 * self.e123 * other.e0 * other.e2 - 2.0 * self.e123 * other.e1 * other.e021 + 2.0 * self.e123 * other.e01 * other.e12 + 2.0 * self.e123 * other.e3 * other.e032 - 2.0 * self.e123 * other.e03 * other.e23 - 2.0 * self.e123 * other.e31 * other.e0123 + 2.0 * self.e123 * other.e013 * other.e123,
            e23: -2.0 * self.e021 * other.s * other.e013 + 2.0 * self.e021 * other.e0 * other.e31 + 2.0 * self.e021 * other.e1 * other.e03 + 2.0 * self.e021 * other.e01 * other.e3 + 2.0 * self.e021 * other.e2 * other.e0123 + 2.0 * self.e021 * other.e02 * other.e123 + 2.0 * self.e021 * other.e12 * other.e032 + 2.0 * self.e021 * other.e021 * other.e23 + 2.0 * self.e013 * other.s * other.e021 - 2.0 * self.e013 * other.e0 * other.e12 + 2.0 * self.e013 * other.e1 * other.e02 + 2.0 * self.e013 * other.e01 * other.e2 - 2.0 * self.e013 * other.e3 * other.e0123 - 2.0 * self.e013 * other.e03 * other.e123 + 2.0 * self.e013 * other.e31 * other.e032 + 2.0 * self.e013 * other.e013 * other.e23 - 2.0 * self.e032 * other.s * other.e0 + 2.0 * self.e032 * other.e1 * other.e01 - 2.0 * self.e032 * other.e2 * other.e02 - 2.0 * self.e032 * other.e12 * other.e021 - 2.0 * self.e032 * other.e3 * other.e03 - 2.0 * self.e032 * other.e31 * other.e013 + 2.0 * self.e032 * other.e23 * other.e032 + 2.0 * self.e032 * other.e123 * other.e0123 + 2.0 * self.e123 * other.s * other.e1 + 2.0 * self.e123 * other.e0 * other.e01 + 2.0 * self.e123 * other.e2 * other.e12 - 2.0 * self.e123 * other.e02 * other.e021 - 2.0 * self.e123 * other.e3 * other.e31 + 2.0 * self.e123 * other.e03 * other.e013 + 2.0 * self.e123 * other.e23 * other.e123 - 2.0 * self.e123 * other.e032 * other.e0123,
            e032: -2.0 * self.e021 * other.s * other.e31 - 2.0 * self.e021 * other.e0 * other.e013 - 2.0 * self.e021 * other.e1 * other.e3 + 2.0 * self.e021 * other.e01 * other.e03 - 2.0 * self.e021 * other.e2 * other.e123 + 2.0 * self.e021 * other.e02 * other.e0123 + 2.0 * self.e021 * other.e12 * other.e23 - 2.0 * self.e021 * other.e021 * other.e032 + 2.0 * self.e013 * other.s * other.e12 + 2.0 * self.e013 * other.e0 * other.e021 - 2.0 * self.e013 * other.e1 * other.e2 + 2.0 * self.e013 * other.e01 * other.e02 + 2.0 * self.e013 * other.e3 * other.e123 - 2.0 * self.e013 * other.e03 * other.e0123 + 2.0 * self.e013 * other.e31 * other.e23 - 2.0 * self.e013 * other.e013 * other.e032 + self.e032 * other.s * other.s - self.e032 * other.e0 * other.e0 - self.e032 * other.e1 * other.e1 + self.e032 * other.e01 * other.e01 + self.e032 * other.e2 * other.e2 - self.e032 * other.e02 * other.e02 - self.e032 * other.e12 * other.e12 + self.e032 * other.e021 * other.e021 + self.e032 * other.e3 * other.e3 - self.e032 * other.e03 * other.e03 - self.e032 * other.e31 * other.e31 + self.e032 * other.e013 * other.e013 + self.e032 * other.e23 * other.e23 - self.e032 * other.e032 * other.e032 - self.e032 * other.e123 * other.e123 + self.e032 * other.e0123 * other.e0123 - 2.0 * self.e123 * other.s * other.e01 - 2.0 * self.e123 * other.e0 * other.e1 + 2.0 * self.e123 * other.e2 * other.e021 - 2.0 * self.e123 * other.e02 * other.e12 - 2.0 * self.e123 * other.e3 * other.e013 + 2.0 * self.e123 * other.e03 * other.e31 - 2.0 * self.e123 * other.e23 * other.e0123 + 2.0 * self.e123 * other.e032 * other.e123,
            e123: 2.0 * self.e021 * other.s * other.e03 - 2.0 * self.e021 * other.e0 * other.e3 - 2.0 * self.e021 * other.e1 * other.e013 - 2.0 * self.e021 * other.e01 * other.e31 + 2.0 * self.e021 * other.e2 * other.e032 + 2.0 * self.e021 * other.e02 * other.e23 + 2.0 * self.e021 * other.e12 * other.e0123 + 2.0 * self.e021 * other.e021 * other.e123 + 2.0 * self.e013 * other.s * other.e02 - 2.0 * self.e013 * other.e0 * other.e2 + 2.0 * self.e013 * other.e1 * other.e021 + 2.0 * self.e013 * other.e01 * other.e12 - 2.0 * self.e013 * other.e3 * other.e032 - 2.0 * self.e013 * other.e03 * other.e23 + 2.0 * self.e013 * other.e31 * other.e0123 + 2.0 * self.e013 * other.e013 * other.e123 + 2.0 * self.e032 * other.s * other.e01 - 2.0 * self.e032 * other.e0 * other.e1 - 2.0 * self.e032 * other.e2 * other.e021 - 2.0 * self.e032 * other.e02 * other.e12 + 2.0 * self.e032 * other.e3 * other.e013 + 2.0 * self.e032 * other.e03 * other.e31 + 2.0 * self.e032 * other.e23 * other.e0123 + 2.0 * self.e032 * other.e032 * other.e123 + self.e123 * other.s * other.s - self.e123 * other.e0 * other.e0 + self.e123 * other.e1 * other.e1 - self.e123 * other.e01 * other.e01 + self.e123 * other.e2 * other.e2 - self.e123 * other.e02 * other.e02 + self.e123 * other.e12 * other.e12 - self.e123 * other.e021 * other.e021 + self.e123 * other.e3 * other.e3 - self.e123 * other.e03 * other.e03 + self.e123 * other.e31 * other.e31 - self.e123 * other.e013 * other.e013 + self.e123 * other.e23 * other.e23 - self.e123 * other.e032 * other.e032 + self.e123 * other.e123 * other.e123 - self.e123 * other.e0123 * other.e0123,
            e0123: -2.0 * self.e0 * other.s * other.e123 - 2.0 * self.e0 * other.e0 * other.e0123 + 2.0 * self.e0 * other.e1 * other.e23 - 2.0 * self.e0 * other.e01 * other.e032 + 2.0 * self.e0 * other.e2 * other.e31 - 2.0 * self.e0 * other.e02 * other.e013 + 2.0 * self.e0 * other.e12 * other.e3 - 2.0 * self.e0 * other.e021 * other.e03 - 2.0 * self.e1 * other.s * other.e032 - 2.0 * self.e1 * other.e0 * other.e23 + 2.0 * self.e1 * other.e1 * other.e0123 - 2.0 * self.e1 * other.e01 * other.e123 + 2.0 * self.e1 * other.e2 * other.e03 - 2.0 * self.e1 * other.e02 * other.e3 + 2.0 * self.e1 * other.e12 * other.e013 - 2.0 * self.e1 * other.e021 * other.e31 - 2.0 * self.e2 * other.s * other.e013 - 2.0 * self.e2 * other.e0 * other.e31 - 2.0 * self.e2 * other.e1 * other.e03 + 2.0 * self.e2 * other.e01 * other.e3 + 2.0 * self.e2 * other.e2 * other.e0123 - 2.0 * self.e2 * other.e02 * other.e123 - 2.0 * self.e2 * other.e12 * other.e032 + 2.0 * self.e2 * other.e021 * other.e23 - 2.0 * self.e3 * other.s * other.e021 - 2.0 * self.e3 * other.e0 * other.e12 + 2.0 * self.e3 * other.e1 * other.e02 - 2.0 * self.e3 * other.e01 * other.e2 + 2.0 * self.e3 * other.e3 * other.e0123 - 2.0 * self.e3 * other.e03 * other.e123 + 2.0 * self.e3 * other.e31 * other.e032 - 2.0 * self.e3 * other.e013 * other.e23,
        }
    }
}

impl Transform<Scalar> for EvenMultivector {
    type Output = EvenMultivector;
    fn transform(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s * other.s,
            e01: self.e01 * other.s * other.s,
            e02: self.e02 * other.s * other.s,
            e12: self.e12 * other.s * other.s,
            e03: self.e03 * other.s * other.s,
            e31: self.e31 * other.s * other.s,
            e23: self.e23 * other.s * other.s,
            e0123: self.e0123 * other.s * other.s,
        }
    }
}

impl Transform<Vector> for EvenMultivector {
    type Output = EvenMultivector;
    fn transform(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e3 * other.e3,
            e01: self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1 + self.e01 * other.e2 * other.e2 + self.e01 * other.e3 * other.e3 - 2.0 * self.e02 * other.e1 * other.e2 + 2.0 * self.e12 * other.e0 * other.e2 - 2.0 * self.e03 * other.e1 * other.e3 - 2.0 * self.e31 * other.e0 * other.e3,
            e02: -2.0 * self.e01 * other.e1 * other.e2 + self.e02 * other.e0 * other.e0 + self.e02 * other.e1 * other.e1 - self.e02 * other.e2 * other.e2 + self.e02 * other.e3 * other.e3 - 2.0 * self.e12 * other.e0 * other.e1 - 2.0 * self.e03 * other.e2 * other.e3 + 2.0 * self.e23 * other.e0 * other.e3,
            e12: 2.0 * self.e01 * other.e0 * other.e2 - 2.0 * self.e02 * other.e0 * other.e1 + self.e12 * other.e0 * other.e0 - self.e12 * other.e1 * other.e1 - self.e12 * other.e2 * other.e2 + self.e12 * other.e3 * other.e3 + 2.0 * self.e31 * other.e2 * other.e3 + 2.0 * self.e23 * other.e1 * other.e3,
            e03: -2.0 * self.e01 * other.e1 * other.e3 - 2.0 * self.e02 * other.e2 * other.e3 + self.e03 * other.e0 * other.e0 + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 - self.e03 * other.e3 * other.e3 + 2.0 * self.e31 * other.e0 * other.e1 - 2.0 * self.e23 * other.e0 * other.e2,
            e31: -2.0 * self.e01 * other.e0 * other.e3 + 2.0 * self.e12 * other.e2 * other.e3 + 2.0 * self.e03 * other.e0 * other.e1 + self.e31 * other.e0 * other.e0 - self.e31 * other.e1 * other.e1 + self.e31 * other.e2 * other.e2 - self.e31 * other.e3 * other.e3 + 2.0 * self.e23 * other.e1 * other.e2,
            e23: 2.0 * self.e02 * other.e0 * other.e3 + 2.0 * self.e12 * other.e1 * other.e3 - 2.0 * self.e03 * other.e0 * other.e2 + 2.0 * self.e31 * other.e1 * other.e2 + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1 - self.e23 * other.e2 * other.e2 - self.e23 * other.e3 * other.e3,
            e0123: self.e0123 * other.e0 * other.e0 - self.e0123 * other.e1 * other.e1 - self.e0123 * other.e2 * other.e2 - self.e0123 * other.e3 * other.e3,
        }
    }
}

impl Transform<Bivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn transform(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23 - 2.0 * self.e0123 * other.e01 * other.e23 - 2.0 * self.e0123 * other.e02 * other.e31 - 2.0 * self.e0123 * other.e12 * other.e03,
            e01: -self.e01 * other.e01 * other.e01 + self.e01 * other.e02 * other.e02 - self.e01 * other.e12 * other.e12 + self.e01 * other.e03 * other.e03 - self.e01 * other.e31 * other.e31 + self.e01 * other.e23 * other.e23 - 2.0 * self.e02 * other.e01 * other.e02 + 2.0 * self.e02 * other.e31 * other.e23 + 2.0 * self.e12 * other.e01 * other.e12 + 2.0 * self.e12 * other.e03 * other.e23 - 2.0 * self.e03 * other.e01 * other.e03 + 2.0 * self.e03 * other.e12 * other.e23 + 2.0 * self.e31 * other.e01 * other.e31 + 2.0 * self.e31 * other.e02 * other.e23 + 2.0 * self.e23 * other.e01 * other.e23 - 2.0 * self.e23 * other.e02 * other.e31 - 2.0 * self.e23 * other.e12 * other.e03,
            e02: -2.0 * self.e01 * other.e01 * other.e02 + 2.0 * self.e01 * other.e31 * other.e23 + self.e02 * other.e01 * other.e01 - self.e02 * other.e02 * other.e02 - self.e02 * other.e12 * other.e12 + self.e02 * other.e03 * other.e03 + self.e02 * other.e31 * other.e31 - self.e02 * other.e23 * other.e23 + 2.0 * self.e12 * other.e02 * other.e12 + 2.0 * self.e12 * other.e03 * other.e31 - 2.0 * self.e03 * other.e02 * other.e03 + 2.0 * self.e03 * other.e12 * other.e31 - 2.0 * self.e31 * other.e01 * other.e23 + 2.0 * self.e31 * other.e02 * other.e31 - 2.0 * self.e31 * other.e12 * other.e03 + 2.0 * self.e23 * other.e01 * other.e31 + 2.0 * self.e23 * other.e02 * other.e23,
            e12: 2.0 * self.e01 * other.e01 * other.e12 + 2.0 * self.e01 * other.e03 * other.e23 + 2.0 * self.e02 * other.e02 * other.e12 + 2.0 * self.e02 * other.e03 * other.e31 - self.e12 * other.e01 * other.e01 - self.e12 * other.e02 * other.e02 + self.e12 * other.e12 * other.e12 + self.e12 * other.e03 * other.e03 - self.e12 * other.e31 * other.e31 - self.e12 * other.e23 * other.e23 - 2.0 * self.e03 * other.e01 * other.e23 - 2.0 * self.e03 * other.e02 * other.e31 + 2.0 * self.e03 * other.e12 * other.e03 + 2.0 * self.e31 * other.e02 * other.e03 + 2.0 * self.e31 * other.e12 * other.e31 + 2.0 * self.e23 * other.e01 * other.e03 + 2.0 * self.e23 * other.e12 * other.e23,
            e03: -2.0 * self.e01 * other.e01 * other.e03 + 2.0 * self.e01 * other.e12 * other.e23 - 2.0 * self.e02 * other.e02 * other.e03 + 2.0 * self.e02 * other.e12 * other.e31 - 2.0 * self.e12 * other.e01 * other.e23 - 2.0 * self.e12 * other.e02 * other.e31 + 2.0 * self.e12 * other.e12 * other.e03 + self.e03 * other.e01 * other.e01 + self.e03 * other.e02 * other.e02 + self.e03 * other.e12 * other.e12 - self.e03 * other.e03 * other.e03 - self.e03 * other.e31 * other.e31 - self.e03 * other.e23 * other.e23 + 2.0 * self.e31 * other.e02 * other.e12 + 2.0 * self.e31 * other.e03 * other.e31 + 2.0 * self.e23 * other.e01 * other.e12 + 2.0 * self.e23 * other.e03 * other.e23,
            e31: 2.0 * self.e01 * other.e01 * other.e31 + 2.0 * self.e01 * other.e02 * other.e23 - 2.0 * self.e02 * other.e01 * other.e23 + 2.0 * self.e02 * other.e02 * other.e31 - 2.0 * self.e02 * other.e12 * other.e03 + 2.0 * self.e12 * other.e02 * other.e03 + 2.0 * self.e12 * other.e12 * other.e31 + 2.0 * self.e03 * other.e02 * other.e12 + 2.0 * self.e03 * other.e03 * other.e31 - self.e31 * other.e01 * other.e01 + self.e31 * other.e02 * other.e02 - self.e31 * other.e12 * other.e12 - self.e31 * other.e03 * other.e03 + self.e31 * other.e31 * other.e31 - self.e31 * other.e23 * other.e23 + 2.0 * self.e23 * other.e01 * other.e02 + 2.0 * self.e23 * other.e31 * other.e23,
            e23: 2.0 * self.e01 * other.e01 * other.e23 - 2.0 * self.e01 * other.e02 * other.e31 - 2.0 * self.e01 * other.e12 * other.e03 + 2.0 * self.e02 * other.e01 * other.e31 + 2.0 * self.e02 * other.e02 * other.e23 + 2.0 * self.e12 * other.e01 * other.e03 + 2.0 * self.e12 * other.e12 * other.e23 + 2.0 * self.e03 * other.e01 * other.e12 + 2.0 * self.e03 * other.e03 * other.e23 + 2.0 * self.e31 * other.e01 * other.e02 + 2.0 * self.e31 * other.e31 * other.e23 + self.e23 * other.e01 * other.e01 - self.e23 * other.e02 * other.e02 - self.e23 * other.e12 * other.e12 - self.e23 * other.e03 * other.e03 - self.e23 * other.e31 * other.e31 + self.e23 * other.e23 * other.e23,
            e0123: -2.0 * self.s * other.e01 * other.e23 - 2.0 * self.s * other.e02 * other.e31 - 2.0 * self.s * other.e12 * other.e03 - self.e0123 * other.e01 * other.e01 - self.e0123 * other.e02 * other.e02 + self.e0123 * other.e12 * other.e12 - self.e0123 * other.e03 * other.e03 + self.e0123 * other.e31 * other.e31 + self.e0123 * other.e23 * other.e23,
        }
    }
}

impl Transform<Trivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn transform(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e021 * other.e021 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123,
            e01: -self.e01 * other.e021 * other.e021 - self.e01 * other.e013 * other.e013 + self.e01 * other.e032 * other.e032 - self.e01 * other.e123 * other.e123 + 2.0 * self.e02 * other.e013 * other.e032 + 2.0 * self.e12 * other.e013 * other.e123 + 2.0 * self.e03 * other.e021 * other.e032 - 2.0 * self.e31 * other.e021 * other.e123,
            e02: 2.0 * self.e01 * other.e013 * other.e032 - self.e02 * other.e021 * other.e021 + self.e02 * other.e013 * other.e013 - self.e02 * other.e032 * other.e032 - self.e02 * other.e123 * other.e123 - 2.0 * self.e12 * other.e032 * other.e123 + 2.0 * self.e03 * other.e021 * other.e013 + 2.0 * self.e23 * other.e021 * other.e123,
            e12: 2.0 * self.e01 * other.e013 * other.e123 - 2.0 * self.e02 * other.e032 * other.e123 + self.e12 * other.e021 * other.e021 - self.e12 * other.e013 * other.e013 - self.e12 * other.e032 * other.e032 + self.e12 * other.e123 * other.e123 + 2.0 * self.e31 * other.e021 * other.e013 + 2.0 * self.e23 * other.e021 * other.e032,
            e03: 2.0 * self.e01 * other.e021 * other.e032 + 2.0 * self.e02 * other.e021 * other.e013 + self.e03 * other.e021 * other.e021 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 - self.e03 * other.e123 * other.e123 + 2.0 * self.e31 * other.e032 * other.e123 - 2.0 * self.e23 * other.e013 * other.e123,
            e31: -2.0 * self.e01 * other.e021 * other.e123 + 2.0 * self.e12 * other.e021 * other.e013 + 2.0 * self.e03 * other.e032 * other.e123 - self.e31 * other.e021 * other.e021 + self.e31 * other.e013 * other.e013 - self.e31 * other.e032 * other.e032 + self.e31 * other.e123 * other.e123 + 2.0 * self.e23 * other.e013 * other.e032,
            e23: 2.0 * self.e02 * other.e021 * other.e123 + 2.0 * self.e12 * other.e021 * other.e032 - 2.0 * self.e03 * other.e013 * other.e123 + 2.0 * self.e31 * other.e013 * other.e032 - self.e23 * other.e021 * other.e021 - self.e23 * other.e013 * other.e013 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123,
            e0123: self.e0123 * other.e021 * other.e021 + self.e0123 * other.e013 * other.e013 + self.e0123 * other.e032 * other.e032 - self.e0123 * other.e123 * other.e123,
        }
    }
}

impl Transform<FourVector> for EvenMultivector {
    type Output = EvenMultivector;
    fn transform(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0123 * other.e0123,
            e01: -self.e01 * other.e0123 * other.e0123,
            e02: -self.e02 * other.e0123 * other.e0123,
            e12: self.e12 * other.e0123 * other.e0123,
            e03: -self.e03 * other.e0123 * other.e0123,
            e31: self.e31 * other.e0123 * other.e0123,
            e23: self.e23 * other.e0123 * other.e0123,
            e0123: -self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Transform<Null> for EvenMultivector {
    type Output = Null;
    fn transform(self, other: Null) -> Null {
        Null
    }
}

impl Transform<OddMultivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn transform(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 + 2.0 * self.e0123 * other.e0 * other.e123 + 2.0 * self.e0123 * other.e1 * other.e032 + 2.0 * self.e0123 * other.e2 * other.e013 + 2.0 * self.e0123 * other.e021 * other.e3,
            e01: self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1 + self.e01 * other.e2 * other.e2 - self.e01 * other.e021 * other.e021 + self.e01 * other.e3 * other.e3 - self.e01 * other.e013 * other.e013 + self.e01 * other.e032 * other.e032 - self.e01 * other.e123 * other.e123 - 2.0 * self.e02 * other.e0 * other.e021 - 2.0 * self.e02 * other.e1 * other.e2 + 2.0 * self.e02 * other.e3 * other.e123 + 2.0 * self.e02 * other.e013 * other.e032 + 2.0 * self.e12 * other.e0 * other.e2 + 2.0 * self.e12 * other.e1 * other.e021 + 2.0 * self.e12 * other.e3 * other.e032 + 2.0 * self.e12 * other.e013 * other.e123 + 2.0 * self.e03 * other.e0 * other.e013 - 2.0 * self.e03 * other.e1 * other.e3 - 2.0 * self.e03 * other.e2 * other.e123 + 2.0 * self.e03 * other.e021 * other.e032 - 2.0 * self.e31 * other.e0 * other.e3 + 2.0 * self.e31 * other.e1 * other.e013 + 2.0 * self.e31 * other.e2 * other.e032 - 2.0 * self.e31 * other.e021 * other.e123 + 2.0 * self.e23 * other.e0 * other.e123 + 2.0 * self.e23 * other.e1 * other.e032 - 2.0 * self.e23 * other.e2 * other.e013 - 2.0 * self.e23 * other.e021 * other.e3,
            e02: 2.0 * self.e01 * other.e0 * other.e021 - 2.0 * self.e01 * other.e1 * other.e2 - 2.0 * self.e01 * other.e3 * other.e123 + 2.0 * self.e01 * other.e013 * other.e032 + self.e02 * other.e0 * other.e0 + self.e02 * other.e1 * other.e1 - self.e02 * other.e2 * other.e2 - self.e02 * other.e021 * other.e021 + self.e02 * other.e3 * other.e3 + self.e02 * other.e013 * other.e013 - self.e02 * other.e032 * other.e032 - self.e02 * other.e123 * other.e123 - 2.0 * self.e12 * other.e0 * other.e1 + 2.0 * self.e12 * other.e2 * other.e021 + 2.0 * self.e12 * other.e3 * other.e013 - 2.0 * self.e12 * other.e032 * other.e123 - 2.0 * self.e03 * other.e0 * other.e032 + 2.0 * self.e03 * other.e1 * other.e123 - 2.0 * self.e03 * other.e2 * other.e3 + 2.0 * self.e03 * other.e021 * other.e013 + 2.0 * self.e31 * other.e0 * other.e123 - 2.0 * self.e31 * other.e1 * other.e032 + 2.0 * self.e31 * other.e2 * other.e013 - 2.0 * self.e31 * other.e021 * other.e3 + 2.0 * self.e23 * other.e0 * other.e3 + 2.0 * self.e23 * other.e1 * other.e013 + 2.0 * self.e23 * other.e2 * other.e032 + 2.0 * self.e23 * other.e021 * other.e123,
            e12: 2.0 * self.e01 * other.e0 * other.e2 - 2.0 * self.e01 * other.e1 * other.e021 - 2.0 * self.e01 * other.e3 * other.e032 + 2.0 * self.e01 * other.e013 * other.e123 - 2.0 * self.e02 * other.e0 * other.e1 - 2.0 * self.e02 * other.e2 * other.e021 - 2.0 * self.e02 * other.e3 * other.e013 - 2.0 * self.e02 * other.e032 * other.e123 + self.e12 * other.e0 * other.e0 - self.e12 * other.e1 * other.e1 - self.e12 * other.e2 * other.e2 + self.e12 * other.e021 * other.e021 + self.e12 * other.e3 * other.e3 - self.e12 * other.e013 * other.e013 - self.e12 * other.e032 * other.e032 + self.e12 * other.e123 * other.e123 - 2.0 * self.e03 * other.e0 * other.e123 + 2.0 * self.e03 * other.e1 * other.e032 + 2.0 * self.e03 * other.e2 * other.e013 - 2.0 * self.e03 * other.e021 * other.e3 + 2.0 * self.e31 * other.e0 * other.e032 + 2.0 * self.e31 * other.e1 * other.e123 + 2.0 * self.e31 * other.e2 * other.e3 + 2.0 * self.e31 * other.e021 * other.e013 - 2.0 * self.e23 * other.e0 * other.e013 + 2.0 * self.e23 * other.e1 * other.e3 - 2.0 * self.e23 * other.e2 * other.e123 + 2.0 * self.e23 * other.e021 * other.e032,
            e03: -2.0 * self.e01 * other.e0 * other.e013 - 2.0 * self.e01 * other.e1 * other.e3 + 2.0 * self.e01 * other.e2 * other.e123 + 2.0 * self.e01 * other.e021 * other.e032 + 2.0 * self.e02 * other.e0 * other.e032 - 2.0 * self.e02 * other.e1 * other.e123 - 2.0 * self.e02 * other.e2 * other.e3 + 2.0 * self.e02 * other.e021 * other.e013 + 2.0 * self.e12 * other.e0 * other.e123 - 2.0 * self.e12 * other.e1 * other.e032 - 2.0 * self.e12 * other.e2 * other.e013 + 2.0 * self.e12 * other.e021 * other.e3 + self.e03 * other.e0 * other.e0 + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 + self.e03 * other.e021 * other.e021 - self.e03 * other.e3 * other.e3 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 - self.e03 * other.e123 * other.e123 + 2.0 * self.e31 * other.e0 * other.e1 + 2.0 * self.e31 * other.e2 * other.e021 + 2.0 * self.e31 * other.e3 * other.e013 + 2.0 * self.e31 * other.e032 * other.e123 - 2.0 * self.e23 * other.e0 * other.e2 + 2.0 * self.e23 * other.e1 * other.e021 + 2.0 * self.e23 * other.e3 * other.e032 - 2.0 * self.e23 * other.e013 * other.e123,
            e31: -2.0 * self.e01 * other.e0 * other.e3 - 2.0 * self.e01 * other.e1 * other.e013 - 2.0 * self.e01 * other.e2 * other.e032 - 2.0 * self.e01 * other.e021 * other.e123 - 2.0 * self.e02 * other.e0 * other.e123 + 2.0 * self.e02 * other.e1 * other.e032 - 2.0 * self.e02 * other.e2 * other.e013 + 2.0 * self.e02 * other.e021 * other.e3 - 2.0 * self.e12 * other.e0 * other.e032 - 2.0 * self.e12 * other.e1 * other.e123 + 2.0 * self.e12 * other.e2 * other.e3 + 2.0 * self.e12 * other.e021 * other.e013 + 2.0 * self.e03 * other.e0 * other.e1 - 2.0 * self.e03 * other.e2 * other.e021 - 2.0 * self.e03 * other.e3 * other.e013 + 2.0 * self.e03 * other.e032 * other.e123 + self.e31 * other.e0 * other.e0 - self.e31 * other.e1 * other.e1 + self.e31 * other.e2 * other.e2 - self.e31 * other.e021 * other.e021 - self.e31 * other.e3 * other.e3 + self.e31 * other.e013 * other.e013 - self.e31 * other.e032 * other.e032 + self.e31 * other.e123 * other.e123 + 2.0 * self.e23 * other.e0 * other.e021 + 2.0 * self.e23 * other.e1 * other.e2 + 2.0 * self.e23 * other.e3 * other.e123 + 2.0 * self.e23 * other.e013 * other.e032,
            e23: -2.0 * self.e01 * other.e0 * other.e123 - 2.0 * self.e01 * other.e1 * other.e032 + 2.0 * self.e01 * other.e2 * other.e013 + 2.0 * self.e01 * other.e021 * other.e3 + 2.0 * self.e02 * other.e0 * other.e3 - 2.0 * self.e02 * other.e1 * other.e013 - 2.0 * self.e02 * other.e2 * other.e032 + 2.0 * self.e02 * other.e021 * other.e123 + 2.0 * self.e12 * other.e0 * other.e013 + 2.0 * self.e12 * other.e1 * other.e3 + 2.0 * self.e12 * other.e2 * other.e123 + 2.0 * self.e12 * other.e021 * other.e032 - 2.0 * self.e03 * other.e0 * other.e2 - 2.0 * self.e03 * other.e1 * other.e021 - 2.0 * self.e03 * other.e3 * other.e032 - 2.0 * self.e03 * other.e013 * other.e123 - 2.0 * self.e31 * other.e0 * other.e021 + 2.0 * self.e31 * other.e1 * other.e2 - 2.0 * self.e31 * other.e3 * other.e123 + 2.0 * self.e31 * other.e013 * other.e032 + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1 - self.e23 * other.e2 * other.e2 - self.e23 * other.e021 * other.e021 - self.e23 * other.e3 * other.e3 - self.e23 * other.e013 * other.e013 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123,
            e0123: -2.0 * self.s * other.e0 * other.e123 - 2.0 * self.s * other.e1 * other.e032 - 2.0 * self.s * other.e2 * other.e013 - 2.0 * self.s * other.e021 * other.e3 + self.e0123 * other.e0 * other.e0 - self.e0123 * other.e1 * other.e1 - self.e0123 * other.e2 * other.e2 + self.e0123 * other.e021 * other.e021 - self.e0123 * other.e3 * other.e3 + self.e0123 * other.e013 * other.e013 + self.e0123 * other.e032 * other.e032 - self.e0123 * other.e123 * other.e123,
        }
    }
}

impl Transform<EvenMultivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn transform(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s * other.s + self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23 + self.s * other.e0123 * other.e0123 + 2.0 * self.e0123 * other.s * other.e0123 - 2.0 * self.e0123 * other.e01 * other.e23 - 2.0 * self.e0123 * other.e02 * other.e31 - 2.0 * self.e0123 * other.e12 * other.e03,
            e01: self.e01 * other.s * other.s - self.e01 * other.e01 * other.e01 + self.e01 * other.e02 * other.e02 - self.e01 * other.e12 * other.e12 + self.e01 * other.e03 * other.e03 - self.e01 * other.e31 * other.e31 + self.e01 * other.e23 * other.e23 - self.e01 * other.e0123 * other.e0123 + 2.0 * self.e02 * other.s * other.e12 - 2.0 * self.e02 * other.e01 * other.e02 + 2.0 * self.e02 * other.e03 * other.e0123 + 2.0 * self.e02 * other.e31 * other.e23 - 2.0 * self.e12 * other.s * other.e02 + 2.0 * self.e12 * other.e01 * other.e12 + 2.0 * self.e12 * other.e03 * other.e23 + 2.0 * self.e12 * other.e31 * other.e0123 - 2.0 * self.e03 * other.s * other.e31 - 2.0 * self.e03 * other.e01 * other.e03 - 2.0 * self.e03 * other.e02 * other.e0123 + 2.0 * self.e03 * other.e12 * other.e23 + 2.0 * self.e31 * other.s * other.e03 + 2.0 * self.e31 * other.e01 * other.e31 + 2.0 * self.e31 * other.e02 * other.e23 - 2.0 * self.e31 * other.e12 * other.e0123 - 2.0 * self.e23 * other.s * other.e0123 + 2.0 * self.e23 * other.e01 * other.e23 - 2.0 * self.e23 * other.e02 * other.e31 - 2.0 * self.e23 * other.e12 * other.e03,
            e02: -2.0 * self.e01 * other.s * other.e12 - 2.0 * self.e01 * other.e01 * other.e02 - 2.0 * self.e01 * other.e03 * other.e0123 + 2.0 * self.e01 * other.e31 * other.e23 + self.e02 * other.s * other.s + self.e02 * other.e01 * other.e01 - self.e02 * other.e02 * other.e02 - self.e02 * other.e12 * other.e12 + self.e02 * other.e03 * other.e03 + self.e02 * other.e31 * other.e31 - self.e02 * other.e23 * other.e23 - self.e02 * other.e0123 * other.e0123 + 2.0 * self.e12 * other.s * other.e01 + 2.0 * self.e12 * other.e02 * other.e12 + 2.0 * self.e12 * other.e03 * other.e31 - 2.0 * self.e12 * other.e23 * other.e0123 + 2.0 * self.e03 * other.s * other.e23 + 2.0 * self.e03 * other.e01 * other.e0123 - 2.0 * self.e03 * other.e02 * other.e03 + 2.0 * self.e03 * other.e12 * other.e31 - 2.0 * self.e31 * other.s * other.e0123 - 2.0 * self.e31 * other.e01 * other.e23 + 2.0 * self.e31 * other.e02 * other.e31 - 2.0 * self.e31 * other.e12 * other.e03 - 2.0 * self.e23 * other.s * other.e03 + 2.0 * self.e23 * other.e01 * other.e31 + 2.0 * self.e23 * other.e02 * other.e23 + 2.0 * self.e23 * other.e12 * other.e0123,
            e12: 2.0 * self.e01 * other.s * other.e02 + 2.0 * self.e01 * other.e01 * other.e12 + 2.0 * self.e01 * other.e03 * other.e23 - 2.0 * self.e01 * other.e31 * other.e0123 - 2.0 * self.e02 * other.s * other.e01 + 2.0 * self.e02 * other.e02 * other.e12 + 2.0 * self.e02 * other.e03 * other.e31 + 2.0 * self.e02 * other.e23 * other.e0123 + self.e12 * other.s * other.s - self.e12 * other.e01 * other.e01 - self.e12 * other.e02 * other.e02 + self.e12 * other.e12 * other.e12 + self.e12 * other.e03 * other.e03 - self.e12 * other.e31 * other.e31 - self.e12 * other.e23 * other.e23 + self.e12 * other.e0123 * other.e0123 - 2.0 * self.e03 * other.s * other.e0123 - 2.0 * self.e03 * other.e01 * other.e23 - 2.0 * self.e03 * other.e02 * other.e31 + 2.0 * self.e03 * other.e12 * other.e03 - 2.0 * self.e31 * other.s * other.e23 + 2.0 * self.e31 * other.e01 * other.e0123 + 2.0 * self.e31 * other.e02 * other.e03 + 2.0 * self.e31 * other.e12 * other.e31 + 2.0 * self.e23 * other.s * other.e31 + 2.0 * self.e23 * other.e01 * other.e03 - 2.0 * self.e23 * other.e02 * other.e0123 + 2.0 * self.e23 * other.e12 * other.e23,
            e03: 2.0 * self.e01 * other.s * other.e31 - 2.0 * self.e01 * other.e01 * other.e03 + 2.0 * self.e01 * other.e02 * other.e0123 + 2.0 * self.e01 * other.e12 * other.e23 - 2.0 * self.e02 * other.s * other.e23 - 2.0 * self.e02 * other.e01 * other.e0123 - 2.0 * self.e02 * other.e02 * other.e03 + 2.0 * self.e02 * other.e12 * other.e31 - 2.0 * self.e12 * other.s * other.e0123 - 2.0 * self.e12 * other.e01 * other.e23 - 2.0 * self.e12 * other.e02 * other.e31 + 2.0 * self.e12 * other.e12 * other.e03 + self.e03 * other.s * other.s + self.e03 * other.e01 * other.e01 + self.e03 * other.e02 * other.e02 + self.e03 * other.e12 * other.e12 - self.e03 * other.e03 * other.e03 - self.e03 * other.e31 * other.e31 - self.e03 * other.e23 * other.e23 - self.e03 * other.e0123 * other.e0123 - 2.0 * self.e31 * other.s * other.e01 + 2.0 * self.e31 * other.e02 * other.e12 + 2.0 * self.e31 * other.e03 * other.e31 + 2.0 * self.e31 * other.e23 * other.e0123 + 2.0 * self.e23 * other.s * other.e02 + 2.0 * self.e23 * other.e01 * other.e12 + 2.0 * self.e23 * other.e03 * other.e23 - 2.0 * self.e23 * other.e31 * other.e0123,
            e31: -2.0 * self.e01 * other.s * other.e03 + 2.0 * self.e01 * other.e01 * other.e31 + 2.0 * self.e01 * other.e02 * other.e23 + 2.0 * self.e01 * other.e12 * other.e0123 - 2.0 * self.e02 * other.s * other.e0123 - 2.0 * self.e02 * other.e01 * other.e23 + 2.0 * self.e02 * other.e02 * other.e31 - 2.0 * self.e02 * other.e12 * other.e03 + 2.0 * self.e12 * other.s * other.e23 - 2.0 * self.e12 * other.e01 * other.e0123 + 2.0 * self.e12 * other.e02 * other.e03 + 2.0 * self.e12 * other.e12 * other.e31 + 2.0 * self.e03 * other.s * other.e01 + 2.0 * self.e03 * other.e02 * other.e12 + 2.0 * self.e03 * other.e03 * other.e31 - 2.0 * self.e03 * other.e23 * other.e0123 + self.e31 * other.s * other.s - self.e31 * other.e01 * other.e01 + self.e31 * other.e02 * other.e02 - self.e31 * other.e12 * other.e12 - self.e31 * other.e03 * other.e03 + self.e31 * other.e31 * other.e31 - self.e31 * other.e23 * other.e23 + self.e31 * other.e0123 * other.e0123 - 2.0 * self.e23 * other.s * other.e12 + 2.0 * self.e23 * other.e01 * other.e02 + 2.0 * self.e23 * other.e03 * other.e0123 + 2.0 * self.e23 * other.e31 * other.e23,
            e23: -2.0 * self.e01 * other.s * other.e0123 + 2.0 * self.e01 * other.e01 * other.e23 - 2.0 * self.e01 * other.e02 * other.e31 - 2.0 * self.e01 * other.e12 * other.e03 + 2.0 * self.e02 * other.s * other.e03 + 2.0 * self.e02 * other.e01 * other.e31 + 2.0 * self.e02 * other.e02 * other.e23 - 2.0 * self.e02 * other.e12 * other.e0123 - 2.0 * self.e12 * other.s * other.e31 + 2.0 * self.e12 * other.e01 * other.e03 + 2.0 * self.e12 * other.e02 * other.e0123 + 2.0 * self.e12 * other.e12 * other.e23 - 2.0 * self.e03 * other.s * other.e02 + 2.0 * self.e03 * other.e01 * other.e12 + 2.0 * self.e03 * other.e03 * other.e23 + 2.0 * self.e03 * other.e31 * other.e0123 + 2.0 * self.e31 * other.s * other.e12 + 2.0 * self.e31 * other.e01 * other.e02 - 2.0 * self.e31 * other.e03 * other.e0123 + 2.0 * self.e31 * other.e31 * other.e23 + self.e23 * other.s * other.s + self.e23 * other.e01 * other.e01 - self.e23 * other.e02 * other.e02 - self.e23 * other.e12 * other.e12 - self.e23 * other.e03 * other.e03 - self.e23 * other.e31 * other.e31 + self.e23 * other.e23 * other.e23 + self.e23 * other.e0123 * other.e0123,
            e0123: 2.0 * self.s * other.s * other.e0123 - 2.0 * self.s * other.e01 * other.e23 - 2.0 * self.s * other.e02 * other.e31 - 2.0 * self.s * other.e12 * other.e03 + self.e0123 * other.s * other.s - self.e0123 * other.e01 * other.e01 - self.e0123 * other.e02 * other.e02 + self.e0123 * other.e12 * other.e12 - self.e0123 * other.e03 * other.e03 + self.e0123 * other.e31 * other.e31 + self.e0123 * other.e23 * other.e23 - self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Transform<Multivector> for EvenMultivector {
    type Output = Multivector;
    fn transform(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01 + self.s * other.e2 * other.e2 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e013 * other.e013 + self.s * other.e23 * other.e23 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 + self.s * other.e0123 * other.e0123 + 2.0 * self.e0123 * other.s * other.e0123 + 2.0 * self.e0123 * other.e0 * other.e123 + 2.0 * self.e0123 * other.e1 * other.e032 - 2.0 * self.e0123 * other.e01 * other.e23 + 2.0 * self.e0123 * other.e2 * other.e013 - 2.0 * self.e0123 * other.e02 * other.e31 - 2.0 * self.e0123 * other.e12 * other.e03 + 2.0 * self.e0123 * other.e021 * other.e3,
            e0: 2.0 * self.s * other.s * other.e0 + 2.0 * self.s * other.e1 * other.e01 + 2.0 * self.s * other.e2 * other.e02 - 2.0 * self.s * other.e12 * other.e021 + 2.0 * self.s * other.e3 * other.e03 - 2.0 * self.s * other.e31 * other.e013 - 2.0 * self.s * other.e23 * other.e032 + 2.0 * self.s * other.e123 * other.e0123 + 2.0 * self.e0123 * other.s * other.e123 - 2.0 * self.e0123 * other.e0 * other.e0123 - 2.0 * self.e0123 * other.e1 * other.e23 - 2.0 * self.e0123 * other.e01 * other.e032 - 2.0 * self.e0123 * other.e2 * other.e31 - 2.0 * self.e0123 * other.e02 * other.e013 - 2.0 * self.e0123 * other.e12 * other.e3 - 2.0 * self.e0123 * other.e021 * other.e03,
            e1: 2.0 * self.s * other.s * other.e1 - 2.0 * self.s * other.e0 * other.e01 + 2.0 * self.s * other.e2 * other.e12 + 2.0 * self.s * other.e02 * other.e021 - 2.0 * self.s * other.e3 * other.e31 - 2.0 * self.s * other.e03 * other.e013 + 2.0 * self.s * other.e23 * other.e123 + 2.0 * self.s * other.e032 * other.e0123 + 2.0 * self.e0123 * other.s * other.e032 + 2.0 * self.e0123 * other.e0 * other.e23 + 2.0 * self.e0123 * other.e1 * other.e0123 - 2.0 * self.e0123 * other.e01 * other.e123 - 2.0 * self.e0123 * other.e2 * other.e03 + 2.0 * self.e0123 * other.e02 * other.e3 + 2.0 * self.e0123 * other.e12 * other.e013 - 2.0 * self.e0123 * other.e021 * other.e31,
            e01: self.e01 * other.s * other.s + self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1 - self.e01 * other.e01 * other.e01 + self.e01 * other.e2 * other.e2 + self.e01 * other.e02 * other.e02 - self.e01 * other.e12 * other.e12 - self.e01 * other.e021 * other.e021 + self.e01 * other.e3 * other.e3 + self.e01 * other.e03 * other.e03 - self.e01 * other.e31 * other.e31 - self.e01 * other.e013 * other.e013 + self.e01 * other.e23 * other.e23 + self.e01 * other.e032 * other.e032 - self.e01 * other.e123 * other.e123 - self.e01 * other.e0123 * other.e0123 + 2.0 * self.e02 * other.s * other.e12 - 2.0 * self.e02 * other.e0 * other.e021 - 2.0 * self.e02 * other.e1 * other.e2 - 2.0 * self.e02 * other.e01 * other.e02 + 2.0 * self.e02 * other.e3 * other.e123 + 2.0 * self.e02 * other.e03 * other.e0123 + 2.0 * self.e02 * other.e31 * other.e23 + 2.0 * self.e02 * other.e013 * other.e032 - 2.0 * self.e12 * other.s * other.e02 + 2.0 * self.e12 * other.e0 * other.e2 + 2.0 * self.e12 * other.e1 * other.e021 + 2.0 * self.e12 * other.e01 * other.e12 + 2.0 * self.e12 * other.e3 * other.e032 + 2.0 * self.e12 * other.e03 * other.e23 + 2.0 * self.e12 * other.e31 * other.e0123 + 2.0 * self.e12 * other.e013 * other.e123 - 2.0 * self.e03 * other.s * other.e31 + 2.0 * self.e03 * other.e0 * other.e013 - 2.0 * self.e03 * other.e1 * other.e3 - 2.0 * self.e03 * other.e01 * other.e03 - 2.0 * self.e03 * other.e2 * other.e123 - 2.0 * self.e03 * other.e02 * other.e0123 + 2.0 * self.e03 * other.e12 * other.e23 + 2.0 * self.e03 * other.e021 * other.e032 + 2.0 * self.e31 * other.s * other.e03 - 2.0 * self.e31 * other.e0 * other.e3 + 2.0 * self.e31 * other.e1 * other.e013 + 2.0 * self.e31 * other.e01 * other.e31 + 2.0 * self.e31 * other.e2 * other.e032 + 2.0 * self.e31 * other.e02 * other.e23 - 2.0 * self.e31 * other.e12 * other.e0123 - 2.0 * self.e31 * other.e021 * other.e123 - 2.0 * self.e23 * other.s * other.e0123 + 2.0 * self.e23 * other.e0 * other.e123 + 2.0 * self.e23 * other.e1 * other.e032 + 2.0 * self.e23 * other.e01 * other.e23 - 2.0 * self.e23 * other.e2 * other.e013 - 2.0 * self.e23 * other.e02 * other.e31 - 2.0 * self.e23 * other.e12 * other.e03 - 2.0 * self.e23 * other.e021 * other.e3,
            e2: 2.0 * self.s * other.s * other.e2 - 2.0 * self.s * other.e0 * other.e02 - 2.0 * self.s * other.e1 * other.e12 - 2.0 * self.s * other.e01 * other.e021 + 2.0 * self.s * other.e3 * other.e23 + 2.0 * self.s * other.e03 * other.e032 + 2.0 * self.s * other.e31 * other.e123 + 2.0 * self.s * other.e013 * other.e0123 + 2.0 * self.e0123 * other.s * other.e013 + 2.0 * self.e0123 * other.e0 * other.e31 + 2.0 * self.e0123 * other.e1 * other.e03 - 2.0 * self.e0123 * other.e01 * other.e3 + 2.0 * self.e0123 * other.e2 * other.e0123 - 2.0 * self.e0123 * other.e02 * other.e123 - 2.0 * self.e0123 * other.e12 * other.e032 + 2.0 * self.e0123 * other.e021 * other.e23,
            e02: -2.0 * self.e01 * other.s * other.e12 + 2.0 * self.e01 * other.e0 * other.e021 - 2.0 * self.e01 * other.e1 * other.e2 - 2.0 * self.e01 * other.e01 * other.e02 - 2.0 * self.e01 * other.e3 * other.e123 - 2.0 * self.e01 * other.e03 * other.e0123 + 2.0 * self.e01 * other.e31 * other.e23 + 2.0 * self.e01 * other.e013 * other.e032 + self.e02 * other.s * other.s + self.e02 * other.e0 * other.e0 + self.e02 * other.e1 * other.e1 + self.e02 * other.e01 * other.e01 - self.e02 * other.e2 * other.e2 - self.e02 * other.e02 * other.e02 - self.e02 * other.e12 * other.e12 - self.e02 * other.e021 * other.e021 + self.e02 * other.e3 * other.e3 + self.e02 * other.e03 * other.e03 + self.e02 * other.e31 * other.e31 + self.e02 * other.e013 * other.e013 - self.e02 * other.e23 * other.e23 - self.e02 * other.e032 * other.e032 - self.e02 * other.e123 * other.e123 - self.e02 * other.e0123 * other.e0123 + 2.0 * self.e12 * other.s * other.e01 - 2.0 * self.e12 * other.e0 * other.e1 + 2.0 * self.e12 * other.e2 * other.e021 + 2.0 * self.e12 * other.e02 * other.e12 + 2.0 * self.e12 * other.e3 * other.e013 + 2.0 * self.e12 * other.e03 * other.e31 - 2.0 * self.e12 * other.e23 * other.e0123 - 2.0 * self.e12 * other.e032 * other.e123 + 2.0 * self.e03 * other.s * other.e23 - 2.0 * self.e03 * other.e0 * other.e032 + 2.0 * self.e03 * other.e1 * other.e123 + 2.0 * self.e03 * other.e01 * other.e0123 - 2.0 * self.e03 * other.e2 * other.e3 - 2.0 * self.e03 * other.e02 * other.e03 + 2.0 * self.e03 * other.e12 * other.e31 + 2.0 * self.e03 * other.e021 * other.e013 - 2.0 * self.e31 * other.s * other.e0123 + 2.0 * self.e31 * other.e0 * other.e123 - 2.0 * self.e31 * other.e1 * other.e032 - 2.0 * self.e31 * other.e01 * other.e23 + 2.0 * self.e31 * other.e2 * other.e013 + 2.0 * self.e31 * other.e02 * other.e31 - 2.0 * self.e31 * other.e12 * other.e03 - 2.0 * self.e31 * other.e021 * other.e3 - 2.0 * self.e23 * other.s * other.e03 + 2.0 * self.e23 * other.e0 * other.e3 + 2.0 * self.e23 * other.e1 * other.e013 + 2.0 * self.e23 * other.e01 * other.e31 + 2.0 * self.e23 * other.e2 * other.e032 + 2.0 * self.e23 * other.e02 * other.e23 + 2.0 * self.e23 * other.e12 * other.e0123 + 2.0 * self.e23 * other.e021 * other.e123,
            e12: 2.0 * self.e01 * other.s * other.e02 + 2.0 * self.e01 * other.e0 * other.e2 - 2.0 * self.e01 * other.e1 * other.e021 + 2.0 * self.e01 * other.e01 * other.e12 - 2.0 * self.e01 * other.e3 * other.e032 + 2.0 * self.e01 * other.e03 * other.e23 - 2.0 * self.e01 * other.e31 * other.e0123 + 2.0 * self.e01 * other.e013 * other.e123 - 2.0 * self.e02 * other.s * other.e01 - 2.0 * self.e02 * other.e0 * other.e1 - 2.0 * self.e02 * other.e2 * other.e021 + 2.0 * self.e02 * other.e02 * other.e12 - 2.0 * self.e02 * other.e3 * other.e013 + 2.0 * self.e02 * other.e03 * other.e31 + 2.0 * self.e02 * other.e23 * other.e0123 - 2.0 * self.e02 * other.e032 * other.e123 + self.e12 * other.s * other.s + self.e12 * other.e0 * other.e0 - self.e12 * other.e1 * other.e1 - self.e12 * other.e01 * other.e01 - self.e12 * other.e2 * other.e2 - self.e12 * other.e02 * other.e02 + self.e12 * other.e12 * other.e12 + self.e12 * other.e021 * other.e021 + self.e12 * other.e3 * other.e3 + self.e12 * other.e03 * other.e03 - self.e12 * other.e31 * other.e31 - self.e12 * other.e013 * other.e013 - self.e12 * other.e23 * other.e23 - self.e12 * other.e032 * other.e032 + self.e12 * other.e123 * other.e123 + self.e12 * other.e0123 * other.e0123 - 2.0 * self.e03 * other.s * other.e0123 - 2.0 * self.e03 * other.e0 * other.e123 + 2.0 * self.e03 * other.e1 * other.e032 - 2.0 * self.e03 * other.e01 * other.e23 + 2.0 * self.e03 * other.e2 * other.e013 - 2.0 * self.e03 * other.e02 * other.e31 + 2.0 * self.e03 * other.e12 * other.e03 - 2.0 * self.e03 * other.e021 * other.e3 - 2.0 * self.e31 * other.s * other.e23 + 2.0 * self.e31 * other.e0 * other.e032 + 2.0 * self.e31 * other.e1 * other.e123 + 2.0 * self.e31 * other.e01 * other.e0123 + 2.0 * self.e31 * other.e2 * other.e3 + 2.0 * self.e31 * other.e02 * other.e03 + 2.0 * self.e31 * other.e12 * other.e31 + 2.0 * self.e31 * other.e021 * other.e013 + 2.0 * self.e23 * other.s * other.e31 - 2.0 * self.e23 * other.e0 * other.e013 + 2.0 * self.e23 * other.e1 * other.e3 + 2.0 * self.e23 * other.e01 * other.e03 - 2.0 * self.e23 * other.e2 * other.e123 - 2.0 * self.e23 * other.e02 * other.e0123 + 2.0 * self.e23 * other.e12 * other.e23 + 2.0 * self.e23 * other.e021 * other.e032,
            e021: -2.0 * self.e01 * other.s * other.e2 + 2.0 * self.e01 * other.e0 * other.e02 - 2.0 * self.e01 * other.e1 * other.e12 - 2.0 * self.e01 * other.e01 * other.e021 - 2.0 * self.e01 * other.e3 * other.e23 - 2.0 * self.e01 * other.e03 * other.e032 + 2.0 * self.e01 * other.e31 * other.e123 + 2.0 * self.e01 * other.e013 * other.e0123 + 2.0 * self.e02 * other.s * other.e1 - 2.0 * self.e02 * other.e0 * other.e01 - 2.0 * self.e02 * other.e2 * other.e12 - 2.0 * self.e02 * other.e02 * other.e021 - 2.0 * self.e02 * other.e3 * other.e31 - 2.0 * self.e02 * other.e03 * other.e013 - 2.0 * self.e02 * other.e23 * other.e123 - 2.0 * self.e02 * other.e032 * other.e0123 - 2.0 * self.e12 * other.s * other.e0 + 2.0 * self.e12 * other.e1 * other.e01 + 2.0 * self.e12 * other.e2 * other.e02 + 2.0 * self.e12 * other.e12 * other.e021 - 2.0 * self.e12 * other.e3 * other.e03 - 2.0 * self.e12 * other.e31 * other.e013 - 2.0 * self.e12 * other.e23 * other.e032 - 2.0 * self.e12 * other.e123 * other.e0123 + 2.0 * self.e03 * other.s * other.e123 - 2.0 * self.e03 * other.e0 * other.e0123 + 2.0 * self.e03 * other.e1 * other.e23 + 2.0 * self.e03 * other.e01 * other.e032 + 2.0 * self.e03 * other.e2 * other.e31 + 2.0 * self.e03 * other.e02 * other.e013 - 2.0 * self.e03 * other.e12 * other.e3 - 2.0 * self.e03 * other.e021 * other.e03 - 2.0 * self.e31 * other.s * other.e032 + 2.0 * self.e31 * other.e0 * other.e23 - 2.0 * self.e31 * other.e1 * other.e0123 - 2.0 * self.e31 * other.e01 * other.e123 - 2.0 * self.e31 * other.e2 * other.e03 - 2.0 * self.e31 * other.e02 * other.e3 + 2.0 * self.e31 * other.e12 * other.e013 + 2.0 * self.e31 * other.e021 * other.e31 + 2.0 * self.e23 * other.s * other.e013 - 2.0 * self.e23 * other.e0 * other.e31 - 2.0 * self.e23 * other.e1 * other.e03 - 2.0 * self.e23 * other.e01 * other.e3 + 2.0 * self.e23 * other.e2 * other.e0123 + 2.0 * self.e23 * other.e02 * other.e123 + 2.0 * self.e23 * other.e12 * other.e032 + 2.0 * self.e23 * other.e021 * other.e23,
            e3: 2.0 * self.s * other.s * other.e3 - 2.0 * self.s * other.e0 * other.e03 + 2.0 * self.s * other.e1 * other.e31 + 2.0 * self.s * other.e01 * other.e013 - 2.0 * self.s * other.e2 * other.e23 - 2.0 * self.s * other.e02 * other.e032 + 2.0 * self.s * other.e12 * other.e123 + 2.0 * self.s * other.e021 * other.e0123 + 2.0 * self.e0123 * other.s * other.e021 + 2.0 * self.e0123 * other.e0 * other.e12 - 2.0 * self.e0123 * other.e1 * other.e02 + 2.0 * self.e0123 * other.e01 * other.e2 + 2.0 * self.e0123 * other.e3 * other.e0123 - 2.0 * self.e0123 * other.e03 * other.e123 + 2.0 * self.e0123 * other.e31 * other.e032 - 2.0 * self.e0123 * other.e013 * other.e23,
            e03: 2.0 * self.e01 * other.s * other.e31 - 2.0 * self.e01 * other.e0 * other.e013 - 2.0 * self.e01 * other.e1 * other.e3 - 2.0 * self.e01 * other.e01 * other.e03 + 2.0 * self.e01 * other.e2 * other.e123 + 2.0 * self.e01 * other.e02 * other.e0123 + 2.0 * self.e01 * other.e12 * other.e23 + 2.0 * self.e01 * other.e021 * other.e032 - 2.0 * self.e02 * other.s * other.e23 + 2.0 * self.e02 * other.e0 * other.e032 - 2.0 * self.e02 * other.e1 * other.e123 - 2.0 * self.e02 * other.e01 * other.e0123 - 2.0 * self.e02 * other.e2 * other.e3 - 2.0 * self.e02 * other.e02 * other.e03 + 2.0 * self.e02 * other.e12 * other.e31 + 2.0 * self.e02 * other.e021 * other.e013 - 2.0 * self.e12 * other.s * other.e0123 + 2.0 * self.e12 * other.e0 * other.e123 - 2.0 * self.e12 * other.e1 * other.e032 - 2.0 * self.e12 * other.e01 * other.e23 - 2.0 * self.e12 * other.e2 * other.e013 - 2.0 * self.e12 * other.e02 * other.e31 + 2.0 * self.e12 * other.e12 * other.e03 + 2.0 * self.e12 * other.e021 * other.e3 + self.e03 * other.s * other.s + self.e03 * other.e0 * other.e0 + self.e03 * other.e1 * other.e1 + self.e03 * other.e01 * other.e01 + self.e03 * other.e2 * other.e2 + self.e03 * other.e02 * other.e02 + self.e03 * other.e12 * other.e12 + self.e03 * other.e021 * other.e021 - self.e03 * other.e3 * other.e3 - self.e03 * other.e03 * other.e03 - self.e03 * other.e31 * other.e31 - self.e03 * other.e013 * other.e013 - self.e03 * other.e23 * other.e23 - self.e03 * other.e032 * other.e032 - self.e03 * other.e123 * other.e123 - self.e03 * other.e0123 * other.e0123 - 2.0 * self.e31 * other.s * other.e01 + 2.0 * self.e31 * other.e0 * other.e1 + 2.0 * self.e31 * other.e2 * other.e021 + 2.0 * self.e31 * other.e02 * other.e12 + 2.0 * self.e31 * other.e3 * other.e013 + 2.0 * self.e31 * other.e03 * other.e31 + 2.0 * self.e31 * other.e23 * other.e0123 + 2.0 * self.e31 * other.e032 * other.e123 + 2.0 * self.e23 * other.s * other.e02 - 2.0 * self.e23 * other.e0 * other.e2 + 2.0 * self.e23 * other.e1 * other.e021 + 2.0 * self.e23 * other.e01 * other.e12 + 2.0 * self.e23 * other.e3 * other.e032 + 2.0 * self.e23 * other.e03 * other.e23 - 2.0 * self.e23 * other.e31 * other.e0123 - 2.0 * self.e23 * other.e013 * other.e123,
            e31: -2.0 * self.e01 * other.s * other.e03 - 2.0 * self.e01 * other.e0 * other.e3 - 2.0 * self.e01 * other.e1 * other.e013 + 2.0 * self.e01 * other.e01 * other.e31 - 2.0 * self.e01 * other.e2 * other.e032 + 2.0 * self.e01 * other.e02 * other.e23 + 2.0 * self.e01 * other.e12 * other.e0123 - 2.0 * self.e01 * other.e021 * other.e123 - 2.0 * self.e02 * other.s * other.e0123 - 2.0 * self.e02 * other.e0 * other.e123 + 2.0 * self.e02 * other.e1 * other.e032 - 2.0 * self.e02 * other.e01 * other.e23 - 2.0 * self.e02 * other.e2 * other.e013 + 2.0 * self.e02 * other.e02 * other.e31 - 2.0 * self.e02 * other.e12 * other.e03 + 2.0 * self.e02 * other.e021 * other.e3 + 2.0 * self.e12 * other.s * other.e23 - 2.0 * self.e12 * other.e0 * other.e032 - 2.0 * self.e12 * other.e1 * other.e123 - 2.0 * self.e12 * other.e01 * other.e0123 + 2.0 * self.e12 * other.e2 * other.e3 + 2.0 * self.e12 * other.e02 * other.e03 + 2.0 * self.e12 * other.e12 * other.e31 + 2.0 * self.e12 * other.e021 * other.e013 + 2.0 * self.e03 * other.s * other.e01 + 2.0 * self.e03 * other.e0 * other.e1 - 2.0 * self.e03 * other.e2 * other.e021 + 2.0 * self.e03 * other.e02 * other.e12 - 2.0 * self.e03 * other.e3 * other.e013 + 2.0 * self.e03 * other.e03 * other.e31 - 2.0 * self.e03 * other.e23 * other.e0123 + 2.0 * self.e03 * other.e032 * other.e123 + self.e31 * other.s * other.s + self.e31 * other.e0 * other.e0 - self.e31 * other.e1 * other.e1 - self.e31 * other.e01 * other.e01 + self.e31 * other.e2 * other.e2 + self.e31 * other.e02 * other.e02 - self.e31 * other.e12 * other.e12 - self.e31 * other.e021 * other.e021 - self.e31 * other.e3 * other.e3 - self.e31 * other.e03 * other.e03 + self.e31 * other.e31 * other.e31 + self.e31 * other.e013 * other.e013 - self.e31 * other.e23 * other.e23 - self.e31 * other.e032 * other.e032 + self.e31 * other.e123 * other.e123 + self.e31 * other.e0123 * other.e0123 - 2.0 * self.e23 * other.s * other.e12 + 2.0 * self.e23 * other.e0 * other.e021 + 2.0 * self.e23 * other.e1 * other.e2 + 2.0 * self.e23 * other.e01 * other.e02 + 2.0 * self.e23 * other.e3 * other.e123 + 2.0 * self.e23 * other.e03 * other.e0123 + 2.0 * self.e23 * other.e31 * other.e23 + 2.0 * self.e23 * other.e013 * other.e032,
            e013: 2.0 * self.e01 * other.s * other.e3 - 2.0 * self.e01 * other.e0 * other.e03 - 2.0 * self.e01 * other.e1 * other.e31 - 2.0 * self.e01 * other.e01 * other.e013 - 2.0 * self.e01 * other.e2 * other.e23 - 2.0 * self.e01 * other.e02 * other.e032 - 2.0 * self.e01 * other.e12 * other.e123 - 2.0 * self.e01 * other.e021 * other.e0123 + 2.0 * self.e02 * other.s * other.e123 - 2.0 * self.e02 * other.e0 * other.e0123 + 2.0 * self.e02 * other.e1 * other.e23 + 2.0 * self.e02 * other.e01 * other.e032 - 2.0 * self.e02 * other.e2 * other.e31 - 2.0 * self.e02 * other.e02 * other.e013 + 2.0 * self.e02 * other.e12 * other.e3 + 2.0 * self.e02 * other.e021 * other.e03 + 2.0 * self.e12 * other.s * other.e032 - 2.0 * self.e12 * other.e0 * other.e23 + 2.0 * self.e12 * other.e1 * other.e0123 + 2.0 * self.e12 * other.e01 * other.e123 - 2.0 * self.e12 * other.e2 * other.e03 - 2.0 * self.e12 * other.e02 * other.e3 + 2.0 * self.e12 * other.e12 * other.e013 + 2.0 * self.e12 * other.e021 * other.e31 - 2.0 * self.e03 * other.s * other.e1 + 2.0 * self.e03 * other.e0 * other.e01 - 2.0 * self.e03 * other.e2 * other.e12 - 2.0 * self.e03 * other.e02 * other.e021 - 2.0 * self.e03 * other.e3 * other.e31 - 2.0 * self.e03 * other.e03 * other.e013 + 2.0 * self.e03 * other.e23 * other.e123 + 2.0 * self.e03 * other.e032 * other.e0123 - 2.0 * self.e31 * other.s * other.e0 + 2.0 * self.e31 * other.e1 * other.e01 - 2.0 * self.e31 * other.e2 * other.e02 - 2.0 * self.e31 * other.e12 * other.e021 + 2.0 * self.e31 * other.e3 * other.e03 + 2.0 * self.e31 * other.e31 * other.e013 - 2.0 * self.e31 * other.e23 * other.e032 - 2.0 * self.e31 * other.e123 * other.e0123 - 2.0 * self.e23 * other.s * other.e021 + 2.0 * self.e23 * other.e0 * other.e12 - 2.0 * self.e23 * other.e1 * other.e02 - 2.0 * self.e23 * other.e01 * other.e2 - 2.0 * self.e23 * other.e3 * other.e0123 - 2.0 * self.e23 * other.e03 * other.e123 + 2.0 * self.e23 * other.e31 * other.e032 + 2.0 * self.e23 * other.e013 * other.e23,
            e23: -2.0 * self.e01 * other.s * other.e0123 - 2.0 * self.e01 * other.e0 * other.e123 - 2.0 * self.e01 * other.e1 * other.e032 + 2.0 * self.e01 * other.e01 * other.e23 + 2.0 * self.e01 * other.e2 * other.e013 - 2.0 * self.e01 * other.e02 * other.e31 - 2.0 * self.e01 * other.e12 * other.e03 + 2.0 * self.e01 * other.e021 * other.e3 + 2.0 * self.e02 * other.s * other.e03 + 2.0 * self.e02 * other.e0 * other.e3 - 2.0 * self.e02 * other.e1 * other.e013 + 2.0 * self.e02 * other.e01 * other.e31 - 2.0 * self.e02 * other.e2 * other.e032 + 2.0 * self.e02 * other.e02 * other.e23 - 2.0 * self.e02 * other.e12 * other.e0123 + 2.0 * self.e02 * other.e021 * other.e123 - 2.0 * self.e12 * other.s * other.e31 + 2.0 * self.e12 * other.e0 * other.e013 + 2.0 * self.e12 * other.e1 * other.e3 + 2.0 * self.e12 * other.e01 * other.e03 + 2.0 * self.e12 * other.e2 * other.e123 + 2.0 * self.e12 * other.e02 * other.e0123 + 2.0 * self.e12 * other.e12 * other.e23 + 2.0 * self.e12 * other.e021 * other.e032 - 2.0 * self.e03 * other.s * other.e02 - 2.0 * self.e03 * other.e0 * other.e2 - 2.0 * self.e03 * other.e1 * other.e021 + 2.0 * self.e03 * other.e01 * other.e12 - 2.0 * self.e03 * other.e3 * other.e032 + 2.0 * self.e03 * other.e03 * other.e23 + 2.0 * self.e03 * other.e31 * other.e0123 - 2.0 * self.e03 * other.e013 * other.e123 + 2.0 * self.e31 * other.s * other.e12 - 2.0 * self.e31 * other.e0 * other.e021 + 2.0 * self.e31 * other.e1 * other.e2 + 2.0 * self.e31 * other.e01 * other.e02 - 2.0 * self.e31 * other.e3 * other.e123 - 2.0 * self.e31 * other.e03 * other.e0123 + 2.0 * self.e31 * other.e31 * other.e23 + 2.0 * self.e31 * other.e013 * other.e032 + self.e23 * other.s * other.s + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1 + self.e23 * other.e01 * other.e01 - self.e23 * other.e2 * other.e2 - self.e23 * other.e02 * other.e02 - self.e23 * other.e12 * other.e12 - self.e23 * other.e021 * other.e021 - self.e23 * other.e3 * other.e3 - self.e23 * other.e03 * other.e03 - self.e23 * other.e31 * other.e31 - self.e23 * other.e013 * other.e013 + self.e23 * other.e23 * other.e23 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123 + self.e23 * other.e0123 * other.e0123,
            e032: 2.0 * self.e01 * other.s * other.e123 - 2.0 * self.e01 * other.e0 * other.e0123 - 2.0 * self.e01 * other.e1 * other.e23 - 2.0 * self.e01 * other.e01 * other.e032 + 2.0 * self.e01 * other.e2 * other.e31 + 2.0 * self.e01 * other.e02 * other.e013 + 2.0 * self.e01 * other.e12 * other.e3 + 2.0 * self.e01 * other.e021 * other.e03 - 2.0 * self.e02 * other.s * other.e3 + 2.0 * self.e02 * other.e0 * other.e03 - 2.0 * self.e02 * other.e1 * other.e31 - 2.0 * self.e02 * other.e01 * other.e013 - 2.0 * self.e02 * other.e2 * other.e23 - 2.0 * self.e02 * other.e02 * other.e032 + 2.0 * self.e02 * other.e12 * other.e123 + 2.0 * self.e02 * other.e021 * other.e0123 - 2.0 * self.e12 * other.s * other.e013 + 2.0 * self.e12 * other.e0 * other.e31 - 2.0 * self.e12 * other.e1 * other.e03 - 2.0 * self.e12 * other.e01 * other.e3 - 2.0 * self.e12 * other.e2 * other.e0123 - 2.0 * self.e12 * other.e02 * other.e123 + 2.0 * self.e12 * other.e12 * other.e032 + 2.0 * self.e12 * other.e021 * other.e23 + 2.0 * self.e03 * other.s * other.e2 - 2.0 * self.e03 * other.e0 * other.e02 - 2.0 * self.e03 * other.e1 * other.e12 - 2.0 * self.e03 * other.e01 * other.e021 - 2.0 * self.e03 * other.e3 * other.e23 - 2.0 * self.e03 * other.e03 * other.e032 - 2.0 * self.e03 * other.e31 * other.e123 - 2.0 * self.e03 * other.e013 * other.e0123 + 2.0 * self.e31 * other.s * other.e021 - 2.0 * self.e31 * other.e0 * other.e12 - 2.0 * self.e31 * other.e1 * other.e02 - 2.0 * self.e31 * other.e01 * other.e2 + 2.0 * self.e31 * other.e3 * other.e0123 + 2.0 * self.e31 * other.e03 * other.e123 + 2.0 * self.e31 * other.e31 * other.e032 + 2.0 * self.e31 * other.e013 * other.e23 - 2.0 * self.e23 * other.s * other.e0 - 2.0 * self.e23 * other.e1 * other.e01 + 2.0 * self.e23 * other.e2 * other.e02 - 2.0 * self.e23 * other.e12 * other.e021 + 2.0 * self.e23 * other.e3 * other.e03 - 2.0 * self.e23 * other.e31 * other.e013 + 2.0 * self.e23 * other.e23 * other.e032 - 2.0 * self.e23 * other.e123 * other.e0123,
            e123: -2.0 * self.e01 * other.s * other.e032 - 2.0 * self.e01 * other.e0 * other.e23 - 2.0 * self.e01 * other.e1 * other.e0123 + 2.0 * self.e01 * other.e01 * other.e123 - 2.0 * self.e01 * other.e2 * other.e03 + 2.0 * self.e01 * other.e02 * other.e3 + 2.0 * self.e01 * other.e12 * other.e013 - 2.0 * self.e01 * other.e021 * other.e31 - 2.0 * self.e02 * other.s * other.e013 - 2.0 * self.e02 * other.e0 * other.e31 + 2.0 * self.e02 * other.e1 * other.e03 - 2.0 * self.e02 * other.e01 * other.e3 - 2.0 * self.e02 * other.e2 * other.e0123 + 2.0 * self.e02 * other.e02 * other.e123 - 2.0 * self.e02 * other.e12 * other.e032 + 2.0 * self.e02 * other.e021 * other.e23 + 2.0 * self.e12 * other.s * other.e3 - 2.0 * self.e12 * other.e0 * other.e03 - 2.0 * self.e12 * other.e1 * other.e31 - 2.0 * self.e12 * other.e01 * other.e013 + 2.0 * self.e12 * other.e2 * other.e23 + 2.0 * self.e12 * other.e02 * other.e032 + 2.0 * self.e12 * other.e12 * other.e123 + 2.0 * self.e12 * other.e021 * other.e0123 - 2.0 * self.e03 * other.s * other.e021 - 2.0 * self.e03 * other.e0 * other.e12 - 2.0 * self.e03 * other.e1 * other.e02 + 2.0 * self.e03 * other.e01 * other.e2 - 2.0 * self.e03 * other.e3 * other.e0123 + 2.0 * self.e03 * other.e03 * other.e123 + 2.0 * self.e03 * other.e31 * other.e032 - 2.0 * self.e03 * other.e013 * other.e23 + 2.0 * self.e31 * other.s * other.e2 - 2.0 * self.e31 * other.e0 * other.e02 + 2.0 * self.e31 * other.e1 * other.e12 + 2.0 * self.e31 * other.e01 * other.e021 - 2.0 * self.e31 * other.e3 * other.e23 - 2.0 * self.e31 * other.e03 * other.e032 + 2.0 * self.e31 * other.e31 * other.e123 + 2.0 * self.e31 * other.e013 * other.e0123 + 2.0 * self.e23 * other.s * other.e1 - 2.0 * self.e23 * other.e0 * other.e01 - 2.0 * self.e23 * other.e2 * other.e12 - 2.0 * self.e23 * other.e02 * other.e021 + 2.0 * self.e23 * other.e3 * other.e31 + 2.0 * self.e23 * other.e03 * other.e013 + 2.0 * self.e23 * other.e23 * other.e123 + 2.0 * self.e23 * other.e032 * other.e0123,
            e0123: 2.0 * self.s * other.s * other.e0123 - 2.0 * self.s * other.e0 * other.e123 - 2.0 * self.s * other.e1 * other.e032 - 2.0 * self.s * other.e01 * other.e23 - 2.0 * self.s * other.e2 * other.e013 - 2.0 * self.s * other.e02 * other.e31 - 2.0 * self.s * other.e12 * other.e03 - 2.0 * self.s * other.e021 * other.e3 + self.e0123 * other.s * other.s + self.e0123 * other.e0 * other.e0 - self.e0123 * other.e1 * other.e1 - self.e0123 * other.e01 * other.e01 - self.e0123 * other.e2 * other.e2 - self.e0123 * other.e02 * other.e02 + self.e0123 * other.e12 * other.e12 + self.e0123 * other.e021 * other.e021 - self.e0123 * other.e3 * other.e3 - self.e0123 * other.e03 * other.e03 + self.e0123 * other.e31 * other.e31 + self.e0123 * other.e013 * other.e013 + self.e0123 * other.e23 * other.e23 + self.e0123 * other.e032 * other.e032 - self.e0123 * other.e123 * other.e123 - self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Transform<Scalar> for Multivector {
    type Output = Multivector;
    fn transform(self, other: Scalar) -> Multivector {
        Multivector {
            s: self.s * other.s * other.s,
            e0: self.e0 * other.s * other.s,
            e1: self.e1 * other.s * other.s,
            e01: self.e01 * other.s * other.s,
            e2: self.e2 * other.s * other.s,
            e02: self.e02 * other.s * other.s,
            e12: self.e12 * other.s * other.s,
            e021: self.e021 * other.s * other.s,
            e3: self.e3 * other.s * other.s,
            e03: self.e03 * other.s * other.s,
            e31: self.e31 * other.s * other.s,
            e013: self.e013 * other.s * other.s,
            e23: self.e23 * other.s * other.s,
            e032: self.e032 * other.s * other.s,
            e123: self.e123 * other.s * other.s,
            e0123: self.e0123 * other.s * other.s,
        }
    }
}

impl Transform<Vector> for Multivector {
    type Output = Multivector;
    fn transform(self, other: Vector) -> Multivector {
        Multivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e3 * other.e3,
            e0: -self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 - self.e0 * other.e2 * other.e2 - self.e0 * other.e3 * other.e3 + 2.0 * self.e1 * other.e0 * other.e1 + 2.0 * self.e2 * other.e0 * other.e2 + 2.0 * self.e3 * other.e0 * other.e3,
            e1: 2.0 * self.e0 * other.e0 * other.e1 - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1 - self.e1 * other.e2 * other.e2 - self.e1 * other.e3 * other.e3 + 2.0 * self.e2 * other.e1 * other.e2 + 2.0 * self.e3 * other.e1 * other.e3,
            e01: self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1 + self.e01 * other.e2 * other.e2 + self.e01 * other.e3 * other.e3 - 2.0 * self.e02 * other.e1 * other.e2 + 2.0 * self.e12 * other.e0 * other.e2 - 2.0 * self.e03 * other.e1 * other.e3 - 2.0 * self.e31 * other.e0 * other.e3,
            e2: 2.0 * self.e0 * other.e0 * other.e2 + 2.0 * self.e1 * other.e1 * other.e2 - self.e2 * other.e0 * other.e0 - self.e2 * other.e1 * other.e1 + self.e2 * other.e2 * other.e2 - self.e2 * other.e3 * other.e3 + 2.0 * self.e3 * other.e2 * other.e3,
            e02: -2.0 * self.e01 * other.e1 * other.e2 + self.e02 * other.e0 * other.e0 + self.e02 * other.e1 * other.e1 - self.e02 * other.e2 * other.e2 + self.e02 * other.e3 * other.e3 - 2.0 * self.e12 * other.e0 * other.e1 - 2.0 * self.e03 * other.e2 * other.e3 + 2.0 * self.e23 * other.e0 * other.e3,
            e12: 2.0 * self.e01 * other.e0 * other.e2 - 2.0 * self.e02 * other.e0 * other.e1 + self.e12 * other.e0 * other.e0 - self.e12 * other.e1 * other.e1 - self.e12 * other.e2 * other.e2 + self.e12 * other.e3 * other.e3 + 2.0 * self.e31 * other.e2 * other.e3 + 2.0 * self.e23 * other.e1 * other.e3,
            e021: -self.e021 * other.e0 * other.e0 + self.e021 * other.e1 * other.e1 + self.e021 * other.e2 * other.e2 - self.e021 * other.e3 * other.e3 - 2.0 * self.e013 * other.e2 * other.e3 - 2.0 * self.e032 * other.e1 * other.e3 - 2.0 * self.e123 * other.e0 * other.e3,
            e3: 2.0 * self.e0 * other.e0 * other.e3 + 2.0 * self.e1 * other.e1 * other.e3 + 2.0 * self.e2 * other.e2 * other.e3 - self.e3 * other.e0 * other.e0 - self.e3 * other.e1 * other.e1 - self.e3 * other.e2 * other.e2 + self.e3 * other.e3 * other.e3,
            e03: -2.0 * self.e01 * other.e1 * other.e3 - 2.0 * self.e02 * other.e2 * other.e3 + self.e03 * other.e0 * other.e0 + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 - self.e03 * other.e3 * other.e3 + 2.0 * self.e31 * other.e0 * other.e1 - 2.0 * self.e23 * other.e0 * other.e2,
            e31: -2.0 * self.e01 * other.e0 * other.e3 + 2.0 * self.e12 * other.e2 * other.e3 + 2.0 * self.e03 * other.e0 * other.e1 + self.e31 * other.e0 * other.e0 - self.e31 * other.e1 * other.e1 + self.e31 * other.e2 * other.e2 - self.e31 * other.e3 * other.e3 + 2.0 * self.e23 * other.e1 * other.e2,
            e013: -2.0 * self.e021 * other.e2 * other.e3 - self.e013 * other.e0 * other.e0 + self.e013 * other.e1 * other.e1 - self.e013 * other.e2 * other.e2 + self.e013 * other.e3 * other.e3 - 2.0 * self.e032 * other.e1 * other.e2 - 2.0 * self.e123 * other.e0 * other.e2,
            e23: 2.0 * self.e02 * other.e0 * other.e3 + 2.0 * self.e12 * other.e1 * other.e3 - 2.0 * self.e03 * other.e0 * other.e2 + 2.0 * self.e31 * other.e1 * other.e2 + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1 - self.e23 * other.e2 * other.e2 - self.e23 * other.e3 * other.e3,
            e032: -2.0 * self.e021 * other.e1 * other.e3 - 2.0 * self.e013 * other.e1 * other.e2 - self.e032 * other.e0 * other.e0 - self.e032 * other.e1 * other.e1 + self.e032 * other.e2 * other.e2 + self.e032 * other.e3 * other.e3 - 2.0 * self.e123 * other.e0 * other.e1,
            e123: -2.0 * self.e021 * other.e0 * other.e3 - 2.0 * self.e013 * other.e0 * other.e2 - 2.0 * self.e032 * other.e0 * other.e1 - self.e123 * other.e0 * other.e0 + self.e123 * other.e1 * other.e1 + self.e123 * other.e2 * other.e2 + self.e123 * other.e3 * other.e3,
            e0123: self.e0123 * other.e0 * other.e0 - self.e0123 * other.e1 * other.e1 - self.e0123 * other.e2 * other.e2 - self.e0123 * other.e3 * other.e3,
        }
    }
}

impl Transform<Bivector> for Multivector {
    type Output = Multivector;
    fn transform(self, other: Bivector) -> Multivector {
        Multivector {
            s: self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23 - 2.0 * self.e0123 * other.e01 * other.e23 - 2.0 * self.e0123 * other.e02 * other.e31 - 2.0 * self.e0123 * other.e12 * other.e03,
            e0: self.e0 * other.e01 * other.e01 + self.e0 * other.e02 * other.e02 + self.e0 * other.e12 * other.e12 + self.e0 * other.e03 * other.e03 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 - 2.0 * self.e1 * other.e02 * other.e12 + 2.0 * self.e1 * other.e03 * other.e31 + 2.0 * self.e2 * other.e01 * other.e12 - 2.0 * self.e2 * other.e03 * other.e23 - 2.0 * self.e3 * other.e01 * other.e31 + 2.0 * self.e3 * other.e02 * other.e23,
            e1: -2.0 * self.e0 * other.e02 * other.e12 + 2.0 * self.e0 * other.e03 * other.e31 - self.e1 * other.e01 * other.e01 + self.e1 * other.e02 * other.e02 - self.e1 * other.e12 * other.e12 + self.e1 * other.e03 * other.e03 - self.e1 * other.e31 * other.e31 + self.e1 * other.e23 * other.e23 - 2.0 * self.e2 * other.e01 * other.e02 + 2.0 * self.e2 * other.e31 * other.e23 - 2.0 * self.e3 * other.e01 * other.e03 + 2.0 * self.e3 * other.e12 * other.e23,
            e01: -self.e01 * other.e01 * other.e01 + self.e01 * other.e02 * other.e02 - self.e01 * other.e12 * other.e12 + self.e01 * other.e03 * other.e03 - self.e01 * other.e31 * other.e31 + self.e01 * other.e23 * other.e23 - 2.0 * self.e02 * other.e01 * other.e02 + 2.0 * self.e02 * other.e31 * other.e23 + 2.0 * self.e12 * other.e01 * other.e12 + 2.0 * self.e12 * other.e03 * other.e23 - 2.0 * self.e03 * other.e01 * other.e03 + 2.0 * self.e03 * other.e12 * other.e23 + 2.0 * self.e31 * other.e01 * other.e31 + 2.0 * self.e31 * other.e02 * other.e23 + 2.0 * self.e23 * other.e01 * other.e23 - 2.0 * self.e23 * other.e02 * other.e31 - 2.0 * self.e23 * other.e12 * other.e03,
            e2: 2.0 * self.e0 * other.e01 * other.e12 - 2.0 * self.e0 * other.e03 * other.e23 - 2.0 * self.e1 * other.e01 * other.e02 + 2.0 * self.e1 * other.e31 * other.e23 + self.e2 * other.e01 * other.e01 - self.e2 * other.e02 * other.e02 - self.e2 * other.e12 * other.e12 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e2 * other.e23 * other.e23 - 2.0 * self.e3 * other.e02 * other.e03 + 2.0 * self.e3 * other.e12 * other.e31,
            e02: -2.0 * self.e01 * other.e01 * other.e02 + 2.0 * self.e01 * other.e31 * other.e23 + self.e02 * other.e01 * other.e01 - self.e02 * other.e02 * other.e02 - self.e02 * other.e12 * other.e12 + self.e02 * other.e03 * other.e03 + self.e02 * other.e31 * other.e31 - self.e02 * other.e23 * other.e23 + 2.0 * self.e12 * other.e02 * other.e12 + 2.0 * self.e12 * other.e03 * other.e31 - 2.0 * self.e03 * other.e02 * other.e03 + 2.0 * self.e03 * other.e12 * other.e31 - 2.0 * self.e31 * other.e01 * other.e23 + 2.0 * self.e31 * other.e02 * other.e31 - 2.0 * self.e31 * other.e12 * other.e03 + 2.0 * self.e23 * other.e01 * other.e31 + 2.0 * self.e23 * other.e02 * other.e23,
            e12: 2.0 * self.e01 * other.e01 * other.e12 + 2.0 * self.e01 * other.e03 * other.e23 + 2.0 * self.e02 * other.e02 * other.e12 + 2.0 * self.e02 * other.e03 * other.e31 - self.e12 * other.e01 * other.e01 - self.e12 * other.e02 * other.e02 + self.e12 * other.e12 * other.e12 + self.e12 * other.e03 * other.e03 - self.e12 * other.e31 * other.e31 - self.e12 * other.e23 * other.e23 - 2.0 * self.e03 * other.e01 * other.e23 - 2.0 * self.e03 * other.e02 * other.e31 + 2.0 * self.e03 * other.e12 * other.e03 + 2.0 * self.e31 * other.e02 * other.e03 + 2.0 * self.e31 * other.e12 * other.e31 + 2.0 * self.e23 * other.e01 * other.e03 + 2.0 * self.e23 * other.e12 * other.e23,
            e021: -self.e021 * other.e01 * other.e01 - self.e021 * other.e02 * other.e02 + self.e021 * other.e12 * other.e12 + self.e021 * other.e03 * other.e03 - self.e021 * other.e31 * other.e31 - self.e021 * other.e23 * other.e23 + 2.0 * self.e013 * other.e02 * other.e03 + 2.0 * self.e013 * other.e12 * other.e31 + 2.0 * self.e032 * other.e01 * other.e03 + 2.0 * self.e032 * other.e12 * other.e23 - 2.0 * self.e123 * other.e01 * other.e31 + 2.0 * self.e123 * other.e02 * other.e23,
            e3: -2.0 * self.e0 * other.e01 * other.e31 + 2.0 * self.e0 * other.e02 * other.e23 - 2.0 * self.e1 * other.e01 * other.e03 + 2.0 * self.e1 * other.e12 * other.e23 - 2.0 * self.e2 * other.e02 * other.e03 + 2.0 * self.e2 * other.e12 * other.e31 + self.e3 * other.e01 * other.e01 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12 - self.e3 * other.e03 * other.e03 - self.e3 * other.e31 * other.e31 - self.e3 * other.e23 * other.e23,
            e03: -2.0 * self.e01 * other.e01 * other.e03 + 2.0 * self.e01 * other.e12 * other.e23 - 2.0 * self.e02 * other.e02 * other.e03 + 2.0 * self.e02 * other.e12 * other.e31 - 2.0 * self.e12 * other.e01 * other.e23 - 2.0 * self.e12 * other.e02 * other.e31 + 2.0 * self.e12 * other.e12 * other.e03 + self.e03 * other.e01 * other.e01 + self.e03 * other.e02 * other.e02 + self.e03 * other.e12 * other.e12 - self.e03 * other.e03 * other.e03 - self.e03 * other.e31 * other.e31 - self.e03 * other.e23 * other.e23 + 2.0 * self.e31 * other.e02 * other.e12 + 2.0 * self.e31 * other.e03 * other.e31 + 2.0 * self.e23 * other.e01 * other.e12 + 2.0 * self.e23 * other.e03 * other.e23,
            e31: 2.0 * self.e01 * other.e01 * other.e31 + 2.0 * self.e01 * other.e02 * other.e23 - 2.0 * self.e02 * other.e01 * other.e23 + 2.0 * self.e02 * other.e02 * other.e31 - 2.0 * self.e02 * other.e12 * other.e03 + 2.0 * self.e12 * other.e02 * other.e03 + 2.0 * self.e12 * other.e12 * other.e31 + 2.0 * self.e03 * other.e02 * other.e12 + 2.0 * self.e03 * other.e03 * other.e31 - self.e31 * other.e01 * other.e01 + self.e31 * other.e02 * other.e02 - self.e31 * other.e12 * other.e12 - self.e31 * other.e03 * other.e03 + self.e31 * other.e31 * other.e31 - self.e31 * other.e23 * other.e23 + 2.0 * self.e23 * other.e01 * other.e02 + 2.0 * self.e23 * other.e31 * other.e23,
            e013: 2.0 * self.e021 * other.e02 * other.e03 + 2.0 * self.e021 * other.e12 * other.e31 - self.e013 * other.e01 * other.e01 + self.e013 * other.e02 * other.e02 - self.e013 * other.e12 * other.e12 - self.e013 * other.e03 * other.e03 + self.e013 * other.e31 * other.e31 - self.e013 * other.e23 * other.e23 + 2.0 * self.e032 * other.e01 * other.e02 + 2.0 * self.e032 * other.e31 * other.e23 + 2.0 * self.e123 * other.e01 * other.e12 - 2.0 * self.e123 * other.e03 * other.e23,
            e23: 2.0 * self.e01 * other.e01 * other.e23 - 2.0 * self.e01 * other.e02 * other.e31 - 2.0 * self.e01 * other.e12 * other.e03 + 2.0 * self.e02 * other.e01 * other.e31 + 2.0 * self.e02 * other.e02 * other.e23 + 2.0 * self.e12 * other.e01 * other.e03 + 2.0 * self.e12 * other.e12 * other.e23 + 2.0 * self.e03 * other.e01 * other.e12 + 2.0 * self.e03 * other.e03 * other.e23 + 2.0 * self.e31 * other.e01 * other.e02 + 2.0 * self.e31 * other.e31 * other.e23 + self.e23 * other.e01 * other.e01 - self.e23 * other.e02 * other.e02 - self.e23 * other.e12 * other.e12 - self.e23 * other.e03 * other.e03 - self.e23 * other.e31 * other.e31 + self.e23 * other.e23 * other.e23,
            e032: 2.0 * self.e021 * other.e01 * other.e03 + 2.0 * self.e021 * other.e12 * other.e23 + 2.0 * self.e013 * other.e01 * other.e02 + 2.0 * self.e013 * other.e31 * other.e23 + self.e032 * other.e01 * other.e01 - self.e032 * other.e02 * other.e02 - self.e032 * other.e12 * other.e12 - self.e032 * other.e03 * other.e03 - self.e032 * other.e31 * other.e31 + self.e032 * other.e23 * other.e23 - 2.0 * self.e123 * other.e02 * other.e12 + 2.0 * self.e123 * other.e03 * other.e31,
            e123: -2.0 * self.e021 * other.e01 * other.e31 + 2.0 * self.e021 * other.e02 * other.e23 + 2.0 * self.e013 * other.e01 * other.e12 - 2.0 * self.e013 * other.e03 * other.e23 - 2.0 * self.e032 * other.e02 * other.e12 + 2.0 * self.e032 * other.e03 * other.e31 - self.e123 * other.e01 * other.e01 - self.e123 * other.e02 * other.e02 + self.e123 * other.e12 * other.e12 - self.e123 * other.e03 * other.e03 + self.e123 * other.e31 * other.e31 + self.e123 * other.e23 * other.e23,
            e0123: -2.0 * self.s * other.e01 * other.e23 - 2.0 * self.s * other.e02 * other.e31 - 2.0 * self.s * other.e12 * other.e03 - self.e0123 * other.e01 * other.e01 - self.e0123 * other.e02 * other.e02 + self.e0123 * other.e12 * other.e12 - self.e0123 * other.e03 * other.e03 + self.e0123 * other.e31 * other.e31 + self.e0123 * other.e23 * other.e23,
        }
    }
}

impl Transform<Trivector> for Multivector {
    type Output = Multivector;
    fn transform(self, other: Trivector) -> Multivector {
        Multivector {
            s: self.s * other.e021 * other.e021 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123,
            e0: -self.e0 * other.e021 * other.e021 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 - self.e0 * other.e123 * other.e123 - 2.0 * self.e1 * other.e032 * other.e123 - 2.0 * self.e2 * other.e013 * other.e123 - 2.0 * self.e3 * other.e021 * other.e123,
            e1: -2.0 * self.e0 * other.e032 * other.e123 + self.e1 * other.e021 * other.e021 + self.e1 * other.e013 * other.e013 - self.e1 * other.e032 * other.e032 + self.e1 * other.e123 * other.e123 - 2.0 * self.e2 * other.e013 * other.e032 - 2.0 * self.e3 * other.e021 * other.e032,
            e01: -self.e01 * other.e021 * other.e021 - self.e01 * other.e013 * other.e013 + self.e01 * other.e032 * other.e032 - self.e01 * other.e123 * other.e123 + 2.0 * self.e02 * other.e013 * other.e032 + 2.0 * self.e12 * other.e013 * other.e123 + 2.0 * self.e03 * other.e021 * other.e032 - 2.0 * self.e31 * other.e021 * other.e123,
            e2: -2.0 * self.e0 * other.e013 * other.e123 - 2.0 * self.e1 * other.e013 * other.e032 + self.e2 * other.e021 * other.e021 - self.e2 * other.e013 * other.e013 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 - 2.0 * self.e3 * other.e021 * other.e013,
            e02: 2.0 * self.e01 * other.e013 * other.e032 - self.e02 * other.e021 * other.e021 + self.e02 * other.e013 * other.e013 - self.e02 * other.e032 * other.e032 - self.e02 * other.e123 * other.e123 - 2.0 * self.e12 * other.e032 * other.e123 + 2.0 * self.e03 * other.e021 * other.e013 + 2.0 * self.e23 * other.e021 * other.e123,
            e12: 2.0 * self.e01 * other.e013 * other.e123 - 2.0 * self.e02 * other.e032 * other.e123 + self.e12 * other.e021 * other.e021 - self.e12 * other.e013 * other.e013 - self.e12 * other.e032 * other.e032 + self.e12 * other.e123 * other.e123 + 2.0 * self.e31 * other.e021 * other.e013 + 2.0 * self.e23 * other.e021 * other.e032,
            e021: -self.e021 * other.e021 * other.e021 + self.e021 * other.e013 * other.e013 + self.e021 * other.e032 * other.e032 - self.e021 * other.e123 * other.e123 - 2.0 * self.e013 * other.e021 * other.e013 - 2.0 * self.e032 * other.e021 * other.e032 + 2.0 * self.e123 * other.e021 * other.e123,
            e3: -2.0 * self.e0 * other.e021 * other.e123 - 2.0 * self.e1 * other.e021 * other.e032 - 2.0 * self.e2 * other.e021 * other.e013 - self.e3 * other.e021 * other.e021 + self.e3 * other.e013 * other.e013 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123,
            e03: 2.0 * self.e01 * other.e021 * other.e032 + 2.0 * self.e02 * other.e021 * other.e013 + self.e03 * other.e021 * other.e021 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 - self.e03 * other.e123 * other.e123 + 2.0 * self.e31 * other.e032 * other.e123 - 2.0 * self.e23 * other.e013 * other.e123,
            e31: -2.0 * self.e01 * other.e021 * other.e123 + 2.0 * self.e12 * other.e021 * other.e013 + 2.0 * self.e03 * other.e032 * other.e123 - self.e31 * other.e021 * other.e021 + self.e31 * other.e013 * other.e013 - self.e31 * other.e032 * other.e032 + self.e31 * other.e123 * other.e123 + 2.0 * self.e23 * other.e013 * other.e032,
            e013: -2.0 * self.e021 * other.e021 * other.e013 + self.e013 * other.e021 * other.e021 - self.e013 * other.e013 * other.e013 + self.e013 * other.e032 * other.e032 - self.e013 * other.e123 * other.e123 - 2.0 * self.e032 * other.e013 * other.e032 + 2.0 * self.e123 * other.e013 * other.e123,
            e23: 2.0 * self.e02 * other.e021 * other.e123 + 2.0 * self.e12 * other.e021 * other.e032 - 2.0 * self.e03 * other.e013 * other.e123 + 2.0 * self.e31 * other.e013 * other.e032 - self.e23 * other.e021 * other.e021 - self.e23 * other.e013 * other.e013 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123,
            e032: -2.0 * self.e021 * other.e021 * other.e032 - 2.0 * self.e013 * other.e013 * other.e032 + self.e032 * other.e021 * other.e021 + self.e032 * other.e013 * other.e013 - self.e032 * other.e032 * other.e032 - self.e032 * other.e123 * other.e123 + 2.0 * self.e123 * other.e032 * other.e123,
            e123: 2.0 * self.e021 * other.e021 * other.e123 + 2.0 * self.e013 * other.e013 * other.e123 + 2.0 * self.e032 * other.e032 * other.e123 - self.e123 * other.e021 * other.e021 - self.e123 * other.e013 * other.e013 - self.e123 * other.e032 * other.e032 + self.e123 * other.e123 * other.e123,
            e0123: self.e0123 * other.e021 * other.e021 + self.e0123 * other.e013 * other.e013 + self.e0123 * other.e032 * other.e032 - self.e0123 * other.e123 * other.e123,
        }
    }
}

impl Transform<FourVector> for Multivector {
    type Output = Multivector;
    fn transform(self, other: FourVector) -> Multivector {
        Multivector {
            s: self.s * other.e0123 * other.e0123,
            e0: self.e0 * other.e0123 * other.e0123,
            e1: -self.e1 * other.e0123 * other.e0123,
            e01: -self.e01 * other.e0123 * other.e0123,
            e2: -self.e2 * other.e0123 * other.e0123,
            e02: -self.e02 * other.e0123 * other.e0123,
            e12: self.e12 * other.e0123 * other.e0123,
            e021: self.e021 * other.e0123 * other.e0123,
            e3: -self.e3 * other.e0123 * other.e0123,
            e03: -self.e03 * other.e0123 * other.e0123,
            e31: self.e31 * other.e0123 * other.e0123,
            e013: self.e013 * other.e0123 * other.e0123,
            e23: self.e23 * other.e0123 * other.e0123,
            e032: self.e032 * other.e0123 * other.e0123,
            e123: -self.e123 * other.e0123 * other.e0123,
            e0123: -self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Transform<Null> for Multivector {
    type Output = Null;
    fn transform(self, other: Null) -> Null {
        Null
    }
}

impl Transform<OddMultivector> for Multivector {
    type Output = Multivector;
    fn transform(self, other: OddMultivector) -> Multivector {
        Multivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 + 2.0 * self.e0123 * other.e0 * other.e123 + 2.0 * self.e0123 * other.e1 * other.e032 + 2.0 * self.e0123 * other.e2 * other.e013 + 2.0 * self.e0123 * other.e021 * other.e3,
            e0: -self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 - self.e0 * other.e2 * other.e2 - self.e0 * other.e021 * other.e021 - self.e0 * other.e3 * other.e3 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 - self.e0 * other.e123 * other.e123 + 2.0 * self.e1 * other.e0 * other.e1 + 2.0 * self.e1 * other.e2 * other.e021 - 2.0 * self.e1 * other.e3 * other.e013 - 2.0 * self.e1 * other.e032 * other.e123 + 2.0 * self.e2 * other.e0 * other.e2 - 2.0 * self.e2 * other.e1 * other.e021 + 2.0 * self.e2 * other.e3 * other.e032 - 2.0 * self.e2 * other.e013 * other.e123 + 2.0 * self.e3 * other.e0 * other.e3 + 2.0 * self.e3 * other.e1 * other.e013 - 2.0 * self.e3 * other.e2 * other.e032 - 2.0 * self.e3 * other.e021 * other.e123,
            e1: 2.0 * self.e0 * other.e0 * other.e1 - 2.0 * self.e0 * other.e2 * other.e021 + 2.0 * self.e0 * other.e3 * other.e013 - 2.0 * self.e0 * other.e032 * other.e123 - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1 - self.e1 * other.e2 * other.e2 + self.e1 * other.e021 * other.e021 - self.e1 * other.e3 * other.e3 + self.e1 * other.e013 * other.e013 - self.e1 * other.e032 * other.e032 + self.e1 * other.e123 * other.e123 + 2.0 * self.e2 * other.e0 * other.e021 + 2.0 * self.e2 * other.e1 * other.e2 - 2.0 * self.e2 * other.e3 * other.e123 - 2.0 * self.e2 * other.e013 * other.e032 - 2.0 * self.e3 * other.e0 * other.e013 + 2.0 * self.e3 * other.e1 * other.e3 + 2.0 * self.e3 * other.e2 * other.e123 - 2.0 * self.e3 * other.e021 * other.e032,
            e01: self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1 + self.e01 * other.e2 * other.e2 - self.e01 * other.e021 * other.e021 + self.e01 * other.e3 * other.e3 - self.e01 * other.e013 * other.e013 + self.e01 * other.e032 * other.e032 - self.e01 * other.e123 * other.e123 - 2.0 * self.e02 * other.e0 * other.e021 - 2.0 * self.e02 * other.e1 * other.e2 + 2.0 * self.e02 * other.e3 * other.e123 + 2.0 * self.e02 * other.e013 * other.e032 + 2.0 * self.e12 * other.e0 * other.e2 + 2.0 * self.e12 * other.e1 * other.e021 + 2.0 * self.e12 * other.e3 * other.e032 + 2.0 * self.e12 * other.e013 * other.e123 + 2.0 * self.e03 * other.e0 * other.e013 - 2.0 * self.e03 * other.e1 * other.e3 - 2.0 * self.e03 * other.e2 * other.e123 + 2.0 * self.e03 * other.e021 * other.e032 - 2.0 * self.e31 * other.e0 * other.e3 + 2.0 * self.e31 * other.e1 * other.e013 + 2.0 * self.e31 * other.e2 * other.e032 - 2.0 * self.e31 * other.e021 * other.e123 + 2.0 * self.e23 * other.e0 * other.e123 + 2.0 * self.e23 * other.e1 * other.e032 - 2.0 * self.e23 * other.e2 * other.e013 - 2.0 * self.e23 * other.e021 * other.e3,
            e2: 2.0 * self.e0 * other.e0 * other.e2 + 2.0 * self.e0 * other.e1 * other.e021 - 2.0 * self.e0 * other.e3 * other.e032 - 2.0 * self.e0 * other.e013 * other.e123 - 2.0 * self.e1 * other.e0 * other.e021 + 2.0 * self.e1 * other.e1 * other.e2 + 2.0 * self.e1 * other.e3 * other.e123 - 2.0 * self.e1 * other.e013 * other.e032 - self.e2 * other.e0 * other.e0 - self.e2 * other.e1 * other.e1 + self.e2 * other.e2 * other.e2 + self.e2 * other.e021 * other.e021 - self.e2 * other.e3 * other.e3 - self.e2 * other.e013 * other.e013 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 + 2.0 * self.e3 * other.e0 * other.e032 - 2.0 * self.e3 * other.e1 * other.e123 + 2.0 * self.e3 * other.e2 * other.e3 - 2.0 * self.e3 * other.e021 * other.e013,
            e02: 2.0 * self.e01 * other.e0 * other.e021 - 2.0 * self.e01 * other.e1 * other.e2 - 2.0 * self.e01 * other.e3 * other.e123 + 2.0 * self.e01 * other.e013 * other.e032 + self.e02 * other.e0 * other.e0 + self.e02 * other.e1 * other.e1 - self.e02 * other.e2 * other.e2 - self.e02 * other.e021 * other.e021 + self.e02 * other.e3 * other.e3 + self.e02 * other.e013 * other.e013 - self.e02 * other.e032 * other.e032 - self.e02 * other.e123 * other.e123 - 2.0 * self.e12 * other.e0 * other.e1 + 2.0 * self.e12 * other.e2 * other.e021 + 2.0 * self.e12 * other.e3 * other.e013 - 2.0 * self.e12 * other.e032 * other.e123 - 2.0 * self.e03 * other.e0 * other.e032 + 2.0 * self.e03 * other.e1 * other.e123 - 2.0 * self.e03 * other.e2 * other.e3 + 2.0 * self.e03 * other.e021 * other.e013 + 2.0 * self.e31 * other.e0 * other.e123 - 2.0 * self.e31 * other.e1 * other.e032 + 2.0 * self.e31 * other.e2 * other.e013 - 2.0 * self.e31 * other.e021 * other.e3 + 2.0 * self.e23 * other.e0 * other.e3 + 2.0 * self.e23 * other.e1 * other.e013 + 2.0 * self.e23 * other.e2 * other.e032 + 2.0 * self.e23 * other.e021 * other.e123,
            e12: 2.0 * self.e01 * other.e0 * other.e2 - 2.0 * self.e01 * other.e1 * other.e021 - 2.0 * self.e01 * other.e3 * other.e032 + 2.0 * self.e01 * other.e013 * other.e123 - 2.0 * self.e02 * other.e0 * other.e1 - 2.0 * self.e02 * other.e2 * other.e021 - 2.0 * self.e02 * other.e3 * other.e013 - 2.0 * self.e02 * other.e032 * other.e123 + self.e12 * other.e0 * other.e0 - self.e12 * other.e1 * other.e1 - self.e12 * other.e2 * other.e2 + self.e12 * other.e021 * other.e021 + self.e12 * other.e3 * other.e3 - self.e12 * other.e013 * other.e013 - self.e12 * other.e032 * other.e032 + self.e12 * other.e123 * other.e123 - 2.0 * self.e03 * other.e0 * other.e123 + 2.0 * self.e03 * other.e1 * other.e032 + 2.0 * self.e03 * other.e2 * other.e013 - 2.0 * self.e03 * other.e021 * other.e3 + 2.0 * self.e31 * other.e0 * other.e032 + 2.0 * self.e31 * other.e1 * other.e123 + 2.0 * self.e31 * other.e2 * other.e3 + 2.0 * self.e31 * other.e021 * other.e013 - 2.0 * self.e23 * other.e0 * other.e013 + 2.0 * self.e23 * other.e1 * other.e3 - 2.0 * self.e23 * other.e2 * other.e123 + 2.0 * self.e23 * other.e021 * other.e032,
            e021: -self.e021 * other.e0 * other.e0 + self.e021 * other.e1 * other.e1 + self.e021 * other.e2 * other.e2 - self.e021 * other.e021 * other.e021 - self.e021 * other.e3 * other.e3 + self.e021 * other.e013 * other.e013 + self.e021 * other.e032 * other.e032 - self.e021 * other.e123 * other.e123 - 2.0 * self.e013 * other.e0 * other.e032 - 2.0 * self.e013 * other.e1 * other.e123 - 2.0 * self.e013 * other.e2 * other.e3 - 2.0 * self.e013 * other.e021 * other.e013 + 2.0 * self.e032 * other.e0 * other.e013 - 2.0 * self.e032 * other.e1 * other.e3 + 2.0 * self.e032 * other.e2 * other.e123 - 2.0 * self.e032 * other.e021 * other.e032 - 2.0 * self.e123 * other.e0 * other.e3 + 2.0 * self.e123 * other.e1 * other.e013 - 2.0 * self.e123 * other.e2 * other.e032 + 2.0 * self.e123 * other.e021 * other.e123,
            e3: 2.0 * self.e0 * other.e0 * other.e3 - 2.0 * self.e0 * other.e1 * other.e013 + 2.0 * self.e0 * other.e2 * other.e032 - 2.0 * self.e0 * other.e021 * other.e123 + 2.0 * self.e1 * other.e0 * other.e013 + 2.0 * self.e1 * other.e1 * other.e3 - 2.0 * self.e1 * other.e2 * other.e123 - 2.0 * self.e1 * other.e021 * other.e032 - 2.0 * self.e2 * other.e0 * other.e032 + 2.0 * self.e2 * other.e1 * other.e123 + 2.0 * self.e2 * other.e2 * other.e3 - 2.0 * self.e2 * other.e021 * other.e013 - self.e3 * other.e0 * other.e0 - self.e3 * other.e1 * other.e1 - self.e3 * other.e2 * other.e2 - self.e3 * other.e021 * other.e021 + self.e3 * other.e3 * other.e3 + self.e3 * other.e013 * other.e013 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123,
            e03: -2.0 * self.e01 * other.e0 * other.e013 - 2.0 * self.e01 * other.e1 * other.e3 + 2.0 * self.e01 * other.e2 * other.e123 + 2.0 * self.e01 * other.e021 * other.e032 + 2.0 * self.e02 * other.e0 * other.e032 - 2.0 * self.e02 * other.e1 * other.e123 - 2.0 * self.e02 * other.e2 * other.e3 + 2.0 * self.e02 * other.e021 * other.e013 + 2.0 * self.e12 * other.e0 * other.e123 - 2.0 * self.e12 * other.e1 * other.e032 - 2.0 * self.e12 * other.e2 * other.e013 + 2.0 * self.e12 * other.e021 * other.e3 + self.e03 * other.e0 * other.e0 + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 + self.e03 * other.e021 * other.e021 - self.e03 * other.e3 * other.e3 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 - self.e03 * other.e123 * other.e123 + 2.0 * self.e31 * other.e0 * other.e1 + 2.0 * self.e31 * other.e2 * other.e021 + 2.0 * self.e31 * other.e3 * other.e013 + 2.0 * self.e31 * other.e032 * other.e123 - 2.0 * self.e23 * other.e0 * other.e2 + 2.0 * self.e23 * other.e1 * other.e021 + 2.0 * self.e23 * other.e3 * other.e032 - 2.0 * self.e23 * other.e013 * other.e123,
            e31: -2.0 * self.e01 * other.e0 * other.e3 - 2.0 * self.e01 * other.e1 * other.e013 - 2.0 * self.e01 * other.e2 * other.e032 - 2.0 * self.e01 * other.e021 * other.e123 - 2.0 * self.e02 * other.e0 * other.e123 + 2.0 * self.e02 * other.e1 * other.e032 - 2.0 * self.e02 * other.e2 * other.e013 + 2.0 * self.e02 * other.e021 * other.e3 - 2.0 * self.e12 * other.e0 * other.e032 - 2.0 * self.e12 * other.e1 * other.e123 + 2.0 * self.e12 * other.e2 * other.e3 + 2.0 * self.e12 * other.e021 * other.e013 + 2.0 * self.e03 * other.e0 * other.e1 - 2.0 * self.e03 * other.e2 * other.e021 - 2.0 * self.e03 * other.e3 * other.e013 + 2.0 * self.e03 * other.e032 * other.e123 + self.e31 * other.e0 * other.e0 - self.e31 * other.e1 * other.e1 + self.e31 * other.e2 * other.e2 - self.e31 * other.e021 * other.e021 - self.e31 * other.e3 * other.e3 + self.e31 * other.e013 * other.e013 - self.e31 * other.e032 * other.e032 + self.e31 * other.e123 * other.e123 + 2.0 * self.e23 * other.e0 * other.e021 + 2.0 * self.e23 * other.e1 * other.e2 + 2.0 * self.e23 * other.e3 * other.e123 + 2.0 * self.e23 * other.e013 * other.e032,
            e013: 2.0 * self.e021 * other.e0 * other.e032 + 2.0 * self.e021 * other.e1 * other.e123 - 2.0 * self.e021 * other.e2 * other.e3 - 2.0 * self.e021 * other.e021 * other.e013 - self.e013 * other.e0 * other.e0 + self.e013 * other.e1 * other.e1 - self.e013 * other.e2 * other.e2 + self.e013 * other.e021 * other.e021 + self.e013 * other.e3 * other.e3 - self.e013 * other.e013 * other.e013 + self.e013 * other.e032 * other.e032 - self.e013 * other.e123 * other.e123 - 2.0 * self.e032 * other.e0 * other.e021 - 2.0 * self.e032 * other.e1 * other.e2 - 2.0 * self.e032 * other.e3 * other.e123 - 2.0 * self.e032 * other.e013 * other.e032 - 2.0 * self.e123 * other.e0 * other.e2 - 2.0 * self.e123 * other.e1 * other.e021 + 2.0 * self.e123 * other.e3 * other.e032 + 2.0 * self.e123 * other.e013 * other.e123,
            e23: -2.0 * self.e01 * other.e0 * other.e123 - 2.0 * self.e01 * other.e1 * other.e032 + 2.0 * self.e01 * other.e2 * other.e013 + 2.0 * self.e01 * other.e021 * other.e3 + 2.0 * self.e02 * other.e0 * other.e3 - 2.0 * self.e02 * other.e1 * other.e013 - 2.0 * self.e02 * other.e2 * other.e032 + 2.0 * self.e02 * other.e021 * other.e123 + 2.0 * self.e12 * other.e0 * other.e013 + 2.0 * self.e12 * other.e1 * other.e3 + 2.0 * self.e12 * other.e2 * other.e123 + 2.0 * self.e12 * other.e021 * other.e032 - 2.0 * self.e03 * other.e0 * other.e2 - 2.0 * self.e03 * other.e1 * other.e021 - 2.0 * self.e03 * other.e3 * other.e032 - 2.0 * self.e03 * other.e013 * other.e123 - 2.0 * self.e31 * other.e0 * other.e021 + 2.0 * self.e31 * other.e1 * other.e2 - 2.0 * self.e31 * other.e3 * other.e123 + 2.0 * self.e31 * other.e013 * other.e032 + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1 - self.e23 * other.e2 * other.e2 - self.e23 * other.e021 * other.e021 - self.e23 * other.e3 * other.e3 - self.e23 * other.e013 * other.e013 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123,
            e032: -2.0 * self.e021 * other.e0 * other.e013 - 2.0 * self.e021 * other.e1 * other.e3 - 2.0 * self.e021 * other.e2 * other.e123 - 2.0 * self.e021 * other.e021 * other.e032 + 2.0 * self.e013 * other.e0 * other.e021 - 2.0 * self.e013 * other.e1 * other.e2 + 2.0 * self.e013 * other.e3 * other.e123 - 2.0 * self.e013 * other.e013 * other.e032 - self.e032 * other.e0 * other.e0 - self.e032 * other.e1 * other.e1 + self.e032 * other.e2 * other.e2 + self.e032 * other.e021 * other.e021 + self.e032 * other.e3 * other.e3 + self.e032 * other.e013 * other.e013 - self.e032 * other.e032 * other.e032 - self.e032 * other.e123 * other.e123 - 2.0 * self.e123 * other.e0 * other.e1 + 2.0 * self.e123 * other.e2 * other.e021 - 2.0 * self.e123 * other.e3 * other.e013 + 2.0 * self.e123 * other.e032 * other.e123,
            e123: -2.0 * self.e021 * other.e0 * other.e3 - 2.0 * self.e021 * other.e1 * other.e013 + 2.0 * self.e021 * other.e2 * other.e032 + 2.0 * self.e021 * other.e021 * other.e123 - 2.0 * self.e013 * other.e0 * other.e2 + 2.0 * self.e013 * other.e1 * other.e021 - 2.0 * self.e013 * other.e3 * other.e032 + 2.0 * self.e013 * other.e013 * other.e123 - 2.0 * self.e032 * other.e0 * other.e1 - 2.0 * self.e032 * other.e2 * other.e021 + 2.0 * self.e032 * other.e3 * other.e013 + 2.0 * self.e032 * other.e032 * other.e123 - self.e123 * other.e0 * other.e0 + self.e123 * other.e1 * other.e1 + self.e123 * other.e2 * other.e2 - self.e123 * other.e021 * other.e021 + self.e123 * other.e3 * other.e3 - self.e123 * other.e013 * other.e013 - self.e123 * other.e032 * other.e032 + self.e123 * other.e123 * other.e123,
            e0123: -2.0 * self.s * other.e0 * other.e123 - 2.0 * self.s * other.e1 * other.e032 - 2.0 * self.s * other.e2 * other.e013 - 2.0 * self.s * other.e021 * other.e3 + self.e0123 * other.e0 * other.e0 - self.e0123 * other.e1 * other.e1 - self.e0123 * other.e2 * other.e2 + self.e0123 * other.e021 * other.e021 - self.e0123 * other.e3 * other.e3 + self.e0123 * other.e013 * other.e013 + self.e0123 * other.e032 * other.e032 - self.e0123 * other.e123 * other.e123,
        }
    }
}

impl Transform<EvenMultivector> for Multivector {
    type Output = Multivector;
    fn transform(self, other: EvenMultivector) -> Multivector {
        Multivector {
            s: self.s * other.s * other.s + self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23 + self.s * other.e0123 * other.e0123 + 2.0 * self.e0123 * other.s * other.e0123 - 2.0 * self.e0123 * other.e01 * other.e23 - 2.0 * self.e0123 * other.e02 * other.e31 - 2.0 * self.e0123 * other.e12 * other.e03,
            e0: self.e0 * other.s * other.s + self.e0 * other.e01 * other.e01 + self.e0 * other.e02 * other.e02 + self.e0 * other.e12 * other.e12 + self.e0 * other.e03 * other.e03 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 + self.e0 * other.e0123 * other.e0123 + 2.0 * self.e1 * other.s * other.e01 - 2.0 * self.e1 * other.e02 * other.e12 + 2.0 * self.e1 * other.e03 * other.e31 + 2.0 * self.e1 * other.e23 * other.e0123 + 2.0 * self.e2 * other.s * other.e02 + 2.0 * self.e2 * other.e01 * other.e12 - 2.0 * self.e2 * other.e03 * other.e23 + 2.0 * self.e2 * other.e31 * other.e0123 + 2.0 * self.e3 * other.s * other.e03 - 2.0 * self.e3 * other.e01 * other.e31 + 2.0 * self.e3 * other.e02 * other.e23 + 2.0 * self.e3 * other.e12 * other.e0123,
            e1: -2.0 * self.e0 * other.s * other.e01 - 2.0 * self.e0 * other.e02 * other.e12 + 2.0 * self.e0 * other.e03 * other.e31 - 2.0 * self.e0 * other.e23 * other.e0123 + self.e1 * other.s * other.s - self.e1 * other.e01 * other.e01 + self.e1 * other.e02 * other.e02 - self.e1 * other.e12 * other.e12 + self.e1 * other.e03 * other.e03 - self.e1 * other.e31 * other.e31 + self.e1 * other.e23 * other.e23 - self.e1 * other.e0123 * other.e0123 + 2.0 * self.e2 * other.s * other.e12 - 2.0 * self.e2 * other.e01 * other.e02 + 2.0 * self.e2 * other.e03 * other.e0123 + 2.0 * self.e2 * other.e31 * other.e23 - 2.0 * self.e3 * other.s * other.e31 - 2.0 * self.e3 * other.e01 * other.e03 - 2.0 * self.e3 * other.e02 * other.e0123 + 2.0 * self.e3 * other.e12 * other.e23,
            e01: self.e01 * other.s * other.s - self.e01 * other.e01 * other.e01 + self.e01 * other.e02 * other.e02 - self.e01 * other.e12 * other.e12 + self.e01 * other.e03 * other.e03 - self.e01 * other.e31 * other.e31 + self.e01 * other.e23 * other.e23 - self.e01 * other.e0123 * other.e0123 + 2.0 * self.e02 * other.s * other.e12 - 2.0 * self.e02 * other.e01 * other.e02 + 2.0 * self.e02 * other.e03 * other.e0123 + 2.0 * self.e02 * other.e31 * other.e23 - 2.0 * self.e12 * other.s * other.e02 + 2.0 * self.e12 * other.e01 * other.e12 + 2.0 * self.e12 * other.e03 * other.e23 + 2.0 * self.e12 * other.e31 * other.e0123 - 2.0 * self.e03 * other.s * other.e31 - 2.0 * self.e03 * other.e01 * other.e03 - 2.0 * self.e03 * other.e02 * other.e0123 + 2.0 * self.e03 * other.e12 * other.e23 + 2.0 * self.e31 * other.s * other.e03 + 2.0 * self.e31 * other.e01 * other.e31 + 2.0 * self.e31 * other.e02 * other.e23 - 2.0 * self.e31 * other.e12 * other.e0123 - 2.0 * self.e23 * other.s * other.e0123 + 2.0 * self.e23 * other.e01 * other.e23 - 2.0 * self.e23 * other.e02 * other.e31 - 2.0 * self.e23 * other.e12 * other.e03,
            e2: -2.0 * self.e0 * other.s * other.e02 + 2.0 * self.e0 * other.e01 * other.e12 - 2.0 * self.e0 * other.e03 * other.e23 - 2.0 * self.e0 * other.e31 * other.e0123 - 2.0 * self.e1 * other.s * other.e12 - 2.0 * self.e1 * other.e01 * other.e02 - 2.0 * self.e1 * other.e03 * other.e0123 + 2.0 * self.e1 * other.e31 * other.e23 + self.e2 * other.s * other.s + self.e2 * other.e01 * other.e01 - self.e2 * other.e02 * other.e02 - self.e2 * other.e12 * other.e12 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e2 * other.e23 * other.e23 - self.e2 * other.e0123 * other.e0123 + 2.0 * self.e3 * other.s * other.e23 + 2.0 * self.e3 * other.e01 * other.e0123 - 2.0 * self.e3 * other.e02 * other.e03 + 2.0 * self.e3 * other.e12 * other.e31,
            e02: -2.0 * self.e01 * other.s * other.e12 - 2.0 * self.e01 * other.e01 * other.e02 - 2.0 * self.e01 * other.e03 * other.e0123 + 2.0 * self.e01 * other.e31 * other.e23 + self.e02 * other.s * other.s + self.e02 * other.e01 * other.e01 - self.e02 * other.e02 * other.e02 - self.e02 * other.e12 * other.e12 + self.e02 * other.e03 * other.e03 + self.e02 * other.e31 * other.e31 - self.e02 * other.e23 * other.e23 - self.e02 * other.e0123 * other.e0123 + 2.0 * self.e12 * other.s * other.e01 + 2.0 * self.e12 * other.e02 * other.e12 + 2.0 * self.e12 * other.e03 * other.e31 - 2.0 * self.e12 * other.e23 * other.e0123 + 2.0 * self.e03 * other.s * other.e23 + 2.0 * self.e03 * other.e01 * other.e0123 - 2.0 * self.e03 * other.e02 * other.e03 + 2.0 * self.e03 * other.e12 * other.e31 - 2.0 * self.e31 * other.s * other.e0123 - 2.0 * self.e31 * other.e01 * other.e23 + 2.0 * self.e31 * other.e02 * other.e31 - 2.0 * self.e31 * other.e12 * other.e03 - 2.0 * self.e23 * other.s * other.e03 + 2.0 * self.e23 * other.e01 * other.e31 + 2.0 * self.e23 * other.e02 * other.e23 + 2.0 * self.e23 * other.e12 * other.e0123,
            e12: 2.0 * self.e01 * other.s * other.e02 + 2.0 * self.e01 * other.e01 * other.e12 + 2.0 * self.e01 * other.e03 * other.e23 - 2.0 * self.e01 * other.e31 * other.e0123 - 2.0 * self.e02 * other.s * other.e01 + 2.0 * self.e02 * other.e02 * other.e12 + 2.0 * self.e02 * other.e03 * other.e31 + 2.0 * self.e02 * other.e23 * other.e0123 + self.e12 * other.s * other.s - self.e12 * other.e01 * other.e01 - self.e12 * other.e02 * other.e02 + self.e12 * other.e12 * other.e12 + self.e12 * other.e03 * other.e03 - self.e12 * other.e31 * other.e31 - self.e12 * other.e23 * other.e23 + self.e12 * other.e0123 * other.e0123 - 2.0 * self.e03 * other.s * other.e0123 - 2.0 * self.e03 * other.e01 * other.e23 - 2.0 * self.e03 * other.e02 * other.e31 + 2.0 * self.e03 * other.e12 * other.e03 - 2.0 * self.e31 * other.s * other.e23 + 2.0 * self.e31 * other.e01 * other.e0123 + 2.0 * self.e31 * other.e02 * other.e03 + 2.0 * self.e31 * other.e12 * other.e31 + 2.0 * self.e23 * other.s * other.e31 + 2.0 * self.e23 * other.e01 * other.e03 - 2.0 * self.e23 * other.e02 * other.e0123 + 2.0 * self.e23 * other.e12 * other.e23,
            e021: self.e021 * other.s * other.s - self.e021 * other.e01 * other.e01 - self.e021 * other.e02 * other.e02 + self.e021 * other.e12 * other.e12 + self.e021 * other.e03 * other.e03 - self.e021 * other.e31 * other.e31 - self.e021 * other.e23 * other.e23 + self.e021 * other.e0123 * other.e0123 - 2.0 * self.e013 * other.s * other.e23 + 2.0 * self.e013 * other.e01 * other.e0123 + 2.0 * self.e013 * other.e02 * other.e03 + 2.0 * self.e013 * other.e12 * other.e31 + 2.0 * self.e032 * other.s * other.e31 + 2.0 * self.e032 * other.e01 * other.e03 - 2.0 * self.e032 * other.e02 * other.e0123 + 2.0 * self.e032 * other.e12 * other.e23 - 2.0 * self.e123 * other.s * other.e03 - 2.0 * self.e123 * other.e01 * other.e31 + 2.0 * self.e123 * other.e02 * other.e23 - 2.0 * self.e123 * other.e12 * other.e0123,
            e3: -2.0 * self.e0 * other.s * other.e03 - 2.0 * self.e0 * other.e01 * other.e31 + 2.0 * self.e0 * other.e02 * other.e23 - 2.0 * self.e0 * other.e12 * other.e0123 + 2.0 * self.e1 * other.s * other.e31 - 2.0 * self.e1 * other.e01 * other.e03 + 2.0 * self.e1 * other.e02 * other.e0123 + 2.0 * self.e1 * other.e12 * other.e23 - 2.0 * self.e2 * other.s * other.e23 - 2.0 * self.e2 * other.e01 * other.e0123 - 2.0 * self.e2 * other.e02 * other.e03 + 2.0 * self.e2 * other.e12 * other.e31 + self.e3 * other.s * other.s + self.e3 * other.e01 * other.e01 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12 - self.e3 * other.e03 * other.e03 - self.e3 * other.e31 * other.e31 - self.e3 * other.e23 * other.e23 - self.e3 * other.e0123 * other.e0123,
            e03: 2.0 * self.e01 * other.s * other.e31 - 2.0 * self.e01 * other.e01 * other.e03 + 2.0 * self.e01 * other.e02 * other.e0123 + 2.0 * self.e01 * other.e12 * other.e23 - 2.0 * self.e02 * other.s * other.e23 - 2.0 * self.e02 * other.e01 * other.e0123 - 2.0 * self.e02 * other.e02 * other.e03 + 2.0 * self.e02 * other.e12 * other.e31 - 2.0 * self.e12 * other.s * other.e0123 - 2.0 * self.e12 * other.e01 * other.e23 - 2.0 * self.e12 * other.e02 * other.e31 + 2.0 * self.e12 * other.e12 * other.e03 + self.e03 * other.s * other.s + self.e03 * other.e01 * other.e01 + self.e03 * other.e02 * other.e02 + self.e03 * other.e12 * other.e12 - self.e03 * other.e03 * other.e03 - self.e03 * other.e31 * other.e31 - self.e03 * other.e23 * other.e23 - self.e03 * other.e0123 * other.e0123 - 2.0 * self.e31 * other.s * other.e01 + 2.0 * self.e31 * other.e02 * other.e12 + 2.0 * self.e31 * other.e03 * other.e31 + 2.0 * self.e31 * other.e23 * other.e0123 + 2.0 * self.e23 * other.s * other.e02 + 2.0 * self.e23 * other.e01 * other.e12 + 2.0 * self.e23 * other.e03 * other.e23 - 2.0 * self.e23 * other.e31 * other.e0123,
            e31: -2.0 * self.e01 * other.s * other.e03 + 2.0 * self.e01 * other.e01 * other.e31 + 2.0 * self.e01 * other.e02 * other.e23 + 2.0 * self.e01 * other.e12 * other.e0123 - 2.0 * self.e02 * other.s * other.e0123 - 2.0 * self.e02 * other.e01 * other.e23 + 2.0 * self.e02 * other.e02 * other.e31 - 2.0 * self.e02 * other.e12 * other.e03 + 2.0 * self.e12 * other.s * other.e23 - 2.0 * self.e12 * other.e01 * other.e0123 + 2.0 * self.e12 * other.e02 * other.e03 + 2.0 * self.e12 * other.e12 * other.e31 + 2.0 * self.e03 * other.s * other.e01 + 2.0 * self.e03 * other.e02 * other.e12 + 2.0 * self.e03 * other.e03 * other.e31 - 2.0 * self.e03 * other.e23 * other.e0123 + self.e31 * other.s * other.s - self.e31 * other.e01 * other.e01 + self.e31 * other.e02 * other.e02 - self.e31 * other.e12 * other.e12 - self.e31 * other.e03 * other.e03 + self.e31 * other.e31 * other.e31 - self.e31 * other.e23 * other.e23 + self.e31 * other.e0123 * other.e0123 - 2.0 * self.e23 * other.s * other.e12 + 2.0 * self.e23 * other.e01 * other.e02 + 2.0 * self.e23 * other.e03 * other.e0123 + 2.0 * self.e23 * other.e31 * other.e23,
            e013: 2.0 * self.e021 * other.s * other.e23 - 2.0 * self.e021 * other.e01 * other.e0123 + 2.0 * self.e021 * other.e02 * other.e03 + 2.0 * self.e021 * other.e12 * other.e31 + self.e013 * other.s * other.s - self.e013 * other.e01 * other.e01 + self.e013 * other.e02 * other.e02 - self.e013 * other.e12 * other.e12 - self.e013 * other.e03 * other.e03 + self.e013 * other.e31 * other.e31 - self.e013 * other.e23 * other.e23 + self.e013 * other.e0123 * other.e0123 - 2.0 * self.e032 * other.s * other.e12 + 2.0 * self.e032 * other.e01 * other.e02 + 2.0 * self.e032 * other.e03 * other.e0123 + 2.0 * self.e032 * other.e31 * other.e23 - 2.0 * self.e123 * other.s * other.e02 + 2.0 * self.e123 * other.e01 * other.e12 - 2.0 * self.e123 * other.e03 * other.e23 - 2.0 * self.e123 * other.e31 * other.e0123,
            e23: -2.0 * self.e01 * other.s * other.e0123 + 2.0 * self.e01 * other.e01 * other.e23 - 2.0 * self.e01 * other.e02 * other.e31 - 2.0 * self.e01 * other.e12 * other.e03 + 2.0 * self.e02 * other.s * other.e03 + 2.0 * self.e02 * other.e01 * other.e31 + 2.0 * self.e02 * other.e02 * other.e23 - 2.0 * self.e02 * other.e12 * other.e0123 - 2.0 * self.e12 * other.s * other.e31 + 2.0 * self.e12 * other.e01 * other.e03 + 2.0 * self.e12 * other.e02 * other.e0123 + 2.0 * self.e12 * other.e12 * other.e23 - 2.0 * self.e03 * other.s * other.e02 + 2.0 * self.e03 * other.e01 * other.e12 + 2.0 * self.e03 * other.e03 * other.e23 + 2.0 * self.e03 * other.e31 * other.e0123 + 2.0 * self.e31 * other.s * other.e12 + 2.0 * self.e31 * other.e01 * other.e02 - 2.0 * self.e31 * other.e03 * other.e0123 + 2.0 * self.e31 * other.e31 * other.e23 + self.e23 * other.s * other.s + self.e23 * other.e01 * other.e01 - self.e23 * other.e02 * other.e02 - self.e23 * other.e12 * other.e12 - self.e23 * other.e03 * other.e03 - self.e23 * other.e31 * other.e31 + self.e23 * other.e23 * other.e23 + self.e23 * other.e0123 * other.e0123,
            e032: -2.0 * self.e021 * other.s * other.e31 + 2.0 * self.e021 * other.e01 * other.e03 + 2.0 * self.e021 * other.e02 * other.e0123 + 2.0 * self.e021 * other.e12 * other.e23 + 2.0 * self.e013 * other.s * other.e12 + 2.0 * self.e013 * other.e01 * other.e02 - 2.0 * self.e013 * other.e03 * other.e0123 + 2.0 * self.e013 * other.e31 * other.e23 + self.e032 * other.s * other.s + self.e032 * other.e01 * other.e01 - self.e032 * other.e02 * other.e02 - self.e032 * other.e12 * other.e12 - self.e032 * other.e03 * other.e03 - self.e032 * other.e31 * other.e31 + self.e032 * other.e23 * other.e23 + self.e032 * other.e0123 * other.e0123 - 2.0 * self.e123 * other.s * other.e01 - 2.0 * self.e123 * other.e02 * other.e12 + 2.0 * self.e123 * other.e03 * other.e31 - 2.0 * self.e123 * other.e23 * other.e0123,
            e123: 2.0 * self.e021 * other.s * other.e03 - 2.0 * self.e021 * other.e01 * other.e31 + 2.0 * self.e021 * other.e02 * other.e23 + 2.0 * self.e021 * other.e12 * other.e0123 + 2.0 * self.e013 * other.s * other.e02 + 2.0 * self.e013 * other.e01 * other.e12 - 2.0 * self.e013 * other.e03 * other.e23 + 2.0 * self.e013 * other.e31 * other.e0123 + 2.0 * self.e032 * other.s * other.e01 - 2.0 * self.e032 * other.e02 * other.e12 + 2.0 * self.e032 * other.e03 * other.e31 + 2.0 * self.e032 * other.e23 * other.e0123 + self.e123 * other.s * other.s - self.e123 * other.e01 * other.e01 - self.e123 * other.e02 * other.e02 + self.e123 * other.e12 * other.e12 - self.e123 * other.e03 * other.e03 + self.e123 * other.e31 * other.e31 + self.e123 * other.e23 * other.e23 - self.e123 * other.e0123 * other.e0123,
            e0123: 2.0 * self.s * other.s * other.e0123 - 2.0 * self.s * other.e01 * other.e23 - 2.0 * self.s * other.e02 * other.e31 - 2.0 * self.s * other.e12 * other.e03 + self.e0123 * other.s * other.s - self.e0123 * other.e01 * other.e01 - self.e0123 * other.e02 * other.e02 + self.e0123 * other.e12 * other.e12 - self.e0123 * other.e03 * other.e03 + self.e0123 * other.e31 * other.e31 + self.e0123 * other.e23 * other.e23 - self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Transform<Multivector> for Multivector {
    type Output = Multivector;
    fn transform(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01 + self.s * other.e2 * other.e2 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e013 * other.e013 + self.s * other.e23 * other.e23 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 + self.s * other.e0123 * other.e0123 + 2.0 * self.e0 * other.s * other.e0 - 2.0 * self.e0 * other.e1 * other.e01 - 2.0 * self.e0 * other.e2 * other.e02 - 2.0 * self.e0 * other.e12 * other.e021 - 2.0 * self.e0 * other.e3 * other.e03 - 2.0 * self.e0 * other.e31 * other.e013 - 2.0 * self.e0 * other.e23 * other.e032 - 2.0 * self.e0 * other.e123 * other.e0123 + 2.0 * self.e1 * other.s * other.e1 + 2.0 * self.e1 * other.e0 * other.e01 - 2.0 * self.e1 * other.e2 * other.e12 + 2.0 * self.e1 * other.e02 * other.e021 + 2.0 * self.e1 * other.e3 * other.e31 - 2.0 * self.e1 * other.e03 * other.e013 + 2.0 * self.e1 * other.e23 * other.e123 - 2.0 * self.e1 * other.e032 * other.e0123 + 2.0 * self.e2 * other.s * other.e2 + 2.0 * self.e2 * other.e0 * other.e02 + 2.0 * self.e2 * other.e1 * other.e12 - 2.0 * self.e2 * other.e01 * other.e021 - 2.0 * self.e2 * other.e3 * other.e23 + 2.0 * self.e2 * other.e03 * other.e032 + 2.0 * self.e2 * other.e31 * other.e123 - 2.0 * self.e2 * other.e013 * other.e0123 + 2.0 * self.e3 * other.s * other.e3 + 2.0 * self.e3 * other.e0 * other.e03 - 2.0 * self.e3 * other.e1 * other.e31 + 2.0 * self.e3 * other.e01 * other.e013 + 2.0 * self.e3 * other.e2 * other.e23 - 2.0 * self.e3 * other.e02 * other.e032 + 2.0 * self.e3 * other.e12 * other.e123 - 2.0 * self.e3 * other.e021 * other.e0123 + 2.0 * self.e0123 * other.s * other.e0123 + 2.0 * self.e0123 * other.e0 * other.e123 + 2.0 * self.e0123 * other.e1 * other.e032 - 2.0 * self.e0123 * other.e01 * other.e23 + 2.0 * self.e0123 * other.e2 * other.e013 - 2.0 * self.e0123 * other.e02 * other.e31 - 2.0 * self.e0123 * other.e12 * other.e03 + 2.0 * self.e0123 * other.e021 * other.e3,
            e0: 2.0 * self.s * other.s * other.e0 + 2.0 * self.s * other.e1 * other.e01 + 2.0 * self.s * other.e2 * other.e02 - 2.0 * self.s * other.e12 * other.e021 + 2.0 * self.s * other.e3 * other.e03 - 2.0 * self.s * other.e31 * other.e013 - 2.0 * self.s * other.e23 * other.e032 + 2.0 * self.s * other.e123 * other.e0123 + self.e0 * other.s * other.s - self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 + self.e0 * other.e01 * other.e01 - self.e0 * other.e2 * other.e2 + self.e0 * other.e02 * other.e02 + self.e0 * other.e12 * other.e12 - self.e0 * other.e021 * other.e021 - self.e0 * other.e3 * other.e3 + self.e0 * other.e03 * other.e03 + self.e0 * other.e31 * other.e31 - self.e0 * other.e013 * other.e013 + self.e0 * other.e23 * other.e23 - self.e0 * other.e032 * other.e032 - self.e0 * other.e123 * other.e123 + self.e0 * other.e0123 * other.e0123 + 2.0 * self.e1 * other.s * other.e01 + 2.0 * self.e1 * other.e0 * other.e1 + 2.0 * self.e1 * other.e2 * other.e021 - 2.0 * self.e1 * other.e02 * other.e12 - 2.0 * self.e1 * other.e3 * other.e013 + 2.0 * self.e1 * other.e03 * other.e31 + 2.0 * self.e1 * other.e23 * other.e0123 - 2.0 * self.e1 * other.e032 * other.e123 + 2.0 * self.e2 * other.s * other.e02 + 2.0 * self.e2 * other.e0 * other.e2 - 2.0 * self.e2 * other.e1 * other.e021 + 2.0 * self.e2 * other.e01 * other.e12 + 2.0 * self.e2 * other.e3 * other.e032 - 2.0 * self.e2 * other.e03 * other.e23 + 2.0 * self.e2 * other.e31 * other.e0123 - 2.0 * self.e2 * other.e013 * other.e123 + 2.0 * self.e3 * other.s * other.e03 + 2.0 * self.e3 * other.e0 * other.e3 + 2.0 * self.e3 * other.e1 * other.e013 - 2.0 * self.e3 * other.e01 * other.e31 - 2.0 * self.e3 * other.e2 * other.e032 + 2.0 * self.e3 * other.e02 * other.e23 + 2.0 * self.e3 * other.e12 * other.e0123 - 2.0 * self.e3 * other.e021 * other.e123 + 2.0 * self.e0123 * other.s * other.e123 - 2.0 * self.e0123 * other.e0 * other.e0123 - 2.0 * self.e0123 * other.e1 * other.e23 - 2.0 * self.e0123 * other.e01 * other.e032 - 2.0 * self.e0123 * other.e2 * other.e31 - 2.0 * self.e0123 * other.e02 * other.e013 - 2.0 * self.e0123 * other.e12 * other.e3 - 2.0 * self.e0123 * other.e021 * other.e03,
            e1: 2.0 * self.s * other.s * other.e1 - 2.0 * self.s * other.e0 * other.e01 + 2.0 * self.s * other.e2 * other.e12 + 2.0 * self.s * other.e02 * other.e021 - 2.0 * self.s * other.e3 * other.e31 - 2.0 * self.s * other.e03 * other.e013 + 2.0 * self.s * other.e23 * other.e123 + 2.0 * self.s * other.e032 * other.e0123 - 2.0 * self.e0 * other.s * other.e01 + 2.0 * self.e0 * other.e0 * other.e1 - 2.0 * self.e0 * other.e2 * other.e021 - 2.0 * self.e0 * other.e02 * other.e12 + 2.0 * self.e0 * other.e3 * other.e013 + 2.0 * self.e0 * other.e03 * other.e31 - 2.0 * self.e0 * other.e23 * other.e0123 - 2.0 * self.e0 * other.e032 * other.e123 + self.e1 * other.s * other.s - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1 - self.e1 * other.e01 * other.e01 - self.e1 * other.e2 * other.e2 + self.e1 * other.e02 * other.e02 - self.e1 * other.e12 * other.e12 + self.e1 * other.e021 * other.e021 - self.e1 * other.e3 * other.e3 + self.e1 * other.e03 * other.e03 - self.e1 * other.e31 * other.e31 + self.e1 * other.e013 * other.e013 + self.e1 * other.e23 * other.e23 - self.e1 * other.e032 * other.e032 + self.e1 * other.e123 * other.e123 - self.e1 * other.e0123 * other.e0123 + 2.0 * self.e2 * other.s * other.e12 + 2.0 * self.e2 * other.e0 * other.e021 + 2.0 * self.e2 * other.e1 * other.e2 - 2.0 * self.e2 * other.e01 * other.e02 - 2.0 * self.e2 * other.e3 * other.e123 + 2.0 * self.e2 * other.e03 * other.e0123 + 2.0 * self.e2 * other.e31 * other.e23 - 2.0 * self.e2 * other.e013 * other.e032 - 2.0 * self.e3 * other.s * other.e31 - 2.0 * self.e3 * other.e0 * other.e013 + 2.0 * self.e3 * other.e1 * other.e3 - 2.0 * self.e3 * other.e01 * other.e03 + 2.0 * self.e3 * other.e2 * other.e123 - 2.0 * self.e3 * other.e02 * other.e0123 + 2.0 * self.e3 * other.e12 * other.e23 - 2.0 * self.e3 * other.e021 * other.e032 + 2.0 * self.e0123 * other.s * other.e032 + 2.0 * self.e0123 * other.e0 * other.e23 + 2.0 * self.e0123 * other.e1 * other.e0123 - 2.0 * self.e0123 * other.e01 * other.e123 - 2.0 * self.e0123 * other.e2 * other.e03 + 2.0 * self.e0123 * other.e02 * other.e3 + 2.0 * self.e0123 * other.e12 * other.e013 - 2.0 * self.e0123 * other.e021 * other.e31,
            e01: self.e01 * other.s * other.s + self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1 - self.e01 * other.e01 * other.e01 + self.e01 * other.e2 * other.e2 + self.e01 * other.e02 * other.e02 - self.e01 * other.e12 * other.e12 - self.e01 * other.e021 * other.e021 + self.e01 * other.e3 * other.e3 + self.e01 * other.e03 * other.e03 - self.e01 * other.e31 * other.e31 - self.e01 * other.e013 * other.e013 + self.e01 * other.e23 * other.e23 + self.e01 * other.e032 * other.e032 - self.e01 * other.e123 * other.e123 - self.e01 * other.e0123 * other.e0123 + 2.0 * self.e02 * other.s * other.e12 - 2.0 * self.e02 * other.e0 * other.e021 - 2.0 * self.e02 * other.e1 * other.e2 - 2.0 * self.e02 * other.e01 * other.e02 + 2.0 * self.e02 * other.e3 * other.e123 + 2.0 * self.e02 * other.e03 * other.e0123 + 2.0 * self.e02 * other.e31 * other.e23 + 2.0 * self.e02 * other.e013 * other.e032 - 2.0 * self.e12 * other.s * other.e02 + 2.0 * self.e12 * other.e0 * other.e2 + 2.0 * self.e12 * other.e1 * other.e021 + 2.0 * self.e12 * other.e01 * other.e12 + 2.0 * self.e12 * other.e3 * other.e032 + 2.0 * self.e12 * other.e03 * other.e23 + 2.0 * self.e12 * other.e31 * other.e0123 + 2.0 * self.e12 * other.e013 * other.e123 - 2.0 * self.e021 * other.s * other.e2 - 2.0 * self.e021 * other.e0 * other.e02 + 2.0 * self.e021 * other.e1 * other.e12 - 2.0 * self.e021 * other.e01 * other.e021 + 2.0 * self.e021 * other.e3 * other.e23 - 2.0 * self.e021 * other.e03 * other.e032 + 2.0 * self.e021 * other.e31 * other.e123 - 2.0 * self.e021 * other.e013 * other.e0123 - 2.0 * self.e03 * other.s * other.e31 + 2.0 * self.e03 * other.e0 * other.e013 - 2.0 * self.e03 * other.e1 * other.e3 - 2.0 * self.e03 * other.e01 * other.e03 - 2.0 * self.e03 * other.e2 * other.e123 - 2.0 * self.e03 * other.e02 * other.e0123 + 2.0 * self.e03 * other.e12 * other.e23 + 2.0 * self.e03 * other.e021 * other.e032 + 2.0 * self.e31 * other.s * other.e03 - 2.0 * self.e31 * other.e0 * other.e3 + 2.0 * self.e31 * other.e1 * other.e013 + 2.0 * self.e31 * other.e01 * other.e31 + 2.0 * self.e31 * other.e2 * other.e032 + 2.0 * self.e31 * other.e02 * other.e23 - 2.0 * self.e31 * other.e12 * other.e0123 - 2.0 * self.e31 * other.e021 * other.e123 + 2.0 * self.e013 * other.s * other.e3 + 2.0 * self.e013 * other.e0 * other.e03 + 2.0 * self.e013 * other.e1 * other.e31 - 2.0 * self.e013 * other.e01 * other.e013 + 2.0 * self.e013 * other.e2 * other.e23 - 2.0 * self.e013 * other.e02 * other.e032 - 2.0 * self.e013 * other.e12 * other.e123 + 2.0 * self.e013 * other.e021 * other.e0123 - 2.0 * self.e23 * other.s * other.e0123 + 2.0 * self.e23 * other.e0 * other.e123 + 2.0 * self.e23 * other.e1 * other.e032 + 2.0 * self.e23 * other.e01 * other.e23 - 2.0 * self.e23 * other.e2 * other.e013 - 2.0 * self.e23 * other.e02 * other.e31 - 2.0 * self.e23 * other.e12 * other.e03 - 2.0 * self.e23 * other.e021 * other.e3 - 2.0 * self.e032 * other.s * other.e123 - 2.0 * self.e032 * other.e0 * other.e0123 + 2.0 * self.e032 * other.e1 * other.e23 - 2.0 * self.e032 * other.e01 * other.e032 - 2.0 * self.e032 * other.e2 * other.e31 + 2.0 * self.e032 * other.e02 * other.e013 - 2.0 * self.e032 * other.e12 * other.e3 + 2.0 * self.e032 * other.e021 * other.e03 + 2.0 * self.e123 * other.s * other.e032 + 2.0 * self.e123 * other.e0 * other.e23 - 2.0 * self.e123 * other.e1 * other.e0123 + 2.0 * self.e123 * other.e01 * other.e123 + 2.0 * self.e123 * other.e2 * other.e03 - 2.0 * self.e123 * other.e02 * other.e3 + 2.0 * self.e123 * other.e12 * other.e013 - 2.0 * self.e123 * other.e021 * other.e31,
            e2: 2.0 * self.s * other.s * other.e2 - 2.0 * self.s * other.e0 * other.e02 - 2.0 * self.s * other.e1 * other.e12 - 2.0 * self.s * other.e01 * other.e021 + 2.0 * self.s * other.e3 * other.e23 + 2.0 * self.s * other.e03 * other.e032 + 2.0 * self.s * other.e31 * other.e123 + 2.0 * self.s * other.e013 * other.e0123 - 2.0 * self.e0 * other.s * other.e02 + 2.0 * self.e0 * other.e0 * other.e2 + 2.0 * self.e0 * other.e1 * other.e021 + 2.0 * self.e0 * other.e01 * other.e12 - 2.0 * self.e0 * other.e3 * other.e032 - 2.0 * self.e0 * other.e03 * other.e23 - 2.0 * self.e0 * other.e31 * other.e0123 - 2.0 * self.e0 * other.e013 * other.e123 - 2.0 * self.e1 * other.s * other.e12 - 2.0 * self.e1 * other.e0 * other.e021 + 2.0 * self.e1 * other.e1 * other.e2 - 2.0 * self.e1 * other.e01 * other.e02 + 2.0 * self.e1 * other.e3 * other.e123 - 2.0 * self.e1 * other.e03 * other.e0123 + 2.0 * self.e1 * other.e31 * other.e23 - 2.0 * self.e1 * other.e013 * other.e032 + self.e2 * other.s * other.s - self.e2 * other.e0 * other.e0 - self.e2 * other.e1 * other.e1 + self.e2 * other.e01 * other.e01 + self.e2 * other.e2 * other.e2 - self.e2 * other.e02 * other.e02 - self.e2 * other.e12 * other.e12 + self.e2 * other.e021 * other.e021 - self.e2 * other.e3 * other.e3 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e2 * other.e013 * other.e013 - self.e2 * other.e23 * other.e23 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 - self.e2 * other.e0123 * other.e0123 + 2.0 * self.e3 * other.s * other.e23 + 2.0 * self.e3 * other.e0 * other.e032 - 2.0 * self.e3 * other.e1 * other.e123 + 2.0 * self.e3 * other.e01 * other.e0123 + 2.0 * self.e3 * other.e2 * other.e3 - 2.0 * self.e3 * other.e02 * other.e03 + 2.0 * self.e3 * other.e12 * other.e31 - 2.0 * self.e3 * other.e021 * other.e013 + 2.0 * self.e0123 * other.s * other.e013 + 2.0 * self.e0123 * other.e0 * other.e31 + 2.0 * self.e0123 * other.e1 * other.e03 - 2.0 * self.e0123 * other.e01 * other.e3 + 2.0 * self.e0123 * other.e2 * other.e0123 - 2.0 * self.e0123 * other.e02 * other.e123 - 2.0 * self.e0123 * other.e12 * other.e032 + 2.0 * self.e0123 * other.e021 * other.e23,
            e02: -2.0 * self.e01 * other.s * other.e12 + 2.0 * self.e01 * other.e0 * other.e021 - 2.0 * self.e01 * other.e1 * other.e2 - 2.0 * self.e01 * other.e01 * other.e02 - 2.0 * self.e01 * other.e3 * other.e123 - 2.0 * self.e01 * other.e03 * other.e0123 + 2.0 * self.e01 * other.e31 * other.e23 + 2.0 * self.e01 * other.e013 * other.e032 + self.e02 * other.s * other.s + self.e02 * other.e0 * other.e0 + self.e02 * other.e1 * other.e1 + self.e02 * other.e01 * other.e01 - self.e02 * other.e2 * other.e2 - self.e02 * other.e02 * other.e02 - self.e02 * other.e12 * other.e12 - self.e02 * other.e021 * other.e021 + self.e02 * other.e3 * other.e3 + self.e02 * other.e03 * other.e03 + self.e02 * other.e31 * other.e31 + self.e02 * other.e013 * other.e013 - self.e02 * other.e23 * other.e23 - self.e02 * other.e032 * other.e032 - self.e02 * other.e123 * other.e123 - self.e02 * other.e0123 * other.e0123 + 2.0 * self.e12 * other.s * other.e01 - 2.0 * self.e12 * other.e0 * other.e1 + 2.0 * self.e12 * other.e2 * other.e021 + 2.0 * self.e12 * other.e02 * other.e12 + 2.0 * self.e12 * other.e3 * other.e013 + 2.0 * self.e12 * other.e03 * other.e31 - 2.0 * self.e12 * other.e23 * other.e0123 - 2.0 * self.e12 * other.e032 * other.e123 + 2.0 * self.e021 * other.s * other.e1 + 2.0 * self.e021 * other.e0 * other.e01 + 2.0 * self.e021 * other.e2 * other.e12 - 2.0 * self.e021 * other.e02 * other.e021 + 2.0 * self.e021 * other.e3 * other.e31 - 2.0 * self.e021 * other.e03 * other.e013 - 2.0 * self.e021 * other.e23 * other.e123 + 2.0 * self.e021 * other.e032 * other.e0123 + 2.0 * self.e03 * other.s * other.e23 - 2.0 * self.e03 * other.e0 * other.e032 + 2.0 * self.e03 * other.e1 * other.e123 + 2.0 * self.e03 * other.e01 * other.e0123 - 2.0 * self.e03 * other.e2 * other.e3 - 2.0 * self.e03 * other.e02 * other.e03 + 2.0 * self.e03 * other.e12 * other.e31 + 2.0 * self.e03 * other.e021 * other.e013 - 2.0 * self.e31 * other.s * other.e0123 + 2.0 * self.e31 * other.e0 * other.e123 - 2.0 * self.e31 * other.e1 * other.e032 - 2.0 * self.e31 * other.e01 * other.e23 + 2.0 * self.e31 * other.e2 * other.e013 + 2.0 * self.e31 * other.e02 * other.e31 - 2.0 * self.e31 * other.e12 * other.e03 - 2.0 * self.e31 * other.e021 * other.e3 - 2.0 * self.e013 * other.s * other.e123 - 2.0 * self.e013 * other.e0 * other.e0123 - 2.0 * self.e013 * other.e1 * other.e23 + 2.0 * self.e013 * other.e01 * other.e032 + 2.0 * self.e013 * other.e2 * other.e31 - 2.0 * self.e013 * other.e02 * other.e013 - 2.0 * self.e013 * other.e12 * other.e3 + 2.0 * self.e013 * other.e021 * other.e03 - 2.0 * self.e23 * other.s * other.e03 + 2.0 * self.e23 * other.e0 * other.e3 + 2.0 * self.e23 * other.e1 * other.e013 + 2.0 * self.e23 * other.e01 * other.e31 + 2.0 * self.e23 * other.e2 * other.e032 + 2.0 * self.e23 * other.e02 * other.e23 + 2.0 * self.e23 * other.e12 * other.e0123 + 2.0 * self.e23 * other.e021 * other.e123 - 2.0 * self.e032 * other.s * other.e3 - 2.0 * self.e032 * other.e0 * other.e03 + 2.0 * self.e032 * other.e1 * other.e31 - 2.0 * self.e032 * other.e01 * other.e013 + 2.0 * self.e032 * other.e2 * other.e23 - 2.0 * self.e032 * other.e02 * other.e032 + 2.0 * self.e032 * other.e12 * other.e123 - 2.0 * self.e032 * other.e021 * other.e0123 + 2.0 * self.e123 * other.s * other.e013 + 2.0 * self.e123 * other.e0 * other.e31 - 2.0 * self.e123 * other.e1 * other.e03 + 2.0 * self.e123 * other.e01 * other.e3 - 2.0 * self.e123 * other.e2 * other.e0123 + 2.0 * self.e123 * other.e02 * other.e123 - 2.0 * self.e123 * other.e12 * other.e032 + 2.0 * self.e123 * other.e021 * other.e23,
            e12: 2.0 * self.e01 * other.s * other.e02 + 2.0 * self.e01 * other.e0 * other.e2 - 2.0 * self.e01 * other.e1 * other.e021 + 2.0 * self.e01 * other.e01 * other.e12 - 2.0 * self.e01 * other.e3 * other.e032 + 2.0 * self.e01 * other.e03 * other.e23 - 2.0 * self.e01 * other.e31 * other.e0123 + 2.0 * self.e01 * other.e013 * other.e123 - 2.0 * self.e02 * other.s * other.e01 - 2.0 * self.e02 * other.e0 * other.e1 - 2.0 * self.e02 * other.e2 * other.e021 + 2.0 * self.e02 * other.e02 * other.e12 - 2.0 * self.e02 * other.e3 * other.e013 + 2.0 * self.e02 * other.e03 * other.e31 + 2.0 * self.e02 * other.e23 * other.e0123 - 2.0 * self.e02 * other.e032 * other.e123 + self.e12 * other.s * other.s + self.e12 * other.e0 * other.e0 - self.e12 * other.e1 * other.e1 - self.e12 * other.e01 * other.e01 - self.e12 * other.e2 * other.e2 - self.e12 * other.e02 * other.e02 + self.e12 * other.e12 * other.e12 + self.e12 * other.e021 * other.e021 + self.e12 * other.e3 * other.e3 + self.e12 * other.e03 * other.e03 - self.e12 * other.e31 * other.e31 - self.e12 * other.e013 * other.e013 - self.e12 * other.e23 * other.e23 - self.e12 * other.e032 * other.e032 + self.e12 * other.e123 * other.e123 + self.e12 * other.e0123 * other.e0123 - 2.0 * self.e021 * other.s * other.e0 - 2.0 * self.e021 * other.e1 * other.e01 - 2.0 * self.e021 * other.e2 * other.e02 + 2.0 * self.e021 * other.e12 * other.e021 + 2.0 * self.e021 * other.e3 * other.e03 - 2.0 * self.e021 * other.e31 * other.e013 - 2.0 * self.e021 * other.e23 * other.e032 + 2.0 * self.e021 * other.e123 * other.e0123 - 2.0 * self.e03 * other.s * other.e0123 - 2.0 * self.e03 * other.e0 * other.e123 + 2.0 * self.e03 * other.e1 * other.e032 - 2.0 * self.e03 * other.e01 * other.e23 + 2.0 * self.e03 * other.e2 * other.e013 - 2.0 * self.e03 * other.e02 * other.e31 + 2.0 * self.e03 * other.e12 * other.e03 - 2.0 * self.e03 * other.e021 * other.e3 - 2.0 * self.e31 * other.s * other.e23 + 2.0 * self.e31 * other.e0 * other.e032 + 2.0 * self.e31 * other.e1 * other.e123 + 2.0 * self.e31 * other.e01 * other.e0123 + 2.0 * self.e31 * other.e2 * other.e3 + 2.0 * self.e31 * other.e02 * other.e03 + 2.0 * self.e31 * other.e12 * other.e31 + 2.0 * self.e31 * other.e021 * other.e013 - 2.0 * self.e013 * other.s * other.e032 + 2.0 * self.e013 * other.e0 * other.e23 + 2.0 * self.e013 * other.e1 * other.e0123 + 2.0 * self.e013 * other.e01 * other.e123 + 2.0 * self.e013 * other.e2 * other.e03 + 2.0 * self.e013 * other.e02 * other.e3 + 2.0 * self.e013 * other.e12 * other.e013 + 2.0 * self.e013 * other.e021 * other.e31 + 2.0 * self.e23 * other.s * other.e31 - 2.0 * self.e23 * other.e0 * other.e013 + 2.0 * self.e23 * other.e1 * other.e3 + 2.0 * self.e23 * other.e01 * other.e03 - 2.0 * self.e23 * other.e2 * other.e123 - 2.0 * self.e23 * other.e02 * other.e0123 + 2.0 * self.e23 * other.e12 * other.e23 + 2.0 * self.e23 * other.e021 * other.e032 + 2.0 * self.e032 * other.s * other.e013 - 2.0 * self.e032 * other.e0 * other.e31 + 2.0 * self.e032 * other.e1 * other.e03 + 2.0 * self.e032 * other.e01 * other.e3 - 2.0 * self.e032 * other.e2 * other.e0123 - 2.0 * self.e032 * other.e02 * other.e123 + 2.0 * self.e032 * other.e12 * other.e032 + 2.0 * self.e032 * other.e021 * other.e23 + 2.0 * self.e123 * other.s * other.e3 + 2.0 * self.e123 * other.e0 * other.e03 + 2.0 * self.e123 * other.e1 * other.e31 - 2.0 * self.e123 * other.e01 * other.e013 - 2.0 * self.e123 * other.e2 * other.e23 + 2.0 * self.e123 * other.e02 * other.e032 + 2.0 * self.e123 * other.e12 * other.e123 - 2.0 * self.e123 * other.e021 * other.e0123,
            e021: -2.0 * self.e01 * other.s * other.e2 + 2.0 * self.e01 * other.e0 * other.e02 - 2.0 * self.e01 * other.e1 * other.e12 - 2.0 * self.e01 * other.e01 * other.e021 - 2.0 * self.e01 * other.e3 * other.e23 - 2.0 * self.e01 * other.e03 * other.e032 + 2.0 * self.e01 * other.e31 * other.e123 + 2.0 * self.e01 * other.e013 * other.e0123 + 2.0 * self.e02 * other.s * other.e1 - 2.0 * self.e02 * other.e0 * other.e01 - 2.0 * self.e02 * other.e2 * other.e12 - 2.0 * self.e02 * other.e02 * other.e021 - 2.0 * self.e02 * other.e3 * other.e31 - 2.0 * self.e02 * other.e03 * other.e013 - 2.0 * self.e02 * other.e23 * other.e123 - 2.0 * self.e02 * other.e032 * other.e0123 - 2.0 * self.e12 * other.s * other.e0 + 2.0 * self.e12 * other.e1 * other.e01 + 2.0 * self.e12 * other.e2 * other.e02 + 2.0 * self.e12 * other.e12 * other.e021 - 2.0 * self.e12 * other.e3 * other.e03 - 2.0 * self.e12 * other.e31 * other.e013 - 2.0 * self.e12 * other.e23 * other.e032 - 2.0 * self.e12 * other.e123 * other.e0123 + self.e021 * other.s * other.s - self.e021 * other.e0 * other.e0 + self.e021 * other.e1 * other.e1 - self.e021 * other.e01 * other.e01 + self.e021 * other.e2 * other.e2 - self.e021 * other.e02 * other.e02 + self.e021 * other.e12 * other.e12 - self.e021 * other.e021 * other.e021 - self.e021 * other.e3 * other.e3 + self.e021 * other.e03 * other.e03 - self.e021 * other.e31 * other.e31 + self.e021 * other.e013 * other.e013 - self.e021 * other.e23 * other.e23 + self.e021 * other.e032 * other.e032 - self.e021 * other.e123 * other.e123 + self.e021 * other.e0123 * other.e0123 + 2.0 * self.e03 * other.s * other.e123 - 2.0 * self.e03 * other.e0 * other.e0123 + 2.0 * self.e03 * other.e1 * other.e23 + 2.0 * self.e03 * other.e01 * other.e032 + 2.0 * self.e03 * other.e2 * other.e31 + 2.0 * self.e03 * other.e02 * other.e013 - 2.0 * self.e03 * other.e12 * other.e3 - 2.0 * self.e03 * other.e021 * other.e03 - 2.0 * self.e31 * other.s * other.e032 + 2.0 * self.e31 * other.e0 * other.e23 - 2.0 * self.e31 * other.e1 * other.e0123 - 2.0 * self.e31 * other.e01 * other.e123 - 2.0 * self.e31 * other.e2 * other.e03 - 2.0 * self.e31 * other.e02 * other.e3 + 2.0 * self.e31 * other.e12 * other.e013 + 2.0 * self.e31 * other.e021 * other.e31 - 2.0 * self.e013 * other.s * other.e23 - 2.0 * self.e013 * other.e0 * other.e032 - 2.0 * self.e013 * other.e1 * other.e123 + 2.0 * self.e013 * other.e01 * other.e0123 - 2.0 * self.e013 * other.e2 * other.e3 + 2.0 * self.e013 * other.e02 * other.e03 + 2.0 * self.e013 * other.e12 * other.e31 - 2.0 * self.e013 * other.e021 * other.e013 + 2.0 * self.e23 * other.s * other.e013 - 2.0 * self.e23 * other.e0 * other.e31 - 2.0 * self.e23 * other.e1 * other.e03 - 2.0 * self.e23 * other.e01 * other.e3 + 2.0 * self.e23 * other.e2 * other.e0123 + 2.0 * self.e23 * other.e02 * other.e123 + 2.0 * self.e23 * other.e12 * other.e032 + 2.0 * self.e23 * other.e021 * other.e23 + 2.0 * self.e032 * other.s * other.e31 + 2.0 * self.e032 * other.e0 * other.e013 - 2.0 * self.e032 * other.e1 * other.e3 + 2.0 * self.e032 * other.e01 * other.e03 + 2.0 * self.e032 * other.e2 * other.e123 - 2.0 * self.e032 * other.e02 * other.e0123 + 2.0 * self.e032 * other.e12 * other.e23 - 2.0 * self.e032 * other.e021 * other.e032 - 2.0 * self.e123 * other.s * other.e03 - 2.0 * self.e123 * other.e0 * other.e3 + 2.0 * self.e123 * other.e1 * other.e013 - 2.0 * self.e123 * other.e01 * other.e31 - 2.0 * self.e123 * other.e2 * other.e032 + 2.0 * self.e123 * other.e02 * other.e23 - 2.0 * self.e123 * other.e12 * other.e0123 + 2.0 * self.e123 * other.e021 * other.e123,
            e3: 2.0 * self.s * other.s * other.e3 - 2.0 * self.s * other.e0 * other.e03 + 2.0 * self.s * other.e1 * other.e31 + 2.0 * self.s * other.e01 * other.e013 - 2.0 * self.s * other.e2 * other.e23 - 2.0 * self.s * other.e02 * other.e032 + 2.0 * self.s * other.e12 * other.e123 + 2.0 * self.s * other.e021 * other.e0123 - 2.0 * self.e0 * other.s * other.e03 + 2.0 * self.e0 * other.e0 * other.e3 - 2.0 * self.e0 * other.e1 * other.e013 - 2.0 * self.e0 * other.e01 * other.e31 + 2.0 * self.e0 * other.e2 * other.e032 + 2.0 * self.e0 * other.e02 * other.e23 - 2.0 * self.e0 * other.e12 * other.e0123 - 2.0 * self.e0 * other.e021 * other.e123 + 2.0 * self.e1 * other.s * other.e31 + 2.0 * self.e1 * other.e0 * other.e013 + 2.0 * self.e1 * other.e1 * other.e3 - 2.0 * self.e1 * other.e01 * other.e03 - 2.0 * self.e1 * other.e2 * other.e123 + 2.0 * self.e1 * other.e02 * other.e0123 + 2.0 * self.e1 * other.e12 * other.e23 - 2.0 * self.e1 * other.e021 * other.e032 - 2.0 * self.e2 * other.s * other.e23 - 2.0 * self.e2 * other.e0 * other.e032 + 2.0 * self.e2 * other.e1 * other.e123 - 2.0 * self.e2 * other.e01 * other.e0123 + 2.0 * self.e2 * other.e2 * other.e3 - 2.0 * self.e2 * other.e02 * other.e03 + 2.0 * self.e2 * other.e12 * other.e31 - 2.0 * self.e2 * other.e021 * other.e013 + self.e3 * other.s * other.s - self.e3 * other.e0 * other.e0 - self.e3 * other.e1 * other.e1 + self.e3 * other.e01 * other.e01 - self.e3 * other.e2 * other.e2 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12 - self.e3 * other.e021 * other.e021 + self.e3 * other.e3 * other.e3 - self.e3 * other.e03 * other.e03 - self.e3 * other.e31 * other.e31 + self.e3 * other.e013 * other.e013 - self.e3 * other.e23 * other.e23 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123 - self.e3 * other.e0123 * other.e0123 + 2.0 * self.e0123 * other.s * other.e021 + 2.0 * self.e0123 * other.e0 * other.e12 - 2.0 * self.e0123 * other.e1 * other.e02 + 2.0 * self.e0123 * other.e01 * other.e2 + 2.0 * self.e0123 * other.e3 * other.e0123 - 2.0 * self.e0123 * other.e03 * other.e123 + 2.0 * self.e0123 * other.e31 * other.e032 - 2.0 * self.e0123 * other.e013 * other.e23,
            e03: 2.0 * self.e01 * other.s * other.e31 - 2.0 * self.e01 * other.e0 * other.e013 - 2.0 * self.e01 * other.e1 * other.e3 - 2.0 * self.e01 * other.e01 * other.e03 + 2.0 * self.e01 * other.e2 * other.e123 + 2.0 * self.e01 * other.e02 * other.e0123 + 2.0 * self.e01 * other.e12 * other.e23 + 2.0 * self.e01 * other.e021 * other.e032 - 2.0 * self.e02 * other.s * other.e23 + 2.0 * self.e02 * other.e0 * other.e032 - 2.0 * self.e02 * other.e1 * other.e123 - 2.0 * self.e02 * other.e01 * other.e0123 - 2.0 * self.e02 * other.e2 * other.e3 - 2.0 * self.e02 * other.e02 * other.e03 + 2.0 * self.e02 * other.e12 * other.e31 + 2.0 * self.e02 * other.e021 * other.e013 - 2.0 * self.e12 * other.s * other.e0123 + 2.0 * self.e12 * other.e0 * other.e123 - 2.0 * self.e12 * other.e1 * other.e032 - 2.0 * self.e12 * other.e01 * other.e23 - 2.0 * self.e12 * other.e2 * other.e013 - 2.0 * self.e12 * other.e02 * other.e31 + 2.0 * self.e12 * other.e12 * other.e03 + 2.0 * self.e12 * other.e021 * other.e3 - 2.0 * self.e021 * other.s * other.e123 - 2.0 * self.e021 * other.e0 * other.e0123 - 2.0 * self.e021 * other.e1 * other.e23 + 2.0 * self.e021 * other.e01 * other.e032 - 2.0 * self.e021 * other.e2 * other.e31 + 2.0 * self.e021 * other.e02 * other.e013 + 2.0 * self.e021 * other.e12 * other.e3 - 2.0 * self.e021 * other.e021 * other.e03 + self.e03 * other.s * other.s + self.e03 * other.e0 * other.e0 + self.e03 * other.e1 * other.e1 + self.e03 * other.e01 * other.e01 + self.e03 * other.e2 * other.e2 + self.e03 * other.e02 * other.e02 + self.e03 * other.e12 * other.e12 + self.e03 * other.e021 * other.e021 - self.e03 * other.e3 * other.e3 - self.e03 * other.e03 * other.e03 - self.e03 * other.e31 * other.e31 - self.e03 * other.e013 * other.e013 - self.e03 * other.e23 * other.e23 - self.e03 * other.e032 * other.e032 - self.e03 * other.e123 * other.e123 - self.e03 * other.e0123 * other.e0123 - 2.0 * self.e31 * other.s * other.e01 + 2.0 * self.e31 * other.e0 * other.e1 + 2.0 * self.e31 * other.e2 * other.e021 + 2.0 * self.e31 * other.e02 * other.e12 + 2.0 * self.e31 * other.e3 * other.e013 + 2.0 * self.e31 * other.e03 * other.e31 + 2.0 * self.e31 * other.e23 * other.e0123 + 2.0 * self.e31 * other.e032 * other.e123 - 2.0 * self.e013 * other.s * other.e1 - 2.0 * self.e013 * other.e0 * other.e01 + 2.0 * self.e013 * other.e2 * other.e12 - 2.0 * self.e013 * other.e02 * other.e021 + 2.0 * self.e013 * other.e3 * other.e31 - 2.0 * self.e013 * other.e03 * other.e013 + 2.0 * self.e013 * other.e23 * other.e123 - 2.0 * self.e013 * other.e032 * other.e0123 + 2.0 * self.e23 * other.s * other.e02 - 2.0 * self.e23 * other.e0 * other.e2 + 2.0 * self.e23 * other.e1 * other.e021 + 2.0 * self.e23 * other.e01 * other.e12 + 2.0 * self.e23 * other.e3 * other.e032 + 2.0 * self.e23 * other.e03 * other.e23 - 2.0 * self.e23 * other.e31 * other.e0123 - 2.0 * self.e23 * other.e013 * other.e123 + 2.0 * self.e032 * other.s * other.e2 + 2.0 * self.e032 * other.e0 * other.e02 + 2.0 * self.e032 * other.e1 * other.e12 - 2.0 * self.e032 * other.e01 * other.e021 + 2.0 * self.e032 * other.e3 * other.e23 - 2.0 * self.e032 * other.e03 * other.e032 - 2.0 * self.e032 * other.e31 * other.e123 + 2.0 * self.e032 * other.e013 * other.e0123 + 2.0 * self.e123 * other.s * other.e021 + 2.0 * self.e123 * other.e0 * other.e12 + 2.0 * self.e123 * other.e1 * other.e02 - 2.0 * self.e123 * other.e01 * other.e2 - 2.0 * self.e123 * other.e3 * other.e0123 + 2.0 * self.e123 * other.e03 * other.e123 + 2.0 * self.e123 * other.e31 * other.e032 - 2.0 * self.e123 * other.e013 * other.e23,
            e31: -2.0 * self.e01 * other.s * other.e03 - 2.0 * self.e01 * other.e0 * other.e3 - 2.0 * self.e01 * other.e1 * other.e013 + 2.0 * self.e01 * other.e01 * other.e31 - 2.0 * self.e01 * other.e2 * other.e032 + 2.0 * self.e01 * other.e02 * other.e23 + 2.0 * self.e01 * other.e12 * other.e0123 - 2.0 * self.e01 * other.e021 * other.e123 - 2.0 * self.e02 * other.s * other.e0123 - 2.0 * self.e02 * other.e0 * other.e123 + 2.0 * self.e02 * other.e1 * other.e032 - 2.0 * self.e02 * other.e01 * other.e23 - 2.0 * self.e02 * other.e2 * other.e013 + 2.0 * self.e02 * other.e02 * other.e31 - 2.0 * self.e02 * other.e12 * other.e03 + 2.0 * self.e02 * other.e021 * other.e3 + 2.0 * self.e12 * other.s * other.e23 - 2.0 * self.e12 * other.e0 * other.e032 - 2.0 * self.e12 * other.e1 * other.e123 - 2.0 * self.e12 * other.e01 * other.e0123 + 2.0 * self.e12 * other.e2 * other.e3 + 2.0 * self.e12 * other.e02 * other.e03 + 2.0 * self.e12 * other.e12 * other.e31 + 2.0 * self.e12 * other.e021 * other.e013 + 2.0 * self.e021 * other.s * other.e032 - 2.0 * self.e021 * other.e0 * other.e23 - 2.0 * self.e021 * other.e1 * other.e0123 - 2.0 * self.e021 * other.e01 * other.e123 + 2.0 * self.e021 * other.e2 * other.e03 + 2.0 * self.e021 * other.e02 * other.e3 + 2.0 * self.e021 * other.e12 * other.e013 + 2.0 * self.e021 * other.e021 * other.e31 + 2.0 * self.e03 * other.s * other.e01 + 2.0 * self.e03 * other.e0 * other.e1 - 2.0 * self.e03 * other.e2 * other.e021 + 2.0 * self.e03 * other.e02 * other.e12 - 2.0 * self.e03 * other.e3 * other.e013 + 2.0 * self.e03 * other.e03 * other.e31 - 2.0 * self.e03 * other.e23 * other.e0123 + 2.0 * self.e03 * other.e032 * other.e123 + self.e31 * other.s * other.s + self.e31 * other.e0 * other.e0 - self.e31 * other.e1 * other.e1 - self.e31 * other.e01 * other.e01 + self.e31 * other.e2 * other.e2 + self.e31 * other.e02 * other.e02 - self.e31 * other.e12 * other.e12 - self.e31 * other.e021 * other.e021 - self.e31 * other.e3 * other.e3 - self.e31 * other.e03 * other.e03 + self.e31 * other.e31 * other.e31 + self.e31 * other.e013 * other.e013 - self.e31 * other.e23 * other.e23 - self.e31 * other.e032 * other.e032 + self.e31 * other.e123 * other.e123 + self.e31 * other.e0123 * other.e0123 - 2.0 * self.e013 * other.s * other.e0 - 2.0 * self.e013 * other.e1 * other.e01 + 2.0 * self.e013 * other.e2 * other.e02 - 2.0 * self.e013 * other.e12 * other.e021 - 2.0 * self.e013 * other.e3 * other.e03 + 2.0 * self.e013 * other.e31 * other.e013 - 2.0 * self.e013 * other.e23 * other.e032 + 2.0 * self.e013 * other.e123 * other.e0123 - 2.0 * self.e23 * other.s * other.e12 + 2.0 * self.e23 * other.e0 * other.e021 + 2.0 * self.e23 * other.e1 * other.e2 + 2.0 * self.e23 * other.e01 * other.e02 + 2.0 * self.e23 * other.e3 * other.e123 + 2.0 * self.e23 * other.e03 * other.e0123 + 2.0 * self.e23 * other.e31 * other.e23 + 2.0 * self.e23 * other.e013 * other.e032 - 2.0 * self.e032 * other.s * other.e021 + 2.0 * self.e032 * other.e0 * other.e12 + 2.0 * self.e032 * other.e1 * other.e02 + 2.0 * self.e032 * other.e01 * other.e2 + 2.0 * self.e032 * other.e3 * other.e0123 + 2.0 * self.e032 * other.e03 * other.e123 + 2.0 * self.e032 * other.e31 * other.e032 + 2.0 * self.e032 * other.e013 * other.e23 + 2.0 * self.e123 * other.s * other.e2 + 2.0 * self.e123 * other.e0 * other.e02 - 2.0 * self.e123 * other.e1 * other.e12 + 2.0 * self.e123 * other.e01 * other.e021 + 2.0 * self.e123 * other.e3 * other.e23 - 2.0 * self.e123 * other.e03 * other.e032 + 2.0 * self.e123 * other.e31 * other.e123 - 2.0 * self.e123 * other.e013 * other.e0123,
            e013: 2.0 * self.e01 * other.s * other.e3 - 2.0 * self.e01 * other.e0 * other.e03 - 2.0 * self.e01 * other.e1 * other.e31 - 2.0 * self.e01 * other.e01 * other.e013 - 2.0 * self.e01 * other.e2 * other.e23 - 2.0 * self.e01 * other.e02 * other.e032 - 2.0 * self.e01 * other.e12 * other.e123 - 2.0 * self.e01 * other.e021 * other.e0123 + 2.0 * self.e02 * other.s * other.e123 - 2.0 * self.e02 * other.e0 * other.e0123 + 2.0 * self.e02 * other.e1 * other.e23 + 2.0 * self.e02 * other.e01 * other.e032 - 2.0 * self.e02 * other.e2 * other.e31 - 2.0 * self.e02 * other.e02 * other.e013 + 2.0 * self.e02 * other.e12 * other.e3 + 2.0 * self.e02 * other.e021 * other.e03 + 2.0 * self.e12 * other.s * other.e032 - 2.0 * self.e12 * other.e0 * other.e23 + 2.0 * self.e12 * other.e1 * other.e0123 + 2.0 * self.e12 * other.e01 * other.e123 - 2.0 * self.e12 * other.e2 * other.e03 - 2.0 * self.e12 * other.e02 * other.e3 + 2.0 * self.e12 * other.e12 * other.e013 + 2.0 * self.e12 * other.e021 * other.e31 + 2.0 * self.e021 * other.s * other.e23 + 2.0 * self.e021 * other.e0 * other.e032 + 2.0 * self.e021 * other.e1 * other.e123 - 2.0 * self.e021 * other.e01 * other.e0123 - 2.0 * self.e021 * other.e2 * other.e3 + 2.0 * self.e021 * other.e02 * other.e03 + 2.0 * self.e021 * other.e12 * other.e31 - 2.0 * self.e021 * other.e021 * other.e013 - 2.0 * self.e03 * other.s * other.e1 + 2.0 * self.e03 * other.e0 * other.e01 - 2.0 * self.e03 * other.e2 * other.e12 - 2.0 * self.e03 * other.e02 * other.e021 - 2.0 * self.e03 * other.e3 * other.e31 - 2.0 * self.e03 * other.e03 * other.e013 + 2.0 * self.e03 * other.e23 * other.e123 + 2.0 * self.e03 * other.e032 * other.e0123 - 2.0 * self.e31 * other.s * other.e0 + 2.0 * self.e31 * other.e1 * other.e01 - 2.0 * self.e31 * other.e2 * other.e02 - 2.0 * self.e31 * other.e12 * other.e021 + 2.0 * self.e31 * other.e3 * other.e03 + 2.0 * self.e31 * other.e31 * other.e013 - 2.0 * self.e31 * other.e23 * other.e032 - 2.0 * self.e31 * other.e123 * other.e0123 + self.e013 * other.s * other.s - self.e013 * other.e0 * other.e0 + self.e013 * other.e1 * other.e1 - self.e013 * other.e01 * other.e01 - self.e013 * other.e2 * other.e2 + self.e013 * other.e02 * other.e02 - self.e013 * other.e12 * other.e12 + self.e013 * other.e021 * other.e021 + self.e013 * other.e3 * other.e3 - self.e013 * other.e03 * other.e03 + self.e013 * other.e31 * other.e31 - self.e013 * other.e013 * other.e013 - self.e013 * other.e23 * other.e23 + self.e013 * other.e032 * other.e032 - self.e013 * other.e123 * other.e123 + self.e013 * other.e0123 * other.e0123 - 2.0 * self.e23 * other.s * other.e021 + 2.0 * self.e23 * other.e0 * other.e12 - 2.0 * self.e23 * other.e1 * other.e02 - 2.0 * self.e23 * other.e01 * other.e2 - 2.0 * self.e23 * other.e3 * other.e0123 - 2.0 * self.e23 * other.e03 * other.e123 + 2.0 * self.e23 * other.e31 * other.e032 + 2.0 * self.e23 * other.e013 * other.e23 - 2.0 * self.e032 * other.s * other.e12 - 2.0 * self.e032 * other.e0 * other.e021 - 2.0 * self.e032 * other.e1 * other.e2 + 2.0 * self.e032 * other.e01 * other.e02 - 2.0 * self.e032 * other.e3 * other.e123 + 2.0 * self.e032 * other.e03 * other.e0123 + 2.0 * self.e032 * other.e31 * other.e23 - 2.0 * self.e032 * other.e013 * other.e032 - 2.0 * self.e123 * other.s * other.e02 - 2.0 * self.e123 * other.e0 * other.e2 - 2.0 * self.e123 * other.e1 * other.e021 + 2.0 * self.e123 * other.e01 * other.e12 + 2.0 * self.e123 * other.e3 * other.e032 - 2.0 * self.e123 * other.e03 * other.e23 - 2.0 * self.e123 * other.e31 * other.e0123 + 2.0 * self.e123 * other.e013 * other.e123,
            e23: -2.0 * self.e01 * other.s * other.e0123 - 2.0 * self.e01 * other.e0 * other.e123 - 2.0 * self.e01 * other.e1 * other.e032 + 2.0 * self.e01 * other.e01 * other.e23 + 2.0 * self.e01 * other.e2 * other.e013 - 2.0 * self.e01 * other.e02 * other.e31 - 2.0 * self.e01 * other.e12 * other.e03 + 2.0 * self.e01 * other.e021 * other.e3 + 2.0 * self.e02 * other.s * other.e03 + 2.0 * self.e02 * other.e0 * other.e3 - 2.0 * self.e02 * other.e1 * other.e013 + 2.0 * self.e02 * other.e01 * other.e31 - 2.0 * self.e02 * other.e2 * other.e032 + 2.0 * self.e02 * other.e02 * other.e23 - 2.0 * self.e02 * other.e12 * other.e0123 + 2.0 * self.e02 * other.e021 * other.e123 - 2.0 * self.e12 * other.s * other.e31 + 2.0 * self.e12 * other.e0 * other.e013 + 2.0 * self.e12 * other.e1 * other.e3 + 2.0 * self.e12 * other.e01 * other.e03 + 2.0 * self.e12 * other.e2 * other.e123 + 2.0 * self.e12 * other.e02 * other.e0123 + 2.0 * self.e12 * other.e12 * other.e23 + 2.0 * self.e12 * other.e021 * other.e032 - 2.0 * self.e021 * other.s * other.e013 + 2.0 * self.e021 * other.e0 * other.e31 + 2.0 * self.e021 * other.e1 * other.e03 + 2.0 * self.e021 * other.e01 * other.e3 + 2.0 * self.e021 * other.e2 * other.e0123 + 2.0 * self.e021 * other.e02 * other.e123 + 2.0 * self.e021 * other.e12 * other.e032 + 2.0 * self.e021 * other.e021 * other.e23 - 2.0 * self.e03 * other.s * other.e02 - 2.0 * self.e03 * other.e0 * other.e2 - 2.0 * self.e03 * other.e1 * other.e021 + 2.0 * self.e03 * other.e01 * other.e12 - 2.0 * self.e03 * other.e3 * other.e032 + 2.0 * self.e03 * other.e03 * other.e23 + 2.0 * self.e03 * other.e31 * other.e0123 - 2.0 * self.e03 * other.e013 * other.e123 + 2.0 * self.e31 * other.s * other.e12 - 2.0 * self.e31 * other.e0 * other.e021 + 2.0 * self.e31 * other.e1 * other.e2 + 2.0 * self.e31 * other.e01 * other.e02 - 2.0 * self.e31 * other.e3 * other.e123 - 2.0 * self.e31 * other.e03 * other.e0123 + 2.0 * self.e31 * other.e31 * other.e23 + 2.0 * self.e31 * other.e013 * other.e032 + 2.0 * self.e013 * other.s * other.e021 - 2.0 * self.e013 * other.e0 * other.e12 + 2.0 * self.e013 * other.e1 * other.e02 + 2.0 * self.e013 * other.e01 * other.e2 - 2.0 * self.e013 * other.e3 * other.e0123 - 2.0 * self.e013 * other.e03 * other.e123 + 2.0 * self.e013 * other.e31 * other.e032 + 2.0 * self.e013 * other.e013 * other.e23 + self.e23 * other.s * other.s + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1 + self.e23 * other.e01 * other.e01 - self.e23 * other.e2 * other.e2 - self.e23 * other.e02 * other.e02 - self.e23 * other.e12 * other.e12 - self.e23 * other.e021 * other.e021 - self.e23 * other.e3 * other.e3 - self.e23 * other.e03 * other.e03 - self.e23 * other.e31 * other.e31 - self.e23 * other.e013 * other.e013 + self.e23 * other.e23 * other.e23 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123 + self.e23 * other.e0123 * other.e0123 - 2.0 * self.e032 * other.s * other.e0 + 2.0 * self.e032 * other.e1 * other.e01 - 2.0 * self.e032 * other.e2 * other.e02 - 2.0 * self.e032 * other.e12 * other.e021 - 2.0 * self.e032 * other.e3 * other.e03 - 2.0 * self.e032 * other.e31 * other.e013 + 2.0 * self.e032 * other.e23 * other.e032 + 2.0 * self.e032 * other.e123 * other.e0123 + 2.0 * self.e123 * other.s * other.e1 + 2.0 * self.e123 * other.e0 * other.e01 + 2.0 * self.e123 * other.e2 * other.e12 - 2.0 * self.e123 * other.e02 * other.e021 - 2.0 * self.e123 * other.e3 * other.e31 + 2.0 * self.e123 * other.e03 * other.e013 + 2.0 * self.e123 * other.e23 * other.e123 - 2.0 * self.e123 * other.e032 * other.e0123,
            e032: 2.0 * self.e01 * other.s * other.e123 - 2.0 * self.e01 * other.e0 * other.e0123 - 2.0 * self.e01 * other.e1 * other.e23 - 2.0 * self.e01 * other.e01 * other.e032 + 2.0 * self.e01 * other.e2 * other.e31 + 2.0 * self.e01 * other.e02 * other.e013 + 2.0 * self.e01 * other.e12 * other.e3 + 2.0 * self.e01 * other.e021 * other.e03 - 2.0 * self.e02 * other.s * other.e3 + 2.0 * self.e02 * other.e0 * other.e03 - 2.0 * self.e02 * other.e1 * other.e31 - 2.0 * self.e02 * other.e01 * other.e013 - 2.0 * self.e02 * other.e2 * other.e23 - 2.0 * self.e02 * other.e02 * other.e032 + 2.0 * self.e02 * other.e12 * other.e123 + 2.0 * self.e02 * other.e021 * other.e0123 - 2.0 * self.e12 * other.s * other.e013 + 2.0 * self.e12 * other.e0 * other.e31 - 2.0 * self.e12 * other.e1 * other.e03 - 2.0 * self.e12 * other.e01 * other.e3 - 2.0 * self.e12 * other.e2 * other.e0123 - 2.0 * self.e12 * other.e02 * other.e123 + 2.0 * self.e12 * other.e12 * other.e032 + 2.0 * self.e12 * other.e021 * other.e23 - 2.0 * self.e021 * other.s * other.e31 - 2.0 * self.e021 * other.e0 * other.e013 - 2.0 * self.e021 * other.e1 * other.e3 + 2.0 * self.e021 * other.e01 * other.e03 - 2.0 * self.e021 * other.e2 * other.e123 + 2.0 * self.e021 * other.e02 * other.e0123 + 2.0 * self.e021 * other.e12 * other.e23 - 2.0 * self.e021 * other.e021 * other.e032 + 2.0 * self.e03 * other.s * other.e2 - 2.0 * self.e03 * other.e0 * other.e02 - 2.0 * self.e03 * other.e1 * other.e12 - 2.0 * self.e03 * other.e01 * other.e021 - 2.0 * self.e03 * other.e3 * other.e23 - 2.0 * self.e03 * other.e03 * other.e032 - 2.0 * self.e03 * other.e31 * other.e123 - 2.0 * self.e03 * other.e013 * other.e0123 + 2.0 * self.e31 * other.s * other.e021 - 2.0 * self.e31 * other.e0 * other.e12 - 2.0 * self.e31 * other.e1 * other.e02 - 2.0 * self.e31 * other.e01 * other.e2 + 2.0 * self.e31 * other.e3 * other.e0123 + 2.0 * self.e31 * other.e03 * other.e123 + 2.0 * self.e31 * other.e31 * other.e032 + 2.0 * self.e31 * other.e013 * other.e23 + 2.0 * self.e013 * other.s * other.e12 + 2.0 * self.e013 * other.e0 * other.e021 - 2.0 * self.e013 * other.e1 * other.e2 + 2.0 * self.e013 * other.e01 * other.e02 + 2.0 * self.e013 * other.e3 * other.e123 - 2.0 * self.e013 * other.e03 * other.e0123 + 2.0 * self.e013 * other.e31 * other.e23 - 2.0 * self.e013 * other.e013 * other.e032 - 2.0 * self.e23 * other.s * other.e0 - 2.0 * self.e23 * other.e1 * other.e01 + 2.0 * self.e23 * other.e2 * other.e02 - 2.0 * self.e23 * other.e12 * other.e021 + 2.0 * self.e23 * other.e3 * other.e03 - 2.0 * self.e23 * other.e31 * other.e013 + 2.0 * self.e23 * other.e23 * other.e032 - 2.0 * self.e23 * other.e123 * other.e0123 + self.e032 * other.s * other.s - self.e032 * other.e0 * other.e0 - self.e032 * other.e1 * other.e1 + self.e032 * other.e01 * other.e01 + self.e032 * other.e2 * other.e2 - self.e032 * other.e02 * other.e02 - self.e032 * other.e12 * other.e12 + self.e032 * other.e021 * other.e021 + self.e032 * other.e3 * other.e3 - self.e032 * other.e03 * other.e03 - self.e032 * other.e31 * other.e31 + self.e032 * other.e013 * other.e013 + self.e032 * other.e23 * other.e23 - self.e032 * other.e032 * other.e032 - self.e032 * other.e123 * other.e123 + self.e032 * other.e0123 * other.e0123 - 2.0 * self.e123 * other.s * other.e01 - 2.0 * self.e123 * other.e0 * other.e1 + 2.0 * self.e123 * other.e2 * other.e021 - 2.0 * self.e123 * other.e02 * other.e12 - 2.0 * self.e123 * other.e3 * other.e013 + 2.0 * self.e123 * other.e03 * other.e31 - 2.0 * self.e123 * other.e23 * other.e0123 + 2.0 * self.e123 * other.e032 * other.e123,
            e123: -2.0 * self.e01 * other.s * other.e032 - 2.0 * self.e01 * other.e0 * other.e23 - 2.0 * self.e01 * other.e1 * other.e0123 + 2.0 * self.e01 * other.e01 * other.e123 - 2.0 * self.e01 * other.e2 * other.e03 + 2.0 * self.e01 * other.e02 * other.e3 + 2.0 * self.e01 * other.e12 * other.e013 - 2.0 * self.e01 * other.e021 * other.e31 - 2.0 * self.e02 * other.s * other.e013 - 2.0 * self.e02 * other.e0 * other.e31 + 2.0 * self.e02 * other.e1 * other.e03 - 2.0 * self.e02 * other.e01 * other.e3 - 2.0 * self.e02 * other.e2 * other.e0123 + 2.0 * self.e02 * other.e02 * other.e123 - 2.0 * self.e02 * other.e12 * other.e032 + 2.0 * self.e02 * other.e021 * other.e23 + 2.0 * self.e12 * other.s * other.e3 - 2.0 * self.e12 * other.e0 * other.e03 - 2.0 * self.e12 * other.e1 * other.e31 - 2.0 * self.e12 * other.e01 * other.e013 + 2.0 * self.e12 * other.e2 * other.e23 + 2.0 * self.e12 * other.e02 * other.e032 + 2.0 * self.e12 * other.e12 * other.e123 + 2.0 * self.e12 * other.e021 * other.e0123 + 2.0 * self.e021 * other.s * other.e03 - 2.0 * self.e021 * other.e0 * other.e3 - 2.0 * self.e021 * other.e1 * other.e013 - 2.0 * self.e021 * other.e01 * other.e31 + 2.0 * self.e021 * other.e2 * other.e032 + 2.0 * self.e021 * other.e02 * other.e23 + 2.0 * self.e021 * other.e12 * other.e0123 + 2.0 * self.e021 * other.e021 * other.e123 - 2.0 * self.e03 * other.s * other.e021 - 2.0 * self.e03 * other.e0 * other.e12 - 2.0 * self.e03 * other.e1 * other.e02 + 2.0 * self.e03 * other.e01 * other.e2 - 2.0 * self.e03 * other.e3 * other.e0123 + 2.0 * self.e03 * other.e03 * other.e123 + 2.0 * self.e03 * other.e31 * other.e032 - 2.0 * self.e03 * other.e013 * other.e23 + 2.0 * self.e31 * other.s * other.e2 - 2.0 * self.e31 * other.e0 * other.e02 + 2.0 * self.e31 * other.e1 * other.e12 + 2.0 * self.e31 * other.e01 * other.e021 - 2.0 * self.e31 * other.e3 * other.e23 - 2.0 * self.e31 * other.e03 * other.e032 + 2.0 * self.e31 * other.e31 * other.e123 + 2.0 * self.e31 * other.e013 * other.e0123 + 2.0 * self.e013 * other.s * other.e02 - 2.0 * self.e013 * other.e0 * other.e2 + 2.0 * self.e013 * other.e1 * other.e021 + 2.0 * self.e013 * other.e01 * other.e12 - 2.0 * self.e013 * other.e3 * other.e032 - 2.0 * self.e013 * other.e03 * other.e23 + 2.0 * self.e013 * other.e31 * other.e0123 + 2.0 * self.e013 * other.e013 * other.e123 + 2.0 * self.e23 * other.s * other.e1 - 2.0 * self.e23 * other.e0 * other.e01 - 2.0 * self.e23 * other.e2 * other.e12 - 2.0 * self.e23 * other.e02 * other.e021 + 2.0 * self.e23 * other.e3 * other.e31 + 2.0 * self.e23 * other.e03 * other.e013 + 2.0 * self.e23 * other.e23 * other.e123 + 2.0 * self.e23 * other.e032 * other.e0123 + 2.0 * self.e032 * other.s * other.e01 - 2.0 * self.e032 * other.e0 * other.e1 - 2.0 * self.e032 * other.e2 * other.e021 - 2.0 * self.e032 * other.e02 * other.e12 + 2.0 * self.e032 * other.e3 * other.e013 + 2.0 * self.e032 * other.e03 * other.e31 + 2.0 * self.e032 * other.e23 * other.e0123 + 2.0 * self.e032 * other.e032 * other.e123 + self.e123 * other.s * other.s - self.e123 * other.e0 * other.e0 + self.e123 * other.e1 * other.e1 - self.e123 * other.e01 * other.e01 + self.e123 * other.e2 * other.e2 - self.e123 * other.e02 * other.e02 + self.e123 * other.e12 * other.e12 - self.e123 * other.e021 * other.e021 + self.e123 * other.e3 * other.e3 - self.e123 * other.e03 * other.e03 + self.e123 * other.e31 * other.e31 - self.e123 * other.e013 * other.e013 + self.e123 * other.e23 * other.e23 - self.e123 * other.e032 * other.e032 + self.e123 * other.e123 * other.e123 - self.e123 * other.e0123 * other.e0123,
            e0123: 2.0 * self.s * other.s * other.e0123 - 2.0 * self.s * other.e0 * other.e123 - 2.0 * self.s * other.e1 * other.e032 - 2.0 * self.s * other.e01 * other.e23 - 2.0 * self.s * other.e2 * other.e013 - 2.0 * self.s * other.e02 * other.e31 - 2.0 * self.s * other.e12 * other.e03 - 2.0 * self.s * other.e021 * other.e3 - 2.0 * self.e0 * other.s * other.e123 - 2.0 * self.e0 * other.e0 * other.e0123 + 2.0 * self.e0 * other.e1 * other.e23 - 2.0 * self.e0 * other.e01 * other.e032 + 2.0 * self.e0 * other.e2 * other.e31 - 2.0 * self.e0 * other.e02 * other.e013 + 2.0 * self.e0 * other.e12 * other.e3 - 2.0 * self.e0 * other.e021 * other.e03 - 2.0 * self.e1 * other.s * other.e032 - 2.0 * self.e1 * other.e0 * other.e23 + 2.0 * self.e1 * other.e1 * other.e0123 - 2.0 * self.e1 * other.e01 * other.e123 + 2.0 * self.e1 * other.e2 * other.e03 - 2.0 * self.e1 * other.e02 * other.e3 + 2.0 * self.e1 * other.e12 * other.e013 - 2.0 * self.e1 * other.e021 * other.e31 - 2.0 * self.e2 * other.s * other.e013 - 2.0 * self.e2 * other.e0 * other.e31 - 2.0 * self.e2 * other.e1 * other.e03 + 2.0 * self.e2 * other.e01 * other.e3 + 2.0 * self.e2 * other.e2 * other.e0123 - 2.0 * self.e2 * other.e02 * other.e123 - 2.0 * self.e2 * other.e12 * other.e032 + 2.0 * self.e2 * other.e021 * other.e23 - 2.0 * self.e3 * other.s * other.e021 - 2.0 * self.e3 * other.e0 * other.e12 + 2.0 * self.e3 * other.e1 * other.e02 - 2.0 * self.e3 * other.e01 * other.e2 + 2.0 * self.e3 * other.e3 * other.e0123 - 2.0 * self.e3 * other.e03 * other.e123 + 2.0 * self.e3 * other.e31 * other.e032 - 2.0 * self.e3 * other.e013 * other.e23 + self.e0123 * other.s * other.s + self.e0123 * other.e0 * other.e0 - self.e0123 * other.e1 * other.e1 - self.e0123 * other.e01 * other.e01 - self.e0123 * other.e2 * other.e2 - self.e0123 * other.e02 * other.e02 + self.e0123 * other.e12 * other.e12 + self.e0123 * other.e021 * other.e021 - self.e0123 * other.e3 * other.e3 - self.e0123 * other.e03 * other.e03 + self.e0123 * other.e31 * other.e31 + self.e0123 * other.e013 * other.e013 + self.e0123 * other.e23 * other.e23 + self.e0123 * other.e032 * other.e032 - self.e0123 * other.e123 * other.e123 - self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Scalar> for Scalar {
    type Output = Null;
    fn project(self, other: Scalar) -> Null {
        Null
    }
}

impl Project<Vector> for Scalar {
    type Output = Scalar;
    fn project(self, other: Vector) -> Scalar {
        Scalar {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e3 * other.e3,
        }
    }
}

impl Project<Bivector> for Scalar {
    type Output = Scalar;
    fn project(self, other: Bivector) -> Scalar {
        Scalar {
            s: self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23,
        }
    }
}

impl Project<Trivector> for Scalar {
    type Output = Scalar;
    fn project(self, other: Trivector) -> Scalar {
        Scalar {
            s: self.s * other.e021 * other.e021 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123,
        }
    }
}

impl Project<FourVector> for Scalar {
    type Output = Scalar;
    fn project(self, other: FourVector) -> Scalar {
        Scalar {
            s: self.s * other.e0123 * other.e0123,
        }
    }
}

impl Project<Null> for Scalar {
    type Output = EvenMultivector;
    fn project(self, other: Null) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Project<OddMultivector> for Scalar {
    type Output = EvenMultivector;
    fn project(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123,
            e01: self.s * other.e2 * other.e021 - self.s * other.e3 * other.e013,
            e02: -self.s * other.e1 * other.e021 + self.s * other.e3 * other.e032,
            e12: self.s * other.e0 * other.e021 - self.s * other.e3 * other.e123,
            e03: self.s * other.e1 * other.e013 - self.s * other.e2 * other.e032,
            e31: self.s * other.e0 * other.e013 - self.s * other.e2 * other.e123,
            e23: self.s * other.e0 * other.e032 - self.s * other.e1 * other.e123,
            e0123: 0.0,
        }
    }
}

impl Project<EvenMultivector> for Scalar {
    type Output = EvenMultivector;
    fn project(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s * other.s + self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23 + self.s * other.e0123 * other.e0123,
            e01: -self.s * other.s * other.e01 - self.s * other.e23 * other.e0123,
            e02: -self.s * other.s * other.e02 - self.s * other.e31 * other.e0123,
            e12: -self.s * other.s * other.e12 - self.s * other.e03 * other.e0123,
            e03: -self.s * other.s * other.e03 - self.s * other.e12 * other.e0123,
            e31: -self.s * other.s * other.e31 - self.s * other.e02 * other.e0123,
            e23: -self.s * other.s * other.e23 - self.s * other.e01 * other.e0123,
            e0123: self.s * other.s * other.e0123,
        }
    }
}

impl Project<Multivector> for Scalar {
    type Output = EvenMultivector;
    fn project(self, other: Multivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01 + self.s * other.e2 * other.e2 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e013 * other.e013 + self.s * other.e23 * other.e23 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 + self.s * other.e0123 * other.e0123,
            e01: -self.s * other.s * other.e01 + self.s * other.e2 * other.e021 - self.s * other.e3 * other.e013 - self.s * other.e23 * other.e0123,
            e02: -self.s * other.s * other.e02 - self.s * other.e1 * other.e021 + self.s * other.e3 * other.e032 - self.s * other.e31 * other.e0123,
            e12: -self.s * other.s * other.e12 + self.s * other.e0 * other.e021 - self.s * other.e3 * other.e123 - self.s * other.e03 * other.e0123,
            e03: -self.s * other.s * other.e03 + self.s * other.e1 * other.e013 - self.s * other.e2 * other.e032 - self.s * other.e12 * other.e0123,
            e31: -self.s * other.s * other.e31 + self.s * other.e0 * other.e013 - self.s * other.e2 * other.e123 - self.s * other.e02 * other.e0123,
            e23: -self.s * other.s * other.e23 + self.s * other.e0 * other.e032 - self.s * other.e1 * other.e123 - self.s * other.e01 * other.e0123,
            e0123: self.s * other.s * other.e0123,
        }
    }
}

impl Project<Scalar> for Vector {
    type Output = Null;
    fn project(self, other: Scalar) -> Null {
        Null
    }
}

impl Project<Vector> for Vector {
    type Output = Null;
    fn project(self, other: Vector) -> Null {
        Null
    }
}

impl Project<Bivector> for Vector {
    type Output = Vector;
    fn project(self, other: Bivector) -> Vector {
        Vector {
            e0: -self.e0 * other.e01 * other.e01 - self.e0 * other.e02 * other.e02 - self.e0 * other.e03 * other.e03 + self.e1 * other.e02 * other.e12 - self.e1 * other.e03 * other.e31 - self.e2 * other.e01 * other.e12 + self.e2 * other.e03 * other.e23 + self.e3 * other.e01 * other.e31 - self.e3 * other.e02 * other.e23,
            e1: self.e0 * other.e02 * other.e12 - self.e0 * other.e03 * other.e31 + self.e1 * other.e01 * other.e01 + self.e1 * other.e12 * other.e12 + self.e1 * other.e31 * other.e31 + self.e2 * other.e01 * other.e02 - self.e2 * other.e31 * other.e23 + self.e3 * other.e01 * other.e03 - self.e3 * other.e12 * other.e23,
            e2: -self.e0 * other.e01 * other.e12 + self.e0 * other.e03 * other.e23 + self.e1 * other.e01 * other.e02 - self.e1 * other.e31 * other.e23 + self.e2 * other.e02 * other.e02 + self.e2 * other.e12 * other.e12 + self.e2 * other.e23 * other.e23 + self.e3 * other.e02 * other.e03 - self.e3 * other.e12 * other.e31,
            e3: self.e0 * other.e01 * other.e31 - self.e0 * other.e02 * other.e23 + self.e1 * other.e01 * other.e03 - self.e1 * other.e12 * other.e23 + self.e2 * other.e02 * other.e03 - self.e2 * other.e12 * other.e31 + self.e3 * other.e03 * other.e03 + self.e3 * other.e31 * other.e31 + self.e3 * other.e23 * other.e23,
        }
    }
}

impl Project<Trivector> for Vector {
    type Output = Vector;
    fn project(self, other: Trivector) -> Vector {
        Vector {
            e0: -self.e0 * other.e021 * other.e021 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 - self.e1 * other.e032 * other.e123 - self.e2 * other.e013 * other.e123 - self.e3 * other.e021 * other.e123,
            e1: -self.e0 * other.e032 * other.e123 + self.e1 * other.e021 * other.e021 + self.e1 * other.e013 * other.e013 + self.e1 * other.e123 * other.e123 - self.e2 * other.e013 * other.e032 - self.e3 * other.e021 * other.e032,
            e2: -self.e0 * other.e013 * other.e123 - self.e1 * other.e013 * other.e032 + self.e2 * other.e021 * other.e021 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 - self.e3 * other.e021 * other.e013,
            e3: -self.e0 * other.e021 * other.e123 - self.e1 * other.e021 * other.e032 - self.e2 * other.e021 * other.e013 + self.e3 * other.e013 * other.e013 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123,
        }
    }
}

impl Project<FourVector> for Vector {
    type Output = Vector;
    fn project(self, other: FourVector) -> Vector {
        Vector {
            e0: -self.e0 * other.e0123 * other.e0123,
            e1: self.e1 * other.e0123 * other.e0123,
            e2: self.e2 * other.e0123 * other.e0123,
            e3: self.e3 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Null> for Vector {
    type Output = OddMultivector;
    fn project(self, other: Null) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Project<OddMultivector> for Vector {
    type Output = OddMultivector;
    fn project(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e0 * other.e0 - self.e0 * other.e021 * other.e021 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 + self.e1 * other.e0 * other.e1 - self.e1 * other.e032 * other.e123 + self.e2 * other.e0 * other.e2 - self.e2 * other.e013 * other.e123 + self.e3 * other.e0 * other.e3 - self.e3 * other.e021 * other.e123,
            e1: self.e0 * other.e0 * other.e1 - self.e0 * other.e032 * other.e123 + self.e1 * other.e1 * other.e1 + self.e1 * other.e021 * other.e021 + self.e1 * other.e013 * other.e013 + self.e1 * other.e123 * other.e123 + self.e2 * other.e1 * other.e2 - self.e2 * other.e013 * other.e032 + self.e3 * other.e1 * other.e3 - self.e3 * other.e021 * other.e032,
            e2: self.e0 * other.e0 * other.e2 - self.e0 * other.e013 * other.e123 + self.e1 * other.e1 * other.e2 - self.e1 * other.e013 * other.e032 + self.e2 * other.e2 * other.e2 + self.e2 * other.e021 * other.e021 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 + self.e3 * other.e2 * other.e3 - self.e3 * other.e021 * other.e013,
            e021: self.e0 * other.e0 * other.e021 - self.e1 * other.e1 * other.e021 - self.e2 * other.e2 * other.e021 - self.e3 * other.e021 * other.e3,
            e3: self.e0 * other.e0 * other.e3 - self.e0 * other.e021 * other.e123 + self.e1 * other.e1 * other.e3 - self.e1 * other.e021 * other.e032 + self.e2 * other.e2 * other.e3 - self.e2 * other.e021 * other.e013 + self.e3 * other.e3 * other.e3 + self.e3 * other.e013 * other.e013 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123,
            e013: self.e0 * other.e0 * other.e013 - self.e1 * other.e1 * other.e013 - self.e2 * other.e2 * other.e013 - self.e3 * other.e3 * other.e013,
            e032: self.e0 * other.e0 * other.e032 - self.e1 * other.e1 * other.e032 - self.e2 * other.e2 * other.e032 - self.e3 * other.e3 * other.e032,
            e123: -self.e0 * other.e0 * other.e123 - self.e1 * other.e1 * other.e123 - self.e2 * other.e2 * other.e123 - self.e3 * other.e3 * other.e123,
        }
    }
}

impl Project<EvenMultivector> for Vector {
    type Output = OddMultivector;
    fn project(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e01 * other.e01 - self.e0 * other.e02 * other.e02 - self.e0 * other.e03 * other.e03 - self.e0 * other.e0123 * other.e0123 + self.e1 * other.e02 * other.e12 - self.e1 * other.e03 * other.e31 - self.e2 * other.e01 * other.e12 + self.e2 * other.e03 * other.e23 + self.e3 * other.e01 * other.e31 - self.e3 * other.e02 * other.e23,
            e1: self.e0 * other.e02 * other.e12 - self.e0 * other.e03 * other.e31 + self.e1 * other.e01 * other.e01 + self.e1 * other.e12 * other.e12 + self.e1 * other.e31 * other.e31 + self.e1 * other.e0123 * other.e0123 + self.e2 * other.e01 * other.e02 - self.e2 * other.e31 * other.e23 + self.e3 * other.e01 * other.e03 - self.e3 * other.e12 * other.e23,
            e2: -self.e0 * other.e01 * other.e12 + self.e0 * other.e03 * other.e23 + self.e1 * other.e01 * other.e02 - self.e1 * other.e31 * other.e23 + self.e2 * other.e02 * other.e02 + self.e2 * other.e12 * other.e12 + self.e2 * other.e23 * other.e23 + self.e2 * other.e0123 * other.e0123 + self.e3 * other.e02 * other.e03 - self.e3 * other.e12 * other.e31,
            e021: -self.e0 * other.e03 * other.e0123 - self.e1 * other.e31 * other.e0123 + self.e2 * other.e23 * other.e0123,
            e3: self.e0 * other.e01 * other.e31 - self.e0 * other.e02 * other.e23 + self.e1 * other.e01 * other.e03 - self.e1 * other.e12 * other.e23 + self.e2 * other.e02 * other.e03 - self.e2 * other.e12 * other.e31 + self.e3 * other.e03 * other.e03 + self.e3 * other.e31 * other.e31 + self.e3 * other.e23 * other.e23 + self.e3 * other.e0123 * other.e0123,
            e013: -self.e0 * other.e02 * other.e0123 + self.e1 * other.e12 * other.e0123 - self.e3 * other.e23 * other.e0123,
            e032: -self.e0 * other.e01 * other.e0123 - self.e2 * other.e12 * other.e0123 + self.e3 * other.e31 * other.e0123,
            e123: -self.e1 * other.e01 * other.e0123 - self.e2 * other.e02 * other.e0123 - self.e3 * other.e03 * other.e0123,
        }
    }
}

impl Project<Multivector> for Vector {
    type Output = OddMultivector;
    fn project(self, other: Multivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e0 * other.e0 - self.e0 * other.e01 * other.e01 - self.e0 * other.e02 * other.e02 - self.e0 * other.e021 * other.e021 - self.e0 * other.e03 * other.e03 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 - self.e0 * other.e0123 * other.e0123 + self.e1 * other.e0 * other.e1 + self.e1 * other.e02 * other.e12 - self.e1 * other.e03 * other.e31 - self.e1 * other.e032 * other.e123 + self.e2 * other.e0 * other.e2 - self.e2 * other.e01 * other.e12 + self.e2 * other.e03 * other.e23 - self.e2 * other.e013 * other.e123 + self.e3 * other.e0 * other.e3 + self.e3 * other.e01 * other.e31 - self.e3 * other.e02 * other.e23 - self.e3 * other.e021 * other.e123,
            e1: self.e0 * other.e0 * other.e1 + self.e0 * other.e02 * other.e12 - self.e0 * other.e03 * other.e31 - self.e0 * other.e032 * other.e123 + self.e1 * other.e1 * other.e1 + self.e1 * other.e01 * other.e01 + self.e1 * other.e12 * other.e12 + self.e1 * other.e021 * other.e021 + self.e1 * other.e31 * other.e31 + self.e1 * other.e013 * other.e013 + self.e1 * other.e123 * other.e123 + self.e1 * other.e0123 * other.e0123 + self.e2 * other.e1 * other.e2 + self.e2 * other.e01 * other.e02 - self.e2 * other.e31 * other.e23 - self.e2 * other.e013 * other.e032 + self.e3 * other.e1 * other.e3 + self.e3 * other.e01 * other.e03 - self.e3 * other.e12 * other.e23 - self.e3 * other.e021 * other.e032,
            e2: self.e0 * other.e0 * other.e2 - self.e0 * other.e01 * other.e12 + self.e0 * other.e03 * other.e23 - self.e0 * other.e013 * other.e123 + self.e1 * other.e1 * other.e2 + self.e1 * other.e01 * other.e02 - self.e1 * other.e31 * other.e23 - self.e1 * other.e013 * other.e032 + self.e2 * other.e2 * other.e2 + self.e2 * other.e02 * other.e02 + self.e2 * other.e12 * other.e12 + self.e2 * other.e021 * other.e021 + self.e2 * other.e23 * other.e23 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 + self.e2 * other.e0123 * other.e0123 + self.e3 * other.e2 * other.e3 + self.e3 * other.e02 * other.e03 - self.e3 * other.e12 * other.e31 - self.e3 * other.e021 * other.e013,
            e021: self.e0 * other.e0 * other.e021 - self.e0 * other.e03 * other.e0123 - self.e1 * other.e1 * other.e021 - self.e1 * other.e31 * other.e0123 - self.e2 * other.e2 * other.e021 + self.e2 * other.e23 * other.e0123 - self.e3 * other.e021 * other.e3,
            e3: self.e0 * other.e0 * other.e3 + self.e0 * other.e01 * other.e31 - self.e0 * other.e02 * other.e23 - self.e0 * other.e021 * other.e123 + self.e1 * other.e1 * other.e3 + self.e1 * other.e01 * other.e03 - self.e1 * other.e12 * other.e23 - self.e1 * other.e021 * other.e032 + self.e2 * other.e2 * other.e3 + self.e2 * other.e02 * other.e03 - self.e2 * other.e12 * other.e31 - self.e2 * other.e021 * other.e013 + self.e3 * other.e3 * other.e3 + self.e3 * other.e03 * other.e03 + self.e3 * other.e31 * other.e31 + self.e3 * other.e013 * other.e013 + self.e3 * other.e23 * other.e23 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123 + self.e3 * other.e0123 * other.e0123,
            e013: self.e0 * other.e0 * other.e013 - self.e0 * other.e02 * other.e0123 - self.e1 * other.e1 * other.e013 + self.e1 * other.e12 * other.e0123 - self.e2 * other.e2 * other.e013 - self.e3 * other.e3 * other.e013 - self.e3 * other.e23 * other.e0123,
            e032: self.e0 * other.e0 * other.e032 - self.e0 * other.e01 * other.e0123 - self.e1 * other.e1 * other.e032 - self.e2 * other.e2 * other.e032 - self.e2 * other.e12 * other.e0123 - self.e3 * other.e3 * other.e032 + self.e3 * other.e31 * other.e0123,
            e123: -self.e0 * other.e0 * other.e123 - self.e1 * other.e1 * other.e123 - self.e1 * other.e01 * other.e0123 - self.e2 * other.e2 * other.e123 - self.e2 * other.e02 * other.e0123 - self.e3 * other.e3 * other.e123 - self.e3 * other.e03 * other.e0123,
        }
    }
}

impl Project<Scalar> for Bivector {
    type Output = Null;
    fn project(self, other: Scalar) -> Null {
        Null
    }
}

impl Project<Vector> for Bivector {
    type Output = Null;
    fn project(self, other: Vector) -> Null {
        Null
    }
}

impl Project<Bivector> for Bivector {
    type Output = Null;
    fn project(self, other: Bivector) -> Null {
        Null
    }
}

impl Project<Trivector> for Bivector {
    type Output = Bivector;
    fn project(self, other: Trivector) -> Bivector {
        Bivector {
            e01: -self.e01 * other.e021 * other.e021 - self.e01 * other.e013 * other.e013 + self.e02 * other.e013 * other.e032 + self.e12 * other.e013 * other.e123 + self.e03 * other.e021 * other.e032 - self.e31 * other.e021 * other.e123,
            e02: self.e01 * other.e013 * other.e032 - self.e02 * other.e021 * other.e021 - self.e02 * other.e032 * other.e032 - self.e12 * other.e032 * other.e123 + self.e03 * other.e021 * other.e013 + self.e23 * other.e021 * other.e123,
            e12: self.e01 * other.e013 * other.e123 - self.e02 * other.e032 * other.e123 + self.e12 * other.e021 * other.e021 + self.e12 * other.e123 * other.e123 + self.e31 * other.e021 * other.e013 + self.e23 * other.e021 * other.e032,
            e03: self.e01 * other.e021 * other.e032 + self.e02 * other.e021 * other.e013 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 + self.e31 * other.e032 * other.e123 - self.e23 * other.e013 * other.e123,
            e31: -self.e01 * other.e021 * other.e123 + self.e12 * other.e021 * other.e013 + self.e03 * other.e032 * other.e123 + self.e31 * other.e013 * other.e013 + self.e31 * other.e123 * other.e123 + self.e23 * other.e013 * other.e032,
            e23: self.e02 * other.e021 * other.e123 + self.e12 * other.e021 * other.e032 - self.e03 * other.e013 * other.e123 + self.e31 * other.e013 * other.e032 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123,
        }
    }
}

impl Project<FourVector> for Bivector {
    type Output = Bivector;
    fn project(self, other: FourVector) -> Bivector {
        Bivector {
            e01: -self.e01 * other.e0123 * other.e0123,
            e02: -self.e02 * other.e0123 * other.e0123,
            e12: self.e12 * other.e0123 * other.e0123,
            e03: -self.e03 * other.e0123 * other.e0123,
            e31: self.e31 * other.e0123 * other.e0123,
            e23: self.e23 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Null> for Bivector {
    type Output = EvenMultivector;
    fn project(self, other: Null) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Project<OddMultivector> for Bivector {
    type Output = EvenMultivector;
    fn project(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.e2 * other.e021 - self.e01 * other.e3 * other.e013 - self.e02 * other.e1 * other.e021 + self.e02 * other.e3 * other.e032 + self.e12 * other.e0 * other.e021 - self.e12 * other.e3 * other.e123 + self.e03 * other.e1 * other.e013 - self.e03 * other.e2 * other.e032 + self.e31 * other.e0 * other.e013 - self.e31 * other.e2 * other.e123 + self.e23 * other.e0 * other.e032 - self.e23 * other.e1 * other.e123,
            e01: -self.e01 * other.e021 * other.e021 - self.e01 * other.e013 * other.e013 + self.e02 * other.e013 * other.e032 + self.e12 * other.e013 * other.e123 + self.e03 * other.e021 * other.e032 - self.e31 * other.e021 * other.e123,
            e02: self.e01 * other.e013 * other.e032 - self.e02 * other.e021 * other.e021 - self.e02 * other.e032 * other.e032 - self.e12 * other.e032 * other.e123 + self.e03 * other.e021 * other.e013 + self.e23 * other.e021 * other.e123,
            e12: self.e01 * other.e013 * other.e123 - self.e02 * other.e032 * other.e123 + self.e12 * other.e021 * other.e021 + self.e12 * other.e123 * other.e123 + self.e31 * other.e021 * other.e013 + self.e23 * other.e021 * other.e032,
            e03: self.e01 * other.e021 * other.e032 + self.e02 * other.e021 * other.e013 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 + self.e31 * other.e032 * other.e123 - self.e23 * other.e013 * other.e123,
            e31: -self.e01 * other.e021 * other.e123 + self.e12 * other.e021 * other.e013 + self.e03 * other.e032 * other.e123 + self.e31 * other.e013 * other.e013 + self.e31 * other.e123 * other.e123 + self.e23 * other.e013 * other.e032,
            e23: self.e02 * other.e021 * other.e123 + self.e12 * other.e021 * other.e032 - self.e03 * other.e013 * other.e123 + self.e31 * other.e013 * other.e032 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123,
            e0123: 0.0,
        }
    }
}

impl Project<EvenMultivector> for Bivector {
    type Output = EvenMultivector;
    fn project(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e01 * other.s * other.e01 - self.e01 * other.e23 * other.e0123 - self.e02 * other.s * other.e02 - self.e02 * other.e31 * other.e0123 - self.e12 * other.s * other.e12 - self.e12 * other.e03 * other.e0123 - self.e03 * other.s * other.e03 - self.e03 * other.e12 * other.e0123 - self.e31 * other.s * other.e31 - self.e31 * other.e02 * other.e0123 - self.e23 * other.s * other.e23 - self.e23 * other.e01 * other.e0123,
            e01: -self.e01 * other.e01 * other.e01 - self.e01 * other.e0123 * other.e0123 - self.e02 * other.e01 * other.e02 + self.e12 * other.e01 * other.e12 - self.e03 * other.e01 * other.e03 + self.e31 * other.e01 * other.e31 + self.e23 * other.e01 * other.e23,
            e02: -self.e01 * other.e01 * other.e02 - self.e02 * other.e02 * other.e02 - self.e02 * other.e0123 * other.e0123 + self.e12 * other.e02 * other.e12 - self.e03 * other.e02 * other.e03 + self.e31 * other.e02 * other.e31 + self.e23 * other.e02 * other.e23,
            e12: self.e01 * other.e01 * other.e12 + self.e02 * other.e02 * other.e12 + self.e12 * other.e12 * other.e12 + self.e12 * other.e0123 * other.e0123 + self.e03 * other.e12 * other.e03 + self.e31 * other.e12 * other.e31 + self.e23 * other.e12 * other.e23,
            e03: -self.e01 * other.e01 * other.e03 - self.e02 * other.e02 * other.e03 + self.e12 * other.e12 * other.e03 - self.e03 * other.e03 * other.e03 - self.e03 * other.e0123 * other.e0123 + self.e31 * other.e03 * other.e31 + self.e23 * other.e03 * other.e23,
            e31: self.e01 * other.e01 * other.e31 + self.e02 * other.e02 * other.e31 + self.e12 * other.e12 * other.e31 + self.e03 * other.e03 * other.e31 + self.e31 * other.e31 * other.e31 + self.e31 * other.e0123 * other.e0123 + self.e23 * other.e31 * other.e23,
            e23: self.e01 * other.e01 * other.e23 + self.e02 * other.e02 * other.e23 + self.e12 * other.e12 * other.e23 + self.e03 * other.e03 * other.e23 + self.e31 * other.e31 * other.e23 + self.e23 * other.e23 * other.e23 + self.e23 * other.e0123 * other.e0123,
            e0123: self.e01 * other.e01 * other.e0123 + self.e02 * other.e02 * other.e0123 - self.e12 * other.e12 * other.e0123 + self.e03 * other.e03 * other.e0123 - self.e31 * other.e31 * other.e0123 - self.e23 * other.e23 * other.e0123,
        }
    }
}

impl Project<Multivector> for Bivector {
    type Output = EvenMultivector;
    fn project(self, other: Multivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e01 * other.s * other.e01 + self.e01 * other.e2 * other.e021 - self.e01 * other.e3 * other.e013 - self.e01 * other.e23 * other.e0123 - self.e02 * other.s * other.e02 - self.e02 * other.e1 * other.e021 + self.e02 * other.e3 * other.e032 - self.e02 * other.e31 * other.e0123 - self.e12 * other.s * other.e12 + self.e12 * other.e0 * other.e021 - self.e12 * other.e3 * other.e123 - self.e12 * other.e03 * other.e0123 - self.e03 * other.s * other.e03 + self.e03 * other.e1 * other.e013 - self.e03 * other.e2 * other.e032 - self.e03 * other.e12 * other.e0123 - self.e31 * other.s * other.e31 + self.e31 * other.e0 * other.e013 - self.e31 * other.e2 * other.e123 - self.e31 * other.e02 * other.e0123 - self.e23 * other.s * other.e23 + self.e23 * other.e0 * other.e032 - self.e23 * other.e1 * other.e123 - self.e23 * other.e01 * other.e0123,
            e01: -self.e01 * other.e01 * other.e01 - self.e01 * other.e021 * other.e021 - self.e01 * other.e013 * other.e013 - self.e01 * other.e0123 * other.e0123 - self.e02 * other.e01 * other.e02 + self.e02 * other.e013 * other.e032 + self.e12 * other.e01 * other.e12 + self.e12 * other.e013 * other.e123 - self.e03 * other.e01 * other.e03 + self.e03 * other.e021 * other.e032 + self.e31 * other.e01 * other.e31 - self.e31 * other.e021 * other.e123 + self.e23 * other.e01 * other.e23,
            e02: -self.e01 * other.e01 * other.e02 + self.e01 * other.e013 * other.e032 - self.e02 * other.e02 * other.e02 - self.e02 * other.e021 * other.e021 - self.e02 * other.e032 * other.e032 - self.e02 * other.e0123 * other.e0123 + self.e12 * other.e02 * other.e12 - self.e12 * other.e032 * other.e123 - self.e03 * other.e02 * other.e03 + self.e03 * other.e021 * other.e013 + self.e31 * other.e02 * other.e31 + self.e23 * other.e02 * other.e23 + self.e23 * other.e021 * other.e123,
            e12: self.e01 * other.e01 * other.e12 + self.e01 * other.e013 * other.e123 + self.e02 * other.e02 * other.e12 - self.e02 * other.e032 * other.e123 + self.e12 * other.e12 * other.e12 + self.e12 * other.e021 * other.e021 + self.e12 * other.e123 * other.e123 + self.e12 * other.e0123 * other.e0123 + self.e03 * other.e12 * other.e03 + self.e31 * other.e12 * other.e31 + self.e31 * other.e021 * other.e013 + self.e23 * other.e12 * other.e23 + self.e23 * other.e021 * other.e032,
            e03: -self.e01 * other.e01 * other.e03 + self.e01 * other.e021 * other.e032 - self.e02 * other.e02 * other.e03 + self.e02 * other.e021 * other.e013 + self.e12 * other.e12 * other.e03 - self.e03 * other.e03 * other.e03 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 - self.e03 * other.e0123 * other.e0123 + self.e31 * other.e03 * other.e31 + self.e31 * other.e032 * other.e123 + self.e23 * other.e03 * other.e23 - self.e23 * other.e013 * other.e123,
            e31: self.e01 * other.e01 * other.e31 - self.e01 * other.e021 * other.e123 + self.e02 * other.e02 * other.e31 + self.e12 * other.e12 * other.e31 + self.e12 * other.e021 * other.e013 + self.e03 * other.e03 * other.e31 + self.e03 * other.e032 * other.e123 + self.e31 * other.e31 * other.e31 + self.e31 * other.e013 * other.e013 + self.e31 * other.e123 * other.e123 + self.e31 * other.e0123 * other.e0123 + self.e23 * other.e31 * other.e23 + self.e23 * other.e013 * other.e032,
            e23: self.e01 * other.e01 * other.e23 + self.e02 * other.e02 * other.e23 + self.e02 * other.e021 * other.e123 + self.e12 * other.e12 * other.e23 + self.e12 * other.e021 * other.e032 + self.e03 * other.e03 * other.e23 - self.e03 * other.e013 * other.e123 + self.e31 * other.e31 * other.e23 + self.e31 * other.e013 * other.e032 + self.e23 * other.e23 * other.e23 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123 + self.e23 * other.e0123 * other.e0123,
            e0123: self.e01 * other.e01 * other.e0123 + self.e02 * other.e02 * other.e0123 - self.e12 * other.e12 * other.e0123 + self.e03 * other.e03 * other.e0123 - self.e31 * other.e31 * other.e0123 - self.e23 * other.e23 * other.e0123,
        }
    }
}

impl Project<Scalar> for Trivector {
    type Output = Null;
    fn project(self, other: Scalar) -> Null {
        Null
    }
}

impl Project<Vector> for Trivector {
    type Output = Null;
    fn project(self, other: Vector) -> Null {
        Null
    }
}

impl Project<Bivector> for Trivector {
    type Output = Null;
    fn project(self, other: Bivector) -> Null {
        Null
    }
}

impl Project<Trivector> for Trivector {
    type Output = Null;
    fn project(self, other: Trivector) -> Null {
        Null
    }
}

impl Project<FourVector> for Trivector {
    type Output = Trivector;
    fn project(self, other: FourVector) -> Trivector {
        Trivector {
            e021: -self.e021 * other.e0123 * other.e0123,
            e013: -self.e013 * other.e0123 * other.e0123,
            e032: -self.e032 * other.e0123 * other.e0123,
            e123: self.e123 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Null> for Trivector {
    type Output = OddMultivector;
    fn project(self, other: Null) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Project<OddMultivector> for Trivector {
    type Output = OddMultivector;
    fn project(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e021 * other.e0 * other.e021 + self.e013 * other.e0 * other.e013 + self.e032 * other.e0 * other.e032 - self.e123 * other.e0 * other.e123,
            e1: -self.e021 * other.e1 * other.e021 - self.e013 * other.e1 * other.e013 - self.e032 * other.e1 * other.e032 - self.e123 * other.e1 * other.e123,
            e2: -self.e021 * other.e2 * other.e021 - self.e013 * other.e2 * other.e013 - self.e032 * other.e2 * other.e032 - self.e123 * other.e2 * other.e123,
            e021: -self.e021 * other.e021 * other.e021 - self.e013 * other.e021 * other.e013 - self.e032 * other.e021 * other.e032 + self.e123 * other.e021 * other.e123,
            e3: -self.e021 * other.e021 * other.e3 - self.e013 * other.e3 * other.e013 - self.e032 * other.e3 * other.e032 - self.e123 * other.e3 * other.e123,
            e013: -self.e021 * other.e021 * other.e013 - self.e013 * other.e013 * other.e013 - self.e032 * other.e013 * other.e032 + self.e123 * other.e013 * other.e123,
            e032: -self.e021 * other.e021 * other.e032 - self.e013 * other.e013 * other.e032 - self.e032 * other.e032 * other.e032 + self.e123 * other.e032 * other.e123,
            e123: self.e021 * other.e021 * other.e123 + self.e013 * other.e013 * other.e123 + self.e032 * other.e032 * other.e123 + self.e123 * other.e123 * other.e123,
        }
    }
}

impl Project<EvenMultivector> for Trivector {
    type Output = OddMultivector;
    fn project(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e021 * other.e03 * other.e0123 - self.e013 * other.e02 * other.e0123 - self.e032 * other.e01 * other.e0123,
            e1: -self.e021 * other.e31 * other.e0123 + self.e013 * other.e12 * other.e0123 - self.e123 * other.e01 * other.e0123,
            e2: self.e021 * other.e23 * other.e0123 - self.e032 * other.e12 * other.e0123 - self.e123 * other.e02 * other.e0123,
            e021: -self.e021 * other.e0123 * other.e0123,
            e3: -self.e013 * other.e23 * other.e0123 + self.e032 * other.e31 * other.e0123 - self.e123 * other.e03 * other.e0123,
            e013: -self.e013 * other.e0123 * other.e0123,
            e032: -self.e032 * other.e0123 * other.e0123,
            e123: self.e123 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Multivector> for Trivector {
    type Output = OddMultivector;
    fn project(self, other: Multivector) -> OddMultivector {
        OddMultivector {
            e0: self.e021 * other.e0 * other.e021 - self.e021 * other.e03 * other.e0123 + self.e013 * other.e0 * other.e013 - self.e013 * other.e02 * other.e0123 + self.e032 * other.e0 * other.e032 - self.e032 * other.e01 * other.e0123 - self.e123 * other.e0 * other.e123,
            e1: -self.e021 * other.e1 * other.e021 - self.e021 * other.e31 * other.e0123 - self.e013 * other.e1 * other.e013 + self.e013 * other.e12 * other.e0123 - self.e032 * other.e1 * other.e032 - self.e123 * other.e1 * other.e123 - self.e123 * other.e01 * other.e0123,
            e2: -self.e021 * other.e2 * other.e021 + self.e021 * other.e23 * other.e0123 - self.e013 * other.e2 * other.e013 - self.e032 * other.e2 * other.e032 - self.e032 * other.e12 * other.e0123 - self.e123 * other.e2 * other.e123 - self.e123 * other.e02 * other.e0123,
            e021: -self.e021 * other.e021 * other.e021 - self.e021 * other.e0123 * other.e0123 - self.e013 * other.e021 * other.e013 - self.e032 * other.e021 * other.e032 + self.e123 * other.e021 * other.e123,
            e3: -self.e021 * other.e021 * other.e3 - self.e013 * other.e3 * other.e013 - self.e013 * other.e23 * other.e0123 - self.e032 * other.e3 * other.e032 + self.e032 * other.e31 * other.e0123 - self.e123 * other.e3 * other.e123 - self.e123 * other.e03 * other.e0123,
            e013: -self.e021 * other.e021 * other.e013 - self.e013 * other.e013 * other.e013 - self.e013 * other.e0123 * other.e0123 - self.e032 * other.e013 * other.e032 + self.e123 * other.e013 * other.e123,
            e032: -self.e021 * other.e021 * other.e032 - self.e013 * other.e013 * other.e032 - self.e032 * other.e032 * other.e032 - self.e032 * other.e0123 * other.e0123 + self.e123 * other.e032 * other.e123,
            e123: self.e021 * other.e021 * other.e123 + self.e013 * other.e013 * other.e123 + self.e032 * other.e032 * other.e123 + self.e123 * other.e123 * other.e123 + self.e123 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Scalar> for FourVector {
    type Output = Null;
    fn project(self, other: Scalar) -> Null {
        Null
    }
}

impl Project<Vector> for FourVector {
    type Output = Null;
    fn project(self, other: Vector) -> Null {
        Null
    }
}

impl Project<Bivector> for FourVector {
    type Output = Null;
    fn project(self, other: Bivector) -> Null {
        Null
    }
}

impl Project<Trivector> for FourVector {
    type Output = Null;
    fn project(self, other: Trivector) -> Null {
        Null
    }
}

impl Project<FourVector> for FourVector {
    type Output = Null;
    fn project(self, other: FourVector) -> Null {
        Null
    }
}

impl Project<Null> for FourVector {
    type Output = EvenMultivector;
    fn project(self, other: Null) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Project<OddMultivector> for FourVector {
    type Output = EvenMultivector;
    fn project(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Project<EvenMultivector> for FourVector {
    type Output = EvenMultivector;
    fn project(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e0123 * other.s * other.e0123,
            e01: self.e0123 * other.e01 * other.e0123,
            e02: self.e0123 * other.e02 * other.e0123,
            e12: -self.e0123 * other.e12 * other.e0123,
            e03: self.e0123 * other.e03 * other.e0123,
            e31: -self.e0123 * other.e31 * other.e0123,
            e23: -self.e0123 * other.e23 * other.e0123,
            e0123: -self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Multivector> for FourVector {
    type Output = EvenMultivector;
    fn project(self, other: Multivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e0123 * other.s * other.e0123,
            e01: self.e0123 * other.e01 * other.e0123,
            e02: self.e0123 * other.e02 * other.e0123,
            e12: -self.e0123 * other.e12 * other.e0123,
            e03: self.e0123 * other.e03 * other.e0123,
            e31: -self.e0123 * other.e31 * other.e0123,
            e23: -self.e0123 * other.e23 * other.e0123,
            e0123: -self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Scalar> for Null {
    type Output = Null;
    fn project(self, other: Scalar) -> Null {
        Null
    }
}

impl Project<Vector> for Null {
    type Output = Null;
    fn project(self, other: Vector) -> Null {
        Null
    }
}

impl Project<Bivector> for Null {
    type Output = Null;
    fn project(self, other: Bivector) -> Null {
        Null
    }
}

impl Project<Trivector> for Null {
    type Output = Null;
    fn project(self, other: Trivector) -> Null {
        Null
    }
}

impl Project<FourVector> for Null {
    type Output = Null;
    fn project(self, other: FourVector) -> Null {
        Null
    }
}

impl Project<Null> for Null {
    type Output = Null;
    fn project(self, other: Null) -> Null {
        Null
    }
}

impl Project<OddMultivector> for Null {
    type Output = Null;
    fn project(self, other: OddMultivector) -> Null {
        Null
    }
}

impl Project<EvenMultivector> for Null {
    type Output = Null;
    fn project(self, other: EvenMultivector) -> Null {
        Null
    }
}

impl Project<Multivector> for Null {
    type Output = Null;
    fn project(self, other: Multivector) -> Null {
        Null
    }
}

impl Project<Scalar> for OddMultivector {
    type Output = OddMultivector;
    fn project(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Project<Vector> for OddMultivector {
    type Output = OddMultivector;
    fn project(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e0 * other.e0 + self.e1 * other.e0 * other.e1 + self.e2 * other.e0 * other.e2 + self.e3 * other.e0 * other.e3,
            e1: self.e0 * other.e0 * other.e1 + self.e1 * other.e1 * other.e1 + self.e2 * other.e1 * other.e2 + self.e3 * other.e1 * other.e3,
            e2: self.e0 * other.e0 * other.e2 + self.e1 * other.e1 * other.e2 + self.e2 * other.e2 * other.e2 + self.e3 * other.e2 * other.e3,
            e021: 0.0,
            e3: self.e0 * other.e0 * other.e3 + self.e1 * other.e1 * other.e3 + self.e2 * other.e2 * other.e3 + self.e3 * other.e3 * other.e3,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Project<Bivector> for OddMultivector {
    type Output = OddMultivector;
    fn project(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e01 * other.e01 - self.e0 * other.e02 * other.e02 - self.e0 * other.e03 * other.e03 + self.e1 * other.e02 * other.e12 - self.e1 * other.e03 * other.e31 - self.e2 * other.e01 * other.e12 + self.e2 * other.e03 * other.e23 + self.e3 * other.e01 * other.e31 - self.e3 * other.e02 * other.e23,
            e1: self.e0 * other.e02 * other.e12 - self.e0 * other.e03 * other.e31 + self.e1 * other.e01 * other.e01 + self.e1 * other.e12 * other.e12 + self.e1 * other.e31 * other.e31 + self.e2 * other.e01 * other.e02 - self.e2 * other.e31 * other.e23 + self.e3 * other.e01 * other.e03 - self.e3 * other.e12 * other.e23,
            e2: -self.e0 * other.e01 * other.e12 + self.e0 * other.e03 * other.e23 + self.e1 * other.e01 * other.e02 - self.e1 * other.e31 * other.e23 + self.e2 * other.e02 * other.e02 + self.e2 * other.e12 * other.e12 + self.e2 * other.e23 * other.e23 + self.e3 * other.e02 * other.e03 - self.e3 * other.e12 * other.e31,
            e021: 0.0,
            e3: self.e0 * other.e01 * other.e31 - self.e0 * other.e02 * other.e23 + self.e1 * other.e01 * other.e03 - self.e1 * other.e12 * other.e23 + self.e2 * other.e02 * other.e03 - self.e2 * other.e12 * other.e31 + self.e3 * other.e03 * other.e03 + self.e3 * other.e31 * other.e31 + self.e3 * other.e23 * other.e23,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Project<Trivector> for OddMultivector {
    type Output = OddMultivector;
    fn project(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e021 * other.e021 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 - self.e1 * other.e032 * other.e123 - self.e2 * other.e013 * other.e123 - self.e3 * other.e021 * other.e123,
            e1: -self.e0 * other.e032 * other.e123 + self.e1 * other.e021 * other.e021 + self.e1 * other.e013 * other.e013 + self.e1 * other.e123 * other.e123 - self.e2 * other.e013 * other.e032 - self.e3 * other.e021 * other.e032,
            e2: -self.e0 * other.e013 * other.e123 - self.e1 * other.e013 * other.e032 + self.e2 * other.e021 * other.e021 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 - self.e3 * other.e021 * other.e013,
            e021: -self.e021 * other.e021 * other.e021 - self.e013 * other.e021 * other.e013 - self.e032 * other.e021 * other.e032 + self.e123 * other.e021 * other.e123,
            e3: -self.e0 * other.e021 * other.e123 - self.e1 * other.e021 * other.e032 - self.e2 * other.e021 * other.e013 + self.e3 * other.e013 * other.e013 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123,
            e013: -self.e021 * other.e021 * other.e013 - self.e013 * other.e013 * other.e013 - self.e032 * other.e013 * other.e032 + self.e123 * other.e013 * other.e123,
            e032: -self.e021 * other.e021 * other.e032 - self.e013 * other.e013 * other.e032 - self.e032 * other.e032 * other.e032 + self.e123 * other.e032 * other.e123,
            e123: self.e021 * other.e021 * other.e123 + self.e013 * other.e013 * other.e123 + self.e032 * other.e032 * other.e123 + self.e123 * other.e123 * other.e123,
        }
    }
}

impl Project<FourVector> for OddMultivector {
    type Output = OddMultivector;
    fn project(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e0123 * other.e0123,
            e1: self.e1 * other.e0123 * other.e0123,
            e2: self.e2 * other.e0123 * other.e0123,
            e021: -self.e021 * other.e0123 * other.e0123,
            e3: self.e3 * other.e0123 * other.e0123,
            e013: -self.e013 * other.e0123 * other.e0123,
            e032: -self.e032 * other.e0123 * other.e0123,
            e123: self.e123 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Null> for OddMultivector {
    type Output = OddMultivector;
    fn project(self, other: Null) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Project<OddMultivector> for OddMultivector {
    type Output = OddMultivector;
    fn project(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e0 * other.e0 - self.e0 * other.e021 * other.e021 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 + self.e1 * other.e0 * other.e1 - self.e1 * other.e032 * other.e123 + self.e2 * other.e0 * other.e2 - self.e2 * other.e013 * other.e123 + self.e021 * other.e0 * other.e021 + self.e3 * other.e0 * other.e3 - self.e3 * other.e021 * other.e123 + self.e013 * other.e0 * other.e013 + self.e032 * other.e0 * other.e032 - self.e123 * other.e0 * other.e123,
            e1: self.e0 * other.e0 * other.e1 - self.e0 * other.e032 * other.e123 + self.e1 * other.e1 * other.e1 + self.e1 * other.e021 * other.e021 + self.e1 * other.e013 * other.e013 + self.e1 * other.e123 * other.e123 + self.e2 * other.e1 * other.e2 - self.e2 * other.e013 * other.e032 - self.e021 * other.e1 * other.e021 + self.e3 * other.e1 * other.e3 - self.e3 * other.e021 * other.e032 - self.e013 * other.e1 * other.e013 - self.e032 * other.e1 * other.e032 - self.e123 * other.e1 * other.e123,
            e2: self.e0 * other.e0 * other.e2 - self.e0 * other.e013 * other.e123 + self.e1 * other.e1 * other.e2 - self.e1 * other.e013 * other.e032 + self.e2 * other.e2 * other.e2 + self.e2 * other.e021 * other.e021 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 - self.e021 * other.e2 * other.e021 + self.e3 * other.e2 * other.e3 - self.e3 * other.e021 * other.e013 - self.e013 * other.e2 * other.e013 - self.e032 * other.e2 * other.e032 - self.e123 * other.e2 * other.e123,
            e021: self.e0 * other.e0 * other.e021 - self.e1 * other.e1 * other.e021 - self.e2 * other.e2 * other.e021 - self.e021 * other.e021 * other.e021 - self.e3 * other.e021 * other.e3 - self.e013 * other.e021 * other.e013 - self.e032 * other.e021 * other.e032 + self.e123 * other.e021 * other.e123,
            e3: self.e0 * other.e0 * other.e3 - self.e0 * other.e021 * other.e123 + self.e1 * other.e1 * other.e3 - self.e1 * other.e021 * other.e032 + self.e2 * other.e2 * other.e3 - self.e2 * other.e021 * other.e013 - self.e021 * other.e021 * other.e3 + self.e3 * other.e3 * other.e3 + self.e3 * other.e013 * other.e013 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123 - self.e013 * other.e3 * other.e013 - self.e032 * other.e3 * other.e032 - self.e123 * other.e3 * other.e123,
            e013: self.e0 * other.e0 * other.e013 - self.e1 * other.e1 * other.e013 - self.e2 * other.e2 * other.e013 - self.e021 * other.e021 * other.e013 - self.e3 * other.e3 * other.e013 - self.e013 * other.e013 * other.e013 - self.e032 * other.e013 * other.e032 + self.e123 * other.e013 * other.e123,
            e032: self.e0 * other.e0 * other.e032 - self.e1 * other.e1 * other.e032 - self.e2 * other.e2 * other.e032 - self.e021 * other.e021 * other.e032 - self.e3 * other.e3 * other.e032 - self.e013 * other.e013 * other.e032 - self.e032 * other.e032 * other.e032 + self.e123 * other.e032 * other.e123,
            e123: -self.e0 * other.e0 * other.e123 - self.e1 * other.e1 * other.e123 - self.e2 * other.e2 * other.e123 + self.e021 * other.e021 * other.e123 - self.e3 * other.e3 * other.e123 + self.e013 * other.e013 * other.e123 + self.e032 * other.e032 * other.e123 + self.e123 * other.e123 * other.e123,
        }
    }
}

impl Project<EvenMultivector> for OddMultivector {
    type Output = OddMultivector;
    fn project(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e01 * other.e01 - self.e0 * other.e02 * other.e02 - self.e0 * other.e03 * other.e03 - self.e0 * other.e0123 * other.e0123 + self.e1 * other.e02 * other.e12 - self.e1 * other.e03 * other.e31 - self.e2 * other.e01 * other.e12 + self.e2 * other.e03 * other.e23 - self.e021 * other.e03 * other.e0123 + self.e3 * other.e01 * other.e31 - self.e3 * other.e02 * other.e23 - self.e013 * other.e02 * other.e0123 - self.e032 * other.e01 * other.e0123,
            e1: self.e0 * other.e02 * other.e12 - self.e0 * other.e03 * other.e31 + self.e1 * other.e01 * other.e01 + self.e1 * other.e12 * other.e12 + self.e1 * other.e31 * other.e31 + self.e1 * other.e0123 * other.e0123 + self.e2 * other.e01 * other.e02 - self.e2 * other.e31 * other.e23 - self.e021 * other.e31 * other.e0123 + self.e3 * other.e01 * other.e03 - self.e3 * other.e12 * other.e23 + self.e013 * other.e12 * other.e0123 - self.e123 * other.e01 * other.e0123,
            e2: -self.e0 * other.e01 * other.e12 + self.e0 * other.e03 * other.e23 + self.e1 * other.e01 * other.e02 - self.e1 * other.e31 * other.e23 + self.e2 * other.e02 * other.e02 + self.e2 * other.e12 * other.e12 + self.e2 * other.e23 * other.e23 + self.e2 * other.e0123 * other.e0123 + self.e021 * other.e23 * other.e0123 + self.e3 * other.e02 * other.e03 - self.e3 * other.e12 * other.e31 - self.e032 * other.e12 * other.e0123 - self.e123 * other.e02 * other.e0123,
            e021: -self.e0 * other.e03 * other.e0123 - self.e1 * other.e31 * other.e0123 + self.e2 * other.e23 * other.e0123 - self.e021 * other.e0123 * other.e0123,
            e3: self.e0 * other.e01 * other.e31 - self.e0 * other.e02 * other.e23 + self.e1 * other.e01 * other.e03 - self.e1 * other.e12 * other.e23 + self.e2 * other.e02 * other.e03 - self.e2 * other.e12 * other.e31 + self.e3 * other.e03 * other.e03 + self.e3 * other.e31 * other.e31 + self.e3 * other.e23 * other.e23 + self.e3 * other.e0123 * other.e0123 - self.e013 * other.e23 * other.e0123 + self.e032 * other.e31 * other.e0123 - self.e123 * other.e03 * other.e0123,
            e013: -self.e0 * other.e02 * other.e0123 + self.e1 * other.e12 * other.e0123 - self.e3 * other.e23 * other.e0123 - self.e013 * other.e0123 * other.e0123,
            e032: -self.e0 * other.e01 * other.e0123 - self.e2 * other.e12 * other.e0123 + self.e3 * other.e31 * other.e0123 - self.e032 * other.e0123 * other.e0123,
            e123: -self.e1 * other.e01 * other.e0123 - self.e2 * other.e02 * other.e0123 - self.e3 * other.e03 * other.e0123 + self.e123 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Multivector> for OddMultivector {
    type Output = OddMultivector;
    fn project(self, other: Multivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e0 * other.e0 * other.e0 - self.e0 * other.e01 * other.e01 - self.e0 * other.e02 * other.e02 - self.e0 * other.e021 * other.e021 - self.e0 * other.e03 * other.e03 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 - self.e0 * other.e0123 * other.e0123 + self.e1 * other.e0 * other.e1 + self.e1 * other.e02 * other.e12 - self.e1 * other.e03 * other.e31 - self.e1 * other.e032 * other.e123 + self.e2 * other.e0 * other.e2 - self.e2 * other.e01 * other.e12 + self.e2 * other.e03 * other.e23 - self.e2 * other.e013 * other.e123 + self.e021 * other.e0 * other.e021 - self.e021 * other.e03 * other.e0123 + self.e3 * other.e0 * other.e3 + self.e3 * other.e01 * other.e31 - self.e3 * other.e02 * other.e23 - self.e3 * other.e021 * other.e123 + self.e013 * other.e0 * other.e013 - self.e013 * other.e02 * other.e0123 + self.e032 * other.e0 * other.e032 - self.e032 * other.e01 * other.e0123 - self.e123 * other.e0 * other.e123,
            e1: self.e0 * other.e0 * other.e1 + self.e0 * other.e02 * other.e12 - self.e0 * other.e03 * other.e31 - self.e0 * other.e032 * other.e123 + self.e1 * other.e1 * other.e1 + self.e1 * other.e01 * other.e01 + self.e1 * other.e12 * other.e12 + self.e1 * other.e021 * other.e021 + self.e1 * other.e31 * other.e31 + self.e1 * other.e013 * other.e013 + self.e1 * other.e123 * other.e123 + self.e1 * other.e0123 * other.e0123 + self.e2 * other.e1 * other.e2 + self.e2 * other.e01 * other.e02 - self.e2 * other.e31 * other.e23 - self.e2 * other.e013 * other.e032 - self.e021 * other.e1 * other.e021 - self.e021 * other.e31 * other.e0123 + self.e3 * other.e1 * other.e3 + self.e3 * other.e01 * other.e03 - self.e3 * other.e12 * other.e23 - self.e3 * other.e021 * other.e032 - self.e013 * other.e1 * other.e013 + self.e013 * other.e12 * other.e0123 - self.e032 * other.e1 * other.e032 - self.e123 * other.e1 * other.e123 - self.e123 * other.e01 * other.e0123,
            e2: self.e0 * other.e0 * other.e2 - self.e0 * other.e01 * other.e12 + self.e0 * other.e03 * other.e23 - self.e0 * other.e013 * other.e123 + self.e1 * other.e1 * other.e2 + self.e1 * other.e01 * other.e02 - self.e1 * other.e31 * other.e23 - self.e1 * other.e013 * other.e032 + self.e2 * other.e2 * other.e2 + self.e2 * other.e02 * other.e02 + self.e2 * other.e12 * other.e12 + self.e2 * other.e021 * other.e021 + self.e2 * other.e23 * other.e23 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 + self.e2 * other.e0123 * other.e0123 - self.e021 * other.e2 * other.e021 + self.e021 * other.e23 * other.e0123 + self.e3 * other.e2 * other.e3 + self.e3 * other.e02 * other.e03 - self.e3 * other.e12 * other.e31 - self.e3 * other.e021 * other.e013 - self.e013 * other.e2 * other.e013 - self.e032 * other.e2 * other.e032 - self.e032 * other.e12 * other.e0123 - self.e123 * other.e2 * other.e123 - self.e123 * other.e02 * other.e0123,
            e021: self.e0 * other.e0 * other.e021 - self.e0 * other.e03 * other.e0123 - self.e1 * other.e1 * other.e021 - self.e1 * other.e31 * other.e0123 - self.e2 * other.e2 * other.e021 + self.e2 * other.e23 * other.e0123 - self.e021 * other.e021 * other.e021 - self.e021 * other.e0123 * other.e0123 - self.e3 * other.e021 * other.e3 - self.e013 * other.e021 * other.e013 - self.e032 * other.e021 * other.e032 + self.e123 * other.e021 * other.e123,
            e3: self.e0 * other.e0 * other.e3 + self.e0 * other.e01 * other.e31 - self.e0 * other.e02 * other.e23 - self.e0 * other.e021 * other.e123 + self.e1 * other.e1 * other.e3 + self.e1 * other.e01 * other.e03 - self.e1 * other.e12 * other.e23 - self.e1 * other.e021 * other.e032 + self.e2 * other.e2 * other.e3 + self.e2 * other.e02 * other.e03 - self.e2 * other.e12 * other.e31 - self.e2 * other.e021 * other.e013 - self.e021 * other.e021 * other.e3 + self.e3 * other.e3 * other.e3 + self.e3 * other.e03 * other.e03 + self.e3 * other.e31 * other.e31 + self.e3 * other.e013 * other.e013 + self.e3 * other.e23 * other.e23 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123 + self.e3 * other.e0123 * other.e0123 - self.e013 * other.e3 * other.e013 - self.e013 * other.e23 * other.e0123 - self.e032 * other.e3 * other.e032 + self.e032 * other.e31 * other.e0123 - self.e123 * other.e3 * other.e123 - self.e123 * other.e03 * other.e0123,
            e013: self.e0 * other.e0 * other.e013 - self.e0 * other.e02 * other.e0123 - self.e1 * other.e1 * other.e013 + self.e1 * other.e12 * other.e0123 - self.e2 * other.e2 * other.e013 - self.e021 * other.e021 * other.e013 - self.e3 * other.e3 * other.e013 - self.e3 * other.e23 * other.e0123 - self.e013 * other.e013 * other.e013 - self.e013 * other.e0123 * other.e0123 - self.e032 * other.e013 * other.e032 + self.e123 * other.e013 * other.e123,
            e032: self.e0 * other.e0 * other.e032 - self.e0 * other.e01 * other.e0123 - self.e1 * other.e1 * other.e032 - self.e2 * other.e2 * other.e032 - self.e2 * other.e12 * other.e0123 - self.e021 * other.e021 * other.e032 - self.e3 * other.e3 * other.e032 + self.e3 * other.e31 * other.e0123 - self.e013 * other.e013 * other.e032 - self.e032 * other.e032 * other.e032 - self.e032 * other.e0123 * other.e0123 + self.e123 * other.e032 * other.e123,
            e123: -self.e0 * other.e0 * other.e123 - self.e1 * other.e1 * other.e123 - self.e1 * other.e01 * other.e0123 - self.e2 * other.e2 * other.e123 - self.e2 * other.e02 * other.e0123 + self.e021 * other.e021 * other.e123 - self.e3 * other.e3 * other.e123 - self.e3 * other.e03 * other.e0123 + self.e013 * other.e013 * other.e123 + self.e032 * other.e032 * other.e123 + self.e123 * other.e123 * other.e123 + self.e123 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Scalar> for EvenMultivector {
    type Output = EvenMultivector;
    fn project(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s * other.s,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Project<Vector> for EvenMultivector {
    type Output = EvenMultivector;
    fn project(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e3 * other.e3,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Project<Bivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn project(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23,
            e01: -self.e01 * other.e01 * other.e01 - self.e02 * other.e01 * other.e02 + self.e12 * other.e01 * other.e12 - self.e03 * other.e01 * other.e03 + self.e31 * other.e01 * other.e31 + self.e23 * other.e01 * other.e23,
            e02: -self.e01 * other.e01 * other.e02 - self.e02 * other.e02 * other.e02 + self.e12 * other.e02 * other.e12 - self.e03 * other.e02 * other.e03 + self.e31 * other.e02 * other.e31 + self.e23 * other.e02 * other.e23,
            e12: self.e01 * other.e01 * other.e12 + self.e02 * other.e02 * other.e12 + self.e12 * other.e12 * other.e12 + self.e03 * other.e12 * other.e03 + self.e31 * other.e12 * other.e31 + self.e23 * other.e12 * other.e23,
            e03: -self.e01 * other.e01 * other.e03 - self.e02 * other.e02 * other.e03 + self.e12 * other.e12 * other.e03 - self.e03 * other.e03 * other.e03 + self.e31 * other.e03 * other.e31 + self.e23 * other.e03 * other.e23,
            e31: self.e01 * other.e01 * other.e31 + self.e02 * other.e02 * other.e31 + self.e12 * other.e12 * other.e31 + self.e03 * other.e03 * other.e31 + self.e31 * other.e31 * other.e31 + self.e23 * other.e31 * other.e23,
            e23: self.e01 * other.e01 * other.e23 + self.e02 * other.e02 * other.e23 + self.e12 * other.e12 * other.e23 + self.e03 * other.e03 * other.e23 + self.e31 * other.e31 * other.e23 + self.e23 * other.e23 * other.e23,
            e0123: 0.0,
        }
    }
}

impl Project<Trivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn project(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e021 * other.e021 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123,
            e01: -self.e01 * other.e021 * other.e021 - self.e01 * other.e013 * other.e013 + self.e02 * other.e013 * other.e032 + self.e12 * other.e013 * other.e123 + self.e03 * other.e021 * other.e032 - self.e31 * other.e021 * other.e123,
            e02: self.e01 * other.e013 * other.e032 - self.e02 * other.e021 * other.e021 - self.e02 * other.e032 * other.e032 - self.e12 * other.e032 * other.e123 + self.e03 * other.e021 * other.e013 + self.e23 * other.e021 * other.e123,
            e12: self.e01 * other.e013 * other.e123 - self.e02 * other.e032 * other.e123 + self.e12 * other.e021 * other.e021 + self.e12 * other.e123 * other.e123 + self.e31 * other.e021 * other.e013 + self.e23 * other.e021 * other.e032,
            e03: self.e01 * other.e021 * other.e032 + self.e02 * other.e021 * other.e013 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 + self.e31 * other.e032 * other.e123 - self.e23 * other.e013 * other.e123,
            e31: -self.e01 * other.e021 * other.e123 + self.e12 * other.e021 * other.e013 + self.e03 * other.e032 * other.e123 + self.e31 * other.e013 * other.e013 + self.e31 * other.e123 * other.e123 + self.e23 * other.e013 * other.e032,
            e23: self.e02 * other.e021 * other.e123 + self.e12 * other.e021 * other.e032 - self.e03 * other.e013 * other.e123 + self.e31 * other.e013 * other.e032 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123,
            e0123: 0.0,
        }
    }
}

impl Project<FourVector> for EvenMultivector {
    type Output = EvenMultivector;
    fn project(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0123 * other.e0123,
            e01: -self.e01 * other.e0123 * other.e0123,
            e02: -self.e02 * other.e0123 * other.e0123,
            e12: self.e12 * other.e0123 * other.e0123,
            e03: -self.e03 * other.e0123 * other.e0123,
            e31: self.e31 * other.e0123 * other.e0123,
            e23: self.e23 * other.e0123 * other.e0123,
            e0123: -self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Null> for EvenMultivector {
    type Output = EvenMultivector;
    fn project(self, other: Null) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Project<OddMultivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn project(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 + self.e01 * other.e2 * other.e021 - self.e01 * other.e3 * other.e013 - self.e02 * other.e1 * other.e021 + self.e02 * other.e3 * other.e032 + self.e12 * other.e0 * other.e021 - self.e12 * other.e3 * other.e123 + self.e03 * other.e1 * other.e013 - self.e03 * other.e2 * other.e032 + self.e31 * other.e0 * other.e013 - self.e31 * other.e2 * other.e123 + self.e23 * other.e0 * other.e032 - self.e23 * other.e1 * other.e123,
            e01: self.s * other.e2 * other.e021 - self.s * other.e3 * other.e013 - self.e01 * other.e021 * other.e021 - self.e01 * other.e013 * other.e013 + self.e02 * other.e013 * other.e032 + self.e12 * other.e013 * other.e123 + self.e03 * other.e021 * other.e032 - self.e31 * other.e021 * other.e123,
            e02: -self.s * other.e1 * other.e021 + self.s * other.e3 * other.e032 + self.e01 * other.e013 * other.e032 - self.e02 * other.e021 * other.e021 - self.e02 * other.e032 * other.e032 - self.e12 * other.e032 * other.e123 + self.e03 * other.e021 * other.e013 + self.e23 * other.e021 * other.e123,
            e12: self.s * other.e0 * other.e021 - self.s * other.e3 * other.e123 + self.e01 * other.e013 * other.e123 - self.e02 * other.e032 * other.e123 + self.e12 * other.e021 * other.e021 + self.e12 * other.e123 * other.e123 + self.e31 * other.e021 * other.e013 + self.e23 * other.e021 * other.e032,
            e03: self.s * other.e1 * other.e013 - self.s * other.e2 * other.e032 + self.e01 * other.e021 * other.e032 + self.e02 * other.e021 * other.e013 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 + self.e31 * other.e032 * other.e123 - self.e23 * other.e013 * other.e123,
            e31: self.s * other.e0 * other.e013 - self.s * other.e2 * other.e123 - self.e01 * other.e021 * other.e123 + self.e12 * other.e021 * other.e013 + self.e03 * other.e032 * other.e123 + self.e31 * other.e013 * other.e013 + self.e31 * other.e123 * other.e123 + self.e23 * other.e013 * other.e032,
            e23: self.s * other.e0 * other.e032 - self.s * other.e1 * other.e123 + self.e02 * other.e021 * other.e123 + self.e12 * other.e021 * other.e032 - self.e03 * other.e013 * other.e123 + self.e31 * other.e013 * other.e032 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123,
            e0123: 0.0,
        }
    }
}

impl Project<EvenMultivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn project(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s * other.s + self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23 + self.s * other.e0123 * other.e0123 - self.e01 * other.s * other.e01 - self.e01 * other.e23 * other.e0123 - self.e02 * other.s * other.e02 - self.e02 * other.e31 * other.e0123 - self.e12 * other.s * other.e12 - self.e12 * other.e03 * other.e0123 - self.e03 * other.s * other.e03 - self.e03 * other.e12 * other.e0123 - self.e31 * other.s * other.e31 - self.e31 * other.e02 * other.e0123 - self.e23 * other.s * other.e23 - self.e23 * other.e01 * other.e0123 + self.e0123 * other.s * other.e0123,
            e01: -self.s * other.s * other.e01 - self.s * other.e23 * other.e0123 - self.e01 * other.e01 * other.e01 - self.e01 * other.e0123 * other.e0123 - self.e02 * other.e01 * other.e02 + self.e12 * other.e01 * other.e12 - self.e03 * other.e01 * other.e03 + self.e31 * other.e01 * other.e31 + self.e23 * other.e01 * other.e23 + self.e0123 * other.e01 * other.e0123,
            e02: -self.s * other.s * other.e02 - self.s * other.e31 * other.e0123 - self.e01 * other.e01 * other.e02 - self.e02 * other.e02 * other.e02 - self.e02 * other.e0123 * other.e0123 + self.e12 * other.e02 * other.e12 - self.e03 * other.e02 * other.e03 + self.e31 * other.e02 * other.e31 + self.e23 * other.e02 * other.e23 + self.e0123 * other.e02 * other.e0123,
            e12: -self.s * other.s * other.e12 - self.s * other.e03 * other.e0123 + self.e01 * other.e01 * other.e12 + self.e02 * other.e02 * other.e12 + self.e12 * other.e12 * other.e12 + self.e12 * other.e0123 * other.e0123 + self.e03 * other.e12 * other.e03 + self.e31 * other.e12 * other.e31 + self.e23 * other.e12 * other.e23 - self.e0123 * other.e12 * other.e0123,
            e03: -self.s * other.s * other.e03 - self.s * other.e12 * other.e0123 - self.e01 * other.e01 * other.e03 - self.e02 * other.e02 * other.e03 + self.e12 * other.e12 * other.e03 - self.e03 * other.e03 * other.e03 - self.e03 * other.e0123 * other.e0123 + self.e31 * other.e03 * other.e31 + self.e23 * other.e03 * other.e23 + self.e0123 * other.e03 * other.e0123,
            e31: -self.s * other.s * other.e31 - self.s * other.e02 * other.e0123 + self.e01 * other.e01 * other.e31 + self.e02 * other.e02 * other.e31 + self.e12 * other.e12 * other.e31 + self.e03 * other.e03 * other.e31 + self.e31 * other.e31 * other.e31 + self.e31 * other.e0123 * other.e0123 + self.e23 * other.e31 * other.e23 - self.e0123 * other.e31 * other.e0123,
            e23: -self.s * other.s * other.e23 - self.s * other.e01 * other.e0123 + self.e01 * other.e01 * other.e23 + self.e02 * other.e02 * other.e23 + self.e12 * other.e12 * other.e23 + self.e03 * other.e03 * other.e23 + self.e31 * other.e31 * other.e23 + self.e23 * other.e23 * other.e23 + self.e23 * other.e0123 * other.e0123 - self.e0123 * other.e23 * other.e0123,
            e0123: self.s * other.s * other.e0123 + self.e01 * other.e01 * other.e0123 + self.e02 * other.e02 * other.e0123 - self.e12 * other.e12 * other.e0123 + self.e03 * other.e03 * other.e0123 - self.e31 * other.e31 * other.e0123 - self.e23 * other.e23 * other.e0123 - self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Multivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn project(self, other: Multivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01 + self.s * other.e2 * other.e2 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e013 * other.e013 + self.s * other.e23 * other.e23 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 + self.s * other.e0123 * other.e0123 - self.e01 * other.s * other.e01 + self.e01 * other.e2 * other.e021 - self.e01 * other.e3 * other.e013 - self.e01 * other.e23 * other.e0123 - self.e02 * other.s * other.e02 - self.e02 * other.e1 * other.e021 + self.e02 * other.e3 * other.e032 - self.e02 * other.e31 * other.e0123 - self.e12 * other.s * other.e12 + self.e12 * other.e0 * other.e021 - self.e12 * other.e3 * other.e123 - self.e12 * other.e03 * other.e0123 - self.e03 * other.s * other.e03 + self.e03 * other.e1 * other.e013 - self.e03 * other.e2 * other.e032 - self.e03 * other.e12 * other.e0123 - self.e31 * other.s * other.e31 + self.e31 * other.e0 * other.e013 - self.e31 * other.e2 * other.e123 - self.e31 * other.e02 * other.e0123 - self.e23 * other.s * other.e23 + self.e23 * other.e0 * other.e032 - self.e23 * other.e1 * other.e123 - self.e23 * other.e01 * other.e0123 + self.e0123 * other.s * other.e0123,
            e01: -self.s * other.s * other.e01 + self.s * other.e2 * other.e021 - self.s * other.e3 * other.e013 - self.s * other.e23 * other.e0123 - self.e01 * other.e01 * other.e01 - self.e01 * other.e021 * other.e021 - self.e01 * other.e013 * other.e013 - self.e01 * other.e0123 * other.e0123 - self.e02 * other.e01 * other.e02 + self.e02 * other.e013 * other.e032 + self.e12 * other.e01 * other.e12 + self.e12 * other.e013 * other.e123 - self.e03 * other.e01 * other.e03 + self.e03 * other.e021 * other.e032 + self.e31 * other.e01 * other.e31 - self.e31 * other.e021 * other.e123 + self.e23 * other.e01 * other.e23 + self.e0123 * other.e01 * other.e0123,
            e02: -self.s * other.s * other.e02 - self.s * other.e1 * other.e021 + self.s * other.e3 * other.e032 - self.s * other.e31 * other.e0123 - self.e01 * other.e01 * other.e02 + self.e01 * other.e013 * other.e032 - self.e02 * other.e02 * other.e02 - self.e02 * other.e021 * other.e021 - self.e02 * other.e032 * other.e032 - self.e02 * other.e0123 * other.e0123 + self.e12 * other.e02 * other.e12 - self.e12 * other.e032 * other.e123 - self.e03 * other.e02 * other.e03 + self.e03 * other.e021 * other.e013 + self.e31 * other.e02 * other.e31 + self.e23 * other.e02 * other.e23 + self.e23 * other.e021 * other.e123 + self.e0123 * other.e02 * other.e0123,
            e12: -self.s * other.s * other.e12 + self.s * other.e0 * other.e021 - self.s * other.e3 * other.e123 - self.s * other.e03 * other.e0123 + self.e01 * other.e01 * other.e12 + self.e01 * other.e013 * other.e123 + self.e02 * other.e02 * other.e12 - self.e02 * other.e032 * other.e123 + self.e12 * other.e12 * other.e12 + self.e12 * other.e021 * other.e021 + self.e12 * other.e123 * other.e123 + self.e12 * other.e0123 * other.e0123 + self.e03 * other.e12 * other.e03 + self.e31 * other.e12 * other.e31 + self.e31 * other.e021 * other.e013 + self.e23 * other.e12 * other.e23 + self.e23 * other.e021 * other.e032 - self.e0123 * other.e12 * other.e0123,
            e03: -self.s * other.s * other.e03 + self.s * other.e1 * other.e013 - self.s * other.e2 * other.e032 - self.s * other.e12 * other.e0123 - self.e01 * other.e01 * other.e03 + self.e01 * other.e021 * other.e032 - self.e02 * other.e02 * other.e03 + self.e02 * other.e021 * other.e013 + self.e12 * other.e12 * other.e03 - self.e03 * other.e03 * other.e03 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 - self.e03 * other.e0123 * other.e0123 + self.e31 * other.e03 * other.e31 + self.e31 * other.e032 * other.e123 + self.e23 * other.e03 * other.e23 - self.e23 * other.e013 * other.e123 + self.e0123 * other.e03 * other.e0123,
            e31: -self.s * other.s * other.e31 + self.s * other.e0 * other.e013 - self.s * other.e2 * other.e123 - self.s * other.e02 * other.e0123 + self.e01 * other.e01 * other.e31 - self.e01 * other.e021 * other.e123 + self.e02 * other.e02 * other.e31 + self.e12 * other.e12 * other.e31 + self.e12 * other.e021 * other.e013 + self.e03 * other.e03 * other.e31 + self.e03 * other.e032 * other.e123 + self.e31 * other.e31 * other.e31 + self.e31 * other.e013 * other.e013 + self.e31 * other.e123 * other.e123 + self.e31 * other.e0123 * other.e0123 + self.e23 * other.e31 * other.e23 + self.e23 * other.e013 * other.e032 - self.e0123 * other.e31 * other.e0123,
            e23: -self.s * other.s * other.e23 + self.s * other.e0 * other.e032 - self.s * other.e1 * other.e123 - self.s * other.e01 * other.e0123 + self.e01 * other.e01 * other.e23 + self.e02 * other.e02 * other.e23 + self.e02 * other.e021 * other.e123 + self.e12 * other.e12 * other.e23 + self.e12 * other.e021 * other.e032 + self.e03 * other.e03 * other.e23 - self.e03 * other.e013 * other.e123 + self.e31 * other.e31 * other.e23 + self.e31 * other.e013 * other.e032 + self.e23 * other.e23 * other.e23 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123 + self.e23 * other.e0123 * other.e0123 - self.e0123 * other.e23 * other.e0123,
            e0123: self.s * other.s * other.e0123 + self.e01 * other.e01 * other.e0123 + self.e02 * other.e02 * other.e0123 - self.e12 * other.e12 * other.e0123 + self.e03 * other.e03 * other.e0123 - self.e31 * other.e31 * other.e0123 - self.e23 * other.e23 * other.e0123 - self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Scalar> for Multivector {
    type Output = Multivector;
    fn project(self, other: Scalar) -> Multivector {
        Multivector {
            s: self.s * other.s * other.s,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl Project<Vector> for Multivector {
    type Output = Multivector;
    fn project(self, other: Vector) -> Multivector {
        Multivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e3 * other.e3,
            e0: -self.e0 * other.e0 * other.e0 + self.e1 * other.e0 * other.e1 + self.e2 * other.e0 * other.e2 + self.e3 * other.e0 * other.e3,
            e1: self.e0 * other.e0 * other.e1 + self.e1 * other.e1 * other.e1 + self.e2 * other.e1 * other.e2 + self.e3 * other.e1 * other.e3,
            e01: 0.0,
            e2: self.e0 * other.e0 * other.e2 + self.e1 * other.e1 * other.e2 + self.e2 * other.e2 * other.e2 + self.e3 * other.e2 * other.e3,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: self.e0 * other.e0 * other.e3 + self.e1 * other.e1 * other.e3 + self.e2 * other.e2 * other.e3 + self.e3 * other.e3 * other.e3,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl Project<Bivector> for Multivector {
    type Output = Multivector;
    fn project(self, other: Bivector) -> Multivector {
        Multivector {
            s: self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23,
            e0: -self.e0 * other.e01 * other.e01 - self.e0 * other.e02 * other.e02 - self.e0 * other.e03 * other.e03 + self.e1 * other.e02 * other.e12 - self.e1 * other.e03 * other.e31 - self.e2 * other.e01 * other.e12 + self.e2 * other.e03 * other.e23 + self.e3 * other.e01 * other.e31 - self.e3 * other.e02 * other.e23,
            e1: self.e0 * other.e02 * other.e12 - self.e0 * other.e03 * other.e31 + self.e1 * other.e01 * other.e01 + self.e1 * other.e12 * other.e12 + self.e1 * other.e31 * other.e31 + self.e2 * other.e01 * other.e02 - self.e2 * other.e31 * other.e23 + self.e3 * other.e01 * other.e03 - self.e3 * other.e12 * other.e23,
            e01: -self.e01 * other.e01 * other.e01 - self.e02 * other.e01 * other.e02 + self.e12 * other.e01 * other.e12 - self.e03 * other.e01 * other.e03 + self.e31 * other.e01 * other.e31 + self.e23 * other.e01 * other.e23,
            e2: -self.e0 * other.e01 * other.e12 + self.e0 * other.e03 * other.e23 + self.e1 * other.e01 * other.e02 - self.e1 * other.e31 * other.e23 + self.e2 * other.e02 * other.e02 + self.e2 * other.e12 * other.e12 + self.e2 * other.e23 * other.e23 + self.e3 * other.e02 * other.e03 - self.e3 * other.e12 * other.e31,
            e02: -self.e01 * other.e01 * other.e02 - self.e02 * other.e02 * other.e02 + self.e12 * other.e02 * other.e12 - self.e03 * other.e02 * other.e03 + self.e31 * other.e02 * other.e31 + self.e23 * other.e02 * other.e23,
            e12: self.e01 * other.e01 * other.e12 + self.e02 * other.e02 * other.e12 + self.e12 * other.e12 * other.e12 + self.e03 * other.e12 * other.e03 + self.e31 * other.e12 * other.e31 + self.e23 * other.e12 * other.e23,
            e021: 0.0,
            e3: self.e0 * other.e01 * other.e31 - self.e0 * other.e02 * other.e23 + self.e1 * other.e01 * other.e03 - self.e1 * other.e12 * other.e23 + self.e2 * other.e02 * other.e03 - self.e2 * other.e12 * other.e31 + self.e3 * other.e03 * other.e03 + self.e3 * other.e31 * other.e31 + self.e3 * other.e23 * other.e23,
            e03: -self.e01 * other.e01 * other.e03 - self.e02 * other.e02 * other.e03 + self.e12 * other.e12 * other.e03 - self.e03 * other.e03 * other.e03 + self.e31 * other.e03 * other.e31 + self.e23 * other.e03 * other.e23,
            e31: self.e01 * other.e01 * other.e31 + self.e02 * other.e02 * other.e31 + self.e12 * other.e12 * other.e31 + self.e03 * other.e03 * other.e31 + self.e31 * other.e31 * other.e31 + self.e23 * other.e31 * other.e23,
            e013: 0.0,
            e23: self.e01 * other.e01 * other.e23 + self.e02 * other.e02 * other.e23 + self.e12 * other.e12 * other.e23 + self.e03 * other.e03 * other.e23 + self.e31 * other.e31 * other.e23 + self.e23 * other.e23 * other.e23,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl Project<Trivector> for Multivector {
    type Output = Multivector;
    fn project(self, other: Trivector) -> Multivector {
        Multivector {
            s: self.s * other.e021 * other.e021 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123,
            e0: -self.e0 * other.e021 * other.e021 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 - self.e1 * other.e032 * other.e123 - self.e2 * other.e013 * other.e123 - self.e3 * other.e021 * other.e123,
            e1: -self.e0 * other.e032 * other.e123 + self.e1 * other.e021 * other.e021 + self.e1 * other.e013 * other.e013 + self.e1 * other.e123 * other.e123 - self.e2 * other.e013 * other.e032 - self.e3 * other.e021 * other.e032,
            e01: -self.e01 * other.e021 * other.e021 - self.e01 * other.e013 * other.e013 + self.e02 * other.e013 * other.e032 + self.e12 * other.e013 * other.e123 + self.e03 * other.e021 * other.e032 - self.e31 * other.e021 * other.e123,
            e2: -self.e0 * other.e013 * other.e123 - self.e1 * other.e013 * other.e032 + self.e2 * other.e021 * other.e021 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 - self.e3 * other.e021 * other.e013,
            e02: self.e01 * other.e013 * other.e032 - self.e02 * other.e021 * other.e021 - self.e02 * other.e032 * other.e032 - self.e12 * other.e032 * other.e123 + self.e03 * other.e021 * other.e013 + self.e23 * other.e021 * other.e123,
            e12: self.e01 * other.e013 * other.e123 - self.e02 * other.e032 * other.e123 + self.e12 * other.e021 * other.e021 + self.e12 * other.e123 * other.e123 + self.e31 * other.e021 * other.e013 + self.e23 * other.e021 * other.e032,
            e021: -self.e021 * other.e021 * other.e021 - self.e013 * other.e021 * other.e013 - self.e032 * other.e021 * other.e032 + self.e123 * other.e021 * other.e123,
            e3: -self.e0 * other.e021 * other.e123 - self.e1 * other.e021 * other.e032 - self.e2 * other.e021 * other.e013 + self.e3 * other.e013 * other.e013 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123,
            e03: self.e01 * other.e021 * other.e032 + self.e02 * other.e021 * other.e013 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 + self.e31 * other.e032 * other.e123 - self.e23 * other.e013 * other.e123,
            e31: -self.e01 * other.e021 * other.e123 + self.e12 * other.e021 * other.e013 + self.e03 * other.e032 * other.e123 + self.e31 * other.e013 * other.e013 + self.e31 * other.e123 * other.e123 + self.e23 * other.e013 * other.e032,
            e013: -self.e021 * other.e021 * other.e013 - self.e013 * other.e013 * other.e013 - self.e032 * other.e013 * other.e032 + self.e123 * other.e013 * other.e123,
            e23: self.e02 * other.e021 * other.e123 + self.e12 * other.e021 * other.e032 - self.e03 * other.e013 * other.e123 + self.e31 * other.e013 * other.e032 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123,
            e032: -self.e021 * other.e021 * other.e032 - self.e013 * other.e013 * other.e032 - self.e032 * other.e032 * other.e032 + self.e123 * other.e032 * other.e123,
            e123: self.e021 * other.e021 * other.e123 + self.e013 * other.e013 * other.e123 + self.e032 * other.e032 * other.e123 + self.e123 * other.e123 * other.e123,
            e0123: 0.0,
        }
    }
}

impl Project<FourVector> for Multivector {
    type Output = Multivector;
    fn project(self, other: FourVector) -> Multivector {
        Multivector {
            s: self.s * other.e0123 * other.e0123,
            e0: -self.e0 * other.e0123 * other.e0123,
            e1: self.e1 * other.e0123 * other.e0123,
            e01: -self.e01 * other.e0123 * other.e0123,
            e2: self.e2 * other.e0123 * other.e0123,
            e02: -self.e02 * other.e0123 * other.e0123,
            e12: self.e12 * other.e0123 * other.e0123,
            e021: -self.e021 * other.e0123 * other.e0123,
            e3: self.e3 * other.e0123 * other.e0123,
            e03: -self.e03 * other.e0123 * other.e0123,
            e31: self.e31 * other.e0123 * other.e0123,
            e013: -self.e013 * other.e0123 * other.e0123,
            e23: self.e23 * other.e0123 * other.e0123,
            e032: -self.e032 * other.e0123 * other.e0123,
            e123: self.e123 * other.e0123 * other.e0123,
            e0123: -self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Null> for Multivector {
    type Output = Multivector;
    fn project(self, other: Null) -> Multivector {
        Multivector {
            s: 0.0,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl Project<OddMultivector> for Multivector {
    type Output = Multivector;
    fn project(self, other: OddMultivector) -> Multivector {
        Multivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 + self.e01 * other.e2 * other.e021 - self.e01 * other.e3 * other.e013 - self.e02 * other.e1 * other.e021 + self.e02 * other.e3 * other.e032 + self.e12 * other.e0 * other.e021 - self.e12 * other.e3 * other.e123 + self.e03 * other.e1 * other.e013 - self.e03 * other.e2 * other.e032 + self.e31 * other.e0 * other.e013 - self.e31 * other.e2 * other.e123 + self.e23 * other.e0 * other.e032 - self.e23 * other.e1 * other.e123,
            e0: -self.e0 * other.e0 * other.e0 - self.e0 * other.e021 * other.e021 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 + self.e1 * other.e0 * other.e1 - self.e1 * other.e032 * other.e123 + self.e2 * other.e0 * other.e2 - self.e2 * other.e013 * other.e123 + self.e021 * other.e0 * other.e021 + self.e3 * other.e0 * other.e3 - self.e3 * other.e021 * other.e123 + self.e013 * other.e0 * other.e013 + self.e032 * other.e0 * other.e032 - self.e123 * other.e0 * other.e123,
            e1: self.e0 * other.e0 * other.e1 - self.e0 * other.e032 * other.e123 + self.e1 * other.e1 * other.e1 + self.e1 * other.e021 * other.e021 + self.e1 * other.e013 * other.e013 + self.e1 * other.e123 * other.e123 + self.e2 * other.e1 * other.e2 - self.e2 * other.e013 * other.e032 - self.e021 * other.e1 * other.e021 + self.e3 * other.e1 * other.e3 - self.e3 * other.e021 * other.e032 - self.e013 * other.e1 * other.e013 - self.e032 * other.e1 * other.e032 - self.e123 * other.e1 * other.e123,
            e01: self.s * other.e2 * other.e021 - self.s * other.e3 * other.e013 - self.e01 * other.e021 * other.e021 - self.e01 * other.e013 * other.e013 + self.e02 * other.e013 * other.e032 + self.e12 * other.e013 * other.e123 + self.e03 * other.e021 * other.e032 - self.e31 * other.e021 * other.e123,
            e2: self.e0 * other.e0 * other.e2 - self.e0 * other.e013 * other.e123 + self.e1 * other.e1 * other.e2 - self.e1 * other.e013 * other.e032 + self.e2 * other.e2 * other.e2 + self.e2 * other.e021 * other.e021 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 - self.e021 * other.e2 * other.e021 + self.e3 * other.e2 * other.e3 - self.e3 * other.e021 * other.e013 - self.e013 * other.e2 * other.e013 - self.e032 * other.e2 * other.e032 - self.e123 * other.e2 * other.e123,
            e02: -self.s * other.e1 * other.e021 + self.s * other.e3 * other.e032 + self.e01 * other.e013 * other.e032 - self.e02 * other.e021 * other.e021 - self.e02 * other.e032 * other.e032 - self.e12 * other.e032 * other.e123 + self.e03 * other.e021 * other.e013 + self.e23 * other.e021 * other.e123,
            e12: self.s * other.e0 * other.e021 - self.s * other.e3 * other.e123 + self.e01 * other.e013 * other.e123 - self.e02 * other.e032 * other.e123 + self.e12 * other.e021 * other.e021 + self.e12 * other.e123 * other.e123 + self.e31 * other.e021 * other.e013 + self.e23 * other.e021 * other.e032,
            e021: self.e0 * other.e0 * other.e021 - self.e1 * other.e1 * other.e021 - self.e2 * other.e2 * other.e021 - self.e021 * other.e021 * other.e021 - self.e3 * other.e021 * other.e3 - self.e013 * other.e021 * other.e013 - self.e032 * other.e021 * other.e032 + self.e123 * other.e021 * other.e123,
            e3: self.e0 * other.e0 * other.e3 - self.e0 * other.e021 * other.e123 + self.e1 * other.e1 * other.e3 - self.e1 * other.e021 * other.e032 + self.e2 * other.e2 * other.e3 - self.e2 * other.e021 * other.e013 - self.e021 * other.e021 * other.e3 + self.e3 * other.e3 * other.e3 + self.e3 * other.e013 * other.e013 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123 - self.e013 * other.e3 * other.e013 - self.e032 * other.e3 * other.e032 - self.e123 * other.e3 * other.e123,
            e03: self.s * other.e1 * other.e013 - self.s * other.e2 * other.e032 + self.e01 * other.e021 * other.e032 + self.e02 * other.e021 * other.e013 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 + self.e31 * other.e032 * other.e123 - self.e23 * other.e013 * other.e123,
            e31: self.s * other.e0 * other.e013 - self.s * other.e2 * other.e123 - self.e01 * other.e021 * other.e123 + self.e12 * other.e021 * other.e013 + self.e03 * other.e032 * other.e123 + self.e31 * other.e013 * other.e013 + self.e31 * other.e123 * other.e123 + self.e23 * other.e013 * other.e032,
            e013: self.e0 * other.e0 * other.e013 - self.e1 * other.e1 * other.e013 - self.e2 * other.e2 * other.e013 - self.e021 * other.e021 * other.e013 - self.e3 * other.e3 * other.e013 - self.e013 * other.e013 * other.e013 - self.e032 * other.e013 * other.e032 + self.e123 * other.e013 * other.e123,
            e23: self.s * other.e0 * other.e032 - self.s * other.e1 * other.e123 + self.e02 * other.e021 * other.e123 + self.e12 * other.e021 * other.e032 - self.e03 * other.e013 * other.e123 + self.e31 * other.e013 * other.e032 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123,
            e032: self.e0 * other.e0 * other.e032 - self.e1 * other.e1 * other.e032 - self.e2 * other.e2 * other.e032 - self.e021 * other.e021 * other.e032 - self.e3 * other.e3 * other.e032 - self.e013 * other.e013 * other.e032 - self.e032 * other.e032 * other.e032 + self.e123 * other.e032 * other.e123,
            e123: -self.e0 * other.e0 * other.e123 - self.e1 * other.e1 * other.e123 - self.e2 * other.e2 * other.e123 + self.e021 * other.e021 * other.e123 - self.e3 * other.e3 * other.e123 + self.e013 * other.e013 * other.e123 + self.e032 * other.e032 * other.e123 + self.e123 * other.e123 * other.e123,
            e0123: 0.0,
        }
    }
}

impl Project<EvenMultivector> for Multivector {
    type Output = Multivector;
    fn project(self, other: EvenMultivector) -> Multivector {
        Multivector {
            s: self.s * other.s * other.s + self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23 + self.s * other.e0123 * other.e0123 - self.e01 * other.s * other.e01 - self.e01 * other.e23 * other.e0123 - self.e02 * other.s * other.e02 - self.e02 * other.e31 * other.e0123 - self.e12 * other.s * other.e12 - self.e12 * other.e03 * other.e0123 - self.e03 * other.s * other.e03 - self.e03 * other.e12 * other.e0123 - self.e31 * other.s * other.e31 - self.e31 * other.e02 * other.e0123 - self.e23 * other.s * other.e23 - self.e23 * other.e01 * other.e0123 + self.e0123 * other.s * other.e0123,
            e0: -self.e0 * other.e01 * other.e01 - self.e0 * other.e02 * other.e02 - self.e0 * other.e03 * other.e03 - self.e0 * other.e0123 * other.e0123 + self.e1 * other.e02 * other.e12 - self.e1 * other.e03 * other.e31 - self.e2 * other.e01 * other.e12 + self.e2 * other.e03 * other.e23 - self.e021 * other.e03 * other.e0123 + self.e3 * other.e01 * other.e31 - self.e3 * other.e02 * other.e23 - self.e013 * other.e02 * other.e0123 - self.e032 * other.e01 * other.e0123,
            e1: self.e0 * other.e02 * other.e12 - self.e0 * other.e03 * other.e31 + self.e1 * other.e01 * other.e01 + self.e1 * other.e12 * other.e12 + self.e1 * other.e31 * other.e31 + self.e1 * other.e0123 * other.e0123 + self.e2 * other.e01 * other.e02 - self.e2 * other.e31 * other.e23 - self.e021 * other.e31 * other.e0123 + self.e3 * other.e01 * other.e03 - self.e3 * other.e12 * other.e23 + self.e013 * other.e12 * other.e0123 - self.e123 * other.e01 * other.e0123,
            e01: -self.s * other.s * other.e01 - self.s * other.e23 * other.e0123 - self.e01 * other.e01 * other.e01 - self.e01 * other.e0123 * other.e0123 - self.e02 * other.e01 * other.e02 + self.e12 * other.e01 * other.e12 - self.e03 * other.e01 * other.e03 + self.e31 * other.e01 * other.e31 + self.e23 * other.e01 * other.e23 + self.e0123 * other.e01 * other.e0123,
            e2: -self.e0 * other.e01 * other.e12 + self.e0 * other.e03 * other.e23 + self.e1 * other.e01 * other.e02 - self.e1 * other.e31 * other.e23 + self.e2 * other.e02 * other.e02 + self.e2 * other.e12 * other.e12 + self.e2 * other.e23 * other.e23 + self.e2 * other.e0123 * other.e0123 + self.e021 * other.e23 * other.e0123 + self.e3 * other.e02 * other.e03 - self.e3 * other.e12 * other.e31 - self.e032 * other.e12 * other.e0123 - self.e123 * other.e02 * other.e0123,
            e02: -self.s * other.s * other.e02 - self.s * other.e31 * other.e0123 - self.e01 * other.e01 * other.e02 - self.e02 * other.e02 * other.e02 - self.e02 * other.e0123 * other.e0123 + self.e12 * other.e02 * other.e12 - self.e03 * other.e02 * other.e03 + self.e31 * other.e02 * other.e31 + self.e23 * other.e02 * other.e23 + self.e0123 * other.e02 * other.e0123,
            e12: -self.s * other.s * other.e12 - self.s * other.e03 * other.e0123 + self.e01 * other.e01 * other.e12 + self.e02 * other.e02 * other.e12 + self.e12 * other.e12 * other.e12 + self.e12 * other.e0123 * other.e0123 + self.e03 * other.e12 * other.e03 + self.e31 * other.e12 * other.e31 + self.e23 * other.e12 * other.e23 - self.e0123 * other.e12 * other.e0123,
            e021: -self.e0 * other.e03 * other.e0123 - self.e1 * other.e31 * other.e0123 + self.e2 * other.e23 * other.e0123 - self.e021 * other.e0123 * other.e0123,
            e3: self.e0 * other.e01 * other.e31 - self.e0 * other.e02 * other.e23 + self.e1 * other.e01 * other.e03 - self.e1 * other.e12 * other.e23 + self.e2 * other.e02 * other.e03 - self.e2 * other.e12 * other.e31 + self.e3 * other.e03 * other.e03 + self.e3 * other.e31 * other.e31 + self.e3 * other.e23 * other.e23 + self.e3 * other.e0123 * other.e0123 - self.e013 * other.e23 * other.e0123 + self.e032 * other.e31 * other.e0123 - self.e123 * other.e03 * other.e0123,
            e03: -self.s * other.s * other.e03 - self.s * other.e12 * other.e0123 - self.e01 * other.e01 * other.e03 - self.e02 * other.e02 * other.e03 + self.e12 * other.e12 * other.e03 - self.e03 * other.e03 * other.e03 - self.e03 * other.e0123 * other.e0123 + self.e31 * other.e03 * other.e31 + self.e23 * other.e03 * other.e23 + self.e0123 * other.e03 * other.e0123,
            e31: -self.s * other.s * other.e31 - self.s * other.e02 * other.e0123 + self.e01 * other.e01 * other.e31 + self.e02 * other.e02 * other.e31 + self.e12 * other.e12 * other.e31 + self.e03 * other.e03 * other.e31 + self.e31 * other.e31 * other.e31 + self.e31 * other.e0123 * other.e0123 + self.e23 * other.e31 * other.e23 - self.e0123 * other.e31 * other.e0123,
            e013: -self.e0 * other.e02 * other.e0123 + self.e1 * other.e12 * other.e0123 - self.e3 * other.e23 * other.e0123 - self.e013 * other.e0123 * other.e0123,
            e23: -self.s * other.s * other.e23 - self.s * other.e01 * other.e0123 + self.e01 * other.e01 * other.e23 + self.e02 * other.e02 * other.e23 + self.e12 * other.e12 * other.e23 + self.e03 * other.e03 * other.e23 + self.e31 * other.e31 * other.e23 + self.e23 * other.e23 * other.e23 + self.e23 * other.e0123 * other.e0123 - self.e0123 * other.e23 * other.e0123,
            e032: -self.e0 * other.e01 * other.e0123 - self.e2 * other.e12 * other.e0123 + self.e3 * other.e31 * other.e0123 - self.e032 * other.e0123 * other.e0123,
            e123: -self.e1 * other.e01 * other.e0123 - self.e2 * other.e02 * other.e0123 - self.e3 * other.e03 * other.e0123 + self.e123 * other.e0123 * other.e0123,
            e0123: self.s * other.s * other.e0123 + self.e01 * other.e01 * other.e0123 + self.e02 * other.e02 * other.e0123 - self.e12 * other.e12 * other.e0123 + self.e03 * other.e03 * other.e0123 - self.e31 * other.e31 * other.e0123 - self.e23 * other.e23 * other.e0123 - self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Project<Multivector> for Multivector {
    type Output = Multivector;
    fn project(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01 + self.s * other.e2 * other.e2 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e013 * other.e013 + self.s * other.e23 * other.e23 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 + self.s * other.e0123 * other.e0123 + self.e0 * other.s * other.e0 + self.e0 * other.e1 * other.e01 + self.e0 * other.e2 * other.e02 - self.e0 * other.e12 * other.e021 + self.e0 * other.e3 * other.e03 - self.e0 * other.e31 * other.e013 - self.e0 * other.e23 * other.e032 + self.e0 * other.e123 * other.e0123 + self.e1 * other.s * other.e1 - self.e1 * other.e0 * other.e01 + self.e1 * other.e2 * other.e12 + self.e1 * other.e02 * other.e021 - self.e1 * other.e3 * other.e31 - self.e1 * other.e03 * other.e013 + self.e1 * other.e23 * other.e123 + self.e1 * other.e032 * other.e0123 - self.e01 * other.s * other.e01 + self.e01 * other.e2 * other.e021 - self.e01 * other.e3 * other.e013 - self.e01 * other.e23 * other.e0123 + self.e2 * other.s * other.e2 - self.e2 * other.e0 * other.e02 - self.e2 * other.e1 * other.e12 - self.e2 * other.e01 * other.e021 + self.e2 * other.e3 * other.e23 + self.e2 * other.e03 * other.e032 + self.e2 * other.e31 * other.e123 + self.e2 * other.e013 * other.e0123 - self.e02 * other.s * other.e02 - self.e02 * other.e1 * other.e021 + self.e02 * other.e3 * other.e032 - self.e02 * other.e31 * other.e0123 - self.e12 * other.s * other.e12 + self.e12 * other.e0 * other.e021 - self.e12 * other.e3 * other.e123 - self.e12 * other.e03 * other.e0123 - self.e021 * other.s * other.e021 + self.e021 * other.e3 * other.e0123 + self.e3 * other.s * other.e3 - self.e3 * other.e0 * other.e03 + self.e3 * other.e1 * other.e31 + self.e3 * other.e01 * other.e013 - self.e3 * other.e2 * other.e23 - self.e3 * other.e02 * other.e032 + self.e3 * other.e12 * other.e123 + self.e3 * other.e021 * other.e0123 - self.e03 * other.s * other.e03 + self.e03 * other.e1 * other.e013 - self.e03 * other.e2 * other.e032 - self.e03 * other.e12 * other.e0123 - self.e31 * other.s * other.e31 + self.e31 * other.e0 * other.e013 - self.e31 * other.e2 * other.e123 - self.e31 * other.e02 * other.e0123 - self.e013 * other.s * other.e013 + self.e013 * other.e2 * other.e0123 - self.e23 * other.s * other.e23 + self.e23 * other.e0 * other.e032 - self.e23 * other.e1 * other.e123 - self.e23 * other.e01 * other.e0123 - self.e032 * other.s * other.e032 + self.e032 * other.e1 * other.e0123 - self.e123 * other.s * other.e123 + self.e123 * other.e0 * other.e0123 + self.e0123 * other.s * other.e0123,
            e0: self.s * other.s * other.e0 + self.s * other.e1 * other.e01 + self.s * other.e2 * other.e02 - self.s * other.e12 * other.e021 + self.s * other.e3 * other.e03 - self.s * other.e31 * other.e013 - self.s * other.e23 * other.e032 + self.s * other.e123 * other.e0123 - self.e0 * other.e0 * other.e0 - self.e0 * other.e01 * other.e01 - self.e0 * other.e02 * other.e02 - self.e0 * other.e021 * other.e021 - self.e0 * other.e03 * other.e03 - self.e0 * other.e013 * other.e013 - self.e0 * other.e032 * other.e032 - self.e0 * other.e0123 * other.e0123 + self.e1 * other.e0 * other.e1 + self.e1 * other.e02 * other.e12 - self.e1 * other.e03 * other.e31 - self.e1 * other.e032 * other.e123 + self.e01 * other.e0 * other.e01 - self.e01 * other.e02 * other.e021 + self.e01 * other.e03 * other.e013 - self.e01 * other.e032 * other.e0123 + self.e2 * other.e0 * other.e2 - self.e2 * other.e01 * other.e12 + self.e2 * other.e03 * other.e23 - self.e2 * other.e013 * other.e123 + self.e02 * other.e0 * other.e02 + self.e02 * other.e01 * other.e021 - self.e02 * other.e03 * other.e032 - self.e02 * other.e013 * other.e0123 - self.e12 * other.e0 * other.e12 - self.e12 * other.e03 * other.e123 + self.e021 * other.e0 * other.e021 - self.e021 * other.e03 * other.e0123 + self.e3 * other.e0 * other.e3 + self.e3 * other.e01 * other.e31 - self.e3 * other.e02 * other.e23 - self.e3 * other.e021 * other.e123 + self.e03 * other.e0 * other.e03 - self.e03 * other.e01 * other.e013 + self.e03 * other.e02 * other.e032 - self.e03 * other.e021 * other.e0123 - self.e31 * other.e0 * other.e31 - self.e31 * other.e02 * other.e123 + self.e013 * other.e0 * other.e013 - self.e013 * other.e02 * other.e0123 - self.e23 * other.e0 * other.e23 - self.e23 * other.e01 * other.e123 + self.e032 * other.e0 * other.e032 - self.e032 * other.e01 * other.e0123 - self.e123 * other.e0 * other.e123 - self.e0123 * other.e0 * other.e0123,
            e1: self.s * other.s * other.e1 - self.s * other.e0 * other.e01 + self.s * other.e2 * other.e12 + self.s * other.e02 * other.e021 - self.s * other.e3 * other.e31 - self.s * other.e03 * other.e013 + self.s * other.e23 * other.e123 + self.s * other.e032 * other.e0123 + self.e0 * other.e0 * other.e1 + self.e0 * other.e02 * other.e12 - self.e0 * other.e03 * other.e31 - self.e0 * other.e032 * other.e123 + self.e1 * other.e1 * other.e1 + self.e1 * other.e01 * other.e01 + self.e1 * other.e12 * other.e12 + self.e1 * other.e021 * other.e021 + self.e1 * other.e31 * other.e31 + self.e1 * other.e013 * other.e013 + self.e1 * other.e123 * other.e123 + self.e1 * other.e0123 * other.e0123 - self.e01 * other.e1 * other.e01 + self.e01 * other.e12 * other.e021 + self.e01 * other.e31 * other.e013 - self.e01 * other.e123 * other.e0123 + self.e2 * other.e1 * other.e2 + self.e2 * other.e01 * other.e02 - self.e2 * other.e31 * other.e23 - self.e2 * other.e013 * other.e032 - self.e02 * other.e1 * other.e02 - self.e02 * other.e31 * other.e032 - self.e12 * other.e1 * other.e12 - self.e12 * other.e01 * other.e021 + self.e12 * other.e31 * other.e123 + self.e12 * other.e013 * other.e0123 - self.e021 * other.e1 * other.e021 - self.e021 * other.e31 * other.e0123 + self.e3 * other.e1 * other.e3 + self.e3 * other.e01 * other.e03 - self.e3 * other.e12 * other.e23 - self.e3 * other.e021 * other.e032 - self.e03 * other.e1 * other.e03 - self.e03 * other.e12 * other.e032 - self.e31 * other.e1 * other.e31 - self.e31 * other.e01 * other.e013 - self.e31 * other.e12 * other.e123 - self.e31 * other.e021 * other.e0123 - self.e013 * other.e1 * other.e013 + self.e013 * other.e12 * other.e0123 - self.e23 * other.e1 * other.e23 - self.e23 * other.e01 * other.e032 - self.e032 * other.e1 * other.e032 - self.e123 * other.e1 * other.e123 - self.e123 * other.e01 * other.e0123 + self.e0123 * other.e1 * other.e0123,
            e01: -self.s * other.s * other.e01 + self.s * other.e2 * other.e021 - self.s * other.e3 * other.e013 - self.s * other.e23 * other.e0123 + self.e0 * other.e0 * other.e01 - self.e0 * other.e02 * other.e021 + self.e0 * other.e03 * other.e013 - self.e0 * other.e032 * other.e0123 - self.e1 * other.e1 * other.e01 + self.e1 * other.e12 * other.e021 + self.e1 * other.e31 * other.e013 - self.e1 * other.e123 * other.e0123 - self.e01 * other.e01 * other.e01 - self.e01 * other.e021 * other.e021 - self.e01 * other.e013 * other.e013 - self.e01 * other.e0123 * other.e0123 - self.e2 * other.e01 * other.e2 - self.e2 * other.e013 * other.e23 - self.e02 * other.e01 * other.e02 + self.e02 * other.e013 * other.e032 + self.e12 * other.e01 * other.e12 + self.e12 * other.e013 * other.e123 - self.e021 * other.e01 * other.e021 + self.e021 * other.e013 * other.e0123 - self.e3 * other.e01 * other.e3 - self.e3 * other.e021 * other.e23 - self.e03 * other.e01 * other.e03 + self.e03 * other.e021 * other.e032 + self.e31 * other.e01 * other.e31 - self.e31 * other.e021 * other.e123 - self.e013 * other.e01 * other.e013 - self.e013 * other.e021 * other.e0123 + self.e23 * other.e01 * other.e23 - self.e032 * other.e01 * other.e032 + self.e123 * other.e01 * other.e123 + self.e0123 * other.e01 * other.e0123,
            e2: self.s * other.s * other.e2 - self.s * other.e0 * other.e02 - self.s * other.e1 * other.e12 - self.s * other.e01 * other.e021 + self.s * other.e3 * other.e23 + self.s * other.e03 * other.e032 + self.s * other.e31 * other.e123 + self.s * other.e013 * other.e0123 + self.e0 * other.e0 * other.e2 - self.e0 * other.e01 * other.e12 + self.e0 * other.e03 * other.e23 - self.e0 * other.e013 * other.e123 + self.e1 * other.e1 * other.e2 + self.e1 * other.e01 * other.e02 - self.e1 * other.e31 * other.e23 - self.e1 * other.e013 * other.e032 - self.e01 * other.e01 * other.e2 - self.e01 * other.e013 * other.e23 + self.e2 * other.e2 * other.e2 + self.e2 * other.e02 * other.e02 + self.e2 * other.e12 * other.e12 + self.e2 * other.e021 * other.e021 + self.e2 * other.e23 * other.e23 + self.e2 * other.e032 * other.e032 + self.e2 * other.e123 * other.e123 + self.e2 * other.e0123 * other.e0123 - self.e02 * other.e2 * other.e02 + self.e02 * other.e12 * other.e021 + self.e02 * other.e23 * other.e032 - self.e02 * other.e123 * other.e0123 - self.e12 * other.e2 * other.e12 - self.e12 * other.e02 * other.e021 - self.e12 * other.e23 * other.e123 - self.e12 * other.e032 * other.e0123 - self.e021 * other.e2 * other.e021 + self.e021 * other.e23 * other.e0123 + self.e3 * other.e2 * other.e3 + self.e3 * other.e02 * other.e03 - self.e3 * other.e12 * other.e31 - self.e3 * other.e021 * other.e013 - self.e03 * other.e2 * other.e03 - self.e03 * other.e12 * other.e013 - self.e31 * other.e2 * other.e31 - self.e31 * other.e02 * other.e013 - self.e013 * other.e2 * other.e013 - self.e23 * other.e2 * other.e23 - self.e23 * other.e02 * other.e032 + self.e23 * other.e12 * other.e123 + self.e23 * other.e021 * other.e0123 - self.e032 * other.e2 * other.e032 - self.e032 * other.e12 * other.e0123 - self.e123 * other.e2 * other.e123 - self.e123 * other.e02 * other.e0123 + self.e0123 * other.e2 * other.e0123,
            e02: -self.s * other.s * other.e02 - self.s * other.e1 * other.e021 + self.s * other.e3 * other.e032 - self.s * other.e31 * other.e0123 + self.e0 * other.e0 * other.e02 + self.e0 * other.e01 * other.e021 - self.e0 * other.e03 * other.e032 - self.e0 * other.e013 * other.e0123 - self.e1 * other.e1 * other.e02 - self.e1 * other.e31 * other.e032 - self.e01 * other.e01 * other.e02 + self.e01 * other.e013 * other.e032 - self.e2 * other.e2 * other.e02 + self.e2 * other.e12 * other.e021 + self.e2 * other.e23 * other.e032 - self.e2 * other.e123 * other.e0123 - self.e02 * other.e02 * other.e02 - self.e02 * other.e021 * other.e021 - self.e02 * other.e032 * other.e032 - self.e02 * other.e0123 * other.e0123 + self.e12 * other.e02 * other.e12 - self.e12 * other.e032 * other.e123 - self.e021 * other.e02 * other.e021 - self.e021 * other.e032 * other.e0123 - self.e3 * other.e02 * other.e3 - self.e3 * other.e021 * other.e31 - self.e03 * other.e02 * other.e03 + self.e03 * other.e021 * other.e013 + self.e31 * other.e02 * other.e31 - self.e013 * other.e02 * other.e013 + self.e23 * other.e02 * other.e23 + self.e23 * other.e021 * other.e123 - self.e032 * other.e02 * other.e032 + self.e032 * other.e021 * other.e0123 + self.e123 * other.e02 * other.e123 + self.e0123 * other.e02 * other.e0123,
            e12: -self.s * other.s * other.e12 + self.s * other.e0 * other.e021 - self.s * other.e3 * other.e123 - self.s * other.e03 * other.e0123 - self.e0 * other.e0 * other.e12 - self.e0 * other.e03 * other.e123 - self.e1 * other.e1 * other.e12 - self.e1 * other.e01 * other.e021 + self.e1 * other.e31 * other.e123 + self.e1 * other.e013 * other.e0123 + self.e01 * other.e01 * other.e12 + self.e01 * other.e013 * other.e123 - self.e2 * other.e2 * other.e12 - self.e2 * other.e02 * other.e021 - self.e2 * other.e23 * other.e123 - self.e2 * other.e032 * other.e0123 + self.e02 * other.e02 * other.e12 - self.e02 * other.e032 * other.e123 + self.e12 * other.e12 * other.e12 + self.e12 * other.e021 * other.e021 + self.e12 * other.e123 * other.e123 + self.e12 * other.e0123 * other.e0123 + self.e021 * other.e12 * other.e021 - self.e021 * other.e123 * other.e0123 - self.e3 * other.e12 * other.e3 - self.e3 * other.e021 * other.e03 + self.e03 * other.e12 * other.e03 + self.e31 * other.e12 * other.e31 + self.e31 * other.e021 * other.e013 + self.e013 * other.e12 * other.e013 + self.e23 * other.e12 * other.e23 + self.e23 * other.e021 * other.e032 + self.e032 * other.e12 * other.e032 + self.e123 * other.e12 * other.e123 + self.e123 * other.e021 * other.e0123 - self.e0123 * other.e12 * other.e0123,
            e021: -self.s * other.s * other.e021 + self.s * other.e3 * other.e0123 + self.e0 * other.e0 * other.e021 - self.e0 * other.e03 * other.e0123 - self.e1 * other.e1 * other.e021 - self.e1 * other.e31 * other.e0123 - self.e01 * other.e01 * other.e021 + self.e01 * other.e013 * other.e0123 - self.e2 * other.e2 * other.e021 + self.e2 * other.e23 * other.e0123 - self.e02 * other.e02 * other.e021 - self.e02 * other.e032 * other.e0123 + self.e12 * other.e12 * other.e021 - self.e12 * other.e123 * other.e0123 - self.e021 * other.e021 * other.e021 - self.e021 * other.e0123 * other.e0123 - self.e3 * other.e021 * other.e3 - self.e03 * other.e021 * other.e03 + self.e31 * other.e021 * other.e31 - self.e013 * other.e021 * other.e013 + self.e23 * other.e021 * other.e23 - self.e032 * other.e021 * other.e032 + self.e123 * other.e021 * other.e123 + self.e0123 * other.e021 * other.e0123,
            e3: self.s * other.s * other.e3 - self.s * other.e0 * other.e03 + self.s * other.e1 * other.e31 + self.s * other.e01 * other.e013 - self.s * other.e2 * other.e23 - self.s * other.e02 * other.e032 + self.s * other.e12 * other.e123 + self.s * other.e021 * other.e0123 + self.e0 * other.e0 * other.e3 + self.e0 * other.e01 * other.e31 - self.e0 * other.e02 * other.e23 - self.e0 * other.e021 * other.e123 + self.e1 * other.e1 * other.e3 + self.e1 * other.e01 * other.e03 - self.e1 * other.e12 * other.e23 - self.e1 * other.e021 * other.e032 - self.e01 * other.e01 * other.e3 - self.e01 * other.e021 * other.e23 + self.e2 * other.e2 * other.e3 + self.e2 * other.e02 * other.e03 - self.e2 * other.e12 * other.e31 - self.e2 * other.e021 * other.e013 - self.e02 * other.e02 * other.e3 - self.e02 * other.e021 * other.e31 - self.e12 * other.e12 * other.e3 - self.e12 * other.e021 * other.e03 - self.e021 * other.e021 * other.e3 + self.e3 * other.e3 * other.e3 + self.e3 * other.e03 * other.e03 + self.e3 * other.e31 * other.e31 + self.e3 * other.e013 * other.e013 + self.e3 * other.e23 * other.e23 + self.e3 * other.e032 * other.e032 + self.e3 * other.e123 * other.e123 + self.e3 * other.e0123 * other.e0123 - self.e03 * other.e3 * other.e03 + self.e03 * other.e31 * other.e013 + self.e03 * other.e23 * other.e032 - self.e03 * other.e123 * other.e0123 - self.e31 * other.e3 * other.e31 - self.e31 * other.e03 * other.e013 + self.e31 * other.e23 * other.e123 + self.e31 * other.e032 * other.e0123 - self.e013 * other.e3 * other.e013 - self.e013 * other.e23 * other.e0123 - self.e23 * other.e3 * other.e23 - self.e23 * other.e03 * other.e032 - self.e23 * other.e31 * other.e123 - self.e23 * other.e013 * other.e0123 - self.e032 * other.e3 * other.e032 + self.e032 * other.e31 * other.e0123 - self.e123 * other.e3 * other.e123 - self.e123 * other.e03 * other.e0123 + self.e0123 * other.e3 * other.e0123,
            e03: -self.s * other.s * other.e03 + self.s * other.e1 * other.e013 - self.s * other.e2 * other.e032 - self.s * other.e12 * other.e0123 + self.e0 * other.e0 * other.e03 - self.e0 * other.e01 * other.e013 + self.e0 * other.e02 * other.e032 - self.e0 * other.e021 * other.e0123 - self.e1 * other.e1 * other.e03 - self.e1 * other.e12 * other.e032 - self.e01 * other.e01 * other.e03 + self.e01 * other.e021 * other.e032 - self.e2 * other.e2 * other.e03 - self.e2 * other.e12 * other.e013 - self.e02 * other.e02 * other.e03 + self.e02 * other.e021 * other.e013 + self.e12 * other.e12 * other.e03 - self.e021 * other.e021 * other.e03 - self.e3 * other.e3 * other.e03 + self.e3 * other.e31 * other.e013 + self.e3 * other.e23 * other.e032 - self.e3 * other.e123 * other.e0123 - self.e03 * other.e03 * other.e03 - self.e03 * other.e013 * other.e013 - self.e03 * other.e032 * other.e032 - self.e03 * other.e0123 * other.e0123 + self.e31 * other.e03 * other.e31 + self.e31 * other.e032 * other.e123 - self.e013 * other.e03 * other.e013 + self.e013 * other.e032 * other.e0123 + self.e23 * other.e03 * other.e23 - self.e23 * other.e013 * other.e123 - self.e032 * other.e03 * other.e032 - self.e032 * other.e013 * other.e0123 + self.e123 * other.e03 * other.e123 + self.e0123 * other.e03 * other.e0123,
            e31: -self.s * other.s * other.e31 + self.s * other.e0 * other.e013 - self.s * other.e2 * other.e123 - self.s * other.e02 * other.e0123 - self.e0 * other.e0 * other.e31 - self.e0 * other.e02 * other.e123 - self.e1 * other.e1 * other.e31 - self.e1 * other.e01 * other.e013 - self.e1 * other.e12 * other.e123 - self.e1 * other.e021 * other.e0123 + self.e01 * other.e01 * other.e31 - self.e01 * other.e021 * other.e123 - self.e2 * other.e2 * other.e31 - self.e2 * other.e02 * other.e013 + self.e02 * other.e02 * other.e31 + self.e12 * other.e12 * other.e31 + self.e12 * other.e021 * other.e013 + self.e021 * other.e021 * other.e31 - self.e3 * other.e3 * other.e31 - self.e3 * other.e03 * other.e013 + self.e3 * other.e23 * other.e123 + self.e3 * other.e032 * other.e0123 + self.e03 * other.e03 * other.e31 + self.e03 * other.e032 * other.e123 + self.e31 * other.e31 * other.e31 + self.e31 * other.e013 * other.e013 + self.e31 * other.e123 * other.e123 + self.e31 * other.e0123 * other.e0123 + self.e013 * other.e31 * other.e013 - self.e013 * other.e123 * other.e0123 + self.e23 * other.e31 * other.e23 + self.e23 * other.e013 * other.e032 + self.e032 * other.e31 * other.e032 + self.e123 * other.e31 * other.e123 + self.e123 * other.e013 * other.e0123 - self.e0123 * other.e31 * other.e0123,
            e013: -self.s * other.s * other.e013 + self.s * other.e2 * other.e0123 + self.e0 * other.e0 * other.e013 - self.e0 * other.e02 * other.e0123 - self.e1 * other.e1 * other.e013 + self.e1 * other.e12 * other.e0123 - self.e01 * other.e01 * other.e013 - self.e01 * other.e021 * other.e0123 - self.e2 * other.e2 * other.e013 - self.e02 * other.e02 * other.e013 + self.e12 * other.e12 * other.e013 - self.e021 * other.e021 * other.e013 - self.e3 * other.e3 * other.e013 - self.e3 * other.e23 * other.e0123 - self.e03 * other.e03 * other.e013 + self.e03 * other.e032 * other.e0123 + self.e31 * other.e31 * other.e013 - self.e31 * other.e123 * other.e0123 - self.e013 * other.e013 * other.e013 - self.e013 * other.e0123 * other.e0123 + self.e23 * other.e013 * other.e23 - self.e032 * other.e013 * other.e032 + self.e123 * other.e013 * other.e123 + self.e0123 * other.e013 * other.e0123,
            e23: -self.s * other.s * other.e23 + self.s * other.e0 * other.e032 - self.s * other.e1 * other.e123 - self.s * other.e01 * other.e0123 - self.e0 * other.e0 * other.e23 - self.e0 * other.e01 * other.e123 - self.e1 * other.e1 * other.e23 - self.e1 * other.e01 * other.e032 + self.e01 * other.e01 * other.e23 - self.e2 * other.e2 * other.e23 - self.e2 * other.e02 * other.e032 + self.e2 * other.e12 * other.e123 + self.e2 * other.e021 * other.e0123 + self.e02 * other.e02 * other.e23 + self.e02 * other.e021 * other.e123 + self.e12 * other.e12 * other.e23 + self.e12 * other.e021 * other.e032 + self.e021 * other.e021 * other.e23 - self.e3 * other.e3 * other.e23 - self.e3 * other.e03 * other.e032 - self.e3 * other.e31 * other.e123 - self.e3 * other.e013 * other.e0123 + self.e03 * other.e03 * other.e23 - self.e03 * other.e013 * other.e123 + self.e31 * other.e31 * other.e23 + self.e31 * other.e013 * other.e032 + self.e013 * other.e013 * other.e23 + self.e23 * other.e23 * other.e23 + self.e23 * other.e032 * other.e032 + self.e23 * other.e123 * other.e123 + self.e23 * other.e0123 * other.e0123 + self.e032 * other.e23 * other.e032 - self.e032 * other.e123 * other.e0123 + self.e123 * other.e23 * other.e123 + self.e123 * other.e032 * other.e0123 - self.e0123 * other.e23 * other.e0123,
            e032: -self.s * other.s * other.e032 + self.s * other.e1 * other.e0123 + self.e0 * other.e0 * other.e032 - self.e0 * other.e01 * other.e0123 - self.e1 * other.e1 * other.e032 - self.e01 * other.e01 * other.e032 - self.e2 * other.e2 * other.e032 - self.e2 * other.e12 * other.e0123 - self.e02 * other.e02 * other.e032 + self.e02 * other.e021 * other.e0123 + self.e12 * other.e12 * other.e032 - self.e021 * other.e021 * other.e032 - self.e3 * other.e3 * other.e032 + self.e3 * other.e31 * other.e0123 - self.e03 * other.e03 * other.e032 - self.e03 * other.e013 * other.e0123 + self.e31 * other.e31 * other.e032 - self.e013 * other.e013 * other.e032 + self.e23 * other.e23 * other.e032 - self.e23 * other.e123 * other.e0123 - self.e032 * other.e032 * other.e032 - self.e032 * other.e0123 * other.e0123 + self.e123 * other.e032 * other.e123 + self.e0123 * other.e032 * other.e0123,
            e123: -self.s * other.s * other.e123 + self.s * other.e0 * other.e0123 - self.e0 * other.e0 * other.e123 - self.e1 * other.e1 * other.e123 - self.e1 * other.e01 * other.e0123 + self.e01 * other.e01 * other.e123 - self.e2 * other.e2 * other.e123 - self.e2 * other.e02 * other.e0123 + self.e02 * other.e02 * other.e123 + self.e12 * other.e12 * other.e123 + self.e12 * other.e021 * other.e0123 + self.e021 * other.e021 * other.e123 - self.e3 * other.e3 * other.e123 - self.e3 * other.e03 * other.e0123 + self.e03 * other.e03 * other.e123 + self.e31 * other.e31 * other.e123 + self.e31 * other.e013 * other.e0123 + self.e013 * other.e013 * other.e123 + self.e23 * other.e23 * other.e123 + self.e23 * other.e032 * other.e0123 + self.e032 * other.e032 * other.e123 + self.e123 * other.e123 * other.e123 + self.e123 * other.e0123 * other.e0123 - self.e0123 * other.e123 * other.e0123,
            e0123: self.s * other.s * other.e0123 - self.e0 * other.e0 * other.e0123 + self.e1 * other.e1 * other.e0123 + self.e01 * other.e01 * other.e0123 + self.e2 * other.e2 * other.e0123 + self.e02 * other.e02 * other.e0123 - self.e12 * other.e12 * other.e0123 + self.e021 * other.e021 * other.e0123 + self.e3 * other.e3 * other.e0123 + self.e03 * other.e03 * other.e0123 - self.e31 * other.e31 * other.e0123 + self.e013 * other.e013 * other.e0123 - self.e23 * other.e23 * other.e0123 + self.e032 * other.e032 * other.e0123 - self.e123 * other.e123 * other.e0123 - self.e0123 * other.e0123 * other.e0123,
        }
    }
}

impl Reject<Scalar> for Scalar {
    type Output = Scalar;
    fn reject(self, other: Scalar) -> Scalar {
        Scalar {
            s: self.s * other.s * other.s,
        }
    }
}

impl Reject<Vector> for Scalar {
    type Output = Scalar;
    fn reject(self, other: Vector) -> Scalar {
        Scalar {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e3 * other.e3,
        }
    }
}

impl Reject<Bivector> for Scalar {
    type Output = Scalar;
    fn reject(self, other: Bivector) -> Scalar {
        Scalar {
            s: self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23,
        }
    }
}

impl Reject<Trivector> for Scalar {
    type Output = Scalar;
    fn reject(self, other: Trivector) -> Scalar {
        Scalar {
            s: self.s * other.e021 * other.e021 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123,
        }
    }
}

impl Reject<FourVector> for Scalar {
    type Output = Scalar;
    fn reject(self, other: FourVector) -> Scalar {
        Scalar {
            s: self.s * other.e0123 * other.e0123,
        }
    }
}

impl Reject<Null> for Scalar {
    type Output = EvenMultivector;
    fn reject(self, other: Null) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Reject<OddMultivector> for Scalar {
    type Output = EvenMultivector;
    fn reject(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123,
            e01: -self.s * other.e2 * other.e021 + self.s * other.e3 * other.e013,
            e02: self.s * other.e1 * other.e021 - self.s * other.e3 * other.e032,
            e12: -self.s * other.e0 * other.e021 + self.s * other.e3 * other.e123,
            e03: -self.s * other.e1 * other.e013 + self.s * other.e2 * other.e032,
            e31: -self.s * other.e0 * other.e013 + self.s * other.e2 * other.e123,
            e23: -self.s * other.e0 * other.e032 + self.s * other.e1 * other.e123,
            e0123: 0.0,
        }
    }
}

impl Reject<EvenMultivector> for Scalar {
    type Output = EvenMultivector;
    fn reject(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s * other.s + self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23 + self.s * other.e0123 * other.e0123,
            e01: self.s * other.s * other.e01 + self.s * other.e23 * other.e0123,
            e02: self.s * other.s * other.e02 + self.s * other.e31 * other.e0123,
            e12: self.s * other.s * other.e12 + self.s * other.e03 * other.e0123,
            e03: self.s * other.s * other.e03 + self.s * other.e12 * other.e0123,
            e31: self.s * other.s * other.e31 + self.s * other.e02 * other.e0123,
            e23: self.s * other.s * other.e23 + self.s * other.e01 * other.e0123,
            e0123: self.s * other.s * other.e0123,
        }
    }
}

impl Reject<Multivector> for Scalar {
    type Output = EvenMultivector;
    fn reject(self, other: Multivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01 + self.s * other.e2 * other.e2 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e013 * other.e013 + self.s * other.e23 * other.e23 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 + self.s * other.e0123 * other.e0123,
            e01: self.s * other.s * other.e01 - self.s * other.e2 * other.e021 + self.s * other.e3 * other.e013 + self.s * other.e23 * other.e0123,
            e02: self.s * other.s * other.e02 + self.s * other.e1 * other.e021 - self.s * other.e3 * other.e032 + self.s * other.e31 * other.e0123,
            e12: self.s * other.s * other.e12 - self.s * other.e0 * other.e021 + self.s * other.e3 * other.e123 + self.s * other.e03 * other.e0123,
            e03: self.s * other.s * other.e03 - self.s * other.e1 * other.e013 + self.s * other.e2 * other.e032 + self.s * other.e12 * other.e0123,
            e31: self.s * other.s * other.e31 - self.s * other.e0 * other.e013 + self.s * other.e2 * other.e123 + self.s * other.e02 * other.e0123,
            e23: self.s * other.s * other.e23 - self.s * other.e0 * other.e032 + self.s * other.e1 * other.e123 + self.s * other.e01 * other.e0123,
            e0123: self.s * other.s * other.e0123,
        }
    }
}

impl Reject<Scalar> for Vector {
    type Output = Vector;
    fn reject(self, other: Scalar) -> Vector {
        Vector {
            e0: self.e0 * other.s * other.s,
            e1: self.e1 * other.s * other.s,
            e2: self.e2 * other.s * other.s,
            e3: self.e3 * other.s * other.s,
        }
    }
}

impl Reject<Vector> for Vector {
    type Output = Vector;
    fn reject(self, other: Vector) -> Vector {
        Vector {
            e0: self.e0 * other.e1 * other.e1 + self.e0 * other.e2 * other.e2 + self.e0 * other.e3 * other.e3 - self.e1 * other.e0 * other.e1 - self.e2 * other.e0 * other.e2 - self.e3 * other.e0 * other.e3,
            e1: -self.e0 * other.e0 * other.e1 + self.e1 * other.e0 * other.e0 + self.e1 * other.e2 * other.e2 + self.e1 * other.e3 * other.e3 - self.e2 * other.e1 * other.e2 - self.e3 * other.e1 * other.e3,
            e2: -self.e0 * other.e0 * other.e2 - self.e1 * other.e1 * other.e2 + self.e2 * other.e0 * other.e0 + self.e2 * other.e1 * other.e1 + self.e2 * other.e3 * other.e3 - self.e3 * other.e2 * other.e3,
            e3: -self.e0 * other.e0 * other.e3 - self.e1 * other.e1 * other.e3 - self.e2 * other.e2 * other.e3 + self.e3 * other.e0 * other.e0 + self.e3 * other.e1 * other.e1 + self.e3 * other.e2 * other.e2,
        }
    }
}

impl Reject<Bivector> for Vector {
    type Output = Vector;
    fn reject(self, other: Bivector) -> Vector {
        Vector {
            e0: self.e0 * other.e12 * other.e12 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 - self.e1 * other.e02 * other.e12 + self.e1 * other.e03 * other.e31 + self.e2 * other.e01 * other.e12 - self.e2 * other.e03 * other.e23 - self.e3 * other.e01 * other.e31 + self.e3 * other.e02 * other.e23,
            e1: -self.e0 * other.e02 * other.e12 + self.e0 * other.e03 * other.e31 + self.e1 * other.e02 * other.e02 + self.e1 * other.e03 * other.e03 + self.e1 * other.e23 * other.e23 - self.e2 * other.e01 * other.e02 + self.e2 * other.e31 * other.e23 - self.e3 * other.e01 * other.e03 + self.e3 * other.e12 * other.e23,
            e2: self.e0 * other.e01 * other.e12 - self.e0 * other.e03 * other.e23 - self.e1 * other.e01 * other.e02 + self.e1 * other.e31 * other.e23 + self.e2 * other.e01 * other.e01 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e3 * other.e02 * other.e03 + self.e3 * other.e12 * other.e31,
            e3: -self.e0 * other.e01 * other.e31 + self.e0 * other.e02 * other.e23 - self.e1 * other.e01 * other.e03 + self.e1 * other.e12 * other.e23 - self.e2 * other.e02 * other.e03 + self.e2 * other.e12 * other.e31 + self.e3 * other.e01 * other.e01 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12,
        }
    }
}

impl Reject<Trivector> for Vector {
    type Output = Vector;
    fn reject(self, other: Trivector) -> Vector {
        Vector {
            e0: self.e0 * other.e123 * other.e123 + self.e1 * other.e032 * other.e123 + self.e2 * other.e013 * other.e123 + self.e3 * other.e021 * other.e123,
            e1: self.e0 * other.e032 * other.e123 + self.e1 * other.e032 * other.e032 + self.e2 * other.e013 * other.e032 + self.e3 * other.e021 * other.e032,
            e2: self.e0 * other.e013 * other.e123 + self.e1 * other.e013 * other.e032 + self.e2 * other.e013 * other.e013 + self.e3 * other.e021 * other.e013,
            e3: self.e0 * other.e021 * other.e123 + self.e1 * other.e021 * other.e032 + self.e2 * other.e021 * other.e013 + self.e3 * other.e021 * other.e021,
        }
    }
}

impl Reject<FourVector> for Vector {
    type Output = Null;
    fn reject(self, other: FourVector) -> Null {
        Null
    }
}

impl Reject<Null> for Vector {
    type Output = OddMultivector;
    fn reject(self, other: Null) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Reject<OddMultivector> for Vector {
    type Output = OddMultivector;
    fn reject(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.e1 * other.e1 + self.e0 * other.e2 * other.e2 + self.e0 * other.e3 * other.e3 + self.e0 * other.e123 * other.e123 - self.e1 * other.e0 * other.e1 + self.e1 * other.e032 * other.e123 - self.e2 * other.e0 * other.e2 + self.e2 * other.e013 * other.e123 - self.e3 * other.e0 * other.e3 + self.e3 * other.e021 * other.e123,
            e1: -self.e0 * other.e0 * other.e1 + self.e0 * other.e032 * other.e123 + self.e1 * other.e0 * other.e0 + self.e1 * other.e2 * other.e2 + self.e1 * other.e3 * other.e3 + self.e1 * other.e032 * other.e032 - self.e2 * other.e1 * other.e2 + self.e2 * other.e013 * other.e032 - self.e3 * other.e1 * other.e3 + self.e3 * other.e021 * other.e032,
            e2: -self.e0 * other.e0 * other.e2 + self.e0 * other.e013 * other.e123 - self.e1 * other.e1 * other.e2 + self.e1 * other.e013 * other.e032 + self.e2 * other.e0 * other.e0 + self.e2 * other.e1 * other.e1 + self.e2 * other.e3 * other.e3 + self.e2 * other.e013 * other.e013 - self.e3 * other.e2 * other.e3 + self.e3 * other.e021 * other.e013,
            e021: -self.e0 * other.e3 * other.e123 - self.e1 * other.e3 * other.e032 - self.e2 * other.e3 * other.e013 - self.e3 * other.e021 * other.e3,
            e3: -self.e0 * other.e0 * other.e3 + self.e0 * other.e021 * other.e123 - self.e1 * other.e1 * other.e3 + self.e1 * other.e021 * other.e032 - self.e2 * other.e2 * other.e3 + self.e2 * other.e021 * other.e013 + self.e3 * other.e0 * other.e0 + self.e3 * other.e1 * other.e1 + self.e3 * other.e2 * other.e2 + self.e3 * other.e021 * other.e021,
            e013: -self.e0 * other.e2 * other.e123 - self.e1 * other.e2 * other.e032 - self.e2 * other.e2 * other.e013 - self.e3 * other.e2 * other.e021,
            e032: -self.e0 * other.e1 * other.e123 - self.e1 * other.e1 * other.e032 - self.e2 * other.e1 * other.e013 - self.e3 * other.e1 * other.e021,
            e123: -self.e0 * other.e0 * other.e123 - self.e1 * other.e0 * other.e032 - self.e2 * other.e0 * other.e013 - self.e3 * other.e0 * other.e021,
        }
    }
}

impl Reject<EvenMultivector> for Vector {
    type Output = OddMultivector;
    fn reject(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s * other.s + self.e0 * other.e12 * other.e12 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 - self.e1 * other.e02 * other.e12 + self.e1 * other.e03 * other.e31 + self.e2 * other.e01 * other.e12 - self.e2 * other.e03 * other.e23 - self.e3 * other.e01 * other.e31 + self.e3 * other.e02 * other.e23,
            e1: -self.e0 * other.e02 * other.e12 + self.e0 * other.e03 * other.e31 + self.e1 * other.s * other.s + self.e1 * other.e02 * other.e02 + self.e1 * other.e03 * other.e03 + self.e1 * other.e23 * other.e23 - self.e2 * other.e01 * other.e02 + self.e2 * other.e31 * other.e23 - self.e3 * other.e01 * other.e03 + self.e3 * other.e12 * other.e23,
            e2: self.e0 * other.e01 * other.e12 - self.e0 * other.e03 * other.e23 - self.e1 * other.e01 * other.e02 + self.e1 * other.e31 * other.e23 + self.e2 * other.s * other.s + self.e2 * other.e01 * other.e01 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e3 * other.e02 * other.e03 + self.e3 * other.e12 * other.e31,
            e021: -self.e0 * other.s * other.e12 + self.e1 * other.s * other.e02 - self.e2 * other.s * other.e01,
            e3: -self.e0 * other.e01 * other.e31 + self.e0 * other.e02 * other.e23 - self.e1 * other.e01 * other.e03 + self.e1 * other.e12 * other.e23 - self.e2 * other.e02 * other.e03 + self.e2 * other.e12 * other.e31 + self.e3 * other.s * other.s + self.e3 * other.e01 * other.e01 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12,
            e013: -self.e0 * other.s * other.e31 - self.e1 * other.s * other.e03 + self.e3 * other.s * other.e01,
            e032: -self.e0 * other.s * other.e23 + self.e2 * other.s * other.e03 - self.e3 * other.s * other.e02,
            e123: self.e1 * other.s * other.e23 + self.e2 * other.s * other.e31 + self.e3 * other.s * other.e12,
        }
    }
}

impl Reject<Multivector> for Vector {
    type Output = OddMultivector;
    fn reject(self, other: Multivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s * other.s + self.e0 * other.e1 * other.e1 + self.e0 * other.e2 * other.e2 + self.e0 * other.e12 * other.e12 + self.e0 * other.e3 * other.e3 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 + self.e0 * other.e123 * other.e123 - self.e1 * other.e0 * other.e1 - self.e1 * other.e02 * other.e12 + self.e1 * other.e03 * other.e31 + self.e1 * other.e032 * other.e123 - self.e2 * other.e0 * other.e2 + self.e2 * other.e01 * other.e12 - self.e2 * other.e03 * other.e23 + self.e2 * other.e013 * other.e123 - self.e3 * other.e0 * other.e3 - self.e3 * other.e01 * other.e31 + self.e3 * other.e02 * other.e23 + self.e3 * other.e021 * other.e123,
            e1: -self.e0 * other.e0 * other.e1 - self.e0 * other.e02 * other.e12 + self.e0 * other.e03 * other.e31 + self.e0 * other.e032 * other.e123 + self.e1 * other.s * other.s + self.e1 * other.e0 * other.e0 + self.e1 * other.e2 * other.e2 + self.e1 * other.e02 * other.e02 + self.e1 * other.e3 * other.e3 + self.e1 * other.e03 * other.e03 + self.e1 * other.e23 * other.e23 + self.e1 * other.e032 * other.e032 - self.e2 * other.e1 * other.e2 - self.e2 * other.e01 * other.e02 + self.e2 * other.e31 * other.e23 + self.e2 * other.e013 * other.e032 - self.e3 * other.e1 * other.e3 - self.e3 * other.e01 * other.e03 + self.e3 * other.e12 * other.e23 + self.e3 * other.e021 * other.e032,
            e2: -self.e0 * other.e0 * other.e2 + self.e0 * other.e01 * other.e12 - self.e0 * other.e03 * other.e23 + self.e0 * other.e013 * other.e123 - self.e1 * other.e1 * other.e2 - self.e1 * other.e01 * other.e02 + self.e1 * other.e31 * other.e23 + self.e1 * other.e013 * other.e032 + self.e2 * other.s * other.s + self.e2 * other.e0 * other.e0 + self.e2 * other.e1 * other.e1 + self.e2 * other.e01 * other.e01 + self.e2 * other.e3 * other.e3 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 + self.e2 * other.e013 * other.e013 - self.e3 * other.e2 * other.e3 - self.e3 * other.e02 * other.e03 + self.e3 * other.e12 * other.e31 + self.e3 * other.e021 * other.e013,
            e021: -self.e0 * other.s * other.e12 - self.e0 * other.e3 * other.e123 + self.e1 * other.s * other.e02 - self.e1 * other.e3 * other.e032 - self.e2 * other.s * other.e01 - self.e2 * other.e3 * other.e013 - self.e3 * other.e021 * other.e3,
            e3: -self.e0 * other.e0 * other.e3 - self.e0 * other.e01 * other.e31 + self.e0 * other.e02 * other.e23 + self.e0 * other.e021 * other.e123 - self.e1 * other.e1 * other.e3 - self.e1 * other.e01 * other.e03 + self.e1 * other.e12 * other.e23 + self.e1 * other.e021 * other.e032 - self.e2 * other.e2 * other.e3 - self.e2 * other.e02 * other.e03 + self.e2 * other.e12 * other.e31 + self.e2 * other.e021 * other.e013 + self.e3 * other.s * other.s + self.e3 * other.e0 * other.e0 + self.e3 * other.e1 * other.e1 + self.e3 * other.e01 * other.e01 + self.e3 * other.e2 * other.e2 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12 + self.e3 * other.e021 * other.e021,
            e013: -self.e0 * other.s * other.e31 - self.e0 * other.e2 * other.e123 - self.e1 * other.s * other.e03 - self.e1 * other.e2 * other.e032 - self.e2 * other.e2 * other.e013 + self.e3 * other.s * other.e01 - self.e3 * other.e2 * other.e021,
            e032: -self.e0 * other.s * other.e23 - self.e0 * other.e1 * other.e123 - self.e1 * other.e1 * other.e032 + self.e2 * other.s * other.e03 - self.e2 * other.e1 * other.e013 - self.e3 * other.s * other.e02 - self.e3 * other.e1 * other.e021,
            e123: -self.e0 * other.e0 * other.e123 + self.e1 * other.s * other.e23 - self.e1 * other.e0 * other.e032 + self.e2 * other.s * other.e31 - self.e2 * other.e0 * other.e013 + self.e3 * other.s * other.e12 - self.e3 * other.e0 * other.e021,
        }
    }
}

impl Reject<Scalar> for Bivector {
    type Output = Bivector;
    fn reject(self, other: Scalar) -> Bivector {
        Bivector {
            e01: self.e01 * other.s * other.s,
            e02: self.e02 * other.s * other.s,
            e12: self.e12 * other.s * other.s,
            e03: self.e03 * other.s * other.s,
            e31: self.e31 * other.s * other.s,
            e23: self.e23 * other.s * other.s,
        }
    }
}

impl Reject<Vector> for Bivector {
    type Output = Bivector;
    fn reject(self, other: Vector) -> Bivector {
        Bivector {
            e01: self.e01 * other.e2 * other.e2 + self.e01 * other.e3 * other.e3 - self.e02 * other.e1 * other.e2 + self.e12 * other.e0 * other.e2 - self.e03 * other.e1 * other.e3 - self.e31 * other.e0 * other.e3,
            e02: -self.e01 * other.e1 * other.e2 + self.e02 * other.e1 * other.e1 + self.e02 * other.e3 * other.e3 - self.e12 * other.e0 * other.e1 - self.e03 * other.e2 * other.e3 + self.e23 * other.e0 * other.e3,
            e12: self.e01 * other.e0 * other.e2 - self.e02 * other.e0 * other.e1 + self.e12 * other.e0 * other.e0 + self.e12 * other.e3 * other.e3 + self.e31 * other.e2 * other.e3 + self.e23 * other.e1 * other.e3,
            e03: -self.e01 * other.e1 * other.e3 - self.e02 * other.e2 * other.e3 + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 + self.e31 * other.e0 * other.e1 - self.e23 * other.e0 * other.e2,
            e31: -self.e01 * other.e0 * other.e3 + self.e12 * other.e2 * other.e3 + self.e03 * other.e0 * other.e1 + self.e31 * other.e0 * other.e0 + self.e31 * other.e2 * other.e2 + self.e23 * other.e1 * other.e2,
            e23: self.e02 * other.e0 * other.e3 + self.e12 * other.e1 * other.e3 - self.e03 * other.e0 * other.e2 + self.e31 * other.e1 * other.e2 + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1,
        }
    }
}

impl Reject<Bivector> for Bivector {
    type Output = Bivector;
    fn reject(self, other: Bivector) -> Bivector {
        Bivector {
            e01: self.e01 * other.e23 * other.e23 + self.e02 * other.e31 * other.e23 + self.e12 * other.e03 * other.e23 + self.e03 * other.e12 * other.e23 + self.e31 * other.e02 * other.e23 + self.e23 * other.e01 * other.e23,
            e02: self.e01 * other.e31 * other.e23 + self.e02 * other.e31 * other.e31 + self.e12 * other.e03 * other.e31 + self.e03 * other.e12 * other.e31 + self.e31 * other.e02 * other.e31 + self.e23 * other.e01 * other.e31,
            e12: self.e01 * other.e03 * other.e23 + self.e02 * other.e03 * other.e31 + self.e12 * other.e03 * other.e03 + self.e03 * other.e12 * other.e03 + self.e31 * other.e02 * other.e03 + self.e23 * other.e01 * other.e03,
            e03: self.e01 * other.e12 * other.e23 + self.e02 * other.e12 * other.e31 + self.e12 * other.e12 * other.e03 + self.e03 * other.e12 * other.e12 + self.e31 * other.e02 * other.e12 + self.e23 * other.e01 * other.e12,
            e31: self.e01 * other.e02 * other.e23 + self.e02 * other.e02 * other.e31 + self.e12 * other.e02 * other.e03 + self.e03 * other.e02 * other.e12 + self.e31 * other.e02 * other.e02 + self.e23 * other.e01 * other.e02,
            e23: self.e01 * other.e01 * other.e23 + self.e02 * other.e01 * other.e31 + self.e12 * other.e01 * other.e03 + self.e03 * other.e01 * other.e12 + self.e31 * other.e01 * other.e02 + self.e23 * other.e01 * other.e01,
        }
    }
}

impl Reject<Trivector> for Bivector {
    type Output = Null;
    fn reject(self, other: Trivector) -> Null {
        Null
    }
}

impl Reject<FourVector> for Bivector {
    type Output = Null;
    fn reject(self, other: FourVector) -> Null {
        Null
    }
}

impl Reject<Null> for Bivector {
    type Output = EvenMultivector;
    fn reject(self, other: Null) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Reject<OddMultivector> for Bivector {
    type Output = EvenMultivector;
    fn reject(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: -self.e01 * other.e2 * other.e021 + self.e01 * other.e3 * other.e013 + self.e02 * other.e1 * other.e021 - self.e02 * other.e3 * other.e032 - self.e12 * other.e0 * other.e021 + self.e12 * other.e3 * other.e123 - self.e03 * other.e1 * other.e013 + self.e03 * other.e2 * other.e032 - self.e31 * other.e0 * other.e013 + self.e31 * other.e2 * other.e123 - self.e23 * other.e0 * other.e032 + self.e23 * other.e1 * other.e123,
            e01: self.e01 * other.e2 * other.e2 + self.e01 * other.e3 * other.e3 - self.e02 * other.e1 * other.e2 + self.e12 * other.e0 * other.e2 - self.e03 * other.e1 * other.e3 - self.e31 * other.e0 * other.e3,
            e02: -self.e01 * other.e1 * other.e2 + self.e02 * other.e1 * other.e1 + self.e02 * other.e3 * other.e3 - self.e12 * other.e0 * other.e1 - self.e03 * other.e2 * other.e3 + self.e23 * other.e0 * other.e3,
            e12: self.e01 * other.e0 * other.e2 - self.e02 * other.e0 * other.e1 + self.e12 * other.e0 * other.e0 + self.e12 * other.e3 * other.e3 + self.e31 * other.e2 * other.e3 + self.e23 * other.e1 * other.e3,
            e03: -self.e01 * other.e1 * other.e3 - self.e02 * other.e2 * other.e3 + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 + self.e31 * other.e0 * other.e1 - self.e23 * other.e0 * other.e2,
            e31: -self.e01 * other.e0 * other.e3 + self.e12 * other.e2 * other.e3 + self.e03 * other.e0 * other.e1 + self.e31 * other.e0 * other.e0 + self.e31 * other.e2 * other.e2 + self.e23 * other.e1 * other.e2,
            e23: self.e02 * other.e0 * other.e3 + self.e12 * other.e1 * other.e3 - self.e03 * other.e0 * other.e2 + self.e31 * other.e1 * other.e2 + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1,
            e0123: 0.0,
        }
    }
}

impl Reject<EvenMultivector> for Bivector {
    type Output = EvenMultivector;
    fn reject(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.s * other.e01 + self.e01 * other.e23 * other.e0123 + self.e02 * other.s * other.e02 + self.e02 * other.e31 * other.e0123 + self.e12 * other.s * other.e12 + self.e12 * other.e03 * other.e0123 + self.e03 * other.s * other.e03 + self.e03 * other.e12 * other.e0123 + self.e31 * other.s * other.e31 + self.e31 * other.e02 * other.e0123 + self.e23 * other.s * other.e23 + self.e23 * other.e01 * other.e0123,
            e01: self.e01 * other.s * other.s + self.e01 * other.e23 * other.e23 + self.e02 * other.e31 * other.e23 + self.e12 * other.e03 * other.e23 + self.e03 * other.e12 * other.e23 + self.e31 * other.e02 * other.e23 + self.e23 * other.e01 * other.e23,
            e02: self.e01 * other.e31 * other.e23 + self.e02 * other.s * other.s + self.e02 * other.e31 * other.e31 + self.e12 * other.e03 * other.e31 + self.e03 * other.e12 * other.e31 + self.e31 * other.e02 * other.e31 + self.e23 * other.e01 * other.e31,
            e12: self.e01 * other.e03 * other.e23 + self.e02 * other.e03 * other.e31 + self.e12 * other.s * other.s + self.e12 * other.e03 * other.e03 + self.e03 * other.e12 * other.e03 + self.e31 * other.e02 * other.e03 + self.e23 * other.e01 * other.e03,
            e03: self.e01 * other.e12 * other.e23 + self.e02 * other.e12 * other.e31 + self.e12 * other.e12 * other.e03 + self.e03 * other.s * other.s + self.e03 * other.e12 * other.e12 + self.e31 * other.e02 * other.e12 + self.e23 * other.e01 * other.e12,
            e31: self.e01 * other.e02 * other.e23 + self.e02 * other.e02 * other.e31 + self.e12 * other.e02 * other.e03 + self.e03 * other.e02 * other.e12 + self.e31 * other.s * other.s + self.e31 * other.e02 * other.e02 + self.e23 * other.e01 * other.e02,
            e23: self.e01 * other.e01 * other.e23 + self.e02 * other.e01 * other.e31 + self.e12 * other.e01 * other.e03 + self.e03 * other.e01 * other.e12 + self.e31 * other.e01 * other.e02 + self.e23 * other.s * other.s + self.e23 * other.e01 * other.e01,
            e0123: self.e01 * other.s * other.e23 + self.e02 * other.s * other.e31 + self.e12 * other.s * other.e03 + self.e03 * other.s * other.e12 + self.e31 * other.s * other.e02 + self.e23 * other.s * other.e01,
        }
    }
}

impl Reject<Multivector> for Bivector {
    type Output = EvenMultivector;
    fn reject(self, other: Multivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e01 * other.s * other.e01 - self.e01 * other.e2 * other.e021 + self.e01 * other.e3 * other.e013 + self.e01 * other.e23 * other.e0123 + self.e02 * other.s * other.e02 + self.e02 * other.e1 * other.e021 - self.e02 * other.e3 * other.e032 + self.e02 * other.e31 * other.e0123 + self.e12 * other.s * other.e12 - self.e12 * other.e0 * other.e021 + self.e12 * other.e3 * other.e123 + self.e12 * other.e03 * other.e0123 + self.e03 * other.s * other.e03 - self.e03 * other.e1 * other.e013 + self.e03 * other.e2 * other.e032 + self.e03 * other.e12 * other.e0123 + self.e31 * other.s * other.e31 - self.e31 * other.e0 * other.e013 + self.e31 * other.e2 * other.e123 + self.e31 * other.e02 * other.e0123 + self.e23 * other.s * other.e23 - self.e23 * other.e0 * other.e032 + self.e23 * other.e1 * other.e123 + self.e23 * other.e01 * other.e0123,
            e01: self.e01 * other.s * other.s + self.e01 * other.e2 * other.e2 + self.e01 * other.e3 * other.e3 + self.e01 * other.e23 * other.e23 - self.e02 * other.e1 * other.e2 + self.e02 * other.e31 * other.e23 + self.e12 * other.e0 * other.e2 + self.e12 * other.e03 * other.e23 - self.e03 * other.e1 * other.e3 + self.e03 * other.e12 * other.e23 - self.e31 * other.e0 * other.e3 + self.e31 * other.e02 * other.e23 + self.e23 * other.e01 * other.e23,
            e02: -self.e01 * other.e1 * other.e2 + self.e01 * other.e31 * other.e23 + self.e02 * other.s * other.s + self.e02 * other.e1 * other.e1 + self.e02 * other.e3 * other.e3 + self.e02 * other.e31 * other.e31 - self.e12 * other.e0 * other.e1 + self.e12 * other.e03 * other.e31 - self.e03 * other.e2 * other.e3 + self.e03 * other.e12 * other.e31 + self.e31 * other.e02 * other.e31 + self.e23 * other.e0 * other.e3 + self.e23 * other.e01 * other.e31,
            e12: self.e01 * other.e0 * other.e2 + self.e01 * other.e03 * other.e23 - self.e02 * other.e0 * other.e1 + self.e02 * other.e03 * other.e31 + self.e12 * other.s * other.s + self.e12 * other.e0 * other.e0 + self.e12 * other.e3 * other.e3 + self.e12 * other.e03 * other.e03 + self.e03 * other.e12 * other.e03 + self.e31 * other.e2 * other.e3 + self.e31 * other.e02 * other.e03 + self.e23 * other.e1 * other.e3 + self.e23 * other.e01 * other.e03,
            e03: -self.e01 * other.e1 * other.e3 + self.e01 * other.e12 * other.e23 - self.e02 * other.e2 * other.e3 + self.e02 * other.e12 * other.e31 + self.e12 * other.e12 * other.e03 + self.e03 * other.s * other.s + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 + self.e03 * other.e12 * other.e12 + self.e31 * other.e0 * other.e1 + self.e31 * other.e02 * other.e12 - self.e23 * other.e0 * other.e2 + self.e23 * other.e01 * other.e12,
            e31: -self.e01 * other.e0 * other.e3 + self.e01 * other.e02 * other.e23 + self.e02 * other.e02 * other.e31 + self.e12 * other.e2 * other.e3 + self.e12 * other.e02 * other.e03 + self.e03 * other.e0 * other.e1 + self.e03 * other.e02 * other.e12 + self.e31 * other.s * other.s + self.e31 * other.e0 * other.e0 + self.e31 * other.e2 * other.e2 + self.e31 * other.e02 * other.e02 + self.e23 * other.e1 * other.e2 + self.e23 * other.e01 * other.e02,
            e23: self.e01 * other.e01 * other.e23 + self.e02 * other.e0 * other.e3 + self.e02 * other.e01 * other.e31 + self.e12 * other.e1 * other.e3 + self.e12 * other.e01 * other.e03 - self.e03 * other.e0 * other.e2 + self.e03 * other.e01 * other.e12 + self.e31 * other.e1 * other.e2 + self.e31 * other.e01 * other.e02 + self.e23 * other.s * other.s + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1 + self.e23 * other.e01 * other.e01,
            e0123: self.e01 * other.s * other.e23 + self.e02 * other.s * other.e31 + self.e12 * other.s * other.e03 + self.e03 * other.s * other.e12 + self.e31 * other.s * other.e02 + self.e23 * other.s * other.e01,
        }
    }
}

impl Reject<Scalar> for Trivector {
    type Output = Trivector;
    fn reject(self, other: Scalar) -> Trivector {
        Trivector {
            e021: self.e021 * other.s * other.s,
            e013: self.e013 * other.s * other.s,
            e032: self.e032 * other.s * other.s,
            e123: self.e123 * other.s * other.s,
        }
    }
}

impl Reject<Vector> for Trivector {
    type Output = Trivector;
    fn reject(self, other: Vector) -> Trivector {
        Trivector {
            e021: self.e021 * other.e3 * other.e3 + self.e013 * other.e2 * other.e3 + self.e032 * other.e1 * other.e3 + self.e123 * other.e0 * other.e3,
            e013: self.e021 * other.e2 * other.e3 + self.e013 * other.e2 * other.e2 + self.e032 * other.e1 * other.e2 + self.e123 * other.e0 * other.e2,
            e032: self.e021 * other.e1 * other.e3 + self.e013 * other.e1 * other.e2 + self.e032 * other.e1 * other.e1 + self.e123 * other.e0 * other.e1,
            e123: self.e021 * other.e0 * other.e3 + self.e013 * other.e0 * other.e2 + self.e032 * other.e0 * other.e1 + self.e123 * other.e0 * other.e0,
        }
    }
}

impl Reject<Bivector> for Trivector {
    type Output = Null;
    fn reject(self, other: Bivector) -> Null {
        Null
    }
}

impl Reject<Trivector> for Trivector {
    type Output = Null;
    fn reject(self, other: Trivector) -> Null {
        Null
    }
}

impl Reject<FourVector> for Trivector {
    type Output = Null;
    fn reject(self, other: FourVector) -> Null {
        Null
    }
}

impl Reject<Null> for Trivector {
    type Output = OddMultivector;
    fn reject(self, other: Null) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Reject<OddMultivector> for Trivector {
    type Output = OddMultivector;
    fn reject(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e021 * other.e3 * other.e123 - self.e013 * other.e2 * other.e123 - self.e032 * other.e1 * other.e123 - self.e123 * other.e0 * other.e123,
            e1: -self.e021 * other.e3 * other.e032 - self.e013 * other.e2 * other.e032 - self.e032 * other.e1 * other.e032 - self.e123 * other.e0 * other.e032,
            e2: -self.e021 * other.e3 * other.e013 - self.e013 * other.e2 * other.e013 - self.e032 * other.e1 * other.e013 - self.e123 * other.e0 * other.e013,
            e021: self.e021 * other.e3 * other.e3 + self.e013 * other.e2 * other.e3 + self.e032 * other.e1 * other.e3 + self.e123 * other.e0 * other.e3,
            e3: -self.e021 * other.e021 * other.e3 - self.e013 * other.e2 * other.e021 - self.e032 * other.e1 * other.e021 - self.e123 * other.e0 * other.e021,
            e013: self.e021 * other.e2 * other.e3 + self.e013 * other.e2 * other.e2 + self.e032 * other.e1 * other.e2 + self.e123 * other.e0 * other.e2,
            e032: self.e021 * other.e1 * other.e3 + self.e013 * other.e1 * other.e2 + self.e032 * other.e1 * other.e1 + self.e123 * other.e0 * other.e1,
            e123: self.e021 * other.e0 * other.e3 + self.e013 * other.e0 * other.e2 + self.e032 * other.e0 * other.e1 + self.e123 * other.e0 * other.e0,
        }
    }
}

impl Reject<EvenMultivector> for Trivector {
    type Output = OddMultivector;
    fn reject(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e021 * other.s * other.e12 - self.e013 * other.s * other.e31 - self.e032 * other.s * other.e23,
            e1: self.e021 * other.s * other.e02 - self.e013 * other.s * other.e03 + self.e123 * other.s * other.e23,
            e2: -self.e021 * other.s * other.e01 + self.e032 * other.s * other.e03 + self.e123 * other.s * other.e31,
            e021: self.e021 * other.s * other.s,
            e3: self.e013 * other.s * other.e01 - self.e032 * other.s * other.e02 + self.e123 * other.s * other.e12,
            e013: self.e013 * other.s * other.s,
            e032: self.e032 * other.s * other.s,
            e123: self.e123 * other.s * other.s,
        }
    }
}

impl Reject<Multivector> for Trivector {
    type Output = OddMultivector;
    fn reject(self, other: Multivector) -> OddMultivector {
        OddMultivector {
            e0: -self.e021 * other.s * other.e12 - self.e021 * other.e3 * other.e123 - self.e013 * other.s * other.e31 - self.e013 * other.e2 * other.e123 - self.e032 * other.s * other.e23 - self.e032 * other.e1 * other.e123 - self.e123 * other.e0 * other.e123,
            e1: self.e021 * other.s * other.e02 - self.e021 * other.e3 * other.e032 - self.e013 * other.s * other.e03 - self.e013 * other.e2 * other.e032 - self.e032 * other.e1 * other.e032 + self.e123 * other.s * other.e23 - self.e123 * other.e0 * other.e032,
            e2: -self.e021 * other.s * other.e01 - self.e021 * other.e3 * other.e013 - self.e013 * other.e2 * other.e013 + self.e032 * other.s * other.e03 - self.e032 * other.e1 * other.e013 + self.e123 * other.s * other.e31 - self.e123 * other.e0 * other.e013,
            e021: self.e021 * other.s * other.s + self.e021 * other.e3 * other.e3 + self.e013 * other.e2 * other.e3 + self.e032 * other.e1 * other.e3 + self.e123 * other.e0 * other.e3,
            e3: -self.e021 * other.e021 * other.e3 + self.e013 * other.s * other.e01 - self.e013 * other.e2 * other.e021 - self.e032 * other.s * other.e02 - self.e032 * other.e1 * other.e021 + self.e123 * other.s * other.e12 - self.e123 * other.e0 * other.e021,
            e013: self.e021 * other.e2 * other.e3 + self.e013 * other.s * other.s + self.e013 * other.e2 * other.e2 + self.e032 * other.e1 * other.e2 + self.e123 * other.e0 * other.e2,
            e032: self.e021 * other.e1 * other.e3 + self.e013 * other.e1 * other.e2 + self.e032 * other.s * other.s + self.e032 * other.e1 * other.e1 + self.e123 * other.e0 * other.e1,
            e123: self.e021 * other.e0 * other.e3 + self.e013 * other.e0 * other.e2 + self.e032 * other.e0 * other.e1 + self.e123 * other.s * other.s + self.e123 * other.e0 * other.e0,
        }
    }
}

impl Reject<Scalar> for FourVector {
    type Output = FourVector;
    fn reject(self, other: Scalar) -> FourVector {
        FourVector {
            e0123: self.e0123 * other.s * other.s,
        }
    }
}

impl Reject<Vector> for FourVector {
    type Output = Null;
    fn reject(self, other: Vector) -> Null {
        Null
    }
}

impl Reject<Bivector> for FourVector {
    type Output = Null;
    fn reject(self, other: Bivector) -> Null {
        Null
    }
}

impl Reject<Trivector> for FourVector {
    type Output = Null;
    fn reject(self, other: Trivector) -> Null {
        Null
    }
}

impl Reject<FourVector> for FourVector {
    type Output = Null;
    fn reject(self, other: FourVector) -> Null {
        Null
    }
}

impl Reject<Null> for FourVector {
    type Output = EvenMultivector;
    fn reject(self, other: Null) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Reject<OddMultivector> for FourVector {
    type Output = EvenMultivector;
    fn reject(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Reject<EvenMultivector> for FourVector {
    type Output = EvenMultivector;
    fn reject(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e0123 * other.s * other.e0123,
            e01: self.e0123 * other.s * other.e23,
            e02: self.e0123 * other.s * other.e31,
            e12: self.e0123 * other.s * other.e03,
            e03: self.e0123 * other.s * other.e12,
            e31: self.e0123 * other.s * other.e02,
            e23: self.e0123 * other.s * other.e01,
            e0123: self.e0123 * other.s * other.s,
        }
    }
}

impl Reject<Multivector> for FourVector {
    type Output = EvenMultivector;
    fn reject(self, other: Multivector) -> EvenMultivector {
        EvenMultivector {
            s: self.e0123 * other.s * other.e0123,
            e01: self.e0123 * other.s * other.e23,
            e02: self.e0123 * other.s * other.e31,
            e12: self.e0123 * other.s * other.e03,
            e03: self.e0123 * other.s * other.e12,
            e31: self.e0123 * other.s * other.e02,
            e23: self.e0123 * other.s * other.e01,
            e0123: self.e0123 * other.s * other.s,
        }
    }
}

impl Reject<Scalar> for Null {
    type Output = Null;
    fn reject(self, other: Scalar) -> Null {
        Null
    }
}

impl Reject<Vector> for Null {
    type Output = Null;
    fn reject(self, other: Vector) -> Null {
        Null
    }
}

impl Reject<Bivector> for Null {
    type Output = Null;
    fn reject(self, other: Bivector) -> Null {
        Null
    }
}

impl Reject<Trivector> for Null {
    type Output = Null;
    fn reject(self, other: Trivector) -> Null {
        Null
    }
}

impl Reject<FourVector> for Null {
    type Output = Null;
    fn reject(self, other: FourVector) -> Null {
        Null
    }
}

impl Reject<Null> for Null {
    type Output = Null;
    fn reject(self, other: Null) -> Null {
        Null
    }
}

impl Reject<OddMultivector> for Null {
    type Output = Null;
    fn reject(self, other: OddMultivector) -> Null {
        Null
    }
}

impl Reject<EvenMultivector> for Null {
    type Output = Null;
    fn reject(self, other: EvenMultivector) -> Null {
        Null
    }
}

impl Reject<Multivector> for Null {
    type Output = Null;
    fn reject(self, other: Multivector) -> Null {
        Null
    }
}

impl Reject<Scalar> for OddMultivector {
    type Output = OddMultivector;
    fn reject(self, other: Scalar) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s * other.s,
            e1: self.e1 * other.s * other.s,
            e2: self.e2 * other.s * other.s,
            e021: self.e021 * other.s * other.s,
            e3: self.e3 * other.s * other.s,
            e013: self.e013 * other.s * other.s,
            e032: self.e032 * other.s * other.s,
            e123: self.e123 * other.s * other.s,
        }
    }
}

impl Reject<Vector> for OddMultivector {
    type Output = OddMultivector;
    fn reject(self, other: Vector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.e1 * other.e1 + self.e0 * other.e2 * other.e2 + self.e0 * other.e3 * other.e3 - self.e1 * other.e0 * other.e1 - self.e2 * other.e0 * other.e2 - self.e3 * other.e0 * other.e3,
            e1: -self.e0 * other.e0 * other.e1 + self.e1 * other.e0 * other.e0 + self.e1 * other.e2 * other.e2 + self.e1 * other.e3 * other.e3 - self.e2 * other.e1 * other.e2 - self.e3 * other.e1 * other.e3,
            e2: -self.e0 * other.e0 * other.e2 - self.e1 * other.e1 * other.e2 + self.e2 * other.e0 * other.e0 + self.e2 * other.e1 * other.e1 + self.e2 * other.e3 * other.e3 - self.e3 * other.e2 * other.e3,
            e021: self.e021 * other.e3 * other.e3 + self.e013 * other.e2 * other.e3 + self.e032 * other.e1 * other.e3 + self.e123 * other.e0 * other.e3,
            e3: -self.e0 * other.e0 * other.e3 - self.e1 * other.e1 * other.e3 - self.e2 * other.e2 * other.e3 + self.e3 * other.e0 * other.e0 + self.e3 * other.e1 * other.e1 + self.e3 * other.e2 * other.e2,
            e013: self.e021 * other.e2 * other.e3 + self.e013 * other.e2 * other.e2 + self.e032 * other.e1 * other.e2 + self.e123 * other.e0 * other.e2,
            e032: self.e021 * other.e1 * other.e3 + self.e013 * other.e1 * other.e2 + self.e032 * other.e1 * other.e1 + self.e123 * other.e0 * other.e1,
            e123: self.e021 * other.e0 * other.e3 + self.e013 * other.e0 * other.e2 + self.e032 * other.e0 * other.e1 + self.e123 * other.e0 * other.e0,
        }
    }
}

impl Reject<Bivector> for OddMultivector {
    type Output = OddMultivector;
    fn reject(self, other: Bivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.e12 * other.e12 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 - self.e1 * other.e02 * other.e12 + self.e1 * other.e03 * other.e31 + self.e2 * other.e01 * other.e12 - self.e2 * other.e03 * other.e23 - self.e3 * other.e01 * other.e31 + self.e3 * other.e02 * other.e23,
            e1: -self.e0 * other.e02 * other.e12 + self.e0 * other.e03 * other.e31 + self.e1 * other.e02 * other.e02 + self.e1 * other.e03 * other.e03 + self.e1 * other.e23 * other.e23 - self.e2 * other.e01 * other.e02 + self.e2 * other.e31 * other.e23 - self.e3 * other.e01 * other.e03 + self.e3 * other.e12 * other.e23,
            e2: self.e0 * other.e01 * other.e12 - self.e0 * other.e03 * other.e23 - self.e1 * other.e01 * other.e02 + self.e1 * other.e31 * other.e23 + self.e2 * other.e01 * other.e01 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e3 * other.e02 * other.e03 + self.e3 * other.e12 * other.e31,
            e021: 0.0,
            e3: -self.e0 * other.e01 * other.e31 + self.e0 * other.e02 * other.e23 - self.e1 * other.e01 * other.e03 + self.e1 * other.e12 * other.e23 - self.e2 * other.e02 * other.e03 + self.e2 * other.e12 * other.e31 + self.e3 * other.e01 * other.e01 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Reject<Trivector> for OddMultivector {
    type Output = OddMultivector;
    fn reject(self, other: Trivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.e123 * other.e123 + self.e1 * other.e032 * other.e123 + self.e2 * other.e013 * other.e123 + self.e3 * other.e021 * other.e123,
            e1: self.e0 * other.e032 * other.e123 + self.e1 * other.e032 * other.e032 + self.e2 * other.e013 * other.e032 + self.e3 * other.e021 * other.e032,
            e2: self.e0 * other.e013 * other.e123 + self.e1 * other.e013 * other.e032 + self.e2 * other.e013 * other.e013 + self.e3 * other.e021 * other.e013,
            e021: 0.0,
            e3: self.e0 * other.e021 * other.e123 + self.e1 * other.e021 * other.e032 + self.e2 * other.e021 * other.e013 + self.e3 * other.e021 * other.e021,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Reject<FourVector> for OddMultivector {
    type Output = OddMultivector;
    fn reject(self, other: FourVector) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Reject<Null> for OddMultivector {
    type Output = OddMultivector;
    fn reject(self, other: Null) -> OddMultivector {
        OddMultivector {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
            e021: 0.0,
            e3: 0.0,
            e013: 0.0,
            e032: 0.0,
            e123: 0.0,
        }
    }
}

impl Reject<OddMultivector> for OddMultivector {
    type Output = OddMultivector;
    fn reject(self, other: OddMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.e1 * other.e1 + self.e0 * other.e2 * other.e2 + self.e0 * other.e3 * other.e3 + self.e0 * other.e123 * other.e123 - self.e1 * other.e0 * other.e1 + self.e1 * other.e032 * other.e123 - self.e2 * other.e0 * other.e2 + self.e2 * other.e013 * other.e123 - self.e021 * other.e3 * other.e123 - self.e3 * other.e0 * other.e3 + self.e3 * other.e021 * other.e123 - self.e013 * other.e2 * other.e123 - self.e032 * other.e1 * other.e123 - self.e123 * other.e0 * other.e123,
            e1: -self.e0 * other.e0 * other.e1 + self.e0 * other.e032 * other.e123 + self.e1 * other.e0 * other.e0 + self.e1 * other.e2 * other.e2 + self.e1 * other.e3 * other.e3 + self.e1 * other.e032 * other.e032 - self.e2 * other.e1 * other.e2 + self.e2 * other.e013 * other.e032 - self.e021 * other.e3 * other.e032 - self.e3 * other.e1 * other.e3 + self.e3 * other.e021 * other.e032 - self.e013 * other.e2 * other.e032 - self.e032 * other.e1 * other.e032 - self.e123 * other.e0 * other.e032,
            e2: -self.e0 * other.e0 * other.e2 + self.e0 * other.e013 * other.e123 - self.e1 * other.e1 * other.e2 + self.e1 * other.e013 * other.e032 + self.e2 * other.e0 * other.e0 + self.e2 * other.e1 * other.e1 + self.e2 * other.e3 * other.e3 + self.e2 * other.e013 * other.e013 - self.e021 * other.e3 * other.e013 - self.e3 * other.e2 * other.e3 + self.e3 * other.e021 * other.e013 - self.e013 * other.e2 * other.e013 - self.e032 * other.e1 * other.e013 - self.e123 * other.e0 * other.e013,
            e021: -self.e0 * other.e3 * other.e123 - self.e1 * other.e3 * other.e032 - self.e2 * other.e3 * other.e013 + self.e021 * other.e3 * other.e3 - self.e3 * other.e021 * other.e3 + self.e013 * other.e2 * other.e3 + self.e032 * other.e1 * other.e3 + self.e123 * other.e0 * other.e3,
            e3: -self.e0 * other.e0 * other.e3 + self.e0 * other.e021 * other.e123 - self.e1 * other.e1 * other.e3 + self.e1 * other.e021 * other.e032 - self.e2 * other.e2 * other.e3 + self.e2 * other.e021 * other.e013 - self.e021 * other.e021 * other.e3 + self.e3 * other.e0 * other.e0 + self.e3 * other.e1 * other.e1 + self.e3 * other.e2 * other.e2 + self.e3 * other.e021 * other.e021 - self.e013 * other.e2 * other.e021 - self.e032 * other.e1 * other.e021 - self.e123 * other.e0 * other.e021,
            e013: -self.e0 * other.e2 * other.e123 - self.e1 * other.e2 * other.e032 - self.e2 * other.e2 * other.e013 + self.e021 * other.e2 * other.e3 - self.e3 * other.e2 * other.e021 + self.e013 * other.e2 * other.e2 + self.e032 * other.e1 * other.e2 + self.e123 * other.e0 * other.e2,
            e032: -self.e0 * other.e1 * other.e123 - self.e1 * other.e1 * other.e032 - self.e2 * other.e1 * other.e013 + self.e021 * other.e1 * other.e3 - self.e3 * other.e1 * other.e021 + self.e013 * other.e1 * other.e2 + self.e032 * other.e1 * other.e1 + self.e123 * other.e0 * other.e1,
            e123: -self.e0 * other.e0 * other.e123 - self.e1 * other.e0 * other.e032 - self.e2 * other.e0 * other.e013 + self.e021 * other.e0 * other.e3 - self.e3 * other.e0 * other.e021 + self.e013 * other.e0 * other.e2 + self.e032 * other.e0 * other.e1 + self.e123 * other.e0 * other.e0,
        }
    }
}

impl Reject<EvenMultivector> for OddMultivector {
    type Output = OddMultivector;
    fn reject(self, other: EvenMultivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s * other.s + self.e0 * other.e12 * other.e12 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 - self.e1 * other.e02 * other.e12 + self.e1 * other.e03 * other.e31 + self.e2 * other.e01 * other.e12 - self.e2 * other.e03 * other.e23 - self.e021 * other.s * other.e12 - self.e3 * other.e01 * other.e31 + self.e3 * other.e02 * other.e23 - self.e013 * other.s * other.e31 - self.e032 * other.s * other.e23,
            e1: -self.e0 * other.e02 * other.e12 + self.e0 * other.e03 * other.e31 + self.e1 * other.s * other.s + self.e1 * other.e02 * other.e02 + self.e1 * other.e03 * other.e03 + self.e1 * other.e23 * other.e23 - self.e2 * other.e01 * other.e02 + self.e2 * other.e31 * other.e23 + self.e021 * other.s * other.e02 - self.e3 * other.e01 * other.e03 + self.e3 * other.e12 * other.e23 - self.e013 * other.s * other.e03 + self.e123 * other.s * other.e23,
            e2: self.e0 * other.e01 * other.e12 - self.e0 * other.e03 * other.e23 - self.e1 * other.e01 * other.e02 + self.e1 * other.e31 * other.e23 + self.e2 * other.s * other.s + self.e2 * other.e01 * other.e01 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e021 * other.s * other.e01 - self.e3 * other.e02 * other.e03 + self.e3 * other.e12 * other.e31 + self.e032 * other.s * other.e03 + self.e123 * other.s * other.e31,
            e021: -self.e0 * other.s * other.e12 + self.e1 * other.s * other.e02 - self.e2 * other.s * other.e01 + self.e021 * other.s * other.s,
            e3: -self.e0 * other.e01 * other.e31 + self.e0 * other.e02 * other.e23 - self.e1 * other.e01 * other.e03 + self.e1 * other.e12 * other.e23 - self.e2 * other.e02 * other.e03 + self.e2 * other.e12 * other.e31 + self.e3 * other.s * other.s + self.e3 * other.e01 * other.e01 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12 + self.e013 * other.s * other.e01 - self.e032 * other.s * other.e02 + self.e123 * other.s * other.e12,
            e013: -self.e0 * other.s * other.e31 - self.e1 * other.s * other.e03 + self.e3 * other.s * other.e01 + self.e013 * other.s * other.s,
            e032: -self.e0 * other.s * other.e23 + self.e2 * other.s * other.e03 - self.e3 * other.s * other.e02 + self.e032 * other.s * other.s,
            e123: self.e1 * other.s * other.e23 + self.e2 * other.s * other.e31 + self.e3 * other.s * other.e12 + self.e123 * other.s * other.s,
        }
    }
}

impl Reject<Multivector> for OddMultivector {
    type Output = OddMultivector;
    fn reject(self, other: Multivector) -> OddMultivector {
        OddMultivector {
            e0: self.e0 * other.s * other.s + self.e0 * other.e1 * other.e1 + self.e0 * other.e2 * other.e2 + self.e0 * other.e12 * other.e12 + self.e0 * other.e3 * other.e3 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 + self.e0 * other.e123 * other.e123 - self.e1 * other.e0 * other.e1 - self.e1 * other.e02 * other.e12 + self.e1 * other.e03 * other.e31 + self.e1 * other.e032 * other.e123 - self.e2 * other.e0 * other.e2 + self.e2 * other.e01 * other.e12 - self.e2 * other.e03 * other.e23 + self.e2 * other.e013 * other.e123 - self.e021 * other.s * other.e12 - self.e021 * other.e3 * other.e123 - self.e3 * other.e0 * other.e3 - self.e3 * other.e01 * other.e31 + self.e3 * other.e02 * other.e23 + self.e3 * other.e021 * other.e123 - self.e013 * other.s * other.e31 - self.e013 * other.e2 * other.e123 - self.e032 * other.s * other.e23 - self.e032 * other.e1 * other.e123 - self.e123 * other.e0 * other.e123,
            e1: -self.e0 * other.e0 * other.e1 - self.e0 * other.e02 * other.e12 + self.e0 * other.e03 * other.e31 + self.e0 * other.e032 * other.e123 + self.e1 * other.s * other.s + self.e1 * other.e0 * other.e0 + self.e1 * other.e2 * other.e2 + self.e1 * other.e02 * other.e02 + self.e1 * other.e3 * other.e3 + self.e1 * other.e03 * other.e03 + self.e1 * other.e23 * other.e23 + self.e1 * other.e032 * other.e032 - self.e2 * other.e1 * other.e2 - self.e2 * other.e01 * other.e02 + self.e2 * other.e31 * other.e23 + self.e2 * other.e013 * other.e032 + self.e021 * other.s * other.e02 - self.e021 * other.e3 * other.e032 - self.e3 * other.e1 * other.e3 - self.e3 * other.e01 * other.e03 + self.e3 * other.e12 * other.e23 + self.e3 * other.e021 * other.e032 - self.e013 * other.s * other.e03 - self.e013 * other.e2 * other.e032 - self.e032 * other.e1 * other.e032 + self.e123 * other.s * other.e23 - self.e123 * other.e0 * other.e032,
            e2: -self.e0 * other.e0 * other.e2 + self.e0 * other.e01 * other.e12 - self.e0 * other.e03 * other.e23 + self.e0 * other.e013 * other.e123 - self.e1 * other.e1 * other.e2 - self.e1 * other.e01 * other.e02 + self.e1 * other.e31 * other.e23 + self.e1 * other.e013 * other.e032 + self.e2 * other.s * other.s + self.e2 * other.e0 * other.e0 + self.e2 * other.e1 * other.e1 + self.e2 * other.e01 * other.e01 + self.e2 * other.e3 * other.e3 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 + self.e2 * other.e013 * other.e013 - self.e021 * other.s * other.e01 - self.e021 * other.e3 * other.e013 - self.e3 * other.e2 * other.e3 - self.e3 * other.e02 * other.e03 + self.e3 * other.e12 * other.e31 + self.e3 * other.e021 * other.e013 - self.e013 * other.e2 * other.e013 + self.e032 * other.s * other.e03 - self.e032 * other.e1 * other.e013 + self.e123 * other.s * other.e31 - self.e123 * other.e0 * other.e013,
            e021: -self.e0 * other.s * other.e12 - self.e0 * other.e3 * other.e123 + self.e1 * other.s * other.e02 - self.e1 * other.e3 * other.e032 - self.e2 * other.s * other.e01 - self.e2 * other.e3 * other.e013 + self.e021 * other.s * other.s + self.e021 * other.e3 * other.e3 - self.e3 * other.e021 * other.e3 + self.e013 * other.e2 * other.e3 + self.e032 * other.e1 * other.e3 + self.e123 * other.e0 * other.e3,
            e3: -self.e0 * other.e0 * other.e3 - self.e0 * other.e01 * other.e31 + self.e0 * other.e02 * other.e23 + self.e0 * other.e021 * other.e123 - self.e1 * other.e1 * other.e3 - self.e1 * other.e01 * other.e03 + self.e1 * other.e12 * other.e23 + self.e1 * other.e021 * other.e032 - self.e2 * other.e2 * other.e3 - self.e2 * other.e02 * other.e03 + self.e2 * other.e12 * other.e31 + self.e2 * other.e021 * other.e013 - self.e021 * other.e021 * other.e3 + self.e3 * other.s * other.s + self.e3 * other.e0 * other.e0 + self.e3 * other.e1 * other.e1 + self.e3 * other.e01 * other.e01 + self.e3 * other.e2 * other.e2 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12 + self.e3 * other.e021 * other.e021 + self.e013 * other.s * other.e01 - self.e013 * other.e2 * other.e021 - self.e032 * other.s * other.e02 - self.e032 * other.e1 * other.e021 + self.e123 * other.s * other.e12 - self.e123 * other.e0 * other.e021,
            e013: -self.e0 * other.s * other.e31 - self.e0 * other.e2 * other.e123 - self.e1 * other.s * other.e03 - self.e1 * other.e2 * other.e032 - self.e2 * other.e2 * other.e013 + self.e021 * other.e2 * other.e3 + self.e3 * other.s * other.e01 - self.e3 * other.e2 * other.e021 + self.e013 * other.s * other.s + self.e013 * other.e2 * other.e2 + self.e032 * other.e1 * other.e2 + self.e123 * other.e0 * other.e2,
            e032: -self.e0 * other.s * other.e23 - self.e0 * other.e1 * other.e123 - self.e1 * other.e1 * other.e032 + self.e2 * other.s * other.e03 - self.e2 * other.e1 * other.e013 + self.e021 * other.e1 * other.e3 - self.e3 * other.s * other.e02 - self.e3 * other.e1 * other.e021 + self.e013 * other.e1 * other.e2 + self.e032 * other.s * other.s + self.e032 * other.e1 * other.e1 + self.e123 * other.e0 * other.e1,
            e123: -self.e0 * other.e0 * other.e123 + self.e1 * other.s * other.e23 - self.e1 * other.e0 * other.e032 + self.e2 * other.s * other.e31 - self.e2 * other.e0 * other.e013 + self.e021 * other.e0 * other.e3 + self.e3 * other.s * other.e12 - self.e3 * other.e0 * other.e021 + self.e013 * other.e0 * other.e2 + self.e032 * other.e0 * other.e1 + self.e123 * other.s * other.s + self.e123 * other.e0 * other.e0,
        }
    }
}

impl Reject<Scalar> for EvenMultivector {
    type Output = EvenMultivector;
    fn reject(self, other: Scalar) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s * other.s,
            e01: self.e01 * other.s * other.s,
            e02: self.e02 * other.s * other.s,
            e12: self.e12 * other.s * other.s,
            e03: self.e03 * other.s * other.s,
            e31: self.e31 * other.s * other.s,
            e23: self.e23 * other.s * other.s,
            e0123: self.e0123 * other.s * other.s,
        }
    }
}

impl Reject<Vector> for EvenMultivector {
    type Output = EvenMultivector;
    fn reject(self, other: Vector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e3 * other.e3,
            e01: self.e01 * other.e2 * other.e2 + self.e01 * other.e3 * other.e3 - self.e02 * other.e1 * other.e2 + self.e12 * other.e0 * other.e2 - self.e03 * other.e1 * other.e3 - self.e31 * other.e0 * other.e3,
            e02: -self.e01 * other.e1 * other.e2 + self.e02 * other.e1 * other.e1 + self.e02 * other.e3 * other.e3 - self.e12 * other.e0 * other.e1 - self.e03 * other.e2 * other.e3 + self.e23 * other.e0 * other.e3,
            e12: self.e01 * other.e0 * other.e2 - self.e02 * other.e0 * other.e1 + self.e12 * other.e0 * other.e0 + self.e12 * other.e3 * other.e3 + self.e31 * other.e2 * other.e3 + self.e23 * other.e1 * other.e3,
            e03: -self.e01 * other.e1 * other.e3 - self.e02 * other.e2 * other.e3 + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 + self.e31 * other.e0 * other.e1 - self.e23 * other.e0 * other.e2,
            e31: -self.e01 * other.e0 * other.e3 + self.e12 * other.e2 * other.e3 + self.e03 * other.e0 * other.e1 + self.e31 * other.e0 * other.e0 + self.e31 * other.e2 * other.e2 + self.e23 * other.e1 * other.e2,
            e23: self.e02 * other.e0 * other.e3 + self.e12 * other.e1 * other.e3 - self.e03 * other.e0 * other.e2 + self.e31 * other.e1 * other.e2 + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1,
            e0123: 0.0,
        }
    }
}

impl Reject<Bivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn reject(self, other: Bivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23,
            e01: self.e01 * other.e23 * other.e23 + self.e02 * other.e31 * other.e23 + self.e12 * other.e03 * other.e23 + self.e03 * other.e12 * other.e23 + self.e31 * other.e02 * other.e23 + self.e23 * other.e01 * other.e23,
            e02: self.e01 * other.e31 * other.e23 + self.e02 * other.e31 * other.e31 + self.e12 * other.e03 * other.e31 + self.e03 * other.e12 * other.e31 + self.e31 * other.e02 * other.e31 + self.e23 * other.e01 * other.e31,
            e12: self.e01 * other.e03 * other.e23 + self.e02 * other.e03 * other.e31 + self.e12 * other.e03 * other.e03 + self.e03 * other.e12 * other.e03 + self.e31 * other.e02 * other.e03 + self.e23 * other.e01 * other.e03,
            e03: self.e01 * other.e12 * other.e23 + self.e02 * other.e12 * other.e31 + self.e12 * other.e12 * other.e03 + self.e03 * other.e12 * other.e12 + self.e31 * other.e02 * other.e12 + self.e23 * other.e01 * other.e12,
            e31: self.e01 * other.e02 * other.e23 + self.e02 * other.e02 * other.e31 + self.e12 * other.e02 * other.e03 + self.e03 * other.e02 * other.e12 + self.e31 * other.e02 * other.e02 + self.e23 * other.e01 * other.e02,
            e23: self.e01 * other.e01 * other.e23 + self.e02 * other.e01 * other.e31 + self.e12 * other.e01 * other.e03 + self.e03 * other.e01 * other.e12 + self.e31 * other.e01 * other.e02 + self.e23 * other.e01 * other.e01,
            e0123: 0.0,
        }
    }
}

impl Reject<Trivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn reject(self, other: Trivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e021 * other.e021 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Reject<FourVector> for EvenMultivector {
    type Output = EvenMultivector;
    fn reject(self, other: FourVector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0123 * other.e0123,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Reject<Null> for EvenMultivector {
    type Output = EvenMultivector;
    fn reject(self, other: Null) -> EvenMultivector {
        EvenMultivector {
            s: 0.0,
            e01: 0.0,
            e02: 0.0,
            e12: 0.0,
            e03: 0.0,
            e31: 0.0,
            e23: 0.0,
            e0123: 0.0,
        }
    }
}

impl Reject<OddMultivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn reject(self, other: OddMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 - self.e01 * other.e2 * other.e021 + self.e01 * other.e3 * other.e013 + self.e02 * other.e1 * other.e021 - self.e02 * other.e3 * other.e032 - self.e12 * other.e0 * other.e021 + self.e12 * other.e3 * other.e123 - self.e03 * other.e1 * other.e013 + self.e03 * other.e2 * other.e032 - self.e31 * other.e0 * other.e013 + self.e31 * other.e2 * other.e123 - self.e23 * other.e0 * other.e032 + self.e23 * other.e1 * other.e123,
            e01: -self.s * other.e2 * other.e021 + self.s * other.e3 * other.e013 + self.e01 * other.e2 * other.e2 + self.e01 * other.e3 * other.e3 - self.e02 * other.e1 * other.e2 + self.e12 * other.e0 * other.e2 - self.e03 * other.e1 * other.e3 - self.e31 * other.e0 * other.e3,
            e02: self.s * other.e1 * other.e021 - self.s * other.e3 * other.e032 - self.e01 * other.e1 * other.e2 + self.e02 * other.e1 * other.e1 + self.e02 * other.e3 * other.e3 - self.e12 * other.e0 * other.e1 - self.e03 * other.e2 * other.e3 + self.e23 * other.e0 * other.e3,
            e12: -self.s * other.e0 * other.e021 + self.s * other.e3 * other.e123 + self.e01 * other.e0 * other.e2 - self.e02 * other.e0 * other.e1 + self.e12 * other.e0 * other.e0 + self.e12 * other.e3 * other.e3 + self.e31 * other.e2 * other.e3 + self.e23 * other.e1 * other.e3,
            e03: -self.s * other.e1 * other.e013 + self.s * other.e2 * other.e032 - self.e01 * other.e1 * other.e3 - self.e02 * other.e2 * other.e3 + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 + self.e31 * other.e0 * other.e1 - self.e23 * other.e0 * other.e2,
            e31: -self.s * other.e0 * other.e013 + self.s * other.e2 * other.e123 - self.e01 * other.e0 * other.e3 + self.e12 * other.e2 * other.e3 + self.e03 * other.e0 * other.e1 + self.e31 * other.e0 * other.e0 + self.e31 * other.e2 * other.e2 + self.e23 * other.e1 * other.e2,
            e23: -self.s * other.e0 * other.e032 + self.s * other.e1 * other.e123 + self.e02 * other.e0 * other.e3 + self.e12 * other.e1 * other.e3 - self.e03 * other.e0 * other.e2 + self.e31 * other.e1 * other.e2 + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1,
            e0123: 0.0,
        }
    }
}

impl Reject<EvenMultivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn reject(self, other: EvenMultivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s * other.s + self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23 + self.s * other.e0123 * other.e0123 + self.e01 * other.s * other.e01 + self.e01 * other.e23 * other.e0123 + self.e02 * other.s * other.e02 + self.e02 * other.e31 * other.e0123 + self.e12 * other.s * other.e12 + self.e12 * other.e03 * other.e0123 + self.e03 * other.s * other.e03 + self.e03 * other.e12 * other.e0123 + self.e31 * other.s * other.e31 + self.e31 * other.e02 * other.e0123 + self.e23 * other.s * other.e23 + self.e23 * other.e01 * other.e0123 + self.e0123 * other.s * other.e0123,
            e01: self.s * other.s * other.e01 + self.s * other.e23 * other.e0123 + self.e01 * other.s * other.s + self.e01 * other.e23 * other.e23 + self.e02 * other.e31 * other.e23 + self.e12 * other.e03 * other.e23 + self.e03 * other.e12 * other.e23 + self.e31 * other.e02 * other.e23 + self.e23 * other.e01 * other.e23 + self.e0123 * other.s * other.e23,
            e02: self.s * other.s * other.e02 + self.s * other.e31 * other.e0123 + self.e01 * other.e31 * other.e23 + self.e02 * other.s * other.s + self.e02 * other.e31 * other.e31 + self.e12 * other.e03 * other.e31 + self.e03 * other.e12 * other.e31 + self.e31 * other.e02 * other.e31 + self.e23 * other.e01 * other.e31 + self.e0123 * other.s * other.e31,
            e12: self.s * other.s * other.e12 + self.s * other.e03 * other.e0123 + self.e01 * other.e03 * other.e23 + self.e02 * other.e03 * other.e31 + self.e12 * other.s * other.s + self.e12 * other.e03 * other.e03 + self.e03 * other.e12 * other.e03 + self.e31 * other.e02 * other.e03 + self.e23 * other.e01 * other.e03 + self.e0123 * other.s * other.e03,
            e03: self.s * other.s * other.e03 + self.s * other.e12 * other.e0123 + self.e01 * other.e12 * other.e23 + self.e02 * other.e12 * other.e31 + self.e12 * other.e12 * other.e03 + self.e03 * other.s * other.s + self.e03 * other.e12 * other.e12 + self.e31 * other.e02 * other.e12 + self.e23 * other.e01 * other.e12 + self.e0123 * other.s * other.e12,
            e31: self.s * other.s * other.e31 + self.s * other.e02 * other.e0123 + self.e01 * other.e02 * other.e23 + self.e02 * other.e02 * other.e31 + self.e12 * other.e02 * other.e03 + self.e03 * other.e02 * other.e12 + self.e31 * other.s * other.s + self.e31 * other.e02 * other.e02 + self.e23 * other.e01 * other.e02 + self.e0123 * other.s * other.e02,
            e23: self.s * other.s * other.e23 + self.s * other.e01 * other.e0123 + self.e01 * other.e01 * other.e23 + self.e02 * other.e01 * other.e31 + self.e12 * other.e01 * other.e03 + self.e03 * other.e01 * other.e12 + self.e31 * other.e01 * other.e02 + self.e23 * other.s * other.s + self.e23 * other.e01 * other.e01 + self.e0123 * other.s * other.e01,
            e0123: self.s * other.s * other.e0123 + self.e01 * other.s * other.e23 + self.e02 * other.s * other.e31 + self.e12 * other.s * other.e03 + self.e03 * other.s * other.e12 + self.e31 * other.s * other.e02 + self.e23 * other.s * other.e01 + self.e0123 * other.s * other.s,
        }
    }
}

impl Reject<Multivector> for EvenMultivector {
    type Output = EvenMultivector;
    fn reject(self, other: Multivector) -> EvenMultivector {
        EvenMultivector {
            s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01 + self.s * other.e2 * other.e2 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e013 * other.e013 + self.s * other.e23 * other.e23 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 + self.s * other.e0123 * other.e0123 + self.e01 * other.s * other.e01 - self.e01 * other.e2 * other.e021 + self.e01 * other.e3 * other.e013 + self.e01 * other.e23 * other.e0123 + self.e02 * other.s * other.e02 + self.e02 * other.e1 * other.e021 - self.e02 * other.e3 * other.e032 + self.e02 * other.e31 * other.e0123 + self.e12 * other.s * other.e12 - self.e12 * other.e0 * other.e021 + self.e12 * other.e3 * other.e123 + self.e12 * other.e03 * other.e0123 + self.e03 * other.s * other.e03 - self.e03 * other.e1 * other.e013 + self.e03 * other.e2 * other.e032 + self.e03 * other.e12 * other.e0123 + self.e31 * other.s * other.e31 - self.e31 * other.e0 * other.e013 + self.e31 * other.e2 * other.e123 + self.e31 * other.e02 * other.e0123 + self.e23 * other.s * other.e23 - self.e23 * other.e0 * other.e032 + self.e23 * other.e1 * other.e123 + self.e23 * other.e01 * other.e0123 + self.e0123 * other.s * other.e0123,
            e01: self.s * other.s * other.e01 - self.s * other.e2 * other.e021 + self.s * other.e3 * other.e013 + self.s * other.e23 * other.e0123 + self.e01 * other.s * other.s + self.e01 * other.e2 * other.e2 + self.e01 * other.e3 * other.e3 + self.e01 * other.e23 * other.e23 - self.e02 * other.e1 * other.e2 + self.e02 * other.e31 * other.e23 + self.e12 * other.e0 * other.e2 + self.e12 * other.e03 * other.e23 - self.e03 * other.e1 * other.e3 + self.e03 * other.e12 * other.e23 - self.e31 * other.e0 * other.e3 + self.e31 * other.e02 * other.e23 + self.e23 * other.e01 * other.e23 + self.e0123 * other.s * other.e23,
            e02: self.s * other.s * other.e02 + self.s * other.e1 * other.e021 - self.s * other.e3 * other.e032 + self.s * other.e31 * other.e0123 - self.e01 * other.e1 * other.e2 + self.e01 * other.e31 * other.e23 + self.e02 * other.s * other.s + self.e02 * other.e1 * other.e1 + self.e02 * other.e3 * other.e3 + self.e02 * other.e31 * other.e31 - self.e12 * other.e0 * other.e1 + self.e12 * other.e03 * other.e31 - self.e03 * other.e2 * other.e3 + self.e03 * other.e12 * other.e31 + self.e31 * other.e02 * other.e31 + self.e23 * other.e0 * other.e3 + self.e23 * other.e01 * other.e31 + self.e0123 * other.s * other.e31,
            e12: self.s * other.s * other.e12 - self.s * other.e0 * other.e021 + self.s * other.e3 * other.e123 + self.s * other.e03 * other.e0123 + self.e01 * other.e0 * other.e2 + self.e01 * other.e03 * other.e23 - self.e02 * other.e0 * other.e1 + self.e02 * other.e03 * other.e31 + self.e12 * other.s * other.s + self.e12 * other.e0 * other.e0 + self.e12 * other.e3 * other.e3 + self.e12 * other.e03 * other.e03 + self.e03 * other.e12 * other.e03 + self.e31 * other.e2 * other.e3 + self.e31 * other.e02 * other.e03 + self.e23 * other.e1 * other.e3 + self.e23 * other.e01 * other.e03 + self.e0123 * other.s * other.e03,
            e03: self.s * other.s * other.e03 - self.s * other.e1 * other.e013 + self.s * other.e2 * other.e032 + self.s * other.e12 * other.e0123 - self.e01 * other.e1 * other.e3 + self.e01 * other.e12 * other.e23 - self.e02 * other.e2 * other.e3 + self.e02 * other.e12 * other.e31 + self.e12 * other.e12 * other.e03 + self.e03 * other.s * other.s + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 + self.e03 * other.e12 * other.e12 + self.e31 * other.e0 * other.e1 + self.e31 * other.e02 * other.e12 - self.e23 * other.e0 * other.e2 + self.e23 * other.e01 * other.e12 + self.e0123 * other.s * other.e12,
            e31: self.s * other.s * other.e31 - self.s * other.e0 * other.e013 + self.s * other.e2 * other.e123 + self.s * other.e02 * other.e0123 - self.e01 * other.e0 * other.e3 + self.e01 * other.e02 * other.e23 + self.e02 * other.e02 * other.e31 + self.e12 * other.e2 * other.e3 + self.e12 * other.e02 * other.e03 + self.e03 * other.e0 * other.e1 + self.e03 * other.e02 * other.e12 + self.e31 * other.s * other.s + self.e31 * other.e0 * other.e0 + self.e31 * other.e2 * other.e2 + self.e31 * other.e02 * other.e02 + self.e23 * other.e1 * other.e2 + self.e23 * other.e01 * other.e02 + self.e0123 * other.s * other.e02,
            e23: self.s * other.s * other.e23 - self.s * other.e0 * other.e032 + self.s * other.e1 * other.e123 + self.s * other.e01 * other.e0123 + self.e01 * other.e01 * other.e23 + self.e02 * other.e0 * other.e3 + self.e02 * other.e01 * other.e31 + self.e12 * other.e1 * other.e3 + self.e12 * other.e01 * other.e03 - self.e03 * other.e0 * other.e2 + self.e03 * other.e01 * other.e12 + self.e31 * other.e1 * other.e2 + self.e31 * other.e01 * other.e02 + self.e23 * other.s * other.s + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1 + self.e23 * other.e01 * other.e01 + self.e0123 * other.s * other.e01,
            e0123: self.s * other.s * other.e0123 + self.e01 * other.s * other.e23 + self.e02 * other.s * other.e31 + self.e12 * other.s * other.e03 + self.e03 * other.s * other.e12 + self.e31 * other.s * other.e02 + self.e23 * other.s * other.e01 + self.e0123 * other.s * other.s,
        }
    }
}

impl Reject<Scalar> for Multivector {
    type Output = Multivector;
    fn reject(self, other: Scalar) -> Multivector {
        Multivector {
            s: self.s * other.s * other.s,
            e0: self.e0 * other.s * other.s,
            e1: self.e1 * other.s * other.s,
            e01: self.e01 * other.s * other.s,
            e2: self.e2 * other.s * other.s,
            e02: self.e02 * other.s * other.s,
            e12: self.e12 * other.s * other.s,
            e021: self.e021 * other.s * other.s,
            e3: self.e3 * other.s * other.s,
            e03: self.e03 * other.s * other.s,
            e31: self.e31 * other.s * other.s,
            e013: self.e013 * other.s * other.s,
            e23: self.e23 * other.s * other.s,
            e032: self.e032 * other.s * other.s,
            e123: self.e123 * other.s * other.s,
            e0123: self.e0123 * other.s * other.s,
        }
    }
}

impl Reject<Vector> for Multivector {
    type Output = Multivector;
    fn reject(self, other: Vector) -> Multivector {
        Multivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e3 * other.e3,
            e0: self.e0 * other.e1 * other.e1 + self.e0 * other.e2 * other.e2 + self.e0 * other.e3 * other.e3 - self.e1 * other.e0 * other.e1 - self.e2 * other.e0 * other.e2 - self.e3 * other.e0 * other.e3,
            e1: -self.e0 * other.e0 * other.e1 + self.e1 * other.e0 * other.e0 + self.e1 * other.e2 * other.e2 + self.e1 * other.e3 * other.e3 - self.e2 * other.e1 * other.e2 - self.e3 * other.e1 * other.e3,
            e01: self.e01 * other.e2 * other.e2 + self.e01 * other.e3 * other.e3 - self.e02 * other.e1 * other.e2 + self.e12 * other.e0 * other.e2 - self.e03 * other.e1 * other.e3 - self.e31 * other.e0 * other.e3,
            e2: -self.e0 * other.e0 * other.e2 - self.e1 * other.e1 * other.e2 + self.e2 * other.e0 * other.e0 + self.e2 * other.e1 * other.e1 + self.e2 * other.e3 * other.e3 - self.e3 * other.e2 * other.e3,
            e02: -self.e01 * other.e1 * other.e2 + self.e02 * other.e1 * other.e1 + self.e02 * other.e3 * other.e3 - self.e12 * other.e0 * other.e1 - self.e03 * other.e2 * other.e3 + self.e23 * other.e0 * other.e3,
            e12: self.e01 * other.e0 * other.e2 - self.e02 * other.e0 * other.e1 + self.e12 * other.e0 * other.e0 + self.e12 * other.e3 * other.e3 + self.e31 * other.e2 * other.e3 + self.e23 * other.e1 * other.e3,
            e021: self.e021 * other.e3 * other.e3 + self.e013 * other.e2 * other.e3 + self.e032 * other.e1 * other.e3 + self.e123 * other.e0 * other.e3,
            e3: -self.e0 * other.e0 * other.e3 - self.e1 * other.e1 * other.e3 - self.e2 * other.e2 * other.e3 + self.e3 * other.e0 * other.e0 + self.e3 * other.e1 * other.e1 + self.e3 * other.e2 * other.e2,
            e03: -self.e01 * other.e1 * other.e3 - self.e02 * other.e2 * other.e3 + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 + self.e31 * other.e0 * other.e1 - self.e23 * other.e0 * other.e2,
            e31: -self.e01 * other.e0 * other.e3 + self.e12 * other.e2 * other.e3 + self.e03 * other.e0 * other.e1 + self.e31 * other.e0 * other.e0 + self.e31 * other.e2 * other.e2 + self.e23 * other.e1 * other.e2,
            e013: self.e021 * other.e2 * other.e3 + self.e013 * other.e2 * other.e2 + self.e032 * other.e1 * other.e2 + self.e123 * other.e0 * other.e2,
            e23: self.e02 * other.e0 * other.e3 + self.e12 * other.e1 * other.e3 - self.e03 * other.e0 * other.e2 + self.e31 * other.e1 * other.e2 + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1,
            e032: self.e021 * other.e1 * other.e3 + self.e013 * other.e1 * other.e2 + self.e032 * other.e1 * other.e1 + self.e123 * other.e0 * other.e1,
            e123: self.e021 * other.e0 * other.e3 + self.e013 * other.e0 * other.e2 + self.e032 * other.e0 * other.e1 + self.e123 * other.e0 * other.e0,
            e0123: 0.0,
        }
    }
}

impl Reject<Bivector> for Multivector {
    type Output = Multivector;
    fn reject(self, other: Bivector) -> Multivector {
        Multivector {
            s: self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23,
            e0: self.e0 * other.e12 * other.e12 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 - self.e1 * other.e02 * other.e12 + self.e1 * other.e03 * other.e31 + self.e2 * other.e01 * other.e12 - self.e2 * other.e03 * other.e23 - self.e3 * other.e01 * other.e31 + self.e3 * other.e02 * other.e23,
            e1: -self.e0 * other.e02 * other.e12 + self.e0 * other.e03 * other.e31 + self.e1 * other.e02 * other.e02 + self.e1 * other.e03 * other.e03 + self.e1 * other.e23 * other.e23 - self.e2 * other.e01 * other.e02 + self.e2 * other.e31 * other.e23 - self.e3 * other.e01 * other.e03 + self.e3 * other.e12 * other.e23,
            e01: self.e01 * other.e23 * other.e23 + self.e02 * other.e31 * other.e23 + self.e12 * other.e03 * other.e23 + self.e03 * other.e12 * other.e23 + self.e31 * other.e02 * other.e23 + self.e23 * other.e01 * other.e23,
            e2: self.e0 * other.e01 * other.e12 - self.e0 * other.e03 * other.e23 - self.e1 * other.e01 * other.e02 + self.e1 * other.e31 * other.e23 + self.e2 * other.e01 * other.e01 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e3 * other.e02 * other.e03 + self.e3 * other.e12 * other.e31,
            e02: self.e01 * other.e31 * other.e23 + self.e02 * other.e31 * other.e31 + self.e12 * other.e03 * other.e31 + self.e03 * other.e12 * other.e31 + self.e31 * other.e02 * other.e31 + self.e23 * other.e01 * other.e31,
            e12: self.e01 * other.e03 * other.e23 + self.e02 * other.e03 * other.e31 + self.e12 * other.e03 * other.e03 + self.e03 * other.e12 * other.e03 + self.e31 * other.e02 * other.e03 + self.e23 * other.e01 * other.e03,
            e021: 0.0,
            e3: -self.e0 * other.e01 * other.e31 + self.e0 * other.e02 * other.e23 - self.e1 * other.e01 * other.e03 + self.e1 * other.e12 * other.e23 - self.e2 * other.e02 * other.e03 + self.e2 * other.e12 * other.e31 + self.e3 * other.e01 * other.e01 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12,
            e03: self.e01 * other.e12 * other.e23 + self.e02 * other.e12 * other.e31 + self.e12 * other.e12 * other.e03 + self.e03 * other.e12 * other.e12 + self.e31 * other.e02 * other.e12 + self.e23 * other.e01 * other.e12,
            e31: self.e01 * other.e02 * other.e23 + self.e02 * other.e02 * other.e31 + self.e12 * other.e02 * other.e03 + self.e03 * other.e02 * other.e12 + self.e31 * other.e02 * other.e02 + self.e23 * other.e01 * other.e02,
            e013: 0.0,
            e23: self.e01 * other.e01 * other.e23 + self.e02 * other.e01 * other.e31 + self.e12 * other.e01 * other.e03 + self.e03 * other.e01 * other.e12 + self.e31 * other.e01 * other.e02 + self.e23 * other.e01 * other.e01,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl Reject<Trivector> for Multivector {
    type Output = Multivector;
    fn reject(self, other: Trivector) -> Multivector {
        Multivector {
            s: self.s * other.e021 * other.e021 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123,
            e0: self.e0 * other.e123 * other.e123 + self.e1 * other.e032 * other.e123 + self.e2 * other.e013 * other.e123 + self.e3 * other.e021 * other.e123,
            e1: self.e0 * other.e032 * other.e123 + self.e1 * other.e032 * other.e032 + self.e2 * other.e013 * other.e032 + self.e3 * other.e021 * other.e032,
            e01: 0.0,
            e2: self.e0 * other.e013 * other.e123 + self.e1 * other.e013 * other.e032 + self.e2 * other.e013 * other.e013 + self.e3 * other.e021 * other.e013,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: self.e0 * other.e021 * other.e123 + self.e1 * other.e021 * other.e032 + self.e2 * other.e021 * other.e013 + self.e3 * other.e021 * other.e021,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl Reject<FourVector> for Multivector {
    type Output = Multivector;
    fn reject(self, other: FourVector) -> Multivector {
        Multivector {
            s: self.s * other.e0123 * other.e0123,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl Reject<Null> for Multivector {
    type Output = Multivector;
    fn reject(self, other: Null) -> Multivector {
        Multivector {
            s: 0.0,
            e0: 0.0,
            e1: 0.0,
            e01: 0.0,
            e2: 0.0,
            e02: 0.0,
            e12: 0.0,
            e021: 0.0,
            e3: 0.0,
            e03: 0.0,
            e31: 0.0,
            e013: 0.0,
            e23: 0.0,
            e032: 0.0,
            e123: 0.0,
            e0123: 0.0,
        }
    }
}

impl Reject<OddMultivector> for Multivector {
    type Output = Multivector;
    fn reject(self, other: OddMultivector) -> Multivector {
        Multivector {
            s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e2 * other.e2 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e013 * other.e013 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 - self.e01 * other.e2 * other.e021 + self.e01 * other.e3 * other.e013 + self.e02 * other.e1 * other.e021 - self.e02 * other.e3 * other.e032 - self.e12 * other.e0 * other.e021 + self.e12 * other.e3 * other.e123 - self.e03 * other.e1 * other.e013 + self.e03 * other.e2 * other.e032 - self.e31 * other.e0 * other.e013 + self.e31 * other.e2 * other.e123 - self.e23 * other.e0 * other.e032 + self.e23 * other.e1 * other.e123,
            e0: self.e0 * other.e1 * other.e1 + self.e0 * other.e2 * other.e2 + self.e0 * other.e3 * other.e3 + self.e0 * other.e123 * other.e123 - self.e1 * other.e0 * other.e1 + self.e1 * other.e032 * other.e123 - self.e2 * other.e0 * other.e2 + self.e2 * other.e013 * other.e123 - self.e021 * other.e3 * other.e123 - self.e3 * other.e0 * other.e3 + self.e3 * other.e021 * other.e123 - self.e013 * other.e2 * other.e123 - self.e032 * other.e1 * other.e123 - self.e123 * other.e0 * other.e123,
            e1: -self.e0 * other.e0 * other.e1 + self.e0 * other.e032 * other.e123 + self.e1 * other.e0 * other.e0 + self.e1 * other.e2 * other.e2 + self.e1 * other.e3 * other.e3 + self.e1 * other.e032 * other.e032 - self.e2 * other.e1 * other.e2 + self.e2 * other.e013 * other.e032 - self.e021 * other.e3 * other.e032 - self.e3 * other.e1 * other.e3 + self.e3 * other.e021 * other.e032 - self.e013 * other.e2 * other.e032 - self.e032 * other.e1 * other.e032 - self.e123 * other.e0 * other.e032,
            e01: -self.s * other.e2 * other.e021 + self.s * other.e3 * other.e013 + self.e01 * other.e2 * other.e2 + self.e01 * other.e3 * other.e3 - self.e02 * other.e1 * other.e2 + self.e12 * other.e0 * other.e2 - self.e03 * other.e1 * other.e3 - self.e31 * other.e0 * other.e3,
            e2: -self.e0 * other.e0 * other.e2 + self.e0 * other.e013 * other.e123 - self.e1 * other.e1 * other.e2 + self.e1 * other.e013 * other.e032 + self.e2 * other.e0 * other.e0 + self.e2 * other.e1 * other.e1 + self.e2 * other.e3 * other.e3 + self.e2 * other.e013 * other.e013 - self.e021 * other.e3 * other.e013 - self.e3 * other.e2 * other.e3 + self.e3 * other.e021 * other.e013 - self.e013 * other.e2 * other.e013 - self.e032 * other.e1 * other.e013 - self.e123 * other.e0 * other.e013,
            e02: self.s * other.e1 * other.e021 - self.s * other.e3 * other.e032 - self.e01 * other.e1 * other.e2 + self.e02 * other.e1 * other.e1 + self.e02 * other.e3 * other.e3 - self.e12 * other.e0 * other.e1 - self.e03 * other.e2 * other.e3 + self.e23 * other.e0 * other.e3,
            e12: -self.s * other.e0 * other.e021 + self.s * other.e3 * other.e123 + self.e01 * other.e0 * other.e2 - self.e02 * other.e0 * other.e1 + self.e12 * other.e0 * other.e0 + self.e12 * other.e3 * other.e3 + self.e31 * other.e2 * other.e3 + self.e23 * other.e1 * other.e3,
            e021: -self.e0 * other.e3 * other.e123 - self.e1 * other.e3 * other.e032 - self.e2 * other.e3 * other.e013 + self.e021 * other.e3 * other.e3 - self.e3 * other.e021 * other.e3 + self.e013 * other.e2 * other.e3 + self.e032 * other.e1 * other.e3 + self.e123 * other.e0 * other.e3,
            e3: -self.e0 * other.e0 * other.e3 + self.e0 * other.e021 * other.e123 - self.e1 * other.e1 * other.e3 + self.e1 * other.e021 * other.e032 - self.e2 * other.e2 * other.e3 + self.e2 * other.e021 * other.e013 - self.e021 * other.e021 * other.e3 + self.e3 * other.e0 * other.e0 + self.e3 * other.e1 * other.e1 + self.e3 * other.e2 * other.e2 + self.e3 * other.e021 * other.e021 - self.e013 * other.e2 * other.e021 - self.e032 * other.e1 * other.e021 - self.e123 * other.e0 * other.e021,
            e03: -self.s * other.e1 * other.e013 + self.s * other.e2 * other.e032 - self.e01 * other.e1 * other.e3 - self.e02 * other.e2 * other.e3 + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 + self.e31 * other.e0 * other.e1 - self.e23 * other.e0 * other.e2,
            e31: -self.s * other.e0 * other.e013 + self.s * other.e2 * other.e123 - self.e01 * other.e0 * other.e3 + self.e12 * other.e2 * other.e3 + self.e03 * other.e0 * other.e1 + self.e31 * other.e0 * other.e0 + self.e31 * other.e2 * other.e2 + self.e23 * other.e1 * other.e2,
            e013: -self.e0 * other.e2 * other.e123 - self.e1 * other.e2 * other.e032 - self.e2 * other.e2 * other.e013 + self.e021 * other.e2 * other.e3 - self.e3 * other.e2 * other.e021 + self.e013 * other.e2 * other.e2 + self.e032 * other.e1 * other.e2 + self.e123 * other.e0 * other.e2,
            e23: -self.s * other.e0 * other.e032 + self.s * other.e1 * other.e123 + self.e02 * other.e0 * other.e3 + self.e12 * other.e1 * other.e3 - self.e03 * other.e0 * other.e2 + self.e31 * other.e1 * other.e2 + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1,
            e032: -self.e0 * other.e1 * other.e123 - self.e1 * other.e1 * other.e032 - self.e2 * other.e1 * other.e013 + self.e021 * other.e1 * other.e3 - self.e3 * other.e1 * other.e021 + self.e013 * other.e1 * other.e2 + self.e032 * other.e1 * other.e1 + self.e123 * other.e0 * other.e1,
            e123: -self.e0 * other.e0 * other.e123 - self.e1 * other.e0 * other.e032 - self.e2 * other.e0 * other.e013 + self.e021 * other.e0 * other.e3 - self.e3 * other.e0 * other.e021 + self.e013 * other.e0 * other.e2 + self.e032 * other.e0 * other.e1 + self.e123 * other.e0 * other.e0,
            e0123: 0.0,
        }
    }
}

impl Reject<EvenMultivector> for Multivector {
    type Output = Multivector;
    fn reject(self, other: EvenMultivector) -> Multivector {
        Multivector {
            s: self.s * other.s * other.s + self.s * other.e01 * other.e01 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e23 * other.e23 + self.s * other.e0123 * other.e0123 + self.e01 * other.s * other.e01 + self.e01 * other.e23 * other.e0123 + self.e02 * other.s * other.e02 + self.e02 * other.e31 * other.e0123 + self.e12 * other.s * other.e12 + self.e12 * other.e03 * other.e0123 + self.e03 * other.s * other.e03 + self.e03 * other.e12 * other.e0123 + self.e31 * other.s * other.e31 + self.e31 * other.e02 * other.e0123 + self.e23 * other.s * other.e23 + self.e23 * other.e01 * other.e0123 + self.e0123 * other.s * other.e0123,
            e0: self.e0 * other.s * other.s + self.e0 * other.e12 * other.e12 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 - self.e1 * other.e02 * other.e12 + self.e1 * other.e03 * other.e31 + self.e2 * other.e01 * other.e12 - self.e2 * other.e03 * other.e23 - self.e021 * other.s * other.e12 - self.e3 * other.e01 * other.e31 + self.e3 * other.e02 * other.e23 - self.e013 * other.s * other.e31 - self.e032 * other.s * other.e23,
            e1: -self.e0 * other.e02 * other.e12 + self.e0 * other.e03 * other.e31 + self.e1 * other.s * other.s + self.e1 * other.e02 * other.e02 + self.e1 * other.e03 * other.e03 + self.e1 * other.e23 * other.e23 - self.e2 * other.e01 * other.e02 + self.e2 * other.e31 * other.e23 + self.e021 * other.s * other.e02 - self.e3 * other.e01 * other.e03 + self.e3 * other.e12 * other.e23 - self.e013 * other.s * other.e03 + self.e123 * other.s * other.e23,
            e01: self.s * other.s * other.e01 + self.s * other.e23 * other.e0123 + self.e01 * other.s * other.s + self.e01 * other.e23 * other.e23 + self.e02 * other.e31 * other.e23 + self.e12 * other.e03 * other.e23 + self.e03 * other.e12 * other.e23 + self.e31 * other.e02 * other.e23 + self.e23 * other.e01 * other.e23 + self.e0123 * other.s * other.e23,
            e2: self.e0 * other.e01 * other.e12 - self.e0 * other.e03 * other.e23 - self.e1 * other.e01 * other.e02 + self.e1 * other.e31 * other.e23 + self.e2 * other.s * other.s + self.e2 * other.e01 * other.e01 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 - self.e021 * other.s * other.e01 - self.e3 * other.e02 * other.e03 + self.e3 * other.e12 * other.e31 + self.e032 * other.s * other.e03 + self.e123 * other.s * other.e31,
            e02: self.s * other.s * other.e02 + self.s * other.e31 * other.e0123 + self.e01 * other.e31 * other.e23 + self.e02 * other.s * other.s + self.e02 * other.e31 * other.e31 + self.e12 * other.e03 * other.e31 + self.e03 * other.e12 * other.e31 + self.e31 * other.e02 * other.e31 + self.e23 * other.e01 * other.e31 + self.e0123 * other.s * other.e31,
            e12: self.s * other.s * other.e12 + self.s * other.e03 * other.e0123 + self.e01 * other.e03 * other.e23 + self.e02 * other.e03 * other.e31 + self.e12 * other.s * other.s + self.e12 * other.e03 * other.e03 + self.e03 * other.e12 * other.e03 + self.e31 * other.e02 * other.e03 + self.e23 * other.e01 * other.e03 + self.e0123 * other.s * other.e03,
            e021: -self.e0 * other.s * other.e12 + self.e1 * other.s * other.e02 - self.e2 * other.s * other.e01 + self.e021 * other.s * other.s,
            e3: -self.e0 * other.e01 * other.e31 + self.e0 * other.e02 * other.e23 - self.e1 * other.e01 * other.e03 + self.e1 * other.e12 * other.e23 - self.e2 * other.e02 * other.e03 + self.e2 * other.e12 * other.e31 + self.e3 * other.s * other.s + self.e3 * other.e01 * other.e01 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12 + self.e013 * other.s * other.e01 - self.e032 * other.s * other.e02 + self.e123 * other.s * other.e12,
            e03: self.s * other.s * other.e03 + self.s * other.e12 * other.e0123 + self.e01 * other.e12 * other.e23 + self.e02 * other.e12 * other.e31 + self.e12 * other.e12 * other.e03 + self.e03 * other.s * other.s + self.e03 * other.e12 * other.e12 + self.e31 * other.e02 * other.e12 + self.e23 * other.e01 * other.e12 + self.e0123 * other.s * other.e12,
            e31: self.s * other.s * other.e31 + self.s * other.e02 * other.e0123 + self.e01 * other.e02 * other.e23 + self.e02 * other.e02 * other.e31 + self.e12 * other.e02 * other.e03 + self.e03 * other.e02 * other.e12 + self.e31 * other.s * other.s + self.e31 * other.e02 * other.e02 + self.e23 * other.e01 * other.e02 + self.e0123 * other.s * other.e02,
            e013: -self.e0 * other.s * other.e31 - self.e1 * other.s * other.e03 + self.e3 * other.s * other.e01 + self.e013 * other.s * other.s,
            e23: self.s * other.s * other.e23 + self.s * other.e01 * other.e0123 + self.e01 * other.e01 * other.e23 + self.e02 * other.e01 * other.e31 + self.e12 * other.e01 * other.e03 + self.e03 * other.e01 * other.e12 + self.e31 * other.e01 * other.e02 + self.e23 * other.s * other.s + self.e23 * other.e01 * other.e01 + self.e0123 * other.s * other.e01,
            e032: -self.e0 * other.s * other.e23 + self.e2 * other.s * other.e03 - self.e3 * other.s * other.e02 + self.e032 * other.s * other.s,
            e123: self.e1 * other.s * other.e23 + self.e2 * other.s * other.e31 + self.e3 * other.s * other.e12 + self.e123 * other.s * other.s,
            e0123: self.s * other.s * other.e0123 + self.e01 * other.s * other.e23 + self.e02 * other.s * other.e31 + self.e12 * other.s * other.e03 + self.e03 * other.s * other.e12 + self.e31 * other.s * other.e02 + self.e23 * other.s * other.e01 + self.e0123 * other.s * other.s,
        }
    }
}

impl Reject<Multivector> for Multivector {
    type Output = Multivector;
    fn reject(self, other: Multivector) -> Multivector {
        Multivector {
            s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01 + self.s * other.e2 * other.e2 + self.s * other.e02 * other.e02 + self.s * other.e12 * other.e12 + self.s * other.e021 * other.e021 + self.s * other.e3 * other.e3 + self.s * other.e03 * other.e03 + self.s * other.e31 * other.e31 + self.s * other.e013 * other.e013 + self.s * other.e23 * other.e23 + self.s * other.e032 * other.e032 + self.s * other.e123 * other.e123 + self.s * other.e0123 * other.e0123 + self.e0 * other.s * other.e0 + self.e0 * other.e1 * other.e01 + self.e0 * other.e2 * other.e02 - self.e0 * other.e12 * other.e021 + self.e0 * other.e3 * other.e03 - self.e0 * other.e31 * other.e013 - self.e0 * other.e23 * other.e032 + self.e0 * other.e123 * other.e0123 + self.e1 * other.s * other.e1 - self.e1 * other.e0 * other.e01 + self.e1 * other.e2 * other.e12 + self.e1 * other.e02 * other.e021 - self.e1 * other.e3 * other.e31 - self.e1 * other.e03 * other.e013 + self.e1 * other.e23 * other.e123 + self.e1 * other.e032 * other.e0123 + self.e01 * other.s * other.e01 - self.e01 * other.e2 * other.e021 + self.e01 * other.e3 * other.e013 + self.e01 * other.e23 * other.e0123 + self.e2 * other.s * other.e2 - self.e2 * other.e0 * other.e02 - self.e2 * other.e1 * other.e12 - self.e2 * other.e01 * other.e021 + self.e2 * other.e3 * other.e23 + self.e2 * other.e03 * other.e032 + self.e2 * other.e31 * other.e123 + self.e2 * other.e013 * other.e0123 + self.e02 * other.s * other.e02 + self.e02 * other.e1 * other.e021 - self.e02 * other.e3 * other.e032 + self.e02 * other.e31 * other.e0123 + self.e12 * other.s * other.e12 - self.e12 * other.e0 * other.e021 + self.e12 * other.e3 * other.e123 + self.e12 * other.e03 * other.e0123 + self.e021 * other.s * other.e021 - self.e021 * other.e3 * other.e0123 + self.e3 * other.s * other.e3 - self.e3 * other.e0 * other.e03 + self.e3 * other.e1 * other.e31 + self.e3 * other.e01 * other.e013 - self.e3 * other.e2 * other.e23 - self.e3 * other.e02 * other.e032 + self.e3 * other.e12 * other.e123 + self.e3 * other.e021 * other.e0123 + self.e03 * other.s * other.e03 - self.e03 * other.e1 * other.e013 + self.e03 * other.e2 * other.e032 + self.e03 * other.e12 * other.e0123 + self.e31 * other.s * other.e31 - self.e31 * other.e0 * other.e013 + self.e31 * other.e2 * other.e123 + self.e31 * other.e02 * other.e0123 + self.e013 * other.s * other.e013 - self.e013 * other.e2 * other.e0123 + self.e23 * other.s * other.e23 - self.e23 * other.e0 * other.e032 + self.e23 * other.e1 * other.e123 + self.e23 * other.e01 * other.e0123 + self.e032 * other.s * other.e032 - self.e032 * other.e1 * other.e0123 + self.e123 * other.s * other.e123 - self.e123 * other.e0 * other.e0123 + self.e0123 * other.s * other.e0123,
            e0: self.s * other.s * other.e0 + self.s * other.e1 * other.e01 + self.s * other.e2 * other.e02 - self.s * other.e12 * other.e021 + self.s * other.e3 * other.e03 - self.s * other.e31 * other.e013 - self.s * other.e23 * other.e032 + self.s * other.e123 * other.e0123 + self.e0 * other.s * other.s + self.e0 * other.e1 * other.e1 + self.e0 * other.e2 * other.e2 + self.e0 * other.e12 * other.e12 + self.e0 * other.e3 * other.e3 + self.e0 * other.e31 * other.e31 + self.e0 * other.e23 * other.e23 + self.e0 * other.e123 * other.e123 - self.e1 * other.e0 * other.e1 - self.e1 * other.e02 * other.e12 + self.e1 * other.e03 * other.e31 + self.e1 * other.e032 * other.e123 + self.e01 * other.s * other.e1 + self.e01 * other.e2 * other.e12 - self.e01 * other.e3 * other.e31 + self.e01 * other.e23 * other.e123 - self.e2 * other.e0 * other.e2 + self.e2 * other.e01 * other.e12 - self.e2 * other.e03 * other.e23 + self.e2 * other.e013 * other.e123 + self.e02 * other.s * other.e2 - self.e02 * other.e1 * other.e12 + self.e02 * other.e3 * other.e23 + self.e02 * other.e31 * other.e123 + self.e12 * other.e0 * other.e12 + self.e12 * other.e03 * other.e123 - self.e021 * other.s * other.e12 - self.e021 * other.e3 * other.e123 - self.e3 * other.e0 * other.e3 - self.e3 * other.e01 * other.e31 + self.e3 * other.e02 * other.e23 + self.e3 * other.e021 * other.e123 + self.e03 * other.s * other.e3 + self.e03 * other.e1 * other.e31 - self.e03 * other.e2 * other.e23 + self.e03 * other.e12 * other.e123 + self.e31 * other.e0 * other.e31 + self.e31 * other.e02 * other.e123 - self.e013 * other.s * other.e31 - self.e013 * other.e2 * other.e123 + self.e23 * other.e0 * other.e23 + self.e23 * other.e01 * other.e123 - self.e032 * other.s * other.e23 - self.e032 * other.e1 * other.e123 - self.e123 * other.e0 * other.e123 + self.e0123 * other.s * other.e123,
            e1: self.s * other.s * other.e1 - self.s * other.e0 * other.e01 + self.s * other.e2 * other.e12 + self.s * other.e02 * other.e021 - self.s * other.e3 * other.e31 - self.s * other.e03 * other.e013 + self.s * other.e23 * other.e123 + self.s * other.e032 * other.e0123 - self.e0 * other.e0 * other.e1 - self.e0 * other.e02 * other.e12 + self.e0 * other.e03 * other.e31 + self.e0 * other.e032 * other.e123 + self.e1 * other.s * other.s + self.e1 * other.e0 * other.e0 + self.e1 * other.e2 * other.e2 + self.e1 * other.e02 * other.e02 + self.e1 * other.e3 * other.e3 + self.e1 * other.e03 * other.e03 + self.e1 * other.e23 * other.e23 + self.e1 * other.e032 * other.e032 - self.e01 * other.s * other.e0 - self.e01 * other.e2 * other.e02 - self.e01 * other.e3 * other.e03 + self.e01 * other.e23 * other.e032 - self.e2 * other.e1 * other.e2 - self.e2 * other.e01 * other.e02 + self.e2 * other.e31 * other.e23 + self.e2 * other.e013 * other.e032 + self.e02 * other.e1 * other.e02 + self.e02 * other.e31 * other.e032 + self.e12 * other.s * other.e2 - self.e12 * other.e0 * other.e02 + self.e12 * other.e3 * other.e23 + self.e12 * other.e03 * other.e032 + self.e021 * other.s * other.e02 - self.e021 * other.e3 * other.e032 - self.e3 * other.e1 * other.e3 - self.e3 * other.e01 * other.e03 + self.e3 * other.e12 * other.e23 + self.e3 * other.e021 * other.e032 + self.e03 * other.e1 * other.e03 + self.e03 * other.e12 * other.e032 - self.e31 * other.s * other.e3 + self.e31 * other.e0 * other.e03 + self.e31 * other.e2 * other.e23 + self.e31 * other.e02 * other.e032 - self.e013 * other.s * other.e03 - self.e013 * other.e2 * other.e032 + self.e23 * other.e1 * other.e23 + self.e23 * other.e01 * other.e032 - self.e032 * other.e1 * other.e032 + self.e123 * other.s * other.e23 - self.e123 * other.e0 * other.e032 + self.e0123 * other.s * other.e032,
            e01: self.s * other.s * other.e01 - self.s * other.e2 * other.e021 + self.s * other.e3 * other.e013 + self.s * other.e23 * other.e0123 + self.e0 * other.s * other.e1 + self.e0 * other.e2 * other.e12 - self.e0 * other.e3 * other.e31 + self.e0 * other.e23 * other.e123 - self.e1 * other.s * other.e0 - self.e1 * other.e2 * other.e02 - self.e1 * other.e3 * other.e03 + self.e1 * other.e23 * other.e032 + self.e01 * other.s * other.s + self.e01 * other.e2 * other.e2 + self.e01 * other.e3 * other.e3 + self.e01 * other.e23 * other.e23 + self.e2 * other.e01 * other.e2 + self.e2 * other.e013 * other.e23 - self.e02 * other.e1 * other.e2 + self.e02 * other.e31 * other.e23 + self.e12 * other.e0 * other.e2 + self.e12 * other.e03 * other.e23 - self.e021 * other.s * other.e2 - self.e021 * other.e3 * other.e23 + self.e3 * other.e01 * other.e3 + self.e3 * other.e021 * other.e23 - self.e03 * other.e1 * other.e3 + self.e03 * other.e12 * other.e23 - self.e31 * other.e0 * other.e3 + self.e31 * other.e02 * other.e23 + self.e013 * other.s * other.e3 - self.e013 * other.e2 * other.e23 + self.e23 * other.e01 * other.e23 - self.e032 * other.e1 * other.e23 - self.e123 * other.e0 * other.e23 + self.e0123 * other.s * other.e23,
            e2: self.s * other.s * other.e2 - self.s * other.e0 * other.e02 - self.s * other.e1 * other.e12 - self.s * other.e01 * other.e021 + self.s * other.e3 * other.e23 + self.s * other.e03 * other.e032 + self.s * other.e31 * other.e123 + self.s * other.e013 * other.e0123 - self.e0 * other.e0 * other.e2 + self.e0 * other.e01 * other.e12 - self.e0 * other.e03 * other.e23 + self.e0 * other.e013 * other.e123 - self.e1 * other.e1 * other.e2 - self.e1 * other.e01 * other.e02 + self.e1 * other.e31 * other.e23 + self.e1 * other.e013 * other.e032 + self.e01 * other.e01 * other.e2 + self.e01 * other.e013 * other.e23 + self.e2 * other.s * other.s + self.e2 * other.e0 * other.e0 + self.e2 * other.e1 * other.e1 + self.e2 * other.e01 * other.e01 + self.e2 * other.e3 * other.e3 + self.e2 * other.e03 * other.e03 + self.e2 * other.e31 * other.e31 + self.e2 * other.e013 * other.e013 - self.e02 * other.s * other.e0 - self.e02 * other.e1 * other.e01 - self.e02 * other.e3 * other.e03 + self.e02 * other.e31 * other.e013 - self.e12 * other.s * other.e1 + self.e12 * other.e0 * other.e01 + self.e12 * other.e3 * other.e31 + self.e12 * other.e03 * other.e013 - self.e021 * other.s * other.e01 - self.e021 * other.e3 * other.e013 - self.e3 * other.e2 * other.e3 - self.e3 * other.e02 * other.e03 + self.e3 * other.e12 * other.e31 + self.e3 * other.e021 * other.e013 + self.e03 * other.e2 * other.e03 + self.e03 * other.e12 * other.e013 + self.e31 * other.e2 * other.e31 + self.e31 * other.e02 * other.e013 - self.e013 * other.e2 * other.e013 + self.e23 * other.s * other.e3 - self.e23 * other.e0 * other.e03 + self.e23 * other.e1 * other.e31 + self.e23 * other.e01 * other.e013 + self.e032 * other.s * other.e03 - self.e032 * other.e1 * other.e013 + self.e123 * other.s * other.e31 - self.e123 * other.e0 * other.e013 + self.e0123 * other.s * other.e013,
            e02: self.s * other.s * other.e02 + self.s * other.e1 * other.e021 - self.s * other.e3 * other.e032 + self.s * other.e31 * other.e0123 + self.e0 * other.s * other.e2 - self.e0 * other.e1 * other.e12 + self.e0 * other.e3 * other.e23 + self.e0 * other.e31 * other.e123 + self.e1 * other.e1 * other.e02 + self.e1 * other.e31 * other.e032 - self.e01 * other.e1 * other.e2 + self.e01 * other.e31 * other.e23 - self.e2 * other.s * other.e0 - self.e2 * other.e1 * other.e01 - self.e2 * other.e3 * other.e03 + self.e2 * other.e31 * other.e013 + self.e02 * other.s * other.s + self.e02 * other.e1 * other.e1 + self.e02 * other.e3 * other.e3 + self.e02 * other.e31 * other.e31 - self.e12 * other.e0 * other.e1 + self.e12 * other.e03 * other.e31 + self.e021 * other.s * other.e1 - self.e021 * other.e3 * other.e31 + self.e3 * other.e02 * other.e3 + self.e3 * other.e021 * other.e31 - self.e03 * other.e2 * other.e3 + self.e03 * other.e12 * other.e31 + self.e31 * other.e02 * other.e31 - self.e013 * other.e2 * other.e31 + self.e23 * other.e0 * other.e3 + self.e23 * other.e01 * other.e31 - self.e032 * other.s * other.e3 - self.e032 * other.e1 * other.e31 - self.e123 * other.e0 * other.e31 + self.e0123 * other.s * other.e31,
            e12: self.s * other.s * other.e12 - self.s * other.e0 * other.e021 + self.s * other.e3 * other.e123 + self.s * other.e03 * other.e0123 + self.e0 * other.e0 * other.e12 + self.e0 * other.e03 * other.e123 + self.e1 * other.s * other.e2 - self.e1 * other.e0 * other.e02 + self.e1 * other.e3 * other.e23 + self.e1 * other.e03 * other.e032 + self.e01 * other.e0 * other.e2 + self.e01 * other.e03 * other.e23 - self.e2 * other.s * other.e1 + self.e2 * other.e0 * other.e01 + self.e2 * other.e3 * other.e31 + self.e2 * other.e03 * other.e013 - self.e02 * other.e0 * other.e1 + self.e02 * other.e03 * other.e31 + self.e12 * other.s * other.s + self.e12 * other.e0 * other.e0 + self.e12 * other.e3 * other.e3 + self.e12 * other.e03 * other.e03 - self.e021 * other.s * other.e0 - self.e021 * other.e3 * other.e03 + self.e3 * other.e12 * other.e3 + self.e3 * other.e021 * other.e03 + self.e03 * other.e12 * other.e03 + self.e31 * other.e2 * other.e3 + self.e31 * other.e02 * other.e03 - self.e013 * other.e2 * other.e03 + self.e23 * other.e1 * other.e3 + self.e23 * other.e01 * other.e03 - self.e032 * other.e1 * other.e03 + self.e123 * other.s * other.e3 - self.e123 * other.e0 * other.e03 + self.e0123 * other.s * other.e03,
            e021: self.s * other.s * other.e021 - self.s * other.e3 * other.e0123 - self.e0 * other.s * other.e12 - self.e0 * other.e3 * other.e123 + self.e1 * other.s * other.e02 - self.e1 * other.e3 * other.e032 - self.e01 * other.s * other.e2 - self.e01 * other.e3 * other.e23 - self.e2 * other.s * other.e01 - self.e2 * other.e3 * other.e013 + self.e02 * other.s * other.e1 - self.e02 * other.e3 * other.e31 - self.e12 * other.s * other.e0 - self.e12 * other.e3 * other.e03 + self.e021 * other.s * other.s + self.e021 * other.e3 * other.e3 - self.e3 * other.e021 * other.e3 - self.e03 * other.e12 * other.e3 - self.e31 * other.e02 * other.e3 + self.e013 * other.e2 * other.e3 - self.e23 * other.e01 * other.e3 + self.e032 * other.e1 * other.e3 + self.e123 * other.e0 * other.e3 - self.e0123 * other.s * other.e3,
            e3: self.s * other.s * other.e3 - self.s * other.e0 * other.e03 + self.s * other.e1 * other.e31 + self.s * other.e01 * other.e013 - self.s * other.e2 * other.e23 - self.s * other.e02 * other.e032 + self.s * other.e12 * other.e123 + self.s * other.e021 * other.e0123 - self.e0 * other.e0 * other.e3 - self.e0 * other.e01 * other.e31 + self.e0 * other.e02 * other.e23 + self.e0 * other.e021 * other.e123 - self.e1 * other.e1 * other.e3 - self.e1 * other.e01 * other.e03 + self.e1 * other.e12 * other.e23 + self.e1 * other.e021 * other.e032 + self.e01 * other.e01 * other.e3 + self.e01 * other.e021 * other.e23 - self.e2 * other.e2 * other.e3 - self.e2 * other.e02 * other.e03 + self.e2 * other.e12 * other.e31 + self.e2 * other.e021 * other.e013 + self.e02 * other.e02 * other.e3 + self.e02 * other.e021 * other.e31 + self.e12 * other.e12 * other.e3 + self.e12 * other.e021 * other.e03 - self.e021 * other.e021 * other.e3 + self.e3 * other.s * other.s + self.e3 * other.e0 * other.e0 + self.e3 * other.e1 * other.e1 + self.e3 * other.e01 * other.e01 + self.e3 * other.e2 * other.e2 + self.e3 * other.e02 * other.e02 + self.e3 * other.e12 * other.e12 + self.e3 * other.e021 * other.e021 - self.e03 * other.s * other.e0 - self.e03 * other.e1 * other.e01 - self.e03 * other.e2 * other.e02 + self.e03 * other.e12 * other.e021 + self.e31 * other.s * other.e1 - self.e31 * other.e0 * other.e01 + self.e31 * other.e2 * other.e12 + self.e31 * other.e02 * other.e021 + self.e013 * other.s * other.e01 - self.e013 * other.e2 * other.e021 - self.e23 * other.s * other.e2 + self.e23 * other.e0 * other.e02 + self.e23 * other.e1 * other.e12 + self.e23 * other.e01 * other.e021 - self.e032 * other.s * other.e02 - self.e032 * other.e1 * other.e021 + self.e123 * other.s * other.e12 - self.e123 * other.e0 * other.e021 + self.e0123 * other.s * other.e021,
            e03: self.s * other.s * other.e03 - self.s * other.e1 * other.e013 + self.s * other.e2 * other.e032 + self.s * other.e12 * other.e0123 + self.e0 * other.s * other.e3 + self.e0 * other.e1 * other.e31 - self.e0 * other.e2 * other.e23 + self.e0 * other.e12 * other.e123 + self.e1 * other.e1 * other.e03 + self.e1 * other.e12 * other.e032 - self.e01 * other.e1 * other.e3 + self.e01 * other.e12 * other.e23 + self.e2 * other.e2 * other.e03 + self.e2 * other.e12 * other.e013 - self.e02 * other.e2 * other.e3 + self.e02 * other.e12 * other.e31 + self.e12 * other.e12 * other.e03 - self.e021 * other.e12 * other.e3 - self.e3 * other.s * other.e0 - self.e3 * other.e1 * other.e01 - self.e3 * other.e2 * other.e02 + self.e3 * other.e12 * other.e021 + self.e03 * other.s * other.s + self.e03 * other.e1 * other.e1 + self.e03 * other.e2 * other.e2 + self.e03 * other.e12 * other.e12 + self.e31 * other.e0 * other.e1 + self.e31 * other.e02 * other.e12 - self.e013 * other.s * other.e1 - self.e013 * other.e2 * other.e12 - self.e23 * other.e0 * other.e2 + self.e23 * other.e01 * other.e12 + self.e032 * other.s * other.e2 - self.e032 * other.e1 * other.e12 - self.e123 * other.e0 * other.e12 + self.e0123 * other.s * other.e12,
            e31: self.s * other.s * other.e31 - self.s * other.e0 * other.e013 + self.s * other.e2 * other.e123 + self.s * other.e02 * other.e0123 + self.e0 * other.e0 * other.e31 + self.e0 * other.e02 * other.e123 - self.e1 * other.s * other.e3 + self.e1 * other.e0 * other.e03 + self.e1 * other.e2 * other.e23 + self.e1 * other.e02 * other.e032 - self.e01 * other.e0 * other.e3 + self.e01 * other.e02 * other.e23 + self.e2 * other.e2 * other.e31 + self.e2 * other.e02 * other.e013 + self.e02 * other.e02 * other.e31 + self.e12 * other.e2 * other.e3 + self.e12 * other.e02 * other.e03 - self.e021 * other.e02 * other.e3 + self.e3 * other.s * other.e1 - self.e3 * other.e0 * other.e01 + self.e3 * other.e2 * other.e12 + self.e3 * other.e02 * other.e021 + self.e03 * other.e0 * other.e1 + self.e03 * other.e02 * other.e12 + self.e31 * other.s * other.s + self.e31 * other.e0 * other.e0 + self.e31 * other.e2 * other.e2 + self.e31 * other.e02 * other.e02 - self.e013 * other.s * other.e0 - self.e013 * other.e2 * other.e02 + self.e23 * other.e1 * other.e2 + self.e23 * other.e01 * other.e02 - self.e032 * other.e1 * other.e02 + self.e123 * other.s * other.e2 - self.e123 * other.e0 * other.e02 + self.e0123 * other.s * other.e02,
            e013: self.s * other.s * other.e013 - self.s * other.e2 * other.e0123 - self.e0 * other.s * other.e31 - self.e0 * other.e2 * other.e123 - self.e1 * other.s * other.e03 - self.e1 * other.e2 * other.e032 + self.e01 * other.s * other.e3 - self.e01 * other.e2 * other.e23 - self.e2 * other.e2 * other.e013 - self.e02 * other.e2 * other.e31 - self.e12 * other.e2 * other.e03 + self.e021 * other.e2 * other.e3 + self.e3 * other.s * other.e01 - self.e3 * other.e2 * other.e021 - self.e03 * other.s * other.e1 - self.e03 * other.e2 * other.e12 - self.e31 * other.s * other.e0 - self.e31 * other.e2 * other.e02 + self.e013 * other.s * other.s + self.e013 * other.e2 * other.e2 - self.e23 * other.e01 * other.e2 + self.e032 * other.e1 * other.e2 + self.e123 * other.e0 * other.e2 - self.e0123 * other.s * other.e2,
            e23: self.s * other.s * other.e23 - self.s * other.e0 * other.e032 + self.s * other.e1 * other.e123 + self.s * other.e01 * other.e0123 + self.e0 * other.e0 * other.e23 + self.e0 * other.e01 * other.e123 + self.e1 * other.e1 * other.e23 + self.e1 * other.e01 * other.e032 + self.e01 * other.e01 * other.e23 + self.e2 * other.s * other.e3 - self.e2 * other.e0 * other.e03 + self.e2 * other.e1 * other.e31 + self.e2 * other.e01 * other.e013 + self.e02 * other.e0 * other.e3 + self.e02 * other.e01 * other.e31 + self.e12 * other.e1 * other.e3 + self.e12 * other.e01 * other.e03 - self.e021 * other.e01 * other.e3 - self.e3 * other.s * other.e2 + self.e3 * other.e0 * other.e02 + self.e3 * other.e1 * other.e12 + self.e3 * other.e01 * other.e021 - self.e03 * other.e0 * other.e2 + self.e03 * other.e01 * other.e12 + self.e31 * other.e1 * other.e2 + self.e31 * other.e01 * other.e02 - self.e013 * other.e01 * other.e2 + self.e23 * other.s * other.s + self.e23 * other.e0 * other.e0 + self.e23 * other.e1 * other.e1 + self.e23 * other.e01 * other.e01 - self.e032 * other.s * other.e0 - self.e032 * other.e1 * other.e01 + self.e123 * other.s * other.e1 - self.e123 * other.e0 * other.e01 + self.e0123 * other.s * other.e01,
            e032: self.s * other.s * other.e032 - self.s * other.e1 * other.e0123 - self.e0 * other.s * other.e23 - self.e0 * other.e1 * other.e123 - self.e1 * other.e1 * other.e032 - self.e01 * other.e1 * other.e23 + self.e2 * other.s * other.e03 - self.e2 * other.e1 * other.e013 - self.e02 * other.s * other.e3 - self.e02 * other.e1 * other.e31 - self.e12 * other.e1 * other.e03 + self.e021 * other.e1 * other.e3 - self.e3 * other.s * other.e02 - self.e3 * other.e1 * other.e021 + self.e03 * other.s * other.e2 - self.e03 * other.e1 * other.e12 - self.e31 * other.e1 * other.e02 + self.e013 * other.e1 * other.e2 - self.e23 * other.s * other.e0 - self.e23 * other.e1 * other.e01 + self.e032 * other.s * other.s + self.e032 * other.e1 * other.e1 + self.e123 * other.e0 * other.e1 - self.e0123 * other.s * other.e1,
            e123: self.s * other.s * other.e123 - self.s * other.e0 * other.e0123 - self.e0 * other.e0 * other.e123 + self.e1 * other.s * other.e23 - self.e1 * other.e0 * other.e032 - self.e01 * other.e0 * other.e23 + self.e2 * other.s * other.e31 - self.e2 * other.e0 * other.e013 - self.e02 * other.e0 * other.e31 + self.e12 * other.s * other.e3 - self.e12 * other.e0 * other.e03 + self.e021 * other.e0 * other.e3 + self.e3 * other.s * other.e12 - self.e3 * other.e0 * other.e021 - self.e03 * other.e0 * other.e12 + self.e31 * other.s * other.e2 - self.e31 * other.e0 * other.e02 + self.e013 * other.e0 * other.e2 + self.e23 * other.s * other.e1 - self.e23 * other.e0 * other.e01 + self.e032 * other.e0 * other.e1 + self.e123 * other.s * other.s + self.e123 * other.e0 * other.e0 - self.e0123 * other.s * other.e0,
            e0123: self.s * other.s * other.e0123 + self.e0 * other.s * other.e123 + self.e1 * other.s * other.e032 + self.e01 * other.s * other.e23 + self.e2 * other.s * other.e013 + self.e02 * other.s * other.e31 + self.e12 * other.s * other.e03 - self.e021 * other.s * other.e3 + self.e3 * other.s * other.e021 + self.e03 * other.s * other.e12 + self.e31 * other.s * other.e02 - self.e013 * other.s * other.e2 + self.e23 * other.s * other.e01 - self.e032 * other.s * other.e1 - self.e123 * other.s * other.e0 + self.e0123 * other.s * other.s,
        }
    }
}
