//! 🌐️ Every string the native dashboard shows, in every language, in one compile-checked catalogue.
//! A missing German text is a compile error, not a blank row. Identifiers (command ids, parameter
//! ids and values, key names) are data and stay verbatim.
//!
//! @see 🧰️framework/🔨️modules/🖱️ui/🌐️locale/🧩️labels/🦀️.rs

use ui_locale::{AppLabels, Locale, Terminology};

macro_rules! dashboard_labels {
    ($($field:ident: $en:expr, $de:expr;)+) => {
        ui_locale::app_labels! {
            pub struct DashboardLabels {
                $($field: native_en $en, native_de $de, reuse_en $en, reuse_de $de;)+
            }
        }
    };
}

dashboard_labels! {
    nav_brand: "semio", "semio";
    nav_mode: "dashboard", "Übersicht";
    title_tasks: "Tasks", "Aufgaben";
    title_commands: "Commands", "Befehle";
    title_settings: "Settings", "Einstellungen";
    title_keyboard: "Keyboard", "Tastatur";
    row_new_task: "New task — type to find a command", "Neue Aufgabe — Befehl suchen";
    row_settings: "Settings", "Einstellungen";
    row_keyboard: "Keyboard help", "Tastaturhilfe";
    row_refresh: "Refresh commands", "Befehle aktualisieren";
    row_cancel_discovery: "Cancel discovery", "Suche abbrechen";
    row_no_tasks: "No tasks yet — start one with New task", "Noch keine Aufgaben — mit Neue Aufgabe starten";
    row_back: "Back to tasks", "Zurück zu Aufgaben";
    status_pending: "waiting", "wartet";
    status_running: "running", "läuft";
    status_ready: "ready", "bereit";
    status_stopping: "stopping", "stoppt";
    status_exited: "exited", "beendet";
    status_failed: "failed", "fehlgeschlagen";
    status_interrupted: "interrupted", "unterbrochen";
    exit_code: "exit {code}", "Code {code}";
    setting_language: "Language", "Sprache";
    setting_appearance: "Appearance", "Darstellung";
    setting_terminology: "Terminology", "Begriffe";
    setting_renderer: "Preferred renderer", "Bevorzugter Renderer";
    setting_layout: "Output layout", "Ausgabelayout";
    setting_scope: "Save scope", "Speicherbereich";
    setting_prefix: "Prefix key", "Präfixtaste";
    scope_workspace: "workspace shared", "Arbeitsbereich gemeinsam";
    scope_local: "local only", "nur lokal";
    state_connecting: "connecting", "verbinde";
    state_connected: "connected", "verbunden";
    state_reconnecting: "reconnecting", "verbinde neu";
    state_daemon_stopped: "daemon stopped", "Dienst gestoppt";
    state_shutdown_timeout: "daemon shutdown timed out", "Beenden des Dienstes dauerte zu lange";
    state_discovering: "Discovering commands", "Befehle suchen";
    state_discovery_cancelled: "Discovery cancelled; available commands retained", "Suche abgebrochen; verfügbare Befehle bleiben";
    state_discovering_progress: "discovering {seconds}s · {cancel} cancels", "suche {seconds}s · {cancel} bricht ab";
    state_commands: "{count} commands", "{count} Befehle";
    state_running: "{count} running", "{count} laufen";
    phase_known: "from cache", "aus Zwischenspeicher";
    phase_sources: "sources read", "Quellen gelesen";
    phase_walk: "scanning", "durchsuche";
    phase_complete: "complete", "vollständig";
    prefs_saving: "Saving preferences", "Einstellungen speichern";
    prefs_busy: "Saving preferences; try again shortly", "Einstellungen werden gespeichert; gleich erneut versuchen";
    prefs_saved: "Preferences saved", "Einstellungen gespeichert";
    prefs_keymap_problem: "keymap: {problem}", "Tastenbelegung: {problem}";
    key_unbound: "{keys} is not bound", "{keys} ist nicht belegt";
    key_armed: "waiting for the next key", "nächste Taste erwartet";
    start_waiting: "Waiting for workspace daemon; {cancel} cancels", "Warten auf Arbeitsbereich; {cancel} bricht ab";
    pane_waiting: "[semio] waiting for workspace daemon", "[semio] warten auf Arbeitsbereich";
    pane_limit: "[semio] pending task limit reached", "[semio] Grenze wartender Aufgaben erreicht";
    pane_error: "[semio] {message}", "[semio] {message}";
    launcher_prompt: "Type to search commands", "Tippen zum Suchen von Befehlen";
    launcher_counts: "{shown} of {total} commands", "{shown} von {total} Befehlen";
    launcher_empty: "No command matches", "Kein Befehl passt";
    launcher_loading: "Discovering commands…", "Befehle werden gesucht…";
    form_prompt_text: "Type the value", "Wert eintippen";
    form_prompt_choice: "Left and Right choose a value", "Links und Rechts wählen einen Wert";
    form_parameters: "Parameters", "Parameter";
    form_no_parameters: "No parameters", "Keine Parameter";
    form_extra_args: "Extra arguments", "Zusätzliche Argumente";
    form_extra_env: "Extra environment", "Zusätzliche Umgebung";
    form_extra_args_prompt: "Words appended after the command's own arguments; quotes group", "Wörter nach den eigenen Argumenten des Befehls; Anführungszeichen gruppieren";
    form_extra_env_prompt: "KEY=value pairs added to the environment; quotes group", "KEY=Wert-Paare für die Umgebung; Anführungszeichen gruppieren";
    form_requires: "Starts first", "Startet zuerst";
    form_members: "Starts together ({count})", "Startet gemeinsam ({count})";
    form_command: "Command", "Aufruf";
    form_start: "Start", "Starten";
    form_mutating: "Changes the repository; asks to confirm", "Ändert das Repository; fragt nach Bestätigung";
    form_required: "required", "erforderlich";
    form_default: "default", "Standard";
    form_on: "on", "an";
    form_off: "off", "aus";
    form_unset: "not set", "nicht gesetzt";
    form_problem: "Cannot start: {problem}", "Start nicht möglich: {problem}";
    confirm_prompt: "{command} changes the repository. Enter runs it, back returns.", "{command} ändert das Repository. Enter führt aus, zurück kehrt um.";
    help_prefix: "After the prefix key {prefix}", "Nach der Präfixtaste {prefix}";
    help_window: "Windows and tabs", "Fenster und Tabs";
    help_view: "Lists, launcher and forms", "Listen, Starter und Formulare";
    help_terminal: "While a terminal has the keyboard only the prefix key is reserved; every other key reaches the program.", "Hat ein Terminal die Tastatur, ist nur die Präfixtaste reserviert; jede andere Taste erreicht das Programm.";
    help_customize: "Customize with: semio preferences set --prefix KEY --bindings JSON", "Anpassen mit: semio preferences set --prefix TASTE --bindings JSON";
    chrome_close: "Close tab", "Tab schließen";
    chrome_maximize: "Maximize window", "Fenster maximieren";
    chrome_restore: "Restore window", "Fenster wiederherstellen";
    chrome_new_tab: "New task in this stack", "Neue Aufgabe in diesem Stapel";
    chrome_previous_tabs: "Earlier tabs", "Frühere Tabs";
    chrome_next_tabs: "Later tabs", "Spätere Tabs";
    skew_incompatible: "daemon speaks protocol {daemon} but this client speaks {client}; stop the daemon once its tasks finish (semio daemon stop) and start it again", "Der Dienst spricht Protokoll {daemon}, dieser Client {client}; Dienst beenden, sobald seine Aufgaben fertig sind (semio daemon stop), und neu starten";
    skew_build: "daemon build {daemon} differs from this client's build {client}; its tasks keep running", "Dienst-Build {daemon} weicht vom Build dieses Clients {client} ab; seine Aufgaben laufen weiter";
    replay_truncated: "earlier output of a task was no longer kept", "Frühere Ausgabe einer Aufgabe wurde nicht mehr aufbewahrt";
    error_protocol: "the daemon refused this protocol", "Der Dienst lehnt dieses Protokoll ab";
    error_decode: "the daemon could not read a request", "Der Dienst konnte eine Anfrage nicht lesen";
    error_invalid: "the daemon refused an invalid request: {message}", "Der Dienst lehnt eine ungültige Anfrage ab: {message}";
    error_hello_required: "the daemon expects a greeting first", "Der Dienst erwartet zuerst eine Begrüßung";
    error_unknown_session: "that task no longer exists", "Diese Aufgabe gibt es nicht mehr";
    error_unknown_group: "that group of tasks no longer exists", "Diese Aufgabengruppe gibt es nicht mehr";
    error_already_running: "that task is already running", "Diese Aufgabe läuft bereits";
    error_limit: "the daemon holds as many tasks as it allows", "Der Dienst hält so viele Aufgaben, wie er darf";
    error_view_limit: "the daemon serves as many views as it allows; close another view and try again", "Der Dienst bedient so viele Ansichten, wie er darf; schließe eine andere Ansicht und versuche es erneut";
    error_spawn: "the task could not be started: {message}", "Die Aufgabe konnte nicht gestartet werden: {message}";
    error_not_running: "that task is not running", "Diese Aufgabe läuft nicht";
    error_input_backlog: "the task is not reading its input; keys were dropped", "Die Aufgabe liest ihre Eingabe nicht; Tasten gingen verloren";
    error_unspecified: "{message}", "{message}";
    start_pending_full: "too many starts are waiting for the daemon", "Zu viele Starts warten auf den Dienst";
    start_unavailable: "commands are still being discovered", "Befehle werden noch gesucht";
    start_requeued: "the daemon connection dropped; the start is kept and sent again on reconnect", "Die Verbindung zum Dienst brach ab; der Start bleibt erhalten und wird nach dem Verbinden gesendet";
    cli_daemon_usage: "usage: semio daemon start|stop|status|attach|serve", "Verwendung: semio daemon start|stop|status|attach|serve";
    cli_daemon_serve_failed: "daemon serve failed: {error}", "Der Dienst konnte nicht laufen: {error}";
    cli_daemon_ready: "workspace dashboard daemon ready at pid {pid}", "Dashboard-Dienst des Arbeitsbereichs bereit unter pid {pid}";
    cli_daemon_start_failed: "failed to start daemon: {error}", "Der Dienst konnte nicht gestartet werden: {error}";
    cli_daemon_startup_failed: "daemon startup failed", "Der Dienst konnte nicht starten";
    cli_daemon_stopped: "stopped workspace dashboard daemon", "Dashboard-Dienst des Arbeitsbereichs gestoppt";
    cli_daemon_stop_failed: "daemon shutdown failed: {error}", "Der Dienst konnte nicht beendet werden: {error}";
    cli_daemon_stop_timed_out: "daemon shutdown timed out", "Das Beenden des Dienstes dauerte zu lange";
    cli_daemon_not_running: "daemon not running", "Der Dienst läuft nicht";
    cli_daemon_status: "daemon pid {pid} · build {build} · {count} active tasks · {endpoint}", "Dienst pid {pid} · Build {build} · {count} aktive Aufgaben · {endpoint}";
    cli_warning: "warning: {message}", "Warnung: {message}";
    cli_dashboard_unexpected_arguments: "[dashboard] unexpected arguments: {arguments}", "[dashboard] unerwartete Argumente: {arguments}";
    cli_dashboard_attach_failed: "[dashboard] failed to attach to the terminal", "[dashboard] das Terminal konnte nicht übernommen werden";
    cli_unknown_verb: "[semio] unknown verb {verb}", "[semio] unbekannter Befehl {verb}";
    cli_run_failed: "[semio] failed to run {command}: {error}", "[semio] {command} konnte nicht gestartet werden: {error}";
    prefs_journal_problem: "preferences journal ignored: {problem}", "Einstellungsjournal ignoriert: {problem}";
    hint_activate: "select", "wählen";
    hint_controls: "controls", "Steuerung";
    hint_type: "type to search", "tippen zum Suchen";
    action_new_task: "new task", "neue Aufgabe";
    action_show_tasks: "show tasks", "Aufgaben zeigen";
    action_show_settings: "settings", "Einstellungen";
    action_show_help: "keyboard help", "Tastaturhilfe";
    action_show_hidden_tasks: "restore tasks", "Aufgaben wiederherstellen";
    action_refresh_commands: "refresh commands", "Befehle aktualisieren";
    action_cancel_discovery: "cancel discovery", "Suche abbrechen";
    action_restart_task: "restart", "neu starten";
    action_stop_task: "stop", "anhalten";
    action_kill_task: "kill", "beenden";
    action_copy_selection: "copy selection", "Auswahl kopieren";
    action_search_output: "search output", "Ausgabe durchsuchen";
    action_toggle_input: "terminal input", "Terminaleingabe";
    action_toggle_appearance: "appearance", "Darstellung";
    action_toggle_language: "language", "Sprache";
    action_next_window: "next window", "nächstes Fenster";
    action_previous_window: "previous window", "voriges Fenster";
    action_split_down: "split down", "unten teilen";
    action_split_right: "split right", "rechts teilen";
    action_grow_window: "grow window", "Fenster vergrößern";
    action_shrink_window: "shrink window", "Fenster verkleinern";
    action_zoom: "zoom", "Zoom";
    action_close_window: "close window", "Fenster schließen";
    action_detach: "detach", "trennen";
    action_shutdown: "shutdown all", "alles stoppen";
    action_cancel_prefix: "cancel", "abbrechen";
    action_send_prefix: "send the prefix key to the terminal", "Präfixtaste ans Terminal senden";
    action_up: "move up", "nach oben";
    action_down: "move down", "nach unten";
    action_left: "collapse, previous value", "einklappen, voriger Wert";
    action_right: "expand, next value", "ausklappen, nächster Wert";
    action_page_up: "page up", "Seite hoch";
    action_page_down: "page down", "Seite runter";
    action_activate: "select, start", "auswählen, starten";
    action_clear: "clear text", "Text löschen";
    action_first: "first", "erste";
    action_last: "last", "letzte";
    action_delete_word: "delete word", "Wort löschen";
    action_back: "back", "zurück";
}

