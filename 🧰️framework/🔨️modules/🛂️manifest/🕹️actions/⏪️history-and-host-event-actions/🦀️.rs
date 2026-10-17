//! ⏪️ History, host event and clipboard actions: the framework-owned history verbs, the history command filter, the shell note, the host event input verb and the clipboard actions. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
/// 🕹️ The seven framework-owned History verbs, auto-injected into every `AppDefinition`: they move the applied stack or
/// the head of the artifact. A feature that reserves further history verbs contributes them through a
/// [`ReservedContribution`].
pub fn history_action_definitions() -> Vec<ActionDefinition> {
    vec![
        ActionDefinition { keys: Some("mod+z".into()), ..ActionDefinition::resumable_framework("undo", LocalizedLabel::native("Undo", "Rückgängig"), ActionKind::History, "undo-2") }
            .describe(LocalizedLabel::native("Reverts the most recent edit on the open artifact and moves its head back one revision.", "Macht die letzte Änderung am geöffneten Artefakt rückgängig und setzt den Kopf eine Revision zurück."))
            .use_when(["undo the last change", "take that back", "revert my last edit"]),
        ActionDefinition { keys: Some("mod+shift+z".into()), ..ActionDefinition::resumable_framework("redo", LocalizedLabel::native("Redo", "Wiederholen"), ActionKind::History, "redo-2") }
            .describe(LocalizedLabel::native("Re-applies the edit that was last undone and moves the artifact head forward one revision.", "Wendet die zuletzt rückgängig gemachte Änderung erneut an und setzt den Kopf eine Revision vor."))
            .use_when(["redo", "re-apply what I undid"]),
        ActionDefinition::resumable_framework("commitCheckpoint", LocalizedLabel::native("Commit Checkpoint", "Checkpoint festschreiben"), ActionKind::History, "git-commit")
            .describe(LocalizedLabel::native("Writes a named checkpoint into the artifact's history so a later edit can be reverted back to exactly this state.", "Schreibt einen benannten Checkpoint in die Historie des Artefakts, auf den später zurückgesetzt werden kann."))
            .use_when(["save a checkpoint", "mark this state"]),
        ActionDefinition::resumable_framework("createAlternative", LocalizedLabel::native("Create Alternative", "Alternative erstellen"), ActionKind::History, "git-branch")
            .describe(LocalizedLabel::native("Branches the artifact's history into a new named alternative that can be edited without disturbing the current one. Only this viewer's head moves; every collaborator stays on theirs.", "Verzweigt die Historie des Artefakts in eine neue benannte Alternative, die bearbeitet werden kann, ohne die aktuelle zu stören. Nur der Kopf dieses Betrachters wechselt; jeder Mitarbeiter bleibt auf seinem."))
            .use_when(["try a variant", "branch this design"]),
        ActionDefinition::resumable_framework(SWITCH_ALTERNATIVE_ACTION_ID, LocalizedLabel::native("Switch Alternative", "Alternative wechseln"), ActionKind::History, "git-branch")
            .describe(LocalizedLabel::native(
                "Moves this viewer's head onto another existing alternative. Collaborators stay on theirs. This viewer sees that alternative's history, including the history edits kept as that alternative, and not those of the alternative they left.",
                "Setzt den Kopf dieses Betrachters auf eine andere vorhandene Alternative. Mitarbeiter bleiben auf ihrem. Dieser Betrachter sieht deren Verlauf einschließlich der als diese Alternative behaltenen Verlaufsbearbeitungen und nicht den der verlassenen Alternative.",
            ))
            .use_when(["switch to the other variant", "go back to the original history", "compare the edited history with the original"])
            .with_args([ActionArgDef::text(SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID, LocalizedLabel::native("Alternative", "Alternative"))
                .describe(LocalizedLabel::native("Id of an existing alternative of this artifact, as the history lists it.", "Kennung einer vorhandenen Alternative dieses Artefakts, wie der Verlauf sie auflistet."))
                .required()]),
        ActionDefinition::resumable_framework("checkoutCheckpoint", LocalizedLabel::native("Checkout Checkpoint", "Checkpoint auschecken"), ActionKind::History, "git-branch")
            .describe(LocalizedLabel::native("Restores this viewer's view of the artifact to a previously committed checkpoint by id. Collaborators stay where they are.", "Setzt die Ansicht dieses Betrachters auf einen zuvor festgeschriebenen Checkpoint zurück. Mitarbeiter bleiben, wo sie sind."))
            .use_when(["go back to that checkpoint"]),
        ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(REVERT_TO_COMMAND_ACTION_ID, LocalizedLabel::native("Revert to Command", "Auf Befehl zurücksetzen"), ActionKind::History, "clock") }
            .describe(LocalizedLabel::native("Rewinds the artifact to the state it had just after one numbered entry of the session command log.", "Setzt das Artefakt auf den Zustand direkt nach einem nummerierten Eintrag des Sitzungsprotokolls zurück."))
            .with_args([ActionArgDef::number("entrySeq", LocalizedLabel::native("Entry", "Eintrag")).required()]),
    ]
}

