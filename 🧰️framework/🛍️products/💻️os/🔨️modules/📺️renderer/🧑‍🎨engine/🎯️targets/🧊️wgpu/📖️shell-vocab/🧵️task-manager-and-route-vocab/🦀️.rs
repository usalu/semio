//! 🧵️ Task Manager and route-not-found chrome strings: React's `os.taskManager.*` bundle (`🧵️TaskManager/🟦️.tsx`) and `ui.common.routeNotFound`, "normal" labels, en then de, key for key.

const ROWS: &[(&str, &str, &str)] = &[
    ("status.routeAdmission.awaitIdentity", "Opening this space as soon as the hub confirms your session…", "Dieser Space öffnet sich, sobald der Hub deine Sitzung bestätigt…"),
    ("status.routeAdmission.awaitSignIn", "Sign in to the hub to open this space.", "Melde dich beim Hub an, um diesen Space zu öffnen."),
    ("status.routeAdmission.applyOffline", "The hub is not answering — this space opens with what this device has.", "Der Hub antwortet nicht — dieser Space öffnet sich mit dem, was dieses Gerät hat."),
    ("status.routeAdmission.signIn", "Sign in", "Anmelden"),
    ("status.bootstrap.rebootstrap", "The server requires a fresh authoritative restore. Stale document UI was discarded while reconnecting.", "Der Server verlangt eine neue autoritative Wiederherstellung. Veraltete Dokumentansichten wurden beim Neuverbinden verworfen."),
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
];

/// 🧵️ EN/DE lookup of the Task Manager and route-not-found strings; `None` for any other key.
pub(super) fn shell_chrome_task_manager_and_route(key: &'static str, is_de: bool) -> Option<&'static str> {
    ROWS.iter().find(|(row, _, _)| *row == key).map(|(_, en, de)| if is_de { *de } else { *en })
}

/// 🧵️ A chrome string with its `{{name}}` placeholders filled from `values`.
pub fn shell_chrome_format(key: &'static str, is_de: bool, values: &[(&str, &str)]) -> String {
    values.iter().fold(super::shell_chrome_string(key, is_de).to_string(), |text, (name, value)| text.replace(&format!("{{{{{name}}}}}"), value))
}
