//! 🔺️ Sparse diff builder for `CreateFenestration` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateFenestration, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.fenestrations.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Fenestration {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A fenestration name must not be blank.", [payload.id.0.to_string()]);
    }
    if !base.model.surfaces.iter().any(|item| item.id == payload.surface_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Surface {} does not exist.", payload.surface_id.0), [payload.id.0.to_string()]);
    }
    if payload.glazing_construction_id.is_some_and(|glazing| !base.model.constructions.iter().any(|item| item.id == glazing)) {
        return protocol::MutationOutcome::error("mutation.target-missing", "The named glazing construction does not exist.", [payload.id.0.to_string()]);
    }
    if !payload.u_value_w_m2k.is_finite() || payload.u_value_w_m2k <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Fenestration {} needs a positive finite U-value.", payload.id.0), [payload.id.0.to_string()]);
    }
    if !payload.shgc.is_finite() || !(0.0..=1.0).contains(&payload.shgc) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Fenestration {} needs an SHGC in 0..=1.", payload.id.0), [payload.id.0.to_string()]);
    }
    if !payload.vlt.is_finite() || !(0.0..=1.0).contains(&payload.vlt) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Fenestration {} needs a visible transmittance in 0..=1.", payload.id.0), [payload.id.0.to_string()]);
    }
    if !payload.area_m2.is_finite() || payload.area_m2 <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Fenestration {} needs a positive finite area.", payload.id.0), [payload.id.0.to_string()]);
    }
    if !payload.height_m.is_finite() || payload.height_m <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Fenestration {} needs a positive finite height.", payload.id.0), [payload.id.0.to_string()]);
    }
    if !payload.sill_height_m.is_finite() || payload.sill_height_m < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Fenestration {} needs a non-negative finite sill height.", payload.id.0), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    let position = model.fenestrations.iter().position(|item| item.id > payload.id).unwrap_or(model.fenestrations.len());
    model.fenestrations.insert(position, crate::model::Fenestration { id: payload.id, name: payload.name.clone(), surface_id: payload.surface_id, u_value_w_m2k: payload.u_value_w_m2k, shgc: payload.shgc, vlt: payload.vlt, area_m2: payload.area_m2, height_m: payload.height_m, sill_height_m: payload.sill_height_m, frame_conductance_w_k: payload.frame_conductance_w_k, divider_conductance_w_k: payload.divider_conductance_w_k, overhang_depth_m: payload.overhang_depth_m, overhang_offset_m: payload.overhang_offset_m, fin_depth_m: payload.fin_depth_m, fin_offset_m: payload.fin_offset_m, glazing_construction_id: payload.glazing_construction_id });
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
