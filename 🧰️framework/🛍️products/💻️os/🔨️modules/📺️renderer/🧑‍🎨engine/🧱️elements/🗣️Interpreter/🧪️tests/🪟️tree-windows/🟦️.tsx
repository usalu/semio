type TestSource = { readonly url: string };

/** 🪟️ The interpreter's half of the virtualised panel tree (🎫️ 26/09/16
 * ARTIFACT-TREE-VIRTUALISED-STREAMING, 📓️design-virtualised-tree.md §6.2): a container's authored
 * `window`/`windowKey` reaching the `🌳️Tree` element unchanged, and a `granularity` row synthesising
 * the tree-level `interactionSelect` pick that replaces the per-row action maps. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { TreeWindowContext, UiDocumentStore, UiNodeView, treeItemToTreeData, treePickIntentInputV1, treePickTargetsV1, treeWindowBodyRequestsV1, treeWindowContainersUnder, treeWindowRowHeightPx, treeWindowScrollViewport, treeWindowViewportMetrics } = dependencies;
  const { describe, expect, it, afterEach } = vitest;
  void source;

  const { cleanup, fireEvent, render } = await import("@semio-tech/ui-react/test");
  const { Scrollable, TREE_WINDOW_BODY_NODE_BUDGET, TREE_WINDOW_PATH_SEPARATOR } = await import("@semio-tech/ui-react");
  const { createElement } = await import("react");

  type AnyRecord = Record<string, any>;

  const TEST_STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
  const TEST_ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false };
  const SURFACE = "panel:outliner";

  function node(id: number, key: string, component: AnyRecord, children: readonly number[] = [], bindings: readonly AnyRecord[] = []): AnyRecord {
    return { id, key, component, layout: { kind: "leaf", width: "hug", height: "hug" }, style: TEST_STYLE, activity: "idle", disabled: false, transition: null, accessibility: TEST_ACCESSIBILITY, bindings: [...bindings], menu: null, children: [...children] };
  }

  function treeItem(id: number, key: string, label: string, extra: AnyRecord = {}, children: readonly number[] = [], bindings: readonly AnyRecord[] = []): AnyRecord {
    return node(id, key, { type: "treeItem", label, description: null, icon: null, defaultOpen: null, draggable: null, dragData: null, dimmed: null, window: null, granularity: null, rowActions: [], ...extra }, children, bindings);
  }

  const selectBinding = { trigger: "activate", action: { scope: "outliner", name: "interactionSelect", version: 1 }, args: { domainId: "outliner.objects" }, capability: null };

  function mount(nodes: readonly AnyRecord[], root: number, onIntent: (intent: unknown) => void) {
    const store = new UiDocumentStore(SURFACE);
    store.loadSnapshot({ surface: SURFACE, revision: 1, root, nodes: [...nodes] });
    return render(createElement(UiNodeView, { store, id: root, context: { store, onAction: () => {}, onIntent } }));
  }

  const rowHeightPx = treeWindowRowHeightPx();
  const frame = () => new Promise((resolve) => setTimeout(resolve, 40));

  /** 📐️ jsdom measures every box as zero, so a layout law has to state the layout it is testing. */
  function stubExtent(element: HTMLElement, clientHeight: number, scrollHeight: number): void {
    Object.defineProperty(element, "clientHeight", { value: clientHeight, configurable: true });
    Object.defineProperty(element, "scrollHeight", { value: scrollHeight, configurable: true });
  }

  function stubRect(element: HTMLElement, topRows: number, heightRows: number): void {
    const top = topRows * rowHeightPx;
    const height = heightRows * rowHeightPx;
    element.getBoundingClientRect = () => ({ top, bottom: top + height, left: 0, right: 0, width: 0, height, x: 0, y: top, toJSON: () => ({}) }) as DOMRect;
  }

  function scrollTo(area: HTMLElement, scrollTopRows: number, viewportRows: number): void {
    Object.defineProperty(area, "scrollTop", { value: scrollTopRows * rowHeightPx, configurable: true });
    stubRect(area, 0, viewportRows);
  }

  /** 📐️ The container's own materialised rows, laid out at one row pitch each starting `firstRowTopRows`
   * below the viewport origin — the geometry a container none of whose rows is expanded really has. */
  function ownRows(container: HTMLElement): HTMLElement[] {
    return (Array.from(container.querySelectorAll("[data-tree-window-row]")) as HTMLElement[]).filter((row) => row.closest("[data-tree-window-path]") === container);
  }

  function stubRows(container: HTMLElement, firstRowTopRows: number): void {
    ownRows(container).forEach((row, index) => stubRect(row, firstRowTopRows + index, 1));
  }

  /** 🔑️ Collects what the observer shouts while `run` executes, so a law can assert on silence too. */
  async function captureErrors(run: () => Promise<void>): Promise<string[]> {
    const messages: string[] = [];
    const original = console.error;
    console.error = (...args: unknown[]) => void messages.push(String(args[0]));
    try {
      await run();
    } finally {
      console.error = original;
    }
    return messages;
  }

  const nodeCost = (requests: readonly AnyRecord[]) => requests.length + requests.reduce((sum: number, request: AnyRecord) => sum + request.rows, 0);

  /** 🪟️ The real `📜️Scrollable` around a windowed guest tree — the exact chain a panel body renders. */
  function mountScrollable(containers: readonly { readonly key: string; readonly total: number }[], reportWindows: (requests: readonly AnyRecord[], viewportRows: number) => void) {
    const nodes: AnyRecord[] = [node(1, "outliner", { type: "tree", interactionDomain: "outliner.objects" }, containers.map((_, index) => 2 + index * 2))];
    containers.forEach((container, index) => {
      const sectionId = 2 + index * 2;
      nodes.push(node(sectionId, container.key, { type: "treeSection", label: container.key, defaultOpen: true, window: { rowExtent: "standard", total: container.total, offset: 0 } }, [sectionId + 1]));
      nodes.push(treeItem(sectionId + 1, `${container.key}.0`, `${container.key} row`));
    });
    return renderWindowedTree(nodes, reportWindows, (tree) => createElement(Scrollable, null, tree));
  }

  /** 🪟️ The same guest tree under an arbitrary host chain, so the scroll-container rule can be stated
   * against a `📜️Scrollable`, a plain `overflow-y: auto` ancestor and no scroller at all. */
  function renderWindowedTree(nodes: readonly AnyRecord[], reportWindows: (requests: readonly AnyRecord[], viewportRows: number) => void, wrap: (tree: any) => any) {
    const store = new UiDocumentStore(SURFACE);
    store.loadSnapshot({ surface: SURFACE, revision: 1, root: 1, nodes: [...nodes] });
    const channel = { bodyKey: SURFACE, openStates: {}, setOpen: () => {}, reportWindows };
    return render(wrap(createElement(TreeWindowContext.Provider, { value: channel }, createElement(UiNodeView, { store, id: 1, context: { store, onAction: () => {}, onIntent: () => {} } }))));
  }

  /** 🪟️ A section of `total` rows whose materialised row `objects.1` is itself an OPEN windowed group —
   * the "item inside section" shape every app panel builds once a branch is expanded. */
  function nestedNodes(sectionTotal: number, sectionLength: number, childTotal: number, childLength: number): AnyRecord[] {
    const childRows = Array.from({ length: childLength }, (_, index) => treeItem(100 + index, `objects.1.${index}`, `Child ${index}`));
    const sectionRows: AnyRecord[] = [
      treeItem(10, "objects.0", "Row 0"),
      treeItem(11, "objects.1", "Row 1", { defaultOpen: true, window: { rowExtent: "standard", total: childTotal, offset: 0 } }, childRows.map((_, index) => 100 + index)),
      ...Array.from({ length: Math.max(0, sectionLength - 2) }, (_, index) => treeItem(12 + index, `objects.${2 + index}`, `Row ${2 + index}`)),
    ];
    return [
      node(1, "outliner", { type: "tree", interactionDomain: "outliner.objects" }, [2]),
      node(2, "objects", { type: "treeSection", label: "Objects", defaultOpen: true, window: { rowExtent: "standard", total: sectionTotal, offset: 0 } }, sectionRows.map((row) => row.id)),
      ...sectionRows,
      ...childRows,
    ];
  }

  /** 🪟️ Windowed containers by their window PATH — the identity the host keys everything by. */
  const windowedByKey = (container: HTMLElement): Map<string, HTMLElement> => new Map(Array.from(container.querySelectorAll("[data-tree-window-path]")).map((element) => [(element as HTMLElement).getAttribute("data-tree-window-path")!, element as HTMLElement]));

  describe("🪟️ interpreted tree windows", () => {
    afterEach(() => cleanup());

    it("carries a container's authored window and its authored key onto the tree data, never the volatile node id", () => {
      const store = new UiDocumentStore(SURFACE);
      store.loadSnapshot({
        surface: SURFACE,
        revision: 1,
        root: 1,
        nodes: [treeItem(1, "objects", "Objects", { window: { rowExtent: "standard", total: 4096, offset: 128 } }, [2]), treeItem(2, "objects.7", "Object 7")],
      });
      const state = store.getState();
      const record = state.nodes.get(1);
      const data = treeItemToTreeData(store, state, { record, props: record.component }, { store, onAction: () => {}, onIntent: () => {} }, { byKey: new Map() });
      expect(data.window).toEqual({ total: 4096, offset: 128 });
      expect(data.windowKey).toBe("objects");
      expect(data.id).toBe(`${SURFACE}/objects`);
      // 🪟️ An unwindowed row stamps neither, so the `🌳️Tree` element renders no spacers for it.
      const leaf = state.nodes.get(2);
      const leafData = treeItemToTreeData(store, state, { record: leaf, props: leaf.component }, { store, onAction: () => {}, onIntent: () => {} }, { byKey: new Map() });
      expect(leafData.window).toBeUndefined();
      expect(leafData.windowKey).toBe("objects.7");
    });

    it("synthesises the tree-level interactionSelect pick for a granularity row that binds nothing itself", () => {
      const intents: any[] = [];
      const rendered = mount(
        [
          node(1, "outliner", { type: "tree", interactionDomain: "outliner.objects" }, [2], [selectBinding]),
          node(2, "outliner.objects", { type: "treeSection", label: "Objects", defaultOpen: true, window: null }, [3]),
          treeItem(3, "seed-left-001", "Seed Left", { granularity: "piece" }),
        ],
        1,
        (intent) => intents.push(intent),
      );
      fireEvent.click(rendered.getByText("Seed Left"));
      expect(intents.length).toBe(1);
      // 🕹️ The intent is the TREE's binding — the row itself costs zero argument arena.
      expect(intents[0].nodeKey).toBe("outliner");
      expect(intents[0].action).toEqual({ scope: "outliner", name: "interactionSelect", version: 1 });
      expect(intents[0].args).toEqual({ domainId: "outliner.objects" });
      expect(intents[0].input).toEqual({ merge: "replace", method: "pick", targets: JSON.stringify([{ granularity: "piece", id: "seed-left-001" }]) });
    });

    it("leaves a row that binds its own activate on its own action, granularity or not", () => {
      const intents: any[] = [];
      const own = { trigger: "activate", action: { scope: "outliner", name: "addObjectKind", version: 1 }, args: { objectKind: "Seed Left" }, capability: null };
      const rendered = mount(
        [
          node(1, "outliner", { type: "tree", interactionDomain: "outliner.objects" }, [2], [selectBinding]),
          node(2, "outliner.objects", { type: "treeSection", label: "Objects", defaultOpen: true, window: null }, [3]),
          treeItem(3, "seed-left-001", "Seed Left", { granularity: "piece" }, [], [own]),
        ],
        1,
        (intent) => intents.push(intent),
      );
      fireEvent.click(rendered.getByText("Seed Left"));
      expect(intents.length).toBe(1);
      expect(intents[0].action.name).toBe("addObjectKind");
      expect(intents[0].input).toBeNull();
    });

    /** 🧯️ The defect 📓️w3-browser-verification.md §5 measured in three browser lanes: the observer bound
     * to `📜️Scrollable`'s INNER content div, whose height is its content's, so `scrollTop` was
     * permanently 0 and no scroll ever produced a new window request. The numbers below are the ones
     * read off the fem3d House panel (466/3722 against 3720/3720). */
    it("resolves the scroll container to the element that actually scrolls, never the unbounded Scrollable viewport", () => {
      const rendered = mountScrollable([{ key: "objects", total: 200 }], () => {});
      const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
      const viewport = rendered.container.querySelector('[data-slot="scroll-area-viewport"]') as HTMLElement;
      stubExtent(area, 466, 3722);
      stubExtent(viewport, 3720, 3720);
      const windowed = rendered.container.querySelector("[data-tree-window-key]") as HTMLElement;
      expect(windowed).toBeTruthy();

      const resolved = treeWindowScrollViewport(windowed) as HTMLElement;

      expect(resolved).toBe(area);
      expect(resolved.scrollHeight).toBeGreaterThan(resolved.clientHeight);
      expect(viewport.scrollHeight - viewport.clientHeight).toBe(0);
    });

    it("falls back to a plain overflow ancestor, and to the page itself, for a tree outside any Scrollable", () => {
      const nodes = [
        node(1, "outliner", { type: "tree", interactionDomain: "outliner.objects" }, [2]),
        node(2, "objects", { type: "treeSection", label: "Objects", defaultOpen: true, window: { rowExtent: "standard", total: 200, offset: 0 } }, [3]),
        treeItem(3, "objects.0", "Row 0"),
      ];
      const overflowing = renderWindowedTree(nodes, () => {}, (tree) => createElement("div", { "data-testid": "plain", style: { overflowY: "auto" } }, tree));
      const plain = overflowing.container.querySelector('[data-testid="plain"]') as HTMLElement;
      const contents = overflowing.container.querySelector("div.contents") as HTMLElement;
      expect(treeWindowScrollViewport(contents)).toBe(plain);
      cleanup();

      const bare = renderWindowedTree(nodes, () => {}, (tree) => tree);
      const bareContents = bare.container.querySelector("div.contents") as HTMLElement;
      // 🧯️ No scrolling ancestor at all is still a scrolling document: the page scrolls, so the observer
      // measures against it rather than returning `null` and never streaming a row.
      // 📐️ jsdom leaves `document.scrollingElement` unimplemented; the resolver falls through to
      // `documentElement`, which is what a browser's `scrollingElement` is in standards mode anyway.
      expect(treeWindowScrollViewport(bareContents)).toBe(document.scrollingElement ?? document.documentElement);
    });

    it("prefers whichever candidate actually overflows, even when it is the guest tree's own root", () => {
      const rendered = mountScrollable([{ key: "objects", total: 200 }], () => {});
      const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
      const contents = rendered.container.querySelector("div.contents") as HTMLElement;
      const treeRoot = contents.firstElementChild as HTMLElement;
      treeRoot.style.overflowY = "auto";
      stubExtent(area, 500, 500);
      stubExtent(treeRoot, 400, 3000);

      expect(treeWindowScrollViewport(contents)).toBe(treeRoot);
    });

    it("measures the viewport by its client box, never by the border box a scrollbar and a border inflate", () => {
      const rendered = mountScrollable([{ key: "objects", total: 200 }], () => {});
      const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
      stubRect(area, 0, 10);
      Object.defineProperty(area, "clientHeight", { value: 7 * rowHeightPx, configurable: true });
      Object.defineProperty(area, "clientTop", { value: 2, configurable: true });
      const windowed = windowedByKey(rendered.container).get("objects")!;
      stubRect(windowed, 0, 200);

      expect(treeWindowViewportMetrics(area)).toEqual({ originTop: 2, height: 7 * rowHeightPx });
      // 🎯️ The container's own top is relative to the viewport's CONTENT origin, so the border is gone.
      expect(treeWindowContainersUnder(rendered.container, area)).toEqual([{ key: "objects", total: 200, offset: 0, length: 1, top: -2, height: 200 * rowHeightPx, rows: [{ index: 0, top: -2 }] }]);
    });

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

      const last = reports[reports.length - 1];
      expect(last.viewportRows).toBe(10);
      // 🎯️ 40 rows scrolled away, 10 on screen, 8 rows of overscan per edge: rows 32…57 of 200.
      expect(last.requests).toEqual([{ nodeKey: "objects", offset: 32, rows: 26 }]);
    });

    /** 🧯️ The loop this whole mechanism dies of: the observer re-measures on every store revision, so a
     * window the guest has just SETTLED — the rows now on screen, already covered with overscan — must
     * recompute to byte-identical numbers and report nothing. Otherwise every answer asks another
     * question and the body refreshes forever. */
    it("asks for exactly the window it already has once the guest has answered", () => {
      const rows = (offset: number, length: number, firstTopRows: number) => Array.from({ length }, (_, index) => ({ index: offset + index, top: (firstTopRows + index) * rowHeightPx }));
      // 📐️ Before: 200 rows announced, one materialised, the viewport 40 rows down its extent.
      const before = [{ key: "objects", total: 200, offset: 0, length: 1, top: -40 * rowHeightPx, height: 200 * rowHeightPx, rows: rows(0, 1, -40) }];
      const asked = treeWindowBodyRequestsV1(before, 10 * rowHeightPx, rowHeightPx);
      expect(asked).toEqual([{ key: "objects", offset: 32, rows: 26 }]);

      // 📐️ After: the guest materialised rows 32…57, so the leading spacer is 32 rows and the slice starts
      // 8 rows above the viewport top. Same total, same extent, same scroll position.
      const after = [{ ...before[0], offset: 32, length: 26, rows: rows(32, 26, -8) }];

      expect(treeWindowBodyRequestsV1(after, 10 * rowHeightPx, rowHeightPx)).toEqual(asked);
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
      expect(reports[settled - 1].requests).toEqual([{ nodeKey: "objects", offset: 32, rows: 26 }]);

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
      // 🔑️ The nested container is addressed by its PATH, its parent's key then its own.
      expect([...windowed.keys()]).toEqual(["objects", `objects${TREE_WINDOW_PATH_SEPARATOR}objects.1`]);

      scrollTo(area, 30, 20);
      // 📐️ Section: 0 leading + (row 0, then row 1 = a 50-row subtree, then row 2) + 97 trailing = 150 rows.
      const section = windowed.get("objects")!;
      stubRect(section, -30, 150);
      const sectionRows = ownRows(section);
      expect(sectionRows.map((row) => row.getAttribute("data-tree-window-row"))).toEqual(["0", "1", "2"]);
      stubRect(sectionRows[0]!, -30, 1);
      stubRect(sectionRows[1]!, -29, 1);
      stubRect(sectionRows[2]!, 22, 1);
      // 📐️ Child: its own 20 materialised rows + a 30-row trailing spacer, starting 28 rows up.
      const child = windowed.get(`objects${TREE_WINDOW_PATH_SEPARATOR}objects.1`)!;
      stubRect(child, -28, 50);
      stubRows(child, -28);

      area.dispatchEvent(new Event("scroll"));
      await frame();

      const last = reports[reports.length - 1];
      const sectionRequest = last.requests.find((request) => request.nodeKey === "objects")!;
      const childRequest = last.requests.find((request) => request.nodeKey === `objects${TREE_WINDOW_PATH_SEPARATOR}objects.1`)!;
      // 🎯️ The viewport sits inside the section's SECOND row, not its thirtieth.
      expect(sectionRequest).toEqual({ nodeKey: "objects", offset: 0, rows: 17 });
      expect(sectionRequest.offset).toBeLessThanOrEqual(1);
      expect(sectionRequest.offset + sectionRequest.rows).toBeGreaterThan(1);
      // 🎯️ The child is a windowed container in its own right and streams on its own row index.
      expect(childRequest).toEqual({ nodeKey: `objects${TREE_WINDOW_PATH_SEPARATOR}objects.1`, offset: 14, rows: 36 });
      expect(childRequest.offset + childRequest.rows).toBeLessThanOrEqual(50);
      expect(nodeCost(last.requests)).toBeLessThanOrEqual(TREE_WINDOW_BODY_NODE_BUDGET);
    });

    /** 🧯️ The cad lane's `structure-classic`: a default-open container below the fold at a 440 px viewport,
     * answered `rows: 0` on first paint. `rows: 0` renders as an open container with a full-height spacer and
     * no rows — to a reader, indistinguishable from an empty one, wearing the pending ring. It must be seeded
     * with one row while it is off screen, and it must materialise its real window within ONE refresh of
     * being scrolled into view. */
    it("seeds a never-materialised container below the fold, then gives it a real window when it scrolls in", () => {
      const rows = (offset: number, length: number, firstTopRows: number) => Array.from({ length }, (_, index) => ({ index: offset + index, top: (firstTopRows + index) * rowHeightPx }));
      const onScreen = { key: "shape", total: 200, offset: 0, length: 20, top: 0, height: 200 * rowHeightPx, rows: rows(0, 20, 0) };
      // 📐️ Never materialised, and its whole extent sits below a 20-row viewport.
      const belowFold = { key: "structure-classic", total: 11, offset: 0, length: 0, top: 200 * rowHeightPx, height: 11 * rowHeightPx };

      const first = treeWindowBodyRequestsV1([onScreen, belowFold], 20 * rowHeightPx, rowHeightPx);
      const seeded = first.find((request: AnyRecord) => request.key === "structure-classic")!;
      expect(seeded).toEqual({ key: "structure-classic", offset: 0, rows: 1 });
      expect(first.find((request: AnyRecord) => request.key === "shape")!.rows).toBeGreaterThan(0);

      // 🖱️ The reader scrolls it into view — the guest answered the seed, so it now shows one row.
      const scrolledIn = { ...belowFold, length: 1, top: -2 * rowHeightPx, rows: rows(0, 1, -2) };
      const after = treeWindowBodyRequestsV1([{ ...onScreen, top: -202 * rowHeightPx }, scrolledIn], 20 * rowHeightPx, rowHeightPx);
      const materialised = after.find((request: AnyRecord) => request.key === "structure-classic")!;

      expect(materialised.rows).toBeGreaterThan(1);
      // 🎯️ …and within that ONE refresh the whole container fits, so nothing is left behind a spacer.
      expect(materialised).toEqual({ key: "structure-classic", offset: 0, rows: 11 });
      expect(after.find((request: AnyRecord) => request.key === "shape")!.rows).toBe(0);
    });

    /** 🔑️ `📐️cad` builds the same `object.id` under four pane sections and `🏗️fem` builds `case.id` and
     * `combination.id` in one body — and that key is also the pick target id, so it cannot be namespaced
     * (📓️f2-sdk-body-node-ledger.md §10). Under key identity the two containers would share one open state,
     * one window and each other's measurements; under PATH identity they are two windows that happen to name
     * the same entity, and the pick still dispatches the bare key. */
    it("gives the same node key under two different parents two independent windows", async () => {
      const reports: { requests: readonly AnyRecord[]; viewportRows: number }[] = [];
      const shared = (parent: string, id: number) => [
        treeItem(id, "shared", "Shared", { defaultOpen: true, window: { rowExtent: "standard", total: 40, offset: 0 } }, [id + 1]),
        treeItem(id + 1, `${parent}.shared.0`, "Child 0"),
      ];
      const nodes: AnyRecord[] = [
        node(1, "outliner", { type: "tree", interactionDomain: "outliner.objects" }, [2, 3]),
        node(2, "left", { type: "treeSection", label: "Left", defaultOpen: true, window: { rowExtent: "standard", total: 4, offset: 0 } }, [10]),
        node(3, "right", { type: "treeSection", label: "Right", defaultOpen: true, window: { rowExtent: "standard", total: 4, offset: 0 } }, [20]),
        ...shared("left", 10),
        ...shared("right", 20),
      ];
      const rendered = renderWindowedTree(nodes, (requests, viewportRows) => reports.push({ requests, viewportRows }), (tree) => createElement(Scrollable, null, tree));
      const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
      const windowed = windowedByKey(rendered.container);
      const leftShared = `left${TREE_WINDOW_PATH_SEPARATOR}shared`;
      const rightShared = `right${TREE_WINDOW_PATH_SEPARATOR}shared`;
      expect([...windowed.keys()]).toEqual(["left", leftShared, "right", rightShared]);
      // 🎯️ The authored key — the pick target id — is the SAME string for both nested containers.
      expect(windowed.get(leftShared)!.getAttribute("data-tree-window-key")).toBe("shared");
      expect(windowed.get(rightShared)!.getAttribute("data-tree-window-key")).toBe("shared");

      scrollTo(area, 0, 20);
      // 📐️ Only the left branch is on screen; the right one sits far below it.
      stubRect(windowed.get("left")!, 0, 42);
      stubRows(windowed.get("left")!, 0);
      stubRect(windowed.get(leftShared)!, 1, 40);
      stubRows(windowed.get(leftShared)!, 1);
      stubRect(windowed.get("right")!, 60, 42);
      stubRows(windowed.get("right")!, 60);
      stubRect(windowed.get(rightShared)!, 61, 40);
      stubRows(windowed.get(rightShared)!, 61);

      const shouted = await captureErrors(async () => {
        area.dispatchEvent(new Event("scroll"));
        await frame();
      });

      const last = reports[reports.length - 1];
      expect(last.requests.map((request) => request.nodeKey)).toEqual(["left", leftShared, "right", rightShared]);
      // 🎯️ Two independent measurements: the visible one streams rows, the one below the fold asks for none.
      expect(last.requests.find((request) => request.nodeKey === leftShared)!.rows).toBeGreaterThan(0);
      expect(last.requests.find((request) => request.nodeKey === rightShared)).toEqual({ nodeKey: rightShared, offset: 0, rows: 0 });
      // 🎯️ …and one shared node key under two parents is NOT a duplicate.
      expect(shouted.filter((message) => message.includes("[tree-window] duplicate key"))).toEqual([]);
    });

    /** 🔑️ Two containers under one key share one open state and one window on both sides of the wire, and
     * each other's measurement silently overwrites the other's. The host cannot repair it — it says so. */
    it("says so, once, when two windowed containers in one body share a node key", async () => {
      const errors: string[] = [];
      const original = console.error;
      console.error = (...args: unknown[]) => void errors.push(String(args[0]));
      try {
        const rendered = mountScrollable([{ key: "objects", total: 20 }, { key: "shadow", total: 20 }], () => {});
        const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
        const windowed = windowedByKey(rendered.container);
        windowed.get("shadow")!.setAttribute("data-tree-window-path", "objects");
        scrollTo(area, 0, 20);
        stubRect(windowed.get("objects")!, 0, 20);
        stubRect(windowed.get("shadow")!, 20, 20);

        area.dispatchEvent(new Event("scroll"));
        await frame();
        area.dispatchEvent(new Event("scroll"));
        await frame();

        expect(errors.filter((message) => message.includes("[tree-window] duplicate key")).length).toBe(1);
        expect(errors.some((message) => message.includes('"objects"'))).toBe(true);
      } finally {
        console.error = original;
      }
    });

    it("never asks one body for more rows than the guest can present, however many containers are on screen", async () => {
      const reports: { requests: readonly AnyRecord[]; viewportRows: number }[] = [];
      const containers = Array.from({ length: 12 }, (_, index) => ({ key: `c${index}`, total: 12 }));
      const rendered = mountScrollable(containers, (requests, viewportRows) => reports.push({ requests, viewportRows }));
      const area = rendered.container.querySelector('[data-slot="scroll-area"]') as HTMLElement;
      scrollTo(area, 0, 144);
      const elements = Array.from(rendered.container.querySelectorAll("[data-tree-window-key]")) as HTMLElement[];
      expect(elements.length).toBe(12);
      elements.forEach((element, index) => {
        stubRect(element, index * 12, 12);
        stubRows(element, index * 12);
      });

      area.dispatchEvent(new Event("scroll"));
      await frame();

      const last = reports[reports.length - 1];
      // 🧮️ Every container costs its own node plus its rows — the guest's ledger, priced here.
      expect(last.requests.length).toBe(12);
      expect(nodeCost(last.requests)).toBe(TREE_WINDOW_BODY_NODE_BUDGET);
      for (const request of last.requests) expect(request.offset + request.rows).toBeLessThanOrEqual(12);
    });

    it("resolves a range pick into the whole selection under replace, deduplicated per (granularity, id)", () => {
      const walk = { pickTargets: new Map([[`${SURFACE}/a`, { key: "a", granularity: "piece" }], [`${SURFACE}/b`, { key: "b", granularity: "port" }]]) };
      const targets = treePickTargetsV1(walk, "b", "port", "range", [`${SURFACE}/a`, `${SURFACE}/b`, `${SURFACE}/unknown`]);
      expect(targets).toEqual([{ granularity: "piece", id: "a" }, { granularity: "port", id: "b" }]);
      expect(treePickIntentInputV1("range", targets)).toEqual({ merge: "replace", method: "pick", targets: JSON.stringify([{ granularity: "piece", id: "a" }, { granularity: "port", id: "b" }]) });
      // 🎯️ Every other merge word rides through verbatim — `parse_merge_mode` accepts exactly these.
      expect(treePickIntentInputV1("invertive", treePickTargetsV1(walk, "a", "piece", "invertive", [])).merge).toBe("invertive");
      expect(treePickTargetsV1(walk, "a", "piece", "invertive", [`${SURFACE}/b`])).toEqual([{ granularity: "piece", id: "a" }]);
    });
  });
}
