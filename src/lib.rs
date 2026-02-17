#![allow(unused)]

mod ast;
mod emitter;
mod traits;

use crate::ast::{Expr, Items, OperationSignature, Ownership, Pretype, StructureSignature};
use ast::ContextTrait;
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

// struct BladeInfo {
//     ident: Symbol,
//     intrinsic_sign: Sign,
// }

struct GeometricAlgebra {
    name: Symbol,
    dim: usize,
    signs: Vec<Coefficient>,
    blade_idents: Vec<Symbol>,
    blade_intrinsic_signs: Vec<Sign>,
}

#[derive(Clone, Copy)]
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

    fn blade_expr<Context, const PARAMS: usize>(
        &self,
        structure_map: &std::collections::BTreeMap<Space, StructureSignature<Context, Blade>>,
        blade: Blade,
        spaces: [Space; PARAMS],
        symbols: [Symbol; PARAMS],
    ) -> Expr
    where
        Context: ContextTrait<Component = Blade>,
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

    fn expr<Context, const PARAMS: usize>(
        &self,
        structure_map: &std::collections::BTreeMap<Space, StructureSignature<Context, Blade>>,
        space: Space,
        spaces: [Space; PARAMS],
        symbols: [Symbol; PARAMS],
    ) -> Expr
    where
        Context: ContextTrait<Component = Blade>,
    {
        structure_map[&space]
            .construct(|&blade| self.blade_expr(structure_map, blade, spaces, symbols))
    }
}

// struct MultinomialConfig<const DEGREE: usize> {
//     reindex: fn([Blade; DEGREE], usize) -> [Blade; DEGREE],
//     term_filter: fn([Blade; DEGREE], usize) -> bool,
//     term_sign: fn([Blade; DEGREE], usize) -> Sign,
// }

// impl<const DEGREE: usize> Default for MultinomialConfig<DEGREE> {
//     fn default() -> Self {
//         Self {
//             reindex: |blades, _| blades,
//             term_filter: |_, _| true,
//             term_sign: |_, _| Sign::Pos,
//         }
//     }
// }

impl ContextTrait for GeometricAlgebra {
    type Component = Blade;
    type Type = Type;

    fn resolve_component(&self, blade: &Self::Component) -> Symbol {
        self.blade_idents[blade.generator_bits]
    }

    fn resolve_type(&self, r#type: &Self::Type) -> Symbol {
        match r#type {
            Type::Base => "f32".into(),
            Type::Space(Space::GradedVector { grade: 0 }) => "Scalar".into(),
            Type::Space(Space::GradedVector { grade: 1 }) => "Vector".into(),
            Type::Space(Space::GradedVector { grade: 2 }) => "Bivector".into(),
            Type::Space(Space::GradedVector { grade: 3 }) => "Trivector".into(),
            Type::Space(Space::GradedVector { grade: 4 }) => "FourVector".into(),
            Type::Space(Space::GradedVector { grade: 5 }) => "FiveVector".into(),
            Type::Space(Space::GradedVector { grade: 6 }) => "SixVector".into(),
            Type::Space(Space::GradedVector { grade: 7 }) => "SevenVector".into(),
            Type::Space(Space::GradedVector { grade: 8 }) => "EightVector".into(),
            Type::Space(Space::GradedVector { grade: 9 }) => "NineVector".into(),
            Type::Space(Space::GradedVector { grade: 10 }) => "TenVector".into(),
            Type::Space(Space::GradedVector { grade: 11 }) => "ElevenVector".into(),
            Type::Space(Space::GradedVector { grade: 12 }) => "TwelveVector".into(),
            Type::Space(Space::GradedVector { grade }) => format!("Vector{grade}").into(),
            Type::Space(Space::SaturatedVector {
                even: false,
                odd: false,
            }) => "Null".into(),
            Type::Space(Space::SaturatedVector {
                even: false,
                odd: true,
            }) => "OddMultivector".into(),
            Type::Space(Space::SaturatedVector {
                even: true,
                odd: false,
            }) => "EvenMultivector".into(),
            Type::Space(Space::SaturatedVector {
                even: true,
                odd: true,
            }) => "Multivector".into(),
        }
    }
}

