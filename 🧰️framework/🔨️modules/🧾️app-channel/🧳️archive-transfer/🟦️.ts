/** 🧳️ App channel archive transfer: reading the document archive and driving a whole-document load or merge to its acknowledged end. */

import { decodePackValue } from "../../🎁️packs/🟦️.ts";
import type { AppFrameValue, DocumentArchiveLoadStatus, DocumentArchivePack } from "../../🪢️wire-contracts/🟦️.ts";
import { DOCUMENT_ARCHIVE_UNMERGEABLE_CODES, DocumentArchiveFaultError, type DocumentArchiveLoadAnswer, DocumentArchiveLoadHost, documentArchiveLoadReplyV1, type DocumentArchiveMergeV1, nextArchivePollTurnV1 } from "../🗃️archive-load-host/🟦️.ts";
import type { DocumentPackCache } from "../📦️document-pack-cache/🟦️.ts";
import { channelLabel, throwOnFault } from "../🧯️reply-faults/🟦️.ts";
import { decodeFaultFromWire, faultDisplayMessage } from "../📋️report-decoders/🟦️.ts";
import type { ReplyLedger } from "../📮️reply-ledger/🟦️.ts";

//#region 🔖️AppChannelClient
/** 🚚️ Moves whole documents in and out of one app instance, keeping the document pack cache in step with what it loaded or read. */
export class ArchiveTransfer {
  private readonly ledger: ReplyLedger;
  private readonly cache: DocumentPackCache;
  private readonly appId: string;

  constructor(ledger: ReplyLedger, cache: DocumentPackCache, appId: string) {
    this.ledger = ledger;
    this.cache = cache;
    this.appId = appId;
  }

  /** 🗃️ Restores one root envelope and its complete recursive owned-member closure atomically, stepped through
   * {@link DocumentArchiveLoadHost}: `progress` hears every polled status, `signal` cancels. A cancel before admission sends
   * nothing; after it the guest restores the previous document, and the load rejects with the signal's reason — or an
   * `AbortError` when the guest cancelled on its own (a person's Cancel in the history body). A cancel that lost the race
   * resolves, because the document did load. A refusal or a guest fault rejects with the guest's own fault text. Driven by
   * {@link drive}, like a merge. */
  async load(
    archive: DocumentArchivePack,
    signal?: AbortSignal,
    progress?: (status: DocumentArchiveLoadStatus) => void,
  ): Promise<void> {
    const root = { pack: Uint8Array.from(archive.parent_pack), spr: Uint8Array.from(archive.parent_spr) };
    await this.drive("loadDocumentArchive", new DocumentArchiveLoadHost(archive), [], signal, progress);
    this.cache.remember(root.pack, root.spr);
  }

  /** 🔀️ Merges a read-back of the document this program already shows (design §22.22), stepped through
   * {@link DocumentArchiveLoadHost.merging}: the archive's events join the program's log, nothing is replaced, an open history
   * edit and the program's viewed alternative stay. Resolves `merged` with the events it took and the events the program is
   * `ahead` of the archive by — the cached root pair is dropped when it took any — or `unmergeable` with the guest's code
   * ({@link DOCUMENT_ARCHIVE_UNMERGEABLE_CODES}), which touched nothing. Cancels, refusals and every other fault reject like
   * {@link load}'s. */
  async merge(
    archive: DocumentArchivePack,
    signal?: AbortSignal,
    progress?: (status: DocumentArchiveLoadStatus) => void,
  ): Promise<DocumentArchiveMergeV1> {
    const merge = DocumentArchiveLoadHost.merging(archive);
    const ended = await this.drive("mergeDocumentArchive", merge, DOCUMENT_ARCHIVE_UNMERGEABLE_CODES, signal, progress);
    if ("code" in ended) return { kind: "unmergeable", code: ended.code };
    if (ended.completed > 0) this.cache.forget();
    return { kind: "merged", merged: ended.completed, ahead: merge.ahead };
  }

