import { parsePanelState, type SpawnedAppEntry } from "../../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import type { ActiveSession } from "../../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { OpenDocumentSession, RetainedBrowserActorUiV1 } from "../../../🔒️identity-document-ports/🧰️identity-port-types/🟦️.ts";

export function sameBrowserActorProgramV1(left: ActiveSession, right: ActiveSession): boolean {
  return left.pluginId === right.pluginId && left.instanceId === right.instanceId && left.app.id === right.app.id && left.app.role === right.app.role;
}

export function browserActorSpawnedRowV1(primary: ActiveSession, target: ActiveSession): SpawnedAppEntry | undefined {
  return parsePanelState(primary.viewState)?.spawnedApps.find(row => row.pluginId === target.pluginId && row.instanceId === target.instanceId && row.appId === target.app.id);
}

export function browserActorProgramIsPublishedV1(primary: ActiveSession | null, target: ActiveSession): boolean {
  return primary !== null && (sameBrowserActorProgramV1(primary, target) || browserActorSpawnedRowV1(primary, target) !== undefined);
}

export function browserActorProgramIsFocusedV1(primary: ActiveSession | null, target: ActiveSession): boolean {
  if (primary === null) return false;
  const active = parsePanelState(primary.viewState)?.activeSpawnedId;
  return active === undefined ? sameBrowserActorProgramV1(primary, target) : browserActorSpawnedRowV1(primary, target)?.id === active;
}

export function browserDocumentActorIdentityV1(entry: OpenDocumentSession, retained: RetainedBrowserActorUiV1 | undefined) {
  const identity = retained?.identity;
  const scope = entry.scope;
  if (retained === undefined || identity == null || scope === undefined
    || identity.activationGeneration !== retained.activationGeneration
    || identity.clientInstanceId !== retained.clientInstanceId || retained.clientInstanceId !== entry.clientInstanceId
    || identity.verifiedSurfaceId !== retained.verifiedSurfaceId || retained.verifiedSurfaceId !== entry.session.app.id
    || retained.sessionInstanceId !== entry.session.instanceId || entry.documentId !== scope.documentId
    || identity.scope.spaceId !== scope.spaceId || retained.scope.spaceId !== scope.spaceId
    || identity.scope.documentId !== scope.documentId || retained.scope.documentId !== scope.documentId) return null;
  return identity;
}


/** 🧭️ Admits a Workflow node route only once its exact focused role owns the mounted document. */
export function browserActorWorkflowNodeIsMountedV1(primary: ActiveSession | null, nodeId: string, signed?: Readonly<{
  spaceId: string;
  workflowArtifactId: string;
  documents: ReadonlyMap<string, OpenDocumentSession>;
  actors: ReadonlyMap<string, RetainedBrowserActorUiV1>;
  retired: WeakSet<OpenDocumentSession>;
}>): boolean {
  const panel = primary === null ? null : parsePanelState(primary.viewState);
  const row = panel?.spawnedApps.find(entry => entry.id === nodeId);
  if (panel?.activeSpawnedId !== nodeId || row === undefined) return false;
  if (signed === undefined) return true;
  for (const [runtimeKey, entry] of signed.documents) {
    if (entry.scope?.spaceId === signed.spaceId && entry.documentId !== signed.workflowArtifactId
      && entry.session.pluginId === row.pluginId && entry.session.instanceId === row.instanceId && entry.session.app.id === row.appId
      && !signed.retired.has(entry) && browserDocumentActorIdentityV1(entry, signed.actors.get(runtimeKey)) !== null) return true;
  }
  return false;
}
