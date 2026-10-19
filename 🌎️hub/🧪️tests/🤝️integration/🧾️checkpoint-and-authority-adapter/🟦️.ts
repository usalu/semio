/** 🧾️ The checkpoint event and authority adapter vector tests. */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { expect, it } from "vitest";
import { getWorkspaceRoot } from "../../../📦️packages/🟦️typescript/🟦️.ts";

/** 🧮 Registers the checkpoint event, identity and cap, and the authority adapter SHA-256 vector tests. */
export function registerCheckpointAndAuthorityAdapterTests(): void {
  it("validates the neutral checkpoint event, identity, and exact caps with AJV and Node crypto", () => {
    const root = getWorkspaceRoot();
    const fixture = JSON.parse(readFileSync(join(root, "🌎️hub", "📇️directory", "🧪️tests/🧫️fixtures", "📸️artifact-checkpoint-projection", "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(root, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: false, discriminator: true });
    ajv.addSchema(schema);
    const validateEvent = ajv.compile({ $ref: `${schema.$id}#/$defs/DirectoryEventBody` });
    const event = { kind: "artifact.checkpoint-published", checkpoint: fixture.checkpoint1 };
    expect(validateEvent(event), JSON.stringify(validateEvent.errors)).toBe(true);
    expect(JSON.stringify(event)).not.toContain("storageKey");
    const leaked = structuredClone(event) as typeof event & { checkpoint: { pack: Record<string, unknown> } };
    leaked.checkpoint.pack.storageKey = "private";
    expect(validateEvent(leaked)).toBe(false);

    const u64 = (value: number): Buffer => {
      const bytes = Buffer.alloc(8);
      bytes.writeBigUInt64BE(BigInt(value));
      return bytes;
    };
    const field = (bytes: Uint8Array): Buffer => Buffer.concat([u64(bytes.byteLength), Buffer.from(bytes)]);
    const version = Buffer.alloc(4);
    version.writeUInt32BE(fixture.descriptor.bootstrapVersion);
    const d = fixture.descriptor;
    const descriptorEncoding = Buffer.concat([
      Buffer.from("semio.document-descriptor.digest.v1\0"),
      ...[d.spaceId, d.documentId, d.artifactKind, d.artifactSchema, d.owner.pluginId, d.owner.packageId, d.owner.version].map(value => field(Buffer.from(value))),
      ...[d.owner.packageHash, d.packSchemaHash].map(value => field(Buffer.from(value, "hex"))),
      field(version), ...[d.bootstrapFrontier.headSeq, d.bootstrapFrontier.commitSeq, d.bootstrapFrontier.epoch].map(value => field(u64(value))),
      field(Buffer.from(d.bootstrapSnapshotHash, "hex")),
    ]);
    const descriptorDigest = [...createHash("sha256").update(descriptorEncoding).digest()];
    const identity = (checkpoint: any): Buffer => Buffer.concat([
      Buffer.from(checkpoint.baselineFrontier.headEditOrdinal === 0 ? "semio.hub.artifact-genesis.v1\0" : "semio.hub.artifact-checkpoint.v1\0"),
      field(Buffer.from(checkpoint.scope.spaceId)),
      field(Buffer.from(checkpoint.scope.documentId)),
      field(Uint8Array.from(checkpoint.parentCheckpointId ?? [])),
      field(Uint8Array.from(checkpoint.descriptorDigestV1)),
      field(Buffer.from(checkpoint.baselineFrontier.documentId)),
      field(u64(checkpoint.baselineFrontier.headEditOrdinal)),
      field(Buffer.from(checkpoint.baselineFrontier.headEditId)),
      field(u64(checkpoint.baselineFrontier.lastCommitSeq)),
      field(Uint8Array.from(checkpoint.baselineFrontier.chainHash)),
      field(Uint8Array.from(checkpoint.pack.sha256)),
      field(u64(checkpoint.pack.byteLength)),
      field(Uint8Array.from(checkpoint.spr.sha256)),
      field(u64(checkpoint.spr.byteLength)),
      field(Uint8Array.from(checkpoint.aggregateSha256)),
    ]);
    for (const checkpoint of [fixture.genesis, fixture.checkpoint1, fixture.checkpoint2]) {
      expect(checkpoint.descriptorDigestV1).toEqual(descriptorDigest);
      expect(validateEvent({ kind: "artifact.checkpoint-published", checkpoint }), JSON.stringify(validateEvent.errors)).toBe(true);
      expect([...createHash("sha256").update(identity(checkpoint)).digest()]).toEqual(checkpoint.checkpointId);
    }
    for (const kind of ["pack", "spr"] as const) {
      expect([...createHash("sha256").update(Uint8Array.from(fixture.genesisPair[kind])).digest()]).toEqual(fixture.genesis[kind].sha256);
      expect(fixture.genesis[kind].byteLength).toBe(fixture.genesisPair[kind].length);
    }
    expect(fixture.descriptor.bootstrapFrontier).toEqual({ headSeq: 0, commitSeq: 0, epoch: 0 });
    expect(fixture.descriptor.bootstrapSnapshotHash).toBe(Buffer.from(fixture.genesis.pack.sha256).toString("hex"));
    expect(fixture.genesis.baselineFrontier).toEqual({ documentId: fixture.descriptor.documentId, headEditOrdinal: 0, headEditId: "", lastCommitSeq: 0, chainHash: Array(32).fill(0) });
    expect(fixture.genesis.parentCheckpointId).toBeUndefined();
    expect(fixture.checkpoint1.parentCheckpointId).toEqual(fixture.genesis.checkpointId);
    expect(fixture.checkpoint2.parentCheckpointId).toEqual(fixture.checkpoint1.checkpointId);
    expect(validateEvent({ kind: "document.indexed", scope: fixture.genesis.scope, descriptorDigestV1: descriptorDigest, entry: fixture.indexEntry }), JSON.stringify(validateEvent.errors)).toBe(true);
    expect(Number.isSafeInteger(fixture.wireIntegerMaximum)).toBe(true);
    expect(Number.isSafeInteger(fixture.wireIntegerMaximum + 1)).toBe(false);
    expect(Buffer.byteLength("x".repeat(fixture.privateLocatorMaximumBytes))).toBe(4096);
    expect(Buffer.byteLength("x".repeat(fixture.privateLocatorMaximumBytes + 1))).toBe(4097);
    expect(fixture.lineageMaximum).toBe(16_384);
    expect(fixture.eventReadMaximum).toBe(10_000);
  });
  it("recomputes the language-neutral authority adapter SHA-256 vector with Node crypto", () => {
    const path = join(getWorkspaceRoot(), "🌎️hub", "🗿️artifact-authority", "🧪️tests/🧫️fixtures", "🔌️authority-adapter", "🔣️.json");
    const fixture = JSON.parse(readFileSync(path, "utf8")) as {
      identity: { pluginId: string; packageId: string };
      pack: number[];
      spr: number[];
      packSha256: string;
      sprSha256: string;
      aggregateSha256: string;
      expectedPublisherCallsOnAnyPrepublicationFailure: number;
    };
    const pack = Uint8Array.from(fixture.pack);
    const spr = Uint8Array.from(fixture.spr);
    const sha256 = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
    expect(fixture.identity.pluginId).not.toBe(fixture.identity.packageId);
    expect(sha256(pack)).toBe(fixture.packSha256);
    expect(sha256(spr)).toBe(fixture.sprSha256);
    expect(sha256(Uint8Array.from([...pack, ...spr]))).toBe(fixture.aggregateSha256);
    expect(fixture.expectedPublisherCallsOnAnyPrepublicationFailure).toBe(0);
  });
}
