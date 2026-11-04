/** 🔘️ The visible trigger raises the same `contextmenu` event as a right click, anchored on its target. */
// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { contextMenuAnchorPointV1, dispatchContextMenuAtV1 } from "../../🔘️trigger/🟦️.tsx";

describe("context menu trigger", () => {
  it("anchors just below the trailing edge of the target", () => {
    expect(contextMenuAnchorPointV1({ left: 10, top: 20, right: 210, bottom: 400 })).toEqual({ x: 206, y: 44 });
    expect(contextMenuAnchorPointV1({ left: 10, top: 20, right: 12, bottom: 30 })).toEqual({ x: 10, y: 30 });
  });

  it("raises a bubbling, cancelable contextmenu with the anchor coordinates and reports a claimed menu", () => {
    const host = document.createElement("div");
    const row = document.createElement("tr");
    host.append(row);
    document.body.append(host);
    const seen: { x: number; y: number; button: number }[] = [];
    host.addEventListener("contextmenu", (event) => {
      seen.push({ x: event.clientX, y: event.clientY, button: event.button });
      event.preventDefault();
    });
    expect(dispatchContextMenuAtV1(row, { x: 5, y: 9 })).toBe(true);
    expect(seen).toEqual([{ x: 5, y: 9, button: 2 }]);
    const bare = document.createElement("div");
    document.body.append(bare);
    expect(dispatchContextMenuAtV1(bare, { x: 1, y: 1 })).toBe(false);
  });
});
