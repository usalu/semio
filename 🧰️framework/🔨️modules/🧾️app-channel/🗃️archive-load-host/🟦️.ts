/** 🗃️ App channel document archive load host: the archive load steps, outcomes, fault error and the host that drives a load to its answer. */

import type { Fault } from "@semio-tech/framework";
import { type AppCommandValue, type AppFrameValue, type DocumentArchiveLoadStatus, type DocumentArchivePack } from "../../🪢️wire-contracts/🟦️.ts";

//#region 🔖️AppChannelClient
//#region 🗃️DocumentArchiveLoadHost
/** 🏁️ How one host-driven whole-document load ended (`📓️api-stepped-document-load.md` §2.3): twin of the Rust
 * `DocumentArchiveLoadOutcome`. `cancelled` and `fault` both left the previous document exactly as it was. */
export type DocumentArchiveLoadOutcome = { readonly kind: "ready" } | { readonly kind: "cancelled" } | { readonly kind: "fault"; readonly fault: readonly number[] };

/** 🧭️ The next command to send, stamped with the sequence the host minted for it, or the outcome once the load is over. */
export type DocumentArchiveLoadStep = { readonly kind: "send"; readonly seq: number; readonly command: AppCommandValue } | { readonly kind: "finished"; readonly outcome: DocumentArchiveLoadOutcome };

/** 📬️ What one answer moved the load to: the polled status (`null` for every other command), or what it cannot go on from —
 * the guest's encoded refusal, or a frame that answers another command. */
export type DocumentArchiveLoadAnswer = { readonly kind: "status"; readonly status: DocumentArchiveLoadStatus | null } | { readonly kind: "refused"; readonly fault: readonly number[] } | { readonly kind: "unanswered" };

/** 🔀️ How one read-back merge ended (design §22.22): `merged` events of the archive joined the program's log and the program
 * holds `ahead` events the archive lacks — or the program cannot merge the archive into the document it shows and `code` is
 * its own refusal ({@link DOCUMENT_ARCHIVE_UNMERGEABLE_CODES}); nothing was touched, and loading the archive is the way on. */
export type DocumentArchiveMergeV1 = { readonly kind: "merged"; readonly merged: number; readonly ahead: number } | { readonly kind: "unmergeable"; readonly code: string };

/** 🧯️ A whole-document load or merge the program refused or faulted. `fault` is the program's own fault when its bytes decode
 * — its code names the notice a person is told — and `message` keeps the engineering text for a log, never for a person. */
export class DocumentArchiveFaultError extends Error {
  readonly fault: Fault | null;

  constructor(message: string, fault: Fault | null) {
    super(message);
    this.name = "DocumentArchiveFaultError";
    this.fault = fault;
  }
}

/** 🚫️ The guest's refusals of a read-back merge that leave a load as the way on: the archive is another document's, or its
 * members differ from the live ones (twins of `DOCUMENT_LOAD_OTHER_DOCUMENT_CODE` and `DOCUMENT_LOAD_MEMBERS_DIFFER_CODE`,
 * `🔨️modules/🔌️plugin/🦀️.rs`). */
export const DOCUMENT_ARCHIVE_UNMERGEABLE_CODES: readonly string[] = ["plugin.document-load.other-document", "plugin.document-load.members-differ"];

type DocumentArchiveLoadSent = "admit" | "poll" | "cancel" | "acknowledge";

/** 🗃️ The host half of the stepped, ACK-owned whole-document load: the TS twin of the Rust `DocumentArchiveLoadHost`
 * (`🔨️modules/📡️spr/🧵️channel/🦀️.rs`), a transport-free state machine both replay against one corpus
 * (`🔨️modules/📡️spr/🧵️channel/🧫️fixtures/🧫️document-archive-load-host/🔣️.json`). The admission's own sequence is the operation, a
 * host's sequence is minted only for a command that is sent, and steps and answers alternate. A cancel before admission sends nothing; after admission it is sent once and polling goes on
 * until the guest's terminal status says whether the previous document was restored (a refused cancel lost the race). A
 * cancelled terminal whose acknowledgement is refused still owns retained input, so it is polled again. A read-back of the
 * document the program already shows is a merge ({@link merging}): the same exchange under its own admission command. */
export class DocumentArchiveLoadHost {
  private archive: DocumentArchivePack | null;
  private admitted: number | null = null;
  private sent: { readonly seq: number; readonly kind: DocumentArchiveLoadSent } | null = null;
  private cancelRequested = false;
  private cancelSent = false;
  private terminal: DocumentArchiveLoadStatus | null = null;
  private outcome: DocumentArchiveLoadOutcome | null = null;
  private merge = false;
  private aheadOfArchive = 0;

  constructor(archive: DocumentArchivePack) {
    this.archive = archive;
  }

  /** 🔀️ A read-back merge of the document the program already shows (design §22.22): admitted with `MergeDocumentArchive`,
   * then polled, cancelled and acknowledged exactly like a load. Its events join the program's log and nothing is
   * replaced; {@link ahead} answers what the program then holds that the archive lacks. */
  static merging(archive: DocumentArchivePack): DocumentArchiveLoadHost {
    const host = new DocumentArchiveLoadHost(archive);
    host.merge = true;
    return host;
  }

