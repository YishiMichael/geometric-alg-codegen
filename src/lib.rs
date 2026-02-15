mod ast;
mod emitter;

use crate::ast::{
    Expr, ExprRepr, Items, OperationHandle, OperationSignature, Ownership, Pretype, ToSymbol,
};
use itertools::Itertools;
use symbol::Symbol;

type Coefficient = i32;

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
            .map(|offset| self.generator_bits >> offset)
            .take_while(|&generator_bits| generator_bits != 0)
            .fold(0, std::ops::BitXor::bitxor);
        Sign::from((gray_inv >> 1 & other.generator_bits).count_ones() as usize)
    }
}

struct BladeInfo {
    ident: Symbol,
    intrinsic_sign: Sign,
}

struct GeometricAlgebra {
    name: Symbol,
    dim: usize,
    signs: Vec<Coefficient>,
    blades: Vec<BladeInfo>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Space {
    GradedVector(usize),
    EvenMultivector,
    OddMultivector,
    Multivector,
}

impl ToSymbol for Space {
    fn to_symbol(&self) -> Symbol {
        match self {
            Self::GradedVector(0) => "f32".into(), // TODO
            Self::GradedVector(1) => "Vector".into(),
            Self::GradedVector(2) => "Bivector".into(),
            Self::GradedVector(3) => "Trivector".into(),
            Self::GradedVector(4) => "FourVector".into(),
            Self::GradedVector(5) => "FiveVector".into(),
            Self::GradedVector(6) => "SixVector".into(),
            Self::GradedVector(7) => "SevenVector".into(),
            Self::GradedVector(8) => "EightVector".into(),
            Self::GradedVector(9) => "NineVector".into(),
            Self::GradedVector(10) => "TenVector".into(),
            Self::GradedVector(11) => "ElevenVector".into(),
            Self::GradedVector(12) => "TwelveVector".into(),
            Self::GradedVector(grade) => format!("Vector{grade}").into(),
            Self::EvenMultivector => "EvenMultivector".into(),
            Self::OddMultivector => "OddMultivector".into(),
            Self::Multivector => "Multivector".into(),
        }
    }
}

impl Space {
    const SCALAR: Self = Self::GradedVector(0);

    fn iter(dim: usize) -> impl Iterator<Item = Self> + Clone {
        (0..=dim).map(Space::GradedVector).chain([
            Space::EvenMultivector,
            Space::OddMultivector,
            Space::Multivector,
        ])
    }

    fn contains_blade(self, blade: Blade) -> bool {
        match self {
            Self::GradedVector(grade) => blade.grade() == grade,
            Self::EvenMultivector => blade.grade() & 1 == 0,
            Self::OddMultivector => blade.grade() & 1 == 1,
            Self::Multivector => true,
        }
    }

    fn blades(self, dim: usize) -> Vec<Blade> {
        Blade::iter(dim)
            .filter(|&blade| self.contains_blade(blade))
            .collect()
    }
}

struct Multinomial<const PARAMS: usize, const DEGREE: usize>(
    std::collections::BTreeMap<
        Blade,
        std::collections::BTreeMap<[(usize, Blade); DEGREE], Coefficient>,
    >,
);

impl<const PARAMS: usize, const DEGREE: usize>
    FromIterator<(Blade, [(usize, Blade); DEGREE], Coefficient)> for Multinomial<PARAMS, DEGREE>
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Blade, [(usize, Blade); DEGREE], Coefficient)>,
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

struct MultinomialConfig<const DEGREE: usize> {
    reindex: fn([Blade; DEGREE], usize) -> [Blade; DEGREE],
    term_filter: fn([Blade; DEGREE], usize) -> bool,
    term_sign: fn([Blade; DEGREE], usize) -> Sign,
}

impl<const DEGREE: usize> Default for MultinomialConfig<DEGREE> {
    fn default() -> Self {
        Self {
            reindex: |blades, _| blades,
            term_filter: |_, _| true,
            term_sign: |_, _| Sign::POS,
        }
    }
}

impl GeometricAlgebra {
    fn iter_fields(&self, space: Space) -> Option<impl Iterator<Item = (Symbol, Space)>> {
        (space != Space::SCALAR).then(|| {
            space
                .blades(self.dim)
                .into_iter()
                .map(|blade| (self.blades[blade.generator_bits].ident, Space::SCALAR))
        })
    }

