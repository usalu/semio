import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import clamp from "lodash/clamp";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue, resolveSelectPlacement } from "../../🟦️.tsx";
import { ShellScopeContext, type ShellScope } from "../../../🐚️ShellScope/🟦️.tsx";

afterEach(() => vi.restoreAllMocks());

function Fixture({ windowed = false, offset }: { windowed?: boolean; offset?: number }) {
  return <div data-window-silhouette={windowed ? "" : undefined}><Select defaultOpen><SelectTrigger aria-label="Mode"><SelectValue /></SelectTrigger><SelectContent sideOffset={offset}><SelectItem value="alpha">Alpha</SelectItem><SelectItem value="beta">Beta</SelectItem></SelectContent></Select></div>;
}

describe("select shared boundaries", () => {
  it("registers positioned frames and preserves measured default and explicit gaps", () => {
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      return (this.dataset.slot === "select-trigger" ? { left: 100, right: 140, top: 100, bottom: 120, width: 40, height: 20 } : { width: 80, height: 50 }) as DOMRect;
    });
    const { getByRole, rerender } = render(<Fixture windowed />);
    const trigger = getByRole("combobox");
    const content = getByRole("listbox");
    for (const frame of [trigger, content]) {
      expect(frame.getAttribute("data-shared-border-frame")).toBe("");
      expect(frame.getAttribute("data-shared-border-edges")).toBe("top right bottom left");
      expect(frame.hasAttribute("data-shared-border-attached")).toBe(false);
    }
    expect(trigger.className.split(" ")).toContain("relative");
    expect(content.style.position).toBe("fixed");
    expect(content.style.top).toBe("120px");
    expect(content.className).not.toMatch(/(?:^|:)translate-y-1|(?:^|:)-translate-y-1/);
    rerender(<Fixture windowed offset={6} />);
    expect(content.style.top).toBe("126px");
    rerender(<Fixture />);
    expect(content.style.top).toBe("124px");
    rerender(<Fixture offset={0} />);
    expect(content.style.top).toBe("120px");
  });

  it("matches independent viewport clamping and preserves a flipped zero gap", () => {
    for (const left of [-100, 0, 50, 500]) {
      const result = resolveSelectPlacement({ left, right: left + 40, top: 180, bottom: 200, width: 40, height: 20 }, { width: 80, height: 50 }, { width: 300, height: 220 }, "bottom", "start", 0, 8, false);
      expect([result.left, result.top, result.side]).toEqual([clamp(left, 8, 212), 130, "top"]);
    }
  });

  it("keeps the shell portal and listbox focus and keyboard contract", () => {
    const layer = document.createElement("div");
    document.body.append(layer);
    const scope = { portalLayerRef: { current: layer } } as ShellScope;
    const { getByRole, unmount } = render(<ShellScopeContext.Provider value={scope}><div style={{ overflow: "hidden" }}><Fixture windowed /></div></ShellScopeContext.Provider>);
    const content = getByRole("listbox");
    const trigger = getByRole("combobox");
    expect(content.parentElement).toBe(layer);
    expect(document.activeElement).toBe(content);
    fireEvent.keyDown(content, { key: "ArrowDown" });
    expect(document.getElementById(content.getAttribute("aria-activedescendant")!)?.textContent).toContain("Beta");
    fireEvent.keyDown(content, { key: "Enter" });
    expect(trigger.textContent).toContain("Beta");
    expect(document.activeElement).toBe(trigger);
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    unmount();
    layer.remove();
  });
});
