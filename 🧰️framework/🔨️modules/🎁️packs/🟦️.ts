/** 🎁️ The pack value codec: tags and integer carriers, the symbol-table container, the public encode and decode API with base64 and exact-JSON projections, invocation-result decoding, and the scene and mesh pack decoders.
 *
 * 🧱️ Framework module: the os root re-exports the public names its clients use, and plugins build on it through the SDK entry `@semio-tech/framework/packs`.
 */

import type {
  ArtifactDiff,
  InverseMutation,
  KernelMutation,
  PluginViewState,
  UndoGroup,
  UndoPolicy
} from "@semio-tech/framework";
import { viewContextWithIntegerCarriers } from "../🛂️manifest/👁️view-state-and-public-invocation/🟦️.ts";
import type { MutationEnvelope } from "../📡️replication/📦️packages/🟦️typescript/🟦️.ts";
import {
  decodeCausalEnvelopeBatch,
  encodeCausalEnvelopeBatch,
  readF64,
  readVarintU64,
  writeF64,
  writeVarintU64
} from "../📡️replication/📦️packages/🟦️typescript/🟦️.ts";
import { GUEST_HOST_ANSWER_CEILING_BYTES } from "../⏱️trace/🧮️memory/🟦️.ts";

export const replicationPackCodec = { encode: encodePackValue, decode: decodePackValue };

/**
 * 📦️ TS mirror of `store::pack_rt::encode_wire_value`/`decode_wire_value`
 * (`framework/product/os/module/store/rs/lib.rs`) — the schema-less `serde_json::Value` bridge
 * for per-message wire payloads (UI tree diffs, host effects, events, manifests), NOT whole
 * documents (that's `encode_json_value`/`decode_json_value`'s job, backed by
 * `pack::encode_document`'s full `.spk` container — 32-byte header, deflate-compressed segments,
 * an 84-byte footer with a BLAKE3 content hash, 200+ bytes of overhead per value, and
 * deflate-compressed bytes that are NOT portable byte-for-byte across a spec-compliant TS
 * deflate implementation). `encode_wire_value` instead calls `pack::encode_record_body` — the
 * container-less twin used by `dsl::op_rt::encode_op` — for a `symbol_count varint, (len varint,
 * utf8)*, record fields` grammar with no header, segments, manifest, or footer. Every JSON value
 * is still wrapped as a single `Shape::Value` field (id 1) of the same synthetic one-field
 * `json_bridge_spec()` record; only the outer framing changed. Fully deterministic and
 * byte-exact against real Rust output in both directions (no compression involved, unlike the
 * old container-backed encoding this replaces).
 */

//#region 🔖️PackValueCodec
//#region 🔖️PackContainerPrimitives
/** 🌱️ `store::pack_rt`'s synthetic single-field record spec (`{ id: 1, key: "value", shape:
 * Shape::Value }`) every JSON value is wrapped in before hitting `encode_record_body`. */
const JSON_BRIDGE_FIELD_ID = 1;

/** 🌱️ `pack_value`'s wire tags actually reachable from a `DslValue` (`encode_dsl_value`/
 * `decode_dsl_value`, `pack/value/rs/lib.rs`'s `🔖️Tags` region) — the subset `PackValueCodec`
 * needs (no `Bytes64`/`Enum`/... — a dynamic value never produces those). `TAG_INT`/`TAG_UINT`
 * carry a whole 64-bit integer exactly; a JS `number` can only ever be `TAG_F64`. */
const PACK_TAG_FALSE = 0x01;
const PACK_TAG_TRUE = 0x02;
const PACK_TAG_INT = 0x03;
const PACK_TAG_UINT = 0x04;
const PACK_TAG_F64 = 0x05;
export const PACK_TAG_STR = 0x06;
export const PACK_TAG_STR_INLINE = 0x07;
export const PACK_TAG_LIST = 0x0c;
const PACK_TAG_MAP = 0x10;
const PACK_TAG_VALUE = 0x11;
const PACK_TAG_NULL = 0x12;

export function packPushBytes(out: number[], bytes: Uint8Array): void {
  for (let index = 0; index < bytes.length; index++) out.push(bytes[index]!);
}
/** 🔤️ Byte-lexicographic string comparison — the TS twin of Rust `str`'s `Ord` (which compares
 * the UTF-8 byte sequence), used everywhere `pack_value` sorts by `.as_bytes()` (symbol table,
 * `DslValue::Object` keys). UTF-8 byte order IS code point order, and code point order equals UTF-16
 * code unit order at the first differing unit unless that unit is a surrogate — so the units are
 * compared directly and only a surrogate at the first difference (outside the BMP, or a lone surrogate
 * that `TextEncoder` writes as U+FFFD) takes the exact encode-and-compare path. A shared prefix of
 * units is a shared prefix of bytes, so the shorter string sorts first.
 *
 * 🐛️ The comparator used to allocate a `TextEncoder` and encode BOTH strings on every comparison:
 * encoding the plugin manifests at boot spent ~0.5 s in it alone (ticket 26/09/23 F2). */
export function packByteCompare(a: string, b: string): number {
  if (a === b) return 0;
  const length = Math.min(a.length, b.length);
  for (let index = 0; index < length; index++) {
    const left = a.charCodeAt(index);
    const right = b.charCodeAt(index);
    if (left === right) continue;
    if ((left & 0xf800) === 0xd800 || (right & 0xf800) === 0xd800) return packEncodedByteCompare(a, b);
    return left - right;
  }
  return a.length - b.length;
}

function packEncodedByteCompare(a: string, b: string): number {
  const encoder = new TextEncoder();
  const ab = encoder.encode(a);
  const bb = encoder.encode(b);
  const len = Math.min(ab.length, bb.length);
  for (let index = 0; index < len; index++) {
    const diff = ab[index]! - bb[index]!;
    if (diff !== 0) return diff;
  }
  return ab.length - bb.length;
}
//#endregion 🔖️PackContainerPrimitives

//#region 🔖️PackInteger
/** 🔢️ A whole 64-bit dynamic integer, carrying the writer's `int`/`uint` form as well as its
 * magnitude. A bare `bigint` would lose the form — Rust `Int(7)` and `UInt(7)` are equal under
 * `Number::PartialEq` yet emit different tags and therefore different canonical bytes and hashes —
 * and a JS `number` would lose magnitude past 2^53. Construct one only through {@link packInt} or
 * {@link packUInt}: the codec accepts nothing else as an integer. */
export type PackInteger = Readonly<{ readonly kind: "int" | "uint"; readonly value: bigint }>;

/** 🌱️ Everything the dynamic pack grammar can carry. `number` is always `TAG_F64`; an exact
 * integer is always a {@link PackInteger}. */
export type PackValue = null | boolean | number | string | PackInteger | readonly PackValue[] | { readonly [key: string]: PackValue };

