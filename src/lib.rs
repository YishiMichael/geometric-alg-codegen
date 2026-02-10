use itertools::Itertools;
use symbol::Symbol;

trait EvalType<Type> {
    fn eval_type(&self, generic_types: &[Type], associate_types: &[Type]) -> Type;
}

struct Context<Pretype, Type> {
    structs: Vec<Struct<Type>>,
    traits: Vec<Trait<Pretype, Type>>,
}

struct Struct<Type> {
    ident: Symbol,
    r#type: Type,
    fields: Vec<Field<Type>>,
}

struct Field<Type> {
    ident: Symbol,
    r#type: Type,
}

struct Trait<Pretype, Type> {
    ident: Symbol,
    call_ident: Symbol,
    generics: Vec<Symbol>,
    associates: Vec<Symbol>,
    params: Vec<Symbol>,
    param_pretypes: Vec<Pretype>,
    return_pretype: Pretype,
    implementations: Vec<Implementation<Type>>,
    implementor: Box<dyn Fn(&Context<Pretype, Type>, Vec<Expr<Type>>, Type) -> Option<Expr<Type>>>, // Lazily evaluated; None = builtin
    call_expr_repr: Box<dyn Fn(Symbol, Vec<Expr<Type>>, &Implementation<Type>) -> ExprRepr<Type>>,
}

struct Implementation<Type> {
    generic_types: Vec<Type>,
    associate_types: Vec<Type>,
    param_types: Vec<Type>,
    return_type: Type,
}

impl<Pretype, Type> Context<Pretype, Type>
where
    Pretype: EvalType<Type>,
{
    fn add_struct<const FIELDS: usize>(
        &mut self,
        ident: impl AsRef<str>,
        r#type: Type,
        field_items: [(impl AsRef<str>, Type); FIELDS],
    ) -> &mut Self {
        self.structs.push(Struct {
            ident: Symbol::from(ident),
            r#type,
            fields: Vec::from(field_items.map(|(field_ident, field_type)| Field {
                ident: Symbol::from(field_ident),
                r#type: field_type,
            })),
        });
        self
    }

    fn add_trait<
        const GENERICS: usize,
        const ASSOCIATES: usize,
        const PARAMS: usize,
        const IMPLEMENTATIONS: usize,
    >(
        &mut self,
        ident: impl AsRef<str>,
        call_ident: impl AsRef<str>,
        generics: [impl AsRef<str>; GENERICS],
        associates: [impl AsRef<str>; ASSOCIATES],
        param_items: [(impl AsRef<str>, Pretype); PARAMS],
        return_pretype: Pretype,
        implementations: [([Type; GENERICS], [Type; ASSOCIATES]); IMPLEMENTATIONS],
        implementor: impl Fn(&Context<Pretype, Type>, [Expr<Type>; PARAMS], Type) -> Option<Expr<Type>>
            + 'static,
        call_expr_repr: impl Fn(Symbol, [Expr<Type>; PARAMS], &Implementation<Type>) -> ExprRepr<Type>
            + 'static,
    ) -> &mut Self {
        let implementations = implementations.map(|(generic_types, associate_types)| {
            let param_types = param_items.each_ref().map(|(_, param_pretype)| {
                param_pretype.eval_type(&generic_types, &associate_types)
            });
            let return_type = return_pretype.eval_type(&generic_types, &associate_types);
            Implementation {
                generic_types: Vec::from(generic_types),
                associate_types: Vec::from(associate_types),
                param_types: Vec::from(param_types),
                return_type,
            }
        });
        self.traits.push(Trait {
            ident: Symbol::from(ident),
            call_ident: Symbol::from(call_ident),
            generics: Vec::from(generics.map(Symbol::from)),
            associates: Vec::from(associates.map(Symbol::from)),
            params: Vec::from(param_items.each_ref().map(|(ident, _)| Symbol::from(ident))),
            param_pretypes: Vec::from(param_items.map(|(_, param_pretype)| param_pretype)),
            return_pretype,
            implementations: Vec::from(implementations),
            implementor: Box::new(move |context, param_exprs, return_type| {
                implementor(context, param_exprs.try_into().ok().unwrap(), return_type)
            }),
            call_expr_repr: Box::new(move |call_ident, param_exprs, implementation| {
                call_expr_repr(
                    call_ident,
                    param_exprs.try_into().ok().unwrap(),
                    implementation,
                )
            }),
        });
        self
    }

    // fn build_expr(&mut self) {
    //     self.traits.iter_mut().for_each(|trait_info| {
    //         trait_info
    //             .implementations
    //             .iter_mut()
    //             .for_each(|implementation| {
    //                 implementation.expr = (trait_info.implementor)(implementation.generic_types)
    //             })
    //     })
    // }
}

struct VectorSpace<Basis> {
    bases: Vec<Basis>,
}

trait Subspace<Basis, Type> {
    fn get_type(&self) -> Type;
    fn get_coeff_expr(&self, basis: &Basis, expr: Expr<Type>) -> Option<Expr<Type>>;
}

