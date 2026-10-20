//! 🌐️ Every string the native dashboard shows, in every language, in one compile-checked catalogue.
//! A missing German text is a compile error, not a blank row. Identifiers (command ids, parameter
//! ids and values, key names) are data and stay verbatim.
//!
//! @see 🧰️framework/🔨️modules/🖱️ui/🌐️locale/🧩️labels/🦀️.rs

use dashboard_launch::TaskLabel;
use semio_framework_filesystem_workspace::{Error,ErrorKind,Progress,Stage,commands::EntryAction};
use ui_locale::{AppLabels, Locale, Terminology};
use semio_framework_ui_tui_core::text::{elide_end, elide_middle};

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
    source_history: "retained history", "aufbewahrter Verlauf";
    action_inspect_history: "Inspect history", "Verlauf prüfen";
    action_reconcile_history: "Reconcile history", "Verlauf abgleichen";
    action_archive_history: "Archive history segment", "Verlaufsabschnitt archivieren";
    action_next_history_page: "Next history page", "Nächste Verlaufsseite";
    source_history_empty: "No history segment is active.", "Kein Verlaufsabschnitt ist aktiv.";
    source_history_end: "No further history page.", "Keine weitere Verlaufsseite.";
    source_history_confirm_error: "Enter the exact inspected selection identifier.", "Die genaue Kennung der geprüften Auswahl eingeben.";
    source_history_pending: "Reconcile the pending transition before archiving.", "Vor dem Archivieren den ausstehenden Übergang abgleichen.";
    source_history_confirmation: "{action} · selection {selection} · type {selection} and press Enter; Esc cancels", "{action} · Auswahl {selection} · {selection} eingeben und Eingabetaste drücken; Esc bricht ab";
    source_history_summary: "History {active} · {segments} segments · {names} names · {bytes} journal/save bytes (excluding retained removals) · {unresolved} unresolved · {quarantined} quarantined · rollover {rollover} · pending {pending}", "Verlauf {active} · {segments} Abschnitte · {names} Namen · {bytes} Journal-/Sicherungsbytes (ohne aufbewahrte entfernte Nutzdaten) · {unresolved} ungeklärt · {quarantined} gesondert aufbewahrt · Wechsel {rollover} · ausstehend {pending}";
    source_history_keys: "Next {next} · inspect {inspect} · reconcile {reconcile} · archive {archive}", "Weiter {next} · prüfen {inspect} · abgleichen {reconcile} · archivieren {archive}";
    source_reload_read_only: "read only · reload before editing", "schreibgeschützt · vor Bearbeitung neu laden";
    source_read_only: "read only", "schreibgeschützt";
    action_inspect_recovery: "inspect retained version", "aufbewahrte Version prüfen";
    action_create_file: "new file", "neue Datei";
    action_create_directory: "new folder", "neuer Ordner";
    action_rename_entry: "rename entry", "Eintrag umbenennen";
    action_retain_entry: "remove and retain entry", "Eintrag entfernen und aufbewahren";
    source_choose_entry: "Browse workspace files and select an entry first", "Zuerst Arbeitsbereich-Dateien öffnen und einen Eintrag wählen";
    source_name: "Name", "Name";
    source_management_form: "{action} {path} · Enter applies · Esc cancels", "{action} {path} · Enter führt aus · Esc bricht ab";
    source_confirm_retain: "Remove {path} and retain it in local history? Type {name} exactly and press Enter; Esc cancels.", "{path} entfernen und im lokalen Verlauf aufbewahren? Genau {name} eingeben und Enter drücken; Esc bricht ab.";
    source_management_partial: "Command did not settle; an actual retained item is available at {path}. Inspect it before continuing.", "Befehl wurde nicht abgeschlossen; ein tatsächlich aufbewahrter Eintrag liegt unter {path}. Vor dem Fortfahren prüfen.";
    source_retained: "Entry retained at {path}; inspect with {inspect}", "Eintrag unter {path} aufbewahrt; mit {inspect} prüfen";
    title_files: "Workspace Files", "Arbeitsbereich-Dateien";
    source_listing: "listing", "auflisten";
    source_reading: "reading", "lesen";
    source_staging: "preparing save", "Speichern vorbereiten";
    source_checking: "checking current version", "aktuelle Version prüfen";
    source_publishing: "saving", "speichern";
    source_complete: "complete", "abgeschlossen";
    source_worker_failed: "File worker ended unexpectedly; buffer retained", "Dateiarbeiter wurde unerwartet beendet; Puffer bleibt erhalten";
    source_busy: "File operation is settling; latest request queued", "Dateivorgang wird beendet; letzte Anfrage wartet";
    source_path_error: "An owned relative file path is required", "Ein zugehöriger relativer Dateipfad ist erforderlich";
    source_boundary_error: "The path crosses the workspace or a symbolic link", "Der Pfad überschreitet den Arbeitsbereich oder einen symbolischen Link";
    source_not_file: "The selected path is not a regular file", "Der gewählte Pfad ist keine reguläre Datei";
    source_management_too_large: "A managed file exceeds the 64 MiB command limit", "Eine verwaltete Datei überschreitet das Befehlslimit von 64 MiB";
    source_too_large: "The file exceeds the editable byte limit", "Die Datei überschreitet das bearbeitbare Bytelimit";
    source_too_many: "The directory or tree exceeds the 4096-entry limit", "Das Verzeichnis oder der Baum überschreitet das Limit von 4096 Einträgen";
    source_not_text: "The file is not supported UTF8 text", "Die Datei ist kein unterstützter UTF8-Text";
    source_partial_publication: "Save could not settle. Retained bytes are available at {path}; local buffer is retained. Inspect the retained version before continuing.", "Speichern konnte nicht abgeschlossen werden. Aufbewahrte Bytes liegen unter {path}; der lokale Puffer bleibt erhalten. Vor dem Fortfahren die aufbewahrte Version prüfen.";
    source_publication_race: "Save raced with another writer; your version was published and the predecessor is retained at {path}. Buffer retained.", "Speichern kollidierte mit anderem Schreiber; Ihre Version wurde veröffentlicht und die Vorgängerversion liegt unter {path}. Puffer bleibt erhalten.";
    source_conflict: "File changed elsewhere; buffer retained. Reload after saving your changes elsewhere or explicitly discard.", "Datei wurde andernorts geändert; Puffer bleibt erhalten. Änderungen anderweitig sichern oder ausdrücklich verwerfen und neu laden.";
    source_locked: "Another writer owns the file; try again after it finishes", "Ein anderer Schreiber besitzt die Datei; nach dessen Ende erneut versuchen";
    source_io_error: "Filesystem access failed", "Dateisystemzugriff fehlgeschlagen";
    source_unsupported: "Safe file publication is unavailable on this filesystem", "Sichere Dateiveröffentlichung ist auf diesem Dateisystem nicht verfügbar";
    source_starting: "File operation starting; {cancel} cancels", "Dateivorgang startet; {cancel} bricht ab";
    source_history_full: "Document history reached its safe storage limit; file and local buffer are retained", "Der Dokumentverlauf hat sein sicheres Speicherlimit erreicht; Datei und lokaler Puffer bleiben erhalten";
    source_loading: "File operation: {stage} {completed}/{total} · {cancel} cancels", "Dateivorgang: {stage} {completed}/{total} · {cancel} bricht ab";
    source_failed: "File operation failed: {problem}", "Dateivorgang fehlgeschlagen: {problem}";
    source_dirty: "unsaved", "ungespeichert";
    source_saved: "saved", "gespeichert";
    source_document_cancelled: "File cancellation requested; buffer retained", "Dateiabbruch angefordert; Puffer bleibt erhalten";
    source_confirm_discard: "Unsaved changes retained. Save with {save}, or press {discard} again to discard.", "Ungespeicherte Änderungen bleiben erhalten. Mit {save} speichern oder zum Verwerfen erneut {discard} drücken.";
    source_reloaded: "File loaded", "Datei geladen";
    source_empty: "No matching files", "Keine passenden Dateien";
    source_filter: "Filter", "Filter";
    source_position: "line {line} · column {column}", "Zeile {line} · Spalte {column}";
    source_limit: "Document byte limit reached; edit refused", "Dokument-Bytelimit erreicht; Bearbeitung abgelehnt";
    action_show_files: "browse workspace files", "Arbeitsbereich-Dateien durchsuchen";
    action_save_document: "save document", "Dokument speichern";
    action_reload_document: "reload document", "Dokument neu laden";
    action_discard_document: "discard document changes", "Dokumentänderungen verwerfen";
    action_cancel_source: "cancel file operation", "Dateivorgang abbrechen";
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
    open_starting: "Opening ready address; {cancel} cancels", "Bereite Adresse öffnen; {cancel} bricht ab";
    open_finished: "Ready address opened", "Bereite Adresse geöffnet";
    open_unavailable: "The selected task has no active ready address", "Die ausgewählte Aufgabe hat keine aktive bereite Adresse";
    open_failed: "Cannot open ready address: {problem}", "Bereite Adresse lässt sich nicht öffnen: {problem}";
    open_cancelled: "Opening cancelled; task keeps running", "Öffnen abgebrochen; Aufgabe läuft weiter";
    status_pending: "waiting", "wartet";
    status_running: "running", "läuft";
    status_ready: "ready", "bereit";
    status_stopping: "stopping", "stoppt";
    status_exited: "exited", "beendet";
    status_failed: "failed", "fehlgeschlagen";
    status_interrupted: "interrupted", "unterbrochen";
    status_timed_out: "time limit exceeded", "Zeitlimit überschritten";
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
    source_help: "Type to filter; {previous} / {next} select; {accept} accepts; {refresh} refreshes; {cancel} cancels", "Tippen zum Filtern; {previous} / {next} wählen; {accept} übernimmt; {refresh} aktualisiert; {cancel} bricht ab";
    source_progress: "Source suggestions: {count} inspected", "Quellvorschläge: {count} geprüft";
    source_scanning: "searching…", "Suche läuft…";
    source_limited: "limited results; choose an owner, narrow the search or type an explicit value", "begrenzte Ergebnisse; Eigentümer wählen, Suche eingrenzen oder Wert direkt eingeben";
    source_cancelled: "cancelled; refresh to resume", "abgebrochen; zum Fortsetzen aktualisieren";
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
    help_customize: "Customize with: repo dashboard preferences set --prefix KEY --bindings JSON", "Anpassen mit: repo dashboard preferences set --prefix TASTE --bindings JSON";
    chrome_close: "Close tab", "Tab schließen";
    chrome_drag: "Drag window", "Fenster ziehen";
    chrome_maximize: "Maximize window", "Fenster maximieren";
    chrome_restore: "Restore window", "Fenster wiederherstellen";
    chrome_new_tab: "New task in this stack", "Neue Aufgabe in diesem Stapel";
    chrome_previous_tabs: "Earlier tabs", "Frühere Tabs";
    chrome_next_tabs: "Later tabs", "Spätere Tabs";
    skew_incompatible: "daemon speaks protocol {daemon} but this client speaks {client}; stop the daemon once its tasks finish (repo dashboard daemon stop) and start it again", "Der Dienst spricht Protokoll {daemon}, dieser Client {client}; Dienst beenden, sobald seine Aufgaben fertig sind (repo dashboard daemon stop), und neu starten";
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
    cli_daemon_usage: "usage: repo dashboard daemon start|stop|status|attach|serve", "Verwendung: repo dashboard daemon start|stop|status|attach|serve";
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
    action_open_ready: "Open ready address", "Bereite Adresse öffnen";
    action_cancel_open_ready: "Cancel opening", "Öffnen abbrechen";
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
    action_tab_top_left: "place tab at top left", "Tab oben links platzieren";
    action_tab_top_right: "place tab at top right", "Tab oben rechts platzieren";
    action_tab_bottom_left: "place tab at bottom left", "Tab unten links platzieren";
    action_tab_bottom_right: "place tab at bottom right", "Tab unten rechts platzieren";
    action_dock_left: "move tab left of next window", "Tab links vom nächsten Fenster";
    action_dock_right: "move tab right of next window", "Tab rechts vom nächsten Fenster";
    action_dock_top: "move tab above next window", "Tab über nächstes Fenster";
    action_dock_bottom: "move tab below next window", "Tab unter nächstes Fenster";
    action_join_window: "join next window as a tab", "als Tab in nächstes Fenster";
    action_tab_left: "move tab left", "Tab nach links";
    action_tab_right: "move tab right", "Tab nach rechts";
    dock_top_left: "tab at top left", "Tab oben links";
    dock_top_right: "tab at top right", "Tab oben rechts";
    dock_bottom_left: "tab at bottom left", "Tab unten links";
    dock_bottom_right: "tab at bottom right", "Tab unten rechts";
    dock_tabs: "join tabs", "Tabs zusammenführen";
    dock_left: "split left", "links teilen";
    dock_right: "split right", "rechts teilen";
    dock_top: "split above", "oben teilen";
    dock_bottom: "split below", "unten teilen";
    dock_preview: "{action} · Esc cancels · release to dock", "{action} · Esc bricht ab · zum Andocken loslassen";
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
    pub fn error(&self, code: Option<dashboard_wire::ipc::ErrorCode>, message: &str) -> String {
        use dashboard_wire::ipc::ErrorCode as Kind;
        let text = match code {
            Some(Kind::Protocol) => self.error_protocol, Some(Kind::Decode) => self.error_decode, Some(Kind::Invalid) => self.error_invalid, Some(Kind::HelloRequired) => self.error_hello_required,
            Some(Kind::UnknownSession) => self.error_unknown_session, Some(Kind::UnknownGroup) => self.error_unknown_group, Some(Kind::AlreadyRunning) => self.error_already_running, Some(Kind::Limit) => self.error_limit, Some(Kind::ViewLimit) => self.error_view_limit,
            Some(Kind::Spawn) => self.error_spawn, Some(Kind::NotRunning) => self.error_not_running, Some(Kind::InputBacklog) => self.error_input_backlog, None => self.error_unspecified,
        };
        text.fill(&[("message", message)]).into_string()
    }

    /// ⚖️ The sentence that names how the daemon differs from this client, in the language of the dashboard.
    pub fn skew(&self, skew: &dashboard_wire::client::Skew) -> String {
        if skew.incompatible() { self.skew_incompatible.fill(&[("daemon", &skew.daemon_protocol.to_string()), ("client", &skew.client_protocol.to_string())]).into_string() } else { self.skew_build.fill(&[("daemon", &skew.daemon_build), ("client", &skew.client_build)]).into_string() }
    }

    /// 🗣️ What an action does, by the id the keymap states; an id without a text shows as itself.
    pub fn action<'a>(&self, id: &'a str) -> &'a str { self.try_action(id).unwrap_or(id) }

    /// 🔎️ What an action does, or `None` for an id this catalogue has no text for.
    pub fn try_action(&self, id: &str) -> Option<&'static str> {
        let text = match id {
            "inspect-recovery" => self.action_inspect_recovery,
            "inspect-history"=>self.action_inspect_history,"reconcile-history"=>self.action_reconcile_history,"archive-history"=>self.action_archive_history,"next-history-page"=>self.action_next_history_page,
            "create-file"=>self.action_create_file,"create-directory"=>self.action_create_directory,"rename-entry"=>self.action_rename_entry,"retain-entry"=>self.action_retain_entry,
            "show-files" => self.action_show_files, "save-document" => self.action_save_document, "reload-document" => self.action_reload_document, "discard-document" => self.action_discard_document, "cancel-source" => self.action_cancel_source,
            "new-task" => self.action_new_task, "show-tasks" => self.action_show_tasks, "show-settings" => self.action_show_settings, "show-help" => self.action_show_help,
            "show-hidden-tasks" => self.action_show_hidden_tasks, "refresh-commands" => self.action_refresh_commands, "cancel-discovery" => self.action_cancel_discovery,
            "search-output" => self.action_search_output, "restart-task" => self.action_restart_task, "stop-task" => self.action_stop_task, "kill-task" => self.action_kill_task, "copy-selection" => self.action_copy_selection,
            "open-ready" => self.action_open_ready, "cancel-open-ready" => self.action_cancel_open_ready,
            "toggle-input" => self.action_toggle_input, "toggle-appearance" => self.action_toggle_appearance, "toggle-language" => self.action_toggle_language,
            "next-window" => self.action_next_window, "previous-window" => self.action_previous_window, "split-down" => self.action_split_down, "split-right" => self.action_split_right,
            "dock-left" => self.action_dock_left, "dock-right" => self.action_dock_right, "dock-top" => self.action_dock_top, "dock-bottom" => self.action_dock_bottom,
            "tab-top-left" => self.action_tab_top_left, "tab-top-right" => self.action_tab_top_right, "tab-bottom-left" => self.action_tab_bottom_left, "tab-bottom-right" => self.action_tab_bottom_right,
            "join-window" => self.action_join_window, "tab-left" => self.action_tab_left, "tab-right" => self.action_tab_right,
            "grow-window" => self.action_grow_window, "shrink-window" => self.action_shrink_window, "zoom" => self.action_zoom, "close-window" => self.action_close_window,
            "detach" => self.action_detach, "shutdown" => self.action_shutdown, "cancel-prefix" => self.action_cancel_prefix, "send-prefix" => self.action_send_prefix,
            "up" => self.action_up, "down" => self.action_down, "left" => self.action_left, "right" => self.action_right, "page-up" => self.action_page_up, "page-down" => self.action_page_down,
            "first" => self.action_first, "last" => self.action_last, "delete-word" => self.action_delete_word, "activate" => self.action_activate, "clear" => self.action_clear, "back" => self.action_back,
            _ => return None,
        };
        Some(text.as_str())
    }
}

