/** 🧰️ The shared interpreted tree window test kit: the destructured interpreter and ui-react test API, the node and tree item builders, the scroll stubs and the shared window fixtures. */


export type TestSource = { readonly url: string };

export type AnyRecord = Record<string, any>;

/** 📐️ jsdom measures every box as zero, so a layout law has to state the layout it is testing. */
export function stubExtent(element: HTMLElement, clientHeight: number, scrollHeight: number): void {
  Object.defineProperty(element, "clientHeight", { value: clientHeight, configurable: true });
  Object.defineProperty(element, "scrollHeight", { value: scrollHeight, configurable: true });
}

/** 📐️ The container's own materialised rows, laid out at one row pitch each starting `firstRowTopRows`
 * below the viewport origin — the geometry a container none of whose rows is expanded really has. */
export function ownRows(container: HTMLElement): HTMLElement[] {
  return (Array.from(container.querySelectorAll("[data-tree-window-row]")) as HTMLElement[]).filter((row) => row.closest("[data-tree-window-path]") === container);
}

/** 🔑️ Collects what the observer shouts while `run` executes, so a law can assert on silence too. */
export async function captureErrors(run: () => Promise<void>): Promise<string[]> {
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

/** 🏭 Creates the shared tree window test kit of one registration from the injected dependencies. */
export async function createTreeWindowsTestKit(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource) {
  const { TreeWindowContext, UiDocumentStore, UiNodeView, treeItemToTreeData, treePickIntentInputV1, treePickTargetsV1, treeWindowBodyRequestsV1, treeWindowContainersUnder, treeWindowRowHeightPx, treeWindowScrollViewport, treeWindowServedRequestsV1, treeWindowViewportMetrics } = dependencies;
  const { describe, expect, it, afterEach } = vitest;
  const { cleanup, fireEvent, render } = await import("@semio-tech/ui-react/test");
  const { Scrollable, TREE_WINDOW_BODY_NODE_BUDGET, TREE_WINDOW_PATH_SEPARATOR } = await import("@semio-tech/ui-react");
  const { createElement } = await import("react");
  const TEST_STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
  const TEST_ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false };
  const SURFACE = "panel:outliner";
  function node(id: number, key: string, component: AnyRecord, children: readonly number[] = [], bindings: readonly AnyRecord[] = []): AnyRecord {
    return { id, key, component, layout: { kind: "leaf", width: "hug", height: "hug" }, style: TEST_STYLE, activity: "idle", disabled: false, transition: null, accessibility: TEST_ACCESSIBILITY, bindings: [...bindings], menu: null, children: [...children] };
  }
  function treeItem(id: number, key: string, label: string, extra: AnyRecord = {}, children: readonly number[] = [], bindings: readonly AnyRecord[] = []): AnyRecord {
    return node(id, key, { type: "treeItem", label, description: null, icon: null, defaultOpen: null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, rowActions: [], target: null, ...extra }, children, bindings);
  }
  const selectBinding = { trigger: "activate", action: { scope: "outliner", name: "interactionSelect", version: 1 }, args: { domainId: "outliner.objects" }, capability: null };
  function mount(nodes: readonly AnyRecord[], root: number, onIntent: (intent: unknown) => void) {
    const store = new UiDocumentStore(SURFACE);
    store.loadSnapshot({ surface: SURFACE, revision: 1, root, nodes: [...nodes] });
    return render(createElement(UiNodeView, { store, id: root, context: { store, onAction: () => {}, onIntent } }));
  }
  const rowHeightPx = treeWindowRowHeightPx();
  const frame = () => new Promise((resolve) => setTimeout(resolve, 40));
  function stubRect(element: HTMLElement, topRows: number, heightRows: number): void {
    const top = topRows * rowHeightPx;
    const height = heightRows * rowHeightPx;
    element.getBoundingClientRect = () => ({ top, bottom: top + height, left: 0, right: 0, width: 0, height, x: 0, y: top, toJSON: () => ({}) }) as DOMRect;
  }
  function scrollTo(area: HTMLElement, scrollTopRows: number, viewportRows: number): void {
    Object.defineProperty(area, "scrollTop", { value: scrollTopRows * rowHeightPx, configurable: true });
    stubRect(area, 0, viewportRows);
  }
  function stubRows(container: HTMLElement, firstRowTopRows: number): void {
    ownRows(container).forEach((row, index) => stubRect(row, firstRowTopRows + index, 1));
  }
  const nodeCost = (requests: readonly AnyRecord[]) => requests.length + requests.reduce((sum: number, request: AnyRecord) => sum + request.rows, 0);
  /** 🪟️ The real `📜️Scrollable` around a windowed guest tree — the exact chain a panel body renders. */
  function mountScrollable(containers: readonly { readonly key: string; readonly total: number }[], reportWindows: (requests: readonly AnyRecord[], viewportRows: number) => void) {
    const nodes: AnyRecord[] = [node(1, "outliner", { type: "tree", interactionDomain: "outliner.objects" }, containers.map((_, index) => 2 + index * 2))];
    containers.forEach((container, index) => {
      const sectionId = 2 + index * 2;
      nodes.push(node(sectionId, container.key, { type: "treeSection", label: container.key, defaultOpen: true, headerToolbar: null, window: { rowExtent: "standard", total: container.total, offset: 0 } }, [sectionId + 1]));
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
      node(2, "objects", { type: "treeSection", label: "Objects", defaultOpen: true, headerToolbar: null, window: { rowExtent: "standard", total: sectionTotal, offset: 0 } }, sectionRows.map((row) => row.id)),
      ...sectionRows,
      ...childRows,
    ];
  }
  /** 🪟️ Windowed containers by their window PATH — the identity the host keys everything by. */
  const windowedByKey = (container: HTMLElement): Map<string, HTMLElement> => new Map(Array.from(container.querySelectorAll("[data-tree-window-path]")).map((element) => [(element as HTMLElement).getAttribute("data-tree-window-path")!, element as HTMLElement]));
  const { readFileSync } = await import("node:fs");
  const { dirname, join } = await import("node:path");
  const { fileURLToPath } = await import("node:url");
  return { SURFACE, Scrollable, TREE_WINDOW_BODY_NODE_BUDGET, TREE_WINDOW_PATH_SEPARATOR, UiDocumentStore, afterEach, cleanup, createElement, describe, dirname, expect, fileURLToPath, fireEvent, frame, it, join, mount, mountScrollable, nestedNodes, node, nodeCost, readFileSync, renderWindowedTree, rowHeightPx, scrollTo, selectBinding, source, stubRect, stubRows, treeItem, treeItemToTreeData, treePickIntentInputV1, treePickTargetsV1, treeWindowBodyRequestsV1, treeWindowContainersUnder, treeWindowScrollViewport, treeWindowServedRequestsV1, treeWindowViewportMetrics, windowedByKey };
}
export type TreeWindowsTestKit = Awaited<ReturnType<typeof createTreeWindowsTestKit>>;
