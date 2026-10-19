//! 📜️ Kernel history wire, history and fault notices, and the manifest fault notice definitions they are localized against.

use semio_framework_diagnostic::Fault;
use semio_framework_diagnostic::Severity;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

//#region 🔖️HistoryWire
/// 🚦️ `Severity` crosses the serde half of the history wire as its bare camelCase name, like its value form.
mod history_severity_serde {
    use super::Severity;
    use serde::{Deserialize, Deserializer, Serializer};

    fn name(severity: Severity) -> &'static str {
        match severity {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
            Severity::Fatal => "fatal",
        }
    }

    fn parse<E: serde::de::Error>(text: &str) -> Result<Severity, E> {
        match text {
            "info" => Ok(Severity::Info),
            "warning" => Ok(Severity::Warning),
            "error" => Ok(Severity::Error),
            "fatal" => Ok(Severity::Fatal),
            other => Err(E::custom(format!("unknown severity {other:?}"))),
        }
    }

    pub fn serialize<S: Serializer>(severity: &Severity, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(name(*severity))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Severity, D::Error> {
        parse(&String::deserialize(deserializer)?)
    }

    /// 🫥️ The optional twin: absent or `null` is `None`.
    pub mod option {
        use super::Severity;
        use serde::{Deserialize, Deserializer, Serializer};

        pub fn serialize<S: Serializer>(severity: &Option<Severity>, serializer: S) -> Result<S::Ok, S::Error> {
            match severity {
                Some(severity) => serializer.serialize_some(super::name(*severity)),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Severity>, D::Error> {
            Option::<String>::deserialize(deserializer)?.map(|text| super::parse(&text)).transpose()
        }
    }
}

/// 🛠️ The committed tool transaction a history row's edit carries (`MutationMeta.transaction`): its id and the
/// `<appId>#<toolId>` tool that authored it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryTransaction {
    pub id: String,
    pub tool: String,
}

/// 📨️ One outcome message of a history mutation row, the wire mirror of `MutationMessage`: `code` is one of the frozen
/// `mutation.*` codes a host localizes, `message` is English prose for logs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryMutationMessage {
    #[serde(with = "history_severity_serde")]
    pub level: Severity,
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub op_index: Option<u32>,
}

/// ✏️ One applied mutation of a history row: its replica-independent id, applied position and index inside its edit,
/// its localized kind label, its outcome (the time-travel replay's while a session holds a report, else the durable
/// one) and its editing state. `editable` = the op has an input schema and emits no foreign steps; `withdrawable` = the
/// store's supersede law lets this actor withdraw it and it is not withdrawn yet (design §22.1); `pending` = it is
/// downstream of the mutation being edited and not applied in the preview; `edited` = the session holds a draft for it;
/// `introduced` = the session's replay outcome carries a message (level and code) its pre-edit durable outcome does not,
/// so a host marks what the edit made new (design §16.5); `store` = the composed member store that holds it
/// (`<slot>/<childId>`, design §12), absent for the document's own.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryMutationEntry {
    pub mutation_id: String,
    pub position: u32,
    pub op_index: u32,
    pub label: semio_framework_ui_locale::LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "history_severity_serde::option")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub worst: Option<Severity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<HistoryMutationMessage>,
    #[serde(default)]
    #[value(default)]
    pub superseded: bool,
    #[serde(default)]
    #[value(default)]
    pub withdrawn: bool,
    #[serde(default)]
    #[value(default)]
    pub editable: bool,
    #[serde(default)]
    #[value(default)]
    pub withdrawable: bool,
    #[serde(default)]
    #[value(default)]
    pub pending: bool,
    #[serde(default)]
    #[value(default)]
    pub edited: bool,
    #[serde(default)]
    #[value(default)]
    pub introduced: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<String>,
}

/// 🚦️ The stage of a live history-edit session (an inactive session is an absent `HistoryPatch.timeTravel`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum HistoryTimeTravelStage {
    #[default]
    Editing,
    Replaying,
    Reviewing,
    Choosing,
    Finalizing,
}

/// 🧭️ What a reviewing session shows: nothing accepted (the committed head), drafts awaiting a replay (after a cancelled or
/// faulted one), a report that blocks finalizing, or a report ready to finalize.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum HistoryTimeTravelReview {
    NoChanges,
    NeedsReplay,
    Blocked,
    Ready,
}

/// ⛔️ The first mutation, in replay order, whose outcome blocks finalizing a history edit: its id and the composed member
/// store that holds it (`<slot>/<childId>`, design §12; absent for the document's own) — exactly the arguments of
/// `historyEditBegin`, so a host's "Next problem" control dispatches them and never computes them (design §22.2).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryTimeTravelProblem {
    pub mutation_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<String>,
}

/// ⏪️ The live history-edit session of one instance, as every host renders its band: identity and generation (every
/// `historyEdit*` verb may echo `generation`; a stale one is `timeTravel.stale`), stage, the edited mutation and its
/// label, replay progress, the report's worst severity, whether that report blocks finalizing, the last fault code, how
/// many drafts are accepted, what a review shows, whether a replay can be run again and the first mutation that blocks
/// finalizing.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryTimeTravel {
    pub session_id: String,
    pub generation: u32,
    pub stage: HistoryTimeTravelStage,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target_label: Option<semio_framework_ui_locale::LocalizedLabel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub done: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub processed: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "history_severity_serde::option")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub worst: Option<Severity>,
    #[serde(default)]
    #[value(default)]
    pub blocking: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub accepted_count: u32,
    /// 🧭️ While reviewing: what the review shows (absent in every other stage).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<HistoryTimeTravelReview>,
    /// 🔁️ Whether `historyEditRerun` would start a replay now (a cancelled or faulted replay left drafts to replay).
    #[serde(default)]
    #[value(default)]
    pub rerunnable: bool,
    /// 👉️ The first mutation whose replay outcome blocks finalizing; present exactly while `blocking`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub next_problem: Option<HistoryTimeTravelProblem>,
}

