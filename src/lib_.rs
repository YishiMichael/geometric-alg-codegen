use itertools::Itertools;

type Coefficient = i8;

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Generator<const DIM: usize> {
    generator: usize,
}

impl<const DIM: usize> Generator<DIM> {
    fn iter() -> impl Iterator<Item = Self> + Clone {
        (0..DIM).map(|generator| Self { generator })
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Blade<const DIM: usize> {
    generator_bits: usize,
}

impl<const DIM: usize> std::ops::Not for Blade<DIM> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            generator_bits: ((1 << DIM) - 1) ^ self.generator_bits,
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

    fn zero() -> Self {
        Self { generator_bits: 0 }
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
}

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

    fn zero() -> Self {
        Self { grade: 0 }
    }

    fn dual(self) -> Self {
        Self {
            grade: DIM - self.grade,
        }
    }
}

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

    fn iter_blades(self) -> impl Iterator<Item = Blade<DIM>> + Clone {
        self.iter_grades().flat_map(|grade| grade.iter_blades())
    }

    fn from_grades(grades: impl Iterator<Item = Grade<DIM>>) -> Self {
        Self {
            grade_bits: grades
                .map(|grade| 1 << grade.grade)
                .fold(0, std::ops::BitOr::bitor),
        }
    }

    fn from_blades(blades: impl Iterator<Item = Blade<DIM>>) -> Self {
        Self::from_grades(blades.map(|blade| blade.grade()))
    }

