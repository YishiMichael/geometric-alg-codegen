use itertools::Itertools;

type Coefficient = i8;

enum Sign {
    POS,
    NEG,
}

impl std::ops::BitXor for Sign {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::POS, Self::POS) => Self::POS,
            (Self::POS, Self::NEG) => Self::NEG,
            (Self::NEG, Self::POS) => Self::NEG,
            (Self::NEG, Self::NEG) => Self::POS,
        }
    }
}

impl From<usize> for Sign {
    fn from(value: usize) -> Self {
        if 1 & value == 0 {
            Self::POS
        } else {
            Self::NEG
        }
    }
}

impl From<Sign> for Coefficient {
    fn from(value: Sign) -> Self {
        match value {
            Sign::POS => 1,
            Sign::NEG => -1,
        }
    }
}

struct BladeInfo {
    name: String,
    intrinsic_sign: Sign,
}

struct SpaceInfo {
    name: String,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Generator<const DIM: usize> {
    generator: usize,
}

impl<const DIM: usize> Generator<DIM> {
    fn iter() -> impl Iterator<Item = Self> + Clone {
        (0..DIM).map(|generator| Self { generator })
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Blade<const DIM: usize> {
    generator_bits: usize,
}

impl<const DIM: usize> std::ops::Not for Blade<DIM> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            generator_bits: (1 << DIM - 1) ^ self.generator_bits,
        }
    }
}

impl<const DIM: usize> std::ops::BitXor for Blade<DIM> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            generator_bits: self.generator_bits ^ rhs.generator_bits,
        }
    }
}

impl<const DIM: usize> std::ops::BitAnd for Blade<DIM> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            generator_bits: self.generator_bits & rhs.generator_bits,
        }
    }
}

impl<const DIM: usize> Blade<DIM> {
    fn iter() -> impl Iterator<Item = Self> + Clone {
        (0..1 << DIM).map(|generator_bits| Self { generator_bits })
    }

    fn iter_generators(self) -> impl Iterator<Item = Generator<DIM>> + Clone {
        Generator::iter()
            .filter(move |generator| self.generator_bits & 1 << generator.generator != 0)
    }

    fn grade(self) -> Grade<DIM> {
        Grade {
            grade: self.generator_bits.count_ones() as usize,
        }
    }

    fn parity(self, other: Self) -> Sign {
        let gray_inv = (0..DIM)
            .map(|generator| self.generator_bits >> generator)
            .fold(0, std::ops::BitXor::bitxor);
        Sign::from((gray_inv >> 1 & other.generator_bits).count_ones() as usize)
    }

    // fn involution(self) -> (Self, Sign) {
    //     (self, Sign::from(self.grade().grade))
    // }

    // fn reverse(self) -> (Self, Sign) {
    //     (self, Sign::from(self.grade().grade >> 1))
    // }

    // fn conjugate(self) -> (Self, Sign) {
    //     (self, Sign::from((self.grade().grade + 1) >> 1))
    // }

    // fn dual(self) -> (Self, Sign) {
    //     (!self, self.parity(!self))
    // }

    // fn inverse_dual(self) -> (Self, Sign) {
    //     (!self, (!self).parity(self))
    // }

    // fn geometric_product(self, other: Self) -> (Self, Option<Sign>) {
    //     (self ^ other, Some(self.parity(other)))
    // }

    // fn scalar_product(self, other: Self) -> (Self, Option<Sign>) {
    //     (
    //         self ^ other,
    //         ((self ^ other).grade() == Grade { grade: 0 }).then(|| self.parity(other)),
    //     )
    // }

    // fn left_inner_product(self, other: Self) -> (Self, Option<Sign>) {
    //     (
    //         self ^ other,
    //         (self.grade() + (self ^ other).grade() == other.grade()).then(|| self.parity(other)),
    //     )
    // }

    // fn right_inner_product(self, other: Self) -> (Self, Option<Sign>) {
    //     (
    //         self ^ other,
    //         (other.grade() + (self ^ other).grade() == self.grade()).then(|| self.parity(other)),
    //     )
    // }

