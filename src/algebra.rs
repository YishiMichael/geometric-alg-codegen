use crate::ast::{
    Ast, Expr, Implementation, ImplementationBody, Item, OperationName, OperationSignature,
    Ownership, Record, Stmt, Stringifier, Structure, TemplateSignature, TypeBinding,
};
use crate::syntax::{Syntax, Writer};
use itertools::Itertools;
use strum::{EnumIter, EnumProperty, IntoEnumIterator};

type Coefficient = i32;

#[derive(Clone, Copy, PartialEq, Eq)]
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
    fn zero() -> Self {
        Self { generator_bits: 0 }
    }

    fn grade(self) -> usize {
        self.generator_bits.count_ones() as usize
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum Space {
    Homogeneous { grade: usize },
    Mixed { even: bool, odd: bool },
}

impl Space {
    fn null() -> Self {
        Self::Mixed {
            even: false,
            odd: false,
        }
    }

    fn contains(self, blade: Blade) -> bool {
        match self {
            Self::Homogeneous { grade } => blade.grade() == grade,
            Self::Mixed { even, odd } => {
                even && blade.grade() & 1 == 0 || odd && blade.grade() & 1 == 1
            }
        }
    }

    fn contains_even_grade(self) -> bool {
        matches!(self, Self::Homogeneous {grade} if grade & 1 == 0)
            || matches!(self, Self::Mixed { even: true, odd: _ })
    }

    fn contains_odd_grade(self) -> bool {
        matches!(self, Self::Homogeneous {grade} if grade & 1 == 1)
            || matches!(self, Self::Mixed { even: _, odd: true })
    }

    fn to_mixed(self) -> Self {
        Self::Mixed {
            even: self.contains_even_grade(),
            odd: self.contains_odd_grade(),
        }
    }

    fn sum_space<const N: usize>(spaces: [Self; N]) -> Self {
        if let Ok(space) = spaces.into_iter().all_equal_value() {
            space
        } else {
            Self::Mixed {
                even: spaces.into_iter().any(|space| space.contains_even_grade()),
                odd: spaces.into_iter().any(|space| space.contains_odd_grade()),
            }
        }
    }

    fn product_space<const N: usize>(spaces: [Self; N]) -> Self {
        Self::Mixed {
            even: std::iter::repeat_n([false, true], N)
                .multi_cartesian_product()
                .filter(|oddnesses| oddnesses.iter().fold(false, std::ops::BitXor::bitxor))
                .any(|oddnesses| {
                    oddnesses.iter().enumerate().all(|(index, &oddness)| {
                        if oddness {
                            spaces[index].contains_odd_grade()
                        } else {
                            spaces[index].contains_even_grade()
                        }
                    })
                }),
            odd: std::iter::repeat_n([false, true], N)
                .multi_cartesian_product()
                .filter(|oddnesses| oddnesses.iter().fold(true, std::ops::BitXor::bitxor))
                .any(|oddnesses| {
                    oddnesses.iter().enumerate().all(|(index, &oddness)| {
                        if oddness {
                            spaces[index].contains_odd_grade()
                        } else {
                            spaces[index].contains_even_grade()
                        }
                    })
                }),
        }
    }

    fn homogeneous_space<const N: usize>(
        spaces: [Self; N],
        homogeneous_fn: impl Fn([usize; N]) -> Option<usize>,
    ) -> Option<Self> {
        spaces
            .map(|space| match space {
                Self::Homogeneous { grade } => Some(grade),
                _ => None,
            })
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .map(
                |grades| match homogeneous_fn(grades.try_into().ok().unwrap()) {
                    Some(grade) => Self::Homogeneous { grade },
                    None => Self::null(),
                },
            )
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Class {
    Base,
    Space(Space),
}

// impl Class {
//     fn product_space(self, other: Self) -> Self {
//         fn contains_even_grade(space: Space) -> bool {
//             matches!(space, Space::Homogeneous {grade} if grade & 1 == 0)
//                 || matches!(space, Space::Mixed { even: true, odd: _ })
//         }

//         fn contains_odd_grade(space: Space) -> bool {
//             matches!(space, Space::Homogeneous {grade} if grade & 1 == 1)
//                 || matches!(space, Space::Mixed { even: _, odd: true })
//         }

//         Self::Mixed {
//             even: contains_even_grade(self) && contains_even_grade(other)
//                 || contains_odd_grade(self) && contains_odd_grade(other),
//             odd: contains_even_grade(self) && contains_odd_grade(other)
//                 || contains_odd_grade(self) && contains_even_grade(other),
//         }
//     }
// }

type NullaryConstructorSignature = OperationSignature<GeometricAlgebra, 0, 0, 0, 0, 1>;
type UnaryConstructorSignature = OperationSignature<GeometricAlgebra, 1, 0, 0, 1, 1>;
type UnarySignature = OperationSignature<GeometricAlgebra, 0, 1, 1, 0, 1>;
type BinarySignature = OperationSignature<GeometricAlgebra, 1, 1, 1, 1, 1>;
type UnaryInplaceSignature = OperationSignature<GeometricAlgebra, 0, 0, 1, 0, 0>;
type BinaryInplaceSignature = OperationSignature<GeometricAlgebra, 1, 0, 1, 1, 0>;

impl NullaryConstructorSignature {
    fn impl_for<Body>(
        &self,
        alg: &GeometricAlgebra,
        filter: impl Fn(<GeometricAlgebra as Ast>::Type) -> bool,
        body_fn: impl Fn(
            <GeometricAlgebra as Ast>::Type,
            [(
                <GeometricAlgebra as Ast>::Param,
                <GeometricAlgebra as Ast>::Type,
            ); 0],
        ) -> Body,
    ) -> Vec<Implementation<GeometricAlgebra>>
    where
        Body: Into<ImplementationBody<GeometricAlgebra>>,
    {
        self.implementations(
            alg.classes(),
            |_| std::iter::once([]),
            |class, []| filter(class).then_some([]),
            |class, [], [], [], []| body_fn(class, []),
        )
    }

    fn call(
        &self,
        class: <GeometricAlgebra as Ast>::Type,
        exprs: [(Expr<GeometricAlgebra>, <GeometricAlgebra as Ast>::Type); 0],
    ) -> Expr<GeometricAlgebra> {
        let [] = exprs;
        self.call_expr(class, [], [])
    }
}

impl UnaryConstructorSignature {
    fn impl_for<Body>(
        &self,
        alg: &GeometricAlgebra,
        filter: impl Fn(<GeometricAlgebra as Ast>::Type, [<GeometricAlgebra as Ast>::Type; 1]) -> bool,
        body_fn: impl Fn(
            <GeometricAlgebra as Ast>::Type,
            [(
                <GeometricAlgebra as Ast>::Param,
                <GeometricAlgebra as Ast>::Type,
            ); 1],
        ) -> Body,
    ) -> Vec<Implementation<GeometricAlgebra>>
    where
        Body: Into<ImplementationBody<GeometricAlgebra>>,
    {
        self.implementations(
            alg.classes(),
            |_| alg.classes().map(|class| [class]),
            |class, [class_0]| filter(class, [class_0]).then_some([]),
            |class, [class_0], [], [], [param_0]| body_fn(class, [(param_0, class_0)]),
        )
    }

    fn call(
        &self,
        class: <GeometricAlgebra as Ast>::Type,
        exprs: [(Expr<GeometricAlgebra>, <GeometricAlgebra as Ast>::Type); 1],
    ) -> Expr<GeometricAlgebra> {
        let [(expr_0, class_0)] = exprs;
        self.call_expr(class, [class_0], [expr_0])
    }
}

impl UnarySignature {
    fn impl_for<Body>(
        &self,
        alg: &GeometricAlgebra,
        associate_type: impl Fn(
            [<GeometricAlgebra as Ast>::Type; 1],
        ) -> Option<<GeometricAlgebra as Ast>::Type>,
        body_fn: impl Fn(
            <GeometricAlgebra as Ast>::Type,
            [(
                <GeometricAlgebra as Ast>::Param,
                <GeometricAlgebra as Ast>::Type,
            ); 1],
        ) -> Body,
    ) -> Vec<Implementation<GeometricAlgebra>>
    where
        Body: Into<ImplementationBody<GeometricAlgebra>>,
    {
        self.implementations(
            alg.classes(),
            |_| std::iter::once([]),
            |class_0, []| associate_type([class_0]).map(|class| [class]),
            |class_0, [], [class], [param_0], []| body_fn(class, [(param_0, class_0)]),
        )
    }

    fn call(
        &self,
        exprs: [(Expr<GeometricAlgebra>, <GeometricAlgebra as Ast>::Type); 1],
    ) -> Expr<GeometricAlgebra> {
        let [(expr_0, class_0)] = exprs;
        self.call_expr(class_0, [], expr_0, [])
    }
}

impl BinarySignature {
    fn impl_for<Body>(
        &self,
        alg: &GeometricAlgebra,
        associate_type: impl Fn(
            [<GeometricAlgebra as Ast>::Type; 2],
        ) -> Option<<GeometricAlgebra as Ast>::Type>,
        body_fn: impl Fn(
            <GeometricAlgebra as Ast>::Type,
            [(
                <GeometricAlgebra as Ast>::Param,
                <GeometricAlgebra as Ast>::Type,
            ); 2],
        ) -> Body,
    ) -> Vec<Implementation<GeometricAlgebra>>
    where
        Body: Into<ImplementationBody<GeometricAlgebra>>,
    {
        self.implementations(
            alg.classes(),
            |_| alg.classes().map(|class| [class]),
            |class_0, [class_1]| associate_type([class_0, class_1]).map(|class| [class]),
            |class_0, [class_1], [class], [param_0], [param_1]| {
                body_fn(class, [(param_0, class_0), (param_1, class_1)])
            },
        )
    }

    fn call(
        &self,
        exprs: [(Expr<GeometricAlgebra>, <GeometricAlgebra as Ast>::Type); 2],
    ) -> Expr<GeometricAlgebra> {
        let [(expr_0, class_0), (expr_1, class_1)] = exprs;
        self.call_expr(class_0, [class_1], expr_0, [expr_1])
    }
}

impl UnaryInplaceSignature {
    fn impl_for<Body>(
        &self,
        alg: &GeometricAlgebra,
        filter: impl Fn([<GeometricAlgebra as Ast>::Type; 1]) -> bool,
        body_fn: impl Fn(
            [(
                <GeometricAlgebra as Ast>::Param,
                <GeometricAlgebra as Ast>::Type,
            ); 1],
        ) -> Body,
    ) -> Vec<Implementation<GeometricAlgebra>>
    where
        Body: Into<ImplementationBody<GeometricAlgebra>>,
    {
        self.implementations(
            alg.classes(),
            |_| std::iter::once([]),
            |class_0, []| filter([class_0]).then_some([]),
            |class_0, [], [], [param_0], []| body_fn([(param_0, class_0)]),
        )
    }

    fn call(
        &self,
        exprs: [(Expr<GeometricAlgebra>, <GeometricAlgebra as Ast>::Type); 1],
    ) -> Stmt<GeometricAlgebra> {
        let [(expr_0, class_0)] = exprs;
        self.call_stmt(class_0, [], expr_0, [])
    }
}

impl BinaryInplaceSignature {
    fn impl_for<Body>(
        &self,
        alg: &GeometricAlgebra,
        filter: impl Fn([<GeometricAlgebra as Ast>::Type; 2]) -> bool,
        body_fn: impl Fn(
            [(
                <GeometricAlgebra as Ast>::Param,
                <GeometricAlgebra as Ast>::Type,
            ); 2],
        ) -> Body,
    ) -> Vec<Implementation<GeometricAlgebra>>
    where
        Body: Into<ImplementationBody<GeometricAlgebra>>,
    {
        self.implementations(
            alg.classes(),
            |_| alg.classes().map(|class| [class]),
            |class_0, [class_1]| filter([class_0, class_1]).then_some([]),
            |class_0, [class_1], [], [param_0], [param_1]| {
                body_fn([(param_0, class_0), (param_1, class_1)])
            },
        )
    }

    fn call(
        &self,
        exprs: [(Expr<GeometricAlgebra>, <GeometricAlgebra as Ast>::Type); 2],
    ) -> Stmt<GeometricAlgebra> {
        let [(expr_0, class_0), (expr_1, class_1)] = exprs;
        self.call_stmt(class_0, [class_1], expr_0, [expr_1])
    }
}

// impl NullaryConstructorSignature {
//     fn impl_for<Body>(
//         &self,
//         dim: usize,
//         associate_types: impl Fn([Class; 1]) -> Option<[Class; 0]>,
//         body_fn: impl Fn([Class; 1], [<GeometricAlgebra as Ast>::Param; 0]) -> Body,
//     ) -> Vec<Implementation<GeometricAlgebra>>
//     where
//         Body: Into<ImplementationBody<GeometricAlgebra>>,
//     {
//         self.implementations(
//             Class::iter(dim),
//             |_| [],
//             |self_class, []| associate_types([self_class]),
//             |self_class, [], [], [], []| body_fn([self_class], []),
//         )
//     }
// }

#[derive(Clone, Copy, EnumIter, EnumProperty)]
enum Operation {
    #[strum(props(trait = "Abs", fn = "abs", builtin = true))]
    Abs,
    #[strum(props(trait = "Sqrt", fn = "sqrt", builtin = true))]
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
    #[strum(props(trait = "Involute", fn = "involute"))]
    Involute,
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

impl OperationName for Operation {
    fn trait_name(&self) -> &str {
        self.get_str("trait").unwrap()
    }

    fn fn_name(&self) -> &str {
        self.get_str("fn").unwrap()
    }

    fn is_builtin(&self) -> bool {
        self.get_bool("builtin").unwrap_or_default()
    }
}

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
const INVOLUTE: UnarySignature = Operation::Involute.unary_signature();
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

struct Multinomial<const VARIABLE: usize, const DEGREE: usize> {
    polynomials: std::collections::BTreeMap<
        Blade,
        std::collections::BTreeMap<[(usize, Blade); DEGREE], Coefficient>,
    >,
}

impl<const VARIABLE: usize, const DEGREE: usize>
    FromIterator<(Blade, [(usize, Blade); DEGREE], Coefficient)> for Multinomial<VARIABLE, DEGREE>
{
    fn from_iter<I: IntoIterator<Item = (Blade, [(usize, Blade); DEGREE], Coefficient)>>(
        iter: I,
    ) -> Self {
        Self {
            polynomials: iter.into_iter().fold(
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
}

impl<const VARIABLE: usize, const DEGREE: usize> Multinomial<VARIABLE, DEGREE> {
    fn blade_expr(
        &self,
        blade: Blade,
        blade_expr_fn: impl Fn((usize, Blade)) -> Option<Expr<GeometricAlgebra>>,
    ) -> Option<Expr<GeometricAlgebra>> {
        self.polynomials.get(&blade).and_then(|polynomial| {
            polynomial
                .iter()
                .filter_map(|(multi_index, coeff)| {
                    multi_index
                        .iter()
                        .map(|&(variable, blade)| {
                            blade_expr_fn((variable, blade))
                            // classes[index].access(Expr::param(params[index]), blade)
                        })
                        .collect::<Option<Vec<_>>>()
                        .map(|exprs| (exprs.into_iter(), *coeff))
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
        })
    }

    fn body_fn(
        self,
        alg: &GeometricAlgebra,
    ) -> impl Fn(
        <GeometricAlgebra as Ast>::Type,
        [(
            <GeometricAlgebra as Ast>::Param,
            <GeometricAlgebra as Ast>::Type,
        ); VARIABLE],
    ) -> Expr<GeometricAlgebra> {
        move |class, param_items| {
            alg.construct(class, |blade| {
                self.blade_expr(blade, |(variable, blade)| {
                    let (param, class) = param_items[variable];
                    alg.access(class, blade, Expr::param(param))
                })
                .unwrap_or_else(|| Expr::literal(0))
            })
        }
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
        self.spaces().map(|space| {
            TemplateSignature { template: space }.structure(self.space_blades(space).map(|blade| {
                Item {
                    key: blade,
                    value: Class::Base,
                }
            }))
        })
    }

    fn implementations(&self) -> impl Iterator<Item = Implementation<Self>> {
        Operation::iter().flat_map(|operation| self.operation_implementations(operation))
    }
}

impl GeometricAlgebra {
    fn blades(&self) -> impl Iterator<Item = Blade> + Clone {
        (0..1 << self.dim).map(|generator_bits| Blade { generator_bits })
    }

    fn blade_generators(&self, blade: Blade) -> impl Iterator<Item = usize> {
        (0..self.dim).filter(move |generator| blade.generator_bits & 1 << generator != 0)
    }

    fn parity<const N: usize>(&self, blades: [Blade; N]) -> Sign {
        Sign::from_count(
            blades
                .iter()
                .enumerate()
                .flat_map(|(index, blade)| {
                    let gray_inv = (0..self.dim)
                        .map(|generator| blade.generator_bits >> generator)
                        .fold(0, std::ops::BitXor::bitxor);
                    blades[index + 1..].iter().map(move |blade| {
                        (gray_inv >> 1 & blade.generator_bits).count_ones() as usize
                    })
                })
                .sum(),
        )
    }

    fn spaces(&self) -> impl Iterator<Item = Space> {
        (0..=self.dim)
            .map(|grade| Space::Homogeneous { grade })
            .chain(
                [false, true]
                    .into_iter()
                    .cartesian_product([false, true])
                    .map(|(even, odd)| Space::Mixed { even, odd }),
            )
    }

    fn space_blades(&self, space: Space) -> impl Iterator<Item = Blade> {
        self.blades().filter(move |&blade| space.contains(blade))
    }

    fn dual_space(&self, space: Space) -> Space {
        match space {
            Space::Homogeneous { grade } => Space::Homogeneous {
                grade: self.dim - grade,
            },
            Space::Mixed { even, odd } if self.dim & 1 == 0 => Space::Mixed { even, odd },
            Space::Mixed { even, odd } => Space::Mixed {
                even: odd,
                odd: even,
            },
        }
    }

    fn classes(&self) -> impl Iterator<Item = Class> {
        std::iter::once(Class::Base).chain(self.spaces().map(Class::Space))
    }

    fn construct(
        &self,
        class: Class,
        field: impl Fn(Blade) -> Expr<GeometricAlgebra>,
    ) -> Expr<GeometricAlgebra> {
        match class {
            Class::Base => field(Blade::zero()),
            Class::Space(space) => TemplateSignature { template: space }.construct(
                self.space_blades(space).map(|blade| Item {
                    key: blade,
                    value: field(blade),
                }),
            ),
        }
    }

    fn access(
        &self,
        class: Class,
        blade: Blade,
        expr: Expr<GeometricAlgebra>,
    ) -> Option<Expr<GeometricAlgebra>> {
        match class {
            Class::Base => (blade == Blade::zero()).then_some(expr),
            Class::Space(space) => space
                .contains(blade)
                .then(|| TemplateSignature { template: space }.access(expr, blade)),
        }
    }

    fn access_deref(
        &self,
        class: Class,
        blade: Blade,
        expr: Expr<GeometricAlgebra>,
    ) -> Option<Expr<GeometricAlgebra>> {
        match class {
            Class::Base => (blade == Blade::zero()).then_some(expr.deref()),
            Class::Space(space) => space
                .contains(blade)
                .then(|| TemplateSignature { template: space }.access(expr, blade)),
        }
    }

    // fn construct(
    //     self,
    //     field: impl Fn(Blade) -> Expr<GeometricAlgebra>,
    // ) -> Expr<GeometricAlgebra> {

    // }

    // fn access(
    //     self,
    //     expr: Expr<GeometricAlgebra>,
    //     blade: Blade,
    // ) -> Option<Expr<GeometricAlgebra>> {
    //     self.contains(blade)
    //         .then(|| TemplateSignature { template: self }.access(expr, blade))
    // }

    // fn multinomial_filter_metric

    fn multinomial<const VARIABLE: usize, const DEGREE: usize>(
        &self,
        prototype: [usize; DEGREE],
        input_blades_remap: impl Fn(Blade) -> (Sign, Blade),
        term_sign: impl Fn([Blade; DEGREE]) -> Option<Sign>,
        output_blade_remap: impl Fn(Blade) -> (Sign, Blade),
    ) -> Multinomial<VARIABLE, DEGREE> {
        std::iter::repeat_n(self.blades(), DEGREE)
            .multi_cartesian_product()
            // .map(|blades| -> [Blade; DEGREE] { blades.try_into().ok().unwrap() })
            .filter_map(|blades| {
                let (input_signs, input_blades): (Vec<_>, Vec<_>) = blades
                    .iter()
                    .map(|&blade| input_blades_remap(blade))
                    .unzip();
                let input_blades = input_blades.try_into().ok().unwrap();
                term_sign(input_blades).map(|term_sign| {
                    // let multi_index: [_; DEGREE] = prototype
                    //     .into_iter()
                    //     .zip(blades)
                    //     .sorted()
                    //     .collect_vec()
                    //     .try_into()
                    //     .ok()
                    //     .unwrap();
                    // let input_sign = input_signs
                    //     .into_iter()
                    //     .chain(input_blades.iter().map(|input_blade| {
                    //         self.blade_intrinsic_signs[input_blade.generator_bits]
                    //     }))
                    //     .fold(
                    //         self.parity(input_blades.try_into().ok().unwrap()),
                    //         std::ops::BitXor::bitxor,
                    //     );
                    let (output_blade, square_product) = input_blades
                        .into_iter()
                        // .map(|input_blade| {
                        //     let (input_sign, input_blade) = input_blades_remap[index](blade);
                        //     (
                        //         Coefficient::from(
                        //             input_sign
                        //                 ^ self.blade_intrinsic_signs[input_blade.generator_bits],
                        //         ),
                        //         input_blade,
                        //     )
                        // })
                        .fold(
                            (Blade::zero(), 1),
                            |(output_blade, square_product), input_blade| {
                                (
                                    output_blade ^ input_blade,
                                    square_product
                                        * self
                                            .blade_generators(output_blade & input_blade)
                                            .map(|generator| self.generator_squares[generator])
                                            .product::<Coefficient>(),
                                )
                            },
                        );
                    let (output_sign, blade) = output_blade_remap(output_blade);
                    let coeff = square_product
                        * Coefficient::from(
                            self.parity(input_blades)
                                ^ term_sign
                                ^ input_signs
                                    .into_iter()
                                    .fold(output_sign, std::ops::BitXor::bitxor)
                                ^ blades
                                    .iter()
                                    .map(|blade| self.blade_intrinsic_signs[blade.generator_bits])
                                    .fold(
                                        self.blade_intrinsic_signs[blade.generator_bits],
                                        std::ops::BitXor::bitxor,
                                    ),
                        );
                    (
                        blade,
                        prototype
                            .into_iter()
                            .zip(blades)
                            .sorted()
                            .collect_vec()
                            .try_into()
                            .ok()
                            .unwrap(),
                        coeff,
                    )
                })
            })
            .collect()
    }

    fn identity_blade_remap(&self) -> impl Fn(Blade) -> (Sign, Blade) {
        |blade| (Sign::Pos, blade)
    }

    fn dual_blade_remap(&self) -> impl Fn(Blade) -> (Sign, Blade) {
        |blade| {
            let complement_blade = Blade {
                generator_bits: ((1 << self.dim) - 1) ^ blade.generator_bits,
            };
            (self.parity([blade, complement_blade]), complement_blade)
        }
    }

    fn undual_blade_remap(&self) -> impl Fn(Blade) -> (Sign, Blade) {
        |blade| {
            let complement_blade = Blade {
                generator_bits: ((1 << self.dim) - 1) ^ blade.generator_bits,
            };
            (self.parity([complement_blade, blade]), complement_blade)
        }
    }

    // fn dual(self) -> Self {
    //     Self {
    //         alg: self.alg,
    //         polynomials: self
    //             .polynomials
    //             .into_iter()
    //             .map(|(blade, polynomial)| (blade.dual_blade(self.alg.dim), polynomial)) // TODO: change sign by blade.parity(!blade) ?
    //             .collect(),
    //     }
    // }

    // fn unary_operator_implementations(
    //     &self,
    //     signature: &UnarySignature,
    //     op: fn(Expr<GeometricAlgebra>) -> Expr<GeometricAlgebra>,
    // ) -> Vec<Implementation<GeometricAlgebra>> {
    //     Space::spaces(self.dim)
    //         .map(|space_0| {
    //             signature.implementation(
    //                 Class::Space(space_0),
    //                 [],
    //                 [Class::Space(space_0)],
    //                 |[param_0], []| {
    //                     space_0.construct(self.dim, |blade| {
    //                         op(space_0.access(Expr::param(param_0), blade))
    //                     })
    //                 },
    //             )
    //         })
    //         .collect()
    // }

    // fn binary_operator_implementations(
    //     &self,
    //     signature: &BinarySignature,
    //     op: fn(Expr<GeometricAlgebra>, Expr<GeometricAlgebra>) -> Expr<GeometricAlgebra>,
    // ) -> Vec<Implementation<GeometricAlgebra>> {
    //     signature.implementations(
    //         Space::spaces(self.dim).map(|space| (Class::Space(space), [])),
    //         |&class_0, &[]| match class_0 {
    //             Class::Space(space_0) => [class_fn([space_0])],
    //             _ => unreachable!(),
    //         },
    //         |&class_0, &[], &[class], [param_0], []| {
    //             class.construct(self.dim, |blade| {
    //                 multinomial.blade_expr(
    //                     blade,
    //                     [Box::new(move |blade_0| {
    //                         class_0.access(Expr::param(param_0), blade_0)
    //                     })],
    //                 )
    //             })
    //         },
    //     )
    //     // Space::iter(self.dim)
    //     //     .map(|space| {
    //     //         signature.implementation(
    //     //             Class::Space(space),
    //     //             [Class::Space(space)],
    //     //             [Class::Space(space)],
    //     //             |[param_0], [param_1]| {
    //     //                 space.construct(self.dim, |blade| {
    //     //                     op(
    //     //                         space.access(Expr::param(param_0), blade),
    //     //                         space.access(Expr::param(param_1), blade),
    //     //                     )
    //     //                 })
    //     //             },
    //     //         )
    //     //     })
    //     //     .chain(Space::iter(self.dim).map(|space| {
    //     //         signature.implementation(
    //     //             Class::Space(space),
    //     //             [Class::Base],
    //     //             [Class::Space(space)],
    //     //             |[param_0], [param_1]| {
    //     //                 space.construct(self.dim, |blade| {
    //     //                     op(
    //     //                         space.access(Expr::param(param_0), blade),
    //     //                         Expr::param(param_1),
    //     //                     )
    //     //                 })
    //     //             },
    //     //         )
    //     //     }))
    //     //     .chain(Space::iter(self.dim).map(|space| {
    //     //         signature.implementation(
    //     //             Class::Base,
    //     //             [Class::Space(space)],
    //     //             [Class::Space(space)],
    //     //             |[param_0], [param_1]| {
    //     //                 space.construct(self.dim, |blade| {
    //     //                     op(
    //     //                         Expr::param(param_0),
    //     //                         space.access(Expr::param(param_1), blade),
    //     //                     )
    //     //                 })
    //     //             },
    //     //         )
    //     //     }))
    //     //     .collect()
    // }

    // fn binary_inplace_operator_implementations(
    //     &self,
    //     signature: &BinaryInplaceSignature,
    //     op: fn(Expr<GeometricAlgebra>, Expr<GeometricAlgebra>) -> Stmt<GeometricAlgebra>,
    // ) -> Vec<Implementation<GeometricAlgebra>> {
    //     Space::spaces(self.dim)
    //         .map(|space| {
    //             signature.implementation(
    //                 Class::Space(space),
    //                 [Class::Space(space)],
    //                 [],
    //                 |[param_0], [param_1]| {
    //                     space
    //                         .space_blades(self.dim)
    //                         .map(|blade| {
    //                             op(
    //                                 space.access(Expr::param(param_0), blade),
    //                                 space.access(Expr::param(param_1), blade),
    //                             )
    //                         })
    //                         .collect_vec()
    //                 },
    //             )
    //         })
    //         .chain(Space::spaces(self.dim).map(|space| {
    //             signature.implementation(
    //                 Class::Space(space),
    //                 [Class::Base],
    //                 [],
    //                 |[param_0], [param_1]| {
    //                     space
    //                         .space_blades(self.dim)
    //                         .map(|blade| {
    //                             op(
    //                                 space.access(Expr::param(param_0), blade),
    //                                 Expr::param(param_1),
    //                             )
    //                         })
    //                         .collect_vec()
    //                 },
    //             )
    //         }))
    //         .collect()
    // }

    // fn unary_multinomial_implementations<const DEGREE: usize>(
    //     &self,
    //     signature: &UnarySignature,
    //     class_fn: fn([Space; 1]) -> Class,
    //     multinomial: Multinomial<UnaryVariable, DEGREE>,
    // ) -> Vec<Implementation<GeometricAlgebra>> {
    //     signature.implementations(
    //         self.classes(),
    //         |_| std::iter::once([]),
    //         |class_0, []| match class_0 {
    //             Class::Space(space_0) => Some([class_fn([space_0])]),
    //             _ => None,
    //         },
    //         |class_0, [], [class], [param_0], []| {
    //             self.construct(class, |blade| {
    //                 multinomial.blade_expr(blade, |item| match item {
    //                     (UnaryVariable::Index0, blade_0) => {
    //                         self.access(class_0, blade_0, Expr::param(param_0))
    //                     }
    //                 })
    //             })
    //         },
    //     )
    // }

    // fn binary_multinomial_implementations<const DEGREE: usize>(
    //     &self,
    //     signature: &BinarySignature,
    //     class_fn: fn([Space; 2]) -> Class,
    //     multinomial: Multinomial<BinaryVariable, DEGREE>,
    // ) -> Vec<Implementation<GeometricAlgebra>> {
    //     signature.implementations(
    //         self.classes(),
    //         |_| self.classes().map(|class| [class]),
    //         |class_0, [class_1]| match (class_0, class_1) {
    //             (Class::Space(space_0), Class::Space(space_1)) => {
    //                 Some([class_fn([space_0, space_1])])
    //             }
    //             _ => None,
    //         },
    //         |class_0, [class_1], [class], [param_0], [param_1]| {
    //             self.construct(class, |blade| {
    //                 multinomial.blade_expr(blade, |item| match item {
    //                     (BinaryVariable::Index0, blade_0) => {
    //                         self.access(class_0, blade_0, Expr::param(param_0))
    //                     }
    //                     (BinaryVariable::Index1, blade_1) => {
    //                         self.access(class_1, blade_1, Expr::param(param_1))
    //                     }
    //                 })
    //             })
    //         },
    //     )
    // }

    // fn transform_multinomial_implementations<const DEGREE: usize>(
    //     &self,
    //     signature: &BinarySignature,
    //     class_fn: fn([Space; 2]) -> Class,
    //     multinomial: Multinomial<BinaryVariable, DEGREE>,
    // ) -> Vec<Implementation<GeometricAlgebra>> {
    //     signature.implementations(
    //         self.classes(),
    //         |_| self.classes().map(|class| [class]),
    //         |class_0, [class_1]| match (class_0, class_1) {
    //             (Class::Space(space_0), Class::Space(space_1)) => {
    //                 Some([class_fn([space_0, space_1])])
    //             }
    //             _ => None,
    //         },
    //         |class_0, [class_1], [class], [param_0], [param_1]| {
    //             self.construct(class, |blade| {
    //                 multinomial.blade_expr(blade, |item| match item {
    //                     (BinaryVariable::Index0, blade_0) => (blade.grade() == blade_0.grade())
    //                         .then(|| self.access(class_0, blade_0, Expr::param(param_0)))
    //                         .flatten(),
    //                     (BinaryVariable::Index1, blade_1) => {
    //                         self.access(class_1, blade_1, Expr::param(param_1))
    //                     }
    //                 })
    //             })
    //         },
    //     )
    // }

    // fn unary_inherited_implementations(
    //     &self,
    //     signature: &UnarySignature,
    //     class_fn: fn([Space; 1]) -> Class,
    //     expr_fn: fn([Space; 1], [<GeometricAlgebra as Ast>::Param; 1]) -> Expr<GeometricAlgebra>,
    // ) -> Vec<Implementation<GeometricAlgebra>> {
    //     Space::spaces(self.dim)
    //         .map(|space_0| {
    //             let class = class_fn([space_0]);
    //             signature.implementation(Class::Space(space_0), [], [class], |[param_0], []| {
    //                 expr_fn([space_0], [param_0])
    //             })
    //         })
    //         .collect()
    // }

    // fn unary_inplace_inherited_implementations(
    //     &self,
    //     signature: &UnaryInplaceSignature,
    //     stmts_fn: fn(
    //         [Space; 1],
    //         [<GeometricAlgebra as Ast>::Param; 1],
    //     ) -> Vec<Stmt<GeometricAlgebra>>,
    // ) -> Vec<Implementation<GeometricAlgebra>> {
    //     Space::spaces(self.dim)
    //         .map(|space_0| {
    //             signature.implementation(Class::Space(space_0), [], [], |[param_0], []| {
    //                 stmts_fn([space_0], [param_0])
    //             })
    //         })
    //         .collect()
    // }

    fn operation_implementations(
        &self,
        operation: Operation,
    ) -> Vec<Implementation<GeometricAlgebra>> {
        match operation {
            Operation::Abs => Vec::new(),

            Operation::Sqrt => Vec::new(),

            Operation::From => FROM.impl_for(
                self,
                |class, [class_0]| class != class_0,
                |class, [(param_0, class_0)]| {
                    self.construct(class, |blade| {
                        self.access(class_0, blade, Expr::param(param_0))
                            .unwrap_or_else(|| Expr::literal(0))
                    })
                },
            ),

            Operation::Zero => ZERO.impl_for(
                self,
                |class| matches!(class, Class::Space(_)),
                |class, []| self.construct(class, |_| Expr::literal(0)),
            ),

            Operation::One => ONE.impl_for(
                self,
                |class| matches!(class, Class::Space(space) if space.contains(Blade::zero())),
                self.multinomial(
                    [],
                    self.identity_blade_remap(),
                    |[]| Some(Sign::Pos),
                    self.identity_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::Neg => NEG.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(space_0)] => Some(Class::Space(space_0)),
                    _ => None,
                },
                |class, [(param_0, class_0)]| {
                    self.construct(class, |blade| {
                        -self.access(class_0, blade, Expr::param(param_0)).unwrap()
                    })
                },
            ),

            Operation::Add => ADD.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Base, Class::Space(space_1)] => Some(Class::Space(Space::sum_space([
                        Space::Homogeneous { grade: 0 },
                        space_1,
                    ]))),
                    [Class::Space(space_0), Class::Base] => Some(Class::Space(Space::sum_space([
                        space_0,
                        Space::Homogeneous { grade: 0 },
                    ]))),
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(Space::sum_space([space_0, space_1])))
                    }
                    _ => None,
                },
                |class, [(param_0, class_0), (param_1, class_1)]| {
                    self.construct(class, |blade| {
                        match (
                            self.access(class_0, blade, Expr::param(param_0)),
                            self.access(class_1, blade, Expr::param(param_1)),
                        ) {
                            (None, None) => Expr::literal(0),
                            (None, Some(expr_1)) => expr_1,
                            (Some(expr_0), None) => expr_0,
                            (Some(expr_0), Some(expr_1)) => expr_0 + expr_1,
                        }
                    })
                },
            ),

            Operation::Sub => SUB.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Base, Class::Space(space_1)] => Some(Class::Space(Space::sum_space([
                        Space::Homogeneous { grade: 0 },
                        space_1,
                    ]))),
                    [Class::Space(space_0), Class::Base] => Some(Class::Space(Space::sum_space([
                        space_0,
                        Space::Homogeneous { grade: 0 },
                    ]))),
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(Space::sum_space([space_0, space_1])))
                    }
                    _ => None,
                },
                |class, [(param_0, class_0), (param_1, class_1)]| {
                    self.construct(class, |blade| {
                        match (
                            self.access(class_0, blade, Expr::param(param_0)),
                            self.access(class_1, blade, Expr::param(param_1)),
                        ) {
                            (None, None) => Expr::literal(0),
                            (None, Some(expr_1)) => -expr_1,
                            (Some(expr_0), None) => expr_0,
                            (Some(expr_0), Some(expr_1)) => expr_0 - expr_1,
                        }
                    })
                },
            ),

            Operation::Mul => MUL.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Base] => Some(Class::Space(space_0)),
                    [Class::Base, Class::Space(space_1)] => Some(Class::Space(space_1)),
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(Space::product_space([space_0, space_1])))
                    }
                    _ => None,
                },
                |class, [(param_0, class_0), (param_1, class_1)]| match [class_0, class_1] {
                    [Class::Space(_), Class::Base] => self.construct(class, |blade| {
                        self.access(class_0, blade, Expr::param(param_0)).unwrap()
                            * Expr::param(param_1)
                    }),
                    [Class::Base, Class::Space(_)] => self.construct(class, |blade| {
                        Expr::param(param_0)
                            * self.access(class_1, blade, Expr::param(param_1)).unwrap()
                    }),
                    [Class::Space(_), Class::Space(_)] => GEOMETRIC_PRODUCT.call([
                        (Expr::param(param_0), class_0),
                        (Expr::param(param_1), class_1),
                    ]),
                    _ => unreachable!(),
                },
            ),

            Operation::Div => DIV.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Base] => Some(Class::Space(space_0)),
                    [Class::Base, Class::Space(space_1)] => Some(Class::Space(space_1.to_mixed())),
                    _ => None,
                },
                |class, [(param_0, class_0), (param_1, class_1)]| match [class_0, class_1] {
                    [Class::Space(_), Class::Base] => self.construct(class, |blade| {
                        self.access(class_0, blade, Expr::param(param_0)).unwrap()
                            / Expr::param(param_1)
                    }),
                    [Class::Base, Class::Space(space_1)] => MUL.call([
                        (Expr::param(param_0), Class::Base),
                        (
                            INVERSE.call([(Expr::param(param_1), class_1)]),
                            Class::Space(space_1.to_mixed()),
                        ),
                    ]),
                    _ => unreachable!(),
                },
            ),

            Operation::AddAssign => ADD_ASSIGN.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Base, Class::Space(space_1)] => {
                        Space::sum_space([Space::Homogeneous { grade: 0 }, space_1])
                            == Space::Homogeneous { grade: 0 }
                    }
                    [Class::Space(space_0), Class::Base] => {
                        Space::sum_space([space_0, Space::Homogeneous { grade: 0 }]) == space_0
                    }
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Space::sum_space([space_0, space_1]) == space_0
                    }
                    _ => false,
                },
                |[(param_0, class_0), (param_1, class_1)]| {
                    match class_1 {
                        Class::Base => Vec::from([Blade::zero()]),
                        Class::Space(space_1) => self.space_blades(space_1).collect(),
                    }
                    .into_iter()
                    .map(|blade| {
                        self.access_deref(class_0, blade, Expr::param(param_0))
                            .unwrap()
                            .add_assign(self.access(class_1, blade, Expr::param(param_1)).unwrap())
                    })
                    .collect_vec()
                },
            ),

            Operation::SubAssign => SUB_ASSIGN.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Base, Class::Space(space_1)] => {
                        Space::sum_space([Space::Homogeneous { grade: 0 }, space_1])
                            == Space::Homogeneous { grade: 0 }
                    }
                    [Class::Space(space_0), Class::Base] => {
                        Space::sum_space([space_0, Space::Homogeneous { grade: 0 }]) == space_0
                    }
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Space::sum_space([space_0, space_1]) == space_0
                    }
                    _ => false,
                },
                |[(param_0, class_0), (param_1, class_1)]| {
                    match class_1 {
                        Class::Base => Vec::from([Blade::zero()]),
                        Class::Space(space_1) => self.space_blades(space_1).collect(),
                    }
                    .into_iter()
                    .map(|blade| {
                        self.access_deref(class_0, blade, Expr::param(param_0))
                            .unwrap()
                            .sub_assign(self.access(class_1, blade, Expr::param(param_1)).unwrap())
                    })
                    .collect_vec()
                },
            ),

            Operation::MulAssign => MUL_ASSIGN.impl_for(
                self,
                |[class_0, class_1]| matches!([class_0, class_1], [Class::Space(_), Class::Base]),
                |[(param_0, class_0), (param_1, _)]| match class_0 {
                    Class::Space(space_0) => self
                        .space_blades(space_0)
                        .map(|blade| {
                            self.access_deref(class_0, blade, Expr::param(param_0))
                                .unwrap()
                                .mul_assign(Expr::param(param_1))
                        })
                        .collect_vec(),
                    _ => unreachable!(),
                },
            ),

            Operation::DivAssign => DIV_ASSIGN.impl_for(
                self,
                |[class_0, class_1]| matches!([class_0, class_1], [Class::Space(_), Class::Base]),
                |[(param_0, class_0), (param_1, _)]| match class_0 {
                    Class::Space(space_0) => self
                        .space_blades(space_0)
                        .map(|blade| {
                            self.access_deref(class_0, blade, Expr::param(param_0))
                                .unwrap()
                                .div_assign(Expr::param(param_1))
                        })
                        .collect_vec(),
                    _ => unreachable!(),
                },
            ),

            Operation::Involute => INVOLUTE.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(space_0)] => Some(Class::Space(space_0)),
                    _ => None,
                },
                self.multinomial(
                    [0],
                    self.identity_blade_remap(),
                    |[blade_0]| Some(Sign::from_count(blade_0.grade())),
                    self.identity_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::Reverse => REVERSE.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(space_0)] => Some(Class::Space(space_0)),
                    _ => None,
                },
                self.multinomial(
                    [0],
                    self.identity_blade_remap(),
                    |[blade_0]| Some(Sign::from_count(blade_0.grade() >> 1)),
                    self.identity_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::Conjugate => CONJUGATE.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(space_0)] => Some(Class::Space(space_0)),
                    _ => None,
                },
                self.multinomial(
                    [0],
                    self.identity_blade_remap(),
                    |[blade_0]| Some(Sign::from_count((blade_0.grade() + 1) >> 1)),
                    self.identity_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::Dual => DUAL.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(space_0)] => Some(Class::Space(self.dual_space(space_0))),
                    _ => None,
                },
                self.multinomial(
                    [0],
                    self.identity_blade_remap(),
                    |_| Some(Sign::Pos),
                    self.dual_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::Undual => UNDUAL.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(space_0)] => Some(Class::Space(self.dual_space(space_0))),
                    _ => None,
                },
                self.multinomial(
                    [0],
                    self.identity_blade_remap(),
                    |_| Some(Sign::Pos),
                    self.undual_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::NormSquared => NORM_SQUARED.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(_)] => Some(Class::Base),
                    _ => None,
                },
                self.multinomial(
                    [0, 0],
                    self.identity_blade_remap(),
                    |[blade_0, blade_1]| {
                        (blade_0 == blade_1).then_some(Sign::from_count(blade_1.grade() >> 1))
                    },
                    self.undual_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::Norm => NORM.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(_)] => Some(Class::Base),
                    _ => None,
                },
                |_, [(param_0, class_0)]| {
                    SQRT.call([(
                        ABS.call([(
                            NORM_SQUARED.call([(Expr::param(param_0), class_0)]),
                            Class::Base,
                        )]),
                        Class::Base,
                    )])
                },
            ),

            Operation::Inverse => Vec::new(), // TODO

            Operation::Normalized => NORMALIZED.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(space_0)] => Some(Class::Space(space_0)),
                    _ => None,
                },
                |_, [(param_0, class_0)]| {
                    DIV.call([
                        (Expr::param(param_0), class_0),
                        (NORM.call([(Expr::param(param_0), class_0)]), Class::Base),
                    ])
                },
            ),

            Operation::Normalize => NORMALIZE.impl_for(
                self,
                |[class_0]| matches!(class_0, Class::Space(_)),
                |[(param_0, class_0)]| {
                    Vec::from([DIV_ASSIGN.call([
                        (Expr::param(param_0), class_0),
                        (NORM.call([(Expr::param(param_0), class_0)]), Class::Base),
                    ])])
                },
            ),

            Operation::GeometricProduct => GEOMETRIC_PRODUCT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(Space::product_space([space_0, space_1])))
                    }
                    _ => None,
                },
                self.multinomial(
                    [0, 1],
                    self.identity_blade_remap(),
                    |_| Some(Sign::Pos),
                    self.identity_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::ScalarProduct => SCALAR_PRODUCT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(_), Class::Space(_)] => Some(Class::Base),
                    _ => None,
                },
                self.multinomial(
                    [0, 1],
                    self.identity_blade_remap(),
                    |[blade_0, blade_1]| (blade_0 == blade_1).then_some(Sign::Pos),
                    self.identity_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::LeftInnerProduct => LEFT_INNER_PRODUCT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => Some(Class::Space(
                        Space::homogeneous_space([space_0, space_1], |[grade_0, grade_1]| {
                            grade_1.checked_sub(grade_0)
                        })
                        .unwrap_or_else(|| Space::product_space([space_0, space_1])),
                    )),
                    _ => None,
                },
                self.multinomial(
                    [0, 1],
                    self.identity_blade_remap(),
                    |[blade_0, blade_1]| (blade_0 & blade_1 == blade_0).then_some(Sign::Pos),
                    self.identity_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::RightInnerProduct => RIGHT_INNER_PRODUCT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => Some(Class::Space(
                        Space::homogeneous_space([space_0, space_1], |[grade_0, grade_1]| {
                            grade_0.checked_sub(grade_1)
                        })
                        .unwrap_or_else(|| Space::product_space([space_0, space_1])),
                    )),
                    _ => None,
                },
                self.multinomial(
                    [0, 1],
                    self.identity_blade_remap(),
                    |[blade_0, blade_1]| (blade_0 & blade_1 == blade_1).then_some(Sign::Pos),
                    self.identity_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::InnerProduct => INNER_PRODUCT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => Some(Class::Space(
                        Space::homogeneous_space([space_0, space_1], |[grade_0, grade_1]| {
                            Some(grade_0.abs_diff(grade_1))
                        })
                        .unwrap_or_else(|| Space::product_space([space_0, space_1])),
                    )),
                    _ => None,
                },
                self.multinomial(
                    [0, 1],
                    self.identity_blade_remap(),
                    |[blade_0, blade_1]| {
                        (blade_0 & blade_1 == blade_0 || blade_0 & blade_1 == blade_1)
                            .then_some(Sign::Pos)
                    },
                    self.identity_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::OuterProduct => OUTER_PRODUCT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => Some(Class::Space(
                        Space::homogeneous_space([space_0, space_1], |[grade_0, grade_1]| {
                            Some(grade_0 + grade_1).filter(|&grade| grade <= self.dim)
                        })
                        .unwrap_or_else(|| Space::product_space([space_0, space_1])),
                    )),
                    _ => None,
                },
                self.multinomial(
                    [0, 1],
                    self.identity_blade_remap(),
                    |[blade_0, blade_1]| (blade_0 & blade_1 == Blade::zero()).then_some(Sign::Pos),
                    self.identity_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::RegressiveProduct => REGRESSIVE_PRODUCT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => Some(Class::Space(
                        self.dual_space(
                            Space::homogeneous_space(
                                [self.dual_space(space_0), self.dual_space(space_1)],
                                |[grade_0, grade_1]| {
                                    Some(grade_0 + grade_1).filter(|&grade| grade <= self.dim)
                                },
                            )
                            .unwrap_or_else(|| {
                                Space::product_space([
                                    self.dual_space(space_0),
                                    self.dual_space(space_1),
                                ])
                            }),
                        ),
                    )),
                    _ => None,
                },
                self.multinomial(
                    [0, 1],
                    self.dual_blade_remap(),
                    |[blade_0, blade_1]| (blade_0 & blade_1 == Blade::zero()).then_some(Sign::Pos),
                    self.undual_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::Commutator => COMMUTATOR.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(Space::product_space([space_0, space_1])))
                    }
                    _ => None,
                },
                self.multinomial(
                    [0, 1],
                    self.identity_blade_remap(),
                    |[blade_0, blade_1]| {
                        (self.parity([blade_0, blade_1]) != self.parity([blade_1, blade_0]))
                            .then_some(Sign::Pos)
                    },
                    self.identity_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::Anticommutator => ANTICOMMUTATOR.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(Space::product_space([space_0, space_1])))
                    }
                    _ => None,
                },
                self.multinomial(
                    [0, 1],
                    self.identity_blade_remap(),
                    |[blade_0, blade_1]| {
                        (self.parity([blade_0, blade_1]) == self.parity([blade_1, blade_0]))
                            .then_some(Sign::Pos)
                    },
                    self.identity_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::Transform => TRANSFORM.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(_)] => {
                        Space::homogeneous_space([space_0], |[grade_0]| Some(grade_0))
                            .map(Class::Space)
                    }
                    _ => None,
                },
                self.multinomial(
                    [1, 0, 1],
                    self.identity_blade_remap(),
                    |[blade_0, blade_1, blade_2]| {
                        Some(
                            Sign::from_count(blade_0.grade())
                                ^ Sign::from_count(blade_1.grade())
                                ^ Sign::from_count(blade_2.grade() >> 1),
                        )
                    },
                    self.identity_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::Project => PROJECT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Space::homogeneous_space([space_0, space_1], |[grade_0, grade_1]| {
                            (grade_0 <= grade_1).then_some(grade_0)
                        })
                        .or_else(|| Space::homogeneous_space([space_0], |[grade_0]| Some(grade_0)))
                        .map(Class::Space)
                    }
                    _ => None,
                },
                self.multinomial(
                    [0, 1, 1],
                    self.identity_blade_remap(),
                    |[blade_0, blade_1, blade_2]| {
                        (blade_0 & blade_1 == blade_0
                            && (blade_0 ^ blade_1) & blade_2 == blade_0 ^ blade_1)
                            .then(|| {
                                Sign::from_count(blade_0.grade())
                                    ^ Sign::from_count(blade_1.grade())
                                    ^ Sign::from_count(blade_2.grade() >> 1)
                            })
                    },
                    self.identity_blade_remap(),
                )
                .body_fn(self),
            ),

            Operation::Reject => REJECT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Space::homogeneous_space([space_0, space_1], |[grade_0, grade_1]| {
                            (grade_0 + grade_1 <= self.dim).then_some(grade_0)
                        })
                        .or_else(|| Space::homogeneous_space([space_0], |[grade_0]| Some(grade_0)))
                        .map(Class::Space)
                    }
                    _ => None,
                },
                self.multinomial(
                    [0, 1, 1],
                    self.identity_blade_remap(),
                    |[blade_0, blade_1, blade_2]| {
                        (blade_0 & blade_1 == Blade::zero()
                            && (blade_0 ^ blade_1) & blade_2 == blade_2)
                            .then(|| {
                                Sign::from_count(blade_0.grade())
                                    ^ Sign::from_count(blade_1.grade())
                                    ^ Sign::from_count(blade_2.grade() >> 1)
                            })
                    },
                    self.identity_blade_remap(),
                )
                .body_fn(self),
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
            Space::Homogeneous { grade } => &self.names.graded_spaces[*grade],
            Space::Mixed {
                even: false,
                odd: false,
            } => &self.names.null_space,
            Space::Mixed {
                even: true,
                odd: false,
            } => &self.names.even_space,
            Space::Mixed {
                even: false,
                odd: true,
            } => &self.names.odd_space,
            Space::Mixed {
                even: true,
                odd: true,
            } => &self.names.full_space,
        }
    }

    fn stringify_field(&self, field: &<GeometricAlgebra as Ast>::Field) -> &str {
        &self.names.blade_names[field.generator_bits]
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
                    .map(|grade| Self::space_name_default(Space::Homogeneous { grade }))
                    .collect(),
                null_space: Self::space_name_default(Space::Mixed {
                    even: false,
                    odd: false,
                }),
                even_space: Self::space_name_default(Space::Mixed {
                    even: true,
                    odd: false,
                }),
                odd_space: Self::space_name_default(Space::Mixed {
                    even: false,
                    odd: true,
                }),
                full_space: Self::space_name_default(Space::Mixed {
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
            Space::Homogeneous { grade } => GRADED_SPACES
                .get(grade)
                .map_or_else(|| format!("Vector{grade}"), |name| name.to_string()),
            Space::Mixed {
                even: false,
                odd: false,
            } => String::from("Null"),
            Space::Mixed {
                even: true,
                odd: false,
            } => String::from("EvenMultivector"),
            Space::Mixed {
                even: false,
                odd: true,
            } => String::from("OddMultivector"),
            Space::Mixed {
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
