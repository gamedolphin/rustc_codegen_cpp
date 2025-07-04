use rustc_abi::{FieldIdx, FieldsShape, LayoutData, VariantIdx, Variants};

pub fn get_field_offsets(fields: &FieldsShape<FieldIdx>) -> Vec<usize> {
    match fields {
        FieldsShape::Primitive => vec![],
        FieldsShape::Union(non_zero) => vec![0; usize::from(*non_zero)],
        FieldsShape::Array { stride, count } => {
            let mut offsets = vec![0; usize::try_from(*count).expect("array size too large")];
            for i in 1..offsets.len() {
                offsets[i] = offsets[i - 1] + stride.bytes_usize();
            }
            offsets
        }
        FieldsShape::Arbitrary {
            offsets,
            memory_index,
        } => memory_index
            .iter()
            .enumerate()
            .map(|(index, _)| offsets[FieldIdx::from_usize(index)].bytes_usize())
            .collect(),
    }
}

pub fn get_variant_at_index(
    variant_index: VariantIdx,
    layout: LayoutData<FieldIdx, rustc_abi::VariantIdx>,
) -> LayoutData<FieldIdx, rustc_abi::VariantIdx> {
    match layout.variants {
        Variants::Single { .. } => layout,
        Variants::Multiple { variants, .. } => variants[variant_index].clone(),
        Variants::Empty => todo!("Empty variants have no variants."),
    }
}

pub fn get_field_name(index: usize) -> String {
    format!("field_{index}")
}