/// 🧾️ One host-projectable row in the session command timeline, one per committed tool transaction (else per edit;
/// rows without an edit are session commands such as undo or a noted shell command). Hosts key rows by `edit_id`,
/// falling back to `seq` for a row without an edit. The payload is deliberately presentation-neutral: the host owns
/// windowing and retains entries beyond any visible range.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryEntry {
    pub seq: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edit_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<HistoryTransaction>,
    /// ✏️ The `Supersede` history transition this row is — a history edit, or the undo or redo of one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transition_id: Option<String>,
    /// 🖋️ The actor who authored this row's edit or history transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    pub action_id: String,
    /// 🏷️ Every shell locale's text for this row, resolved by the renderer against the
    /// active locale — never a pre-resolved string, so switching the shell locale re-renders the
    /// whole ledger instead of leaving already-logged rows in the locale they were dispatched in.
    pub label: semio_framework_ui_locale::LocalizedLabel,
    pub kind: String,
    pub timestamp: String,
    /// 📜️ The newest forward operations of this row's edit, newest last — a bounded preview; `op_count` says how many the
    /// edit holds.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub op_lines: Vec<String>,
    /// 🔢️ Forward operations of this row's edit; more than `op_lines` holds means the preview omits the older ones.
    #[serde(default)]
    #[value(default)]
    pub op_count: u64,
    #[serde(default)]
    #[value(default)]
    pub applied: bool,
    #[serde(default)]
    #[value(default)]
    pub revertible: bool,
    #[serde(default = "history_entry_count")]
    #[value(default = "history_entry_count")]
    pub count: u32,
    /// 🚦️ The worst severity over `mutations`.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "history_severity_serde::option")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub worst: Option<Severity>,
    /// ✏️ The edit's applied mutations in op order, empty for a row without an applied edit.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub mutations: Vec<HistoryMutationEntry>,
}

// 🚫️async: E4 fn-pointer slot
fn history_entry_count() -> u32 {
    1
}

impl HistoryEntry {
    /// 🔑️ The key a host folds this row under: `edit:<editId>`, else `transition:<transitionId>`, else `seq:<seq>`.
    /// TypeScript twin `historyEntryKey`.
    pub fn key(&self) -> String {
        match (&self.edit_id, &self.transition_id) {
            (Some(edit_id), _) => format!("edit:{edit_id}"),
            (None, Some(transition_id)) => format!("transition:{transition_id}"),
            (None, None) => format!("seq:{}", self.seq),
        }
    }
}

/// 📡️ Which history change replays before this replica adopts it ([`HistoryReprojection`]): another replica's change
/// (`remote`), this replica's own deferred history step (`step`: an interior undo or redo, a checkout, an alternative
/// switch) or a whole-document load (`load`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum HistoryReprojectionKind {
    #[default]
    Remote,
    Step,
    Load,
}

/// 📡️ The replay a history change needs before this replica adopts it, stepped per reactor turn (design §16.6, gap N17),
/// by `kind`: replayed operations of the total, whether the user paused a remote one (`historyEditCancelReplay` while no
/// session is open; `historyEditRerun` resumes — cancelling a step or a load drops it with zero trace instead) and the code
/// of a refused adoption.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryReprojection {
    pub done: u32,
    pub total: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub processed: Option<u32>,
    #[serde(default)]
    #[value(default)]
    pub kind: HistoryReprojectionKind,
    #[serde(default)]
    #[value(default)]
    pub paused: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<String>,
}

/// 🧾️ Ordered history delta returned in the same response as an accepted interaction.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryPatch {
    /// 🖱️ Monotonic command-log cursor after applying this patch.
    pub cursor: u64,
    /// 📶️ Upserts, ordered newest-first to match the logical history projection.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub upserts: Vec<HistoryEntry>,
    #[serde(default)]
    #[value(default)]
    pub can_undo: bool,
    #[serde(default)]
    #[value(default)]
    pub can_redo: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub current_checkpoint_id: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub command_filter: String,
    /// ⏪️ The live history-edit session; absent while none is open. Every patch carries the current status in full.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub time_travel: Option<HistoryTimeTravel>,
    /// 📡️ The history change (remote, or this replica's own step) waiting for its replay; absent while none waits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub reprojection: Option<HistoryReprojection>,
    /// 🔢️ How many edits the document's history holds — the `{n}` of the `history.full` notice, carried as data so no host
    /// parses it from a fault message.
    #[serde(default)]
    #[value(default)]
    pub edit_count: u32,
}

