
import { Matrix3, Vector2 } from "three";
import { dagWorldToScreen } from "../../🧱️elements/🕸️NodeGraph/🟦️.tsx";
import graphFixture from "../../🧱️elements/⚙️EngineCanvas/🧫️fixtures/🕸️wgpu-node-graph/🔣️.json" with { type: "json" };
import { applyPatch } from "fast-json-patch";
import { describe, expect, it, vi } from "vitest";
import { ANCHORS, composeTutorialUi, createTutorialClock } from "@semio-tech/ui-react";
import { createMemoryStoragePort, type TutorialDefinition, type TutorialUiChange, type TutorialUiSnapshot } from "@semio-tech/framework";
import { createLocalInteractionStoreV1, initialShellState, shellReducer, type LocalInteractionStoreV1, type ShellAction } from "../../🧱️elements/🐚️Shell/🟦️.tsx";
import { applyTutorialUiChangeToShell, applyTutorialUiSnapshotToShell, captureTutorialUiSnapshot, type TutorialUiBridgeContext } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
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

const bridge = (interaction: LocalInteractionStoreV1): TutorialUiBridgeContext => ({
  session: null,
  restoreDialog: (dialogId) => ({ openingId: 1, dialogId, origin: {} }) as never,
  appLabelsOverlay: { windowKindLabels: {}, panelTabLabels: {}, modeLabels: {}, actionLabels: {}, utilityLabels: {}, exampleLabels: {}, actionArgLabels: {}, dialogLabels: {}, introductionLabels: {}, groupLabels: {} },
  terminology: "native",
  locale: "en",
  interactionSelection: () => interaction.get().selection,
  publishInteractionSelection: (selection) => interaction.observe({ ...interaction.get(), selection }),
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
    expect(Object.keys(fixture.snapshot.activePanelTabByGroup)).toEqual(ANCHORS);
    expect(new Set(fixture.gestureMatrix.map((point) => point.kind))).toEqual(new Set(["scene", "canvas", "entity", "curve", "domain"]));
  });

  it("self-schedules the exported tutorial clock until the neutral terminal frame", () => {
    let nextHandle = 0;
    const frames = new Map<number, FrameRequestCallback>();
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      const handle = ++nextHandle;
      frames.set(handle, callback);
      return handle;
    });
    vi.stubGlobal("cancelAnimationFrame", (handle: number) => {
      frames.delete(handle);
    });
    try {
      const clock = createTutorialClock(fixture.clock.durationMs);
      clock.setRate(fixture.clock.rate);
      expect(frames.size).toBe(0);
      clock.play();
      expect(frames.size).toBe(1);
      for (let index = 0; index < fixture.clock.wallFramesMs.length; index += 1) {
        const entry = frames.entries().next().value as [number, FrameRequestCallback] | undefined;
        expect(entry).toBeDefined();
        const [handle, frame] = entry!;
        frames.delete(handle);
        frame(fixture.clock.wallFramesMs[index]);
        expect(clock.getTimeMs()).toBeCloseTo(fixture.clock.expectedPlayheadMs[index], 8);
        expect(clock.isPlaying()).toBe(fixture.clock.expectedPlaying[index]);
        expect(frames.size).toBe(fixture.clock.expectedPlaying[index] ? 1 : 0);
      }

      clock.seek(0);
      clock.play();
      expect(frames.size).toBe(1);
      clock.pause();
      expect(frames.size).toBe(0);
      clock.play();
      expect(frames.size).toBe(1);
      clock.dispose();
      expect(frames.size).toBe(0);
    } finally {
      vi.unstubAllGlobals();
    }
  });

  it("applies and captures every neutral field, including closing command search", () => {
    let state = initialShellState({ plugins: [], storage: createMemoryStoragePort() });
    const dispatch = (action: ShellAction) => {
      state = shellReducer(state, action);
    };
    const interaction = createLocalInteractionStoreV1();
    const context = bridge(interaction);
    applyTutorialUiSnapshotToShell(dispatch, snapshot(fixture.snapshot), context);
    expect(project(captureTutorialUiSnapshot(state, interaction.get(), null))).toEqual(fixture.snapshot);
    applyTutorialUiSnapshotToShell(dispatch, snapshot(fixture.mutated), context);
    expect(project(captureTutorialUiSnapshot(state, interaction.get(), null))).toEqual(fixture.mutated);
    applyTutorialUiSnapshotToShell(dispatch, snapshot(fixture.snapshot), context);
    expect(project(captureTutorialUiSnapshot(state, interaction.get(), null))).toEqual(fixture.snapshot);
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
    const interaction = createLocalInteractionStoreV1();
    const context = bridge(interaction);
    applyTutorialUiSnapshotToShell(dispatch, snapshot(fixture.snapshot), context);
    for (const change of changes) applyTutorialUiChangeToShell(dispatch, change, context);
    expect(project(captureTutorialUiSnapshot(state, interaction.get(), null))).toEqual(fixture.afterDeltas);
  });
});
