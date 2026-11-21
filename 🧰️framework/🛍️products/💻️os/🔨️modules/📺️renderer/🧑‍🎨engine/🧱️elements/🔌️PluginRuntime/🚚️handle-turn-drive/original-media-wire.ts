/** 🪪️ Carries a single picked file under one guest-issued loan and actor command cursor. */
import { ACTOR_BYTE_PAGE_BYTES, createActorBytePage } from "@semio-tech/framework-actor/page";
import { SHARD_COMMAND_MAXIMUM_PAGES, type ShardCommandIngressPage, type ShardEventEnvelope } from "@semio-tech/framework/actor/shard-client/wire";
import { ORIGINAL_MEDIA_PAGE_BYTES, type OriginalMediaFrame, type OriginalMediaLoan } from "./original-media-sender.ts";

export type OriginalMediaRequest = Readonly<{
  loan: OriginalMediaLoan;
  kind: number;
  maximumFileBytes: number;
}>;

export type OriginalMediaExchange = Readonly<{
  current(): boolean;
  cross(frame: OriginalMediaFrame, bytes: Uint8Array | undefined, current: () => boolean): Promise<void>;
}>;

export type OriginalMediaAckIdentity = Readonly<{
  loan: OriginalMediaLoan;
  owner: bigint;
  seq: bigint;
  kind: number;
  phase: OriginalMediaFrame["kind"];
  pageIndex?: number;
  extent: number;
}>;

type Crossing = Readonly<{
  identity: OriginalMediaAckIdentity;
  events: readonly ShardEventEnvelope[];
  page?: ShardCommandIngressPage;
}>;

const boundedText = (text: unknown, maximum: number): text is string => {
  if (typeof text !== "string" || !text) return false;
  let count = 0;
  for (const _ of text) if (++count > maximum) return false;
  return true;
};

const exactHex = (value: unknown, width: number): value is string =>
  typeof value === "string" && value.length === width && /^[0-9a-f]+$/.test(value);

const record = (value: unknown): Record<string, unknown> | undefined =>
  value !== null && typeof value === "object" ? value as Record<string, unknown> : undefined;

export function originalMediaRequest(value: unknown, instance: number): OriginalMediaRequest {
  const request = record(value);
  const loan = record(request?.loan);
  const kind = request?.kind;
  const maximumFileBytes = request?.maximumFileBytes;
  if (!loan || typeof kind !== "number" || !Number.isInteger(kind) || kind < 1 || kind > 255 ||
      typeof maximumFileBytes !== "number" || !Number.isInteger(maximumFileBytes) || maximumFileBytes < 1 ||
      maximumFileBytes > ACTOR_BYTE_PAGE_BYTES * SHARD_COMMAND_MAXIMUM_PAGES ||
      !Number.isInteger(instance) || instance < 1 || instance > 0xffffffff || loan.instanceId !== instance ||
      !exactHex(loan.operationId, 16) || !exactHex(loan.generation, 16) ||
      !boundedText(loan.documentId, 256) || !exactHex(loan.baseRevision, 64)) {
    throw new Error("original-media.invalid-issued-request");
  }
  return Object.freeze({ kind, maximumFileBytes, loan: Object.freeze({
    operationId: loan.operationId, generation: loan.generation, instanceId: instance,
    documentId: loan.documentId, baseRevision: loan.baseRevision,
  }) });
}

function sameLoan(value: unknown, expected: OriginalMediaLoan): boolean {
  const loan = record(value);
  return !!loan && loan.operationId === expected.operationId && loan.generation === expected.generation &&
    loan.instanceId === expected.instanceId && loan.documentId === expected.documentId &&
    loan.baseRevision === expected.baseRevision;
}

export function originalMediaCrossing(
  request: OriginalMediaRequest, owner: bigint, seq: bigint, extent: number,
  frame: OriginalMediaFrame, bytes?: Uint8Array,
): Crossing {
  if (!sameLoan(frame.loan, request.loan) || extent < 0 || extent > request.maximumFileBytes ||
      (frame.kind === "begin" && (!extent || frame.byteLength !== extent))) throw new Error("original-media.changed-request");
  const { kind, loan } = request;
  const phase = frame.kind;
  const pageIndex = phase === "page" ? frame.index : undefined;
  const identity = { loan, owner, seq, kind, phase, pageIndex, extent };
  if (phase !== "page") {
    if (bytes !== undefined) throw new Error("original-media.control-body");
    const control = phase === "begin" ? { tag: phase, val: { name: frame.name, mediaType: frame.mediaType } } : { tag: phase };
    return { identity, events: [{ kind: "original-media-control", payload: { loan, owner, seq, kind, extent, control } }] };
  }
  if (ACTOR_BYTE_PAGE_BYTES !== ORIGINAL_MEDIA_PAGE_BYTES) throw new Error("original-media.page-contract");
  const pageCount = Math.ceil(extent / ACTOR_BYTE_PAGE_BYTES);
  if (!Number.isInteger(frame.index) || frame.index < 0 || frame.index >= pageCount ||
      !bytes || bytes.length !== frame.byteLength ||
      bytes.length !== Math.min(ACTOR_BYTE_PAGE_BYTES, extent - frame.index * ACTOR_BYTE_PAGE_BYTES)) {
    throw new Error("original-media.page-extent");
  }
  return { identity, events: [], page: {
    cursor: {
      owner, seq, kind, generation: BigInt("0x" + loan.generation), instance: loan.instanceId,
      commandIndex: 0, commandCount: 1, itemCount: 1, metadata: extent, pageIndex: frame.index, pageCount,
    },
    bytes, page: createActorBytePage(bytes),
  } };
}

export function readOriginalMediaAck(status: unknown, expected: OriginalMediaAckIdentity):
  "retry" | "pending" | "accepted" | "stale" | "refused" | undefined {
  const variant = record(status);
  if (variant?.tag !== "original-media-ack") return undefined;
  const ack = record(variant.val);
  if (!ack || !sameLoan(ack.loan, expected.loan) || ack.owner !== expected.owner || ack.seq !== expected.seq ||
      ack.kind !== expected.kind || ack.phase !== expected.phase || (ack.pageIndex ?? undefined) !== expected.pageIndex ||
      ack.extent !== expected.extent) throw new Error("original-media.foreign-ack");
  const result = ack.result;
  if (result === "retry" || result === "pending" || result === "accepted" || result === "stale" || result === "refused") return result;
  throw new Error("original-media.invalid-ack");
}

export function rawPageBackpressure(status: unknown, page: ShardCommandIngressPage | undefined): boolean {
  const variant = record(status);
  if (!page || variant?.tag !== "backpressure") return false;
  const cursor = record(variant.val);
  return Object.entries(page.cursor).every(([key, value]) => cursor?.[key] === value);
}
