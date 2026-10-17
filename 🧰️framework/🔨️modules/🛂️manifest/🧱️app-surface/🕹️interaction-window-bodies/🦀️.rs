//! 🕹️ Interaction window bodies of the app surface: the window bodies by domain, the panel leaf body keys and the interaction action definitions. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️Interaction
//#region 🔖️InteractionRefreshScope
/// 🕹️ Every declared window body key indexed by the interaction domain that window kind declares —
/// the schema-first half of [`interaction_verb_surface_scope`], read once when an app's action
/// registry is built rather than per dispatched hover.
pub fn interaction_window_bodies_by_domain(app: &AppDefinition) -> BTreeMap<String, Vec<String>> {
    let mut index: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for window in app.window_kinds.iter() {
        for interaction in &window.interactions {
            let bodies = index.entry(interaction.as_str().to_string()).or_default();
            if !bodies.iter().any(|body| body == &window.body_key) {
                bodies.push(window.body_key.clone());
            }
        }
    }
    index
}

/// 🌳️ Every declared panel LEAF body key, in declaration order, branches walked — the panel half of
/// [`interaction_verb_surface_scope`].
pub fn panel_leaf_body_keys(app: &AppDefinition) -> Vec<String> {
    fn walk(tab: &PanelTabDefinition, out: &mut Vec<String>) {
        if let Some(body_key) = &tab.body_key {
            if !out.iter().any(|entry| entry == body_key) {
                out.push(body_key.clone());
            }
        }
        for child in &tab.children {
            walk(child, out);
        }
    }
    let mut out = Vec::new();
    for tab in &app.panel_tabs {
        walk(tab, &mut out);
    }
    out
}
//#endregion 🔖️InteractionRefreshScope

/// 🕹️ The six framework-owned Interaction actions, auto-injected into any `AppDefinition` that
/// declares at least one `InteractionDefinition` — mirrors `history_action_definitions`/
/// `clipboard_action_definitions`, except conditional (like `set_active_utility_action_definition`)
/// rather than unconditional: returns `[]` when `app.interactions` is empty. `interactionSelect`/
/// `interactionHover` are the raw dispatch verbs renderers translate clicks/marquee/hover into
/// (never in the palette); `clearSelection`/`selectAll`/`setSelectionMode`/`setInteractionGranularity`
/// are user-facing and drive the per-domain Select controls.
pub fn interaction_action_definitions(app: &AppDefinition) -> Vec<ActionDefinition> {
    if app.interactions.is_empty() {
        return Vec::new();
    }
    let merge_options = vec![
        ActionArgOption::new("replace", LocalizedLabel::native("Replace", "Ersetzen")),
        ActionArgOption::new("additive", LocalizedLabel::native("Additive", "Additiv")),
        ActionArgOption::new("subtractive", LocalizedLabel::native("Subtractive", "Subtraktiv")),
        ActionArgOption::new("invertive", LocalizedLabel::native("Invertive", "Invertierend")),
        ActionArgOption::new("range", LocalizedLabel::native("Range", "Bereich")),
    ];
    let method_options =
        vec![ActionArgOption::new("pick", LocalizedLabel::native("Pick", "Auswahl")), ActionArgOption::new("rectangle", LocalizedLabel::native("Rectangle", "Rechteck")), ActionArgOption::new("lasso", LocalizedLabel::native("Lasso", "Lasso"))];
    let mode_options = vec![ActionArgOption::new("single", LocalizedLabel::native("Single", "Einzeln")), ActionArgOption::new("multiple", LocalizedLabel::native("Multiple", "Mehrfach"))];
    vec![
        ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(INTERACTION_SELECT_ACTION_ID, LocalizedLabel::native("Select", "Auswählen"), ActionKind::Interaction, "mouse-pointer") }.with_args([
            ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(),
            ActionArgDef::text("targets", LocalizedLabel::native("Targets", "Ziele")).required(),
            ActionArgDef::select("merge", LocalizedLabel::native("Merge", "Zusammenführen"), merge_options).required(),
            ActionArgDef::select("method", LocalizedLabel::native("Method", "Methode"), method_options).required(),
        ]),
        ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(INTERACTION_HOVER_ACTION_ID, LocalizedLabel::native("Hover", "Darüberfahren"), ActionKind::Interaction, "eye") }.with_args([
            ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(),
            ActionArgDef::text("channel", LocalizedLabel::native("Channel", "Kanal")).required(),
            ActionArgDef::text("targets", LocalizedLabel::native("Targets", "Ziele")).required(),
        ]),
        ActionDefinition { keys: Some("escape".into()), ..ActionDefinition::resumable_framework(CLEAR_SELECTION_ACTION_ID, LocalizedLabel::native("Clear Selection", "Auswahl aufheben"), ActionKind::Interaction, "mouse-pointer-2") },
        ActionDefinition { keys: Some("mod+a".into()), ..ActionDefinition::resumable_framework(SELECT_ALL_ACTION_ID, LocalizedLabel::native("Select All", "Alles auswählen"), ActionKind::Interaction, "maximize-2") },
        ActionDefinition::resumable_framework(SET_SELECTION_MODE_ACTION_ID, LocalizedLabel::native("Set Selection Mode", "Auswahlmodus festlegen"), ActionKind::Interaction, "sliders-horizontal")
            .with_args([ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(), ActionArgDef::select("mode", LocalizedLabel::native("Mode", "Modus"), mode_options).required()]),
        ActionDefinition::resumable_framework(SET_INTERACTION_GRANULARITY_ACTION_ID, LocalizedLabel::native("Set Granularity", "Granularität festlegen"), ActionKind::Interaction, "layers")
            .with_args([ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(), ActionArgDef::text("granularityId", LocalizedLabel::native("Granularity", "Granularität")).required()]),
    ]
}
//#endregion 🔖️Interaction
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