impl GeometricAlgebra {
    // fn iter_fields(&self, space: Space) -> Option<impl Iterator<Item = (Symbol, Space)>> {
    //     (space != Space::SCALAR).then(|| {
    //         space
    //             .blades(self.dim)
    //             .into_iter()
    //             .map(|blade| (self.blades[blade.generator_bits].ident, Space::SCALAR))
    //     })
    // }

    // fn field_expr(&self, space: Space, blade: Blade, expr: Expr) -> Option<Expr> {
    //     space.contains_blade(blade).then(|| {
    //         if space == Space::SCALAR {
    //             expr
    //         } else {
    //             ExprRepr::Field {
    //                 expr,
    //                 ident: self.blades[blade.generator_bits].ident,
    //             }
    //             .into()
    //         }
    //     })
    // }

    // fn struct_expr(&self, space: Space, field_expr_fn: impl Fn(Blade) -> Option<Expr>) -> Expr {
    //     if space == Space::SCALAR {
    //         field_expr_fn(Blade::zero()).unwrap_or(0.into())
    //     } else {
    //         ExprRepr::Struct {
    //             ident: space.to_symbol(),
    //             fields: space
    //                 .blades(self.dim)
    //                 .into_iter()
    //                 .map(|blade| {
    //                     (
    //                         self.blades[blade.generator_bits].ident,
    //                         field_expr_fn(blade).unwrap_or(0.into()),
    //                     )
    //                 })
    //                 .collect(),
    //         }
    //         .into()
    //     }
    // }

