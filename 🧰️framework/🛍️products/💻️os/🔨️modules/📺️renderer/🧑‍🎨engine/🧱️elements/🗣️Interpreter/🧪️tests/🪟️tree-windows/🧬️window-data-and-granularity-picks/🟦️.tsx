/** 🧬️ The authored window data and granularity row pick tests. */
import type { TreeWindowsTestKit } from "../🧰️tree-windows-test-kit/🟦️.tsx";

/** 🎯 Registers the cleanup hook, the authored window and key data and the granularity pick tests. */
export function registerWindowDataAndGranularityPickTests(kit: TreeWindowsTestKit): void {
  const { SURFACE, UiDocumentStore, afterEach, cleanup, expect, fireEvent, it, mount, node, selectBinding, treeItem, treeItemToTreeData } = kit;
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
    expect(data.window).toEqual({ rowExtent: "standard", total: 4096, offset: 128 });
    expect(data.windowKey).toBe("objects");
    expect(data.id).toBe(`${SURFACE}/objects`);
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
        node(2, "outliner.objects", { type: "treeSection", label: "Objects", defaultOpen: true, headerToolbar: null, window: null }, [3]),
        treeItem(3, "seed-left-001", "Seed Left", { granularity: "piece" }),
      ],
      1,
      (intent) => intents.push(intent),
    );
    fireEvent.click(rendered.getByText("Seed Left"));
    expect(intents.length).toBe(1);
    expect(intents[0].nodeKey).toBe("outliner");
    expect(intents[0].action).toEqual({ scope: "outliner", name: "interactionSelect", version: 1 });
    expect(intents[0].args).toEqual({ domainId: "outliner.objects" });
    expect(intents[0].input).toEqual({ merge: "replace", method: "pick", targets: JSON.stringify([{ granularity: "piece", id: "seed-left-001" }]) });
  });
  it("leaves a row that activates its own target verb on that verb, granularity or not", () => {
    const intents: any[] = [];
    const own = { scope: "outliner", version: 1, args: { objectKind: "Seed Left" }, activation: "addObjectKind" };
    const rendered = mount(
      [
        node(1, "outliner", { type: "tree", interactionDomain: "outliner.objects" }, [2], [selectBinding]),
        node(2, "outliner.objects", { type: "treeSection", label: "Objects", defaultOpen: true, headerToolbar: null, window: null }, [3]),
        treeItem(3, "seed-left-001", "Seed Left", { granularity: "piece", target: own }),
      ],
      1,
      (intent) => intents.push(intent),
    );
    fireEvent.click(rendered.getByText("Seed Left"));
    expect(intents.length).toBe(1);
    expect(intents[0].action.name).toBe("addObjectKind");
    expect(intents[0].input).toBeNull();
  });
}
