// Automatically generated

struct Scalar {
    s: f32,
}

struct Vector {
    e0: f32,
    e1: f32,
}

struct Bivector {
    e01: f32,
}

struct Null {
    _phantom: f32,
}

struct OddMultivector {
    e0: f32,
    e1: f32,
}

struct EvenMultivector {
    s: f32,
    e01: f32,
}

struct Multivector {
    s: f32,
    e0: f32,
    e1: f32,
    e01: f32,
}

fn vector_from_scalar(value: Scalar) -> Vector {
    return Vector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn bivector_from_scalar(value: Scalar) -> Bivector {
    return Bivector {
        e01: 0.0,
    };
}

fn null_from_scalar(value: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_from_scalar(value: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn evenmultivector_from_scalar(value: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: value.s,
        e01: 0.0,
    };
}

fn multivector_from_scalar(value: Scalar) -> Multivector {
    return Multivector {
        s: value.s,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn scalar_from_vector(value: Vector) -> Scalar {
    return Scalar {
        s: 0.0,
    };
}

fn bivector_from_vector(value: Vector) -> Bivector {
    return Bivector {
        e01: 0.0,
    };
}

fn null_from_vector(value: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_from_vector(value: Vector) -> OddMultivector {
    return OddMultivector {
        e0: value.e0,
        e1: value.e1,
    };
}

fn evenmultivector_from_vector(value: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn multivector_from_vector(value: Vector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: value.e0,
        e1: value.e1,
        e01: 0.0,
    };
}

fn scalar_from_bivector(value: Bivector) -> Scalar {
    return Scalar {
        s: 0.0,
    };
}

fn vector_from_bivector(value: Bivector) -> Vector {
    return Vector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn null_from_bivector(value: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_from_bivector(value: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn evenmultivector_from_bivector(value: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: value.e01,
    };
}

fn multivector_from_bivector(value: Bivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: 0.0,
        e1: 0.0,
        e01: value.e01,
    };
}

fn scalar_from_null(value: Null) -> Scalar {
    return Scalar {
        s: 0.0,
    };
}

fn vector_from_null(value: Null) -> Vector {
    return Vector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn bivector_from_null(value: Null) -> Bivector {
    return Bivector {
        e01: 0.0,
    };
}

fn oddmultivector_from_null(value: Null) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn evenmultivector_from_null(value: Null) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn multivector_from_null(value: Null) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn scalar_from_oddmultivector(value: OddMultivector) -> Scalar {
    return Scalar {
        s: 0.0,
    };
}

fn vector_from_oddmultivector(value: OddMultivector) -> Vector {
    return Vector {
        e0: value.e0,
        e1: value.e1,
    };
}

fn bivector_from_oddmultivector(value: OddMultivector) -> Bivector {
    return Bivector {
        e01: 0.0,
    };
}

fn null_from_oddmultivector(value: OddMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn evenmultivector_from_oddmultivector(value: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn multivector_from_oddmultivector(value: OddMultivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: value.e0,
        e1: value.e1,
        e01: 0.0,
    };
}

fn scalar_from_evenmultivector(value: EvenMultivector) -> Scalar {
    return Scalar {
        s: value.s,
    };
}

fn vector_from_evenmultivector(value: EvenMultivector) -> Vector {
    return Vector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn bivector_from_evenmultivector(value: EvenMultivector) -> Bivector {
    return Bivector {
        e01: value.e01,
    };
}

fn null_from_evenmultivector(value: EvenMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_from_evenmultivector(value: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn multivector_from_evenmultivector(value: EvenMultivector) -> Multivector {
    return Multivector {
        s: value.s,
        e0: 0.0,
        e1: 0.0,
        e01: value.e01,
    };
}

fn scalar_from_multivector(value: Multivector) -> Scalar {
    return Scalar {
        s: value.s,
    };
}

fn vector_from_multivector(value: Multivector) -> Vector {
    return Vector {
        e0: value.e0,
        e1: value.e1,
    };
}

fn bivector_from_multivector(value: Multivector) -> Bivector {
    return Bivector {
        e01: value.e01,
    };
}

fn null_from_multivector(value: Multivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_from_multivector(value: Multivector) -> OddMultivector {
    return OddMultivector {
        e0: value.e0,
        e1: value.e1,
    };
}

fn evenmultivector_from_multivector(value: Multivector) -> EvenMultivector {
    return EvenMultivector {
        s: value.s,
        e01: value.e01,
    };
}

fn scalar_from_f32(value: f32) -> Scalar {
    return Scalar {
        s: value,
    };
}

fn vector_from_f32(value: f32) -> Vector {
    return Vector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn bivector_from_f32(value: f32) -> Bivector {
    return Bivector {
        e01: 0.0,
    };
}

fn null_from_f32(value: f32) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_from_f32(value: f32) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn evenmultivector_from_f32(value: f32) -> EvenMultivector {
    return EvenMultivector {
        s: value,
        e01: 0.0,
    };
}

fn multivector_from_f32(value: f32) -> Multivector {
    return Multivector {
        s: value,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn f32_from_scalar(value: Scalar) -> f32 {
    return value.s;
}

fn f32_from_vector(value: Vector) -> f32 {
    return 0.0;
}

fn f32_from_bivector(value: Bivector) -> f32 {
    return 0.0;
}

fn f32_from_null(value: Null) -> f32 {
    return 0.0;
}

fn f32_from_oddmultivector(value: OddMultivector) -> f32 {
    return 0.0;
}

fn f32_from_evenmultivector(value: EvenMultivector) -> f32 {
    return value.s;
}

fn f32_from_multivector(value: Multivector) -> f32 {
    return value.s;
}

fn scalar_neg(self: Scalar) -> Scalar {
    return Scalar {
        s: -self.s,
    };
}

fn vector_neg(self: Vector) -> Vector {
    return Vector {
        e0: -self.e0,
        e1: -self.e1,
    };
}

fn bivector_neg(self: Bivector) -> Bivector {
    return Bivector {
        e01: -self.e01,
    };
}

fn null_neg(self: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_neg(self: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e0,
        e1: -self.e1,
    };
}

fn evenmultivector_neg(self: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.s,
        e01: -self.e01,
    };
}

fn multivector_neg(self: Multivector) -> Multivector {
    return Multivector {
        s: -self.s,
        e0: -self.e0,
        e1: -self.e1,
        e01: -self.e01,
    };
}

fn scalar_add_scalar(self: Scalar, other: Scalar) -> Scalar {
    return Scalar {
        s: self.s + other.s,
    };
}

fn scalar_add_f32(self: Scalar, other: f32) -> Scalar {
    return Scalar {
        s: self.s + other,
    };
}

fn f32_add_scalar(self: f32, other: Scalar) -> Scalar {
    return Scalar {
        s: self + other.s,
    };
}

fn vector_add_vector(self: Vector, other: Vector) -> Vector {
    return Vector {
        e0: self.e0 + other.e0,
        e1: self.e1 + other.e1,
    };
}

fn vector_add_f32(self: Vector, other: f32) -> Vector {
    return Vector {
        e0: self.e0 + other,
        e1: self.e1 + other,
    };
}

fn f32_add_vector(self: f32, other: Vector) -> Vector {
    return Vector {
        e0: self + other.e0,
        e1: self + other.e1,
    };
}

fn bivector_add_bivector(self: Bivector, other: Bivector) -> Bivector {
    return Bivector {
        e01: self.e01 + other.e01,
    };
}

fn bivector_add_f32(self: Bivector, other: f32) -> Bivector {
    return Bivector {
        e01: self.e01 + other,
    };
}

fn f32_add_bivector(self: f32, other: Bivector) -> Bivector {
    return Bivector {
        e01: self + other.e01,
    };
}

fn null_add_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_add_f32(self: Null, other: f32) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn f32_add_null(self: f32, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_add_oddmultivector(self: OddMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 + other.e0,
        e1: self.e1 + other.e1,
    };
}

fn oddmultivector_add_f32(self: OddMultivector, other: f32) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 + other,
        e1: self.e1 + other,
    };
}

fn f32_add_oddmultivector(self: f32, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self + other.e0,
        e1: self + other.e1,
    };
}

fn evenmultivector_add_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s + other.s,
        e01: self.e01 + other.e01,
    };
}

fn evenmultivector_add_f32(self: EvenMultivector, other: f32) -> EvenMultivector {
    return EvenMultivector {
        s: self.s + other,
        e01: self.e01 + other,
    };
}

fn f32_add_evenmultivector(self: f32, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self + other.s,
        e01: self + other.e01,
    };
}

fn multivector_add_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s + other.s,
        e0: self.e0 + other.e0,
        e1: self.e1 + other.e1,
        e01: self.e01 + other.e01,
    };
}

fn multivector_add_f32(self: Multivector, other: f32) -> Multivector {
    return Multivector {
        s: self.s + other,
        e0: self.e0 + other,
        e1: self.e1 + other,
        e01: self.e01 + other,
    };
}

fn f32_add_multivector(self: f32, other: Multivector) -> Multivector {
    return Multivector {
        s: self + other.s,
        e0: self + other.e0,
        e1: self + other.e1,
        e01: self + other.e01,
    };
}

fn scalar_sub_scalar(self: Scalar, other: Scalar) -> Scalar {
    return Scalar {
        s: self.s - other.s,
    };
}

fn scalar_sub_f32(self: Scalar, other: f32) -> Scalar {
    return Scalar {
        s: self.s - other,
    };
}

fn f32_sub_scalar(self: f32, other: Scalar) -> Scalar {
    return Scalar {
        s: self - other.s,
    };
}

fn vector_sub_vector(self: Vector, other: Vector) -> Vector {
    return Vector {
        e0: self.e0 - other.e0,
        e1: self.e1 - other.e1,
    };
}

fn vector_sub_f32(self: Vector, other: f32) -> Vector {
    return Vector {
        e0: self.e0 - other,
        e1: self.e1 - other,
    };
}

fn f32_sub_vector(self: f32, other: Vector) -> Vector {
    return Vector {
        e0: self - other.e0,
        e1: self - other.e1,
    };
}

fn bivector_sub_bivector(self: Bivector, other: Bivector) -> Bivector {
    return Bivector {
        e01: self.e01 - other.e01,
    };
}

fn bivector_sub_f32(self: Bivector, other: f32) -> Bivector {
    return Bivector {
        e01: self.e01 - other,
    };
}

fn f32_sub_bivector(self: f32, other: Bivector) -> Bivector {
    return Bivector {
        e01: self - other.e01,
    };
}

fn null_sub_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_sub_f32(self: Null, other: f32) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn f32_sub_null(self: f32, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_sub_oddmultivector(self: OddMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 - other.e0,
        e1: self.e1 - other.e1,
    };
}

fn oddmultivector_sub_f32(self: OddMultivector, other: f32) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 - other,
        e1: self.e1 - other,
    };
}

fn f32_sub_oddmultivector(self: f32, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self - other.e0,
        e1: self - other.e1,
    };
}

fn evenmultivector_sub_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s - other.s,
        e01: self.e01 - other.e01,
    };
}

fn evenmultivector_sub_f32(self: EvenMultivector, other: f32) -> EvenMultivector {
    return EvenMultivector {
        s: self.s - other,
        e01: self.e01 - other,
    };
}

fn f32_sub_evenmultivector(self: f32, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self - other.s,
        e01: self - other.e01,
    };
}

fn multivector_sub_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s - other.s,
        e0: self.e0 - other.e0,
        e1: self.e1 - other.e1,
        e01: self.e01 - other.e01,
    };
}

fn multivector_sub_f32(self: Multivector, other: f32) -> Multivector {
    return Multivector {
        s: self.s - other,
        e0: self.e0 - other,
        e1: self.e1 - other,
        e01: self.e01 - other,
    };
}

fn f32_sub_multivector(self: f32, other: Multivector) -> Multivector {
    return Multivector {
        s: self - other.s,
        e0: self - other.e0,
        e1: self - other.e1,
        e01: self - other.e01,
    };
}

fn scalar_mul_scalar(self: Scalar, other: Scalar) -> Scalar {
    return Scalar {
        s: self.s * other.s,
    };
}

fn scalar_mul_f32(self: Scalar, other: f32) -> Scalar {
    return Scalar {
        s: self.s * other,
    };
}

fn f32_mul_scalar(self: f32, other: Scalar) -> Scalar {
    return Scalar {
        s: self * other.s,
    };
}

fn vector_mul_vector(self: Vector, other: Vector) -> Vector {
    return Vector {
        e0: self.e0 * other.e0,
        e1: self.e1 * other.e1,
    };
}

fn vector_mul_f32(self: Vector, other: f32) -> Vector {
    return Vector {
        e0: self.e0 * other,
        e1: self.e1 * other,
    };
}

fn f32_mul_vector(self: f32, other: Vector) -> Vector {
    return Vector {
        e0: self * other.e0,
        e1: self * other.e1,
    };
}

fn bivector_mul_bivector(self: Bivector, other: Bivector) -> Bivector {
    return Bivector {
        e01: self.e01 * other.e01,
    };
}

fn bivector_mul_f32(self: Bivector, other: f32) -> Bivector {
    return Bivector {
        e01: self.e01 * other,
    };
}

fn f32_mul_bivector(self: f32, other: Bivector) -> Bivector {
    return Bivector {
        e01: self * other.e01,
    };
}

fn null_mul_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_mul_f32(self: Null, other: f32) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn f32_mul_null(self: f32, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_mul_oddmultivector(self: OddMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e0,
        e1: self.e1 * other.e1,
    };
}

