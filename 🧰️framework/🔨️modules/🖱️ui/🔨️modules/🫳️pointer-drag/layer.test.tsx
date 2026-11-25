import * as React from "react";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { act, cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { DragGhost, beginNativeDragGhost, dragLabelOf, dragSourceAttributes, dropPlaceAttributes, useDndGhost, useDragActive, useDragEscape, useNativeDropOver } from "./🟦️.tsx";

const css = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🎨️styling/🖌️ui/🎨️.css"), "utf8");
const rule = (selector: string): string => css.slice(css.indexOf(selector), css.indexOf("}", css.indexOf(selector)));

const Probe: React.FC<{ readonly active: boolean }> = ({ active }) => {
  useDragActive(active);
  return null;
};

afterEach(() => {
  cleanup();
  document.documentElement.removeAttribute("data-drag-active");
  document.body.replaceChildren();
});

describe("🫳️ a dragged element travels as one floating glass rectangle", () => {
  it("renders the copy on a layer of its own as inert menu-level glass with the chip tint", () => {
    const { unmount } = render(<DragGhost x={12} y={34}>label</DragGhost>);
    const ghost = document.body.querySelector<HTMLElement>("[data-drag-ghost]")!;
    expect(ghost.parentElement).toBe(document.body);
    expect(ghost.getAttribute("data-level")).toBe("menu");
    expect(ghost.hasAttribute("data-glass-chip")).toBe(true);
    expect(ghost.classList.contains("ui-glass")).toBe(true);
    expect(ghost.getAttribute("aria-hidden")).toBe("true");
    expect(ghost.hasAttribute("inert")).toBe(true);
    expect(ghost.style.transform).toBe("translate(20px, 44px)");
    expect(ghost.textContent).toBe("label");
    unmount();
    expect(document.body.querySelector("[data-drag-ghost]")).toBeNull();
  });

  it("follows the pointer", () => {
    const { rerender } = render(<DragGhost x={1} y={2}>a</DragGhost>);
    rerender(<DragGhost x={40} y={50}>a</DragGhost>);
    expect(document.body.querySelector<HTMLElement>("[data-drag-ghost]")!.style.transform).toBe("translate(48px, 60px)");
  });

  it("marks the page while an element travels and clears the mark when the last drag ends", () => {
    const first = render(<Probe active />);
    expect(document.documentElement.hasAttribute("data-drag-active")).toBe(true);
    const second = render(<Probe active />);
    first.unmount();
    expect(document.documentElement.hasAttribute("data-drag-active")).toBe(true);
    second.rerender(<Probe active={false} />);
    expect(document.documentElement.hasAttribute("data-drag-active")).toBe(false);
  });

  it("states a place by attributes: none outside a drag, available, and active under the pointer", () => {
    expect(dropPlaceAttributes(undefined)).toEqual({});
    expect(dropPlaceAttributes("available")).toEqual({ "data-drop-place": "" });
    expect(dropPlaceAttributes("active")).toEqual({ "data-drop-place": "", "data-drop-active": "" });
  });

  it("hides the source in place while it travels", () => {
    expect(dragSourceAttributes(false)).toEqual({});
    expect(dragSourceAttributes(true)).toEqual({ "data-drag-hidden": "" });
  });

  it("abandons the drag with Escape and with no other key", () => {
    const cancel = vi.fn();
    const Escapable: React.FC<{ readonly active: boolean }> = ({ active }) => {
      useDragEscape(active, cancel);
      return null;
    };
    const view = render(<Escapable active />);
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true }));
    expect(cancel).not.toHaveBeenCalled();
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
    expect(cancel).toHaveBeenCalledTimes(1);
    view.rerender(<Escapable active={false} />);
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
    expect(cancel).toHaveBeenCalledTimes(1);
  });
});

describe("🫳️ a native drag is detached the same way", () => {
  const start = (source: HTMLElement) => {
    const setDragImage = vi.fn();
    const end = beginNativeDragGhost({ dataTransfer: { setDragImage } as unknown as DataTransfer, currentTarget: source, clientX: 10, clientY: 20 }, "Row");
    return { setDragImage, end };
  };

  it("replaces the browser's drag image, shows the copy at the pointer and hides the source", async () => {
    const source = document.body.appendChild(document.createElement("div"));
    const { setDragImage } = start(source);
    const ghost = document.body.querySelector<HTMLElement>("[data-drag-ghost]")!;
    expect(setDragImage).toHaveBeenCalledTimes(1);
    expect(ghost.textContent).toBe("Row");
    expect(ghost.getAttribute("data-level")).toBe("menu");
    expect(ghost.hasAttribute("data-glass-chip")).toBe(true);
    expect(document.documentElement.hasAttribute("data-drag-active")).toBe(true);
    await act(async () => void (await new Promise((done) => setTimeout(done, 0))));
    expect(source.hasAttribute("data-drag-hidden")).toBe(true);
    act(() => {
      const move = new Event("dragover", { bubbles: true }) as Event & { clientX: number; clientY: number };
      move.clientX = 100;
      move.clientY = 200;
      source.dispatchEvent(move);
    });
    expect(ghost.style.transform).toBe("translate(108px, 210px)");
  });

  it("removes the copy, the mark and the hidden state when the drag ends", async () => {
    const source = document.body.appendChild(document.createElement("div"));
    start(source);
    await act(async () => void (await new Promise((done) => setTimeout(done, 0))));
    act(() => void source.dispatchEvent(new Event("dragend", { bubbles: true })));
    expect(document.body.querySelector("[data-drag-ghost]")).toBeNull();
    expect(document.documentElement.hasAttribute("data-drag-active")).toBe(false);
    expect(source.hasAttribute("data-drag-hidden")).toBe(false);
  });

  it("ends by the function it returns as well", () => {
    const source = document.body.appendChild(document.createElement("div"));
    const { end } = start(source);
    end();
    expect(document.body.querySelector("[data-drag-ghost]")).toBeNull();
    expect(document.documentElement.hasAttribute("data-drag-active")).toBe(false);
  });
});