    // fn inner_product(self, other: Self) -> (Self, Option<Sign>) {
    //     (
    //         self ^ other,
    //         (self.grade() + (self ^ other).grade() == other.grade()
    //             || other.grade() + (self ^ other).grade() == self.grade())
    //         .then(|| self.parity(other)),
    //     )
    // }

    // fn outer_product(self, other: Self) -> (Self, Option<Sign>) {
    //     (
    //         self ^ other,
    //         (self.grade() + other.grade() == (self ^ other).grade()).then(|| self.parity(other)),
    //     )
    // }

    // fn regressive_product(self, other: Self) -> (Self, Option<Sign>) {
    //     (
    //         self ^ other,
    //         ((!self).grade() + (!other).grade() == (!(self ^ other)).grade())
    //             .then(|| (!other).parity(!self)),
    //     )
    // }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Grade<const DIM: usize> {
    grade: usize,
}

impl<const DIM: usize> std::ops::Add for Grade<DIM> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Grade {
            grade: self.grade + rhs.grade,
        }
    }
}

impl<const DIM: usize> Grade<DIM> {
    fn iter() -> impl Iterator<Item = Self> + Clone {
        (0..(DIM + 1)).map(|grade| Self { grade })
    }

    fn iter_blades(self) -> impl Iterator<Item = Blade<DIM>> + Clone {
        Blade::iter().filter(move |blade| self == blade.grade())
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Space<const DIM: usize> {
    grade_bits: usize,
}

impl<const DIM: usize> Space<DIM> {
    fn iter() -> impl Iterator<Item = Self> + Clone {
        (0..1 << (DIM + 1)).map(|grade_bits| Self { grade_bits })
    }

    fn iter_grades(self) -> impl Iterator<Item = Grade<DIM>> + Clone {
        Grade::iter().filter(move |grade| self.grade_bits & 1 << grade.grade != 0)
    }

    fn from_grades(iter: impl Iterator<Item = Grade<DIM>>) -> Self {
        Self {grade_bits: iter.map(|grade| 1 << grade.grade).fold(0, std::ops::BitXor::bitxor)}
    }

    // fn iter_blades(self) -> impl Iterator<Item = Blade<DIM>> + Clone {
    //     Blade::iter().filter(move |blade| self.grade_bits & 1 << blade.grade() != 0)
    // }

    // fn from_blades(blades: impl Iterator<Item = Blade<DIM>>) -> Self {
    //     Self {
    //         grade_bits: blades
    //             .map(|blade| 1 << blade.grade())
    //             .fold(0, std::ops::BitOr::bitor),
    //     }
    // }

    // fn contains(self, other: Self) -> bool {
    //     self.grade_bits & other.grade_bits == other.grade_bits
    // }
}

#[derive(Clone, Copy)]
struct GeometricAlgebraRenames {
    blades: &'static [(usize, &'static str)],
    spaces: &'static [(usize, &'static str)],
}

struct GeometricAlgebraBuilder<const DIM: usize> {
    generator_squares: [Coefficient; DIM],
    blades: Vec<String>,
    spaces: Vec<String>,
}

fn generator_squares<V, const DIM: usize>(generator_squares: V) -> GeometricAlgebraBuilder<DIM>
where
    V: Into<[Coefficient; DIM]>,
{
    GeometricAlgebraBuilder {
        generator_squares: generator_squares.into(),
        blades: (0..1 << DIM)
            .map(|blade_index| format!("e{blade_index:0w$b}", w = DIM))
            .collect(),
        spaces: (0..1 << (DIM + 1))
            .map(|space_index| format!("V{space_index:0w$b}", w = DIM + 1))
            .collect(),
    }
}

impl<const DIM: usize> GeometricAlgebraBuilder<DIM> {
    fn rename_blade<T>(mut self, index: usize, name: T) -> Self
    where
        T: Into<String>,
    {
        self.blades[index] = name.into();
        self
    }

