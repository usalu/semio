/** 🌊️ Directory stream: connection phases, presence actors, rebootstrap and access changes carried by the directory stream. */

import { type ArtifactFrontier, type ArtifactHash, type DocumentScope } from "../../../📄️document/🪪️scope/🟦️.ts";
import type { DirectoryEvent } from "../📜️event/🟦️.ts";
import type { ConnectionView } from "../👀️views/🟦️.ts";

//#region 🔖️Stream
export type DirectoryConnectionPhase = "opened" | "closed";

/** 👥️ One live presence actor in a document's roster (Amendment 3 to C1) — the hub knows all four
 * fields without ever decoding the actor's opaque `PresencePeer` bytes. */
export interface DirectoryPresenceActor {
  actor: string;
  userId?: string;
  surface: string;
  color: number;
}

/** 🛟️ Public checkpoint identity that makes a lagged client discard discontinuous live state. */
export interface RebootstrapRequired {
  scope: DocumentScope;
  checkpointId: ArtifactHash;
  descriptorDigestV1: ArtifactHash;
  baselineFrontier: ArtifactFrontier;
}

/** 🔑️ Which way one reader's own access to a space moved. */
export type DirectoryAccessChange = "granted" | "revoked";

export type DirectoryStreamMessage =
  | { kind: "event"; event: DirectoryEvent }
  | { kind: "connection"; phase: DirectoryConnectionPhase; connection: ConnectionView }
  | { kind: "presence"; spaceId: string; documentId: string; actors: DirectoryPresenceActor[] }
  | { kind: "heartbeat"; headSeq: number }
  | { kind: "rebootstrap-required"; control: RebootstrapRequired }
  | { kind: "access-changed"; spaceId: string; change: DirectoryAccessChange };
//#endregion 🔖️Stream
