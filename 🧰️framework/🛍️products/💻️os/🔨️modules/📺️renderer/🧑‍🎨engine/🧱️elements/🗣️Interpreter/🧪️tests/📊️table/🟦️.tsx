type TestSource = { readonly url: string };

/** 📊️ The interpreter's windowed table (ticket 26/09/18 U5 §6b): the language-neutral conformance fixture
 * (`🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/📊️table`, also read by the Rust contract laws) renders as an
 * ARIA grid over the table's WHOLE logical extent, its rows sit at their logical positions behind the tree
 * windows' own spacers, and the keyboard moves one tab stop across rows the host has not streamed yet.
 * Testing Library's role queries are the third-party oracle for the accessibility tree. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { TreeWindowContext, UiDocumentStore, UiNodeView, tableWindowNextRowV1, tableWindowScrollTopForRowV1 } = dependencies;
  const { describe, expect, it, afterEach } = vitest;

  const { cleanup, fireEvent, render, screen } = await import("@semio-tech/ui-react/test");
  const { treeRowHeightPx } = await import("@semio-tech/ui-react");
  const { createElement } = await import("react");
  const { readFileSync } = await import("node:fs");
  const { dirname, join } = await import("node:path");
  const { fileURLToPath } = await import("node:url");

  const fixtureDir = join(dirname(fileURLToPath(source.url)), "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/📊️table");
  const snapshot = JSON.parse(readFileSync(join(fixtureDir, "📸️snapshot.json"), "utf8"));

  function mount(onIntent: (intent: any) => void, windows: unknown = null) {
    const store = new UiDocumentStore(snapshot.surface);
    store.loadSnapshot(snapshot);
    const view = createElement(UiNodeView, { store, id: snapshot.root, context: { store, onAction: () => {}, onIntent } });
    return render(windows ? createElement(TreeWindowContext.Provider, { value: windows }, view) : view);
  }

  afterEach(() => cleanup());

  describe("windowed table", () => {
    it("renders the conformance fixture as a grid over its whole logical extent", () => {
      mount(() => {});
      const grid = screen.getByRole("grid", { name: "Spaces" });
      expect(grid.getAttribute("aria-rowcount")).toBe("41");
      expect(grid.getAttribute("aria-colcount")).toBe("3");
      expect(Array.from(grid.querySelectorAll<HTMLElement>("[role='columnheader']")).map((header) => header.textContent)).toEqual(["Name", "Kind", "Actions"]);
      const rows = Array.from(grid.querySelectorAll<HTMLElement>("[role='row']"));
      expect(rows.map((row) => row.getAttribute("aria-rowindex"))).toEqual(["1", "14", "15"]);
      expect(Array.from(rows[2]!.querySelectorAll<HTMLElement>("[role='gridcell']")).map((cell) => cell.textContent?.trim())).toEqual(["Werkstatt Ada", "atelier", ""]);
      expect(rows[2]!.contains(screen.getByRole("button", { name: "Open: Werkstatt Ada" }))).toBe(true);
      const container = grid.querySelector<HTMLElement>("[data-tree-window-key]")!;
      expect(container.getAttribute("data-tree-window-key")).toBe("spaces");
      expect(container.getAttribute("data-tree-window-total")).toBe("40");
      expect(container.getAttribute("data-tree-window-offset")).toBe("12");
      expect(container.getAttribute("data-tree-window-length")).toBe("2");
      expect(grid.querySelector<HTMLElement>("[data-tree-window-spacer='leading']")!.style.height).toBe(`${12 * treeRowHeightPx}px`);
      expect(grid.querySelector<HTMLElement>("[data-tree-window-spacer='trailing']")!.style.height).toBe(`${26 * treeRowHeightPx}px`);
      expect(screen.getByRole("status").textContent).toBe("Rows 13–14 of 40");
    });

    it("keeps one tab stop and fires a row's own activation on Enter", () => {
      const intents: any[] = [];
      mount((intent) => intents.push(intent));
      const rows = Array.from(screen.getByRole("grid", { name: "Spaces" }).querySelectorAll<HTMLElement>("[role='row']")).slice(1);
      expect(rows.map((row) => row.tabIndex)).toEqual([0, -1]);
      fireEvent.keyDown(rows[0]!, { key: "ArrowDown" });
      expect(document.activeElement).toBe(rows[1]);
      fireEvent.keyDown(rows[1]!, { key: "Enter" });
      expect(intents.map((intent) => intent.action.name)).toEqual(["openSpace"]);
      fireEvent.keyDown(rows[1]!, { key: "ArrowRight" });
      expect(document.activeElement?.getAttribute("aria-label")).toBe("Open: Werkstatt Ada");
      fireEvent.keyDown(document.activeElement!, { key: "Escape" });
      expect(document.activeElement).toBe(rows[1]);
    });

    it("scrolls a row the host has not streamed into view and reports the window it needs", async () => {
      const reports: any[] = [];
      mount(() => {}, { bodyKey: "framework.window.table", openStates: {}, setOpen: () => {}, reportWindows: (requests: unknown, viewportRows: number) => reports.push({ requests, viewportRows }) });
      const grid = screen.getByRole("grid", { name: "Spaces" });
      const scroller = grid.querySelector<HTMLElement>("[data-slot='table-window-scroll']")!;
      const container = grid.querySelector<HTMLElement>("[data-tree-window-key]")!;
      let scrollTop = 0;
      const rect = (top: number, height: number) => ({ top, bottom: top + height, left: 0, right: 0, width: 0, height, x: 0, y: top, toJSON: () => ({}) }) as DOMRect;
      Object.defineProperty(scroller, "scrollTop", { get: () => scrollTop, set: (value: number) => { scrollTop = value; }, configurable: true });
      Object.defineProperty(scroller, "clientHeight", { value: 10 * treeRowHeightPx, configurable: true });
      Object.defineProperty(scroller, "scrollHeight", { value: 40 * treeRowHeightPx, configurable: true });
      scroller.getBoundingClientRect = () => rect(0, 10 * treeRowHeightPx);
      container.getBoundingClientRect = () => rect(-scrollTop, 40 * treeRowHeightPx);
      for (const row of Array.from(container.querySelectorAll<HTMLElement>("[data-tree-window-row]"))) {
        const index = Number(row.getAttribute("data-tree-window-row"));
        row.getBoundingClientRect = () => rect(index * treeRowHeightPx - scrollTop, treeRowHeightPx);
      }
      const rows = Array.from(grid.querySelectorAll<HTMLElement>("[role='row']")).slice(1);
      fireEvent.keyDown(rows[1]!, { key: "End" });
      expect(scrollTop).toBe((40 - 10) * treeRowHeightPx);
      scroller.dispatchEvent(new Event("scroll"));
      await new Promise((resolve) => setTimeout(resolve, 60));
      const last = reports.at(-1);
      expect(last.viewportRows).toBe(10);
      expect(last.requests.map((request: any) => request.nodeKey)).toEqual(["spaces"]);
      expect(last.requests[0].offset + last.requests[0].rows).toBeGreaterThanOrEqual(40);
    });

    it("moves the active row over the whole logical extent by the fixture's key law", () => {
      const cases: readonly [string, number, number, number, number | null][] = [
        ["ArrowDown", 0, 40, 10, 1],
        ["ArrowDown", 39, 40, 10, 39],
        ["ArrowUp", 0, 40, 10, 0],
        ["PageDown", 12, 40, 10, 22],
        ["PageDown", 35, 40, 10, 39],
        ["PageUp", 5, 40, 10, 0],
        ["Home", 17, 40, 10, 0],
        ["End", 3, 40, 10, 39],
        ["Tab", 3, 40, 10, null],
        ["ArrowDown", 0, 0, 10, null],
      ];
      for (const [key, current, total, page, expected] of cases) expect(tableWindowNextRowV1(key, current, total, page), `${key} from ${current}`).toBe(expected);
      expect(tableWindowScrollTopForRowV1(39, 20, 0, 200)).toBe(600);
      expect(tableWindowScrollTopForRowV1(3, 20, 200, 200)).toBe(60);
      expect(tableWindowScrollTopForRowV1(12, 20, 200, 200)).toBe(200);
    });
  });
}