const PACK_U64_MAX = (1n << 64n) - 1n;
const PACK_I64_MIN = -(1n << 63n);
const PACK_I64_MAX = (1n << 63n) - 1n;
/** 🔒️ Module-private mint. Membership, not shape, is what makes a carrier an integer — an
 * ordinary dynamic map `{ kind, value }` is otherwise indistinguishable from one. */
const packIntegerMint = new WeakSet<object>();

function packMintInteger(kind: "int" | "uint", value: bigint): PackInteger {
  const carrier = Object.freeze({ kind, value });
  packIntegerMint.add(carrier);
  return carrier;
}

/** 🔢️ Mints a signed 64-bit dynamic integer (`-2^63 <= value <= 2^63-1`). */
export function packInt(value: bigint): PackInteger {
  if (typeof value !== "bigint" || value < PACK_I64_MIN || value > PACK_I64_MAX) throw new Error(`packInt: ${String(value)} is outside the exact i64 range`);
  return packMintInteger("int", value);
}

/** 🔢️ Mints an unsigned 64-bit dynamic integer (`0 <= value <= 2^64-1`). */
export function packUInt(value: bigint): PackInteger {
  if (typeof value !== "bigint" || value < 0n || value > PACK_U64_MAX) throw new Error(`packUInt: ${String(value)} is outside the exact u64 range`);
  return packMintInteger("uint", value);
}

/** 🔎️ True only for a carrier this module minted — never for a look-alike literal. */
export function isPackInteger(value: unknown): value is PackInteger {
  return typeof value === "object" && value !== null && packIntegerMint.has(value);
}

/** 🗺️ True only for the MAP arm of {@link PackValue} — not an array, not a minted integer
 * carrier. The one narrowing a reader of a decoded pack needs before it may index by key. */
export function isPackMap(value: PackValue): value is { readonly [key: string]: PackValue } {
  return typeof value === "object" && value !== null && !Array.isArray(value) && !isPackInteger(value);
}

/** 🧬️ Structural-clone replacement. The mint is a `WeakSet`, so a raw `structuredClone`
 * silently degrades every integer carrier into an ambiguous `{ kind, value }` map; this rebuilds
 * them, and rejects the look-alikes a clone would have produced. */
export function clonePackValue(value: PackValue): PackValue {
  if (isPackInteger(value)) return value.kind === "uint" ? packUInt(value.value) : packInt(value.value);
  if (Array.isArray(value)) return (value as readonly PackValue[]).map(clonePackValue);
  if (value !== null && typeof value === "object") {
    packRejectIntegerLookAlike(value);
    return Object.fromEntries(Object.entries(value as Record<string, PackValue>).map(([key, entry]) => [key, clonePackValue(entry)]));
  }
  if (typeof value === "bigint") throw new Error("clonePackValue: a bare bigint is not a PackValue — mint it with packInt/packUInt");
  return value;
}

function packRejectIntegerLookAlike(value: object): void {
  if (typeof (value as { value?: unknown }).value === "bigint") throw new Error("PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint");
}

/** ✍️ Canonical unsigned LEB128 over the whole `u64` range — Pack-local on purpose, because
 * `writeVarintU64` computes in `number` for bounded framing/counts and cannot reach 2^64. */
function packWriteVarintBigInt(out: number[], value: bigint): void {
  let remaining = value;
  for (;;) {
    const byte = Number(remaining & 0x7fn);
    remaining >>= 7n;
    if (remaining === 0n) {
      out.push(byte);
      return;
    }
    out.push(byte | 0x80);
  }
}

/** 📖️ Inverse of {@link packWriteVarintBigInt} with `codec::read_varint_u64`'s exact ten-byte
 * overflow rule: the tenth byte may not continue and may not carry a payload above 1. */
export function packReadVarintBigInt(bytes: Uint8Array, pos: [number]): bigint {
  let result = 0n;
  for (let index = 0; index < 10; index++) {
    const byte = bytes[pos[0]];
    if (byte === undefined) throw new Error("decodePackValue: truncated integer varint");
    pos[0] += 1;
    const payload = byte & 0x7f;
    const more = (byte & 0x80) !== 0;
    if (index === 9 && (more || payload > 1)) throw new Error("decodePackValue: overlong integer varint (exceeds 10 bytes / 64 bits)");
    result |= BigInt(payload) << BigInt(index * 7);
    if (!more) return result;
  }
  throw new Error("decodePackValue: overlong integer varint (exceeds 10 bytes)");
}

const packZigzagEncode = (value: bigint): bigint => (value < 0n ? (-value << 1n) - 1n : value << 1n);
const packZigzagDecode = (raw: bigint): bigint => (raw >> 1n) ^ -(raw & 1n);
//#endregion 🔖️PackInteger

//#region 🔖️JsonValueTags
/** 🔎️ `pack_value::build_symbols`, specialized to a JSON-bridge document (one `Shape::Value`
 * field — no `TableSoA`/`Statements` forced-symbol cases apply). Walks `value` counting only
 * STRING LEAVES (object/array keys are never counted — `pack_value::walk_dsl_value_for_symbols`'s
 * `DslValue::Object` case only walks entry VALUES); a string is interned (added to the symbol
 * table) iff its UTF-8 byte length is `<= 128` or it occurs `>= 2` times, matching `pack_value`'s
 * rule exactly (note: `.len()` on the Rust side is UTF-8 BYTE length, not char count). */
function packCollectStrings(value: unknown, counts: Map<string, number>): void {
  if (typeof value === "string") {
    counts.set(value, (counts.get(value) ?? 0) + 1);
    return;
  }
  if (isPackInteger(value)) return;
  if (Array.isArray(value)) {
    for (const item of value as readonly PackValue[]) packCollectStrings(item, counts);
    return;
  }
  if (value !== null && typeof value === "object") {
    for (const item of Object.values(value as Record<string, PackValue>)) packCollectStrings(item, counts);
  }
}
export function packBuildSymbols(value: unknown): string[] {
  const counts = new Map<string, number>();
  packCollectStrings(value, counts);
  const encoder = new TextEncoder();
  const symbols: string[] = [];
  for (const [text, count] of counts) if (encoder.encode(text).length <= 128 || count >= 2) symbols.push(text);
  symbols.sort(packByteCompare);
  return symbols;
}

/** ✍️ `pack_value::encode_string`: `TAG_STR + symref varint` if interned, else
 * `TAG_STR_INLINE + len varint + utf8 bytes`. */
export function packEncodeString(text: string, symbolIndex: ReadonlyMap<string, number>, out: number[]): void {
  const index = symbolIndex.get(text);
  if (index !== undefined) {
    out.push(PACK_TAG_STR);
    writeVarintU64(out, index);
    return;
  }
  packEncodeStringInline(text, out);
}
/** ✍️ `pack_value::encode_string_inline` — forced, e.g. every `DslValue::Object` key. */
function packEncodeStringInline(text: string, out: number[]): void {
  const bytes = new TextEncoder().encode(text);
  out.push(PACK_TAG_STR_INLINE);
  writeVarintU64(out, bytes.length);
  packPushBytes(out, bytes);
}
/** 📖️ `pack_value::decode_string` — reads its OWN leading tag (`TAG_STR`/`TAG_STR_INLINE`), used
 * both for `Map`/object keys and inside {@link packDecodeValue}'s `TAG_STR` case. */
