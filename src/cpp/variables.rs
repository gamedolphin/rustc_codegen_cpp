use rustc_middle::mir::{Place, ProjectionElem};

use crate::cpp::fields::get_field_name;

pub fn get_variable<'tcx>(place: &Place<'tcx>) -> String {
    if let Some(local_index) = place.as_local() {
        return format!("_{}", local_index.index());
    }

    let mut output = String::new();
    place.iter_projections().for_each(|(p, r)| {
        if let Some(local_index) = p.as_local() {
            output.push_str(&format!("_{}", local_index.index()));
        }
        match r {
            ProjectionElem::Deref => output.push_str("->"),
            ProjectionElem::Field(field_idx, _) => {
                let field_name = get_field_name(field_idx.as_usize());
                output.push_str(&format!(".{field_name}"));
            }
            ProjectionElem::Index(local) => {
                output.push_str(&format!("[{}]", local.index()));
            }
            ProjectionElem::ConstantIndex {
                offset,
                min_length,
                from_end,
            } => output.push_str(&format!("[{}..{}]", offset, min_length)),
            ProjectionElem::Subslice { from, to, from_end } => {
                // use std::slice here instead
                output.push_str(&format!("[{}..{}]", from, to));
                if from_end {
                    output.push_str(" (from end)");
                }
            }
            _ => {
                output.push_str(&format!(".unhandled-{r:?}"));
            }
        }
    });

    output
}
