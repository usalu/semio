/**
 * 🫱️ The Flow node-graph INTERACTION PUBLICATION contract: a surface tells the plugin what CHANGED,
 * so a click that moved the selection costs one hop, a click that moved nothing costs none, and a
 * mark the plugin itself made is never published back at it.
 *
 * The defect this pins: `emitInteractionState` dispatched `interactionSelect` AND `interactionHover`
 * on every non-pan pointer-up, and the same pointer-up reaches it twice (the pick hook's
 * `onSelectTarget` and the surface's own `onPointerUp`) — so one click on a node cost four guest
 * invocations and a plain click on empty canvas cost two, each a `performInvocation` → `refreshUi` →
 * React commit over the whole interaction scope (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️flow-scroll-render-perf-2026-09-15.md` §9, `📓️interaction-scope-narrowing-2026-09-15.md` §5).
 *
 * Driven through the REAL exported ledger — no browser, no wasm — so a regression fails here and not
 * only in the probe.
 *
 * @see `🐍️flow-surface-followup-probe.mjs` — the live twin, which counts the same hops on :6022
 */
import { describe, expect, it } from "vitest";
import { FLOW_SHARED_PAYLOAD_COALESCE_MS, createFlowSharedPayloadCoalescer, createNodeGraphInteractionLedger, nodeGraphHoverMarkKey, nodeGraphSelectionMarkKey, workflowNodesToDiagramNodes, type FlowSharedPayloadCoalescerPorts } from "../../🟦️.tsx";

describe("flow node-graph interaction publication", () => {
  it("publishes a selection that moved and nothing for one that did not", () => {
    const ledger = createNodeGraphInteractionLedger();
    expect(ledger.publishSelection({ nodeIds: ["a"] })).toBe(true);
    expect(ledger.publishSelection({ nodeIds: ["a"] })).toBe(false);
    expect(ledger.publishSelection({ nodeIds: ["a", "b"] })).toBe(true);
    expect(ledger.publishSelection({ nodeIds: [] })).toBe(true);
    expect(ledger.publishSelection({ nodeIds: [] })).toBe(false);
  });

  it("costs the plugin ONE hop for a click that selects and ZERO for the same click repeated", () => {
    const ledger = createNodeGraphInteractionLedger();
    // 🖱️ One pointer-up reaches the ledger twice — the pick hook and the surface's own handler read
    // the same session state and both ask to publish it.
    const clickOn = (nodeIds: readonly string[]) => [ledger.publishSelection({ nodeIds }), ledger.publishSelection({ nodeIds })].filter(Boolean).length;
    expect(clickOn(["a"])).toBe(1);
    expect(clickOn(["a"])).toBe(0);
    expect(clickOn(["b"])).toBe(1);
    expect(clickOn([])).toBe(1);
    expect(clickOn([])).toBe(0);
  });

  it("owes nothing for a plain click on empty canvas that changed neither lane", () => {
    const ledger = createNodeGraphInteractionLedger();
    ledger.adoptSelection({ nodeIds: [] });
    ledger.adoptHover({});
    expect(ledger.publishSelection({ nodeIds: [] })).toBe(false);
    expect(ledger.publishHover({})).toBe(false);
  });

  it("keeps selection and hover as independent lanes", () => {
    const ledger = createNodeGraphInteractionLedger();
    expect(ledger.publishSelection({ nodeIds: ["a"] })).toBe(true);
    expect(ledger.publishHover({ hoveredId: "a" })).toBe(true);
    expect(ledger.publishSelection({ nodeIds: ["a"] })).toBe(false);
    expect(ledger.publishHover({ hoveredId: "a" })).toBe(false);
    expect(ledger.publishHover({ hoveredId: "a", portId: "in" })).toBe(true);
    expect(ledger.publishSelection({ nodeIds: ["a"] })).toBe(false);
  });

  it("never re-publishes a mark the PLUGIN itself made", () => {
    const ledger = createNodeGraphInteractionLedger();
    ledger.adoptSelection({ nodeIds: ["a", "b", "c"] });
    expect(ledger.publishSelection({ nodeIds: ["a", "b", "c"] })).toBe(false);
    ledger.adoptHover({ hoveredId: "b", portId: "out" });
    expect(ledger.publishHover({ hoveredId: "b", portId: "out" })).toBe(false);
  });

  it("publishes again after the plugin moved the mark under the surface", () => {
    const ledger = createNodeGraphInteractionLedger();
    expect(ledger.publishSelection({ nodeIds: ["a"] })).toBe(true);
    // 🧾️ A keyboard `selectAll` in the plugin arrives on the next scene; the board then answers its
    // own single pick, which is a REAL change against what the plugin now holds.
    ledger.adoptSelection({ nodeIds: ["a", "b", "c"] });
    expect(ledger.publishSelection({ nodeIds: ["a"] })).toBe(true);
  });

  it("counts a domain the plugin never hears about as a different mark", () => {
    const ledger = createNodeGraphInteractionLedger();
    expect(ledger.publishSelection({ nodeIds: ["a"] })).toBe(true);
    expect(ledger.publishSelection({ nodeIds: ["a"], edgeIds: ["e1"] })).toBe(true);
    expect(ledger.publishSelection({ nodeIds: ["a"], edgeIds: ["e1"], handleIds: ["h1"] })).toBe(true);
    expect(ledger.publishSelection({ nodeIds: ["a"], edgeIds: ["e1"], handleIds: ["h1"] })).toBe(false);
  });

  it("keys a mark by exactly what the publication would carry", () => {
    expect(nodeGraphSelectionMarkKey({ nodeIds: ["a"] })).toBe(nodeGraphSelectionMarkKey({ nodeIds: ["a"], edgeIds: [], handleIds: [] }));
    expect(nodeGraphSelectionMarkKey({ nodeIds: ["a", "b"] })).not.toBe(nodeGraphSelectionMarkKey({ nodeIds: ["b", "a"] }));
    expect(nodeGraphHoverMarkKey({})).toBe(nodeGraphHoverMarkKey({ hoveredId: undefined, portId: undefined }));
    expect(nodeGraphHoverMarkKey({ hoveredId: "a" })).not.toBe(nodeGraphHoverMarkKey({ hoveredId: "a", portId: "in" }));
  });

  it("owes both lanes again after a retirement", () => {
    const ledger = createNodeGraphInteractionLedger();
    expect(ledger.publishSelection({ nodeIds: [] })).toBe(true);
    expect(ledger.publishHover({})).toBe(true);
    ledger.retire();
    expect(ledger.publishSelection({ nodeIds: [] })).toBe(true);
    expect(ledger.publishHover({})).toBe(true);
  });
});

