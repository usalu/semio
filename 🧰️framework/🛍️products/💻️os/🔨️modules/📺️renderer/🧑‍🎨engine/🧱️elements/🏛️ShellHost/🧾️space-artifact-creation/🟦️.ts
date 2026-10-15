/** 🧾️ Space index constants and finders, the directory command result slots, space artifact creation catalog authority, projection, request and ready-opening helpers, and directory command mapping.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { type ShellDialogOriginV1 } from "../🗨️dialog-origin/🟦️.ts";
// #region 🔌️Adapters
import {
  type AppDefinition,
  type AppRole,
  type ArtifactDialect,
  type ArtifactKindChoice,
  dialectCoordinate,
  spaceIndexOf,
  type SpaceIndexContribution,
  decodeArtifactKindChoice,
  encodeArtifactKindChoice
} from "@semio-tech/framework";
import {
  type BackboneWorkerRequest,
  type BackboneWorkerResponse,
  type DirectoryCommand,
  type DirectoryCommandErrorCodeV1,
  type DirectoryCommandReceiptV1,
  type DocumentScope
} from "@semio-tech/framework-os";
// #endregion 🔌️Adapters
import { type IconName } from "@semio-tech/ui-react";
import { type ActiveSession, type LoadedProgramState } from "../../🐚️Shell/🟦️.tsx";
import { type ArtifactCreationProgressOwnerV1 } from "../🌱️artifact-creation/🟦️.tsx";

/** 🪪️ ticket §C4 — the space's own artifact-index document: its id is always the literal `"index"`
 * (one per hub space). Which plugin app edits it, and the document schema it is opened with, are named
 * by that plugin's own `os.space-index` topic contribution ({@link spaceIndexOf}), never by this shell. */
export const SPACE_INDEX_DOCUMENT_ID = "index";

/** 🪐️ The loaded plugin that names the space index, with its contribution and the editor app that serves it. */
export function findSpaceIndexEditor(plugins: readonly LoadedProgramState[]): { readonly plugin: LoadedProgramState; readonly index: SpaceIndexContribution; readonly app: AppDefinition } | undefined {
  for (const plugin of plugins) {
    const index = spaceIndexOf(plugin.manifest.topicContributions);
    const app = index && findDialectApp(plugin, index.dialect, "editor");
    if (index && app) return { plugin, index, app };
  }
  return undefined;
}

/** 🪐️ Whether `session` is the app that serves the space index its own plugin names. */
export function sessionIsSpaceIndex(session: ActiveSession | null | undefined, plugins: readonly LoadedProgramState[]): session is ActiveSession {
  if (!session || session.app.dialect === undefined) return false;
  const index = spaceIndexOf(plugins.find((entry) => entry.handle.pluginId === session.pluginId)?.manifest.topicContributions);
  return index !== undefined && dialectCoordinate(session.app.dialect) === dialectCoordinate(index.dialect);
}

//#region FrameworkOsShell
//#region 🐚️ShellMount
//#region 🔖️Identity
/** 📇️ §5 "wire the routing by dialect/surface id so it works the moment 2-B lands" — a direct
 * manifest scan (rather than the `AppRouter`/`appRouter.entriesFor` machinery, which is declared
 * later in `FrameworkOsShellInner` than `applyShellUri` and would be a `const` temporal-dead-zone
 * reference from there) for the one app a plugin declares for a given `(dialect, role)`. */
export function findDialectApp(plugin: LoadedProgramState | undefined, dialect: ArtifactDialect, role: AppRole): AppDefinition | undefined {
  return plugin?.manifest.apps.find((app) => app.dialect && dialectCoordinate(app.dialect) === dialectCoordinate(dialect) && app.role === role);
}

/** 👁️✏️ Fixed per-role icons for the navbar role group. Deliberately NOT the target app's own
 * `iconId`: both surfaces of one artifact normally carry the SAME artifact icon, which would make the
 * two buttons indistinguishable at a glance. */
export const SURFACE_ROLE_ICON_IDS: Readonly<Record<AppRole, IconName>> = { editor: "pencil", viewer: "eye" };

/** 🧾️ One retained, request-id-keyed command completion. A receipt lands here BEFORE any accepted
 * event is folded, so a completion always has an owner; a one-shot invite capability lives only in
 * this slot until an explicit administration copy handler consumes it. */
export type DirectoryCommandResultSlotV1 = { readonly kind: "receipt"; readonly receipt: DirectoryCommandReceiptV1 } | { readonly kind: "failed"; readonly code: DirectoryCommandErrorCodeV1 };