/// 🎚️ The framework-owned action id apps dispatch to change the history panel's mutations
/// filter — auto-injected unconditionally.
pub const SET_HISTORY_COMMAND_FILTER_ACTION_ID: &str = "setHistoryCommandFilter";

/// 🎚️ The `setHistoryCommandFilter` option (and `HistoryPatch.commandFilter` value) that shows every row.
pub const HISTORY_COMMAND_FILTER_ALL: &str = "all";

/// 🎚️ The option that hides the rows of document mutations.
pub const HISTORY_COMMAND_FILTER_WITHOUT_MUTATIONS: &str = "withoutMutations";

/// 🎚️ The option that shows only the rows of document mutations.
pub const HISTORY_COMMAND_FILTER_ONLY_MUTATIONS: &str = "onlyMutations";

/// 🎚️ The framework-injected `setHistoryCommandFilter` View action (never in the palette):
/// switches the history panel's tri-state operations filter. Ephemeral UI state, never an
/// operation — `ActionKind::View`. Arg id is `"value"` (not `"filter"`) — a top-level `UiNode::Select`
/// always dispatches its picked option merged into `args` under the `"value"` key (both renderers'
/// `Select` interpreters hardcode that key; see `with_item_value_arg` in ui_wgpu).
pub fn set_history_command_filter_action_definition() -> ActionDefinition {
    let options = vec![
        ActionArgOption::new(HISTORY_COMMAND_FILTER_ALL, LocalizedLabel::native("All", "Alle")),
        ActionArgOption::new(HISTORY_COMMAND_FILTER_WITHOUT_MUTATIONS, LocalizedLabel::native("Without Mutations", "Ohne Mutationen")),
        ActionArgOption::new(HISTORY_COMMAND_FILTER_ONLY_MUTATIONS, LocalizedLabel::native("Only Mutations", "Nur Mutationen")),
    ];
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(SET_HISTORY_COMMAND_FILTER_ACTION_ID, LocalizedLabel::native("Set History Filter", "Verlaufsfilter festlegen"), ActionKind::View, "list") }
    .describe(LocalizedLabel::native("Filters the history panel to all entries, only document mutations, or everything except document mutations; the document is not changed.", "Filtert den Verlaufsbereich auf alle Einträge, nur Dokumentmutationen oder alles außer Dokumentmutationen; das Dokument wird nicht verändert."))
    .with_args([ActionArgDef::select(
        "value",
        LocalizedLabel::native("Filter", "Filter"),
        options,
    )
    .default_value(&HISTORY_COMMAND_FILTER_ALL)])
}

