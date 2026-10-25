//! 🙈️ Generated folders and bundle labels of the codebase: the generated output folder test and the bundle label normalisation. See [`super`].

use super::*;

//#region 🙈️Ignore
/// 🩶️ Reports whether any segment of a path marks generated output.
pub fn is_generated_folder(path: &str) -> bool {
    let normalized = normalize_path(path);
    for part in normalized.split('/') {
        if matches!(part, "generated" | "dist" | "build" | "node_modules" | "__pycache__" | ".next" | "coverage") {
            return true;
        }
    }
    for generated in ["js/vscode/generated", "js/compose/generated"] {
        if normalized == generated || normalized.starts_with(&format!("{generated}/")) {
            return true;
        }
    }
    false
}
//#endregion 🙈️Ignore

//#region 🏷️Kinds
/// 🏷️ Normalises a raw bundle name to the label the codebase aggregates key on.
pub fn normalize_bundle_label(name: &str) -> String {
    if name.is_empty() {
        return String::new();
    }
    if name.starts_with('@') {
        return name.to_string();
    }
    match name {
        "vscode" => "repo/vscode".to_string(),
        "repo" => "repo/go".to_string(),
        other => format!("compose/{other}"),
    }
}
//#endregion 🏷️Kinds
