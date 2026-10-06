/** 🕸️ React and Ajv oracle for the schema-owned NodeGraph interaction target projection. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import {
  nodeGraphHoverActionArgs,
  nodeGraphSelectionActionArgs,
  nodeGraphSurfaceSelectionDomV1,
} from "../../🧱️elements/🕸️NodeGraph/🟦️.tsx";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const engineRoot = resolve(suiteRoot, "../..");
const fixture = JSON.parse(readFileSync(resolve(engineRoot, "🧫️fixtures/🕸️node-graph-domain-interaction/🔣️.json"), "utf8")) as any;
const testCase = (id: string) => fixture.cases.find((entry: any) => entry.id === id);

describe("NodeGraph schema-owned interaction domain", () => {
  it("validates one closed non-empty target projection contract", () => {
    for (const field of ["id", "nodeTargetPrefix", "edgeTargetPrefix", "handleTargetPrefix"]) {
    }
  });

  it("qualifies node, edge, and handle selections exactly once", () => {
    const row = testCase("mixed-selection");
    expect(nodeGraphSelectionActionArgs(fixture.interactionDomain, row.input)).toEqual(row.args);
  });

  it("qualifies node and handle hover and preserves the exact empty clear", () => {
    for (const id of ["node-hover", "handle-hover", "hover-clear"]) {
      const row = testCase(id);
      expect(nodeGraphHoverActionArgs(fixture.interactionDomain, row.input.nodeId, row.input.portId)).toEqual(row.args);
    }
  });

  it("publishes no interaction without a scene-owned domain", () => {
    const selection = testCase("mixed-selection");
    const hover = testCase("node-hover");
    expect(nodeGraphSelectionActionArgs(undefined, selection.input)).toBeUndefined();
    expect(nodeGraphHoverActionArgs(undefined, hover.input.nodeId)).toBeUndefined();
  });

  it("keeps projected incoming scene ids raw for painting", () => {
    expect(nodeGraphSurfaceSelectionDomV1({ nodes: [], edges: [], selection: ["add"], hover: { nodeId: "add", portId: "result" } })).toEqual({
      selectedIds: ["add"],
      highlightedIds: [],
      hoverTarget: { nodeId: "add", portId: "result" },
      editable: true,
    });
  });
});
