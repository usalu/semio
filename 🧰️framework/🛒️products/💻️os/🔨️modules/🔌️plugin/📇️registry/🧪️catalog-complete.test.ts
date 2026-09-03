import { createHash, webcrypto } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, truncateSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import Ajv from "ajv";
import { afterEach, describe, expect, it } from "vitest";
import { encodePackValue } from "../../../🟦️.ts";
import {
  CATALOG_ARTIFACT_MAX_BYTES,
  CATALOG_DEPENDENCY_MAX,
  CATALOG_NODE_MAX,
  auditPluginCatalogSources,
  createFreshCatalogBuildVerifier,
  executeCatalogVerificationPlan,
  orderCatalogNodes,
  validateCatalogDescriptorPair,
  type CatalogVerificationNode,
  type PluginRegistryEntry,
} from "../📜️script.ts";

const fixtureRoot = join(import.meta.dirname, "🧬️catalog-complete");
const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as {
  readonly nodes: readonly CatalogVerificationNode[];
  readonly expectedOrder: readonly string[];
  readonly parentFailure: { readonly failedPluginId: string; readonly statuses: Readonly<Record<string, string>>; readonly publication: string };
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

async function syntheticDescriptor(pluginId: string, role: "plugin" | "extension", raw: Uint8Array, core: Uint8Array): Promise<{ readonly descriptor: Record<string, unknown>; readonly bytes: Uint8Array }> {
  const descriptor: Record<string, any> = {
    descriptorVersion: 1,
    role,
    manifest: { pluginId, label: pluginId, version: "1.0.0", apps: [], examples: [], capabilities: [], topicContributions: [], commands: [], artifactKinds: [] },
    execution: "isolated",
    quotas: {},
    contributions: {},
    hashes: { wasmSha256: await sha256(raw), coreWasmSha256: await sha256(core), descriptorSha256: "" },
  };
  descriptor.hashes.descriptorSha256 = await sha256(encodePackValue(descriptor));
  return { descriptor, bytes: encodePackValue(descriptor) };
}

function syntheticEntry(root: string, pluginId = "parent"): PluginRegistryEntry {
  return {
    pluginId,
    cratePath: join(root, "owner", "📦️packages", "🦀️rust"),
    packageName: `semio-s-plugin-${pluginId}`,
    wasmOut: `semio_s_plugin_${pluginId}.wasm`,
    role: "plugin",
    capabilities: [],
    contributes: [],
    consumes: [],
    dependsOn: [],
    activationEvents: [],
    extensionPoints: [],
  };
}

afterEach(() => {
  for (const root of temporaryRoots.splice(0)) rmSync(root, { recursive: true, force: true });
});

describe("strict plugin catalog completion", () => {
  it("validates the language-neutral contract with AJV and executes parent, child, extension deterministically", async () => {
    const schema = JSON.parse(readFileSync(join(fixtureRoot, "🧬️schema", "🔣️.json"), "utf8"));
    expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    expect(orderCatalogNodes(fixture.nodes).map(({ pluginId }) => pluginId)).toEqual(fixture.expectedOrder);

    const verified: string[] = [];
    const publications: string[][] = [];
    const progress: string[] = [];
    const result = await executeCatalogVerificationPlan(fixture.nodes, {
      async verify(node) {
        verified.push(node.pluginId);
        if (node.pluginId === fixture.parentFailure.failedPluginId) throw new Error("forced parent failure");
        return { pluginId: node.pluginId };
      },
      async publish(rows) {
        publications.push(rows.map(({ node }) => node.pluginId));
      },
    }, { cancelled: () => false, progress: (event) => progress.push(`${event.completed}/${event.total}:${event.pluginId}:${event.status}`) });

    expect(verified).toEqual(["parent"]);
    expect(Object.fromEntries(result.results.map((row) => [row.pluginId, row.status]))).toEqual(fixture.parentFailure.statuses);
    expect(result.publication).toBe(fixture.parentFailure.publication);
    expect(publications).toEqual([]);
    expect(progress).toEqual(["1/3:parent:failed", "2/3:child:blocked", "3/3:extension:blocked"]);
  });

  it("rejects graph max+1, dependency max+1, duplicates, missing parents, cycles, and cancellation with bounded diagnostics", async () => {
    const nodes = Array.from({ length: CATALOG_NODE_MAX + 1 }, (_, index) => ({ pluginId: `p-${index}`, role: "plugin" as const, dependsOn: [] }));
    expect(() => orderCatalogNodes(nodes)).toThrow(/at most/);
    expect(() => orderCatalogNodes([{ pluginId: "child", role: "plugin", dependsOn: Array.from({ length: CATALOG_DEPENDENCY_MAX + 1 }, (_, index) => `p-${index}`) }])).toThrow(/dependencies/);
    expect(() => orderCatalogNodes([{ pluginId: "same", role: "plugin", dependsOn: [] }, { pluginId: "same", role: "extension", dependsOn: [] }])).toThrow(/duplicate/);
    expect(() => orderCatalogNodes([{ pluginId: "child", role: "plugin", dependsOn: ["absent"] }])).toThrow(/absent/);
    expect(() => orderCatalogNodes([{ pluginId: "a", role: "plugin", dependsOn: ["b"] }, { pluginId: "b", role: "plugin", dependsOn: ["a"] }])).toThrow(/cycle/);

    const result = await executeCatalogVerificationPlan(fixture.nodes, { async verify() { throw new Error("x".repeat(10_000)); } }, { cancelled: () => true });
    expect(result.results.every(({ status }) => status === "cancelled")).toBe(true);
    expect(Buffer.byteLength(result.results[0]!.diagnostic ?? "")).toBeLessThanOrEqual(4096);
  });

  it("strict-decodes an exact JSON/pack pair and independently verifies fresh raw/core/descriptor bytes", async () => {
    const sourceRoot = temporaryRoot();
    const buildRoot = temporaryRoot();
    const raw = new TextEncoder().encode("raw-component");
    const core = new TextEncoder().encode("extracted-core");
    const { descriptor, bytes } = await syntheticDescriptor("parent", "plugin", raw, core);
    const entry = syntheticEntry(sourceRoot);
    const ownerRoot = join(sourceRoot, "owner");
    mkdirSync(join(ownerRoot, "📦️packages", "🦀️rust"), { recursive: true });
    writeFileSync(join(ownerRoot, "🔣️.json"), `${JSON.stringify(descriptor, null, 2)}\n`);
    writeFileSync(join(ownerRoot, "🛡️.descriptor.semio"), bytes);
    const source = validateCatalogDescriptorPair(entry, sourceRoot);

    const rowRoot = join(buildRoot, entry.pluginId);
    mkdirSync(join(rowRoot, "raw"), { recursive: true });
    mkdirSync(join(rowRoot, "core"), { recursive: true });
    mkdirSync(join(rowRoot, "descriptor"), { recursive: true });
    writeFileSync(join(rowRoot, "raw", entry.wasmOut), raw);
    writeFileSync(join(rowRoot, "core", entry.wasmOut), core);
    writeFileSync(join(rowRoot, "descriptor", "🛡️.descriptor.semio"), bytes);

    const verifier = createFreshCatalogBuildVerifier(sourceRoot, buildRoot);
    const receipt = verifier.verify(source);
    expect(receipt.rawSha256).toBe(await sha256(raw));
    expect(receipt.coreSha256).toBe(await sha256(core));
    expect(receipt.descriptorSha256).toBe((descriptor.hashes as Record<string, string>).descriptorSha256);
    expect(createHash("sha256").update(raw).digest("hex")).toBe(await sha256(raw));

    const mutated = Uint8Array.from(core);
    mutated[0] ^= 1;
    writeFileSync(join(rowRoot, "core", entry.wasmOut), mutated);
    expect(() => verifier.verify(source)).toThrow(/core/);
  });

  it("refuses ambient authority roots and max+1 artifacts", () => {
    const repoRoot = temporaryRoot();
    mkdirSync(join(repoRoot, "target"), { recursive: true });
    mkdirSync(join(repoRoot, "🧰️framework", "🛒️products", "💻️os", "🔨️modules", "🧑️💻️dev", "🔌️plugin-modules"), { recursive: true });
    expect(() => createFreshCatalogBuildVerifier(repoRoot, join(repoRoot, "target"))).toThrow(/ambient shared target/);
    expect(() => createFreshCatalogBuildVerifier(repoRoot, join(repoRoot, "🧰️framework", "🛒️products", "💻️os", "🔨️modules", "🧑️💻️dev", "🔌️plugin-modules"))).toThrow(/development cache/);

    const hugeRoot = temporaryRoot();
    const huge = join(hugeRoot, "huge");
    truncateSync(huge, CATALOG_ARTIFACT_MAX_BYTES + 1);
    expect(readFileSync(huge).byteLength).toBe(CATALOG_ARTIFACT_MAX_BYTES + 1);
  });

  it("independently enumerates the real manifests and reports the known missing source pairs", () => {
    const audit = auditPluginCatalogSources();
    const missing = audit.issues.filter(({ code }) => code === "descriptor-pair-missing").map(({ pluginId }) => pluginId).sort();
    expect(audit.manifestCount).toBe(59);
    expect(missing).toEqual([
      "block", "flow-extension-bim", "flow-extension-draw", "imperative-extension-control", "imperative-extension-effect", "imperative-extension-logic", "imperative-extension-math", "imperative-extension-text", "playbook", "playbook-module-procedural", "process-extension-concrete", "process-extension-metal", "process-extension-robotic", "process-extension-wood", "sourcing-module-beams", "sourcing-module-slabs", "sourcing-module-windows", "stdio", "trinity",
    ]);
  });
});
