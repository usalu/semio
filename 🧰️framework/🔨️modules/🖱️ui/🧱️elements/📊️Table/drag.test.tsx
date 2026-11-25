import * as React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { act, cleanup, fireEvent, render } from "@testing-library/react";
import { Table } from "./🟦️.tsx";

afterEach(() => {
  cleanup();
  document.body.replaceChildren();
  document.documentElement.removeAttribute("data-drag-active");
});

const rows = [{ id: "a", name: "Alpha" }, { id: "b", name: "Beta" }];
const columns = [{ id: "name", header: "Name", accessor: (row: (typeof rows)[number]) => row.name }];

describe("🫳️ a dragged table row", () => {
  it("travels as glass with its text while its source is hidden in place, and the end of the drag takes it back", async () => {
    const onDragStart = vi.fn();
    const { container } = render(<Table columns={columns} data={rows} rowKey={(row) => row.id} rowDragProps={() => ({ draggable: true, onDragStart })} />);
    const first = container.querySelector<HTMLElement>('[data-row-id="a"]')!;
    const dataTransfer = { setData: vi.fn(), setDragImage: vi.fn(), types: [] as string[], effectAllowed: "" };
    act(() => void fireEvent.dragStart(first, { dataTransfer }));
    expect(onDragStart).toHaveBeenCalledTimes(1);
    expect(document.body.querySelector<HTMLElement>("[data-drag-ghost]")!.textContent).toContain("Alpha");
    expect(dataTransfer.setDragImage).toHaveBeenCalledTimes(1);
    await act(async () => void (await new Promise((done) => setTimeout(done, 0))));
    expect(first.hasAttribute("data-drag-hidden")).toBe(true);
    act(() => void fireEvent.dragEnd(first, { dataTransfer }));
    expect(document.body.querySelector("[data-drag-ghost]")).toBeNull();
    expect(document.documentElement.hasAttribute("data-drag-active")).toBe(false);
    expect(first.hasAttribute("data-drag-hidden")).toBe(false);
  });

  it("stays plain when the row does not ask to be dragged", () => {
    const { container } = render(<Table columns={columns} data={rows} rowKey={(row) => row.id} />);
    act(() => void fireEvent.dragStart(container.querySelector('[data-row-id="a"]')!));
    expect(document.body.querySelector("[data-drag-ghost]")).toBeNull();
  });
});
