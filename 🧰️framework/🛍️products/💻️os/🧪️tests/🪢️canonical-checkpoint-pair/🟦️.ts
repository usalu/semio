/** 🪢️ The hub's canonical checkpoint pair — the one seed a native, a wasm32 and a React shell open a hub document on —
 * replayed from the language-agnostic fixture `🧫️fixtures/📇️directory/🪢️canonical-checkpoint-pair-v1.json`, the same
 * rows the kernel's Rust law (`📇️directory/🧬️schema/🪢️canonical-checkpoint-pair-v1`) replays. Ajv validates decoded scope and frontier payloads; `node:crypto` is the independent digest oracle (the TypeScript decoder leaves digests to the bootstrap
 * assembler, so this runner verifies them the way the Rust decoder does). */
import { describe, expect, it } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { semioSchemaAjvV1 } from "../../../../🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import {
  admitCanonicalCheckpointPairForRebootstrapV1,
  admitCanonicalCheckpointPairV1,
  type CanonicalCheckpointPairV1,
} from "../../🔨️modules/📇️directory/🧬️schema/🟦️.ts";

import { admitCheckpointSelectionV1, CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES, CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES, CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1, CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES, decodeCanonicalCheckpointPairV1 } from "../../🔨️modules/📇️directory/🚪️io/🧱️binary/🪢️checkpoint-pair/🟦️.ts";

const here = (path: string) => JSON.parse(readFileSync(fileURLToPath(new URL(path, import.meta.url)), "utf8"));
const fixture = here("../../🧫️fixtures/📇️directory/🪢️canonical-checkpoint-pair-v1.json");
const schema = here("../../🧬️schema/🪢️canonical-checkpoint-pair-v1/🔣️.json");
const hexBytes = (value: string) => Uint8Array.from(value.match(/../gu) ?? [], (pair) => Number.parseInt(pair, 16));
const sha256 = (...parts: Uint8Array[]) => parts.reduce((hash, part) => hash.update(part), createHash("sha256")).digest("hex");
const hex = (hash: readonly number[]) => Buffer.from(hash).toString("hex");
const part = (spec: { byte: number; length: number }) => new Uint8Array(spec.length).fill(spec.byte);

/** 🔏️ The digest stage of the Rust decoder, decided by `node:crypto`. */
function verifyDigests(pair: CanonicalCheckpointPairV1): void {
  if (sha256(pair.packBytes) !== hex(pair.pack.sha256)) throw new Error("canonical-checkpoint-pair.pack-digest");
  if (sha256(pair.sprBytes) !== hex(pair.spr.sha256)) throw new Error("canonical-checkpoint-pair.spr-digest");
  if (sha256(pair.packBytes, pair.sprBytes) !== hex(pair.aggregateSha256)) throw new Error("canonical-checkpoint-pair.aggregate-digest");
}

