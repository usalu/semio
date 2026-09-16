import { describe, expect, it } from "vitest";
import { contributionIsCapabilityPack, exampleArtifactSources, resolveDocumentOperatorKinds, reachableKindsFromUnknown, scopeContributionsJson } from "../../🟦️.ts";
import type { PluginManifest } from "../../../🛂️manifest/🟦️.ts";

describe("exampleArtifactSources", () => {
  const generation3d = { artifactKind: "s.procedural.generation3d", standard: "1", subset: "*" } as const;
  const examples = [
    { id: "box-shell-preview", dialect: generation3d, artifactJson: "neuron id=box neuron-kind=brep.prim3d.box neuron-kind=brep.solid.shell" },
    { id: "face-sweep-extrude", dialect: generation3d, artifactJson: "neuron-kind=brep.surf.planarFaceWire neuron-kind=brep.sweep.extrude" },
  ];
  it("reads neuron-kind from published example artifactJson", () => {
    const sources = exampleArtifactSources(examples, generation3d, "box-shell-preview");
    expect(resolveDocumentOperatorKinds(sources)).toEqual({ status: "resolved", kinds: ["brep.prim3d.box", "brep.solid.shell"] });
  });
  it("uses the same dialect graphs when the open app is the viewer of that artifact", () => {
    const sources = exampleArtifactSources(examples, generation3d);
    const scope = resolveDocumentOperatorKinds(sources);
    expect(scope.status).toBe("resolved");
    if (scope.status === "resolved") {
      expect(scope.kinds).toContain("brep.prim3d.box");
      expect(scope.kinds).toContain("brep.surf.planarFaceWire");
    }
  });
});

