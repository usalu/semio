/** 👥️ Shell plumbing, presence identity: the presence storage key and timing constants, the ephemeral snapshot bound and peer fields and the packed client identity. */
import { packValueFromBase64, packValueToBase64 } from "@semio-tech/framework/packs";
import {
  decodePresenceHistoryEdit,
  decodePresenceInteraction,
  decodePresenceToolRun,
  type ArtifactPresenceHistoryEdit,
  type ArtifactPresenceInteraction,
  type ArtifactPresenceToolRun
} from "@semio-tech/framework/replication";

//#region ShellHelpers
export const PRESENCE_CLIENT_STORAGE_KEY = "semio.presence.client";
export const PRESENCE_HEARTBEAT_INTERVAL_MS = 5000;

/** 🖱️ The least spacing between two presence beats a local window-view change (pointer, camera, caret) triggers, so a
 * peer's cursor follows within ~one frame budget instead of the next {@link PRESENCE_HEARTBEAT_INTERVAL_MS} beat. Pinned
 * by `💓️presence-liveness-v1` (`a-moving-pointer-beats-live`). */
export const PRESENCE_LIVE_VIEW_BEAT_INTERVAL_MS = 100;

/** ⏳️ How long a view-change beat waits: none once the last beat is {@link PRESENCE_LIVE_VIEW_BEAT_INTERVAL_MS} old,
 * else the rest of that spacing (a clock that ran backwards waits the whole spacing). */
export function presenceLiveViewBeatDelayMsV1(lastBeatAtMs: number, nowMs: number): number {
  const elapsed = nowMs - lastBeatAtMs;
  return elapsed < 0 ? PRESENCE_LIVE_VIEW_BEAT_INTERVAL_MS : Math.max(0, PRESENCE_LIVE_VIEW_BEAT_INTERVAL_MS - elapsed);
}

/** ⏱️ How long one presence beat may wait for the document component's ephemeral snapshot before it
 * goes out without one. Strictly below {@link PRESENCE_HEARTBEAT_INTERVAL_MS} so a slow document can
 * never push a beat into the next interval, and the pair sits far below the hub's own
 * `PRESENCE_LEASE_TTL_MS` (15 000) — three whole beats of headroom. The three numbers are pinned
 * together, in both languages, by `🧫️fixtures/💓️presence-liveness-v1/🔣️.json`. */
export const PRESENCE_EPHEMERAL_SNAPSHOT_DEADLINE_MS = 2000;

/** 💓️ One document's ephemeral snapshot, bounded. A wedged document component answers
 * `ephemeralSnapshot` never; awaiting it bare inside the beat's single flight silenced the shell's
 * heartbeat for that document for the rest of the session, and the hub then let the human's roster
 * row lapse under a socket that was still open (ticket 26/09/18 slice PR1). Past the bound the beat
 * carries no app-owned pack — the human stays present, only the app's ephemerals are dropped, which
 * is precisely what the hub publishes for a lapsed lease. A rejection is the same answer as a
 * timeout: presence is not the place to surface a document fault. */
export async function presenceEphemeralSnapshotWithinBoundV1(
  plugin: {
    readonly ephemeralSnapshot?: (
      instanceId: number,
    ) => Promise<{
      readonly presence: readonly number[];
      readonly interaction?: readonly number[];
      readonly toolRun?: readonly number[];
      readonly historyEdit?: readonly number[];
      readonly activeTool?: string;
      readonly presenceGeneration?: number;
      readonly transientGeneration?: number;
    } | null>;
  },
  instanceId: number,
  deadlineMs: number = PRESENCE_EPHEMERAL_SNAPSHOT_DEADLINE_MS,
): Promise<
  | {
      readonly presence: readonly number[];
      readonly interaction?: readonly number[];
      readonly toolRun?: readonly number[];
      readonly historyEdit?: readonly number[];
      readonly activeTool?: string;
      readonly presenceGeneration?: number;
      readonly transientGeneration?: number;
    }
  | undefined
