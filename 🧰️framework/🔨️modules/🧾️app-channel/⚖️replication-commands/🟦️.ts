/** ⚖️ App channel replication commands: the merge-policy authority, conflict resolution and the document-wide presence roster. */

import type { ConflictResolution, MergePolicy } from "@semio-tech/framework";
import { conflictResolutionAsU8, mergePolicyAsU8 } from "../../🎠️kernel/🔄️merge-outcome/🟦️.ts";
import type { ArtifactPresencePeer } from "@semio-tech/framework-replication";
import { encodePresencePeer } from "@semio-tech/framework-replication";
import type { AppFrameValue } from "../../🪢️wire-contracts/🟦️.ts";
import type { ReplyLedger } from "../📮️reply-ledger/🟦️.ts";

//#region 🔖️AppChannelClient
/** 🤝️ Sends the commands that keep this instance consistent with its peers. */
export class ReplicationCommands {
  private readonly ledger: ReplyLedger;

  constructor(ledger: ReplyLedger) {
    this.ledger = ledger;
  }

  //#region 🔖️Merge
  /** 🧭️ Sets this instance's local merge-policy authority (`os.set-merge-policy`, C6/C9) — a `Done`
   * reply, never a `MergeReport`/`Conflicts` (those only follow an ingest). */
  async setMergePolicy(policy: MergePolicy): Promise<AppFrameValue[]> {
    return this.ledger.send({ setMergePolicy: { seq: this.ledger.nextSeq(), policy: mergePolicyAsU8(policy) } });
  }

  /** ⚔️ Accepts or discards an `Open` {@link Conflict} (`os.resolve-conflict`) — replays it under
   * `LaissezFaire` on `accept` (Quarantined) or acks it in place (Degraded); `discard` on a
   * Quarantined conflict seeds the DAG as already-seen without ever relaying it, on a Degraded
   * conflict it is rejected (never rewrites shared history, C6 §`resolve_conflict`). Returns the
   * authoritative `MergeReport` + `Conflicts` frames. */
  async resolveConflict(conflictId: string, resolution: ConflictResolution): Promise<AppFrameValue[]> {
    return this.ledger.send({ resolveConflict: { seq: this.ledger.nextSeq(), conflict_id: conflictId, resolution: conflictResolutionAsU8(resolution) } });
  }

  /** 📖️ Reads the open-conflict projection (`os.read-conflicts`) — one `Conflicts` reply frame. */
  async readConflicts(): Promise<AppFrameValue[]> {
    return this.ledger.send({ readConflicts: { seq: this.ledger.nextSeq() } });
  }
  //#endregion 🔖️Merge

  //#region 🔖️Presence
  /** 👥️ Pushes the document-wide presence roster into this instance's plugin app — the ONLY plugin
   * ingress for peers (contract-freeze §C7.6). Encodes each {@link ArtifactPresencePeer} via
   * {@link encodePresencePeer}; the caller has already dropped its own actor from `peers` before
   * calling this (`ownColor` carries this actor's own hub-assigned palette index separately, `null`
   * for a folder-only session with no hub). A plain `Done` reply, never decoded further here. */
  async pushPresence(ownColor: number | null, peers: readonly ArtifactPresencePeer[]): Promise<AppFrameValue[]> {
    return this.ledger.send({ presence: { seq: this.ledger.nextSeq(), own_color: ownColor, peers: peers.map((peer) => encodePresencePeer(peer)) } });
  }
  //#endregion 🔖️Presence
}
//#endregion 🔖️AppChannelClient
