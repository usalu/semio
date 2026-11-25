/** 🧩️ A table band draws its seam only through the compositor: header and rows are band frames with the `bottom` edge, none carries a hand-drawn bottom border, and the row height already contains the 1 px seam the native border used to take. */
// @vitest-environment jsdom
import { cleanup, render } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import * as React from "react";
import { afterEach, describe, expect, it } from "vitest";
import { Table, TableSkeleton } from "@semio-tech/ui-react";

type Row = { id: string; name: string };
const columns = [{ id: "name", header: "Name", accessor: (row: Row) => row.name }] as never;
const rows: Row[] = [{ id: "a", name: "Alpha" }, { id: "b", name: "Beta" }];
const css = readFileSync(resolve(import.meta.dirname, "../../../../🎨️styling/🖌️ui/🎨️.css"), "utf8");

afterEach(() => cleanup());

describe("table band seams", () => {
  it("draws no hand-drawn bottom border on the header or on a row", () => {
    const view = render(<Table columns={columns} data={rows} getRowId={(row: Row) => row.id} onRowClick={() => {}} />);
    const bands = [...view.container.querySelectorAll("thead, tbody > tr")];
    expect(bands).toHaveLength(3);
    for (const band of bands) {
      expect(band.className).not.toMatch(/(^|\s)border-b(\s|$)/u);
      expect(band.getAttribute("data-shared-border-edges")).toBe("bottom");
      expect(band.getAttribute("data-shared-border-delegate")).toBe("self");
    }
  });

  it("gives every band cell the seam pixel inside its height instead of a collapsed border", () => {
    const view = render(<Table columns={columns} data={rows} getRowId={(row: Row) => row.id} />);
    const cells = view.container.querySelectorAll("thead th, tbody td");
    expect(cells).toHaveLength(3);
    for (const cell of cells) expect(cell.className).toContain("var(--stroke-hairline)");
  });

  it("makes the skeleton rows band frames as well", () => {
    const view = render(<TableSkeleton columns={columns} rowCount={2} />);
    const bands = [...view.container.querySelectorAll("thead, tbody > tr")];
    expect(bands).toHaveLength(3);
    for (const band of bands) {
      expect(band.className).not.toMatch(/(^|\s)border-b(\s|$)/u);
      expect(band.getAttribute("data-shared-border-edges")).toBe("bottom");
    }
  });

  it("keeps the last band of a framed table at the row height so the frame edge and the seam coincide", () => {
    expect(css).toMatch(/tr\[data-shared-border-edges="bottom"\]:last-child > td \{\s*padding-block: 0;/u);
  });
});