/// 🔤️ The catalogue for a language.
pub fn labels(locale: Locale) -> &'static DashboardLabels { <DashboardLabels as AppLabels>::labels(locale, Terminology::Native) }

impl DashboardLabels {
    /// 🚨 What the daemon's error kind means, in the language of the dashboard; `message` is its own detail.
    pub fn error(&self, code: Option<crate::ipc::ErrorCode>, message: &str) -> String {
        use crate::ipc::ErrorCode as Kind;
        let text = match code {
            Some(Kind::Protocol) => self.error_protocol, Some(Kind::Decode) => self.error_decode, Some(Kind::Invalid) => self.error_invalid, Some(Kind::HelloRequired) => self.error_hello_required,
            Some(Kind::UnknownSession) => self.error_unknown_session, Some(Kind::UnknownGroup) => self.error_unknown_group, Some(Kind::AlreadyRunning) => self.error_already_running, Some(Kind::Limit) => self.error_limit, Some(Kind::ViewLimit) => self.error_view_limit,
            Some(Kind::Spawn) => self.error_spawn, Some(Kind::NotRunning) => self.error_not_running, Some(Kind::InputBacklog) => self.error_input_backlog, None => self.error_unspecified,
        };
        text.fill(&[("message", message)]).into_string()
    }

    /// ⚖️ The sentence that names how the daemon differs from this client, in the language of the dashboard.
    pub fn skew(&self, skew: &crate::daemon::client::Skew) -> String {
        if skew.incompatible() { self.skew_incompatible.fill(&[("daemon", &skew.daemon_protocol.to_string()), ("client", &skew.client_protocol.to_string())]).into_string() } else { self.skew_build.fill(&[("daemon", &skew.daemon_build), ("client", &skew.client_build)]).into_string() }
    }

