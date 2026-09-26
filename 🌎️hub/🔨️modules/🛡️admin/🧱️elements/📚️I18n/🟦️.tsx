// #region 🧲️Header
// 💻️ hub/modules/admin/elements/📚️I18n/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
// #endregion 🔌️Adapters

// #region 🔖️Bundles
/** 📚️ The hub admin SPA's own translation bundle — deliberately separate from the shell's giant
 * `ui/🧱️elements/📚️I18n` chrome schema (a different domain: window/panel/ribbon chrome, not this
 * app's spaces/users/connections vocabulary). No default language — `en` and `de` are two equally
 * complete, hand-maintained bundles (contract-freeze §C0's "no default language" law). */
export type AdminLocale = "en" | "de";

const en = {
  "admin.nav.overview": "Overview",
  "admin.nav.spaces": "Spaces",
  "admin.nav.users": "Users",
  "admin.nav.connections": "Connections",
  "admin.nav.documents": "Documents",
  "admin.nav.events": "Events",
  "admin.nav.observability": "Observability",

  "admin.session.title": "Hub admin",
  "admin.session.description": "Administrator access is delivered by the protected local launcher.",
  "admin.session.probing": "Checking admin access…",
  "admin.session.error": "Administrator access is absent or expired. Start a fresh protected admin session:",
  "admin.session.unreachableTitle": "No hub is answering",
  "admin.session.unreachableDescription": "This page could not reach a hub. Start one, then reload:",
  "admin.session.unreachableHint": "A running hub also serves this admin page itself at /admin on its own port — the separate admin dev server is only for iterating on this UI.",

  "admin.overview.title": "Overview",
  "admin.overview.spaces": "Spaces",
  "admin.overview.users": "Users",
  "admin.overview.connections": "Live connections",
  "admin.overview.dataDirBytes": "Data directory size",
  "admin.overview.headSeq": "Directory head seq",
  "admin.overview.openArtifacts": "Open documents",
  "admin.overview.backends": "Compiled backends",
  "admin.overview.rebuild": "Rebuild projections",
  "admin.overview.rebuilding": "Rebuilding…",
  "admin.overview.rebuildSuccess": "Replayed {count} events.",
  "admin.overview.rebuildError": "Rebuild failed.",

  "admin.spaces.title": "Spaces",
  "admin.spaces.create": "New space",
  "admin.spaces.createTitle": "Create a space",
  "admin.spaces.name": "Name",
  "admin.spaces.kind": "Kind",
  "admin.spaces.visibility": "Visibility",
  "admin.spaces.owner": "Owner",
  "admin.spaces.members": "Members",
  "admin.spaces.documents": "Documents",
  "admin.spaces.actions": "Actions",
  "admin.spaces.rename": "Rename",
  "admin.spaces.setVisibility": "Toggle visibility",
  "admin.spaces.archive": "Archive",
  "admin.spaces.delete": "Delete",
  "admin.spaces.membersTitle": "Members",
  "admin.spaces.addMember": "Add member",
  "admin.spaces.email": "Email",
  "admin.spaces.role": "Role",
  "admin.spaces.remove": "Remove",
  "admin.spaces.inviteLink": "Invite link",
  "admin.spaces.inviteCreate": "Create invite",
  "admin.spaces.inviteCopy": "Copy",
  "admin.spaces.inviteCopied": "Copied.",
  "admin.spaces.empty": "No spaces yet.",
  "admin.spaces.loadMore": "Load more spaces",
  "admin.spaces.loadMoreMembers": "Load more members",
  "admin.spaces.confirmDelete": "Delete space \"{name}\"? This cannot be undone.",
  "admin.spaces.confirmArchive": "Archive space \"{name}\"?",
  "admin.spaces.cancel": "Cancel",
  "admin.spaces.save": "Save",
  "admin.spaces.roleAuthor": "Author",
  "admin.spaces.roleSpectator": "Spectator",
  "admin.spaces.kindAtelier": "Atelier",
  "admin.spaces.kindStudio": "Studio",
  "admin.spaces.kindArchive": "Archive",
  "admin.spaces.visibilityPrivate": "Private",
  "admin.spaces.visibilityPublic": "Public",

  "admin.users.title": "Users",
  "admin.users.email": "Email",
  "admin.users.displayName": "Display name",
  "admin.users.createdAt": "Created",
  "admin.users.empty": "No users yet.",
  "admin.users.revokeSessions": "Revoke sessions",

  "admin.connections.title": "Connections",
  "admin.connections.space": "Space",
  "admin.connections.document": "Document",
  "admin.connections.surface": "Surface",
  "admin.connections.actor": "Actor",
  "admin.connections.user": "User",
  "admin.connections.role": "Role",
  "admin.connections.connectedAt": "Connected",
  "admin.connections.kick": "Kick",
  "admin.connections.empty": "No live connections.",
  "admin.connections.fresh": "Fresh snapshot",
  "admin.connections.stale": "Stale snapshot",

  "admin.documents.title": "Documents",
  "admin.documents.space": "Space",
  "admin.documents.allSpaces": "All spaces",
  "admin.documents.id": "Document",
  "admin.documents.headSeq": "Head seq",
  "admin.documents.commitSeq": "Commit seq",
  "admin.documents.epoch": "Epoch",
  "admin.documents.activeConnections": "Active connections",
  "admin.documents.empty": "No documents yet.",

  "admin.events.title": "Events",
  "admin.events.since": "Since seq",
  "admin.events.refresh": "Refresh",
  "admin.events.empty": "No events yet.",
  "admin.events.kind": "Kind",
  "admin.events.actor": "Actor",
  "admin.events.time": "Time",
  "admin.events.loadMore": "Load newer",

  "admin.observability.title": "Observability",
  "admin.observability.refresh": "Refresh",
  "admin.observability.unavailable": "The hub did not answer its observability reading.",
  "admin.observability.uptime": "Uptime",
  "admin.observability.droppedEvents": "Dropped event names",
  "admin.observability.residencyTitle": "Compiled guests",
  "admin.observability.residencyNone": "This hub has no trusted catalog.",
  "admin.observability.residencyResident": "Resident",
  "admin.observability.residencyBudget": "Budget",
  "admin.observability.residencyHits": "Hits",
  "admin.observability.residencyCompiles": "Compiles",
  "admin.observability.residencyAdmitted": "Admitted",
  "admin.observability.residencyBypassed": "Served without staying",
  "admin.observability.residencyReleased": "Released",
  "admin.observability.residencyCompileTime": "Compile time",
  "admin.observability.catalogTitle": "Trusted catalog",
  "admin.observability.catalogSummary": "{ready} of {total} packages ready, {refused} refused · rows pinned {pinned}, verified {verified} of {rows}",
  "admin.observability.catalogPackage": "Package",
  "admin.observability.catalogPhase": "Phase",
  "admin.observability.catalogBytes": "Component",
  "admin.observability.catalogRows": "Rows (pinned / verified / declared)",
  "admin.observability.phase.pending": "Pending",
  "admin.observability.phase.reading": "Reading",
  "admin.observability.phase.staged": "Waiting for verification",
  "admin.observability.phase.verifying": "Verifying",
  "admin.observability.phase.ready": "Ready",
  "admin.observability.phase.refused": "Refused",
  "admin.observability.routesTitle": "Routes",
  "admin.observability.routesOverflowed": "{count} answers of routes beyond the table",
  "admin.observability.route": "Route",
  "admin.observability.requests": "Requests",
  "admin.observability.refusals": "Refusals (4xx / 429 / 503)",
  "admin.observability.failures": "Failures (5xx)",
  "admin.observability.latency": "p50 / p95 / p99 / max",
  "admin.observability.eventsTitle": "Trace events",
  "admin.observability.event": "Event",
  "admin.observability.outcomes": "ok / refused / failed / cancelled",
  "admin.observability.dbIoTitle": "DB I/O",
  "admin.observability.dbIoSummary": "{tasks} tasks · {steps} steps · {turns} worker turns",
  "admin.observability.dbIoKind": "Task kind",
  "admin.observability.dbIoCounts": "tasks / steps / turns",
  "admin.observability.dbIoAdmission": "admission waits / waited / refused",
  "admin.observability.dbIoAdmissionSummary": "{waits} submissions waited for admission ({waited}), {refusals} still refused",
} as const;

