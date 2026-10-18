import { DirectoryClient, DirectoryHttpError, createSocketGrantIssuerV1 } from "../../../../🟦️";
import type { DirectoryAcknowledgedStream } from "../../../../🟦️";
import { sealDirectoryCommandRequestV1 } from "@semio-tech/framework/directory/io/text";
import { directoryCommandErrorIsTransient } from "@semio-tech/framework/directory/schema";
import { browserDirectoryRequest, browserSessionAuthority, directoryCommandErrorCode, requestSocketGrant } from "../🔐️hub-session/🟦️.ts";
import { post } from "../🧾️document-state/🟦️.ts";

/** 🌐️ The signed-in user's preference lane: `user.preference-recorded` pages of the principal from the lane's frontier,
 * one page in flight, re-fetched whenever the directory head passes that frontier (any live event or heartbeat of its own
 * socket, which reconnects on its own). The lane never decodes the preference vocabulary — it hands the shell the
 * events, and the shell folds them (`🎚️UiPreferences` `foldUiPreferenceLanePageV1`). */
type PreferenceLaneV1 = {
  readonly userId: string;
  readonly client: DirectoryClient;
  readonly abort: AbortController;
  after: number;
  fetching: boolean;
  again: boolean;
  stream: DirectoryAcknowledgedStream | null;
};

let preferenceLane: PreferenceLaneV1 | null = null;

export function closePreferenceLane(): void {
  const lane = preferenceLane;
  preferenceLane = null;
  if (lane === null) return;
  lane.abort.abort();
  lane.stream?.close();
}

async function fetchPreferenceLanePages(lane: PreferenceLaneV1): Promise<void> {
  if (lane.fetching) {
    lane.again = true;
    return;
  }
  lane.fetching = true;
  try {
    do {
      lane.again = false;
      const page = await lane.client.preferencePage(lane.after, lane.userId, { signal: lane.abort.signal });
      if (preferenceLane !== lane) return;
      const authority = browserSessionAuthority;
      if (authority === null || page.sessionBindingSha256 !== authority.sessionBindingSha256 || page.authorizationGeneration !== authority.authorizationGeneration) {
        post({ kind: "preference-lane-failed", requestId: null, code: "unauthorized", retryable: false });
        closePreferenceLane();
        return;
      }
      lane.after = page.throughSeqInclusive;
      post({ kind: "preference-lane-page", userId: lane.userId, afterSeqExclusive: page.afterSeqExclusive, throughSeqInclusive: page.throughSeqInclusive, hasMore: page.hasMore, events: page.events });
      if (page.hasMore) lane.again = true;
    } while (lane.again && preferenceLane === lane);
  } catch (error) {
    if (preferenceLane !== lane || lane.abort.signal.aborted) return;
    const unauthorized = error instanceof DirectoryHttpError && error.status === 401;
    post({ kind: "preference-lane-failed", requestId: null, code: unauthorized ? "unauthorized" : "transport", retryable: !unauthorized });
  } finally {
    lane.fetching = false;
  }
}

export function openPreferenceLane(baseUrl: string, userId: string, after: number): void {
  closePreferenceLane();
  const client = new DirectoryClient(baseUrl, {
    requestBaseUrl: "",
    socketGrantIssuer: createSocketGrantIssuerV1({ post: (path, options) => requestSocketGrant(baseUrl, path, options?.signal) }),
    request: browserDirectoryRequest,
  });
  const lane: PreferenceLaneV1 = { userId, client, abort: new AbortController(), after, fetching: false, again: false, stream: null };
  preferenceLane = lane;
  void fetchPreferenceLanePages(lane);
  lane.stream = client.streamAcknowledged(after, (message) => {
    if (preferenceLane !== lane) return;
    const head = message.kind === "event" ? message.event.seq : message.kind === "heartbeat" ? message.headSeq : message.kind === "rebootstrap-required" ? Number.MAX_SAFE_INTEGER : 0;
    if (head > lane.after) void fetchPreferenceLanePages(lane);
  });
}

export async function recordPreferenceLaneChange(requestId: string, schema: string, mutation: string): Promise<void> {
  const lane = preferenceLane;
  if (lane === null) {
    post({ kind: "preference-lane-failed", requestId, code: "closed", retryable: true });
    return;
  }
  try {
    const receipt = await lane.client.command(sealDirectoryCommandRequestV1(requestId, { kind: "record-user-preference", schema, mutation }), { signal: lane.abort.signal });
    post({ kind: "preference-lane-recorded", requestId, outcome: receipt.outcome });
    void fetchPreferenceLanePages(lane);
  } catch (error) {
    const code = directoryCommandErrorCode(error, lane.abort.signal.aborted);
    post({ kind: "preference-lane-failed", requestId, code, retryable: directoryCommandErrorIsTransient(code) });
  }
}