    fn multinomial<const DEGREE: usize>(
        &self,
        prototype: [usize; DEGREE],
        term_sign: fn([Blade; DEGREE], usize) -> Option<Sign>,
        // config: MultinomialConfig<DEGREE>,
    ) -> Multinomial {
        std::iter::repeat_n(Blade::iter(self.dim), DEGREE)
            .multi_cartesian_product()
            .map(|blades| -> [Blade; DEGREE] { blades.try_into().unwrap() })
            // .map(|blades| (config.reindex)(blades, self.dim))
            // .filter(|blades| (config.term_filter)(*blades, self.dim))
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
                        .map(|generator| self.signs[generator])
                        .product::<Coefficient>();
                    (blade, multi_index, coeff)
                })
            })
            .collect()
    }

    fn unary_multinomial_operations(
        &self,
    ) -> [(
        &'static str,
        &'static str,
        fn([Space; 1], usize) -> Type,
        Multinomial,
    ); 6] {
        [
            (
                "GradeInvolution",
                "grade_involution",
                |[space_0], _| Type::Space(space_0),
                self.multinomial([0], |[blade_0], _| Some(Sign::from(blade_0.grade()))),
            ),
            (
                "Reverse",
                "reverse",
                |[space_0], _| Type::Space(space_0),
                self.multinomial([0], |[blade_0], _| Some(Sign::from(blade_0.grade() >> 1))),
            ),
            (
                "Conjugate",
                "conjugate",
                |[space_0], _| Type::Space(space_0),
                self.multinomial([0], |[blade_0], _| {
                    Some(Sign::from((blade_0.grade() + 1) >> 1))
                }),
            ),
            (
                "Dual",
                "dual",
                |[space_0], dim| Type::Space(space_0.dual(dim)),
                self.multinomial([0], |[blade_0], dim| {
                    Some(blade_0.dual(dim).parity(blade_0))
                })
                .dual(self.dim),
            ),
            (
                "Undual",
                "undual",
                |[space_0], dim| Type::Space(space_0.dual(dim)),
                self.multinomial([0], |[blade_0], dim| {
                    Some(blade_0.parity(blade_0.dual(dim)))
                })
                .dual(self.dim),
            ),
            (
                "NormSquared",
                "norm_squared",
                |_, _| Type::Base,
                self.multinomial([0, 0], |_, _| Some(Sign::Pos)),
            ),
        ]
    }

    fn binary_multinomial_operations(
        &self,
    ) -> [(
        &'static str,
        &'static str,
        fn([Space; 2], usize) -> Type,
        Multinomial,
    ); 12] {
        [
            (
                "GeometricProduct",
                "geometric_product",
                |[space_0, space_1], _| Type::Space(space_0.product_space(space_1)),
                self.multinomial([0, 1], |[blade_0, blade_1], _| {
                    Some(blade_0.parity(blade_1))
                }),
            ),
            (
                "ScalarProduct",
                "scalar_product",
                |_, _| Type::Base,
                self.multinomial([0, 1], |[blade_0, blade_1], _| {
                    (blade_0 ^ blade_1 == Blade::zero()).then(|| blade_0.parity(blade_1))
                }),
            ),
            (
                "LeftInnerProduct",
                "left_inner_product",
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
                "RightInnerProduct",
                "right_inner_product",
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
                "InnerProduct",
                "inner_product",
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
                "OuterProduct",
                "outer_product",
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
                "RegressiveProduct",
                "regressive_product",
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
                "Commutator",
                "commutator",
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
                "Anticommutator",
                "anticommutator",
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
                "Transform",
                "transform",
                |[space_0, space_1], _| {
                    Type::Space(space_1.product_space(space_0).product_space(space_1))
                },
                self.multinomial([1, 0, 1], |[blade_0, blade_1, blade_2], dim| {
                    Some(
                        blade_0.parity(blade_1)
                            ^ blade_0.parity(blade_2)
                            ^ blade_1.parity(blade_2)
                            ^ Sign::from(blade_2.grade() >> 1),
                    )
                }),
            ),
            (
                "Project",
                "project",
                |[space_0, space_1], _| {
                    Type::Space(match [space_0, space_1] {
                        [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                            Some(grade_0)
                                .filter(|&grade| grade < grade_1)
                                .map(|grade| Space::GradedVector { grade })
                                .unwrap_or_else(Space::null)
                        }
                        [space_0, space_1] => Space::SaturatedVector {
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
                "Reject",
                "reject",
                |[space_0, space_1], dim| {
                    Type::Space(match [space_0, space_1] {
                        [Space::GradedVector { grade: grade_0 }, Space::GradedVector { grade: grade_1 }] => {
                            Some(grade_0)
                                .filter(|&grade| grade + grade_1 <= dim)
                                .map(|grade| Space::GradedVector { grade })
                                .unwrap_or_else(Space::null)
                        }
                        [space_0, space_1] => Space::SaturatedVector {
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
        ]
    }

    fn items(&self) -> Items {
        let mut items = Items::default();

        let structure_map = Space::iter(self.dim)
            .map(|space| {
                (
                    space,
                    self.register_structure(
                        &mut items,
                        self.resolve_type(&Type::Space(space)).as_str(),
                        space.iter_blades(self.dim),
                        |_| Type::Base,
                    ),
                )
            })
            .collect();

        // for (ident, op_ident, space_fn, multinomial) in self.binary_multinomial_operations() {
        //     self.register_operation(
        //         &mut items,
        //         ident,
        //         op_ident,
        //         ["T"],
        //         ["Output"],
        //         [("self", Ownership::Owned(Pretype::SelfBining))],
        //         [("other", Ownership::Owned(Pretype::GenericBinding(0)))],
        //         [Pretype::AssociateBinding(0)],
        //         |[symbol_0], [symbol_1]| {
        //             let structure_map = &structure_map;
        //             let multinomial = &multinomial;
        //             Space::iter(self.dim)
        //                 .cartesian_product(Space::iter(self.dim))
        //                 .map(move |(space_0, space_1)| {
        //                     let space = space_fn([space_0, space_1], self.dim);
        //                     (
        //                         Type::Space(space_0),
        //                         [Type::Space(space_1)],
        //                         [Type::Space(space)],
        //                         self.multinomial_expr(
        //                             structure_map,
        //                             multinomial,
        //                             space,
        //                             [space_0, space_1],
        //                             [symbol_0, symbol_1],
        //                         ),
        //                     )
        //                 })
        //         },
        //     );
        // }

        for (ident, op_ident, space_fn, multinomial) in self.unary_multinomial_operations() {
            self.register_operation(
                &mut items,
                ident,
                op_ident,
                [],
                ["Output"],
                [("self", Ownership::Owned(Pretype::SelfBining))],
                [],
                [Pretype::AssociateBinding(0)],
                |[symbol_0], []| {
                    let structure_map = &structure_map;
                    Space::iter(self.dim).map(move |space_0| {
                        let r#type = space_fn([space_0], self.dim);
                        (
                            Type::Space(space_0),
                            [],
                            [r#type],
                            match r#type {
                                Type::Base => multinomial.blade_expr(
                                    structure_map,
                                    Blade::zero(),
                                    [space_0],
                                    [symbol_0],
                                ),
                                Type::Space(space) => {
                                    multinomial.expr(structure_map, space, [space_0], [symbol_0])
                                }
                            },
                        )
                    })
                },
            );
        }

        for (ident, op_ident, space_fn, multinomial) in self.binary_multinomial_operations() {
            self.register_operation(
                &mut items,
                ident,
                op_ident,
                ["T"],
                ["Output"],
                [("self", Ownership::Owned(Pretype::SelfBining))],
                [("other", Ownership::Owned(Pretype::GenericBinding(0)))],
                [Pretype::AssociateBinding(0)],
                |[symbol_0], [symbol_1]| {
                    let structure_map = &structure_map;
                    Space::iter(self.dim)
                        .cartesian_product(Space::iter(self.dim))
                        .map(move |(space_0, space_1)| {
                            let r#type = space_fn([space_0, space_1], self.dim);
                            (
                                Type::Space(space_0),
                                [Type::Space(space_1)],
                                [r#type],
                                match r#type {
                                    Type::Base => multinomial.blade_expr(
                                        structure_map,
                                        Blade::zero(),
                                        [space_0, space_1],
                                        [symbol_0, symbol_1],
                                    ),
                                    Type::Space(space) => multinomial.expr(
                                        structure_map,
                                        space,
                                        [space_0, space_1],
                                        [symbol_0, symbol_1],
                                    ),
                                },
                            )
                        })
                },
            );
        }

        // let unary_signature = OperationSignature {
        //     generics: [],
        //     associates: ["Output"],
        //     self_param_item: [("self", Ownership::Owned(Pretype::<Space>::SelfBining))],
        //     param_items: [],
        //     return_pretype: [Pretype::AssociateBinding(0)],
        // };
        // let binary_signature = OperationSignature {
        //     generics: ["T"],
        //     associates: ["Output"],
        //     self_param_item: [("self", Ownership::Owned(Pretype::SelfBining))],
        //     param_items: [("other", Ownership::Owned(Pretype::GenericBinding(0)))],
        //     return_pretype: [Pretype::AssociateBinding(0)],
        // };

        // let mut add_product_operation_general =
        //     |ident, op_ident, space_fn: fn([Space; 2], usize) -> Space, multinomial| {
        //         self.register_operation(
        //             &mut items,
        //             ident,
        //             op_ident,
        //             ["T"],
        //             ["Output"],
        //             [("self", Ownership::Owned(Pretype::SelfBining))],
        //             [("other", Ownership::Owned(Pretype::GenericBinding(0)))],
        //             [Pretype::AssociateBinding(0)],
        //             |[symbol_0], [symbol_1]| {
        //                 let structure_map = &structure_map;
        //                 let multinomial = &multinomial;
        //                 Space::iter(self.dim)
        //                     .cartesian_product(Space::iter(self.dim))
        //                     .map(move |(space_0, space_1)| {
        //                         let space = space_fn([space_0, space_1], self.dim);
        //                         (
        //                             Type::Space(space_0),
        //                             [Type::Space(space_1)],
        //                             [Type::Space(space)],
        //                             self.multinomial_expr(
        //                                 structure_map,
        //                                 multinomial,
        //                                 space,
        //                                 [space_0, space_1],
        //                                 [symbol_0, symbol_1],
        //                             ),
        //                         )
        //                     })
        //             },
        //         )
        //     };
        // let mut add_product_operation =
        //     |ident, op_ident, space_fn: fn([Space; 2], usize) -> Space, term_sign| {
        //         add_product_operation_general(
        //             ident,
        //             op_ident,
        //             space_fn,
        //             self.multinomial([0, 1], term_sign),
        //         )
        //     };
        // struct BinaryProduct {
        //     ident: &'static str,
        //     op_ident: &'static str,
        //     space_fn: fn([Space; 2], usize) -> Space,
        //     multinomial: Multinomial,
        // }

        items
    }
}

struct GeometricAlgebraMeta {
    name: &'static str,
    signs: &'static str,
    blades: &'static str,
}

impl From<&GeometricAlgebraMeta> for GeometricAlgebra {
    fn from(value: &GeometricAlgebraMeta) -> Self {
        let signs = value
            .signs
            .split_whitespace()
            .map(|sign| match sign {
                "+" => 1,
                "-" => -1,
                "0" => 0,
                _ => unreachable!(),
            })
            .collect_vec();
        let (blade_idents, blade_intrinsic_signs) = value
            .blades
            .split_whitespace()
            .map(|name| match name.strip_prefix('-') {
                None => (Symbol::from(name), Sign::Pos),
                Some(name) => (Symbol::from(name), Sign::Neg),
            })
            .unzip();
        Self {
            name: Symbol::from(value.name),
            dim: signs.len(),
            signs,
            blade_idents,
            blade_intrinsic_signs,
        }
    }
}

const ALGEBRAS: &[GeometricAlgebraMeta] = &[
    GeometricAlgebraMeta {
        name: "epga1d",
        signs: "- +",
        blades: "s e0 e1 e01",
    },
    GeometricAlgebraMeta {
        name: "ppga1d",
        signs: "0 +",
        blades: "s e0 e1 e01",
    },
    GeometricAlgebraMeta {
        name: "hpga1d",
        signs: "+ +",
        blades: "s e0 e1 e01",
    },
    GeometricAlgebraMeta {
        name: "epga2d",
        signs: "- + +",
        blades: "s e0 e1 e01 e2 -e20 e12 e012",
    },
    GeometricAlgebraMeta {
        name: "ppga2d",
        signs: "0 + +",
        blades: "s e0 e1 e01 e2 -e20 e12 e012",
    },
    GeometricAlgebraMeta {
        name: "hpga2d",
        signs: "+ + +",
        blades: "s e0 e1 e01 e2 -e20 e12 e012",
    },
    GeometricAlgebraMeta {
        name: "epga3d",
        signs: "- + + +",
        blades: "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123",
    },
    GeometricAlgebraMeta {
        name: "ppga3d",
        signs: "0 + + +",
        blades: "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123",
    },
    GeometricAlgebraMeta {
        name: "hpga3d",
        signs: "+ + + +",
        blades: "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123",
    },
];

mod epga3d;

#[test]
fn generate() {
    use emitter::rust::RustLang;
    use emitter::{Emitter, EmitterTrait};

    let alg = GeometricAlgebra::from(&GeometricAlgebraMeta {
        name: "epga3d",
        signs: "- + + +",
        blades: "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123",
    });
    let items = alg.items();

    let mut emitter = Emitter::new(std::fs::File::create("src/epga3d.rs").unwrap(), RustLang);
    emitter.emit_items(items).unwrap();
}
