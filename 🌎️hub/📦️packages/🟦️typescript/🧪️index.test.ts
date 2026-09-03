// #region Header
/**
 * 🧪️ Hub e2e — boots the real `os-hub` binary and drives it with two independent
 * `DirectoryClient`s plus raw document-WS wire frames to prove the hub's collaboration contract
 * (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0-C2, lane 3-E): studio
 * creation + visibility, live membership over `/directory/ws`, surface-scoped presence vs.
 * document-scoped command relay on `/spaces/{space}/documents/index/ws`, an admin kick, and
 * survival of a real hub restart against the same `OS_HUB_DATA`.
 *
 * Gated behind `HUB_E2E=1` — it compiles/boots a real server, so the default `bun nx run
 * os-hub-ts:test` must stay fast; without the env var this test reports as skipped in well under
 * a second. Run it for real: `HUB_E2E=1 bun nx run os-hub-ts:test` (`📜️script.ts`'s `TestScript`
 * builds the binary first, default cargo features only — contract-freeze Amendment 2, never
 * `--all-features`).
 */
// #endregion Header

import { createHash, createHmac, webcrypto } from "node:crypto";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { createServer } from "node:http";
import { tmpdir } from "node:os";
import { join } from "node:path";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import {
  type ArtifactPresencePeer,
  type ClientFrame,
  type ConnectionView,
  type DirectoryStreamMessage,
  type DocumentDescriptor,
  type ServerFrame,
  DirectoryClient,
  decodePresencePeer,
  decodeServerFrame,
  encodeClientFrame,
  encodePresencePeer,
  encodeServerFrame,
  isDirectoryEventBodyKind,
  isDirectoryStreamMessageKind,
} from "@semio-tech/framework-os";
import { describe, expect, it } from "vitest";
import { type HubHandle, findFreePort, getWorkspaceRoot, resolveHubBinaryPath, startHub, waitForHttpReady } from "./🟦️.ts";
import { parseInviteCapabilityV1, parseSessionCapabilityV1, parseShareCapabilityV1, parseSocketGrantCapabilityV1 } from "../../🔐️auth/🧬️schema/🟦️.ts";

const HUB_E2E = process.env.HUB_E2E === "1";
const TEST_TIMEOUT_MS = 240_000;
const EDITOR_SURFACE = "s.space.space@1/*#editor";
const VIEWER_SURFACE = "s.space.space@1/*#viewer";

