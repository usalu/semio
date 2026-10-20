/** 🧾️ Document transfer notices, open-with grouping, check-in and sync status, and the tutorial UI bridge.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHelpers}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  type ActionDefinition,
  type AppDefinition,
  type AppRef,
  type AppRole,
  type AppRouter,
  type HistoryTimeTravel,
  type ArtifactDialect,
  type CommandDefinition,
  type PluginAppLabelsOverlay,
  type TutorialUiChange,
  type TutorialUiSnapshot,
  NATURAL_FILE_PORT_ID,
  OPEN_ARTIFACT_FILE_ACTION_ID,
  SAVE_ARTIFACT_FILE_ACTION_ID
} from "@semio-tech/framework";
// #endregion 🔌️Adapters
import { type ArtifactSyncStatus } from "@semio-tech/framework/backbone";
import { type Anchor, ANCHORS } from "@semio-tech/ui-react";
import { useEffect } from "react";
import type { DomainSelection, InteractionState } from "@semio-tech/framework-interaction";
import type { ShellDialogV1 } from "../../🏛️ShellHost/🗨️dialog-origin/🟦️.ts";
import { type ActiveSession } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { type ShellState } from "../../🐚️Shell/🗃️state-shapes/🟦️.ts";
import { type ShellAction } from "../../🐚️Shell/🎛️shell-actions/🟦️.ts";
import { type FrozenLabel, frozenLabelText } from "../🏷️labels/🟦️.tsx";
import { applyFrameworkLayoutSeed, captureCurrentFrameworkLayout } from "../🛣️route-and-chrome/🟦️.tsx";

//#region ShellHelpers
//#region 👁️✏️SurfaceRoleLabels
//#region 📤️DocumentTransfer
/** 🗃️ File extension of the document archive the framework's Export Document writes and Import Document reads
 * (`EXPORT_ARTIFACT_DOCUMENT_ACTION_ID`/`IMPORT_ARTIFACT_DOCUMENT_ACTION_ID`, shell-intercepted in `🏛️ShellHost`): the
 * program's root envelope, its op log and every owned member (`DocumentArchivePack`, `encodeDocumentArchiveBytes`). */
export const DOCUMENT_ARCHIVE_FILE_EXTENSION = ".semio-archive";

/** 🗃️ Media type of an exported document archive. */
export const DOCUMENT_ARCHIVE_MEDIA_TYPE = "application/vnd.semio.document-archive";

/** 🏷️ `<kind>-<UTC stamp>.semio-archive`, the kind being the app id's artifact segment (`s.puzzle.puzzle2d@1/*#editor` →
 * `puzzle2d`). */
export function documentArchiveFileNameV1(appId: string, at: Date): string {
  const kind = /^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? (appId.replace(/[^A-Za-z0-9]+/gu, "-").replace(/^-+|-+$/gu, "") || "document");
  return `${kind}-${at.toISOString().replace(/[-:]/gu, "").replace(/\.\d{3}Z$/u, "Z")}${DOCUMENT_ARCHIVE_FILE_EXTENSION}`;
}

/** 📄️ `<kind>-<UTC stamp><extension>` for one declared natural representation. */
export function naturalFileNameV1(appId: string, extension: string, at: Date): string {
  const kind = /^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? (appId.replace(/[^A-Za-z0-9]+/gu, "-").replace(/^-+|-+$/gu, "") || "document");
  return `${kind}-${at.toISOString().replace(/[-:]/gu, "").replace(/\.\d{3}Z$/u, "Z")}${extension}`;
}

/** 📄️ Exact natural representation mounted by one editor definition. */
export type NaturalFileFormatV1 = Readonly<{ formatKind: string; extension: string; mediaType: string; binary: boolean }>;

