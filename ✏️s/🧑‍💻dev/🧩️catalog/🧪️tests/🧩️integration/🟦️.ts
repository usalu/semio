import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { resolvePlaygroundBoot } from "@semio-tech/framework";
import { composeSpecificOsCatalogV1 } from "../../🟦️.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";
import { auditPluginCatalogSources, auditNavbarExamplePickerCoverage, auditNavbarExampleArtifactPayload } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts";
import { getWorkspaceRoot } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { examplesForApp, normalizeManifestExamples, type PluginManifest } from "../../../../../🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";
import { installedScope, sourceDeploymentOwnersV1 } from "../🧩️scope/🟦️.ts";
import { resolveFrameworkOsCatalogConfigurationV1 } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧩️catalog/🟦️.ts";
import { readGeneratedCatalogProjection } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
const composition = composeSpecificOsCatalogV1("http://localhost/", { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => false, progress: () => {} });
const catalog = composition.catalog;
describe("Specific original catalog completion", () => {
  it("independently enumerates every current deployed source identity and verifies each committed descriptor pair", async () => {
    const operation = new AbortController(), started = Date.now();
    const audit = auditPluginCatalogSources(getWorkspaceRoot(), { cancelled: () => operation.signal.aborted || Date.now() - started >= installedScope.sourceCensus.maxMs, progress: (completed, total) => console.log(`[DEBUG] Specific descriptor audit ${completed}/${total}`) });
    const missing = audit.issues.filter(({ code }) => code === "descriptor-pair-missing").map(({ pluginId }) => pluginId).sort();
    const owners = await sourceDeploymentOwnersV1({ signal: operation.signal, progress: completed => console.log(`[DEBUG] Specific source census ${completed}`) });
    expect(audit.manifestCount).toBe(owners.length);
    expect(audit.entries.map(row => row.pluginId).sort()).toEqual(owners);
    expect(audit.order).toHaveLength(owners.length);
    // 🔗️ `dependsOn` is the DECLARED runtime-actor set, never the crate's Cargo library links:
    // `sequence` links four `semio-s-plugin-imperative-*` rlibs and `raster` links `stdio`'s codecs,
    // and none of those crates' actors has to be loaded for them to run.
    expect(audit.entries.find(({ pluginId }) => pluginId === "sequence")?.dependsOn).toEqual([]);
    expect(audit.entries.find(({ pluginId }) => pluginId === "raster")?.dependsOn).toEqual([]);
    expect(audit.entries.find(({ pluginId }) => pluginId === "demonstrator")?.dependsOn).toEqual(["cad", "gis", "procedural", "process", "puzzle", "sourcing", "flow-extension-bim", "flow-extension-brep", "flow-extension-dictionary", "flow-extension-list", "flow-extension-logic", "flow-extension-math", "flow-extension-primitive", "flow-extension-text"]);
    expect(audit.entries.find(({ pluginId }) => pluginId === "cad-extension-aec-building")?.dependsOn).toEqual(["cad"]);
    expect(audit.issues.filter(({ code }) => code === "dependency-invalid")).toEqual([]);
    // 📇️ Every registry plugin is paired. The list this asserted used to hold 15 ids; A3b/CE1 closed
    // fourteen of them and `stdio` — the last one, and the only one that had NEVER been described —
    // landed on 2026-09-21 (`🎫️…/📓️pz1-catalog-zero-diagnostics.md` §1). An empty list is the
    // contract: a plugin the registry enumerates but cannot read is a plugin agents cannot reach.
    expect(missing).toEqual([]);
  }, 120_000);

  it("checked-in manifest examples carry dialect so the navbar picker can resolve them", () => {
    const pluginsRoot = join(getWorkspaceRoot(), "✏️s/🔌️plugins");
    const legacy: string[] = [];
    for (const entry of readdirSync(pluginsRoot, { withFileTypes: true })) {
      if (!entry.isDirectory()) continue;
      const descriptorPath = join(pluginsRoot, entry.name, "🔣️.json");
      if (!existsSync(descriptorPath)) continue;
      const descriptor = JSON.parse(readFileSync(descriptorPath, "utf8")) as { manifest?: PluginManifest & { examples?: readonly Record<string, unknown>[] } };
      const manifest = descriptor.manifest;
      if (!manifest?.pluginId || !Array.isArray(manifest.examples)) continue;
      for (const example of manifest.examples) {
        if (example && typeof example === "object" && "appId" in example && !("dialect" in example)) {
          legacy.push(`${manifest.pluginId}: example ${String(example.id ?? "?")} is legacy appId-only`);
        }
      }
    }
    expect(legacy, legacy.join("\n")).toEqual([]);
  });

  it("every playground editor dialect has at least one manifest example for NavbarExampleSelect", () => {
    const pluginsRoot = join(getWorkspaceRoot(), "✏️s/🔌️plugins");
    const coverage: string[] = [];
    for (const entry of readdirSync(pluginsRoot, { withFileTypes: true })) {
      if (!entry.isDirectory()) continue;
      const descriptorPath = join(pluginsRoot, entry.name, "🔣️.json");
      if (!existsSync(descriptorPath)) continue;
      const descriptor = JSON.parse(readFileSync(descriptorPath, "utf8")) as { manifest?: PluginManifest };
      const manifest = descriptor.manifest;
      if (!manifest?.pluginId) continue;
      coverage.push(...auditNavbarExamplePickerCoverage(manifest));
    }
    expect(coverage, coverage.join("\n")).toEqual([]);
  });

  it("every manifest example row carries a body — inline artifactJson, or a declared example-body asset once the body is over the inline ceiling — so setActiveExample can load a document", () => {
    const pluginsRoot = join(getWorkspaceRoot(), "✏️s/🔌️plugins");
    const payload: string[] = [];
    for (const entry of readdirSync(pluginsRoot, { withFileTypes: true })) {
      if (!entry.isDirectory()) continue;
      const descriptorPath = join(pluginsRoot, entry.name, "🔣️.json");
      if (!existsSync(descriptorPath)) continue;
      const descriptor = JSON.parse(readFileSync(descriptorPath, "utf8")) as { manifest?: PluginManifest; assets?: { name?: string }[] };
      const manifest = descriptor.manifest;
      if (!manifest?.pluginId) continue;
      payload.push(...auditNavbarExampleArtifactPayload(manifest, descriptor.assets ?? []));
    }
    expect(payload, payload.join("\n")).toEqual([]);
  });

  it("sourcing editor surface resolves the demo row the ShellHost navbar picker reads", () => {
    const descriptorPath = join(getWorkspaceRoot(), "🌎️hub/🧩️compositions/🪵️sourcing/🔣️.json");
    const descriptor = JSON.parse(readFileSync(descriptorPath, "utf8")) as { manifest?: PluginManifest };
    const manifest = descriptor.manifest;
    expect(manifest?.pluginId).toBe("sourcing");
    const admitted = normalizeManifestExamples(manifest!);
    const editor = admitted.apps?.find((app) => (app as { id?: string }).id === "s.sourcing.curation@1/*#editor");
    expect(editor).toBeTruthy();
    const options = examplesForApp(admitted.examples ?? [], editor as { dialect?: { artifactKind: string; standard: string; subset: string } });
    expect(options.map((row) => row.id)).toEqual(["demo"]);
    expect(options[0]?.artifactJson?.includes("curation.curation.dsl")).toBe(true);
  });

  it("resolves a non-empty, dependency-fault-free boot plan for every admitted installed playground variant — the wgpu frame Worker resolves this graph itself at boot, so an empty plan is a dead playground", () => {
    const deployed = readGeneratedCatalogProjection(join(getWorkspaceRoot(), "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated"));
    expect(composition.rows.targets.map(row => row.pluginId).sort()).toEqual([...installedScope.installedTargetIds].sort());
    expect(composition.rows.playgrounds.map(row => row.variant).sort()).toEqual([...installedScope.installedVariants].sort());
    expect(composition.rows.playgrounds.map(row => row.variant).sort()).toEqual(deployed.playgrounds.map(row => row.variant).sort());
    const faults = composition.rows.playgrounds.map((row) => ({ variant: row.variant, boot: resolvePlaygroundBoot(catalog, row.variant) }))
      .filter(({ boot }) => boot.plugins.length === 0 || boot.dependencyErrors.length > 0)
      .map(({ variant, boot }) => `${variant}: ${boot.plugins.length} plugins, ${JSON.stringify(boot.dependencyErrors)}`);
    expect(faults, faults.join("\n")).toEqual([]);
    const installed = new Set(composition.rows.targets.map(row => row.pluginId));
    for (const row of PLAYGROUND_BUILD_TARGETS.filter(row => !installed.has(row.pluginId))) {
      const operation = new AbortController();
      expect(() => resolveFrameworkOsCatalogConfigurationV1(composition.rows, row.variant, { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => operation.signal.aborted, progress: event => console.log(`[DEBUG] withheld ${row.variant} ${event.completed}/${event.total}`) }), row.variant).toThrow();
    }
  });

  it("keeps host plugin and extension edges one-way: an extension declares its host in `depends-on`, the host declares the contribution topic in `consumes` and never names the extension back", () => {
    const extensions = new Set(catalog.extensions.map((target) => target.pluginId));
    const backEdges = catalog.plugins.flatMap((target) =>
      (target.dependsOn ?? [])
        .filter((pluginId) => extensions.has(pluginId) && (catalog.extensions.find((row) => row.pluginId === pluginId)?.dependsOn ?? []).includes(target.pluginId))
        .map((pluginId) => `${target.pluginId} → ${pluginId} → ${target.pluginId}`),
    );
    expect(backEdges, backEdges.join("\n")).toEqual([]);
  });
});
