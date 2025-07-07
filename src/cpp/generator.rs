use rustc_hash::FxHashSet as HashSet;
use rustc_middle::mir::BinOp;

use super::{
    constants::Constant, dyad::Dyad, function::FunctionSignature, project::Project,
    structs::get_struct_name, typ::Type,
};
use crate::cpp::{
    enums::{get_enum_name, get_enum_variant_name, EnumVariant},
    fields::get_field_name,
    ops::Op,
    statements::Line,
    typ::{FloatType, IntType, UIntType},
    value::Value,
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
        Type::Char => "char".to_string(),
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
                FloatType::F16 => "std::float16_t",
                FloatType::F32 => "float",
                FloatType::F64 => "double",
                FloatType::F128 => "std::float128_t",
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
        Type::RawPtr(inner, mutable) => {
            let inner_ty = proj.typs.get(inner).unwrap();
            let inner_type = get_arg_type(&inner_ty.ty, proj, includes);
            let mut_str = if *mutable { "" } else { "const " };
            // comment = format!("raw pointer to {}", inner_type.actual);
            format!("{}{}*", mut_str, inner_type.actual)
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

pub fn get_body(body: &[Vec<Line>], includes: &mut HashSet<String>) -> String {
    body.iter()
        .map(|statements| {
            statements
                .iter()
                .map(|line| match line {
                    Line::Assignment { lhs, rhs } => {
                        let val = get_value_string(rhs, includes);
                        if let Value::Todo(_) = rhs {
                            return format!("// {lhs} = {val};");
                        } else {
                            format!("{lhs} = {val};",)
                        }
                    }
                    Line::Todo(todo) => format!("// Statement: {todo}"),
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .enumerate()
        .map(|(index, stmts)| format!("bb{index}:\n{stmts}"))
        .collect::<Vec<_>>()
        .join("\n\n")
}

pub fn get_value_string(val: &Value, includes: &mut HashSet<String>) -> String {
    match val {
        Value::Use(op) => get_op_string(op, includes),
        Value::BinaryOp(dyad) => get_dyad_string(dyad, includes),
        Value::Todo(output) => output.to_string(),
    }
}

pub fn get_dyad_string(dyad: &Dyad, includes: &mut HashSet<String>) -> String {
    let left = get_op_string(&dyad.left, includes);
    let right = get_op_string(&dyad.right, includes);
    match dyad.operation {
        BinOp::Add => format!("{left} + {right}"),
        BinOp::AddUnchecked => format!("{left} + {right} /* unchecked */"),
        BinOp::AddWithOverflow => {
            includes.insert("<stdckdint.h>".to_string());
            format!("CHECKED_ADD({left}, {right})")
        }
        BinOp::Sub => format!("{left} - {right}"),
        BinOp::SubUnchecked => format!("{left} - {right} /* unchecked */"),
        BinOp::SubWithOverflow => {
            includes.insert("<stdckdint.h>".to_string());
            format!("CHECKED_SUB({left}, {right})")
        }
        BinOp::Mul => format!("{left} * {right}"),
        BinOp::MulUnchecked => format!("{left} * {right} /* unchecked */"),
        BinOp::MulWithOverflow => {
            includes.insert("<stdckdint.h>".to_string());
            format!("CHECKED_MUL({left}, {right})")
        }
        BinOp::Div => format!("{left} / {right}"),
        BinOp::Rem => format!("{left} % {right}"),
        BinOp::BitXor => format!("{left} ^ {right}"),
        BinOp::BitAnd => format!("{left} & {right}"),
        BinOp::BitOr => format!("{left} | {right}"),
        BinOp::Shl => format!("{left} << {right}"),
        BinOp::ShlUnchecked => format!("{left} << {right} /* unchecked */"),
        BinOp::Shr => format!("{left} >> {right}"),
        BinOp::ShrUnchecked => format!("{left} >> {right} /* unchecked */"),
        BinOp::Eq => format!("{left} == {right}"),
        BinOp::Lt => format!("{left} < {right}"),
        BinOp::Le => format!("{left} <= {right}"),
        BinOp::Ne => format!("{left} != {right}"),
        BinOp::Ge => format!("{left} >= {right}"),
        BinOp::Gt => format!("{left} > {right}"),
        BinOp::Cmp => format!("{left} <=> {right}"),
        BinOp::Offset => format!("{left} + {right} * sizeof({})", left),
    }
}

pub fn get_op_string(op: &Op, includes: &mut HashSet<String>) -> String {
    match op {
        Op::Constant(val) => val.to_string(),
        Op::Copy(output, ty) => {
            // checked to be trivially copyable by rust compiler
            includes.insert("<bit>".to_string());
            let cast_type = get_arg_type(ty, &Project::default(), includes).actual;
            format!("std::bit_cast<{cast_type}>({output})")
        }
        Op::Move(output) => {
            includes.insert("<utility>".to_string());
            format!("std::move({})", output)
        }
    }
}

pub fn get_constant_definition(
    name: &str,
    constant: &Constant,
    project: &Project,
    includes: &mut HashSet<String>,
) -> String {
    let typ = get_arg_type(&constant.typ, project, includes);
    let const_val = constant.value.clone();
    let mut output = String::new();
    let type_val = match &constant.typ {
        Type::Void => panic!("Cannot generate string for void type constant"),
        Type::Bool => {
            let value = u8::from_ne_bytes(const_val.try_into().unwrap());
            let value = if value == 0 { "false" } else { "true" };
            format!("constexpr {} {} = {};", typ.actual, name, value)
        }
        Type::Char => {
            let char_value = u32::from_ne_bytes(const_val.try_into().unwrap());
            format!(
                "constexpr {} {} = '{}';",
                typ.actual,
                name,
                char::from_u32(char_value).expect("Invalid char value")
            )
        }
        Type::Int(int_type) => match int_type {
            IntType::I8 => {
                let value = i8::from_ne_bytes(
                    u8::from_ne_bytes(const_val.try_into().unwrap()).to_ne_bytes(),
                );
                format!("constexpr {} {} = {};", typ.actual, name, value)
            }
            IntType::I16 => {
                let value = i16::from_ne_bytes(
                    u16::from_ne_bytes(const_val.try_into().unwrap()).to_ne_bytes(),
                );
                format!("constexpr {} {} = {};", typ.actual, name, value)
            }
            IntType::I32 => {
                let value = i32::from_ne_bytes(
                    u32::from_ne_bytes(const_val.try_into().expect(&format!(
                        "expected val to be of length 4, but was {}, with size: {}",
                        constant.value.len(),
                        constant.size
                    )))
                    .to_ne_bytes(),
                );
                format!("constexpr {} {} = {};", typ.actual, name, value)
            }
            IntType::I64 => {
                let value = i64::from_ne_bytes(
                    u64::from_ne_bytes(const_val.try_into().unwrap()).to_ne_bytes(),
                );
                format!("constexpr {} {} = {};", typ.actual, name, value)
            }
            IntType::I128 => format!(
                "constexpr {} {} = {};",
                typ.actual,
                name,
                i128::from_ne_bytes(const_val.try_into().unwrap())
            ),
            IntType::Isize => {
                let value = isize::from_ne_bytes(
                    u64::from_ne_bytes(const_val.try_into().unwrap()).to_ne_bytes(),
                );
                format!("constexpr {} {} = {};", typ.actual, name, value)
            }
        },
        Type::UInt(uint_type) => match uint_type {
            UIntType::U8 => {
                let value = u8::from_ne_bytes(const_val.try_into().unwrap());
                format!("constexpr {} {} = {};", typ.actual, name, value)
            }
            UIntType::U16 => {
                let value = u16::from_ne_bytes(const_val.try_into().unwrap());
                format!("constexpr {} {} = {};", typ.actual, name, value)
            }
            UIntType::U32 => {
                let value = u32::from_ne_bytes(const_val.try_into().unwrap());
                format!("constexpr {} {} = {};", typ.actual, name, value)
            }
            UIntType::U64 => {
                let value = u64::from_ne_bytes(const_val.try_into().unwrap());
                format!("constexpr {} {} = {};", typ.actual, name, value)
            }
            UIntType::U128 => {
                format!(
                    "constexpr {} {} = {};",
                    typ.actual,
                    name,
                    u128::from_ne_bytes(const_val.try_into().unwrap())
                )
            }
            UIntType::Usize => {
                let value = usize::from_ne_bytes(
                    u64::from_ne_bytes(const_val.try_into().unwrap()).to_ne_bytes(),
                );
                format!("constexpr {} {} = {};", typ.actual, name, value)
            }
        },
        Type::Float(float_type) => match float_type {
            FloatType::F16 => {
                includes.insert("<stdfloat>".to_string());
                let value = f32::from_ne_bytes(
                    u32::from_ne_bytes(const_val.try_into().unwrap()).to_ne_bytes(),
                );
                format!(
                    "constexpr {} {} = {}; // F16 not supported",
                    typ.actual, name, value
                )
            }
            FloatType::F32 => {
                let value = f32::from_ne_bytes(
                    u32::from_ne_bytes(const_val.try_into().unwrap()).to_ne_bytes(),
                );
                format!("constexpr {} {} = {}; ", typ.actual, name, value)
            }
            FloatType::F64 => {
                let value = f64::from_ne_bytes(
                    (u64::from_ne_bytes(const_val.try_into().unwrap())).to_ne_bytes(),
                );
                format!("constexpr {} {} = {};", typ.actual, name, value)
            }
            FloatType::F128 => {
                includes.insert("<stdfloat>".to_string());
                format!(
                    "constexpr {} {} = {}; // F128 not supported",
                    typ.actual,
                    name,
                    f128::from_ne_bytes(const_val.try_into().unwrap()).is_finite()
                )
            }
        },
        Type::Struct(hash) => {
            includes.insert("<bit>".to_string());
            // get array of bytes
            let bytes = const_val
                .iter()
                .map(|b| format!("{b}"))
                .collect::<Vec<_>>()
                .join(",");
            let bytes = format!("{{{bytes}}}");
            let bytes_constexpr = format!(
                "constexpr std::array<uint8_t, {}> {name}_bytes = {};",
                constant.size, bytes
            );

            let struct_name = get_struct_name(*hash);
            format!(
                r#"
{bytes_constexpr}
constexpr {struct_name} {name} = std::bit_cast<{struct_name}>({name}_bytes);
"#
            )
        }
        Type::Array(inner, size, _, _) => {
            let inner_ty = project.typs.get(inner).unwrap();
            let mut inner_type = get_arg_type(&inner_ty.ty, project, includes);
            let data = if constant.deps.is_empty() {
                let inner_base_type = inner_ty.ty.get_base_cgen_type(project);
                inner_type = get_arg_type(&inner_base_type, project, includes);
                let bytes = const_val
                    .iter()
                    .map(|b| format!("{b}"))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{{{bytes}}}")
            } else {
                constant
                    .deps
                    .iter()
                    .map(|d| format!("{}({})", inner_type.actual, d))
                    .collect::<Vec<_>>()
                    .join(",")
            };

            format!(
                r#"
constexpr std::array<{}, {}> {name} = {{{}}};
"#,
                inner_type.actual, size, data
            )
        }
        Type::RawPtr(hash, _) => {
            let inner_ty = project.typs.get(hash).unwrap();
            let inner_type = get_arg_type(&inner_ty.ty, project, includes);
            let data_name = format!("{name}_data");
            let constant = Constant {
                typ: inner_ty.ty.clone(),
                value: const_val,
                size: constant.size,
                deps: constant.deps.clone(),
            };
            let output_str = get_constant_definition(&data_name, &constant, project, includes);
            format!(
                "{output_str}\n\nconstexpr const {}* {name} = &{};",
                inner_type.actual, data_name
            )
        }
        _ => format!(
            "// Cannot generate string for constant of type {:?} with value {:?}",
            typ.actual, constant.value
        ),
        // Type::Closure(_) => todo!(),
        // Type::Todo(_) => todo!(),
        // Type::String => todo!(),
        // Type::StringView => todo!(),
        // Type::FnPtr(_) => todo!(),

        // Type::Struct(_) => todo!(),
        // Type::Enum(_) => todo!(),
        // Type::Array(_, _, _, _) => todo!(),
        // Type::Span(_, _) => todo!(),
        // Type::Tuple(items) => todo!(),
    };

    output.push_str(&type_val);

    output
}