/** 📄️ Reads a native codec only from the paired Open/Save action declaration and matching App I/O lists. */
export function naturalFileFormatV1(app: AppDefinition): NaturalFileFormatV1 | null {
  const actions = [...(app.actions ?? []), ...app.windowKinds.flatMap((kind) => kind.actions ?? [])];
  const save = actions.find((candidate) => candidate.id === SAVE_ARTIFACT_FILE_ACTION_ID);
  const open = actions.find((candidate) => candidate.id === OPEN_ARTIFACT_FILE_ACTION_ID);
  if (save === undefined || open === undefined) return null;
  const read = (action: ActionDefinition): NaturalFileFormatV1 | null => {
    const defaults = new Map((action.args ?? []).map((argument) => [argument.id, argument.default]));
    const formatKind = defaults.get("formatKind");
    const extension = defaults.get("extension");
    const mediaType = defaults.get("mediaType");
    const binary = defaults.get("binary");
    return typeof formatKind === "string" && typeof extension === "string" && /^\.[A-Za-z0-9]+$/u.test(extension) && typeof mediaType === "string" && typeof binary === "boolean"
      ? { formatKind, extension, mediaType, binary }
      : null;
  };
  const saveFormat = read(save);
  const openFormat = read(open);
  if (
    saveFormat === null ||
    openFormat === null ||
    saveFormat.formatKind !== openFormat.formatKind ||
    saveFormat.extension !== openFormat.extension ||
    saveFormat.mediaType !== openFormat.mediaType ||
    saveFormat.binary !== openFormat.binary
  )
    return null;
  if (!app.io.exportFormats.includes(saveFormat.formatKind) || !app.io.importFormats.includes(saveFormat.formatKind)) return null;
  return saveFormat;
}

/** 📄️ Checks the native producer's typed descriptor before bytes are offered as a natural file. */
export function naturalMediaDescriptorMatchesV1(value: unknown, format: NaturalFileFormatV1): boolean {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return false;
  const descriptor = value as Readonly<Record<string, unknown>>;
  const wire = descriptor.wire;
  if (typeof wire !== "object" || wire === null || Array.isArray(wire)) return false;
  const encoded = wire as Readonly<Record<string, unknown>>;
  return descriptor.portId === NATURAL_FILE_PORT_ID && descriptor.kindId === format.formatKind && encoded.kind === "binary" && encoded.format_kind === format.formatKind;
}

/** 📂️ Imports bytes into a newly created owner and retires that owner on every failed or cancelled path. */
export async function openNaturalFileOwnerV1(input: Readonly<{
  signal: AbortSignal;
  create: () => Promise<number>;
  importBytes: (instanceId: number) => Promise<void>;
  retire: (instanceId: number) => Promise<void>;
}>): Promise<number> {
  if (input.signal.aborted) throw input.signal.reason ?? new DOMException("The operation was aborted", "AbortError");
  const instanceId = await input.create();
  try {
    if (input.signal.aborted) throw input.signal.reason ?? new DOMException("The operation was aborted", "AbortError");
    await input.importBytes(instanceId);
    if (input.signal.aborted) throw input.signal.reason ?? new DOMException("The operation was aborted", "AbortError");
    return instanceId;
  } catch (error) {
    await input.retire(instanceId);
    throw error;
  }
}

/** 📦️ The bytes of a `readAs: "dataUrl"` pick (`data:<type>;base64,<payload>`). */
export function bytesOfDataUrlV1(dataUrl: string): Uint8Array {
  const comma = dataUrl.indexOf(",");
  if (!dataUrl.startsWith("data:") || comma < 0 || !dataUrl.slice(0, comma).endsWith(";base64")) throw new Error("document-transfer.data-url-invalid");
  return Uint8Array.from(atob(dataUrl.slice(comma + 1)), (character) => character.charCodeAt(0));
}

/** 🗣️ Every outcome of an export or import a person is told about. */
export type DocumentTransferNoticeV1 =
  | "exported"
  | "export-failed"
  | "file-saved"
  | "file-save-failed"
  | "file-opened"
  | "file-open-failed"
  | "no-document"
  | "import-unreadable"
  | "import-failed"
  | "import-cancelled"
  | "imported"
  | "load-cancelled"
  | "load-failed"
  | "folder-detached";