/** 📏️ Bounded retained result depth; the oldest completion is retired first. */
export const DIRECTORY_COMMAND_RESULT_SLOTS = 64;

/** 🧾️ Retains one bounded completion, newest last, replacing any prior entry for the same id. */
export function retainDirectoryCommandResult(slots: Map<string, DirectoryCommandResultSlotV1>, requestId: string, result: DirectoryCommandResultSlotV1): void {
  slots.delete(requestId);
  while (slots.size >= DIRECTORY_COMMAND_RESULT_SLOTS) {
    const oldest = slots.keys().next();
    if (oldest.done === true) break;
    slots.delete(oldest.value);
  }
  slots.set(requestId, result);
}

/** 🆔️ Mints one fresh 32-hex nonzero idempotency correlation. It is a correlation, never a
 * capability: the hub re-runs authentication and authorization before returning any completion. */
export function mintDirectoryCommandRequestId(): string {
  const bytes = new Uint8Array(16);
  globalThis.crypto.getRandomValues(bytes);
  bytes[0] = (bytes[0] ?? 0) | 1;
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** 🌱️ Converts the trusted-catalog presentation choice into the closed host creation request.
 * The decoded schema/dialect/labels are deliberately discarded; Hub resolves `kindId` again. */
export type SpaceArtifactCreationCatalogAuthorityV1 = Readonly<{
  runtimeKey: string;
  clientInstanceId: string;
  spaceId: string;
  catalogGenerationId: string;
  kindChoices: readonly string[];
}>;

export type SpaceArtifactCreationCatalogProjectionV1 = Readonly<{
  phase: "loading" | "ready" | "unavailable";
  kinds: readonly ArtifactKindChoice[];
  choiceRevision: string;
  authority: SpaceArtifactCreationCatalogAuthorityV1 | null;
}>;

export type ArtifactKindChoiceDraftDefinitionV1 = Readonly<{
  ownerId: string;
  args: readonly Readonly<{ id: string; artifactKind: boolean }>[];
}>;

/** 🧹️ Selects only staged artifact-kind fields; static choices and text remain untouched. */
export function artifactKindChoiceDraftRetirementsV1(
  stagedByOwner: Readonly<Record<string, Readonly<Record<string, unknown>>>>,
  definitions: readonly ArtifactKindChoiceDraftDefinitionV1[],
): readonly Readonly<{ ownerId: string; argId: string }>[] {
  const retirements: { ownerId: string; argId: string }[] = [];
  for (const definition of definitions) {
    const staged = stagedByOwner[definition.ownerId];
    if (staged === undefined) continue;
    for (const arg of definition.args) if (arg.artifactKind && Object.prototype.hasOwnProperty.call(staged, arg.id) && staged[arg.id] !== undefined) retirements.push({ ownerId: definition.ownerId, argId: arg.id });
  }
  return retirements;
}

/** 🌱️ Captures only a ready catalog owned by the exact mounted Space-index instance. */
export function captureSpaceArtifactCreationCatalogAuthorityV1(
  catalog: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog" }> | null,
  status: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog-status" }> | null,
  origin: ShellDialogOriginV1 | null,
): SpaceArtifactCreationCatalogAuthorityV1 | null {
  const document = origin?.document;
  const scope = document?.scope;
  if (document === null || document === undefined || scope === null || scope === undefined || scope.documentId !== SPACE_INDEX_DOCUMENT_ID
    || status?.phase !== "ready" || status.clientInstanceId !== document.clientInstanceId || status.spaceId !== scope.spaceId
    || catalog === null || catalog.clientInstanceId !== document.clientInstanceId || catalog.spaceId !== scope.spaceId) return null;
  return {
    runtimeKey: document.runtimeKey,
    clientInstanceId: document.clientInstanceId,
    spaceId: scope.spaceId,
    catalogGenerationId: catalog.catalogGenerationId,
    kindChoices: catalog.kinds.map(encodeArtifactKindChoice),
  };
}

/** 🗂️ Projects the current mounted catalog without falling back to process manifests. */
export function selectedSpaceArtifactCreationCatalogV1(
  catalog: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog" }> | null,
  status: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog-status" }> | null,
  origin: ShellDialogOriginV1 | null,
): SpaceArtifactCreationCatalogProjectionV1 | null {
  const document = origin?.document;
  const scope = document?.scope;
  if (document === null || document === undefined || scope === null || scope === undefined || scope.documentId !== SPACE_INDEX_DOCUMENT_ID) return null;
  const authority = captureSpaceArtifactCreationCatalogAuthorityV1(catalog, status, origin);
  const phase = authority !== null ? "ready" : status?.clientInstanceId === document.clientInstanceId && status.spaceId === scope.spaceId ? status.phase : "loading";
  const safePhase = phase === "ready" ? "unavailable" : phase;
  const effectivePhase = authority === null ? safePhase : "ready";
  return {
    phase: effectivePhase,
    kinds: authority === null ? [] : catalog!.kinds,
    choiceRevision: `${document.runtimeKey}:${document.clientInstanceId}:${scope.spaceId}:${effectivePhase}:${authority?.catalogGenerationId ?? "none"}`,
    authority,
  };
}

/** 📇️ An opening or directory command a mounted space index asks for names the index's own space: the index is the
 * space's directory projection and the guest's document carries no space id of its own, so an absent or empty `spaceId`
 * is the index's scope. Any other origin, and any request that already names a space, is left exactly as it came. */
export function spaceIndexOpeningArgsV1(args: Readonly<Record<string, unknown>> | undefined, origin: DocumentScope | undefined, indexDocumentId: string): Readonly<Record<string, unknown>> | undefined {
  if (args === undefined || origin === undefined || origin.documentId !== indexDocumentId) return args;
  if (typeof args.spaceId === "string" && args.spaceId.length > 0) return args;
  return { ...args, spaceId: origin.spaceId };
}

/** 🌱️ Converts only an exact current selected-catalog member into the closed host request. The hub catalog is the
 * authority for both halves of a member: the creation kind (`kindId`, e.g. `2d.drawing`) and the dialect whose surface
 * opens it (`dialect.artifactKind`, e.g. `s.draw.drawing`) are independent names, never required to be equal. */
export function spaceArtifactCreationRequestFromAction(
  actionId: string,
  args: Readonly<Record<string, unknown>> | undefined,
  spaceId: string,
  requestId: string,
  capturedCatalog: SpaceArtifactCreationCatalogAuthorityV1 | null,
  currentCatalog: SpaceArtifactCreationCatalogAuthorityV1 | null,
): Extract<BackboneWorkerRequest, { readonly kind: "space-artifact-create" }> | null {
  if (actionId !== "os.create-space-artifact" || args === undefined || Object.keys(args).sort().join(",") !== "kindChoice,name" || typeof args.kindChoice !== "string" || typeof args.name !== "string") return null;
  try {
    const choice = decodeArtifactKindChoice(args.kindChoice);
    const encoded = encodeArtifactKindChoice(choice);
    if (encoded !== args.kindChoice || capturedCatalog === null || currentCatalog === null
      || capturedCatalog.runtimeKey !== currentCatalog.runtimeKey || capturedCatalog.clientInstanceId !== currentCatalog.clientInstanceId
      || capturedCatalog.spaceId !== spaceId || currentCatalog.spaceId !== spaceId || capturedCatalog.catalogGenerationId !== currentCatalog.catalogGenerationId
      || !capturedCatalog.kindChoices.includes(encoded) || !currentCatalog.kindChoices.includes(encoded)) return null;
    return { kind: "space-artifact-create", requestId, spaceId, expectedCatalogGenerationId: capturedCatalog.catalogGenerationId, kindId: choice.kindId, name: args.name };
  } catch {
    return null;
  }
}

export type SpaceArtifactCreationOwnerV1 = ArtifactCreationProgressOwnerV1 & Readonly<{
  opening: boolean;
  cancelRequested: boolean;
  ready: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }> | null;
}>;

