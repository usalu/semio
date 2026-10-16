//! ⏳️ `shell-async` staging of `dock-model-and-measures`: RENDERER-INTEGRATION-2, the items the direction graph homes in this crate (they were in a higher crate or in the old chunk file).

use super::*;

//#region ShellLifecycle
//#region 📏️WindowMeasures
/// 🪟️ The window instance a Measures overlay surface belongs to, `None` for every other surface.
pub fn window_measures_owner(surface_id: &str) -> Option<&str> {
    surface_id.strip_suffix(semio_framework::UiRefreshSection::Measures.body_key()).and_then(|owner| owner.strip_suffix('/')).filter(|owner| !owner.is_empty())
}
//#endregion 📏️WindowMeasures
//#endregion ShellLifecycle