fn oddmultivector_mul_f32(self: OddMultivector, other: f32) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other,
        e1: self.e1 * other,
    };
}

fn f32_mul_oddmultivector(self: f32, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self * other.e0,
        e1: self * other.e1,
    };
}

fn evenmultivector_mul_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: self.e01 * other.e01,
    };
}

fn evenmultivector_mul_f32(self: EvenMultivector, other: f32) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other,
        e01: self.e01 * other,
    };
}

fn f32_mul_evenmultivector(self: f32, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self * other.s,
        e01: self * other.e01,
    };
}

fn multivector_mul_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: self.e0 * other.e0,
        e1: self.e1 * other.e1,
        e01: self.e01 * other.e01,
    };
}

fn multivector_mul_f32(self: Multivector, other: f32) -> Multivector {
    return Multivector {
        s: self.s * other,
        e0: self.e0 * other,
        e1: self.e1 * other,
        e01: self.e01 * other,
    };
}

fn f32_mul_multivector(self: f32, other: Multivector) -> Multivector {
    return Multivector {
        s: self * other.s,
        e0: self * other.e0,
        e1: self * other.e1,
        e01: self * other.e01,
    };
}

fn scalar_div_scalar(self: Scalar, other: Scalar) -> Scalar {
    return Scalar {
        s: self.s / other.s,
    };
}

fn scalar_div_f32(self: Scalar, other: f32) -> Scalar {
    return Scalar {
        s: self.s / other,
    };
}

fn f32_div_scalar(self: f32, other: Scalar) -> Scalar {
    return Scalar {
        s: self / other.s,
    };
}

fn vector_div_vector(self: Vector, other: Vector) -> Vector {
    return Vector {
        e0: self.e0 / other.e0,
        e1: self.e1 / other.e1,
    };
}

fn vector_div_f32(self: Vector, other: f32) -> Vector {
    return Vector {
        e0: self.e0 / other,
        e1: self.e1 / other,
    };
}

fn f32_div_vector(self: f32, other: Vector) -> Vector {
    return Vector {
        e0: self / other.e0,
        e1: self / other.e1,
    };
}

fn bivector_div_bivector(self: Bivector, other: Bivector) -> Bivector {
    return Bivector {
        e01: self.e01 / other.e01,
    };
}

fn bivector_div_f32(self: Bivector, other: f32) -> Bivector {
    return Bivector {
        e01: self.e01 / other,
    };
}

fn f32_div_bivector(self: f32, other: Bivector) -> Bivector {
    return Bivector {
        e01: self / other.e01,
    };
}

fn null_div_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_div_f32(self: Null, other: f32) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn f32_div_null(self: f32, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_div_oddmultivector(self: OddMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 / other.e0,
        e1: self.e1 / other.e1,
    };
}

fn oddmultivector_div_f32(self: OddMultivector, other: f32) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 / other,
        e1: self.e1 / other,
    };
}

fn f32_div_oddmultivector(self: f32, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self / other.e0,
        e1: self / other.e1,
    };
}

fn evenmultivector_div_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s / other.s,
        e01: self.e01 / other.e01,
    };
}

fn evenmultivector_div_f32(self: EvenMultivector, other: f32) -> EvenMultivector {
    return EvenMultivector {
        s: self.s / other,
        e01: self.e01 / other,
    };
}

fn f32_div_evenmultivector(self: f32, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self / other.s,
        e01: self / other.e01,
    };
}

fn multivector_div_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s / other.s,
        e0: self.e0 / other.e0,
        e1: self.e1 / other.e1,
        e01: self.e01 / other.e01,
    };
}

fn multivector_div_f32(self: Multivector, other: f32) -> Multivector {
    return Multivector {
        s: self.s / other,
        e0: self.e0 / other,
        e1: self.e1 / other,
        e01: self.e01 / other,
    };
}

fn f32_div_multivector(self: f32, other: Multivector) -> Multivector {
    return Multivector {
        s: self / other.s,
        e0: self / other.e0,
        e1: self / other.e1,
        e01: self / other.e01,
    };
}

fn scalar_add_assign_scalar(self: ptr<function, Scalar>, other: Scalar) {
    self.s += other.s;
}

fn scalar_add_assign_f32(self: ptr<function, Scalar>, other: f32) {
    self.s += other;
}

fn vector_add_assign_vector(self: ptr<function, Vector>, other: Vector) {
    self.e0 += other.e0;
    self.e1 += other.e1;
}

fn vector_add_assign_f32(self: ptr<function, Vector>, other: f32) {
    self.e0 += other;
    self.e1 += other;
}

fn bivector_add_assign_bivector(self: ptr<function, Bivector>, other: Bivector) {
    self.e01 += other.e01;
}

fn bivector_add_assign_f32(self: ptr<function, Bivector>, other: f32) {
    self.e01 += other;
}

fn null_add_assign_null(self: ptr<function, Null>, other: Null) {
}

fn null_add_assign_f32(self: ptr<function, Null>, other: f32) {
}

fn oddmultivector_add_assign_oddmultivector(self: ptr<function, OddMultivector>, other: OddMultivector) {
    self.e0 += other.e0;
    self.e1 += other.e1;
}

fn oddmultivector_add_assign_f32(self: ptr<function, OddMultivector>, other: f32) {
    self.e0 += other;
    self.e1 += other;
}

fn evenmultivector_add_assign_evenmultivector(self: ptr<function, EvenMultivector>, other: EvenMultivector) {
    self.s += other.s;
    self.e01 += other.e01;
}

fn evenmultivector_add_assign_f32(self: ptr<function, EvenMultivector>, other: f32) {
    self.s += other;
    self.e01 += other;
}

fn multivector_add_assign_multivector(self: ptr<function, Multivector>, other: Multivector) {
    self.s += other.s;
    self.e0 += other.e0;
    self.e1 += other.e1;
    self.e01 += other.e01;
}

fn multivector_add_assign_f32(self: ptr<function, Multivector>, other: f32) {
    self.s += other;
    self.e0 += other;
    self.e1 += other;
    self.e01 += other;
}

fn scalar_sub_assign_scalar(self: ptr<function, Scalar>, other: Scalar) {
    self.s -= other.s;
}

fn scalar_sub_assign_f32(self: ptr<function, Scalar>, other: f32) {
    self.s -= other;
}

fn vector_sub_assign_vector(self: ptr<function, Vector>, other: Vector) {
    self.e0 -= other.e0;
    self.e1 -= other.e1;
}

fn vector_sub_assign_f32(self: ptr<function, Vector>, other: f32) {
    self.e0 -= other;
    self.e1 -= other;
}

fn bivector_sub_assign_bivector(self: ptr<function, Bivector>, other: Bivector) {
    self.e01 -= other.e01;
}

fn bivector_sub_assign_f32(self: ptr<function, Bivector>, other: f32) {
    self.e01 -= other;
}

fn null_sub_assign_null(self: ptr<function, Null>, other: Null) {
}

fn null_sub_assign_f32(self: ptr<function, Null>, other: f32) {
}

fn oddmultivector_sub_assign_oddmultivector(self: ptr<function, OddMultivector>, other: OddMultivector) {
    self.e0 -= other.e0;
    self.e1 -= other.e1;
}

fn oddmultivector_sub_assign_f32(self: ptr<function, OddMultivector>, other: f32) {
    self.e0 -= other;
    self.e1 -= other;
}

fn evenmultivector_sub_assign_evenmultivector(self: ptr<function, EvenMultivector>, other: EvenMultivector) {
    self.s -= other.s;
    self.e01 -= other.e01;
}

fn evenmultivector_sub_assign_f32(self: ptr<function, EvenMultivector>, other: f32) {
    self.s -= other;
    self.e01 -= other;
}

fn multivector_sub_assign_multivector(self: ptr<function, Multivector>, other: Multivector) {
    self.s -= other.s;
    self.e0 -= other.e0;
    self.e1 -= other.e1;
    self.e01 -= other.e01;
}

fn multivector_sub_assign_f32(self: ptr<function, Multivector>, other: f32) {
    self.s -= other;
    self.e0 -= other;
    self.e1 -= other;
    self.e01 -= other;
}

fn scalar_mul_assign_scalar(self: ptr<function, Scalar>, other: Scalar) {
    self.s *= other.s;
}

fn scalar_mul_assign_f32(self: ptr<function, Scalar>, other: f32) {
    self.s *= other;
}

fn vector_mul_assign_vector(self: ptr<function, Vector>, other: Vector) {
    self.e0 *= other.e0;
    self.e1 *= other.e1;
}

fn vector_mul_assign_f32(self: ptr<function, Vector>, other: f32) {
    self.e0 *= other;
    self.e1 *= other;
}

fn bivector_mul_assign_bivector(self: ptr<function, Bivector>, other: Bivector) {
    self.e01 *= other.e01;
}

fn bivector_mul_assign_f32(self: ptr<function, Bivector>, other: f32) {
    self.e01 *= other;
}

fn null_mul_assign_null(self: ptr<function, Null>, other: Null) {
}

fn null_mul_assign_f32(self: ptr<function, Null>, other: f32) {
}

fn oddmultivector_mul_assign_oddmultivector(self: ptr<function, OddMultivector>, other: OddMultivector) {
    self.e0 *= other.e0;
    self.e1 *= other.e1;
}

fn oddmultivector_mul_assign_f32(self: ptr<function, OddMultivector>, other: f32) {
    self.e0 *= other;
    self.e1 *= other;
}

fn evenmultivector_mul_assign_evenmultivector(self: ptr<function, EvenMultivector>, other: EvenMultivector) {
    self.s *= other.s;
    self.e01 *= other.e01;
}

fn evenmultivector_mul_assign_f32(self: ptr<function, EvenMultivector>, other: f32) {
    self.s *= other;
    self.e01 *= other;
}

fn multivector_mul_assign_multivector(self: ptr<function, Multivector>, other: Multivector) {
    self.s *= other.s;
    self.e0 *= other.e0;
    self.e1 *= other.e1;
    self.e01 *= other.e01;
}

fn multivector_mul_assign_f32(self: ptr<function, Multivector>, other: f32) {
    self.s *= other;
    self.e0 *= other;
    self.e1 *= other;
    self.e01 *= other;
}

fn scalar_div_assign_scalar(self: ptr<function, Scalar>, other: Scalar) {
    self.s /= other.s;
}

fn scalar_div_assign_f32(self: ptr<function, Scalar>, other: f32) {
    self.s /= other;
}

fn vector_div_assign_vector(self: ptr<function, Vector>, other: Vector) {
    self.e0 /= other.e0;
    self.e1 /= other.e1;
}

fn vector_div_assign_f32(self: ptr<function, Vector>, other: f32) {
    self.e0 /= other;
    self.e1 /= other;
}

fn bivector_div_assign_bivector(self: ptr<function, Bivector>, other: Bivector) {
    self.e01 /= other.e01;
}

fn bivector_div_assign_f32(self: ptr<function, Bivector>, other: f32) {
    self.e01 /= other;
}

fn null_div_assign_null(self: ptr<function, Null>, other: Null) {
}

fn null_div_assign_f32(self: ptr<function, Null>, other: f32) {
}

fn oddmultivector_div_assign_oddmultivector(self: ptr<function, OddMultivector>, other: OddMultivector) {
    self.e0 /= other.e0;
    self.e1 /= other.e1;
}

fn oddmultivector_div_assign_f32(self: ptr<function, OddMultivector>, other: f32) {
    self.e0 /= other;
    self.e1 /= other;
}

fn evenmultivector_div_assign_evenmultivector(self: ptr<function, EvenMultivector>, other: EvenMultivector) {
    self.s /= other.s;
    self.e01 /= other.e01;
}

fn evenmultivector_div_assign_f32(self: ptr<function, EvenMultivector>, other: f32) {
    self.s /= other;
    self.e01 /= other;
}

fn multivector_div_assign_multivector(self: ptr<function, Multivector>, other: Multivector) {
    self.s /= other.s;
    self.e0 /= other.e0;
    self.e1 /= other.e1;
    self.e01 /= other.e01;
}

fn multivector_div_assign_f32(self: ptr<function, Multivector>, other: f32) {
    self.s /= other;
    self.e0 /= other;
    self.e1 /= other;
    self.e01 /= other;
}

fn scalar_zero() -> Scalar {
    return Scalar {
        s: 0.0,
    };
}

