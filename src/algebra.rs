use crate::ast::{
    Ast, Expr, Implementation, Item, OperationSignature, Ownership, Record, Stmt, Stringifier,
    Structure, TemplateSignature, TypeBinding,
};
use crate::syntax::{Syntax, Writer};
use itertools::Itertools;
use strum::{EnumIter, EnumProperty, IntoEnumIterator};

type Coefficient = i32;

#[derive(Clone, Copy)]
enum Sign {
    Pos,
    Neg,
}

impl Sign {
    fn from_count(count: usize) -> Self {
        if 1 & count == 0 {
            Self::Pos
        } else {
            Self::Neg
        }
    }
}

impl std::ops::BitXor for Sign {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Pos, Self::Pos) => Self::Pos,
            (Self::Pos, Self::Neg) => Self::Neg,
            (Self::Neg, Self::Pos) => Self::Neg,
            (Self::Neg, Self::Neg) => Self::Pos,
        }
    }
}

impl From<Sign> for Coefficient {
    fn from(value: Sign) -> Self {
        match value {
            Sign::Pos => 1,
            Sign::Neg => -1,
        }
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct Blade {
    generator_bits: usize,
}

impl std::ops::BitXor for Blade {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            generator_bits: self.generator_bits ^ rhs.generator_bits,
        }
    }
}

impl std::ops::BitAnd for Blade {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            generator_bits: self.generator_bits & rhs.generator_bits,
        }
    }
}

impl Blade {
    fn iter(dim: usize) -> impl Iterator<Item = Self> + Clone {
        (0..1 << dim).map(|generator_bits| Self { generator_bits })
    }

    fn zero() -> Self {
        Self { generator_bits: 0 }
    }

    fn iter_generators(self, dim: usize) -> impl Iterator<Item = usize> + Clone {
        (0..dim).filter(move |generator| self.generator_bits & 1 << generator != 0)
    }

    fn grade(self) -> usize {
        self.generator_bits.count_ones() as usize
    }

    fn dual(self, dim: usize) -> Self {
        Self {
            generator_bits: ((1 << dim) - 1) ^ self.generator_bits,
        }
    }

