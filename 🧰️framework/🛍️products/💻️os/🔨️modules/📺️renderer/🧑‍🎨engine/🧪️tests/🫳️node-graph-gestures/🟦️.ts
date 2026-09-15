/**
 * 🫳️ The TypeScript twin of
 * `⚙️EngineCanvas/🧪️tests/🫳️node-graph-gestures/🦀️.rs`. Both read the SAME neutral fixture
 * (`🧑‍🎨engine/🧫️fixtures/🫳️node-graph-gestures/🔣️.json`): the Rust law drives the REAL wgpu pointer
 * entries over the REAL `FlowHost` and asserts what each gesture published; this one pins the two
 * halves a Rust law cannot reach.
 *
 * 1. THE VOCABULARY. wgpu hand-builds every `nodeGraphEdit` sub-operation into a bounded action, so
 *    nothing in Rust catches it drifting from the shape the GUEST decodes. Each operation's name and
 *    field names are read out of generation3d's own `node-graph-edit` command source and out of
 *    React's `NodeGraph`, never restated here — a rename on either side fails this test instead of
 *    silently splitting the two renderers.
 * 2. THE ADMISSION RULE. That a plain pointer move must not be priced like a mutation is a property
 *    of the dispatch code's SHAPE — no reservation before the change is known — and this pins that
 *    shape in the one file that owns it.
 *
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, lane `wgpu-node-graph-gestures`.
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const engineRoot = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
const canvasSource = resolve(engineRoot, "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs");
const guestSource = resolve(repoRoot, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs");

type EditRow = Record<string, unknown>;
type FixtureCase = { readonly name: string; readonly rule: string; readonly gesture: Record<string, unknown>; readonly expectedEdits?: readonly EditRow[]; readonly expectedSelection?: readonly string[]; readonly expectedCameraMoves?: boolean };

const law = JSON.parse(readFileSync(resolve(engineRoot, "🧫️fixtures/🫳️node-graph-gestures/🔣️.json"), "utf8")) as {
  readonly rules: Record<string, string>;
  readonly graph: Record<string, unknown> & { readonly freeInputs: readonly string[]; readonly wiredInputs: readonly string[]; readonly outputs: readonly string[] };
  readonly cases: readonly FixtureCase[];
  readonly operationFields: Record<string, readonly string[]>;
};

const canvas = readFileSync(canvasSource, "utf8");
const guest = readFileSync(guestSource, "utf8");

/** 🔗️ The field names the GUEST reads out of one `nodeGraphEdit` sub-operation, from its own match arm. */
const guestFields = (operation: string): readonly string[] => {
  const arm = guest.slice(guest.indexOf(`"${operation}" =>`));
  const body = arm.slice(0, arm.indexOf("\n                }"));
  return [...new Set([...body.matchAll(/operation\.get\("(\w+)"\)/gu)].map((match) => match[1]!))];
};

/** 🔌️ Splits a `"<nodeId>@<portId>"` endpoint — the one grammar the graph reports hover, picks and wire ends in. */
const splitEndpoint = (endpoint: string): readonly [string, string] => {
  const at = endpoint.lastIndexOf("@");
  return [endpoint.slice(0, at), endpoint.slice(at + 1)];
};