// #region 🔖️Labels
/// 📐️ The cells a tab may take, glyph and close control included.
pub const TAB_CELLS: usize = 24;
const TAIL_CELLS: usize = 6;

/// 🗣️ The word a verb is shown as; identifiers stay language-neutral.
pub fn verb_text(verb: &str, locale: Locale) -> &str {
    if locale != Locale::De { return verb; }
    match verb {
        "setup" => "einrichten", "start" => "starten", "dev" => "entwickeln", "serve" => "ausliefern", "watch" => "beobachten", "activate" => "aktivieren", "prepare" => "vorbereiten", "run" => "ausführen", "build" => "bauen", "package" => "paketieren", "test" => "testen", "smoke" => "rauchtest", "check" => "prüfen", "typecheck" => "typprüfung", "verify" => "verifizieren", "gate" => "freigabe", "lint" => "linten", "format" => "formatieren", "generate" => "generieren", "publish" => "veröffentlichen", "deploy" => "bereitstellen", "preview" => "vorschau", "bench" => "messen", "clean" => "aufräumen", "task" => "aufgabe", "goals" => "ziele", "analyze" => "analysieren", "tree" => "baum", "statutes" => "statuten",
        other => other,
    }
}

fn cells(text: &str) -> usize { usize::from(semio_framework_ui_tui_core::text::display_width(text)) }

