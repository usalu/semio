// #region Header
/**
 * 🧪️ Hub e2e — boots the real `os-hub` binary and drives it with two independent
 * language-neutral contracts and boots the real binary for strict rejection checks against the
 * removed bearer-in-URL WebSocket admission surface.
 *
 * Gated behind `HUB_E2E=1` — it compiles/boots a real server, so the default `bun repo run
 * os-hub-ts:test` must stay fast; without the env var this test reports as skipped in well under
 * a second. Run it for real: `HUB_E2E=1 bun repo run os-hub-ts:test` (`📜️script.ts`'s `TestScript`
 * builds the binary first, default cargo features only — contract-freeze Amendment 2, never
 * `--all-features`).
 */
// #endregion Header

import { createHash, webcrypto } from "node:crypto";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { decodeClientFrame } from "@semio-tech/framework-replication";
import { describe, expect, it } from "vitest";
import { getWorkspaceRoot, startHub } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { scopeExport } from "./🧰️hub-integration-fixtures/🟦️.ts";
import { registerBootstrapAuthAndTrustedCatalogTests } from "./🔐️bootstrap-auth-and-trusted-catalog/🟦️.ts";
import { registerCheckpointAndAuthorityAdapterTests } from "./🧾️checkpoint-and-authority-adapter/🟦️.ts";
import { registerArtifactChunkCasBoundaryTests } from "./📦️artifact-chunk-cas-boundaries/🟦️.ts";
import { registerLagRebootstrapAndLoopbackPortTests } from "./🔁️lag-rebootstrap-and-loopback-ports/🟦️.ts";

const HUB_E2E = process.env.HUB_E2E === "1";
const TEST_TIMEOUT_MS = 240_000;

