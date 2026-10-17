//! 🏭️ Recipes, generation and verification: the DXF and STEP recipes, the fixture directories, fixture generation and the STEP round trip check. See [`super`].

use super::*;

//#region 🏭️Fixtures
pub(super) const DXF_RECIPES: &[&str] = &[
    "add-layer-hidden-services",
    "remove-layer-scratch",
    "set-layer-walls-color",
    "add-block-door",
    "remove-block-window",
    "set-block-base-point-door",
    "add-entity-arc-fillet",
    "remove-entity-middle-polyline",
    "set-entity-layer-text-to-annotations",
    "set-entity-geometry-circle-radius",
    "add-block-entity-door-swing",
    "remove-block-entity-window-mullion",
    "set-block-entity-layer-door-leaf",
    "set-block-entity-geometry-window-pane",
];

pub(super) const STEP_RECIPES: &[&str] = &["step-add-entity-circle", "step-remove-entity-line", "step-set-entity-geometry-circle-radius"];

const FIXTURE_DIRECTORY_BY_RECIPE: &[(&str, &str)] = &[
    ("add-block-door", "🚪️add-block-door"),
    ("add-block-entity-door-swing", "🌀️add-block-entity-door-swing"),
    ("add-entity-arc-fillet", "🌙️add-entity-arc-fillet"),
    ("add-layer-hidden-services", "🫥️add-layer-hidden-services"),
    ("remove-block-entity-window-mullion", "🪟️remove-block-entity-window-mullion"),
    ("remove-block-window", "🏚️remove-block-window"),
    ("remove-entity-middle-polyline", "〰️remove-entity-middle-polyline"),
    ("remove-layer-scratch", "🧹️remove-layer-scratch"),
    ("set-block-base-point-door", "📍️set-block-base-point-door"),
    ("set-block-entity-geometry-window-pane", "🔲️set-block-entity-geometry-window-pane"),
    ("set-block-entity-layer-door-leaf", "🍃️set-block-entity-layer-door-leaf"),
    ("set-entity-geometry-circle-radius", "⭕️set-entity-geometry-circle-radius"),
    ("set-entity-layer-text-to-annotations", "🏷️set-entity-layer-text-to-annotations"),
    ("set-layer-walls-color", "🎨️set-layer-walls-color"),
    ("step-add-entity-circle", "➕️step-add-entity-circle"),
    ("step-remove-entity-line", "➖️step-remove-entity-line"),
    ("step-set-entity-geometry-circle-radius", "📏️step-set-entity-geometry-circle-radius"),
];

fn fixture_directory(recipe: &str) -> Option<&'static str> {
    FIXTURE_DIRECTORY_BY_RECIPE.iter().find_map(|(registered_recipe, directory)| (*registered_recipe == recipe).then_some(*directory))
}

/// 🏗️ Writes one recipe's bundle into its reviewed emoji-bearing directory.
// 🚫️async: filesystem entry point.
pub(super) fn generate(recipe: &str, out_root: &str) -> Result<Vec<String>, String> {
    let fixture_directory = fixture_directory(recipe).ok_or_else(|| format!("no reviewed fixture directory for {recipe}"))?;
    let dir = format!("{out_root}/{fixture_directory}");
    std::fs::create_dir_all(&dir).map_err(|e| format!("mkdir {dir}: {e}"))?;
    let mut written = Vec::new();
    if DXF_RECIPES.contains(&recipe) {
        for (name, drawing) in [("⬅️before.dxf", base_drawing()), ("➡️after.dxf", after_drawing(recipe).ok_or_else(|| format!("unknown recipe {recipe}"))?)] {
            let path = format!("{dir}/{name}");
            pin_wall_clock(&drawing)?.save_file(&path).map_err(|e| format!("dxf save {path}: {e}"))?;
            written.push(path);
        }
        if recipe == "set-entity-geometry-circle-radius" {
            let path = format!("{dir}/⚠️counterexample-after.dxf");
            let counterexample = after_drawing("set-entity-geometry-circle-radius-counterexample").ok_or("counterexample")?;
            pin_wall_clock(&counterexample)?.save_file(&path).map_err(|e| format!("dxf save {path}: {e}"))?;
            written.push(path);
        }
    } else if STEP_RECIPES.contains(&recipe) {
        for (name, after) in [("⬅️before.step", false), ("➡️after.step", true)] {
            let entities = step_entities(recipe, after).ok_or_else(|| format!("unknown recipe {recipe}"))?;
            let path = format!("{dir}/{name}");
            std::fs::write(&path, write_step(&entities)).map_err(|e| format!("write {path}: {e}"))?;
            verify_step_roundtrip(&path, &entities)?;
            written.push(path);
        }
        if recipe == "step-set-entity-geometry-circle-radius" {
            let entities = step_entities("step-set-entity-geometry-circle-radius-counterexample", true).ok_or("counterexample")?;
            let path = format!("{dir}/⚠️counterexample-after.step");
            std::fs::write(&path, write_step(&entities)).map_err(|e| format!("write {path}: {e}"))?;
            verify_step_roundtrip(&path, &entities)?;
            written.push(path);
        }
    } else {
        return Err(format!("unknown recipe {recipe}"));
    }
    Ok(written)
}

/// ✅️ Refuses to emit a STEP fixture ruststep cannot read back to exactly what went in.
// 🚫️async: filesystem verification helper.
fn verify_step_roundtrip(path: &str, entities: &[(String, Vec<f64>)]) -> Result<(), String> {
    let (reading, unresolved) = read_step(path)?;
    if unresolved != 0 {
        return Err(format!("{path}: ruststep left {unresolved} entity(ies) unresolved"));
    }
    if reading.entities.len() != entities.len() {
        return Err(format!("{path}: ruststep recovered {} entity(ies), wrote {}", reading.entities.len(), entities.len()));
    }
    for (i, (want, got)) in entities.iter().zip(reading.entities.iter()).enumerate() {
        if want.0 != got.kind {
            return Err(format!("{path}: entity {i} kind {} became {}", want.0, got.kind));
        }
        for (j, (x, y)) in want.1.iter().zip(got.geom.iter()).enumerate() {
            if (x - y).abs() > 1e-9 {
                return Err(format!("{path}: entity {i} coordinate {j} {x} became {y}"));
            }
        }
    }
    Ok(())
}
//#endregion 🏭️Fixtures