describe("node graph gestures", () => {
  it("declares a rule for every case, and a case for every rule", () => {
    for (const testCase of law.cases) expect(law.rules[testCase.rule], testCase.name).toBeTruthy();
    const covered = new Set(law.cases.map((testCase) => testCase.rule));
    // 🪶 Two rules are about a SHAPE no wgpu gesture case can express — what a changed move publishes,
    // and what the SCREEN path's release dispatches when nothing changed — so each is pinned by its own
    // `it` below instead of by a case row.
    expect([...Object.keys(law.rules)].filter((rule) => !covered.has(rule) && rule !== "aChangedMovePublishesOnlyWhatChanged" && rule !== "aContentlessGestureDispatchesNothing")).toEqual([]);
  });

  it("names every sub-operation with the very fields the guest reads back", () => {
    for (const [operation, fields] of Object.entries(law.operationFields)) {
      expect([...fields].sort(), operation).toEqual([...guestFields(operation)].sort());
    }
  });

  it("writes each of those fields, under that name, in the wgpu action builder", () => {
    for (const [operation, fields] of Object.entries(law.operationFields)) {
      expect(canvas, operation).toContain(`builder.string(Some("operation"), "${operation}")?`);
      for (const field of fields) {
        const snake = field.replace(/[A-Z]/gu, (letter) => `_${letter.toLowerCase()}`);
        expect(canvas.includes(`Some("${field}"), ${snake}`) || canvas.includes(`Some("${field}"), *${snake}`), `${operation}.${field}`).toBe(true);
      }
    }
  });

  it("expects every declared edit to carry exactly its operation's own fields", () => {
    for (const testCase of law.cases) {
      for (const edit of testCase.expectedEdits ?? []) {
        const operation = edit.operation as string;
        const declared = law.operationFields[operation];
        expect(declared, `${testCase.name}: ${operation}`).toBeTruthy();
        for (const key of Object.keys(edit)) {
          if (key === "operation") continue;
          expect(declared, `${testCase.name}: ${key}`).toContain(key);
        }
      }
    }
  });

  it("wires an output into an input that the authored graph really leaves free", () => {
    const wire = law.cases.find((testCase) => testCase.gesture.kind === "wire");
    const [from, to] = [wire!.gesture.from as string, wire!.gesture.to as string];
    expect(law.graph.outputs).toContain(from);
    expect(law.graph.freeInputs).toContain(to);
    expect(law.graph.wiredInputs).not.toContain(to);
    expect(splitEndpoint(from)[0]).not.toBe(splitEndpoint(to)[0]);
  });

  it("cuts a wire from an input the authored graph really has wired", () => {
    const cut = law.cases.find((testCase) => testCase.gesture.kind === "cut");
    expect(law.graph.wiredInputs).toContain(cut!.gesture.from as string);
  });

  it("decides what a gesture publishes BEFORE it reserves anything, so a plain hover reserves nothing", () => {
    // 🩸️ Every move published `interactionSelect` + `interactionHover` + `nodeGraphViewport` whether
    // or not any had changed, and the 256-slot queue filled mid-sweep: `BoundedActionFault::ItemCredits`
    // at admission, after which the session published nothing at all. Measured on 6118 — a 22x20 sweep
    // faulted at point ~50 and not one of the six presses that followed ever reached the host.
    const bounded = canvas.slice(canvas.indexOf("fn node_graph_bounded_publish"));
    const body = bounded.slice(0, bounded.indexOf("\n}\n"));
    const dispatchAt = body.indexOf("graph_interaction_dispatch(");
    const reserveAt = body.indexOf("input.reserve_actions(");
    expect(dispatchAt).toBeGreaterThan(-1);
    expect(reserveAt).toBeGreaterThan(dispatchAt);
    expect(body).toContain("if items == 0 {");
    expect(body).toContain("input.reserve_actions(items, items *");
    expect(law.rules.anUnchangedMoveCostsNothing).toContain("reserves NO action credit");
  });

  it("keeps the pre-reservation for the phases that can still mutate the graph", () => {
    // 🪸️ The rule the change-detection must NOT weaken: a press, a release, or a move inside a live
    // wire draw can still edit the graph, and a bounded dispatch may never mutate what it cannot then
    // publish. Only a plain hover — which can produce no edit at all — pays after the fact.
    const screen = canvas.slice(canvas.indexOf("fn node_graph_screen_pointer_into"));
    const body = screen.slice(0, screen.indexOf("\n}\n"));
    expect(body).toContain("let may_mutate = !matches!(intent.phase, flow::dag::DagPointerPhase::Move) || node_graph_gesture_is_screen_path(surface_id, None);");
    const guardAt = body.indexOf("if may_mutate {");
    expect(body.indexOf("input.reserve_actions(4,")).toBeGreaterThan(guardAt);
    expect(body.indexOf("input.reserve_actions(4,")).toBeLessThan(body.indexOf("apply_node_graph_screen_pointer"));
  });

  it("publishes a node move once, on release, the way React's own fallback dispatches onNodeDragStop", () => {
    // 🩸️ A bounded drag wrote the new positions into the fixture layout and told the guest nothing, so
    // every dragged node snapped back on the next fixture push. React's SSR `Diagram` fallback has
    // dispatched exactly this narrow `move` all along; wgpu had no edit for it at all.
    const dag = readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs"), "utf8");
    const plan = dag.slice(dag.indexOf("pub fn plan_graph_edits"));
    const body = plan.slice(0, plan.indexOf("\n    }\n"));
    expect(body).toContain("!plan.previous_active || !plan.dragged");
    expect(body).toContain("DagProjectionGesture::Idle");
    expect(law.rules.aReleasedDragPublishesMove).toContain("once, on release");
    expect(law.rules.aClickIsNotAMove).toContain("zero-delta");
  });

  it("lets the HOST decide whether a released screen gesture owes a document write at all", () => {
    // 🩸️ React read an empty `operations` as permission to re-publish the WHOLE fixture, so every plain
    // click on the graph canvas dispatched a `setFixture` `nodeGraphEdit` — and since a landed document
    // mutation owes every attached preview a fresh evaluation, a QUIET shell invoked
    // `["nodeGraphEdit","flowEvalTick","flowEvalTick"]` against a preview already settled at 7/7 nodes.
    // The predicate exists and always did: `commit_gesture_history` decides an undo entry by
    // `content_changed`; it is now published as `fixtureChanged` and is the only thing that authorises
    // the commit (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️generate-add-flow-wire-quiet-tick-2026-09-14.md`).
    const flowHost = readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs"), "utf8");
    const answer = flowHost.slice(flowHost.indexOf("pub fn take_graph_edits_json"));
    expect(answer.slice(0, answer.indexOf("\n    }\n"))).toContain('("fixtureChanged".to_string(), crate::os_pack::json::Value::Bool(self.gesture_changed_content))');
    const commit = flowHost.slice(flowHost.indexOf("fn commit_gesture_history"));
    const commitBody = commit.slice(0, commit.indexOf("\n    }\n"));
    expect(commitBody).toContain("self.gesture_changed_content = false;");
    expect(commitBody).toContain("if !Self::content_changed(&baseline, &self.fixture)");
    expect(commitBody).toContain("self.gesture_changed_content = true;");

    const reactGraph = readFileSync(resolve(engineRoot, "🧱️elements/🕸️NodeGraph/🟦️.tsx"), "utf8");
    expect(reactGraph).toContain("fixtureChanged: parsed?.fixtureChanged === true");
    expect(reactGraph).toContain("const { operations, fixtureChanged } = graphGestureAnswer(value);");
    expect(reactGraph).toContain("else if (fixtureChanged) commitFixture();");
    expect(reactGraph).not.toContain("else commitFixture();");
    expect(law.rules.aContentlessGestureDispatchesNothing).toContain("dispatches NOTHING");
  });

  it("publishes the flow-graph viewport only after a camera gesture settles, not on every selection sync", () => {
    const reactGraph = readFileSync(resolve(engineRoot, "🧱️elements/🕸️NodeGraph/🟦️.tsx"), "utf8");
    let searchFrom = 0;
    let emitCount = 0;
    while (searchFrom < reactGraph.length) {
      const at = reactGraph.indexOf("const emitInteractionState = useCallback", searchFrom);
      if (at < 0) break;
      emitCount += 1;
      const bodyEnd = reactGraph.indexOf("}, [", at);
      const body = reactGraph.slice(at, bodyEnd);
      expect(body, `emitInteractionState #${emitCount} must not replay nodeGraphViewport`).not.toContain("nodeGraphActions.viewport");
      searchFrom = at + 1;
    }
    expect(emitCount).toBeGreaterThanOrEqual(2);
    expect(reactGraph).toContain("publishCameraRef.current()");
  });

  it("classifies a surface point exactly once, and routes the gesture by that same answer", () => {
    // 🩺️ Which part of a node is draggable body and which is inline widget used to be knowable only by
    // pressing and seeing what happened. `DagScreenHit` is the one classification, `is_screen_path`
    // routes from it and `is_draggable_body` reports from it, so a press can never be explained by a
    // different hit test than the one that routed it.
    const dag = readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs"), "utf8");
    const begins = dag.slice(dag.indexOf("pub fn screen_pointer_gesture_begins_at"));
    expect(begins.slice(0, begins.indexOf("\n    }\n"))).toContain("self.screen_hit(sx, sy).is_screen_path()");
    expect(dag).toContain("pub fn is_draggable_body(&self) -> bool {");
    expect(dag).toContain("self.node_id.is_some() && !self.is_screen_path()");
    expect(law.rules.aBodyPressSelectsThatNode).toContain("is_draggable_body");
  });
});
