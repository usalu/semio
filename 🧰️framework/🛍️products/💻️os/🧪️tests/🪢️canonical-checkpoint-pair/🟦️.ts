/** 🪢️ The hub's canonical checkpoint pair — the one seed a native, a wasm32 and a React shell open a hub document on —
 * replayed from the language-agnostic fixture `🧫️fixtures/📇️directory/🪢️canonical-checkpoint-pair-v1.json`, the same
 * rows the kernel's Rust law (`📇️directory/🧬️schema/🪢️canonical-checkpoint-pair-v1`) replays. Ajv holds the fixture to
 * its schema; `node:crypto` is the independent digest oracle (the TypeScript decoder leaves digests to the bootstrap
 * assembler, so this runner verifies them the way the Rust decoder does). */
import { describe, expect, it } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { semioSchemaAjvV1 } from "../🧬️schema-oracle/🟦️.ts";
import {
  CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES,
  CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES,
  CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS,
  CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1,
  CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES,
  admitCanonicalCheckpointPairV1,
  decodeCanonicalCheckpointPairV1,
  type CanonicalCheckpointPairV1,
} from "../../🔨️modules/📇️directory/🧬️schema/🟦️.ts";

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
  it("the fixture satisfies its schema", () => {
    const validate = semioSchemaAjvV1({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("the fixture limits are the TypeScript wire constants", () => {
    expect([fixture.mediaType, fixture.limits.headerBytes, fixture.limits.recordBytes, fixture.limits.records, fixture.limits.pairBytes]).toEqual([
      CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1,
      CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES,
      CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES,
      CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS,
      CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES,
    ]);
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

  for (const admission of fixture.admissions) {
    it(`${admission.id} is ${admission.refusal ?? "admitted"}`, () => {
      const pair = fixture.pairs.find((candidate: { id: string }) => candidate.id === admission.pair);
      const decoded = decodeCanonicalCheckpointPairV1(hexBytes(pair.bodyHex));
      const admit = () => admitCanonicalCheckpointPairV1(decoded, admission.scope, admission.expected);
      if (admission.refusal === null) expect(admit).not.toThrow();
      else expect(admit).toThrow(admission.refusal);
    });
  }
});