/// 📢️ Framework-owned EN/DE notices of the store and runtime refusals a dispatch answers outside a history-edit session
/// (an open tool transaction, an exhausted or still replaying history, a whole-document load in flight, a history step whose
/// replay would leave errors, a withdrawal of a step that also changed other documents), by fault code — every shell shows
/// these bytes; `{n}` is the edit count a refusal names. Fixture `🧫️fixtures/🧫️history-notices/🔣️.json`; TS twin
/// `HISTORY_NOTICE_LABELS`.
pub const HISTORY_NOTICE_LABELS: [(&str, &str, &str); 9] = [
    ("toolTransaction.open", "A tool is still recording — finish or cancel it first.", "Ein Werkzeug zeichnet noch auf — zuerst abschließen oder abbrechen."),
    ("toolTransaction.unknown", "The tool's recording has already ended.", "Die Aufzeichnung des Werkzeugs ist bereits beendet."),
    ("history.full", "This document's history is full ({n} edits).", "Der Verlauf dieses Dokuments ist voll ({n} Bearbeitungen)."),
    ("history.replaying", "History is still replaying — wait for it or cancel it first.", "Der Verlauf wird noch neu angewendet — abwarten oder zuerst abbrechen."),
    ("document.loading", "The document is still loading — wait for it or cancel it first.", "Das Dokument wird noch geladen — abwarten oder zuerst abbrechen."),
    ("pure.history-unavailable", "This is a head-only evaluation without history — open the document in a live instance to use its history.", "Dies ist eine Auswertung nur des aktuellen Stands ohne Verlauf — für den Verlauf das Dokument in einer laufenden Instanz öffnen."),
    ("history.step-blocked", "Later mutations would end with errors — fix or withdraw them first.", "Spätere Mutationen würden mit Fehlern enden — zuerst beheben oder zurückziehen."),
    ("history.unit-spans-documents", "This step also changed other documents, so it cannot be withdrawn on its own yet.", "Dieser Schritt hat auch andere Dokumente geändert und lässt sich daher noch nicht einzeln zurückziehen."),
    ("history.ledger-not-replayable", "This document's history does not replay without errors yet — fix or withdraw the mutations that end with errors, then check in again.", "Der Verlauf dieses Dokuments lässt sich noch nicht fehlerfrei neu anwenden — die Mutationen mit Fehlern beheben oder zurückziehen, dann erneut einchecken."),
];

/// 🔔️ The `(en, de)` notice of a history-lane refusal `code` ([`HISTORY_NOTICE_LABELS`]); `None` for any other code.
pub fn history_notice(code: &str) -> Option<(&'static str, &'static str)> {
    HISTORY_NOTICE_LABELS.iter().find(|(known, _, _)| *known == code).map(|(_, en, de)| (*en, *de))
}

//#endregion 🔖️HistoryWire
//#region 🔖️FaultNotices

/// 📣️ The localized notice of a refused dispatch (design §20.12): the code it was told under and its text in one shell
/// axis pair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaultNotice {
    pub code: String,
    pub text: String,
}