    /// 🗣️ What an action does, by the id the keymap states; an id without a text shows as itself.
    pub fn action<'a>(&self, id: &'a str) -> &'a str { self.try_action(id).unwrap_or(id) }

    /// 🔎️ What an action does, or `None` for an id this catalogue has no text for.
    pub fn try_action(&self, id: &str) -> Option<&'static str> {
        let text = match id {
            "new-task" => self.action_new_task, "show-tasks" => self.action_show_tasks, "show-settings" => self.action_show_settings, "show-help" => self.action_show_help,
            "show-hidden-tasks" => self.action_show_hidden_tasks, "refresh-commands" => self.action_refresh_commands, "cancel-discovery" => self.action_cancel_discovery,
            "search-output" => self.action_search_output, "restart-task" => self.action_restart_task, "stop-task" => self.action_stop_task, "kill-task" => self.action_kill_task, "copy-selection" => self.action_copy_selection,
            "toggle-input" => self.action_toggle_input, "toggle-appearance" => self.action_toggle_appearance, "toggle-language" => self.action_toggle_language,
            "next-window" => self.action_next_window, "previous-window" => self.action_previous_window, "split-down" => self.action_split_down, "split-right" => self.action_split_right,
            "grow-window" => self.action_grow_window, "shrink-window" => self.action_shrink_window, "zoom" => self.action_zoom, "close-window" => self.action_close_window,
            "detach" => self.action_detach, "shutdown" => self.action_shutdown, "cancel-prefix" => self.action_cancel_prefix, "send-prefix" => self.action_send_prefix,
            "up" => self.action_up, "down" => self.action_down, "left" => self.action_left, "right" => self.action_right, "page-up" => self.action_page_up, "page-down" => self.action_page_down,
            "first" => self.action_first, "last" => self.action_last, "delete-word" => self.action_delete_word, "activate" => self.action_activate, "clear" => self.action_clear, "back" => self.action_back,
            _ => return None,
        };
        Some(text.as_str())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
