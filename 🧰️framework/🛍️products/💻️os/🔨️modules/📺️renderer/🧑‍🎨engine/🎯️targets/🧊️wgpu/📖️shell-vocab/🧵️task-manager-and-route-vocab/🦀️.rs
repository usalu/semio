//! 🧵️ Task Manager and route-not-found chrome strings: React's `os.taskManager.*` bundle (`🧵️TaskManager/🟦️.tsx`) and `ui.common.routeNotFound`, "normal" labels, en then de, key for key.

const ROWS: &[(&str, &str, &str)] = &[
    ("status.routeAdmission.awaitIdentity", "Opening this space as soon as the hub confirms your session…", "Dieser Space öffnet sich, sobald der Hub deine Sitzung bestätigt…"),
    ("status.routeAdmission.awaitSignIn", "Sign in to the hub to open this space.", "Melde dich beim Hub an, um diesen Space zu öffnen."),
    ("status.routeAdmission.applyOffline", "The hub is not answering — this space opens with what this device has.", "Der Hub antwortet nicht — dieser Space öffnet sich mit dem, was dieses Gerät hat."),
    ("status.routeAdmission.signIn", "Sign in", "Anmelden"),
    ("status.sessionAuthority.pending", "Verifying shared access…", "Gemeinsamen Zugriff prüfen…"),
    ("status.sessionAuthority.cancel", "Cancel shared access", "Prüfung abbrechen"),
    ("status.sessionAuthority.unavailable", "Shared access is unavailable. Open a fresh authorized session from the secure launcher.", "Der gemeinsame Zugriff ist nicht verfügbar. Öffne über den sicheren Starter eine neu autorisierte Sitzung."),
    ("closeGuard.title", "Unsaved changes", "Nicht gespeicherte Änderungen"),
    ("closeGuard.body", "“{{name}}” has changes that were not saved. Close anyway and lose them?", "„{{name}}“ hat Änderungen, die nicht gespeichert wurden. Trotzdem schließen und sie verlieren?"),
    ("closeGuard.saving", "Saving “{{name}}” before closing…", "„{{name}}“ wird vor dem Schließen gespeichert…"),
    ("closeGuard.cancel", "Cancel", "Abbrechen"),
    ("saveFailure.folder-write-failed", "The folder refused the write.", "Der Ordner hat das Schreiben abgelehnt."),
    ("saveFailure.folder-unreadable", "The folder could not be read.", "Der Ordner konnte nicht gelesen werden."),
    ("saveFailure.external-divergence", "The folder changed outside this app.", "Der Ordner wurde außerhalb dieser App geändert."),
    ("saveFailure.hub-rejected", "The hub rejected the change.", "Der Hub hat die Änderung abgelehnt."),
    ("saveFailure.quota-exceeded", "The storage quota is exhausted.", "Das Speicherkontingent ist erschöpft."),
    ("saveFailure.generic", "The save was not acknowledged.", "Das Speichern wurde nicht bestätigt."),
    ("closeGuard.keep", "Keep editing", "Weiter bearbeiten"),
    ("closeGuard.discard", "Close and discard", "Schließen und verwerfen"),
    ("closeGuard.unsaved", "Unsaved changes", "Nicht gespeicherte Änderungen"),
    ("status.bootstrap.rebootstrap", "The server requires a fresh authoritative restore. Stale document UI was discarded while reconnecting.", "Der Server verlangt eine neue autoritative Wiederherstellung. Veraltete Dokumentansichten wurden beim Neuverbinden verworfen."),
    ("status.bootstrap.progress", "Restoring document: {{received}} of {{total}} bytes; {{chunks}} of {{totalChunks}} chunks.", "Dokument wird wiederhergestellt: {{received}} von {{total}} Bytes; {{chunks}} von {{totalChunks}} Blöcken."),
    ("status.bootstrap.cancel", "Cancel restore", "Wiederherstellung abbrechen"),
    ("taskManager.columns.actorId", "Actor", "Akteur"),
    ("taskManager.columns.packageId", "Package", "Paket"),
    ("taskManager.columns.lane", "Lane", "Spur"),
    ("taskManager.columns.status", "Status", "Status"),
    ("taskManager.columns.stage", "Stage", "Zustand"),
    ("taskManager.columns.shard", "Shard", "Shard"),
    ("taskManager.columns.wallUsP95", "p95 wall (µs)", "p95 Zeit (µs)"),
    ("taskManager.columns.mailboxLen", "Mailbox", "Postfach"),
    ("taskManager.columns.turns", "Turns", "Züge"),
    ("taskManager.columns.traps", "Traps", "Abstürze"),
    ("taskManager.columns.restarts", "Restarts", "Neustarts"),
    ("taskManager.columns.actions", "Actions", "Aktionen"),
    ("taskManager.lanes.interactive", "Interactive", "Interaktiv"),
    ("taskManager.lanes.userVisible", "User-visible", "Sichtbar"),
    ("taskManager.lanes.background", "Background", "Hintergrund"),
    ("taskManager.lanes.maintenance", "Maintenance", "Wartung"),
    ("taskManager.statuses.cold", "Cold", "Kalt"),
    ("taskManager.statuses.activating", "Activating", "Aktiviert…"),
    ("taskManager.statuses.active", "Active", "Aktiv"),
    ("taskManager.statuses.suspended", "Suspended", "Angehalten"),
    ("taskManager.statuses.draining", "Draining", "Wird beendet…"),
    ("taskManager.statuses.trapped", "Trapped", "Abgestürzt"),
    ("taskManager.statuses.quarantined", "Quarantined", "Isoliert"),
    ("taskManager.statuses.disabled", "Disabled", "Deaktiviert"),
    ("taskManager.actions.suspend", "Suspend", "Anhalten"),
    ("taskManager.actions.resume", "Resume", "Fortsetzen"),
    ("taskManager.actions.cancel", "Cancel", "Abbrechen"),
    ("taskManager.empty", "No actors are running.", "Es laufen keine Akteure."),
    ("taskManager.actorsTitle", "Actors", "Akteure"),
    ("taskManager.tasks.empty", "No task is running.", "Es läuft keine Aufgabe."),
    ("taskManager.tasks.lanes.job", "Plugin job", "Plugin-Auftrag"),
    ("taskManager.tasks.lanes.activation", "Plugin installation", "Plugin-Installation"),
    ("taskManager.tasks.lanes.toolCall", "Agent tool call", "Werkzeugaufruf des Agenten"),
    ("taskManager.tasks.lanes.toolRun", "Tool run", "Werkzeuglauf"),
    ("taskManager.tasks.lanes.documentTransfer", "Document import", "Dokumentimport"),
    ("taskManager.tasks.lanes.export", "Media export", "Medienexport"),
    ("taskManager.tasks.running", "Running", "Läuft"),
    ("taskManager.tasks.suspended", "Suspended", "Angehalten"),
    ("taskManager.tasks.cancelling", "Cancelling…", "Wird abgebrochen…"),
    ("taskManager.tasks.suspend", "Suspend", "Anhalten"),
    ("taskManager.tasks.suspendTask", "Suspend {{task}}", "{{task}} anhalten"),
    ("taskManager.tasks.resume", "Resume", "Fortsetzen"),
    ("taskManager.tasks.resumeTask", "Resume {{task}}", "{{task}} fortsetzen"),
    ("taskManager.tasks.cancel", "Cancel", "Abbrechen"),
    ("taskManager.tasks.cancelTask", "Cancel {{task}}", "{{task}} abbrechen"),
    ("taskManager.tasks.progressSteps", "{{steps}} steps · {{seconds}} s", "{{steps}} Schritte · {{seconds}} s"),
    ("taskManager.tasks.progressElapsed", "{{seconds}} s", "{{seconds}} s"),
    ("taskManager.tasks.progressLabel", "Progress of {{task}}", "Fortschritt von {{task}}"),
    ("common.routeNotFound", "Route not found: {{path}}", "Route nicht gefunden: {{path}}"),
    ("surfaceContextMenu.placementSuggestions", "Placement suggestions", "Platzierungsvorschläge"),
    ("host.checkingPlacement", "Checking collision-free placements…", "Prüfe kollisionsfreie Platzierungen…"),
    ("host.noPlacement", "No collision-free placement at this connector", "Keine kollisionsfreie Platzierung an diesem Verbinder"),
    ("plugins.recovery.title", "Plugin Recovery", "Plugin-Wiederherstellung"),
    ("plugins.recovery.crashed", "This program crashed.", "Dieses Programm ist abgestürzt."),
    ("plugins.recovery.quarantined", "This program was quarantined after repeated crashes.", "Dieses Programm wurde nach wiederholten Abstürzen isoliert."),
    ("plugins.recovery.restartApp", "Restart App", "App neu starten"),
    ("plugins.recovery.disablePlugin", "Disable Plugin", "Plugin deaktivieren"),
];

