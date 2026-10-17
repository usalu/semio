/** 🪪️ App channel document reads: the document, its identity, child heads, history projection and the per-window config envelopes. */

import type { AppDocumentIdentity, AppFrameValue, ChildHeadPackEntry, WindowConfigPackEntry } from "../../🪢️wire-contracts/🟦️.ts";
import { assertAppDocumentIdentity } from "../🏷️codec-primitives/🟦️.ts";
import { channelLabel, throwOnFault } from "../🧯️reply-faults/🟦️.ts";
import type { ReplyLedger } from "../📮️reply-ledger/🟦️.ts";

//#region 🔖️AppChannelClient
/** 📖️ The read side of one app instance's document: every method sends one command and decodes the one reply frame that answers its `seq`. */
export class DocumentReads {
  private readonly ledger: ReplyLedger;
  private readonly instanceId: number;
  private readonly appId: string;

  constructor(ledger: ReplyLedger, instanceId: number, appId: string) {
    this.ledger = ledger;
    this.instanceId = instanceId;
    this.appId = appId;
  }

  /** 📄️ Reads the document; the frames are routed by the caller (`AppFrame::Document` also refreshes the pack cache). */
  async readDocument(): Promise<AppFrameValue[]> {
    return this.ledger.send({ ReadDocument: { seq: this.ledger.nextSeq() } });
  }

  /** 🆔️ Reads the actor's trusted document envelope identity without exporting its bytes. */
  async readDocumentIdentity(): Promise<AppDocumentIdentity> {
    const seq = BigInt(this.ledger.nextSeq());
    const frames = await this.ledger.send({ ReadDocumentIdentity: { seq } });
    throwOnFault(frames, "document identity");
    const replies = frames.filter((frame): frame is Extract<AppFrameValue, { readonly DocumentIdentity: unknown }> => "DocumentIdentity" in frame && frame.DocumentIdentity.in_reply_to === seq);
    if (replies.length !== 1) throw new Error("document identity: missing or ambiguous reply");
    const identity = replies[0]!.DocumentIdentity.identity;
    assertAppDocumentIdentity(identity);
    if (identity.app_instance_id !== this.instanceId) throw new Error("document identity: foreign instance");
    return identity;
  }

  /** 🪆️ Reads every owned child's CURRENT head snapshot pack (`AppCommand::ReadChildHeads` → `AppFrame::ChildHeads`), for a
   * reader that composes parent + children on read (design §20.15). A guest fault rejects with the guest's own fault text. */
  async readChildHeads(): Promise<readonly ChildHeadPackEntry[]> {
    const seq = this.ledger.nextSeq();
    const frames = await this.ledger.send({ ReadChildHeads: { seq } });
    const label = channelLabel("readChildHeads", this.appId);
    throwOnFault(frames, label);
    const heads = frames.find((frame): frame is Extract<AppFrameValue, { readonly ChildHeads: unknown }> => "ChildHeads" in frame && frame.ChildHeads.in_reply_to === seq);
    if (!heads) throw new Error(`${label}: missing ChildHeads frame for seq ${seq}`);
    return heads.ChildHeads.entries;
  }

  /** 🧾️ Retrieves the complete history projection for initial load or cursor-gap recovery. */
  async readHistory(): Promise<AppFrameValue[]> {
    return this.ledger.send({ ReadHistory: { seq: this.ledger.nextSeq() } });
  }

  /** 🪟️ Restores one exact concrete window's persisted-local config envelope. */
  async loadWindowConfig(entry: WindowConfigPackEntry): Promise<void> {
    const seq = this.ledger.nextSeq();
    const frames = await this.ledger.send({
      LoadWindowConfig: {
        seq,
        entry: { window_id: entry.window_id, window_kind_id: entry.window_kind_id, envelope_pack: Array.from(entry.envelope_pack) },
      },
    });
    const label = channelLabel("loadWindowConfig", this.appId);
    throwOnFault(frames, label);
    if (!frames.some((frame) => "Done" in frame && frame.Done.in_reply_to === seq)) {
      throw new Error(`${label}: missing Done frame for seq ${seq}`);
    }
  }

  /** 🗂️ Reads every exact concrete window config envelope for persisted-local replay. */
  async readWindowConfigs(): Promise<readonly WindowConfigPackEntry[]> {
    const seq = this.ledger.nextSeq();
    const frames = await this.ledger.send({ ReadWindowConfigs: { seq } });
    const label = channelLabel("readWindowConfigs", this.appId);
    throwOnFault(frames, label);
    const configs = frames.find(
      (frame): frame is Extract<AppFrameValue, { readonly WindowConfigs: unknown }> =>
        "WindowConfigs" in frame && frame.WindowConfigs.in_reply_to === seq,
    );
    if (!configs) throw new Error(`${label}: missing WindowConfigs frame for seq ${seq}`);
    return configs.WindowConfigs.entries.map((entry) => ({
      window_id: entry.window_id,
      window_kind_id: entry.window_kind_id,
      envelope_pack: Array.from(entry.envelope_pack),
    }));
  }
}
//#endregion 🔖️AppChannelClient
