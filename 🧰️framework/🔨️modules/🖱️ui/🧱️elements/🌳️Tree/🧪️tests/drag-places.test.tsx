import * as React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { act, cleanup, fireEvent, render } from "@testing-library/react";
import { Tree } from "../🟦️.tsx";
import { TreeReorderDropPreview } from "../🧲️drag-drop/🟦️.tsx";
import { type TreeDataSection } from "../📋️data/🟦️.ts";
import { treeDropBand } from "../../../🔨️modules/🫳️pointer-drag/🟦️.tsx";

afterEach(() => {
  cleanup();
  document.body.replaceChildren();
  vi.restoreAllMocks();
  document.documentElement.removeAttribute("data-drag-active");
});

const row = new DOMRect(10, 100, 200, 24);

describe("🫳️ the preview of a tree drop is a rectangle inside the row", () => {
  const preview = (position: "before" | "after" | "inside") => {
    const target = document.body.appendChild(document.createElement("div"));
    target.id = "row";
    target.getBoundingClientRect = () => row;
    render(<TreeReorderDropPreview preview={{ targetId: "row", position }} />);
    return document.body.querySelector<HTMLElement>('[data-slot="tree-drop-preview"]')!;
  };
  const box = (element: HTMLElement) => [element.style.left, element.style.top, element.style.width, element.style.height];

  it.each(["before", "after", "inside"] as const)("takes exactly the band of the row for a drop %s it, as the active place", (position) => {
    const element = preview(position);
    const band = treeDropBand({ left: row.left, top: row.top, right: row.right, bottom: row.bottom }, position);
    expect(box(element)).toEqual([`${band.left}px`, `${band.top}px`, `${band.right - band.left}px`, `${band.bottom - band.top}px`]);
    expect(element.hasAttribute("data-drop-place")).toBe(true);
    expect(element.hasAttribute("data-drop-active")).toBe(true);
    expect(element.className).not.toMatch(/border|h-0\.5|bg-primary/);
  });
});

describe("🫳️ a dragged tree row", () => {
  const sections: TreeDataSection[] = [{ id: "s", label: "Section", defaultOpen: true, items: [{ id: "one", label: "One", draggable: true, dragData: { "application/x-test": "1" } }, { id: "two", label: "Two" }] }];
  const transfer = () => ({ setData: vi.fn(), setDragImage: vi.fn(), types: [] as string[], effectAllowed: "", dropEffect: "" });
  const rowOf = (container: HTMLElement, id: string) => container.querySelector<HTMLElement>(`[id="${id}"]`)!;

  it("leaves its place hidden, travels as glass and shows every other row as a place; the end of the drag takes it all back", () => {
    const { container } = render(<Tree sections={sections} dragAndDropController={{ handleDrop: vi.fn() }} />);
    const dataTransfer = transfer();
    act(() => void fireEvent.dragStart(rowOf(container, "one"), { dataTransfer }));
    const ghost = document.body.querySelector<HTMLElement>("[data-drag-ghost]")!;
    expect(ghost.textContent).toBe("One");
    expect(ghost.getAttribute("data-level")).toBe("menu");
    expect(dataTransfer.setDragImage).toHaveBeenCalledTimes(1);
    expect(document.documentElement.hasAttribute("data-drag-active")).toBe(true);
    expect(rowOf(container, "one").hasAttribute("data-drag-hidden")).toBe(true);
    expect(rowOf(container, "one").className).not.toMatch(/opacity-40/);
    expect(rowOf(container, "one").hasAttribute("data-drop-place")).toBe(false);
    expect(rowOf(container, "two").hasAttribute("data-drop-place")).toBe(true);
    act(() => void fireEvent.dragEnd(rowOf(container, "one"), { dataTransfer }));
    expect(document.body.querySelector("[data-drag-ghost]")).toBeNull();
    expect(document.documentElement.hasAttribute("data-drag-active")).toBe(false);
    expect(rowOf(container, "one").hasAttribute("data-drag-hidden")).toBe(false);
    expect(rowOf(container, "two").hasAttribute("data-drop-place")).toBe(false);
  });
});