describe("canonical checkpoint pair neutral contract", () => {
  it("validates the schema and independently proves hashes, framing, order, terminal, and ETag", async () => {
    const root = getWorkspaceRoot();
    const fixtureRoot = join(root, "🌎️hub", "🛰️lag-rebootstrap", "🧪️fixtures", "🧬️canonical-pair");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(fixtureRoot, "🧬️.schema.json"), "utf8"));
    const validate = new Ajv2020({ strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, locator: "private" })).toBe(false);

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
  it("validates local bootstrap, one-shot credential, and readiness schemas with independent HMAC", () => {
    const root = getWorkspaceRoot();
    const contractRoot = join(root, "🌎️hub", "🔐️local-bootstrap");
    const fixture = JSON.parse(readFileSync(join(contractRoot, "🧪️fixtures", "🧬️pipe-v1", "🔣️.json"), "utf8"));
    const schemas = [
      ["🚇️pipe-v1", fixture.initialize, fixture.hello, fixture.issue],
      ["📨️credential-envelope-v1", fixture.credential],
      ["🩺️readiness-v1", fixture.ready, fixture.bootstrapReadyButArtifactUnavailable, fixture.notReady],
    ] as const;
    for (const [schemaName, ...values] of schemas) {
      const schema = JSON.parse(readFileSync(join(contractRoot, "🧬️schema", schemaName, "🔣️.json"), "utf8"));
      const validate = new Ajv2020({ strict: true }).compile(schema);
      for (const value of values) expect(validate(value), JSON.stringify(validate.errors)).toBe(true);
    }
    const validatePipe = new Ajv2020({ strict: true }).compile(JSON.parse(readFileSync(join(contractRoot, "🧬️schema", "🚇️pipe-v1", "🔣️.json"), "utf8")));
    const oversizedDevice = structuredClone(fixture.issue);
    oversizedDevice.deviceInstanceId = "d".repeat(fixture.limits.deviceIdentityBytesMax + 1);
    expect(validatePipe(oversizedDevice)).toBe(false);
    const oversizedProfiles = structuredClone(fixture.initialize);
    oversizedProfiles.profiles = Array.from({ length: fixture.limits.profilesMax + 1 }, (_, index) => ({
      ...fixture.initialize.profiles[0],
      profileId: `developer-${index}`,
    }));
    expect(validatePipe(oversizedProfiles)).toBe(false);
    const unknownField = { ...fixture.issue, assertedEmail: "attacker@example.invalid" };
    expect(validatePipe(unknownField)).toBe(false);
    const validateReadiness = new Ajv2020({ strict: true }).compile(JSON.parse(readFileSync(join(contractRoot, "🧬️schema", "🩺️readiness-v1", "🔣️.json"), "utf8")));
    expect(validateReadiness({ ...fixture.ready, capability: fixture.credential.capability })).toBe(false);
    expect(validateReadiness({ ...fixture.ready, sessionKind: fixture.credential.sessionKind })).toBe(false);
    expect(validateReadiness({ ...fixture.ready, authorizationGeneration: fixture.credential.authorizationGeneration })).toBe(false);
    expect(validateReadiness({ ...fixture.ready, artifactAuthority: { ready: false } })).toBe(false);
    expect(validateReadiness({ ...fixture.bootstrapReadyButArtifactUnavailable, status: "ready" })).toBe(false);
    const validateCredential = new Ajv2020({ strict: true }).compile(JSON.parse(readFileSync(join(contractRoot, "🧬️schema", "📨️credential-envelope-v1", "🔣️.json"), "utf8")));
    expect(validateCredential({ ...fixture.credential, sessionKind: "external" })).toBe(false);
    expect(validateCredential({ ...fixture.credential, authorizationGeneration: 0 })).toBe(false);
    const aggregateReady = (value: any): boolean => value.authentication.bootstrapReady === true && value.directory.ready === true && value.storage.ready === true && value.artifactAuthority.ready === true && value.adminAssets.ready === true;
    expect(aggregateReady(fixture.ready)).toBe(true);
    expect(fixture.ready.status).toBe("ready");
    expect(aggregateReady(fixture.bootstrapReadyButArtifactUnavailable)).toBe(false);
    expect(fixture.bootstrapReadyButArtifactUnavailable.status).toBe("not-ready");
    expect(fixture.bootstrapReadyButArtifactUnavailable.authentication.bootstrapReady).toBe(true);
    expect(fixture.bootstrapReadyButArtifactUnavailable.artifactAuthority.ready).toBe(false);
    const key = Buffer.from(fixture.channelKey, "hex");
    const proof = (value: object): string => {
      const canonical = Buffer.from(JSON.stringify(value));
      const length = Buffer.alloc(4);
      length.writeUInt32BE(canonical.length);
      return createHmac("sha256", key).update("semio/hub/local-bootstrap/v1\0").update(length).update(canonical).digest("hex");
    };
    expect(proof(fixture.helloWithoutProof)).toBe(fixture.hello.proof);
    expect(proof(fixture.issueWithoutProof)).toBe(fixture.issue.proof);
    expect(proof(fixture.credentialWithoutProof)).toBe(fixture.credential.proof);
    expect(fixture.hostile.wrongProof).not.toBe(fixture.hello.proof);
    expect(fixture.hostile.replayedSequence).toBe(fixture.hello.sequence);
    expect(fixture.limits.frameBytesMaxPlusOne).toBe(fixture.limits.frameBytesMax + 1);
    expect(Buffer.byteLength("x".repeat(fixture.limits.deviceIdentityBytesMax + 1))).toBe(129);
    const publicBodies = JSON.stringify([fixture.ready, fixture.bootstrapReadyButArtifactUnavailable, fixture.notReady]);
    expect(publicBodies).not.toContain(fixture.channelKey);
    expect(publicBodies).not.toContain(fixture.credential.capability);
    expect(publicBodies).not.toContain(fixture.initialize.profiles[0].subject);
    expect(publicBodies).not.toContain("sessionKind");
    expect(publicBodies).not.toContain("authorizationGeneration");
  });

  it("validates typed auth capabilities and independently recomputes the socket grant with AJV and WebCrypto", async () => {
    const root = getWorkspaceRoot();
    const fixture = JSON.parse(readFileSync(join(root, "🌎️hub", "🔐️auth", "🧪️fixtures", "🧬️capability-v1", "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(root, "🌎️hub", "🔐️auth", "🧬️schema", "🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const digest = (domain: string, secretHex: string): string => createHash("sha256").update(`semio/hub/${domain}/v1\0`).update(Buffer.from(secretHex, "hex")).digest("hex");
    for (const kind of ["session", "share", "invite"] as const) {
      expect(digest(kind, fixture[kind].secretHex)).toBe(fixture[kind].digestHex);
      expect(fixture[kind].capability.split(".")[2]).toBe(fixture[kind].selector);
    }
    expect(parseSessionCapabilityV1(fixture.session.capability)).toBe(fixture.session.capability);
    expect(parseShareCapabilityV1(fixture.share.capability)).toBe(fixture.share.capability);
    expect(parseInviteCapabilityV1(fixture.invite.capability)).toBe(fixture.invite.capability);
    expect(parseSocketGrantCapabilityV1(fixture.socket.capability)).toBe(fixture.socket.capability);
    expect(fixture.socket.capability).toHaveLength(107);
    const socketDomain = new TextEncoder().encode("semio/hub/socket/v1\0");
    const socketSecret = Buffer.from(fixture.socket.secretHex, "hex");
    const socketDigestInput = new Uint8Array(socketDomain.length + socketSecret.length);
    socketDigestInput.set(socketDomain);
    socketDigestInput.set(socketSecret, socketDomain.length);
    expect(Buffer.from(await webcrypto.subtle.digest("SHA-256", socketDigestInput)).toString("hex")).toBe(fixture.socket.digestHex);
    for (const hostile of fixture.socket.rejectedCapabilities.slice(0, 4)) expect(() => parseSocketGrantCapabilityV1(hostile)).toThrow();
    const wrongSecret = parseSocketGrantCapabilityV1(fixture.socket.rejectedCapabilities[4]);
    const wrongSecretBytes = Buffer.from(wrongSecret.split(".")[3], "hex");
    const wrongDigestInput = new Uint8Array(socketDomain.length + wrongSecretBytes.length);
    wrongDigestInput.set(socketDomain);
    wrongDigestInput.set(wrongSecretBytes, socketDomain.length);
    expect(Buffer.from(await webcrypto.subtle.digest("SHA-256", wrongDigestInput)).toString("hex")).not.toBe(fixture.socket.digestHex);
    expect(Object.keys(fixture.socket.receipt).sort()).toEqual(["actorId", "expiresAtMs", "grant", "protocol", "schema"]);
    expect(fixture.socket.receipt.grant).toBe(fixture.socket.capability);
    const publicReceiptWithoutGrant = JSON.stringify({ ...fixture.socket.receipt, grant: "[REDACTED]" });
    expect(publicReceiptWithoutGrant).not.toContain(fixture.socket.secretHex);
    expect(publicReceiptWithoutGrant).not.toContain(fixture.socket.digestHex);
    expect(() => parseSessionCapabilityV1(fixture.share.capability)).toThrow();
    expect(() => parseShareCapabilityV1(fixture.share.capability.toUpperCase())).toThrow();
    const u32 = (value: number): Buffer => {
      const bytes = Buffer.alloc(4);
      bytes.writeUInt32BE(value);
      return bytes;
    };
    const provider = Buffer.from(fixture.identity.provider);
    const subject = Buffer.from(fixture.identity.subject);
    expect(createHash("sha256").update("semio/hub/identity-subject/v1\0").update(u32(provider.length)).update(provider).update(u32(subject.length)).update(subject).digest("hex")).toBe(fixture.identity.digestHex);
    expect(fixture.revocation.nextGeneration).toBe(fixture.revocation.previousGeneration + 1);
    const audit = JSON.stringify(fixture.audit);
    expect(audit).not.toContain(fixture.session.secretHex);
    expect(audit).not.toContain(fixture.session.capability);
    expect(audit).not.toContain("@");
    expect(audit).not.toMatch(/\b(?:\d{1,3}\.){3}\d{1,3}\b/);
    expect(fixture.limits.ttlSecondsMax + 1).toBe(31_536_001);
    expect(Buffer.alloc(fixture.limits.deviceIdentityBytesMax + 1).length).toBe(129);
    expect(Buffer.alloc(fixture.limits.assertionBytesMax + 1).length).toBe(16_385);
  });

  it("validates the neutral immutable trusted-catalog bundle with AJV and Node crypto", () => {
    const root = getWorkspaceRoot();
    const catalogRoot = join(root, "🌎️hub", "🗿️artifact-authority", "🗂️trusted-catalog");
    const fixture = JSON.parse(readFileSync(join(catalogRoot, "🧪️fixtures", "🧬️two-package", "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(catalogRoot, "🧬️schema", "🔣️bundle.schema.json"), "utf8"));
    const validate = new Ajv2020({ strict: true }).compile(schema);
    expect(validate(fixture.bundle), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.bundle.packages[0].pluginId).not.toBe(fixture.bundle.packages[0].packageId);

    const component = Buffer.from(fixture.componentHex, "hex");
    expect(createHash("sha256").update(component).digest("hex")).toBe(fixture.componentSha256);
    const mutated = Buffer.from(component);
    mutated[mutated.length - 1] ^= 1;
    expect(createHash("sha256").update(mutated).digest("hex")).not.toBe(fixture.componentSha256);
    expect(createHash("sha256").update(Buffer.from([1])).digest("hex")).toBe(fixture.bundle.packages[0].descriptor.sha256);

    const dependencyFirst = (bundle: any, profileId: string): string[] => {
      const packages = new Map<string, any>();
      const packageIds = new Set<string>();
      for (const entry of bundle.packages) {
        if (packages.has(entry.pluginId) || packageIds.has(entry.packageId)) throw new Error("duplicate independent identity");
        packages.set(entry.pluginId, entry);
        packageIds.add(entry.packageId);
      }
      const profile = bundle.profiles.find((entry: any) => entry.id === profileId);
      if (!profile) throw new Error("missing profile");
      const visiting = new Set<string>();
      const visited = new Set<string>();
      const order: string[] = [];
      const visit = (required: any): void => {
        const entry = packages.get(required.pluginId);
        if (!entry || entry.packageId !== required.packageId || entry.version !== required.version) throw new Error("incomplete or conflicting closure");
        if (visiting.has(entry.pluginId)) throw new Error("dependency cycle");
        if (visited.has(entry.pluginId)) return;
        visiting.add(entry.pluginId);
        for (const dependency of entry.dependencies) visit(dependency);
        visiting.delete(entry.pluginId);
        visited.add(entry.pluginId);
        order.push(entry.pluginId);
      };
      for (const required of profile.roots) visit(required);
      return order;
    };
    expect(dependencyFirst(fixture.bundle, "fixture")).toEqual(["fixture.base", "fixture.editor"]);
    const incomplete = structuredClone(fixture.bundle);
    incomplete.packages.pop();
    expect(() => dependencyFirst(incomplete, "fixture")).toThrow("incomplete");
    const lossy = structuredClone(fixture.bundle);
    lossy.profiles[0].roots[0].packageId = lossy.profiles[0].roots[0].pluginId;
    expect(() => dependencyFirst(lossy, "fixture")).toThrow("conflicting");

    const componentMaximum = structuredClone(fixture.bundle);
    componentMaximum.packages[0].component.byteLength = fixture.limits.componentBytesMax;
    expect(validate(componentMaximum), JSON.stringify(validate.errors)).toBe(true);
    componentMaximum.packages[0].component.byteLength = fixture.limits.componentBytesMaxPlusOne;
    expect(validate(componentMaximum)).toBe(false);
    const descriptorMaximum = structuredClone(fixture.bundle);
    descriptorMaximum.packages[0].descriptor.byteLength = fixture.limits.descriptorBytesMax;
    expect(validate(descriptorMaximum), JSON.stringify(validate.errors)).toBe(true);
    descriptorMaximum.packages[0].descriptor.byteLength = fixture.limits.descriptorBytesMaxPlusOne;
    expect(validate(descriptorMaximum)).toBe(false);
    const identityMaximum = structuredClone(fixture.bundle);
    identityMaximum.packages[0].pluginId = "a".repeat(fixture.limits.identityBytesMax);
    expect(validate(identityMaximum), JSON.stringify(validate.errors)).toBe(true);
    identityMaximum.packages[0].pluginId = "a".repeat(fixture.limits.identityBytesMaxPlusOne);
    expect(validate(identityMaximum)).toBe(false);
    const codecsMaximum = structuredClone(fixture.bundle);
    codecsMaximum.packages[0].nativeCodecs = Array.from({ length: fixture.limits.codecCountMax }, (_, index) => ({ artifactKind: `fixture.kind.${index}`, artifactSchema: `fixture.schema.${index}`, packSchemaHash: "11".repeat(32) }));
    expect(validate(codecsMaximum), JSON.stringify(validate.errors)).toBe(true);
    codecsMaximum.packages[0].nativeCodecs.push({ artifactKind: "fixture.kind.overflow", artifactSchema: "fixture.schema.overflow", packSchemaHash: "11".repeat(32) });
    expect(validate(codecsMaximum)).toBe(false);
    const zeroHash = structuredClone(fixture.bundle);
    zeroHash.packages[0].nativeCodecs[0].packSchemaHash = "00".repeat(32);
    expect(validate(zeroHash)).toBe(false);
  });

  it("validates the neutral checkpoint event, identity, and exact caps with AJV and Node crypto", () => {
    const root = getWorkspaceRoot();
    const fixture = JSON.parse(readFileSync(join(root, "🌎️hub", "📇️directory", "🧪️tests", "🔣️artifact-checkpoint-projection.json"), "utf8"));
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
    const identity = (checkpoint: any): Buffer => Buffer.concat([
      Buffer.from("semio.hub.artifact-checkpoint.v1\0"),
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
    for (const checkpoint of [fixture.checkpoint1, fixture.checkpoint2]) {
      expect([...createHash("sha256").update(identity(checkpoint)).digest()]).toEqual(checkpoint.checkpointId);
    }
    expect(Number.isSafeInteger(fixture.wireIntegerMaximum)).toBe(true);
    expect(Number.isSafeInteger(fixture.wireIntegerMaximum + 1)).toBe(false);
    expect(Buffer.byteLength("x".repeat(fixture.privateLocatorMaximumBytes))).toBe(4096);
    expect(Buffer.byteLength("x".repeat(fixture.privateLocatorMaximumBytes + 1))).toBe(4097);
    expect(fixture.lineageMaximum).toBe(16_384);
    expect(fixture.eventReadMaximum).toBe(10_000);
  });

  it("recomputes the language-neutral authority adapter SHA-256 vector with Node crypto", () => {
    const path = join(getWorkspaceRoot(), "🌎️hub", "🗿️artifact-authority", "🧪️fixtures", "🧬️authority-adapter", "🔣️.json");
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

  it("validates and independently derives every artifact chunk-CAS boundary with AJV and WebCrypto", async () => {
    const root = getWorkspaceRoot();
    const fixtureRoot = join(root, "🌎️hub", "🗿️artifact-authority", "🧪️fixtures", "🧬️artifact-chunk-cas");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(fixtureRoot, "🧬️schema", "🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const u64 = (value: number): Buffer => {
      const bytes = Buffer.alloc(8);
      bytes.writeBigUInt64BE(BigInt(value));
      return bytes;
    };
    const u32 = (value: number): Buffer => {
      const bytes = Buffer.alloc(4);
      bytes.writeUInt32BE(value);
      return bytes;
    };
    const field = (bytes: Uint8Array): Buffer => Buffer.concat([u64(bytes.byteLength), Buffer.from(bytes)]);
    const sha256 = async (bytes: Uint8Array): Promise<Buffer> => Buffer.from(await webcrypto.subtle.digest("SHA-256", bytes));
    const pattern = (length: number): Buffer => {
      const bytes = Buffer.alloc(length);
      for (let index = 0; index < length; index += 1) bytes[index] = (index * 31 + Math.floor(index / 251)) % 256;
      return bytes;
    };
    const derive = async (length: number) => {
      const raw = pattern(length);
      const rawSha256 = await sha256(raw);
      const chunks = Array.from({ length: Math.ceil(length / fixture.chunkBytes) }, (_, ordinal) => raw.subarray(ordinal * fixture.chunkBytes, Math.min(length, (ordinal + 1) * fixture.chunkBytes)));
      const chunkIds = await Promise.all(chunks.map((chunk) => sha256(Buffer.concat([
        Buffer.from("semio.hub.artifact-cas.chunk.v1\0"),
        field(Buffer.from(fixture.spaceId)),
        field(u64(chunk.length)),
        field(chunk),
      ]))));
      const manifest = Buffer.concat([
        Buffer.from("semio.hub.artifact-cas.manifest.v1\0"),
        field(Buffer.from(fixture.spaceId)),
        field(rawSha256),
        field(u64(length)),
        field(u32(fixture.chunkBytes)),
        field(u32(chunks.length)),
        ...chunks.flatMap((chunk, ordinal) => [field(u32(ordinal)), field(u32(chunk.length)), field(chunkIds[ordinal])]),
      ]);
      return {
        length,
        rawSha256: rawSha256.toString("hex"),
        chunkCount: chunks.length,
        manifestBytes: manifest.length,
        manifestId: (await sha256(manifest)).toString("hex"),
        firstChunkId: chunkIds[0]?.toString("hex") ?? null,
        lastChunkId: chunkIds.at(-1)?.toString("hex") ?? null,
      };
    };
    for (const vector of fixture.vectors) expect(await derive(vector.length)).toEqual(vector);
    const large = await derive(fixture.largePair.pack.length);
    expect(large).toEqual(fixture.largePair.pack);
    expect(large).toEqual(fixture.largePair.spr);
    expect(fixture.largePair.pack.length + fixture.largePair.spr.length).toBe(fixture.maximumRawBytes);
    expect(fixture.vectors.map((vector: { length: number }) => vector.length)).toEqual([0, 1, 262_143, 262_144, 262_145]);

    type Reservation = { expiresAtMs: number; objects: string[] };
    const events = fixture.retentionLedger.events as Array<{ generation: number; operation: "reserve" | "publish" | "retention" | "space-delete"; checkpointId?: string; expiresAtMs?: number; objects?: string[] }>;
    expect(events.map((event) => event.generation)).toEqual([1, 2, 3, 4, 5, 6, 7]);
    const reachabilityAt = (throughGeneration: number, nowMs: number) => {
      const universe = new Set<string>();
      const reservations = new Map<string, Reservation>();
      const references = new Map<string, string[]>();
      for (const event of events.filter((candidate) => candidate.generation <= throughGeneration)) {
        if (event.operation === "reserve") {
          if (!event.checkpointId || event.expiresAtMs === undefined || !event.objects) throw new Error("invalid reserve fixture");
          reservations.set(event.checkpointId, { expiresAtMs: event.expiresAtMs, objects: [...event.objects] });
          event.objects.forEach((object) => universe.add(object));
        } else if (event.operation === "publish") {
          if (!event.checkpointId) throw new Error("invalid publish fixture");
          const reservation = reservations.get(event.checkpointId);
          if (!reservation) throw new Error("publish without reservation");
          references.set(event.checkpointId, reservation.objects);
          reservations.delete(event.checkpointId);
        } else if (event.operation === "retention") {
          if (!event.checkpointId) throw new Error("invalid retention fixture");
          for (const checkpointId of references.keys()) if (checkpointId !== event.checkpointId) references.delete(checkpointId);
        } else {
          reservations.clear();
          references.clear();
        }
      }
      const protectedObjects = new Set<string>();
      for (const objects of references.values()) objects.forEach((object) => protectedObjects.add(object));
      for (const reservation of reservations.values()) if (reservation.expiresAtMs > nowMs) reservation.objects.forEach((object) => protectedObjects.add(object));
      return {
        protected: [...protectedObjects].sort(),
        eligible: [...universe].filter((object) => !protectedObjects.has(object)).sort(),
      };
    };
    for (const snapshot of fixture.retentionLedger.snapshots) {
      expect(reachabilityAt(snapshot.throughGeneration, snapshot.nowMs)).toEqual({ protected: snapshot.protected, eligible: snapshot.eligible });
    }
    expect(fixture.retentionLedger.reservationMaximumTtlMs).toBe(300_000);
    expect(fixture.retentionLedger.reservationGraceMs).toBe(60_000);
    expect(fixture.retentionLedger.sweepPageMaximum).toBe(16);
    expect(fixture.retentionLedger.sweepObjectMaximum).toBe(4_096);

    type SweepCursor = { observedGeneration: number; afterGeneration: number; objectOffset: number };
    const sweep = fixture.sweepContinuation as {
      tokenPayloadBytes: number;
      tokenAuthenticationBytes: number;
      cursorExposesObjectIdentity: boolean;
      invalidAfterGenerationChange: boolean;
      invalidAfterRestart: boolean;
      ledgerGeneration: number;
      pageLedgerEvents: number;
      planObjectCounts: number[];
      totalObjects: number;
      requestMaximumObjects: number;
      expectedExaminedPerRequest: number[];
      expectedFirstCursor: SweepCursor;
    };
    const advanceSweep = (continuation: SweepCursor | undefined, maximum: number) => {
      let afterGeneration = continuation?.afterGeneration ?? 0;
      let objectOffset = continuation?.objectOffset ?? 0;
      let examined = 0;
      while (examined < maximum && afterGeneration < sweep.ledgerGeneration) {
        const page = sweep.planObjectCounts.slice(afterGeneration, afterGeneration + sweep.pageLedgerEvents);
        const pageObjects = page.reduce((total, count) => total + count, 0);
        const taken = Math.min(maximum - examined, pageObjects - objectOffset);
        examined += taken;
        objectOffset += taken;
        if (objectOffset < pageObjects) return { examined, continuation: { observedGeneration: sweep.ledgerGeneration, afterGeneration, objectOffset } };
        afterGeneration += page.length;
        objectOffset = 0;
      }
      return { examined, continuation: afterGeneration < sweep.ledgerGeneration ? { observedGeneration: sweep.ledgerGeneration, afterGeneration, objectOffset } : undefined };
    };
    const firstSweep = advanceSweep(undefined, sweep.requestMaximumObjects);
    const secondSweep = advanceSweep(firstSweep.continuation, sweep.requestMaximumObjects);
    expect(firstSweep).toEqual({ examined: sweep.expectedExaminedPerRequest[0], continuation: sweep.expectedFirstCursor });
    expect(secondSweep).toEqual({ examined: sweep.expectedExaminedPerRequest[1], continuation: undefined });
    expect(firstSweep.examined + secondSweep.examined).toBe(sweep.totalObjects);

    const encodeSweepContinuation = async (secret: Buffer, execute: boolean, cursor: SweepCursor): Promise<Buffer> => {
      const payload = Buffer.alloc(sweep.tokenPayloadBytes);
      payload[0] = Number(execute);
      payload.writeBigUInt64BE(BigInt(cursor.observedGeneration), 1);
      payload.writeBigUInt64BE(BigInt(cursor.afterGeneration), 9);
      payload.writeUInt32BE(cursor.objectOffset, 17);
      const authentication = await sha256(Buffer.concat([Buffer.from("semio.hub.artifact-cas.sweep-continuation.v1\0"), secret, payload]));
      return Buffer.concat([payload, authentication]);
    };
    const acceptsSweepContinuation = async (token: Buffer, secret: Buffer, execute: boolean, generation: number): Promise<boolean> => {
      if (token.length !== sweep.tokenPayloadBytes + sweep.tokenAuthenticationBytes || token[0] !== Number(execute) || Number(token.readBigUInt64BE(1)) !== generation) return false;
      const expected = await encodeSweepContinuation(secret, execute, {
        observedGeneration: Number(token.readBigUInt64BE(1)),
        afterGeneration: Number(token.readBigUInt64BE(9)),
        objectOffset: token.readUInt32BE(17),
      });
      return expected.equals(token);
    };
    const firstInstanceSecret = await sha256(Buffer.from("first-instance"));
    const restartedInstanceSecret = await sha256(Buffer.from("restarted-instance"));
    const token = await encodeSweepContinuation(firstInstanceSecret, true, sweep.expectedFirstCursor);
    expect(token.length).toBe(sweep.tokenPayloadBytes + sweep.tokenAuthenticationBytes);
    expect(await acceptsSweepContinuation(token, firstInstanceSecret, true, sweep.ledgerGeneration)).toBe(true);
    expect(await acceptsSweepContinuation(token, firstInstanceSecret, true, sweep.ledgerGeneration + 1)).toBe(!sweep.invalidAfterGenerationChange);
    expect(await acceptsSweepContinuation(token, restartedInstanceSecret, true, sweep.ledgerGeneration)).toBe(!sweep.invalidAfterRestart);
    expect(await acceptsSweepContinuation(token, firstInstanceSecret, false, sweep.ledgerGeneration)).toBe(false);
    expect(sweep.cursorExposesObjectIdentity).toBe(false);

    type BarrierOrder = {
      id: "delete-first" | "successor-first";
      actions: string[];
      oldDeleteOutcome: "deleted-before-successor-stage" | "stale-fence-rejected";
      publishedReadOutcome: "exact";
    };
    const barrier = fixture.deleteBarrier as {
      coordinatorId: string;
      spaceId: string;
      objectBytesHex: string;
      initialPhysicalEpoch: number;
      deleteLeaseEpoch: number;
      successorReservationEpoch: number;
      leaseMaximumMs: number;
      dryRunAdvancesEpoch: boolean;
      orders: BarrierOrder[];
    };
    expect(Buffer.from(barrier.coordinatorId, "hex").length).toBe(32);
    expect(barrier.leaseMaximumMs).toBe(5_000);
    expect(barrier.dryRunAdvancesEpoch).toBe(false);
    const objectBytes = Buffer.from(barrier.objectBytesHex, "hex");
    const executeBarrierOrder = (order: BarrierOrder) => {
      let physicalEpoch = barrier.initialPhysicalEpoch;
      let leaseEpoch: number | undefined;
      let leaseLive = false;
      let reservationEpoch: number | undefined;
      let bytes: Buffer | undefined = Buffer.from(objectBytes);
      let referenced = false;
      let oldDeleteOutcome: BarrierOrder["oldDeleteOutcome"] | undefined;
      for (const action of order.actions) {
        const [kind, encodedEpoch] = action.split(":");
        const epoch = encodedEpoch === undefined ? undefined : Number(encodedEpoch);
        if (kind === "lease") {
          expect(epoch).toBeGreaterThan(physicalEpoch);
          leaseEpoch = epoch;
          leaseLive = true;
        } else if (kind === "lease-expire") {
          leaseLive = false;
        } else if (kind === "reserve") {
          expect(leaseLive).toBe(false);
          expect(epoch).toBeGreaterThan(leaseEpoch ?? 0);
          reservationEpoch = epoch;
        } else if (kind === "cas-advance") {
          if (epoch !== leaseEpoch && epoch !== reservationEpoch) throw new Error("advance without directory epoch");
          physicalEpoch = Math.max(physicalEpoch, epoch ?? 0);
        } else if (kind === "validate") {
          expect(leaseLive && leaseEpoch === epoch && physicalEpoch === epoch && !referenced).toBe(true);
        } else if (kind === "delete") {
          if (physicalEpoch === epoch) {
            bytes = undefined;
            leaseLive = false;
            oldDeleteOutcome = "deleted-before-successor-stage";
          } else {
            oldDeleteOutcome = "stale-fence-rejected";
          }
        } else if (kind === "stage") {
          expect(physicalEpoch).toBe(reservationEpoch);
          bytes = Buffer.from(objectBytes);
        } else if (kind === "publish") {
          if (!bytes?.equals(objectBytes)) throw new Error("publication without exact staged bytes");
          referenced = true;
        } else if (kind === "read") {
          expect(referenced && bytes?.equals(objectBytes)).toBe(true);
        } else {
          throw new Error(`unknown barrier action: ${action}`);
        }
      }
      return { oldDeleteOutcome, publishedReadOutcome: referenced && bytes?.equals(objectBytes) ? "exact" : "missing" };
    };
    for (const order of barrier.orders) {
      expect(executeBarrierOrder(order)).toEqual({ oldDeleteOutcome: order.oldDeleteOutcome, publishedReadOutcome: order.publishedReadOutcome });
    }
  });

  it("validates and independently encodes the typed lag rebootstrap control", () => {
    const root = getWorkspaceRoot();
    const fixture = JSON.parse(readFileSync(join(root, "🌎️hub", "🛰️lag-rebootstrap", "🧪️fixtures", "🧬️lag-rebootstrap", "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(root, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: false, discriminator: true });
    ajv.addSchema(schema);
    const validate = ajv.compile({ $ref: `${schema.$id}#/$defs/DirectoryStreamMessage` });
    expect(validate(fixture.control), JSON.stringify(validate.errors)).toBe(true);
    const leaked = structuredClone(fixture.control);
    leaked.control.storageKey = `semio.artifact-cas.manifest/v1/${"ab".repeat(32)}`;
    expect(validate(leaked)).toBe(false);
    expect(fixture.closeCode).toBe(1013);
    expect(fixture.closeReason).toBe("rebootstrap-required");
    expect(fixture.scopeMaximumBytes).toBe(256);
    expect(new TextEncoder().encode(fixture.control.control.scope.spaceId).length).toBeLessThanOrEqual(fixture.scopeMaximumBytes);
    expect(new TextEncoder().encode(fixture.control.control.scope.documentId).length).toBeLessThanOrEqual(fixture.scopeMaximumBytes);
    expect(fixture.inlineMaximumBytes).toBe(4_096);
    expect(fixture.chunkMaximumBytes).toBe(4_096);
    expect(fixture.totalMaximumBytes).toBe(64 * 1024 * 1024);
    expect(fixture.chunkMaximumCount).toBe(16_384);

    const value = fixture.control.control;
    const frame: ServerFrame = { RebootstrapRequired: { control: {
      space_id: value.scope.spaceId,
      document_id: value.scope.documentId,
      checkpoint_id: value.checkpointId,
      descriptor_hash: value.descriptorDigestV1,
      baseline_frontier: {
        document_id: value.baselineFrontier.documentId,
        head_edit_ordinal: value.baselineFrontier.headEditOrdinal,
        head_edit_id: value.baselineFrontier.headEditId,
        last_commit_seq: value.baselineFrontier.lastCommitSeq,
        chain_hash: value.baselineFrontier.chainHash,
      },
    } } };
    const encoded = encodeServerFrame(frame, "Command");
    const manual: number[] = [0, 12];
    const varint = (input: number): void => {
      let remaining = input;
      while (remaining >= 128) {
        manual.push((remaining & 0x7f) | 0x80);
        remaining = Math.floor(remaining / 128);
      }
      manual.push(remaining);
    };
    const bytes = (input: readonly number[]): void => { varint(input.length); manual.push(...input); };
    const text = (input: string): void => bytes([...new TextEncoder().encode(input)]);
    text(value.scope.spaceId);
    text(value.scope.documentId);
    manual.push(...value.checkpointId, ...value.descriptorDigestV1);
    text(value.baselineFrontier.documentId);
    varint(value.baselineFrontier.headEditOrdinal);
    text(value.baselineFrontier.headEditId);
    varint(value.baselineFrontier.lastCommitSeq);
    manual.push(...value.baselineFrontier.chainHash);
    expect([...encoded]).toEqual(manual);
    expect(decodeServerFrame(Uint8Array.from(manual)).frame).toEqual(frame);
    expect(createHash("sha256").update(encoded).digest()).toEqual(createHash("sha256").update(Uint8Array.from(manual)).digest());
  });

  it("allocates a released loopback port that can be rebound", async () => {
    const port = await findFreePort();
    expect(Number.isSafeInteger(port) && port > 0 && port <= 65_535).toBe(true);
    const server = createServer();
    await new Promise<void>((resolveListen, rejectListen) => server.once("error", rejectListen).listen(port, "127.0.0.1", resolveListen));
    await new Promise<void>((resolveClose, rejectClose) => server.close((error) => error ? rejectClose(error) : resolveClose()));
  });

  it("polls a real HTTP listener and retains the platform debug-binary contract", async () => {
    const server = createServer((_request, response) => response.writeHead(204).end());
    await new Promise<void>((resolveListen, rejectListen) => server.once("error", rejectListen).listen(0, "127.0.0.1", resolveListen));
    const address = server.address();
    if (!address || typeof address === "string") throw new Error("quick harness: HTTP listener has no TCP address");
    try {
      await waitForHttpReady(`http://127.0.0.1:${address.port}/ready`, {}, 1_000);
      expect(resolveHubBinaryPath("/repo")).toBe(join("/repo", "target", "debug", process.platform === "win32" ? "os-hub.exe" : "os-hub"));
    } finally {
      await new Promise<void>((resolveClose, rejectClose) => server.close((error) => error ? rejectClose(error) : resolveClose()));
    }
  });

});

//#region 🔖️Polling
/** ⏳️ Polls `predicate` until it's true or `timeoutMs` elapses. */
async function waitUntil(predicate: () => boolean, timeoutMs = 5_000, intervalMs = 50): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    if (predicate()) return;
    await new Promise((resolveDelay) => setTimeout(resolveDelay, intervalMs));
  }
  if (!predicate()) throw new Error("waitUntil: condition never became true within budget");
}
//#endregion 🔖️Polling

//#region 🔖️FrameSocket
/** 📡️ A raw document-WS connection speaking `protocol_wire`'s binary frames via
 * `@semio-tech/framework-os`'s `encodeClientFrame`/`decodeServerFrame` — never hand-rolled. */
type FrameSocket = {
  readonly ws: WebSocket;
  next(timeoutMs?: number): Promise<ServerFrame>;
  waitFor(predicate: (frame: ServerFrame) => boolean, timeoutMs?: number): Promise<ServerFrame>;
  send(frame: ClientFrame): void;
  close(): void;
};

/** 🔌️ Opens a document WS, wiring an inbox that either resolves a pending `next()`/`waitFor()`
 * call immediately or queues the frame for the next call. */
function openFrameSocket(url: string): Promise<FrameSocket> {
  return new Promise((resolveOpen, rejectOpen) => {
    const ws = new WebSocket(url);
    ws.binaryType = "arraybuffer";
    const queue: ServerFrame[] = [];
    let waiters: Array<(error: Error | null, frame?: ServerFrame) => void> = [];
    let closedWith: Error | null = null;
    ws.onmessage = (event: MessageEvent) => {
      const bytes = new Uint8Array(event.data as ArrayBuffer);
      const { frame } = decodeServerFrame(bytes);
      const waiter = waiters.shift();
      if (waiter) waiter(null, frame);
      else queue.push(frame);
    };
    ws.onerror = () => rejectOpen(new Error(`openFrameSocket: failed to open ${url}`));
    ws.onopen = () => {
      ws.onerror = () => {
        closedWith = new Error(`openFrameSocket: ${url} errored`);
        for (const waiter of waiters.splice(0)) waiter(closedWith);
      };
      ws.onclose = (event: CloseEvent) => {
        closedWith = new Error(`openFrameSocket: ${url} closed (code ${event.code}, reason ${JSON.stringify(event.reason)})`);
        for (const waiter of waiters.splice(0)) waiter(closedWith);
      };
      const next = (timeoutMs = 5_000): Promise<ServerFrame> => {
        const queued = queue.shift();
        if (queued) return Promise.resolve(queued);
        if (closedWith) return Promise.reject(closedWith);
        return new Promise((resolveNext, rejectNext) => {
          const timer = setTimeout(() => {
            waiters = waiters.filter((waiter) => waiter !== onFrame);
            rejectNext(new Error("openFrameSocket: timed out waiting for a frame"));
          }, timeoutMs);
          const onFrame = (error: Error | null, frame?: ServerFrame): void => {
            clearTimeout(timer);
            if (error) rejectNext(error);
            else resolveNext(frame as ServerFrame);
          };
          waiters.push(onFrame);
        });
      };
      const waitFor = async (predicate: (frame: ServerFrame) => boolean, timeoutMs = 5_000): Promise<ServerFrame> => {
        const deadline = Date.now() + timeoutMs;
        while (Date.now() < deadline) {
          const frame = await next(Math.max(50, deadline - Date.now()));
          if (predicate(frame)) return frame;
        }
        throw new Error("openFrameSocket: timed out waiting for a matching frame");
      };
      resolveOpen({
        ws,
        next,
        waitFor,
        send: (frame: ClientFrame) => ws.send(encodeClientFrame(frame, "command")),
        close: () => ws.close(),
      });
    };
  });
}

function helloFrame(actor: string, token: string): ClientFrame {
  return { Hello: { wire_version: 1, protocol_version: 1, schema: "s.space.space@1/*", pack_schema_hash: new Array(32).fill(17), actor, token, resume_token: null, frontier: null } };
}

function documentDescriptor(spaceId: string, documentId: string): DocumentDescriptor {
  return {
    spaceId,
    documentId,
    artifactKind: "s.space.space",
    artifactSchema: "s.space.space@1/*",
    owner: { pluginId: "s.space", packageId: "s.space.space", version: "1.0.0", packageHash: "22".repeat(32) },
    packSchemaHash: "11".repeat(32),
    bootstrapVersion: 1,
    bootstrapFrontier: { headSeq: 0, commitSeq: 0, epoch: 0 },
    bootstrapSnapshotHash: "33".repeat(32),
  };
}

function presenceFrame(actor: string): ClientFrame {
  const peer: ArtifactPresencePeer = { actor, connectedAtMs: Date.now() };
  return { Presence: { peer: encodePresencePeer(peer) } };
}

function commandsFrame(batchId: number, documentId: string, actor: string, mutationId: string): ClientFrame {
  return {
    Commands: {
      batch_id: batchId,
      envelopes: [
        {
          mutation_id: mutationId,
          document_id: documentId,
          actor,
          dependencies: [],
          diff: { schema: "e2e.opaque.v1", payload: [] },
          inverse: { schema: "e2e.opaque.v1", payload: [] },
          timestamp: { actor: 0, physical_ms: Date.now(), logical: 0 },
        },
      ],
    },
  };
}

/** 🪣️ Collects every frame `socket` receives within `windowMs`, stopping early only on the
 * final per-call timeout — used to assert something about a whole burst of activity rather than
 * exactly one frame, which real network/broadcast timing makes fragile to pin down. */
async function drainFrames(socket: FrameSocket, windowMs: number): Promise<ServerFrame[]> {
  const frames: ServerFrame[] = [];
  const deadline = Date.now() + windowMs;
  for (;;) {
    const remaining = deadline - Date.now();
    if (remaining <= 0) return frames;
    try {
      frames.push(await socket.next(remaining));
    } catch {
      return frames;
    }
  }
}

function presenceActors(frame: ServerFrame): Set<string> {
  if (!("Presence" in frame)) throw new Error("presenceActors: not a Presence frame");
  return new Set(frame.Presence.peers.map((raw) => decodePresencePeer(new Uint8Array(raw), [0]).actor));
}
//#endregion 🔖️FrameSocket

//#region 🔖️Scenario
it.skipIf(!HUB_E2E)(
  "boots the real hub and proves directory + presence-per-surface + document-scoped commands + admin kick + restart persistence",
  async () => {
    const repoRoot = getWorkspaceRoot();
    const dataDir = mkdtempSync(join(tmpdir(), "os-hub-e2e-"));
    const adminToken = "e2e-admin";
    const sockets: FrameSocket[] = [];
    let hub: HubHandle | null = null;

    try {
      //#region 🔖️Boot
      hub = await startHub({ repoRoot, dataDir, adminToken });
      //#endregion

      //#region 🔖️Sessions
      const client1 = new DirectoryClient(hub.baseUrl);
      const client2 = new DirectoryClient(hub.baseUrl);
      const session1 = await client1.mintSession("user1@semio.dev");
      const session2 = await client2.mintSession("user2@semio.dev");
      const me1 = await client1.me();
      const me2 = await client2.me();
      expect(me1?.email).toBe("user1@semio.dev");
      expect(me2?.email).toBe("user2@semio.dev");
      //#endregion

      //#region 🔖️CreateSpace
      const created = await client1.command({ kind: "create-space", name: "E2E Studio", spaceKind: "studio", visibility: "private" });
      const spaceCreated = created.events.find((event) => isDirectoryEventBodyKind(event.body, "space.created"));
      expect(spaceCreated).toBeDefined();
      if (!spaceCreated || !isDirectoryEventBodyKind(spaceCreated.body, "space.created")) throw new Error("unreachable");
      const spaceId = spaceCreated.body.spaceId;
      expect(spaceCreated.body.name).toBe("E2E Studio");
      expect(spaceCreated.body.spaceKind).toBe("studio");
      const spacesForUser2Before = await client2.spaces();
      expect(spacesForUser2Before.some((space) => space.id === spaceId)).toBe(false);
      //#endregion

      //#region 🔖️Membership
      const streamMessages2: DirectoryStreamMessage[] = [];
      const stream2 = client2.stream(0, (message) => streamMessages2.push(message));
      await client1.command({ kind: "upsert-member", spaceId, email: "user2@semio.dev", role: "author" });
      await waitUntil(
        () =>
          streamMessages2.some(
            (message) => isDirectoryStreamMessageKind(message, "event") && isDirectoryEventBodyKind(message.event.body, "member.upserted") && message.event.body.spaceId === spaceId && message.event.body.userId === session2.userId,
          ),
        5_000,
      );
      stream2.close();
      const spacesForUser2After = await client2.spaces();
      const memberSpace = spacesForUser2After.find((space) => space.id === spaceId);
      expect(memberSpace?.role).toBe("author");
      await client1.command({ kind: "announce-document", descriptor: documentDescriptor(spaceId, "index") });
      //#endregion

      //#region 🔖️PresenceAndCommands
      const actorA = `user:${session1.userId}#e2e-a`;
      const actorB = `user:${session2.userId}#e2e-b`;
      const actorC = `user:${session1.userId}#e2e-c`;
      const documentWsUrl = (surface: string): string => `${hub!.wsBaseUrl}/spaces/${encodeURIComponent(spaceId)}/documents/index/ws?surface=${encodeURIComponent(surface)}`;

      const sockA = await openFrameSocket(documentWsUrl(EDITOR_SURFACE));
      sockets.push(sockA);
      sockA.send(helloFrame(actorA, session1.token));
      await sockA.waitFor((frame) => "Welcome" in frame);

      const sockB = await openFrameSocket(documentWsUrl(EDITOR_SURFACE));
      sockets.push(sockB);
      sockB.send(helloFrame(actorB, session2.token));
      await sockB.waitFor((frame) => "Welcome" in frame);

      sockA.send(presenceFrame(actorA));
      sockB.send(presenceFrame(actorB));
      const rosterA = presenceActors(await sockA.waitFor((frame) => "Presence" in frame && frame.Presence.peers.length === 2));
      const rosterB = presenceActors(await sockB.waitFor((frame) => "Presence" in frame && frame.Presence.peers.length === 2));
      expect(rosterA).toEqual(new Set([actorA, actorB]));
      expect(rosterB).toEqual(new Set([actorA, actorB]));

      const sockC = await openFrameSocket(documentWsUrl(VIEWER_SURFACE));
      sockets.push(sockC);
      sockC.send(helloFrame(actorC, session1.token));
      await sockC.waitFor((frame) => "Welcome" in frame);
      sockC.send(presenceFrame(actorC));

      // 🔬️ Drain (rather than assert zero) a short window here: `handle_client_frame`'s
      // `ClientFrame::Presence` arm does `presence.insert(...)` then a separate
      // `presence_peers(...)` snapshot read (`📦️bin.rs` ~566-568) — under near-simultaneous
      // presence sends from A and B above, this can legitimately produce one extra, IDENTICAL
      // [actorA, actorB] broadcast (a benign race in the ephemeral/best-effort presence lane, not
      // a correctness bug — observed in `🧪️3-e-hub-e2e-run1.txt`). The actual contract this step
      // proves — C's surface never appears in A/B's roster — is checked directly below regardless
      // of how many (harmless) frames arrive.
      for (const frame of await drainFrames(sockA, 500)) {
        if (!("Presence" in frame)) continue;
        const actors = presenceActors(frame);
        expect(actors.has(actorC)).toBe(false);
        expect(actors.size).toBeLessThanOrEqual(2);
      }
      for (const frame of await drainFrames(sockB, 500)) {
        if (!("Presence" in frame)) continue;
        const actors = presenceActors(frame);
        expect(actors.has(actorC)).toBe(false);
        expect(actors.size).toBeLessThanOrEqual(2);
      }

      const documentId = `${spaceId}:index`;
      const mutationId = crypto.randomUUID();
      sockA.send(commandsFrame(1, documentId, actorA, mutationId));
      const ack = await sockA.waitFor((frame) => "Ack" in frame && frame.Ack.batch_id === 1);
      if (!("Ack" in ack)) throw new Error("unreachable");
      const applied = ack.Ack.stages.find((stage) => typeof stage === "object" && "Applied" in stage);
      expect(applied && typeof applied === "object" && "Applied" in applied && applied.Applied.outcome === "Accepted").toBe(true);

      const commandsOnB = await sockB.waitFor((frame) => "Commands" in frame);
      const commandsOnC = await sockC.waitFor((frame) => "Commands" in frame);
      if (!("Commands" in commandsOnB) || !("Commands" in commandsOnC)) throw new Error("unreachable");
      expect(commandsOnB.Commands.envelopes[0]?.mutation_id).toBe(mutationId);
      expect(commandsOnC.Commands.envelopes[0]?.mutation_id).toBe(mutationId);
      //#endregion

      //#region 🔖️AdminKick
      const connections = (await (await fetch(`${hub.baseUrl}/admin/api/connections`, { headers: { authorization: `Bearer ${adminToken}` } })).json()) as ConnectionView[];
      expect(connections.length).toBe(3);
      const surfaces = connections.map((connection) => connection.surface).sort();
      expect(surfaces).toEqual([EDITOR_SURFACE, EDITOR_SURFACE, VIEWER_SURFACE].sort());
      const connectionC = connections.find((connection) => connection.actor === actorC);
      expect(connectionC).toBeDefined();
      if (!connectionC) throw new Error("unreachable");

      let sockCClosed = false;
      sockC.ws.onclose = () => {
        sockCClosed = true;
      };
      const closeResponse = await fetch(`${hub.baseUrl}/admin/api/connections/${encodeURIComponent(connectionC.syncSessionId)}/close`, { method: "POST", headers: { authorization: `Bearer ${adminToken}` } });
      expect(closeResponse.status).toBe(204);
      await waitUntil(() => sockCClosed, 5_000);
      //#endregion

      //#region 🔖️RestartPersistence
      const beforeRestartStatus = (await (await fetch(`${hub.baseUrl}/spaces/${encodeURIComponent(spaceId)}/documents/index`, { headers: { authorization: `Bearer ${session1.token}` } })).json()) as { commit_seq: number; head_seq: number };
      expect(beforeRestartStatus.commit_seq).toBeGreaterThan(0);

      sockA.close();
      sockB.close();
      const oldHub = hub;
      hub = null;
      await oldHub.stop();

      hub = await startHub({ repoRoot, dataDir, adminToken, port: await findFreePort() });
      const client1AfterRestart = new DirectoryClient(hub.baseUrl);
      const sessionAfterRestart = await client1AfterRestart.mintSession("user1@semio.dev");
      const spacesAfterRestart = await client1AfterRestart.spaces();
      const spaceAfterRestart = spacesAfterRestart.find((space) => space.id === spaceId);
      expect(spaceAfterRestart?.role).toBe("author");

      const detailAfterRestart = await client1AfterRestart.space(spaceId);
      expect(detailAfterRestart.members.some((member) => member.userId === session2.userId && member.role === "author")).toBe(true);
      expect(detailAfterRestart.documents[0]?.descriptor).toEqual(documentDescriptor(spaceId, "index"));

      const afterRestartStatus = (await (await fetch(`${hub.baseUrl}/spaces/${encodeURIComponent(spaceId)}/documents/index`, { headers: { authorization: `Bearer ${sessionAfterRestart.token}` } })).json()) as { commit_seq: number; head_seq: number };
      expect(afterRestartStatus.commit_seq).toBe(beforeRestartStatus.commit_seq);
      expect(afterRestartStatus.head_seq).toBe(beforeRestartStatus.head_seq);
      //#endregion
    } finally {
      for (const socket of sockets) socket.close();
      if (hub) await hub.stop();
      rmSync(dataDir, { recursive: true, force: true });
    }
  },
  TEST_TIMEOUT_MS,
);
//#endregion 🔖️Scenario
