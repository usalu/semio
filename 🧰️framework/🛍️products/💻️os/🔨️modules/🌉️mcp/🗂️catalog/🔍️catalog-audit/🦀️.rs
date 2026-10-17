//! 🔍️ Catalog audit: the gesture and destructive word lexicons, the audit finding and the declaration and source audits. See [`super`].

use super::*;

//#region 🔖️Audit
/// 🖱️ Verb-id word runs that NAME a raw live-surface gesture route. A route whose id reads like one
/// of these is dispatched by a cursor, a key or an engagement draft — never by an agent choosing a
/// tool — but `manifest::derive_audience` cannot see that: a pointer handler that commits a real
/// operation is `ActionKind::Mutation`, structurally identical to a panel-dispatched `patchLayer`.
/// So the lexicon is not a classifier — [`audit_source`] uses it only to ask the question the
/// derivation cannot: "this id names an event; did anyone actually declare its audience?".
pub const GESTURE_ROUTE_WORDS: &[&str] = &[
    "pointerdown",
    "pointerup",
    "pointermove",
    "pointercancel",
    "pointerenter",
    "pointerleave",
    "mousedown",
    "mouseup",
    "mousemove",
    "doubleclick",
    "dblclick",
    "dragstart",
    "dragmove",
    "dragend",
    "dragover",
    "dragenter",
    "dragleave",
    "drop",
    "wheel",
    "keydown",
    "keyup",
    "keypress",
    "escape",
    "hover",
    "touchstart",
    "touchmove",
    "touchend",
    "gesture",
    "engagementinput",
    "engagementsubmit",
    "engagementcancel",
    "engagementabort",
    "engagementcommit",
    "commitdraft",
    "canceldraft",
    "updatedraft",
    "applyevents",
];

/// ⚠️ Verb-id word runs that NAME the delete/clear/reset class — the verbs a human wants to be asked
/// about before an agent commits them. Same contract as [`GESTURE_ROUTE_WORDS`]: the lexicon asks the
/// question, the declaration (`ActionDefinition::destructive` / `AppBuilder::action_destructive`)
/// answers it. Asked of document mutations and of shell verbs, whose side effects (a deleted space, a
/// removed member) live outside the document history entirely.
pub const DESTRUCTIVE_VERB_WORDS: &[&str] = &["delete", "remove", "clear", "discard", "purge", "wipe", "erase", "truncate", "reset", "replace", "overwrite"];

/// 📄️ Verb-id word runs that NAME a replacement of the whole document by supplied or example content.
/// Only a `Mutation` replaces the document — a shell verb of the same name navigates elsewhere — so
/// this lexicon is asked of mutations alone.
pub const DOCUMENT_REPLACE_WORDS: &[&str] = &["setactiveexample", "setfixturejson", "setspecjson", "setsnapshot", "commitdocument", "loaddocument", "setdocument"];

/// 💾️ Verb-id word runs that NAME a write to a path the user owns — an export, a download, a save.
/// Such a verb creates or overwrites a file outside the artifact's own history, so no undo reaches it;
/// published to an agent it must ask first. Asked of shell verbs and of palette view verbs (a
/// `View`-kind `exportDocument` renders and hands the file to the host just the same).
pub const USER_PATH_WRITE_WORDS: &[&str] = &["export", "download", "save"];