describe("🫳️ the paint of a drag", () => {
  it("tints every place at 10 % and the one under the pointer at 32 % while a drag runs", () => {
    expect(css).toMatch(/:root\[data-drag-active\] \[data-drop-place\][^{]*\{[^}]*color-mix\(in srgb, var\(--active-base\) 10%/);
    expect(css).toMatch(/:root\[data-drag-active\] \[data-drop-place\]\[data-drop-active\][^{]*\{[^}]*color-mix\(in srgb, var\(--active-base\) 32%/);
  });

  it("hides the source with its room kept and puts the copy above everything", () => {
    expect(rule("[data-drag-hidden]")).toContain("opacity: 0");
    expect(rule("[data-drag-ghost] {")).toMatch(/position: fixed/);
    expect(rule("[data-drag-ghost] {")).toMatch(/z-index: calc\(var\(--z-tutorial\)/);
    expect(rule("[data-drag-ghost] {")).toMatch(/pointer-events: none/);
  });

  it("animates nothing, so reduced motion changes nothing, and outlines the places in forced colours", () => {
    const paint = css.slice(css.indexOf("[data-drag-hidden] {"), css.indexOf("/* ⬛️ No rounded corner"));
    expect(paint).not.toMatch(/transition|animation/);
    expect(paint).toMatch(/@media \(forced-colors: active\) \{\s*:root\[data-drag-active\] \[data-drop-place\][^}]*outline/);
    expect(paint).toMatch(/\[data-drop-active\][^}]*Highlight/);
  });
});

describe("🫳️ the hooks that detach the drags of rows", () => {
  it("keeps the source visible when asked to and still shows the copy", async () => {
    const source = document.body.appendChild(document.createElement("div"));
    beginNativeDragGhost({ dataTransfer: null, currentTarget: source, clientX: 0, clientY: 0 }, "Row", { hideSource: false });
    await act(async () => void (await new Promise((done) => setTimeout(done, 0))));
    expect(source.hasAttribute("data-drag-hidden")).toBe(false);
    expect(document.body.querySelector("[data-drag-ghost]")).not.toBeNull();
  });

  it("is over a place from the first dragenter to the last dragleave of the place and its children", () => {
    const Place: React.FC = () => {
      const { over, bind } = useNativeDropOver();
      return <div data-testid="place" data-over={over ? "true" : undefined} {...bind}><span data-testid="child" /></div>;
    };
    const view = render(<Place />);
    const place = view.getByTestId("place");
    const child = view.getByTestId("child");
    act(() => void fireEvent.dragEnter(place));
    act(() => void fireEvent.dragEnter(child));
    act(() => void fireEvent.dragLeave(place));
    expect(place.getAttribute("data-over")).toBe("true");
    act(() => void fireEvent.dragLeave(child));
    expect(place.getAttribute("data-over")).toBeNull();
    act(() => void fireEvent.dragEnter(place));
    act(() => void document.dispatchEvent(new Event("dragend")));
    expect(place.getAttribute("data-over")).toBeNull();
  });

  it("detaches a dnd-kit drag: the ghost follows the activator plus the travel and ends with the drag", () => {
    const row = document.body.appendChild(document.createElement("div"));
    row.textContent = "  Alpha   row ";
    const Host: React.FC = () => {
      const { activeId, ghost, start, move, end } = useDndGhost(() => dragLabelOf(row));
      return (
        <div>
          <button data-testid="start" onClick={() => start({ active: { id: "a", rect: { current: { initial: { left: 3, top: 4 } } } }, activatorEvent: { clientX: 10, clientY: 20 } as unknown as Event, delta: { x: 0, y: 0 } })} />
          <button data-testid="move" onClick={() => move({ active: { id: "a", rect: { current: { initial: { left: 3, top: 4 } } } }, activatorEvent: { clientX: 10, clientY: 20 } as unknown as Event, delta: { x: 5, y: 7 } })} />
          <button data-testid="end" onClick={end} />
          <output data-testid="active">{activeId ?? "none"}</output>
          {ghost}
        </div>
      );
    };
    const view = render(<Host />);
    fireEvent.click(view.getByTestId("start"));
    expect(view.getByTestId("active").textContent).toBe("a");
    expect(document.body.querySelector<HTMLElement>("[data-drag-ghost]")!.textContent).toBe("Alpha row");
    expect(document.documentElement.hasAttribute("data-drag-active")).toBe(true);
    fireEvent.click(view.getByTestId("move"));
    expect(document.body.querySelector<HTMLElement>("[data-drag-ghost]")!.style.transform).toBe("translate(23px, 37px)");
    fireEvent.click(view.getByTestId("end"));
    expect(document.body.querySelector("[data-drag-ghost]")).toBeNull();
    expect(document.documentElement.hasAttribute("data-drag-active")).toBe(false);
  });
});