/** 🗣️ Notice text per outcome, authored beside the code like `📣️replay-refusal`; `{file}` is the file name. */
export const DOCUMENT_TRANSFER_NOTICE_LABELS_V1: Readonly<Record<DocumentTransferNoticeV1, { readonly en: string; readonly de: string }>> = {
  exported: { en: "Exported “{file}”.", de: "„{file}“ exportiert." },
  "export-failed": { en: "The document could not be exported.", de: "Das Dokument konnte nicht exportiert werden." },
  "file-saved": { en: "Saved “{file}”.", de: "„{file}“ gespeichert." },
  "file-save-failed": { en: "The file could not be saved.", de: "Die Datei konnte nicht gespeichert werden." },
  "file-opened": { en: "Opened “{file}” as a new document.", de: "„{file}“ als neues Dokument geöffnet." },
  "file-open-failed": { en: "“{file}” could not be opened; the current document is unchanged.", de: "„{file}“ konnte nicht geöffnet werden; das aktuelle Dokument ist unverändert." },
  "no-document": { en: "Focus a document window first — the Home has no document to export or import into.", de: "Bitte zuerst ein Dokumentfenster wählen — die Startseite hat kein Dokument zum Exportieren oder Importieren." },
  "import-unreadable": { en: "“{file}” is not a document archive.", de: "„{file}“ ist kein Dokumentarchiv." },
  "import-failed": { en: "“{file}” could not be opened as a document.", de: "„{file}“ konnte nicht als Dokument geöffnet werden." },
  "import-cancelled": { en: "Import of “{file}” cancelled.", de: "Import von „{file}“ abgebrochen." },
  imported: { en: "Opened “{file}” as a new document.", de: "„{file}“ als neues Dokument geöffnet." },
  "load-cancelled": { en: "Loading “{file}” was cancelled; the previous document is unchanged.", de: "Laden von „{file}“ abgebrochen; das bisherige Dokument ist unverändert." },
  "load-failed": { en: "“{file}” could not be loaded; the previous document is unchanged.", de: "„{file}“ konnte nicht geladen werden; das bisherige Dokument ist unverändert." },
  "folder-detached": {
    en: "The folder was detached because its document was not loaded; nothing is saved to it. Reconnect the folder to load its document.",
    de: "Der Ordner wurde getrennt, weil sein Dokument nicht geladen wurde; es wird nichts darin gespeichert. Verbinde den Ordner erneut, um sein Dokument zu laden.",
  },
};

/** ⏹️ Whether a whole-document load ended by a cancel: the caller's `signal`, or the guest's own (a person's Cancel in the
 * history body), which the stepped archive load reports as an `AbortError`. */
export function documentLoadCancelledV1(error: unknown, signal?: AbortSignal): boolean {
  return signal?.aborted === true || (error instanceof DOMException && error.name === "AbortError");
}

/** 🗣️ The localized notice for one outcome; only an unknown locale falls back to English. */
export function documentTransferNoticeTextV1(notice: DocumentTransferNoticeV1, file: string, locale: string): string {
  const label = DOCUMENT_TRANSFER_NOTICE_LABELS_V1[notice];
  return (locale === "de" ? label.de : label.en).replace("{file}", file);
}

/** 🩺️ The notice's code, shared by the notice, the console line and every probe. */
export function documentTransferNoticeCodeV1(notice: DocumentTransferNoticeV1): string {
  return `shell.documentTransfer.${notice}`;
}
//#endregion 📤️DocumentTransfer

/** 👁️✏️ `true` for a `mutation`-kind action/command — the one predicate every viewer-chrome hiding
 * rule in this lease (context menu, command palette, dispatch guard) shares, so "what counts as an
 * editing verb" has exactly one definition. */
export function isMutationKindDefinition(definition: Pick<ActionDefinition, "kind"> | Pick<CommandDefinition, "kind">): boolean {
  return definition.kind === "mutation";
}

/** 👁️✏️ Filters `Mutation`-kind entries out of an action/command list for a `"viewer"` role — a no-op
 * for `"editor"` (and for `undefined`, since an app with no resolved session has no role to gate on
 * yet). Shared by the shell context menu and the command palette so both hide the exact same set. */
export function filterDefinitionsForRole<T extends Pick<ActionDefinition, "kind"> | Pick<CommandDefinition, "kind">>(definitions: readonly T[], role: AppRole | undefined): readonly T[] {
  if (role !== "viewer") return definitions;
  return definitions.filter((definition) => !isMutationKindDefinition(definition));
}

/** 👁️✏️ One `AppRouter` entry resolved for display in the "Open with…" surfaces (Document panel,
 * context menu, command palette) — plugin-labelled, flagged as the current session's own app and/or
 * the pinned default so the picker can render both without a second lookup. */
export type OpenWithEntry = {
  readonly app: AppRef;
  readonly pluginLabel: string;
  readonly current: boolean;
  readonly isDefault: boolean;
};

/** 👁️✏️ Groups one dialect's `AppRouter` entries by role for the "Open with…" surfaces — owner-plugin
 * first within each role group (the router's own build-time ordering, contract freeze §3), annotated
 * with `current`/`isDefault` against the live session and pinned prefs. `pluginLabel` falls back to
 * the raw `pluginId` when the plugin's own manifest label isn't available (e.g. not loaded yet). */
