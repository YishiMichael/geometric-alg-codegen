use crate::ast::{
    Ast, Expr, Implementation, ImplementationBody, Item, OperationName, OperationSignature,
    Ownership, Record, Stmt, Stringifier, Structure, TemplateSignature, TypeBinding,
};
use crate::syntax::{Syntax, Writer};
use itertools::Itertools;
use strum::{EnumIter, EnumProperty, IntoEnumIterator};

type Coefficient = i32;

#[derive(Clone, Copy, Eq, PartialEq)]
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

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Generator {
    generator: usize,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Grade {
    grade: usize,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
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

    fn grade(self) -> Grade {
        Grade {
            grade: self.generator_bits.count_ones() as usize,
        }
    }

    fn involute_sign(self) -> Sign {
        Sign::from_count(self.grade().grade)
    }

    fn reverse_sign(self) -> Sign {
        Sign::from_count(self.grade().grade >> 1)
    }

    fn conjugate_sign(self) -> Sign {
        Sign::from_count((self.grade().grade + 1) >> 1)
    }

    fn parity<const N: usize>(blades: [Blade; N]) -> Sign {
        Sign::from_count(
            blades
                .iter()
                .enumerate()
                .flat_map(|(index, blade)| {
                    let gray_inv = (0..)
                        .fold_while(blade.generator_bits, |bits, base| {
                            let shifted = bits >> (1 << base);
                            if shifted == 0 {
                                itertools::FoldWhile::Done(bits)
                            } else {
                                itertools::FoldWhile::Continue(bits ^ shifted)
                            }
                        })
                        .into_inner();
                    blades[index + 1..].iter().map(move |blade| {
                        (gray_inv >> 1 & blade.generator_bits).count_ones() as usize
                    })
                })
                .sum(),
        )
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Space {
    Homogeneous { grade: Grade },
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
                even && blade.grade().grade & 1 == 0 || odd && blade.grade().grade & 1 == 1
            }
        }
    }

    fn contains_even_grade(self) -> bool {
        match self {
            Self::Homogeneous { grade } => grade.grade & 1 == 0,
            Self::Mixed { even, odd: _ } => even,
        }
    }

    fn contains_odd_grade(self) -> bool {
        match self {
            Self::Homogeneous { grade } => grade.grade & 1 == 1,
            Self::Mixed { even: _, odd } => odd,
        }
    }

    fn add(self, other: Self) -> Self {
        if self == other {
            self
        } else {
            Self::Mixed {
                even: self.contains_even_grade() || other.contains_even_grade(),
                odd: self.contains_odd_grade() || other.contains_odd_grade(),
            }
        }
    }

    fn mul(self, other: Self) -> Self {
        Self::Mixed {
            even: self.contains_even_grade() && other.contains_even_grade()
                || self.contains_odd_grade() && other.contains_odd_grade(),
            odd: self.contains_even_grade() && other.contains_odd_grade()
                || self.contains_odd_grade() && other.contains_even_grade(),
        }
    }

    fn zip_homogeneous(
        self,
        other: Self,
        homogeneous_fn: impl FnOnce(Grade, Grade) -> Option<Grade>,
        inhomogeneous_fn: impl FnOnce(Self, Self) -> Self,
    ) -> Self {
        match (self, other) {
            (Self::Homogeneous { grade: grade_0 }, Self::Homogeneous { grade: grade_1 }) => {
                match homogeneous_fn(grade_0, grade_1) {
                    Some(grade) => Self::Homogeneous { grade },
                    None => Self::null(),
                }
            }
            (space_0, space_1) => inhomogeneous_fn(space_0, space_1),
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Class {
    Base,
    Space(Space),
}

impl From<Class> for Space {
    fn from(class: Class) -> Self {
        match class {
            Class::Base => Self::Homogeneous {
                grade: Grade { grade: 0 },
            },
            Class::Space(space) => space,
        }
    }
}

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
    #[strum(props(trait = "Normalized", fn = "normalized"))]
    Normalized,
    #[strum(props(trait = "Normalize", fn = "normalize"))]
    Normalize,
    #[strum(props(trait = "Inverse", fn = "inverse"))]
    Inverse,
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
                key: "a",
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
                key: "a",
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
                key: "a",
                value: Ownership::Owned(TypeBinding::SelfBining),
            }],
            param_items: [Item {
                key: "b",
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
                key: "a",
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
                key: "a",
                value: Ownership::BorrowedMut(TypeBinding::SelfBining),
            }],
            param_items: [Item {
                key: "b",
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

struct MultinomialConfig<const DEGREE: usize> {
    input_blades_remap: fn(&GeometricAlgebra, Blade) -> (Sign, Blade),
    term_filter: fn([Blade; DEGREE]) -> bool,
    term_sign: fn([Blade; DEGREE]) -> Sign,
    output_blade_remap: fn(&GeometricAlgebra, Blade) -> (Sign, Blade),
}

impl<const DEGREE: usize> Default for MultinomialConfig<DEGREE> {
    fn default() -> Self {
        Self {
            input_blades_remap: |_, blade| (Sign::Pos, blade),
            term_filter: |_| true,
            term_sign: |_| Sign::Pos,
            output_blade_remap: |_, blade| (Sign::Pos, blade),
        }
    }
}

impl<const DEGREE: usize> MultinomialConfig<DEGREE> {
    fn body_fn<const VARIABLE: usize>(
        self,
        alg: &GeometricAlgebra,
        prototype: [usize; DEGREE],
    ) -> impl Fn(
        <GeometricAlgebra as Ast>::Type,
        [(
            <GeometricAlgebra as Ast>::Param,
            <GeometricAlgebra as Ast>::Type,
        ); VARIABLE],
    ) -> Expr<GeometricAlgebra> {
        let multinomial = std::iter::repeat_n(alg.blades(), DEGREE)
            .multi_cartesian_product()
            .filter_map(|blades| {
                let (input_signs, input_blades): (Vec<_>, Vec<_>) = blades
                    .iter()
                    .map(|&blade| (self.input_blades_remap)(alg, blade))
                    .unzip();
                let input_blades = input_blades.try_into().ok().unwrap();
                (self.term_filter)(input_blades).then(|| {
                    let (output_blade, square_product) = input_blades.into_iter().fold(
                        (Blade::zero(), 1),
                        |(output_blade, square_product), input_blade| {
                            (
                                output_blade ^ input_blade,
                                square_product
                                    * alg
                                        .blade_generators(output_blade & input_blade)
                                        .map(|generator| alg.generator_squares[&generator])
                                        .product::<Coefficient>(),
                            )
                        },
                    );
                    let (output_sign, blade) = (self.output_blade_remap)(alg, output_blade);
                    let coeff = square_product
                        * Coefficient::from(
                            Blade::parity(input_blades)
                                ^ (self.term_sign)(input_blades)
                                ^ input_signs
                                    .into_iter()
                                    .fold(output_sign, std::ops::BitXor::bitxor)
                                ^ blades
                                    .iter()
                                    .map(|&blade| alg.blade_intrinsic_signs[&blade])
                                    .fold(
                                        alg.blade_intrinsic_signs[&blade],
                                        std::ops::BitXor::bitxor,
                                    ),
                        );
                    (
                        blade,
                        prototype
                            .into_iter()
                            .zip(blades)
                            .sorted_by_key(|(variable, blade)| {
                                (*variable, alg.blade_intrinsic_signs.get_index_of(blade))
                            })
                            .collect_vec()
                            .try_into()
                            .ok()
                            .unwrap(),
                        coeff,
                    )
                })
            })
            .fold(
                std::collections::HashMap::<
                    Blade,
                    std::collections::HashMap<[(usize, Blade); DEGREE], Coefficient>,
                >::new(),
                |mut polynomials, (blade, multi_index, coeff)| {
                    if coeff != 0 {
                        let polynomial = polynomials.entry(blade).or_default();
                        match polynomial.entry(multi_index) {
                            std::collections::hash_map::Entry::Vacant(vacant) => {
                                vacant.insert(coeff);
                            }
                            std::collections::hash_map::Entry::Occupied(mut occupied) => {
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
            )
            .into_iter()
            .map(|(blade, polynomial)| {
                (
                    blade,
                    polynomial
                        .into_iter()
                        .sorted_by_key(|(multi_index, _)| {
                            multi_index.each_ref().map(|(variable, blade)| {
                                (*variable, alg.blade_intrinsic_signs.get_index_of(blade))
                            })
                        })
                        .collect_vec(),
                )
            })
            .collect::<std::collections::HashMap<_, _>>();

        move |class, param_items| {
            alg.construct(class, |blade| {
                multinomial
                    .get(&blade)
                    .and_then(|polynomial| {
                        polynomial
                            .iter()
                            .filter_map(|(multi_index, coeff)| {
                                multi_index
                                    .iter()
                                    .map(|&(variable, blade)| {
                                        let (param, class) = param_items[variable];
                                        alg.access(class, blade, Expr::param(param))
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
                                        expr_acc
                                            + exprs.reduce(std::ops::Mul::mul).unwrap_or(literal)
                                    }
                                    (Some(expr_acc), true, true) => {
                                        expr_acc
                                            - exprs.reduce(std::ops::Mul::mul).unwrap_or(literal)
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
            })
        }
    }
}

struct GeometricAlgebra {
    dim: usize,
    generator_squares: indexmap::IndexMap<Generator, Coefficient>,
    blade_intrinsic_signs: indexmap::IndexMap<Blade, Sign>,
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
    fn grade_add(&self, grade_0: Grade, grade_1: Grade) -> Option<Grade> {
        Some(grade_0.grade + grade_1.grade)
            .filter(|&grade| grade <= self.dim)
            .map(|grade| Grade { grade })
    }

    fn grade_sub(&self, grade_0: Grade, grade_1: Grade) -> Option<Grade> {
        grade_0
            .grade
            .checked_sub(grade_1.grade)
            .map(|grade| Grade { grade })
    }

    fn blades(&self) -> impl Iterator<Item = Blade> + Clone {
        self.blade_intrinsic_signs.keys().cloned()
    }

    fn blade_generators(&self, blade: Blade) -> impl Iterator<Item = Generator> {
        (0..self.dim)
            .filter(move |generator| blade.generator_bits & 1 << generator != 0)
            .map(|generator| Generator { generator })
    }

    fn dual_blade(&self, blade: Blade) -> (Sign, Blade) {
        let complement_blade = Blade {
            generator_bits: ((1 << self.dim) - 1) ^ blade.generator_bits,
        };
        (Blade::parity([blade, complement_blade]), complement_blade)
    }

    fn undual_blade(&self, blade: Blade) -> (Sign, Blade) {
        let complement_blade = Blade {
            generator_bits: ((1 << self.dim) - 1) ^ blade.generator_bits,
        };
        (Blade::parity([complement_blade, blade]), complement_blade)
    }

    fn spaces(&self) -> impl Iterator<Item = Space> {
        (0..=self.dim)
            .map(|grade| Space::Homogeneous {
                grade: Grade { grade },
            })
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
            Space::Homogeneous {
                grade: Grade { grade },
            } => Space::Homogeneous {
                grade: Grade {
                    grade: self.dim - grade,
                },
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
                MultinomialConfig::default().body_fn(self, []),
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
                    [Class::Base, Class::Base] => None,
                    [class_0, class_1] => {
                        Some(Class::Space(Space::from(class_0).add(Space::from(class_1))))
                    }
                },
                |class, [(param_0, class_0), (param_1, class_1)]| {
                    self.construct(class, |blade| {
                        match (
                            self.access(class_0, blade, Expr::param(param_0)),
                            self.access(class_1, blade, Expr::param(param_1)),
                        ) {
                            (Some(expr_0), Some(expr_1)) => expr_0 + expr_1,
                            (Some(expr_0), None) => expr_0,
                            (None, Some(expr_1)) => expr_1,
                            (None, None) => Expr::literal(0),
                        }
                    })
                },
            ),

            Operation::Sub => SUB.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Base, Class::Base] => None,
                    [class_0, class_1] => {
                        Some(Class::Space(Space::from(class_0).add(Space::from(class_1))))
                    }
                },
                |class, [(param_0, class_0), (param_1, class_1)]| {
                    self.construct(class, |blade| {
                        match (
                            self.access(class_0, blade, Expr::param(param_0)),
                            self.access(class_1, blade, Expr::param(param_1)),
                        ) {
                            (Some(expr_0), Some(expr_1)) => expr_0 - expr_1,
                            (Some(expr_0), None) => expr_0,
                            (None, Some(expr_1)) => -expr_1,
                            (None, None) => Expr::literal(0),
                        }
                    })
                },
            ),

            Operation::Mul => MUL.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(space_0.mul(space_1)))
                    }
                    [Class::Space(space_0), Class::Base] => Some(Class::Space(space_0)),
                    [Class::Base, Class::Space(space_1)] => Some(Class::Space(space_1)),
                    _ => None,
                },
                |class, [(param_0, class_0), (param_1, class_1)]| match [class_0, class_1] {
                    [Class::Space(_), Class::Space(_)] => GEOMETRIC_PRODUCT.call([
                        (Expr::param(param_0), class_0),
                        (Expr::param(param_1), class_1),
                    ]),
                    [Class::Space(_), Class::Base] => self.construct(class, |blade| {
                        self.access(class_0, blade, Expr::param(param_0)).unwrap()
                            * Expr::param(param_1)
                    }),
                    [Class::Base, Class::Space(_)] => self.construct(class, |blade| {
                        Expr::param(param_0)
                            * self.access(class_1, blade, Expr::param(param_1)).unwrap()
                    }),
                    _ => unreachable!(),
                },
            ),

            Operation::Div => DIV.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Base] => Some(Class::Space(space_0)),
                    [Class::Base, Class::Space(space_1)] if space_1 != Space::null() => {
                        Some(Class::Space(space_1.add(Space::null())))
                    }
                    _ => None,
                },
                |class, [(param_0, class_0), (param_1, class_1)]| match [class_0, class_1] {
                    [Class::Space(_), Class::Base] => self.construct(class, |blade| {
                        self.access(class_0, blade, Expr::param(param_0)).unwrap()
                            / Expr::param(param_1)
                    }),
                    [Class::Base, Class::Space(_)] => MUL.call([
                        (Expr::param(param_0), Class::Base),
                        (INVERSE.call([(Expr::param(param_1), class_1)]), class),
                    ]),
                    _ => unreachable!(),
                },
            ),

            Operation::AddAssign => ADD_ASSIGN.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Base, Class::Base] => false,
                    [class_0, class_1] => {
                        Space::from(class_0).add(Space::from(class_1)) == Space::from(class_0)
                    }
                },
                |[(param_0, class_0), (param_1, class_1)]| {
                    self.space_blades(Space::from(class_1))
                        .map(|blade| {
                            self.access_deref(class_0, blade, Expr::param(param_0))
                                .unwrap()
                                .add_assign(
                                    self.access(class_1, blade, Expr::param(param_1)).unwrap(),
                                )
                        })
                        .collect_vec()
                },
            ),

            Operation::SubAssign => SUB_ASSIGN.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Base, Class::Base] => false,
                    [class_0, class_1] => {
                        Space::from(class_0).add(Space::from(class_1)) == Space::from(class_0)
                    }
                },
                |[(param_0, class_0), (param_1, class_1)]| {
                    self.space_blades(Space::from(class_1))
                        .map(|blade| {
                            self.access_deref(class_0, blade, Expr::param(param_0))
                                .unwrap()
                                .sub_assign(
                                    self.access(class_1, blade, Expr::param(param_1)).unwrap(),
                                )
                        })
                        .collect_vec()
                },
            ),

            Operation::MulAssign => MUL_ASSIGN.impl_for(
                self,
                |[class_0, class_1]| matches!([class_0, class_1], [Class::Space(_), Class::Base]),
                |[(param_0, class_0), (param_1, class_1)]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Base] => self
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
                |[(param_0, class_0), (param_1, class_1)]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Base] => self
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
                MultinomialConfig {
                    term_sign: |[blade_0]| blade_0.involute_sign(),
                    ..Default::default()
                }
                .body_fn(self, [0]),
            ),

            Operation::Reverse => REVERSE.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(space_0)] => Some(Class::Space(space_0)),
                    _ => None,
                },
                MultinomialConfig {
                    term_sign: |[blade_0]| blade_0.reverse_sign(),
                    ..Default::default()
                }
                .body_fn(self, [0]),
            ),

            Operation::Conjugate => CONJUGATE.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(space_0)] => Some(Class::Space(space_0)),
                    _ => None,
                },
                MultinomialConfig {
                    term_sign: |[blade_0]| blade_0.conjugate_sign(),
                    ..Default::default()
                }
                .body_fn(self, [0]),
            ),

            Operation::Dual => DUAL.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(space_0)] => Some(Class::Space(self.dual_space(space_0))),
                    _ => None,
                },
                MultinomialConfig {
                    output_blade_remap: Self::dual_blade,
                    ..Default::default()
                }
                .body_fn(self, [0]),
            ),

            Operation::Undual => UNDUAL.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(space_0)] => Some(Class::Space(self.dual_space(space_0))),
                    _ => None,
                },
                MultinomialConfig {
                    output_blade_remap: Self::undual_blade,
                    ..Default::default()
                }
                .body_fn(self, [0]),
            ),

            Operation::NormSquared => NORM_SQUARED.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(_)] => Some(Class::Base),
                    _ => None,
                },
                MultinomialConfig {
                    term_filter: |[blade_0, blade_1]| blade_0 == blade_1,
                    term_sign: |[_, blade_1]| blade_1.reverse_sign(),
                    ..Default::default()
                }
                .body_fn(self, [0, 0]),
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
                        (
                            NORM.call([(Expr::param(param_0).deref(), class_0)]),
                            Class::Base,
                        ),
                    ])])
                },
            ),

            Operation::Inverse => INVERSE.impl_for(
                self,
                |[class_0]| match [class_0] {
                    [Class::Space(space_0)] if space_0 != Space::null() => {
                        Some(Class::Space(space_0.add(Space::null())))
                    }
                    _ => None,
                },
                |class, [(param_0, class_0)]| {
                    let adjoint_param = "adjoint";
                    let coadjoint_param = "coadjoint";
                    let k = 1 << ((self.dim + 1) >> 1);
                    let mut param_map = std::collections::HashMap::new();
                    ImplementationBody {
                        stmts: std::iter::successors(Some(Class::Base), |&prev_class| {
                            Some(match prev_class {
                                Class::Base => Class::Space(Space::from(class).add(Space::null())),
                                Class::Space(space) => Class::Space(Space::from(class).mul(space)),
                            })
                        })
                        .tuple_windows()
                        .zip(1..k)
                        .flat_map(|((rhs_class, lhs_class), i)| {
                            let expr = match rhs_class {
                                Class::Base => {
                                    if lhs_class == class_0 {
                                        Expr::param(param_0)
                                    } else {
                                        FROM.call(lhs_class, [(Expr::param(param_0), class_0)])
                                    }
                                }
                                rhs_class => GEOMETRIC_PRODUCT.call([
                                    (Expr::param(param_0), class_0),
                                    (Expr::param(param_map[&rhs_class]), rhs_class),
                                ]),
                            };
                            let (lhs_param, stmt) = match param_map.entry(lhs_class) {
                                std::collections::hash_map::Entry::Vacant(entry) => {
                                    let lhs_param = *entry.insert(if lhs_class == class {
                                        adjoint_param
                                    } else {
                                        coadjoint_param
                                    });
                                    (lhs_param, Expr::bind_mut(lhs_param, lhs_class, expr))
                                }
                                std::collections::hash_map::Entry::Occupied(entry) => {
                                    let lhs_param = *entry.get();
                                    (lhs_param, Expr::param(lhs_param).assign(expr))
                                }
                            };
                            std::iter::once(stmt).chain(
                                self.access_deref(lhs_class, Blade::zero(), Expr::param(lhs_param))
                                    .map(|expr| {
                                        expr.mul_assign(
                                            Expr::literal(1) - Expr::literal(k) / Expr::literal(i),
                                        )
                                    }),
                            )
                        })
                        .chain(std::iter::once(DIV_ASSIGN.call([
                            (Expr::param(adjoint_param).borrow_mut(), class),
                            (
                                SCALAR_PRODUCT.call([
                                    (Expr::param(param_0), class_0),
                                    (Expr::param(adjoint_param), class),
                                ]),
                                Class::Base,
                            ),
                        ])))
                        .collect(),
                        expr: Some(Expr::param(adjoint_param)),
                    }
                },
            ),

            Operation::GeometricProduct => GEOMETRIC_PRODUCT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(space_0.mul(space_1)))
                    }
                    _ => None,
                },
                MultinomialConfig::default().body_fn(self, [0, 1]),
            ),

            Operation::ScalarProduct => SCALAR_PRODUCT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(_), Class::Space(_)] => Some(Class::Base),
                    _ => None,
                },
                MultinomialConfig {
                    term_filter: |[blade_0, blade_1]| blade_0 == blade_1,
                    ..Default::default()
                }
                .body_fn(self, [0, 1]),
            ),

            Operation::LeftInnerProduct => LEFT_INNER_PRODUCT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(space_0.zip_homogeneous(
                            space_1,
                            |grade_0, grade_1| self.grade_sub(grade_1, grade_0),
                            Space::mul,
                        )))
                    }
                    _ => None,
                },
                MultinomialConfig {
                    term_filter: |[blade_0, blade_1]| blade_0 & blade_1 == blade_0,
                    ..Default::default()
                }
                .body_fn(self, [0, 1]),
            ),

            Operation::RightInnerProduct => RIGHT_INNER_PRODUCT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(space_0.zip_homogeneous(
                            space_1,
                            |grade_0, grade_1| self.grade_sub(grade_0, grade_1),
                            Space::mul,
                        )))
                    }
                    _ => None,
                },
                MultinomialConfig {
                    term_filter: |[blade_0, blade_1]| blade_0 & blade_1 == blade_1,
                    ..Default::default()
                }
                .body_fn(self, [0, 1]),
            ),

            Operation::InnerProduct => INNER_PRODUCT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(space_0.zip_homogeneous(
                            space_1,
                            |grade_0, grade_1| {
                                self.grade_sub(grade_0, grade_1)
                                    .or(self.grade_sub(grade_1, grade_0))
                            },
                            Space::mul,
                        )))
                    }
                    _ => None,
                },
                MultinomialConfig {
                    term_filter: |[blade_0, blade_1]| {
                        blade_0 & blade_1 == blade_0 || blade_0 & blade_1 == blade_1
                    },
                    ..Default::default()
                }
                .body_fn(self, [0, 1]),
            ),

            Operation::OuterProduct => OUTER_PRODUCT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(space_0.zip_homogeneous(
                            space_1,
                            |grade_0, grade_1| self.grade_add(grade_0, grade_1),
                            Space::mul,
                        )))
                    }
                    _ => None,
                },
                MultinomialConfig {
                    term_filter: |[blade_0, blade_1]| blade_0 & blade_1 == Blade::zero(),
                    ..Default::default()
                }
                .body_fn(self, [0, 1]),
            ),

            Operation::RegressiveProduct => REGRESSIVE_PRODUCT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => Some(Class::Space(
                        self.dual_space(self.dual_space(space_0).zip_homogeneous(
                            self.dual_space(space_1),
                            |grade_0, grade_1| self.grade_add(grade_0, grade_1),
                            Space::mul,
                        )),
                    )),
                    _ => None,
                },
                MultinomialConfig {
                    input_blades_remap: Self::dual_blade,
                    term_filter: |[blade_0, blade_1]| blade_0 & blade_1 == Blade::zero(),
                    output_blade_remap: Self::undual_blade,
                    ..Default::default()
                }
                .body_fn(self, [0, 1]),
            ),

            Operation::Commutator => COMMUTATOR.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(space_0.mul(space_1)))
                    }
                    _ => None,
                },
                MultinomialConfig {
                    term_filter: |[blade_0, blade_1]| {
                        Blade::parity([blade_0, blade_1]) != Blade::parity([blade_1, blade_0])
                    },
                    ..Default::default()
                }
                .body_fn(self, [0, 1]),
            ),

            Operation::Anticommutator => ANTICOMMUTATOR.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(space_0.mul(space_1)))
                    }
                    _ => None,
                },
                MultinomialConfig {
                    term_filter: |[blade_0, blade_1]| {
                        Blade::parity([blade_0, blade_1]) == Blade::parity([blade_1, blade_0])
                    },
                    ..Default::default()
                }
                .body_fn(self, [0, 1]),
            ),

            Operation::Transform => TRANSFORM.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(_)] => Some(Class::Space(space_0)),
                    _ => None,
                },
                MultinomialConfig {
                    term_filter: |[blade_0, blade_1, blade_2]| {
                        (blade_0 ^ blade_1 ^ blade_2).grade() == blade_1.grade()
                    },
                    term_sign: |[blade_0, blade_1, blade_2]| {
                        blade_0.involute_sign() ^ blade_1.involute_sign() ^ blade_2.reverse_sign()
                    },
                    ..Default::default()
                }
                .body_fn(self, [1, 0, 1]),
            ),

            Operation::Project => PROJECT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(space_0.zip_homogeneous(
                            space_1,
                            |grade_0, grade_1| {
                                self.grade_sub(grade_1, grade_0)
                                    .is_some()
                                    .then_some(grade_0)
                            },
                            |space_0, _| space_0,
                        )))
                    }
                    _ => None,
                },
                MultinomialConfig {
                    term_filter: |[blade_0, blade_1, blade_2]| {
                        blade_0 & blade_1 == blade_0
                            && (blade_0 ^ blade_1) & blade_2 == blade_0 ^ blade_1
                            && (blade_0 ^ blade_1 ^ blade_2).grade() == blade_0.grade()
                    },
                    term_sign: |[_, _, blade_2]| blade_2.reverse_sign(),
                    ..Default::default()
                }
                .body_fn(self, [0, 1, 1]),
            ),

            Operation::Reject => REJECT.impl_for(
                self,
                |[class_0, class_1]| match [class_0, class_1] {
                    [Class::Space(space_0), Class::Space(space_1)] => {
                        Some(Class::Space(space_0.zip_homogeneous(
                            space_1,
                            |grade_0, grade_1| {
                                self.grade_sub(grade_0, grade_1)
                                    .is_some()
                                    .then_some(grade_0)
                            },
                            |space_0, _| space_0,
                        )))
                    }
                    _ => None,
                },
                MultinomialConfig {
                    term_filter: |[blade_0, blade_1, blade_2]| {
                        blade_0 & blade_1 == Blade::zero()
                            && (blade_0 ^ blade_1) & blade_2 == blade_2
                            && (blade_0 ^ blade_1 ^ blade_2).grade() == blade_0.grade()
                    },
                    term_sign: |[_, _, blade_2]| blade_2.reverse_sign(),
                    ..Default::default()
                }
                .body_fn(self, [0, 1, 1]),
            ),
        }
    }
}

