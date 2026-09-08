import { actorInstanceLifetimeEquals, type ActorInstanceLifetime } from "../🚪️lifetime/🟦️.ts";

export const COLD_PAIR_MAXIMUM_PAGES = 64;
export const COLD_PAIR_FAULT_MAXIMUM_BYTES = 4 * 1024;

export type ColdDocumentPairFrontier = Readonly<{
  documentId: string;
  headEditOrdinal: bigint;
  headEditId: string;
  lastCommitSeq: bigint;
  chainSha256: Uint8Array;
}>;

export type ColdDocumentPairCursor = Readonly<{
  lifetime: ActorInstanceLifetime;
  transferGeneration: bigint;
  pageIndex: number;
  pageCount: number;
}>;

export type ColdDocumentPairApplied = Readonly<{
  lifetime: ActorInstanceLifetime;
  transferGeneration: bigint;
  baselineFrontier: ColdDocumentPairFrontier;
  aggregateSha256: Uint8Array;
}>;

export type ColdPairIngressStatus =
  | Readonly<{ kind: "idle" }>
  | Readonly<{ kind: "pageAccepted" | "backpressure" | "loading"; cursor: ColdDocumentPairCursor }>
  | Readonly<{ kind: "applied"; receipt: ColdDocumentPairApplied }>
  | Readonly<{ kind: "fault"; cursor: ColdDocumentPairCursor; fault: Uint8Array }>;

function exactRecord(value: unknown, keys: readonly string[], code: string): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(code);
  const record = value as Record<string, unknown>;
  const actual = Object.keys(record).sort();
  const expected = [...keys].sort();
  if (actual.length !== expected.length || actual.some((key, index) => key !== expected[index])) throw new Error(code);
  return record;
}

function unsigned64(value: unknown, nonzero: boolean, code: string): bigint {
  if (typeof value !== "bigint" || value < (nonzero ? 1n : 0n) || value > 0xffffffffffffffffn) throw new Error(code);
  return value;
}

function unsigned32(value: unknown, code: string): number {
  if (!Number.isInteger(value) || (value as number) < 0 || (value as number) > 0xffffffff) throw new Error(code);
  return value as number;
}

function exactBytes(value: unknown, length: number | null, maximum: number, code: string): Uint8Array {
  const bytes = value instanceof Uint8Array ? Uint8Array.from(value) : Array.isArray(value) && value.every((byte) => Number.isInteger(byte) && byte >= 0 && byte <= 255) ? Uint8Array.from(value) : null;
  if (!bytes || (length !== null && bytes.byteLength !== length) || bytes.byteLength < 1 || bytes.byteLength > maximum) throw new Error(code);
  return bytes;
}

function exactHash(value: unknown, code: string): Uint8Array {
  const hash = exactBytes(value, 32, 32, code);
  if (hash.every((byte) => byte === 0)) throw new Error(code);
  return hash;
}

function text(value: unknown, code: string): string {
  if (typeof value !== "string" || new TextEncoder().encode(value).byteLength < 1 || new TextEncoder().encode(value).byteLength > 512) throw new Error(code);
  return value;
}

export function parseColdDocumentPairLifetime(value: unknown): ActorInstanceLifetime {
  const record = exactRecord(value, ["activationGeneration", "instanceId", "guestLifetime"], "cold-pair.lifetime");
  return Object.freeze({
    activationGeneration: unsigned64(record.activationGeneration, true, "cold-pair.lifetime"),
    instanceId: unsigned32(record.instanceId, "cold-pair.lifetime"),
    guestLifetime: unsigned64(record.guestLifetime, true, "cold-pair.lifetime"),
  });
}

export function parseColdDocumentPairFrontier(value: unknown): ColdDocumentPairFrontier {
  const record = exactRecord(value, ["documentId", "headEditOrdinal", "headEditId", "lastCommitSeq", "chainSha256"], "cold-pair.frontier");
  const frontier = Object.freeze({
    documentId: text(record.documentId, "cold-pair.frontier"),
    headEditOrdinal: unsigned64(record.headEditOrdinal, false, "cold-pair.frontier"),
    headEditId: text(record.headEditId, "cold-pair.frontier"),
    lastCommitSeq: unsigned64(record.lastCommitSeq, false, "cold-pair.frontier"),
    chainSha256: exactHash(record.chainSha256, "cold-pair.frontier"),
  });
  if (frontier.lastCommitSeq > frontier.headEditOrdinal) throw new Error("cold-pair.frontier");
  return frontier;
}