> {
  if (plugin.ephemeralSnapshot === undefined) return undefined;
  let timer: ReturnType<typeof setTimeout> | undefined;
  const bound = new Promise<undefined>((resolve) => {
    timer = setTimeout(() => resolve(undefined), deadlineMs);
  });
  try {
    return await Promise.race([Promise.resolve(plugin.ephemeralSnapshot(instanceId)).then((snapshot) => snapshot ?? undefined).catch(() => undefined), bound]);
  } finally {
    if (timer !== undefined) clearTimeout(timer);
  }
}

/** 👥️ The fields of this human's presence heartbeat an app's ephemeral snapshot owns, decoded and present only when the
 * snapshot carries them: the declared-broadcast interaction slice (bit 5), the tool-run summary (bit 10) and the open
 * history edit (bit 13) — the React twin of what the native host stamps, so a peer sees who edits history on which
 * mutation whichever renderer that human runs. */
export function presenceEphemeralPeerFieldsV1(snapshot: { readonly interaction?: readonly number[]; readonly toolRun?: readonly number[]; readonly historyEdit?: readonly number[] } | undefined): { readonly interaction?: ArtifactPresenceInteraction; readonly toolRun?: ArtifactPresenceToolRun; readonly historyEdit?: ArtifactPresenceHistoryEdit } {
  return {
    ...(snapshot?.interaction !== undefined && snapshot.interaction.length > 0 ? { interaction: decodePresenceInteraction(Uint8Array.from(snapshot.interaction), [0]) } : {}),
    ...(snapshot?.toolRun !== undefined && snapshot.toolRun.length > 0 ? { toolRun: decodePresenceToolRun(Uint8Array.from(snapshot.toolRun)) } : {}),
    ...(snapshot?.historyEdit !== undefined && snapshot.historyEdit.length > 0 ? { historyEdit: decodePresenceHistoryEdit(Uint8Array.from(snapshot.historyEdit)) } : {}),
  };
}

function presenceIdentityPackBase64(identity: { readonly clientId: string; readonly name: string }): string {
  return packValueToBase64(identity);
}

function presenceIdentityFromPackBase64(encoded: string): { readonly clientId: string; readonly name: string } | null {
  try {
    const decoded = packValueFromBase64(encoded) as { readonly clientId?: string; readonly name?: string };
    if (decoded.clientId && decoded.name) return { clientId: decoded.clientId, name: decoded.name };
  } catch {
    return null;
  }
  return null;
}

/** 🪪️ `real`, when given, is the ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS
 * shell's resolved `Identity` presence pair (`user:{userId}#{shellSessionId}` + `displayName`) —
 * ShellHost's only caller passes it once sign-in resolves, so presence/heartbeat labels show the real
 * signed-in user instead of a random per-tab `Guest ####`. Absent (no hub env, offline, or not yet
 * resolved) falls back to the pre-existing session-storage-cached guest identity unchanged. */
export function presenceClientIdentity(ephemeral = false, real?: { readonly clientId: string; readonly name: string }): { readonly clientId: string; readonly name: string } {
  if (real) return real;
  if (typeof window === "undefined") return { clientId: "server", name: "Server" };
  if (!ephemeral) {
    const stored = window.sessionStorage.getItem(PRESENCE_CLIENT_STORAGE_KEY);
    if (stored) {
      const parsed = presenceIdentityFromPackBase64(stored);
      if (parsed) return parsed;
    }
  }
  const clientId = `client-${Math.random().toString(36).slice(2, 10)}`;
  const identity = { clientId, name: `Guest ${clientId.slice(-4).toUpperCase()}` };
  if (!ephemeral) window.sessionStorage.setItem(PRESENCE_CLIENT_STORAGE_KEY, presenceIdentityPackBase64(identity));
  return identity;
}
//#endregion ShellHelpers
