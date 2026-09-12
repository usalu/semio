import { describe, expect, it } from "vitest";
import { exampleArtifactSources, resolveDocumentOperatorKinds, reachableKindsFromUnknown, scopeContributionsJson } from "../../🟦️.ts";
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