    fn rename_space<T>(mut self, index: usize, name: T) -> Self
    where
        T: Into<String>,
    {
        self.spaces[index] = name.into();
        self
    }

    fn rename(mut self, renames: GeometricAlgebraRenames) -> Self {
        renames
            .blades
            .into_iter()
            .for_each(|&(index, name)| self.blades[index] = name.into());
        renames
            .spaces
            .into_iter()
            .for_each(|&(index, name)| self.spaces[index] = name.into());
        self
    }

    fn build(self) -> GeometricAlgebra<DIM> {
        GeometricAlgebra {
            generator_squares: self.generator_squares,
            blades: self
                .blades
                .into_iter()
                .map(|name| match name.strip_prefix('-') {
                    None => BladeInfo {
                        name,
                        intrinsic_sign: Sign::POS,
                    },
                    Some(name) => BladeInfo {
                        name: name.to_string(),
                        intrinsic_sign: Sign::NEG,
                    },
                })
                .collect(),
            spaces: self
                .spaces
                .into_iter()
                .map(|name| SpaceInfo { name })
                .collect(),
        }
    }
}

#[derive(strum::EnumIter, strum::EnumProperty)]
enum Operation {
    /// Additive identity.
    #[strum(props(trait = "Zero"))]
    Zero,
    /// Multiplicative identity.
    #[strum(props(trait = "One"))]
    One,
    /// Grade-0 selection.
    #[strum(props(trait = "SelectGrade0"))]
    SelectGrade0,
    /// Grade-1 selection.
    #[strum(props(trait = "SelectGrade1"))]
    SelectGrade1,
    /// Grade-2 selection.
    #[strum(props(trait = "SelectGrade2"))]
    SelectGrade2,
    /// Grade-3 selection.
    #[strum(props(trait = "SelectGrade3"))]
    SelectGrade3,
    /// Grade-4 selection.
    #[strum(props(trait = "SelectGrade4"))]
    SelectGrade4,
    /// Grade-5 selection.
    #[strum(props(trait = "SelectGrade5"))]
    SelectGrade5,
    /// Grade-6 selection.
    #[strum(props(trait = "SelectGrade6"))]
    SelectGrade6,
    /// Scalar (grade-0) selection.
    #[strum(props(trait = "SelectScalar"))]
    SelectScalar,
    /// Even grade selection.
    #[strum(props(trait = "SelectEven"))]
    SelectEven,
    /// Odd grade selection.
    #[strum(props(trait = "SelectOdd"))]
    SelectOdd,
    /// Vector space injection.
    #[strum(props(trait = "Inject"))]
    Inject,
    /// Vector space surjection.
    #[strum(props(trait = "Surject"))]
    Surject,
    /// Inject scalar into vector space.
    #[strum(props(trait = "Inject"))]
    SInject,
    /// Surject scalar from vector space.
    #[strum(props(trait = "Surject"))]
    SurjectS,
    /// Flip signs by `g % 2 = 1`
    #[strum(props(trait = "Involution"))]
    Involution,
    /// Flip signs by `g % 4 = 2, 3`
    #[strum(props(trait = "Reverse"))]
    Reverse,
    /// Flip signs by `g % 4 = 1, 2`
    #[strum(props(trait = "Conjugate"))]
    Conjugate,
    /// `geometric_product(a, dual(a)) = I`
    #[strum(props(trait = "Dual"))]
    Dual,
    /// `geometric_product(inverse_dual(a), a) = I`
    #[strum(props(trait = "InverseDual"))]
    InverseDual,
    /// `reverse(A) / norm_squared(A)`
    #[strum(props(trait = "Inverse"))]
    Inverse,
    /// `scalar(geometric_product(reverse(A), A))`
    #[strum(props(trait = "NormSquared"))]
    NormSquared,
    /// `sqrt(abs(norm_squared(A)))`
    #[strum(props(trait = "Norm"))]
    Norm,
    /// `A / norm(A)`
    #[strum(props(trait = "Normalized"))]
    Normalized,
    /// In-place normalization.
    #[strum(props(trait = "Normalize"))]
    Normalize,
    /// The geometric product.
    #[strum(props(trait = "GeometricProduct"))]
    GeometricProduct,
    /// Geometric product grade-filtered by `c = 0`
    #[strum(props(trait = "ScalarProduct"))]
    ScalarProduct,
    /// Geometric product grade-filtered by `c = b - a`
    #[strum(props(trait = "LeftInnerProduct"))]
    LeftInnerProduct,
    /// Geometric product grade-filtered by `c = a - b`
    #[strum(props(trait = "RightInnerProduct"))]
    RightInnerProduct,
    /// Geometric product grade-filtered by `c = |a - b|`
    #[strum(props(trait = "InnerProduct"))]
    InnerProduct,
    /// Geometric product grade-filtered by `c = a + b`
    #[strum(props(trait = "OuterProduct"))]
    OuterProduct,
    /// `inverse_dual(outer_product(dual(a), dual(b)))`
    #[strum(props(trait = "RegressiveProduct"))]
    RegressiveProduct,
    /// `(geometric_product(A, B) - geometric_product(B, A)) / 2`
    #[strum(props(trait = "Commutator"))]
    Commutator,
    /// `(geometric_product(A, B) + geometric_product(B, A)) / 2`
    #[strum(props(trait = "Anticommutator"))]
    Anticommutator,
    /// `geometric_product(geometric_product(A, B), reverse(A))`
    #[strum(props(trait = "Transform"))]
    Transform,
    /// `left_inner(left_inner(A, B), inverse(B))`
    #[strum(props(trait = "Project"))]
    Project,
    /// `A - project(A, B)`
    #[strum(props(trait = "Reject"))]
    Reject,
    /// `A + B`
    #[strum(props(trait = "Add"))]
    Add,
    /// `A - B`
    #[strum(props(trait = "Sub"))]
    Sub,
    /// `A * B` (geometric product)
    #[strum(props(trait = "Mul"))]
    Mul,
    /// `A += B`
    #[strum(props(trait = "AddAssign"))]
    AddAssign,
    /// `A -= B`
    #[strum(props(trait = "SubAssign"))]
    SubAssign,
    /// `A *= B` (geometric product)
    #[strum(props(trait = "MulAssign"))]
    MulAssign,
    /// `A * s`
    #[strum(props(trait = "Mul"))]
    MulS,
    /// `A / s`
    #[strum(props(trait = "Div"))]
    DivS,
    /// `A *= s`
    #[strum(props(trait = "MulAssign"))]
    MulSAssign,
    /// `A /= s`
    #[strum(props(trait = "DivAssign"))]
    DivSAssign,
    /// `s * A`
    #[strum(props(trait = "Mul"))]
    SMul,
    /// `s / A = s * inverse(A)`
    #[strum(props(trait = "Div"))]
    SDiv,
}

