type TestSource = { readonly url: string };

/** 🪟️ The interpreter's half of the virtualised panel tree (🎫️ 26/09/16
 * ARTIFACT-TREE-VIRTUALISED-STREAMING, 📓️design-virtualised-tree.md §6.2): a container's authored
 * `window`/`windowKey` reaching the `🌳️Tree` element unchanged, and a `granularity` row synthesising
 * the tree-level `interactionSelect` pick that replaces the per-row action maps. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { UiDocumentStore, UiNodeView, treeItemToTreeData, treePickIntentInputV1, treePickTargetsV1 } = dependencies;
  const { describe, expect, it, afterEach } = vitest;
  void source;

  const { cleanup, fireEvent, render } = await import("@semio-tech/ui-react/test");
  const { createElement } = await import("react");

  type AnyRecord = Record<string, any>;

  const TEST_STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
  const TEST_ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false };
  const SURFACE = "panel:outliner";

  function node(id: number, key: string, component: AnyRecord, children: readonly number[] = [], bindings: readonly AnyRecord[] = []): AnyRecord {
    return { id, key, component, layout: { kind: "leaf", width: "hug", height: "hug" }, style: TEST_STYLE, activity: "idle", disabled: false, transition: null, accessibility: TEST_ACCESSIBILITY, bindings: [...bindings], menu: null, children: [...children] };
  }

  function treeItem(id: number, key: string, label: string, extra: AnyRecord = {}, children: readonly number[] = [], bindings: readonly AnyRecord[] = []): AnyRecord {
    return node(id, key, { type: "treeItem", label, description: null, icon: null, defaultOpen: null, draggable: null, dragData: null, dimmed: null, rowActions: [], ...extra }, children, bindings);
  }

  const selectBinding = { trigger: "activate", action: { scope: "outliner", name: "interactionSelect", version: 1 }, args: { domainId: "outliner.objects" }, capability: null };

  function mount(nodes: readonly AnyRecord[], root: number, onIntent: (intent: unknown) => void) {
    const store = new UiDocumentStore(SURFACE);
    store.loadSnapshot({ surface: SURFACE, revision: 1, root, nodes: [...nodes] });
    return render(createElement(UiNodeView, { store, id: root, context: { store, onAction: () => {}, onIntent } }));
  }

  describe("🪟️ interpreted tree windows", () => {
    afterEach(() => cleanup());

    it("carries a container's authored window and its authored key onto the tree data, never the volatile node id", () => {
      const store = new UiDocumentStore(SURFACE);
      store.loadSnapshot({
        surface: SURFACE,
        revision: 1,
        root: 1,
        nodes: [treeItem(1, "objects", "Objects", { window: { total: 4096, offset: 128 } }, [2]), treeItem(2, "objects.7", "Object 7")],
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
          node(2, "outliner.objects", { type: "treeSection", label: "Objects", defaultOpen: true }, [3]),
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
          node(2, "outliner.objects", { type: "treeSection", label: "Objects", defaultOpen: true }, [3]),
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