export function groupOpenWithEntries(
  router: AppRouter,
  dialect: ArtifactDialect,
  currentApp: AppRef | undefined,
  pinnedApp: (role: AppRole) => AppRef | undefined,
  pluginLabelById: ReadonlyMap<string, string>,
): Readonly<Record<AppRole, readonly OpenWithEntry[]>> {
  const forRole = (role: AppRole): readonly OpenWithEntry[] => {
    const pinned = pinnedApp(role);
    return router.entriesFor(dialect, role).map((app) => ({
      app,
      pluginLabel: pluginLabelById.get(app.pluginId) ?? app.pluginId,
      current: currentApp?.pluginId === app.pluginId && currentApp?.appId === app.appId,
      isDefault: pinned?.pluginId === app.pluginId && pinned?.appId === app.appId,
    }));
  };
  return { viewer: forRole("viewer"), editor: forRole("editor") };
}
//#endregion 👁️✏️SurfaceRoleLabels

//#region 🔖️CheckInAndSyncStatus
/** 📌️ ticket `26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS` §C5 — auto check-in
 * policy constants: an open editor session with uncommitted edits checkpoints once it has been idle
 * (no new edit) for this long, or immediately once this many uncommitted edits have piled up (never
 * waiting for an idle period once the threshold is crossed). */
export const AUTO_CHECKIN_IDLE_MS = 20_000;
export const AUTO_CHECKIN_EDIT_THRESHOLD = 200;

/** 📌️ A tiny, framework-free debounce scheduler for the auto check-in policy: call {@link notify}
 * every time the uncommitted-edit count changes for the currently open editor session; it fires
 * `onCheckpoint` at most once per idle period — immediately once `threshold` is reached (never
 * waiting out the idle period once crossed), otherwise `idleMs` after the LAST `notify` call, and
 * never again until `notify(0)` (a landed checkpoint) clears the `pending` latch — the "never a
 * storm" guard: a `notify` that arrives while a checkpoint is already pending is a no-op. Deliberately
 * framework-free (`setTimeout`/`clearTimeout` only, no React) so it is unit-testable with vitest's
 * fake timers without mounting a component tree — `ShellHost`'s own effect is a thin wrapper around
 * one instance per open editor session, `cancel`ed on unmount/session-switch. */
export class AutoCheckinScheduler {
  private timer: ReturnType<typeof setTimeout> | null = null;
  private pending = false;

  constructor(
    private readonly onCheckpoint: () => void,
    private readonly idleMs: number = AUTO_CHECKIN_IDLE_MS,
    private readonly threshold: number = AUTO_CHECKIN_EDIT_THRESHOLD,
  ) {}

  notify(uncommittedEditCount: number): void {
    if (uncommittedEditCount === 0) {
      this.cancel();
      this.pending = false;
      return;
    }
    if (this.pending) return;
    if (uncommittedEditCount >= this.threshold) {
      this.cancel();
      this.pending = true;
      this.onCheckpoint();
      return;
    }
    this.cancel();
    this.arm();
  }

  /** ⏳️ The checkpoint just asked for cannot be dispatched now — its document is loading, its port is not bound, its
   * history is under edit: the latch is released and the idle period starts again, so the check-in is asked for again
   * instead of being lost or told to a person who pressed nothing (live finding O4). */
  defer(): void {
    this.cancel();
    this.pending = false;
    this.arm();
  }

  private arm(): void {
    this.timer = setTimeout(() => {
      this.timer = null;
      this.pending = true;
      this.onCheckpoint();
    }, this.idleMs);
  }

  cancel(): void {
    if (this.timer != null) {
      clearTimeout(this.timer);
      this.timer = null;
    }
  }
}

/** ⏳️ Whether an AUTOMATIC check-in waits (live finding O4): an automatic action never surfaces a refusal, so it is not
 * dispatched into a document that cannot take a checkpoint now — one that is loading (the guest would answer
 * `document.loading`), or one that is attached while its port is not bound (what it published would reach no folder and
 * no hub). Corpus `🧫️fixtures/🧫️automatic-checkin/🔣️.json`. */
export function automaticCheckinWaitsV1(document: Readonly<{ loading: boolean; attached: boolean; bound: boolean }>): boolean {
  return document.loading || (document.attached && !document.bound);
}

/** 📌️ The checkpoint message a `framework.checkin` `submit` carries: its trimmed `message`, else `"check-in"`. */
export function checkinSubmitMessageV1(args: unknown): string {
  const message = typeof args === "object" && args !== null && typeof (args as { readonly message?: unknown }).message === "string" ? (args as { readonly message: string }).message.trim() : "";
  return message === "" ? "check-in" : message;
}

