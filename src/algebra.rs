use crate::ast::{Expr, Items, Ownership, Pretype, Resolve, Stmt, StructureSignature};
use itertools::Itertools;
use symbol::Symbol;

type Coefficient = i32;

#[derive(Clone, Copy)]
enum Sign {
    Pos,
    Neg,
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

impl From<usize> for Sign {
    fn from(value: usize) -> Self {
        if 1 & value == 0 {
            Self::Pos
        } else {
            Self::Neg
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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
        Sign::from((gray_inv >> 1 & other.generator_bits).count_ones() as usize)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Type {
    Base,
    Space(Space),
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
}

struct Multinomial(
    std::collections::BTreeMap<Blade, std::collections::BTreeMap<Vec<(usize, Blade)>, Coefficient>>,
);

impl FromIterator<(Blade, Vec<(usize, Blade)>, Coefficient)> for Multinomial {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Blade, Vec<(usize, Blade)>, Coefficient)>,
    {
        iter.into_iter().fold(
            Multinomial(std::collections::BTreeMap::new()),
            |mut multinomial, (blade, multi_index, coeff)| {
                if coeff != 0 {
                    let polynomial = multinomial.0.entry(blade).or_default();
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
                        multinomial.0.remove(&blade);
                    }
                }
                multinomial
            },
        )
    }
}

impl Multinomial {
    fn dual(self, dim: usize) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|(blade, polynomial)| (blade.dual(dim), polynomial))
                .collect(),
        )
    }

    fn blade_expr<Resolver, const PARAMS: usize>(
        &self,
        structure_map: &std::collections::BTreeMap<Space, StructureSignature<Resolver, Blade>>,
        blade: Blade,
        spaces: [Space; PARAMS],
        symbols: [Symbol; PARAMS],
    ) -> Expr
    where
        Resolver: Resolve<Component = Blade>,
    {
        self.0
            .get(&blade)
            .and_then(|polynomial| {
                polynomial
                    .iter()
                    .filter_map(|(multi_index, coeff)| {
                        multi_index
                            .iter()
                            .map(|&(index, blade)| {
                                spaces[index].contains_blade(blade).then(|| {
                                    structure_map[&spaces[index]]
                                        .access(symbols[index].into(), &blade)
                                })
                            })
                            .collect::<Option<Vec<_>>>()
                            .map(|exprs| (exprs.into_iter(), coeff))
                    })
                    .fold(None, |expr_acc, (exprs, &coeff)| {
                        let coeff_abs = coeff.unsigned_abs();
                        let literal = coeff_abs.into();
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
            .unwrap_or_else(|| 0.into())
    }

    fn expr<Resolver, const PARAMS: usize>(
        &self,
        structure_map: &std::collections::BTreeMap<Space, StructureSignature<Resolver, Blade>>,
        space: Space,
        spaces: [Space; PARAMS],
        symbols: [Symbol; PARAMS],
    ) -> Expr
    where
        Resolver: Resolve<Component = Blade>,
    {
        structure_map[&space]
            .construct(|&blade| self.blade_expr(structure_map, blade, spaces, symbols))
    }
}

struct GeometricAlgebraIdents {
    blade_idents: Vec<Symbol>,
    precision: Symbol,
    graded_spaces: Vec<Symbol>,
    null_space: Symbol,
    even_space: Symbol,
    odd_space: Symbol,
    full_space: Symbol,
}

impl Resolve for GeometricAlgebraIdents {
    type Component = Blade;
    type Type = Type;

    fn resolve_component(&self, blade: &Self::Component) -> Symbol {
        self.blade_idents[blade.generator_bits]
    }

    fn resolve_type(&self, r#type: &Self::Type) -> Symbol {
        match r#type {
            Type::Base => self.precision,
            Type::Space(Space::GradedVector { grade }) => self.graded_spaces[*grade],
            Type::Space(Space::SaturatedVector {
                even: false,
                odd: false,
            }) => self.null_space,
            Type::Space(Space::SaturatedVector {
                even: true,
                odd: false,
            }) => self.even_space,
            Type::Space(Space::SaturatedVector {
                even: false,
                odd: true,
            }) => self.odd_space,
            Type::Space(Space::SaturatedVector {
                even: true,
                odd: true,
            }) => self.full_space,
        }
    }
}

struct GeometricAlgebra {
    dim: usize,
    generator_squares: Vec<Coefficient>,
    blade_intrinsic_signs: Vec<Sign>,
}

impl GeometricAlgebra {
    fn multinomial<const DEGREE: usize>(
        &self,
        prototype: [usize; DEGREE],
        term_sign: fn([Blade; DEGREE], usize) -> Option<Sign>,
    ) -> Multinomial {
        std::iter::repeat_n(Blade::iter(self.dim), DEGREE)
            .multi_cartesian_product()
            .map(|blades| -> [Blade; DEGREE] { blades.try_into().unwrap() })
            .filter_map(|blades| {
                term_sign(blades, self.dim).map(|sign| {
                    let multi_index = prototype.into_iter().zip_eq(blades).sorted().collect_vec();
                    let blade = blades
                        .into_iter()
                        .fold(Blade::zero(), std::ops::BitXor::bitxor);
                    let coeff = Coefficient::from(
                        sign ^ blades
                            .into_iter()
                            .map(|blade| self.blade_intrinsic_signs[blade.generator_bits])
                            .fold(
                                self.blade_intrinsic_signs[blade.generator_bits],
                                std::ops::BitXor::bitxor,
                            ),
                    ) * blades
                        .into_iter()
                        .fold(Blade::zero().dual(self.dim), std::ops::BitAnd::bitand)
                        .iter_generators(self.dim)
                        .map(|generator| self.generator_squares[generator])
                        .product::<Coefficient>();
                    (blade, multi_index, coeff)
                })
            })
            .collect()
    }

    fn items(&self, idents: &GeometricAlgebraIdents) -> Items {
        #![allow(non_snake_case, clippy::type_complexity)]
        let mut items = Items::default();

        let structure_map: std::collections::BTreeMap<_, _> = Space::iter(self.dim)
            .map(|space| {
                (
                    space,
                    idents.register_structure(
                        &mut items,
                        idents.resolve_type(&Type::Space(space)).as_str(),
                        space.iter_blades(self.dim),
                        |_| Type::Base,
                    ),
                )
            })
            .collect();

        let nullary_constructor_operation = |ident, op_ident| {
            idents.define_operation(ident, op_ident, [], [], [], [], [Pretype::SelfBining])
        };
        let unary_constructor_operation = |ident, op_ident| {
            idents.define_operation(
                ident,
                op_ident,
                ["T"],
                [],
                [],
                [("value", Ownership::Owned(Pretype::GenericBinding(0)))],
                [Pretype::SelfBining],
            )
        };
        let unary_operation = |ident, op_ident| {
            idents.define_operation(
                ident,
                op_ident,
                [],
                ["Output"],
                [("self", Ownership::Owned(Pretype::SelfBining))],
                [],
                [Pretype::AssociateBinding(0)],
            )
        };
        let binary_operation = |ident, op_ident| {
            idents.define_operation(
                ident,
                op_ident,
                ["T"],
                ["Output"],
                [("self", Ownership::Owned(Pretype::SelfBining))],
                [("other", Ownership::Owned(Pretype::GenericBinding(0)))],
                [Pretype::AssociateBinding(0)],
            )
        };
        let unary_inplace_operation = |ident, op_ident| {
            idents.define_operation(
                ident,
                op_ident,
                [],
                [],
                [("self", Ownership::BorrowedMut(Pretype::SelfBining))],
                [],
                [],
            )
        };
        let binary_inplace_operation = |ident, op_ident| {
            idents.define_operation(
                ident,
                op_ident,
                ["T"],
                [],
                [("self", Ownership::BorrowedMut(Pretype::SelfBining))],
                [("other", Ownership::Owned(Pretype::GenericBinding(0)))],
                [],
            )
        };

        let ABS = unary_operation("Abs", "abs");
        let SQRT = unary_operation("Sqrt", "sqrt");
        let FROM = unary_constructor_operation("From", "from");
        let NEG = unary_operation("Neg", "neg");
        let ADD = binary_operation("Add", "add");
        let SUB = binary_operation("Sub", "sub");
        let MUL = binary_operation("Mul", "mul");
        let DIV = binary_operation("Div", "div");
        let ADD_ASSIGN = binary_inplace_operation("AddAssign", "add_assign");
        let SUB_ASSIGN = binary_inplace_operation("SubAssign", "sub_assign");
        let MUL_ASSIGN = binary_inplace_operation("MulAssign", "mul_assign");
        let DIV_ASSIGN = binary_inplace_operation("DivAssign", "div_assign");

        let ZERO = nullary_constructor_operation("Zero", "zero");
        let ONE = nullary_constructor_operation("One", "one");

        let GRADE_INVOLUTION = unary_operation("GradeInvolution", "grade_involution");
        let REVERSE = unary_operation("Reverse", "reverse");
        let CONJUGATE = unary_operation("Conjugate", "conjugate");
        let DUAL = unary_operation("Dual", "dual");
        let UNDUAL = unary_operation("Undual", "undual");
        let NORM_SQUARED = unary_operation("NormSquared", "norm_squared");
        let NORM = unary_operation("Norm", "norm");
        let INVERSE = unary_operation("Inverse", "inverse");
        let NORMALIZED = unary_operation("Normalized", "normalized");
        let NORMALIZE = unary_inplace_operation("Normalize", "normalize");

        let GEOMETRIC_PRODUCT = binary_operation("GeometricProduct", "geometric_product");
        let SCALAR_PRODUCT = binary_operation("ScalarProduct", "scalar_product");
        let LEFT_INNER_PRODUCT = binary_operation("LeftInnerProduct", "left_inner_product");
        let RIGHT_INNER_PRODUCT = binary_operation("RightInnerProduct", "right_inner_product");
        let INNER_PRODUCT = binary_operation("InnerProduct", "inner_product");
        let OUTER_PRODUCT = binary_operation("OuterProduct", "outer_product");
        let REGRESSIVE_PRODUCT = binary_operation("RegressiveProduct", "regressive_product");
        let COMMUTATOR = binary_operation("Commutator", "commutator");
        let ANTICOMMUTATOR = binary_operation("Anticommutator", "anticommutator");
        let TRANSFORM = binary_operation("Transform", "transform");
        let PROJECT = binary_operation("Project", "project");
        let REJECT = binary_operation("Reject", "reject");

        for (space_0, space_1) in Space::iter(self.dim)
            .cartesian_product(Space::iter(self.dim))
            .filter(|(space_0, space_1)| space_0 != space_1)
        {
            FROM.register_implementation(
                &mut items,
                Type::Space(space_1),
                [Type::Space(space_0)],
                [],
                |[], [symbol_0]| {
                    structure_map[&space_1].construct(|&blade| {
                        if space_0.contains_blade(blade) {
                            structure_map[&space_0].access(symbol_0.into(), &blade)
                        } else {
                            0.into()
                        }
                    })
                },
            )
        }
        for space_1 in Space::iter(self.dim) {
            FROM.register_implementation(
                &mut items,
                Type::Space(space_1),
                [Type::Base],
                [],
                |[], [symbol_0]| {
                    structure_map[&space_1].construct(|&blade| {
                        if blade == Blade::zero() {
                            symbol_0.into()
                        } else {
                            0.into()
                        }
                    })
                },
            )
        }
        for space_0 in Space::iter(self.dim) {
            FROM.register_implementation(
                &mut items,
                Type::Base,
                [Type::Space(space_0)],
                [],
                |[], [symbol_0]| {
                    if space_0.contains_blade(Blade::zero()) {
                        structure_map[&space_0].access(symbol_0.into(), &Blade::zero())
                    } else {
                        0.into()
                    }
                },
            )
        }

        for space in Space::iter(self.dim) {
            NEG.register_implementation(
                &mut items,
                Type::Space(space),
                [],
                [Type::Space(space)],
                |[symbol_0], []| {
                    structure_map[&space]
                        .construct(|&blade| -structure_map[&space].access(symbol_0.into(), &blade))
                },
            );
        }
        let arithmetic_operations: [(_, fn(Expr, Expr) -> Expr); _] = [
            (&ADD, std::ops::Add::add),
            (&SUB, std::ops::Sub::sub),
            (&MUL, std::ops::Mul::mul),
            (&DIV, std::ops::Div::div),
        ];
        for (operation, expr_op) in arithmetic_operations {
            for space in Space::iter(self.dim) {
                operation.register_implementation(
                    &mut items,
                    Type::Space(space),
                    [Type::Space(space)],
                    [Type::Space(space)],
                    |[symbol_0], [symbol_1]| {
                        structure_map[&space].construct(|&blade| {
                            expr_op(
                                structure_map[&space].access(symbol_0.into(), &blade),
                                structure_map[&space].access(symbol_1.into(), &blade),
                            )
                        })
                    },
                );
                operation.register_implementation(
                    &mut items,
                    Type::Space(space),
                    [Type::Base],
                    [Type::Space(space)],
                    |[symbol_0], [symbol_1]| {
                        structure_map[&space].construct(|&blade| {
                            expr_op(
                                structure_map[&space].access(symbol_0.into(), &blade),
                                symbol_1.into(),
                            )
                        })
                    },
                );
                operation.register_implementation(
                    &mut items,
                    Type::Base,
                    [Type::Space(space)],
                    [Type::Space(space)],
                    |[symbol_0], [symbol_1]| {
                        structure_map[&space].construct(|&blade| {
                            expr_op(
                                symbol_0.into(),
                                structure_map[&space].access(symbol_1.into(), &blade),
                            )
                        })
                    },
                );
            }
        }

        let arithmetic_inplace_operations: [(_, fn(Expr, Expr) -> Stmt); _] = [
            (&ADD_ASSIGN, Expr::add_assign),
            (&SUB_ASSIGN, Expr::sub_assign),
            (&MUL_ASSIGN, Expr::mul_assign),
            (&DIV_ASSIGN, Expr::div_assign),
        ];
        for (operation, stmt_op) in arithmetic_inplace_operations {
            for space in Space::iter(self.dim) {
                operation.register_implementation(
                    &mut items,
                    Type::Space(space),
                    [Type::Space(space)],
                    [],
                    |[symbol_0], [symbol_1]| {
                        space
                            .iter_blades(self.dim)
                            .map(|blade| {
                                stmt_op(
                                    structure_map[&space].access(symbol_0.into(), &blade),
                                    structure_map[&space].access(symbol_1.into(), &blade),
                                )
                            })
                            .collect_vec()
                    },
                );
                operation.register_implementation(
                    &mut items,
                    Type::Space(space),
                    [Type::Base],
                    [],
                    |[symbol_0], [symbol_1]| {
                        space
                            .iter_blades(self.dim)
                            .map(|blade| {
                                stmt_op(
                                    structure_map[&space].access(symbol_0.into(), &blade),
                                    symbol_1.into(),
                                )
                            })
                            .collect_vec()
                    },
                );
            }
        }

        for space_0 in Space::iter(self.dim) {
            ZERO.register_implementation(&mut items, Type::Space(space_0), [], [], |[], []| {
                structure_map[&space_0].construct(|_| 0.into())
            });
        }
        for space_0 in Space::iter(self.dim).filter(|space| space.contains_blade(Blade::zero())) {
            ONE.register_implementation(&mut items, Type::Space(space_0), [], [], |[], []| {
                structure_map[&space_0].construct(|&blade| {
                    if blade == Blade::zero() {
                        match self.blade_intrinsic_signs[blade.generator_bits] {
                            Sign::Pos => 1.into(),
                            Sign::Neg => (-1).into(),
                        }
                    } else {
                        0.into()
                    }
                })
            });
        }

        let unary_multinomial_operations: [(_, fn([Space; 1], usize) -> Type, _); _] = [
            (
                &GRADE_INVOLUTION,
                |[space_0], _| Type::Space(space_0),
                self.multinomial([0], |[blade_0], _| Some(Sign::from(blade_0.grade()))),
            ),
            (
                &REVERSE,
                |[space_0], _| Type::Space(space_0),
                self.multinomial([0], |[blade_0], _| Some(Sign::from(blade_0.grade() >> 1))),
            ),
            (
                &CONJUGATE,
                |[space_0], _| Type::Space(space_0),
                self.multinomial([0], |[blade_0], _| {
                    Some(Sign::from((blade_0.grade() + 1) >> 1))
                }),
            ),
            (
                &DUAL,
                |[space_0], dim| Type::Space(space_0.dual(dim)),
                self.multinomial([0], |[blade_0], dim| {
                    Some(blade_0.dual(dim).parity(blade_0))
                })
                .dual(self.dim),
            ),
            (
                &UNDUAL,
                |[space_0], dim| Type::Space(space_0.dual(dim)),
                self.multinomial([0], |[blade_0], dim| {
                    Some(blade_0.parity(blade_0.dual(dim)))
                })
                .dual(self.dim),
            ),
            (
                &NORM_SQUARED,
                |_, _| Type::Base,
                self.multinomial([0, 0], |_, _| Some(Sign::Pos)),
            ),
        ];
        for (operation, space_fn, multinomial) in unary_multinomial_operations {
            for space_0 in Space::iter(self.dim) {
                let r#type = space_fn([space_0], self.dim);
                operation.register_implementation(
                    &mut items,
                    Type::Space(space_0),
                    [],
                    [r#type],
                    |[symbol_0], []| match r#type {
                        Type::Base => multinomial.blade_expr(
                            &structure_map,
                            Blade::zero(),
                            [space_0],
                            [symbol_0],
                        ),
                        Type::Space(space) => {
                            multinomial.expr(&structure_map, space, [space_0], [symbol_0])
                        }
                    },
                );
            }
        }

        for space_0 in Space::iter(self.dim) {
            NORM.register_implementation(
                &mut items,
                Type::Space(space_0),
                [],
                [Type::Base],
                |[symbol_0], []| {
                    SQRT.call_builtin(ABS.call_builtin(NORM_SQUARED.call(
                        Type::Space(space_0),
                        [],
                        symbol_0.into(),
                        [],
                    )))
                },
            );
        }
        for space_0 in Space::iter(self.dim) {
            INVERSE.register_implementation(
                &mut items,
                Type::Space(space_0),
                [],
                [Type::Space(space_0)],
                |[symbol_0], []| {
                    DIV.call(
                        Type::Space(space_0),
                        [Type::Base],
                        REVERSE.call(Type::Space(space_0), [], symbol_0.into(), []),
                        [NORM_SQUARED.call(Type::Space(space_0), [], symbol_0.into(), [])],
                    )
                },
            );
        }
        for space_0 in Space::iter(self.dim) {
            NORMALIZED.register_implementation(
                &mut items,
                Type::Space(space_0),
                [],
                [Type::Space(space_0)],
                |[symbol_0], []| {
                    DIV.call(
                        Type::Space(space_0),
                        [Type::Base],
                        symbol_0.into(),
                        [NORM.call(Type::Space(space_0), [], symbol_0.into(), [])],
                    )
                },
            );
        }
        for space_0 in Space::iter(self.dim) {
            NORMALIZE.register_implementation(
                &mut items,
                Type::Space(space_0),
                [],
                [],
                |[symbol_0], []| {
                    Vec::from([DIV_ASSIGN.call(
                        Type::Space(space_0),
                        [Type::Base],
                        symbol_0.into(),
                        [NORM.call(Type::Space(space_0), [], symbol_0.into(), [])],
                    )])
                },
            );
        }

        let binary_multinomial_operations: [(_, fn([Space; 2], usize) -> Type, _); _] = [
            (
                &GEOMETRIC_PRODUCT,
                |[space_0, space_1], _| Type::Space(space_0.product_space(space_1)),
                self.multinomial([0, 1], |[blade_0, blade_1], _| {
                    Some(blade_0.parity(blade_1))
                }),
            ),
            (
                &SCALAR_PRODUCT,
                |_, _| Type::Base,
                self.multinomial([0, 1], |[blade_0, blade_1], _| {
                    (blade_0 ^ blade_1 == Blade::zero()).then(|| blade_0.parity(blade_1))
                }),
            ),
            (
                &LEFT_INNER_PRODUCT,
                |[space_0, space_1], _| {
                    Type::Space(match [space_0, space_1] {
                        [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                            grade_1
                                .checked_sub(grade_0)
                                .map(|grade| Space::GradedVector { grade })
                                .unwrap_or_else(Space::null)
                        }
                        [space_0, space_1] => space_0.product_space(space_1),
                    })
                },
                self.multinomial([0, 1], |[blade_0, blade_1], _| {
                    (blade_0 & blade_1 == blade_0).then(|| blade_0.parity(blade_1))
                }),
            ),
            (
                &RIGHT_INNER_PRODUCT,
                |[space_0, space_1], _| {
                    Type::Space(match [space_0, space_1] {
                        [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                            grade_0
                                .checked_sub(grade_1)
                                .map(|grade| Space::GradedVector { grade })
                                .unwrap_or_else(Space::null)
                        }
                        [space_0, space_1] => space_0.product_space(space_1),
                    })
                },
                self.multinomial([0, 1], |[blade_0, blade_1], _| {
                    (blade_0 & blade_1 == blade_1).then(|| blade_0.parity(blade_1))
                }),
            ),
            (
                &INNER_PRODUCT,
                |[space_0, space_1], _| {
                    Type::Space(match [space_0, space_1] {
                        [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                            Space::GradedVector {
                                grade: grade_0.abs_diff(grade_1),
                            }
                        }
                        [space_0, space_1] => space_0.product_space(space_1),
                    })
                },
                self.multinomial([0, 1], |[blade_0, blade_1], _| {
                    (blade_0 & blade_1 == blade_0 || blade_0 & blade_1 == blade_1)
                        .then(|| blade_0.parity(blade_1))
                }),
            ),
            (
                &OUTER_PRODUCT,
                |[space_0, space_1], dim| {
                    Type::Space(match [space_0, space_1] {
                        [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                            Some(grade_0 + grade_1)
                                .filter(|&grade| grade <= dim)
                                .map(|grade| Space::GradedVector { grade })
                                .unwrap_or_else(Space::null)
                        }
                        [space_0, space_1] => space_0.product_space(space_1),
                    })
                },
                self.multinomial([0, 1], |[blade_0, blade_1], _| {
                    (blade_0 & blade_1 == Blade::zero()).then(|| blade_0.parity(blade_1))
                }),
            ),
            (
                &REGRESSIVE_PRODUCT,
                |[space_0, space_1], dim| {
                    Type::Space(match [space_0.dual(dim), space_1.dual(dim)] {
                    [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                        Some(grade_0 + grade_1)
                            .filter(|&grade| grade <= dim)
                            .map(|grade| Space::GradedVector { grade })
                            .unwrap_or_else(Space::null)
                    }
                    [space_0, space_1] => space_0.product_space(space_1),
                }.dual(dim))
                },
                self.multinomial([0, 1], |[blade_0, blade_1], dim| {
                    {
                        (blade_0.dual(dim) & blade_1.dual(dim) == Blade::zero())
                            .then(|| blade_1.dual(dim).parity(blade_0.dual(dim)))
                    }
                })
                .dual(self.dim),
            ),
            (
                &COMMUTATOR,
                |[space_0, space_1], _| Type::Space(space_0.product_space(space_1)),
                self.multinomial([0, 1], |[blade_0, blade_1], _| {
                    match (blade_0.parity(blade_1), blade_1.parity(blade_0)) {
                        (Sign::Pos, Sign::Pos) => None,
                        (Sign::Pos, Sign::Neg) => Some(Sign::Pos),
                        (Sign::Neg, Sign::Pos) => Some(Sign::Neg),
                        (Sign::Neg, Sign::Neg) => None,
                    }
                }),
            ),
            (
                &ANTICOMMUTATOR,
                |[space_0, space_1], _| Type::Space(space_0.product_space(space_1)),
                self.multinomial([0, 1], |[blade_0, blade_1], _| {
                    match (blade_0.parity(blade_1), blade_1.parity(blade_0)) {
                        (Sign::Pos, Sign::Pos) => Some(Sign::Pos),
                        (Sign::Pos, Sign::Neg) => None,
                        (Sign::Neg, Sign::Pos) => None,
                        (Sign::Neg, Sign::Neg) => Some(Sign::Neg),
                    }
                }),
            ),
            (
                &TRANSFORM,
                |[space_0, space_1], _| {
                    Type::Space(space_1.product_space(space_0).product_space(space_1))
                },
                self.multinomial([1, 0, 1], |[blade_0, blade_1, blade_2], _| {
                    Some(
                        blade_0.parity(blade_1)
                            ^ blade_0.parity(blade_2)
                            ^ blade_1.parity(blade_2)
                            ^ Sign::from(blade_2.grade() >> 1),
                    )
                }),
            ),
            (
                &PROJECT,
                |[space_0, space_1], _| {
                    Type::Space(match [space_0, space_1] {
                        [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                            Some(grade_0)
                                .filter(|&grade| grade < grade_1)
                                .map(|grade| Space::GradedVector { grade })
                                .unwrap_or_else(Space::null)
                        }
                        [space_0, _] => Space::SaturatedVector {
                            even: space_0.contains_even_grade(),
                            odd: space_0.contains_odd_grade(),
                        },
                    })
                },
                self.multinomial([0, 1, 1], |[blade_0, blade_1, blade_2], _| {
                    (blade_0 & blade_1 == blade_0
                        && (blade_0 ^ blade_1) & blade_2 == blade_0 ^ blade_1)
                        .then(|| {
                            blade_0.parity(blade_1)
                                ^ blade_0.parity(blade_2)
                                ^ blade_1.parity(blade_2)
                                ^ Sign::from(blade_2.grade() >> 1)
                        })
                }),
            ),
            (
                &REJECT,
                |[space_0, space_1], dim| {
                    Type::Space(match [space_0, space_1] {
                        [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                            Some(grade_0)
                                .filter(|&grade| grade + grade_1 <= dim)
                                .map(|grade| Space::GradedVector { grade })
                                .unwrap_or_else(Space::null)
                        }
                        [space_0, _] => Space::SaturatedVector {
                            even: space_0.contains_even_grade(),
                            odd: space_0.contains_odd_grade(),
                        },
                    })
                },
                self.multinomial([0, 1, 1], |[blade_0, blade_1, blade_2], _| {
                    (blade_0 & blade_1 == Blade::zero() && (blade_0 ^ blade_1) & blade_2 == blade_2)
                        .then(|| {
                            blade_0.parity(blade_1)
                                ^ blade_0.parity(blade_2)
                                ^ blade_1.parity(blade_2)
                                ^ Sign::from(blade_2.grade() >> 1)
                        })
                }),
            ),
        ];
        for (operation, space_fn, multinomial) in binary_multinomial_operations {
            for (space_0, space_1) in Space::iter(self.dim).cartesian_product(Space::iter(self.dim))
            {
                let r#type = space_fn([space_0, space_1], self.dim);
                operation.register_implementation(
                    &mut items,
                    Type::Space(space_0),
                    [Type::Space(space_1)],
                    [r#type],
                    |[symbol_0], [symbol_1]| match r#type {
                        Type::Base => multinomial.blade_expr(
                            &structure_map,
                            Blade::zero(),
                            [space_0, space_1],
                            [symbol_0, symbol_1],
                        ),
                        Type::Space(space) => multinomial.expr(
                            &structure_map,
                            space,
                            [space_0, space_1],
                            [symbol_0, symbol_1],
                        ),
                    },
                )
            }
        }

        items
    }
}

pub struct GeometricAlgebraDimension {
    dim: usize,
    blade_intrinsic_signs: Vec<Sign>,
    idents: GeometricAlgebraIdents,
}

impl GeometricAlgebraDimension {
    pub fn new(dim: usize, blades: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        let blades = blades.into_iter().collect_vec();
        assert_eq!(blades.len(), 1 << dim);
        let (blade_idents, blade_intrinsic_signs): (Vec<_>, Vec<_>) = blades
            .into_iter()
            .map(|name| match name.as_ref().strip_prefix('-') {
                None => (Symbol::from(name), Sign::Pos),
                Some(name) => (Symbol::from(name), Sign::Neg),
            })
            .unzip();

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
        Self {
            dim,
            blade_intrinsic_signs,
            idents: GeometricAlgebraIdents {
                blade_idents,
                precision: Symbol::from("f32"),
                graded_spaces: (0..=dim)
                    .map(|index| {
                        GRADED_SPACES
                            .get(index)
                            .map_or_else(|| Symbol::from(format!("Vector{index}")), Symbol::from)
                    })
                    .collect(),
                null_space: Symbol::from("Null"),
                even_space: Symbol::from("EvenMultivector"),
                odd_space: Symbol::from("OddMultivector"),
                full_space: Symbol::from("Multivector"),
            },
        }
    }

    pub fn precision(mut self, precision: impl AsRef<str>) -> Self {
        self.idents.precision = Symbol::from(precision);
        self
    }

    pub fn graded_spaces(
        mut self,
        graded_spaces: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        let graded_spaces = graded_spaces.into_iter().map(Symbol::from).collect_vec();
        assert_eq!(graded_spaces.len(), self.dim + 1);
        self.idents.graded_spaces = graded_spaces;
        self
    }

    pub fn saturated_spaces(
        mut self,
        null_space: impl AsRef<str>,
        even_space: impl AsRef<str>,
        odd_space: impl AsRef<str>,
        full_space: impl AsRef<str>,
    ) -> Self {
        self.idents.null_space = Symbol::from(null_space);
        self.idents.even_space = Symbol::from(even_space);
        self.idents.odd_space = Symbol::from(odd_space);
        self.idents.full_space = Symbol::from(full_space);
        self
    }

    pub fn items(&self, generator_squares: impl IntoIterator<Item = Coefficient>) -> Items {
        let generator_squares = generator_squares.into_iter().collect_vec();
        assert_eq!(self.dim, generator_squares.len());
        let algebra = GeometricAlgebra {
            dim: self.dim,
            generator_squares,
            blade_intrinsic_signs: self.blade_intrinsic_signs.clone(),
        };
        algebra.items(&self.idents)
    }
}
