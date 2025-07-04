use rustc_hash::FxHashSet as HashSet;

use super::{function::FunctionSignature, project::Project, structs::get_struct_name, typ::Type};
use crate::cpp::{
    enums::{get_enum_name, get_enum_variant_name, EnumVariant},
    fields::get_field_name,
    typ::{FloatType, IntType, UIntType},
};

pub struct ArgInfo {
    pub actual: String,
    pub comment: String,
}

impl ArgInfo {
    pub fn to_print(&self, name: &str) -> String {
        format!(
            "{} {}{}",
            self.actual,
            if self.actual.is_empty() {
                format!("/* {name} */")
            } else {
                name.to_string()
            },
            if !self.comment.is_empty() {
                format!(" /*{}*/", self.comment)
            } else {
                String::new()
            }
        )
    }
}

pub fn get_decl_type(
    hash: u128,
    ty: &Type,
    proj: &Project,
    includes: &mut HashSet<String>,
) -> String {
    let mut output = String::new();
    match ty {
        Type::Int(_) => {
            includes.insert("<cstdint>".to_string());
        }
        Type::UInt(_) => {
            includes.insert("<cstdint>".to_string());
        }
        Type::Float(_) => {
            includes.insert("<stdfloat>".to_string());
        }
        Type::Closure(index) => {
            includes.insert("<functional>".to_string());
            let closure = proj.closures.get(index).unwrap();
            let typedef = get_fn_signature(&closure.signature, proj, includes);
            let name = get_closure_name(hash);
            output = format!("using {name} = {typedef};");
        }
        Type::Todo(_) => {}
        Type::String => {}
        Type::FnPtr(index) => {
            includes.insert("<functional>".to_string());
            let signature = proj.function_signatures.get(index).unwrap();
            let typedef = get_fn_signature(signature, proj, includes);
            let name = get_fn_pointer_name(hash);
            output = format!("using {name} = {typedef};");
        }
        Type::Struct(index) => {
            let name = get_struct_name(*index);
            let struct_def = proj.strukts.get(index).unwrap();
            let fields = struct_def
                .fields
                .iter()
                .enumerate()
                .map(|(index, (ty, offset))| {
                    let arg_info = get_arg_type(&ty.ty, proj, includes);
                    format!(
                        "{} {}; {}",
                        arg_info.actual,
                        get_field_name(index),
                        if !arg_info.comment.is_empty() {
                            format!(" /*{}*/", arg_info.comment)
                        } else {
                            String::new()
                        }
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            output = format!(
                r#"
// {index}
struct {name} {{
{fields}
}};
"#
            );
        }
        Type::Enum(hash) => {
            let val = proj.enums.get(hash).unwrap();
            let variants = val
                .variants
                .iter()
                .map(|(discriminant, variant)| {
                    let name = get_enum_variant_name(*hash, *discriminant);
                    match variant {
                        EnumVariant::Tag => {
                            format!("struct {name} {{}};")
                        }
                        EnumVariant::Fields(fields) => {
                            let fields_str = fields
                                .iter()
                                .enumerate()
                                .map(|(index, (ty, offset))| {
                                    let arg_info = get_arg_type(&ty.ty, proj, includes);
                                    format!(
                                        "    {} {}; {}",
                                        arg_info.actual,
                                        get_field_name(index),
                                        if !arg_info.comment.is_empty() {
                                            format!(" /*{}*/", arg_info.comment)
                                        } else {
                                            String::new()
                                        }
                                    )
                                })
                                .collect::<Vec<_>>()
                                .join("\n");
                            format!("struct {name} {{\n{fields_str}\n}};")
                        }
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            let variant_names = val
                .variants
                .iter()
                .map(|(discriminant, _)| get_enum_variant_name(*hash, *discriminant))
                .collect::<Vec<_>>()
                .join(", ");
            let enum_name = get_enum_name(*hash);
            output = format!(
                r#"
{variants}

// {hash}
using {enum_name} = std::variant<{variant_names}>;
"#
            );
        }
        _ => {}
    };

    output
}

fn get_fn_signature(
    signature: &FunctionSignature,
    proj: &Project,
    includes: &mut HashSet<String>,
) -> String {
    let return_type = get_arg_type(&signature.return_type.ty, proj, includes);
    let args = signature
        .args
        .iter()
        .filter(|ty| !matches!(ty.ty, Type::Void))
        .map(|ty| get_arg_type(&ty.ty, proj, includes))
        .collect::<Vec<_>>();

    let return_type = return_type.actual;
    let args_str = args
        .into_iter()
        .map(|arg| arg.actual)
        .collect::<Vec<_>>()
        .join(", ");

    format!("std::function<{return_type}({args_str})>")
}

pub fn get_arg_type(ty: &Type, proj: &Project, includes: &mut HashSet<String>) -> ArgInfo {
    let mut comment = String::new();
    let actual = match ty {
        Type::Void => "void".to_string(),
        Type::Bool => "bool".to_string(),
        Type::Char => "char32_t".to_string(),
        Type::Int(int_type) => {
            includes.insert("<cstdint>".to_string());
            match int_type {
                IntType::I8 => "int8_t",
                IntType::I16 => "int16_t",
                IntType::I32 => "int32_t",
                IntType::I64 => "int64_t",
                IntType::I128 => "int128_t",
                IntType::Isize => "intptr_t",
            }
            .to_string()
        }
        Type::UInt(uint_type) => {
            includes.insert("<cstdint>".to_string());
            match uint_type {
                UIntType::U8 => "uint8_t",
                UIntType::U16 => "uint16_t",
                UIntType::U32 => "uint32_t",
                UIntType::U64 => "uint64_t",
                UIntType::U128 => "uint128_t",
                UIntType::Usize => "uintptr_t",
            }
            .to_string()
        }
        Type::Float(float_type) => {
            includes.insert("<stdfloat>".to_string());
            match float_type {
                FloatType::F16 => "float16_t",
                FloatType::F32 => "float32_t",
                FloatType::F64 => "float64_t",
                FloatType::F128 => "float128_t",
            }
            .to_string()
        }
        Type::Closure(index) => {
            includes.insert("<functional>".to_string());
            get_closure_name(*index)
        }
        Type::Todo(v) => {
            comment = v.to_string();
            "int".to_string()
        }
        Type::String => {
            includes.insert("<string>".to_string());
            "std::string".to_string()
        }
        Type::FnPtr(hash) => {
            includes.insert("<functional>".to_string());
            get_fn_pointer_name(*hash)
        }
        Type::RawPtr(inner, _) => {
            let inner_ty = proj.typs.get(inner).unwrap();
            let inner_type = get_arg_type(&inner_ty.ty, proj, includes);
            // comment = format!("raw pointer to {}", inner_type.actual);
            format!("{}*", inner_type.actual)
        }
        Type::Struct(hash) => get_struct_name(*hash),
        Type::Enum(hash) => {
            includes.insert("<variant>".to_string());
            get_enum_name(*hash)
        }
        Type::Array(inner, size, _, _) => {
            let inner_ty = proj.typs.get(inner).unwrap();
            let inner_type = get_arg_type(&inner_ty.ty, proj, includes);
            comment = format!("array of {} with size {}", inner_type.actual, size);

            includes.insert("<array>".to_string());
            format!("std::array<{}, {}>", inner_type.actual, size)
        }
        Type::StringView => {
            includes.insert("<string_view>".to_string());
            "std::string_view".to_string()
        }
        Type::Span(inner, mutable) => {
            let inner_ty = proj.typs.get(inner).unwrap();
            let inner_type = get_arg_type(&inner_ty.ty, proj, includes);
            comment = format!("span of {}", inner_type.actual);

            includes.insert("<span>".to_string());
            if *mutable {
                format!("std::span<const {}>", inner_type.actual)
            } else {
                format!("std::span<{}>", inner_type.actual)
            }
        }
        Type::Tuple(tys) => {
            includes.insert("<tuple>".to_string());
            let inner_types = tys
                .iter()
                .map(|internal| {
                    let typ = proj.typs.get(internal).unwrap();
                    get_arg_type(&typ.ty, proj, includes).actual
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("std::tuple<{inner_types}>")
        }
    };

    ArgInfo { actual, comment }
}

fn get_fn_pointer_name(hash: u128) -> String {
    format!("fn_ptr_{hash:x}")
}

fn get_closure_name(hash: u128) -> String {
    format!("closure_{hash:x}")
}
