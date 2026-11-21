import { act, renderHook } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import type { CanvasPickTarget } from "@semio-tech/framework-manifest/ui-contract-and-canvas-types";
import { useCanvasPickInteraction } from "../../🟦️.tsx";
import cases from "../🧫️fixtures/async-pick.json";

describe("asynchronous canvas pick replies", () => {
  for (const row of cases) {
    it(row.id, async () => {
      const events: string[] = [];
      const requests: number[] = [];
      const aborted: number[] = [];
      const pending = new Map<number, { resolve: (targets: readonly CanvasPickTarget[]) => void; reject: (reason: Error) => void }>();
      const hook = renderHook(() => useCanvasPickInteraction({
        resolveTargetsAtClientAsync: ({ x }, signal) => new Promise((resolve, reject) => {
          requests.push(x);
          pending.set(x, { resolve, reject });
          signal.addEventListener("abort", () => aborted.push(x), { once: true });
        }),
        onHoverFocus: ({ target }) => events.push("hover:" + (target?.id ?? "")),
        onSelectTarget: (target) => events.push("select:" + target.id),
      }));
      for (const [kind, x, id] of row.steps) {
        await act(async () => {
          const point = { x: Number(x), y: 0 };
          switch (kind) {
            case "move": hook.result.current.onCanvasPointerMove(point); break;
            case "down": hook.result.current.onCanvasPointerDown(point); break;
            case "up": hook.result.current.onCanvasPointerUp(point); break;
            case "leave": hook.result.current.onCanvasPointerLeave(); break;
            case "cancel": hook.result.current.onCanvasPointerCancel(); break;
            case "unmount": hook.unmount(); break;
            case "resolve": pending.get(Number(x))?.resolve([{ domain: "handle", id: String(id), label: String(id), generality: 5 }]); break;
            case "reject": pending.get(Number(x))?.reject(new Error("stale native session")); break;
          }
        });
      }
      expect(events).toEqual(row.events);
      expect(requests).toEqual(row.requests);
      expect(aborted).toEqual(row.aborted);
    });
  }

  it("keeps synchronous canvas hover immediate", () => {
    const events: string[] = [];
    const hook = renderHook(() => useCanvasPickInteraction({
      resolveTargetsAtClient: () => [{ domain: "node", id: "node", label: "Node", generality: 1 }],
      onHoverFocus: ({ target }) => events.push(target?.id ?? ""),
      onSelectTarget: () => {},
    }));
    act(() => hook.result.current.onCanvasPointerMove({ x: 2, y: 1 }));
    expect(events).toEqual(["node"]);
  });
});
