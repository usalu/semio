import { createHash, webcrypto } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, symlinkSync, truncateSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import Ajv from "ajv";
import { parseModuleDirectories, moduleDirectoryName, moduleIdForDirectoryName } from "./📦️deployment/🟦️.ts";
import { pluginModuleUrl, extensionModuleUrl } from "./🤖️generated/🧩️plugins.ts";
import { afterEach, describe, expect, it } from "vitest";
import { encodePackValue } from "../../../🟦️.ts";
import { emitOwnerDescriptorPairV1 } from "../🖨️describe/📦️packages/🦀️rust/📜️script.ts";
import {
  CATALOG_ARTIFACT_MAX_BYTES,
  CATALOG_COMMIT_MARKER_FILENAME,
  CATALOG_DEPENDENCY_MAX,
  CATALOG_NODE_MAX,
  auditInteractiveJobClassificationDrift,
  auditPluginCatalogSources,
  createFreshCatalogCommitMarker,
  createFreshCatalogBuildVerifier,
  executeCatalogVerificationPlan,
  orderCatalogNodes,
  parseComponentPackageId,
  rejectPlaceholderCatalogIdentity,
  sha256CatalogArtifact,
  validateCatalogDescriptorPair,
  verifyDescriptorPairBytesV1,
  type CatalogVerificationNode,
  type PluginRegistryEntry,
} from "./📜️script.ts";

const fixtureRoot = join(import.meta.dirname, "🧪️tests", "🧬️catalog-complete");
const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as {
  readonly nodes: readonly CatalogVerificationNode[];
  readonly expectedOrder: readonly string[];
  readonly parentFailure: { readonly failedPluginId: string; readonly statuses: Readonly<Record<string, string>>; readonly publication: string };
  readonly expectedProgress: readonly object[];
  readonly expectedResult: object;
};
const temporaryRoots: string[] = [];

