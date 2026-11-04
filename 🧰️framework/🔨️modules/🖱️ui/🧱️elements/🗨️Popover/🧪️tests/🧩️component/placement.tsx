import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import clamp from "lodash/clamp";
import { Popover, PopoverContent, PopoverTrigger, resolvePopoverPlacement } from "../../🟦️.tsx";
import { ShellScopeContext, type ShellScope } from "../../../🐚️ShellScope/🟦️.tsx";

afterEach(() => vi.restoreAllMocks());

describe("floating placement", () => {
  it("matches an independent clamp reference for detached viewport placement", () => {
    for (const left of [-100, 0, 50, 500]) {
      for (const top of [-100, 0, 50, 500]) {
        const result = resolvePopoverPlacement({ left, right: left + 40, top, bottom: top + 20, width: 40, height: 20 }, { width: 80, height: 50 }, { width: 300, height: 220 }, "bottom", "start", 4, 0, 8, false, true);
        const flipped = top + 74 > 212 && top - 54 >= 8;
        expect([result.left, result.top]).toEqual([clamp(left, 8, 212), clamp(flipped ? top - 54 : top + 24, 8, 162)]);
      }
    }
  });
  it("uses a flush chrome default, retains explicit offsets and keeps flipped borders complete", () => {
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      return (this.getAttribute("data-slot") === "popover-trigger"
        ? { left: 100, right: 140, top: 100, bottom: 120, width: 40, height: 20 }
        : { width: 80, height: 50 }) as DOMRect;
    });
    const menu = (offset?: number) => <div data-window-silhouette><Popover open><PopoverTrigger>Open</PopoverTrigger><PopoverContent side="bottom" sideOffset={offset}>Body</PopoverContent></Popover></div>;
    const { getByRole, rerender } = render(menu());
    const content = getByRole("dialog");
    expect(content.style.top).toBe("120px");
    expect(content.hasAttribute("data-shared-border-frame")).toBe(true);
    rerender(menu(6));
    expect(content.style.top).toBe("126px");
    expect(content.hasAttribute("data-shared-border-attached")).toBe(false);
    const result = resolvePopoverPlacement({ left: 10, right: 50, top: 180, bottom: 200, width: 40, height: 20 }, { width: 80, height: 50 }, { width: 300, height: 220 }, "bottom", "start", 0, 0, 8, false, true);
    expect(result).toMatchObject({ side: "top", top: 130 });
  });

  it("uses the shell portal rather than a clipped trigger ancestor or body", () => {
    const layer = document.createElement("div");
    document.body.append(layer);
    const scope = { portalLayerRef: { current: layer } } as ShellScope;
    const { getByRole, unmount } = render(<ShellScopeContext.Provider value={scope}><div style={{ overflow: "hidden" }}><Popover open><PopoverTrigger>Open</PopoverTrigger><PopoverContent>Body</PopoverContent></Popover></div></ShellScopeContext.Provider>);
    expect(getByRole("dialog").parentElement).toBe(layer);
    fireEvent.keyDown(document, { key: "Escape" });
    unmount();
    layer.remove();
  });
});