/** 🎯️ The BOOT-SELECTION law. React Flow reports its selection once at mount, and the fallback board
 * used to dispatch that report unconditionally — so a selection the plugin had just restored was wiped
 * by a surface that had no selection of its own yet. Measured on generation3d (:6023, boot):
 * `interactionSelect {domainId:"graph", targets:"[]", merge:"replace"}` left the fallback 15 ms BEFORE
 * `node-graph host mount`, with no pointer event anywhere in the run and no `flow interaction publish`
 * line — i.e. not the wasm board, and not a user (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️hot-swap-board-remount-2026-09-15.md` §2).
 *
 * The fix is the ledger's own `adopt*` rule plus reflection: a mark that arrived FROM the plugin sets
 * the baseline, and the mounted nodes carry it, so the mount-time report EQUALS the baseline and owes
 * nothing. Both halves are asserted, and the counter-proof states what the unreflected board did. */
describe("flow node-graph boot selection", () => {
  const records = [
    { id: "height", x: 0, y: 0, label: "Height", inputs: [], outputs: [], width: 120, height: 40 },
    { id: "extrude", x: 10, y: 10, label: "Extrude", inputs: [], outputs: [], width: 120, height: 40 },
    { id: "column-preview", x: 20, y: 20, label: "Preview", inputs: [], outputs: [], width: 120, height: 40 },
  ] as never;
  /** 🖱️ What React Flow reports for a board mounted with these nodes: its own `selected` flags. */
  const mountReport = (selection: readonly string[]) => workflowNodesToDiagramNodes(records, selection).filter((entry) => entry.selected).map((entry) => entry.id);

  it("mounts the board carrying the selection the plugin already holds", () => {
    expect(mountReport(["extrude"])).toEqual(["extrude"]);
    expect(mountReport([])).toEqual([]);
    expect(workflowNodesToDiagramNodes(records).every((entry) => entry.selected === false)).toBe(true);
  });

  it("owes the plugin NOTHING for the mount-time report of a selection the plugin itself made", () => {
    const ledger = createNodeGraphInteractionLedger();
    ledger.adoptSelection({ nodeIds: ["extrude"] });
    expect(ledger.publishSelection({ nodeIds: mountReport(["extrude"]) })).toBe(false);
  });

  it("counter-proof: a board that does not reflect the adopted selection publishes the empty one that erased it", () => {
    const ledger = createNodeGraphInteractionLedger();
    ledger.adoptSelection({ nodeIds: ["extrude"] });
    const unreflected = workflowNodesToDiagramNodes(records).filter((entry) => entry.selected).map((entry) => entry.id);
    expect(unreflected).toEqual([]);
    expect(ledger.publishSelection({ nodeIds: unreflected }), "this `true` is the defect: an empty replace lands on the plugin").toBe(true);
  });

  it("still publishes a real user pick made after mount, and a real deselect after that", () => {
    const ledger = createNodeGraphInteractionLedger();
    ledger.adoptSelection({ nodeIds: ["extrude"] });
    expect(ledger.publishSelection({ nodeIds: mountReport(["extrude"]) })).toBe(false);
    expect(ledger.publishSelection({ nodeIds: ["height"] })).toBe(true);
    expect(ledger.publishSelection({ nodeIds: [] })).toBe(true);
    expect(ledger.publishSelection({ nodeIds: [] })).toBe(false);
  });

  it("re-adopts a selection the plugin changes underneath the board without owing a hop", () => {
    const ledger = createNodeGraphInteractionLedger();
    ledger.adoptSelection({ nodeIds: [] });
    ledger.adoptSelection({ nodeIds: ["column-preview"] });
    expect(ledger.publishSelection({ nodeIds: mountReport(["column-preview"]) })).toBe(false);
    console.log(`[DEBUG] boot selection: adopted=${JSON.stringify(["column-preview"])} mountReport=${JSON.stringify(mountReport(["column-preview"]))} owed=0`);
  });
});