describe("🪢️ canonical checkpoint pair", () => {
  it("decoded scopes and frontiers satisfy their actual payload contracts", () => {
    const ajv = semioSchemaAjvV1({ allErrors: true, strict: true }).addSchema(schema);
    for (const row of fixture.pairs) {
      const decoded = decodeCanonicalCheckpointPairV1(hexBytes(row.bodyHex));
      expect(ajv.validate({ $ref: `${schema.$id}#/definitions/scope` }, decoded.scope)).toBe(true);
      expect(ajv.validate({ $ref: `${schema.$id}#/definitions/frontier` }, decoded.baselineFrontier)).toBe(true);
    }
  });

  it("the fixture limits are the TypeScript wire constants", () => {
    expect([fixture.mediaType, fixture.limits.headerBytes, fixture.limits.recordBytes, fixture.limits.records, fixture.limits.pairBytes]).toEqual([CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1, CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES, CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES, CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS, CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES]);
  });

  for (const pair of fixture.pairs) {
    it(`${pair.id} decodes to its selection and its verified bytes`, () => {
      const decoded = decodeCanonicalCheckpointPairV1(hexBytes(pair.bodyHex));
      verifyDigests(decoded);
      expect(decoded.scope).toEqual({ spaceId: pair.selection.spaceId, documentId: pair.selection.documentId });
      expect([hex(decoded.descriptorDigestV1), hex(decoded.activeCheckpointId)]).toEqual([pair.selection.descriptorDigestV1, pair.selection.checkpointId]);
      expect(decoded.baselineFrontier).toEqual(pair.selection.baselineFrontier);
      expect([hex(decoded.pack.sha256), hex(decoded.spr.sha256), hex(decoded.aggregateSha256)]).toEqual([pair.packSha256, pair.sprSha256, pair.aggregateSha256]);
      expect([decoded.packBytes, decoded.sprBytes]).toEqual([part(pair.pack), part(pair.spr)]);
      expect([sha256(part(pair.pack)), sha256(part(pair.spr)), sha256(part(pair.pack), part(pair.spr))]).toEqual([pair.packSha256, pair.sprSha256, pair.aggregateSha256]);
    });
  }

  for (const refusal of fixture.refusals) {
    it(`${refusal.id} is refused at its ${refusal.stage} stage as ${refusal.refusal}`, () => {
      const body = hexBytes(refusal.bodyHex);
      if (refusal.stage === "decode") expect(() => decodeCanonicalCheckpointPairV1(body)).toThrow(refusal.refusal);
      else expect(() => verifyDigests(decodeCanonicalCheckpointPairV1(body))).toThrow(refusal.refusal);
    });
  }

  for (const admission of fixture.rebootstrapAdmissions) {
    it(`rebootstrap ${admission.id} is ${admission.refusal ?? "admitted"}`, () => {
      const pair = fixture.pairs.find((candidate: { id: string }) => candidate.id === admission.pair);
      const decoded = decodeCanonicalCheckpointPairV1(hexBytes(pair.bodyHex));
      const control = { ...admission.control, checkpointId: [...hexBytes(admission.control.checkpointId)], descriptorDigestV1: [...hexBytes(admission.control.descriptorDigestV1)] };
      const admit = () => admitCanonicalCheckpointPairForRebootstrapV1(decoded, control);
      if (admission.refusal === null) expect(admit).not.toThrow();
      else expect(admit).toThrow(admission.refusal);
    });
  }

  for (const admission of fixture.admissions) {
    it(`${admission.id} is ${admission.refusal ?? "admitted"}`, () => {
      const pair = fixture.pairs.find((candidate: { id: string }) => candidate.id === admission.pair);
      const decoded = decodeCanonicalCheckpointPairV1(hexBytes(pair.bodyHex));
      const admit = () => admitCanonicalCheckpointPairV1(decoded, admission.scope, admitCheckpointSelectionV1(admission.expected));
      if (admission.refusal === null) expect(admit).not.toThrow();
      else expect(admit).toThrow(admission.refusal);
    });
  }
});


it("canonical checkpoint pair framing has an explicit IO owner", () => {
  const owner = here("../../🧫️fixtures/📇️directory/🪢️checkpoint-pair-io-owner-v1.json");
  const directory = new URL("../../🔨️modules/📇️directory/", import.meta.url);
  const semanticRust = readFileSync(new URL("🧬️schema/🪢️canonical-checkpoint-pair-v1/🦀️.rs", directory), "utf8");
  const semanticTs = readFileSync(new URL("🧬️schema/🟦️.ts", directory), "utf8");
  for (const body of owner.physicalBodies) expect(semanticRust).not.toContain(body);
  for (const body of owner.typescriptBodies) expect(semanticTs).not.toContain(body);
  const native = readFileSync(new URL("🚪️io/🧱️binary/🪢️checkpoint-pair/🦀️.rs", directory), "utf8");
  const wire = readFileSync(new URL("🚪️io/🧱️binary/🪢️checkpoint-pair/🟦️.ts", directory), "utf8");
  for (const body of owner.physicalBodies) expect(native).toContain(body);
  for (const body of owner.typescriptBodies) expect(wire).toContain(body);
  const law = readFileSync(new URL("🚪️io/🧱️binary/🪢️checkpoint-pair/🧪️tests/🔬️unit/🦀️.rs", directory), "utf8");
  expect(law).toContain(`fn ${owner.nativeLaw}()`);
  expect(law).toContain("module_path!()");
  const ajv = semioSchemaAjvV1({ allErrors: true, strict: true });
  expect(ajv.validate({ type: "object", additionalProperties: false, required: ["schema", "owner", "nativeLaw", "semanticMethods", "physicalBodies", "typescriptBodies"], properties: { schema: {const: "directory.checkpoint-pair.io-owner/v1"}, owner: {const: "os_directory::io::binary::checkpoint_pair"}, nativeLaw: {type: "string", minLength: 1}, semanticMethods: {type: "array", items: {type: "string"}, minItems: 2}, physicalBodies: {type: "array", items: {type: "string"}, minItems: 2}, typescriptBodies: {type: "array", items: {type: "string"}, minItems: 2} } }, owner)).toBe(true);
  for (const method of owner.semanticMethods) expect(semanticRust).toContain(`pub fn ${method}(`);
  console.log("[DEBUG] Directory neutral IO owner separates binary framing from pure checkpoint admission; independent crypto framing corpus retained");
});