fn vector_zero() -> Vector {
    return Vector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn bivector_zero() -> Bivector {
    return Bivector {
        e01: 0.0,
    };
}

fn null_zero() -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_zero() -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn evenmultivector_zero() -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn multivector_zero() -> Multivector {
    return Multivector {
        s: 0.0,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn scalar_one() -> Scalar {
    return Scalar {
        s: 1.0,
    };
}

fn evenmultivector_one() -> EvenMultivector {
    return EvenMultivector {
        s: 1.0,
        e01: 0.0,
    };
}

fn multivector_one() -> Multivector {
    return Multivector {
        s: 1.0,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn scalar_grade_involution(self: Scalar) -> Scalar {
    return Scalar {
        s: self.s,
    };
}

fn vector_grade_involution(self: Vector) -> Vector {
    return Vector {
        e0: -self.e0,
        e1: -self.e1,
    };
}

fn bivector_grade_involution(self: Bivector) -> Bivector {
    return Bivector {
        e01: self.e01,
    };
}

fn null_grade_involution(self: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_grade_involution(self: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e0,
        e1: -self.e1,
    };
}

fn evenmultivector_grade_involution(self: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s,
        e01: self.e01,
    };
}

fn multivector_grade_involution(self: Multivector) -> Multivector {
    return Multivector {
        s: self.s,
        e0: -self.e0,
        e1: -self.e1,
        e01: self.e01,
    };
}

fn scalar_reverse(self: Scalar) -> Scalar {
    return Scalar {
        s: self.s,
    };
}

fn vector_reverse(self: Vector) -> Vector {
    return Vector {
        e0: self.e0,
        e1: self.e1,
    };
}

fn bivector_reverse(self: Bivector) -> Bivector {
    return Bivector {
        e01: -self.e01,
    };
}

fn null_reverse(self: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_reverse(self: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0,
        e1: self.e1,
    };
}

fn evenmultivector_reverse(self: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s,
        e01: -self.e01,
    };
}

fn multivector_reverse(self: Multivector) -> Multivector {
    return Multivector {
        s: self.s,
        e0: self.e0,
        e1: self.e1,
        e01: -self.e01,
    };
}

fn scalar_conjugate(self: Scalar) -> Scalar {
    return Scalar {
        s: self.s,
    };
}

fn vector_conjugate(self: Vector) -> Vector {
    return Vector {
        e0: -self.e0,
        e1: -self.e1,
    };
}

fn bivector_conjugate(self: Bivector) -> Bivector {
    return Bivector {
        e01: -self.e01,
    };
}

fn null_conjugate(self: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_conjugate(self: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e0,
        e1: -self.e1,
    };
}

fn evenmultivector_conjugate(self: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s,
        e01: -self.e01,
    };
}

fn multivector_conjugate(self: Multivector) -> Multivector {
    return Multivector {
        s: self.s,
        e0: -self.e0,
        e1: -self.e1,
        e01: -self.e01,
    };
}

fn scalar_dual(self: Scalar) -> Bivector {
    return Bivector {
        e01: self.s,
    };
}

fn vector_dual(self: Vector) -> Vector {
    return Vector {
        e0: self.e1,
        e1: -self.e0,
    };
}

fn bivector_dual(self: Bivector) -> Scalar {
    return Scalar {
        s: self.e01,
    };
}

fn null_dual(self: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_dual(self: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e1,
        e1: -self.e0,
    };
}

fn evenmultivector_dual(self: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e01,
        e01: self.s,
    };
}

fn multivector_dual(self: Multivector) -> Multivector {
    return Multivector {
        s: self.e01,
        e0: self.e1,
        e1: -self.e0,
        e01: self.s,
    };
}

fn scalar_undual(self: Scalar) -> Bivector {
    return Bivector {
        e01: self.s,
    };
}

fn vector_undual(self: Vector) -> Vector {
    return Vector {
        e0: -self.e1,
        e1: self.e0,
    };
}

fn bivector_undual(self: Bivector) -> Scalar {
    return Scalar {
        s: self.e01,
    };
}

fn null_undual(self: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_undual(self: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e1,
        e1: self.e0,
    };
}

fn evenmultivector_undual(self: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e01,
        e01: self.s,
    };
}

fn multivector_undual(self: Multivector) -> Multivector {
    return Multivector {
        s: self.e01,
        e0: -self.e1,
        e1: self.e0,
        e01: self.s,
    };
}

fn scalar_norm_squared(self: Scalar) -> f32 {
    return self.s * self.s;
}

fn vector_norm_squared(self: Vector) -> f32 {
    return self.e0 * self.e0 + self.e1 * self.e1;
}

fn bivector_norm_squared(self: Bivector) -> f32 {
    return self.e01 * self.e01;
}

fn null_norm_squared(self: Null) -> f32 {
    return 0.0;
}

fn oddmultivector_norm_squared(self: OddMultivector) -> f32 {
    return self.e0 * self.e0 + self.e1 * self.e1;
}

fn evenmultivector_norm_squared(self: EvenMultivector) -> f32 {
    return self.s * self.s + self.e01 * self.e01;
}

fn multivector_norm_squared(self: Multivector) -> f32 {
    return self.s * self.s + self.e0 * self.e0 + self.e1 * self.e1 + self.e01 * self.e01;
}

fn scalar_norm(self: Scalar) -> f32 {
    return sqrt(abs(scalar_norm_squared(self)));
}

fn vector_norm(self: Vector) -> f32 {
    return sqrt(abs(vector_norm_squared(self)));
}

fn bivector_norm(self: Bivector) -> f32 {
    return sqrt(abs(bivector_norm_squared(self)));
}

fn null_norm(self: Null) -> f32 {
    return sqrt(abs(null_norm_squared(self)));
}

fn oddmultivector_norm(self: OddMultivector) -> f32 {
    return sqrt(abs(oddmultivector_norm_squared(self)));
}

fn evenmultivector_norm(self: EvenMultivector) -> f32 {
    return sqrt(abs(evenmultivector_norm_squared(self)));
}

fn multivector_norm(self: Multivector) -> f32 {
    return sqrt(abs(multivector_norm_squared(self)));
}

fn scalar_inverse(self: Scalar) -> Scalar {
    return scalar_div_f32(scalar_reverse(self), scalar_norm_squared(self));
}

fn vector_inverse(self: Vector) -> Vector {
    return vector_div_f32(vector_reverse(self), vector_norm_squared(self));
}

fn bivector_inverse(self: Bivector) -> Bivector {
    return bivector_div_f32(bivector_reverse(self), bivector_norm_squared(self));
}

fn null_inverse(self: Null) -> Null {
    return null_div_f32(null_reverse(self), null_norm_squared(self));
}

fn oddmultivector_inverse(self: OddMultivector) -> OddMultivector {
    return oddmultivector_div_f32(oddmultivector_reverse(self), oddmultivector_norm_squared(self));
}

fn evenmultivector_inverse(self: EvenMultivector) -> EvenMultivector {
    return evenmultivector_div_f32(evenmultivector_reverse(self), evenmultivector_norm_squared(self));
}

fn multivector_inverse(self: Multivector) -> Multivector {
    return multivector_div_f32(multivector_reverse(self), multivector_norm_squared(self));
}

fn scalar_normalized(self: Scalar) -> Scalar {
    return scalar_div_f32(self, scalar_norm(self));
}

fn vector_normalized(self: Vector) -> Vector {
    return vector_div_f32(self, vector_norm(self));
}

fn bivector_normalized(self: Bivector) -> Bivector {
    return bivector_div_f32(self, bivector_norm(self));
}

fn null_normalized(self: Null) -> Null {
    return null_div_f32(self, null_norm(self));
}

fn oddmultivector_normalized(self: OddMultivector) -> OddMultivector {
    return oddmultivector_div_f32(self, oddmultivector_norm(self));
}

fn evenmultivector_normalized(self: EvenMultivector) -> EvenMultivector {
    return evenmultivector_div_f32(self, evenmultivector_norm(self));
}

fn multivector_normalized(self: Multivector) -> Multivector {
    return multivector_div_f32(self, multivector_norm(self));
}

fn scalar_normalize(self: ptr<function, Scalar>) {
    scalar_div_assign_f32(self, scalar_norm(self));
}

fn vector_normalize(self: ptr<function, Vector>) {
    vector_div_assign_f32(self, vector_norm(self));
}

fn bivector_normalize(self: ptr<function, Bivector>) {
    bivector_div_assign_f32(self, bivector_norm(self));
}

fn null_normalize(self: ptr<function, Null>) {
    null_div_assign_f32(self, null_norm(self));
}

fn oddmultivector_normalize(self: ptr<function, OddMultivector>) {
    oddmultivector_div_assign_f32(self, oddmultivector_norm(self));
}

fn evenmultivector_normalize(self: ptr<function, EvenMultivector>) {
    evenmultivector_div_assign_f32(self, evenmultivector_norm(self));
}

fn multivector_normalize(self: ptr<function, Multivector>) {
    multivector_div_assign_f32(self, multivector_norm(self));
}

fn scalar_geometric_product_scalar(self: Scalar, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: 0.0,
    };
}

fn scalar_geometric_product_vector(self: Scalar, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn scalar_geometric_product_bivector(self: Scalar, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.s * other.e01,
    };
}

fn scalar_geometric_product_null(self: Scalar, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn scalar_geometric_product_oddmultivector(self: Scalar, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn scalar_geometric_product_evenmultivector(self: Scalar, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: self.s * other.e01,
    };
}

fn scalar_geometric_product_multivector(self: Scalar, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: self.s * other.e0,
        e1: self.s * other.e1,
        e01: self.s * other.e01,
    };
}

fn vector_geometric_product_scalar(self: Vector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn vector_geometric_product_vector(self: Vector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn vector_geometric_product_bivector(self: Vector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
    };
}

fn vector_geometric_product_null(self: Vector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_geometric_product_oddmultivector(self: Vector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn vector_geometric_product_evenmultivector(self: Vector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s - self.e1 * other.e01,
        e1: self.e0 * other.e01 + self.e1 * other.s,
    };
}

fn vector_geometric_product_multivector(self: Vector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.e0 * other.s - self.e1 * other.e01,
        e1: self.e0 * other.e01 + self.e1 * other.s,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn bivector_geometric_product_scalar(self: Bivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.e01 * other.s,
    };
}

fn bivector_geometric_product_vector(self: Bivector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
    };
}

fn bivector_geometric_product_bivector(self: Bivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.e01 * other.e01,
        e01: 0.0,
    };
}

fn bivector_geometric_product_null(self: Bivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_geometric_product_oddmultivector(self: Bivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
    };
}

fn bivector_geometric_product_evenmultivector(self: Bivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.e01 * other.e01,
        e01: self.e01 * other.s,
    };
}

fn bivector_geometric_product_multivector(self: Bivector, other: Multivector) -> Multivector {
    return Multivector {
        s: -self.e01 * other.e01,
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
        e01: self.e01 * other.s,
    };
}