export function parseColdDocumentPairCursor(value: unknown): ColdDocumentPairCursor {
  const record = exactRecord(value, ["lifetime", "transferGeneration", "pageIndex", "pageCount"], "cold-pair.cursor");
  const cursor = Object.freeze({
    lifetime: parseColdDocumentPairLifetime(record.lifetime),
    transferGeneration: unsigned64(record.transferGeneration, true, "cold-pair.cursor"),
    pageIndex: unsigned32(record.pageIndex, "cold-pair.cursor"),
    pageCount: unsigned32(record.pageCount, "cold-pair.cursor"),
  });
  if (cursor.pageCount < 1 || cursor.pageCount > COLD_PAIR_MAXIMUM_PAGES || cursor.pageIndex >= cursor.pageCount) throw new Error("cold-pair.cursor");
  return cursor;
}

export function parseColdDocumentPairApplied(value: unknown): ColdDocumentPairApplied {
  const record = exactRecord(value, ["lifetime", "transferGeneration", "baselineFrontier", "aggregateSha256"], "cold-pair.applied");
  return Object.freeze({
    lifetime: parseColdDocumentPairLifetime(record.lifetime),
    transferGeneration: unsigned64(record.transferGeneration, true, "cold-pair.applied"),
    baselineFrontier: parseColdDocumentPairFrontier(record.baselineFrontier),
    aggregateSha256: exactHash(record.aggregateSha256, "cold-pair.applied"),
  });
}

export function parseWitColdPairIngressStatus(value: unknown): ColdPairIngressStatus {
  const tag = value && typeof value === "object" ? Reflect.get(value, "tag") : undefined;
  const tagged = exactRecord(value, ["tag", ...(tag === "idle" ? [] : ["val"])], "cold-pair.status");
  if (tagged.tag === "idle") return Object.freeze({ kind: "idle" });
  if (tagged.tag === "page-accepted" || tagged.tag === "backpressure" || tagged.tag === "loading") {
    const kind = tagged.tag === "page-accepted" ? "pageAccepted" : tagged.tag;
    return Object.freeze({ kind, cursor: parseColdDocumentPairCursor(tagged.val) });
  }
  if (tagged.tag === "applied") return Object.freeze({ kind: "applied", receipt: parseColdDocumentPairApplied(tagged.val) });
  if (tagged.tag === "fault") {
    const fault = exactRecord(tagged.val, ["cursor", "fault"], "cold-pair.fault");
    return Object.freeze({ kind: "fault", cursor: parseColdDocumentPairCursor(fault.cursor), fault: exactBytes(fault.fault, null, COLD_PAIR_FAULT_MAXIMUM_BYTES, "cold-pair.fault") });
  }
  throw new Error("cold-pair.status");
}

export function coldDocumentPairCursorEquals(left: ColdDocumentPairCursor, right: ColdDocumentPairCursor): boolean {
  return actorInstanceLifetimeEquals(left.lifetime, right.lifetime) && left.transferGeneration === right.transferGeneration && left.pageIndex === right.pageIndex && left.pageCount === right.pageCount;
}

export function coldDocumentPairFrontierEquals(left: ColdDocumentPairFrontier, right: ColdDocumentPairFrontier): boolean {
  return (
    left.documentId === right.documentId &&
    left.headEditOrdinal === right.headEditOrdinal &&
    left.headEditId === right.headEditId &&
    left.lastCommitSeq === right.lastCommitSeq &&
    left.chainSha256.length === right.chainSha256.length &&
    left.chainSha256.every((byte, index) => byte === right.chainSha256[index])
  );
}

//#region 🧪️StatusCodecLaws
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️cold-pair-wit-status-codec-agrees-with-the-neutral-schema-and-rejects-ev/🟦️.ts");
  await registerTests1(import.meta.vitest, { parseWitColdPairIngressStatus }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️StatusCodecLaws