  /** 🎞️ A load the guest admitted itself under `operation`, the host sequence of the command that started it (a `MediaIn` of a
   * whole document of the app's own schema answers its pending status): driven from its first poll like an admission this host sent. */
  static admitted(operation: number): DocumentArchiveLoadHost {
    const host = new DocumentArchiveLoadHost({ parent_pack: [], parent_spr: [], members: [] });
    host.archive = null;
    host.admitted = operation;
    return host;
  }

  /** 🔢️ The admitted operation, once the admission was sent. */
  get operation(): number | null {
    return this.admitted;
  }

  /** ⏭️ The events the program holds that the archive lacks, as its acknowledged ready terminal counted them: what a writer
   * persists again for. 0 for a replacement and before the operation ended ready. */
  get ahead(): number {
    return this.aheadOfArchive;
  }

  /** 🛑️ Asks for the load to stop; idempotent, and a no-op once a terminal status is known. */
  requestCancel(): void {
    this.cancelRequested = true;
    if (this.admitted === null && this.outcome === null) {
      this.archive = null;
      this.outcome = { kind: "cancelled" };
    }
  }

  /** ➡️ The next command, stamped with a sequence from `nextSeq`, or the outcome once the load is over. */
  step(nextSeq: () => number): DocumentArchiveLoadStep {
    if (this.outcome) return { kind: "finished", outcome: this.outcome };
    const seq = nextSeq();
    const operation = this.admitted;
    const archive = this.archive ?? { parent_pack: [], parent_spr: [], members: [] };
    const [kind, command]: readonly [DocumentArchiveLoadSent, AppCommandValue] =
      operation === null ? ["admit", this.merge ? { MergeDocumentArchive: { seq, archive } } : { LoadDocumentArchive: { seq, archive } }]
      : this.terminal ? ["acknowledge", { AcknowledgeDocumentArchiveLoad: { seq, operation } }]
      : this.cancelRequested && !this.cancelSent ? ["cancel", { CancelDocumentArchiveLoad: { seq, operation } }]
      : ["poll", { PollDocumentArchiveLoad: { seq, operation } }];
    if (kind === "admit") {
      this.admitted = seq;
      this.archive = null;
    }
    this.sent = { seq, kind };
    return { kind: "send", seq, command };
  }

  /** 📬️ Takes the guest's answer to the command sent under `seq`. */
  answer(seq: number, frame: AppFrameValue): DocumentArchiveLoadAnswer {
    const sent = this.sent;
    const operation = this.admitted;
    this.sent = null;
    if (sent === null || sent.seq !== seq || operation === null) return { kind: "unanswered" };
    if ("Done" in frame && frame.Done.in_reply_to === seq && sent.kind !== "poll") {
      if (sent.kind === "cancel") this.cancelSent = true;
      if (sent.kind === "acknowledge") {
        const terminal = this.terminal;
        if (!terminal) return { kind: "unanswered" };
        this.terminal = null;
        if (terminal.state === "ready") this.aheadOfArchive = terminal.ahead;
        this.outcome = terminal.state === "ready" ? { kind: "ready" } : terminal.state === "cancelled" ? { kind: "cancelled" } : { kind: "fault", fault: terminal.fault };
      }
      return { kind: "status", status: null };
    }
    if ("DocumentArchiveLoad" in frame && sent.kind === "poll" && frame.DocumentArchiveLoad.in_reply_to === seq && frame.DocumentArchiveLoad.status.operation === operation) {
      const status = frame.DocumentArchiveLoad.status;
      if (status.state === "ready" || status.state === "cancelled" || status.state === "fault") this.terminal = status;
      return { kind: "status", status };
    }
    if ("Error" in frame && frame.Error.in_reply_to === seq) {
      if (sent.kind === "cancel") {
        this.cancelSent = true;
        return { kind: "status", status: null };
      }
      if (sent.kind === "acknowledge" && this.terminal?.state === "cancelled") {
        this.terminal = null;
        return { kind: "status", status: null };
      }
      return { kind: "refused", fault: frame.Error.fault };
    }
    return { kind: "unanswered" };
  }
}

/** 🎯️ The one frame of a command's reply that answers `seq`: its refusal first, else its `Done` or archive-load status. */
export function documentArchiveLoadReplyV1(frames: readonly AppFrameValue[], seq: number): AppFrameValue | undefined {
  return frames.find((frame) => "Error" in frame && frame.Error.in_reply_to === seq)
    ?? frames.find((frame) => ("Done" in frame && frame.Done.in_reply_to === seq) || ("DocumentArchiveLoad" in frame && frame.DocumentArchiveLoad.in_reply_to === seq));
}

//#endregion 🗃️DocumentArchiveLoadHost

/** ⏳️ Resolves on the next macrotask — where a document-archive load waits between polls, so the page's timers, input
 * and rendering (the Tasks window's Cancel) run while the guest works; a microtask-only re-poll starved them. */
export function nextArchivePollTurnV1(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 0));
}
//#endregion 🔖️AppChannelClient