/// 🏢️ Framework-owned EN/DE notices of the refusal codes any app raises under a framework namespace (`app.command.*`,
/// `mutation.*`, design §20.12) and of the host's guest admission (`plugin.channel-mismatch`, `{guest}`/`{host}`) — a shell tells them with the fault's own severity, before the app's own table. Fixture
/// `🧫️fixtures/🧫️framework-notices/🔣️.json`; TS twin `FRAMEWORK_FAULT_NOTICE_LABELS`.
pub const FRAMEWORK_FAULT_NOTICE_LABELS: [(&str, &str, &str); 87] = [
    ("app.command.unsupported", "This action is not available here.", "Diese Aktion ist hier nicht verfügbar."),
    ("app.command.invalid", "This action is not valid.", "Diese Aktion ist ungültig."),
    ("app.command.invalid-args", "The action's input is not valid.", "Die Eingabe der Aktion ist ungültig."),
    ("app.command.invalid-payload", "The action's data is not valid.", "Die Daten der Aktion sind ungültig."),
    ("app.command.targets-required", "Select at least one item first.", "Zuerst mindestens ein Element auswählen."),
    ("app.command.kind-unavailable", "This kind of item is not available.", "Diese Art von Element ist nicht verfügbar."),
    ("app.command.target-in-use", "The item is still in use.", "Das Element wird noch verwendet."),
    ("app.command.tool-mismatch", "This action does not belong to the active tool.", "Diese Aktion gehört nicht zum aktiven Werkzeug."),
    ("toolTransaction.closed", "The tool's recording had already ended — this step was not recorded.", "Die Aufzeichnung des Werkzeugs war bereits beendet — dieser Schritt wurde nicht aufgezeichnet."),
    ("toolTransaction.unclosed", "The tool stopped without finishing its recording — this step was not recorded.", "Das Werkzeug hat angehalten, ohne seine Aufzeichnung abzuschließen — dieser Schritt wurde nicht aufgezeichnet."),
    ("toolTransaction.slot-poisoned", "The tool could not continue this gesture — release and try again.", "Das Werkzeug konnte diese Geste nicht fortsetzen — loslassen und erneut versuchen."),
    ("mutation.target-missing", "The target no longer exists.", "Das Ziel existiert nicht mehr."),
    ("mutation.target-mismatch", "The change does not fit the target's current state.", "Die Änderung passt nicht zum aktuellen Zustand des Ziels."),
    ("mutation.too-large", "This change is too large to record at once — split it into smaller steps.", "Diese Änderung ist zu groß, um sie auf einmal aufzuzeichnen — in kleinere Schritte aufteilen."),
    ("plugin.media.schema-mismatch", "This input is a {found} document, but only {expected} documents can be loaded here.", "Diese Eingabe ist ein {found}-Dokument, hier lassen sich aber nur {expected}-Dokumente laden."),
    ("plugin.channel-mismatch", "This plugin was built for app channel {guest}, but this app speaks app channel {host} — rebuild the plugin.", "Dieses Plugin wurde für App-Kanal {guest} gebaut, diese App spricht aber App-Kanal {host} — Plugin neu bauen."),
    ("history-filter.unknown", "This history filter is not known — choose one the history panel offers.", "Dieser Verlaufsfilter ist unbekannt — einen im Verlaufsbereich angebotenen wählen."),
    ("window-transient.window-required", "This needs an open window — focus a window first.", "Dafür wird ein offenes Fenster benötigt — zuerst ein Fenster fokussieren."),
    ("window-transient.window-stale", "The window is no longer open.", "Das Fenster ist nicht mehr geöffnet."),
    ("window-transient.kind-unknown", "This window cannot hold this state — use a window of the matching kind.", "Dieses Fenster kann diesen Zustand nicht halten — ein Fenster der passenden Art verwenden."),
    ("timeTravel.member-owners", "A history edit could not finish closing a composed part — try again.", "Eine Verlaufsbearbeitung konnte einen zusammengesetzten Teil nicht fertig schließen — erneut versuchen."),
    ("timeTravel.snapshot-close", "The history view could not close its snapshot cleanly — try again.", "Die Verlaufsansicht konnte ihren Schnappschuss nicht sauber schließen — erneut versuchen."),
    ("timeTravel.snapshot-retirement", "The history view could not release its snapshot — try again.", "Die Verlaufsansicht konnte ihren Schnappschuss nicht freigeben — erneut versuchen."),
    ("timeTravel.unknown-action", "This history action is not known.", "Diese Verlaufsaktion ist unbekannt."),
    ("timeTravel.member-store-kind", "This part keeps its history differently and cannot be edited here.", "Dieser Teil führt seinen Verlauf anders und kann hier nicht bearbeitet werden."),
    ("timeTravel.preview-mismatch", "The history preview answered a different request — try again.", "Die Verlaufsvorschau hat auf eine andere Anfrage geantwortet — erneut versuchen."),
    ("toolRun.unknown-action", "This tool run action is not known.", "Diese Werkzeuglauf-Aktion ist unbekannt."),
    ("toolRun.tool-id", "Choose a tool before starting a run.", "Vor dem Start eines Laufs ein Werkzeug wählen."),
    ("toolRun.unknown-tool", "This tool cannot run step by step.", "Dieses Werkzeug kann nicht schrittweise laufen."),
    ("toolRun.busy", "A tool run is still active — finish or cancel it first.", "Ein Werkzeuglauf ist noch aktiv — ihn zuerst beenden oder abbrechen."),
    ("toolRun.tick-bytes", "A tool run step is too large to show.", "Ein Schritt des Werkzeuglaufs ist zu groß zum Anzeigen."),
    ("toolRun.tick-decode", "A tool run step could not be read.", "Ein Schritt des Werkzeuglaufs konnte nicht gelesen werden."),
    ("toolRun.trace-lane", "The tool run's progress could not be shown.", "Der Fortschritt des Werkzeuglaufs konnte nicht angezeigt werden."),
    ("toolRun.member-unavailable", "This tool run needs exactly one open part to work on.", "Dieser Werkzeuglauf braucht genau einen geöffneten Teil zum Bearbeiten."),
    ("toolRun.member-gone", "The part this tool run edits is no longer there.", "Der Teil, den dieser Werkzeuglauf bearbeitet, ist nicht mehr vorhanden."),
    ("toolRun.member-store-kind", "This part keeps its history differently and cannot run this tool.", "Dieser Teil führt seinen Verlauf anders und kann dieses Werkzeug nicht ausführen."),
    ("toolRun.member-base-lost", "The tool run lost the state it started from — start it again.", "Der Werkzeuglauf hat seinen Ausgangszustand verloren — ihn erneut starten."),
    ("toolRun.job-close", "The tool run could not close cleanly — try again.", "Der Werkzeuglauf konnte nicht sauber beendet werden — erneut versuchen."),
    ("toolRun.snapshot-close", "The tool run could not close its snapshot cleanly — try again.", "Der Werkzeuglauf konnte seinen Schnappschuss nicht sauber schließen — erneut versuchen."),
    ("toolRun.snapshot-retirement", "The tool run could not release its snapshot — try again.", "Der Werkzeuglauf konnte seinen Schnappschuss nicht freigeben — erneut versuchen."),
    ("toolRun.publication-close", "The tool run could not finish publishing its result — try again.", "Der Werkzeuglauf konnte sein Ergebnis nicht fertig veröffentlichen — erneut versuchen."),
    ("toolRun.publication-handoff", "The tool run lost track of its result — try again.", "Der Werkzeuglauf hat den Überblick über sein Ergebnis verloren — erneut versuchen."),
    ("toolRun.publication-retirement", "The tool run could not release its result — try again.", "Der Werkzeuglauf konnte sein Ergebnis nicht freigeben — erneut versuchen."),
    ("toolTransaction.shape", "This tool change does not have the shape its tool declares.", "Diese Werkzeugänderung hat nicht die Form, die ihr Werkzeug angibt."),
    ("transaction.instance-busy", "Another linked change is still pending — wait for it to finish.", "Eine andere verknüpfte Änderung steht noch aus — warten, bis sie abgeschlossen ist."),
    ("transaction.member-rejected", "One part refused the linked change.", "Ein Teil hat die verknüpfte Änderung abgelehnt."),
    ("transaction.unknown-mutation", "A linked change holds an edit this part does not know.", "Eine verknüpfte Änderung enthält eine Bearbeitung, die dieser Teil nicht kennt."),
    ("transaction.generation-mismatch", "The part changed while the linked change was prepared — try again.", "Der Teil hat sich geändert, während die verknüpfte Änderung vorbereitet wurde — erneut versuchen."),
    ("transaction.commit-failed", "A linked change could not be completed in every part.", "Eine verknüpfte Änderung konnte nicht in allen Teilen abgeschlossen werden."),
    ("transaction.child-groups-malformed", "A linked change to the parts could not be read.", "Eine verknüpfte Änderung der Teile konnte nicht gelesen werden."),
    ("transaction.rollback-unknown", "There is no pending linked change to roll back.", "Es gibt keine ausstehende verknüpfte Änderung zum Zurücknehmen."),
    ("transaction.undo-foreign-tail", "The latest step of this part belongs to another change — undo that first.", "Der letzte Schritt dieses Teils gehört zu einer anderen Änderung — diese zuerst rückgängig machen."),
    ("transaction.redo-foreign-tail", "The next redo step of this part belongs to another change.", "Der nächste Wiederherstellungsschritt dieses Teils gehört zu einer anderen Änderung."),
    ("transaction.undo-failed", "This linked change could not be undone.", "Diese verknüpfte Änderung konnte nicht rückgängig gemacht werden."),
    ("transaction.redo-failed", "This linked change could not be redone.", "Diese verknüpfte Änderung konnte nicht wiederhergestellt werden."),
    ("transaction.group-history-dialect", "This composed document's type does not allow this history step.", "Der Typ dieses zusammengesetzten Dokuments erlaubt diesen Verlaufsschritt nicht."),
    ("transaction.group-history-root", "This history step would move a part it may not move.", "Dieser Verlaufsschritt würde einen Teil verschieben, den er nicht verschieben darf."),
    ("transaction.group-history-tail", "This change is no longer the latest step of any part.", "Diese Änderung ist bei keinem Teil mehr der letzte Schritt."),
    ("interactive-job.cancelled", "The action was cancelled.", "Die Aktion wurde abgebrochen."),
    ("interactive-job.tool-completion-busy", "The tool is still finishing — try again in a moment.", "Das Werkzeug wird noch abgeschlossen — gleich erneut versuchen."),
    ("interactive-job.maintenance-tool-authority", "A running tool changed during upkeep — try again.", "Ein laufendes Werkzeug hat sich während der Wartung geändert — erneut versuchen."),
    ("interactive-job.tool-document-retirement-invariant", "The tool could not release its document cleanly — try again.", "Das Werkzeug konnte sein Dokument nicht sauber freigeben — erneut versuchen."),
    ("interactive-job.publication-authority-missing", "This command publishes its result in a way this app does not support.", "Dieser Befehl veröffentlicht sein Ergebnis auf eine Weise, die diese App nicht unterstützt."),
    ("interactive-job.missing-owned-reducer", "This command has no editing step of its own in this app.", "Dieser Befehl hat in dieser App keinen eigenen Bearbeitungsschritt."),
    ("interactive-job.missing-factory", "This command is not registered as a tool of this app.", "Dieser Befehl ist nicht als Werkzeug dieser App registriert."),
    ("interactive-job.incomplete-operation-authority", "This command lacks a complete prepare, edit and commit path.", "Diesem Befehl fehlt ein vollständiger Ablauf aus Vorbereiten, Bearbeiten und Übernehmen."),
    ("interactive-job.catalog-controller", "This app's tools are registered to a different controller.", "Die Werkzeuge dieser App sind bei einer anderen Steuerung registriert."),
    ("interactive-job.catalog-authority", "A tool of this app is registered without its full authority.", "Ein Werkzeug dieser App ist ohne seine vollständige Berechtigung registriert."),
    ("interactive-job.catalog-incomplete", "A command of this app lacks its registered editing step.", "Einem Befehl dieser App fehlt sein registrierter Bearbeitungsschritt."),
    ("interactive-job.child-emission-retirement-refused", "A change to a part could not be released cleanly — try again.", "Eine Änderung an einem Teil konnte nicht sauber freigegeben werden — erneut versuchen."),
    ("plugin.document-load.unavailable", "This app cannot load or save whole documents.", "Diese App kann keine ganzen Dokumente laden oder speichern."),
    ("plugin.document-load.too-many-members", "The document has {count} parts, but at most {maximum} can be loaded.", "Das Dokument hat {count} Teile, es lassen sich aber höchstens {maximum} laden."),
    ("plugin.document-load.too-large", "The document is {bytes} bytes large, but at most {maximum} bytes can be loaded.", "Das Dokument ist {bytes} Bytes groß, es lassen sich aber höchstens {maximum} Bytes laden."),
    ("plugin.document-load.busy", "Another document is still loading. Try again when it has finished.", "Ein anderes Dokument wird noch geladen. Versuche es erneut, sobald es fertig ist."),
    ("plugin.document-load.incomplete", "The document is incomplete: its content or its history is missing.", "Das Dokument ist unvollständig: Inhalt oder Verlauf fehlt."),
    ("plugin.document-load.member-invalid", "Part {ordinal} of the document is damaged, so the document was not loaded.", "Teil {ordinal} des Dokuments ist beschädigt, daher wurde das Dokument nicht geladen."),
    ("plugin.document-load.history-invalid", "The history of the document cannot be read, so the document was not loaded.", "Der Verlauf des Dokuments lässt sich nicht lesen, daher wurde das Dokument nicht geladen."),
    ("plugin.document-load.other-document", "This is another document, so it cannot be merged into the open one.", "Das ist ein anderes Dokument, es lässt sich nicht mit dem geöffneten zusammenführen."),
    ("plugin.document-load.members-differ", "Parts of this document changed elsewhere, so it is loaded again instead of merged.", "Teile dieses Dokuments wurden anderswo geändert, daher wird es neu geladen statt zusammengeführt."),
    ("plugin.document-load.operation-unknown", "This load is no longer running.", "Dieser Ladevorgang läuft nicht mehr."),
    ("plugin.document-load.changed", "The document changed while it was being saved. Try again.", "Das Dokument hat sich beim Speichern geändert. Versuche es erneut."),
    ("plugin.document-load.failed", "The document could not be loaded. The previous document is unchanged.", "Das Dokument konnte nicht geladen werden. Das vorherige Dokument ist unverändert."),
    ("artifact-envelope.stale-handle", "The document changed while it was being prepared. Try again.", "Das Dokument hat sich während der Vorbereitung geändert. Versuche es erneut."),
    ("artifact-envelope.load-stale-handle", "The document changed while it was being loaded. Try again.", "Das Dokument hat sich während des Ladens geändert. Versuche es erneut."),
    ("artifact-store.replacement-stale-handle", "The document changed while it was being replaced. Try again.", "Das Dokument hat sich während des Ersetzens geändert. Versuche es erneut."),
    ("window-config.owner", "This window's settings belong to another window and were not loaded.", "Die Einstellungen dieses Fensters gehören zu einem anderen Fenster und wurden nicht geladen."),
    ("window-config.load-retirement", "This window's settings could not be replaced. The previous settings are unchanged.", "Die Einstellungen dieses Fensters ließen sich nicht ersetzen. Die bisherigen Einstellungen sind unverändert."),
];