/// 📏️ A cell budget as the shared text helpers take it.
fn cap(budget: usize) -> u16 { u16::try_from(budget).unwrap_or(u16::MAX) }

/// 📑️ The tab text of a task, `verb subject [qualifier] [×members]`, within `budget` cells: the
/// subject gives way first (middle elided, last cells kept), then the qualifier; the verb never does.
pub fn tab_text(label: &TaskLabel, locale: Locale, budget: usize) -> String {
    let verb = verb_text(&label.verb, locale);
    let members = if label.members > 0 { format!(" ×{}", label.members) } else { String::new() };
    let join = |subject: &str, qualifier: &str| format!("{verb} {subject}{}{qualifier}{members}", if qualifier.is_empty() { "" } else { " " });
    let fixed = cells(verb) + 1 + cells(&members);
    let qualified = if label.qualifier.is_empty() { 0 } else { 1 + cells(&label.qualifier) };
    if fixed + cells(&label.subject) + qualified <= budget { return join(&label.subject, &label.qualifier); }
    let subject = elide_middle(&label.subject, cap(budget.saturating_sub(fixed + qualified).max(TAIL_CELLS + 2)), TAIL_CELLS as u16).into_owned();
    if fixed + cells(&subject) + qualified <= budget { return join(&subject, &label.qualifier); }
    let room = budget.saturating_sub(fixed + cells(&subject) + 1);
    let qualifier = if room < 2 { String::new() } else { elide_end(&label.qualifier, cap(room)).into_owned() };
    let text = join(&subject, &qualifier);
    if cells(&text) <= budget { text } else { format!("{verb} {}", elide_end(&text[verb.len() + 1..], cap(budget.saturating_sub(cells(verb) + 1).max(1)))) }
}

