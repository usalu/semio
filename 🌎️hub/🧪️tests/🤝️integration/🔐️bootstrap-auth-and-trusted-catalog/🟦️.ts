/** 🔐️ The local bootstrap, auth capability and trusted catalog bundle contract tests. */
import { createHash, createHmac, webcrypto } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { expect, it } from "vitest";
import { getWorkspaceRoot } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { parseInviteCapabilityV1, parseSessionCapabilityV1, parseShareCapabilityV1, parseSocketGrantCapabilityV1 } from "../../../🔐️auth/🧬️schema/🟦️.ts";
import { scopeExport } from "../🧰️hub-integration-fixtures/🟦️.ts";

/** 🪪 Registers the bootstrap, credential and readiness schema, the auth capability and socket grant, and the trusted catalog bundle tests. */
export function registerBootstrapAuthAndTrustedCatalogTests(): void {
  it("validates local bootstrap, one-shot credential, and readiness schemas with independent HMAC", () => {
    const root = getWorkspaceRoot();
    const contractRoot = join(root, "🌎️hub", "🚀️local-bootstrap");
    const fixture = JSON.parse(readFileSync(join(contractRoot, "🧪️tests/🧫️fixtures", "🚇️pipe-v1", "🔣️.json"), "utf8"));
    const localBootstrapExport = scopeExport(root, "hub.local-bootstrap");
    const validatePipe = localBootstrapExport("LocalBootstrapPipeV1");
    const validateReadiness = localBootstrapExport("LocalBootstrapReadinessV1");
    const validateCredential = localBootstrapExport("LocalBootstrapCredentialEnvelopeV1");
    const validateLiveness = localBootstrapExport("LocalBootstrapLivenessV1");
    const exports = [
      [validatePipe, fixture.initialize, fixture.hello, fixture.issue],
      [localBootstrapExport("LocalBootstrapPipeInitializeV1"), fixture.initialize],
      [localBootstrapExport("LocalBootstrapPipeHelloV1"), fixture.hello],
      [localBootstrapExport("LocalBootstrapPipeIssueV1"), fixture.issue],
      [localBootstrapExport("LocalBootstrapProfileV1"), ...fixture.initialize.profiles],
      [validateCredential, fixture.credential],
      [validateReadiness, fixture.ready, fixture.bootstrapReadyButArtifactUnavailable, fixture.notReady, fixture.starting],
      [validateLiveness, fixture.live],
    ] as const;
    for (const [validate, ...values] of exports) for (const value of values) expect(validate(value), "validate rejected its own fixture").toBe(true);
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
    expect(validateReadiness({ ...fixture.ready, capability: fixture.credential.capability })).toBe(false);
    expect(validateReadiness({ ...fixture.ready, sessionKind: fixture.credential.sessionKind })).toBe(false);
    expect(validateReadiness({ ...fixture.ready, authorizationGeneration: fixture.credential.authorizationGeneration })).toBe(false);
    expect(validateReadiness({ ...fixture.ready, artifactAuthority: { ready: false } })).toBe(false);
    expect(validateReadiness({ ...fixture.bootstrapReadyButArtifactUnavailable, status: "ready" })).toBe(false);
    expect(validateReadiness({ ...fixture.ready, startup: fixture.starting.startup }), "a ready hub never reports startup progress").toBe(false);
    expect(validateLiveness({ ...fixture.live, status: "ready" }), "liveness is only ever live").toBe(false);
    expect(validateLiveness({ ...fixture.live, blockedBy: [] }), "liveness says nothing about readiness").toBe(false);
    expect(validateReadiness({ ...fixture.starting, startup: { ...fixture.starting.startup, stage: "Guest Codec" } })).toBe(false);
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
    const fixture = JSON.parse(readFileSync(join(root, "🌎️hub", "🔐️auth", "🧪️tests/🧫️fixtures", "🔑️capability-v1", "🔣️.json"), "utf8"));
    const authExport = scopeExport(root, "hub.auth");
    for (const [exportId, value] of [
      ["SessionCapabilityV1", fixture.session.capability],
      ["ShareCapabilityV1", fixture.share.capability],
      ["InviteCapabilityV1", fixture.invite.capability],
      ["SocketGrantCapabilityV1", fixture.socket.capability],
      ["SocketGrantReceiptV1", fixture.socket.receipt],
      ["AuthLimitsV1", fixture.limits],
    ] as const) {
      const validateExport = authExport(exportId);
      expect(validateExport(value), `${exportId}: scope export rejected its own fixture`).toBe(true);
    }
    const validateSocketGrant = authExport("SocketGrantCapabilityV1");
    for (const hostile of fixture.socket.rejectedCapabilities.slice(0, 4)) expect(validateSocketGrant(hostile), hostile).toBe(false);
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
    const catalogRoot = join(root, "🌎️hub", "🗿️artifact-authority", "🔏️trusted-catalog");
    const fixture = JSON.parse(readFileSync(join(catalogRoot, "🧪️tests/🧫️fixtures", "👥️two-package", "🔣️.json"), "utf8"));
    const validate = scopeExport(root, "hub.artifact-authority.trusted-catalog")("TrustedBundleV1");
    expect(validate(fixture.bundle), "validate rejected its own fixture").toBe(true);
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
      const selected = new Map<string, any>();
      const count = Buffer.alloc(4);
      count.writeUInt32BE(profile.selectedClosure.length);
      const digest = createHash("sha256").update("semio/hub/trusted-profile-selected-closure/v1\0").update(count);
      let prior = "";
      for (const identity of profile.selectedClosure) {
        if (prior >= identity.pluginId) throw new Error("noncanonical closure order");
        prior = identity.pluginId;
        const entry = packages.get(identity.pluginId);
        if (!entry || entry.packageId !== identity.packageId || entry.version !== identity.version) throw new Error("incomplete or conflicting closure");
        selected.set(identity.pluginId, identity);
        for (const value of [identity.pluginId, identity.packageId, identity.version]) {
          const bytes = Buffer.from(value);
          const length = Buffer.alloc(8);
          length.writeBigUInt64BE(BigInt(bytes.length));
          digest.update(length).update(bytes);
        }
      }
      if (digest.digest("hex") !== profile.selectedClosureSha256) throw new Error("selected closure digest differs");
      for (const selection of profile.openTargets) {
        const target = packages.get(selection.package.pluginId);
        if (!selected.has(target?.pluginId) || target.packageId !== selection.package.packageId || target.version !== selection.package.version
          || !target.openTargets.some((candidate: any) => JSON.stringify(candidate) === JSON.stringify(selection.target))) throw new Error("open target outside selected closure");
      }
      const visiting = new Set<string>();
      const visited = new Set<string>();
      const order: string[] = [];
      const visit = (required: any): void => {
        const entry = packages.get(required.pluginId);
        if (!entry || !selected.has(required.pluginId) || entry.packageId !== required.packageId || entry.version !== required.version) throw new Error("incomplete or conflicting closure");
        if (visiting.has(entry.pluginId)) throw new Error("dependency cycle");
        if (visited.has(entry.pluginId)) return;
        visiting.add(entry.pluginId);
        for (const dependency of entry.dependencies) visit(dependency);
        visiting.delete(entry.pluginId);
        visited.add(entry.pluginId);
        order.push(entry.pluginId);
      };
      for (const required of profile.selectedClosure) visit(required);
      return order;
    };
    expect(dependencyFirst(fixture.bundle, "fixture")).toEqual(["fixture-base", "fixture-editor"]);
    const incomplete = structuredClone(fixture.bundle);
    incomplete.packages.pop();
    expect(() => dependencyFirst(incomplete, "fixture")).toThrow("incomplete");
    const lossy = structuredClone(fixture.bundle);
    lossy.profiles[0].selectedClosure[0].packageId = lossy.profiles[0].selectedClosure[0].pluginId;
    expect(() => dependencyFirst(lossy, "fixture")).toThrow("conflicting");
    const reordered = structuredClone(fixture.bundle);
    reordered.profiles[0].selectedClosure.reverse();
    expect(() => dependencyFirst(reordered, "fixture")).toThrow("noncanonical");
    const changedDigest = structuredClone(fixture.bundle);
    changedDigest.profiles[0].selectedClosureSha256 = "2".repeat(64);
    expect(() => dependencyFirst(changedDigest, "fixture")).toThrow("digest");
    const changedTarget = structuredClone(fixture.bundle);
    changedTarget.profiles[0].openTargets[0].package.packageId = "unselected";
    expect(() => dependencyFirst(changedTarget, "fixture")).toThrow("outside selected closure");

    const componentMaximum = structuredClone(fixture.bundle);
    componentMaximum.packages[0].component.byteLength = fixture.limits.componentBytesMax;
    expect(validate(componentMaximum), "validate rejected its own fixture").toBe(true);
    componentMaximum.packages[0].component.byteLength = fixture.limits.componentBytesMaxPlusOne;
    expect(validate(componentMaximum)).toBe(false);
    const descriptorMaximum = structuredClone(fixture.bundle);
    descriptorMaximum.packages[0].descriptor.byteLength = fixture.limits.descriptorBytesMax;
    expect(validate(descriptorMaximum), "validate rejected its own fixture").toBe(true);
    descriptorMaximum.packages[0].descriptor.byteLength = fixture.limits.descriptorBytesMaxPlusOne;
    expect(validate(descriptorMaximum)).toBe(false);
    const identityMaximum = structuredClone(fixture.bundle);
    identityMaximum.packages[0].pluginId = "a".repeat(fixture.limits.identityBytesMax);
    expect(validate(identityMaximum), "validate rejected its own fixture").toBe(true);
    identityMaximum.packages[0].pluginId = "a".repeat(fixture.limits.identityBytesMaxPlusOne);
    expect(validate(identityMaximum)).toBe(false);
    const codecsMaximum = structuredClone(fixture.bundle);
    codecsMaximum.packages[0].nativeCodecs = Array.from({ length: fixture.limits.codecCountMax }, (_, index) => ({ artifactKind: `fixture.kind.${index}`, artifactSchema: `fixture.schema.${index}`, packSchemaHash: "11".repeat(32) }));
    expect(validate(codecsMaximum), "validate rejected its own fixture").toBe(true);
    codecsMaximum.packages[0].nativeCodecs.push({ artifactKind: "fixture.kind.overflow", artifactSchema: "fixture.schema.overflow", packSchemaHash: "11".repeat(32) });
    expect(validate(codecsMaximum)).toBe(false);
    const zeroHash = structuredClone(fixture.bundle);
    zeroHash.packages[0].nativeCodecs[0].packSchemaHash = "00".repeat(32);
    expect(validate(zeroHash)).toBe(false);
  });
}