describe("flow shared payload coalescing", () => {
  /** 🕰️ A clock the law advances by hand, plus the ledger of everything that actually crossed. */
  const harness = (settleMs = FLOW_SHARED_PAYLOAD_COALESCE_MS) => {
    type ScheduledRun = { readonly at: number; readonly run: () => void; cancelled: boolean };
    const sent: { feature: string; parts: readonly string[] }[] = [];
    const scheduled: ScheduledRun[] = [];
    let nowMs = 0;
    const ports: FlowSharedPayloadCoalescerPorts = {
      send: (feature, parts) => { sent.push({ feature, parts }); },
      schedule: (run, delayMs) => {
        const entry: ScheduledRun = { at: nowMs + delayMs, run, cancelled: false };
        scheduled.push(entry);
        return entry;
      },
      cancel: (handle) => { (handle as ScheduledRun).cancelled = true; },
    };
    const advance = (byMs: number) => {
      nowMs += byMs;
      for (const entry of [...scheduled]) {
        if (entry.cancelled || entry.at > nowMs) continue;
        entry.cancelled = true;
        entry.run();
      }
    };
    return { sent, advance, coalescer: createFlowSharedPayloadCoalescer(ports, settleMs) };
  };

  it("crosses a catalogue that arrives in stages exactly ONCE, with its final content", () => {
    const { sent, advance, coalescer } = harness();
    coalescer.offer("setNeuronKindInfosJson", ["starter"]);
    advance(8);
    coalescer.offer("setNeuronKindInfosJson", ["app", "scene"]);
    advance(8);
    coalescer.offer("setNeuronKindInfosJson", ["app-grown", "scene"]);
    expect(sent).toHaveLength(0);
    advance(FLOW_SHARED_PAYLOAD_COALESCE_MS);
    expect(sent).toHaveLength(1);
    expect(sent[0]!.parts).toEqual(["app-grown", "scene"]);
  });

  it("settles each feature on its own clock", () => {
    const { sent, advance, coalescer } = harness();
    coalescer.offer("setCatalogueJson", ["sections"]);
    advance(16);
    coalescer.offer("setNeuronKindInfosJson", ["operators"]);
    advance(FLOW_SHARED_PAYLOAD_COALESCE_MS - 8);
    expect(sent.map((entry) => entry.feature)).toEqual(["setCatalogueJson"]);
    advance(FLOW_SHARED_PAYLOAD_COALESCE_MS);
    expect(sent.map((entry) => entry.feature)).toEqual(["setCatalogueJson", "setNeuronKindInfosJson"]);
  });

  it("crosses two genuinely separate generations separately", () => {
    const { sent, advance, coalescer } = harness();
    coalescer.offer("setNeuronKindInfosJson", ["first"]);
    advance(FLOW_SHARED_PAYLOAD_COALESCE_MS);
    coalescer.offer("setNeuronKindInfosJson", ["second"]);
    advance(FLOW_SHARED_PAYLOAD_COALESCE_MS);
    expect(sent.map((entry) => entry.parts[0])).toEqual(["first", "second"]);
  });

  it("crosses nothing after disposal", () => {
    const { sent, advance, coalescer } = harness();
    coalescer.offer("setNeuronKindInfosJson", ["operators"]);
    expect(coalescer.pendingFeatures()).toEqual(["setNeuronKindInfosJson"]);
    coalescer.dispose();
    advance(FLOW_SHARED_PAYLOAD_COALESCE_MS * 4);
    expect(sent).toHaveLength(0);
    expect(coalescer.pendingFeatures()).toEqual([]);
  });
});
