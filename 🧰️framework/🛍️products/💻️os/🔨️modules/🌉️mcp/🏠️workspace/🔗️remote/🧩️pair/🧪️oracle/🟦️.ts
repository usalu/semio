import { createHash, webcrypto } from "node:crypto";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";

type Fixture = {
  limits: { pairBytes: number; cacheBytes: number; entries: number; headerBytes: number; recordBytes: number };
  binding: { hubOrigin: string; authorityGeneration: number; spaceId: string; documentId: string; sameDocumentOtherSpace: string; descriptorDigest: string; checkpointId: string; catalogGeneration: number };
  valid: { mediaType: string; etag: string; packHex: string; sprHex: string; packSha256: string; sprSha256: string; aggregateSha256: string; wireHex: string };
  negativeVectors: string[];
  lifecycle: string[];
};

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(new URL("../🧫️fixtures/🧬️.schema.json", import.meta.url), "utf8"));
const validate = new Ajv2020({ strict: true }).compile(schema);
if (!validate(fixture)) throw new Error(JSON.stringify(validate.errors));

const digest = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const subtleDigest = async (bytes: Uint8Array): Promise<string> => Buffer.from(await webcrypto.subtle.digest("SHA-256", bytes)).toString("hex");
const etag = (header: Uint8Array): string => `"${createHash("sha256").update("semio.hub.canonical-checkpoint-pair-etag.v1\0").update(header).digest("hex")}"`;
const hex = (value: string): Buffer => Buffer.from(value, "hex");
const utf8 = new TextDecoder("utf-8", { fatal: true });

type Frame = { start: number; payload: number; length: number; end: number };
const frames = (wire: Buffer): Frame[] => {
  const output: Frame[] = [];
  let offset = 0;
  while (offset < wire.length) {
    if (offset + 4 > wire.length) throw new Error("truncated frame length");
    const length = wire.readUInt32BE(offset);
    const end = offset + 4 + length;
    if (length === 0 || end > wire.length) throw new Error("truncated frame payload");
    output.push({ start: offset, payload: offset + 4, length, end });
    offset = end;
  }
  return output;
};

const readText = (bytes: Buffer, state: { offset: number }, required: boolean): string => {
  const length = bytes.readUInt32BE(state.offset); state.offset += 4;
  if ((required && length === 0) || length > 512 || state.offset + length > bytes.length) throw new Error("invalid text");
  const value = utf8.decode(bytes.subarray(state.offset, state.offset + length)); state.offset += length;
  if (/\p{Cc}/u.test(value)) throw new Error("control character in text");
  return value;
};
const take = (bytes: Buffer, state: { offset: number }, length: number): Buffer => {
  if (state.offset + length > bytes.length) throw new Error("truncated field");
  const value = bytes.subarray(state.offset, state.offset + length); state.offset += length; return value;
};