function packDecodeString(bytes: Uint8Array, symbols: readonly string[], pos: [number]): string {
  const tag = bytes[pos[0]]!;
  pos[0] += 1;
  if (tag === PACK_TAG_STR) {
    const index = readVarintU64(bytes, pos);
    const symbol = symbols[index];
    if (symbol === undefined) throw new Error(`decodePackValue: symref ${index} out of range for table of ${symbols.length}`);
    return symbol;
  }
  if (tag === PACK_TAG_STR_INLINE) {
    const len = readVarintU64(bytes, pos);
    const text = new TextDecoder().decode(bytes.subarray(pos[0], pos[0] + len));
    pos[0] += len;
    return text;
  }
  throw new Error(`decodePackValue: expected a string tag, found 0x${tag.toString(16)}`);
}

/** ✍️ `pack_value::encode_dsl_value` — the tag-prefixed encoding one dynamic value recurses
 * through. A JS `number` is `TAG_F64` because that is exactly what it is; an exact 64-bit integer
 * arrives as a {@link PackInteger} and writes `TAG_UINT`/`TAG_INT` plus a canonical
 * unsigned/zig-zag LEB128. `-0` keeps its sign bit, byte-for-byte with Rust's `normalize_f64`
 * (which only folds `NaN`). Object entries sort by key BYTES with keys always forced inline,
 * never a symref. */
function packEncodeValue(value: unknown, symbolIndex: ReadonlyMap<string, number>, out: number[]): void {
  if (value === null || value === undefined) {
    out.push(PACK_TAG_NULL);
    return;
  }
  if (typeof value === "boolean") {
    out.push(value ? PACK_TAG_TRUE : PACK_TAG_FALSE);
    return;
  }
  if (isPackInteger(value)) {
    out.push(value.kind === "uint" ? PACK_TAG_UINT : PACK_TAG_INT);
    packWriteVarintBigInt(out, value.kind === "uint" ? value.value : packZigzagEncode(value.value));
    return;
  }
  if (typeof value === "number") {
    out.push(PACK_TAG_F64);
    writeF64(out, value);
    return;
  }
  if (typeof value === "string") {
    packEncodeString(value, symbolIndex, out);
    return;
  }
  if (Array.isArray(value)) {
    out.push(PACK_TAG_LIST);
    writeVarintU64(out, value.length);
    for (const item of value as readonly PackValue[]) packEncodeValue(item, symbolIndex, out);
    return;
  }
  if (typeof value === "object") {
    packRejectIntegerLookAlike(value);
    out.push(PACK_TAG_MAP);
    const entries = Object.entries(value as Record<string, PackValue>).sort((a, b) => packByteCompare(a[0], b[0]));
    writeVarintU64(out, entries.length);
    for (const [key, entryValue] of entries) {
      packEncodeStringInline(key, out);
      packEncodeValue(entryValue, symbolIndex, out);
    }
    return;
  }
  throw new Error(`encodePackValue: unsupported dynamic value of type ${typeof value}`);
}
/** 📖️ Inverse of {@link packEncodeValue} — the TS twin of `pack_value::decode_dsl_value`. */
function packDecodeValue(bytes: Uint8Array, symbols: readonly string[], pos: [number]): PackValue {
  const tag = bytes[pos[0]]!;
  pos[0] += 1;
  switch (tag) {
    case PACK_TAG_NULL:
      return null;
    case PACK_TAG_FALSE:
      return false;
    case PACK_TAG_TRUE:
      return true;
    case PACK_TAG_UINT:
      return packUInt(packReadVarintBigInt(bytes, pos));
    case PACK_TAG_INT:
      return packInt(packZigzagDecode(packReadVarintBigInt(bytes, pos)));
    case PACK_TAG_F64:
      return readF64(bytes, pos);
    case PACK_TAG_STR: {
      const index = readVarintU64(bytes, pos);
      const symbol = symbols[index];
      if (symbol === undefined) throw new Error(`decodePackValue: symref ${index} out of range for table of ${symbols.length}`);
      return symbol;
    }
    case PACK_TAG_STR_INLINE: {
      const len = readVarintU64(bytes, pos);
      const text = new TextDecoder().decode(bytes.subarray(pos[0], pos[0] + len));
      pos[0] += len;
      return text;
    }
    case PACK_TAG_LIST: {
      const count = readVarintU64(bytes, pos);
      const items: PackValue[] = [];
      for (let i = 0; i < count; i++) items.push(packDecodeValue(bytes, symbols, pos));
      return items;
    }
    case PACK_TAG_MAP: {
      const count = readVarintU64(bytes, pos);
      const entries: Record<string, PackValue> = {};
      for (let i = 0; i < count; i++) {
        const key = packDecodeString(bytes, symbols, pos);
        entries[key] = packDecodeValue(bytes, symbols, pos);
      }
      return entries;
    }
    default:
      throw new Error(`decodePackValue: unrecognized dsl value tag 0x${tag.toString(16)}`);
  }
}
//#endregion 🔖️JsonValueTags

//#region 🔖️PublicApi
/** 📤️ TS twin of `store::pack_rt::encode_wire_value` — encodes any JSON-shaped `value` (null,
 * bool, number, string, array, nested object) as an `encode_record_body` payload: `symbol_count
 * varint, (len varint, utf8 bytes)*` (the symbol table, written inline — no `Symbols` segment)
 * followed directly by the synthetic one-field record's fields (`field_count=1, field_id=1,
 * TAG_VALUE, <value>`, matching `pack_value::encode_record_fields`'s grammar exactly). No header,
 * segments, manifest, or footer — byte-exact against real Rust output (verified against the
 * `pack_wire_value_fixture_corpus_hex_dump` fixture corpus, `store/rs/lib.rs`'s
 * `🔖️PackValueFixtures` region). Input is runtime-checked; decoded {@link PackValue} retains
 * the closed grammar without requiring callers' typed objects to declare an index signature. */
export function encodePackValue(value: unknown): Uint8Array<ArrayBuffer> {
  const symbols = packBuildSymbols(value);
  const symbolIndex = new Map(symbols.map((symbol, index) => [symbol, index] as const));
  const encoder = new TextEncoder();

  const out: number[] = [];
  writeVarintU64(out, symbols.length);
  for (const symbol of symbols) {
    const bytes = encoder.encode(symbol);
    writeVarintU64(out, bytes.length);
    packPushBytes(out, bytes);
  }
  writeVarintU64(out, 1);
  writeVarintU64(out, JSON_BRIDGE_FIELD_ID);
  out.push(PACK_TAG_VALUE);
  packEncodeValue(value, symbolIndex, out);
  return new Uint8Array(out);
}