  /** 🚶️ Drives one whole-document load or merge to its acknowledged end: resolves with the count its ready terminal completed,
   * or with the guest's fault code when it is one of `unmergeable`. A cancelled operation rejects with the signal's reason
   * (an `AbortError` when the guest cancelled on its own); a refusal and every other fault reject with a
   * {@link DocumentArchiveFaultError} carrying the guest's own fault, a reply that answers nothing with a plain error. */
  private async drive(
    name: string,
    host: DocumentArchiveLoadHost,
    unmergeable: readonly string[],
    signal?: AbortSignal,
    progress?: (status: DocumentArchiveLoadStatus) => void,
  ): Promise<{ readonly completed: number } | { readonly code: string }> {
    const label = channelLabel(name, this.appId);
    let completed = 0;
    for (;;) {
      if (signal?.aborted) host.requestCancel();
      const step = host.step(() => this.ledger.nextSeq());
      if (step.kind === "finished") {
        if (step.outcome.kind === "ready") return { completed };
        if (step.outcome.kind === "cancelled") throw signal?.reason ?? new DOMException(`${label}: cancelled`, "AbortError");
        const code = decodeFaultFromWire(step.outcome.fault, decodePackValue)?.code;
        if (typeof code === "string" && unmergeable.includes(code)) return { code };
        throw new DocumentArchiveFaultError(`${label}: ${faultDisplayMessage(step.outcome.fault, decodePackValue)}`, decodeFaultFromWire(step.outcome.fault, decodePackValue));
      }
      const seq = step.seq;
      const replyFrames = await this.ledger.send(step.command);
      const frame = documentArchiveLoadReplyV1(replyFrames, seq);
      const answer: DocumentArchiveLoadAnswer = frame ? host.answer(seq, frame) : { kind: "unanswered" };
      if (answer.kind === "refused") throw new DocumentArchiveFaultError(`${label}: ${faultDisplayMessage(answer.fault, decodePackValue)}`, decodeFaultFromWire(answer.fault, decodePackValue));
      if (answer.kind === "unanswered") throw new Error(`${label}: no answer to seq ${seq} of operation ${host.operation}`);
      if (answer.status) {
        completed = answer.status.completed;
        progress?.(answer.status);
      }
      if (answer.status ? answer.status.state === "pending" || answer.status.state === "running" : frame !== undefined && "Error" in frame) await nextArchivePollTurnV1();
    }
  }

  /** 📥️ Reads one generation-fenced root envelope and its complete recursive owned-member closure. */
  async read(): Promise<DocumentArchivePack> {
    const seq = this.ledger.nextSeq();
    const frames = await this.ledger.send({ ReadDocumentArchive: { seq } });
    const label = channelLabel("readDocumentArchive", this.appId);
    throwOnFault(frames, label);
    const response = frames.find(
      (frame): frame is Extract<AppFrameValue, { readonly DocumentArchive: unknown }> =>
        "DocumentArchive" in frame && frame.DocumentArchive.in_reply_to === seq,
    );
    if (!response) throw new Error(`${label}: missing DocumentArchive frame for seq ${seq}`);
    this.cache.remember(Uint8Array.from(response.DocumentArchive.archive.parent_pack), Uint8Array.from(response.DocumentArchive.archive.parent_spr));
    return {
      parent_pack: [...response.DocumentArchive.archive.parent_pack],
      parent_spr: [...response.DocumentArchive.archive.parent_spr],
      members: response.DocumentArchive.archive.members.map((entry) => ({
        ordinal: entry.ordinal,
        reference: { ...entry.reference },
        owner: { parent: { ...entry.owner.parent }, slot: entry.owner.slot, child_id: entry.owner.child_id },
        envelope_pack: [...entry.envelope_pack],
      })),
    };
  }
}
//#endregion 🔖️AppChannelClient
