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

    fn dual(self) -> Self {
        Self {
            grade: DIM - self.grade,
        }
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

    // fn from_grades(iter: impl Iterator<Item = Grade<DIM>>) -> Self {
    //     Self {grade_bits: iter.map(|grade| 1 << grade.grade).fold(0, std::ops::BitXor::bitxor)}
    // }

    fn iter_blades(self) -> impl Iterator<Item = Blade<DIM>> + Clone {
        self.iter_grades().flat_map(|grade| grade.iter_blades())
    }

    fn from_blades(blades: impl Iterator<Item = Blade<DIM>>) -> Self {
        Self {
            grade_bits: blades
                .map(|blade| 1 << blade.grade().grade)
                .fold(0, std::ops::BitOr::bitor),
        }
    }

    fn contains(self, other: Self) -> bool {
        self.grade_bits & other.grade_bits == other.grade_bits
    }

    fn contains_blade(self, blade: Blade<DIM>) -> bool {
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

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Binding {
    binding: usize,
}

// #[derive(Default)]
// struct Polynomial<const DIM: usize>(
//     std::collections::BTreeMap<Vec<(Binding, Blade<DIM>)>, Coefficient>,
// );

// impl<const DIM: usize> FromIterator<(Vec<(Binding, Blade<DIM>)>, Coefficient)> for Polynomial<DIM> {
//     fn from_iter<I>(iter: I) -> Self
//     where
//         I: IntoIterator<Item = (Vec<(Binding, Blade<DIM>)>, Coefficient)>,
//     {
//         let mut polynomial = Self::default();
//         iter.into_iter().for_each(|(multi_index, coeff)| {
//             polynomial.insert(multi_index, coeff);
//         });
//         polynomial
//     }
// }

// impl<const DIM: usize> IntoIterator for Polynomial<DIM> {
//     type Item = (Vec<(Binding, Blade<DIM>)>, Coefficient);
//     type IntoIter = std::collections::btree_map::IntoIter<Vec<(Binding, Blade<DIM>)>, Coefficient>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

// impl<const DIM: usize> Polynomial<DIM> {
//     fn is_empty(&self) -> bool {
//         self.0.is_empty()
//     }

//     fn iter(
//         &self,
//     ) -> std::collections::btree_map::Iter<'_, Vec<(Binding, Blade<DIM>)>, Coefficient> {
//         self.0.iter()
//     }

//     fn insert(
//         &mut self,
//         mut multi_index: Vec<(Binding, Blade<DIM>)>,
//         coeff: Coefficient,
//     ) -> &mut Self {
//         if coeff != 0 {
//             multi_index.sort();
//             match self.0.entry(multi_index) {
//                 std::collections::btree_map::Entry::Vacant(vacant) => {
//                     vacant.insert(coeff);
//                 }
//                 std::collections::btree_map::Entry::Occupied(mut occupied) => {
//                     let slot = occupied.get_mut();
//                     *slot += coeff;
//                     if *slot == 0 {
//                         occupied.remove_entry();
//                     }
//                 }
//             }
//         }
//         self
//     }

//     fn substitute(
//         self,
//         binding_substitutes: &std::collections::BTreeMap<(Binding, Blade<DIM>), Self>,
//     ) -> Self {
//         self.into_iter()
//             .flat_map(|(multi_index, coeff)| {
//                 multi_index
//                     .into_iter()
//                     .map(|index| binding_substitutes[&index].iter())
//                     .multi_cartesian_product()
//                     .map(move |monomials| {
//                         monomials.into_iter().fold(
//                             (Vec::new(), coeff),
//                             |(mut index_acc, mut coeff_acc), (index, coeff)| {
//                                 index_acc.extend(index);
//                                 coeff_acc *= coeff;
//                                 (index_acc, coeff_acc)
//                             },
//                         )
//                     })
//             })
//             .collect()
//     }

//     fn space_instantiate(&self, spaces: &std::collections::BTreeMap<Binding, Space<DIM>>) -> Self {
//         self.iter()
//             .filter(|(multi_index, _)| {
//                 multi_index
//                     .iter()
//                     .all(|(binding, blade)| spaces[binding].contains_blade(*blade))
//             })
//             .map(|(multi_index, blade)| (multi_index.clone(), *blade))
//             .collect()
//     }

//     fn into_expr(self) -> Box<Expr<DIM>> {
//         self.into_iter()
//             .fold(None, |expr_acc, (multi_index, coeff)| {
//                 let exprs = multi_index
//                     .into_iter()
//                     .map(|(binding, blade)| Expr::variable(binding).field(blade));
//                 let coeff_abs = coeff.unsigned_abs();
//                 let literal = Expr::literal(coeff_abs);
//                 Some(match (expr_acc, coeff_abs == 1, coeff < 0) {
//                     (Some(expr_acc), false, false) => {
//                         expr_acc + exprs.fold(literal, std::ops::Mul::mul)
//                     }
//                     (Some(expr_acc), false, true) => {
//                         expr_acc - exprs.fold(literal, std::ops::Mul::mul)
//                     }
//                     (Some(expr_acc), true, false) => {
//                         expr_acc + exprs.reduce(std::ops::Mul::mul).unwrap_or(literal)
//                     }
//                     (Some(expr_acc), true, true) => {
//                         expr_acc - exprs.reduce(std::ops::Mul::mul).unwrap_or(literal)
//                     }
//                     (None, false, false) => exprs.fold(literal, std::ops::Mul::mul),
//                     (None, false, true) => exprs.fold(-literal, std::ops::Mul::mul),
//                     (None, true, false) => exprs.reduce(std::ops::Mul::mul).unwrap_or(literal),
//                     (None, true, true) => exprs
//                         .fold(None, |expr_acc, expr| {
//                             Some(match expr_acc {
//                                 Some(expr_acc) => expr_acc * expr,
//                                 None => -expr,
//                             })
//                         })
//                         .unwrap_or(-literal),
//                 })
//             })
//             .unwrap_or_else(|| Expr::literal(0))
//     }
// }

struct Multinomial<const DIM: usize>(
    std::collections::BTreeMap<
        Blade<DIM>,
        std::collections::BTreeMap<Vec<(Binding, Blade<DIM>)>, Coefficient>,
    >,
);

impl<const DIM: usize> FromIterator<(Blade<DIM>, Vec<(Binding, Blade<DIM>)>, Coefficient)>
    for Multinomial<DIM>
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Blade<DIM>, Vec<(Binding, Blade<DIM>)>, Coefficient)>,
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

impl<const DIM: usize> IntoIterator for Multinomial<DIM> {
    type Item = (
        Blade<DIM>,
        std::collections::BTreeMap<Vec<(Binding, Blade<DIM>)>, Coefficient>,
    );
    type IntoIter = std::collections::btree_map::IntoIter<
        Blade<DIM>,
        std::collections::BTreeMap<Vec<(Binding, Blade<DIM>)>, Coefficient>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const DIM: usize> Multinomial<DIM> {
    fn iter(
        &self,
    ) -> std::collections::btree_map::Iter<
        '_,
        Blade<DIM>,
        std::collections::BTreeMap<Vec<(Binding, Blade<DIM>)>, Coefficient>,
    > {
        self.0.iter()
    }

    // fn insert(
    //     &mut self,
    //     blade: Blade<DIM>,
    //     multi_index: Vec<(Binding, Blade<DIM>)>,
    //     coeff: Coefficient,
    // ) -> &mut Self {
    //     if self
    //         .0
    //         .entry(blade)
    //         .or_default()
    //         .insert(multi_index, coeff)
    //         .is_empty()
    //     {
    //         self.0.remove(&blade);
    //     }
    //     self
    // }

    fn substitute(self, binding_substitutes: &[Self]) -> Self {
        self.into_iter()
            .flat_map(|(blade, polynomial)| {
                polynomial
                    .into_iter()
                    .flat_map(move |(multi_index, coeff)| {
                        multi_index
                            .into_iter()
                            .map(|(binding, binding_blade)| {
                                binding_substitutes[binding.binding].0[&binding_blade].iter()
                            })
                            .multi_cartesian_product()
                            .map(move |monomials| {
                                let (multi_index, coeff) = monomials.into_iter().fold(
                                    (Vec::new(), coeff),
                                    |(mut multi_index_acc, mut coeff_acc), (multi_index, coeff)| {
                                        multi_index_acc.extend(multi_index);
                                        coeff_acc *= coeff;
                                        (multi_index_acc, coeff_acc)
                                    },
                                );
                                (blade, multi_index, coeff)
                            })
                    })
            })
            .collect()
    }

    fn space_instantiate(&self, spaces: &[Space<DIM>]) -> Self {
        self.iter()
            .flat_map(|(blade, polynomial)| {
                polynomial
                    .iter()
                    .filter(|(multi_index, _)| {
                        multi_index.iter().all(|(binding, binding_blade)| {
                            spaces[binding.binding].contains_blade(*binding_blade)
                        })
                    })
                    .map(|(multi_index, coeff)| (*blade, multi_index.clone(), *coeff))
            })
            .collect()
    }

    fn polynomial_to_type_expr(
        polynomial: &std::collections::BTreeMap<Vec<(Binding, Blade<DIM>)>, Coefficient>,
    ) -> (Type<DIM>, Box<Expr<DIM>>) {
        (
            Type::Atom,
            polynomial
                .into_iter()
                .fold(None, |expr_acc, (multi_index, &coeff)| {
                    let exprs = multi_index.into_iter().map(|&(binding, binding_blade)| {
                        Expr::variable(binding).field(binding_blade)
                    });
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
        )
    }

    fn to_type_expr(&self) -> (Type<DIM>, Box<Expr<DIM>>) {
        let space = Space::from_blades(self.0.keys().cloned());
        (
            Type::Space(space),
            Expr::structure(
                space,
                space.iter_blades().map(|blade| {
                    (blade, {
                        match self.0.get(&blade) {
                            None => Expr::literal(0),
                            Some(polynomial) => {
                                let (_, polynomial_expr) =
                                    Self::polynomial_to_type_expr(polynomial);
                                polynomial_expr
                            }
                        }
                    })
                }),
            ),
        )
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
    fn involution(&self, grade_sign: fn(Grade<DIM>) -> Sign) -> Vec<Implementation<DIM>> {
        Space::iter()
            .map(|space| {
                let terms = space
                    .iter_blades()
                    .map(|blade| {
                        (
                            blade,
                            std::iter::once((blade, Coefficient::from(grade_sign(blade.grade()))))
                                .collect::<Terms<_, _>>(),
                        )
                    })
                    .collect::<Terms<_, _>>();
                let space_output = Space::from_blades(terms.keys().cloned());
                Implementation {
                    generic_types: [Type::Space(space)].into(),
                    self_var: Var {
                        name: "self",
                        ty: Type::Space(space),
                    },
                    arg_vars: [].into(),
                    return_type: Some(Type::Space(space_output)),
                    statements: [].into(),
                    return_expr: Some(Expr::structure(
                        space_output,
                        space_output.iter_blades().map(|blade| {
                            (blade, {
                                match terms.get(&blade) {
                                    None => Expr::literal(0),
                                    Some(terms) => terms
                                        .to_polynomial_expr(|blade| Expr::var("self").field(blade)),
                                }
                            })
                        }),
                    )),
                }
            })
            .collect()
    }

    fn dual(
        &self,
        // blade_map: fn(Blade<DIM>) -> Blade<DIM>,
        parity: fn(Blade<DIM>) -> Sign,
    ) -> Vec<Implementation<DIM>> {
        Space::iter()
            .map(|space| {
                let terms = space
                    .iter_blades()
                    .map(|blade| {
                        let blade_output = !blade;
                        (
                            blade_output,
                            std::iter::once((
                                blade,
                                Coefficient::from(
                                    parity(blade)
                                        ^ self[blade].intrinsic_sign
                                        ^ self[blade_output].intrinsic_sign,
                                ),
                            ))
                            .collect::<Terms<_, _>>(),
                        )
                    })
                    .collect::<Terms<_, _>>();
                let space_output = Space::from_blades(terms.keys().cloned());
                Implementation {
                    generic_types: [Type::Space(space)].into(),
                    self_var: Var {
                        name: "self",
                        ty: Type::Space(space),
                    },
                    arg_vars: [].into(),
                    return_type: Some(Type::Space(space_output)),
                    statements: [].into(),
                    return_expr: Some(Expr::structure(
                        space_output,
                        space_output.iter_blades().map(|blade| {
                            (blade, {
                                match terms.get(&blade) {
                                    None => Expr::literal(0),
                                    Some(terms) => terms
                                        .to_polynomial_expr(|blade| Expr::var("self").field(blade)),
                                }
                            })
                        }),
                    )),
                }
            })
            .collect()
    }

    fn product(
        &self,
        grade_filter: fn(Grade<DIM>, Grade<DIM>, Grade<DIM>) -> bool,
        parity: fn(Blade<DIM>, Blade<DIM>) -> Sign,
    ) -> Multinomial<DIM> {
        Blade::iter()
            .cartesian_product(Blade::iter())
            .filter_map(|(blade_0, blade_1)| {
                let blade_output = blade_0 ^ blade_1;
                grade_filter(blade_output.grade(), blade_0.grade(), blade_1.grade()).then(|| {
                    (
                        blade_output,
                        [
                            (Binding { binding: 0 }, blade_0),
                            (Binding { binding: 1 }, blade_1),
                        ]
                        .to_vec(),
                        Coefficient::from(
                            parity(blade_0, blade_1)
                                ^ self[blade_0].intrinsic_sign
                                ^ self[blade_1].intrinsic_sign
                                ^ self[blade_output].intrinsic_sign,
                        ) * (blade_0 & blade_1)
                            .iter_generators()
                            .map(|generator| self.generator_squares[generator.generator])
                            .product::<Coefficient>(),
                    )
                })
            })
            .collect()
    }

    fn implementations_from_multinomial(multinomial: Multinomial<DIM>) -> Vec<Implementation<DIM>> {
        Space::iter()
            .cartesian_product(Space::iter())
            .map(|(space_0, space_1)| {
                let (return_type, return_expr) = multinomial
                    .space_instantiate(&[space_0, space_1])
                    .to_type_expr();
                Implementation {
                    generic_types: [Type::Space(space_0), Type::Space(space_1)].into(),
                    self_type: Type::Space(space_0),
                    arg_types: [Type::Space(space_1)].into(),
                    return_type: Some(return_type),
                    statements: [].into(),
                    return_expr: Some(return_expr),
                }
            })
            .collect()
    }

    // fn product(
    //     &self,
    //     grade_filter: fn(Grade<DIM>, Grade<DIM>, Grade<DIM>) -> bool,
    //     parity: fn(Blade<DIM>, Blade<DIM>) -> Sign,
    // ) -> Vec<Implementation<DIM>> {
    //     Space::iter()
    //         .cartesian_product(Space::iter())
    //         .map(|(space_0, space_1)| {
    //             let terms = space_0
    //                 .iter_blades()
    //                 .cartesian_product(space_1.iter_blades())
    //                 .filter_map(|(blade_0, blade_1)| {
    //                     let blade_output = blade_0 ^ blade_1;
    //                     grade_filter(blade_0.grade(), blade_1.grade(), blade_output.grade()).then(
    //                         || {
    //                             (
    //                                 blade_output,
    //                                 std::iter::once((
    //                                     (blade_0, blade_1),
    //                                     Coefficient::from(
    //                                         parity(blade_0, blade_1)
    //                                             ^ self[blade_0].intrinsic_sign
    //                                             ^ self[blade_1].intrinsic_sign
    //                                             ^ self[blade_output].intrinsic_sign,
    //                                     ) * (blade_0 & blade_1)
    //                                         .iter_generators()
    //                                         .map(|generator| {
    //                                             self.generator_squares[generator.generator]
    //                                         })
    //                                         .product::<Coefficient>(),
    //                                 ))
    //                                 .collect::<Terms<_, _>>(),
    //                             )
    //                         },
    //                     )
    //                 })
    //                 .collect::<Terms<_, _>>();
    //             let space_output = Space::from_blades(terms.keys().cloned());
    //             Implementation {
    //                 generic_types: [Type::Space(space_0), Type::Space(space_1)].into(),
    //                 self_var: Var {
    //                     name: "self",
    //                     ty: Type::Space(space_0),
    //                 },
    //                 arg_vars: [Var {
    //                     name: "other",
    //                     ty: Type::Space(space_1),
    //                 }]
    //                 .into(),
    //                 return_type: Some(Type::Space(space_output)),
    //                 statements: [].into(),
    //                 return_expr: Some(Expr::structure(
    //                     space_output,
    //                     space_output.iter_blades().map(|blade| {
    //                         (blade, {
    //                             match terms.get(&blade) {
    //                                 None => Expr::literal(0),
    //                                 Some(terms) => {
    //                                     terms.to_polynomial_expr(|(blade_0, blade_1)| {
    //                                         (Expr::var("self").field(blade_0)
    //                                             * Expr::var("other").field(blade_1))
    //                                         .group()
    //                                     })
    //                                 }
    //                             }
    //                         })
    //                     }),
    //                 )),
    //             }
    //         })
    //         .collect()
    // }

    fn implementations(&self, operation: Operation) -> Vec<Implementation<DIM>> {
        match operation {
            Operation::Zero => todo!(),
            Operation::One => todo!(),
            Operation::SelectGrade0 => todo!(),
            Operation::SelectGrade1 => todo!(),
            Operation::SelectGrade2 => todo!(),
            Operation::SelectGrade3 => todo!(),
            Operation::SelectGrade4 => todo!(),
            Operation::SelectGrade5 => todo!(),
            Operation::SelectGrade6 => todo!(),
            Operation::SelectScalar => todo!(),
            Operation::SelectEven => todo!(),
            Operation::SelectOdd => todo!(),
            Operation::Inject => todo!(),
            Operation::Surject => todo!(),
            Operation::SInject => todo!(),
            Operation::SurjectS => todo!(),
            Operation::Involution => self.involution(|grade| Sign::from(grade.grade)),
            Operation::Reverse => self.involution(|grade| Sign::from(grade.grade >> 1)),
            Operation::Conjugate => self.involution(|grade| Sign::from((grade.grade + 1) >> 1)),
            Operation::Dual => self.dual(|blade| Blade::parity(blade, !blade)),
            Operation::InverseDual => self.dual(|blade| Blade::parity(!blade, blade)),
            Operation::Inverse => todo!(),
            Operation::NormSquared => todo!(),
            Operation::Norm => todo!(),
            Operation::Normalized => todo!(),
            Operation::Normalize => todo!(),
            Operation::GeometricProduct => Self::implementations_from_multinomial(
                self.product(|_grade_output, _grade_0, _grade_1| true, Blade::parity),
            ),
            Operation::ScalarProduct => Self::implementations_from_multinomial(self.product(
                |grade_output, _grade_0, _grade_1| grade_output == Grade { grade: 0 },
                Blade::parity,
            )),
            Operation::LeftInnerProduct => Self::implementations_from_multinomial(self.product(
                |grade_output, grade_0, grade_1| grade_output + grade_0 == grade_1,
                Blade::parity,
            )),
            Operation::RightInnerProduct => Self::implementations_from_multinomial(self.product(
                |grade_output, grade_0, grade_1| grade_output + grade_1 == grade_0,
                Blade::parity,
            )),
            Operation::InnerProduct => Self::implementations_from_multinomial(self.product(
                |grade_output, grade_0, grade_1| {
                    grade_output + grade_0 == grade_1 || grade_output + grade_1 == grade_0
                },
                Blade::parity,
            )),
            Operation::OuterProduct => Self::implementations_from_multinomial(self.product(
                |grade_output, grade_0, grade_1| grade_output == grade_0 + grade_1,
                Blade::parity,
            )),
            Operation::RegressiveProduct => Self::implementations_from_multinomial(self.product(
                |grade_output, grade_0, grade_1| {
                    grade_output.dual() == grade_0.dual() + grade_1.dual()
                },
                |blade_0, blade_1| Blade::parity(!blade_1, !blade_0),
            )),
            Operation::Commutator => todo!(),
            Operation::Anticommutator => todo!(),
            Operation::Transform => todo!(),
            Operation::Project => todo!(),
            Operation::Reject => todo!(),
            Operation::Add => todo!(),
            Operation::Sub => todo!(),
            Operation::Mul => todo!(),
            Operation::AddAssign => todo!(),
            Operation::SubAssign => todo!(),
            Operation::MulAssign => todo!(),
            Operation::MulS => todo!(),
            Operation::DivS => todo!(),
            Operation::MulSAssign => todo!(),
            Operation::DivSAssign => todo!(),
            Operation::SMul => todo!(),
            Operation::SDiv => todo!(),
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
    self_type: Type<DIM>,
    arg_types: Vec<Type<DIM>>,
    return_type: Option<Type<DIM>>,
    statements: Vec<Stmt<DIM>>,
    return_expr: Option<Box<Expr<DIM>>>,
}

// struct AssignmentImpl<Generics, Args> {
//     trait_ident: Rc<str>,
//     generics: Generics,
//     arg_vars: Args,
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

// struct Var<const DIM: usize> {
//     name: &'static str,
//     ty: Type<DIM>,
// }

enum Expr<const DIM: usize> {
    Variable {
        binding: Binding,
    },
    Literal {
        value: u8,
    },
    Structure {
        space: Space<DIM>,
        fields: Vec<(Blade<DIM>, Box<Self>)>,
    },
    Group {
        expr: Box<Self>,
    },
    Field {
        expr: Box<Self>,
        blade: Blade<DIM>,
    },
    Call {
        expr: Box<Self>,
        operation: Operation,
        generic_types: Vec<Type<DIM>>,
        arg_exprs: Vec<Box<Self>>,
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
}

impl<const DIM: usize> Expr<DIM> {
    fn variable(binding: Binding) -> Box<Self> {
        Box::new(Expr::Variable { binding })
    }

    fn literal(value: u8) -> Box<Self> {
        Box::new(Expr::Literal { value })
    }

    fn structure(
        space: Space<DIM>,
        fields: impl Iterator<Item = (Blade<DIM>, Box<Self>)>,
    ) -> Box<Self> {
        Box::new(Expr::Structure {
            space,
            fields: fields.collect(),
        })
    }

    fn group(self: Box<Self>) -> Box<Self> {
        Box::new(Expr::Group { expr: self })
    }

    fn field(self: Box<Self>, blade: Blade<DIM>) -> Box<Self> {
        Box::new(Expr::Field { expr: self, blade })
    }

    fn call<const GENERICS: usize, const ARGS: usize>(
        self: Box<Self>,
        operation: Operation,
        generic_types: [Type<DIM>; GENERICS],
        arg_exprs: [Box<Self>; ARGS],
    ) -> Box<Self> {
        Box::new(Expr::Call {
            expr: self,
            operation,
            generic_types: generic_types.into(),
            arg_exprs: arg_exprs.into(),
        })
    }
}

impl<const DIM: usize> std::ops::Neg for Box<Expr<DIM>> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Box::new(Expr::Neg { expr: self })
    }
}

impl<const DIM: usize> std::ops::Add for Box<Expr<DIM>> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Box::new(Expr::Add { lhs: self, rhs })
    }
}

impl<const DIM: usize> std::ops::Sub for Box<Expr<DIM>> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Box::new(Expr::Sub { lhs: self, rhs })
    }
}

impl<const DIM: usize> std::ops::Mul for Box<Expr<DIM>> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Box::new(Expr::Mul { lhs: self, rhs })
    }
}

impl<const DIM: usize> std::ops::Div for Box<Expr<DIM>> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Box::new(Expr::Div { lhs: self, rhs })
    }
}

// type Expression = Box<ExpressionRepr>;

enum Stmt<const DIM: usize> {
    Expr {
        expr: Box<Expr<DIM>>,
    },
    AddAssign {
        expr: Box<Expr<DIM>>,
        blade_index: usize,
        rhs: Box<Expr<DIM>>,
    },
    SubAssign {
        expr: Box<Expr<DIM>>,
        blade_index: usize,
        rhs: Box<Expr<DIM>>,
    },
    MulAssign {
        expr: Box<Expr<DIM>>,
        blade_index: usize,
        rhs: Box<Expr<DIM>>,
    },
    DivAssign {
        expr: Box<Expr<DIM>>,
        blade_index: usize,
        rhs: Box<Expr<DIM>>,
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