/** 🪟️ ONE host→guest view context as the value its pack encoding is taken from — every
 * exact-integer field minted as a {@link PackInteger} instead of the JavaScript `number` the host
 * builds it with.
 *
 * A JS `number` is an IEEE double and nothing else, so {@link encodePackValue} writes every one of
 * them as `TAG_F64`. The guest decodes `ToolRunTraceCursor { run: u64, … }` through `FromValue`,
 * whose unsigned arm refuses a `Float` by design (`🌱️value/🔁️codec/🦀️.rs`) — so the first
 * integer-typed view-state field the shell ever carried failed to decode and took EVERY guest turn
 * with it (`toolRunTraceCursorByWindowId.procedural-preview.run.expected an exact u64 integer, found
 * Float(1.0)`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The wgpu door has no such hole because its
 * producer is Rust and `ToValue` answers `Number::UInt`; this is the React door's twin of that, and
 * the two are pinned to the same bytes by
 * `🛂️manifest/🪟️view-context/🧫️fixtures/🔢️integer-carriers/🔣️.json`.
 *
 * Which fields are integers is the view-context SCHEMA's knowledge, not this layer's — it is
 * declared once in {@link viewContextWithIntegerCarriers} (`🛂️manifest/🟦️.ts`).
 *
 * 🔁️ Idempotent, because the two renderer doors reach this seam holding different things. The React
 * shell builds its context out of JavaScript numbers; the wgpu bridge decodes one straight out of
 * pack and already holds {@link PackInteger} carriers (and, for a call with no context at all,
 * `undefined`). Anything that is not a view-context object crosses untouched, and a value that is
 * already a carrier is already exact — minting it twice would throw on the `BigInt` and take the
 * dispatch with it, which is the very failure this function exists to end. */
export function viewContextWireValue(view: unknown): unknown {
  if (view === null || typeof view !== "object" || Array.isArray(view)) return view;
  return viewContextWithIntegerCarriers(view as PluginViewState, (value) => (isPackInteger(value) ? value : packUInt(BigInt(value as number))));
}

/** 📥️ TS twin of `store::pack_rt::decode_wire_value` — the inverse of {@link encodePackValue}. */
export function decodePackValue(bytes: Uint8Array): PackValue {
  const pos: [number] = [0];
  const decoder = new TextDecoder();
  const symbolCount = readVarintU64(bytes, pos);
  const symbols: string[] = [];
  for (let i = 0; i < symbolCount; i++) {
    const len = readVarintU64(bytes, pos);
    symbols.push(decoder.decode(bytes.subarray(pos[0], pos[0] + len)));
    pos[0] += len;
  }

  const fieldCount = readVarintU64(bytes, pos);
  let result: PackValue = null;
  for (let i = 0; i < fieldCount; i++) {
    const fieldId = readVarintU64(bytes, pos);
    const outerTag = bytes[pos[0]]!;
    pos[0] += 1;
    if (outerTag !== PACK_TAG_VALUE) throw new Error(`decodePackValue: unexpected field tag 0x${outerTag.toString(16)} for field ${fieldId}`);
    const value = packDecodeValue(bytes, symbols, pos);
    if (fieldId === JSON_BRIDGE_FIELD_ID) result = value;
  }
  return result;
}

const PACK_B64_PREFIX = "pk:";

/** 📦️ Lossless pack snapshot as a `pk:`-prefixed base64 string for `sessionStorage`/`ViewModel` string slots. */
export function packValueToBase64(value: PackValue): string {
  const bytes = encodePackValue(value);
  let binary = "";
  for (let index = 0; index < bytes.length; index += 1) binary += String.fromCharCode(bytes[index]!);
  return `${PACK_B64_PREFIX}${btoa(binary)}`;
}

/** 📥️ Inverse of {@link packValueToBase64}. */
export function packValueFromBase64(encoded: string): PackValue {
  if (!encoded.startsWith(PACK_B64_PREFIX)) throw new Error("packValueFromBase64: expected pk: prefix");
  const binary = atob(encoded.slice(PACK_B64_PREFIX.length));
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index);
  return decodePackValue(bytes);
}

/** 🎯️ Plugin `handleAction` wire: pack-base64 `{ controllerId, action, args? }`. */
export type ActionWire = { readonly controllerId: string; readonly action: string; readonly args?: PackValue };

export function encodeActionWire(descriptor: ActionWire): string {
  return packValueToBase64(descriptor);
}

/** 📥️ Inverse of {@link encodeActionWire}. */
export function decodeActionWire(wire: string): ActionWire {
  const wireValue = packValueFromBase64(wire);
  if (wireValue === null || typeof wireValue !== "object" || Array.isArray(wireValue) || isPackInteger(wireValue)) throw new Error("decodeActionWire: expected a dynamic map");
  const record = wireValue as Readonly<Record<string, PackValue>>;
  if (typeof record.controllerId !== "string" || typeof record.action !== "string") throw new Error("decodeActionWire: controllerId and action are required strings");
  return record.args === undefined ? { controllerId: record.controllerId, action: record.action } : { controllerId: record.controllerId, action: record.action, args: record.args };
}

/** 🎬️ Decodes a component-scene `*Json` field when it carries {@link packValueToBase64} bytes. */
export function decodeScenePackField(encoded: string): PackValue {
  return packValueFromBase64(encoded);
}

/** 📤️ `protocol::encode_envelopes` batch as a {@link packValueToBase64} string for `applyMutations`. */
export function encodeMutationEnvelopesPack(envelopes: readonly MutationEnvelope[]): string {
  return packValueToBase64(Array.from(encodeCausalEnvelopeBatch(envelopes, replicationPackCodec)));
}

/** 📥️ Inverse of {@link encodeMutationEnvelopesPack}. */
export function decodeMutationEnvelopesPack(pack: string): MutationEnvelope[] {
  const wire = packValueFromBase64(pack);
  if (!isPackByteVector(wire)) throw new Error("decodeMutationEnvelopesPack: expected pack byte array");
  return decodeCausalEnvelopeBatch(wire, replicationPackCodec);
}

/** 🧮️ Projects a decoded {@link PackValue} onto strict JSON. An integer carrier survives
 * only when its `bigint` fits a safe JS integer exactly, and then as a `number`; anything else
 * throws rather than rounding. The descriptor pipeline is deliberately JSON-only — canonical-check
 * and hash the raw `PackValue`, then project through this for JSON schema/pair comparison and JSON
 * writing. Never the other way round: JSON never learns to carry an integer carrier. */
