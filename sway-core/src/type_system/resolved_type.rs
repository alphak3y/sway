use crate::semantic_analysis::TypedExpression;
use crate::type_system::*;
use crate::types::{CompileWrapper, ToCompileWrapper};
use crate::{semantic_analysis::ast_node::TypedStructField, CallPath, Ident};

#[derive(Debug, Clone)]
pub enum ResolvedType {
    /// The number in a `Str` represents its size, which must be known at compile time
    Str(u64),
    UnsignedInteger(IntegerBits),
    Boolean,
    Unit,
    Byte,
    B256,
    #[allow(dead_code)]
    Struct {
        name: Ident,
        fields: Vec<TypedStructField>,
    },
    #[allow(dead_code)]
    Enum {
        name: Ident,
        variant_types: Vec<ResolvedType>,
    },
    /// Represents the contract's type as a whole. Used for implementing
    /// traits on the contract itself, to enforce a specific type of ABI.
    #[allow(dead_code)]
    Contract,
    /// Represents a type which contains methods to issue a contract call.
    /// The specific contract is identified via the `Ident` within.
    #[allow(dead_code)]
    ContractCaller {
        abi_name: CallPath,
        address: Box<TypedExpression>,
    },
    #[allow(dead_code)]
    Function {
        from: Box<ResolvedType>,
        to: Box<ResolvedType>,
    },
    /// used for recovering from errors in the ast
    #[allow(dead_code)]
    ErrorRecovery,
}

impl PartialEq for CompileWrapper<'_, ResolvedType> {
    fn eq(&self, other: &Self) -> bool {
        let CompileWrapper {
            inner: me,
            declaration_engine: de,
        } = self;
        let CompileWrapper { inner: them, .. } = other;
        match (me, them) {
            (ResolvedType::Str(l), ResolvedType::Str(r)) => l == r,
            (ResolvedType::UnsignedInteger(l), ResolvedType::UnsignedInteger(r)) => l == r,
            (ResolvedType::Boolean, ResolvedType::Boolean) => true,
            (ResolvedType::Unit, ResolvedType::Unit) => true,
            (ResolvedType::Byte, ResolvedType::Byte) => true,
            (ResolvedType::B256, ResolvedType::B256) => true,
            (
                ResolvedType::Struct {
                    name: l_name,
                    fields: l_fields,
                },
                ResolvedType::Struct {
                    name: r_name,
                    fields: r_fields,
                },
            ) => l_name == r_name && l_fields.wrap(de) == r_fields.wrap(de),
            (
                ResolvedType::Enum {
                    name: l_name,
                    variant_types: l_variant_types,
                },
                ResolvedType::Enum {
                    name: r_name,
                    variant_types: r_variant_types,
                },
            ) => {
                l_name == r_name
                    && l_variant_types
                        .iter()
                        .map(|x| x.wrap(de))
                        .collect::<Vec<_>>()
                        == r_variant_types
                            .iter()
                            .map(|x| x.wrap(de))
                            .collect::<Vec<_>>()
            }
            (ResolvedType::Contract, ResolvedType::Contract) => todo!(),
            (
                ResolvedType::ContractCaller {
                    abi_name: l_name, ..
                },
                ResolvedType::ContractCaller {
                    abi_name: r_name, ..
                },
            ) => l_name == r_name,
            (
                ResolvedType::Function {
                    from: l_from,
                    to: l_to,
                },
                ResolvedType::Function {
                    from: r_from,
                    to: r_to,
                },
            ) => {
                (**l_from).wrap(de) == (**r_from).wrap(de) && (**l_to).wrap(de) == (**r_to).wrap(de)
            }
            (ResolvedType::ErrorRecovery, ResolvedType::ErrorRecovery) => true,
            _ => false,
        }
    }
}

impl Default for ResolvedType {
    fn default() -> Self {
        ResolvedType::Unit
    }
}

impl ResolvedType {
    pub(crate) fn is_copy_type(&self) -> bool {
        matches!(
            self,
            ResolvedType::Boolean
                | ResolvedType::Byte
                | ResolvedType::Unit
                | ResolvedType::UnsignedInteger(_)
        )
    }

    #[allow(dead_code)]
    pub fn is_numeric(&self) -> bool {
        matches!(self, ResolvedType::UnsignedInteger(_))
    }
}