export function spaceArtifactCreationOwnerAcceptsStatus(
  owner: SpaceArtifactCreationOwnerV1,
  message: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }>,
): boolean {
  if (owner.requestId !== message.requestId || owner.spaceId !== message.spaceId || owner.expectedCatalogGenerationId !== message.catalogGenerationId || (message.ready !== undefined && message.ready.kindId !== owner.kindId)) return false;
  if (owner.ready === null) return true;
  const retained = owner.ready.ready;
  return message.phase === "ready" && retained !== undefined && message.ready !== undefined
    && message.ready.artifactId === retained.artifactId && message.ready.kindId === retained.kindId
    && message.ready.artifactSchema === retained.artifactSchema
    && message.ready.parentDialect.artifactKind === retained.parentDialect.artifactKind
    && message.ready.parentDialect.standard === retained.parentDialect.standard
    && message.ready.parentDialect.subset === retained.parentDialect.subset;
}

/** 🔄️ Admits one catalog refresh only for the exact nonterminal creation, selected catalog and live Space-index mount. */
export function spaceArtifactCreationCatalogRefreshRequestV1(
  owner: SpaceArtifactCreationOwnerV1 | null,
  authority: SpaceArtifactCreationCatalogAuthorityV1 | null,
  origin: ShellDialogOriginV1 | null,
  message: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog-refresh-required" }>,
): Extract<BackboneWorkerRequest, { readonly kind: "space-artifact-creation-catalog-open" }> | null {
  const document = origin?.document;
  const scope = document?.scope;
  if (owner === null || owner.ready !== null || owner.opening || authority === null || origin === null || document === null || document === undefined || scope === null || scope === undefined
    || message.requestId !== owner.requestId || message.spaceId !== owner.spaceId || message.catalogGenerationId !== owner.expectedCatalogGenerationId
    || authority.runtimeKey !== owner.runtimeKey || authority.clientInstanceId !== owner.clientInstanceId || authority.spaceId !== owner.spaceId || authority.catalogGenerationId !== owner.expectedCatalogGenerationId
    || origin.sessionInstanceId !== owner.sessionInstanceId || document.runtimeKey !== owner.runtimeKey || document.clientInstanceId !== owner.clientInstanceId
    || scope.spaceId !== owner.spaceId || scope.documentId !== SPACE_INDEX_DOCUMENT_ID) return null;
  return { kind: "space-artifact-creation-catalog-open", clientInstanceId: owner.clientInstanceId, spaceId: owner.spaceId };
}