/** 🛂️ What a checkpoint request does while the focused program's history may be under edit (e2e R2-6): outside a
 * session it dispatches `commitCheckpoint`; inside one the guest freezes the history (`timeTravel.frozen`), so an
 * automatic check-in (`"auto"`: idle, edit count, close) waits for the next one after the session and an explicit one is
 * told why — no `commitCheckpoint` reaches the guest from either. */
export function checkpointGateV1(timeTravel: HistoryTimeTravel | null, message: string): "dispatch" | "wait" | "frozen" {
  if (timeTravel === null) return "dispatch";
  return message === "auto" ? "wait" : "frozen";
}

/** 🔩️ The identity a checkpoint-on-close is keyed on: one program's one document, `null` when no editor holds one. A
 * view-state rewrite (every finalize, the New-alternative submit) mints a new session object of the SAME program, which
 * keeps this key and so closes nothing (e2e R2-6). */
export function checkpointOnCloseKeyV1(editor: boolean, program: { readonly pluginId: string; readonly instanceId: number } | null, documentId: string | null): string | null {
  return editor && program !== null && documentId !== null && documentId !== "" ? `${program.pluginId}#${program.instanceId}@${documentId}` : null;
}

/** 🛬️ Runs `onClose` — the closure of the render that opened `key` — when `key` changes or the shell unmounts: the
 * document that is being left checkpoints, never the one that replaced it. */
export function useCheckpointOnCloseV1(key: string | null, onClose: () => void): void {
  useEffect(() => (key === null ? undefined : onClose), [key]);
}

/** ⚓️ The document a sync attach addresses: a hub target's own document, else the program's own document identity — the
 * id its store stamps on every envelope it publishes (`ReadDocumentIdentity`), so the actor admits the program's edits as
 * that document's instead of refusing each batch `local.backbone-scope-mismatch` (e2e R2-4). `null`: the program holds no
 * document, and there is nothing to attach. */
export function syncAttachDocumentIdV1(target: { readonly kind: "folder" | "file" | "remote"; readonly documentId?: string | null }, identity: { readonly parent_document_id: string | null } | null): string | null {
  const own = target.kind === "remote" ? (target.documentId ?? null) : null;
  const documentId = own ?? identity?.parent_document_id ?? null;
  return documentId === "" ? null : documentId;
}

/** 📌️ The hub Check In's own status vocabulary: a running phase with its progress, and one sentence
 * per terminal outcome and refusal. Mirrors `DocumentCheckInPhaseV1`/`DocumentCheckInRefusalV1`. */
const CHECKIN_STATUS_LABELS = {
  running: { en: "Checking in…", de: "Wird eingecheckt…" },
  ready: { en: "Checked in", de: "Eingecheckt" },
  cancelled: { en: "Check-in cancelled", de: "Einchecken abgebrochen" },
  "unknown-head": { en: "Check-in refused: the hub does not know this version", de: "Einchecken abgelehnt: Der Hub kennt diesen Stand nicht" },
  "stale-head": { en: "Already checked in: a newer check-in exists", de: "Bereits eingecheckt: Es gibt einen neueren Check-in" },
  "active-checkpoint-changed": { en: "Check-in collided with another check-in; try again", de: "Einchecken kollidierte mit einem anderen Check-in; bitte erneut versuchen" },
  "ledger-not-replayable": { en: "Check-in refused: an approval in this range is checked in on its own", de: "Einchecken abgelehnt: Eine Freigabe in diesem Bereich wird separat eingecheckt" },
  "codec-refused": { en: "Check-in refused: the document could not be rebuilt", de: "Einchecken abgelehnt: Das Dokument konnte nicht wiederhergestellt werden" },
  "authority-changed": { en: "Check-in stopped: you can no longer edit this document", de: "Einchecken gestoppt: Sie dürfen dieses Dokument nicht mehr bearbeiten" },
  unavailable: { en: "Check-in unavailable; try again", de: "Einchecken nicht verfügbar; bitte erneut versuchen" },
} as const satisfies Readonly<Record<string, FrozenLabel>>;

/** 📌️ One sentence for a hub Check In status; a running one carries its `completed/total` units. */
export function checkinStatusText(status: Readonly<{ phase: string; progress: Readonly<{ completedUnits: number; totalUnits: number }>; refusal?: string }>, locale: string): string {
  if (status.phase === "ready") return frozenLabelText(CHECKIN_STATUS_LABELS.ready, locale);
  if (status.phase === "cancelled") return frozenLabelText(CHECKIN_STATUS_LABELS.cancelled, locale);
  if (status.phase === "failed") return frozenLabelText(CHECKIN_STATUS_LABELS[(status.refusal ?? "unavailable") as keyof typeof CHECKIN_STATUS_LABELS] ?? CHECKIN_STATUS_LABELS.unavailable, locale);
  return `${frozenLabelText(CHECKIN_STATUS_LABELS.running, locale)} ${status.progress.completedUnits}/${status.progress.totalUnits}`;
}

