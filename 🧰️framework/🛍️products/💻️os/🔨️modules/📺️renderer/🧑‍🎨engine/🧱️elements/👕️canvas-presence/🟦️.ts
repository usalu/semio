/** 👕️ Local presence window-view registry + artifact-roster fan-in for canvas hosts.
 * Ephemeral shared only — hosts publish throttled views; Shell heartbeat collects them into
 * `ArtifactPresencePeer.views`; hosts subscribe to the full roster to paint peer overlays.
 */
import { presenceTypingExcerpt, PRESENCE_PEER_WIRE_LIMITS_V1, type ArtifactPresencePeer, type ArtifactPresenceTyping, type ArtifactPresenceWindowView } from "@semio-tech/framework-replication";

export type LocalPresenceWindowPublicationV1 = {
  readonly runtimeKey: string;
  readonly view: ArtifactPresenceWindowView;
  readonly updatedAtMs: number;
};

const localViews = new Map<string, LocalPresenceWindowPublicationV1>();
const localListeners = new Set<() => void>();

const artifactRosters = new Map<string, readonly ArtifactPresencePeer[]>();
const rosterListeners = new Set<() => void>();

function notify(listeners: Set<() => void>): void {
  for (const listener of listeners) listener();
}

/** 📡️ Publish (or replace) one window's live camera + pointer for the next presence heartbeat. */
export function publishLocalPresenceWindowViewV1(runtimeKey: string, windowId: string, view: ArtifactPresenceWindowView): void {
  localViews.set(windowId, { runtimeKey, view: { ...view, windowId }, updatedAtMs: Date.now() });
  notify(localListeners);
}

export function clearLocalPresenceWindowViewV1(windowId: string): void {
  if (!localViews.delete(windowId)) return;
  notify(localListeners);
}

/** 🧺 Every live local window view for `runtimeKey`, ready to stamp onto an outbound PresenceHeartbeat. */
export function collectLocalPresenceWindowViewsV1(runtimeKey?: string): ArtifactPresenceWindowView[] {
  const views: ArtifactPresenceWindowView[] = [...localViews.values()]
    .filter((entry) => runtimeKey === undefined || entry.runtimeKey === runtimeKey || entry.runtimeKey === "local")
    .map((entry) => entry.view);
  views.sort((a, b) => (a.windowId < b.windowId ? -1 : a.windowId > b.windowId ? 1 : 0));
  return views;
}

const localTyping = new Map<string, { readonly runtimeKey: string; readonly typing: ArtifactPresenceTyping }>();

/** ⌨️ Publishes (or clears, with `null`) one text window's pending typing run for the next presence heartbeat — the
 * ephemeral shared preview peers render at this author's caret (design §13.2 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-
 * EDITING); the run itself lands as ONE edit when it commits. Excerpts are cut to the wire bound. */
export function publishLocalPresenceTypingV1(runtimeKey: string, windowId: string, preview: { readonly deleted: string; readonly insert: string } | null): void {
  const previous = localTyping.get(windowId)?.typing;
  if (preview === null) {
    if (!localTyping.delete(windowId)) return;
  } else {
    const typing = { windowId, deleted: presenceTypingExcerpt(preview.deleted), insert: presenceTypingExcerpt(preview.insert) };
    if (previous !== undefined && previous.deleted === typing.deleted && previous.insert === typing.insert) return;
    localTyping.set(windowId, { runtimeKey, typing });
  }
  notify(localListeners);
}

/** ⌨️ Every pending local typing run for `runtimeKey` (window id order, at most the wire's run limit), ready to stamp onto an
 * outbound PresenceHeartbeat. */
export function collectLocalPresenceTypingV1(runtimeKey?: string): ArtifactPresenceTyping[] {
  return [...localTyping.values()]
    .filter((entry) => runtimeKey === undefined || entry.runtimeKey === runtimeKey || entry.runtimeKey === "local")
    .map((entry) => entry.typing)
    .sort((a, b) => (a.windowId < b.windowId ? -1 : a.windowId > b.windowId ? 1 : 0))
    .slice(0, PRESENCE_PEER_WIRE_LIMITS_V1.maximumTypingRuns);
}