export function spaceArtifactCreationReadyOpening(
  message: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }>,
): Readonly<Record<string, string>> | null {
  if (message.phase !== "ready" || message.ready === undefined) return null;
  return {
    artifactRef: `${message.ready.parentDialect.artifactKind}@${message.ready.parentDialect.standard}/${message.ready.parentDialect.subset}`,
    artifactId: message.ready.artifactId,
    spaceId: message.spaceId,
    schema: message.ready.artifactSchema,
  };
}

/** 📇️ Maps an `os.directory.<verb>` action id (contract §C6's 7 command ids) + its relayed JSON args
 * onto a {@link DirectoryCommand} — `share-link` has no directory-schema command kind of its own
 * (contract §C1), so it's client-side sugar for `create-invite` (see the new
 * `🎮️commands/🔗️directory-share-link/🦀️.rs` header doc). Returns `null` for an
 * unrecognized verb (defensive — every relay source in this codebase today only emits the 7 frozen
 * ids, but a foreign/future caller should never crash the funnel). */
export function directoryCommandFromAction(actionId: string, args: Record<string, unknown> | undefined): DirectoryCommand | null {
  const a = args ?? {};
  switch (actionId) {
    case "os.directory.create-space":
      return { kind: "create-space", name: String(a.name ?? ""), spaceKind: (a.spaceKind as string) ?? "atelier", visibility: (a.visibility as string) ?? "private" } as DirectoryCommand;
    case "os.directory.delete-space":
      return { kind: "delete-space", spaceId: String(a.spaceId ?? "") } as DirectoryCommand;
    case "os.directory.rename-space":
      return { kind: "rename-space", spaceId: String(a.spaceId ?? ""), name: String(a.name ?? "") } as DirectoryCommand;
    case "os.directory.set-visibility":
      return { kind: "set-visibility", spaceId: String(a.spaceId ?? ""), visibility: (a.visibility as string) ?? "private" } as DirectoryCommand;
    case "os.directory.upsert-member":
      return { kind: "upsert-member", spaceId: String(a.spaceId ?? ""), email: String(a.email ?? ""), role: (a.role as string) ?? "spectator" } as DirectoryCommand;
    case "os.directory.remove-member":
      return { kind: "remove-member", spaceId: String(a.spaceId ?? ""), userId: String(a.userId ?? "") } as DirectoryCommand;
    case "os.directory.share-link":
      return { kind: "create-invite", spaceId: String(a.spaceId ?? ""), role: (a.role as string) ?? "spectator", ttlSecs: Number(a.ttlSecs ?? 3600) } as DirectoryCommand;
    default:
      return null;
  }
}
//#endregion 🔖️Identity
//#endregion 🐚️ShellMount
//#endregion FrameworkOsShell