fn null_geometric_product_scalar(self: Null, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_geometric_product_vector(self: Null, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_geometric_product_bivector(self: Null, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_geometric_product_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_geometric_product_oddmultivector(self: Null, other: OddMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_geometric_product_evenmultivector(self: Null, other: EvenMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_geometric_product_multivector(self: Null, other: Multivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_geometric_product_scalar(self: OddMultivector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn oddmultivector_geometric_product_vector(self: OddMultivector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn oddmultivector_geometric_product_bivector(self: OddMultivector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
    };
}

fn oddmultivector_geometric_product_null(self: OddMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_geometric_product_oddmultivector(self: OddMultivector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn oddmultivector_geometric_product_evenmultivector(self: OddMultivector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s - self.e1 * other.e01,
        e1: self.e0 * other.e01 + self.e1 * other.s,
    };
}

fn oddmultivector_geometric_product_multivector(self: OddMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.e0 * other.s - self.e1 * other.e01,
        e1: self.e0 * other.e01 + self.e1 * other.s,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn evenmultivector_geometric_product_scalar(self: EvenMultivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: self.e01 * other.s,
    };
}

fn evenmultivector_geometric_product_vector(self: EvenMultivector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0 + self.e01 * other.e1,
        e1: self.s * other.e1 - self.e01 * other.e0,
    };
}

fn evenmultivector_geometric_product_bivector(self: EvenMultivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.e01 * other.e01,
        e01: self.s * other.e01,
    };
}

fn evenmultivector_geometric_product_null(self: EvenMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn evenmultivector_geometric_product_oddmultivector(self: EvenMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0 + self.e01 * other.e1,
        e1: self.s * other.e1 - self.e01 * other.e0,
    };
}

fn evenmultivector_geometric_product_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s - self.e01 * other.e01,
        e01: self.s * other.e01 + self.e01 * other.s,
    };
}

fn evenmultivector_geometric_product_multivector(self: EvenMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s - self.e01 * other.e01,
        e0: self.s * other.e0 + self.e01 * other.e1,
        e1: self.s * other.e1 - self.e01 * other.e0,
        e01: self.s * other.e01 + self.e01 * other.s,
    };
}

fn multivector_geometric_product_scalar(self: Multivector, other: Scalar) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
        e01: self.e01 * other.s,
    };
}

fn multivector_geometric_product_vector(self: Multivector, other: Vector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.s * other.e0 + self.e01 * other.e1,
        e1: self.s * other.e1 - self.e01 * other.e0,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn multivector_geometric_product_bivector(self: Multivector, other: Bivector) -> Multivector {
    return Multivector {
        s: -self.e01 * other.e01,
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
        e01: self.s * other.e01,
    };
}

fn multivector_geometric_product_null(self: Multivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn multivector_geometric_product_oddmultivector(self: Multivector, other: OddMultivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.s * other.e0 + self.e01 * other.e1,
        e1: self.s * other.e1 - self.e01 * other.e0,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn multivector_geometric_product_evenmultivector(self: Multivector, other: EvenMultivector) -> Multivector {
    return Multivector {
        s: self.s * other.s - self.e01 * other.e01,
        e0: self.e0 * other.s - self.e1 * other.e01,
        e1: self.e0 * other.e01 + self.e1 * other.s,
        e01: self.s * other.e01 + self.e01 * other.s,
    };
}

fn multivector_geometric_product_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s + self.e0 * other.e0 + self.e1 * other.e1 - self.e01 * other.e01,
        e0: self.s * other.e0 + self.e0 * other.s - self.e1 * other.e01 + self.e01 * other.e1,
        e1: self.s * other.e1 + self.e0 * other.e01 + self.e1 * other.s - self.e01 * other.e0,
        e01: self.s * other.e01 + self.e0 * other.e1 - self.e1 * other.e0 + self.e01 * other.s,
    };
}

fn scalar_scalar_product_scalar(self: Scalar, other: Scalar) -> f32 {
    return self.s * other.s;
}

fn scalar_scalar_product_vector(self: Scalar, other: Vector) -> f32 {
    return 0.0;
}

fn scalar_scalar_product_bivector(self: Scalar, other: Bivector) -> f32 {
    return 0.0;
}

fn scalar_scalar_product_null(self: Scalar, other: Null) -> f32 {
    return 0.0;
}

fn scalar_scalar_product_oddmultivector(self: Scalar, other: OddMultivector) -> f32 {
    return 0.0;
}

fn scalar_scalar_product_evenmultivector(self: Scalar, other: EvenMultivector) -> f32 {
    return self.s * other.s;
}

fn scalar_scalar_product_multivector(self: Scalar, other: Multivector) -> f32 {
    return self.s * other.s;
}

fn vector_scalar_product_scalar(self: Vector, other: Scalar) -> f32 {
    return 0.0;
}

fn vector_scalar_product_vector(self: Vector, other: Vector) -> f32 {
    return self.e0 * other.e0 + self.e1 * other.e1;
}

fn vector_scalar_product_bivector(self: Vector, other: Bivector) -> f32 {
    return 0.0;
}

fn vector_scalar_product_null(self: Vector, other: Null) -> f32 {
    return 0.0;
}

fn vector_scalar_product_oddmultivector(self: Vector, other: OddMultivector) -> f32 {
    return self.e0 * other.e0 + self.e1 * other.e1;
}

fn vector_scalar_product_evenmultivector(self: Vector, other: EvenMultivector) -> f32 {
    return 0.0;
}

fn vector_scalar_product_multivector(self: Vector, other: Multivector) -> f32 {
    return self.e0 * other.e0 + self.e1 * other.e1;
}

fn bivector_scalar_product_scalar(self: Bivector, other: Scalar) -> f32 {
    return 0.0;
}

fn bivector_scalar_product_vector(self: Bivector, other: Vector) -> f32 {
    return 0.0;
}

fn bivector_scalar_product_bivector(self: Bivector, other: Bivector) -> f32 {
    return -self.e01 * other.e01;
}

fn bivector_scalar_product_null(self: Bivector, other: Null) -> f32 {
    return 0.0;
}

fn bivector_scalar_product_oddmultivector(self: Bivector, other: OddMultivector) -> f32 {
    return 0.0;
}

fn bivector_scalar_product_evenmultivector(self: Bivector, other: EvenMultivector) -> f32 {
    return -self.e01 * other.e01;
}

fn bivector_scalar_product_multivector(self: Bivector, other: Multivector) -> f32 {
    return -self.e01 * other.e01;
}

fn null_scalar_product_scalar(self: Null, other: Scalar) -> f32 {
    return 0.0;
}

fn null_scalar_product_vector(self: Null, other: Vector) -> f32 {
    return 0.0;
}

fn null_scalar_product_bivector(self: Null, other: Bivector) -> f32 {
    return 0.0;
}

fn null_scalar_product_null(self: Null, other: Null) -> f32 {
    return 0.0;
}

fn null_scalar_product_oddmultivector(self: Null, other: OddMultivector) -> f32 {
    return 0.0;
}

fn null_scalar_product_evenmultivector(self: Null, other: EvenMultivector) -> f32 {
    return 0.0;
}

fn null_scalar_product_multivector(self: Null, other: Multivector) -> f32 {
    return 0.0;
}

fn oddmultivector_scalar_product_scalar(self: OddMultivector, other: Scalar) -> f32 {
    return 0.0;
}

fn oddmultivector_scalar_product_vector(self: OddMultivector, other: Vector) -> f32 {
    return self.e0 * other.e0 + self.e1 * other.e1;
}

fn oddmultivector_scalar_product_bivector(self: OddMultivector, other: Bivector) -> f32 {
    return 0.0;
}

fn oddmultivector_scalar_product_null(self: OddMultivector, other: Null) -> f32 {
    return 0.0;
}

fn oddmultivector_scalar_product_oddmultivector(self: OddMultivector, other: OddMultivector) -> f32 {
    return self.e0 * other.e0 + self.e1 * other.e1;
}

fn oddmultivector_scalar_product_evenmultivector(self: OddMultivector, other: EvenMultivector) -> f32 {
    return 0.0;
}

fn oddmultivector_scalar_product_multivector(self: OddMultivector, other: Multivector) -> f32 {
    return self.e0 * other.e0 + self.e1 * other.e1;
}

fn evenmultivector_scalar_product_scalar(self: EvenMultivector, other: Scalar) -> f32 {
    return self.s * other.s;
}

fn evenmultivector_scalar_product_vector(self: EvenMultivector, other: Vector) -> f32 {
    return 0.0;
}

fn evenmultivector_scalar_product_bivector(self: EvenMultivector, other: Bivector) -> f32 {
    return -self.e01 * other.e01;
}

fn evenmultivector_scalar_product_null(self: EvenMultivector, other: Null) -> f32 {
    return 0.0;
}

fn evenmultivector_scalar_product_oddmultivector(self: EvenMultivector, other: OddMultivector) -> f32 {
    return 0.0;
}

fn evenmultivector_scalar_product_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> f32 {
    return self.s * other.s - self.e01 * other.e01;
}

fn evenmultivector_scalar_product_multivector(self: EvenMultivector, other: Multivector) -> f32 {
    return self.s * other.s - self.e01 * other.e01;
}

fn multivector_scalar_product_scalar(self: Multivector, other: Scalar) -> f32 {
    return self.s * other.s;
}

fn multivector_scalar_product_vector(self: Multivector, other: Vector) -> f32 {
    return self.e0 * other.e0 + self.e1 * other.e1;
}

fn multivector_scalar_product_bivector(self: Multivector, other: Bivector) -> f32 {
    return -self.e01 * other.e01;
}

fn multivector_scalar_product_null(self: Multivector, other: Null) -> f32 {
    return 0.0;
}

fn multivector_scalar_product_oddmultivector(self: Multivector, other: OddMultivector) -> f32 {
    return self.e0 * other.e0 + self.e1 * other.e1;
}

fn multivector_scalar_product_evenmultivector(self: Multivector, other: EvenMultivector) -> f32 {
    return self.s * other.s - self.e01 * other.e01;
}

fn multivector_scalar_product_multivector(self: Multivector, other: Multivector) -> f32 {
    return self.s * other.s + self.e0 * other.e0 + self.e1 * other.e1 - self.e01 * other.e01;
}

fn scalar_left_inner_product_scalar(self: Scalar, other: Scalar) -> Scalar {
    return Scalar {
        s: self.s * other.s,
    };
}

fn scalar_left_inner_product_vector(self: Scalar, other: Vector) -> Vector {
    return Vector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn scalar_left_inner_product_bivector(self: Scalar, other: Bivector) -> Bivector {
    return Bivector {
        e01: self.s * other.e01,
    };
}

fn scalar_left_inner_product_null(self: Scalar, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn scalar_left_inner_product_oddmultivector(self: Scalar, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn scalar_left_inner_product_evenmultivector(self: Scalar, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: self.s * other.e01,
    };
}

fn scalar_left_inner_product_multivector(self: Scalar, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: self.s * other.e0,
        e1: self.s * other.e1,
        e01: self.s * other.e01,
    };
}

fn vector_left_inner_product_scalar(self: Vector, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_left_inner_product_vector(self: Vector, other: Vector) -> Scalar {
    return Scalar {
        s: self.e0 * other.e0 + self.e1 * other.e1,
    };
}

fn vector_left_inner_product_bivector(self: Vector, other: Bivector) -> Vector {
    return Vector {
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
    };
}

fn vector_left_inner_product_null(self: Vector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_left_inner_product_oddmultivector(self: Vector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: 0.0,
    };
}

fn vector_left_inner_product_evenmultivector(self: Vector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
    };
}

fn vector_left_inner_product_multivector(self: Vector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
        e01: 0.0,
    };
}

fn bivector_left_inner_product_scalar(self: Bivector, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_left_inner_product_vector(self: Bivector, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_left_inner_product_bivector(self: Bivector, other: Bivector) -> Scalar {
    return Scalar {
        s: -self.e01 * other.e01,
    };
}

fn bivector_left_inner_product_null(self: Bivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_left_inner_product_oddmultivector(self: Bivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn bivector_left_inner_product_evenmultivector(self: Bivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.e01 * other.e01,
        e01: 0.0,
    };
}

fn bivector_left_inner_product_multivector(self: Bivector, other: Multivector) -> Multivector {
    return Multivector {
        s: -self.e01 * other.e01,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn null_left_inner_product_scalar(self: Null, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_left_inner_product_vector(self: Null, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_left_inner_product_bivector(self: Null, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_left_inner_product_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_left_inner_product_oddmultivector(self: Null, other: OddMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_left_inner_product_evenmultivector(self: Null, other: EvenMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_left_inner_product_multivector(self: Null, other: Multivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_left_inner_product_scalar(self: OddMultivector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn oddmultivector_left_inner_product_vector(self: OddMultivector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: 0.0,
    };
}

fn oddmultivector_left_inner_product_bivector(self: OddMultivector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
    };
}

fn oddmultivector_left_inner_product_null(self: OddMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_left_inner_product_oddmultivector(self: OddMultivector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: 0.0,
    };
}

fn oddmultivector_left_inner_product_evenmultivector(self: OddMultivector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
    };
}

fn oddmultivector_left_inner_product_multivector(self: OddMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
        e01: 0.0,
    };
}

fn evenmultivector_left_inner_product_scalar(self: EvenMultivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: 0.0,
    };
}

fn evenmultivector_left_inner_product_vector(self: EvenMultivector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn evenmultivector_left_inner_product_bivector(self: EvenMultivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.e01 * other.e01,
        e01: self.s * other.e01,
    };
}

fn evenmultivector_left_inner_product_null(self: EvenMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn evenmultivector_left_inner_product_oddmultivector(self: EvenMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn evenmultivector_left_inner_product_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s - self.e01 * other.e01,
        e01: self.s * other.e01,
    };
}

fn evenmultivector_left_inner_product_multivector(self: EvenMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s - self.e01 * other.e01,
        e0: self.s * other.e0,
        e1: self.s * other.e1,
        e01: self.s * other.e01,
    };
}

fn multivector_left_inner_product_scalar(self: Multivector, other: Scalar) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn multivector_left_inner_product_vector(self: Multivector, other: Vector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.s * other.e0,
        e1: self.s * other.e1,
        e01: 0.0,
    };
}

fn multivector_left_inner_product_bivector(self: Multivector, other: Bivector) -> Multivector {
    return Multivector {
        s: -self.e01 * other.e01,
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
        e01: self.s * other.e01,
    };
}

fn multivector_left_inner_product_null(self: Multivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn multivector_left_inner_product_oddmultivector(self: Multivector, other: OddMultivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.s * other.e0,
        e1: self.s * other.e1,
        e01: 0.0,
    };
}

fn multivector_left_inner_product_evenmultivector(self: Multivector, other: EvenMultivector) -> Multivector {
    return Multivector {
        s: self.s * other.s - self.e01 * other.e01,
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
        e01: self.s * other.e01,
    };
}

fn multivector_left_inner_product_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s + self.e0 * other.e0 + self.e1 * other.e1 - self.e01 * other.e01,
        e0: self.s * other.e0 - self.e1 * other.e01,
        e1: self.s * other.e1 + self.e0 * other.e01,
        e01: self.s * other.e01,
    };
}

fn scalar_right_inner_product_scalar(self: Scalar, other: Scalar) -> Scalar {
    return Scalar {
        s: self.s * other.s,
    };
}

fn scalar_right_inner_product_vector(self: Scalar, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn scalar_right_inner_product_bivector(self: Scalar, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn scalar_right_inner_product_null(self: Scalar, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn scalar_right_inner_product_oddmultivector(self: Scalar, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn scalar_right_inner_product_evenmultivector(self: Scalar, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: 0.0,
    };
}

fn scalar_right_inner_product_multivector(self: Scalar, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn vector_right_inner_product_scalar(self: Vector, other: Scalar) -> Vector {
    return Vector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn vector_right_inner_product_vector(self: Vector, other: Vector) -> Scalar {
    return Scalar {
        s: self.e0 * other.e0 + self.e1 * other.e1,
    };
}

fn vector_right_inner_product_bivector(self: Vector, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_right_inner_product_null(self: Vector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_right_inner_product_oddmultivector(self: Vector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: 0.0,
    };
}

fn vector_right_inner_product_evenmultivector(self: Vector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn vector_right_inner_product_multivector(self: Vector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
        e01: 0.0,
    };
}

fn bivector_right_inner_product_scalar(self: Bivector, other: Scalar) -> Bivector {
    return Bivector {
        e01: self.e01 * other.s,
    };
}

fn bivector_right_inner_product_vector(self: Bivector, other: Vector) -> Vector {
    return Vector {
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
    };
}

fn bivector_right_inner_product_bivector(self: Bivector, other: Bivector) -> Scalar {
    return Scalar {
        s: -self.e01 * other.e01,
    };
}

fn bivector_right_inner_product_null(self: Bivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_right_inner_product_oddmultivector(self: Bivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
    };
}

fn bivector_right_inner_product_evenmultivector(self: Bivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.e01 * other.e01,
        e01: self.e01 * other.s,
    };
}

fn bivector_right_inner_product_multivector(self: Bivector, other: Multivector) -> Multivector {
    return Multivector {
        s: -self.e01 * other.e01,
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
        e01: self.e01 * other.s,
    };
}

fn null_right_inner_product_scalar(self: Null, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_right_inner_product_vector(self: Null, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_right_inner_product_bivector(self: Null, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_right_inner_product_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_right_inner_product_oddmultivector(self: Null, other: OddMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_right_inner_product_evenmultivector(self: Null, other: EvenMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_right_inner_product_multivector(self: Null, other: Multivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_right_inner_product_scalar(self: OddMultivector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn oddmultivector_right_inner_product_vector(self: OddMultivector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: 0.0,
    };
}

fn oddmultivector_right_inner_product_bivector(self: OddMultivector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn oddmultivector_right_inner_product_null(self: OddMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_right_inner_product_oddmultivector(self: OddMultivector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: 0.0,
    };
}

fn oddmultivector_right_inner_product_evenmultivector(self: OddMultivector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn oddmultivector_right_inner_product_multivector(self: OddMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
        e01: 0.0,
    };
}

fn evenmultivector_right_inner_product_scalar(self: EvenMultivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: self.e01 * other.s,
    };
}

fn evenmultivector_right_inner_product_vector(self: EvenMultivector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
    };
}

fn evenmultivector_right_inner_product_bivector(self: EvenMultivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.e01 * other.e01,
        e01: 0.0,
    };
}

fn evenmultivector_right_inner_product_null(self: EvenMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn evenmultivector_right_inner_product_oddmultivector(self: EvenMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
    };
}

fn evenmultivector_right_inner_product_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s - self.e01 * other.e01,
        e01: self.e01 * other.s,
    };
}

fn evenmultivector_right_inner_product_multivector(self: EvenMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s - self.e01 * other.e01,
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
        e01: self.e01 * other.s,
    };
}

fn multivector_right_inner_product_scalar(self: Multivector, other: Scalar) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
        e01: self.e01 * other.s,
    };
}

fn multivector_right_inner_product_vector(self: Multivector, other: Vector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
        e01: 0.0,
    };
}

fn multivector_right_inner_product_bivector(self: Multivector, other: Bivector) -> Multivector {
    return Multivector {
        s: -self.e01 * other.e01,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn multivector_right_inner_product_null(self: Multivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn multivector_right_inner_product_oddmultivector(self: Multivector, other: OddMultivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
        e01: 0.0,
    };
}

fn multivector_right_inner_product_evenmultivector(self: Multivector, other: EvenMultivector) -> Multivector {
    return Multivector {
        s: self.s * other.s - self.e01 * other.e01,
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
        e01: self.e01 * other.s,
    };
}

fn multivector_right_inner_product_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s + self.e0 * other.e0 + self.e1 * other.e1 - self.e01 * other.e01,
        e0: self.e0 * other.s + self.e01 * other.e1,
        e1: self.e1 * other.s - self.e01 * other.e0,
        e01: self.e01 * other.s,
    };
}

fn scalar_inner_product_scalar(self: Scalar, other: Scalar) -> Scalar {
    return Scalar {
        s: self.s * other.s,
    };
}

fn scalar_inner_product_vector(self: Scalar, other: Vector) -> Vector {
    return Vector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn scalar_inner_product_bivector(self: Scalar, other: Bivector) -> Bivector {
    return Bivector {
        e01: self.s * other.e01,
    };
}

fn scalar_inner_product_null(self: Scalar, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn scalar_inner_product_oddmultivector(self: Scalar, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn scalar_inner_product_evenmultivector(self: Scalar, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: self.s * other.e01,
    };
}

fn scalar_inner_product_multivector(self: Scalar, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: self.s * other.e0,
        e1: self.s * other.e1,
        e01: self.s * other.e01,
    };
}

fn vector_inner_product_scalar(self: Vector, other: Scalar) -> Vector {
    return Vector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn vector_inner_product_vector(self: Vector, other: Vector) -> Scalar {
    return Scalar {
        s: self.e0 * other.e0 + self.e1 * other.e1,
    };
}

fn vector_inner_product_bivector(self: Vector, other: Bivector) -> Vector {
    return Vector {
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
    };
}

fn vector_inner_product_null(self: Vector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_inner_product_oddmultivector(self: Vector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: 0.0,
    };
}

fn vector_inner_product_evenmultivector(self: Vector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s - self.e1 * other.e01,
        e1: self.e0 * other.e01 + self.e1 * other.s,
    };
}

fn vector_inner_product_multivector(self: Vector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.e0 * other.s - self.e1 * other.e01,
        e1: self.e0 * other.e01 + self.e1 * other.s,
        e01: 0.0,
    };
}

fn bivector_inner_product_scalar(self: Bivector, other: Scalar) -> Bivector {
    return Bivector {
        e01: self.e01 * other.s,
    };
}

fn bivector_inner_product_vector(self: Bivector, other: Vector) -> Vector {
    return Vector {
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
    };
}

fn bivector_inner_product_bivector(self: Bivector, other: Bivector) -> Scalar {
    return Scalar {
        s: -self.e01 * other.e01,
    };
}

fn bivector_inner_product_null(self: Bivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_inner_product_oddmultivector(self: Bivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
    };
}

fn bivector_inner_product_evenmultivector(self: Bivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.e01 * other.e01,
        e01: self.e01 * other.s,
    };
}

fn bivector_inner_product_multivector(self: Bivector, other: Multivector) -> Multivector {
    return Multivector {
        s: -self.e01 * other.e01,
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
        e01: self.e01 * other.s,
    };
}

fn null_inner_product_scalar(self: Null, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_inner_product_vector(self: Null, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_inner_product_bivector(self: Null, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_inner_product_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_inner_product_oddmultivector(self: Null, other: OddMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_inner_product_evenmultivector(self: Null, other: EvenMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_inner_product_multivector(self: Null, other: Multivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_inner_product_scalar(self: OddMultivector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn oddmultivector_inner_product_vector(self: OddMultivector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: 0.0,
    };
}

fn oddmultivector_inner_product_bivector(self: OddMultivector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
    };
}

fn oddmultivector_inner_product_null(self: OddMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_inner_product_oddmultivector(self: OddMultivector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: 0.0,
    };
}

fn oddmultivector_inner_product_evenmultivector(self: OddMultivector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s - self.e1 * other.e01,
        e1: self.e0 * other.e01 + self.e1 * other.s,
    };
}

fn oddmultivector_inner_product_multivector(self: OddMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.e0 * other.s - self.e1 * other.e01,
        e1: self.e0 * other.e01 + self.e1 * other.s,
        e01: 0.0,
    };
}

fn evenmultivector_inner_product_scalar(self: EvenMultivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: self.e01 * other.s,
    };
}

fn evenmultivector_inner_product_vector(self: EvenMultivector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0 + self.e01 * other.e1,
        e1: self.s * other.e1 - self.e01 * other.e0,
    };
}

fn evenmultivector_inner_product_bivector(self: EvenMultivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.e01 * other.e01,
        e01: self.s * other.e01,
    };
}

fn evenmultivector_inner_product_null(self: EvenMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn evenmultivector_inner_product_oddmultivector(self: EvenMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0 + self.e01 * other.e1,
        e1: self.s * other.e1 - self.e01 * other.e0,
    };
}

fn evenmultivector_inner_product_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s - self.e01 * other.e01,
        e01: self.s * other.e01 + self.e01 * other.s,
    };
}

fn evenmultivector_inner_product_multivector(self: EvenMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s - self.e01 * other.e01,
        e0: self.s * other.e0 + self.e01 * other.e1,
        e1: self.s * other.e1 - self.e01 * other.e0,
        e01: self.s * other.e01 + self.e01 * other.s,
    };
}

fn multivector_inner_product_scalar(self: Multivector, other: Scalar) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
        e01: self.e01 * other.s,
    };
}

fn multivector_inner_product_vector(self: Multivector, other: Vector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.s * other.e0 + self.e01 * other.e1,
        e1: self.s * other.e1 - self.e01 * other.e0,
        e01: 0.0,
    };
}

fn multivector_inner_product_bivector(self: Multivector, other: Bivector) -> Multivector {
    return Multivector {
        s: -self.e01 * other.e01,
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
        e01: self.s * other.e01,
    };
}

fn multivector_inner_product_null(self: Multivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn multivector_inner_product_oddmultivector(self: Multivector, other: OddMultivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.s * other.e0 + self.e01 * other.e1,
        e1: self.s * other.e1 - self.e01 * other.e0,
        e01: 0.0,
    };
}

fn multivector_inner_product_evenmultivector(self: Multivector, other: EvenMultivector) -> Multivector {
    return Multivector {
        s: self.s * other.s - self.e01 * other.e01,
        e0: self.e0 * other.s - self.e1 * other.e01,
        e1: self.e0 * other.e01 + self.e1 * other.s,
        e01: self.s * other.e01 + self.e01 * other.s,
    };
}

fn multivector_inner_product_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s + self.e0 * other.e0 + self.e1 * other.e1 - self.e01 * other.e01,
        e0: self.s * other.e0 + self.e0 * other.s - self.e1 * other.e01 + self.e01 * other.e1,
        e1: self.s * other.e1 + self.e0 * other.e01 + self.e1 * other.s - self.e01 * other.e0,
        e01: self.s * other.e01 + self.e01 * other.s,
    };
}

