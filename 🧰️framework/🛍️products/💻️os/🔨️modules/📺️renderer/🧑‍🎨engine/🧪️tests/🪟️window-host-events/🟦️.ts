/** 📨️ The React pane's host-event forwarding (`windowHostEventHandlersV1` in `🏛️ShellHost/🟦️.tsx`): a pane that loses
 * focus to something outside it forwards `hostEvent{windowId, kind: "blur"}`, focus moving within it forwards nothing, the
 * implicit capture release after the pane's own `pointerup` is no loss, and a capture taken away or a cancelled pointer
 * forwards `captureLost` — the React half of the runtime's typed host events (the wgpu half is
 * `a_pane_losing_the_activation_is_blurred_once`). The DOM (happy-dom) supplies the events and the containment test. */

import { describe, expect, it } from "vitest";
import { HOST_EVENT_ACTION_ID } from "@semio-tech/framework";
import { windowHostEventHandlersV1 } from "../../🧱️elements/🏛️ShellHost/🟦️.tsx";

type Forwarded = { readonly controllerId: string; readonly action: string; readonly args?: unknown };

function pane(windowId: string) {
  const forwarded: Forwarded[] = [];
  const handlers = windowHostEventHandlersV1("s.test@1/*#editor", windowId, (action) => void forwarded.push(action as Forwarded));
  const root = document.createElement("div");
  const inner = document.createElement("button");
  root.append(inner);
  const outside = document.createElement("button");
  document.body.append(root, outside);
  const focus = (relatedTarget: EventTarget | null) => handlers.onBlur?.({ currentTarget: root, relatedTarget } as never);
  const pointer = (name: "onPointerUpCapture" | "onPointerCancelCapture" | "onLostPointerCapture", pointerId: number) => handlers[name]?.({ pointerId } as never);
  return { forwarded, focus, pointer, inner, outside };
}

describe("📨️ window host events", () => {
  it("forwards a blur only when focus leaves the pane", () => {
    const { forwarded, focus, inner, outside } = pane("overview");
    focus(inner);
    expect(forwarded).toEqual([]);
    focus(outside);
    focus(null);
    expect(forwarded).toEqual([
      { controllerId: "s.test@1/*#editor", action: HOST_EVENT_ACTION_ID, args: { windowId: "overview", kind: "blur" } },
      { controllerId: "s.test@1/*#editor", action: HOST_EVENT_ACTION_ID, args: { windowId: "overview", kind: "blur" } },
    ]);
  });

  it("forwards a lost capture unless the pane's own pointerup released it", () => {
    const { forwarded, pointer } = pane("detail");
    pointer("onPointerUpCapture", 1);
    pointer("onLostPointerCapture", 1);
    expect(forwarded).toEqual([]);
    pointer("onLostPointerCapture", 2);
    pointer("onPointerCancelCapture", 3);
    pointer("onLostPointerCapture", 3);
    expect(forwarded.map((action) => action.args)).toEqual([
      { windowId: "detail", kind: "captureLost" },
      { windowId: "detail", kind: "captureLost" },
    ]);
  });
});