/// ✂️ Splits a verb id into its lowercase words at camel-case and separator boundaries —
/// `canvasPointerDown` → `["canvas", "pointer", "down"]`.
fn verb_id_words(id: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();
    for character in id.chars() {
        if character == '.' || character == '_' || character == '-' || character == ':' {
            if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
            continue;
        }
        if character.is_ascii_uppercase() && !current.is_empty() {
            words.push(std::mem::take(&mut current));
        }
        current.push(character.to_ascii_lowercase());
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}

/// 🔍️ The first lexicon entry that equals a CONTIGUOUS run of `id`'s words — so `dropOnPool` matches
/// `drop` while `addDropdown` (one word `dropdown`) matches nothing.
fn matching_lexicon_word(id: &str, lexicon: &[&'static str]) -> Option<&'static str> {
    let words = verb_id_words(id);
    for start in 0..words.len() {
        let mut run = String::new();
        for word in &words[start..] {
            run.push_str(word);
            if let Some(entry) = lexicon.iter().find(|candidate| **candidate == run) {
                return Some(entry);
            }
        }
    }
    None
}

/// 🚨️ One thing an agent-published capability gets wrong about itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CatalogAuditFinding {
    /// 🖱️ Published to agents, named like a live-surface gesture route, and nobody declared an
    /// audience — so `derive_audience` guessed `Agent` and the guess is unreviewed. Fix at the
    /// declaration: `ActionDefinition::input_event()` or `AppBuilder::action_audience(id, Input)`,
    /// or declare `Agent` explicitly if the name lies and it really is an intent verb.
    UndeclaredGestureRoute { capability_id: String, matched: &'static str },
    /// ⚠️ A `Mutation` or `Shell` verb published to agents whose id names the delete/clear/replace class but whose
    /// `effects.destructive` is false — so `ApprovalMode::WhenDestructive` never fires and an agent
    /// can discard the user's content without anyone being asked. Fix at the declaration:
    /// `ActionDefinition::destructive()` or `AppBuilder::action_destructive(id)`.
    UnmarkedDestructiveVerb { capability_id: String, matched: &'static str },
    /// 💾️ A shell or view verb published to agents whose id names an export/download/save but whose
    /// `effects.destructive` is false — so an agent writes to the user's disk without anyone being
    /// asked. Fix at the declaration: `AppBuilder::action_destructive(id)`.
    UnmarkedUserPathWrite { capability_id: String, matched: &'static str },
}

impl CatalogAuditFinding {
    pub fn capability_id(&self) -> &str {
        match self {
            CatalogAuditFinding::UndeclaredGestureRoute { capability_id, .. } | CatalogAuditFinding::UnmarkedDestructiveVerb { capability_id, .. } | CatalogAuditFinding::UnmarkedUserPathWrite { capability_id, .. } => capability_id,
        }
    }

    pub fn message(&self) -> String {
        match self {
            CatalogAuditFinding::UndeclaredGestureRoute { capability_id, matched } => {
                format!("{capability_id} reads as a `{matched}` gesture route but declares no audience — derive_audience published it to agents unreviewed")
            }
            CatalogAuditFinding::UnmarkedDestructiveVerb { capability_id, matched } => {
                format!("{capability_id} is a `{matched}`-class verb published to agents with effects.destructive = false — WhenDestructive never fires")
            }
            CatalogAuditFinding::UnmarkedUserPathWrite { capability_id, matched } => {
                format!("{capability_id} is a `{matched}` write to a user path published to agents with effects.destructive = false — no human is asked before the file is written")
            }
        }
    }
}

/// 🧭️ Audits one action/command declaration in place — shared by both walks below.
fn audit_declaration(capability_id: String, verb_id: &str, audience: manifest::CapabilityAudience, declared: bool, kind: manifest::ActionKind, destructive: bool, findings: &mut Vec<CatalogAuditFinding>) {
    if audience != manifest::CapabilityAudience::Agent {
        return;
    }
    if !declared {
        if let Some(matched) = matching_lexicon_word(verb_id, GESTURE_ROUTE_WORDS) {
            findings.push(CatalogAuditFinding::UndeclaredGestureRoute { capability_id: capability_id.clone(), matched });
        }
    }
    if destructive {
        return;
    }
    let replaces_document = if kind == manifest::ActionKind::Mutation { matching_lexicon_word(verb_id, DOCUMENT_REPLACE_WORDS) } else { None };
    let discards = if matches!(kind, manifest::ActionKind::Mutation | manifest::ActionKind::Shell) { matching_lexicon_word(verb_id, DESTRUCTIVE_VERB_WORDS) } else { None };
    if let Some(matched) = replaces_document.or(discards) {
        findings.push(CatalogAuditFinding::UnmarkedDestructiveVerb { capability_id, matched });
        return;
    }
    if matches!(kind, manifest::ActionKind::Shell | manifest::ActionKind::View) {
        if let Some(matched) = matching_lexicon_word(verb_id, USER_PATH_WRITE_WORDS) {
            findings.push(CatalogAuditFinding::UnmarkedUserPathWrite { capability_id, matched });
        }
    }
}

/// 🚨️ Every [`CatalogAuditFinding`] in a `CatalogSource`, sorted by capability id. Runs over the
/// SOURCE rather than a compiled `Catalog` because the one fact it needs — whether an audience was
/// DECLARED or merely derived — exists only in `ActionSemantics.audience: Option<…>`; `compile`
/// resolves that Option away. Empty means: every gesture-named route published to an agent was
/// looked at by a human, and every delete/clear/replace verb an agent can reach asks first.
pub fn audit_source(source: &CatalogSource) -> Vec<CatalogAuditFinding> {
    let mut findings: Vec<CatalogAuditFinding> = Vec::new();
    for descriptor in &source.descriptors {
        let plugin_id = descriptor.manifest.plugin_id.as_str();
        for app in descriptor.manifest.apps.iter() {
            let Ok(verbs) = app_action_verbs(app) else { continue };
            for (action, _) in verbs {
                if is_framework_injected_action(action) {
                    continue;
                }
                audit_declaration(
                    format!("{plugin_id}.{}.{}", app.id, action.id),
                    &action.id,
                    manifest::resolve_audience(action),
                    action.semantics.audience.is_some(),
                    action.kind,
                    action.semantics.effects.destructive,
                    &mut findings,
                );
            }
            for command in app.commands.iter().chain(app.modes.iter().flat_map(|mode| mode.commands.iter())) {
                audit_declaration(
                    format!("{plugin_id}.{}.cmd.{}", app.id, command.id),
                    &command.id,
                    manifest::resolve_command_audience(command),
                    command.semantics.audience.is_some(),
                    command.kind,
                    command.semantics.effects.destructive,
                    &mut findings,
                );
            }
        }
        for command in &descriptor.manifest.commands {
            audit_declaration(
                format!("{plugin_id}.cmd.{}", command.id),
                &command.id,
                manifest::resolve_command_audience(command),
                command.semantics.audience.is_some(),
                command.kind,
                command.semantics.effects.destructive,
                &mut findings,
            );
        }
    }
    findings.sort_by(|left, right| left.capability_id().cmp(right.capability_id()));
    findings
}
//#endregion 🔖️Audit