/// 🧮️ The tab texts of tasks in creation order: a text an earlier task already shows gets ` ·2`,
/// ` ·3`, … and stays within the budget.
pub fn tab_texts(labels: &[&TaskLabel], locale: Locale, budget: usize) -> Vec<String> {
    let plain: Vec<String> = labels.iter().map(|label| tab_text(label, locale, budget)).collect();
    plain.iter().enumerate().map(|(index, text)| match plain[..index].iter().filter(|earlier| *earlier == text).count() {
        0 => text.clone(),
        earlier => { let suffix = format!(" ·{}", earlier + 1); format!("{}{suffix}", tab_text(labels[index], locale, budget.saturating_sub(cells(&suffix)))) }
    }).collect()
}

/// 🪟️ The window title of a task: verb, owner path, subject, qualifier and chosen parameters.
pub fn window_title(label: &TaskLabel, locale: Locale) -> String {
    let mut parts = vec![verb_text(&label.verb, locale).to_string()];
    if !label.owner.is_empty() && label.owner.last() != Some(&label.subject) { parts.push(label.owner.join(" / ")); }
    parts.push(if label.members > 0 { format!("{} ×{}", label.subject, label.members) } else { label.subject.clone() });
    if !label.qualifier.is_empty() { parts.push(label.qualifier.clone()); }
    parts.extend(label.parameters.iter().map(|(key, value)| format!("{key}={value}")));
    parts.join(" · ")
}
// #endregion 🔖️Labels

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