fn scalar_outer_product_scalar(self: Scalar, other: Scalar) -> Scalar {
    return Scalar {
        s: self.s * other.s,
    };
}

fn scalar_outer_product_vector(self: Scalar, other: Vector) -> Vector {
    return Vector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn scalar_outer_product_bivector(self: Scalar, other: Bivector) -> Bivector {
    return Bivector {
        e01: self.s * other.e01,
    };
}

fn scalar_outer_product_null(self: Scalar, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn scalar_outer_product_oddmultivector(self: Scalar, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn scalar_outer_product_evenmultivector(self: Scalar, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: self.s * other.e01,
    };
}

fn scalar_outer_product_multivector(self: Scalar, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: self.s * other.e0,
        e1: self.s * other.e1,
        e01: self.s * other.e01,
    };
}

fn vector_outer_product_scalar(self: Vector, other: Scalar) -> Vector {
    return Vector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn vector_outer_product_vector(self: Vector, other: Vector) -> Bivector {
    return Bivector {
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn vector_outer_product_bivector(self: Vector, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_outer_product_null(self: Vector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_outer_product_oddmultivector(self: Vector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn vector_outer_product_evenmultivector(self: Vector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn vector_outer_product_multivector(self: Vector, other: Multivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn bivector_outer_product_scalar(self: Bivector, other: Scalar) -> Bivector {
    return Bivector {
        e01: self.e01 * other.s,
    };
}

fn bivector_outer_product_vector(self: Bivector, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_outer_product_bivector(self: Bivector, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_outer_product_null(self: Bivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_outer_product_oddmultivector(self: Bivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn bivector_outer_product_evenmultivector(self: Bivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.e01 * other.s,
    };
}

fn bivector_outer_product_multivector(self: Bivector, other: Multivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: 0.0,
        e1: 0.0,
        e01: self.e01 * other.s,
    };
}

fn null_outer_product_scalar(self: Null, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_outer_product_vector(self: Null, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_outer_product_bivector(self: Null, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_outer_product_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_outer_product_oddmultivector(self: Null, other: OddMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_outer_product_evenmultivector(self: Null, other: EvenMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_outer_product_multivector(self: Null, other: Multivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_outer_product_scalar(self: OddMultivector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn oddmultivector_outer_product_vector(self: OddMultivector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn oddmultivector_outer_product_bivector(self: OddMultivector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn oddmultivector_outer_product_null(self: OddMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_outer_product_oddmultivector(self: OddMultivector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn oddmultivector_outer_product_evenmultivector(self: OddMultivector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn oddmultivector_outer_product_multivector(self: OddMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn evenmultivector_outer_product_scalar(self: EvenMultivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: self.e01 * other.s,
    };
}

fn evenmultivector_outer_product_vector(self: EvenMultivector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn evenmultivector_outer_product_bivector(self: EvenMultivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.s * other.e01,
    };
}

fn evenmultivector_outer_product_null(self: EvenMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn evenmultivector_outer_product_oddmultivector(self: EvenMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn evenmultivector_outer_product_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: self.s * other.e01 + self.e01 * other.s,
    };
}

fn evenmultivector_outer_product_multivector(self: EvenMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: self.s * other.e0,
        e1: self.s * other.e1,
        e01: self.s * other.e01 + self.e01 * other.s,
    };
}

fn multivector_outer_product_scalar(self: Multivector, other: Scalar) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
        e01: self.e01 * other.s,
    };
}

fn multivector_outer_product_vector(self: Multivector, other: Vector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: self.s * other.e0,
        e1: self.s * other.e1,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn multivector_outer_product_bivector(self: Multivector, other: Bivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: 0.0,
        e1: 0.0,
        e01: self.s * other.e01,
    };
}

fn multivector_outer_product_null(self: Multivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn multivector_outer_product_oddmultivector(self: Multivector, other: OddMultivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: self.s * other.e0,
        e1: self.s * other.e1,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn multivector_outer_product_evenmultivector(self: Multivector, other: EvenMultivector) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
        e01: self.s * other.e01 + self.e01 * other.s,
    };
}

fn multivector_outer_product_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: self.s * other.e0 + self.e0 * other.s,
        e1: self.s * other.e1 + self.e1 * other.s,
        e01: self.s * other.e01 + self.e0 * other.e1 - self.e1 * other.e0 + self.e01 * other.s,
    };
}

fn scalar_regressive_product_scalar(self: Scalar, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn scalar_regressive_product_vector(self: Scalar, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn scalar_regressive_product_bivector(self: Scalar, other: Bivector) -> Scalar {
    return Scalar {
        s: self.s * other.e01,
    };
}

fn scalar_regressive_product_null(self: Scalar, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn scalar_regressive_product_oddmultivector(self: Scalar, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn scalar_regressive_product_evenmultivector(self: Scalar, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e01,
        e01: 0.0,
    };
}

fn scalar_regressive_product_multivector(self: Scalar, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.e01,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn vector_regressive_product_scalar(self: Vector, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_regressive_product_vector(self: Vector, other: Vector) -> Scalar {
    return Scalar {
        s: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn vector_regressive_product_bivector(self: Vector, other: Bivector) -> Vector {
    return Vector {
        e0: self.e0 * other.e01,
        e1: self.e1 * other.e01,
    };
}

fn vector_regressive_product_null(self: Vector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_regressive_product_oddmultivector(self: Vector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e1 - self.e1 * other.e0,
        e01: 0.0,
    };
}

fn vector_regressive_product_evenmultivector(self: Vector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e01,
        e1: self.e1 * other.e01,
    };
}

fn vector_regressive_product_multivector(self: Vector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e1 - self.e1 * other.e0,
        e0: self.e0 * other.e01,
        e1: self.e1 * other.e01,
        e01: 0.0,
    };
}

fn bivector_regressive_product_scalar(self: Bivector, other: Scalar) -> Scalar {
    return Scalar {
        s: self.e01 * other.s,
    };
}

fn bivector_regressive_product_vector(self: Bivector, other: Vector) -> Vector {
    return Vector {
        e0: self.e01 * other.e0,
        e1: self.e01 * other.e1,
    };
}

fn bivector_regressive_product_bivector(self: Bivector, other: Bivector) -> Bivector {
    return Bivector {
        e01: self.e01 * other.e01,
    };
}

fn bivector_regressive_product_null(self: Bivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_regressive_product_oddmultivector(self: Bivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e01 * other.e0,
        e1: self.e01 * other.e1,
    };
}

fn bivector_regressive_product_evenmultivector(self: Bivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e01 * other.s,
        e01: self.e01 * other.e01,
    };
}

fn bivector_regressive_product_multivector(self: Bivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.e01 * other.s,
        e0: self.e01 * other.e0,
        e1: self.e01 * other.e1,
        e01: self.e01 * other.e01,
    };
}

fn null_regressive_product_scalar(self: Null, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_regressive_product_vector(self: Null, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_regressive_product_bivector(self: Null, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_regressive_product_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_regressive_product_oddmultivector(self: Null, other: OddMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_regressive_product_evenmultivector(self: Null, other: EvenMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_regressive_product_multivector(self: Null, other: Multivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_regressive_product_scalar(self: OddMultivector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn oddmultivector_regressive_product_vector(self: OddMultivector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e1 - self.e1 * other.e0,
        e01: 0.0,
    };
}

fn oddmultivector_regressive_product_bivector(self: OddMultivector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e01,
        e1: self.e1 * other.e01,
    };
}

fn oddmultivector_regressive_product_null(self: OddMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_regressive_product_oddmultivector(self: OddMultivector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e1 - self.e1 * other.e0,
        e01: 0.0,
    };
}

fn oddmultivector_regressive_product_evenmultivector(self: OddMultivector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e01,
        e1: self.e1 * other.e01,
    };
}

fn oddmultivector_regressive_product_multivector(self: OddMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e1 - self.e1 * other.e0,
        e0: self.e0 * other.e01,
        e1: self.e1 * other.e01,
        e01: 0.0,
    };
}

fn evenmultivector_regressive_product_scalar(self: EvenMultivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: self.e01 * other.s,
        e01: 0.0,
    };
}

fn evenmultivector_regressive_product_vector(self: EvenMultivector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.e01 * other.e0,
        e1: self.e01 * other.e1,
    };
}

fn evenmultivector_regressive_product_bivector(self: EvenMultivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e01,
        e01: self.e01 * other.e01,
    };
}

fn evenmultivector_regressive_product_null(self: EvenMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn evenmultivector_regressive_product_oddmultivector(self: EvenMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e01 * other.e0,
        e1: self.e01 * other.e1,
    };
}

fn evenmultivector_regressive_product_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e01 + self.e01 * other.s,
        e01: self.e01 * other.e01,
    };
}

fn evenmultivector_regressive_product_multivector(self: EvenMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.e01 + self.e01 * other.s,
        e0: self.e01 * other.e0,
        e1: self.e01 * other.e1,
        e01: self.e01 * other.e01,
    };
}

fn multivector_regressive_product_scalar(self: Multivector, other: Scalar) -> Multivector {
    return Multivector {
        s: self.e01 * other.s,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn multivector_regressive_product_vector(self: Multivector, other: Vector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e1 - self.e1 * other.e0,
        e0: self.e01 * other.e0,
        e1: self.e01 * other.e1,
        e01: 0.0,
    };
}

fn multivector_regressive_product_bivector(self: Multivector, other: Bivector) -> Multivector {
    return Multivector {
        s: self.s * other.e01,
        e0: self.e0 * other.e01,
        e1: self.e1 * other.e01,
        e01: self.e01 * other.e01,
    };
}

fn multivector_regressive_product_null(self: Multivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn multivector_regressive_product_oddmultivector(self: Multivector, other: OddMultivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e1 - self.e1 * other.e0,
        e0: self.e01 * other.e0,
        e1: self.e01 * other.e1,
        e01: 0.0,
    };
}

fn multivector_regressive_product_evenmultivector(self: Multivector, other: EvenMultivector) -> Multivector {
    return Multivector {
        s: self.s * other.e01 + self.e01 * other.s,
        e0: self.e0 * other.e01,
        e1: self.e1 * other.e01,
        e01: self.e01 * other.e01,
    };
}

fn multivector_regressive_product_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.e01 + self.e0 * other.e1 - self.e1 * other.e0 + self.e01 * other.s,
        e0: self.e0 * other.e01 + self.e01 * other.e0,
        e1: self.e1 * other.e01 + self.e01 * other.e1,
        e01: self.e01 * other.e01,
    };
}

fn scalar_commutator_scalar(self: Scalar, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn scalar_commutator_vector(self: Scalar, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn scalar_commutator_bivector(self: Scalar, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn scalar_commutator_null(self: Scalar, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn scalar_commutator_oddmultivector(self: Scalar, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn scalar_commutator_evenmultivector(self: Scalar, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn scalar_commutator_multivector(self: Scalar, other: Multivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn vector_commutator_scalar(self: Vector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn vector_commutator_vector(self: Vector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn vector_commutator_bivector(self: Vector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
    };
}

fn vector_commutator_null(self: Vector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_commutator_oddmultivector(self: Vector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn vector_commutator_evenmultivector(self: Vector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
    };
}

fn vector_commutator_multivector(self: Vector, other: Multivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn bivector_commutator_scalar(self: Bivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn bivector_commutator_vector(self: Bivector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
    };
}

fn bivector_commutator_bivector(self: Bivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn bivector_commutator_null(self: Bivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_commutator_oddmultivector(self: Bivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
    };
}

fn bivector_commutator_evenmultivector(self: Bivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn bivector_commutator_multivector(self: Bivector, other: Multivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
        e01: 0.0,
    };
}

fn null_commutator_scalar(self: Null, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_commutator_vector(self: Null, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_commutator_bivector(self: Null, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_commutator_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_commutator_oddmultivector(self: Null, other: OddMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_commutator_evenmultivector(self: Null, other: EvenMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_commutator_multivector(self: Null, other: Multivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_commutator_scalar(self: OddMultivector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn oddmultivector_commutator_vector(self: OddMultivector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn oddmultivector_commutator_bivector(self: OddMultivector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
    };
}

fn oddmultivector_commutator_null(self: OddMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_commutator_oddmultivector(self: OddMultivector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn oddmultivector_commutator_evenmultivector(self: OddMultivector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
    };
}

fn oddmultivector_commutator_multivector(self: OddMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn evenmultivector_commutator_scalar(self: EvenMultivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn evenmultivector_commutator_vector(self: EvenMultivector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
    };
}

fn evenmultivector_commutator_bivector(self: EvenMultivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn evenmultivector_commutator_null(self: EvenMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn evenmultivector_commutator_oddmultivector(self: EvenMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
    };
}

fn evenmultivector_commutator_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn evenmultivector_commutator_multivector(self: EvenMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
        e01: 0.0,
    };
}

fn multivector_commutator_scalar(self: Multivector, other: Scalar) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn multivector_commutator_vector(self: Multivector, other: Vector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn multivector_commutator_bivector(self: Multivector, other: Bivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
        e01: 0.0,
    };
}

fn multivector_commutator_null(self: Multivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn multivector_commutator_oddmultivector(self: Multivector, other: OddMultivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: self.e01 * other.e1,
        e1: -self.e01 * other.e0,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn multivector_commutator_evenmultivector(self: Multivector, other: EvenMultivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: -self.e1 * other.e01,
        e1: self.e0 * other.e01,
        e01: 0.0,
    };
}

fn multivector_commutator_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: -self.e1 * other.e01 + self.e01 * other.e1,
        e1: self.e0 * other.e01 - self.e01 * other.e0,
        e01: self.e0 * other.e1 - self.e1 * other.e0,
    };
}

fn scalar_anticommutator_scalar(self: Scalar, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: 0.0,
    };
}

fn scalar_anticommutator_vector(self: Scalar, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn scalar_anticommutator_bivector(self: Scalar, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.s * other.e01,
    };
}

fn scalar_anticommutator_null(self: Scalar, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn scalar_anticommutator_oddmultivector(self: Scalar, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn scalar_anticommutator_evenmultivector(self: Scalar, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: self.s * other.e01,
    };
}

fn scalar_anticommutator_multivector(self: Scalar, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: self.s * other.e0,
        e1: self.s * other.e1,
        e01: self.s * other.e01,
    };
}

fn vector_anticommutator_scalar(self: Vector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn vector_anticommutator_vector(self: Vector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: 0.0,
    };
}

fn vector_anticommutator_bivector(self: Vector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn vector_anticommutator_null(self: Vector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_anticommutator_oddmultivector(self: Vector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: 0.0,
    };
}

fn vector_anticommutator_evenmultivector(self: Vector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn vector_anticommutator_multivector(self: Vector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
        e01: 0.0,
    };
}

fn bivector_anticommutator_scalar(self: Bivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.e01 * other.s,
    };
}

fn bivector_anticommutator_vector(self: Bivector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn bivector_anticommutator_bivector(self: Bivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.e01 * other.e01,
        e01: 0.0,
    };
}

fn bivector_anticommutator_null(self: Bivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_anticommutator_oddmultivector(self: Bivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn bivector_anticommutator_evenmultivector(self: Bivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.e01 * other.e01,
        e01: self.e01 * other.s,
    };
}

fn bivector_anticommutator_multivector(self: Bivector, other: Multivector) -> Multivector {
    return Multivector {
        s: -self.e01 * other.e01,
        e0: 0.0,
        e1: 0.0,
        e01: self.e01 * other.s,
    };
}

fn null_anticommutator_scalar(self: Null, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_anticommutator_vector(self: Null, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_anticommutator_bivector(self: Null, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_anticommutator_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_anticommutator_oddmultivector(self: Null, other: OddMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_anticommutator_evenmultivector(self: Null, other: EvenMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_anticommutator_multivector(self: Null, other: Multivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_anticommutator_scalar(self: OddMultivector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn oddmultivector_anticommutator_vector(self: OddMultivector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: 0.0,
    };
}

fn oddmultivector_anticommutator_bivector(self: OddMultivector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn oddmultivector_anticommutator_null(self: OddMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_anticommutator_oddmultivector(self: OddMultivector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e01: 0.0,
    };
}

fn oddmultivector_anticommutator_evenmultivector(self: OddMultivector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
    };
}

fn oddmultivector_anticommutator_multivector(self: OddMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
        e01: 0.0,
    };
}

fn evenmultivector_anticommutator_scalar(self: EvenMultivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s,
        e01: self.e01 * other.s,
    };
}

fn evenmultivector_anticommutator_vector(self: EvenMultivector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn evenmultivector_anticommutator_bivector(self: EvenMultivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.e01 * other.e01,
        e01: self.s * other.e01,
    };
}

fn evenmultivector_anticommutator_null(self: EvenMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn evenmultivector_anticommutator_oddmultivector(self: EvenMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.s * other.e0,
        e1: self.s * other.e1,
    };
}

fn evenmultivector_anticommutator_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s - self.e01 * other.e01,
        e01: self.s * other.e01 + self.e01 * other.s,
    };
}

fn evenmultivector_anticommutator_multivector(self: EvenMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s - self.e01 * other.e01,
        e0: self.s * other.e0,
        e1: self.s * other.e1,
        e01: self.s * other.e01 + self.e01 * other.s,
    };
}

fn multivector_anticommutator_scalar(self: Multivector, other: Scalar) -> Multivector {
    return Multivector {
        s: self.s * other.s,
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
        e01: self.e01 * other.s,
    };
}

fn multivector_anticommutator_vector(self: Multivector, other: Vector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.s * other.e0,
        e1: self.s * other.e1,
        e01: 0.0,
    };
}

fn multivector_anticommutator_bivector(self: Multivector, other: Bivector) -> Multivector {
    return Multivector {
        s: -self.e01 * other.e01,
        e0: 0.0,
        e1: 0.0,
        e01: self.s * other.e01,
    };
}

fn multivector_anticommutator_null(self: Multivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn multivector_anticommutator_oddmultivector(self: Multivector, other: OddMultivector) -> Multivector {
    return Multivector {
        s: self.e0 * other.e0 + self.e1 * other.e1,
        e0: self.s * other.e0,
        e1: self.s * other.e1,
        e01: 0.0,
    };
}

fn multivector_anticommutator_evenmultivector(self: Multivector, other: EvenMultivector) -> Multivector {
    return Multivector {
        s: self.s * other.s - self.e01 * other.e01,
        e0: self.e0 * other.s,
        e1: self.e1 * other.s,
        e01: self.s * other.e01 + self.e01 * other.s,
    };
}

fn multivector_anticommutator_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s + self.e0 * other.e0 + self.e1 * other.e1 - self.e01 * other.e01,
        e0: self.s * other.e0 + self.e0 * other.s,
        e1: self.s * other.e1 + self.e1 * other.s,
        e01: self.s * other.e01 + self.e01 * other.s,
    };
}

fn scalar_transform_scalar(self: Scalar, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s * other.s,
        e01: 0.0,
    };
}

fn scalar_transform_vector(self: Scalar, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e01: 0.0,
    };
}

fn scalar_transform_bivector(self: Scalar, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e01 * other.e01,
        e01: 0.0,
    };
}

fn scalar_transform_null(self: Scalar, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn scalar_transform_oddmultivector(self: Scalar, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e01: 0.0,
    };
}

fn scalar_transform_evenmultivector(self: Scalar, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s * other.s + self.s * other.e01 * other.e01,
        e01: 0.0,
    };
}

fn scalar_transform_multivector(self: Scalar, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01,
        e0: 2.0 * self.s * other.s * other.e0 + 2.0 * self.s * other.e1 * other.e01,
        e1: 2.0 * self.s * other.s * other.e1 - 2.0 * self.s * other.e0 * other.e01,
        e01: 0.0,
    };
}

fn vector_transform_scalar(self: Vector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s * other.s,
        e1: self.e1 * other.s * other.s,
    };
}

fn vector_transform_vector(self: Vector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 + 2.0 * self.e1 * other.e0 * other.e1,
        e1: 2.0 * self.e0 * other.e0 * other.e1 - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1,
    };
}

fn vector_transform_bivector(self: Vector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e0 * other.e01 * other.e01,
        e1: -self.e1 * other.e01 * other.e01,
    };
}

fn vector_transform_null(self: Vector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_transform_oddmultivector(self: Vector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 + 2.0 * self.e1 * other.e0 * other.e1,
        e1: 2.0 * self.e0 * other.e0 * other.e1 - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1,
    };
}

fn vector_transform_evenmultivector(self: Vector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s * other.s - self.e0 * other.e01 * other.e01 + 2.0 * self.e1 * other.s * other.e01,
        e1: -2.0 * self.e0 * other.s * other.e01 + self.e1 * other.s * other.s - self.e1 * other.e01 * other.e01,
    };
}

fn vector_transform_multivector(self: Vector, other: Multivector) -> Multivector {
    return Multivector {
        s: 2.0 * self.e0 * other.s * other.e0 - 2.0 * self.e0 * other.e1 * other.e01 + 2.0 * self.e1 * other.s * other.e1 + 2.0 * self.e1 * other.e0 * other.e01,
        e0: self.e0 * other.s * other.s + self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 - self.e0 * other.e01 * other.e01 + 2.0 * self.e1 * other.s * other.e01 + 2.0 * self.e1 * other.e0 * other.e1,
        e1: -2.0 * self.e0 * other.s * other.e01 + 2.0 * self.e0 * other.e0 * other.e1 + self.e1 * other.s * other.s - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1 - self.e1 * other.e01 * other.e01,
        e01: 0.0,
    };
}

fn bivector_transform_scalar(self: Bivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.e01 * other.s * other.s,
    };
}

fn bivector_transform_vector(self: Bivector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: -self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1,
    };
}

fn bivector_transform_bivector(self: Bivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.e01 * other.e01 * other.e01,
    };
}

fn bivector_transform_null(self: Bivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_transform_oddmultivector(self: Bivector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: -self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1,
    };
}

fn bivector_transform_evenmultivector(self: Bivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: self.e01 * other.s * other.s + self.e01 * other.e01 * other.e01,
    };
}

fn bivector_transform_multivector(self: Bivector, other: Multivector) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: 0.0,
        e1: 0.0,
        e01: self.e01 * other.s * other.s - self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1 + self.e01 * other.e01 * other.e01,
    };
}

fn null_transform_scalar(self: Null, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_transform_vector(self: Null, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_transform_bivector(self: Null, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_transform_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_transform_oddmultivector(self: Null, other: OddMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_transform_evenmultivector(self: Null, other: EvenMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_transform_multivector(self: Null, other: Multivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_transform_scalar(self: OddMultivector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s * other.s,
        e1: self.e1 * other.s * other.s,
    };
}

fn oddmultivector_transform_vector(self: OddMultivector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 + 2.0 * self.e1 * other.e0 * other.e1,
        e1: 2.0 * self.e0 * other.e0 * other.e1 - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1,
    };
}

fn oddmultivector_transform_bivector(self: OddMultivector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: -self.e0 * other.e01 * other.e01,
        e1: -self.e1 * other.e01 * other.e01,
    };
}

fn oddmultivector_transform_null(self: OddMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_transform_oddmultivector(self: OddMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 + 2.0 * self.e1 * other.e0 * other.e1,
        e1: 2.0 * self.e0 * other.e0 * other.e1 - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1,
    };
}

fn oddmultivector_transform_evenmultivector(self: OddMultivector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s * other.s - self.e0 * other.e01 * other.e01 + 2.0 * self.e1 * other.s * other.e01,
        e1: -2.0 * self.e0 * other.s * other.e01 + self.e1 * other.s * other.s - self.e1 * other.e01 * other.e01,
    };
}

fn oddmultivector_transform_multivector(self: OddMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: 2.0 * self.e0 * other.s * other.e0 - 2.0 * self.e0 * other.e1 * other.e01 + 2.0 * self.e1 * other.s * other.e1 + 2.0 * self.e1 * other.e0 * other.e01,
        e0: self.e0 * other.s * other.s + self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 - self.e0 * other.e01 * other.e01 + 2.0 * self.e1 * other.s * other.e01 + 2.0 * self.e1 * other.e0 * other.e1,
        e1: -2.0 * self.e0 * other.s * other.e01 + 2.0 * self.e0 * other.e0 * other.e1 + self.e1 * other.s * other.s - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1 - self.e1 * other.e01 * other.e01,
        e01: 0.0,
    };
}

fn evenmultivector_transform_scalar(self: EvenMultivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s * other.s,
        e01: self.e01 * other.s * other.s,
    };
}

fn evenmultivector_transform_vector(self: EvenMultivector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e01: -self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1,
    };
}

fn evenmultivector_transform_bivector(self: EvenMultivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e01 * other.e01,
        e01: self.e01 * other.e01 * other.e01,
    };
}

fn evenmultivector_transform_null(self: EvenMultivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn evenmultivector_transform_oddmultivector(self: EvenMultivector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e01: -self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1,
    };
}

fn evenmultivector_transform_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s * other.s + self.s * other.e01 * other.e01,
        e01: self.e01 * other.s * other.s + self.e01 * other.e01 * other.e01,
    };
}

fn evenmultivector_transform_multivector(self: EvenMultivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01,
        e0: 2.0 * self.s * other.s * other.e0 + 2.0 * self.s * other.e1 * other.e01,
        e1: 2.0 * self.s * other.s * other.e1 - 2.0 * self.s * other.e0 * other.e01,
        e01: self.e01 * other.s * other.s - self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1 + self.e01 * other.e01 * other.e01,
    };
}

fn multivector_transform_scalar(self: Multivector, other: Scalar) -> Multivector {
    return Multivector {
        s: self.s * other.s * other.s,
        e0: self.e0 * other.s * other.s,
        e1: self.e1 * other.s * other.s,
        e01: self.e01 * other.s * other.s,
    };
}

fn multivector_transform_vector(self: Multivector, other: Vector) -> Multivector {
    return Multivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e0: self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 + 2.0 * self.e1 * other.e0 * other.e1,
        e1: 2.0 * self.e0 * other.e0 * other.e1 - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1,
        e01: -self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1,
    };
}

fn multivector_transform_bivector(self: Multivector, other: Bivector) -> Multivector {
    return Multivector {
        s: self.s * other.e01 * other.e01,
        e0: -self.e0 * other.e01 * other.e01,
        e1: -self.e1 * other.e01 * other.e01,
        e01: self.e01 * other.e01 * other.e01,
    };
}

fn multivector_transform_null(self: Multivector, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn multivector_transform_oddmultivector(self: Multivector, other: OddMultivector) -> Multivector {
    return Multivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e0: self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 + 2.0 * self.e1 * other.e0 * other.e1,
        e1: 2.0 * self.e0 * other.e0 * other.e1 - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1,
        e01: -self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1,
    };
}

fn multivector_transform_evenmultivector(self: Multivector, other: EvenMultivector) -> Multivector {
    return Multivector {
        s: self.s * other.s * other.s + self.s * other.e01 * other.e01,
        e0: self.e0 * other.s * other.s - self.e0 * other.e01 * other.e01 + 2.0 * self.e1 * other.s * other.e01,
        e1: -2.0 * self.e0 * other.s * other.e01 + self.e1 * other.s * other.s - self.e1 * other.e01 * other.e01,
        e01: self.e01 * other.s * other.s + self.e01 * other.e01 * other.e01,
    };
}

fn multivector_transform_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01 + 2.0 * self.e0 * other.s * other.e0 - 2.0 * self.e0 * other.e1 * other.e01 + 2.0 * self.e1 * other.s * other.e1 + 2.0 * self.e1 * other.e0 * other.e01,
        e0: 2.0 * self.s * other.s * other.e0 + 2.0 * self.s * other.e1 * other.e01 + self.e0 * other.s * other.s + self.e0 * other.e0 * other.e0 - self.e0 * other.e1 * other.e1 - self.e0 * other.e01 * other.e01 + 2.0 * self.e1 * other.s * other.e01 + 2.0 * self.e1 * other.e0 * other.e1,
        e1: 2.0 * self.s * other.s * other.e1 - 2.0 * self.s * other.e0 * other.e01 - 2.0 * self.e0 * other.s * other.e01 + 2.0 * self.e0 * other.e0 * other.e1 + self.e1 * other.s * other.s - self.e1 * other.e0 * other.e0 + self.e1 * other.e1 * other.e1 - self.e1 * other.e01 * other.e01,
        e01: self.e01 * other.s * other.s - self.e01 * other.e0 * other.e0 - self.e01 * other.e1 * other.e1 + self.e01 * other.e01 * other.e01,
    };
}

fn scalar_project_scalar(self: Scalar, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn scalar_project_vector(self: Scalar, other: Vector) -> Scalar {
    return Scalar {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
    };
}

fn scalar_project_bivector(self: Scalar, other: Bivector) -> Scalar {
    return Scalar {
        s: self.s * other.e01 * other.e01,
    };
}

fn scalar_project_null(self: Scalar, other: Null) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn scalar_project_oddmultivector(self: Scalar, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e01: 0.0,
    };
}

fn scalar_project_evenmultivector(self: Scalar, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s * other.s + self.s * other.e01 * other.e01,
        e01: -self.s * other.s * other.e01,
    };
}

fn scalar_project_multivector(self: Scalar, other: Multivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01,
        e01: -self.s * other.s * other.e01,
    };
}

fn vector_project_scalar(self: Vector, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_project_vector(self: Vector, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_project_bivector(self: Vector, other: Bivector) -> Vector {
    return Vector {
        e0: self.e0 * other.e01 * other.e01,
        e1: self.e1 * other.e01 * other.e01,
    };
}

fn vector_project_null(self: Vector, other: Null) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn vector_project_oddmultivector(self: Vector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e0 * other.e0 + self.e1 * other.e0 * other.e1,
        e1: self.e0 * other.e0 * other.e1 + self.e1 * other.e1 * other.e1,
    };
}

fn vector_project_evenmultivector(self: Vector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e01 * other.e01,
        e1: self.e1 * other.e01 * other.e01,
    };
}

fn vector_project_multivector(self: Vector, other: Multivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e0 * other.e0 + self.e0 * other.e01 * other.e01 + self.e1 * other.e0 * other.e1,
        e1: self.e0 * other.e0 * other.e1 + self.e1 * other.e1 * other.e1 + self.e1 * other.e01 * other.e01,
    };
}

fn bivector_project_scalar(self: Bivector, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_project_vector(self: Bivector, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_project_bivector(self: Bivector, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_project_null(self: Bivector, other: Null) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn bivector_project_oddmultivector(self: Bivector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn bivector_project_evenmultivector(self: Bivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.e01 * other.s * other.e01,
        e01: self.e01 * other.e01 * other.e01,
    };
}

fn bivector_project_multivector(self: Bivector, other: Multivector) -> EvenMultivector {
    return EvenMultivector {
        s: -self.e01 * other.s * other.e01,
        e01: self.e01 * other.e01 * other.e01,
    };
}

fn null_project_scalar(self: Null, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_project_vector(self: Null, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_project_bivector(self: Null, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_project_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_project_oddmultivector(self: Null, other: OddMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_project_evenmultivector(self: Null, other: EvenMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_project_multivector(self: Null, other: Multivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_project_scalar(self: OddMultivector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn oddmultivector_project_vector(self: OddMultivector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e0 * other.e0 + self.e1 * other.e0 * other.e1,
        e1: self.e0 * other.e0 * other.e1 + self.e1 * other.e1 * other.e1,
    };
}

fn oddmultivector_project_bivector(self: OddMultivector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e01 * other.e01,
        e1: self.e1 * other.e01 * other.e01,
    };
}

fn oddmultivector_project_null(self: OddMultivector, other: Null) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn oddmultivector_project_oddmultivector(self: OddMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e0 * other.e0 + self.e1 * other.e0 * other.e1,
        e1: self.e0 * other.e0 * other.e1 + self.e1 * other.e1 * other.e1,
    };
}

fn oddmultivector_project_evenmultivector(self: OddMultivector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e01 * other.e01,
        e1: self.e1 * other.e01 * other.e01,
    };
}

fn oddmultivector_project_multivector(self: OddMultivector, other: Multivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e0 * other.e0 + self.e0 * other.e01 * other.e01 + self.e1 * other.e0 * other.e1,
        e1: self.e0 * other.e0 * other.e1 + self.e1 * other.e1 * other.e1 + self.e1 * other.e01 * other.e01,
    };
}

fn evenmultivector_project_scalar(self: EvenMultivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s * other.s,
        e01: 0.0,
    };
}

fn evenmultivector_project_vector(self: EvenMultivector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e01: 0.0,
    };
}

fn evenmultivector_project_bivector(self: EvenMultivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e01 * other.e01,
        e01: self.e01 * other.e01 * other.e01,
    };
}

fn evenmultivector_project_null(self: EvenMultivector, other: Null) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn evenmultivector_project_oddmultivector(self: EvenMultivector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e01: 0.0,
    };
}

fn evenmultivector_project_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s * other.s + self.s * other.e01 * other.e01 - self.e01 * other.s * other.e01,
        e01: -self.s * other.s * other.e01 + self.e01 * other.e01 * other.e01,
    };
}

fn evenmultivector_project_multivector(self: EvenMultivector, other: Multivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01 - self.e01 * other.s * other.e01,
        e01: -self.s * other.s * other.e01 + self.e01 * other.e01 * other.e01,
    };
}

fn multivector_project_scalar(self: Multivector, other: Scalar) -> Multivector {
    return Multivector {
        s: self.s * other.s * other.s,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn multivector_project_vector(self: Multivector, other: Vector) -> Multivector {
    return Multivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e0: self.e0 * other.e0 * other.e0 + self.e1 * other.e0 * other.e1,
        e1: self.e0 * other.e0 * other.e1 + self.e1 * other.e1 * other.e1,
        e01: 0.0,
    };
}

fn multivector_project_bivector(self: Multivector, other: Bivector) -> Multivector {
    return Multivector {
        s: self.s * other.e01 * other.e01,
        e0: self.e0 * other.e01 * other.e01,
        e1: self.e1 * other.e01 * other.e01,
        e01: self.e01 * other.e01 * other.e01,
    };
}

fn multivector_project_null(self: Multivector, other: Null) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn multivector_project_oddmultivector(self: Multivector, other: OddMultivector) -> Multivector {
    return Multivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e0: self.e0 * other.e0 * other.e0 + self.e1 * other.e0 * other.e1,
        e1: self.e0 * other.e0 * other.e1 + self.e1 * other.e1 * other.e1,
        e01: 0.0,
    };
}

fn multivector_project_evenmultivector(self: Multivector, other: EvenMultivector) -> Multivector {
    return Multivector {
        s: self.s * other.s * other.s + self.s * other.e01 * other.e01 - self.e01 * other.s * other.e01,
        e0: self.e0 * other.e01 * other.e01,
        e1: self.e1 * other.e01 * other.e01,
        e01: -self.s * other.s * other.e01 + self.e01 * other.e01 * other.e01,
    };
}

fn multivector_project_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01 + self.e0 * other.s * other.e0 + self.e0 * other.e1 * other.e01 + self.e1 * other.s * other.e1 - self.e1 * other.e0 * other.e01 - self.e01 * other.s * other.e01,
        e0: self.s * other.s * other.e0 + self.s * other.e1 * other.e01 + self.e0 * other.e0 * other.e0 + self.e0 * other.e01 * other.e01 + self.e1 * other.e0 * other.e1 - self.e01 * other.e0 * other.e01,
        e1: self.s * other.s * other.e1 - self.s * other.e0 * other.e01 + self.e0 * other.e0 * other.e1 + self.e1 * other.e1 * other.e1 + self.e1 * other.e01 * other.e01 - self.e01 * other.e1 * other.e01,
        e01: -self.s * other.s * other.e01 - self.e0 * other.e0 * other.e01 - self.e1 * other.e1 * other.e01 + self.e01 * other.e01 * other.e01,
    };
}

fn scalar_reject_scalar(self: Scalar, other: Scalar) -> Scalar {
    return Scalar {
        s: self.s * other.s * other.s,
    };
}

fn scalar_reject_vector(self: Scalar, other: Vector) -> Scalar {
    return Scalar {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
    };
}

fn scalar_reject_bivector(self: Scalar, other: Bivector) -> Scalar {
    return Scalar {
        s: self.s * other.e01 * other.e01,
    };
}

fn scalar_reject_null(self: Scalar, other: Null) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn scalar_reject_oddmultivector(self: Scalar, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e01: 0.0,
    };
}

fn scalar_reject_evenmultivector(self: Scalar, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s * other.s + self.s * other.e01 * other.e01,
        e01: self.s * other.s * other.e01,
    };
}

fn scalar_reject_multivector(self: Scalar, other: Multivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01,
        e01: self.s * other.s * other.e01,
    };
}

fn vector_reject_scalar(self: Vector, other: Scalar) -> Vector {
    return Vector {
        e0: self.e0 * other.s * other.s,
        e1: self.e1 * other.s * other.s,
    };
}

fn vector_reject_vector(self: Vector, other: Vector) -> Vector {
    return Vector {
        e0: self.e0 * other.e1 * other.e1 - self.e1 * other.e0 * other.e1,
        e1: -self.e0 * other.e0 * other.e1 + self.e1 * other.e0 * other.e0,
    };
}

fn vector_reject_bivector(self: Vector, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn vector_reject_null(self: Vector, other: Null) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn vector_reject_oddmultivector(self: Vector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e1 * other.e1 - self.e1 * other.e0 * other.e1,
        e1: -self.e0 * other.e0 * other.e1 + self.e1 * other.e0 * other.e0,
    };
}

fn vector_reject_evenmultivector(self: Vector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s * other.s,
        e1: self.e1 * other.s * other.s,
    };
}

fn vector_reject_multivector(self: Vector, other: Multivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s * other.s + self.e0 * other.e1 * other.e1 - self.e1 * other.e0 * other.e1,
        e1: -self.e0 * other.e0 * other.e1 + self.e1 * other.s * other.s + self.e1 * other.e0 * other.e0,
    };
}