    fn parity(self, other: Self) -> Sign {
        let gray_inv = (0..)
            .fold_while(self.generator_bits, |bits, base| {
                let shifted = bits >> (1 << base);
                if shifted == 0 {
                    itertools::FoldWhile::Done(bits)
                } else {
                    itertools::FoldWhile::Continue(bits ^ shifted)
                }
            })
            .into_inner();
        Sign::from_count((gray_inv >> 1 & other.generator_bits).count_ones() as usize)
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum Space {
    GradedVector { grade: usize },
    SaturatedVector { even: bool, odd: bool },
}

impl Space {
    fn iter(dim: usize) -> impl Iterator<Item = Self> + Clone {
        (0..=dim).map(|grade| Self::GradedVector { grade }).chain(
            [false, true]
                .into_iter()
                .cartesian_product([false, true])
                .map(|(even, odd)| Self::SaturatedVector { even, odd }),
        )
    }

    fn null() -> Self {
        Self::SaturatedVector {
            even: false,
            odd: false,
        }
    }

    fn contains_blade(self, blade: Blade) -> bool {
        match self {
            Self::GradedVector { grade } => blade.grade() == grade,
            Self::SaturatedVector { even, odd } => {
                even && blade.grade() & 1 == 0 || odd && blade.grade() & 1 == 1
            }
        }
    }

    fn iter_blades(self, dim: usize) -> impl Iterator<Item = Blade> {
        Blade::iter(dim).filter(move |&blade| self.contains_blade(blade))
    }

    fn dual(self, dim: usize) -> Self {
        match self {
            Self::GradedVector { grade } => Self::GradedVector { grade: dim - grade },
            Self::SaturatedVector { even, odd } if dim & 1 == 0 => {
                Self::SaturatedVector { even, odd }
            }
            Self::SaturatedVector { even, odd } => Self::SaturatedVector {
                even: odd,
                odd: even,
            },
        }
    }

    fn contains_even_grade(self) -> bool {
        matches!(self, Self::GradedVector {grade} if grade & 1 == 0)
            || matches!(self, Self::SaturatedVector { even: true, odd: _ })
    }

    fn contains_odd_grade(self) -> bool {
        matches!(self, Self::GradedVector {grade} if grade & 1 == 1)
            || matches!(self, Self::SaturatedVector { even: _, odd: true })
    }

    fn product_space(self, other: Self) -> Self {
        Self::SaturatedVector {
            even: self.contains_even_grade() && other.contains_even_grade()
                || self.contains_odd_grade() && other.contains_odd_grade(),
            odd: self.contains_even_grade() && other.contains_odd_grade()
                || self.contains_odd_grade() && other.contains_even_grade(),
        }
    }

    fn structure(self, dim: usize) -> Structure<GeometricAlgebra> {
        TemplateSignature { template: self }.structure(self.iter_blades(dim).map(|blade| Item {
            key: blade,
            value: Class::Base,
        }))
    }

    fn construct(
        self,
        dim: usize,
        field: impl Fn(Blade) -> Expr<GeometricAlgebra>,
    ) -> Expr<GeometricAlgebra> {
        TemplateSignature { template: self }.construct(self.iter_blades(dim).map(|blade| Item {
            key: blade,
            value: field(blade),
        }))
    }

    fn access(self, expr: Expr<GeometricAlgebra>, blade: Blade) -> Expr<GeometricAlgebra> {
        TemplateSignature { template: self }.access(expr, blade)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Class {
    Base,
    Space(Space),
}

#[derive(Clone, Copy, EnumIter, EnumProperty)]
enum Operation {
    #[strum(props(trait = "Abs", fn = "abs"))]
    Abs,
    #[strum(props(trait = "Sqrt", fn = "sqrt"))]
    Sqrt,
    #[strum(props(trait = "From", fn = "from"))]
    From,
    #[strum(props(trait = "Neg", fn = "neg"))]
    Neg,
    #[strum(props(trait = "Add", fn = "add"))]
    Add,
    #[strum(props(trait = "Sub", fn = "sub"))]
    Sub,
    #[strum(props(trait = "Mul", fn = "mul"))]
    Mul,
    #[strum(props(trait = "Div", fn = "div"))]
    Div,
    #[strum(props(trait = "AddAssign", fn = "add_assign"))]
    AddAssign,
    #[strum(props(trait = "SubAssign", fn = "sub_assign"))]
    SubAssign,
    #[strum(props(trait = "MulAssign", fn = "mul_assign"))]
    MulAssign,
    #[strum(props(trait = "DivAssign", fn = "div_assign"))]
    DivAssign,
    #[strum(props(trait = "Zero", fn = "zero"))]
    Zero,
    #[strum(props(trait = "One", fn = "one"))]
    One,
    #[strum(props(trait = "GradeInvolution", fn = "grade_involution"))]
    GradeInvolution,
    #[strum(props(trait = "Reverse", fn = "reverse"))]
    Reverse,
    #[strum(props(trait = "Conjugate", fn = "conjugate"))]
    Conjugate,
    #[strum(props(trait = "Dual", fn = "dual"))]
    Dual,
    #[strum(props(trait = "Undual", fn = "undual"))]
    Undual,
    #[strum(props(trait = "NormSquared", fn = "norm_squared"))]
    NormSquared,
    #[strum(props(trait = "Norm", fn = "norm"))]
    Norm,
    #[strum(props(trait = "Inverse", fn = "inverse"))]
    Inverse,
    #[strum(props(trait = "Normalized", fn = "normalized"))]
    Normalized,
    #[strum(props(trait = "Normalize", fn = "normalize"))]
    Normalize,
    #[strum(props(trait = "GeometricProduct", fn = "geometric_product"))]
    GeometricProduct,
    #[strum(props(trait = "ScalarProduct", fn = "scalar_product"))]
    ScalarProduct,
    #[strum(props(trait = "LeftInnerProduct", fn = "left_inner_product"))]
    LeftInnerProduct,
    #[strum(props(trait = "RightInnerProduct", fn = "right_inner_product"))]
    RightInnerProduct,
    #[strum(props(trait = "InnerProduct", fn = "inner_product"))]
    InnerProduct,
    #[strum(props(trait = "OuterProduct", fn = "outer_product"))]
    OuterProduct,
    #[strum(props(trait = "RegressiveProduct", fn = "regressive_product"))]
    RegressiveProduct,
    #[strum(props(trait = "Commutator", fn = "commutator"))]
    Commutator,
    #[strum(props(trait = "Anticommutator", fn = "anticommutator"))]
    Anticommutator,
    #[strum(props(trait = "Transform", fn = "transform"))]
    Transform,
    #[strum(props(trait = "Project", fn = "project"))]
    Project,
    #[strum(props(trait = "Reject", fn = "reject"))]
    Reject,
}

type NullaryConstructorSignature = OperationSignature<GeometricAlgebra, 0, 0, 0, 0, 1>;
type UnaryConstructorSignature = OperationSignature<GeometricAlgebra, 1, 0, 0, 1, 1>;
type UnarySignature = OperationSignature<GeometricAlgebra, 0, 1, 1, 0, 1>;
type BinarySignature = OperationSignature<GeometricAlgebra, 1, 1, 1, 1, 1>;
type UnaryInplaceSignature = OperationSignature<GeometricAlgebra, 0, 0, 1, 0, 0>;
type BinaryInplaceSignature = OperationSignature<GeometricAlgebra, 1, 0, 1, 1, 0>;

impl Operation {
    const fn nullary_constructor_signature(self) -> NullaryConstructorSignature {
        OperationSignature {
            operation: self,
            generics: [],
            associates: [],
            self_param_item: [],
            param_items: [],
            return_type_binding: [TypeBinding::SelfBining],
        }
    }

    const fn unary_constructor_signature(self) -> UnaryConstructorSignature {
        OperationSignature {
            operation: self,
            generics: ["T"],
            associates: [],
            self_param_item: [],
            param_items: [Item {
                key: "value",
                value: Ownership::Owned(TypeBinding::GenericBinding(0)),
            }],
            return_type_binding: [TypeBinding::SelfBining],
        }
    }

    const fn unary_signature(self) -> UnarySignature {
        OperationSignature {
            operation: self,
            generics: [],
            associates: ["Output"],
            self_param_item: [Item {
                key: "self",
                value: Ownership::Owned(TypeBinding::SelfBining),
            }],
            param_items: [],
            return_type_binding: [TypeBinding::AssociateBinding(0)],
        }
    }

    const fn binary_signature(self) -> BinarySignature {
        OperationSignature {
            operation: self,
            generics: ["T"],
            associates: ["Output"],
            self_param_item: [Item {
                key: "self",
                value: Ownership::Owned(TypeBinding::SelfBining),
            }],
            param_items: [Item {
                key: "other",
                value: Ownership::Owned(TypeBinding::GenericBinding(0)),
            }],
            return_type_binding: [TypeBinding::AssociateBinding(0)],
        }
    }

    const fn unary_inplace_signature(self) -> UnaryInplaceSignature {
        OperationSignature {
            operation: self,
            generics: [],
            associates: [],
            self_param_item: [Item {
                key: "self",
                value: Ownership::BorrowedMut(TypeBinding::SelfBining),
            }],
            param_items: [],
            return_type_binding: [],
        }
    }

    const fn binary_inplace_signature(self) -> BinaryInplaceSignature {
        OperationSignature {
            operation: self,
            generics: ["T"],
            associates: [],
            self_param_item: [Item {
                key: "self",
                value: Ownership::BorrowedMut(TypeBinding::SelfBining),
            }],
            param_items: [Item {
                key: "other",
                value: Ownership::Owned(TypeBinding::GenericBinding(0)),
            }],
            return_type_binding: [],
        }
    }
}

const ABS: UnarySignature = Operation::Abs.unary_signature();
const SQRT: UnarySignature = Operation::Sqrt.unary_signature();
const FROM: UnaryConstructorSignature = Operation::From.unary_constructor_signature();
const NEG: UnarySignature = Operation::Neg.unary_signature();
const ADD: BinarySignature = Operation::Add.binary_signature();
const SUB: BinarySignature = Operation::Sub.binary_signature();
const MUL: BinarySignature = Operation::Mul.binary_signature();
const DIV: BinarySignature = Operation::Div.binary_signature();
const ADD_ASSIGN: BinaryInplaceSignature = Operation::AddAssign.binary_inplace_signature();
const SUB_ASSIGN: BinaryInplaceSignature = Operation::SubAssign.binary_inplace_signature();
const MUL_ASSIGN: BinaryInplaceSignature = Operation::MulAssign.binary_inplace_signature();
const DIV_ASSIGN: BinaryInplaceSignature = Operation::DivAssign.binary_inplace_signature();
const ZERO: NullaryConstructorSignature = Operation::Zero.nullary_constructor_signature();
const ONE: NullaryConstructorSignature = Operation::One.nullary_constructor_signature();
const GRADE_INVOLUTION: UnarySignature = Operation::GradeInvolution.unary_signature();
const REVERSE: UnarySignature = Operation::Reverse.unary_signature();
const CONJUGATE: UnarySignature = Operation::Conjugate.unary_signature();
const DUAL: UnarySignature = Operation::Dual.unary_signature();
const UNDUAL: UnarySignature = Operation::Undual.unary_signature();
const NORM_SQUARED: UnarySignature = Operation::NormSquared.unary_signature();
const NORM: UnarySignature = Operation::Norm.unary_signature();
const INVERSE: UnarySignature = Operation::Inverse.unary_signature();
const NORMALIZED: UnarySignature = Operation::Normalized.unary_signature();
const NORMALIZE: UnaryInplaceSignature = Operation::Normalize.unary_inplace_signature();
const GEOMETRIC_PRODUCT: BinarySignature = Operation::GeometricProduct.binary_signature();
const SCALAR_PRODUCT: BinarySignature = Operation::ScalarProduct.binary_signature();
const LEFT_INNER_PRODUCT: BinarySignature = Operation::LeftInnerProduct.binary_signature();
const RIGHT_INNER_PRODUCT: BinarySignature = Operation::RightInnerProduct.binary_signature();
const INNER_PRODUCT: BinarySignature = Operation::InnerProduct.binary_signature();
const OUTER_PRODUCT: BinarySignature = Operation::OuterProduct.binary_signature();
const REGRESSIVE_PRODUCT: BinarySignature = Operation::RegressiveProduct.binary_signature();
const COMMUTATOR: BinarySignature = Operation::Commutator.binary_signature();
const ANTICOMMUTATOR: BinarySignature = Operation::Anticommutator.binary_signature();
const TRANSFORM: BinarySignature = Operation::Transform.binary_signature();
const PROJECT: BinarySignature = Operation::Project.binary_signature();
const REJECT: BinarySignature = Operation::Reject.binary_signature();

struct Multinomial<'a, const PARAMS: usize> {
    alg: &'a GeometricAlgebra,
    polynomials: std::collections::BTreeMap<
        Blade,
        std::collections::BTreeMap<Vec<(usize, Blade)>, Coefficient>,
    >,
}

impl<'a, const PARAMS: usize> Multinomial<'a, PARAMS> {
    fn new<const DEGREE: usize>(
        alg: &'a GeometricAlgebra,
        prototype: [usize; DEGREE],
        term_sign: fn([Blade; DEGREE], usize) -> Option<Sign>,
    ) -> Self {
        Self {
            alg,
            polynomials: std::iter::repeat_n(Blade::iter(alg.dim), DEGREE)
                .multi_cartesian_product()
                .map(|blades| -> [Blade; DEGREE] { blades.try_into().ok().unwrap() })
                .filter_map(|blades| {
                    term_sign(blades, alg.dim).map(|sign| {
                        let multi_index = prototype.into_iter().zip(blades).sorted().collect_vec();
                        let (blade_product, coeff) = blades.into_iter().fold(
                            (Blade::zero(), Coefficient::from(sign)),
                            |(blade_product, coeff), blade| {
                                (
                                    blade_product ^ blade,
                                    coeff
                                        * (blade_product & blade)
                                            .iter_generators(alg.dim)
                                            .map(|generator| alg.generator_squares[generator])
                                            .product::<Coefficient>(),
                                )
                            },
                        );
                        (blade_product, multi_index, coeff)
                    })
                })
                .fold(
                    std::collections::BTreeMap::new(),
                    |mut polynomials, (blade, multi_index, coeff)| {
                        if coeff != 0 {
                            let polynomial = polynomials.entry(blade).or_default();
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
                                polynomials.remove(&blade);
                            }
                        }
                        polynomials
                    },
                ),
        }
    }

    fn dual(self) -> Self {
        Self {
            alg: self.alg,
            polynomials: self
                .polynomials
                .into_iter()
                .map(|(blade, polynomial)| (blade.dual(self.alg.dim), polynomial))
                .collect(),
        }
    }

    fn blade_expr(
        &self,
        blade: Blade,
        spaces: [Space; PARAMS],
        params: [<GeometricAlgebra as Ast>::Param; PARAMS],
    ) -> Expr<GeometricAlgebra> {
        self.polynomials
            .get(&blade)
            .and_then(|polynomial| {
                polynomial
                    .iter()
                    .filter_map(|(multi_index, coeff)| {
                        multi_index
                            .iter()
                            .map(|&(index, blade)| {
                                spaces[index].contains_blade(blade).then(|| {
                                    spaces[index].access(Expr::param(params[index]), blade)
                                })
                            })
                            .collect::<Option<Vec<_>>>()
                            .map(|exprs| {
                                (
                                    exprs.into_iter(),
                                    *coeff
                                        * Coefficient::from(
                                            multi_index
                                                .iter()
                                                .map(|(_, blade)| {
                                                    self.alg.blade_intrinsic_signs
                                                        [blade.generator_bits]
                                                })
                                                .fold(
                                                    self.alg.blade_intrinsic_signs
                                                        [blade.generator_bits],
                                                    std::ops::BitXor::bitxor,
                                                ),
                                        ),
                                )
                            })
                    })
                    .fold(None, |expr_acc, (exprs, coeff)| {
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
                            (None, true, false) => {
                                exprs.reduce(std::ops::Mul::mul).unwrap_or(literal)
                            }
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
            })
            .unwrap_or_else(|| Expr::literal(0))
    }

    fn space_expr(
        &self,
        space: Space,
        spaces: [Space; PARAMS],
        params: [<GeometricAlgebra as Ast>::Param; PARAMS],
    ) -> Expr<GeometricAlgebra> {
        space.construct(self.alg.dim, |blade| self.blade_expr(blade, spaces, params))
    }
}

struct GeometricAlgebra {
    dim: usize,
    generator_squares: Vec<Coefficient>,
    blade_intrinsic_signs: Vec<Sign>,
}

impl Ast for GeometricAlgebra {
    type Type = Class;
    type Template = Space;
    type Field = Blade;
    type Operation = Operation;
    type Generic = &'static str;
    type Associate = &'static str;
    type Param = &'static str;
    type Literal = u32;

    fn structures(&self) -> impl Iterator<Item = Structure<Self>> {
        Space::iter(self.dim).map(|space| space.structure(self.dim))
    }

    fn implementations(&self) -> impl Iterator<Item = Implementation<Self>> {
        Operation::iter().flat_map(|operation| self.operation_implementations(operation))
    }
}

impl GeometricAlgebra {
    #[allow(clippy::wrong_self_convention)]
    fn from_implementations(
        &self,
        signature: &UnaryConstructorSignature,
    ) -> Vec<Implementation<GeometricAlgebra>> {
        Space::iter(self.dim)
            .map(Class::Space)
            .chain(std::iter::once(Class::Base))
            .cartesian_product(
                Space::iter(self.dim)
                    .map(Class::Space)
                    .chain(std::iter::once(Class::Base)),
            )
            .filter(|(type_0, type_1)| type_0 != type_1)
            .map(|(type_0, type_1)| {
                signature.implementation(type_1, [type_0], [], |[], [param_0]| match type_1 {
                    Class::Base => match type_0 {
                        Class::Base => Expr::param(param_0),
                        Class::Space(space_0) => {
                            if space_0.contains_blade(Blade::zero()) {
                                space_0.access(Expr::param(param_0), Blade::zero())
                            } else {
                                Expr::literal(0)
                            }
                        }
                    },
                    Class::Space(space_1) => space_1.construct(self.dim, |blade| match type_0 {
                        Class::Base => {
                            if blade == Blade::zero() {
                                Expr::param(param_0)
                            } else {
                                Expr::literal(0)
                            }
                        }
                        Class::Space(space_0) => {
                            if space_0.contains_blade(blade) {
                                space_0.access(Expr::param(param_0), blade)
                            } else {
                                Expr::literal(0)
                            }
                        }
                    }),
                })
            })
            .collect()
    }

    fn zero_implementations(
        &self,
        signature: &NullaryConstructorSignature,
    ) -> Vec<Implementation<GeometricAlgebra>> {
        Space::iter(self.dim)
            .map(|space_0| {
                signature.implementation(Class::Space(space_0), [], [], |[], []| {
                    space_0.construct(self.dim, |_| Expr::literal(0u32))
                })
            })
            .collect()
    }

    fn one_implementations(
        &self,
        signature: &NullaryConstructorSignature,
    ) -> Vec<Implementation<GeometricAlgebra>> {
        Space::iter(self.dim)
            .filter(|space| space.contains_blade(Blade::zero()))
            .map(|space_0| {
                signature.implementation(Class::Space(space_0), [], [], |[], []| {
                    space_0.construct(self.dim, |blade| {
                        if blade == Blade::zero() {
                            match self.blade_intrinsic_signs[blade.generator_bits] {
                                Sign::Pos => Expr::literal(1u32),
                                Sign::Neg => -Expr::literal(1u32),
                            }
                        } else {
                            Expr::literal(0u32)
                        }
                    })
                })
            })
            .collect()
    }

    fn unary_operator_implementations(
        &self,
        signature: &UnarySignature,
        op: fn(Expr<GeometricAlgebra>) -> Expr<GeometricAlgebra>,
    ) -> Vec<Implementation<GeometricAlgebra>> {
        Space::iter(self.dim)
            .map(|space_0| {
                signature.implementation(
                    Class::Space(space_0),
                    [],
                    [Class::Space(space_0)],
                    |[param_0], []| {
                        space_0.construct(self.dim, |blade| {
                            op(space_0.access(Expr::param(param_0), blade))
                        })
                    },
                )
            })
            .collect()
    }

    fn binary_operator_implementations(
        &self,
        signature: &BinarySignature,
        op: fn(Expr<GeometricAlgebra>, Expr<GeometricAlgebra>) -> Expr<GeometricAlgebra>,
    ) -> Vec<Implementation<GeometricAlgebra>> {
        Space::iter(self.dim)
            .map(|space| {
                signature.implementation(
                    Class::Space(space),
                    [Class::Space(space)],
                    [Class::Space(space)],
                    |[param_0], [param_1]| {
                        space.construct(self.dim, |blade| {
                            op(
                                space.access(Expr::param(param_0), blade),
                                space.access(Expr::param(param_1), blade),
                            )
                        })
                    },
                )
            })
            .chain(Space::iter(self.dim).map(|space| {
                signature.implementation(
                    Class::Space(space),
                    [Class::Base],
                    [Class::Space(space)],
                    |[param_0], [param_1]| {
                        space.construct(self.dim, |blade| {
                            op(
                                space.access(Expr::param(param_0), blade),
                                Expr::param(param_1),
                            )
                        })
                    },
                )
            }))
            .chain(Space::iter(self.dim).map(|space| {
                signature.implementation(
                    Class::Base,
                    [Class::Space(space)],
                    [Class::Space(space)],
                    |[param_0], [param_1]| {
                        space.construct(self.dim, |blade| {
                            op(
                                Expr::param(param_0),
                                space.access(Expr::param(param_1), blade),
                            )
                        })
                    },
                )
            }))
            .collect()
    }

    fn binary_inplace_operator_implementations(
        &self,
        signature: &BinaryInplaceSignature,
        op: fn(Expr<GeometricAlgebra>, Expr<GeometricAlgebra>) -> Stmt<GeometricAlgebra>,
    ) -> Vec<Implementation<GeometricAlgebra>> {
        Space::iter(self.dim)
            .map(|space| {
                signature.implementation(
                    Class::Space(space),
                    [Class::Space(space)],
                    [],
                    |[param_0], [param_1]| {
                        space
                            .iter_blades(self.dim)
                            .map(|blade| {
                                op(
                                    space.access(Expr::param(param_0), blade),
                                    space.access(Expr::param(param_1), blade),
                                )
                            })
                            .collect_vec()
                    },
                )
            })
            .chain(Space::iter(self.dim).map(|space| {
                signature.implementation(
                    Class::Space(space),
                    [Class::Base],
                    [],
                    |[param_0], [param_1]| {
                        space
                            .iter_blades(self.dim)
                            .map(|blade| {
                                op(
                                    space.access(Expr::param(param_0), blade),
                                    Expr::param(param_1),
                                )
                            })
                            .collect_vec()
                    },
                )
            }))
            .collect()
    }

    fn unary_multinomial_implementations(
        &self,
        signature: &UnarySignature,
        class_fn: fn([Space; 1], usize) -> Class,
        multinomial: Multinomial<1>,
    ) -> Vec<Implementation<GeometricAlgebra>> {
        Space::iter(self.dim)
            .map(|space_0| {
                let class = class_fn([space_0], self.dim);
                signature.implementation(Class::Space(space_0), [], [class], |[param_0], []| {
                    match class {
                        Class::Base => multinomial.blade_expr(Blade::zero(), [space_0], [param_0]),
                        Class::Space(space) => multinomial.space_expr(space, [space_0], [param_0]),
                    }
                })
            })
            .collect()
    }

    fn binary_multinomial_implementations(
        &self,
        signature: &BinarySignature,
        class_fn: fn([Space; 2], usize) -> Class,
        multinomial: Multinomial<2>,
    ) -> Vec<Implementation<GeometricAlgebra>> {
        Space::iter(self.dim)
            .cartesian_product(Space::iter(self.dim))
            .map(|(space_0, space_1)| {
                let class = class_fn([space_0, space_1], self.dim);
                signature.implementation(
                    Class::Space(space_0),
                    [Class::Space(space_1)],
                    [class],
                    |[param_0], [param_1]| match class {
                        Class::Base => multinomial.blade_expr(
                            Blade::zero(),
                            [space_0, space_1],
                            [param_0, param_1],
                        ),
                        Class::Space(space) => {
                            multinomial.space_expr(space, [space_0, space_1], [param_0, param_1])
                        }
                    },
                )
            })
            .collect()
    }

    fn unary_inherited_implementations(
        &self,
        signature: &UnarySignature,
        class_fn: fn([Space; 1]) -> Class,
        expr_fn: fn([Space; 1], [<GeometricAlgebra as Ast>::Param; 1]) -> Expr<GeometricAlgebra>,
    ) -> Vec<Implementation<GeometricAlgebra>> {
        Space::iter(self.dim)
            .map(|space_0| {
                let class = class_fn([space_0]);
                signature.implementation(Class::Space(space_0), [], [class], |[param_0], []| {
                    expr_fn([space_0], [param_0])
                })
            })
            .collect()
    }

    fn unary_inplace_inherited_implementations(
        &self,
        signature: &UnaryInplaceSignature,
        stmts_fn: fn(
            [Space; 1],
            [<GeometricAlgebra as Ast>::Param; 1],
        ) -> Vec<Stmt<GeometricAlgebra>>,
    ) -> Vec<Implementation<GeometricAlgebra>> {
        Space::iter(self.dim)
            .map(|space_0| {
                signature.implementation(Class::Space(space_0), [], [], |[param_0], []| {
                    stmts_fn([space_0], [param_0])
                })
            })
            .collect()
    }

    fn operation_implementations(
        &self,
        operation: Operation,
    ) -> Vec<Implementation<GeometricAlgebra>> {
        match operation {
            Operation::Abs => Vec::new(),

            Operation::Sqrt => Vec::new(),

            Operation::From => self.from_implementations(&FROM),

            Operation::Zero => self.zero_implementations(&ZERO),

            Operation::One => self.one_implementations(&ONE),

            Operation::Neg => self.unary_operator_implementations(
                &NEG,
                <Expr<GeometricAlgebra> as std::ops::Neg>::neg,
            ),

            Operation::Add => self.binary_operator_implementations(
                &ADD,
                <Expr<GeometricAlgebra> as std::ops::Add>::add,
            ),

            Operation::Sub => self.binary_operator_implementations(
                &SUB,
                <Expr<GeometricAlgebra> as std::ops::Sub>::sub,
            ),

            Operation::Mul => self.binary_operator_implementations(
                &MUL,
                <Expr<GeometricAlgebra> as std::ops::Mul>::mul,
            ),

            Operation::Div => self.binary_operator_implementations(
                &DIV,
                <Expr<GeometricAlgebra> as std::ops::Div>::div,
            ),

            Operation::AddAssign => {
                self.binary_inplace_operator_implementations(&ADD_ASSIGN, Expr::add_assign)
            }

            Operation::SubAssign => {
                self.binary_inplace_operator_implementations(&SUB_ASSIGN, Expr::sub_assign)
            }

            Operation::MulAssign => {
                self.binary_inplace_operator_implementations(&MUL_ASSIGN, Expr::mul_assign)
            }

            Operation::DivAssign => {
                self.binary_inplace_operator_implementations(&DIV_ASSIGN, Expr::div_assign)
            }

            Operation::GradeInvolution => self.unary_multinomial_implementations(
                &GRADE_INVOLUTION,
                |[space_0], _| Class::Space(space_0),
                Multinomial::new(self, [0], |[blade_0], _| Some(Sign::from_count(blade_0.grade()))),
            ),

            Operation::Reverse => self.unary_multinomial_implementations(
                &REVERSE,
                |[space_0], _| Class::Space(space_0),
                Multinomial::new(self, [0], |[blade_0], _| Some(Sign::from_count(blade_0.grade() >> 1))),
            ),

            Operation::Conjugate => self.unary_multinomial_implementations(
                &CONJUGATE,
                |[space_0], _| Class::Space(space_0),
                Multinomial::new(self, [0], |[blade_0], _| {
                    Some(Sign::from_count((blade_0.grade() + 1) >> 1))
                }),
            ),

            Operation::Dual => self.unary_multinomial_implementations(
                &DUAL,
                |[space_0], dim| Class::Space(space_0.dual(dim)),
                Multinomial::new(self, [0], |[blade_0], dim| {
                    Some(blade_0.parity(blade_0.dual(dim)))
                })
                .dual(),
            ),

            Operation::Undual => self.unary_multinomial_implementations(
                &UNDUAL,
                |[space_0], dim| Class::Space(space_0.dual(dim)),
                Multinomial::new(self, [0], |[blade_0], dim| {
                    Some(blade_0.dual(dim).parity(blade_0))
                })
                .dual(),
            ),

            Operation::NormSquared => self.unary_multinomial_implementations(
                &NORM_SQUARED,
                |_, _| Class::Base,
                Multinomial::new(self, [0, 0], |_, _| Some(Sign::Pos)),
            ),

            Operation::Norm => self.unary_inherited_implementations(
                &NORM,
                |_| Class::Base,
                |[space_0], [param_0]| {
                    SQRT.call_builtin(ABS.call_builtin(NORM_SQUARED.call(
                        Class::Space(space_0),
                        [],
                        Expr::param(param_0),
                        [],
                    )))
                },
            ),

            Operation::Inverse => self.unary_inherited_implementations(
                &INVERSE,
                |[space_0]| Class::Space(space_0),
                |[space_0], [param_0]| {
                    DIV.call(
                        Class::Space(space_0),
                        [Class::Base],
                        REVERSE.call(Class::Space(space_0), [], Expr::param(param_0), []),
                        [NORM_SQUARED.call(Class::Space(space_0), [], Expr::param(param_0), [])],
                    )
                },
            ),

            Operation::Normalized => self.unary_inherited_implementations(
                &NORMALIZED,
                |[space_0]| Class::Space(space_0),
                |[space_0], [param_0]| {
                    DIV.call(
                        Class::Space(space_0),
                        [Class::Base],
                        REVERSE.call(Class::Space(space_0), [], Expr::param(param_0), []),
                        [NORM.call(Class::Space(space_0), [], Expr::param(param_0), [])],
                    )
                },
            ),

            Operation::Normalize => self.unary_inplace_inherited_implementations(
                &NORMALIZE,
                |[space_0], [param_0]| {
                    Vec::from([DIV_ASSIGN.call(
                        Class::Space(space_0),
                        [Class::Base],
                        REVERSE.call(Class::Space(space_0), [], Expr::param(param_0), []),
                        [NORM.call(Class::Space(space_0), [], Expr::param(param_0), [])],
                    )])
                },
            ),

            Operation::GeometricProduct => self.binary_multinomial_implementations(
                &GEOMETRIC_PRODUCT,
                |[space_0, space_1], _| Class::Space(space_0.product_space(space_1)),
                Multinomial::new(self, [0, 1], |[blade_0, blade_1], _| {
                    Some(blade_0.parity(blade_1))
                }),
            ),

            Operation::ScalarProduct => self.binary_multinomial_implementations(
                &SCALAR_PRODUCT,
                |_, _| Class::Base,
                Multinomial::new(self, [0, 1], |[blade_0, blade_1], _| {
                    (blade_0 ^ blade_1 == Blade::zero()).then(|| blade_0.parity(blade_1))
                }),
            ),

            Operation::LeftInnerProduct => self.binary_multinomial_implementations(
                &LEFT_INNER_PRODUCT,
                |[space_0, space_1], _| {
                    Class::Space(match [space_0, space_1] {
                        [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                            grade_1
                                .checked_sub(grade_0)
                                .map(|grade| Space::GradedVector { grade })
                                .unwrap_or_else(Space::null)
                        }
                        [space_0, space_1] => space_0.product_space(space_1),
                    })
                },
                Multinomial::new(self, [0, 1], |[blade_0, blade_1], _| {
                    (blade_0 & blade_1 == blade_0).then(|| blade_0.parity(blade_1))
                }),
            ),

            Operation::RightInnerProduct => self.binary_multinomial_implementations(
                &RIGHT_INNER_PRODUCT,
                |[space_0, space_1], _| {
                    Class::Space(match [space_0, space_1] {
                        [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                            grade_0
                                .checked_sub(grade_1)
                                .map(|grade| Space::GradedVector { grade })
                                .unwrap_or_else(Space::null)
                        }
                        [space_0, space_1] => space_0.product_space(space_1),
                    })
                },
                Multinomial::new(self, [0, 1], |[blade_0, blade_1], _| {
                    (blade_0 & blade_1 == blade_1).then(|| blade_0.parity(blade_1))
                }),
            ),

            Operation::InnerProduct => self.binary_multinomial_implementations(
                &INNER_PRODUCT,
                |[space_0, space_1], _| {
                    Class::Space(match [space_0, space_1] {
                        [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                            Space::GradedVector {
                                grade: grade_0.abs_diff(grade_1),
                            }
                        }
                        [space_0, space_1] => space_0.product_space(space_1),
                    })
                },
                Multinomial::new(self, [0, 1], |[blade_0, blade_1], _| {
                    (blade_0 & blade_1 == blade_0 || blade_0 & blade_1 == blade_1)
                        .then(|| blade_0.parity(blade_1))
                }),
            ),

            Operation::OuterProduct => self.binary_multinomial_implementations(
                &OUTER_PRODUCT,
                |[space_0, space_1], dim| {
                    Class::Space(match [space_0, space_1] {
                        [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                            Some(grade_0 + grade_1)
                                .filter(|&grade| grade <= dim)
                                .map(|grade| Space::GradedVector { grade })
                                .unwrap_or_else(Space::null)
                        }
                        [space_0, space_1] => space_0.product_space(space_1),
                    })
                },
                Multinomial::new(self, [0, 1], |[blade_0, blade_1], _| {
                    (blade_0 & blade_1 == Blade::zero()).then(|| blade_0.parity(blade_1))
                }),
            ),

            Operation::RegressiveProduct => self.binary_multinomial_implementations(
                &REGRESSIVE_PRODUCT,
                |[space_0, space_1], dim| {
                    Class::Space(match [space_0.dual(dim), space_1.dual(dim)] {
                    [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                        Some(grade_0 + grade_1)
                            .filter(|&grade| grade <= dim)
                            .map(|grade| Space::GradedVector { grade })
                            .unwrap_or_else(Space::null)
                        }
                        [space_0, space_1] => space_0.product_space(space_1),
                    }.dual(dim))
                },
                Multinomial::new(self, [0, 1], |[blade_0, blade_1], dim| {
                    {
                        (blade_0.dual(dim) & blade_1.dual(dim) == Blade::zero())
                            .then(|| blade_1.dual(dim).parity(blade_0.dual(dim)))
                    }
                })
                .dual(),
            ),

            Operation::Commutator => self.binary_multinomial_implementations(
                &COMMUTATOR,
                |[space_0, space_1], _| Class::Space(space_0.product_space(space_1)),
                Multinomial::new(self, [0, 1], |[blade_0, blade_1], _| {
                    match (blade_0.parity(blade_1), blade_1.parity(blade_0)) {
                        (Sign::Pos, Sign::Pos) => None,
                        (Sign::Pos, Sign::Neg) => Some(Sign::Pos),
                        (Sign::Neg, Sign::Pos) => Some(Sign::Neg),
                        (Sign::Neg, Sign::Neg) => None,
                    }
                }),
            ),

            Operation::Anticommutator => self.binary_multinomial_implementations(
                &ANTICOMMUTATOR,
                |[space_0, space_1], _| Class::Space(space_0.product_space(space_1)),
                Multinomial::new(self, [0, 1], |[blade_0, blade_1], _| {
                    match (blade_0.parity(blade_1), blade_1.parity(blade_0)) {
                        (Sign::Pos, Sign::Pos) => Some(Sign::Pos),
                        (Sign::Pos, Sign::Neg) => None,
                        (Sign::Neg, Sign::Pos) => None,
                        (Sign::Neg, Sign::Neg) => Some(Sign::Neg),
                    }
                }),
            ),

            Operation::Transform => self.binary_multinomial_implementations(
                &TRANSFORM,
                |[space_0, space_1], _| {
                    Class::Space(space_1.product_space(space_0).product_space(space_1))
                },
                Multinomial::new(self, [1, 0, 1], |[blade_0, blade_1, blade_2], _| {
                    Some(
                        blade_0.parity(blade_1)
                            ^ blade_0.parity(blade_2)
                            ^ blade_1.parity(blade_2)
                            ^ Sign::from_count(blade_2.grade() >> 1),
                    )
                }),
            ),

            Operation::Project => self.binary_multinomial_implementations(
                &PROJECT,
                |[space_0, space_1], _| {
                    Class::Space(match [space_0, space_1] {
                        [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                            Some(grade_0)
                                .filter(|&grade| grade <= grade_1)
                                .map(|grade| Space::GradedVector { grade })
                                .unwrap_or_else(Space::null)
                        }
                        [space_0, space_1] => space_0.product_space(space_1).product_space(space_1),
                    })
                },
                Multinomial::new(self, [0, 1, 1], |[blade_0, blade_1, blade_2], _| {
                    (blade_0 & blade_1 == blade_0
                        && (blade_0 ^ blade_1) & blade_2 == blade_0 ^ blade_1)
                        .then(|| {
                            blade_0.parity(blade_1)
                                ^ blade_0.parity(blade_2)
                                ^ blade_1.parity(blade_2)
                                ^ Sign::from_count(blade_2.grade() >> 1)
                        })
                }),
            ),

            Operation::Reject => self.binary_multinomial_implementations(
                &REJECT,
                |[space_0, space_1], dim| {
                    Class::Space(match [space_0, space_1] {
                        [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                            Some(grade_0)
                                .filter(|&grade| grade + grade_1 <= dim)
                                .map(|grade| Space::GradedVector { grade })
                                .unwrap_or_else(Space::null)
                        }
                        [space_0, space_1] => space_0.product_space(space_1).product_space(space_1),
                    })
                },
                Multinomial::new(self, [0, 1, 1], |[blade_0, blade_1, blade_2], _| {
                    (blade_0 & blade_1 == Blade::zero() && (blade_0 ^ blade_1) & blade_2 == blade_2)
                        .then(|| {
                            blade_0.parity(blade_1)
                                ^ blade_0.parity(blade_2)
                                ^ blade_1.parity(blade_2)
                                ^ Sign::from_count(blade_2.grade() >> 1)
                        })
                }),
            ),
        }
    }
}

struct GeometricAlgebraNames {
    blade_names: Vec<String>,
    graded_spaces: Vec<String>,
    null_space: String,
    even_space: String,
    odd_space: String,
    full_space: String,
}

struct GeometricAlgebraStringifier<'n> {
    names: &'n GeometricAlgebraNames,
    precision: &'static str,
}

impl Stringifier<GeometricAlgebra> for GeometricAlgebraStringifier<'_> {
    fn stringify_type(&self, r#type: &<GeometricAlgebra as Ast>::Type) -> &str {
        match r#type {
            Class::Base => self.precision,
            Class::Space(space) => self.stringify_template(space),
        }
    }

    fn stringify_template(&self, template: &<GeometricAlgebra as Ast>::Template) -> &str {
        match template {
            Space::GradedVector { grade } => &self.names.graded_spaces[*grade],
            Space::SaturatedVector {
                even: false,
                odd: false,
            } => &self.names.null_space,
            Space::SaturatedVector {
                even: true,
                odd: false,
            } => &self.names.even_space,
            Space::SaturatedVector {
                even: false,
                odd: true,
            } => &self.names.odd_space,
            Space::SaturatedVector {
                even: true,
                odd: true,
            } => &self.names.full_space,
        }
    }

    fn stringify_field(&self, field: &<GeometricAlgebra as Ast>::Field) -> &str {
        &self.names.blade_names[field.generator_bits]
    }

    fn stringify_operation_trait(&self, operation: &<GeometricAlgebra as Ast>::Operation) -> &str {
        operation.get_str("trait").unwrap()
    }

    fn stringify_operation_fn(&self, operation: &<GeometricAlgebra as Ast>::Operation) -> &str {
        operation.get_str("fn").unwrap()
    }
}

pub struct GeometricAlgeberaRecord {
    record: Record<GeometricAlgebra>,
    names: GeometricAlgebraNames,
}

impl GeometricAlgeberaRecord {
    pub fn write<Lang, Buffer>(
        &self,
        lang: Lang,
        buffer: &mut Buffer,
        precision: &'static str,
    ) -> std::io::Result<()>
    where
        Buffer: std::io::Write,
        Lang: Syntax,
    {
        let stringifier = GeometricAlgebraStringifier {
            names: &self.names,
            precision,
        };
        let mut writer = Writer::new(buffer);
        lang.emit_record(&mut writer, &stringifier, &self.record)
    }
}

pub struct GeometricAlgebraBuilder {
    alg: GeometricAlgebra,
    names: GeometricAlgebraNames,
}

impl GeometricAlgebraBuilder {
    pub fn new(
        generator_squares: impl IntoIterator<Item = Coefficient>,
        blades: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        let generator_squares = generator_squares.into_iter().collect_vec();
        let dim = generator_squares.len();
        let blades = blades.into_iter().collect_vec();
        assert_eq!(blades.len(), 1 << dim);
        let (blade_names, blade_intrinsic_signs): (Vec<_>, Vec<_>) = blades
            .into_iter()
            .map(|name| match name.as_ref().strip_prefix('-') {
                None => (name.as_ref().to_string(), Sign::Pos),
                Some(name) => (name.to_string(), Sign::Neg),
            })
            .unzip();

        Self {
            alg: GeometricAlgebra {
                dim,
                generator_squares,
                blade_intrinsic_signs,
            },
            names: GeometricAlgebraNames {
                blade_names,
                graded_spaces: (0..=dim)
                    .map(|grade| Self::space_name_default(Space::GradedVector { grade }))
                    .collect(),
                null_space: Self::space_name_default(Space::SaturatedVector {
                    even: false,
                    odd: false,
                }),
                even_space: Self::space_name_default(Space::SaturatedVector {
                    even: true,
                    odd: false,
                }),
                odd_space: Self::space_name_default(Space::SaturatedVector {
                    even: false,
                    odd: true,
                }),
                full_space: Self::space_name_default(Space::SaturatedVector {
                    even: true,
                    odd: true,
                }),
            },
        }
    }