    fn field_expr(&self, space: Space, blade: Blade, expr: Expr) -> Option<Expr> {
        space.contains_blade(blade).then(|| {
            if space == Space::SCALAR {
                expr
            } else {
                ExprRepr::Field {
                    expr,
                    ident: self.blades[blade.generator_bits].ident,
                }
                .into()
            }
        })
    }

    fn struct_expr(&self, space: Space, field_expr_fn: impl Fn(Blade) -> Option<Expr>) -> Expr {
        if space == Space::SCALAR {
            field_expr_fn(Blade::zero()).unwrap_or(0.into())
        } else {
            ExprRepr::Struct {
                ident: space.to_symbol(),
                fields: space
                    .blades(self.dim)
                    .into_iter()
                    .map(|blade| {
                        (
                            self.blades[blade.generator_bits].ident,
                            field_expr_fn(blade).unwrap_or(0.into()),
                        )
                    })
                    .collect(),
            }
            .into()
        }
    }

    fn multinomial<const PARAMS: usize, const DEGREE: usize>(
        &self,
        prototype: [usize; DEGREE],
        config: MultinomialConfig<DEGREE>,
    ) -> Multinomial<PARAMS, DEGREE> {
        std::iter::repeat_n(Blade::iter(self.dim), DEGREE)
            .multi_cartesian_product()
            .map(|blades| -> [Blade; DEGREE] { blades.try_into().unwrap() })
            .map(|blades| (config.reindex)(blades, self.dim))
            .filter(|blades| (config.term_filter)(*blades, self.dim))
            .map(|blades| {
                let multi_index = std::array::from_fn(|index| (prototype[index], blades[index]));
                let blade = blades
                    .into_iter()
                    .fold(Blade::zero(), std::ops::BitXor::bitxor);
                let coeff = Coefficient::from(
                    (config.term_sign)(blades, self.dim)
                        ^ blades
                            .into_iter()
                            .map(|blade| self.blades[blade.generator_bits].intrinsic_sign)
                            .fold(
                                self.blades[blade.generator_bits].intrinsic_sign,
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
            .collect()
    }

    fn multinomial_expr<const PARAMS: usize, const DEGREE: usize>(
        &self,
        multinomial: &Multinomial<PARAMS, DEGREE>,
        space: Space,
        spaces: [Space; PARAMS],
        symbols: [Symbol; PARAMS],
    ) -> Expr {
        self.struct_expr(space, |blade| {
            multinomial.0.get(&blade).and_then(|polynomial| {
                polynomial
                    .iter()
                    .filter_map(|(multi_index, coeff)| {
                        multi_index
                            .iter()
                            .map(|&(index, blade)| {
                                self.field_expr(spaces[index], blade, symbols[index].into())
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
        })
    }

    fn items(&self) -> Items {
        let mut items = Items::default();
        Space::iter(self.dim)
            .filter_map(|space| self.iter_fields(space).map(|iter| (space, iter)))
            .for_each(|(space, iter)| {
                items.add_struct(space, iter);
            });

        let unary_signature = OperationSignature {
            generics: [],
            associates: ["Output"],
            self_param_item: [("self", Ownership::Owned(Pretype::<Space>::SelfBining))],
            param_items: [],
            return_pretype: [Ownership::Owned(Pretype::AssociateBinding(0))],
        };
        let binary_signature = OperationSignature {
            generics: ["T"],
            associates: ["Output"],
            self_param_item: [("self", Ownership::Owned(Pretype::SelfBining))],
            param_items: [("other", Ownership::Owned(Pretype::GenericBinding(0)))],
            return_pretype: [Ownership::Owned(Pretype::AssociateBinding(0))],
        };

        let mut add_product_operation =
            |ident, op_ident, space_fn: fn([Space; 2], usize) -> Space, multinomial_config| {
                let geometric_product_multinomial =
                    self.multinomial::<2, 2>([0, 1], multinomial_config);
                items.add_operation(
                    ident,
                    op_ident,
                    &binary_signature,
                    Space::iter(self.dim)
                        .cartesian_product(Space::iter(self.dim))
                        .map(|(space_0, space_1)| {
                            (space_0, [space_1], [space_fn([space_0, space_1], self.dim)])
                        }),
                    |space_0, [space_1], [space], [symbol_0], [symbol_1]| {
                        self.multinomial_expr(
                            &geometric_product_multinomial,
                            space,
                            [space_0, space_1],
                            [symbol_0, symbol_1],
                        )
                    },
                )
            };
        add_product_operation(
            "GeometricProduct",
            "geometric_product",
            |[space_0, space_1], _| {
                if matches!(space_0, Space::Multivector) || matches!(space_1, Space::Multivector) {
                    Space::Multivector
                } else if (matches!(space_0, Space::GradedVector(grade) if grade & 1 == 0)
                    || matches!(space_0, Space::EvenMultivector))
                    ^ (matches!(space_1, Space::GradedVector(grade) if grade & 1 == 0)
                        || matches!(space_1, Space::EvenMultivector))
                {
                    Space::OddMultivector
                } else {
                    Space::EvenMultivector
                }
            },
            MultinomialConfig {
                term_sign: |[blade_0, blade_1], _| blade_0.parity(blade_1),
                ..Default::default()
            },
        );
        add_product_operation(
            "ScalarProduct",
            "scalar_product",
            |_, _| Space::SCALAR,
            MultinomialConfig {
                term_filter: |[blade_0, blade_1], _| blade_0 ^ blade_1 == Blade::zero(),
                term_sign: |[blade_0, blade_1], _| blade_0.parity(blade_1),
                ..Default::default()
            },
        );
        add_product_operation(
            "LeftInnerProduct",
            "left_inner_product",
            |[space_0, space_1], _| Space::SCALAR, // TODO
            MultinomialConfig {
                term_filter: |[blade_0, blade_1], _| blade_0 & blade_1 == blade_0,
                term_sign: |[blade_0, blade_1], _| blade_0.parity(blade_1),
                ..Default::default()
            },
        );

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
        let signs: Vec<_> = value
            .signs
            .split_whitespace()
            .map(|sign| match sign {
                "+" => 1,
                "-" => -1,
                "0" => 0,
                _ => unreachable!(),
            })
            .collect();
        let blades: Vec<_> = value
            .blades
            .split_whitespace()
            .map(|name| match name.strip_prefix('-') {
                None => BladeInfo {
                    ident: Symbol::from(name),
                    intrinsic_sign: Sign::POS,
                },
                Some(name) => BladeInfo {
                    ident: Symbol::from(name),
                    intrinsic_sign: Sign::NEG,
                },
            })
            .collect();
        assert!(1 << signs.len() == blades.len());
        Self {
            name: Symbol::from(value.name),
            dim: signs.len(),
            signs,
            blades,
        }
    }
}

const ALGEBRAS: &[GeometricAlgebraMeta] = &[
    GeometricAlgebraMeta {
        name: "epga1d",
        signs: "+ -",
        blades: "s e0 e1 e01",
    },
    GeometricAlgebraMeta {
        name: "ppga1d",
        signs: "+ 0",
        blades: "s e0 e1 e01",
    },
    GeometricAlgebraMeta {
        name: "hpga1d",
        signs: "+ +",
        blades: "s e0 e1 e01",
    },
    GeometricAlgebraMeta {
        name: "epga2d",
        signs: "+ + -",
        blades: "s e0 e1 e01 e2 -e20 e12 e012",
    },
    GeometricAlgebraMeta {
        name: "ppga2d",
        signs: "+ + 0",
        blades: "s e0 e1 e01 e2 -e20 e12 e012",
    },
    GeometricAlgebraMeta {
        name: "hpga2d",
        signs: "+ + +",
        blades: "s e0 e1 e01 e2 -e20 e12 e012",
    },
    GeometricAlgebraMeta {
        name: "epga3d",
        signs: "+ + + -",
        blades: "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123",
    },
    GeometricAlgebraMeta {
        name: "ppga3d",
        signs: "+ + + 0",
        blades: "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123",
    },
    GeometricAlgebraMeta {
        name: "hpga3d",
        signs: "+ + + +",
        blades: "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123",
    },
];

fn f() {
    // let alg = GeometricAlgebra {};
}