const CHECKIN_ABORT_LABEL: FrozenLabel = { en: "Cancel check-in", de: "Einchecken abbrechen" };
/** 🛑️ The running Check In's own cancel control. */
export function checkinAbortText(locale: string): string {
  return frozenLabelText(CHECKIN_ABORT_LABEL, locale);
}

/** 👁️✏️ ticket §C5 item 5 — "viewers never checkpoint": the one predicate gating BOTH the
 * `#s-checkin` affordance's presence and the auto check-in timer's arming, mirroring
 * `isMutationKindDefinition`'s "one definition of what counts as an editing verb" precedent —
 * checkpoint is exactly such a verb. `VcsArtifactApp`'s own host-side guard already rejects a
 * viewer's `CommitCheckpoint` dispatch regardless; this is the client-side mirror that keeps the
 * affordance from ever reaching the wire in the first place. */
export function canCheckIn(role: AppRole | undefined): boolean {
  return role === "editor";
}

/** 🚦️ ticket §C5 — the sync status pill's three-way vocabulary, derived from `ArtifactSyncStatus`. A
 * non-live remote takes priority over a pending-mutation count (the user needs to know the
 * connection itself is degraded before anything about local pending edits); `null` (no status event
 * observed yet, e.g. before the document's first `open`) reads as `remote: "detached"`. */
export type SyncPillState = { readonly kind: "persisted" } | { readonly kind: "pending"; readonly count: number } | { readonly kind: "remote"; readonly remote: "connected" | "connecting" | "backoff" | "detached" };

export function computeSyncPillState(status: ArtifactSyncStatus | null): SyncPillState {
  if (!status || status.remote.kind !== "live") {
    return { kind: "remote", remote: !status ? "detached" : status.remote.kind === "live" ? "connected" : status.remote.kind };
  }
  if (status.pendingMutations > 0) return { kind: "pending", count: status.pendingMutations };
  return { kind: "persisted" };
}

const SYNC_STATUS_PERSISTED_LABEL: FrozenLabel = { en: "Persisted", de: "Gespeichert" };
const SYNC_STATUS_PENDING_LABEL: FrozenLabel = { en: "Pending", de: "Ausstehend" };
const SYNC_STATUS_REMOTE_LABEL: FrozenLabel = { en: "Remote", de: "Remote" };
const SYNC_STATUS_REMOTE_STATE_LABEL: Readonly<Record<"connected" | "connecting" | "backoff" | "detached", FrozenLabel>> = {
  connected: { en: "connected", de: "verbunden" },
  connecting: { en: "connecting", de: "verbindet" },
  backoff: { en: "backoff", de: "erneuter Versuch" },
  detached: { en: "detached", de: "getrennt" },
};

/** 🚦️ Localized pill text for `#s-sync-status` — matches contract §C5's own vocabulary (`persisted |
 * pending(n) | remote(connected|connecting|backoff|detached)`) closely enough that a state can be
 * read back off the rendered string in either locale, not just English. */
export function syncPillText(state: SyncPillState, locale: string): string {
  if (state.kind === "persisted") return frozenLabelText(SYNC_STATUS_PERSISTED_LABEL, locale);
  if (state.kind === "pending") return `${frozenLabelText(SYNC_STATUS_PENDING_LABEL, locale)} (${state.count})`;
  return `${frozenLabelText(SYNC_STATUS_REMOTE_LABEL, locale)}: ${frozenLabelText(SYNC_STATUS_REMOTE_STATE_LABEL[state.remote], locale)}`;
}
//#endregion 🔖️CheckInAndSyncStatus

//#region 🎥️TutorialUiBridge
/** 🕹️ Copies the exact typed selection projection without retaining mutable manifest arrays or invoking special object keys. */
function captureInteractionSelection(interaction: InteractionState): TutorialUiSnapshot["interactionSelection"] {
  const selection: Record<string, { granularity: string; ids: string[]; anchorId?: string }> = {};
  for (const [domainId, current] of Object.entries(interaction.selection)) {
    const captured = { granularity: current.granularity, ids: [...current.ids], ...(current.anchorId === undefined ? {} : { anchorId: current.anchorId }) };
    Object.defineProperty(selection, domainId, { value: captured, enumerable: true, writable: true, configurable: true });
  }
  return selection;
}