/// 🗒️ The framework-owned action id apps dispatch to note a shell effect (navigate, export,
/// spawn, …) into the session command log without any document mutation — mirrors
/// `SET_HISTORY_COMMAND_FILTER_ACTION_ID`'s auto-injected-constant pattern.
pub const NOTE_SHELL_COMMAND_ACTION_ID: &str = "noteShellCommand";

//#region 🔖️HostEvent
/// 📨️ The host-forwarded window fact `hostEvent{windowId, kind}`: the runtime answers it with the app's own typed
/// host-event command for that window (`ArtifactApp::host_event`), so an open gesture there ends without a trace.
pub const HOST_EVENT_ACTION_ID: &str = "hostEvent";

/// 🪟️ `hostEvent`'s window instance argument.
pub const HOST_EVENT_ARG_WINDOW_ID: &str = "windowId";

/// 🏷️ `hostEvent`'s event kind argument: [`HOST_EVENT_KIND_BLUR`], [`HOST_EVENT_KIND_CAPTURE_LOST`] or [`HOST_EVENT_KIND_RETIRING`].
pub const HOST_EVENT_ARG_KIND: &str = "kind";

/// 🌫️ The window lost keyboard focus.
pub const HOST_EVENT_KIND_BLUR: &str = "blur";

/// 🖐️ The window lost the pointer capture of a gesture (the release never reached it).
pub const HOST_EVENT_KIND_CAPTURE_LOST: &str = "captureLost";

/// 🚪️ The window is about to close.
pub const HOST_EVENT_KIND_RETIRING: &str = "retiring";

/// 📨️ The framework-injected `hostEvent` input verb (never in the palette, never an agent verb, never a history row):
/// both hosts forward a window's blur and pointer-capture loss through it, and may announce its closing.
pub fn host_event_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(HOST_EVENT_ACTION_ID, LocalizedLabel::native("Window Event", "Fensterereignis"), ActionKind::Interaction, "hand") }
        .describe(LocalizedLabel::native(
            "Tells the program that a window lost focus, lost the pointer capture of a gesture or is about to close, so an open gesture there ends without a trace.",
            "Teilt dem Programm mit, dass ein Fenster den Fokus oder die Zeigererfassung einer Geste verloren hat oder gleich geschlossen wird, damit eine offene Geste dort spurlos endet.",
        ))
        .with_args([
            ActionArgDef::text(HOST_EVENT_ARG_WINDOW_ID, LocalizedLabel::native("Window", "Fenster")).required(),
            ActionArgDef::select(
                HOST_EVENT_ARG_KIND,
                LocalizedLabel::native("Event", "Ereignis"),
                vec![
                    ActionArgOption::new(HOST_EVENT_KIND_BLUR, LocalizedLabel::native("Window lost focus", "Fenster hat den Fokus verloren")),
                    ActionArgOption::new(HOST_EVENT_KIND_CAPTURE_LOST, LocalizedLabel::native("Pointer capture lost", "Zeigererfassung verloren")),
                    ActionArgOption::new(HOST_EVENT_KIND_RETIRING, LocalizedLabel::native("Window closing", "Fenster wird geschlossen")),
                ],
            )
            .required(),
        ])
}
//#endregion 🔖️HostEvent

