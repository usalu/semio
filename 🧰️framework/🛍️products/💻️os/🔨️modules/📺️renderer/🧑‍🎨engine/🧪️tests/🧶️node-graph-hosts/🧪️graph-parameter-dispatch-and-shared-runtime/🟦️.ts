/** 🧪️ The graph parameter contract, nodeGraphEdit dispatch and shared Flow runtime tests. */
import { act as reactAct, createElement } from "react";
import { artifactsFlowParameterFixture as graphParameterFixture, artifactsFlowParameterSchema as flowParameterSchema, wasmBrowserRuntimeFixture as flowBrowserRuntimeFixture } from "../../../../../🌊️flow/🧪️tests/🧫️fixtures/🟦️.ts";
import * as flowSessionLoader from "../../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import * as infiniteCanvasRenderer from "@semio-tech/canvas-react-renderer";
import { createFlowBrowserRuntime } from "@semio-tech/flow-core/🌐️flow-browser.js";
import { MockFlowBridge } from "../../../../../🌊️flow/🕸️wasm/🧪️tests/🎭️mock-flow-bridge/🟦️.ts";
import { cleanup, fireEvent, render, waitFor } from "@semio-tech/ui-react/test";
import { expect, it, vi } from "vitest";
import { FlowGraphCanvasHost } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { computeAccessibleName } from "../../🧰️engine-contract-helpers/🟦️.ts";
import { peerExport } from "../🧰️node-graph-host-fixtures/🟦️.ts";