/** 🛡️ Converts the generated optional-value map into the total runtime selection projection. */
function tutorialInteractionSelection(selection: TutorialUiSnapshot["interactionSelection"]): InteractionState["selection"] {
  const result: Record<string, DomainSelection> = {};
  for (const [domainId, current] of Object.entries(selection)) {
    if (!current) continue;
    const captured = { granularity: current.granularity, ids: [...current.ids], ...(current.anchorId === undefined ? {} : { anchorId: current.anchorId }) };
    Object.defineProperty(result, domainId, { value: captured, enumerable: true, writable: true, configurable: true });
  }
  return result;
}

/** 🎥️ Captures the shell's current `ShellState`, its local interaction (`LocalInteractionStoreV1`) and the active session as a renderer-neutral `TutorialUiSnapshot` — the recorder's periodic full-snapshot keyframes and the `TutorialBar`'s "record" path both call this. See the Rust doc comment on `TutorialUiSnapshot` for why this is deliberately NOT a serialization of `ShellState` itself. */
export function captureTutorialUiSnapshot(state: ShellState, interaction: InteractionState, session: ActiveSession | null): TutorialUiSnapshot {
  const activeUtilityByWindowId: Record<string, string> = {};
  for (const [windowId, utilityId] of Object.entries(state.actionPane.activeUtilityByWindowId)) {
    if (utilityId) activeUtilityByWindowId[windowId] = utilityId;
  }
  const activePanelTabByGroup: Record<string, string> = {};
  for (const anchor of ANCHORS) {
    const panelState = state.layout.panels[anchor];
    const tabId = panelState.path[panelState.path.length - 1];
    if (panelState.visible && tabId) activePanelTabByGroup[anchor] = tabId;
  }
  return {
    activeModeId: session?.viewState.activeModeId,
    focusedWindowId: state.layout.activeWindowId ?? undefined,
    activeUtilityByWindowId,
    activeToolId: state.actionPane.activeToolId ?? undefined,
    layout: captureCurrentFrameworkLayout(state.layout.shellLayout, state.layout.extraWindowInstances),
    activePanelTabByGroup,
    panelJson: session?.viewState.panelJson,
    interactionSelection: captureInteractionSelection(interaction),
    openDialogId: state.overlays.dialog?.dialogId,
    expandedTreeIds: Object.entries(state.layout.treeOpenStates).filter(([, open]) => open).map(([id]) => id),
    commandPanelOpen: state.overlays.searchOpen,
  };
}

/** 🎥️ Context every `applyTutorialUiSnapshotToShell`/`applyTutorialUiChangeToShell` call needs beyond `dispatch` itself — resolved once per render by the caller (the director/seek/deviation-converge paths all share it). */
export type TutorialUiBridgeContext = {
  readonly session: ActiveSession | null;
  readonly restoreDialog: (dialogId: string, seedArgs?: Readonly<Record<string, unknown>>) => ShellDialogV1 | null;
  readonly appLabelsOverlay: PluginAppLabelsOverlay;
  readonly terminology: string;
  readonly locale: string;
  readonly interactionSelection: () => InteractionState["selection"];
  readonly publishInteractionSelection: (selection: InteractionState["selection"]) => void;
};

