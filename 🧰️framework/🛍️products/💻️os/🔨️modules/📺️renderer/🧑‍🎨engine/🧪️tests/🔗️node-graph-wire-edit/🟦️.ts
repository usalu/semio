/**
 * 🔗️ The TypeScript twin of
 * `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs`. Both read the SAME neutral
 * fixture (`…/🕸️dag/🧫️fixtures/🔗️wire-edit/🔣️.json`): the Rust law drives the live `DagHost` through
 * real port hit-testing, a real `InteractionMode::DrawEdge` and a real minimap layout, and asserts
 * what the host journalled; this one pins the OTHER half of the same contract — that the operation
 * vocabulary the oracle expects is byte-for-byte the vocabulary React already dispatches from
 * ReactFlow's `onConnect`, and the one the guest's `FlowNodeGraphEditOp` decodes.
 *
 * That is the half a Rust-only law cannot prove: the wgpu renderer builds this payload by hand into
 * a bounded action, so nothing in Rust would catch it drifting away from the shape React sends. It
 * is read out of `🕸️NodeGraph/🟦️.tsx`'s own source rather than restated here, so a rename on either
 * side fails this test instead of silently splitting the two renderers.
 *
 * The defect: wgpu's node-graph dispatch called only the bounded `plan_pointer`/`commit_pointer`
 * entry, which cannot reach ports at all — port-to-port wire creation, wire removal and minimap
 * click-to-navigate were dead from its side (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️audit-wgpu-parity-2026-09-13.md` gaps #2/#5).
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const dagRoot = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag");
const nodeGraphSource = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx");

type EditRow = Record<string, string>;
type FixtureCase = { readonly name: string; readonly gesture: { readonly kind: string; readonly from?: string; readonly to?: string }; readonly expectedEdits?: readonly EditRow[]; readonly expectedEdges?: readonly { readonly source: string; readonly target: string }[] };

const law = JSON.parse(readFileSync(resolve(dagRoot, "🧫️fixtures/🔗️wire-edit/🔣️.json"), "utf8")) as {
  readonly rules: Record<string, string>;
  readonly graph: { readonly nodes: readonly { readonly id: string; readonly inputs: readonly string[]; readonly outputs: readonly string[] }[] };
  readonly cases: readonly FixtureCase[];
  readonly minimap: { readonly cases: readonly { readonly name: string; readonly expectedCameraMoves: boolean }[] };
};

/** 🔗️ The argument names React's own `onConnect` dispatches a created wire under — read out of its source, not restated. */
const reactConnectFields = (): readonly string[] => {
  const source = readFileSync(nodeGraphSource, "utf8");
  const block = source.slice(source.indexOf('operation: "connect"'));
  const fields = [...block.slice(0, block.indexOf("]")).matchAll(/^\s*(\w+):\s/gmu)].map((match) => match[1]!);
  return fields.filter((field) => field !== "operation");
};

/** 🔌️ Splits a `"<nodeId>@<portId>"` endpoint — the one grammar the graph reports hover, picks and wire ends in. */
const splitEndpoint = (endpoint: string): readonly [string, string] => {
  const at = endpoint.lastIndexOf("@");
  return [endpoint.slice(0, at), endpoint.slice(at + 1)];
};

describe("node graph wire edit", () => {
  it("dispatches a created wire under the very field names React's onConnect uses", () => {
    expect(reactConnectFields()).toEqual(["sourceNodeId", "sourcePortId", "targetNodeId", "targetPortId"]);
  });

  it("names every expected connect with those same four fields and nothing else", () => {
    const connects = law.cases.flatMap((testCase) => testCase.expectedEdits ?? []).filter((edit) => edit.operation === "connect");
    expect(connects.length).toBeGreaterThan(0);
    for (const edit of connects) {
      expect(Object.keys(edit).sort()).toEqual(["operation", "sourceNodeId", "sourcePortId", "targetNodeId", "targetPortId"]);
    }
  });

  it("resolves every wire endpoint to a port the authored graph actually declares", () => {
    const outputs = new Map(law.graph.nodes.map((node) => [node.id, node.outputs]));
    const inputs = new Map(law.graph.nodes.map((node) => [node.id, node.inputs]));
    for (const testCase of law.cases) {
      if (testCase.gesture.kind !== "wire") continue;
      const [fromNode, fromPort] = splitEndpoint(testCase.gesture.from!);
      const [toNode, toPort] = splitEndpoint(testCase.gesture.to!);
      expect(outputs.get(fromNode), `${testCase.name}: ${fromNode}`).toContain(fromPort);
      expect(inputs.get(toNode), `${testCase.name}: ${toNode}`).toContain(toPort);
    }
  });

  it("keeps an input port holding exactly one incoming wire", () => {
    const replacing = law.cases.find((testCase) => testCase.name === "a-second-wire-into-the-same-input-replaces-the-first");
    expect(replacing?.expectedEdges).toHaveLength(1);
    expect(replacing?.expectedEdges?.[0]?.target).toBe("tgt@in");
    expect(replacing?.expectedEdges?.[0]?.source).toBe("alt@out");
  });

  it("separates a minimap click that navigates from one that grabs the viewport rectangle", () => {
    expect(law.minimap.cases.map((testCase) => testCase.expectedCameraMoves)).toEqual([true, false]);
  });
});
