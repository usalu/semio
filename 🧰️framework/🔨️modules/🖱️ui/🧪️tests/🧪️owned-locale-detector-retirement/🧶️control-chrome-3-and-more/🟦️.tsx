import type { HistoryColumn, PanelDock, PanelTabNode, PanelTabRowDropTarget } from "@semio-tech/ui-react/source";
import type { PanelDockContextValue } from "../../../🔨️modules/🧲️panel-dock/🟦️.tsx";
import panelResizeFixture from "../../🧫️fixtures/↔️panel-resize/🔣️.json" with { type: "json" };
import * as ui from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests2Vitest, Tests2Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerControlChrome3AndMoreTests(vitest: Tests2Vitest, dependencies: Tests2Dependencies, testSource: TestSource): Promise<void> {
  const { CheckIcon, COMPACT_UI_DRIVER, computeTabDockDropZone, DEFAULT_UI_DRIVER, defaultControlRenderer, HistoryTable, moveTabInDock, moveTreeUnitInDock, Navbar, Panel, PanelChromeTabBar, PanelDockContext, pruneEmptyPanelBranches, renderToStaticMarkup, Search, SearchIcon, singleTreeLeaf, Toggle, uiDataLabel, UiDriverProvider } = dependencies;
  const { describe, expect, it, vi } = vitest;

describe("control chrome", () => {

  
      it("moveTabInDock reorders within a row, moves across anchors (incl. into a middle anchor), appends as a child, and no-operations on own-subtree drops", () => {
        const StubIcon = (): null => null;
        const a = singleTreeLeaf({ id: "a", icon: StubIcon, name: "A", tree: { sections: [] } });
        const b = singleTreeLeaf({ id: "b", icon: StubIcon, name: "B", tree: { sections: [] } });
        const c = singleTreeLeaf({ id: "c", icon: StubIcon, name: "C", tree: { sections: [] } });
        const branch: PanelTabNode = { kind: "branch", id: "branch", icon: StubIcon, name: "Branch", children: [] };
        const dock: PanelDock = {
          anchors: { "top-left": [a, b], "top-middle": [], "top-right": [branch], "right-middle": [], "bottom-right": [c], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
        };
  
        const reordered = moveTabInDock(dock, { tabId: "b", fromAnchor: "top-left", target: { kind: "insert", anchor: "top-left", parentPath: [], index: 0 } });
        expect(reordered.anchors["top-left"].map((tab) => tab.id)).toEqual(["b", "a"]);
        expect(reordered.anchors["top-left"].map((tab) => tab.order)).toEqual([0, 1]);
  
        const moved = moveTabInDock(dock, { tabId: "c", fromAnchor: "bottom-right", target: { kind: "insert", anchor: "top-left", parentPath: [], index: 2 } });
        expect(moved.anchors["bottom-right"]).toEqual([]);
        expect(moved.anchors["top-left"].map((tab) => tab.id)).toEqual(["a", "b", "c"]);
  
        const movedToMiddle = moveTabInDock(dock, { tabId: "c", fromAnchor: "bottom-right", target: { kind: "insert", anchor: "top-middle", parentPath: [], index: 0 } });
        expect(movedToMiddle.anchors["bottom-right"]).toEqual([]);
        expect(movedToMiddle.anchors["top-middle"].map((tab) => tab.id)).toEqual(["c"]);
  
        const nested = moveTabInDock(dock, { tabId: "c", fromAnchor: "bottom-right", target: { kind: "child", anchor: "top-right", parentId: "branch" } });
        const nestedBranch = nested.anchors["top-right"][0];
        expect(nestedBranch?.kind === "branch" ? nestedBranch.children.map((child) => child.id) : null).toEqual(["c"]);
        expect(nested.anchors["bottom-right"]).toEqual([]);
  
        expect(moveTabInDock(dock, { tabId: "branch", fromAnchor: "top-right", target: { kind: "child", anchor: "top-right", parentId: "branch" } })).toBe(dock);
        expect(moveTabInDock(dock, { tabId: "branch", fromAnchor: "top-right", target: { kind: "insert", anchor: "top-right", parentPath: ["branch"], index: 0 } })).toBe(dock);
      });
  
      it("moveTabInDock recursively prunes a branch left empty by the move", () => {
        const StubIcon = (): null => null;
        const onlyChild = singleTreeLeaf({ id: "only-child", icon: StubIcon, name: "Only Child", tree: { sections: [] } });
        const innerBranch: PanelTabNode = { kind: "branch", id: "inner", icon: StubIcon, name: "Inner", children: [onlyChild] };
        const outerBranch: PanelTabNode = { kind: "branch", id: "outer", icon: StubIcon, name: "Outer", children: [innerBranch] };
        const dock: PanelDock = {
          anchors: { "top-left": [outerBranch], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
        };
        const result = moveTabInDock(dock, { tabId: "only-child", fromAnchor: "top-left", target: { kind: "insert", anchor: "top-right", parentPath: [], index: 0 } });
        expect(result.anchors["top-left"]).toEqual([]);
        expect(result.anchors["top-right"].map((tab) => tab.id)).toEqual(["only-child"]);
      });
  
      it("pruneEmptyPanelBranches drops branches left with zero children at any depth, and returns the same reference when nothing changes", () => {
        const StubIcon = (): null => null;
        const leaf = singleTreeLeaf({ id: "leaf", icon: StubIcon, name: "Leaf", tree: { sections: [] } });
        const emptyInner: PanelTabNode = { kind: "branch", id: "empty-inner", icon: StubIcon, name: "Empty Inner", children: [] };
        const outer: PanelTabNode = { kind: "branch", id: "outer", icon: StubIcon, name: "Outer", children: [emptyInner] };
        const tabs: readonly PanelTabNode[] = [leaf, outer];
        expect(pruneEmptyPanelBranches(tabs).map((tab) => tab.id)).toEqual(["leaf"]);
        const unchanged: readonly PanelTabNode[] = [leaf];
        expect(pruneEmptyPanelBranches(unchanged)).toBe(unchanged);
      });
  
      it("moveTreeUnitInDock reorders units within a tab and moves a unit across tabs", () => {
        const StubIcon = (): null => null;
        const tabA: PanelTabNode = {
          kind: "leaf",
          id: "tab-a",
          icon: StubIcon,
          name: "Tab A",
          trees: [
            { id: "unit-1", tree: { sections: [] } },
            { id: "unit-2", tree: { sections: [] } },
          ],
        };
        const tabB: PanelTabNode = { kind: "leaf", id: "tab-b", icon: StubIcon, name: "Tab B", trees: [{ id: "unit-3", tree: { sections: [] } }] };
        const dock: PanelDock = {
          anchors: { "top-left": [tabA, tabB], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
        };
  
        const reordered = moveTreeUnitInDock(dock, { unitId: "unit-2", fromTabId: "tab-a", target: { anchor: "top-left", tabId: "tab-a", index: 0 } });
        const reorderedTabA = reordered.anchors["top-left"][0];
        expect(reorderedTabA?.kind === "leaf" ? reorderedTabA.trees.map((unit) => unit.id) : null).toEqual(["unit-2", "unit-1"]);
  
        const crossTab = moveTreeUnitInDock(dock, { unitId: "unit-1", fromTabId: "tab-a", target: { anchor: "top-left", tabId: "tab-b", index: 0 } });
        const crossTabA = crossTab.anchors["top-left"][0];
        const crossTabB = crossTab.anchors["top-left"][1];
        expect(crossTabA?.kind === "leaf" ? crossTabA.trees.map((unit) => unit.id) : null).toEqual(["unit-2"]);
        expect(crossTabB?.kind === "leaf" ? crossTabB.trees.map((unit) => unit.id) : null).toEqual(["unit-1", "unit-3"]);
      });
  
      it("computeTabDockDropZone resolves midpoint inserts and a branch's nest band; misses resolve to null", () => {
        const fabricateRect = (element: HTMLElement, rect: { left: number; right: number; top: number; bottom: number }) => {
          element.getBoundingClientRect = () => ({ ...rect, width: rect.right - rect.left, height: rect.bottom - rect.top, x: rect.left, y: rect.top, toJSON: () => ({}) });
        };
  
        const rowElement = document.createElement("div");
        fabricateRect(rowElement, { left: 0, right: 200, top: 0, bottom: 20 });
        const leafButton = document.createElement("button");
        leafButton.dataset.tabId = "leaf-a";
        leafButton.dataset.tabKind = "leaf";
        fabricateRect(leafButton, { left: 0, right: 100, top: 0, bottom: 20 });
        const branchButton = document.createElement("button");
        branchButton.dataset.tabId = "branch-a";
        branchButton.dataset.tabKind = "branch";
        fabricateRect(branchButton, { left: 100, right: 200, top: 0, bottom: 20 });
        rowElement.append(leafButton, branchButton);
  
        const rows: readonly PanelTabRowDropTarget[] = [{ anchor: "top-left", parentPath: [], rowElement }];
  
        expect(computeTabDockDropZone(20, 10, rows, new Set())).toEqual({ kind: "insert", anchor: "top-left", parentPath: [], index: 0 });
        expect(computeTabDockDropZone(80, 10, rows, new Set())).toEqual({ kind: "insert", anchor: "top-left", parentPath: [], index: 1 });
        expect(computeTabDockDropZone(150, 10, rows, new Set())).toEqual({ kind: "child", anchor: "top-left", parentId: "branch-a" });
        expect(computeTabDockDropZone(150, 10, rows, new Set(["branch-a"]))).toEqual({ kind: "insert", anchor: "top-left", parentPath: [], index: 1 });
  
        expect(computeTabDockDropZone(9999, 9999, rows, new Set())).toBeNull();
      });
  
      it("Panel renders nothing for an empty anchor at rest, but shows a drop zone while a dock drag is in flight", () => {
        const contextValue: PanelDockContextValue = {
          dragTabId: "tab-a",
          draggedSubtreeIds: null,
          dropTarget: { kind: "insert", anchor: "top-middle", parentPath: [], index: 0 },
          startTabDrag: () => {},
          registerTabRowDropTarget: () => {},
          onTreeUnitDockDrop: () => {},
        };
        const atRest = renderToStaticMarkup(<Panel anchor="top-middle" visible={false} tabs={[]} />);
        expect(atRest).toBe("");
  
        const midDrag = renderToStaticMarkup(
          <PanelDockContext.Provider value={contextValue}>
            <Panel anchor="top-middle" visible={false} tabs={[]} />
          </PanelDockContext.Provider>,
        );
        expect(midDrag).toContain('data-slot="panel-empty-drop-zone"');
        expect(midDrag).toContain('data-panel-empty="true"');
        expect(midDrag).toContain("border-accent");
      });
  
      it("PanelChromeTabBar owns the empty-anchor drop zone for chrome-hosted middle anchors", () => {
        const contextValue: PanelDockContextValue = {
          dragTabId: "tab-a",
          draggedSubtreeIds: null,
          dropTarget: { kind: "insert", anchor: "bottom-middle", parentPath: [], index: 0 },
          startTabDrag: () => {},
          registerTabRowDropTarget: () => {},
          onTreeUnitDockDrop: () => {},
        };
        expect(renderToStaticMarkup(<PanelChromeTabBar anchor="bottom-middle" tabs={[]} visible={false} />)).toBe("");
        const midDrag = renderToStaticMarkup(
          <PanelDockContext.Provider value={contextValue}>
            <PanelChromeTabBar anchor="bottom-middle" tabs={[]} visible={false} />
          </PanelDockContext.Provider>,
        );
        expect(midDrag).toContain('data-slot="panel-empty-drop-zone"');
        expect(midDrag).toContain('data-anchor="bottom-middle"');
      });

      it("Panel realizes the schema-owned resize edge, width token, and drag factor for every anchor", async () => {
        const { default: Ajv2020 } = await import("ajv/dist/2020");
        const { render, fireEvent } = await import("@testing-library/react");
        const StubIcon = (): null => null;
        const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } })];
        for (const row of panelResizeFixture.cases) {
          const onSizeChange = vi.fn();
          const view = render(<Panel anchor={row.anchor as never} visible tabs={tabs} size={300} minSize={100} maxSize={1000} onSizeChange={onSizeChange} />);
          const handles = [...view.container.querySelectorAll('[data-slot="panel-resize-handle"]')] as HTMLElement[];
          expect(handles.length, row.anchor).toBe(row.handles.length);
          for (const expected of row.handles) {
            const handle = handles.find((candidate) => candidate.className.includes(`${expected.edge}-0`));
            expect(handle, `${row.anchor}/${expected.edge}`).toBeTruthy();
            expect(handle!.className).toContain("w-single");
            onSizeChange.mockClear();
            fireEvent.pointerDown(handle!, { clientX: 100 });
            fireEvent.pointerMove(handle!, { clientX: 110 });
            expect(onSizeChange, `${row.anchor}/${expected.edge}`).toHaveBeenCalledWith(300 + expected.deltaFactor * 10);
            fireEvent.pointerUp(handle!, { clientX: 110 });
          }
          view.unmount();
        }
      });

      it("Panel's middle anchor centers with translateX(-50%), grows both ways, and gets two independent resize handles", async () => {
        const { render } = await import("@testing-library/react");
        const StubIcon = (): null => null;
        const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } })];
        const { container } = render(<Panel anchor="top-middle" visible tabs={tabs} onSizeChange={() => {}} />);
        const root = container.querySelector('[data-slot="panel"]') as HTMLElement;
        expect(root.style.left).toBe("50%");
        expect(root.style.transform).toBe("translateX(-50%)");
        expect(root.getAttribute("dir")).toBeNull();
        const handles = container.querySelectorAll('[data-slot="panel-resize-handle"]');
        expect(handles.length).toBe(2);
        const sides = [...handles].map((handle) => ((handle as HTMLElement).className.includes("left-0") ? "left" : "right"));
        expect(sides.sort()).toEqual(["left", "right"]);
      });
  
      it("a panel's resize handle lives inside the body stack, never over the tab strip that carries every panel's tab", async () => {
        const { render } = await import("@testing-library/react");
        const StubIcon = (): null => null;
        const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } })];
        const { container } = render(<Panel anchor="bottom-right" visible tabs={tabs} onSizeChange={() => {}} />);
        const handle = container.querySelector('[data-slot="panel-resize-handle"]') as HTMLElement;
        const bodyStack = container.querySelector('[data-slot="panel-body-stack"]') as HTMLElement;
        expect(handle).toBeTruthy();
        expect(bodyStack).toBeTruthy();
        expect(bodyStack.contains(handle)).toBe(true);
        expect(bodyStack.className).toContain("relative");
        const tabBar = container.querySelector('[data-slot="panel-tab-button"], [data-slot="panel-tab-bar"]');
        if (tabBar) expect(handle.contains(tabBar)).toBe(false);
        const panelRoot = container.querySelector('[data-slot="panel"]') as HTMLElement;
        expect([...panelRoot.children].some((child) => child.getAttribute("data-slot") === "panel-resize-handle")).toBe(false);
      });

      it("dragging a middle panel's resize handles changes size by 2x the pointer delta (both edges move to keep it centered); a corner panel changes 1x on its single inner handle", async () => {
        const { render, fireEvent } = await import("@testing-library/react");
        const StubIcon = (): null => null;
        const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } })];
        const onSizeChange = vi.fn();
        const { container } = render(<Panel anchor="top-middle" visible tabs={tabs} size={300} minSize={100} maxSize={1000} onSizeChange={onSizeChange} />);
        const rightHandle = [...container.querySelectorAll('[data-slot="panel-resize-handle"]')].find((handle) => (handle as HTMLElement).className.includes("right-0")) as HTMLElement;
        fireEvent.pointerDown(rightHandle, { clientX: 100 });
        fireEvent.pointerMove(rightHandle, { clientX: 150 });
        expect(onSizeChange).toHaveBeenCalledWith(400);
  
        onSizeChange.mockClear();
        const { container: cornerContainer } = render(<Panel anchor="top-left" visible tabs={tabs} size={300} minSize={100} maxSize={1000} onSizeChange={onSizeChange} />);
        const cornerHandle = cornerContainer.querySelector('[data-slot="panel-resize-handle"]') as HTMLElement;
        fireEvent.pointerDown(cornerHandle, { clientX: 100 });
        fireEvent.pointerMove(cornerHandle, { clientX: 150 });
        expect(onSizeChange).toHaveBeenCalledWith(350);
      });
  
      it("edge-middle panels (left-middle, right-middle) grow from a single inner handle, clamp to the region, and mirror dir on the right", async () => {
        const { render } = await import("@testing-library/react");
        const StubIcon = (): null => null;
        const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } })];
  
        const { container: leftContainer } = render(<Panel anchor="left-middle" visible tabs={tabs} onSizeChange={() => {}} />);
        const leftRoot = leftContainer.querySelector('[data-slot="panel"]') as HTMLElement;
        expect(leftRoot.getAttribute("dir")).toBeNull();
        expect(leftRoot.style.top).toBe("50%");
        expect(leftRoot.style.transform).toBe("translateY(-50%)");
        expect(leftRoot.style.maxHeight).toBe("calc(100% - (var(--panel-inset) * 2))");
        expect(leftContainer.querySelectorAll('[data-slot="panel-resize-handle"]').length).toBe(1);
        expect((leftContainer.querySelector('[data-slot="panel-resize-handle"]') as HTMLElement).className).toContain("right-0");
  
        const { container: rightContainer } = render(<Panel anchor="right-middle" visible tabs={tabs} onSizeChange={() => {}} />);
        const rightRoot = rightContainer.querySelector('[data-slot="panel"]') as HTMLElement;
        expect(rightRoot.getAttribute("dir")).toBe("rtl");
        expect(rightRoot.style.top).toBe("50%");
        expect(rightContainer.querySelectorAll('[data-slot="panel-resize-handle"]').length).toBe(1);
        expect((rightContainer.querySelector('[data-slot="panel-resize-handle"]') as HTMLElement).className).toContain("left-0");
      });
  
      it("navbar hides inline labels under the compact driver", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={COMPACT_UI_DRIVER}>
            <Navbar
              items={[
                {
                  key: "search",
                  content: <Toggle id="ui.search.toggle" pressed={false} onPressedChange={() => undefined} icon={<SearchIcon className="size-small" />} />,
                },
              ]}
            />
          </UiDriverProvider>,
        );
        expect(markup).not.toContain(">Search<");
        expect(markup).toContain('id="ui.search.toggle"');
      });
  
      it("renders search suggestions toggle without internal-id humanized labels", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
            <Search input={{ placeholder: uiDataLabel("Action") }} possibles={[{ id: "primitive.box", label: uiDataLabel("Box"), detail: "b", onSelect: () => {} }]} />
          </UiDriverProvider>,
        );
        expect(markup).toContain('id="ui.windowSearch.suggestions"');
        expect(markup).not.toMatch(/Search Possibles/i);
        expect(markup).not.toMatch(/Possibles Toggle/i);
      });
  
      it("renders panel toggle details with inline label when compact is off", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
            <Toggle id="ui.panelToggle.details" pressed={false} onPressedChange={() => undefined} icon={<CheckIcon className="size-small" />} />
          </UiDriverProvider>,
        );
        expect(markup).toContain("Details");
        expect(markup).toContain("aspect-auto");
        expect(markup).toContain('data-slot="inline-label"');
      });
  
      it("navbar hides workbench and details panel toggle labels under the compact driver", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={COMPACT_UI_DRIVER}>
            <Navbar
              items={[
                {
                  key: "panels",
                  content: (
                    <div className="flex min-w-0 items-stretch border h-medium">
                      <Toggle id="ui.panelToggle.workbench" pressed={false} onPressedChange={() => undefined} icon={<CheckIcon className="size-small" />} className="rounded-none border-0 shrink-0" />
                      <Toggle id="ui.panelToggle.details" pressed={false} onPressedChange={() => undefined} icon={<CheckIcon className="size-small" />} className="rounded-none border-0 border-l shrink-0" />
                      <Toggle id="ui.panelToggle.settings" pressed={false} onPressedChange={() => undefined} icon={<CheckIcon className="size-small" />} className="rounded-none border-0 border-l shrink-0" />
                    </div>
                  ),
                },
              ]}
            />
          </UiDriverProvider>,
        );
        expect(markup).not.toContain(">Workbench<");
        expect(markup).not.toContain(">Details<");
        expect(markup).not.toContain(">Settings<");
      });
  
      it("renders control-tree boolean toggles with inline labels when compact is off", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
            {defaultControlRenderer({
              path: "folder/enabled",
              key: "Enabled",
              controlKind: "boolean",
              value: true,
              onChange: () => undefined,
            })}
          </UiDriverProvider>,
        );
        expect(markup).toContain("Enabled");
        expect(markup).toContain("aspect-auto");
      });
});

  
    describe("HistoryTable", () => {
      it("renders swimlane guides and fork elbows", () => {
        const columns: HistoryColumn[] = [
          {
            checkpointId: "c3",
            timestamp: "3",
            labels: ["feature-b"],
            authors: [],
            parentCheckpointId: "c2",
            description: "branch b",
            lane: 2,
            alternativeIds: ["b"],
          },
          {
            checkpointId: "c2",
            timestamp: "2",
            labels: ["feature-a"],
            authors: [],
            parentCheckpointId: "c1",
            description: "branch a",
            lane: 1,
            alternativeIds: ["a"],
          },
          {
            checkpointId: "c1",
            timestamp: "1",
            labels: ["main"],
            authors: [],
            parentCheckpointId: undefined,
            description: "root",
            lane: 0,
            alternativeIds: [],
          },
        ];
        const markup = renderToStaticMarkup(<HistoryTable id="test.history.table" columns={columns} />);
        expect(markup).toContain('id="test.history.table"');
        expect(markup).toContain('d="M ');
        expect(markup.match(/<line /g)?.length ?? 0).toBeGreaterThanOrEqual(3);
        expect(markup.match(/<circle /g)?.length).toBe(3);
        expect(markup).toMatch(/grid-template-columns:auto \d+px minmax\(0, 1fr\)/);
        expect(markup).toContain("branch b");
        expect(markup).toContain("feature-b");
        expect(markup).toMatch(/width:\d+px/);
      });
  
      it("renders an em dash placeholder for an empty history", () => {
        const markup = renderToStaticMarkup(<HistoryTable id="test.history.table" columns={[]} />);
        expect(markup).toContain('id="test.history.table"');
        expect(markup).toContain("—");
      });
  
      it("falls back to a checkpoint chip and unknown avatar when a row has no labels or authors", () => {
        const columns: HistoryColumn[] = [
          {
            checkpointId: "c1",
            timestamp: "1",
            labels: [],
            authors: [],
            parentCheckpointId: undefined,
            description: undefined,
            lane: 0,
            alternativeIds: [],
          },
        ];
        const markup = renderToStaticMarkup(<HistoryTable id="test.history.table" columns={columns} />);
        expect(markup).toContain("checkpoint");
      });
    });
}
