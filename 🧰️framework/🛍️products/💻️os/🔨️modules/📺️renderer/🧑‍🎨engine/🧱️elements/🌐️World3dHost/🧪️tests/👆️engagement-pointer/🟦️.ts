import { act, cleanup, renderHook } from "@testing-library/react";
import { OrthographicCamera } from "three";
import { afterEach, expect, it, vi } from "vitest";
import { useActiveUtilityState } from "../../👆️pointer-and-drop/🔶️active-utility/🟦️.ts";
import fixture from "../🧫️fixtures/engagement-pointer-order.json";

afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});

for (const row of fixture) it(row.id, async () => {
  const events: [string, number][] = [];
  const requests: (() => void)[] = [];
  const frames: FrameRequestCallback[] = [];
  const camera = new OrthographicCamera(-1, 1, 1, -1, 0.1, 10);
  camera.position.z = 2;
  camera.updateMatrixWorld();
  const record = (action: string, args: { position: [number, number, number] }) => events.push([action, args.position[0]]);
  const idle = () => false;
  const pass = () => ({ kind: "single" });
  const core = {
    activeUtility: "cut",
    interaction: { activeUtility: "cut" },
    cameraRef: { current: camera },
    cameraState: { position: [0, 0, 2], target: [0, 0, 0] },
    hostRef: { current: { getBoundingClientRect: () => new DOMRect(0, 0, 100, 100) } },
    node: { surfaceId: "process/workpiece" },
    selection: { engagementSessionActive: true },
    gestureRecognizer: { down: pass, move: pass },
    beginHandleDrag: idle,
    beginRelocateDrag: idle,
    updateHandleDrag: idle,
    updateRelocateDrag: idle,
    publishWorldPresenceView: idle,
    dispatch: record,
    dispatchSettled: (action: string, args: { position: [number, number, number] }) => {
      record(action, args);
      return new Promise<void>(resolve => requests.push(resolve));
    },
  };
  vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => frames.push(callback));
  const hook = renderHook(({ next }) => useActiveUtilityState({ core: next as unknown as Parameters<typeof useActiveUtilityState>[0]["core"] }), { initialProps: { next: core } });
  for (const [step, x] of row.steps) {
    await act(async () => {
      const pointer = { pointerId: 1, clientX: x, clientY: 50, button: 0 } as Parameters<typeof hook.result.current.handlePointerMove>[0];
      switch (step) {
        case "move": hook.result.current.handlePointerMove(pointer); break;
        case "down": hook.result.current.handlePointerDown(pointer); break;
        case "settle": frames.shift()?.(0); requests.shift()?.(); break;
        case "select": hook.rerender({ next: { ...core, activeUtility: "select", interaction: { activeUtility: "select" }, selection: { engagementSessionActive: false } } }); break;
        case "arm": hook.rerender({ next: core }); break;
      }
    });
  }
  expect(events).toEqual(row.events);
});