struct GeometricAlgebraStringifier<'s, S> {
    blade_names: &'s indexmap::IndexMap<Blade, S>,
    precision: S,
}

impl<S: AsRef<str>> Stringifier<GeometricAlgebra> for GeometricAlgebraStringifier<'_, S> {
    fn stringify_type(&self, r#type: &<GeometricAlgebra as Ast>::Type) -> &str {
        match r#type {
            Class::Base => self.precision.as_ref(),
            Class::Space(space) => self.stringify_template(space),
        }
    }

    fn stringify_template(&self, template: &<GeometricAlgebra as Ast>::Template) -> &str {
        const GRADED_SPACES: [&str; 17] = [
            "Scalar",
            "Vector",
            "Bivector",
            "Trivector",
            "QuadVector",
            "FiveVector",
            "SixVector",
            "SevenVector",
            "EightVector",
            "NineVector",
            "TenVector",
            "ElevenVector",
            "TwelveVector",
            "ThirteenVector",
            "FourteenVector",
            "FifteenVector",
            "SixteenVector",
        ];
        match template {
            Space::Homogeneous {
                grade: Grade { grade },
            } => GRADED_SPACES[*grade],
            Space::Mixed {
                even: false,
                odd: false,
            } => "Null",
            Space::Mixed {
                even: true,
                odd: false,
            } => "EvenMultivector",
            Space::Mixed {
                even: false,
                odd: true,
            } => "OddMultivector",
            Space::Mixed {
                even: true,
                odd: true,
            } => "Multivector",
        }
    }

    fn stringify_field(&self, field: &<GeometricAlgebra as Ast>::Field) -> &str {
        self.blade_names[field].as_ref()
    }
}

