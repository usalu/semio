//! ♻️ Framework dedup of the catalog: the framework-injected shell actions, the app action verbs and the framework capability definitions. See [`super`].

use super::*;

//#region 🔖️FrameworkDedup
/// 🕹️ The 7 framework-injected action ids that carry `ActionKind::View`/`🐚️Shell` (not
/// `History`/`Clipboard`/`Interaction`, which are already unambiguous by kind) — see
/// `🛂️manifest/🦀️.rs` `history_action_definitions`/`clipboard_action_definitions`/
/// `set_active_utility_action_definition`/etc.'s own doc comments for the full auto-injection story.
const FRAMEWORK_VIEW_SHELL_ACTION_IDS: [&str; 7] = ["setActiveUtility", "setActiveTool", "startIntroduction", "setHistoryCommandFilter", "noteShellCommand", manifest::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, manifest::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID];

/// 🕹️ Whether `action` is one of the 21 framework-auto-injected ids (`📓️luna-actions-audit.md` §1) —
/// these are walked ONCE per distinct id across every app (see `framework_capabilities`) rather than
/// once per plugin/app declaration, so they compile into `framework.*`, never `<plugin>.<app>.*`.
pub(super) fn is_framework_injected_action(action: &manifest::ActionDefinition) -> bool {
    matches!(action.kind, manifest::ActionKind::History | manifest::ActionKind::Clipboard | manifest::ActionKind::Interaction) || FRAMEWORK_VIEW_SHELL_ACTION_IDS.contains(&action.id.as_str())
}

/// 🎬️ One entry per distinct verb an app declares across all of its window kinds, in declaration
/// order, each paired with the FIRST window kind that declares it. An app's window kinds are views
/// onto one action registry — every plugin in the installed registry repeats its whole roster in
/// every window kind (`note` 54 verbs × 2 kinds, `architect` 39 × 5, `norm` 240 × 15) — so an action
/// id is a verb of the APP, not of a window, and collapsing them here is what makes
/// `<plugin>.<app>.<verb>` a unique capability id at all. The retained window kind is deliberately a
/// concrete declaring kind rather than a `"*"` marker: it is the `window_kind_id` a dispatch has to
/// address (`ActionAddress.window_kind_id`, checked against the guest's own
/// `registry.window_action`), which `"*"` could not satisfy. Two window kinds declaring the same id
/// with DIFFERENT definitions stay a real collision, reported as
/// [`CatalogError::DuplicateCapabilityId`].
///
/// 🗒️ App-SCOPE actions (`AppDefinition.actions`) are the other half of the surface, and walking
/// only `window_kinds` silently dropped every one of them: `🗒️note` declares all 48 of its verbs
/// here, so `capabilities_search "delete the selected blocks"` could not find a single note
/// capability while `🖍️draw` — which declares on its canvas window kind — worked. An app-scope
/// action belongs to no particular window, so it addresses `"*"`, the same marker
/// `framework_capabilities` already uses for the framework-injected verbs. A window kind that
/// also declares the id wins (it carries a dispatchable concrete `window_kind_id`), which is why
/// this loop runs second and skips what `seen` already holds instead of reporting a collision.
pub(super) fn app_action_verbs(app: &manifest::AppDefinition) -> Result<Vec<(&manifest::ActionDefinition, &str)>, CatalogError> {
    let mut verbs: Vec<(&manifest::ActionDefinition, &str)> = Vec::new();
    let mut seen: BTreeMap<&str, &manifest::ActionDefinition> = BTreeMap::new();
    for window_kind in app.window_kinds.iter() {
        for action in &window_kind.actions {
            match seen.get(action.id.as_str()) {
                Some(declared) if *declared == action => continue,
                Some(_) => return Err(CatalogError::DuplicateCapabilityId(format!("{}.{}", app.id, action.id))),
                None => {
                    seen.insert(action.id.as_str(), action);
                    verbs.push((action, window_kind.id.as_str()));
                }
            }
        }
    }
    for action in &app.actions {
        if seen.contains_key(action.id.as_str()) {
            continue;
        }
        seen.insert(action.id.as_str(), action);
        verbs.push((action, "*"));
    }
    Ok(verbs)
}

/// 🧰️ The framework-injected actions one app receives — the framework's own definitions and the actions of every
/// framework reserved contribution, never the copies a descriptor serialized, so a gateway built from this tree
/// publishes this tree's text.
pub(super) fn framework_action_definitions(app: &manifest::AppDefinition) -> Vec<manifest::ActionDefinition> {
    let mut actions = manifest::history_action_definitions();
    actions.extend(manifest::reserved_app_actions(&semio_framework::FRAMEWORK_RESERVED_CONTRIBUTIONS));
    actions.extend(manifest::clipboard_action_definitions());
    actions.extend(manifest::interaction_action_definitions(app));
    actions.extend(manifest::reserved_window_actions(&semio_framework::FRAMEWORK_RESERVED_CONTRIBUTIONS, app));
    actions.push(manifest::set_history_command_filter_action_definition());
    actions.push(manifest::note_shell_command_action_definition());
    actions.extend(manifest::document_transfer_action_definitions());
    if !app.utilities.is_empty() {
        actions.push(manifest::set_active_utility_action_definition());
    }
    if !app.tools.is_empty() {
        actions.push(manifest::set_active_tool_action_definition());
    }
    if app.introduction.is_some() {
        actions.push(semio_framework_onboarding::start_introduction_action_definition());
    }
    actions
}

/// 🕹️ Every framework-injected action reachable from `apps`, deduped by id (first occurrence wins —
/// every app resolves the identical `ActionDefinition` for a given framework id, since none of these
/// take app-specific data into their manifest shape) — compiled into `framework.<action.id>`
/// capabilities owned by `CapabilityOwner::Framework`.
pub(super) fn framework_capabilities(apps: &[&manifest::AppDefinition], locale: Locale, terminology: Terminology) -> BTreeMap<String, CapabilityDefinition> {
    let mut map = BTreeMap::new();
    for app in apps {
        for action in framework_action_definitions(app) {
            let id = format!("framework.{}", action.id);
            map.entry(id.clone()).or_insert_with(|| {
                capability_from_action(
                    &id,
                    CapabilityOwner::Framework,
                    None,
                    &action,
                    CapabilitySource::Action { plugin_id: "framework".into(), app_id: "framework".into(), window_kind_id: "*".into(), action_id: action.id.clone() },
                    locale,
                    terminology,
                )
            });
        }
    }
    map
}
//#endregion 🔖️FrameworkDedup