fn bivector_reject_scalar(self: Bivector, other: Scalar) -> Bivector {
    return Bivector {
        e01: self.e01 * other.s * other.s,
    };
}

fn bivector_reject_vector(self: Bivector, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_reject_bivector(self: Bivector, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn bivector_reject_null(self: Bivector, other: Null) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn bivector_reject_oddmultivector(self: Bivector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn bivector_reject_evenmultivector(self: Bivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e01 * other.s * other.e01,
        e01: self.e01 * other.s * other.s,
    };
}

fn bivector_reject_multivector(self: Bivector, other: Multivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.e01 * other.s * other.e01,
        e01: self.e01 * other.s * other.s,
    };
}

fn null_reject_scalar(self: Null, other: Scalar) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_reject_vector(self: Null, other: Vector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_reject_bivector(self: Null, other: Bivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_reject_null(self: Null, other: Null) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_reject_oddmultivector(self: Null, other: OddMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_reject_evenmultivector(self: Null, other: EvenMultivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn null_reject_multivector(self: Null, other: Multivector) -> Null {
    return Null {
        _phantom: 0.0,
    };
}

fn oddmultivector_reject_scalar(self: OddMultivector, other: Scalar) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s * other.s,
        e1: self.e1 * other.s * other.s,
    };
}

fn oddmultivector_reject_vector(self: OddMultivector, other: Vector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e1 * other.e1 - self.e1 * other.e0 * other.e1,
        e1: -self.e0 * other.e0 * other.e1 + self.e1 * other.e0 * other.e0,
    };
}

fn oddmultivector_reject_bivector(self: OddMultivector, other: Bivector) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn oddmultivector_reject_null(self: OddMultivector, other: Null) -> OddMultivector {
    return OddMultivector {
        e0: 0.0,
        e1: 0.0,
    };
}

fn oddmultivector_reject_oddmultivector(self: OddMultivector, other: OddMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.e1 * other.e1 - self.e1 * other.e0 * other.e1,
        e1: -self.e0 * other.e0 * other.e1 + self.e1 * other.e0 * other.e0,
    };
}

fn oddmultivector_reject_evenmultivector(self: OddMultivector, other: EvenMultivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s * other.s,
        e1: self.e1 * other.s * other.s,
    };
}

