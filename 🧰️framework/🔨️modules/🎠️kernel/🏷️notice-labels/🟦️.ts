/** 🏷️ Localized notices: history and framework fault notice labels and the lookup that answers a code in both languages.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import type { ShellLocale, ShellTerminology } from "@semio-tech/framework-manifest/generated/ui-axes";
import { faultNoticeText, fillFaultNotice, type FaultNoticeDefinition } from "@semio-tech/framework-manifest";

// #region 🎠️Kernel
//#region InvocationResponse
/** 📢️ Framework-owned EN/DE notices of the store refusals a history-lane dispatch answers outside a history-edit session,
 * mirrored from Rust `HISTORY_NOTICE_LABELS`; `{n}` is the edit count a refusal names. */
export const HISTORY_NOTICE_LABELS = [
  { code: "toolTransaction.open", en: "A tool is still recording — finish or cancel it first.", de: "Ein Werkzeug zeichnet noch auf — zuerst abschließen oder abbrechen." },
  { code: "toolTransaction.unknown", en: "The tool's recording has already ended.", de: "Die Aufzeichnung des Werkzeugs ist bereits beendet." },
  { code: "history.full", en: "This document's history is full ({n} edits).", de: "Der Verlauf dieses Dokuments ist voll ({n} Bearbeitungen)." },
  { code: "history.replaying", en: "History is still replaying — wait for it or cancel it first.", de: "Der Verlauf wird noch neu angewendet — abwarten oder zuerst abbrechen." },
  { code: "document.loading", en: "The document is still loading — wait for it or cancel it first.", de: "Das Dokument wird noch geladen — abwarten oder zuerst abbrechen." },
  { code: "pure.history-unavailable", en: "This is a head-only evaluation without history — open the document in a live instance to use its history.", de: "Dies ist eine Auswertung nur des aktuellen Stands ohne Verlauf — für den Verlauf das Dokument in einer laufenden Instanz öffnen." },
  { code: "history.step-blocked", en: "Later mutations would end with errors — fix or withdraw them first.", de: "Spätere Mutationen würden mit Fehlern enden — zuerst beheben oder zurückziehen." },
  { code: "history.unit-spans-documents", en: "This step also changed other documents, so it cannot be withdrawn on its own yet.", de: "Dieser Schritt hat auch andere Dokumente geändert und lässt sich daher noch nicht einzeln zurückziehen." },
  { code: "history.ledger-not-replayable", en: "This document's history does not replay without errors yet — fix or withdraw the mutations that end with errors, then check in again.", de: "Der Verlauf dieses Dokuments lässt sich noch nicht fehlerfrei neu anwenden — die Mutationen mit Fehlern beheben oder zurückziehen, dann erneut einchecken." },
] as const;

/** 🔔️ The `{en, de}` notice of a history-lane refusal `code`; `undefined` for any other code. */
export function historyNotice(code: string): Readonly<{ en: string; de: string }> | undefined {
  const row = HISTORY_NOTICE_LABELS.find((notice) => notice.code === code);
  return row === undefined ? undefined : { en: row.en, de: row.de };
}

/** 🏢️ Framework-owned EN/DE notices of the refusal codes any app raises under a framework namespace (`app.command.*`,
 * `mutation.*`, design §20.12), mirrored from Rust `FRAMEWORK_FAULT_NOTICE_LABELS`; told with the fault's own severity. */