struct Terms<K, V>(std::collections::BTreeMap<K, V>);

impl<K, V> FromIterator<(K, V)> for Terms<K, V>
where
    K: Copy + std::cmp::Ord,
    V: AbelianGroup,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = std::collections::BTreeMap::new();
        iter.into_iter().for_each(|(key, value)| {
            let slot = map.entry(key).or_insert_with(V::zero);
            slot.add_assign(value);
            if slot.is_zero() {
                map.remove(&key);
            }
        });
        Self(map)
    }
}

impl<K, V> std::ops::Deref for Terms<K, V> {
    type Target = std::collections::BTreeMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

trait AbelianGroup {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
    fn add_assign(&mut self, other: Self);
}

impl AbelianGroup for Coefficient {
    fn zero() -> Self {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn add_assign(&mut self, other: Self) {
        *self += other;
    }
}

impl<K, V> AbelianGroup for Terms<K, V>
where
    K: Copy + std::cmp::Ord,
    V: AbelianGroup,
{
    fn zero() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    fn is_zero(&self) -> bool {
        self.0.is_empty()
    }

    fn add_assign(&mut self, other: Self) {
        other.0.into_iter().for_each(|(key, value)| {
            let slot = self.0.entry(key).or_insert_with(V::zero);
            slot.add_assign(value);
            if slot.is_zero() {
                self.0.remove(&key);
            }
        })
    }
}

struct GeometricAlgebra<const DIM: usize> {
    generator_squares: [Coefficient; DIM],
    blades: Vec<BladeInfo>,
    spaces: Vec<SpaceInfo>,
}

impl<const DIM: usize> std::ops::Index<Blade<DIM>> for GeometricAlgebra<DIM> {
    type Output = BladeInfo;