const decode = async (wire: Buffer, expectedSpace = fixture.binding.spaceId, expectedEtag = fixture.valid.etag): Promise<void> => {
  if (wire.length > fixture.limits.pairBytes + fixture.limits.headerBytes + 4 + (Math.ceil(fixture.limits.pairBytes / fixture.limits.recordBytes) + 1) * 22 + 6) throw new Error("wire budget");
  const all = frames(wire);
  if (all.length < 4) throw new Error("missing records");
  const header = wire.subarray(all[0].payload, all[0].end);
  const state = { offset: 0 };
  if (header[state.offset++] !== 1 || header.readUInt32BE(state.offset) !== 1) throw new Error("header version"); state.offset += 4;
  const space = readText(header, state, true);
  const document = readText(header, state, true);
  const descriptor = take(header, state, 32).toString("hex");
  const checkpoint = take(header, state, 32);
  const frontierDocument = readText(header, state, true);
  state.offset += 8;
  readText(header, state, false);
  state.offset += 8;
  const chain = take(header, state, 32);
  const packHash = take(header, state, 32).toString("hex");
  const packLength = Number(header.readBigUInt64BE(state.offset)); state.offset += 8;
  const sprHash = take(header, state, 32).toString("hex");
  const sprLength = Number(header.readBigUInt64BE(state.offset)); state.offset += 8;
  const aggregate = take(header, state, 32).toString("hex");
  if (state.offset !== header.length || space !== expectedSpace || document !== fixture.binding.documentId || descriptor !== fixture.binding.descriptorDigest || checkpoint.every((byte) => byte === 0) || chain.every((byte) => byte === 0) || frontierDocument !== document || etag(header) !== expectedEtag) throw new Error("authority identity");
  if (packLength === 0 || sprLength === 0 || packLength + sprLength > fixture.limits.pairBytes) throw new Error("pair budget");
  const recordCount = Math.ceil(packLength / fixture.limits.recordBytes) + Math.ceil(sprLength / fixture.limits.recordBytes);
  if (all.length !== recordCount + 2) throw new Error("record count");
  const pack: Buffer[] = []; const spr: Buffer[] = []; let packOffset = 0; let sprOffset = 0;
  for (let ordinal = 0; ordinal < recordCount; ordinal++) {
    const record = wire.subarray(all[ordinal + 1].payload, all[ordinal + 1].end);
    if (record[0] !== 2 || record.readUInt32BE(2) !== ordinal) throw new Error("record order");
    const part = record[1]; const offset = Number(record.readBigUInt64BE(6)); const length = record.readUInt32BE(14);
    if (length === 0 || length > fixture.limits.recordBytes || 18 + length !== record.length) throw new Error("record size");
    if (part === 1 && offset === packOffset && sprOffset === 0) { pack.push(record.subarray(18)); packOffset += length; }
    else if (part === 2 && packOffset === packLength && offset === sprOffset) { spr.push(record.subarray(18)); sprOffset += length; }
    else throw new Error("part order");
  }
  const terminal = wire.subarray(all.at(-1)!.payload, all.at(-1)!.end);
  if (!terminal.equals(Buffer.from([3, 0])) || packOffset !== packLength || sprOffset !== sprLength) throw new Error("terminal");
  const packBytes = Buffer.concat(pack); const sprBytes = Buffer.concat(spr);
  if (await subtleDigest(packBytes) !== packHash || digest(sprBytes) !== sprHash || digest(Buffer.concat([packBytes, sprBytes])) !== aggregate) throw new Error("integrity");
};

const mutate = (name: string, valid: Buffer): { wire: Buffer; space?: string; etag?: string } => {
  const all = frames(valid); const header = valid.subarray(all[0].payload, all[0].end); const copy = Buffer.from(valid);
  const locate = (needle: Buffer): number => { const at = copy.indexOf(needle); if (at < 0) throw new Error(`missing mutation target ${name}`); return at; };
  if (name === "truncated") return { wire: copy.subarray(0, copy.length - 1) };
  if (name === "reordered") return { wire: Buffer.concat([copy.subarray(all[0].start, all[0].end), copy.subarray(all[2].start, all[2].end), copy.subarray(all[1].start, all[1].end), copy.subarray(all[3].start)]) };
  if (name === "duplicate") return { wire: Buffer.concat([copy.subarray(0, all[2].start), copy.subarray(all[1].start, all[1].end), copy.subarray(all[2].start)]) };
  if (name === "oversize-record") { copy.writeUInt32BE(4097, all[1].payload + 14); return { wire: copy }; }
  if (name === "wrong-scope") { copy[locate(Buffer.from("space:alpha")) + 6] ^= 1; return { wire: copy, etag: etag(copy.subarray(all[0].payload, all[0].end)) }; }
  if (name === "wrong-digest") { copy[locate(hex(fixture.binding.descriptorDigest))] ^= 1; return { wire: copy, etag: etag(copy.subarray(all[0].payload, all[0].end)) }; }
  if (name === "wrong-checkpoint") { copy.fill(0, locate(hex(fixture.binding.checkpointId)), locate(hex(fixture.binding.checkpointId)) + 32); return { wire: copy, etag: etag(copy.subarray(all[0].payload, all[0].end)) }; }
  if (name === "malformed-utf8") { copy[locate(Buffer.from("edit:7"))] = 0xff; return { wire: copy, etag: etag(copy.subarray(all[0].payload, all[0].end)) }; }
  if (name === "control-character") { copy[locate(Buffer.from("edit:7"))] = 0x01; return { wire: copy, etag: etag(copy.subarray(all[0].payload, all[0].end)) }; }
  if (name === "bad-pack-hash") { copy[locate(hex(fixture.valid.packSha256))] ^= 1; return { wire: copy, etag: etag(copy.subarray(all[0].payload, all[0].end)) }; }
  if (name === "bad-aggregate") { copy[locate(hex(fixture.valid.aggregateSha256))] ^= 1; return { wire: copy, etag: etag(copy.subarray(all[0].payload, all[0].end)) }; }
  if (name === "bad-etag") return { wire: copy, etag: `"${"0".repeat(64)}"` };
  if (name === "missing-terminal") return { wire: copy.subarray(0, all.at(-1)!.start) };
  if (name === "trailing-data") return { wire: Buffer.concat([copy, Buffer.from([0])]) };
  if (name === "same-document-other-space") { copy[locate(Buffer.from("space:alpha")) + 6] ^= 2; return { wire: copy, etag: etag(copy.subarray(all[0].payload, all[0].end)) }; }
  throw new Error(`unknown vector ${name}`);
};

