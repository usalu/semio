/** 🧰️ The shared identity and document port types: the open document session, background index session and retained browser actor records. */
import { FolderArchivePersistenceV1, FolderReadBackRouteV1, LatestDocumentReplacementV1 } from "../../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { type ArtifactCreationCatalogMountV1 } from "../../🌱️artifact-creation/🚪️ready-opening/🟦️.ts";
import { type AppRole, type ArtifactDialect } from "@semio-tech/framework";
import { type DocumentScope } from "@semio-tech/framework/document/scope";
import { type DocumentArchivePack } from "@semio-tech/framework/wire-contracts";
import { type BrowserActorUiMountedV1, type PersistenceBinding } from "@semio-tech/framework/backbone";
import { type DirectorySessionAuthorityV1 } from "@semio-tech/framework/directory/schema/session-authority-v1";
import { type InstalledServiceHistoryStatusV1 } from "@semio-tech/framework/inference/service";
import { UiDocumentStore } from "../../../📃️UiDocumentStore/🟦️.tsx";
import { type DocumentOpeningReference, type DocumentOpeningTarget } from "../../🧭️opening/🟦️.ts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { type ActorDocumentMessagePortV1 } from "@semio-tech/framework/backbone/binding";
import { BrowserActorActionMailboxV1 } from "@semio-tech/framework/browser-bundle/action-handoff/requests";

/** 📇️ `applyHostEffects`'s new `replayShellCommand` branch (below) needs `openDocument`/
 * `openArtifactWithAppRef`, both declared LATER in this same component (after `applyHostEffects`
 * itself) — a direct reference in `applyHostEffects`'s own dependency array would be a `const`
 * temporal-dead-zone violation at the point `useCallback` evaluates that array. Same ref-forwarding
 * idiom `onActionRef`/`dispatchDirectoryEventsRef` already use: assigned as a plain statement right
 * after each real declaration, read only from inside a later callback body, never from a deps array. */
export type OpenDocumentSessionTarget = DocumentOpeningTarget<ActiveSession, PluginWasmHandle> & { readonly background?: boolean; readonly expectedCatalogGenerationId?: string };

export type PreparedArtifactOpeningTarget = OpenDocumentSessionTarget & Readonly<{ dialect: ArtifactDialect; role: AppRole }>;

export type OpenDocumentSession = {
  readonly openingReference: DocumentOpeningReference;
  readonly persistenceBindings: readonly PersistenceBinding[];
  session: ActiveSession;
  plugin: PluginWasmHandle;
  documentId: string;
  clientInstanceId: string;
  scope?: DocumentScope;
  port: ActorDocumentMessagePortV1 | null;
  pending: Uint8Array[];
  pendingBytes: number;
  replacements: LatestDocumentReplacementV1<DocumentArchivePack>;
  persistence: FolderArchivePersistenceV1;
  readBack: FolderReadBackRouteV1<DocumentArchivePack>;
  creationMount: ArtifactCreationCatalogMountV1 | null;
  ready: Promise<void>;
  resolveReady(): void;
  rejectReady(error: Error): void;
};

export type BackgroundSpaceIndexSession = {
  readonly plugin: PluginWasmHandle;
  readonly session: ActiveSession;
  readonly runtimeKey: string;
  readonly clientInstanceId: string;
  readonly hubBaseUrl: string;
  readonly userId: string;
};

export type MountedInferenceHistoryV1 = Readonly<{ historyEpoch: number; clientInstanceId: string; scope: DocumentScope; sessionInstanceId: number; status: InstalledServiceHistoryStatusV1; owner: string; serviceId: string; order: number; authority: DirectorySessionAuthorityV1 }>;

export type RetainedBrowserActorUiV1 = { readonly clientInstanceId: string; readonly activationGeneration: string; readonly verifiedSurfaceId: string; readonly scope: DocumentScope; readonly sessionInstanceId: number; readonly windows: Map<string, UiDocumentStore>; readonly panels: Map<string, UiDocumentStore>; readonly sections: Map<string, UiDocumentStore>; readonly identity: BrowserActorUiMountedV1 | null; readonly actions: BrowserActorActionMailboxV1 };
