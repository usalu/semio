import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { resolvePlaygroundBoot } from "@semio-tech/framework";
import { composeSpecificOsCatalogV1 } from "../../🟦️.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../🤖️generated/🎮️playgrounds/🟦️.ts";
import { auditPluginCatalogSources } from "@semio-tech/plugin-registry/catalog-verification/source-audit";
import { auditNavbarExamplePickerCoverage, auditNavbarExampleArtifactPayload } from "@semio-tech/plugin-registry/catalog-verification/navbar-example-audits";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { examplesForApp, normalizeManifestExamples, type PluginManifest } from "@semio-tech/framework/manifest";
import { installedScope, sourceDeploymentOwnersV1 } from "../🧩️scope/🟦️.ts";
import { resolveFrameworkOsCatalogConfigurationV1 } from "@semio-tech/framework-os/modules/renderer/engine/elements/Shell/catalog";
import { admitPluginCatalogV1, pluginCatalogV1 } from "@semio-tech/plugin-registry/catalog";
const catalogAdmissionPort = { admit: admitPluginCatalogV1, catalogOf: pluginCatalogV1 };
import { readGeneratedCatalogProjection } from "@semio-tech/plugin-registry/catalog-view";
const composition = composeSpecificOsCatalogV1("http://localhost/", { maxBytes: 2097152, maxRows: 512, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => false, progress: () => {} });
const catalog = composition.catalog;
describe("Specific original catalog completion", () => {
  it("independently enumerates every current deployed source identity and verifies each committed descriptor pair", async () => {
    const operation = new AbortController(), started = Date.now();
    const audit = auditPluginCatalogSources(findWorkspaceRoot(), { cancelled: () => operation.signal.aborted || Date.now() - started >= installedScope.sourceCensus.maxMs, progress: (completed, total) => {} });
    const missing = audit.issues.filter(({ code }) => code === "descriptor-pair-missing").map(({ pluginId }) => pluginId).sort();
    const owners = await sourceDeploymentOwnersV1({ signal: operation.signal, progress: completed => {} });
    expect(audit.manifestCount).toBe(owners.length);
    expect(audit.entries.map(row => row.pluginId).sort()).toEqual(owners);
    expect(audit.order).toHaveLength(owners.length);
    expect(audit.entries.find(({ pluginId }) => pluginId === "sequence")?.dependsOn).toEqual([]);
    expect(audit.entries.find(({ pluginId }) => pluginId === "raster")?.dependsOn).toEqual([]);
    expect(audit.entries.find(({ pluginId }) => pluginId === "demonstrator")?.dependsOn).toEqual(["cad", "gis", "procedural", "process", "puzzle", "sourcing", "flow-extension-bim", "flow-extension-brep", "flow-extension-dictionary", "flow-extension-list", "flow-extension-logic", "flow-extension-math", "flow-extension-primitive", "flow-extension-text"]);
    expect(audit.entries.find(({ pluginId }) => pluginId === "cad-extension-aec-building")?.dependsOn).toEqual(["cad"]);
    expect(audit.issues.filter(({ code }) => code === "dependency-invalid")).toEqual([]);
    expect(missing).toEqual([]);
  }, 120_000);

  it("checked-in manifest examples carry dialect so the navbar picker can resolve them", () => {
    const pluginsRoot = join(findWorkspaceRoot(), "✏️s/🔌️plugins");
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
    const pluginsRoot = join(findWorkspaceRoot(), "✏️s/🔌️plugins");
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
    const pluginsRoot = join(findWorkspaceRoot(), "✏️s/🔌️plugins");
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
    const descriptorPath = join(findWorkspaceRoot(), "🌎️hub/🧩️compositions/🪵️sourcing/🧩️component/🔣️.json");
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
    const deployed = readGeneratedCatalogProjection(join(findWorkspaceRoot(), "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"));
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
      expect(() => resolveFrameworkOsCatalogConfigurationV1(catalogAdmissionPort, composition.rows, row.variant, { maxBytes: 2097152, maxRows: 512, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => operation.signal.aborted, progress: event => {} }), row.variant).toThrow();
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
