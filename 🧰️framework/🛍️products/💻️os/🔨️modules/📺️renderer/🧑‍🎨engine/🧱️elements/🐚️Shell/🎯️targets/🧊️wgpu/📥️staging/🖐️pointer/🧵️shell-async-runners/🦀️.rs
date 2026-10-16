//! 🖐️ `shell-pointer` staging of `shell-async-runners`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
//#region 🧵️ShellDetached
/// 🧾️ The reserved sections one owed refresh reads besides its guest bodies: the app catalogue (only on
/// the instance's one claimed fetch), the window engagements, the window measures and the tool measures.
#[derive(Clone, Copy, Debug, Default)]
pub struct ShellRefreshSections {
    pub catalogue: bool,
    pub engagements: bool,
    pub measures: bool,
    pub tools: bool,
}

/// 🖼️ Reads everything one owed refresh names, in order, off the shell: every body render and every
/// section read is a guest turn, so the settle lane hands them off ([`ShellState::advance_owed_refresh`])
/// and the frame loop keeps building frames while they run; the refresh that consumes them then only
/// applies documents ([`ShellState::refresh_ui_rendered`]).
pub async fn render_refresh_detached(program: ProgramBridgeEntry, instance_id: u32, jobs: Vec<(String, String, ViewModel)>, sections: ShellRefreshSections, panel_view: ViewModel, view_state: ViewModel) -> ShellRenderedRefresh {
    let mut rendered = ShellRenderedRefresh::empty(instance_id);
    for (surface_id, body_key, view) in jobs {
        let mut effects = Vec::new();
        let document = program.render_with_document(instance_id, &surface_id, &body_key, &view, None, Some(&mut effects)).await;
        rendered.surfaces.insert(surface_id, (document, effects));
    }
    if sections.catalogue {
        let body_key = semio_framework::UiRefreshSection::Catalogue.body_key();
        rendered.catalogue = Some(program.render_with_document(instance_id, body_key, body_key, &panel_view, None, None).await);
    }
    if sections.engagements {
        rendered.engagements = Some(program.window_engagements_section(instance_id, &view_state).await);
    }
    if sections.measures {
        rendered.measures = Some(program.window_measures_section(instance_id, &view_state).await);
    }
    if sections.tools {
        rendered.tools = Some(program.tool_measures_section(instance_id, &view_state).await);
    }
    rendered
}

//#endregion 🧵️ShellDetached
//#endregion ShellTypes