fn oddmultivector_reject_multivector(self: OddMultivector, other: Multivector) -> OddMultivector {
    return OddMultivector {
        e0: self.e0 * other.s * other.s + self.e0 * other.e1 * other.e1 - self.e1 * other.e0 * other.e1,
        e1: -self.e0 * other.e0 * other.e1 + self.e1 * other.s * other.s + self.e1 * other.e0 * other.e0,
    };
}

fn evenmultivector_reject_scalar(self: EvenMultivector, other: Scalar) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s * other.s,
        e01: self.e01 * other.s * other.s,
    };
}

fn evenmultivector_reject_vector(self: EvenMultivector, other: Vector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e01: 0.0,
    };
}

fn evenmultivector_reject_bivector(self: EvenMultivector, other: Bivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e01 * other.e01,
        e01: 0.0,
    };
}

fn evenmultivector_reject_null(self: EvenMultivector, other: Null) -> EvenMultivector {
    return EvenMultivector {
        s: 0.0,
        e01: 0.0,
    };
}

fn evenmultivector_reject_oddmultivector(self: EvenMultivector, other: OddMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e01: 0.0,
    };
}

fn evenmultivector_reject_evenmultivector(self: EvenMultivector, other: EvenMultivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s * other.s + self.s * other.e01 * other.e01 + self.e01 * other.s * other.e01,
        e01: self.s * other.s * other.e01 + self.e01 * other.s * other.s,
    };
}