/// 🪧️ The `(en, de)` notice of a framework-namespace refusal `code` ([`FRAMEWORK_FAULT_NOTICE_LABELS`]); `None` for any other code.
pub fn framework_fault_notice(code: &str) -> Option<(&'static str, &'static str)> {
    FRAMEWORK_FAULT_NOTICE_LABELS.iter().find(|(known, _, _)| *known == code).map(|(_, en, de)| (*en, *de))
}

/// 🗯️ The notice a fault earns: its own code, then each cause's, is looked up in the framework's tables
/// ([`HISTORY_NOTICE_LABELS`], [`FRAMEWORK_FAULT_NOTICE_LABELS`]) first and the refusing app's published table
/// (`AppDefinition::fault_notices`) second; the first declared code decides, its `{name}` placeholders filled from
/// `Fault.params` only. `None` when no code is declared or a placeholder has no value — the shell then shows its generic
/// refusal, never a raw code. Corpus `🛂️manifest/🧫️fixtures/🧫️fault-notices`; TS twin `faultNotice`.
pub fn fault_notice(fault: &Fault, notices: &[FaultNoticeDefinition], terminology: semio_framework_ui_locale::Terminology, locale: semio_framework_ui_locale::Locale) -> Option<FaultNotice> {
    let params = fault.params.as_deref();
    let framework = |code: &str| history_notice(code).or_else(|| framework_fault_notice(code));
    let code = std::iter::once(&fault.code).chain(fault.causes.iter().filter_map(|cause| cause.code.as_ref())).map(|code| code.0.as_str()).find(|code| framework(code).is_some() || notices.iter().any(|notice| notice.code == *code))?;
    let text = match framework(code) {
        Some((en, de)) => fill_fault_notice(semio_framework_ui_locale::LocalizedLabel::native(en, de).resolve(terminology, locale), params),
        None => fault_notice_text(notices, code, params, terminology, locale),
    }?;
    Some(FaultNotice { code: code.to_string(), text })
}

