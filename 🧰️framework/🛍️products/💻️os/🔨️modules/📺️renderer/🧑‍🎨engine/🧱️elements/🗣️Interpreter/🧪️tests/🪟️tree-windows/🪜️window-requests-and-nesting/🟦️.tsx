/** 🪜️ The window request, idle re-measure and nested container tests. */
import { ownRows } from "../🧰️tree-windows-test-kit/🟦️.tsx";
import type { AnyRecord, TreeWindowsTestKit } from "../🧰️tree-windows-test-kit/🟦️.tsx";

/** 📨 Registers the scroll anchored window request, the answered window, the idle re-measure and the nested container tests. */
export function registerWindowRequestsAndNestingTests(kit: TreeWindowsTestKit): void {
  const { Scrollable, TREE_WINDOW_BODY_NODE_BUDGET, TREE_WINDOW_PATH_SEPARATOR, createElement, expect, frame, it, mountScrollable, nestedNodes, nodeCost, renderWindowedTree, rowHeightPx, scrollTo, stubRect, stubRows, treeWindowBodyRequestsV1, windowedByKey } = kit;
  it("turns a scroll of that container into a window request anchored on the rows the viewport shows", async () => {
    const reports: { requests: readonly AnyRecord[]; viewportRows: number }[] = [];
    const rendered = mountScrollable([{ key: "objects", total: 200 }], (requests, viewportRows) => reports.push({ requests, viewportRows }));
    const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
    const windowed = windowedByKey(rendered.container).get("objects")!;
    scrollTo(area, 40, 10);
    stubRect(windowed, -40, 200);
    stubRows(windowed, -40);

    area.dispatchEvent(new Event("scroll"));
    await frame();

    const last=reports.at(-1);if(!last)throw Error("Actual tree window report required");
    expect(last.viewportRows).toBe(10);
    expect(last.requests).toEqual([{ nodeKey: "objects", offset: 32, rows: 26 }]);
  });
  /** 🧯️ The loop this whole mechanism dies of: the observer re-measures on every store revision, so a
   * window the guest has just SETTLED — the rows now on screen, already covered with overscan — must
   * recompute to byte-identical numbers and report nothing. Otherwise every answer asks another
   * question and the body refreshes forever. */
  it("asks for exactly the window it already has once the guest has answered", () => {
    const rows = (offset: number, length: number, firstTopRows: number) => Array.from({ length }, (_, index) => ({ index: offset + index, top: (firstTopRows + index) * rowHeightPx }));
    const before = [{ key: "objects", rowExtent: "standard", total: 200, offset: 0, length: 1, top: -40 * rowHeightPx, height: 200 * rowHeightPx, rows: rows(0, 1, -40) }];
    const asked = treeWindowBodyRequestsV1(before, 10 * rowHeightPx);
    expect(asked).toEqual([{ key: "objects", offset: 32, rows: 26 }]);

    const after = [{ ...before[0], offset: 32, length: 26, rows: rows(32, 26, -8) }];

    expect(treeWindowBodyRequestsV1(after, 10 * rowHeightPx)).toEqual(asked);
  });
  it("reports nothing when an idle re-measure recomputes the same answer", async () => {
    const reports: { requests: readonly AnyRecord[]; viewportRows: number }[] = [];
    const rendered = mountScrollable([{ key: "objects", total: 200 }], (requests, viewportRows) => reports.push({ requests, viewportRows }));
    const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
    const windowed = windowedByKey(rendered.container).get("objects")!;
    scrollTo(area, 40, 10);
    stubRect(windowed, -40, 200);
    stubRows(windowed, -40);
    area.dispatchEvent(new Event("scroll"));
    await frame();
    const settled = reports.length;
    const settledReport=reports[settled-1];if(!settledReport)throw Error("Actual settled row window required");
    expect(settledReport.requests).toEqual([{ nodeKey: "objects", offset: 32, rows: 26 }]);

    area.dispatchEvent(new Event("scroll"));
    await frame();
    area.dispatchEvent(new Event("scroll"));
    await frame();

    expect(reports.length).toBe(settled);
  });
  /** 🪟️ An OPEN windowed group inside a windowed section. The section's content element is now TALLER
   * than `total × rowHeight` — the expanded child's own rows and spacers live inside it — so a flat
   * `pixels ÷ rowHeight` reading of the section resolves the viewport to a row far below the child and
   * the section's next window would evict the subtree the reader just opened. */
  it("resolves a nested open container against its own rows, and never evicts it from its parent's window", async () => {
    const reports: { requests: readonly AnyRecord[]; viewportRows: number }[] = [];
    const rendered = renderWindowedTree(nestedNodes(100, 3, 50, 20), (requests, viewportRows) => reports.push({ requests, viewportRows }), (tree) => createElement(Scrollable, null, tree));
    const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
    const windowed = windowedByKey(rendered.container);
    expect([...windowed.keys()]).toEqual(["objects", `objects${TREE_WINDOW_PATH_SEPARATOR}objects.1`]);

    scrollTo(area, 30, 20);
    const section = windowed.get("objects")!;
    stubRect(section, -30, 150);
    const sectionRows = ownRows(section);
    expect(sectionRows.map((row) => row.getAttribute("data-tree-window-row"))).toEqual(["0", "1", "2"]);
    stubRect(sectionRows[0]!, -30, 1);
    stubRect(sectionRows[1]!, -29, 1);
    stubRect(sectionRows[2]!, 22, 1);
    const child = windowed.get(`objects${TREE_WINDOW_PATH_SEPARATOR}objects.1`)!;
    stubRect(child, -28, 50);
    stubRows(child, -28);

    area.dispatchEvent(new Event("scroll"));
    await frame();

    const last=reports.at(-1);if(!last)throw Error("Actual tree window report required");
    const sectionRequest = last.requests.find((request) => request.nodeKey === "objects")!;
    const childRequest = last.requests.find((request) => request.nodeKey === `objects${TREE_WINDOW_PATH_SEPARATOR}objects.1`)!;
    expect(sectionRequest).toEqual({ nodeKey: "objects", offset: 0, rows: 17 });
    expect(sectionRequest.offset).toBeLessThanOrEqual(1);
    expect(sectionRequest.offset + sectionRequest.rows).toBeGreaterThan(1);
    expect(childRequest).toEqual({ nodeKey: `objects${TREE_WINDOW_PATH_SEPARATOR}objects.1`, offset: 14, rows: 36 });
    expect(childRequest.offset + childRequest.rows).toBeLessThanOrEqual(50);
    expect(nodeCost(last.requests)).toBeLessThanOrEqual(TREE_WINDOW_BODY_NODE_BUDGET);
  });
}