/// 🗒️ The framework-injected `noteShellCommand` Shell action (never in the palette): records a
/// shell-kind effect that already happened into the session command log, for effects dispatched
/// outside the normal `ActionDescriptor` path. `commandId` and `label` are required; `detail` is an
/// optional free-text elaboration shown in the history panel. A note is an UNDO TARGET only when the
/// shell also declares `inverseCommandId` (plus optional `inverseArgs`): chrome whose prior state the
/// shell never captured has no computable inverse, so it stays a log row the document undo ledger
/// steps straight over — see `VcsArtifactApp::dispatch_chrome_history_action`.
pub fn note_shell_command_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(NOTE_SHELL_COMMAND_ACTION_ID, LocalizedLabel::native("Note Shell Command", "Shell-Befehl vermerken"), ActionKind::Shell, "book-open") }
        .chrome()
        .describe(LocalizedLabel::native("Records a shell effect that already happened (navigation, export, spawn) in the session command log; the document is not changed.", "Vermerkt eine bereits erfolgte Shell-Wirkung (Navigation, Export, Start) im Sitzungsprotokoll; das Dokument wird nicht verändert."))
        .with_args([
        ActionArgDef::text("commandId", LocalizedLabel::native("Command", "Befehl")).required(),
        ActionArgDef::any("label", LocalizedLabel::native("Label", "Bezeichnung"))
            .describe(LocalizedLabel::native(
                "The row's label in every locale: an object {en, de} (or a full localized label); plain text is kept as locale-invariant data, such as a file name.",
                "Die Bezeichnung der Zeile in jeder Sprache: ein Objekt {en, de} (oder eine vollständige lokalisierte Bezeichnung); reiner Text bleibt sprachunabhängige Angabe, etwa ein Dateiname.",
            ))
            .required(),
        ActionArgDef::text("detail", LocalizedLabel::native("Detail", "Detail")),
    ])
}

//#region 🔖️Clipboard
/// 🕹️ The three framework-owned Clipboard actions, auto-injected into every `AppDefinition` —
/// mirrors `history_action_definitions`. `paste` carries a staged `anchoring` choice (defaulting to
/// `original`) plus an optional `position` override, both consumed as a `PastePlacement`.
pub fn clipboard_action_definitions() -> Vec<ActionDefinition> {
    let anchoring_options = vec![
        ActionArgOption::new("original", LocalizedLabel::native("Original", "Original")),
        ActionArgOption::new("middle", LocalizedLabel::native("Middle", "Mitte")),
        ActionArgOption::new("centroid", LocalizedLabel::native("Centroid", "Schwerpunkt")),
        ActionArgOption::new("bottomLeft", LocalizedLabel::native("Bottom Left", "Unten links")),
        ActionArgOption::new("bottomRight", LocalizedLabel::native("Bottom Right", "Unten rechts")),
        ActionArgOption::new("topLeft", LocalizedLabel::native("Top Left", "Oben links")),
        ActionArgOption::new("topRight", LocalizedLabel::native("Top Right", "Oben rechts")),
    ];
    vec![
        ActionDefinition { keys: Some("mod+c".into()), ..ActionDefinition::resumable_framework("copy", LocalizedLabel::native("Copy", "Kopieren"), ActionKind::Clipboard, "copy") }
            .describe(LocalizedLabel::native("Copies the current selection of the focused window onto the workspace clipboard.", "Kopiert die aktuelle Auswahl des fokussierten Fensters in die Zwischenablage.")),
        ActionDefinition { keys: Some("mod+x".into()), ..ActionDefinition::resumable_framework("cut", LocalizedLabel::native("Cut", "Ausschneiden"), ActionKind::Clipboard, "scissors") }
            .describe(LocalizedLabel::native("Removes the current selection from the artifact and puts it on the workspace clipboard.", "Entfernt die aktuelle Auswahl aus dem Artefakt und legt sie in der Zwischenablage ab.")),
        ActionDefinition { keys: Some("mod+v".into()), ..ActionDefinition::resumable_framework("paste", LocalizedLabel::native("Paste", "Einfügen"), ActionKind::Clipboard, "clipboard") }
            .describe(LocalizedLabel::native("Inserts the workspace clipboard's contents into the artifact at the chosen anchoring.", "Fügt den Inhalt der Zwischenablage an der gewählten Verankerung in das Artefakt ein."))
            .with_args([
            ActionArgDef::select("anchor", LocalizedLabel::native("Anchoring", "Verankerung"), anchoring_options).default_value(&"original"),
            ActionArgDef::vector("position", LocalizedLabel::native("Position", "Position"), 3),
        ]),
    ]
}
//#endregion 🔖️Clipboard
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