impl DashboardLabels {
    pub fn source_stage(&self, stage: &Stage) -> String {
        let text = self;
        match stage { Stage::Listing => text.source_listing, Stage::Reading => text.source_reading, Stage::Staging => text.source_staging, Stage::Checking => text.source_checking, Stage::Publishing => text.source_publishing, Stage::Complete => text.source_complete }.as_str().into()
    }

    pub fn source_error(&self, error: &Error, management:bool) -> String {
        let text = self;
        if let Some(path) = &error.recovery_path { return (if error.kind == ErrorKind::Conflict { text.source_publication_race } else { text.source_partial_publication }).fill(&[("path", path)]).into_string(); }
        if error.kind == ErrorKind::TooLarge && error.path.is_empty() { return text.source_history_full.as_str().into(); }
        if management&&error.kind==ErrorKind::TooLarge{return self.source_management_too_large.as_str().into();}
        let label = match &error.kind { ErrorKind::InvalidPath => text.source_path_error, ErrorKind::Boundary => text.source_boundary_error, ErrorKind::NotFile => text.source_not_file, ErrorKind::TooLarge => text.source_too_large, ErrorKind::TooManyEntries => text.source_too_many, ErrorKind::NotText => text.source_not_text, ErrorKind::Conflict => text.source_conflict, ErrorKind::Cancelled => text.source_document_cancelled, ErrorKind::Busy => text.source_locked, ErrorKind::Io => text.source_io_error, ErrorKind::Unsupported => text.source_unsupported };
        format!("{}: {}", label.as_str(), error.path)
    }


