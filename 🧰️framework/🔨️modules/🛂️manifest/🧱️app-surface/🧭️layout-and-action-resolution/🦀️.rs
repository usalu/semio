//! 🧭️ Layout and action resolution of an app: the layout of a mode, the effective, missing, unresolved and invalid action arguments and the window and mode tool actions. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
/// 🧭️ Resolves the dock layout a mode should present.
pub fn resolve_layout_for_mode(app: &AppDefinition, mode_id: &str) -> Option<WindowLayout> {
    let mode = app.modes.iter().find(|mode| mode.id == mode_id)?;
    if let Some(layout_id) = &mode.layout_id {
        if let Some(named) = app.named_layouts.iter().find(|entry| entry.id == *layout_id) {
            return Some(named.layout.clone());
        }
    }
    app.default_layout.clone()
}

//#region 🔖️action-args
/// 🧮️ Computes the effective argument map for an action: for each declared arg, the staged value
/// if present, else its declared `default`, else omitted. Renderers stage edits locally and pass them
/// here; the contract enforcer ({@link VcsArtifactApp}) materializes defaults before dispatch so plugins
/// never re-implement default-filling.
///
/// 🌱️ `seed` carries a dialog's pre-seeded context args (e.g. a row-scoped `spaceId` that is never a
/// declared, editable form field, per `Effect::OpenDialog { args }`) through untouched: any `seed`
/// key that is not a declared arg id survives into the result unmodified, and a `seed` value for a
/// declared id that hasn't been staged yet acts as that field's initial value. A dialog with zero
/// declared `defs` (a plain confirm/cancel, e.g. `deleteSpace`) passes `seed`+`staged` through
/// wholesale — TS twin: {@link effectiveActionArgs} (`🧩️action-argument-resolution/🟦️.ts`).
pub fn effective_action_args(defs: &[ActionArgDef], staged: &DslValue, seed: Option<&DslValue>) -> DslValue {
    let seed_pairs: Vec<(String, DslValue)> = seed.and_then(DslValue::as_object).map(<[_]>::to_vec).unwrap_or_default();
    if defs.is_empty() {
        let mut effective = seed_pairs;
        if let Some(staged_pairs) = staged.as_object() {
            for (key, value) in staged_pairs {
                if let Some(existing) = effective.iter_mut().find(|(k, _)| k == key) {
                    existing.1 = value.clone();
                } else {
                    effective.push((key.clone(), value.clone()));
                }
            }
        }
        return DslValue::Object(effective);
    }
    let mut effective = seed_pairs;
    for def in defs {
        if let Some(value) = staged.get(&def.id) {
            if let Some(existing) = effective.iter_mut().find(|(k, _)| *k == def.id) {
                existing.1 = value.clone();
            } else {
                effective.push((def.id.clone(), value.clone()));
            }
        } else if effective.iter().any(|(k, _)| *k == def.id) {} else if let Some(default) = &def.default {
            effective.push((def.id.clone(), default.clone()));
        }
    }
    DslValue::Object(effective)
}

/// ❗️ Returns the ids of required args that are still unset in `effective`. "Unset" means absent,
/// `Null`, or an empty string (covers a blank Text/Select/IconSelect/ArtifactKind/SurfaceApp — the
/// latter two resolve to a `String` effective value exactly like `Select`, contract §C8.1); `false`,
/// `0`, and `[]` are valid values for Toggle/Number/Slider/Vec3 and never count as unset.
/// `Any` requires presence, allowing null and empty strings; a string with `min_len: Some(0)`
/// explicitly permits an empty value, such as document text or a root JSON pointer.
pub fn missing_required_args(defs: &[ActionArgDef], effective: &DslValue) -> Vec<String> {
    defs.iter()
        .filter(|def| def.required)
        .filter(|def| match effective.get(&def.id) {
            None => true,
            Some(DslValue::Null) => !matches!(def.schema, ArgSchema::Any),
            Some(DslValue::String(text)) => text.is_empty() && !matches!(def.schema, ArgSchema::Any | ArgSchema::String { min_len: Some(0), .. }),
            Some(_) => false,
        })
        .map(|def| def.id.clone())
        .collect()
}

/// 🔽️ Closed choices must belong to their current host-resolved option set.
pub fn action_arg_requires_choice(def: &ActionArgDef) -> bool {
    matches!(&def.schema, ArgSchema::String { options, format, .. } if !options.is_empty() || matches!(format, Some(ArgFormat::ArtifactKind { .. } | ArgFormat::SurfaceApp { .. })))
}