    fn space_name_default(space: Space) -> String {
        const GRADED_SPACES: [&str; 13] = [
            "Scalar",
            "Vector",
            "Bivector",
            "Trivector",
            "FourVector",
            "FiveVector",
            "SixVector",
            "SevenVector",
            "EightVector",
            "NineVector",
            "TenVector",
            "ElevenVector",
            "TwelveVector",
        ];
        match space {
            Space::GradedVector { grade } => GRADED_SPACES
                .get(grade)
                .map_or_else(|| format!("Vector{grade}"), |name| name.to_string()),
            Space::SaturatedVector {
                even: false,
                odd: false,
            } => String::from("Null"),
            Space::SaturatedVector {
                even: true,
                odd: false,
            } => String::from("EvenMultivector"),
            Space::SaturatedVector {
                even: false,
                odd: true,
            } => String::from("OddMultivector"),
            Space::SaturatedVector {
                even: true,
                odd: true,
            } => String::from("Multivector"),
        }
    }

    pub fn graded_spaces(
        mut self,
        graded_spaces: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        let graded_spaces = graded_spaces
            .into_iter()
            .map(|graded_space| graded_space.as_ref().to_string())
            .collect_vec();
        assert_eq!(graded_spaces.len(), self.alg.dim + 1);
        self.names.graded_spaces = graded_spaces;
        self
    }

    pub fn saturated_spaces(
        mut self,
        null_space: impl AsRef<str>,
        even_space: impl AsRef<str>,
        odd_space: impl AsRef<str>,
        full_space: impl AsRef<str>,
    ) -> Self {
        self.names.null_space = null_space.as_ref().to_string();
        self.names.even_space = even_space.as_ref().to_string();
        self.names.odd_space = odd_space.as_ref().to_string();
        self.names.full_space = full_space.as_ref().to_string();
        self
    }

    pub fn build(self) -> GeometricAlgeberaRecord {
        GeometricAlgeberaRecord {
            record: self.alg.record(),
            names: self.names,
        }
    }
}
