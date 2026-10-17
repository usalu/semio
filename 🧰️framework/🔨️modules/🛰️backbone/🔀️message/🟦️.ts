/** 🔀️ Binary backbone message: the ordinal-tagged binary message codec and the document backbone message parsers. */

import type { ExactWireMutationEnvelope } from "@semio-tech/framework-replication";
import { decodeDocumentBackboneEnvelopeBatchExact, writeVarintU64 } from "@semio-tech/framework-replication";
import { PACK_TAG_LIST, PACK_TAG_STR, PACK_TAG_STR_INLINE, packBuildSymbols, packEncodeString, packPushBytes, packReadVarintBigInt } from "../../🎁️packs/🟦️.ts";
import { BACKBONE_GENESIS_MAXIMUM_BYTES, BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES } from "../../🪢️wire-contracts/🟦️.ts";

//#region 🔖️Backbone
//#region 🔀️BackboneMessage
export type BinaryBackboneMessage =
  | { readonly kind: "genesis"; readonly pack: Uint8Array }
  | { readonly kind: "mutations"; readonly envelopes: Uint8Array }
  | { readonly kind: "ack"; readonly opIds: readonly string[] }
  | { readonly kind: "retract"; readonly mutationIds: readonly string[] };

/** 🏷️ The Rust `BackboneMessage` variant ordinal of each kind this twin speaks; `Member` (3) is a Rust-only lane. */
const BACKBONE_MESSAGE_ORDINALS = { genesis: 0, mutations: 1, ack: 2, retract: 4 } as const;

/** 📤️ Canonical Store OpBinary: version, variant, symbol table and exact record fields. */
export function encodeBackboneMessage(message: BinaryBackboneMessage): Uint8Array {
  const maximum = message.kind === "genesis" ? BACKBONE_GENESIS_MAXIMUM_BYTES : BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES;
  const encoder = new TextEncoder();
  const checkedBytes = (value: Uint8Array) => {
    if (!(value instanceof Uint8Array) || value.length > maximum) throw new Error("backbone message: byte limit");
    return value;
  };
  const ids = message.kind === "ack" ? message.opIds : message.kind === "retract" ? message.mutationIds : null;
  if (ids !== null) {
    if (ids.length > maximum / 2) throw new Error("backbone message: item limit");
    for (const value of ids) {
      if (typeof value !== "string" || value.length > maximum || encoder.encode(value).length > maximum || new TextDecoder("utf-8", { fatal: true }).decode(encoder.encode(value)) !== value) throw new Error("backbone message: invalid string");
    }
  } else if (message.kind === "genesis") checkedBytes(message.pack);
  else if (message.kind === "mutations") checkedBytes(message.envelopes);
  else throw new Error("backbone message: unknown kind");
  const symbols = ids !== null ? packBuildSymbols(ids) : [];
  const symbolIndex = new Map(symbols.map((value, index) => [value, index] as const));
  const out: number[] = [1, BACKBONE_MESSAGE_ORDINALS[message.kind]];
  const bounded = () => { if (out.length > maximum) throw new Error("backbone message: byte limit"); };
  writeVarintU64(out, symbols.length);
  for (const symbol of symbols) {
    const bytes = encoder.encode(symbol);
    writeVarintU64(out, bytes.length);
    packPushBytes(out, bytes);
    bounded();
  }
  writeVarintU64(out, 1);
  const byteField = (id: number, bytes: Uint8Array) => {
    writeVarintU64(out, id);
    out.push(0x08);
    writeVarintU64(out, bytes.length);
    packPushBytes(out, bytes);
    bounded();
  };
  if (message.kind === "genesis") {
    byteField(0, message.pack);
  } else if (message.kind === "mutations") {
    byteField(0, message.envelopes);
  } else {
    out.push(0, PACK_TAG_LIST);
    writeVarintU64(out, ids!.length);
    for (const value of ids!) { packEncodeString(value, symbolIndex, out); bounded(); }
  }
  bounded();
  return new Uint8Array(out);
}

