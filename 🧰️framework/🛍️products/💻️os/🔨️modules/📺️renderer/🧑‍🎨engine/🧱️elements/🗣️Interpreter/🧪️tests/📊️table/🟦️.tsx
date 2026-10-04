type TestSource = { readonly url: string };

/** 📊️ The interpreter's windowed table (ticket 26/09/18 U5 §6b): the language-neutral conformance fixture
 * (`🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/📊️table`, also read by the Rust contract laws) renders as an
 * ARIA grid over the table's WHOLE logical extent, its rows sit at their logical positions behind the tree
 * windows' own spacers, and the keyboard moves one tab stop across rows the host has not streamed yet.
 * Testing Library's role queries are the third-party oracle for the accessibility tree. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { TreeWindowContext, UiDocumentStore, UiNodeView, treeItemToTreeData, tableColumnWindowRequestV1, tableWindowNextColumnV1, tableWindowNextRowV1, tableWindowScrollLeftForColumnV1, tableWindowScrollTopForRowV1, tableWindowViewportCapRowsV1, treeWindowRowHeightPx } = dependencies;
  const { describe, expect, it, afterEach } = vitest;

  const { cleanup, fireEvent, render, screen } = await import("@semio-tech/ui-react/test");
  const { treeRowHeightPx } = await import("@semio-tech/ui-react");
  const { createElement } = await import("react");
  const { readFileSync } = await import("node:fs");
  const { dirname, join } = await import("node:path");
  const { fileURLToPath } = await import("node:url");
  const { default: Ajv2020 } = await import("ajv/dist/2020.js");

  const fixtureDir = join(dirname(fileURLToPath(source.url)), "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/📊️table");
  const snapshot = JSON.parse(readFileSync(join(fixtureDir, "📸️snapshot.json"), "utf8"));
  const matrixFixtureDir = join(dirname(fileURLToPath(source.url)), "../../../../🔌️plugin/🪟️window-kits/📊️table/🧫️fixtures/↔️two-axis");
  const matrixFixture = JSON.parse(readFileSync(join(matrixFixtureDir, "🔣️.json"), "utf8"));
  const matrixSchema = JSON.parse(readFileSync(join(matrixFixtureDir, "🧬️schema/🔣️.json"), "utf8"));
  const rowTargetDir = join(dirname(fileURLToPath(source.url)), "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🎯️row-target");
  const rowTarget = JSON.parse(readFileSync(join(rowTargetDir, "🔣️.json"), "utf8"));
  const rowTargetSchema = JSON.parse(readFileSync(join(rowTargetDir, "🧬️schema/🔣️.json"), "utf8"));

  function mount(onIntent: (intent: any) => void, windows: unknown = null, sourceSnapshot: any = snapshot) {
    const store = new UiDocumentStore(sourceSnapshot.surface);
    store.loadSnapshot(sourceSnapshot);
    const view = createElement(UiNodeView, { store, id: sourceSnapshot.root, context: { store, onAction: () => {}, onIntent } });
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
      expect(screen.getByRole("status").textContent).toBe("Rows 13–14 of 40 · 1–2 / 2");
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

    it("renders windowed cell children as accessible local drafts and commits the typed value", () => {
      const intents: any[] = [];
      mount((intent) => intents.push(intent));
      const name = screen.getByRole("textbox", { name: "Name" });
      expect(name.getAttribute("value")).toBe("Atelier Ada");
      const row = name.closest<HTMLElement>("[role='row']")!;
      fireEvent.click(row);
      fireEvent.doubleClick(row);
      expect(intents).toEqual([]);
      fireEvent.doubleClick(name);
      expect(intents).toEqual([]);
      fireEvent.change(name, { target: { value: "Atelier Neu" } });
      fireEvent.blur(name);
      expect(intents).toHaveLength(1);
      expect(intents[0].action.name).toBe("set-cell");
      expect(intents[0].args).toEqual({ row: 12, column: 0, revision: "0123456789abcdef" });
      expect(intents[0].input).toBe("Atelier Neu");
      const remove = screen.getByRole("button", { name: "Remove row: Atelier Ada" });
      remove.focus();
      expect(document.activeElement).toBe(remove);
      fireEvent.click(remove);
      expect(intents).toHaveLength(2);
      expect(intents[1].action.name).toBe("remove-row");
      expect(intents[1].args).toEqual({ row: 12, revision: "0123456789abcdef" });
    });

    it("discards a cell draft on Escape without row focus committing it on blur", () => {
      const intents: any[] = [];
      const draftFixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "../../🧫️fixtures/♿️editable-controls/🔣️.json"), "utf8"));
      mount((intent) => intents.push(intent));
      const name = screen.getByRole("textbox", { name: "Name" }) as HTMLInputElement;
      const published = name.value;
      name.focus();
      fireEvent.change(name, { target: { value: draftFixture.cases[0].draft } });
      fireEvent.keyDown(name, { key: "Escape" });
      expect(name.value).toBe(published);
      expect(intents).toHaveLength(draftFixture.commitCounts.untouched);
      expect(document.activeElement).toBe(name);
      fireEvent.blur(name);
      expect(intents).toHaveLength(draftFixture.commitCounts.untouched);
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
      const viewportHeight = 10 * treeWindowRowHeightPx();
      Object.defineProperty(scroller, "clientHeight", { value: viewportHeight, configurable: true });
      Object.defineProperty(scroller, "scrollHeight", { value: 40 * treeRowHeightPx, configurable: true });
      scroller.getBoundingClientRect = () => rect(0, viewportHeight);
      container.getBoundingClientRect = () => rect(-scrollTop, 40 * treeRowHeightPx);
      for (const row of Array.from(container.querySelectorAll<HTMLElement>("[data-tree-window-row]"))) {
        const index = Number(row.getAttribute("data-tree-window-row"));
        row.getBoundingClientRect = () => rect(index * treeRowHeightPx - scrollTop, treeRowHeightPx);
      }
      const rows = Array.from(grid.querySelectorAll<HTMLElement>("[role='row']")).slice(1);
      fireEvent.keyDown(rows[1]!, { key: "End" });
      expect(scrollTop).toBe(41 * treeRowHeightPx - viewportHeight);
      scroller.dispatchEvent(new Event("scroll"));
      await new Promise((resolve) => setTimeout(resolve, 60));
      const last = reports.at(-1);
      expect(last.viewportRows).toBe(9);
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

    it("caps the scroll viewport at the rows the guest serves, by the fixture's viewport law", () => {
      const served = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🪟️tree-window-served.json"), "utf8"));
      expect(served.tableViewportCaps.length).toBeGreaterThan(0);
      for (const row of served.tableViewportCaps) expect(tableWindowViewportCapRowsV1(row.learned, row.previous, row.total), row.name).toBe(row.capRows);
    });

    it("renders no DOM id twice: two editable rows' cells live under their own row", () => {
      const two = structuredClone(snapshot);
      const editable = two.nodes.find((node: any) => node.key === "space:sp-1");
      const cells = two.nodes.filter((node: any) => (editable.children ?? []).includes(node.id));
      const offset = Math.max(...two.nodes.map((node: any) => node.id)) + 1;
      const copies = cells.map((cell: any, index: number) => ({ ...structuredClone(cell), id: offset + index + 1 }));
      two.nodes.push({ ...structuredClone(editable), id: offset, key: "space:sp-3", component: { ...structuredClone(editable.component), cells: ["Atelier Bea", "atelier"] }, children: copies.map((cell: any) => cell.id) }, ...copies);
      const table = two.nodes.find((node: any) => node.id === two.root);
      table.children = [...table.children, offset];
      const view = mount(() => {}, null, two);
      const ids = Array.from(view.container.querySelectorAll<HTMLElement>("[id]")).map((element) => element.id);
      expect(ids.filter((id, index) => ids.indexOf(id) !== index)).toEqual([]);
      const rows = Array.from(screen.getByRole("grid", { name: "Spaces" }).querySelectorAll<HTMLElement>("[role='row'][data-ui-node-key^='space:']"));
      for (const row of rows) for (const input of Array.from(row.querySelectorAll<HTMLInputElement>("input"))) expect(input.id.startsWith(`${row.id}/`)).toBe(true);
    });

    it("windows columns independently and preserves their logical addresses", () => {
      const validate = new Ajv2020({ strict: true }).compile(matrixSchema);
      expect(validate(matrixFixture), JSON.stringify(validate.errors)).toBe(true);
      const wide = structuredClone(snapshot);
      const table = wide.nodes[0].component;
      table.columns = matrixFixture.headers;
      table.rowLabel = "Row";
      table.columnLabel = "Column";
      table.columnWindow = { total: matrixFixture.columnTotal, offset: matrixFixture.columnOffset, rowExtent: "standard" };
      for (const [rowPosition, rowId] of wide.nodes[0].children.entries()) {
        const row = wide.nodes.find((node: any) => node.id === rowId);
        row.component.cells = matrixFixture.cells[rowPosition];
        for (const [cellPosition, childId] of (row.children ?? []).entries()) {
          const cell = wide.nodes.find((node: any) => node.id === childId);
          cell.key = `cell-${matrixFixture.columnOffset + cellPosition}`;
          cell.component.value = matrixFixture.cells[rowPosition][cellPosition];
          cell.accessibility.label = matrixFixture.headers[cellPosition];
          cell.bindings[0].args.column = matrixFixture.columnOffset + cellPosition;
        }
      }
      mount(() => {}, null, wide);
      const grid = screen.getByRole("grid", { name: "Spaces" });
      expect(grid.getAttribute("aria-colcount")).toBe("1001");
      expect(Array.from(grid.querySelectorAll<HTMLElement>("[role='columnheader']")).slice(0, 3).map((header) => header.getAttribute("aria-colindex"))).toEqual(["701", "702", "703"]);
      expect(Array.from(grid.querySelectorAll<HTMLElement>("[role='gridcell'][data-table-column-index]")).slice(0, 3).map((cell) => cell.getAttribute("data-table-column-index"))).toEqual(["700", "701", "702"]);
      expect(screen.getByRole("status").textContent).toContain("Column: 701–703 / 1000");
      expect(tableColumnWindowRequestV1("spaces.columns", 1000, 700 * 192, 3 * 192)).toEqual({ nodeKey: "spaces.columns", offset: 699, rows: 5 });
      expect(tableWindowNextColumnV1("ArrowRight", 702, 1000)).toBe(703);
      expect(tableWindowNextColumnV1("End", 702, 1000)).toBe(999);
      expect(tableWindowScrollLeftForColumnV1(703, 192, 700 * 192, 3 * 192)).toBe(701 * 192);
    });

    it("does not borrow a destructive row action as row activation", () => {
      const intents: any[] = [];
      mount((intent) => intents.push(intent));
      const row = screen.getByRole("textbox", { name: "Name" }).closest<HTMLElement>("[role='row']")!;
      row.focus();
      fireEvent.keyDown(row, { key: "Enter" });
      expect(intents).toEqual([]);
      fireEvent.click(screen.getByRole("button", { name: "Remove row: Atelier Ada" }));
      expect(intents.map((intent) => intent.action.name)).toEqual(["remove-row"]);
    });

    it("dispatches a tree row and a table row with one target identically, by the row-target fixture", () => {
      expect(new Ajv2020({ strict: false }).validate(rowTargetSchema, rowTarget)).toBe(true);
      const row = (name: string) => rowTarget.rows.find((candidate: any) => candidate.case === name).component;
      const record = (id: number, key: string, component: any, children: number[] = []) => ({ id, key, component, layout: { kind: "stack", axis: "vertical", gap: "none", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: false }, style: {}, activity: "idle", disabled: false, transition: null, accessibility: {}, bindings: [], menu: null, children });
      const surface = "panel:row-target";
      const nodes = [
        record(0, "root", { type: "container" }, [1, 4]),
        record(1, "tree", { type: "tree" }, [2]),
        record(2, "section", { type: "treeSection", label: "Spaces" }, [3]),
        record(3, "tree-row", row("tree-row")),
        record(4, "table", { type: "table", label: "Spaces", columns: ["Name", "Kind"], actionsLabel: "Actions" }, [5]),
        record(5, "table-row", row("table-row")),
      ];
      const store = new UiDocumentStore(surface);
      store.loadSnapshot({ surface, revision: 1, root: 0, nodes });
      const dispatched = (intents: any[]) => intents.map((intent) => ({ verb: intent.action.name, binding: { trigger: intent.trigger, action: intent.action, args: intent.args } }));
      const treeIntents: any[] = [];
      const treeRecord = store.getState().nodes.get(3);
      const treeData = treeItemToTreeData(store, store.getState(), { record: treeRecord, props: treeRecord.component }, { store, onAction: () => {}, onIntent: (intent: any) => treeIntents.push(intent) }, { byKey: new Map() });
      treeData.onClick();
      for (const action of treeData.actions) action.onClick();
      expect(treeData.actions.map((action: any) => action.disabled)).toEqual(rowTarget.rowActions.map((action: any) => action.disabled === true));
      const tableIntents: any[] = [];
      render(createElement(UiNodeView, { store, id: 4, context: { store, onAction: () => {}, onIntent: (intent: any) => tableIntents.push(intent) } }));
      const tableRow = screen.getByRole("grid", { name: "Spaces" }).querySelectorAll<HTMLElement>("[role='row']")[1]!;
      tableRow.focus();
      fireEvent.keyDown(tableRow, { key: "Enter" });
      for (const action of rowTarget.rowActions) {
        const button = screen.getByRole("button", { name: `${action.label}: Studio` }) as HTMLButtonElement;
        expect([button.getAttribute("aria-disabled") === "true", button.disabled]).toEqual([action.disabled === true, false]);
        fireEvent.click(button);
      }
      const answered = (intents: any[]) => [...dispatched(intents), ...rowTarget.rowActions.filter((action: any) => action.disabled === true).map((action: any) => ({ verb: action.verb, refusal: "disabled" }))];
      expect(dispatched(treeIntents)).toEqual(dispatched(tableIntents));
      expect(answered(treeIntents)).toEqual(rowTarget.dispatch);
    });
  });
}
