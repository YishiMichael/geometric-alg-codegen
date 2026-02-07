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

enum BuiltinOperation {
    Sqrt,
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

struct Multinomial<const PARAMETERS: usize, const DEGREE: usize, const DIM: usize>(
    std::collections::BTreeMap<
        Blade<DIM>,
        std::collections::BTreeMap<[(Binding, Blade<DIM>); DEGREE], Coefficient>,
    >,
);

impl<const PARAMETERS: usize, const DEGREE: usize, const DIM: usize>
    FromIterator<(Blade<DIM>, [(Binding, Blade<DIM>); DEGREE], Coefficient)>
    for Multinomial<PARAMETERS, DEGREE, DIM>
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Blade<DIM>, [(Binding, Blade<DIM>); DEGREE], Coefficient)>,
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

// impl<const DIM: usize> IntoIterator for Multinomial<DIM> {
//     type Item = (
//         Blade<DIM>,
//         std::collections::BTreeMap<[(Binding, Blade<DIM>); DEGREE], Coefficient>,
//     );
//     type IntoIter = std::collections::btree_map::IntoIter<
//         Blade<DIM>,
//         std::collections::BTreeMap<[(Binding, Blade<DIM>); DEGREE], Coefficient>,
//     >;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

impl<const PARAMETERS: usize, const DEGREE: usize, const DIM: usize>
    Multinomial<PARAMETERS, DEGREE, DIM>
{
    // fn iter(
    //     &self,
    // ) -> std::collections::btree_map::Iter<
    //     '_,
    //     Blade<DIM>,
    //     std::collections::BTreeMap<[(Binding, Blade<DIM>); DEGREE], Coefficient>,
    // > {
    //     self.0.iter()
    // }

    // fn substitute(self, binding_substitutes: &[Self]) -> Self {
    //     self.into_iter()
    //         .flat_map(|(blade, polynomial)| {
    //             polynomial
    //                 .into_iter()
    //                 .flat_map(move |(multi_index, coeff)| {
    //                     multi_index
    //                         .into_iter()
    //                         .map(|(binding, binding_blade)| {
    //                             binding_substitutes[binding.binding].0[&binding_blade].iter()
    //                         })
    //                         .multi_cartesian_product()
    //                         .map(move |monomials| {
    //                             let (multi_index, coeff) = monomials.into_iter().fold(
    //                                 (Vec::new(), coeff),
    //                                 |(mut multi_index_acc, mut coeff_acc), (multi_index, coeff)| {
    //                                     multi_index_acc.extend(multi_index);
    //                                     coeff_acc *= coeff;
    //                                     (multi_index_acc, coeff_acc)
    //                                 },
    //                             );
    //                             (blade, multi_index, coeff)
    //                         })
    //                 })
    //         })
    //         .collect()
    // }

    fn dual(self) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|(blade, polynomial)| (!blade, polynomial))
                .collect(),
        )
    }

    fn filtrate(&self, spaces: [Space<DIM>; PARAMETERS]) -> Self {
        self.0
            .iter()
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

    fn space(&self) -> Space<DIM> {
        Space::from_blades(self.0.keys().cloned())
    }

    fn field_expr(&self, blade: Blade<DIM>) -> Box<Expr<DIM>> {
        match self.0.get(&blade) {
            None => Expr::literal(0),
            Some(polynomial) => polynomial
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
        }
    }

    fn struct_expr(&self, space: Space<DIM>) -> Box<Expr<DIM>> {
        Expr::structure(
            space,
            space
                .iter_blades()
                .map(|blade| (blade, self.field_expr(blade))),
        )
    }

    // fn implementations_custom(
    //     &self,
    //     expr_fn: Option<fn(Self) -> (Type<DIM>, Box<Expr<DIM>>)>,
    //     stmt_fn: Option<fn([Type<DIM>; PARAMETERS]) -> Vec<Stmt<DIM>>>,
    // ) -> Vec<Implementation<DIM>> {
    //     let expr_fn = expr_fn.unwrap_or(|multinomial| {
    //         let space = multinomial.space();
    //         (Type::Space(space), multinomial.struct_expr(space))
    //     });
    //     let stmt_fn = stmt_fn.unwrap_or(|_| [].into());
    //     std::iter::repeat_n(Space::iter(), PARAMETERS)
    //         .multi_cartesian_product()
    //         .map(|spaces| -> [Space<DIM>; PARAMETERS] { spaces.try_into().unwrap() })
    //         .map(move |spaces| {
    //             let (return_type, return_expr) = expr_fn(self.filtrate(spaces));
    //             let space_types = spaces.map(Type::Space);
    //             let stmts = stmt_fn(space_types);
    //             Implementation {
    //                 type_params: space_types.into(),
    //                 param_types: space_types.into(),
    //                 return_type: Some(return_type),
    //                 stmts,
    //                 return_expr: Some(return_expr),
    //             }
    //         })
    //         .collect()
    // }

    // fn implementations(&self) -> Vec<Implementation<DIM>> {
    //     self.implementations_custom(None, None)
    // }

    // fn blade_implementation(&self, blade: Blade) -> Implementation<DIM> {
    //     Implementation {
    //                 type_params: [Type::Space(space_0), Type::Space(space_1)].into(),
    //                 self_type: Type::Space(space_0),
    //                 arg_types: [Type::Space(space_1)].into(),
    //                 return_type: Some(return_type),
    //                 stmts: [].into(),
    //                 return_expr: Some(return_expr),
    //             }
    // }

    // fn polynomial_to_type_expr(
    //     polynomial: &std::collections::BTreeMap<[(Binding, Blade<DIM>); DEGREE], Coefficient>,
    // ) -> (Type<DIM>, Box<Expr<DIM>>) {
    //     (
    //         Type::Atom,
    //         polynomial
    //             .into_iter()
    //             .fold(None, |expr_acc, (multi_index, &coeff)| {
    //                 let exprs = multi_index.into_iter().map(|&(binding, binding_blade)| {
    //                     Expr::variable(binding).field(binding_blade)
    //                 });
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
    //             .unwrap_or_else(|| Expr::literal(0)),
    //     )
    // }

    // fn to_type_expr(&self) -> (Type<DIM>, Box<Expr<DIM>>) {
    //     let space = Space::from_blades(self.0.keys().cloned());
    //     (
    //         Type::Space(space),
    //         Expr::structure(
    //             space,
    //             space.iter_blades().map(|blade| {
    //                 (blade, {
    //                     match self.0.get(&blade) {
    //                         None => Expr::literal(0),
    //                         Some(polynomial) => {
    //                             let (_, polynomial_expr) =
    //                                 Self::polynomial_to_type_expr(polynomial);
    //                             polynomial_expr
    //                         }
    //                     }
    //                 })
    //             }),
    //         ),
    //     )
    // }
}