/** 📐 Registers the strict graph parameter contract, the bounded nodeGraphEdit dispatch, the shared Flow runtime and unmounted host retirement tests. */
export function registerGraphParameterDispatchTests(): void {
  //#region 🎚️GraphParameterDispatch
  it("validates the strict language-neutral graph parameter contract for all three consumers", () => {
    
    const command = peerExport(flowParameterSchema, "GraphParameterCommand");
    
    for (const extra of ["snapshotJson", "hostSnapshotJson", "operations"]) {
      const malformed = structuredClone(graphParameterFixture);
      (malformed.cases[0] as Record<string, unknown>)[extra] = "{}";
      
      expect(command({ widgetId: "radius", value: 3, [extra]: "{}" })).toBe(false);
    }
    expect(new Set(graphParameterFixture.cases.map((value) => value.app))).toEqual(new Set(["flow", "generation2d", "generation3d"]));
    for (const value of graphParameterFixture.cases) expect(command({ widgetId: value.widgetId, value: value.request, surfaceId: value.surfaceId })).toBe(true);
  });
  it("dispatches graph parameter keyboard and drag events as bounded nodeGraphEdit operations with explicit commits", async () => {
    const task = <T>(value: T) => ({ result: Promise.resolve(value), subscribe: () => () => {}, cancel: vi.fn() });
    const scheduler = { invalidate: vi.fn(), paintNow: vi.fn(), beginContinuous: vi.fn(), endContinuous: vi.fn(), dispose: vi.fn() };
    const schedulerSpy = vi.spyOn(infiniteCanvasRenderer, "createDemandFrameScheduler").mockReturnValue(scheduler);
    const contextSpy = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const timerSpy = vi.spyOn(globalThis, "setTimeout");
    const createSpy = vi.spyOn(flowSessionLoader, "createFlowSession");
    const originalPointer = globalThis.PointerEvent;
    class TestPointerEvent extends MouseEvent {
      readonly pointerId: number;
      readonly pointerType: string;
      constructor(type: string, init: PointerEventInit = {}) {
        super(type, init);
        this.pointerId = init.pointerId ?? 1;
        this.pointerType = init.pointerType ?? "mouse";
      }
    }
    globalThis.PointerEvent = TestPointerEvent as unknown as typeof PointerEvent;
    try {
      for (const item of graphParameterFixture.cases) {
        const row = { widgetId: item.widgetId, label: item.label, ...item.before, x: 0, y: 0, w: 100, h: 16 };
        const methods: Record<string, ReturnType<typeof vi.fn>> = {
          documentJson: vi.fn(() => {
            throw new Error("slider must not serialize the fixture");
          }),
          sliderOverlayStateJson: vi.fn(() => task(JSON.stringify({ camera: { x: 0, y: 0, zoom: 1 }, sliders: [row] }))),
          labelOverlayPaintStateJson: vi.fn(() => task("{}")),
          setSliderValue: vi.fn((_id: string, value: number) => {
            row.value = value;
            return task(undefined);
          }),
          selectedWidgetIds: vi.fn(() => task("[]")),
          previewOffWidgetIds: vi.fn(() => task("[]")),
          selectionPreviewPointsJson: vi.fn(() => task("[]")),
          selectionPreviewCrossing: vi.fn(() => task(false)),
        };
        const session = new Proxy(methods, {
          get(target, key: string) {
            if (key === "then") return undefined;
            return (target[key] ??= vi.fn(() => task(undefined)));
          },
        }) as unknown as flowSessionLoader.FlowWasmSession;
        createSpy.mockResolvedValueOnce(session);
        const onAction = vi.fn();
        const view = render(
          createElement(FlowGraphCanvasHost, {
            scene: { nodes: [], edges: [], viewport: { x: 0, y: 0, zoom: 1 }, hostSnapshotJson: '{"schema":"flow.host_snapshot","widgets":[]}' },
            controllerId: item.controllerId,
            surfaceId: item.surfaceId,
            editable: true,
            keyboardPort: { current: null },
            onAction,
          }),
        );
        await waitFor(() => expect(view.getByRole("slider", { name: item.label })).toBeTruthy());
        const slider = view.getByRole("slider", { name: item.label });
        const root = slider.closest('[data-slot="slider"]') as HTMLElement;
        const stableId = root.id;
        for (const element of [slider, root]) {
          Object.defineProperties(element, { setPointerCapture: { value: () => {} }, releasePointerCapture: { value: () => {} }, hasPointerCapture: { value: () => true } });
        }
        root.getBoundingClientRect = () => ({ x: 0, y: 0, width: 100, height: 16, left: 0, right: 100, top: 0, bottom: 16, toJSON: () => ({}) });
        (root.querySelector('[data-slot="slider-track"]') as HTMLElement).getBoundingClientRect = root.getBoundingClientRect;
        const gesturePrefix = `${item.surfaceId}:${item.widgetId}`.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
        const gesturePattern = new RegExp(`^${gesturePrefix}:\\d+:\\d+$`);
        slider.focus();
        fireEvent.keyDown(slider, { key: "ArrowRight" });
        fireEvent.keyUp(slider, { key: "ArrowRight" });
        await waitFor(() =>
          expect(onAction).toHaveBeenLastCalledWith({
            controllerId: item.controllerId,
            action: graphParameterFixture.action,
            args: { surfaceId: item.surfaceId, operations: [{ operation: "setSlider", widgetId: item.widgetId, value: 3 }], gesture: expect.stringMatching(gesturePattern), commit: true },
          }),
        );
        expect(document.activeElement).toBe(slider);
        expect(computeAccessibleName(slider)).toBe(item.label);
        expect(root.id).toBe(stableId);
        let staleOverlay: ((value: string) => void) | undefined;
        methods.sliderOverlayStateJson!.mockImplementationOnce(() => ({
          ...task(""),
          result: new Promise<string>((resolve) => {
            staleOverlay = resolve;
          }),
        }));
        fireEvent.pointerDown(root, { pointerId: 1, pointerType: "mouse", button: 0, buttons: 1, clientX: 40, clientY: 8 });
        fireEvent.pointerMove(root, { pointerId: 1, pointerType: "mouse", buttons: 1, clientX: 80, clientY: 8 });
        await waitFor(() => {
          const event = onAction.mock.calls.at(-1)?.[0];
          expect(event).toMatchObject({ controllerId: item.controllerId, action: graphParameterFixture.action, args: { surfaceId: item.surfaceId, operations: [{ operation: "setSlider", widgetId: item.widgetId, value: 8 }], commit: false } });
          expect(event.args.gesture).toMatch(gesturePattern);
        });
        await waitFor(() => expect(slider.getAttribute("aria-valuenow")).toBe("8"));
        await reactAct(async () => {
          staleOverlay?.(JSON.stringify({ sliders: [{ ...row, value: 4 }] }));
        });
        expect(slider.getAttribute("aria-valuenow")).toBe("8");
        const count = onAction.mock.calls.length;
        const gesture = onAction.mock.calls.at(-1)?.[0].args.gesture;
        fireEvent.pointerUp(root, { pointerId: 1, button: 0, clientX: 80, clientY: 8 });
        await waitFor(() => expect(onAction).toHaveBeenCalledTimes(count + 1));
        expect(onAction).toHaveBeenLastCalledWith({
          controllerId: item.controllerId,
          action: graphParameterFixture.action,
          args: { surfaceId: item.surfaceId, operations: [{ operation: "setSlider", widgetId: item.widgetId, value: 8 }], gesture, commit: true },
        });
        fireEvent.pointerCancel(root, { pointerId: 1 });
        expect(onAction).toHaveBeenCalledTimes(count + 1);
        expect(methods.documentJson).not.toHaveBeenCalled();
        expect(
          onAction.mock.calls.every(
            ([event]) =>
              event.action === graphParameterFixture.action &&
              Object.keys(event.args).sort().join() === "commit,gesture,operations,surfaceId" &&
              event.args.operations.every((operation: Record<string, unknown>) => Object.keys(operation).sort().join() === "operation,value,widgetId"),
          ),
        ).toBe(true);
        view.unmount();
      }
      expect(timerSpy.mock.calls.some(([, delay]) => delay === 80)).toBe(false);
      expect(scheduler.beginContinuous).toHaveBeenCalledWith("gesture");
      expect(scheduler.endContinuous).toHaveBeenCalledWith("gesture");
    } finally {
      cleanup();
      createSpy.mockRestore();
      schedulerSpy.mockRestore();
      contextSpy.mockRestore();
      timerSpy.mockRestore();
      globalThis.PointerEvent = originalPointer;
    }
  });
  it("retains one shared Flow browser runtime across two mounted graph hosts and retires only each unmounted session", async () => {
    
    
    const expected = flowBrowserRuntimeFixture.mountedHosts;
    const bridge = new MockFlowBridge(new WebAssembly.Memory({ initial: 400 }));
    const runtime = await createFlowBrowserRuntime({ source: bridge.exports });
    const createSpy = vi.spyOn(flowSessionLoader, "createFlowSession").mockImplementation(async () => runtime.openSession());
    const contextSpy = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const scene = (revision: number) => ({
      nodes: [],
      edges: [],
      viewport: { x: 0, y: 0, zoom: 1 },
      hostSnapshotJson: JSON.stringify({ schema: "flow.host_snapshot", revision, widgets: [] }),
    });
    const host = (id: "A" | "B", revision: number) =>
      createElement(FlowGraphCanvasHost, {
        key: id,
        scene: scene(revision),
        controllerId: `flow.${id}`,
        surfaceId: `flow.${id}`,
        editable: true,
        keyboardPort: { current: null },
        onAction: vi.fn(),
      });
    const pair = (revision: number) => createElement("div", {}, host("A", revision), host("B", revision));
    const single = (revision: number) => createElement("div", {}, host("B", revision));
    const view = render(pair(0));
    try {
      expect(expected.sharedRuntime).toBe(true);
      await waitFor(() => expect(bridge.openRequestIds).toEqual(flowBrowserRuntimeFixture.openRequestIds));
      await waitFor(() => expect(bridge.operationSessions.some((row) => row.operation === expected.afterUnmountA.command && row.slot === expected.afterUnmountA.liveSlot)).toBe(true));
      view.rerender(single(1));
      await waitFor(() => expect(bridge.closedSessionSlots).toEqual(expected.afterUnmountA.closedSlots));
      expect(bridge.globalCloseCalls).toBe(expected.afterUnmountA.globalCloseCalls);
      const beforeSiblingCommand = bridge.operationSessions.filter((row) => row.operation === expected.afterUnmountA.command && row.slot === expected.afterUnmountA.liveSlot).length;
      view.rerender(single(2));
      await waitFor(() => expect(bridge.operationSessions.filter((row) => row.operation === expected.afterUnmountA.command && row.slot === expected.afterUnmountA.liveSlot)).toHaveLength(beforeSiblingCommand + 1));
      view.unmount();
      await waitFor(() => expect(bridge.closedSessionSlots).toEqual(expected.afterUnmountB.closedSlots));
      expect(bridge.globalCloseCalls).toBe(expected.afterUnmountB.globalCloseCalls);
      await runtime.close();
      expect(bridge.globalCloseCalls).toBe(flowBrowserRuntimeFixture.runtimeClose.globalCloseCalls);
      expect(runtime.terminalIsEmpty()).toBe(flowBrowserRuntimeFixture.runtimeClose.terminal);
    } finally {
      view.unmount();
      await runtime.close();
      createSpy.mockRestore();
      contextSpy.mockRestore();
      cleanup();
    }
  });
  it("retires a graph host unmounted before its open reply while its shared-runtime sibling remains live", async () => {
    const expected = flowBrowserRuntimeFixture.mountedHosts.lateUnmount;
    const bridge = new MockFlowBridge(new WebAssembly.Memory({ initial: 400 }), { heldOpenReplies: 1 });
    const runtime = await createFlowBrowserRuntime({ source: bridge.exports });
    const createSpy = vi.spyOn(flowSessionLoader, "createFlowSession").mockImplementation(async () => runtime.openSession());
    const contextSpy = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const scene = (revision: number) => ({
      nodes: [],
      edges: [],
      viewport: { x: 0, y: 0, zoom: 1 },
      hostSnapshotJson: JSON.stringify({ schema: "flow.host_snapshot", revision, widgets: [] }),
    });
    const host = (id: "A" | "B", revision: number) =>
      createElement(FlowGraphCanvasHost, {
        scene: scene(revision),
        controllerId: `flow.late.${id}`,
        surfaceId: `flow.late.${id}`,
        editable: true,
        keyboardPort: { current: null },
        onAction: vi.fn(),
      });
    const first = render(host("A", 0));
    let sibling: ReturnType<typeof render> | undefined;
    try {
      await waitFor(() => expect(bridge.openRequestIds).toEqual(["1"]));
      first.unmount();
      sibling = render(host("B", 0));
      await waitFor(() => expect(bridge.openRequestIds).toEqual(flowBrowserRuntimeFixture.openRequestIds));
      await waitFor(() => expect(bridge.operationSessions.some((row) => row.operation === expected.command && row.slot === expected.liveSlot)).toBe(true));
      bridge.releaseOpenReplies();
      await waitFor(() => expect(bridge.closedSessionSlots).toEqual(expected.closedSlots));
      expect(bridge.globalCloseCalls).toBe(0);
      const beforeSiblingCommand = bridge.operationSessions.filter((row) => row.operation === expected.command && row.slot === expected.liveSlot).length;
      sibling.rerender(host("B", 1));
      await waitFor(() => expect(bridge.operationSessions.filter((row) => row.operation === expected.command && row.slot === expected.liveSlot)).toHaveLength(beforeSiblingCommand + 1));
      sibling.unmount();
      await waitFor(() => expect(bridge.closedSessionSlots).toEqual([expected.heldOpenSlot, expected.liveSlot]));
      await runtime.close();
      expect(bridge.globalCloseCalls).toBe(flowBrowserRuntimeFixture.runtimeClose.globalCloseCalls);
      expect(runtime.terminalIsEmpty()).toBe(flowBrowserRuntimeFixture.runtimeClose.terminal);
    } finally {
      bridge.releaseOpenReplies();
      first.unmount();
      sibling?.unmount();
      await runtime.close();
      createSpy.mockRestore();
      contextSpy.mockRestore();
      cleanup();
    }
  });
  //#endregion 🎚️GraphParameterDispatch
}
