/** 📐️ A rendered open Panel clamps its width to the viewport fraction and folds when the editor column would be too narrow. */
// @vitest-environment jsdom
import { act, cleanup, render } from "@testing-library/react";
import * as React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { Panel, type PanelTabNode } from "@semio-tech/ui-react";

const tabs: readonly PanelTabNode[] = [{ kind: "leaf", id: "tab", icon: () => null, name: "Tab", trees: [] }];

function setViewport(width: number): void {
  Object.defineProperty(window, "innerWidth", { configurable: true, value: width });
  window.dispatchEvent(new Event("resize"));
}

afterEach(() => cleanup());

describe("panel fold", () => {
  it("clamps the open width to the viewport fraction through its style", () => {
    setViewport(1280);
    const view = render(<Panel anchor="top-right" visible size={600} tabs={tabs} onVisibleChange={() => {}} />);
    const styled = [...view.container.querySelectorAll<HTMLElement>("*")].find((element) => element.style.width.includes("45vw"));
    expect(styled?.style.width).toBe("min(600px, 45vw)");
  });

  it("stays open at 1280 and 800 and folds once the viewport is too narrow", () => {
    const onVisibleChange = vi.fn();
    setViewport(1280);
    render(<Panel anchor="top-right" visible size={300} tabs={tabs} onVisibleChange={onVisibleChange} />);
    expect(onVisibleChange).not.toHaveBeenCalled();
    act(() => setViewport(800));
    expect(onVisibleChange).not.toHaveBeenCalled();
    act(() => setViewport(700));
    expect(onVisibleChange).toHaveBeenCalledWith(false);
  });

  it("folds on mount when it opens in a too-narrow viewport and never when it is closed", () => {
    const folded = vi.fn();
    setViewport(600);
    render(<Panel anchor="top-right" visible size={300} tabs={tabs} onVisibleChange={folded} />);
    expect(folded).toHaveBeenCalledWith(false);
    const closed = vi.fn();
    render(<Panel anchor="top-left" visible={false} size={300} tabs={tabs} onVisibleChange={closed} />);
    expect(closed).not.toHaveBeenCalled();
  });
});
