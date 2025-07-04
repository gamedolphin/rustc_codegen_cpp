use rustc_abi::CanonAbi;
use rustc_abi::ExternAbi;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;
use rustc_middle::mir::mono::MonoItem;
use rustc_middle::ty::Instance;
use rustc_middle::ty::List;
use rustc_middle::ty::PseudoCanonicalInput;
use rustc_middle::ty::Ty;
use rustc_middle::ty::TyCtxt;
use rustc_middle::ty::TyKind;
use rustc_middle::ty::TypingEnv;

use crate::cpp::closure::get_closure_type;
use crate::cpp::function::FunctionSignature;
use crate::cpp::generator::get_arg_type;
use crate::cpp::generator::get_decl_type;
use crate::cpp::typ::get_type;
use crate::cpp::typ::sort_types;

use super::closure::Closure;
use super::enums::Enum;
use super::function::add_function;
use super::function::Function;
use super::function::FunctionContext;
use super::structs::Strukt;
use super::typ::get_type_hash;
use super::typ::Type;
use super::typ::TypeVal;

#[derive(Default)]
pub struct Project {
    pub typs: HashMap<u128, TypeVal>,
    pub main: Option<String>,
    pub functions: HashMap<String, Function>,
    pub function_signatures: HashMap<u128, FunctionSignature>,
    pub closures: HashMap<u128, Closure>,
    pub strukts: HashMap<u128, Strukt>,
    pub enums: HashMap<u128, Enum>,
}

impl Project {
    pub fn add_type<'tcx>(&mut self, tcx: TyCtxt<'tcx>, actual: Ty<'tcx>, ty: Type) -> u128 {
        let hash = get_type_hash(tcx, actual);
        self.typs.insert(hash, TypeVal { hash, ty });

        hash
    }

    pub fn add_closure(&mut self, hash: u128, closure: Closure) {
        self.closures.insert(hash, closure);
    }

    pub fn output(&self) -> String {
        let mut output = Vec::new();
        let mut includes = HashSet::default();

        output.push("// TYPES:".to_string());

        let sorted_typs: Vec<_> = sort_types(self, &self.typs);

        for (index, ty) in sorted_typs.iter() {
            let val = get_decl_type(*index, ty, self, &mut includes);
            if val.is_empty() {
                continue;
            }
            output.push(val);
        }
        output.push("\n\n".to_string());

        for (name, function) in &self.functions {
            let args = function
                .signature
                .args
                .iter()
                .filter(|ty| !matches!(ty.ty, Type::Void))
                .enumerate()
                .map(|(index, ty)| {
                    let arg_index = index + 1;
                    get_arg_type(&ty.ty, self, &mut includes).to_print(&format!("_{arg_index}"))
                })
                .collect::<Vec<_>>()
                .join(", ");
            let return_type =
                get_arg_type(&function.signature.return_type.ty, self, &mut includes).to_print("");
            let body = &function.body;

            let arg_count = function.signature.args.len();
            let locals = function
                .locals
                .iter()
                .enumerate()
                .filter(|(index, _)| *index == 0 || *index > arg_count)
                .filter(|(_, ty)| !matches!(&ty.ty, Type::Void))
                .map(|(index, ty)| {
                    let decl = get_arg_type(&ty.ty, self, &mut includes).actual;
                    format!("{decl} _{index};")
                })
                .collect::<Vec<_>>()
                .join("\n");

            let return_val =
                if !function.locals.is_empty() && function.signature.return_type.ty != Type::Void {
                    "return _0;"
                } else {
                    ""
                };

            output.push(format!(
                r#"
{return_type} {name}({args}) {{
{locals}

{body}

{return_val}
}}"#
            ));
        }

        let includes = includes
            .into_iter()
            .map(|include| format!("#include {}", include))
            .collect::<Vec<_>>()
            .join("\n");

        output.insert(0, includes);

        if let Some(main) = &self.main {
            output.push(format!("int main() {{\n {main}(); \n}}"));
        }

        output.join("\n")
    }
}

pub fn add_item<'tcx>(
    project: &mut Project,
    tcx: TyCtxt<'tcx>,
    item: MonoItem<'tcx>,
) -> Result<(), String> {
    match item {
        MonoItem::Fn(instance) => add_function(project, tcx, instance)?,
        MonoItem::Static(def_id) => {}
        MonoItem::GlobalAsm(item_id) => {}
    }

    Ok(())
}