pub struct GeometricAlgebraRecord<S> {
    record: Record<GeometricAlgebra>,
    blade_names: indexmap::IndexMap<Blade, S>,
}

impl<S> GeometricAlgebraRecord<S> {
    pub fn new<G>(
        generators: indexmap::IndexMap<G, Coefficient>,
        blades: indexmap::IndexMap<S, Vec<G>>,
    ) -> Self
    where
        G: Eq + std::hash::Hash,
        S: std::hash::Hash,
    {
        let dim = generators.len();
        assert_eq!(1 << dim, blades.len());
        let alg = GeometricAlgebra {
            dim,
            generator_squares: generators
                .values()
                .enumerate()
                .map(|(generator, generator_square)| (Generator { generator }, *generator_square))
                .collect(),
            blade_intrinsic_signs: blades
                .values()
                .map(|generator_names| {
                    let indices = generator_names
                        .iter()
                        .map(|generator_name| generators.get_index_of(generator_name).unwrap())
                        .collect_vec();
                    let blade = Blade {
                        generator_bits: indices
                            .iter()
                            .map(|index| 1 << index)
                            .fold(0, std::ops::BitXor::bitxor),
                    };
                    let sign = Sign::from_count(
                        indices
                            .iter()
                            .tuple_combinations()
                            .filter(|(index_0, index_1)| index_0 > index_1)
                            .count(),
                    );
                    (blade, sign)
                })
                .collect(),
        };
        assert_eq!(alg.blade_intrinsic_signs.len(), blades.len());
        assert!(alg
            .blade_intrinsic_signs
            .keys()
            .map(|blade| blade.generator_bits)
            .sorted()
            .enumerate()
            .all(|(index, generator_bits)| index == generator_bits));
        Self {
            record: alg.record(),
            blade_names: alg
                .blade_intrinsic_signs
                .keys()
                .cloned()
                .zip(blades.into_keys())
                .collect(),
        }
    }

    pub fn write<Lang, Buffer>(
        &self,
        lang: Lang,
        buffer: &mut Buffer,
        precision: S,
    ) -> std::io::Result<()>
    where
        Buffer: std::io::Write,
        Lang: Syntax,
        S: AsRef<str>,
    {
        let stringifier = GeometricAlgebraStringifier {
            blade_names: &self.blade_names,
            precision,
        };
        let mut writer = Writer::new(buffer);
        lang.emit_record(&mut writer, &stringifier, &self.record)
    }
}