/// 🚫️ Returns supplied choices absent from the current catalog, including an unavailable catalog.
pub fn invalid_action_choice_args(defs: &[ActionArgDef], effective: &DslValue) -> Vec<String> {
    defs.iter().filter(|def| {
        let Some(value) = effective.get(&def.id) else { return false; };
        if matches!(value, DslValue::Null) || value.as_str() == Some("") || !action_arg_requires_choice(def) { return false; }
        matches!(&def.schema, ArgSchema::String { options, .. } if !options.iter().any(|option| value.as_str() == Some(option.value.as_str())))
    }).map(|def| def.id.clone()).collect()
}

/// 🛑️ Gates staged submission on required presence and exact current choice membership.
pub fn unresolved_action_args(defs: &[ActionArgDef], effective: &DslValue) -> Vec<String> {
    let unresolved: std::collections::HashSet<String> = missing_required_args(defs, effective).into_iter().chain(invalid_action_choice_args(defs, effective)).collect();
    defs.iter().filter(|def| unresolved.contains(&def.id)).map(|def| def.id.clone()).collect()
}

/// 🚦️ Whether an action is eligible to appear in a window's Actions panel — excludes the six
/// framework History actions (rendered by the History rail) and the injected `setActiveUtility`/
/// `setActiveTool` (internal View actions wired to the utility bar/tool panel, never the panel).
// 🚫️async: E1 transitive — the only consumer is `Iterator::filter` (external trait), which takes a
// sync closure; pure comparison, no I/O (R9).
fn action_is_panel_eligible(action: &ActionDefinition) -> bool {
    action.kind != ActionKind::History && action.id != SET_ACTIVE_UTILITY_ACTION_ID && action.id != SET_ACTIVE_TOOL_ACTION_ID
}

/// 🕹️ THE window action predicate — every [`ActionDefinition`] dispatchable in `window`, in
/// the order a shell must offer them: the window kind's OWN roster first, then every
/// [`AppDefinition::actions`] row that no window kind of this app claims for itself. An id declared
/// by ANY window kind is that window's authored declaration and is never re-offered from the app
/// roster. The plugin builder used to bake this union into every window kind by cloning, which made
/// a package descriptor grow as `apps × window kinds × actions` — see `AppDefinition::actions`.
/// TypeScript twin: `resolveWindowActions` (`🎯️action-bus/🟦️.ts`).
pub fn window_kind_actions<'a>(app: &'a AppDefinition, window: &'a WindowKindDefinition) -> Vec<&'a ActionDefinition> {
    let claimed: std::collections::BTreeSet<&str> = app.window_kinds.iter().flat_map(|kind| kind.actions.iter().map(|action| action.id.as_str())).collect();
    let mut seen: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    let mut resolved: Vec<&'a ActionDefinition> = Vec::with_capacity(window.actions.len() + app.actions.len());
    for action in window.actions.iter().chain(app.actions.iter().filter(|action| !claimed.contains(action.id.as_str()))) {
        if seen.insert(action.id.as_str()) {
            resolved.push(action);
        }
    }
    resolved
}

/// 📇️ Resolves the actions a window kind presents in its panel — [`window_kind_actions`]
/// minus the framework-only rail actions (the six History verbs and the injected
/// `setActiveUtility`/`setActiveTool`), preserving declaration order.
pub fn resolve_window_actions<'a>(app: &'a AppDefinition, window_kind: &'a WindowKindDefinition) -> Vec<&'a ActionDefinition> {
    window_kind_actions(app, window_kind).into_iter().filter(|action| action_is_panel_eligible(action)).collect()
}

/// 🛠️ Resolves the tools the active mode presents, in declared order — references into
/// `AppDefinition.tools` via `ModeDefinition.tools`. Unlike `resolve_window_actions`, unresolvable or
/// unreferenced tools have no orphan fallback: tools are opt-in per mode, not automatically shown
/// everywhere. Unresolvable refs are skipped (the builder validates them at construction time).
pub fn resolve_mode_tools<'a>(app: &'a AppDefinition, mode_id: &str) -> Vec<&'a ToolDefinition> {
    let Some(mode) = app.modes.iter().find(|mode| mode.id == mode_id) else {
        return Vec::new();
    };
    let mut resolved: Vec<&'a ToolDefinition> = Vec::new();
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for tool_ref in &mode.tools {
        if let Some(tool) = app.tools.iter().find(|tool| tool.id == tool_ref.as_str()) {
            if seen.insert(tool.id.as_str()) {
                resolved.push(tool);
            }
        }
    }
    resolved
}
//#endregion 🔖️action-args
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