/** 🎥️ Applies a full `TutorialUiSnapshot` (a `TutorialUiSample::Snapshot`, or the composed target of a seek/deviation-converge) onto the live `ShellState` — snaps every field instantly (camera is the only interpolated track, applied separately by the director). Dispatches the atomic `APPLY_TUTORIAL_UI_SNAPSHOT` for shell-owned fields, including typed interaction selection, plus one `SET_SESSION` for `ActiveSession.viewState`'s `activeModeId`/`panelJson`. */
export function applyTutorialUiSnapshotToShell(dispatch: (action: ShellAction) => void, snapshot: TutorialUiSnapshot, ctx: TutorialUiBridgeContext): void {
  const windowKinds = ctx.session?.app.windowKinds.map((kind) => ({ id: kind.id, label: kind.label })) ?? [];
  const seed = applyFrameworkLayoutSeed(snapshot.layout, windowKinds, ctx.appLabelsOverlay, ctx.terminology, ctx.locale);
  const panelPatches: Partial<Record<Anchor, { readonly visible: boolean; readonly path: readonly string[] }>> = {};
  for (const anchor of ANCHORS) {
    const tabId = snapshot.activePanelTabByGroup[anchor];
    panelPatches[anchor] = tabId ? { visible: true, path: [tabId] } : { visible: false, path: [] };
  }
  const treeOpenStates: Record<string, boolean> = {};
  for (const id of snapshot.expandedTreeIds) treeOpenStates[id] = true;
  const activeUtilityByWindowId: Record<string, string | null> = {};
  for (const [windowId, utilityId] of Object.entries(snapshot.activeUtilityByWindowId)) {
    if (utilityId !== undefined) activeUtilityByWindowId[windowId] = utilityId;
  }
  dispatch({
    type: "APPLY_TUTORIAL_UI_SNAPSHOT",
    snapshot: {
      activeWindowId: snapshot.focusedWindowId ?? null,
      shellLayout: seed.modeLayout,
      extraWindowInstances: seed.extraInstances,
      panelPatches,
      treeOpenStates,
      activeUtilityByWindowId,
      activeToolId: snapshot.activeToolId ?? null,
      dialog: snapshot.openDialogId === undefined ? null : ctx.restoreDialog(snapshot.openDialogId),
      commandPanelOpen: snapshot.commandPanelOpen,
    },
  });
  ctx.publishInteractionSelection(tutorialInteractionSelection(snapshot.interactionSelection));
  if (ctx.session) {
    dispatch({
      type: "SET_SESSION",
      value: (current) =>
        current
          ? {
              ...current,
              viewState: {
                ...current.viewState,
                activeModeId: snapshot.activeModeId ?? current.viewState.activeModeId,
                panelJson: snapshot.panelJson ?? current.viewState.panelJson,
              },
            }
          : current,
    });
  }
}

/** 🎥️ Applies one sparse `TutorialUiChange` (a `TutorialUiSample::Delta` entry, replayed by the director's per-tick `tutorialSlice`) onto the live `ShellState` by dispatching the SAME existing, targeted `ShellAction`s the real UI's own interactions use — never a bespoke tutorial-only mutation channel. */
export function applyTutorialUiChangeToShell(dispatch: (action: ShellAction) => void, change: TutorialUiChange, ctx: TutorialUiBridgeContext): void {
  switch (change.kind) {
    case "activeMode":
      if (!ctx.session) return;
      dispatch({ type: "SET_SESSION", value: (current) => (current ? { ...current, viewState: { ...current.viewState, activeModeId: change.id } } : current) });
      return;
    case "focusedWindow":
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: change.id ?? null });
      return;
    case "activeUtility":
      dispatch({ type: "SET_ACTIVE_UTILITY", windowId: change.windowId, utilityId: change.utilityId ?? null });
      return;
    case "activeTool":
      dispatch({ type: "SET_ACTIVE_TOOL", toolId: change.id ?? null });
      return;
    case "layout": {
      const windowKinds = ctx.session?.app.windowKinds.map((kind) => ({ id: kind.id, label: kind.label })) ?? [];
      const seed = applyFrameworkLayoutSeed(change.layout, windowKinds, ctx.appLabelsOverlay, ctx.terminology, ctx.locale);
      dispatch({ type: "SET_SHELL_LAYOUT", value: seed.modeLayout });
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seed.extraInstances });
      return;
    }
    case "panelTab": {
      const anchor = change.group as Anchor;
      if (!(ANCHORS as readonly string[]).includes(anchor)) return;
      dispatch({ type: "SET_PANEL_VISIBLE", anchor, value: change.tabId != null });
      dispatch({ type: "SET_PANEL_PATH", anchor, value: change.tabId ? [change.tabId] : [] });
      return;
    }
    case "panelState":
      if (!ctx.session) return;
      dispatch({ type: "SET_SESSION", value: (current) => (current ? { ...current, viewState: { ...current.viewState, panelJson: change.panelJson } } : current) });
      return;
    case "selection": {
      const selection = { ...ctx.interactionSelection() };
      if (change.ids.length === 0) delete selection[change.domainId];
      else Object.defineProperty(selection, change.domainId, { value: { granularity: change.granularity, ids: [...change.ids] }, enumerable: true, writable: true, configurable: true });
      ctx.publishInteractionSelection(selection);
      return;
    }
    case "dialog":
      dispatch({ type: "SET_DIALOG", value: change.id ? ctx.restoreDialog(change.id, change.args as Record<string, unknown> | undefined) : null });
      return;
    case "treeExpansion":
      dispatch({ type: "SET_TREE_OPEN_STATE", id: change.id, open: change.expanded });
      return;
    case "commandPanel":
      dispatch({ type: "SET_SEARCH_OPEN", value: change.open });
      return;
    default:
      return;
  }
}
//#endregion 🎥️TutorialUiBridge
//#endregion ShellHelpers