export const FRAMEWORK_FAULT_NOTICE_LABELS = [
  { code: "app.command.unsupported", en: "This action is not available here.", de: "Diese Aktion ist hier nicht verfügbar." },
  { code: "app.command.invalid", en: "This action is not valid.", de: "Diese Aktion ist ungültig." },
  { code: "app.command.invalid-args", en: "The action's input is not valid.", de: "Die Eingabe der Aktion ist ungültig." },
  { code: "app.command.invalid-payload", en: "The action's data is not valid.", de: "Die Daten der Aktion sind ungültig." },
  { code: "app.command.targets-required", en: "Select at least one item first.", de: "Zuerst mindestens ein Element auswählen." },
  { code: "app.command.kind-unavailable", en: "This kind of item is not available.", de: "Diese Art von Element ist nicht verfügbar." },
  { code: "app.command.target-in-use", en: "The item is still in use.", de: "Das Element wird noch verwendet." },
  { code: "app.command.tool-mismatch", en: "This action does not belong to the active tool.", de: "Diese Aktion gehört nicht zum aktiven Werkzeug." },
  { code: "toolTransaction.closed", en: "The tool's recording had already ended — this step was not recorded.", de: "Die Aufzeichnung des Werkzeugs war bereits beendet — dieser Schritt wurde nicht aufgezeichnet." },
  { code: "toolTransaction.unclosed", en: "The tool stopped without finishing its recording — this step was not recorded.", de: "Das Werkzeug hat angehalten, ohne seine Aufzeichnung abzuschließen — dieser Schritt wurde nicht aufgezeichnet." },
  { code: "toolTransaction.slot-poisoned", en: "The tool could not continue this gesture — release and try again.", de: "Das Werkzeug konnte diese Geste nicht fortsetzen — loslassen und erneut versuchen." },
  { code: "mutation.target-missing", en: "The target no longer exists.", de: "Das Ziel existiert nicht mehr." },
  { code: "mutation.target-mismatch", en: "The change does not fit the target's current state.", de: "Die Änderung passt nicht zum aktuellen Zustand des Ziels." },
  { code: "mutation.too-large", en: "This change is too large to record at once — split it into smaller steps.", de: "Diese Änderung ist zu groß, um sie auf einmal aufzuzeichnen — in kleinere Schritte aufteilen." },
  { code: "plugin.media.schema-mismatch", en: "This input is a {found} document, but only {expected} documents can be loaded here.", de: "Diese Eingabe ist ein {found}-Dokument, hier lassen sich aber nur {expected}-Dokumente laden." },
  { code: "plugin.channel-mismatch", en: "This plugin was built for app channel {guest}, but this app speaks app channel {host} — rebuild the plugin.", de: "Dieses Plugin wurde für App-Kanal {guest} gebaut, diese App spricht aber App-Kanal {host} — Plugin neu bauen." },
  { code: "history-filter.unknown", en: "This history filter is not known — choose one the history panel offers.", de: "Dieser Verlaufsfilter ist unbekannt — einen im Verlaufsbereich angebotenen wählen." },
  { code: "window-transient.window-required", en: "This needs an open window — focus a window first.", de: "Dafür wird ein offenes Fenster benötigt — zuerst ein Fenster fokussieren." },
  { code: "window-transient.window-stale", en: "The window is no longer open.", de: "Das Fenster ist nicht mehr geöffnet." },
  { code: "window-transient.kind-unknown", en: "This window cannot hold this state — use a window of the matching kind.", de: "Dieses Fenster kann diesen Zustand nicht halten — ein Fenster der passenden Art verwenden." },
  { code: "timeTravel.member-owners", en: "A history edit could not finish closing a composed part — try again.", de: "Eine Verlaufsbearbeitung konnte einen zusammengesetzten Teil nicht fertig schließen — erneut versuchen." },
  { code: "timeTravel.snapshot-close", en: "The history view could not close its snapshot cleanly — try again.", de: "Die Verlaufsansicht konnte ihren Schnappschuss nicht sauber schließen — erneut versuchen." },
  { code: "timeTravel.snapshot-retirement", en: "The history view could not release its snapshot — try again.", de: "Die Verlaufsansicht konnte ihren Schnappschuss nicht freigeben — erneut versuchen." },
  { code: "timeTravel.unknown-action", en: "This history action is not known.", de: "Diese Verlaufsaktion ist unbekannt." },
  { code: "timeTravel.member-store-kind", en: "This part keeps its history differently and cannot be edited here.", de: "Dieser Teil führt seinen Verlauf anders und kann hier nicht bearbeitet werden." },
  { code: "timeTravel.preview-mismatch", en: "The history preview answered a different request — try again.", de: "Die Verlaufsvorschau hat auf eine andere Anfrage geantwortet — erneut versuchen." },
  { code: "toolRun.unknown-action", en: "This tool run action is not known.", de: "Diese Werkzeuglauf-Aktion ist unbekannt." },
  { code: "toolRun.tool-id", en: "Choose a tool before starting a run.", de: "Vor dem Start eines Laufs ein Werkzeug wählen." },
  { code: "toolRun.unknown-tool", en: "This tool cannot run step by step.", de: "Dieses Werkzeug kann nicht schrittweise laufen." },
  { code: "toolRun.busy", en: "A tool run is still active — finish or cancel it first.", de: "Ein Werkzeuglauf ist noch aktiv — ihn zuerst beenden oder abbrechen." },
  { code: "toolRun.tick-bytes", en: "A tool run step is too large to show.", de: "Ein Schritt des Werkzeuglaufs ist zu groß zum Anzeigen." },
  { code: "toolRun.tick-decode", en: "A tool run step could not be read.", de: "Ein Schritt des Werkzeuglaufs konnte nicht gelesen werden." },
  { code: "toolRun.trace-lane", en: "The tool run's progress could not be shown.", de: "Der Fortschritt des Werkzeuglaufs konnte nicht angezeigt werden." },
  { code: "toolRun.member-unavailable", en: "This tool run needs exactly one open part to work on.", de: "Dieser Werkzeuglauf braucht genau einen geöffneten Teil zum Bearbeiten." },
  { code: "toolRun.member-gone", en: "The part this tool run edits is no longer there.", de: "Der Teil, den dieser Werkzeuglauf bearbeitet, ist nicht mehr vorhanden." },
  { code: "toolRun.member-store-kind", en: "This part keeps its history differently and cannot run this tool.", de: "Dieser Teil führt seinen Verlauf anders und kann dieses Werkzeug nicht ausführen." },
  { code: "toolRun.member-base-lost", en: "The tool run lost the state it started from — start it again.", de: "Der Werkzeuglauf hat seinen Ausgangszustand verloren — ihn erneut starten." },
  { code: "toolRun.job-close", en: "The tool run could not close cleanly — try again.", de: "Der Werkzeuglauf konnte nicht sauber beendet werden — erneut versuchen." },
  { code: "toolRun.snapshot-close", en: "The tool run could not close its snapshot cleanly — try again.", de: "Der Werkzeuglauf konnte seinen Schnappschuss nicht sauber schließen — erneut versuchen." },
  { code: "toolRun.snapshot-retirement", en: "The tool run could not release its snapshot — try again.", de: "Der Werkzeuglauf konnte seinen Schnappschuss nicht freigeben — erneut versuchen." },
  { code: "toolRun.publication-close", en: "The tool run could not finish publishing its result — try again.", de: "Der Werkzeuglauf konnte sein Ergebnis nicht fertig veröffentlichen — erneut versuchen." },
  { code: "toolRun.publication-handoff", en: "The tool run lost track of its result — try again.", de: "Der Werkzeuglauf hat den Überblick über sein Ergebnis verloren — erneut versuchen." },
  { code: "toolRun.publication-retirement", en: "The tool run could not release its result — try again.", de: "Der Werkzeuglauf konnte sein Ergebnis nicht freigeben — erneut versuchen." },
  { code: "toolTransaction.shape", en: "This tool change does not have the shape its tool declares.", de: "Diese Werkzeugänderung hat nicht die Form, die ihr Werkzeug angibt." },
  { code: "transaction.instance-busy", en: "Another linked change is still pending — wait for it to finish.", de: "Eine andere verknüpfte Änderung steht noch aus — warten, bis sie abgeschlossen ist." },
  { code: "transaction.member-rejected", en: "One part refused the linked change.", de: "Ein Teil hat die verknüpfte Änderung abgelehnt." },
  { code: "transaction.unknown-mutation", en: "A linked change holds an edit this part does not know.", de: "Eine verknüpfte Änderung enthält eine Bearbeitung, die dieser Teil nicht kennt." },
  { code: "transaction.generation-mismatch", en: "The part changed while the linked change was prepared — try again.", de: "Der Teil hat sich geändert, während die verknüpfte Änderung vorbereitet wurde — erneut versuchen." },
  { code: "transaction.commit-failed", en: "A linked change could not be completed in every part.", de: "Eine verknüpfte Änderung konnte nicht in allen Teilen abgeschlossen werden." },
  { code: "transaction.child-groups-malformed", en: "A linked change to the parts could not be read.", de: "Eine verknüpfte Änderung der Teile konnte nicht gelesen werden." },
  { code: "transaction.rollback-unknown", en: "There is no pending linked change to roll back.", de: "Es gibt keine ausstehende verknüpfte Änderung zum Zurücknehmen." },
  { code: "transaction.undo-foreign-tail", en: "The latest step of this part belongs to another change — undo that first.", de: "Der letzte Schritt dieses Teils gehört zu einer anderen Änderung — diese zuerst rückgängig machen." },
  { code: "transaction.redo-foreign-tail", en: "The next redo step of this part belongs to another change.", de: "Der nächste Wiederherstellungsschritt dieses Teils gehört zu einer anderen Änderung." },
  { code: "transaction.undo-failed", en: "This linked change could not be undone.", de: "Diese verknüpfte Änderung konnte nicht rückgängig gemacht werden." },
  { code: "transaction.redo-failed", en: "This linked change could not be redone.", de: "Diese verknüpfte Änderung konnte nicht wiederhergestellt werden." },
  { code: "transaction.group-history-dialect", en: "This composed document's type does not allow this history step.", de: "Der Typ dieses zusammengesetzten Dokuments erlaubt diesen Verlaufsschritt nicht." },
  { code: "transaction.group-history-root", en: "This history step would move a part it may not move.", de: "Dieser Verlaufsschritt würde einen Teil verschieben, den er nicht verschieben darf." },
  { code: "transaction.group-history-tail", en: "This change is no longer the latest step of any part.", de: "Diese Änderung ist bei keinem Teil mehr der letzte Schritt." },
  { code: "interactive-job.cancelled", en: "The action was cancelled.", de: "Die Aktion wurde abgebrochen." },
  { code: "interactive-job.tool-completion-busy", en: "The tool is still finishing — try again in a moment.", de: "Das Werkzeug wird noch abgeschlossen — gleich erneut versuchen." },
  { code: "interactive-job.maintenance-tool-authority", en: "A running tool changed during upkeep — try again.", de: "Ein laufendes Werkzeug hat sich während der Wartung geändert — erneut versuchen." },
  { code: "interactive-job.tool-document-retirement-invariant", en: "The tool could not release its document cleanly — try again.", de: "Das Werkzeug konnte sein Dokument nicht sauber freigeben — erneut versuchen." },
  { code: "interactive-job.publication-authority-missing", en: "This command publishes its result in a way this app does not support.", de: "Dieser Befehl veröffentlicht sein Ergebnis auf eine Weise, die diese App nicht unterstützt." },
  { code: "interactive-job.missing-owned-reducer", en: "This command has no editing step of its own in this app.", de: "Dieser Befehl hat in dieser App keinen eigenen Bearbeitungsschritt." },
  { code: "interactive-job.missing-factory", en: "This command is not registered as a tool of this app.", de: "Dieser Befehl ist nicht als Werkzeug dieser App registriert." },
  { code: "interactive-job.incomplete-operation-authority", en: "This command lacks a complete prepare, edit and commit path.", de: "Diesem Befehl fehlt ein vollständiger Ablauf aus Vorbereiten, Bearbeiten und Übernehmen." },
  { code: "interactive-job.catalog-controller", en: "This app's tools are registered to a different controller.", de: "Die Werkzeuge dieser App sind bei einer anderen Steuerung registriert." },
  { code: "interactive-job.catalog-authority", en: "A tool of this app is registered without its full authority.", de: "Ein Werkzeug dieser App ist ohne seine vollständige Berechtigung registriert." },
  { code: "interactive-job.catalog-incomplete", en: "A command of this app lacks its registered editing step.", de: "Einem Befehl dieser App fehlt sein registrierter Bearbeitungsschritt." },
  { code: "interactive-job.child-emission-retirement-refused", en: "A change to a part could not be released cleanly — try again.", de: "Eine Änderung an einem Teil konnte nicht sauber freigegeben werden — erneut versuchen." },
  { code: "plugin.document-load.unavailable", en: "This app cannot load or save whole documents.", de: "Diese App kann keine ganzen Dokumente laden oder speichern." },
  { code: "plugin.document-load.too-many-members", en: "The document has {count} parts, but at most {maximum} can be loaded.", de: "Das Dokument hat {count} Teile, es lassen sich aber höchstens {maximum} laden." },
  { code: "plugin.document-load.too-large", en: "The document is {bytes} bytes large, but at most {maximum} bytes can be loaded.", de: "Das Dokument ist {bytes} Bytes groß, es lassen sich aber höchstens {maximum} Bytes laden." },
  { code: "plugin.document-load.busy", en: "Another document is still loading. Try again when it has finished.", de: "Ein anderes Dokument wird noch geladen. Versuche es erneut, sobald es fertig ist." },
  { code: "plugin.document-load.incomplete", en: "The document is incomplete: its content or its history is missing.", de: "Das Dokument ist unvollständig: Inhalt oder Verlauf fehlt." },
  { code: "plugin.document-load.member-invalid", en: "Part {ordinal} of the document is damaged, so the document was not loaded.", de: "Teil {ordinal} des Dokuments ist beschädigt, daher wurde das Dokument nicht geladen." },
  { code: "plugin.document-load.history-invalid", en: "The history of the document cannot be read, so the document was not loaded.", de: "Der Verlauf des Dokuments lässt sich nicht lesen, daher wurde das Dokument nicht geladen." },
  { code: "plugin.document-load.other-document", en: "This is another document, so it cannot be merged into the open one.", de: "Das ist ein anderes Dokument, es lässt sich nicht mit dem geöffneten zusammenführen." },
  { code: "plugin.document-load.members-differ", en: "Parts of this document changed elsewhere, so it is loaded again instead of merged.", de: "Teile dieses Dokuments wurden anderswo geändert, daher wird es neu geladen statt zusammengeführt." },
  { code: "plugin.document-load.operation-unknown", en: "This load is no longer running.", de: "Dieser Ladevorgang läuft nicht mehr." },
  { code: "plugin.document-load.changed", en: "The document changed while it was being saved. Try again.", de: "Das Dokument hat sich beim Speichern geändert. Versuche es erneut." },
  { code: "plugin.document-load.failed", en: "The document could not be loaded. The previous document is unchanged.", de: "Das Dokument konnte nicht geladen werden. Das vorherige Dokument ist unverändert." },
  { code: "artifact-envelope.stale-handle", en: "The document changed while it was being prepared. Try again.", de: "Das Dokument hat sich während der Vorbereitung geändert. Versuche es erneut." },
  { code: "artifact-envelope.load-stale-handle", en: "The document changed while it was being loaded. Try again.", de: "Das Dokument hat sich während des Ladens geändert. Versuche es erneut." },
  { code: "artifact-store.replacement-stale-handle", en: "The document changed while it was being replaced. Try again.", de: "Das Dokument hat sich während des Ersetzens geändert. Versuche es erneut." },
  { code: "window-config.owner", en: "This window's settings belong to another window and were not loaded.", de: "Die Einstellungen dieses Fensters gehören zu einem anderen Fenster und wurden nicht geladen." },
  { code: "window-config.load-retirement", en: "This window's settings could not be replaced. The previous settings are unchanged.", de: "Die Einstellungen dieses Fensters ließen sich nicht ersetzen. Die bisherigen Einstellungen sind unverändert." },
] as const;

