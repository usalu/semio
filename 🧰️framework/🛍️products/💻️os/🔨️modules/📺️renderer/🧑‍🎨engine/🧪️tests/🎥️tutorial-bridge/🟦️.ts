import Ajv from "ajv";
import { Matrix3, Vector2 } from "three";
import { dagWorldToScreen } from "../../🧱️elements/🕸️NodeGraph/🟦️.tsx";
import graphFixture from "../../🧱️elements/⚙️EngineCanvas/🧫️fixtures/🕸️wgpu-node-graph/🔣️.json" with { type: "json" };
import { applyPatch } from "fast-json-patch";
import { describe, expect, it } from "vitest";
import { ANCHORS, composeTutorialUi } from "@semio-tech/ui-react";
import { createMemoryStoragePort, type TutorialDefinition, type TutorialUiChange, type TutorialUiSnapshot } from "@semio-tech/framework";
import { initialShellState, shellReducer, type ShellAction, type ShellState } from "../../🧱️elements/🐚️Shell/🟦️.tsx";
import { applyTutorialUiChangeToShell, applyTutorialUiSnapshotToShell, captureTutorialUiSnapshot, type TutorialUiBridgeContext } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import schema from "../../🧬️schema/🎥️tutorial-bridge/🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🎥️tutorial-bridge/🔣️.json" with { type: "json" };

const project = (snapshot: TutorialUiSnapshot) => ({
  focusedWindowId: snapshot.focusedWindowId,
  activeUtilityByWindowId: snapshot.activeUtilityByWindowId,
  activeToolId: snapshot.activeToolId,
  activePanelTabByGroup: snapshot.activePanelTabByGroup,
  interactionSelection: snapshot.interactionSelection,
  openDialogId: snapshot.openDialogId,
  expandedTreeIds: snapshot.expandedTreeIds,
  commandPanelOpen: snapshot.commandPanelOpen,
});

const snapshot = (value: typeof fixture.snapshot | typeof fixture.mutated): TutorialUiSnapshot => structuredClone(value) as TutorialUiSnapshot;

const bridge = (read: () => ShellState, dispatch: (action: ShellAction) => void): TutorialUiBridgeContext => ({
  session: null,
  restoreDialog: (dialogId) => ({ openingId: 1, dialogId, origin: {} }) as never,
  appLabelsOverlay: { windowKindLabels: {}, panelTabLabels: {}, modeLabels: {}, actionLabels: {}, utilityLabels: {}, exampleLabels: {}, actionArgLabels: {}, dialogLabels: {}, introductionLabels: {}, groupLabels: {} },
  terminology: "native",
  locale: "en",
  interactionSelection: () => read().interaction.selection,
  publishInteractionSelection: (selection) => dispatch({ type: "INTERACTION_STATE_OBSERVED", state: { ...read().interaction, selection } }),
});

describe("tutorial bridge parity", () => {
  it("projects graph tutorial visibility with the React and independent matrix oracles", () => {
    const { camera, layout, widgets, synapses } = graphFixture.hostSnapshot;
    const { width, height } = graphFixture.tutorialViewport;
    const transform = new Matrix3().set(camera.zoom, 0, width / 2 - camera.x * camera.zoom, 0, camera.zoom, height / 2 - camera.y * camera.zoom, 0, 0, 1);
    for (const scenario of graphFixture.tutorialVisibility) {
      const point = layout[scenario.entity as keyof typeof layout];
      const react = dagWorldToScreen(camera, width, height, point.x, point.y);
      const oracle = new Vector2(point.x, point.y).applyMatrix3(transform);
      expect(react.x).toBeCloseTo(oracle.x, 8);
      expect(react.y).toBeCloseTo(oracle.y, 8);
      expect(react.x >= 0 && react.x <= width && react.y >= 0 && react.y <= height).toBe(scenario.visible);
    }
    const nodeIds = widgets.filter(widget => widget.kind !== "outputPreview").map(widget => widget.id);
    const ports = widgets.flatMap(widget => {
      if (widget.kind === "inputSlider") return [`${widget.id}@number`];
      if (widget.kind !== "neuron") return [];
      const operator = graphFixture.operators.find(row => row.id === widget.neuronKind)!;
      return [...operator.inputs, ...operator.outputs].map(port => `${widget.id}@${port.code}`);
    });
    expect(nodeIds).toHaveLength(graphFixture.captionExpectation.nodes);
    expect(ports).toHaveLength(graphFixture.captionExpectation.ports);
    for (const output of graphFixture.captionExpectation.outputHandles) expect(ports).toContain(output);
    for (const edge of synapses) {
      expect(ports).toContain(`${edge.from}@${edge.fromPort}`);
      if (edge.toPort) expect(ports).toContain(`${edge.to}@${edge.toPort}`);
    }
  });

  it("validates the neutral round-trip, delta, and semantic-point fixture", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(Object.keys(fixture.snapshot.activePanelTabByGroup)).toEqual(ANCHORS);
    expect(new Set(fixture.gestureMatrix.map((point) => point.kind))).toEqual(new Set(["scene", "canvas", "entity", "curve", "domain"]));
  });

  it("applies and captures every neutral field, including closing command search", () => {
    let state = initialShellState({ plugins: [], storage: createMemoryStoragePort() });
    const dispatch = (action: ShellAction) => {
      state = shellReducer(state, action);
    };
    const context = bridge(() => state, dispatch);
    applyTutorialUiSnapshotToShell(dispatch, snapshot(fixture.snapshot), context);
    expect(project(captureTutorialUiSnapshot(state, null))).toEqual(fixture.snapshot);
    applyTutorialUiSnapshotToShell(dispatch, snapshot(fixture.mutated), context);
    expect(project(captureTutorialUiSnapshot(state, null))).toEqual(fixture.mutated);
    applyTutorialUiSnapshotToShell(dispatch, snapshot(fixture.snapshot), context);
    expect(project(captureTutorialUiSnapshot(state, null))).toEqual(fixture.snapshot);
  });

  it("composes and applies sparse changes with the JSON Patch oracle", () => {
    const changes = fixture.deltas as TutorialUiChange[];
    const definition = {
      id: "tutorial-bridge",
      title: "Tutorial bridge",
      durationMs: 100,
      chapters: [],
      base: { ui: snapshot(fixture.snapshot), cameras: [] },
      tracks: { camera: [], ui: [{ at: 10, sample: { kind: "delta", changes } }], events: [], gestures: [], document: [] },
    } as unknown as TutorialDefinition;
    expect(project(composeTutorialUi(definition, 100))).toEqual(fixture.afterDeltas);

    const patched = applyPatch(structuredClone(fixture.snapshot), [
      { op: "remove", path: "/interactionSelection/world" },
      { op: "remove", path: "/expandedTreeIds/0" },
      { op: "replace", path: "/openDialogId", value: "confirm.other" },
      { op: "remove", path: "/activePanelTabByGroup/bottom-middle" },
    ], true, false).newDocument;
    expect({ ...patched, commandPanelOpen: false }).toEqual(fixture.afterDeltas);

    let state = initialShellState({ plugins: [], storage: createMemoryStoragePort() });
    const dispatch = (action: ShellAction) => {
      state = shellReducer(state, action);
    };
    const context = bridge(() => state, dispatch);
    applyTutorialUiSnapshotToShell(dispatch, snapshot(fixture.snapshot), context);
    for (const change of changes) applyTutorialUiChangeToShell(dispatch, change, context);
    expect(project(captureTutorialUiSnapshot(state, null))).toEqual(fixture.afterDeltas);
  });
});
