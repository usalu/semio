import { createHash, webcrypto } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, truncateSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import Ajv from "ajv";
import { afterEach, describe, expect, it } from "vitest";
import { encodePackValue } from "../../../🟦️.ts";
import {
  CATALOG_ARTIFACT_MAX_BYTES,
  CATALOG_COMMIT_MARKER_FILENAME,
  CATALOG_DEPENDENCY_MAX,
  CATALOG_NODE_MAX,
  auditPluginCatalogSources,
  createFreshCatalogCommitMarker,
  createFreshCatalogBuildVerifier,
  executeCatalogVerificationPlan,
  orderCatalogNodes,
  parseComponentPackageId,
  sha256CatalogArtifact,
  validateCatalogDescriptorPair,
  type CatalogVerificationNode,
  type PluginRegistryEntry,
} from "../📇️registry/📜️script.ts";

const fixtureRoot = join(import.meta.dirname, "🧪️tests", "🧬️catalog-complete");
const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as {
  readonly nodes: readonly CatalogVerificationNode[];
  readonly expectedOrder: readonly string[];
  readonly parentFailure: { readonly failedPluginId: string; readonly statuses: Readonly<Record<string, string>>; readonly publication: string };
  readonly expectedProgress: readonly object[];
  readonly expectedResult: object;
};
const temporaryRoots: string[] = [];

function temporaryRoot(): string {
  const root = mkdtempSync(join(tmpdir(), "semio-catalog-complete-"));
  temporaryRoots.push(root);
  return root;
}

async function sha256(bytes: Uint8Array): Promise<string> {
  return Buffer.from(await webcrypto.subtle.digest("SHA-256", bytes)).toString("hex");
}

async function syntheticDescriptor(pluginId: string, raw: Uint8Array, core: Uint8Array): Promise<{ readonly descriptor: Record<string, any>; readonly bytes: Uint8Array }> {
  const descriptor: Record<string, any> = {
    descriptorVersion: 1,
    packageId: `semio:${pluginId}`,
    role: "plugin",
    manifest: { pluginId, label: pluginId, version: "1.0.0", apps: [], examples: [], capabilities: [], topicContributions: [], commands: [], artifactKinds: [] },
    execution: "isolated",
    quotas: {},
    contributions: {},
    hashes: { wasmSha256: await sha256(raw), coreWasmSha256: await sha256(core), descriptorSha256: "" },
  };
  descriptor.hashes.descriptorSha256 = await sha256(encodePackValue(descriptor));
  return { descriptor, bytes: encodePackValue(descriptor) };
}

function syntheticEntry(hashes: PluginRegistryEntry["hashes"]): PluginRegistryEntry {
  return {
    pluginId: "parent",
    packageId: "semio:parent",
    cratePath: join("owner", "📦️packages", "🦀️rust"),
    packageName: "semio-s-plugin-parent",
    wasmOut: "semio_s_plugin_parent.wasm",
    role: "plugin",
    capabilities: [], contributes: [], consumes: [], dependsOn: [], activationEvents: [], extensionPoints: [], hashes,
  };
}

afterEach(() => {
  for (const root of temporaryRoots.splice(0)) rmSync(root, { recursive: true, force: true });
});

