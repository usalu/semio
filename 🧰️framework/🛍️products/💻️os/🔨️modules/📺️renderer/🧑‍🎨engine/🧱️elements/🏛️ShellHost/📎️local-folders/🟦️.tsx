// #region 🧲️Header
/** 📁️ The device's remembered folder bindings — `os.config.local-folders`, an event-sourced, persisted local-only config
 * facet kept in this shell's own storage (never shared, never in a URL) — and the accessible offer to reconnect one. When a
 * program boots holding a document this device once attached to a folder, the shell offers "Reconnect folder"; the person's
 * own action reattaches it, and the folder's archive is restored like a fresh load (`restoreDocumentArchiveV1`). Ticket
 * 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING follow-up 4.
 * @see ../../../../../../🎚️config/🧬️schema/🧬️mutations/📎️attach-local-folder/🧬️schema/🔣️.json */
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { OsShellConfig, type StoragePort } from "@semio-tech/framework";
import { applyLocalFoldersConfigMutation, LOCAL_FOLDERS_CONFIG_SCHEMA, type LocalFolderBinding, type LocalFolderBindings, type LocalFoldersConfigMutation } from "../../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️EventLog
/** 📁️ No binding remembered. */
export const EMPTY_LOCAL_FOLDER_BINDINGS_V1: LocalFolderBindings = { bindings: [] };

/** 🧾️ The persisted event log of the facet: every mutation this device committed, in order. */
export type LocalFoldersEventLogV1 = { readonly version: 1; readonly events: readonly LocalFoldersConfigMutation[] };

const text = (value: unknown, maximum: number): value is string => typeof value === "string" && value.length > 0 && value.length <= maximum;

/** 🔎️ One persisted event read against the payload schemas (`📎️attach-local-folder`, `✂️detach-local-folder`); `null` when
 * it is not one. */
export function localFoldersMutationOfValueV1(value: unknown): LocalFoldersConfigMutation | null {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return null;
  const row = value as Record<string, unknown>;
  const keys = Object.keys(row).sort().join(",");
  if (row.mutation === "detachLocalFolder") return keys === "documentId,mutation" && text(row.documentId, 512) ? { mutation: "detachLocalFolder", documentId: row.documentId } : null;
  if (row.mutation !== "attachLocalFolder" || keys !== "appId,documentId,folder,mutation,pluginId" || !text(row.documentId, 512) || !text(row.pluginId, 256) || !text(row.appId, 256)) return null;
  const folder = row.folder as Record<string, unknown> | null;
  if (typeof folder !== "object" || folder === null || Object.keys(folder).sort().join(",") !== "kind,path" || folder.kind !== "path" || !text(folder.path, 4096)) return null;
  return { mutation: "attachLocalFolder", documentId: row.documentId, pluginId: row.pluginId, appId: row.appId, folder: { kind: "path", path: folder.path } };
}

/** 📖️ The facet's committed events; a log that does not read as one is empty — nothing is reattached from it. */
export function readLocalFolderEventsV1(storage: StoragePort): readonly LocalFoldersConfigMutation[] {
  const raw = new OsShellConfig(storage).getPreference(LOCAL_FOLDERS_CONFIG_SCHEMA);
  if (raw === undefined) return [];
  try {
    const value = JSON.parse(raw) as { readonly version?: unknown; readonly events?: unknown };
    if (value.version !== 1 || !Array.isArray(value.events)) return [];
    const events = value.events.map(localFoldersMutationOfValueV1);
    return events.every((event) => event !== null) ? (events as LocalFoldersConfigMutation[]) : [];
  } catch {
    return [];
  }
}

/** 🧮️ The bindings the committed events fold to. */
export function replayLocalFolderEventsV1(events: readonly LocalFoldersConfigMutation[]): LocalFolderBindings {
  return events.reduce(applyLocalFoldersConfigMutation, EMPTY_LOCAL_FOLDER_BINDINGS_V1);
}

/** 📖️ This device's remembered folder bindings. */
export function readLocalFolderBindingsV1(storage: StoragePort): LocalFolderBindings {
  return replayLocalFolderEventsV1(readLocalFolderEventsV1(storage));
}

/** ✍️ Appends one mutation to the facet's local-only log and answers the bindings it folds to. A mutation that changes
 * nothing is not recorded. */
export function commitLocalFoldersConfigMutationV1(storage: StoragePort, mutation: LocalFoldersConfigMutation): LocalFolderBindings {
  const events = readLocalFolderEventsV1(storage);
  const before = replayLocalFolderEventsV1(events);
  const after = applyLocalFoldersConfigMutation(before, mutation);
  if (after === before) return before;
  new OsShellConfig(storage).setPreference(LOCAL_FOLDERS_CONFIG_SCHEMA, JSON.stringify({ version: 1, events: [...events, mutation] } satisfies LocalFoldersEventLogV1));
  return after;
}
//#endregion 🔖️EventLog

//#region 🔖️Reconnect
/** 🪪️ The program and the document identity a booted session holds. */
export type LocalFolderIdentityV1 = Readonly<{ documentId: string; pluginId: string; appId: string }>;

/** 📎️ The binding to offer for reconnection: the one remembered for this document in this program, unless the document is
 * attached already. */
export function localFolderReconnectOfferV1(bindings: LocalFolderBindings, identity: LocalFolderIdentityV1 | null, attachedDocumentIds: ReadonlySet<string>): LocalFolderBinding | null {
  if (identity === null || attachedDocumentIds.has(identity.documentId)) return null;
  return bindings.bindings.find((binding) => binding.documentId === identity.documentId && binding.pluginId === identity.pluginId && binding.appId === identity.appId) ?? null;
}

/** 🏷️ The name a person knows a folder by: its last path segment. */
export function localFolderNameV1(path: string): string {
  const segments = path.split(/[\\/]+/u).filter((segment) => segment.length > 0);
  return segments.at(-1) ?? path;
}

/** 📎️ The accessible offer to reconnect a remembered folder: a polite status naming the folder, with "Reconnect folder"
 * (the person's own gesture reattaches it) and "Forget folder". Texts arrive localized. */
export function LocalFolderReconnectBand(props: Readonly<{ message: string; reconnect: string; forget: string; label: string; busy: boolean; onReconnect: () => void; onForget: () => void }>): React.ReactElement {
  const { message, reconnect, forget, label, busy, onReconnect, onForget } = props;
  return (
    <div role="status" aria-live="polite" aria-label={label} data-semio-folder-reconnect={busy ? "reconnecting" : "offered"} className="pointer-events-auto flex max-w-[90vw] flex-wrap items-center gap-single rounded-sm border border-normal bg-menu px-double py-single text-sm shadow-sm">
      <span id="s-folder-reconnect-message">{message}</span>
      <button type="button" id="s-folder-reconnect" className="min-h-medium px-tiny underline" disabled={busy} aria-busy={busy} onClick={onReconnect}>
        {reconnect}
      </button>
      <button type="button" id="s-folder-forget" className="min-h-medium px-tiny underline" disabled={busy} onClick={onForget}>
        {forget}
      </button>
    </div>
  );
}
//#endregion 🔖️Reconnect
