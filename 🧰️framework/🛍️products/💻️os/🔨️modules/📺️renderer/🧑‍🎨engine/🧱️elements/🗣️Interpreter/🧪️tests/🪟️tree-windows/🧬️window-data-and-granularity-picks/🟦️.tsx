/** 🧬️ The authored window data and granularity row pick tests. */
import hover from "../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/tree-hover.json";
import directHover from "../../../../../../../🔌️plugin/🧪️tests/🕹️interaction-selection-laws/👆️hover-targets.json";
import { uiIntentToActionDescriptor } from "../../../../🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx";
import inspectorHover from "../../../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🖱️channel-hover.json";
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
  it("places a diagram below its disclosure label and keeps inline fields in the value column", () => {
    const rendered = mount([
      node(1, "diagrams", { type: "tree" }, [2]),
      node(2, "cards", { type: "treeSection", label: "Diagrams", defaultOpen: true, window: { rowExtent: "standard", total: 2, offset: 0 } }, [3, 6]),
      treeItem(3, "matrix", "Combination matrix", {}, [4]),
      node(4, "matrix.card", { type: "container", role: "plain" }, [5]),
      node(5, "matrix.plot", { type: "surface", kind: "canvas-2d", docSchema: "canvas-2d@1", doc: { bytes: [] } }),
      treeItem(6, "inputs", "Input", {}, [7]),
      node(7, "inputs.value", { type: "text", value: "2.5" }),
    ], 1, () => {});
    const matrix = rendered.getByText("Combination matrix");
    const plot = rendered.container.querySelector('[data-ui-node-key="matrix.plot"]');
    expect(plot?.closest('[data-slot="tree-item-content"]')).toBeTruthy();
    expect(plot?.closest('[data-slot="tree-item-control"]')).toBeNull();
    expect(rendered.getByText("2.5").closest('[data-slot="tree-item-control"]')).not.toBeNull();
    fireEvent.click(matrix);
    expect(rendered.container.querySelector('[data-ui-node-key="matrix.plot"]')).toBeNull();
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
  it("clears the pointer channel when the pointer leaves a hovered graph row", () => {
    const intents: any[] = [];
    const binding = {
      trigger: "hoverPreview", action: {scope: "outliner", name: "interactionHover", version: 1},
      args: {domainId: hover.domain, channel: hover.channel, targets: JSON.stringify(hover.enter)}, capability: null,
    };
    const rendered = mount([
      node(1, "outliner", {type: "tree", interactionDomain: hover.domain}, [2]),
      node(2, "section", {type: "treeSection", label: "Outputs", defaultOpen: true, headerToolbar: null, window: null}, [3]),
      treeItem(3, hover.row, "Shape", {granularity: "handle"}, [], [binding]),
    ], 1, (intent) => intents.push(intent));
    const row = rendered.getByText("Shape");
    row.dispatchEvent(new MouseEvent("mouseover", {bubbles: true}));
    row.dispatchEvent(new MouseEvent("mouseout", {bubbles: true, relatedTarget: document.body}));
    expect(intents.map(intent => intent.input?.targets ?? intent.args.targets)).toEqual([JSON.stringify(hover.enter), JSON.stringify(hover.leave)]);
    expect(intents.map(intent => intent.args.channel)).toEqual([hover.channel, hover.channel]);
  });
  it("keeps a long direct graph target and applies the merged pointer clear", () => {
    const source = directHover.cases[0];
    const args = {
      domainId: directHover.domain, channel: directHover.channel,
      targetId: source.args.targetId, granularityId: source.args.granularityId,
    };
    const intents: any[] = [];
    const binding = { trigger: "hoverPreview", action: {scope: "outliner", name: "interactionHover", version: 1}, args, capability: null };
    const rendered = mount([
      node(1, "outliner", {type: "tree", interactionDomain: directHover.domain}, [2]),
      node(2, "section", {type: "treeSection", label: "Outputs", defaultOpen: true, headerToolbar: null, window: null}, [3]),
      treeItem(3, args.targetId, "Long channel", {granularity: "handle"}, [], [binding]),
    ], 1, (intent) => intents.push(intent));
    const row = rendered.getByText("Long channel");
    row.dispatchEvent(new MouseEvent("mouseover", {bubbles: true}));
    row.dispatchEvent(new MouseEvent("mouseout", {bubbles: true, relatedTarget: document.body}));
    const descriptors = intents.map(uiIntentToActionDescriptor);
    expect(descriptors.map(descriptor => descriptor.action)).toEqual(["interactionHover", "interactionHover"]);
    expect(descriptors[0].args).toEqual(args);
    expect(descriptors[1].args).toEqual({...args, targets: "[]"});
  });
  it("publishes one hover for a spatial input across its label and axis controls", () => {
    const source = inspectorHover.cases.find(row => row.kind === "literal")!;
    const args = {
      domainId: inspectorHover.domainId, channel: inspectorHover.channel,
      targetId: source.target.id, granularityId: source.target.granularity,
    };
    const intents: any[] = [];
    const binding = { trigger: "hoverPreview", action: { scope: "outliner", name: "interactionHover", version: 1 }, args, capability: null };
    const rendered = mount([
      node(1, source.key + ".coordinates", { type: "container", role: "group", label: source.label }, [2], [binding]),
      node(2, "axis", { type: "input", kind: "number", value: "1" }),
    ], 1, intent => intents.push(intent));
    const label = rendered.getByRole("heading", { name: source.label });
    const axis = rendered.container.querySelector("input")!;
    expect(axis).toBeTruthy();
    label.dispatchEvent(new MouseEvent("pointerover", { bubbles: true }));
    label.dispatchEvent(new MouseEvent("pointerout", { bubbles: true, relatedTarget: axis }));
    axis.dispatchEvent(new MouseEvent("pointerover", { bubbles: true, relatedTarget: label }));
    expect(intents.map(uiIntentToActionDescriptor).map(descriptor => descriptor.args)).toEqual([args]);
    axis.dispatchEvent(new MouseEvent("pointerout", { bubbles: true, relatedTarget: document.body }));
    expect(intents.map(uiIntentToActionDescriptor).map(descriptor => descriptor.args)).toEqual([args, {...args, targets: "[]"}]);
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