const de = {
  "admin.nav.overview": "Übersicht",
  "admin.nav.spaces": "Räume",
  "admin.nav.users": "Benutzer",
  "admin.nav.connections": "Verbindungen",
  "admin.nav.documents": "Dokumente",
  "admin.nav.events": "Ereignisse",
  "admin.nav.observability": "Beobachtbarkeit",

  "admin.session.title": "Hub-Administration",
  "admin.session.description": "Der geschützte lokale Starter stellt den Administratorzugriff bereit.",
  "admin.session.probing": "Admin-Zugriff wird geprüft…",
  "admin.session.error": "Der Administratorzugriff fehlt oder ist abgelaufen. Starte eine neue geschützte Administratorsitzung:",
  "admin.session.unreachableTitle": "Kein Hub erreichbar",
  "admin.session.unreachableDescription": "Diese Seite konnte keinen Hub erreichen. Starte einen und lade neu:",
  "admin.session.unreachableHint": "Ein laufender Hub liefert diese Admin-Seite auch selbst unter /admin auf seinem eigenen Port aus — der separate Admin-Dev-Server dient nur der Arbeit an dieser Oberfläche.",

  "admin.overview.title": "Übersicht",
  "admin.overview.spaces": "Räume",
  "admin.overview.users": "Benutzer",
  "admin.overview.connections": "Aktive Verbindungen",
  "admin.overview.dataDirBytes": "Größe des Datenverzeichnisses",
  "admin.overview.headSeq": "Verzeichnis-Head-Seq",
  "admin.overview.openArtifacts": "Offene Dokumente",
  "admin.overview.backends": "Kompilierte Backends",
  "admin.overview.rebuild": "Projektionen neu aufbauen",
  "admin.overview.rebuilding": "Wird neu aufgebaut…",
  "admin.overview.rebuildSuccess": "{count} Ereignisse erneut abgespielt.",
  "admin.overview.rebuildError": "Neuaufbau fehlgeschlagen.",

  "admin.spaces.title": "Räume",
  "admin.spaces.create": "Neuer Raum",
  "admin.spaces.createTitle": "Raum erstellen",
  "admin.spaces.name": "Name",
  "admin.spaces.kind": "Art",
  "admin.spaces.visibility": "Sichtbarkeit",
  "admin.spaces.owner": "Eigentümer",
  "admin.spaces.members": "Mitglieder",
  "admin.spaces.documents": "Dokumente",
  "admin.spaces.actions": "Aktionen",
  "admin.spaces.rename": "Umbenennen",
  "admin.spaces.setVisibility": "Sichtbarkeit umschalten",
  "admin.spaces.archive": "Archivieren",
  "admin.spaces.delete": "Löschen",
  "admin.spaces.membersTitle": "Mitglieder",
  "admin.spaces.addMember": "Mitglied hinzufügen",
  "admin.spaces.email": "E-Mail",
  "admin.spaces.role": "Rolle",
  "admin.spaces.remove": "Entfernen",
  "admin.spaces.inviteLink": "Einladungslink",
  "admin.spaces.inviteCreate": "Einladung erstellen",
  "admin.spaces.inviteCopy": "Kopieren",
  "admin.spaces.inviteCopied": "Kopiert.",
  "admin.spaces.empty": "Noch keine Räume.",
  "admin.spaces.loadMore": "Weitere Räume laden",
  "admin.spaces.loadMoreMembers": "Weitere Mitglieder laden",
  "admin.spaces.confirmDelete": "Raum \"{name}\" löschen? Dies kann nicht rückgängig gemacht werden.",
  "admin.spaces.confirmArchive": "Raum \"{name}\" archivieren?",
  "admin.spaces.cancel": "Abbrechen",
  "admin.spaces.save": "Speichern",
  "admin.spaces.roleAuthor": "Autor",
  "admin.spaces.roleSpectator": "Betrachter",
  "admin.spaces.kindAtelier": "Atelier",
  "admin.spaces.kindStudio": "Studio",
  "admin.spaces.kindArchive": "Archiv",
  "admin.spaces.visibilityPrivate": "Privat",
  "admin.spaces.visibilityPublic": "Öffentlich",

  "admin.users.title": "Benutzer",
  "admin.users.email": "E-Mail",
  "admin.users.displayName": "Anzeigename",
  "admin.users.createdAt": "Erstellt",
  "admin.users.empty": "Noch keine Benutzer.",
  "admin.users.revokeSessions": "Sitzungen widerrufen",

  "admin.connections.title": "Verbindungen",
  "admin.connections.space": "Raum",
  "admin.connections.document": "Dokument",
  "admin.connections.surface": "Oberfläche",
  "admin.connections.actor": "Akteur",
  "admin.connections.user": "Benutzer",
  "admin.connections.role": "Rolle",
  "admin.connections.connectedAt": "Verbunden",
  "admin.connections.kick": "Trennen",
  "admin.connections.empty": "Keine aktiven Verbindungen.",
  "admin.connections.fresh": "Aktueller Stand",
  "admin.connections.stale": "Veralteter Stand",

  "admin.documents.title": "Dokumente",
  "admin.documents.space": "Raum",
  "admin.documents.allSpaces": "Alle Räume",
  "admin.documents.id": "Dokument",
  "admin.documents.headSeq": "Head-Seq",
  "admin.documents.commitSeq": "Commit-Seq",
  "admin.documents.epoch": "Epoche",
  "admin.documents.activeConnections": "Aktive Verbindungen",
  "admin.documents.empty": "Noch keine Dokumente.",

  "admin.events.title": "Ereignisse",
  "admin.events.since": "Ab Seq",
  "admin.events.refresh": "Aktualisieren",
  "admin.events.empty": "Noch keine Ereignisse.",
  "admin.events.kind": "Art",
  "admin.events.actor": "Akteur",
  "admin.events.time": "Zeit",
  "admin.events.loadMore": "Neuere laden",

  "admin.observability.title": "Beobachtbarkeit",
  "admin.observability.refresh": "Aktualisieren",
  "admin.observability.unavailable": "Der Hub hat seine Beobachtbarkeitsmessung nicht beantwortet.",
  "admin.observability.uptime": "Laufzeit",
  "admin.observability.droppedEvents": "Verworfene Ereignisnamen",
  "admin.observability.residencyTitle": "Kompilierte Gäste",
  "admin.observability.residencyNone": "Dieser Hub hat keinen vertrauenswürdigen Katalog.",
  "admin.observability.residencyResident": "Resident",
  "admin.observability.residencyBudget": "Budget",
  "admin.observability.residencyHits": "Treffer",
  "admin.observability.residencyCompiles": "Kompilierungen",
  "admin.observability.residencyAdmitted": "Aufgenommen",
  "admin.observability.residencyBypassed": "Bedient, ohne zu bleiben",
  "admin.observability.residencyReleased": "Freigegeben",
  "admin.observability.residencyCompileTime": "Kompilierzeit",
  "admin.observability.catalogTitle": "Vertrauenswürdiger Katalog",
  "admin.observability.catalogSummary": "{ready} von {total} Paketen bereit, {refused} abgelehnt · Zeilen festgelegt {pinned}, geprüft {verified} von {rows}",
  "admin.observability.catalogPackage": "Paket",
  "admin.observability.catalogPhase": "Phase",
  "admin.observability.catalogBytes": "Komponente",
  "admin.observability.catalogRows": "Zeilen (festgelegt / geprüft / deklariert)",
  "admin.observability.phase.pending": "Ausstehend",
  "admin.observability.phase.reading": "Wird gelesen",
  "admin.observability.phase.staged": "Wartet auf Prüfung",
  "admin.observability.phase.verifying": "Wird geprüft",
  "admin.observability.phase.ready": "Bereit",
  "admin.observability.phase.refused": "Abgelehnt",
  "admin.observability.routesTitle": "Routen",
  "admin.observability.routesOverflowed": "{count} Antworten von Routen jenseits der Tabelle",
  "admin.observability.route": "Route",
  "admin.observability.requests": "Anfragen",
  "admin.observability.refusals": "Ablehnungen (4xx / 429 / 503)",
  "admin.observability.failures": "Fehler (5xx)",
  "admin.observability.latency": "p50 / p95 / p99 / Max.",
  "admin.observability.eventsTitle": "Trace-Ereignisse",
  "admin.observability.event": "Ereignis",
  "admin.observability.outcomes": "ok / abgelehnt / fehlgeschlagen / abgebrochen",
  "admin.observability.dbIoTitle": "DB-E/A",
  "admin.observability.dbIoSummary": "{tasks} Aufgaben · {steps} Schritte · {turns} Worker-Durchläufe",
  "admin.observability.dbIoKind": "Aufgabenart",
  "admin.observability.dbIoCounts": "Aufgaben / Schritte / Durchläufe",
  "admin.observability.dbIoAdmission": "Zulassungswartezeiten / gewartet / abgelehnt",
  "admin.observability.dbIoAdmissionSummary": "{waits} Übermittlungen warteten auf Zulassung ({waited}), {refusals} dennoch abgelehnt",
} satisfies Record<keyof typeof en, string>;