    /// 🎮️ Guided entry command wording belongs to the same localized catalogue as its actions.
    pub fn source_management_caption(&self,kind:EntryAction,path:&str)->String{let label=match kind{EntryAction::CreateFile=>self.action_create_file,EntryAction::CreateDirectory=>self.action_create_directory,EntryAction::Rename=>self.action_rename_entry,EntryAction::Retain=>self.action_retain_entry};if kind==EntryAction::Retain{self.source_confirm_retain.fill(&[("path",path),("name",path.rsplit('/').next().unwrap_or(path))]).into_string()}else{self.source_management_form.fill(&[("action",label.as_str()),("path",path)]).into_string()}}
    /// 📈️ One bounded progress record renders without exposing a host implementation type.
    pub fn source_progress(&self,progress:&Progress,cancel:&str)->String{self.source_loading.fill(&[("stage",&self.source_stage(&progress.stage)),("completed",&progress.completed.to_string()),("total",&progress.total.map_or_else(||"?".into(),|total|total.to_string())),("cancel",cancel)]).into_string()}
    /// 📄️ Document captions distinguish saved drafts, pending reload and authenticated readonly content.
    pub fn source_document_caption(&self,current:&semio_framework_filesystem_workspace::commands::DocumentBuffer,position:&str)->String{let state=if current.read_only{if std::path::Path::new(&current.document.path).is_absolute(){self.source_read_only}else{self.source_reload_read_only}}else if current.dirty{self.source_dirty}else{self.source_saved};format!("{} · {} · {position}",current.document.path,state.as_str())}
    /// ⚠️ Management failures expose actual uncertain locations without claiming successful publication.
    pub fn source_failure(&self,error:&Error,management:bool)->String{let problem=if management{error.recovery_path.as_ref().map_or_else(||self.source_error(error,management),|path|self.source_management_partial.fill(&[("path",path)]).into_string())}else{self.source_error(error,management)};self.source_failed.fill(&[("problem",&problem)]).into_string()}