//#endregion 🔖️FaultNotices
//#region 🔖️HistoryReprojectionStatus

/// 📡️ Framework-owned EN/DE copy of a history change replaying before this replica adopts it ([`HistoryReprojection`]), by
/// `<kind>.<part>` key: every shell announces it in a polite status region outside the History panel and the history body
/// titles its reprojection section with it (design §16.6, gap N17, audit W1E-3). `{done}`/`{total}` are replayed
/// operations, `{reason}` a refusal's notice. Fixture `🧫️fixtures/🧫️history-reprojection/🔣️.json`; TS twin
/// `HISTORY_REPROJECTION_LABELS`.
pub const HISTORY_REPROJECTION_LABELS: [(&str, &str, &str); 12] = [
    ("remote.title", "Remote history change", "Entfernte Verlaufsänderung"),
    ("remote.progress", "Replaying a remote history change: {done} of {total} steps", "Entfernte Verlaufsänderung wird angewendet: {done} von {total} Schritten"),
    ("remote.paused", "Remote history change paused: this replica still shows the history before it", "Entfernte Verlaufsänderung pausiert: dieses Replikat zeigt noch den Verlauf davor"),
    ("remote.refused", "Remote history change refused: {reason}", "Entfernte Verlaufsänderung abgelehnt: {reason}"),
    ("step.title", "History step", "Verlaufsschritt"),
    ("step.progress", "Replaying history: {done} of {total} steps", "Verlauf wird neu angewendet: {done} von {total} Schritten"),
    ("step.refused", "History step refused: {reason}", "Verlaufsschritt abgelehnt: {reason}"),
    ("load.title", "Document load", "Dokument laden"),
    ("load.progress", "Loading document: {done} of {total}", "Dokument wird geladen: {done} von {total}"),
    ("load.refused", "Document load refused: {reason}", "Laden des Dokuments abgelehnt: {reason}"),
    ("reason.unnamed", "the change could not be applied", "die Änderung konnte nicht angewendet werden"),
    ("work.processed", "Work completed: {processed}", "Arbeitsfortschritt: {processed}"),
];