/** 📚️ Both bundles, keyed by locale — every `AdminI18nKey` is guaranteed present in both at compile
 * time (`de`'s `satisfies Record<keyof typeof en, string>` above rejects a missing/extra key). */
export const ADMIN_I18N: Record<AdminLocale, Record<string, string>> = { en, de };
export type AdminI18nKey = keyof typeof en;
export const ADMIN_LOCALES: readonly AdminLocale[] = ["en", "de"];
//#endregion 🔖️Bundles

//#region 🔖️Context
interface AdminLocaleState {
  locale: AdminLocale | undefined;
  setLocale: (locale: AdminLocale) => void;
}

const AdminLocaleContext = React.createContext<AdminLocaleState | null>(null);

/** 🧭️ Accepts only an explicitly selected supported browser locale; unsupported or absent locale
 * state renders the bilingual chooser instead of silently defaulting to one language. */
function detectAdminLocale(): AdminLocale | undefined {
  if (typeof navigator === "undefined") return undefined;
  for (const language of navigator.languages.length > 0 ? navigator.languages : [navigator.language]) {
    const normalized = language.toLowerCase();
    if (normalized === "en" || normalized.startsWith("en-")) return "en";
    if (normalized === "de" || normalized.startsWith("de-")) return "de";
  }
  return undefined;
}

