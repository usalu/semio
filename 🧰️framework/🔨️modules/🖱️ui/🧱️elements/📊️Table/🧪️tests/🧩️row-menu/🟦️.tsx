/** 🧩️ A table row with a menu: the row is a focusable row (not a button), its menu trigger is a sibling control, Enter or Space on the row activates it, and the trigger opens the row menu. */
// @vitest-environment jsdom
import { cleanup, fireEvent, render } from "@testing-library/react";
import * as React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { Table } from "@semio-tech/ui-react";

type Row = { id: string; name: string };
const rows: Row[] = [{ id: "a", name: "Alpha" }, { id: "b", name: "Beta" }];
const columns = [{ id: "name", header: "Name", accessor: (row: Row) => row.name }] as never;

afterEach(() => cleanup());

describe("table row menu", () => {
  it("has no button row, so the trigger is never nested inside another interactive control", () => {
    const view = render(<Table columns={columns} data={rows} getRowId={(row: Row) => row.id} onRowClick={() => {}} onRowContextMenu={() => {}} />);
    for (const row of view.container.querySelectorAll("tbody tr")) {
      expect(row.getAttribute("role")).not.toBe("button");
      expect(row.getAttribute("tabindex")).toBe("0");
      const triggers = row.querySelectorAll("[data-context-menu-trigger]");
      expect(triggers.length).toBe(1);
      expect(triggers[0]!.closest("[role=button]")).toBeNull();
      expect(triggers[0]!.getAttribute("aria-label")).toBeTruthy();
    }
  });

  it("activates a row from the keyboard and leaves keys inside a control to that control", () => {
    const onRowClick = vi.fn();
    const view = render(<Table columns={columns} data={rows} getRowId={(row: Row) => row.id} onRowClick={onRowClick} onRowContextMenu={() => {}} />);
    const first = view.container.querySelector("tbody tr")!;
    fireEvent.keyDown(first, { key: "Enter" });
    fireEvent.keyDown(first, { key: " " });
    fireEvent.keyDown(first, { key: "a" });
    expect(onRowClick).toHaveBeenCalledTimes(2);
    fireEvent.keyDown(first.querySelector("[data-context-menu-trigger]")!, { key: "Enter" });
    expect(onRowClick).toHaveBeenCalledTimes(2);
  });

  it("opens the row menu from the visible trigger without activating the row", () => {
    const onRowClick = vi.fn();
    const onRowContextMenu = vi.fn((_row: Row, _index: number, event: React.MouseEvent) => event.preventDefault());
    const view = render(<Table columns={columns} data={rows} getRowId={(row: Row) => row.id} onRowClick={onRowClick} onRowContextMenu={onRowContextMenu} />);
    fireEvent.click(view.container.querySelectorAll("[data-context-menu-trigger]")[1]!);
    expect(onRowContextMenu).toHaveBeenCalledTimes(1);
    expect(onRowContextMenu.mock.calls[0]![0]).toEqual(rows[1]);
    expect(onRowClick).not.toHaveBeenCalled();
  });

  it("offers no trigger when the table has no row menu", () => {
    const view = render(<Table columns={columns} data={rows} getRowId={(row: Row) => row.id} />);
    expect(view.container.querySelector("[data-context-menu-trigger]")).toBeNull();
  });
});