/// 📢️ What a shell announces while a history change replays before adoption ([`history_reprojection_status`]): the kind's
/// title, one status line, replayed operations of the total, whether the user paused a remote change, and the code of a
/// refused adoption (a shell's `data-notice-code`, never shown as text).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryReprojectionStatus {
    pub title: String,
    pub text: String,
    pub done: u32,
    pub total: u32,
    pub paused: bool,
    pub fault: Option<String>,
}

/// 📢️ The one status copy of a history change replaying before adoption, for both shells and the history body: a refused
/// adoption (a fault with nothing left to replay) reads `<kind>.refused` with the fault's placeholder-free history notice
/// ([`HISTORY_NOTICE_LABELS`]) or `reason.unnamed` — never a raw code; a paused remote change reads `remote.paused` (a step or
/// a load is never paused); anything else its kind's progress. Labels [`HISTORY_REPROJECTION_LABELS`]; TS twin
/// `historyReprojectionStatus`.
pub fn history_reprojection_status(reprojection: &HistoryReprojection, terminology: semio_framework_ui_locale::Terminology, locale: semio_framework_ui_locale::Locale) -> HistoryReprojectionStatus {
    let localized = |en: &str, de: &str| semio_framework_ui_locale::LocalizedLabel::native(en, de).resolve(terminology, locale).to_string();
    let label = |key: String| HISTORY_REPROJECTION_LABELS.iter().find(|(known, _, _)| *known == key).map_or_else(String::new, |(_, en, de)| localized(en, de));
    let kind = match reprojection.kind {
        HistoryReprojectionKind::Remote => "remote",
        HistoryReprojectionKind::Step => "step",
        HistoryReprojectionKind::Load => "load",
    };
    let paused = reprojection.paused && reprojection.kind == HistoryReprojectionKind::Remote;
    let mut text = match reprojection.fault.as_deref() {
        Some(code) if reprojection.total == 0 => {
            let reason = history_notice(code).filter(|(en, de)| !en.contains('{') && !de.contains('{')).map_or_else(|| label("reason.unnamed".to_string()), |(en, de)| localized(en, de));
            label(format!("{kind}.refused")).replace("{reason}", &reason)
        }
        _ if paused => label("remote.paused".to_string()),
        _ => label(format!("{kind}.progress")).replace("{done}", &reprojection.done.to_string()).replace("{total}", &reprojection.total.to_string()),
    };
    if let Some(processed) = reprojection.processed.filter(|_| !paused && reprojection.total > 0) {
        text.push_str(" · ");
        text.push_str(&label("work.processed".to_string()).replace("{processed}", &processed.to_string()));
    }
    HistoryReprojectionStatus { title: label(format!("{kind}.title")), text, done: reprojection.done, total: reprojection.total, paused, fault: reprojection.fault.clone() }
}

//#endregion 🔖️HistoryReprojectionStatus
// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️FaultNotices

/// 📢️ One app-declared fault notice (design §20.12): the `code` a guest refusal carries (`<app>.<area>.<name>`) and its
/// text in every shell locale × terminology, whose `{name}` placeholders are filled from the fault's `params` — never from
/// its English `message`. Published in [`AppDefinition::fault_notices`]; schema `🛂️manifest/🧬️schema` `$defs.FaultNoticeDefinition`,
/// TS twin `FaultNoticeDefinition` (`🤖️generated/🪪️manifest`), corpus `🧫️fixtures/🧫️fault-notices`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct FaultNoticeDefinition {
    pub code: String,
    pub label: LocalizedLabel,
}

/// 🚫️ Why a fault notice table is refused: a code outside the grammar, a code declared twice, an empty text cell, or a cell
/// whose `{name}` placeholders are malformed or differ from the notice's first cell (every locale names the same data).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FaultNoticeError {
    Syntax { code: String },
    Duplicate { code: String },
    EmptyText { code: String, terminology: Terminology, locale: Locale },
    Placeholder { code: String, terminology: Terminology, locale: Locale },
}

impl FaultNoticeError {
    /// 🏷️ The corpus/gate name of this rule (`syntax`, `duplicate`, `emptyText`, `placeholder`).
    pub fn rule(&self) -> &'static str {
        match self {
            Self::Syntax { .. } => "syntax",
            Self::Duplicate { .. } => "duplicate",
            Self::EmptyText { .. } => "emptyText",
            Self::Placeholder { .. } => "placeholder",
        }
    }

    /// 🔎️ The notice code the refusal names.
    pub fn code(&self) -> &str {
        match self {
            Self::Syntax { code } | Self::Duplicate { code } | Self::EmptyText { code, .. } | Self::Placeholder { code, .. } => code,
        }
    }
}

impl std::fmt::Display for FaultNoticeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyText { code, terminology, locale } | Self::Placeholder { code, terminology, locale } => write!(formatter, "fault notice {code}: {} in {terminology}/{locale}", self.rule()),
            Self::Syntax { code } | Self::Duplicate { code } => write!(formatter, "fault notice {code}: {}", self.rule()),
        }
    }
}

/// 🔤️ Whether `code` is an app notice code: three or more dot-separated kebab segments (`generation3d.gumball.mesh-missing`)
/// — the app, the area, the refusal. Framework codes have their own tables and are resolved first.
pub fn is_fault_notice_code(code: &str) -> bool {
    let segment = |segment: &str| segment.split('-').enumerate().all(|(index, part)| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()) && (index > 0 || part.as_bytes()[0].is_ascii_lowercase()));
    code.split('.').count() >= 3 && code.split('.').all(segment)
}