struct MultinomialImplementor<const PARAMETERS: usize, const DEGREE: usize, const DIM: usize> {
    multinomial: Multinomial<PARAMETERS, DEGREE, DIM>,
    expr_fn: fn(Multinomial<PARAMETERS, DEGREE, DIM>) -> (Type<DIM>, Box<Expr<DIM>>),
    stmt_fn: fn([Type<DIM>; PARAMETERS]) -> Vec<Stmt<DIM>>,
}

impl<const PARAMETERS: usize, const DEGREE: usize, const DIM: usize>
    MultinomialImplementor<PARAMETERS, DEGREE, DIM>
{
    fn new(multinomial: Multinomial<PARAMETERS, DEGREE, DIM>) -> Self {
        Self {
            multinomial,
            expr_fn: |multinomial| {
                let space = multinomial.space();
                (Type::Space(space), multinomial.struct_expr(space))
            },
            stmt_fn: |_| [].into(),
        }
    }

    fn dual(mut self) -> Self {
        self.multinomial = self.multinomial.dual();
        self
    }

    fn expr_fn(
        mut self,
        expr_fn: fn(Multinomial<PARAMETERS, DEGREE, DIM>) -> (Type<DIM>, Box<Expr<DIM>>),
    ) -> Self {
        self.expr_fn = expr_fn;
        self
    }

    fn stmt_fn(mut self, stmt_fn: fn([Type<DIM>; PARAMETERS]) -> Vec<Stmt<DIM>>) -> Self {
        self.stmt_fn = stmt_fn;
        self
    }

    fn implementations(self) -> Vec<Implementation<DIM>> {
        std::iter::repeat_n(Space::iter(), PARAMETERS)
            .multi_cartesian_product()
            .map(|spaces| -> [Space<DIM>; PARAMETERS] { spaces.try_into().unwrap() })
            .map(move |spaces| {
                let space_types = spaces.map(Type::Space);
                let (return_type, return_expr) = (self.expr_fn)(self.multinomial.filtrate(spaces));
                let stmts = (self.stmt_fn)(space_types);
                Implementation {
                    type_params: space_types.into(),
                    param_types: space_types.into(),
                    return_type: Some(return_type),
                    stmts,
                    return_expr: Some(return_expr),
                }
            })
            .collect()
    }
}