/** ⌨️ The heartbeat's `typing` field for `runtimeKey`: absent while no local run is pending. */
export function localPresenceTypingFieldsV1(runtimeKey?: string): { readonly typing?: readonly ArtifactPresenceTyping[] } {
  const typing = collectLocalPresenceTypingV1(runtimeKey);
  return typing.length === 0 ? {} : { typing };
}

export function subscribeLocalPresenceWindowViewsV1(listener: () => void): () => void {
  localListeners.add(listener);
  return () => { localListeners.delete(listener); };
}

/** 👥️ Store the verified artifact roster (full wire peers, including views + interaction). */
export function publishArtifactPresenceRosterV1(runtimeKey: string, peers: readonly ArtifactPresencePeer[]): void {
  artifactRosters.set(runtimeKey, peers);
  // 👕️ Surface hosts mount with runtimeKey "local" until they thread the document key; mirror the
  // latest verified roster there so CanvasPresenceOverlayV1 paints without a second wiring path.
  artifactRosters.set("local", peers);
  notify(rosterListeners);
}

export function clearArtifactPresenceRosterV1(runtimeKey: string): void {
  if (!artifactRosters.delete(runtimeKey)) return;
  notify(rosterListeners);
}

/** 👥️ The ONE empty roster: `useSyncExternalStore` compares snapshots by identity, so a fresh `[]` per read is an
 * endless re-render loop on every surface that has no roster yet. */
const EMPTY_ARTIFACT_PRESENCE_ROSTER: readonly ArtifactPresencePeer[] = Object.freeze([]);

export function artifactPresenceRosterV1(runtimeKey: string | null | undefined): readonly ArtifactPresencePeer[] {
  if (!runtimeKey) return EMPTY_ARTIFACT_PRESENCE_ROSTER;
  return artifactRosters.get(runtimeKey) ?? EMPTY_ARTIFACT_PRESENCE_ROSTER;
}

export function subscribeArtifactPresenceRosterV1(listener: () => void): () => void {
  rosterListeners.add(listener);
  return () => { rosterListeners.delete(listener); };
}


const localActiveTools = new Map<string, string>();

/** 🛠️ Publish the document's active tool/utility id for the next presence heartbeat (bit 12). */
export function publishLocalActiveToolV1(runtimeKey: string, toolId: string | null | undefined): void {
  if (!toolId) localActiveTools.delete(runtimeKey);
  else localActiveTools.set(runtimeKey, toolId);
}

export function collectLocalActiveToolV1(runtimeKey: string): string | undefined {
  return localActiveTools.get(runtimeKey) ?? localActiveTools.get("local");
}

const localActors = new Map<string, string>();
const actorListeners = new Set<() => void>();

/** 👥️ Hub-admitted actor id for this runtime — overlay hosts exclude it from peer paint. */
export function publishLocalPresenceActorV1(runtimeKey: string, actorId: string | null | undefined): void {
  if (!actorId || actorId === "__local__") {
    localActors.delete(runtimeKey);
    if (runtimeKey !== "local") localActors.delete("local");
  } else {
    localActors.set(runtimeKey, actorId);
    localActors.set("local", actorId);
  }
  notify(actorListeners);
}

export function clearLocalPresenceActorV1(runtimeKey: string): void {
  const prior = localActors.get(runtimeKey);
  if (!localActors.delete(runtimeKey)) return;
  if (runtimeKey !== "local" && localActors.get("local") === prior) localActors.delete("local");
  notify(actorListeners);
}

export function localPresenceActorV1(runtimeKey: string | null | undefined): string | null {
  if (!runtimeKey) return localActors.get("local") ?? null;
  return localActors.get(runtimeKey) ?? localActors.get("local") ?? null;
}

export function subscribeLocalPresenceActorV1(listener: () => void): () => void {
  actorListeners.add(listener);
  return () => { actorListeners.delete(listener); };
}

export const PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS = 50;