/// 🧩️ The sorted, unique `{name}` placeholders of one notice text; `None` when a brace is unbalanced or names no valid
/// placeholder (`semio_framework_diagnostic::is_fault_param_name`). Texts have no brace escape.
pub fn fault_notice_placeholders(text: &str) -> Option<std::collections::BTreeSet<&str>> {
    let mut names = std::collections::BTreeSet::new();
    let mut rest = text;
    while let Some(open) = rest.find(['{', '}']) {
        if rest.as_bytes()[open] == b'}' {
            return None;
        }
        let close = rest[open + 1..].find(['{', '}'])? + open + 1;
        let name = &rest[open + 1..close];
        if rest.as_bytes()[close] != b'}' || !semio_framework_diagnostic::is_fault_param_name(name) {
            return None;
        }
        names.insert(name);
        rest = &rest[close + 1..];
    }
    Some(names)
}

/// ✅️ Every rule a published fault notice table must satisfy, in table order: code grammar, unique codes, a non-empty text
/// in every terminology × locale cell, and one placeholder set across a notice's cells. Empty = valid. TS twin
/// `validateFaultNotices`.
pub fn validate_fault_notices(notices: &[FaultNoticeDefinition]) -> Vec<FaultNoticeError> {
    let mut errors = Vec::new();
    for (index, notice) in notices.iter().enumerate() {
        let code = || notice.code.clone();
        if !is_fault_notice_code(&notice.code) {
            errors.push(FaultNoticeError::Syntax { code: code() });
        }
        if notices[..index].iter().any(|earlier| earlier.code == notice.code) {
            errors.push(FaultNoticeError::Duplicate { code: code() });
        }
        let first = fault_notice_placeholders(notice.label.resolve(Terminology::ALL[0], Locale::ALL[0]));
        for terminology in Terminology::ALL {
            for locale in Locale::ALL {
                let text = notice.label.resolve(terminology, locale);
                if text.trim().is_empty() {
                    errors.push(FaultNoticeError::EmptyText { code: code(), terminology, locale });
                } else if first.is_none() || fault_notice_placeholders(text) != first {
                    errors.push(FaultNoticeError::Placeholder { code: code(), terminology, locale });
                }
            }
        }
    }
    errors
}

/// 🗣️ The notice text `code` names in `notices` for one shell axis pair, its `{name}` placeholders filled from `params`;
/// `None` when no notice declares `code` or a placeholder has no value (a notice is never shown half-filled). TS twin
/// `faultNoticeText`.
pub fn fault_notice_text(notices: &[FaultNoticeDefinition], code: &str, params: Option<&semio_framework_diagnostic::FaultParams>, terminology: Terminology, locale: Locale) -> Option<String> {
    let text = notices.iter().find(|notice| notice.code == code)?.label.resolve(terminology, locale);
    fill_fault_notice(text, params)
}

/// 🧵️ `text` with each `{name}` replaced by the value `params` names; `None` when one is missing or the text is malformed.
pub fn fill_fault_notice(text: &str, params: Option<&semio_framework_diagnostic::FaultParams>) -> Option<String> {
    let mut filled = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        let close = rest[open..].find('}')? + open;
        filled.push_str(&rest[..open]);
        filled.push_str(params?.get(&rest[open + 1..close])?);
        rest = &rest[close + 1..];
    }
    filled.push_str(rest);
    Some(filled)
}

/// 📋️ An app's declared table (`ArtifactApp::fault_notices`) as the definitions its descriptor publishes.
pub fn fault_notice_definitions(table: &[(&str, LocalizedLabel)]) -> Vec<FaultNoticeDefinition> {
    table.iter().map(|(code, label)| FaultNoticeDefinition { code: (*code).to_string(), label: label.clone() }).collect()
}

//#endregion 🔖️FaultNotices
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest

#[cfg(test)]
use semio_framework_diagnostic::FaultCause;
#[cfg(test)]
use semio_framework_diagnostic::FaultCode;
#[cfg(test)]
use semio_framework_diagnostic::FaultOrigin;
#[cfg(test)]
use semio_framework_value::ValueError;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️FaultNotices
#[cfg(test)]
#[path = "../../🛂️manifest/🧪️tests/🧪️fault-notices/🦀️.rs"]
mod definition_fault_notices_tests;

//#endregion 🔖️FaultNotices
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
//#region 🔖️FaultNotices
#[cfg(test)]
#[path = "../🧪️tests/🧪️fault-notices/🦀️.rs"]
mod fault_notices_tests;

#[cfg(test)]
#[path = "../🧪️tests/🧪️framework-notices/🦀️.rs"]
mod framework_notices_tests;

//#endregion 🔖️FaultNotices
//#region 🔖️HistoryReprojectionStatus
#[cfg(test)]
#[path = "../🧪️tests/🧪️history-reprojection/🦀️.rs"]
mod history_reprojection_tests;

//#endregion 🔖️HistoryReprojectionStatus
//#region 🔖️HistoryWire
#[cfg(test)]
#[path = "../🧪️tests/🧪️history-patch/🦀️.rs"]
mod history_patch_tests;

#[cfg(test)]
#[path = "../🧪️tests/🧪️history-notices/🦀️.rs"]
mod history_notices_tests;
//#endregion 🔖️HistoryWire