/// 🧵️ EN/DE lookup of the Task Manager and route-not-found strings; `None` for any other key.
pub(super) fn shell_chrome_task_manager_and_route(key: &'static str, is_de: bool) -> Option<&'static str> {
    ROWS.iter().find(|(row, _, _)| *row == key).map(|(_, en, de)| if is_de { *de } else { *en })
}

/// 🧵️ A chrome string with its `{{name}}` placeholders filled from `values`.
pub fn shell_chrome_format(key: &'static str, is_de: bool, values: &[(&str, &str)]) -> String {
    values.iter().fold(super::shell_chrome_string(key, is_de).to_string(), |text, (name, value)| text.replace(&format!("{{{{{name}}}}}"), value))
}

/// 🧯️ React's `saveFailureTextV1`: a known code in words; any other code reads as the generic failure, followed by the failure's own message when it carries one.
pub fn shell_save_failure_text(code: &str, message: Option<&str>, is_de: bool) -> String {
    let known = [("folder-write-failed", "saveFailure.folder-write-failed"), ("folder-unreadable", "saveFailure.folder-unreadable"), ("external-divergence", "saveFailure.external-divergence"), ("hub-rejected", "saveFailure.hub-rejected"), ("quota-exceeded", "saveFailure.quota-exceeded")];
    match known.iter().find(|(known_code, _)| *known_code == code) {
        Some((_, key)) => super::shell_chrome_string(key, is_de).to_string(),
        None => {
            let generic = super::shell_chrome_string("saveFailure.generic", is_de);
            message.filter(|message| !message.is_empty()).map_or_else(|| generic.to_string(), |message| format!("{generic} {message}"))
        }
    }
}