fn evenmultivector_reject_multivector(self: EvenMultivector, other: Multivector) -> EvenMultivector {
    return EvenMultivector {
        s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01 + self.e01 * other.s * other.e01,
        e01: self.s * other.s * other.e01 + self.e01 * other.s * other.s,
    };
}

fn multivector_reject_scalar(self: Multivector, other: Scalar) -> Multivector {
    return Multivector {
        s: self.s * other.s * other.s,
        e0: self.e0 * other.s * other.s,
        e1: self.e1 * other.s * other.s,
        e01: self.e01 * other.s * other.s,
    };
}

fn multivector_reject_vector(self: Multivector, other: Vector) -> Multivector {
    return Multivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e0: self.e0 * other.e1 * other.e1 - self.e1 * other.e0 * other.e1,
        e1: -self.e0 * other.e0 * other.e1 + self.e1 * other.e0 * other.e0,
        e01: 0.0,
    };
}

fn multivector_reject_bivector(self: Multivector, other: Bivector) -> Multivector {
    return Multivector {
        s: self.s * other.e01 * other.e01,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn multivector_reject_null(self: Multivector, other: Null) -> Multivector {
    return Multivector {
        s: 0.0,
        e0: 0.0,
        e1: 0.0,
        e01: 0.0,
    };
}

fn multivector_reject_oddmultivector(self: Multivector, other: OddMultivector) -> Multivector {
    return Multivector {
        s: self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1,
        e0: self.e0 * other.e1 * other.e1 - self.e1 * other.e0 * other.e1,
        e1: -self.e0 * other.e0 * other.e1 + self.e1 * other.e0 * other.e0,
        e01: 0.0,
    };
}

fn multivector_reject_evenmultivector(self: Multivector, other: EvenMultivector) -> Multivector {
    return Multivector {
        s: self.s * other.s * other.s + self.s * other.e01 * other.e01 + self.e01 * other.s * other.e01,
        e0: self.e0 * other.s * other.s,
        e1: self.e1 * other.s * other.s,
        e01: self.s * other.s * other.e01 + self.e01 * other.s * other.s,
    };
}

fn multivector_reject_multivector(self: Multivector, other: Multivector) -> Multivector {
    return Multivector {
        s: self.s * other.s * other.s + self.s * other.e0 * other.e0 + self.s * other.e1 * other.e1 + self.s * other.e01 * other.e01 + self.e0 * other.s * other.e0 + self.e0 * other.e1 * other.e01 + self.e1 * other.s * other.e1 - self.e1 * other.e0 * other.e01 + self.e01 * other.s * other.e01,
        e0: self.s * other.s * other.e0 + self.s * other.e1 * other.e01 + self.e0 * other.s * other.s + self.e0 * other.e1 * other.e1 - self.e1 * other.e0 * other.e1 + self.e01 * other.s * other.e1,
        e1: self.s * other.s * other.e1 - self.s * other.e0 * other.e01 - self.e0 * other.e0 * other.e1 + self.e1 * other.s * other.s + self.e1 * other.e0 * other.e0 - self.e01 * other.s * other.e0,
        e01: self.s * other.s * other.e01 + self.e0 * other.s * other.e1 - self.e1 * other.s * other.e0 + self.e01 * other.s * other.s,
    };
}
