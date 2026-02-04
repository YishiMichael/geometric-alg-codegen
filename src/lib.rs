/// Count of negative signs.
type Sign = u32;

// type SpaceIndex = usize;

#[repr(transparent)]
#[derive(Clone, Copy)]
struct BladeIndex {
    generator_bits: usize,
}

struct Blade {
    name: String,
    gray_inv: usize,
    intrinsic_sign: Sign,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
struct SpaceIndex {
    grade_bits: usize,
}

struct Space {
    name: String,
}

// trait GrayCodeInv<const DIM: u32> {
//     fn gray_code_inv(self) -> Self;
// }

// impl GrayCodeInv<0> for BladeIndex {
//     fn gray_code_inv(self) -> Self {
//         0
//     }
// }

// impl<const DIM: u32> GrayCodeInv<DIM> for BladeIndex
// where
//     BladeIndex: GrayCodeInv<{ DIM - 1 }>,
// {
//     fn gray_code_inv(self) -> Self {
//         self >> DIM ^ <BladeIndex as GrayCodeInv<{ DIM - 1 }>>::gray_code_inv(self)
//     }
// }

#[derive(Clone, Copy)]
struct GeometricAlgebraRenames {
    blades: &'static [(usize, &'static str)],
    spaces: &'static [(usize, &'static str)],
}

struct GeometricAlgebraBuilder {
    dim: usize,
    generator_squares: Vec<i32>,
    blade_names: Vec<String>,
    space_names: Vec<String>,
}

fn generator_squares<V>(generator_squares: V) -> GeometricAlgebraBuilder
where
    V: Into<Vec<i32>>,
{
    let generator_squares = generator_squares.into();
    let dim = generator_squares.len();
    GeometricAlgebraBuilder {
        dim,
        generator_squares: generator_squares,
        blade_names: (0..1 << dim)
            .map(|blade_index| format!("e{blade_index}"))
            .collect(),
        space_names: (0..1 << (dim + 1))
            .map(|space_index| format!("Space{space_index}"))
            .collect(),
    }
}

impl GeometricAlgebraBuilder {
    fn rename_blade<V>(mut self, index: usize, name: V) -> Self
    where
        V: Into<String>,
    {
        self.blade_names[index] = name.into();
        self
    }

    fn rename_space<V>(mut self, index: usize, name: V) -> Self
    where
        V: Into<String>,
    {
        self.space_names[index] = name.into();
        self
    }

    fn rename(mut self, renames: GeometricAlgebraRenames) -> Self {
        renames
            .blades
            .into_iter()
            .for_each(|&(index, name)| self.blade_names[index] = name.into());
        renames
            .spaces
            .into_iter()
            .for_each(|&(index, name)| self.space_names[index] = name.into());
        self
    }

    fn build(self) -> GeometricAlgebra {
        let (_, zero_bits, neg_bits) = self.generator_squares.into_iter().rev().fold(
            (1usize, 0usize, 0usize),
            |(mask, mut zero_bits, mut neg_bits), generator_square| {
                match generator_square.signum() {
                    1 => (),
                    0 => zero_bits |= mask,
                    -1 => neg_bits |= mask,
                    _ => unreachable!(),
                };
                (mask << 1, zero_bits, neg_bits)
            },
        );
        let blades: Vec<_> = self
            .blade_names
            .into_iter()
            .enumerate()
            .map(|(generator_bits, blade_name)| {
                let (name, intrinsic_sign) = match blade_name.strip_prefix('-') {
                    None => (blade_name, 0),
                    Some(blade_name) => (blade_name.to_string(), 1),
                };
                Blade {
                    name,
                    gray_inv: (0..self.dim)
                        .map(|grade| generator_bits >> grade)
                        .fold(0, std::ops::BitXor::bitxor),
                    intrinsic_sign,
                }
            })
            .collect();
        let spaces = self
            .space_names
            .into_iter()
            .map(|name| Space { name })
            .collect();
        GeometricAlgebra {
            zero_bits,
            neg_bits,
            blades,
            spaces,
        }
    }
}

struct GeometricAlgebra {
    zero_bits: usize,
    neg_bits: usize,
    blades: Vec<Blade>,
    spaces: Vec<Space>,
}

impl std::ops::Index<BladeIndex> for GeometricAlgebra {
    type Output = Blade;

    fn index(&self, index: BladeIndex) -> &Self::Output {
        self.blades.index(index.generator_bits)
    }
}

impl std::ops::Index<SpaceIndex> for GeometricAlgebra {
    type Output = Space;

    fn index(&self, index: SpaceIndex) -> &Self::Output {
        self.spaces.index(index.grade_bits)
    }
}

impl GeometricAlgebra {
    // fn grade(index: BladeIndex) -> u32 {
    //     index.count_ones()
    // }

    // fn gray_code_inv(index: BladeIndex, DIM: u32) -> usize {
    //     (0..DIM)
    //         .map(|grade| index >> grade)
    //         .fold(0, std::ops::BitXor::bitxor)
    // }

    fn blade_mul(&self, index_0: BladeIndex, index_1: BladeIndex) -> Option<(BladeIndex, Sign)> {
        // blades[index_0] * blades[index_1] = blades[index] * sign
        (index_0.generator_bits & index_1.generator_bits & self.zero_bits == 0).then(|| {
            let index = BladeIndex {
                generator_bits: index_0.generator_bits ^ index_1.generator_bits,
            };
            (
                index,
                (index_0.generator_bits & index_1.generator_bits & self.neg_bits).count_ones()
                    + (self[index_0].gray_inv >> 1 & index_1.generator_bits).count_ones()
                    + self[index_0].intrinsic_sign
                    + self[index_1].intrinsic_sign
                    + self[index].intrinsic_sign,
            )
        })
    }
}

// trait Operation<Generics, Inputs> {
//     fn call()
// }

struct BinaryExpr {}

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