// struct ProductOperation<
//     'a,
//     const TYPE_PARAMETERS: usize,
//     const PARAMETERS: usize,
//     const DEGREE: usize,
//     const DIM: usize,
// > {
//     // binding_spaces: [(Binding, Space<DIM>); PARAMETERS],
//     geometric_algebra: &'a GeometricAlgebra<DIM>,
//     prototype: [Binding; DEGREE],
//     instantiate: fn(
//         &Multinomial<PARAMETERS, DEGREE, DIM>,
//         [Space<DIM>; PARAMETERS],
//     ) -> Vec<([Space<DIM>; TYPE_PARAMETERS], Type<DIM>, Box<Expr<DIM>>)>,
//     term_sign: fn([Blade<DIM>; DEGREE]) -> Option<Sign>,
//     blade_reindex: fn([Blade<DIM>; DEGREE]) -> [Blade<DIM>; DEGREE],
//     shadow_parameters: fn([Type<DIM>; TYPE_PARAMETERS]) -> Vec<Stmt<DIM>>,
// }

// impl<
//         'a,
//         const TYPE_PARAMETERS: usize,
//         const PARAMETERS: usize,
//         const DEGREE: usize,
//         const DIM: usize,
//     > ProductOperation<'a, TYPE_PARAMETERS, PARAMETERS, DEGREE, DIM>
// {
//     fn new(geometric_algebra: &'a GeometricAlgebra<DIM>, prototype: [usize; DEGREE]) -> Self {
//         Self {
//             geometric_algebra,
//             prototype: prototype.map(|binding| Binding { binding }).into(),
//             instantiate: |_, _| [].into(),
//             shadow_parameters: |_| [].into(),
//             term_sign: |_| Some(Sign::POS),
//             blade_reindex: std::convert::identity,
//         }
//     }

//     fn instantiate(
//         mut self,
//         instantiate: fn(
//             &Multinomial<PARAMETERS, DEGREE, DIM>,
//             [Space<DIM>; PARAMETERS],
//         ) -> Vec<([Space<DIM>; TYPE_PARAMETERS], Type<DIM>, Box<Expr<DIM>>)>,
//     ) -> Self {
//         self.instantiate = instantiate;
//         self
//     }

//     fn term_sign(mut self, term_sign: fn([Blade<DIM>; DEGREE]) -> Option<Sign>) -> Self {
//         self.term_sign = term_sign;
//         self
//     }

//     fn blade_reindex(
//         mut self,
//         blade_reindex: fn([Blade<DIM>; DEGREE]) -> [Blade<DIM>; DEGREE],
//     ) -> Self {
//         self.blade_reindex = blade_reindex;
//         self
//     }

//     fn shadow_parameters(
//         mut self,
//         shadow_parameters: fn([Type<DIM>; TYPE_PARAMETERS]) -> Vec<Stmt<DIM>>,
//     ) -> Self {
//         self.shadow_parameters = shadow_parameters;
//         self
//     }

// fn type_expr(
//     &mut self,
//     type_expr: fn(&Multinomial<PARAMETERS, DEGREE, DIM>) -> (Type<DIM>, Box<Expr<DIM>>),
// ) -> &mut Self {
//     self.type_expr = type_expr;
//     self
// }