    fn index(&self, index: Blade<DIM>) -> &Self::Output {
        self.blades.index(index.generator_bits)
    }
}

impl<const DIM: usize> std::ops::Index<Space<DIM>> for GeometricAlgebra<DIM> {
    type Output = SpaceInfo;

    fn index(&self, index: Space<DIM>) -> &Self::Output {
        self.spaces.index(index.grade_bits)
    }
}

impl<const DIM: usize> GeometricAlgebra<DIM> {
    // fn grade(index: BladeIndex) -> u32 {
    //     index.count_ones()
    // }

    // fn gray_code_inv(index: BladeIndex, DIM: u32) -> usize {
    //     (0..DIM)
    //         .map(|grade| index >> grade)
    //         .fold(0, std::ops::BitXor::bitxor)
    // }
    fn product_implementation(&self, grade_filter: fn(Grade<DIM>, Grade<DIM>, Grade<DIM>) -> bool, parity_sign: fn(Blade<DIM>, Blade<DIM>) -> Sign) -> Vec<Implementation<DIM>> {
        Space::iter()
                .zip(Space::iter())
                .map(|(space_0, space_1)| {
                    let terms: Terms<_, Terms<_, _>> = space_0
                        .iter_grades()
                        .cartesian_product(space_1.iter_grades())
                        .flat_map(|(grade_0, grade_1)| {
                            grade_0
                                .iter_blades()
                                .cartesian_product(grade_1.iter_blades())
                                .map(|(blade_0, blade_1)| {
                                    let blade_product = blade_0 ^ blade_1;
                                    (blade_product.grade(), grade_filter(blade_0.grade(), blade_1.grade(), blade_product.grade()).then(|| {
                                        (
                                            blade_product,
                                            Coefficient::from(
                                                parity_sign(blade_0, blade_1)
                                                    ^ self[blade_0].intrinsic_sign
                                                    ^ self[blade_1].intrinsic_sign
                                                    ^ self[blade_product].intrinsic_sign,
                                            ) * (blade_0 & blade_1)
                                                .iter_generators()
                                                .map(|generator| self.generator_squares[generator.generator])
                                                .product::<Coefficient>(),
                                        )
                                    }).into_iter().collect())
                                    
                                })
                        })
                        .collect();
                    let space = Space::from_grades(terms.keys().cloned());
                    Implementation {
                        generic_types: [Type::Space(space_0), Type::Space(space_1)].into(),
                        self_variable: Variable {
                            name: "self",
                            ty: Type::Space(space_0),
                        },
                        arg_variables: [Variable {
                            name: "other",
                            ty: Type::Space(space_1),
                        }]
                        .into(),
                        return_type: Some(Type::Space(space)),
                        statements: [].into(),
                        return_expr: Some(Expression::Struct { space, fields: () }),
                    }
                })
                .collect()
    }

