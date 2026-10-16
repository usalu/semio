//! 🫧️ `shell-surfaces` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
//#region 🔀️ChromeParity
/// ⏳️ The FULL declared shape of a `World3dScene.statusJson` — the Rust twin of
/// `World3dComputeStatusV1`/`world3dComputeStatusV1` (`🔨️modules/🖱️ui/🎬️scene/🟦️.ts`), field for
/// field, so the wgpu World3d surface reads exactly what React's `WorldComputeStatusPane`
/// (`🌐️World3dHost/🟦️.tsx`) reads. Total by construction: malformed JSON, a missing field or a
/// hostile type degrades to the neutral status rather than throwing inside a render — a status
/// overlay must never be able to blank the viewport it annotates.
///
/// 🌍️ `phase_label` is the PRODUCER's own `{en, de}` pair; the shell never invents phase text and
/// never picks a default language — an absent pair falls back to the shell's own bilingual
/// `common.loading`, exactly as React's pane does.
#[derive(Clone, Debug, PartialEq)]
pub struct World3dComputeStatus {
    pub computing: bool,
    pub phase: String,
    pub phase_label: Option<(String, String)>,
    pub units_done: f64,
    pub units_total: f64,
    pub faces_done: f64,
    pub faces_total: f64,
    pub in_flight: f64,
    pub ratio: f64,
    pub cancellable: bool,
    pub cancel_action: String,
    pub cancel_args: serde_json::Map<String, Value>,
}

impl Default for World3dComputeStatus {
    fn default() -> Self {
        Self {
            computing: false,
            phase: "idle".to_string(),
            phase_label: None,
            units_done: 0.0,
            units_total: 0.0,
            faces_done: 0.0,
            faces_total: 0.0,
            in_flight: 0.0,
            ratio: 1.0,
            cancellable: false,
            cancel_action: String::new(),
            cancel_args: serde_json::Map::new(),
        }
    }
}

/// ⏳️ A producer's own non-negative finite count, or `0` — the Rust twin of the TypeScript
/// `world3dComputeNumber`.
fn world3d_status_number(value: Option<&Value>) -> f64 {
    value.and_then(Value::as_f64).filter(|number| number.is_finite() && *number >= 0.0).unwrap_or(0.0)
}

/// 🌍️ A producer's `{en, de}` phase label, honoured only when BOTH tongues carry text — a pair with
/// one empty half would make one language silently fall back to the other, which is exactly the
/// default-language rule this codebase forbids.
fn world3d_status_label(value: Option<&Value>) -> Option<(String, String)> {
    let row = value?.as_object()?;
    let en = row.get("en").and_then(Value::as_str).filter(|text| !text.is_empty())?;
    let de = row.get("de").and_then(Value::as_str).filter(|text| !text.is_empty())?;
    Some((en.to_string(), de.to_string()))
}

/// ⏳️ The ONE reader of `World3dScene.statusJson` on this target — the Rust twin of
/// `world3dComputeStatusV1`, answering the same shared `🛑️surface-controls/🔣️.json` fixture.
pub fn world3d_compute_status(status_json: Option<&str>) -> World3dComputeStatus {
    let Some(raw) = status_json.filter(|json| !json.is_empty()) else {
        return World3dComputeStatus::default();
    };
    let Ok(Value::Object(row)) = serde_json::from_str::<Value>(raw) else {
        return World3dComputeStatus::default();
    };
    let progress = row.get("progress").and_then(Value::as_object);
    let field = |name: &str| progress.and_then(|progress| progress.get(name));
    let cancel_action = row.get("cancelAction").and_then(Value::as_str).unwrap_or_default().to_string();
    let units_total = world3d_status_number(field("unitsTotal"));
    let units_done = if units_total == 0.0 { world3d_status_number(field("unitsDone")) } else { world3d_status_number(field("unitsDone")).min(units_total) };
    let declared_ratio = field("ratio").and_then(Value::as_f64).filter(|ratio| ratio.is_finite());
    World3dComputeStatus {
        computing: row.get("computing") == Some(&Value::Bool(true)),
        phase: row.get("phase").and_then(Value::as_str).filter(|phase| !phase.is_empty()).unwrap_or("idle").to_string(),
        phase_label: world3d_status_label(row.get("phaseLabel")),
        units_done,
        units_total,
        faces_done: world3d_status_number(field("facesDone")),
        faces_total: world3d_status_number(field("facesTotal")),
        in_flight: world3d_status_number(field("inFlight")),
        ratio: match declared_ratio {
            Some(ratio) => ratio.clamp(0.0, 1.0),
            None if units_total > 0.0 => units_done / units_total,
            None => 1.0,
        },
        cancellable: row.get("cancellable") == Some(&Value::Bool(true)) && !cancel_action.is_empty(),
        cancel_action,
        cancel_args: row
            .get("cancelArgs")
            .and_then(Value::as_object)
            .map(|args| args.iter().filter(|(_, value)| value.is_string() || value.as_f64().is_some_and(f64::is_finite)).map(|(key, value)| (key.clone(), value.clone())).collect())
            .unwrap_or_default(),
    }
}

//#endregion 🔀️ChromeParity
//#region 🪟️WindowPaneChrome
impl SurfaceState {
    pub fn world3d_host_id_for_window(&self, window_id: &str) -> Option<&str> {
        self.world3d_window_ids.iter().find_map(|(host_id, owner_window_id)| (owner_window_id == window_id).then_some(host_id.as_str()))
    }

    pub fn world3d_state_for_window(&self, window_id: &str) -> Option<&World3dState> {
        self.world3d_states.get(self.world3d_host_id_for_window(window_id)?)
    }

    pub fn world3d_state_for_window_mut(&mut self, window_id: &str) -> Option<&mut World3dState> {
        let host_id = self.world3d_host_id_for_window(window_id)?.to_string();
        self.world3d_states.get_mut(&host_id)
    }

    pub fn engine_host_id_for_window(&self, window_id: &str) -> Option<&str> {
        self.world3d_host_id_for_window(window_id)
            .or_else(|| self.node_graph_states.iter().find_map(|(host_id, surface)| (surface.window_id == window_id).then_some(host_id.as_str())))
            .or_else(|| self.tiled_map_states.iter().find_map(|(host_id, surface)| (surface.window_id == window_id).then_some(host_id.as_str())))
            .or_else(|| self.board2d_states.iter().find_map(|(host_id, surface)| (surface.window_id == window_id).then_some(host_id.as_str())))
    }
}

//#endregion 🪟️WindowPaneChrome
//#endregion ActionPanelAndUtilities