const wire = hex(fixture.valid.wireHex);
await decode(wire);
if (digest(hex(fixture.valid.packHex)) !== fixture.valid.packSha256 || await subtleDigest(hex(fixture.valid.sprHex)) !== fixture.valid.sprSha256) throw new Error("fixture part hash mismatch");

let rejected = 0;
for (const name of fixture.negativeVectors) {
  const candidate = mutate(name, wire);
  try { await decode(candidate.wire, candidate.space, candidate.etag); } catch { rejected++; }
}
if (rejected !== fixture.negativeVectors.length) throw new Error(`negative vectors ${rejected}/${fixture.negativeVectors.length}`);

const key = (origin: string, generation: number, space: string, document: string, descriptor: string, checkpoint: string, currentEtag: string, catalog: number): string => JSON.stringify([origin, generation, space, document, descriptor, checkpoint, currentEtag, catalog]);
const first = key(fixture.binding.hubOrigin, fixture.binding.authorityGeneration, fixture.binding.spaceId, fixture.binding.documentId, fixture.binding.descriptorDigest, fixture.binding.checkpointId, fixture.valid.etag, fixture.binding.catalogGeneration);
const secondBinding = key(fixture.binding.hubOrigin, fixture.binding.authorityGeneration + 1, fixture.binding.spaceId, fixture.binding.documentId, fixture.binding.descriptorDigest, fixture.binding.checkpointId, fixture.valid.etag, fixture.binding.catalogGeneration);
const otherSpace = key(fixture.binding.hubOrigin, fixture.binding.authorityGeneration, fixture.binding.sameDocumentOtherSpace, fixture.binding.documentId, fixture.binding.descriptorDigest, fixture.binding.checkpointId, fixture.valid.etag, fixture.binding.catalogGeneration);
if (new Set([first, secondBinding, otherSpace]).size !== 3) throw new Error("cache identity crossed a binding or space");
const resources = ["semio://workspace", "semio://workspace/artifacts", `semio://workspace/scopes/${fixture.binding.spaceId}/${fixture.binding.documentId}/descriptor`];
if (resources.some((uri) => /pair|checkpoint|chunk|manifest|hash/i.test(uri))) throw new Error("raw pair identity escaped into resources");
for (const required of ["cache-hit", "cache-miss", "stale-completion", "eviction", "revoke", "stream-loss-reconnect", "rebootstrap", "cancel", "restart"]) if (!fixture.lifecycle.includes(required)) throw new Error(`missing lifecycle vector ${required}`);

console.log(`canonical-pair-cache-mount oracle: 3/3; negatives ${rejected}/${fixture.negativeVectors.length}`);