export function AdminLocaleProvider({ children }: { readonly children: React.ReactNode }): React.ReactElement {
  const [locale, setLocale] = React.useState<AdminLocale | undefined>(detectAdminLocale);
  const value = React.useMemo<AdminLocaleState>(() => ({ locale, setLocale }), [locale]);
  if (!locale) {
    return (
      <div className="flex h-full w-full items-center justify-center" role="dialog" aria-labelledby="admin-language-title">
        <div className="flex flex-col gap-single">
          <h1 id="admin-language-title" className="text-lg font-semibold">Language · Sprache</h1>
          <div className="flex gap-single">
            <button type="button" lang="en" onClick={() => setLocale("en")}>English</button>
            <button type="button" lang="de" onClick={() => setLocale("de")}>Deutsch</button>
          </div>
        </div>
      </div>
    );
  }
  return <AdminLocaleContext.Provider value={value}>{children}</AdminLocaleContext.Provider>;
}

export function useAdminLocale(): AdminLocaleState {
  const state = React.useContext(AdminLocaleContext);
  if (!state) throw new Error("useAdminLocale must be used within AdminLocaleProvider");
  return state;
}

/** 🈯️ `t(key, vars?)` bound to the current locale — `{placeholder}` substitution only (no plural
 * rules needed by this app's vocabulary). Falls back to `en` then the raw key so a missing
 * translation never crashes the page. */
export function useAdminT(): (key: AdminI18nKey, vars?: Record<string, string | number>) => string {
  const { locale } = useAdminLocale();
  if (!locale) throw new Error("admin locale must be explicitly selected");
  return React.useCallback(
    (key: AdminI18nKey, vars?: Record<string, string | number>) => {
      let text: string = ADMIN_I18N[locale][key];
      if (vars) for (const [name, value] of Object.entries(vars)) text = text.replaceAll(`{${name}}`, String(value));
      return text;
    },
    [locale],
  );
}
//#endregion 🔖️Context

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️admin-i18n/🟦️.tsx");
  await registerTests1(import.meta.vitest, { ADMIN_I18N, de, en }, { directory: import.meta.dir, url: import.meta.url });
}
