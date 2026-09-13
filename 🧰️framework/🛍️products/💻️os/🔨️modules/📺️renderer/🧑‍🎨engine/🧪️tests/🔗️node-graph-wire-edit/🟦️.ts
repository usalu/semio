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
  readonly grab: { readonly rule: string; readonly zooms: readonly { readonly zoom: number }[]; readonly cases: readonly { readonly name: string; readonly gesture: { readonly kind: string } }[] };
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

  it("dispatches a released gesture's own wire edits, and falls back to the fixture commit only when it made none", () => {
    // 🩸️ React committed EVERY released gesture as a whole-fixture `setFixture`, so a drawn wire
    // reached the guest only as a state blob — a different vocabulary from the one wgpu dispatches and
    // from the one the guest declares. `pointerUpScreen` now answers with what the gesture did.
    const source = readFileSync(nodeGraphSource, "utf8");
    const handler = source.slice(source.indexOf('observeFlowTask(session, "pointerUpScreen"'));
    const body = handler.slice(0, handler.indexOf("renderFlow()"));
    expect(body).toContain("graphEditOperations(value)");
    expect(body).toContain("dispatch(nodeGraphActions.edit, { operations })");
    expect(body).toContain("else commitFixture()");
  });

  it("holds the pointer for the whole gesture, so a drag that leaves the canvas still releases on it", () => {
    // 🩸️ Measured on 6018: a press on a port entered `InteractionMode::DrawEdge` and the release,
    // landing over the outline tree beside the canvas, never reached `pointer_up_screen` at all — the
    // wire stayed in flight and the gesture made no edit. Cutting a wire means dragging it AWAY, so
    // the gesture that most needs to leave the canvas was the one that could never finish.
    const source = readFileSync(nodeGraphSource, "utf8");
    const down = source.slice(source.indexOf('observeFlowTask(session, "pointerDownScreen"') - 1400, source.indexOf('observeFlowTask(session, "pointerDownScreen"'));
    expect(down).toContain("setPointerCapture(event.pointerId)");
    const up = source.slice(source.indexOf('observeFlowTask(session, "pointerUpScreen"') - 900, source.indexOf('observeFlowTask(session, "pointerUpScreen"'));
    expect(up).toContain("releasePointerCapture(event.pointerId)");
  });

  it("spells a removal the way the guest reads it, in both renderers", () => {
    // 🩸️ The wgpu renderer wrote `edgeId` — the engine's private numbering's name — while the guest's
    // `disconnect` reads `synapseId`: a well-formed command silently dropped.
    const guest = readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs"), "utf8");
    const guestDisconnect = guest.slice(guest.indexOf('"disconnect" =>'));
    const guestField = /operation\.get\("(\w+)"\)/u.exec(guestDisconnect)?.[1];
    expect(guestField).toBe("synapseId");
    const wgpu = readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"), "utf8");
    expect(wgpu).toContain(`builder.string(Some("${guestField}"), synapse_id)?`);
    expect(law.rules.disconnectNamesTheSynapse).toContain(guestField!);
  });

  it("requires the grabbable geometry to be the published geometry, across the zoom bands", () => {
    expect(law.grab.rule).toBe("publishedPortGeometryIsGrabbable");
    expect(law.rules[law.grab.rule]).toBeTruthy();
    expect(law.grab.zooms.map((band) => band.zoom)).toEqual([0.5, 1, 2]);
    const grabbed = law.grab.cases.map((testCase) => testCase.gesture.kind);
    expect(grabbed).toContain("grabCentre");
    expect(grabbed).toContain("detach");
  });

  it("separates a minimap click that navigates from one that grabs the viewport rectangle", () => {
    expect(law.minimap.cases.map((testCase) => testCase.expectedCameraMoves)).toEqual([true, false]);
  });

  it("asks the path discriminator on every phase, the way React drives pointer_*_screen on every phase", () => {
    // 🩸️ The half no Rust law can reach: the wgpu renderer decides per PHASE which of the host's two
    // pointer entries to use. Asking only on press let a plain hover across a port fall through to
    // the bounded path, which faults — measured on 6118 as
    // `wgpu-shell graph move fault surface=procedural-main fault=Structure`, after which the frame
    // loop published nothing further.
    const canvas = readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"), "utf8");
    const asks = [...canvas.matchAll(/node_graph_gesture_is_screen_path\(surface_id, ([^)]*\)?)\)/gu)].map((match) => match[1]);
    // 🖐️ Three PHASE questions — down, move, up — each asked about the point the pointer is at, plus
    // the one the screen-path dispatcher asks with no point at all: "is a gesture already in flight?",
    // which is what tells a plain hover (which can edit nothing) from a move inside a live wire draw.
    expect(asks.filter((argument) => argument === "Some((sx, sy)")).toHaveLength(3);
    expect(asks.filter((argument) => argument === "None")).toHaveLength(1);
    expect(law.rules.everyPhaseOverAnUnsupportedHit).toContain("EVERY phase");
  });
});