/** 📥️ Bounded exact record decoding, with canonical bytes checked before admission. */
export function decodeBackboneMessage(bytes: Uint8Array): BinaryBackboneMessage {
  if (!(bytes instanceof Uint8Array) || bytes.length === 0 || bytes.length > BACKBONE_GENESIS_MAXIMUM_BYTES) throw new Error("backbone message: byte limit");
  if (bytes[0] !== 1) throw new Error("backbone message: invalid format");
  const pos: [number] = [1];
  const natural = (maximum: number) => {
    const start = pos[0];
    const value = packReadVarintBigInt(bytes, pos);
    if (value > BigInt(maximum) || (pos[0] - start > 1 && bytes[pos[0] - 1] === 0)) throw new Error("backbone message: invalid count");
    return Number(value);
  };
  const tag = natural(BACKBONE_MESSAGE_ORDINALS.retract);
  if (tag === 3) throw new Error("backbone message: member lanes are Rust-only");
  if (tag !== 0 && bytes.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES) throw new Error("backbone message: hot byte limit");
  const take = (length: number) => {
    if (length > bytes.length - pos[0]) throw new Error("backbone message: truncated bytes");
    const value = bytes.subarray(pos[0], pos[0] + length);
    pos[0] += length;
    return value;
  };
  const expectTag = (expected: number) => {
    if (take(1)[0] !== expected) throw new Error("backbone message: unexpected tag");
  };
  const text = () => new TextDecoder("utf-8", { fatal: true }).decode(take(natural(bytes.length - pos[0])));
  const symbols: string[] = [];
  const symbolCount = natural(bytes.length - pos[0]);
  for (let index = 0; index < symbolCount; index++) symbols.push(text());
  if (natural(1) !== 1) throw new Error("backbone message: field count");
  const byteField = (id: number) => {
    if (natural(1) !== id) throw new Error("backbone message: field id");
    expectTag(0x08);
    return take(natural(bytes.length - pos[0])).slice();
  };
  let message: BinaryBackboneMessage;
  if (tag === 0) {
    message = { kind: "genesis", pack: byteField(0) };
  } else if (tag === 1) {
    message = { kind: "mutations", envelopes: byteField(0) };
  } else {
    if (natural(0) !== 0) throw new Error("backbone message: field id");
    expectTag(PACK_TAG_LIST);
    const count = natural(Math.floor((bytes.length - pos[0]) / 2));
    const opIds: string[] = [];
    for (let index = 0; index < count; index++) {
      const stringTag = take(1)[0];
      if (stringTag === PACK_TAG_STR_INLINE) opIds.push(text());
      else if (stringTag === PACK_TAG_STR) {
        const symbol = symbols[natural(symbols.length)];
        if (symbol === undefined) throw new Error("backbone message: symbol reference");
        opIds.push(symbol);
      } else throw new Error("backbone message: string tag");
    }
    message = tag === BACKBONE_MESSAGE_ORDINALS.retract ? { kind: "retract", mutationIds: opIds } : { kind: "ack", opIds };
  }
  if (pos[0] !== bytes.length) throw new Error("backbone message: trailing bytes");
  const canonical = encodeBackboneMessage(message);
  if (canonical.length !== bytes.length || canonical.some((value, index) => value !== bytes[index])) throw new Error("backbone message: noncanonical record");
  return message;
}

export type DocumentBackboneMessage = Readonly<{ message: Uint8Array; envelopes: readonly ExactWireMutationEnvelope[] }>;

/** 🪢 Admits one canonical hot mutation message and its exact bounded causal batch. */
export function parseDocumentBackboneMessage(message: Uint8Array): DocumentBackboneMessage {
  if (!(message instanceof Uint8Array) || message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES) throw new Error("document backbone: hot byte limit");
  const parsed = decodeBackboneMessage(message);
  if (parsed.kind !== "mutations") throw new Error("document backbone: mutations required");
  const envelopes = decodeDocumentBackboneEnvelopeBatchExact(parsed.envelopes);
  return { message: message.slice(), envelopes };
}

/** 🔙️ Admits one canonical hot message an actor hands its store: a mutation batch, exactly as
 * {@link parseDocumentBackboneMessage} admits it, or the retraction of history transitions the hub refused. */
export function parseInboundDocumentBackboneMessage(message: Uint8Array): Uint8Array {
  if (!(message instanceof Uint8Array) || message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES) throw new Error("document backbone: hot byte limit");
  if (decodeBackboneMessage(message).kind === "retract") return message.slice();
  return parseDocumentBackboneMessage(message).message;
}
//#endregion 🔀️BackboneMessage
//#endregion 🔖️Backbone