describe("strict plugin catalog completion", () => {
  it("validates the neutral contract and withholds every publication after a parent failure", async () => {
    const schema = JSON.parse(readFileSync(join(fixtureRoot, "🧬️schema", "🔣️.json"), "utf8"));
    expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    expect(orderCatalogNodes(fixture.nodes).map(({ pluginId }) => pluginId)).toEqual(fixture.expectedOrder);
    const verified: string[] = [];
    const publications: string[][] = [];
    const progress: object[] = [];
    const result = await executeCatalogVerificationPlan(fixture.nodes, {
      async verify(node) {
        verified.push(node.pluginId);
        if (node.pluginId === fixture.parentFailure.failedPluginId) throw new Error("forced parent failure");
        return node.pluginId;
      },
      async publish(rows) { publications.push(rows.map(({ node }) => node.pluginId)); },
    }, { cancelled: () => false, progress: (event) => progress.push(event) });
    expect(verified).toEqual(["parent"]);
    expect(Object.fromEntries(result.results.map((row) => [row.pluginId, row.status]))).toEqual(fixture.parentFailure.statuses);
    expect(result.publication).toBe(fixture.parentFailure.publication);
    expect(publications).toEqual([]);
    expect(progress).toEqual(fixture.expectedProgress);
    expect(result).toEqual(fixture.expectedResult);
    const committed: string[][] = [];
    const success = await executeCatalogVerificationPlan(fixture.nodes, {
      async verify(node) { return node.pluginId; },
      async publish(rows) { committed.push(rows.map(({ node }) => node.pluginId)); },
    }, { cancelled: () => false });
    expect(success.publication).toBe("committed");
    expect(committed).toEqual([fixture.expectedOrder]);
  });

  it("rejects max+1, duplicates, missing parents and cycles, and bounds cancellation diagnostics", async () => {
    const nodes = Array.from({ length: CATALOG_NODE_MAX + 1 }, (_, index) => ({ pluginId: `p-${index}`, role: "plugin" as const, dependsOn: [] }));
    expect(() => orderCatalogNodes(nodes)).toThrow(/1\.\.256/);
    expect(() => orderCatalogNodes([{ pluginId: "child", role: "plugin", dependsOn: Array.from({ length: CATALOG_DEPENDENCY_MAX + 1 }, (_, index) => `p-${index}`) }])).toThrow(/dependencies/);
    expect(() => orderCatalogNodes([{ pluginId: "same", role: "plugin", dependsOn: [] }, { pluginId: "same", role: "extension", dependsOn: [] }])).toThrow(/duplicate/);
    expect(() => orderCatalogNodes([{ pluginId: "child", role: "plugin", dependsOn: ["absent"] }])).toThrow(/absent/);
    expect(() => orderCatalogNodes([{ pluginId: "a", role: "plugin", dependsOn: ["b"] }, { pluginId: "b", role: "plugin", dependsOn: ["a"] }])).toThrow(/cycle/);
    const failed = await executeCatalogVerificationPlan(fixture.nodes, { async verify() { throw new Error("x".repeat(10_000)); } }, { cancelled: () => false });
    expect(Buffer.byteLength(failed.results[0]!.diagnostic ?? "")).toBeLessThanOrEqual(4096);
    const cancelled = await executeCatalogVerificationPlan(fixture.nodes, { async verify() { return "unused"; } }, { cancelled: () => true });
    expect(cancelled.results.every(({ status }) => status === "cancelled")).toBe(true);
  });

  it("strict-decodes both descriptor forms and verifies fresh bytes with a WebCrypto oracle", async () => {
    const sourceRoot = temporaryRoot();
    const buildRoot = temporaryRoot();
    const raw = new TextEncoder().encode("raw-component");
    const core = new TextEncoder().encode("extracted-core");
    const { descriptor, bytes } = await syntheticDescriptor("parent", raw, core);
    const entry = syntheticEntry(descriptor.hashes);
    const ownerRoot = join(sourceRoot, "owner");
    mkdirSync(join(ownerRoot, "📦️packages", "🦀️rust"), { recursive: true });
    writeFileSync(join(ownerRoot, "🔣️.json"), `${JSON.stringify(descriptor, null, 2)}\n`);
    writeFileSync(join(ownerRoot, "🛂️.descriptor.semio"), bytes);
    const source = validateCatalogDescriptorPair(entry, sourceRoot);
    expect(entry.pluginId).not.toBe(entry.packageName);
    const wrongPackage = structuredClone(descriptor);
    wrongPackage.packageId = "semio:not-parent";
    wrongPackage.hashes.descriptorSha256 = "";
    wrongPackage.hashes.descriptorSha256 = await sha256(encodePackValue(wrongPackage));
    writeFileSync(join(ownerRoot, "🔣️.json"), `${JSON.stringify(wrongPackage, null, 2)}\n`);
    writeFileSync(join(ownerRoot, "🛂️.descriptor.semio"), encodePackValue(wrongPackage));
    expect(() => validateCatalogDescriptorPair(entry, sourceRoot)).toThrow(/packageId/);
    const mismatch = await syntheticDescriptor("package-name-is-not-plugin-id", raw, core);
    mismatch.descriptor.packageId = "semio:parent";
    mismatch.descriptor.hashes.descriptorSha256 = "";
    mismatch.descriptor.hashes.descriptorSha256 = await sha256(encodePackValue(mismatch.descriptor));
    writeFileSync(join(ownerRoot, "🔣️.json"), `${JSON.stringify(mismatch.descriptor, null, 2)}\n`);
    writeFileSync(join(ownerRoot, "🛂️.descriptor.semio"), encodePackValue(mismatch.descriptor));
    expect(() => validateCatalogDescriptorPair(entry, sourceRoot)).toThrow(/manifest\.pluginId/);
    const malformed = structuredClone(descriptor);
    malformed.hashes.wasmSha256 = malformed.hashes.wasmSha256.toUpperCase();
    writeFileSync(join(ownerRoot, "🔣️.json"), `${JSON.stringify(malformed, null, 2)}\n`);
    writeFileSync(join(ownerRoot, "🛂️.descriptor.semio"), encodePackValue(malformed));
    expect(() => validateCatalogDescriptorPair(entry, sourceRoot)).toThrow(/lowercase 64-hex/);
    const duplicatePackageId = `${JSON.stringify(descriptor, null, 2).replace(/\n\}/u, ',\n  "packageId": "semio:parent"\n}')}\n`;
    writeFileSync(join(ownerRoot, "🔣️.json"), duplicatePackageId);
    expect(() => validateCatalogDescriptorPair(entry, sourceRoot)).toThrow(/duplicate object field "packageId"/);
    writeFileSync(join(ownerRoot, "🔣️.json"), `${JSON.stringify(descriptor, null, 2)}\n`);
    rmSync(join(ownerRoot, "🛂️.descriptor.semio"));
    expect(() => validateCatalogDescriptorPair(entry, sourceRoot)).toThrow();
    writeFileSync(join(ownerRoot, "🛂️.descriptor.semio"), bytes);
    const rowRoot = join(buildRoot, entry.pluginId);
    for (const part of ["raw", "core", "descriptor"]) mkdirSync(join(rowRoot, part), { recursive: true });
    writeFileSync(join(rowRoot, "raw", entry.wasmOut), raw);
    writeFileSync(join(rowRoot, "core", entry.wasmOut), core);
    writeFileSync(join(rowRoot, "descriptor", "🛂️.descriptor.semio"), bytes);
    const verifier = createFreshCatalogBuildVerifier(sourceRoot, buildRoot);
    expect(() => verifier.verify(source.entry)).toThrow(/commit marker/);
    const marker = createFreshCatalogCommitMarker(source, buildRoot);
    writeFileSync(join(rowRoot, CATALOG_COMMIT_MARKER_FILENAME), `${JSON.stringify(marker)}\n`, { flag: "wx" });
    rmSync(source.jsonPath);
    rmSync(source.packPath);
    const { hashes: _ownerProjection, ...ownerlessEntry } = source.entry;
    const receipt = verifier.verify(ownerlessEntry);
    expect(receipt.rawSha256).toBe(await sha256(raw));
    expect(receipt.coreSha256).toBe(await sha256(core));
    expect(receipt.descriptorSha256).toBe(descriptor.hashes.descriptorSha256);
    expect(() => verifier.verify({ ...ownerlessEntry, packageId: "semio:not-parent" })).toThrow(/carried Cargo identity/);
    expect(() => verifier.verify({ ...ownerlessEntry, packageId: "semio:Parent" })).toThrow(/carried Cargo identity/);
    expect(createHash("sha256").update(raw).digest("hex")).toBe(await sha256(raw));
    writeFileSync(join(rowRoot, CATALOG_COMMIT_MARKER_FILENAME), `${JSON.stringify({ ...marker, pluginId: "tampered" })}\n`);
    expect(() => verifier.verify(ownerlessEntry)).toThrow(/commit marker/);
    writeFileSync(join(rowRoot, CATALOG_COMMIT_MARKER_FILENAME), `${JSON.stringify(marker)}\n`);
    const rawMutation = Uint8Array.from(raw);
    rawMutation[0] ^= 1;
    writeFileSync(join(rowRoot, "raw", entry.wasmOut), rawMutation);
    expect(() => verifier.verify(ownerlessEntry)).toThrow(/commit marker/);
    writeFileSync(join(rowRoot, "raw", entry.wasmOut), raw);
    const coreMutation = Uint8Array.from(core);
    coreMutation[0] ^= 1;
    writeFileSync(join(rowRoot, "core", entry.wasmOut), coreMutation);
    expect(() => verifier.verify(ownerlessEntry)).toThrow(/commit marker/);
    writeFileSync(join(rowRoot, "core", entry.wasmOut), core);
    let rewroteRawAfterRead = false;
    const pinned = verifier.verify(ownerlessEntry, {
      afterArtifact(artifact) {
        if (artifact === "raw" && !rewroteRawAfterRead) {
          rewroteRawAfterRead = true;
          writeFileSync(join(rowRoot, "raw", entry.wasmOut), rawMutation);
        }
      },
    });
    expect(Buffer.from(pinned.rawBytes).equals(Buffer.from(raw))).toBe(true);
    expect(Buffer.from(pinned.coreBytes).equals(Buffer.from(core))).toBe(true);
    expect(Buffer.from(pinned.descriptorBytes).equals(Buffer.from(bytes))).toBe(true);
    expect(readFileSync(join(rowRoot, "raw", entry.wasmOut))).toEqual(Buffer.from(rawMutation));
    writeFileSync(join(rowRoot, "raw", entry.wasmOut), raw);
    const descriptorMutation = Uint8Array.from(bytes);
    descriptorMutation[0] ^= 1;
    writeFileSync(join(rowRoot, "descriptor", "🛂️.descriptor.semio"), descriptorMutation);
    expect(() => verifier.verify(ownerlessEntry)).toThrow(/commit marker/);
  });

  it("refuses ambient roots and detects the exact artifact max+1 boundary", () => {
    expect(parseComponentPackageId('[package.metadata.component]\npackage = "semio:parent"\n', "fixture/Cargo.toml")).toBe("semio:parent");
    expect(() => parseComponentPackageId('[package.metadata.component]\npackage = "semio:parent"\n[package.metadata.component]\npackage = "semio:parent"\n', "fixture/Cargo.toml")).toThrow(/repeats/);
    expect(() => parseComponentPackageId('[package.metadata.component]\npackage = "semio:parent"\npackage = "semio:parent"\n', "fixture/Cargo.toml")).toThrow(/repeats/);
    expect(() => parseComponentPackageId('[package.metadata.component]\npackage = "semio:parent"\npackage = 7\n', "fixture/Cargo.toml")).toThrow(/repeats/);
    expect(() => parseComponentPackageId('[package.metadata.component]\npackage = "semio:Parent"\n', "fixture/Cargo.toml")).toThrow(/lowercase/);
    expect(() => parseComponentPackageId('[package.metadata.component]\npackage-name = "semio:parent"\n', "fixture/Cargo.toml")).toThrow(/missing/);
    const repoRoot = temporaryRoot();
    const cache = join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🧑️‍💻️dev", "🔌️plugin-modules");
    mkdirSync(join(repoRoot, "target"), { recursive: true });
    mkdirSync(cache, { recursive: true });
    expect(() => createFreshCatalogBuildVerifier(repoRoot, join(repoRoot, "target"))).toThrow(/ambient shared target/);
    expect(() => createFreshCatalogBuildVerifier(repoRoot, cache)).toThrow(/development cache/);
    const hugeRoot = temporaryRoot();
    const huge = join(hugeRoot, "huge");
    writeFileSync(huge, "");
    truncateSync(huge, CATALOG_ARTIFACT_MAX_BYTES + 1);
    expect(statSync(huge).size).toBe(CATALOG_ARTIFACT_MAX_BYTES + 1);
    expect(() => sha256CatalogArtifact(huge, hugeRoot)).toThrow(/exceeds/);
  });

  it("independently enumerates the 59 real manifests and the known 19 missing source pairs", () => {
    const audit = auditPluginCatalogSources();
    const missing = audit.issues.filter(({ code }) => code === "descriptor-pair-missing").map(({ pluginId }) => pluginId).sort();
    expect(audit.manifestCount).toBe(59);
    expect(audit.order).toHaveLength(59);
    expect(audit.entries.find(({ pluginId }) => pluginId === "sequence")?.dependsOn).toContain("imperative-extension-control");
    expect(audit.issues.filter(({ code }) => code === "dependency-invalid")).toEqual([]);
    expect(missing).toEqual([
      "block", "flow-extension-bim", "flow-extension-draw", "imperative-extension-control", "imperative-extension-effect", "imperative-extension-logic", "imperative-extension-math", "imperative-extension-text", "playbook", "playbook-module-procedural", "process-extension-concrete", "process-extension-metal", "process-extension-robotic", "process-extension-wood", "sourcing-module-beams", "sourcing-module-slabs", "sourcing-module-windows", "stdio", "trinity",
    ]);
  });
});