describe("scopeContributionsJson", () => {
  const manifest = (topic: string, payload: unknown) =>
    ({ topicContributions: [{ topic, payload }], apps: [], workflows: [] }) as PluginManifest;
  const loaded = [
    { pluginId: "procedural", manifest: manifest("flow.extension", { operators: [{ kind: "procedural.example" }] }) },
    { pluginId: "flow-extension-brep", manifest: manifest("flow.extension", { operators: [{ kind: "brep.solid.extrude" }, { kind: "brep.curve.polygon" }] }) },
    { pluginId: "flow-extension-bim", manifest: manifest("flow.extension", { operators: [{ kind: "bim.wall" }] }) },
    { pluginId: "flow-extension-math", manifest: manifest("flow.extension", { operators: [{ kind: "math.vector" }] }) },
  ];
  it("keeps the receiver and only plugins whose operators the document graph can reach", () => {
    const kinds = reachableKindsFromUnknown([
      { widgets: [{ neuronKind: "brep.solid.extrude" }, { neuronKind: "math.vector" }] },
    ]);
    expect(kinds.sort()).toEqual(["brep.solid.extrude", "math.vector"]);
    const scoped = JSON.parse(scopeContributionsJson(loaded, "procedural", kinds)) as { pluginId: string }[];
    expect(scoped.map((entry) => entry.pluginId).sort()).toEqual(["flow-extension-brep", "flow-extension-math", "procedural"]);
  });
  it("drops every foreign contribution when the graph names no operator kind", () => {
    const scoped = JSON.parse(scopeContributionsJson(loaded, "procedural", [])) as { pluginId: string }[];
    expect(scoped.map((entry) => entry.pluginId)).toEqual(["procedural"]);
  });
  const capabilities = [
    { pluginId: "process-extension-wood", manifest: manifest("process.machines", { appId: "s.process.process3d@1/*#editor", moduleId: "wood", catalogJson: JSON.stringify({ machines: [{ id: "cnc-router", label: "CNC router" }] }) }) },
    { pluginId: "cad-extension-aec-building", manifest: manifest("cad.computer", { appId: "s.cad.cad@1/*#editor", moduleId: "aec-building", computersJson: JSON.stringify([{ id: "floor-area" }]) }) },
    { pluginId: "sourcing-module-beams", manifest: manifest("sourcing.module", { appId: "sourcing-curation", moduleId: "beams", typology: ["structure", "beams"] }) },
  ];
  // 🎛️ The demonstrator's own `consumes` row, verbatim from the generated plugin registry.
  const demonstratorConsumes = ["forms.questionKind", "flow.extension", "process.machines", "cad.computer", "sourcing.module"];
  it("passes a capability pack the receiver consumes — no operator graph can ever scope one", () => {
    for (const entry of capabilities) expect(contributionIsCapabilityPack(entry.manifest.topicContributions![0])).toBe(true);
    for (const entry of loaded) expect(contributionIsCapabilityPack(entry.manifest.topicContributions![0])).toBe(false);
    // 🕸️ The cad/process/sourcing receivers resolve NO operator graph at all; the pack crosses anyway.
    const scoped = JSON.parse(scopeContributionsJson([...capabilities, loaded[2]!], "demonstrator", [], demonstratorConsumes)) as { pluginId: string }[];
    expect(scoped.map((entry) => entry.pluginId)).toEqual(["process-extension-wood", "cad-extension-aec-building", "sourcing-module-beams"]);
  });
  it("cuts a capability pack on a topic the receiver does not consume", () => {
    // 📐️ `gis` contributes a 196 400-byte `stdio.artifact-catalog.v1` no plugin consumes; forwarding
    // every capability pack put it in all four demonstrator apps and blew their wire admission.
    const gis = { pluginId: "gis", manifest: manifest("stdio.artifact-catalog.v1", { catalogJson: "x".repeat(512) }) };
    const scoped = JSON.parse(scopeContributionsJson([...capabilities, gis], "demonstrator", [], demonstratorConsumes)) as { pluginId: string }[];
    expect(scoped.map((entry) => entry.pluginId)).not.toContain("gis");
    expect(JSON.parse(scopeContributionsJson([...capabilities, gis], "sourcing", [], ["sourcing.module"])).map((entry: { pluginId: string }) => entry.pluginId)).toEqual(["sourcing-module-beams"]);
  });
  it("still cuts an unreachable operator pack when capability packs pass alongside it", () => {
    const kinds = reachableKindsFromUnknown([{ widgets: [{ neuronKind: "brep.solid.extrude" }] }]);
    const scoped = JSON.parse(scopeContributionsJson([...loaded, ...capabilities], "demonstrator", kinds, demonstratorConsumes)) as { pluginId: string }[];
    expect(scoped.map((entry) => entry.pluginId).sort()).toEqual(["cad-extension-aec-building", "flow-extension-brep", "process-extension-wood", "sourcing-module-beams"]);
    expect(JSON.stringify(scoped).includes("bim.wall")).toBe(false);
    expect(JSON.stringify(scoped).includes("math.vector")).toBe(false);
  });
  it("reaches operators nested in a flow-extension manifestJson string", () => {
    const brep = {
      pluginId: "flow-extension-brep",
      manifest: manifest("flow.extension", { manifestJson: JSON.stringify({ contributes: { operators: [{ id: "brep.solid.extrude" }, { id: "brep.curve.polygon" }] } }) }),
    };
    const kinds = reachableKindsFromUnknown([{ widgets: [{ "neuron-kind": "brep.solid.extrude" }] }, "neuron-kind=brep.solid.extrude"]);
    expect(kinds).toContain("brep.solid.extrude");
    const scoped = JSON.parse(scopeContributionsJson([loaded[0]!, brep, loaded[2]!], "procedural", kinds)) as { pluginId: string }[];
    expect(scoped.map((entry) => entry.pluginId).sort()).toEqual(["flow-extension-brep", "procedural"]);
  });
  it("resolves neuron-kind from the open document DSL and refuses a missing graph", () => {
    const dsl =
      'widgets {\n  neuron id="profile" neuron-kind=brep.curve.polygon\n  neuron id="extrusion-axis" neuron-kind=math.vector\n  neuron id="extrude" neuron-kind=brep.solid.extrude\n}';
    const fromDsl = resolveDocumentOperatorKinds([dsl]);
    expect(fromDsl.status).toBe("resolved");
    if (fromDsl.status === "resolved") expect([...fromDsl.kinds].sort()).toEqual(["brep.curve.polygon", "brep.solid.extrude", "math.vector"]);
    const emptyGraph = resolveDocumentOperatorKinds([{ fixture: { widgets: [] } }]);
    expect(emptyGraph).toEqual({ status: "resolved", kinds: [] });
    expect(resolveDocumentOperatorKinds([{ surface: "lane-split" }])).toEqual({ status: "unresolved", reason: "no-operator-graph" });
    expect(resolveDocumentOperatorKinds([])).toEqual({ status: "unresolved", reason: "no-document-sources" });
  });
});
