/** 🔢️ App channel request sequence: the channel version admission, the handle port, local interaction query ownership, transaction replies and the request sequence. */

import type { Fault, PluginWasmHandle } from "@semio-tech/framework";
import type { LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryToken } from "@semio-tech/framework-replication";
import { localInteractionIdentityEquals } from "@semio-tech/framework-replication";
import { type AppCommandValue, type AppFrameValue } from "../../🪢️wire-contracts/🟦️.ts";

//#region 🔖️AppChannelClient
/** 📡️ TS twin of `protocol_channel::CHANNEL_VERSION`. Both constants are pinned against
 * `🔨️modules/🧑‍💻dev/🔖️channel-version/📌️pin/🔣️.json`, which owns the number — this one sat at 8 while Rust
 * had moved to 10, so the pin exists to make a half-done bump fail a test instead of a session.
 * Channel v12 retired the `Hello`/`Welcome` handshake this constant used to be carried on; since v21 every host admits a guest
 * against it through {@link admitGuestChannelVersion} before any frame. */
export const APP_CHANNEL_VERSION = 23;

/** 🤝️ Fault code of a guest compiled for another app-channel version than its host — TS twin of Rust `CHANNEL_MISMATCH_CODE`. */
export const CHANNEL_MISMATCH_CODE = "plugin.channel-mismatch";

/** 🤝️ Admits a guest compiled for the host's app-channel version (`reactor.channelVersion()`, read once right after instantiation);
 * any other version, older or newer, is refused before a single frame with the framework fault `plugin.channel-mismatch`, whose
 * notice names `{guest}` and `{host}`. `null` admits. TS twin of Rust `admit_guest_channel_version`, corpus
 * `🧫️fixtures/🧫️channel-handshake`. Self-contained: the materialized jco bridge embeds it verbatim. */
export function admitGuestChannelVersion(guest: number, host: number): Fault | null {
  if (guest === host) return null;
  return { origin: "framework", code: "plugin.channel-mismatch", severity: "error", message: `the guest speaks app channel ${guest}, the host app channel ${host}`, scope: {}, params: { guest: String(guest), host: String(host) }, retryable: false };
}

/** 📡️ The slice of {@link PluginWasmHandle} {@link AppChannelClient} needs — deliberately narrower
 * than the full handle so a caller can hand in any object shaped like it (a real handle, a test
 * double, ...) without importing the rest of `@semio-tech/framework`'s plugin-loading surface. */
export type AppChannelHandle = Pick<PluginWasmHandle, "enqueue" | "outcomes">;

//#region 🏠️LocalQueryOwnership
export type LocalInteractionClientQuery = {
  readonly requestId: string;
  readonly cancelSequence: number;
  readonly consume: (page: LocalInteractionPage) => Promise<void>;
  readonly resolve: (identity: LocalInteractionIdentity) => void;
  readonly reject: (error: unknown) => void;
  readonly signal: AbortSignal | undefined;
  readonly abort: () => void;
  token: LocalInteractionQueryToken | null;
  nextOrdinal: bigint;
  consuming: boolean;
  cancelled: boolean;
  terminalConsumed: boolean;
  failure: unknown;
};

export function sameLocalInteractionQuery(left: LocalInteractionQueryToken, right: LocalInteractionQueryToken): boolean {
  return left.requestId === right.requestId && left.queryGeneration === right.queryGeneration && localInteractionIdentityEquals(left.identity, right.identity);
}
export type AppChannelTransactionReply = { readonly kind: "prepared" | "committed" | "rolledBack"; readonly id: string };

export function appChannelTransactionReply(command: AppCommandValue): AppChannelTransactionReply | null {
  if ("transactionPrepare" in command) return { kind: "prepared", id: command.transactionPrepare.txn_id };
  if ("transactionCommit" in command) return { kind: "committed", id: command.transactionCommit.txn_id };
  if ("transactionRollback" in command) return { kind: "rolledBack", id: command.transactionRollback.txn_id };
  return null;
}

export function appChannelReplySequence(frame: AppFrameValue): number | null {
  const value = Object.values(frame)[0];
  if (!value || !("in_reply_to" in value)) return null;
  const sequence = value.in_reply_to;
  if (typeof sequence === "bigint") return sequence >= 0n && sequence <= BigInt(Number.MAX_SAFE_INTEGER) ? Number(sequence) : null;
  return typeof sequence === "number" ? sequence : null;
}

export function appChannelFrameBelongsTo(frame: AppFrameValue, sequence: number, transaction: AppChannelTransactionReply | null): boolean {
  const replySequence = appChannelReplySequence(frame);
  if (replySequence !== null) return replySequence === sequence;
  if ("DocumentIdentity" in frame || "MediaExportSubmitted" in frame || "MediaExportStatus" in frame || "MediaExportChunk" in frame) return false;
  if ("transactionPrepared" in frame) return transaction?.kind === "prepared" && transaction.id === frame.transactionPrepared.txn_id;
  if ("transactionCommitted" in frame) return transaction?.kind === "committed" && transaction.id === frame.transactionCommitted.txn_id;
  if ("transactionRolledBack" in frame) return transaction?.kind === "rolledBack" && transaction.id === frame.transactionRolledBack.txn_id;
  return true;
}
/** 🪪️ One checked handle-lifetime owner spans every client recreation; query admission reserves its cancellation receipt identity. */
export class AppChannelRequestSequence {
  private sequence: number;
  private request: bigint;
  constructor(sequence = 0, request = 0n) {
    if (!Number.isSafeInteger(sequence) || sequence < 0 || request < 0n || request > 0xffffffffffffffffn) throw new Error("app-channel.invalid-sequence-owner");
    this.sequence = sequence;
    this.request = request;
  }

  nextSequence(): number {
    if (this.sequence === Number.MAX_SAFE_INTEGER) throw new Error("app-channel.sequence-exhausted");
    return ++this.sequence;
  }

  nextQuery(): { readonly sequence: number; readonly cancelSequence: number; readonly request: string } {
    if (this.sequence > Number.MAX_SAFE_INTEGER - 2) throw new Error("app-channel.sequence-exhausted");
    if (this.request === 0xffffffffffffffffn) throw new Error("local-interaction.request-exhausted");
    const sequence = this.sequence + 1;
    this.sequence += 2;
    return { sequence, cancelSequence: this.sequence, request: (++this.request).toString() };
  }

  checkpoint(): { readonly sequence: number; readonly request: string } {
    return { sequence: this.sequence, request: this.request.toString() };
  }
}
//#endregion 🏠️LocalQueryOwnership
//#endregion 🔖️AppChannelClient