/** 🪧️ The `{en, de}` notice of a framework-namespace refusal `code`; `undefined` for any other code. */
export function frameworkFaultNotice(code: string): Readonly<{ en: string; de: string }> | undefined {
  const row = FRAMEWORK_FAULT_NOTICE_LABELS.find((notice) => notice.code === code);
  return row === undefined ? undefined : { en: row.en, de: row.de };
}

/** 📣️ The notice a refused dispatch earns (design §20.12) — TS twin of Rust `kernel::fault_notice`: the fault's own code, then each
 * cause's, looked up in the framework's tables ({@link HISTORY_NOTICE_LABELS}, {@link FRAMEWORK_FAULT_NOTICE_LABELS}) first and the
 * refusing app's published `faultNotices` second; the first declared code decides, its `{name}` placeholders filled from
 * `fault.params` only. `null` when no code is declared or a placeholder has no value — the shell then shows its generic refusal,
 * never a raw code. */
export function faultNotice(
  fault: Readonly<{ code: string; causes?: readonly { readonly code?: string }[]; params?: Readonly<Record<string, string>> }>,
  notices: readonly FaultNoticeDefinition[],
  terminology: ShellTerminology,
  locale: ShellLocale,
): Readonly<{ code: string; text: string }> | null {
  const frameworkNotice = (candidate: string): Readonly<{ en: string; de: string }> | undefined => historyNotice(candidate) ?? frameworkFaultNotice(candidate);
  const code = [fault.code, ...(fault.causes ?? []).map((cause) => cause.code)].find((candidate): candidate is string => candidate !== undefined && (frameworkNotice(candidate) !== undefined || notices.some((notice) => notice.code === candidate)));
  if (code === undefined) return null;
  const framework = frameworkNotice(code);
  const text = framework === undefined ? faultNoticeText(notices, code, fault.params, terminology, locale) : fillFaultNotice(framework[locale], fault.params);
  return text === null ? null : { code, text };
}
//#endregion InvocationResponse
// #endregion 🎠️Kernel