describe("handpicked module deployment directories", () => {
  it("admits only schema-owned module routes and canonical encoded request paths", async () => {
    const deployment = await import("./📦️deployment/🟦️.ts"), { URL: IndependentUrl } = await import("whatwg-url");
    const routes = JSON.parse(readFileSync(join(import.meta.dirname, "📦️deployment/🛣️routes.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(import.meta.dirname, "📦️deployment/📐️schema.json"), "utf8"));
    const cases = JSON.parse(readFileSync(join(import.meta.dirname, "📦️deployment/🧪️cases.json"), "utf8"));
    const validate = new Ajv({ strict: true }).compile(schema.definitions.moduleRoutes);
    expect(validate(routes)).toBe(true);
    expect(deployment.parseModuleRoutes(routes)).toEqual(routes);
    for (const hostile of [null, [], {}, { ...routes, plugin: "/plugin-modules" }, { ...routes, extension: "/extensions" }, { ...routes, plugin: routes.extension }, { ...routes, extra: true }, { ...routes, plugin: "/🔌️plugin-modules/child" }]) {
      expect(validate(hostile)).toBe(false);
      expect(() => deployment.parseModuleRoutes(hostile)).toThrow();
    }
    for (const row of cases.routeCases) {
      expect(deployment.moduleRoutePath(row.input), row.input).toBe(row.output);
      if (row.output !== null) expect(decodeURIComponent(new IndependentUrl(row.input, "https://example.invalid").pathname)).toBe(row.output);
    }
  });

  it("uses the declared authored bridge without deriving a filename from the public ID", async () => {
    const emojiRegex = (await import("emoji-regex")).default;
    const cases = JSON.parse(readFileSync(join(import.meta.dirname, "📦️deployment/🧪️cases.json"), "utf8"));
    for (const name of [cases.bridgeFile, cases.installMetaFile]) expect([...name.matchAll(emojiRegex())]).toHaveLength(1);
    const deployment = await import("./📦️deployment/🟦️.ts");
    expect(deployment.moduleStaticDirectoryNames("puzzle", false)).toEqual(cases.staticDirectories);
    expect(deployment.moduleStaticDirectoryNames("s", true)).toBeUndefined();
    expect(() => deployment.moduleStaticDirectoryNames("unknown", true)).toThrow();
    for (const row of cases.moduleUrls) {
      expect(pluginModuleUrl(row.pluginId)).toBe(row.plugin);
      expect(extensionModuleUrl(row.pluginId)).toBe(row.extension);
      expect(decodeURIComponent(new URL(pluginModuleUrl(row.pluginId), "https://example.invalid").pathname)).toBe(row.plugin);
    }
    expect(() => pluginModuleUrl("unknown-public-id")).toThrow();
    expect(() => extensionModuleUrl("unknown-public-id")).toThrow();
  });
  it("matches the schema and independent emoji oracle without changing public identities", async () => {
    const emojiRegex = (await import("emoji-regex")).default;
    const catalog = JSON.parse(readFileSync(join(import.meta.dirname, "📦️deployment/🗺️catalog.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(import.meta.dirname, "📦️deployment/📐️schema.json"), "utf8"));
    const cases = JSON.parse(readFileSync(join(import.meta.dirname, "📦️deployment/🧪️cases.json"), "utf8"));
    const validate = new Ajv({ strict: true }).addSchema(JSON.parse(readFileSync(join(import.meta.dirname, "../../🧩️extension/📐️directory.schema.json"), "utf8"))).compile(schema);
    expect(validate(catalog)).toBe(true);
    expect(parseModuleDirectories(catalog)).toEqual(catalog.modules);
    expect(catalog.modules).toHaveLength(59);
    const ids = JSON.parse(readFileSync(join(import.meta.dirname, "🤖️generated/🔌️plugins.json"), "utf8")).map((entry: { pluginId: string }) => entry.pluginId);
    expect(catalog.modules.map((entry: { pluginId: string }) => entry.pluginId)).toEqual(ids);
    const identities = new Set<string>();
    for (const row of catalog.modules) {
      const matches = [...row.directoryName.matchAll(emojiRegex())];
      expect(matches).toHaveLength(1);
      expect(identities.has(matches[0][0].replaceAll("\uFE0F", ""))).toBe(false);
      identities.add(matches[0][0].replaceAll("\uFE0F", ""));
      expect(moduleDirectoryName(row.pluginId)).toBe(row.directoryName);
      expect(moduleIdForDirectoryName(row.directoryName)).toBe(row.pluginId);
    }
    for (const directoryName of [...cases.validDirectories, ...cases.invalidDirectories]) {
      const candidate = { version: 1, modules: [{ pluginId: "example", directoryName }] };
      const valid = cases.validDirectories.includes(directoryName);
      expect(validate(candidate), directoryName).toBe(valid);
      if (valid) expect(parseModuleDirectories(candidate)).toEqual(candidate.modules);
      else expect(() => parseModuleDirectories(candidate), directoryName).toThrow();
    }
    expect(() => moduleDirectoryName("unknown-public-id")).toThrow();
    expect(moduleIdForDirectoryName("unknown-directory")).toBeUndefined();
    for (const attack of cases.hostileCatalogs) {
      const candidate = structuredClone(catalog);
      if (attack === "duplicate-id") candidate.modules[1].pluginId = candidate.modules[0].pluginId;
      if (attack === "duplicate-emoji") candidate.modules[1].directoryName = "🎞️different-name";
      if (attack === "missing-name") delete candidate.modules[0].directoryName;
      if (attack === "extra-field") candidate.modules[0].extra = true;
      if (attack === "wrong-version") candidate.version = 2;
      if (attack === "empty-modules") candidate.modules = [];
      expect(() => parseModuleDirectories(candidate), attack).toThrow();
    }
  });
});

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
    const cache = join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🧑‍💻dev", "🔌️plugin-modules");
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

  it("verifies one emitted owner pair against its exact raw/core bytes with a WebCrypto oracle", async () => {
    const raw = new TextEncoder().encode("raw-component");
    const core = new TextEncoder().encode("extracted-core");
    const rawSha256 = await sha256(raw);
    const coreWasmSha256 = await sha256(core);
    expect(rawSha256).not.toBe(coreWasmSha256);
    const { descriptor, bytes } = await syntheticDescriptor("parent", raw, core);
    const json = new TextEncoder().encode(`${JSON.stringify(descriptor, null, 2)}\n`);
    const pair = verifyDescriptorPairBytesV1(json, bytes, { wasmSha256: rawSha256, coreWasmSha256 });
    expect(pair).toMatchObject({ pluginId: "parent", packageId: "semio:parent", role: "plugin", version: "1.0.0", execution: "isolated" });
    expect(pair.hashes.wasmSha256).toBe(rawSha256);
    expect(pair.hashes.coreWasmSha256).toBe(coreWasmSha256);
    expect(pair.hashes.descriptorSha256).toBe(createHash("sha256").update(encodePackValue({ ...descriptor, hashes: { ...descriptor.hashes, descriptorSha256: "" } })).digest("hex"));
    const swapped = await syntheticDescriptor("parent", core, raw);
    expect(() => verifyDescriptorPairBytesV1(new TextEncoder().encode(`${JSON.stringify(swapped.descriptor, null, 2)}\n`), swapped.bytes, { wasmSha256: rawSha256, coreWasmSha256 })).toThrow(/exact raw\/core artifacts/);
    const divergent = structuredClone(descriptor);
    divergent.manifest.label = "a different label";
    expect(() => verifyDescriptorPairBytesV1(json, encodePackValue(divergent), { wasmSha256: rawSha256, coreWasmSha256 })).toThrow(/JSON and pack forms disagree/);
    const tampered = structuredClone(descriptor);
    tampered.hashes.descriptorSha256 = await sha256(new TextEncoder().encode("not the self hash"));
    expect(() => verifyDescriptorPairBytesV1(new TextEncoder().encode(`${JSON.stringify(tampered, null, 2)}\n`), encodePackValue(tampered), { wasmSha256: rawSha256, coreWasmSha256 })).toThrow(/self-hash mismatch/);
    const trailing = new Uint8Array([...bytes, 0]);
    expect(() => verifyDescriptorPairBytesV1(json, trailing, { wasmSha256: rawSha256, coreWasmSha256 })).toThrow();
    for (const identity of [{ pluginId: "empty", version: "1.0.0" }, { pluginId: "cad-extension-spatial-shape", version: "0.0.0" }]) {
      const placeholder = await syntheticDescriptor(identity.pluginId, raw, core);
      placeholder.descriptor.manifest.version = identity.version;
      placeholder.descriptor.hashes.descriptorSha256 = "";
      placeholder.descriptor.hashes.descriptorSha256 = await sha256(encodePackValue(placeholder.descriptor));
      const placeholderBytes = encodePackValue(placeholder.descriptor);
      expect(() => verifyDescriptorPairBytesV1(new TextEncoder().encode(`${JSON.stringify(placeholder.descriptor, null, 2)}\n`), placeholderBytes, { wasmSha256: rawSha256, coreWasmSha256 }), identity.pluginId).toThrow(/placeholder identity/);
    }
    expect(() => rejectPlaceholderCatalogIdentity("cad-extension-spatial-shape", "1.0.0")).not.toThrow();
    const hosted = await syntheticDescriptor("host-extension-child", raw, core);
    hosted.descriptor.role = "extension";
    hosted.descriptor.manifest.dependencies = [{ pluginId: "host" }];
    hosted.descriptor.hashes.descriptorSha256 = "";
    hosted.descriptor.hashes.descriptorSha256 = await sha256(encodePackValue(hosted.descriptor));
    expect(verifyDescriptorPairBytesV1(new TextEncoder().encode(`${JSON.stringify(hosted.descriptor, null, 2)}\n`), encodePackValue(hosted.descriptor), { wasmSha256: rawSha256, coreWasmSha256 })).toMatchObject({ role: "extension", extends: "host" });
    const hostless = structuredClone(hosted.descriptor);
    hostless.manifest.dependencies = [];
    hostless.hashes.descriptorSha256 = "";
    hostless.hashes.descriptorSha256 = await sha256(encodePackValue(hostless));
    expect(() => verifyDescriptorPairBytesV1(new TextEncoder().encode(`${JSON.stringify(hostless, null, 2)}\n`), encodePackValue(hostless), { wasmSha256: rawSha256, coreWasmSha256 })).toThrow(/extension host must be its first dependency/);
  });

  it("refuses every unusable emission input and never half-publishes an owner pair", async () => {
    const repoRoot = temporaryRoot();
    const artifactRoot = join(repoRoot, "target-emission");
    const ownerRoot = join(repoRoot, "owner");
    const outsideRoot = temporaryRoot();
    for (const directory of [artifactRoot, ownerRoot]) mkdirSync(directory, { recursive: true });
    const previousJson = `${JSON.stringify({ previous: true }, null, 2)}\n`;
    const previousPack = encodePackValue({ previous: true });
    writeFileSync(join(ownerRoot, "🔣️.json"), previousJson);
    writeFileSync(join(ownerRoot, "🛂️.descriptor.semio"), previousPack);
    const rawComponentPath = join(artifactRoot, "component.wasm");
    const extractedCorePath = join(artifactRoot, "component.core.wasm");
    writeFileSync(rawComponentPath, "raw-component");
    writeFileSync(extractedCorePath, "extracted-core");
    const outsideComponent = join(outsideRoot, "component.wasm");
    writeFileSync(outsideComponent, "raw-component");
    const emptyComponent = join(artifactRoot, "empty.wasm");
    writeFileSync(emptyComponent, "");
    const linkedComponent = join(artifactRoot, "linked.wasm");
    symlinkSync(rawComponentPath, linkedComponent);
    const twinComponent = join(artifactRoot, "twin.wasm");
    writeFileSync(twinComponent, "raw-component");
    const request = { rawComponentPath, extractedCorePath, ownerRoot, artifactRoot };
    const cases: readonly [string, Parameters<typeof emitOwnerDescriptorPairV1>[1], Parameters<typeof emitOwnerDescriptorPairV1>[2], RegExp][] = [
      ["outside the artifact root", { ...request, rawComponentPath: outsideComponent }, {}, /resolves outside the declared artifact root/],
      ["an empty artifact", { ...request, rawComponentPath: emptyComponent }, {}, /regular non-symlink file/],
      ["a symlinked artifact", { ...request, rawComponentPath: linkedComponent }, {}, /regular non-symlink file/],
      ["a directory owner that is a file", { ...request, ownerRoot: rawComponentPath }, {}, /regular non-symlink directory/],
      ["the same file twice", { ...request, extractedCorePath: rawComponentPath }, {}, /the same file/],
      ["byte-identical raw and core", { ...request, extractedCorePath: twinComponent }, {}, /same SHA-256/],
      ["a cancelled emission", request, { cancelled: () => true }, /cancelled at validate/],
      ["an exhausted deadline", request, { deadlineMs: -1 }, /deadline of -1ms exceeded/],
    ];
    for (const [label, input, control, expected] of cases) {
      expect(() => emitOwnerDescriptorPairV1(repoRoot, input, control), label).toThrow(expected);
      expect(readFileSync(join(ownerRoot, "🔣️.json"), "utf8"), label).toBe(previousJson);
      expect(readFileSync(join(ownerRoot, "🛂️.descriptor.semio")), label).toEqual(Buffer.from(previousPack));
    }
    const stages: string[] = [];
    expect(() => emitOwnerDescriptorPairV1(repoRoot, request, { checkpoint: (stage) => { stages.push(stage); if (stages.length === 2) throw new Error("stopped after emit"); } })).toThrow(/stopped after emit/);
    expect(stages).toEqual(["validate", "emit"]);
    expect(readFileSync(join(ownerRoot, "🔣️.json"), "utf8")).toBe(previousJson);
  });

  it("reports only committed classifications that contradict the owner's own Rust", () => {
    const ownerRoot = temporaryRoot();
    const surface = join(ownerRoot, "🗿️artifacts", "✏️editor");
    mkdirSync(surface, { recursive: true });
    writeFileSync(join(surface, "🦀️.rs"), [
      '.action_interactive_job("migratedAction", InteractiveJobClassification::Migrated)',
      '.action_interactive_job("batchAction", semio_framework::InteractiveJobClassification::BatchOnlyPendingRewrite)',
      '.action_interactive_job("perSurfaceAction", InteractiveJobClassification::Migrated)',
    ].join("\n"));
    mkdirSync(join(ownerRoot, "🧩️extensions"), { recursive: true });
    writeFileSync(join(ownerRoot, "🧩️extensions", "🦀️.rs"), '.action_interactive_job("migratedAction", InteractiveJobClassification::Deleted)');
    const viewer = join(ownerRoot, "🗿️artifacts", "👁️viewer");
    mkdirSync(viewer, { recursive: true });
    writeFileSync(join(viewer, "🦀️.rs"), '.action_interactive_job("perSurfaceAction", InteractiveJobClassification::BatchOnlyPendingRewrite)');
    const action = (id: string, interactiveJob?: string) => ({ id, semantics: { execution: interactiveJob ? { interactiveJob } : {} } });
    const descriptor = { manifest: { apps: [{ id: "s.owner.app@1/*#editor", windowKinds: [{ actions: [action("migratedAction", "batchOnlyPendingRewrite"), action("batchAction", "batchOnlyPendingRewrite"), action("perSurfaceAction", "migrated"), action("migratedAction"), action("frameworkAction", "migrated"), action("unclassifiedAction", "unclassified")] }] }] } };
    const drift = auditInteractiveJobClassificationDrift("owner", ownerRoot, descriptor);
    expect(drift).toHaveLength(1);
    expect(drift[0]).toMatch(/s\.owner\.app@1\/\*#editor#migratedAction is committed as "batchOnlyPendingRewrite" but its Rust declares "migrated"/);
    expect(auditInteractiveJobClassificationDrift("owner", join(ownerRoot, "absent"), descriptor)).toEqual([]);
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