    fn implementations(&self, operation: Operation) -> Vec<Implementation<DIM>> {
        match operation {
            Operation::Zero => Space::iter().map(|space| todo!()).collect(),
            Operation::One => Space::iter().map(|space| todo!()).collect(),
            Operation::SelectGrade0 => Space::iter().map(|space| todo!()).collect(),
            Operation::SelectGrade1 => Space::iter().map(|space| todo!()).collect(),
            Operation::SelectGrade2 => Space::iter().map(|space| todo!()).collect(),
            Operation::SelectGrade3 => Space::iter().map(|space| todo!()).collect(),
            Operation::SelectGrade4 => Space::iter().map(|space| todo!()).collect(),
            Operation::SelectGrade5 => Space::iter().map(|space| todo!()).collect(),
            Operation::SelectGrade6 => Space::iter().map(|space| todo!()).collect(),
            Operation::SelectScalar => Space::iter().map(|space| todo!()).collect(),
            Operation::SelectEven => Space::iter().map(|space| todo!()).collect(),
            Operation::SelectOdd => Space::iter().map(|space| todo!()).collect(),
            Operation::Inject => Space::iter().map(|space| todo!()).collect(),
            Operation::Surject => Space::iter().map(|space| todo!()).collect(),
            Operation::SInject => Space::iter().map(|space| todo!()).collect(),
            Operation::SurjectS => Space::iter().map(|space| todo!()).collect(),
            Operation::Involution => Space::iter().map(|space| todo!()).collect(),
            Operation::Reverse => Space::iter().map(|space| todo!()).collect(),
            Operation::Conjugate => Space::iter().map(|space| todo!()).collect(),
            Operation::Dual => Space::iter().map(|space| todo!()).collect(),
            Operation::InverseDual => Space::iter().map(|space| todo!()).collect(),
            Operation::Inverse => Space::iter().map(|space| todo!()).collect(),
            Operation::NormSquared => Space::iter().map(|space| todo!()).collect(),
            Operation::Norm => Space::iter().map(|space| todo!()).collect(),
            Operation::Normalized => Space::iter().map(|space| todo!()).collect(),
            Operation::Normalize => Space::iter().map(|space| todo!()).collect(),
            Operation::GeometricProduct => ,
            Operation::ScalarProduct => Space::iter().map(|space| todo!()).collect(),
            Operation::LeftInnerProduct => Space::iter().map(|space| todo!()).collect(),
            Operation::RightInnerProduct => Space::iter().map(|space| todo!()).collect(),
            Operation::InnerProduct => Space::iter().map(|space| todo!()).collect(),
            Operation::OuterProduct => Space::iter().map(|space| todo!()).collect(),
            Operation::RegressiveProduct => Space::iter().map(|space| todo!()).collect(),
            Operation::Commutator => Space::iter().map(|space| todo!()).collect(),
            Operation::Anticommutator => Space::iter().map(|space| todo!()).collect(),
            Operation::Transform => Space::iter().map(|space| todo!()).collect(),
            Operation::Project => Space::iter().map(|space| todo!()).collect(),
            Operation::Reject => Space::iter().map(|space| todo!()).collect(),
            Operation::Add => Space::iter().map(|space| todo!()).collect(),
            Operation::Sub => Space::iter().map(|space| todo!()).collect(),
            Operation::Mul => Space::iter().map(|space| todo!()).collect(),
            Operation::AddAssign => Space::iter().map(|space| todo!()).collect(),
            Operation::SubAssign => Space::iter().map(|space| todo!()).collect(),
            Operation::MulAssign => Space::iter().map(|space| todo!()).collect(),
            Operation::MulS => Space::iter().map(|space| todo!()).collect(),
            Operation::DivS => Space::iter().map(|space| todo!()).collect(),
            Operation::MulSAssign => Space::iter().map(|space| todo!()).collect(),
            Operation::DivSAssign => Space::iter().map(|space| todo!()).collect(),
            Operation::SMul => Space::iter().map(|space| todo!()).collect(),
            Operation::SDiv => Space::iter().map(|space| todo!()).collect(),
        }
    }
}

// struct Signature<Generics, Args, ReturnType> {
//     generics: Generics,
//     args: Args,
//     return_type: ReturnType,
// }

// struct Return<Type> {
//     ty: Type,
// }

// trait ReturnType {
//     type Expr;
// }

// impl ReturnType for () {
//     type Expr = ();
// }

// impl<Type> ReturnType for Return<Type> {
//     type Expr = TypedExpr<Type>;
// }

struct Implementation<const DIM: usize> {
    generic_types: Vec<Type<DIM>>,
    self_variable: Variable<DIM>,
    arg_variables: Vec<Variable<DIM>>,
    return_type: Option<Type<DIM>>,
    statements: Vec<Statement<DIM>>,
    return_expr: Option<Expression<DIM>>,
}

// struct AssignmentImpl<Generics, Args> {
//     trait_ident: Rc<str>,
//     generics: Generics,
//     arg_variables: Args,
//     statements: Vec<Statement>,
// }

// trait Operation {
//     const NAME: &'static str;

//     fn implementations(&self, alg: &GeometricAlgebra) -> Vec<Implementation>;
// }

// trait Assignment<Generics, Args> {
//     const NAME: &'static str;

//     type Generics;
//     type Args;

//     fn impls(&self, alg: &GeometricAlgebra) -> Vec<OperationImpl<Self::Generics, Self::Args>>;
// }

// trait Operation<Generics, Vars> {
//     type OutputType: Type;

//     fn generics(&self) ->
//     fn output_type(&self, generics: &Generics) -> Self::OutputType;
//     fn statements(&self, generics: &Generics, vars: &Vars) -> Vec<Statement>;
//     fn expression(&self, generics: &Generics, vars: &Vars) -> Expression;
// }

// trait InplaceOperation<Generics, Vars> {
//     fn statements(&self, generics: &Generics, vars: &Vars) -> Vec<Statement>;
// }

// enum Type {
//     Float,
//     Space(usize),
//     PtrSpace(usize),
// }

#[derive(Clone, Copy, Debug, PartialEq)]
enum Type<const DIM: usize> {
    Atom,
    Space(Space<DIM>),
    SpacePtr(Space<DIM>),
}

// #[derive(Clone, Copy, Debug, PartialEq)]
// struct TypeAtom;
// #[derive(Clone, Copy, Debug, PartialEq)]
// struct TypeSpace(usize);
// #[derive(Clone, Copy, Debug, PartialEq)]
// struct TypeSpacePtr(usize);

// trait TypeTrait: Copy + PartialEq + std::fmt::Debug {}
// impl TypeTrait for TypeAtom {}
// impl TypeTrait for TypeSpace {}
// impl TypeTrait for TypeSpacePtr {}

struct Variable<const DIM: usize> {
    name: &'static str,
    ty: Type<DIM>,
}

enum Expression<const DIM: usize> {
    Variable {
        variable: Variable<DIM>,
    },
    FieldAccess {
        expr: Box<Self>,
        blade: Blade<DIM>,
    },
    FieldAccessPtr {
        expr: Box<Self>,
        blade: Blade<DIM>,
    },
    Literal {
        value: u8,
    },
    Struct {
        space: Space<DIM>,
        fields: Vec<(Blade<DIM>, Self)>,
    },
    Group {
        expr: Box<Self>,
    },
    Neg {
        expr: Box<Self>,
    },
    Add {
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
    Sub {
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
    Mul {
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
    Div {
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
    Call {
        operation: Operation,
        generic_types: Vec<Type<DIM>>,
        self_expr: Box<Self>,
        arg_exprs: Vec<Self>,
    },
}

// type Expression = Box<ExpressionRepr>;

enum Statement<const DIM: usize> {
    Expression {
        expr: Expression<DIM>,
    },
    AddAssign {
        expr: Expression<DIM>,
        blade_index: usize,
        rhs: Expression<DIM>,
    },
    SubAssign {
        expr: Expression<DIM>,
        blade_index: usize,
        rhs: Expression<DIM>,
    },
    MulAssign {
        expr: Expression<DIM>,
        blade_index: usize,
        rhs: Expression<DIM>,
    },
    DivAssign {
        expr: Expression<DIM>,
        blade_index: usize,
        rhs: Expression<DIM>,
    },
}

// type Statement = Box<StatementRepr>;

enum Item<const DIM: usize> {
    Struct {
        space: Space<DIM>,
        fields: Vec<Blade<DIM>>,
    },
    Impl {
        operation: Operation,
        implementation: Implementation<DIM>,
    },
}

const PGA1D_RENAMES: GeometricAlgebraRenames = GeometricAlgebraRenames {
    blades: &[(0b00, "s"), (0b01, "e0"), (0b10, "e1"), (0b11, "e01")],
    spaces: &[
        (0b001, "scalar"),
        (0b010, "point"),
        (0b101, "motor"),
        (0b010, "flector"),
        (0b111, "multivector"),
    ],
};
const PGA2D_RENAMES: GeometricAlgebraRenames = GeometricAlgebraRenames {
    blades: &[
        (0b000, "s"),
        (0b001, "e0"),
        (0b010, "e1"),
        (0b011, "e01"),
        (0b100, "e2"),
        (0b101, "-e20"),
        (0b110, "e12"),
        (0b111, "e012"),
    ],
    spaces: &[
        (0b0001, "scalar"),
        (0b0010, "line"),
        (0b0010, "point"),
        (0b0101, "motor"),
        (0b1010, "flector"),
        (0b1111, "multivector"),
    ],
};
const PGA3D_RENAMES: GeometricAlgebraRenames = GeometricAlgebraRenames {
    blades: &[
        (0b0000, "s"),
        (0b0001, "e0"),
        (0b0010, "e1"),
        (0b0011, "e01"),
        (0b0100, "e2"),
        (0b0101, "e02"),
        (0b0110, "e12"),
        (0b0111, "-e021"),
        (0b1000, "e3"),
        (0b1001, "e03"),
        (0b1010, "-e31"),
        (0b1011, "e013"),
        (0b1100, "e23"),
        (0b1101, "-e032"),
        (0b1110, "e123"),
        (0b1111, "e0123"),
    ],
    spaces: &[
        (0b00001, "scalar"),
        (0b00010, "plane"),
        (0b00100, "line"),
        (0b01000, "point"),
        (0b10101, "motor"),
        (0b01010, "flector"),
        (0b11111, "multivector"),
    ],
};

fn algebras() -> Vec<(&'static str, GeometricAlgebra)> {
    [
        (
            "epga1d",
            generator_squares([1, -1]).rename(PGA1D_RENAMES).build(),
        ),
        (
            "ppga1d",
            generator_squares([1, 0]).rename(PGA1D_RENAMES).build(),
        ),
        (
            "hpga1d",
            generator_squares([1, 1]).rename(PGA1D_RENAMES).build(),
        ),
        (
            "epga2d",
            generator_squares([1, 1, -1]).rename(PGA2D_RENAMES).build(),
        ),
        (
            "ppga2d",
            generator_squares([1, 1, 0]).rename(PGA2D_RENAMES).build(),
        ),
        (
            "hpga2d",
            generator_squares([1, 1, 1]).rename(PGA2D_RENAMES).build(),
        ),
        (
            "epga3d",
            generator_squares([1, 1, 1, -1])
                .rename(PGA3D_RENAMES)
                .build(),
        ),
        (
            "ppga3d",
            generator_squares([1, 1, 1, 0])
                .rename(PGA3D_RENAMES)
                .build(),
        ),
        (
            "hpga3d",
            generator_squares([1, 1, 1, 1])
                .rename(PGA3D_RENAMES)
                .build(),
        ),
    ]
    .into()
}

// struct GeometricAlgebra {
//     name: &'static str,
//     generator_squares: &'static str,
//     signed_blades: &'static str,
//     structures: &'static [Structure],
// }

// struct Structure {
//     ty: &'static str,
//     fields: &'static str,
// }
