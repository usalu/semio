//! ↩️ Inverse for `DuplicateLayer` consumes the same admitted target assignments.

use semio_s_artifact_draw_drawing_model::{clone_drawing_layer_node, find_drawing_layer, layer_id};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::delete_layer::mutation::DeleteLayer>>(payload: &super::mutation::DuplicateLayer, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => {
            let duplicate = clone_drawing_layer_node(layer, " copy", &payload.identities)?;
            if payload.identities.iter().any(|identity|find_drawing_layer(base,&identity.target).is_some()){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Drawing duplicate target already exists"));}
            vec![crate::delete_layer::mutation::delete_layer(layer_id(&duplicate).to_string().into())]
        }
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