    fn contains(self, blade: Blade<DIM>) -> bool {
        self.grade_bits & 1 << blade.grade().grade != 0
    }
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

enum BuiltinUnaryOperation {
    Abs,
    Sqrt,
}

macro_rules! name {
    ($(
        $(#[$meta:meta])*
        struct $ident:ident;
    )*) => {$(
        $(#[$meta])*
        struct $ident;

        impl TraitNames for $ident {
            const TRAIT_NAME: &'static str = stringify!($ident);
            const METHOD_NAME: &'static str = stringify!($ident); // TODO
        }
    )*};
}

name! {
/// Additive identity.
struct Zero;
/// Multiplicative identity.
struct One;
/// Grade selection.
struct Select;
/// Flip signs by `g % 2 = 1`
struct GradeInvolution;
/// Flip signs by `g % 4 = 2, 3`
struct Reverse;
/// Flip signs by `g % 4 = 1, 2`
struct Conjugate;
/// `geometric_product(a, dual(a)) = I`
struct Dual;
/// `geometric_product(inverse_dual(a), a) = I`
struct Undual;
/// `scalar(geometric_product(reverse(A), A))`
struct NormSquared;
/// `sqrt(abs(norm_squared(A)))`
struct Norm;
/// `A / norm(A)`
struct Normalized;
/// `reverse(A) / norm_squared(A)`
struct Inverse;
/// The geometric product.
struct GeometricProduct;
/// Geometric product grade-filtered by `c = 0`
struct ScalarProduct;
/// Geometric product grade-filtered by `c = b - a`
struct LeftInnerProduct;
/// Geometric product grade-filtered by `c = a - b`
struct RightInnerProduct;
/// Geometric product grade-filtered by `c = |a - b|`
struct InnerProduct;
/// Geometric product grade-filtered by `c = a + b`
struct OuterProduct;
/// `inverse_dual(outer_product(dual(a), dual(b)))`
struct RegressiveProduct;
/// `(geometric_product(A, B) - geometric_product(B, A)) / 2`
struct Commutator;
/// `(geometric_product(A, B) + geometric_product(B, A)) / 2`
struct Anticommutator;
/// `geometric_product(geometric_product(A, B), reverse(A))`
struct Transform;
/// `left_inner(left_inner(A, B), inverse(B))`
struct Project;
/// `right_inner(outer(A, B), inverse(B))`
struct Reject;
/// `-A`
struct Neg;
/// `A + B`
struct Add;
/// `A - B`
struct Sub;
/// `A * s`, `s * A`, `A * B` (geometric product)
struct Mul;
/// `A / s`, `s / A = s * inverse(A)`
struct Div;
}

impl Operation<1, 0> for Zero {
    fn signature(&self) -> ExprSignature<1, 0> {
        ExprSignature {
            param_pretypes: [],
            return_pretype: Some(Pretype::Binding {
                binding: Binding::new(0),
            }),
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        _geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<1, 0, DIM>> {
        Space::iter()
            .map(|space_0| OperationImpl {
                type_params: [Type::Space(space_0)],
                return_type: Type::Space(space_0),
                return_expr: Expr::structure(space_0, |_| Expr::literal(0)),
            })
            .collect()
    }
}

impl Operation<1, 0> for One {
    fn signature(&self) -> ExprSignature<1, 0> {
        ExprSignature {
            param_pretypes: [],
            return_pretype: Some(Pretype::Binding {
                binding: Binding::new(0),
            }),
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        _geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<1, 0, DIM>> {
        Space::iter()
            .map(|space_0| OperationImpl {
                type_params: [Type::Space(space_0)],
                return_type: Type::Space(space_0),
                return_expr: Expr::structure(space_0, |blade| {
                    if blade == Blade::zero() {
                        Expr::literal(1)
                    } else {
                        Expr::literal(0)
                    }
                }),
            })
            .collect()
    }
}

impl Operation<2, 1> for Select {
    fn signature(&self) -> ExprSignature<2, 1> {
        ExprSignature {
            param_pretypes: [Pretype::Binding {
                binding: Binding::new(0),
            }],
            return_pretype: Some(Pretype::Binding {
                binding: Binding::new(1),
            }),
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        _geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<2, 1, DIM>> {
        Space::iter()
            .cartesian_product(Space::iter())
            .map(|(space_0, space_1)| OperationImpl {
                type_params: [space_0, space_1].map(Type::Space),
                return_type: Type::Space(space_1),
                return_expr: Expr::structure(space_1, |blade| {
                    if space_0.contains(blade) {
                        Expr::variable(Binding::new(0)).field(blade)
                    } else {
                        Expr::literal(0)
                    }
                }),
            })
            .chain(Space::iter().map(|space_0| OperationImpl {
                type_params: [Type::Space(space_0), Type::Atom],
                return_type: Type::Atom,
                return_expr: if space_0.contains(Blade::zero()) {
                    Expr::variable(Binding::new(0)).field(Blade::zero())
                } else {
                    Expr::literal(0)
                },
            }))
            .chain(Space::iter().map(|space_1| OperationImpl {
                type_params: [Type::Atom, Type::Space(space_1)],
                return_type: Type::Space(space_1),
                return_expr: Expr::structure(space_1, |blade| {
                    if blade == Blade::zero() {
                        Expr::variable(Binding::new(0))
                    } else {
                        Expr::literal(0)
                    }
                }),
            }))
            .collect()
    }
}

trait InvolutionOperation {
    fn reindex<const DIM: usize>(blades: [Blade<DIM>; 1]) -> [Blade<DIM>; 1] {
        blades
    }

    fn term_sign<const DIM: usize>(_blades: [Blade<DIM>; 1]) -> Sign {
        Sign::POS
    }
}

impl<Op> Operation<1, 1> for Op
where
    Op: InvolutionOperation,
{
    fn signature(&self) -> ExprSignature<1, 1> {
        ExprSignature {
            param_pretypes: [Pretype::Binding {
                binding: Binding::new(0),
            }],
            return_pretype: None,
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<1, 1, DIM>> {
        let multinomial = geometric_algebra.multinomial::<1, 1>(
            [Binding::new(0)],
            Op::reindex,
            |_| true,
            Op::term_sign,
        );
        Space::iter()
            .map(move |space_0| {
                let (space, return_expr) = multinomial.space_expr([space_0]);
                OperationImpl {
                    type_params: [space_0].map(Type::Space),
                    return_type: Type::Space(space),
                    return_expr,
                }
            })
            .collect()
    }
}

impl InvolutionOperation for GradeInvolution {
    fn term_sign<const DIM: usize>([blade]: [Blade<DIM>; 1]) -> Sign {
        Sign::from(blade.grade().grade)
    }
}

impl InvolutionOperation for Reverse {
    fn term_sign<const DIM: usize>([blade]: [Blade<DIM>; 1]) -> Sign {
        Sign::from(blade.grade().grade >> 1)
    }
}

impl InvolutionOperation for Conjugate {
    fn term_sign<const DIM: usize>([blade]: [Blade<DIM>; 1]) -> Sign {
        Sign::from((blade.grade().grade + 1) >> 1)
    }
}

impl InvolutionOperation for Dual {
    fn reindex<const DIM: usize>([blade]: [Blade<DIM>; 1]) -> [Blade<DIM>; 1] {
        [!blade]
    }

    fn term_sign<const DIM: usize>([blade]: [Blade<DIM>; 1]) -> Sign {
        blade.parity(!blade)
    }
}

impl InvolutionOperation for Undual {
    fn reindex<const DIM: usize>([blade]: [Blade<DIM>; 1]) -> [Blade<DIM>; 1] {
        [!blade]
    }

    fn term_sign<const DIM: usize>([blade]: [Blade<DIM>; 1]) -> Sign {
        (!blade).parity(blade)
    }
}
impl Operation<1, 1> for NormSquared {
    fn signature(&self) -> ExprSignature<1, 1> {
        ExprSignature {
            param_pretypes: [Pretype::Binding {
                binding: Binding::new(0),
            }],
            return_pretype: Some(Pretype::Atom),
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<1, 1, DIM>> {
        let multinomial = geometric_algebra.multinomial::<1, 2>(
            [Binding::new(0), Binding::new(0)],
            std::convert::identity,
            |_| true,
            |_| Sign::POS,
        );
        Space::iter()
            .map(move |space_0| {
                let (space, return_expr) = multinomial.space_expr([space_0]);
                OperationImpl {
                    type_params: [space_0].map(Type::Space),
                    return_type: Type::Atom,
                    return_expr: if space.contains(Blade::zero()) {
                        return_expr.field(Blade::zero())
                    } else {
                        Expr::literal(0)
                    },
                }
            })
            .collect()
    }
}

impl Operation<1, 1> for Norm {
    fn signature(&self) -> ExprSignature<1, 1> {
        ExprSignature {
            param_pretypes: [Pretype::Binding {
                binding: Binding::new(0),
            }],
            return_pretype: Some(Pretype::Atom),
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        _geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<1, 1, DIM>> {
        Space::iter()
            .map(|space_0| OperationImpl {
                type_params: [space_0].map(Type::Space),
                return_type: Type::Atom,
                return_expr: Expr::variable(Binding::new(0))
                    .call(NormSquared, [Type::Space(space_0)], [])
                    .call_builtin_unary(BuiltinUnaryOperation::Abs)
                    .call_builtin_unary(BuiltinUnaryOperation::Sqrt),
            })
            .collect()
    }
}

impl Operation<1, 1> for Normalized {
    fn signature(&self) -> ExprSignature<1, 1> {
        ExprSignature {
            param_pretypes: [Pretype::Binding {
                binding: Binding::new(0),
            }],
            return_pretype: Some(Pretype::Binding {
                binding: Binding::new(0),
            }),
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        _geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<1, 1, DIM>> {
        Space::iter()
            .map(|space_0| OperationImpl {
                type_params: [space_0].map(Type::Space),
                return_type: Type::Atom,
                return_expr: Expr::variable(Binding::new(0)).call(
                    Div,
                    [Type::Space(space_0), Type::Atom],
                    [Expr::variable(Binding::new(0)).call(Norm, [Type::Space(space_0)], [])],
                ),
            })
            .collect()
    }
}

impl Operation<1, 1> for Inverse {
    fn signature(&self) -> ExprSignature<1, 1> {
        ExprSignature {
            param_pretypes: [Pretype::Binding {
                binding: Binding::new(0),
            }],
            return_pretype: Some(Pretype::Binding {
                binding: Binding::new(0),
            }),
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        _geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<1, 1, DIM>> {
        Space::iter()
            .map(|space_0| OperationImpl {
                type_params: [space_0].map(Type::Space),
                return_type: Type::Space(space_0),
                return_expr: Expr::variable(Binding::new(0)).call(
                    Div,
                    [Type::Space(space_0), Type::Atom],
                    [
                        Expr::variable(Binding::new(0)).call(
                            NormSquared,
                            [Type::Space(space_0)],
                            [],
                        ),
                    ],
                ),
            })
            .collect()
    }
}

trait ProductOperation {
    fn term_filter<const DIM: usize>(_blades: [Blade<DIM>; 2]) -> bool {
        true
    }

    // rustfmt fails here?
    fn term_sign<const DIM: usize>(blades: [Blade<DIM>; 2]) -> Sign {
        blades[0].parity(blades[1])
    }
}

impl<Op> Operation<2, 2> for Op
where
    Op: ProductOperation,
{
    fn signature(&self) -> ExprSignature<2, 2> {
        ExprSignature {
            param_pretypes: [
                Pretype::Binding {
                    binding: Binding::new(0),
                },
                Pretype::Binding {
                    binding: Binding::new(1),
                },
            ],
            return_pretype: None,
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<2, 2, DIM>> {
        let multinomial = geometric_algebra.multinomial::<2, 2>(
            [Binding::new(0), Binding::new(1)],
            std::convert::identity,
            Op::term_filter,
            Op::term_sign,
        );
        Space::iter()
            .cartesian_product(Space::iter())
            .map(move |(space_0, space_1)| {
                let (space, return_expr) = multinomial.space_expr([space_0, space_1]);
                OperationImpl {
                    type_params: [space_0, space_1].map(Type::Space),
                    return_type: Type::Space(space),
                    return_expr,
                }
            })
            .collect()
    }
}

impl ProductOperation for GeometricProduct {}

impl ProductOperation for ScalarProduct {
    fn term_filter<const DIM: usize>([blade_0, blade_1]: [Blade<DIM>; 2]) -> bool {
        blade_0 ^ blade_1 == Blade::zero()
    }
}

impl ProductOperation for LeftInnerProduct {
    fn term_filter<const DIM: usize>([blade_0, blade_1]: [Blade<DIM>; 2]) -> bool {
        blade_0 & blade_1 == blade_0
    }
}

impl ProductOperation for RightInnerProduct {
    fn term_filter<const DIM: usize>([blade_0, blade_1]: [Blade<DIM>; 2]) -> bool {
        blade_0 & blade_1 == blade_1
    }
}

impl ProductOperation for InnerProduct {
    fn term_filter<const DIM: usize>([blade_0, blade_1]: [Blade<DIM>; 2]) -> bool {
        blade_0 & blade_1 == blade_0 || blade_0 & blade_1 == blade_1
    }
}

impl ProductOperation for OuterProduct {
    fn term_filter<const DIM: usize>([blade_0, blade_1]: [Blade<DIM>; 2]) -> bool {
        blade_0 & blade_1 == Blade::zero()
    }
}

impl ProductOperation for RegressiveProduct {
    fn term_filter<const DIM: usize>([blade_0, blade_1]: [Blade<DIM>; 2]) -> bool {
        !blade_0 & !blade_1 == Blade::zero()
    }

    fn term_sign<const DIM: usize>([blade_0, blade_1]: [Blade<DIM>; 2]) -> Sign {
        (!blade_1).parity(!blade_0)
    }
}

impl ProductOperation for Commutator {
    fn term_filter<const DIM: usize>([blade_0, blade_1]: [Blade<DIM>; 2]) -> bool {
        blade_0.grade().grade & blade_1.grade().grade & 1 != 0
    }
}

impl ProductOperation for Anticommutator {
    fn term_filter<const DIM: usize>([blade_0, blade_1]: [Blade<DIM>; 2]) -> bool {
        blade_0.grade().grade & blade_1.grade().grade & 1 == 0
    }
}

trait CompoundProductOperation {
    fn prototype() -> [Binding<2>; 3];

    fn term_filter<const DIM: usize>(_blades: [Blade<DIM>; 3]) -> bool {
        true
    }

    fn term_sign<const DIM: usize>(_blades: [Blade<DIM>; 3]) -> Sign {
        Sign::POS
    }

    fn wrap_expr<const DIM: usize>(
        _param_types: [Type<DIM>; 2],
        _return_type: Type<DIM>,
        return_expr: Expr<2, DIM>,
    ) -> Expr<2, DIM> {
        return_expr
    }
}

impl<Op> Operation<2, 2> for Op
where
    Op: CompoundProductOperation,
{
    fn signature(&self) -> ExprSignature<2, 2> {
        ExprSignature {
            param_pretypes: [
                Pretype::Binding {
                    binding: Binding::new(0),
                },
                Pretype::Binding {
                    binding: Binding::new(1),
                },
            ],
            return_pretype: None,
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<2, 2, DIM>> {
        let multinomial = geometric_algebra.multinomial::<2, 3>(
            Op::prototype(),
            std::convert::identity,
            Op::term_filter,
            Op::term_sign,
        );
        Space::iter()
            .cartesian_product(Space::iter())
            .map(move |(space_0, space_1)| {
                let (space, return_expr) = multinomial.space_expr([space_0, space_1]);
                let type_params = [space_0, space_1].map(Type::Space);
                let return_type = Type::Space(space);
                let return_expr = Op::wrap_expr(type_params, return_type, return_expr);
                OperationImpl {
                    type_params,
                    return_type,
                    return_expr,
                }
            })
            .collect()
    }
}

impl CompoundProductOperation for Transform {
    fn prototype() -> [Binding<2>; 3] {
        [Binding::new(0), Binding::new(1), Binding::new(0)]
    }

    fn term_sign<const DIM: usize>([blade_0, blade_1, blade_2]: [Blade<DIM>; 3]) -> Sign {
        blade_0.parity(blade_1)
            ^ blade_0.parity(blade_2)
            ^ blade_1.parity(blade_2)
            ^ Sign::from(blade_2.grade().grade >> 1)
    }
}

impl CompoundProductOperation for Project {
    fn prototype() -> [Binding<2>; 3] {
        [Binding::new(0), Binding::new(1), Binding::new(1)]
    }

    fn term_filter<const DIM: usize>([blade_0, blade_1, blade_2]: [Blade<DIM>; 3]) -> bool {
        blade_0 & blade_1 == blade_0 && (blade_0 ^ blade_1) & blade_2 == blade_0 ^ blade_1
        // TODO: check
    }

    fn term_sign<const DIM: usize>([blade_0, blade_1, blade_2]: [Blade<DIM>; 3]) -> Sign {
        blade_0.parity(blade_1)
            ^ blade_0.parity(blade_2)
            ^ blade_1.parity(blade_2)
            ^ Sign::from(blade_2.grade().grade >> 1)
    }

    fn wrap_expr<const DIM: usize>(
        param_types: [Type<DIM>; 2],
        return_type: Type<DIM>,
        return_expr: Expr<2, DIM>,
    ) -> Expr<2, DIM> {
        return_expr.call(
            Div,
            [return_type, Type::Atom],
            [Expr::variable(Binding::new(1)).call(NormSquared, [param_types[1]], [])],
        )
    }
}

impl CompoundProductOperation for Reject {
    fn prototype() -> [Binding<2>; 3] {
        [Binding::new(0), Binding::new(1), Binding::new(1)]
    }

    fn term_filter<const DIM: usize>([blade_0, blade_1, blade_2]: [Blade<DIM>; 3]) -> bool {
        blade_0 & blade_1 == Blade::zero() && (blade_0 ^ blade_1) & blade_2 == blade_2
        // TODO: check
    }

    fn term_sign<const DIM: usize>([blade_0, blade_1, blade_2]: [Blade<DIM>; 3]) -> Sign {
        blade_0.parity(blade_1)
            ^ blade_0.parity(blade_2)
            ^ blade_1.parity(blade_2)
            ^ Sign::from(blade_2.grade().grade >> 1)
    }

    fn wrap_expr<const DIM: usize>(
        param_types: [Type<DIM>; 2],
        return_type: Type<DIM>,
        return_expr: Expr<2, DIM>,
    ) -> Expr<2, DIM> {
        return_expr.call(
            Div,
            [return_type, Type::Atom],
            [Expr::variable(Binding::new(1)).call(NormSquared, [param_types[1]], [])],
        )
    }
}

impl Operation<1, 1> for Neg {
    fn signature(&self) -> ExprSignature<1, 1> {
        ExprSignature {
            param_pretypes: [Pretype::Binding {
                binding: Binding::new(0),
            }],
            return_pretype: None,
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        _geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<1, 1, DIM>> {
        Space::iter()
            .map(|space_0| OperationImpl {
                type_params: [space_0].map(Type::Space),
                return_type: Type::Space(space_0),
                return_expr: Expr::structure(space_0, |blade| {
                    -Expr::variable(Binding::new(0)).field(blade)
                }),
            })
            .collect()
    }
}

impl Operation<2, 2> for Add {
    fn signature(&self) -> ExprSignature<2, 2> {
        ExprSignature {
            param_pretypes: [
                Pretype::Binding {
                    binding: Binding::new(0),
                },
                Pretype::Binding {
                    binding: Binding::new(1),
                },
            ],
            return_pretype: None,
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        _geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<2, 2, DIM>> {
        Space::iter()
            .cartesian_product(Space::iter())
            .map(|(space_0, space_1)| {
                let space = Space::from_grades(space_0.iter_grades().chain(space_1.iter_grades()));
                OperationImpl {
                    type_params: [space_0, space_1].map(Type::Space),
                    return_type: Type::Space(space),
                    return_expr: Expr::structure(space, |blade| {
                        match (space_0.contains(blade), space_1.contains(blade)) {
                            (true, true) => {
                                Expr::variable(Binding::new(0)).field(blade)
                                    + Expr::variable(Binding::new(1)).field(blade)
                            }
                            (true, false) => Expr::variable(Binding::new(0)).field(blade),
                            (false, true) => Expr::variable(Binding::new(1)).field(blade),
                            (false, false) => Expr::literal(0),
                        }
                    }),
                }
            })
            .collect()
    }
}

impl Operation<2, 2> for Sub {
    fn signature(&self) -> ExprSignature<2, 2> {
        ExprSignature {
            param_pretypes: [
                Pretype::Binding {
                    binding: Binding::new(0),
                },
                Pretype::Binding {
                    binding: Binding::new(1),
                },
            ],
            return_pretype: None,
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        _geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<2, 2, DIM>> {
        Space::iter()
            .cartesian_product(Space::iter())
            .map(|(space_0, space_1)| {
                let space = Space::from_grades(space_0.iter_grades().chain(space_1.iter_grades()));
                OperationImpl {
                    type_params: [space_0, space_1].map(Type::Space),
                    return_type: Type::Space(space),
                    return_expr: Expr::structure(space, |blade| {
                        match (space_0.contains(blade), space_1.contains(blade)) {
                            (true, true) => {
                                Expr::variable(Binding::new(0)).field(blade)
                                    - Expr::variable(Binding::new(1)).field(blade)
                            }
                            (true, false) => Expr::variable(Binding::new(0)).field(blade),
                            (false, true) => -Expr::variable(Binding::new(1)).field(blade),
                            (false, false) => Expr::literal(0),
                        }
                    }),
                }
            })
            .collect()
    }
}

impl Operation<2, 2> for Mul {
    fn signature(&self) -> ExprSignature<2, 2> {
        ExprSignature {
            param_pretypes: [
                Pretype::Binding {
                    binding: Binding::new(0),
                },
                Pretype::Binding {
                    binding: Binding::new(1),
                },
            ],
            return_pretype: None,
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<2, 2, DIM>> {
        let multinomial = geometric_algebra.multinomial::<2, 2>(
            [Binding::new(0), Binding::new(1)],
            std::convert::identity,
            |_| true,
            |[blade_0, blade_1]| blade_0.parity(blade_1),
        );
        Space::iter()
            .cartesian_product(Space::iter())
            .map(move |(space_0, space_1)| {
                let (space, return_expr) = multinomial.space_expr([space_0, space_1]);
                OperationImpl {
                    type_params: [space_0, space_1].map(Type::Space),
                    return_type: Type::Space(space),
                    return_expr,
                }
            })
            .chain(Space::iter().map(|space_0| OperationImpl {
                type_params: [Type::Space(space_0), Type::Atom],
                return_type: Type::Space(space_0),
                return_expr: Expr::structure(space_0, |blade| {
                    Expr::variable(Binding::new(0)).field(blade) * Expr::variable(Binding::new(1))
                }),
            }))
            .chain(Space::iter().map(|space_1| OperationImpl {
                type_params: [Type::Atom, Type::Space(space_1)],
                return_type: Type::Space(space_1),
                return_expr: Expr::structure(space_1, |blade| {
                    Expr::variable(Binding::new(0)) * Expr::variable(Binding::new(1)).field(blade)
                }),
            }))
            .collect()
    }
}

impl Operation<2, 2> for Div {
    fn signature(&self) -> ExprSignature<2, 2> {
        ExprSignature {
            param_pretypes: [
                Pretype::Binding {
                    binding: Binding::new(0),
                },
                Pretype::Binding {
                    binding: Binding::new(1),
                },
            ],
            return_pretype: None,
        }
    }

    fn implementations<const DIM: usize>(
        &self,
        _geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<2, 2, DIM>> {
        Space::iter()
            .map(|space_0| OperationImpl {
                type_params: [Type::Space(space_0), Type::Atom],
                return_type: Type::Space(space_0),
                return_expr: Expr::structure(space_0, |blade| {
                    Expr::variable(Binding::new(0)).field(blade) / Expr::variable(Binding::new(1))
                }),
            })
            .chain(Space::iter().map(|space_1| OperationImpl {
                type_params: [Type::Atom, Type::Space(space_1)],
                return_type: Type::Space(space_1),
                return_expr: Expr::variable(Binding::new(0)).call(
                    Mul,
                    [Type::Atom, Type::Space(space_1)],
                    [Expr::variable(Binding::new(1)).call(Inverse, [Type::Space(space_1)], [])],
                ),
            }))
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Binding<const BOUND: usize>(usize);

impl<const BOUND: usize> Binding<BOUND> {
    fn new(index: usize) -> Self {
        assert!(index < BOUND);
        Self(index)
    }
}

impl<T, const BOUND: usize> std::ops::Index<&Binding<BOUND>> for [T; BOUND] {
    type Output = T;

    fn index(&self, index: &Binding<BOUND>) -> &Self::Output {
        &self[index.0]
    }
}

impl<T, const BOUND: usize> std::ops::Index<Binding<BOUND>> for [T; BOUND] {
    type Output = T;

    fn index(&self, index: Binding<BOUND>) -> &Self::Output {
        &self[index.0]
    }
}

impl<const BOUND: usize> std::ops::Deref for Binding<BOUND> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Multinomial<const PARAMS: usize, const DEGREE: usize, const DIM: usize>(
    std::collections::BTreeMap<
        Blade<DIM>,
        std::collections::BTreeMap<[(Binding<PARAMS>, Blade<DIM>); DEGREE], Coefficient>,
    >,
);

impl<const PARAMS: usize, const DEGREE: usize, const DIM: usize>
    FromIterator<(
        Blade<DIM>,
        [(Binding<PARAMS>, Blade<DIM>); DEGREE],
        Coefficient,
    )> for Multinomial<PARAMS, DEGREE, DIM>
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<
            Item = (
                Blade<DIM>,
                [(Binding<PARAMS>, Blade<DIM>); DEGREE],
                Coefficient,
            ),
        >,
    {
        let mut map = std::collections::BTreeMap::new();
        iter.into_iter()
            .for_each(|(blade, mut multi_index, coeff)| {
                if coeff != 0 {
                    multi_index.sort();
                    let polynomial: &mut std::collections::BTreeMap<_, _> =
                        map.entry(blade).or_default();
                    match polynomial.entry(multi_index) {
                        std::collections::btree_map::Entry::Vacant(vacant) => {
                            vacant.insert(coeff);
                        }
                        std::collections::btree_map::Entry::Occupied(mut occupied) => {
                            let slot = occupied.get_mut();
                            *slot += coeff;
                            if *slot == 0 {
                                occupied.remove_entry();
                            }
                        }
                    }
                    if polynomial.is_empty() {
                        map.remove(&blade);
                    }
                }
            });
        Self(map)
    }
}

impl<const PARAMS: usize, const DEGREE: usize, const DIM: usize> Multinomial<PARAMS, DEGREE, DIM> {
    fn space_expr(&self, spaces: [Space<DIM>; PARAMS]) -> (Space<DIM>, Expr<PARAMS, DIM>) {
        let space = Space::from_blades(self.0.keys().cloned());
        let expr = Expr::structure(space, |blade| match self.0.get(&blade) {
            None => Expr::literal(0),
            Some(polynomial) => polynomial
                .into_iter()
                .filter_map(|(multi_index, coeff)| {
                    multi_index
                        .iter()
                        .map(|&(binding, binding_blade)| {
                            spaces[binding]
                                .contains(binding_blade)
                                .then(|| Expr::variable(binding).field(binding_blade))
                        })
                        .collect::<Option<Vec<_>>>()
                        .map(|exprs| (exprs.into_iter(), coeff))
                })
                .fold(None, |expr_acc, (exprs, &coeff)| {
                    let coeff_abs = coeff.unsigned_abs();
                    let literal = Expr::literal(coeff_abs);
                    Some(match (expr_acc, coeff_abs == 1, coeff < 0) {
                        (Some(expr_acc), false, false) => {
                            expr_acc + exprs.fold(literal, std::ops::Mul::mul)
                        }
                        (Some(expr_acc), false, true) => {
                            expr_acc - exprs.fold(literal, std::ops::Mul::mul)
                        }
                        (Some(expr_acc), true, false) => {
                            expr_acc + exprs.reduce(std::ops::Mul::mul).unwrap_or(literal)
                        }
                        (Some(expr_acc), true, true) => {
                            expr_acc - exprs.reduce(std::ops::Mul::mul).unwrap_or(literal)
                        }
                        (None, false, false) => exprs.fold(literal, std::ops::Mul::mul),
                        (None, false, true) => exprs.fold(-literal, std::ops::Mul::mul),
                        (None, true, false) => exprs.reduce(std::ops::Mul::mul).unwrap_or(literal),
                        (None, true, true) => exprs
                            .fold(None, |expr_acc, expr| {
                                Some(match expr_acc {
                                    Some(expr_acc) => expr_acc * expr,
                                    None => -expr,
                                })
                            })
                            .unwrap_or(-literal),
                    })
                })
                .unwrap_or_else(|| Expr::literal(0)),
        });
        (space, expr)
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
    fn multinomial<const PARAMS: usize, const DEGREE: usize>(
        &self,
        prototype: [Binding<PARAMS>; DEGREE],
        reindex: fn([Blade<DIM>; DEGREE]) -> [Blade<DIM>; DEGREE],
        term_filter: fn([Blade<DIM>; DEGREE]) -> bool,
        term_sign: fn([Blade<DIM>; DEGREE]) -> Sign,
    ) -> Multinomial<PARAMS, DEGREE, DIM> {
        std::iter::repeat_n(Blade::iter(), DEGREE)
            .multi_cartesian_product()
            .map(|blades| -> [Blade<DIM>; DEGREE] { blades.try_into().unwrap() })
            .map(reindex)
            .filter(|blades| term_filter(*blades))
            .map(|blades| {
                let multi_index = std::array::from_fn(|index| (prototype[index], blades[index]));
                let blade = blades
                    .into_iter()
                    .fold(Blade::zero(), std::ops::BitXor::bitxor);
                let coeff = Coefficient::from(
                    term_sign(blades)
                        ^ blades
                            .into_iter()
                            .map(|blade| self[blade].intrinsic_sign)
                            .fold(self[blade].intrinsic_sign, std::ops::BitXor::bitxor),
                ) * blades
                    .into_iter()
                    .fold(!Blade::zero(), std::ops::BitAnd::bitand)
                    .iter_generators()
                    .map(|generator| self.generator_squares[generator.generator])
                    .product::<Coefficient>();
                (blade, multi_index, coeff)
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pretype<const TYPE_PARAMS: usize> {
    Atom,
    Binding { binding: Binding<TYPE_PARAMS> },
}

impl<const TYPE_PARAMS: usize> Pretype<TYPE_PARAMS> {
    fn resolve<const DIM: usize>(self, type_params: &[Type<DIM>; TYPE_PARAMS]) -> Type<DIM> {
        match self {
            Self::Atom => Type::Atom,
            Self::Binding { binding } => type_params[binding],
        }
    }
}

trait TraitNames {
    const TRAIT_NAME: &'static str;
    const METHOD_NAME: &'static str;
}

trait Operation<const TYPE_PARAMS: usize, const PARAMS: usize> {
    fn signature(&self) -> ExprSignature<TYPE_PARAMS, PARAMS>;
    fn implementations<const DIM: usize>(
        &self,
        geometric_algebra: GeometricAlgebra<DIM>,
    ) -> Vec<OperationImpl<TYPE_PARAMS, PARAMS, DIM>>;
}

struct ExprSignature<const TYPE_PARAMS: usize, const PARAMS: usize> {
    param_pretypes: [Pretype<TYPE_PARAMS>; PARAMS],
    return_pretype: Option<Pretype<TYPE_PARAMS>>, // None -> associated type `Output`
}

struct OperationImpl<const TYPE_PARAMS: usize, const PARAMS: usize, const DIM: usize> {
    type_params: [Type<DIM>; TYPE_PARAMS],
    return_type: Type<DIM>,
    return_expr: Expr<PARAMS, DIM>,
}

trait Writer {
    fn write_struct(&mut self, name: &str, fields: Vec<&str>);
    fn write_expr_impl(&mut self, expr_impl: OperationImpl);
    // fn write_operation_trait<const DIM: usize>(&mut self, )
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Type<const DIM: usize> {
    Atom,
    Space(Space<DIM>),
}

enum ExprRepr<const PARAMS: usize, const DIM: usize> {
    Variable {
        binding: Binding<PARAMS>,
    },
    Literal {
        value: u8,
    },
    Structure {
        space: Space<DIM>,
        fields: Vec<(Blade<DIM>, Expr<PARAMS, DIM>)>,
    },
    Field {
        expr: Expr<PARAMS, DIM>,
        blade: Blade<DIM>,
    },
    CallBuiltinUnary {
        operation: BuiltinUnaryOperation,
        param_expr: Expr<PARAMS, DIM>,
    },
    Call {
        trait_name: &'static str,
        method_name: &'static str,
        type_params: Vec<Type<DIM>>,
        param_exprs: Vec<Expr<PARAMS, DIM>>,
    },
    Neg {
        expr: Expr<PARAMS, DIM>,
    },
    Add {
        lhs: Expr<PARAMS, DIM>,
        rhs: Expr<PARAMS, DIM>,
    },
    Sub {
        lhs: Expr<PARAMS, DIM>,
        rhs: Expr<PARAMS, DIM>,
    },
    Mul {
        lhs: Expr<PARAMS, DIM>,
        rhs: Expr<PARAMS, DIM>,
    },
    Div {
        lhs: Expr<PARAMS, DIM>,
        rhs: Expr<PARAMS, DIM>,
    },
}

struct Expr<const PARAMS: usize, const DIM: usize> {
    repr: Box<ExprRepr<PARAMS, DIM>>,
}

impl<const PARAMS: usize, const DIM: usize> Expr<PARAMS, DIM> {
    fn variable(binding: Binding<PARAMS>) -> Self {
        Self {
            repr: Box::new(ExprRepr::Variable { binding }),
        }
    }

    fn literal(value: u8) -> Self {
        Self {
            repr: Box::new(ExprRepr::Literal { value }),
        }
    }

    fn structure(space: Space<DIM>, field: impl Fn(Blade<DIM>) -> Self) -> Self {
        Self {
            repr: Box::new(ExprRepr::Structure {
                space,
                fields: space
                    .iter_blades()
                    .map(|blade| (blade, field(blade)))
                    .collect(),
            }),
        }
    }

    fn field(self, blade: Blade<DIM>) -> Self {
        Self {
            repr: Box::new(ExprRepr::Field { expr: self, blade }),
        }
    }

    fn call_builtin_unary(self, operation: BuiltinUnaryOperation) -> Self {
        Self {
            repr: Box::new(ExprRepr::CallBuiltinUnary {
                operation,
                param_expr: self,
            }),
        }
    }

    fn call<Op, const OP_TYPE_PARAMS: usize, const OP_PARAMS: usize, const ARGS: usize>(
        self,
        _operation: Op,
        type_params: [Type<DIM>; OP_TYPE_PARAMS],
        param_exprs: [Self; ARGS],
    ) -> Self
    where
        Op: TraitNames + Operation<OP_TYPE_PARAMS, OP_PARAMS>,
    {
        assert_eq!(ARGS + 1, OP_PARAMS);
        Self {
            repr: Box::new(ExprRepr::Call {
                trait_name: Op::TRAIT_NAME,
                method_name: Op::METHOD_NAME,
                type_params: type_params.into(),
                param_exprs: std::iter::once(self).chain(param_exprs).collect(),
            }),
        }
    }
}

impl<const PARAMS: usize, const DIM: usize> std::ops::Neg for Expr<PARAMS, DIM> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ExprRepr::Neg { expr: self }.into()
    }
}

impl<const PARAMS: usize, const DIM: usize> std::ops::Add for Expr<PARAMS, DIM> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ExprRepr::Add { lhs: self, rhs }.into()
    }
}

impl<const PARAMS: usize, const DIM: usize> std::ops::Sub for Expr<PARAMS, DIM> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ExprRepr::Sub { lhs: self, rhs }.into()
    }
}

impl<const PARAMS: usize, const DIM: usize> std::ops::Mul for Expr<PARAMS, DIM> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        ExprRepr::Mul { lhs: self, rhs }.into()
    }
}

impl<const PARAMS: usize, const DIM: usize> std::ops::Div for Expr<PARAMS, DIM> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        ExprRepr::Div { lhs: self, rhs }.into()
    }
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