    /// 🚦️ Browser refusals render from closed domain categories in the chosen language.
    pub fn source_browser_error(&self,error:semio_framework_filesystem_workspace::commands::BrowserError,path:Option<&str>)->String{use semio_framework_filesystem_workspace::commands::BrowserError as E;match error{E::ConfirmRetain=>{let path=path.unwrap_or("");self.source_confirm_retain.fill(&[("path",path),("name",path.rsplit('/').next().unwrap_or(path))]).into_string()},_=>match error{E::ReadOnly=>self.source_read_only,E::Busy=>self.source_busy,E::InvalidPath=>self.source_path_error,E::MissingObservation=>self.source_conflict,E::EmptyHistory=>self.source_history_empty,E::PendingHistory=>self.source_history_pending,E::ConfirmHistory=>self.source_history_confirm_error,E::NoPage=>self.source_history_end,E::ConfirmRetain=>unreachable!()}.as_str().into()}}
    /// 🧮️ History summaries expose observed counts without inventing work or reclamation totals.
    pub fn source_history_notice(&self,inspection:Option<&semio_framework_filesystem_workspace::HistoryInspection>,keys:&[(&str,&str)])->String{let summary=inspection.map(|value|self.source_history_inspection(value)).unwrap_or_default();format!("{summary} {}",self.source_history_keys.fill(keys).as_str())}
    pub fn source_history_inspection(&self,value:&semio_framework_filesystem_workspace::HistoryInspection)->String{self.source_history_summary.fill(&[("active",&value.active),("segments",&value.segments.to_string()),("names",&value.names.to_string()),("bytes",&value.bytes.to_string()),("unresolved",&value.unresolved.to_string()),("quarantined",&value.quarantined.to_string()),("rollover",if value.rollover{self.form_on.as_str()}else{self.form_off.as_str()}),("pending",value.pending.as_deref().unwrap_or("—"))]).into_string()}
    /// 🗃️ Browser captions keep confirmation and readonly navigation wording together.
    pub fn source_browser_caption(&self,state:&semio_framework_filesystem_workspace::commands::BrowserState)->String{if let Some(form)=&state.history_form{let action=match form.action{semio_framework_filesystem_workspace::commands::HistoryAction::Reconcile=>self.action_reconcile_history,semio_framework_filesystem_workspace::commands::HistoryAction::Archive=>self.action_archive_history};return self.source_history_confirmation.fill(&[("action",action.as_str()),("selection",form.confirmation().unwrap_or("—"))]).into_string();}state.form.as_ref().map_or_else(||format!("{} · {}: {}",if state.history&&state.directory.is_empty(){self.source_history.as_str()}else if state.directory.is_empty(){"."}else{&state.directory},self.source_filter.as_str(),state.query),|form|self.source_management_caption(form.kind,&form.path))}

}