describe("canonical checkpoint pair neutral contract", () => {
  it("validates the schema and independently proves hashes, framing, order, terminal, and ETag", async () => {
    const root = getWorkspaceRoot();
    const fixtureRoot = join(root, "🌎️hub", "🛰️lag-rebootstrap", "🧪️tests/🧫️fixtures", "🪢️canonical-pair");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const lagExport = scopeExport(root, "hub.lag-rebootstrap");
    const validateSelection = lagExport("CanonicalCheckpointPairSelectionV1");
    const validateBaseline = lagExport("CanonicalCheckpointPairBaselineV1");
    const validateBlob = lagExport("CanonicalCheckpointPairBlobV1");
    const validateLimits = lagExport("CanonicalCheckpointPairLimitsV1");
    expect(validateSelection(fixture.selection), "validateSelection rejected its own fixture").toBe(true);
    expect(validateLimits(fixture.limits), "validateLimits rejected its own fixture").toBe(true);
    for (const part of [fixture.pack, fixture.spr]) expect(validateBlob({ length: part.length, sha256: part.sha256 }), "validateBlob rejected its own fixture").toBe(true);
    expect(fixture.frontierCases.length).toBe(10);
    expect(new Set(fixture.frontierCases.map((row: { id: string }) => row.id)).size).toBe(10);
    for (const { id, accepted: _accepted, ...frontier } of fixture.frontierCases)
      expect(validateBaseline(frontier), `${id}: baseline frontier rejected`).toBe(true);
    expect(fixture.hostile.length).toBe(5);
    expect(new Set(fixture.hostile.map((row: { id: string }) => row.id)).size).toBe(5);
    for (const entry of fixture.hostile as readonly { id: string; member: string; mutation: Record<string, unknown>; stage: string; result: string; code: string }[]) {
      expect([entry.stage, entry.result]).toEqual(["contract", "rejected"]);
      expect(["selection", "selection.baseline"]).toContain(entry.member);
      const mutated = entry.member === "selection"
        ? { ...fixture.selection, ...entry.mutation }
        : { ...fixture.selection, baseline: { ...fixture.selection.baseline, ...entry.mutation } };
      expect(validateSelection(mutated), `${entry.id}/${entry.code}`).toBe(false);
    }
    const exactFrontier = (row: { documentId: string; headEditOrdinal: number; headEditId: string; lastCommitSeq: number; chainHash: string }): boolean => {
      const scopeBound = row.documentId === fixture.selection.documentId;
      const zeroChain = row.chainHash === "0".repeat(64);
      return scopeBound && ((row.headEditOrdinal === 0 && row.headEditId === "" && row.lastCommitSeq === 0 && zeroChain)
        || (Number.isSafeInteger(row.headEditOrdinal) && row.headEditOrdinal > 0 && row.headEditId.length > 0 && Number.isSafeInteger(row.lastCommitSeq) && row.lastCommitSeq > 0 && /^[0-9a-f]{64}$/u.test(row.chainHash) && !zeroChain));
    };
    for (const row of fixture.frontierCases) expect(exactFrontier(row), row.id).toBe(row.accepted);

    const generated = (part: { length: number; multiplier: number; increment: number }): Buffer =>
      Buffer.from(Array.from({ length: part.length }, (_, index) => (index * part.multiplier + part.increment) % 256));
    const pack = generated(fixture.pack);
    const spr = generated(fixture.spr);
    const digest = async (bytes: Uint8Array): Promise<string> => Buffer.from(await webcrypto.subtle.digest("SHA-256", bytes)).toString("hex");
    expect(await digest(pack)).toBe(fixture.pack.sha256);
    expect(await digest(spr)).toBe(fixture.spr.sha256);
    expect(await digest(Buffer.concat([pack, spr]))).toBe(fixture.expected.aggregateSha256);
    expect(createHash("sha256").update(pack).digest("hex")).toBe(fixture.pack.sha256);

    const u32 = (value: number): Buffer => { const bytes = Buffer.alloc(4); bytes.writeUInt32BE(value); return bytes; };
    const u64 = (value: number): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64BE(BigInt(value)); return bytes; };
    const field = (bytes: Uint8Array): Buffer => Buffer.concat([u32(bytes.byteLength), bytes]);
    const hex = (value: string): Buffer => Buffer.from(value, "hex");
    const text = (value: string): Buffer => Buffer.from(value, "utf8");
    const selection = fixture.selection;
    const header = Buffer.concat([
      Buffer.from([1]), u32(1), field(text(selection.spaceId)), field(text(selection.documentId)), hex(selection.descriptorDigest), hex(selection.checkpointId),
      field(text(selection.baseline.documentId)), u64(selection.baseline.headEditOrdinal), field(text(selection.baseline.headEditId)), u64(selection.baseline.lastCommitSeq), hex(selection.baseline.chainHash),
      hex(fixture.pack.sha256), u64(fixture.pack.length), hex(fixture.spr.sha256), u64(fixture.spr.length), hex(fixture.expected.aggregateSha256),
    ]);
    expect(header.byteLength).toBe(fixture.expected.headerPayloadBytes);
    const frame = (payload: Uint8Array): Buffer => Buffer.concat([u32(payload.byteLength), payload]);
    const records: Buffer[] = [];
    let ordinal = 0;
    for (const [part, bytes] of [[1, pack], [2, spr]] as const) {
      for (let offset = 0; offset < bytes.byteLength; offset += fixture.limits.recordBytes) {
        const data = bytes.subarray(offset, offset + fixture.limits.recordBytes);
        records.push(frame(Buffer.concat([Buffer.from([2, part]), u32(ordinal++), u64(offset), field(data)])));
      }
    }
    const wire = Buffer.concat([frame(header), ...records, frame(Buffer.from([3, 0]))]);
    expect(ordinal).toBe(fixture.expected.dataRecords);
    expect(wire.byteLength).toBe(fixture.expected.wireBytes);
    expect(`"${createHash("sha256").update("semio.hub.canonical-checkpoint-pair-etag.v1\0").update(header).digest("hex")}"`).toBe(fixture.expected.etag);

    let position = 0;
    const next = (): Buffer => { const length = wire.readUInt32BE(position); position += 4; const payload = wire.subarray(position, position + length); position += length; return payload; };
    expect(next().equals(header)).toBe(true);
    let packOffset = 0;
    let sprOffset = 0;
    for (let expectedOrdinal = 0; expectedOrdinal < ordinal; expectedOrdinal++) {
      const record = next();
      expect(record[0]).toBe(2);
      expect(record.readUInt32BE(2)).toBe(expectedOrdinal);
      const part = record[1];
      const offset = Number(record.readBigUInt64BE(6));
      const length = record.readUInt32BE(14);
      expect(length).toBeGreaterThan(0);
      expect(length).toBeLessThanOrEqual(fixture.limits.recordBytes);
      if (part === 1) { expect(offset).toBe(packOffset); packOffset += length; }
      else { expect(part).toBe(2); expect(packOffset).toBe(pack.byteLength); expect(offset).toBe(sprOffset); sprOffset += length; }
    }
    expect(next().equals(Buffer.from([3, 0]))).toBe(true);
    expect(position).toBe(wire.byteLength);
    expect([packOffset, sprOffset]).toEqual([pack.byteLength, spr.byteLength]);
  });
});

describe("hub harness quick contract", () => {
  registerBootstrapAuthAndTrustedCatalogTests();
  registerCheckpointAndAuthorityAdapterTests();
  registerArtifactChunkCasBoundaryTests();
  registerLagRebootstrapAndLoopbackPortTests();
});

describe("removed legacy WebSocket admission", () => {
  it("terminally rejects the removed tag-zero bearer Hello frame", () => {
    expect(() => decodeClientFrame(new Uint8Array([0, 0]))).toThrow(/unknown tag 0/);
  });

  it.skipIf(!HUB_E2E)("boots the real hub with only v1 socket routes", async () => {
    const dataDir = mkdtempSync(join(tmpdir(), "os-hub-socket-v1-only-"));
    const hub = await startHub({ repoRoot: getWorkspaceRoot(), dataDir, adminToken: "e2e-admin" });
    try {
      const legacyDirectory = await fetch(`${hub.baseUrl}/directory/ws`);
      const legacyDocument = await fetch(`${hub.baseUrl}/spaces/legacy/documents/index/ws`);
      expect(legacyDirectory.status).toBe(404);
      expect(legacyDocument.status).toBe(404);
    } finally {
      await hub.stop();
      rmSync(dataDir, { recursive: true, force: true });
    }
  }, TEST_TIMEOUT_MS);
});
