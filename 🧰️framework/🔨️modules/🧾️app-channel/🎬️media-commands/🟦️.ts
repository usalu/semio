/** 🎬️ App channel media commands: the agent-facing media lanes and the bounded media-export protocol. */

import type { AppFrameValue, MediaExportHandle } from "../../🪢️wire-contracts/🟦️.ts";
import { encodePackValue } from "../../🎁️packs/🟦️.ts";
import type { ReplyLedger } from "../📮️reply-ledger/🟦️.ts";

//#region 🔖️AppChannelClient
/** 🖼️ Sends the media commands of one app instance; none of them decodes its reply, the caller reads the frames. */
export class MediaCommands {
  private readonly ledger: ReplyLedger;

  constructor(ledger: ReplyLedger) {
    this.ledger = ledger;
  }

  /** 📤️ Produces one OUT port's current bytes (`AppCommand::MediaOut` → `AppFrame::Media`) — the
   * agent-facing export lane, which returns the guest's own bytes to the caller instead of handing
   * the human a download. Deliberately the SAME command the headless gateway drives, so an export
   * over the live-shell route and one over the headless route ask the guest the identical question. */
  async mediaOut(port: string): Promise<AppFrameValue[]> {
    return this.ledger.send({ MediaOut: { seq: this.ledger.nextSeq(), port, request: [] } });
  }

  /** 🗳️ Delivers one typed media artifact to a fresh or existing app instance. */
  async mediaIn(port: string, descriptor: unknown, data: Uint8Array): Promise<AppFrameValue[]> {
    return this.ledger.send({ MediaIn: { seq: this.ledger.nextSeq(), port, descriptor: Array.from(encodePackValue(descriptor)), data: Array.from(data) } });
  }

  /** 🚀️ Admits media work only against the caller's exact document and revision. */
  async submitMediaExport(port: string, parentDocumentId: string, revision: bigint): Promise<AppFrameValue[]> {
    return this.ledger.send({ SubmitMediaExport: { seq: BigInt(this.ledger.nextSeq()), port, expected_parent_document_id: parentDocumentId, expected_base_revision: revision } });
  }

  /** ⏱️ Advances one bounded media-export slice. */
  async pollMediaExport(handle: MediaExportHandle): Promise<AppFrameValue[]> {
    return this.ledger.send({ PollMediaExport: { seq: BigInt(this.ledger.nextSeq()), handle } });
  }

  /** 🛑️ Cancels exactly the submitted media generation. */
  async cancelMediaExport(handle: MediaExportHandle): Promise<AppFrameValue[]> {
    return this.ledger.send({ CancelMediaExport: { seq: BigInt(this.ledger.nextSeq()), handle } });
  }

  /** 🧺️ Takes one bounded output chunk and observes terminal owner retirement. */
  async takeMediaExportChunk(handle: MediaExportHandle): Promise<AppFrameValue[]> {
    return this.ledger.send({ TakeMediaExportChunk: { seq: BigInt(this.ledger.nextSeq()), handle } });
  }
}
//#endregion 🔖️AppChannelClient