export function packValueToExactJson(value: PackValue, path = "$"): unknown {
  if (isPackInteger(value)) {
    if (value.value < BigInt(Number.MIN_SAFE_INTEGER) || value.value > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error(`packValueToExactJson: ${path} carries ${value.value}, which no JSON number represents exactly`);
    return Number(value.value);
  }
  if (Array.isArray(value)) return (value as readonly PackValue[]).map((item, index) => packValueToExactJson(item, `${path}[${index}]`));
  if (value !== null && typeof value === "object") {
    packRejectIntegerLookAlike(value);
    return Object.fromEntries(Object.entries(value as Record<string, PackValue>).map(([key, entry]) => [key, packValueToExactJson(entry, `${path}.${key}`)]));
  }
  if (typeof value === "number" && !Number.isFinite(value)) throw new Error(`packValueToExactJson: ${path} carries ${value}, which no JSON number represents`);
  if (value !== null && typeof value !== "boolean" && typeof value !== "number" && typeof value !== "string") throw new Error(`packValueToExactJson: ${path} carries an unsupported ${typeof value}`);
  return value;
}

/** 🧱️ The one byte-vector boundary every artifact/envelope byte parser shares: each item
 * must be a finite, safe, integral JS `number` in `0..255`. A `PackInteger` and a fractional or
 * out-of-range `number` are both rejected rather than coerced. */
export function isPackByteVector(value: PackValue): value is readonly number[] {
  return Array.isArray(value) && (value as readonly PackValue[]).every((entry) => typeof entry === "number" && Number.isSafeInteger(entry) && entry >= 0 && entry <= 255);
}

/** 🔢️ Narrows one declared unsigned field to an exact JS integer, or `null`. A `TAG_UINT`
 * carrier and a plain integral `number` both qualify; a signed carrier, a fraction, and anything
 * beyond `Number.MAX_SAFE_INTEGER` do not — an out-of-range value is a rejection, never a rounding. */
export function packUIntSafeOrNull(value: PackValue): number | null {
  if (isPackInteger(value)) return value.kind === "uint" && value.value <= BigInt(Number.MAX_SAFE_INTEGER) ? Number(value.value) : null;
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 0 ? value : null;
}

/** 🔍️ Renders one raw wasm-boundary value for a fault message, `bigint`s included, bounded so a
 * whole decoded node never lands in a log line. */
export function describePackWireValue(raw: unknown): string {
  if (typeof raw === "bigint") return `${raw}n`;
  try {
    return JSON.stringify(raw, (_key, value) => (typeof value === "bigint" ? `${value}n` : value))?.slice(0, 400) ?? String(raw);
  } catch {
    return String(raw);
  }
}

/** 🔢️ One natural number off an actor WIT/pack boundary: a WIT `u64` arrives as a `bigint`, a
 * pack-decoded integer as a lossless integer carrier, an absent value means zero. Anything not exactly
 * representable as a safe non-negative JS integer is a fault naming the field and the value, never a
 * rounding. Every renderer's wire decoder shares this one implementation. */
export function packWireNatural(raw: unknown, field = "natural"): number {
  if (raw === undefined || raw === null) return 0;
  const value = typeof raw === "bigint" ? (raw >= 0n && raw <= BigInt(Number.MAX_SAFE_INTEGER) ? Number(raw) : Number.NaN) : typeof raw === "number" ? raw : packUIntSafeOrNull(raw as PackValue) ?? Number.NaN;
  if (!Number.isSafeInteger(value) || value < 0) throw new Error(`actor WIT natural "${field}" is not a safe non-negative integer: ${describePackWireValue(raw)}`);
  return value;
}

/** 📦️ Decodes one pack-encoded wasm-boundary byte payload and projects its lossless integer
 * carriers onto exact JSON numbers — the only shape the UI/effect contract twins declare. The one
 * decoder every renderer injects wherever a `pack`-typed WIT field crosses into TypeScript; decoding
 * without this projection leaks `{kind, value}` carriers into node ids, revisions and effect params. */
export function decodePackWire(bytes: Uint8Array, path = "$"): unknown {
  return packValueToExactJson(decodePackValue(bytes) as PackValue, path);
}

function invocationResultObject(value: unknown, path: string, required: readonly string[], optional: readonly string[] = []): Readonly<Record<string, unknown>> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${path}: expected an object`);
  const record = value as Readonly<Record<string, unknown>>;
  const allowed = new Set([...required, ...optional]);
  if (Object.keys(record).some((key) => !allowed.has(key)) || required.some((key) => !(key in record))) throw new Error(`${path}: invalid exact fields`);
  return record;
}

function invocationResultString(value: unknown, path: string): string {
  if (typeof value !== "string" || value.length === 0) throw new Error(`${path}: expected a nonempty string`);
  return value;
}

function invocationResultNatural(value: unknown, path: string): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) throw new Error(`${path}: expected a safe natural number`);
  return value;
}

function invocationResultBytes(value: unknown, path: string): readonly number[] {
  if (!Array.isArray(value) || value.some((byte) => typeof byte !== "number" || !Number.isInteger(byte) || byte < 0 || byte > 255)) throw new Error(`${path}: expected bytes`);
  return value;
}

function invocationResultStrings(value: unknown, path: string): readonly string[] {
  if (!Array.isArray(value)) throw new Error(`${path}: expected an array`);
  return value.map((entry, index) => invocationResultString(entry, `${path}[${index}]`));
}

function invocationResultDiff(value: unknown, path: string): ArtifactDiff {
  const record = invocationResultObject(value, path, ["schema", "payload"]);
  return { schema: invocationResultString(record.schema, `${path}.schema`), payload: invocationResultBytes(record.payload, `${path}.payload`) };
}

function invocationResultUndoPolicy(value: unknown, path: string): UndoPolicy {
  if (value === "ExactBaseOnly" || value === "TransformAgainstConcurrent" || value === "SemanticUndo" || value === "CompensatingAction") return value;
  throw new Error(`${path}: invalid undo policy`);
}

function invocationResultInverse(value: unknown, path: string): InverseMutation {
  const record = invocationResultObject(value, path, ["targetMutation", "inverseDiff", "baseVersion", "undoPolicy"], ["dependencies"]);
  return {
    targetMutation: invocationResultString(record.targetMutation, `${path}.targetMutation`),
    inverseDiff: invocationResultDiff(record.inverseDiff, `${path}.inverseDiff`),
    baseVersion: invocationResultNatural(record.baseVersion, `${path}.baseVersion`),
    ...(record.dependencies === undefined ? {} : { dependencies: invocationResultStrings(record.dependencies, `${path}.dependencies`) }),
    undoPolicy: invocationResultUndoPolicy(record.undoPolicy, `${path}.undoPolicy`),
  };
}

function invocationResultMutation(value: unknown, path: string): KernelMutation {
  const record = invocationResultObject(value, path, ["id", "document", "baseVersion", "invocationId", "diff", "inverse", "author", "timestamp"], ["dependencies"]);
  const timestamp = invocationResultObject(record.timestamp, `${path}.timestamp`, ["actor", "physical_ms", "logical"]);
  return {
    id: invocationResultString(record.id, `${path}.id`),
    document: invocationResultString(record.document, `${path}.document`),
    baseVersion: invocationResultNatural(record.baseVersion, `${path}.baseVersion`),
    invocationId: invocationResultString(record.invocationId, `${path}.invocationId`),
    diff: invocationResultDiff(record.diff, `${path}.diff`),
    inverse: invocationResultInverse(record.inverse, `${path}.inverse`),
    ...(record.dependencies === undefined ? {} : { dependencies: invocationResultStrings(record.dependencies, `${path}.dependencies`) }),
    author: invocationResultString(record.author, `${path}.author`),
    timestamp: {
      actor: invocationResultNatural(timestamp.actor, `${path}.timestamp.actor`),
      physical_ms: invocationResultNatural(timestamp.physical_ms, `${path}.timestamp.physical_ms`),
      logical: invocationResultNatural(timestamp.logical, `${path}.timestamp.logical`),
    },
  };
}

function invocationResultUndoGroup(value: unknown, path: string): UndoGroup {
  const record = invocationResultObject(value, path, ["invocationId", "mutations", "inverseMutations"], ["memberEdits"]);
  if (typeof record.invocationId !== "string") throw new Error(`${path}.invocationId: expected a string`);
  if (!Array.isArray(record.inverseMutations)) throw new Error(`${path}.inverseMutations: expected an array`);
  if (record.memberEdits !== undefined && !Array.isArray(record.memberEdits)) throw new Error(`${path}.memberEdits: expected an array`);
  const mutations = invocationResultStrings(record.mutations, `${path}.mutations`);
  if (record.invocationId.length === 0 && (mutations.length !== 0 || record.inverseMutations.length !== 0 || (record.memberEdits?.length ?? 0) !== 0)) throw new Error(`${path}.invocationId: empty identity cannot own mutation results`);
  return {
    invocationId: record.invocationId,
    mutations,
    inverseMutations: record.inverseMutations.map((entry, index) => invocationResultInverse(entry, `${path}.inverseMutations[${index}]`)),
    ...(record.memberEdits === undefined
      ? {}
      : {
          memberEdits: record.memberEdits.map((entry, index) => {
            const edit = invocationResultObject(entry, `${path}.memberEdits[${index}]`, ["document", "editId"]);
            return { document: invocationResultString(edit.document, `${path}.memberEdits[${index}].document`), editId: invocationResultString(edit.editId, `${path}.memberEdits[${index}].editId`) };
          }),
        }),
  };
}

/** 📥️ Decodes the exact packed mutation and inverse-group fields retained by an Invocation frame. */
export function decodeInvocationResultPacks(frame: { readonly mutations: ArrayLike<number>; readonly inverse_group: ArrayLike<number> }): Pick<import("@semio-tech/framework").InvocationResponse, "mutations" | "inverseGroup"> {
  if (frame.mutations.length > INVOCATION_RESULT_PACK_MAXIMUM_BYTES || frame.inverse_group.length > INVOCATION_RESULT_PACK_MAXIMUM_BYTES) throw new Error("invocation result: packed field exceeds command transport authority");
  if (frame.mutations.length === 0 && frame.inverse_group.length === 0) return { mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } };
  if (frame.mutations.length === 0 || frame.inverse_group.length === 0) throw new Error("invocation result: mutations and inverse group must be published together");
  const mutations = decodePackWire(new Uint8Array(frame.mutations), "invocation.mutations");
  if (!Array.isArray(mutations)) throw new Error("invocation.mutations: expected an array");
  return { mutations: mutations.map((entry, index) => invocationResultMutation(entry, `invocation.mutations[${index}]`)), inverseGroup: invocationResultUndoGroup(decodePackWire(new Uint8Array(frame.inverse_group), "invocation.inverseGroup"), "invocation.inverseGroup") };
}

/** 🔢️ Throwing form of {@link packUIntSafeOrNull} for a required schema field. */
export function asPackUIntSafe(value: PackValue, name: string): number {
  const parsed = packUIntSafeOrNull(value);
  if (parsed === null) throw new Error(`${name}: expected an exact unsigned integer within the safe JS range`);
  return parsed;
}

/** 🔢️ {@link asPackUIntSafe} additionally bounded to `u32`. */
export function asPackUInt32(value: PackValue, name: string): number {
  const parsed = asPackUIntSafe(value, name);
  if (parsed > 0xffff_ffff) throw new Error(`${name}: ${parsed} exceeds the declared u32 range`);
  return parsed;
}
//#endregion 🔖️PublicApi
//#endregion 🔖️PackValueCodec

//#region 🔖️ScenePackCodec
const SCENE_PACK_TAG_UNIT = 0;
const SCENE_PACK_TAG_FALSE = 1;
const SCENE_PACK_TAG_TRUE = 2;
const SCENE_PACK_TAG_U64 = 3;
const SCENE_PACK_TAG_I64 = 4;
const SCENE_PACK_TAG_F64 = 5;
const SCENE_PACK_TAG_STR = 6;
const SCENE_PACK_TAG_BYTES = 7;
const SCENE_PACK_TAG_NONE = 8;
const SCENE_PACK_TAG_SOME = 9;
const SCENE_PACK_TAG_SEQ = 10;
const SCENE_PACK_TAG_CHAR = 11;
const SCENE_PACK_TAG_VARIANT = 12;
const SCENE_PACK_TAG_MAP = 13;
const SCENE_PACK_UNIT = Symbol("scene-pack-unit");

function readScenePackVarint(bytes: Uint8Array, position: { value: number }): bigint {
  let value = 0n;
  for (let shift = 0n; shift < 70n; shift += 7n) {
    const byte = bytes[position.value];
    if (byte === undefined) throw new Error("decodeScenePackValue: truncated varint");
    position.value += 1;
    value |= BigInt(byte & 0x7f) << shift;
    if ((byte & 0x80) === 0) return value;
  }
  throw new Error("decodeScenePackValue: varint exceeds u64");
}

function scenePackNumber(value: bigint): number {
  const number = Number(value);
  if (!Number.isSafeInteger(number)) throw new Error("decodeScenePackValue: integer exceeds JavaScript's safe range");
  return number;
}

/** 🔢️ A scene integer FIELD keeps its exact value: safe integers decode as `number`, anything wider (a
 * u64 revision hash on a snapshot lease, for instance) stays a `bigint` instead of refusing the whole
 * doc. Lengths and code points still go through {@link scenePackNumber}, where a wide value is a fault. */
function scenePackInteger(value: bigint): number | bigint {
  const number = Number(value);
  return Number.isSafeInteger(number) ? number : value;
}

function readScenePackLength(bytes: Uint8Array, position: { value: number }): number {
  const length = scenePackNumber(readScenePackVarint(bytes, position));
  if (length > bytes.length - position.value) throw new Error("decodeScenePackValue: declared length exceeds remaining bytes");
  return length;
}

function decodeScenePackItem(bytes: Uint8Array, position: { value: number }): unknown | typeof SCENE_PACK_UNIT {
  const tag = bytes[position.value];
  if (tag === undefined) throw new Error("decodeScenePackValue: truncated value");
  position.value += 1;
  if (tag === SCENE_PACK_TAG_UNIT) return SCENE_PACK_UNIT;
  if (tag === SCENE_PACK_TAG_FALSE) return false;
  if (tag === SCENE_PACK_TAG_TRUE) return true;
  if (tag === SCENE_PACK_TAG_U64) return scenePackInteger(readScenePackVarint(bytes, position));
  if (tag === SCENE_PACK_TAG_I64) {
    const raw = readScenePackVarint(bytes, position);
    return scenePackInteger((raw >> 1n) ^ -(raw & 1n));
  }
  if (tag === SCENE_PACK_TAG_F64) {
    if (bytes.length - position.value < 8) throw new Error("decodeScenePackValue: truncated f64");
    const value = new DataView(bytes.buffer, bytes.byteOffset + position.value, 8).getFloat64(0, true);
    position.value += 8;
    return value;
  }
  if (tag === SCENE_PACK_TAG_STR) {
    const length = readScenePackLength(bytes, position);
    const value = new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(position.value, position.value + length));
    position.value += length;
    return value;
  }
  if (tag === SCENE_PACK_TAG_BYTES) {
    const length = readScenePackLength(bytes, position);
    const value = Array.from(bytes.subarray(position.value, position.value + length));
    position.value += length;
    return value;
  }
  if (tag === SCENE_PACK_TAG_NONE) return null;
  if (tag === SCENE_PACK_TAG_SOME) return decodeScenePackItem(bytes, position);
  if (tag === SCENE_PACK_TAG_SEQ) {
    const count = readScenePackLength(bytes, position);
    const value: unknown[] = [];
    for (let index = 0; index < count; index += 1) {
      const item = decodeScenePackItem(bytes, position);
      value.push(item === SCENE_PACK_UNIT ? null : item);
    }
    return value;
  }
  if (tag === SCENE_PACK_TAG_CHAR) {
    const codePoint = scenePackNumber(readScenePackVarint(bytes, position));
    if (codePoint > 0x10ffff || (codePoint >= 0xd800 && codePoint <= 0xdfff)) throw new Error("decodeScenePackValue: invalid Unicode scalar");
    return String.fromCodePoint(codePoint);
  }
  if (tag === SCENE_PACK_TAG_VARIANT) {
    const name = decodeScenePackItem(bytes, position);
    if (typeof name !== "string") throw new Error("decodeScenePackValue: variant name is not a string");
    const payload = decodeScenePackItem(bytes, position);
    return payload === SCENE_PACK_UNIT ? name : { [name]: payload };
  }
  if (tag === SCENE_PACK_TAG_MAP) {
    const count = readScenePackLength(bytes, position);
    const value: Record<string, unknown> = {};
    for (let index = 0; index < count; index += 1) {
      const key = decodeScenePackItem(bytes, position);
      if (typeof key !== "string") throw new Error("decodeScenePackValue: map key is not a string");
      if (Object.hasOwn(value, key)) throw new Error(`decodeScenePackValue: duplicate map key ${key}`);
      const item = decodeScenePackItem(bytes, position);
      value[key] = item === SCENE_PACK_UNIT ? null : item;
    }
    return value;
  }
  throw new Error(`decodeScenePackValue: invalid tag ${tag}`);
}

/** 🎬️ Decodes the self-describing serde packet emitted by `semio-framework-ui-scene`. */
export function decodeScenePackValue(bytes: Uint8Array): unknown {
  const position = { value: 0 };
  const value = decodeScenePackItem(bytes, position);
  if (position.value !== bytes.length) throw new Error(`decodeScenePackValue: ${bytes.length - position.value} trailing bytes`);
  return value === SCENE_PACK_UNIT ? null : value;
}
//#endregion 🔖️ScenePackCodec

//#region 🔖️MeshPackCodec
/**
 * 🧊️ Field ids of the brep preview mesh `pack` record body — the exact ids
 * `semio-framework-os-flow`'s `mesh_pack_spec` declares. Every array rides as one little-endian
 * `Bytes64` blob, so decoding is a typed-array view over the payload rather than a JSON parse of a
 * number-per-element array.
 */
const MESH_PACK_FIELD = {
  positions: 1,
  normals: 2,
  indices: 3,
  colors: 4,
  uvs: 5,
  faceIds: 6,
  vertexIds: 7,
  edgePositions: 8,
  edgeIds: 9,
  edgeUvs: 10,
  edgeIsSeam: 11,
  paintTextureBase64: 12,
  metadata: 13,
} as const;

const MESH_PACK_TAG_ABSENT = 0x00;
const MESH_PACK_TAG_STR = 0x06;
const MESH_PACK_TAG_STR_INLINE = 0x07;
const MESH_PACK_TAG_BYTES = 0x08;

/** 🧊️ One decoded preview mesh — typed arrays, ready for a GPU buffer upload with no copy. */
export type MeshPack = {
  readonly positions: Float32Array;
  readonly normals: Float32Array;
  readonly colors: Float32Array;
  readonly indices: Uint32Array;
  readonly uvs: Float32Array;
  readonly faceIds: Uint32Array;
  readonly vertexIds: Uint32Array;
  readonly edgePositions: Float32Array;
  readonly edgeIds: Uint32Array;
  readonly edgeUvs: Float32Array;
  readonly edgeIsSeam: Uint8Array;
  readonly paintTextureBase64?: string;
  readonly attributes?: Record<string, { domain: "vertex" | "corner" | "face" | "edge"; semantic: "normal" | "uv" | "color" | "material" | "custom"; interpolation: "linear" | "nearest" | "constant"; values: unknown[]; indices?: number[] }>;
  readonly materials?: Record<string, Record<string, unknown>>;
  readonly textures?: Record<string, { mime: string; bytes: number[] }>;
  readonly componentReferences?: Readonly<Record<string, readonly string[]>>;
};

function readMeshPackVarint(bytes: Uint8Array, position: { value: number }): number {
  let result = 0n;
  for (let shift = 0n; shift < 70n; shift += 7n) {
    const byte = bytes[position.value];
    if (byte === undefined) throw new Error("decodeMeshPackBody: truncated varint");
    position.value += 1;
    result |= BigInt(byte & 0x7f) << shift;
    if ((byte & 0x80) === 0) {
      const value = Number(result);
      if (!Number.isSafeInteger(value)) throw new Error("decodeMeshPackBody: varint exceeds JavaScript's safe integer range");
      return value;
    }
  }
  throw new Error("decodeMeshPackBody: varint exceeds u64");
}

function readMeshPackBytes(bytes: Uint8Array, position: { value: number }): Uint8Array {
  const length = readMeshPackVarint(bytes, position);
  if (length > bytes.length - position.value) throw new Error("decodeMeshPackBody: declared length exceeds remaining bytes");
  const slice = bytes.slice(position.value, position.value + length);
  position.value += length;
  return slice;
}

function meshPackFloats(blob: Uint8Array | undefined): Float32Array {
  if (blob === undefined) return new Float32Array(0);
  if (blob.byteLength % 4 !== 0) throw new Error("decodeMeshPackBody: f32 blob length is not a multiple of 4");
  const view = new DataView(blob.buffer, blob.byteOffset, blob.byteLength);
  const out = new Float32Array(blob.byteLength / 4);
  for (let index = 0; index < out.length; index += 1) out[index] = view.getFloat32(index * 4, true);
  return out;
}

function meshPackUints(blob: Uint8Array | undefined): Uint32Array {
  if (blob === undefined) return new Uint32Array(0);
  if (blob.byteLength % 4 !== 0) throw new Error("decodeMeshPackBody: u32 blob length is not a multiple of 4");
  const view = new DataView(blob.buffer, blob.byteOffset, blob.byteLength);
  const out = new Uint32Array(blob.byteLength / 4);
  for (let index = 0; index < out.length; index += 1) out[index] = view.getUint32(index * 4, true);
  return out;
}

/**
 * 🧊️ Decodes the container-less `pack` record body `encode_mesh_pack` emits (symbol table, field
 * count, then `(field id, tag, payload)` triples). The TypeScript half of the cross-language mesh
 * wire — the Rust encoder and this decoder are pinned to one another by the shared golden fixture
 * `🧫️fixtures/🧊️mesh/mesh-pack-body-v1.json`.
 */
export function decodeMeshPackBody(bytes: Uint8Array): MeshPack {
  const position = { value: 0 };
  const symbolCount = readMeshPackVarint(bytes, position);
  const symbols: string[] = [];
  for (let index = 0; index < symbolCount; index += 1) symbols.push(new TextDecoder("utf-8", { fatal: true }).decode(readMeshPackBytes(bytes, position)));
  const fieldCount = readMeshPackVarint(bytes, position);
  const blobs = new Map<number, Uint8Array>();
  let paintTextureBase64: string | undefined;
  for (let index = 0; index < fieldCount; index += 1) {
    const fieldId = readMeshPackVarint(bytes, position);
    const tag = bytes[position.value];
    if (tag === undefined) throw new Error("decodeMeshPackBody: truncated field value");
    position.value += 1;
    if (tag === MESH_PACK_TAG_ABSENT) continue;
    if (tag === MESH_PACK_TAG_BYTES) {
      blobs.set(fieldId, readMeshPackBytes(bytes, position));
      continue;
    }
    if (tag === MESH_PACK_TAG_STR) {
      const symbol = symbols[readMeshPackVarint(bytes, position)];
      if (symbol === undefined) throw new Error("decodeMeshPackBody: symref out of range");
      if (fieldId === MESH_PACK_FIELD.paintTextureBase64) paintTextureBase64 = symbol;
      continue;
    }
    if (tag === MESH_PACK_TAG_STR_INLINE) {
      const text = new TextDecoder("utf-8", { fatal: true }).decode(readMeshPackBytes(bytes, position));
      if (fieldId === MESH_PACK_FIELD.paintTextureBase64) paintTextureBase64 = text;
      continue;
    }
    throw new Error(`decodeMeshPackBody: unexpected tag 0x${tag.toString(16)} for field ${fieldId}`);
  }
  if (position.value !== bytes.length) throw new Error(`decodeMeshPackBody: ${bytes.length - position.value} trailing bytes`);
  const metadataBytes = blobs.get(MESH_PACK_FIELD.metadata);
  if (metadataBytes && metadataBytes.length > 16_000_000) throw new Error("decodeMeshPackBody: metadata exceeds 16 MB");
  const metadata = metadataBytes ? JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(metadataBytes)) as Pick<MeshPack, "attributes" | "materials" | "textures" | "componentReferences"> : {};
  if (!metadata || typeof metadata !== "object" || Array.isArray(metadata) || Object.keys(metadata).some(key => !["attributes", "materials", "textures", "componentReferences"].includes(key))) throw new Error("decodeMeshPackBody: invalid metadata fields");
  if (metadata.componentReferences !== undefined && (!metadata.componentReferences || typeof metadata.componentReferences !== "object" || Array.isArray(metadata.componentReferences) || Object.entries(metadata.componentReferences).some(([domain, labels]) => !["face", "edge", "vertex"].includes(domain) || !Array.isArray(labels) || labels.length > 600_000 || labels.some(label => typeof label !== "string" || !label.length || label.length > 256 || Array.from(label).length > 128)))) throw new Error("decodeMeshPackBody: invalid analytic component references");
  const mesh: MeshPack = {
    ...metadata,
    positions: meshPackFloats(blobs.get(MESH_PACK_FIELD.positions)),
    normals: meshPackFloats(blobs.get(MESH_PACK_FIELD.normals)),
    colors: meshPackFloats(blobs.get(MESH_PACK_FIELD.colors)),
    indices: meshPackUints(blobs.get(MESH_PACK_FIELD.indices)),
    uvs: meshPackFloats(blobs.get(MESH_PACK_FIELD.uvs)),
    faceIds: meshPackUints(blobs.get(MESH_PACK_FIELD.faceIds)),
    vertexIds: meshPackUints(blobs.get(MESH_PACK_FIELD.vertexIds)),
    edgePositions: meshPackFloats(blobs.get(MESH_PACK_FIELD.edgePositions)),
    edgeIds: meshPackUints(blobs.get(MESH_PACK_FIELD.edgeIds)),
    edgeUvs: meshPackFloats(blobs.get(MESH_PACK_FIELD.edgeUvs)),
    edgeIsSeam: blobs.get(MESH_PACK_FIELD.edgeIsSeam) ?? new Uint8Array(0),
  };
  for (const [domain, labels] of Object.entries(mesh.componentReferences ?? {})) {
    const [ids,count]:readonly[Uint32Array,number]=domain==="face"?[mesh.faceIds,mesh.indices.length/3]:domain==="edge"?[mesh.edgeIds,mesh.edgePositions.length/6]:[mesh.vertexIds,mesh.positions.length/3];
    if (labels.length>count || ids.length!==count || ids.some(id=>id>=labels.length)) throw new Error("decodeMeshPackBody: analytic component references require complete picking buffers");
  }
  return paintTextureBase64 === undefined ? mesh : { ...mesh, paintTextureBase64 };
}

/** 🧱️ Reassembles the base64 chunks one tessellate round trip streams and decodes them. */
export function decodeMeshPackChunks(chunks: readonly string[]): MeshPack {
  const binary = atob(chunks.join(""));
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index);
  return decodeMeshPackBody(bytes);
}
//#endregion 🔖️MeshPackCodec

//#region 🔖️AppChannelCodec
//#region 🔖️Types
/** 📦️ Largest packed invocation-result field the command transport carries — the twin of
 * `INVOCATION_RESULT_PACK_MAXIMUM_BYTES` in `📡️spr/🧵️channel/🦀️.rs`, which reads the one declared
 * assembled host-answer budget rather than a page count of its own. */
export const INVOCATION_RESULT_PACK_MAXIMUM_BYTES = GUEST_HOST_ANSWER_CEILING_BYTES;
//#endregion 🔖️Types
//#endregion 🔖️AppChannelCodec