//     fn implementations(&self) -> Vec<Implementation<DIM>> {
//         std::iter::repeat_n(Space::iter(), PARAMETERS)
//             .multi_cartesian_product()
//             .map(|spaces| -> [Space<DIM>; PARAMETERS] { spaces.try_into().unwrap() })
//             .flat_map(|spaces| {
//                 // let spaces = (self.param_spaces)(&type_spaces);
//                 let multinomial = self
//                     .prototype
//                     .iter()
//                     .map(|binding| spaces[binding.binding].iter_blades())
//                     .multi_cartesian_product()
//                     .map(|blades| -> [Blade<DIM>; DEGREE] { blades.try_into().unwrap() })
//                     .map(self.blade_reindex)
//                     .filter_map(|blades| {
//                         (self.term_sign)(blades).map(|sign| {
//                             let multi_index =
//                                 std::array::from_fn(|index| (self.prototype[index], blades[index]));
//                             let blade = blades
//                                 .into_iter()
//                                 .fold(Blade::zero(), std::ops::BitXor::bitxor);
//                             let coeff = Coefficient::from(
//                                 sign ^ blades
//                                     .into_iter()
//                                     .map(|blade| self.geometric_algebra[blade].intrinsic_sign)
//                                     .fold(
//                                         self.geometric_algebra[blade].intrinsic_sign,
//                                         std::ops::BitXor::bitxor,
//                                     ),
//                             ) * blades
//                                 .into_iter()
//                                 .fold(!Blade::zero(), std::ops::BitAnd::bitand)
//                                 .iter_generators()
//                                 .map(|generator| {
//                                     self.geometric_algebra.generator_squares[generator.generator]
//                                 })
//                                 .product::<Coefficient>();
//                             (blade, multi_index, coeff)
//                         })
//                     })
//                     .collect();
//                 (self.instantiate)(&multinomial, spaces).into_iter().map(
//                     move |(type_spaces, return_type, return_expr)| {
//                         let type_params = type_spaces.map(Type::Space);
//                         let param_types = spaces.map(Type::Space);
//                         Implementation {
//                             type_params: type_params.into(),
//                             param_types: param_types.into(),
//                             return_type: Some(return_type),
//                             stmts: (self.shadow_parameters)(type_params),
//                             return_expr: Some(return_expr),
//                         }
//                     },
//                 )
//             })
//             .collect()
//     }
// }

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
    fn multinomial_implementor<const PARAMETERS: usize, const DEGREE: usize>(
        &self,
        prototype: [usize; DEGREE],
        term_sign: fn([Blade<DIM>; DEGREE]) -> Option<Sign>,
        // blade_reindex: fn([Blade<DIM>; DEGREE]) -> [Blade<DIM>; DEGREE],
    ) -> MultinomialImplementor<PARAMETERS, DEGREE, DIM> {
        MultinomialImplementor::new(
            std::iter::repeat_n(Blade::iter(), DEGREE)
                .multi_cartesian_product()
                .map(|blades| -> [Blade<DIM>; DEGREE] { blades.try_into().unwrap() })
                // .map(blade_reindex)
                .filter_map(|blades| {
                    term_sign(blades).map(|sign| {
                        let multi_index = std::array::from_fn(|index| {
                            (
                                Binding {
                                    binding: prototype[index],
                                },
                                blades[index],
                            )
                        });
                        let blade = blades
                            .into_iter()
                            .fold(Blade::zero(), std::ops::BitXor::bitxor);
                        let coeff = Coefficient::from(
                            sign ^ blades
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
                })
                .collect(),
        )
    }

    // fn unary_product<const DEGREE: usize>(
    //     &self,
    //     prototype: [usize; DEGREE],
    // ) -> ProductOperation<1, 1, DEGREE, DIM> {
    //     ProductOperation::new(self, prototype).instantiate(|multinomial, [space_0]| {
    //         let space = multinomial.space();
    //         [(
    //             [space_0],
    //             Type::Space(space),
    //             multinomial.struct_expr(space),
    //         )]
    //         .into()
    //     })
    // }

    // fn binary_product<const DEGREE: usize>(
    //     &self,
    //     prototype: [usize; DEGREE],
    // ) -> ProductOperation<2, 2, DEGREE, DIM> {
    //     ProductOperation::new(self, prototype).instantiate(|multinomial, [space_0, space_1]| {
    //         let space = multinomial.space();
    //         [(
    //             [space_0, space_1],
    //             Type::Space(space),
    //             multinomial.struct_expr(space),
    //         )]
    //         .into()
    //     })
    // }

    // fn unary_product_scalar<const DEGREE: usize>(
    //     &self,
    //     prototype: [usize; DEGREE],
    // ) -> ProductOperation<1, 1, DEGREE, DIM> {
    //     ProductOperation::new(self, prototype).instantiate(|multinomial, [space_0]| {
    //         [([space_0], Type::Atom, multinomial.field_expr(Blade::zero()))].into()
    //     })
    // }

    // fn unary_product_generic<const DEGREE: usize>(
    //     &self,
    //     prototype: [usize; DEGREE],
    // ) -> ProductOperation<2, 1, DEGREE, DIM> {
    //     ProductOperation::new(self, prototype).instantiate(|multinomial, [space_0]| {
    //         Space::iter()
    //             .map(|space| {
    //                 (
    //                     [space_0, space],
    //                     Type::Space(space),
    //                     multinomial.struct_expr(space),
    //                 )
    //             })
    //             .collect()
    //     })
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
            Operation::Involution => self
                .multinomial_implementor::<1, 1>([0], |[blade]| {
                    Some(Sign::from(blade.grade().grade))
                })
                .implementations(),
            Operation::Reverse => self
                .multinomial_implementor::<1, 1>([0], |[blade]| {
                    Some(Sign::from(blade.grade().grade >> 1))
                })
                .implementations(),
            Operation::Conjugate => self
                .multinomial_implementor::<1, 1>([0], |[blade]| {
                    Some(Sign::from((blade.grade().grade + 1) >> 1))
                })
                .implementations(),
            Operation::Dual => self
                .multinomial_implementor::<1, 1>([0], |[blade]| Some((!blade).parity(blade)))
                .dual()
                .implementations(),
            Operation::InverseDual => self
                .multinomial_implementor::<1, 1>([0], |[blade]| Some(blade.parity(!blade)))
                .dual()
                .implementations(),
            Operation::Inverse => todo!(),
            Operation::NormSquared => self
                .multinomial_implementor::<1, 2>([0, 0], |_| Some(Sign::POS))
                .expr_fn(|multinomial| (Type::Atom, multinomial.field_expr(Blade::zero())))
                .implementations(),
            Operation::Norm => todo!(),
            Operation::Normalized => todo!(),
            Operation::Normalize => todo!(),
            Operation::GeometricProduct => self
                .multinomial_implementor::<2, 2>([0, 1], |[blade_0, blade_1]| {
                    Some(blade_0.parity(blade_1))
                })
                .implementations(),
            Operation::ScalarProduct => self
                .multinomial_implementor::<2, 2>([0, 1], |[blade_0, blade_1]| {
                    (blade_0 ^ blade_1 == Blade::zero()).then(|| blade_0.parity(blade_1))
                })
                .implementations(),
            Operation::LeftInnerProduct => self
                .multinomial_implementor::<2, 2>([0, 1], |[blade_0, blade_1]| {
                    (blade_0 & blade_1 == blade_0).then(|| blade_0.parity(blade_1))
                })
                .implementations(),
            Operation::RightInnerProduct => self
                .multinomial_implementor::<2, 2>([0, 1], |[blade_0, blade_1]| {
                    (blade_0 & blade_1 == blade_1).then(|| blade_0.parity(blade_1))
                })
                .implementations(),
            Operation::InnerProduct => self
                .multinomial_implementor::<2, 2>([0, 1], |[blade_0, blade_1]| {
                    (blade_0 & blade_1 == blade_0 || blade_0 & blade_1 == blade_1)
                        .then(|| blade_0.parity(blade_1))
                })
                .implementations(),
            Operation::OuterProduct => self
                .multinomial_implementor::<2, 2>([0, 1], |[blade_0, blade_1]| {
                    (blade_0 & blade_1 == Blade::zero()).then(|| blade_0.parity(blade_1))
                })
                .implementations(),
            Operation::RegressiveProduct => self
                .multinomial_implementor::<2, 2>([0, 1], |[blade_0, blade_1]| {
                    (!blade_0 & !blade_1 == Blade::zero()).then(|| (!blade_1).parity(!blade_0))
                })
                .implementations(),
            Operation::Commutator => self
                .multinomial_implementor::<2, 2>([0, 1], |[blade_0, blade_1]| {
                    (blade_0.grade().grade & blade_1.grade().grade & 1 != 0)
                        .then(|| blade_0.parity(blade_1))
                })
                .implementations(),
            Operation::Anticommutator => self
                .multinomial_implementor::<2, 2>([0, 1], |[blade_0, blade_1]| {
                    (blade_0.grade().grade & blade_1.grade().grade & 1 == 0)
                        .then(|| blade_0.parity(blade_1))
                })
                .implementations(),
            Operation::Transform => self
                .multinomial_implementor::<2, 3>([0, 1, 0], |[blade_0, blade_1, blade_2]| {
                    Some(
                        blade_0.parity(blade_1)
                            ^ blade_0.parity(blade_2)
                            ^ blade_1.parity(blade_2)
                            ^ Sign::from(blade_2.grade().grade >> 1),
                    )
                })
                .implementations(),
            Operation::Project => self
                .multinomial_implementor::<2, 3>([0, 1, 1], |[blade_0, blade_1, blade_2]| {
                    (blade_0 & blade_1 == blade_0).then(|| {
                        blade_0.parity(blade_1)
                            ^ blade_0.parity(blade_2)
                            ^ blade_1.parity(blade_2)
                            ^ Sign::from(blade_2.grade().grade >> 1)
                    })
                })
                .stmt_fn(|[_, type_1]| {
                    [Stmt::ShadowBind {
                        binding: Binding { binding: 1 },
                        expr: Expr::variable(Binding { binding: 1 }).call(
                            Operation::Normalized,
                            [type_1],
                            [],
                        ),
                    }]
                    .into()
                })
                .implementations(),
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
    type_params: Vec<Type<DIM>>,
    param_types: Vec<Type<DIM>>,
    return_type: Option<Type<DIM>>,
    stmts: Vec<Stmt<DIM>>,
    return_expr: Option<Box<Expr<DIM>>>,
}

// struct AssignmentImpl<Generics, Args> {
//     trait_ident: Rc<str>,
//     generics: Generics,
//     arg_vars: Args,
//     stmts: Vec<Statement>,
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
//     fn stmts(&self, generics: &Generics, vars: &Vars) -> Vec<Statement>;
//     fn expression(&self, generics: &Generics, vars: &Vars) -> Expression;
// }

// trait InplaceOperation<Generics, Vars> {
//     fn stmts(&self, generics: &Generics, vars: &Vars) -> Vec<Statement>;
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
        operation: Operation,
        type_params: Vec<Type<DIM>>,
        param_exprs: Vec<Box<Self>>,
    },
    CallBuiltin {
        operation: BuiltinOperation,
        param_exprs: Vec<Box<Self>>,
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

    fn call<const TYPE_PARAMETERS: usize, const PARAMETERS: usize>(
        self: Box<Self>,
        operation: Operation,
        type_params: [Type<DIM>; TYPE_PARAMETERS],
        param_exprs: [Box<Self>; PARAMETERS],
    ) -> Box<Self> {
        Box::new(Expr::Call {
            operation,
            type_params: type_params.into(),
            param_exprs: std::iter::once(self).chain(param_exprs).collect(),
        })
    }

    fn call_builtin<const PARAMETERS: usize>(
        self: Box<Self>,
        operation: BuiltinOperation,
        param_exprs: [Box<Self>; PARAMETERS],
    ) -> Box<Self> {
        Box::new(Expr::CallBuiltin {
            operation,
            param_exprs: std::iter::once(self).chain(param_exprs).collect(),
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
    // Expr {
    //     expr: Box<Expr<DIM>>,
    // },
    ShadowBind {
        binding: Binding,
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