enum ExprRepr<Type> {
    Variable {
        ident: Symbol,
    },
    Literal {
        value: u8,
    },
    Struct {
        ident: Symbol,
        fields: Vec<(Symbol, Expr<Type>)>,
    },
    Field {
        expr: Expr<Type>,
        ident: Symbol,
    },
    // CallBuiltin {
    //     ident: Symbol,
    //     param_exprs: Vec<Expr<Type>>,
    // },
    Call {
        generic_types: Vec<Type>,
        call_ident: Symbol,
        param_exprs: Vec<Expr<Type>>,
    },
    Neg {
        expr: Expr<Type>,
    },
    Add {
        lhs: Expr<Type>,
        rhs: Expr<Type>,
    },
    Sub {
        lhs: Expr<Type>,
        rhs: Expr<Type>,
    },
    Mul {
        lhs: Expr<Type>,
        rhs: Expr<Type>,
    },
    Div {
        lhs: Expr<Type>,
        rhs: Expr<Type>,
    },
}

struct Expr<Type> {
    repr: Box<ExprRepr<Type>>,
    r#type: Type,
}

trait TypeTrait: Clone + PartialEq {
    fn literal_type() -> Self;
    // fn add_ident() -> Symbol;
    // fn sub_ident() -> Symbol;
    // fn mul_ident() -> Symbol;
    // fn div_ident() -> Symbol;
}

impl<Pretype, Type> Context<Pretype, Type>
where
    Type: TypeTrait,
{
    fn variable(&self, ident: Symbol, r#type: Type) -> Expr<Type> {
        Expr {
            repr: Box::new(ExprRepr::Variable { ident }),
            r#type,
        }
    }

    fn literal(&self, value: u8) -> Expr<Type> {
        Expr {
            repr: Box::new(ExprRepr::Literal { value }),
            r#type: Type::literal_type(),
        }
    }

    fn r#struct(&self, ident: Symbol, field_expr_fn: impl Fn(Symbol) -> Expr<Type>) -> Expr<Type> {
        let r#struct = self
            .structs
            .iter()
            .filter(|r#struct| r#struct.ident == ident)
            .exactly_one()
            .ok()
            .unwrap();
        Expr {
            repr: Box::new(ExprRepr::Struct {
                ident,
                fields: r#struct
                    .fields
                    .iter()
                    .map(|field| {
                        let field_expr = field_expr_fn(field.ident);
                        (field.ident, field_expr)
                    })
                    .collect(),
            }),
            r#type: r#struct.r#type.clone(),
        }
    }

    fn field(&self, expr: Expr<Type>, ident: Symbol) -> Expr<Type> {
        let r#struct = self
            .structs
            .iter()
            .filter(|r#struct| r#struct.r#type == expr.r#type)
            .exactly_one()
            .ok()
            .unwrap();
        let field = r#struct
            .fields
            .iter()
            .filter(|field| field.ident == ident)
            .exactly_one()
            .ok()
            .unwrap();
        Expr {
            repr: Box::new(ExprRepr::Field { expr, ident }),
            r#type: field.r#type.clone(),
        }
    }

    fn call<const PARAMS: usize>(
        &self,
        call_ident: Symbol,
        param_exprs: [Expr<Type>; PARAMS],
    ) -> Expr<Type> {
        let r#trait = self
            .traits
            .iter()
            .filter(|r#trait| r#trait.call_ident == call_ident)
            .exactly_one()
            .ok()
            .unwrap();
        let implementation = r#trait
            .implementations
            .iter()
            .filter(|implementation| {
                implementation
                    .param_types
                    .iter()
                    .zip_eq(param_exprs.each_ref())
                    .all(|(param_type, param_expr)| param_type == &param_expr.r#type)
            })
            .exactly_one()
            .ok()
            .unwrap();
        Expr {
            repr: Box::new((r#trait.call_expr_repr)(
                call_ident,
                Vec::from(param_exprs),
                implementation,
            )),
            r#type: implementation.return_type.clone(),
        }
    }
}

// struct Context {
//     map: std::collections::BTreeMap<()>
// }

// impl<const PARAMS: usize, const DIM: usize> std::ops::Neg for Expr<PARAMS, DIM> {
//     type Output = Self;

//     fn neg(self) -> Self::Output {
//         ExprRepr::Neg { expr: self }.into()
//     }
// }

// impl<const PARAMS: usize, const DIM: usize> std::ops::Add for Expr<PARAMS, DIM> {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         ExprRepr::Add { lhs: self, rhs }.into()
//     }
// }

// impl<const PARAMS: usize, const DIM: usize> std::ops::Sub for Expr<PARAMS, DIM> {
//     type Output = Self;

//     fn sub(self, rhs: Self) -> Self::Output {
//         ExprRepr::Sub { lhs: self, rhs }.into()
//     }
// }

// impl<const PARAMS: usize, const DIM: usize> std::ops::Mul for Expr<PARAMS, DIM> {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Self::Output {
//         ExprRepr::Mul { lhs: self, rhs }.into()
//     }
// }

// impl<const PARAMS: usize, const DIM: usize> std::ops::Div for Expr<PARAMS, DIM> {
//     type Output = Self;

//     fn div(self, rhs: Self) -> Self::Output {
//         ExprRepr::Div { lhs: self, rhs }.into()
//     }
// }

//

type Blade = usize;

enum GAType {
    Primitive,
    Compound(Vec<Blade>),
}

enum GAPretype<const GENERICS: usize, const ASSOCIATES: usize> {
    Primitive,
    GenericBinding(usize),
    AssociateBinding(usize),
}

fn f() {
    let geometric_product = OperationSignature {
        name: "GeometricProduct",
        method_name: "geometric_product",
        generics: ["Self", "T"],
        associates: ["Output"],
        params: [
            ("self", GAPretype::GenericBinding(0)),
            ("other", GAPretype::GenericBinding(1)),
        ],
        return_pretype: GAPretype::AssociateBinding(0),
    };
}
