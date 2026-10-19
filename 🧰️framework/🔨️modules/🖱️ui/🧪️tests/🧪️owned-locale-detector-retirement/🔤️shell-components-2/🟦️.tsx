import type { PanelDock, PanelTabNode, TreeDataSection } from "@semio-tech/ui-react/source";
import * as ui from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerShellComponents2Tests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { FlowProvider, Panel, PanelDockProvider, React, SortableTreeItems, TREE_SECTION_REORDER_MIME, Toggle, Tree, TreeContext, TreeItem, flowFromAnchor, mergeTreeSectionOrder, progressPanelTabSelection, renderToStaticMarkup, singleTreeLeaf, useFlow } = dependencies;
  type Anchor = any;
  type PanelDock = any;
  type PanelTabNode = any;
  type PanelTreeUnit = any;
  type TreeDataSection = any;
  const { describe, expect, it, vi } = vitest;
  const { render, screen, fireEvent } = await import("@testing-library/react");

describe("Shell components", () => {

    it('Panel with tabBarHost="chrome" at top-middle and bottom-middle expands with glass panel level and root chips on the chrome cap', () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [{ kind: "branch", id: "root", icon: StubIcon, name: "Command", children: [singleTreeLeaf({ id: "leaf", icon: StubIcon, name: "Palette", tree: { sections: [] } })] }];
      for (const anchor of ["top-middle", "bottom-middle"] as const) {
        const { container, unmount } = render(<Panel anchor={anchor} tabBarHost="chrome" visible tabs={tabs} activeTabPath={["root", "leaf"]} size={360} />);
        const panel = container.querySelector('[data-slot="panel"]');
        expect(panel?.getAttribute("data-level")).toBe("panel");
        expect(panel?.querySelector(".ui-glass")).toBeTruthy();
        expect(screen.getByText("Command")).toBeTruthy();
        expect(screen.getByText("Palette")).toBeTruthy();
        unmount();
      }
    });

    it("flowFromAnchor mirrors right anchors inline and bottom anchors in block; middle anchors never mirror", () => {
      expect(flowFromAnchor("top-left")).toEqual({ inline: "ltr", block: "down" });
      expect(flowFromAnchor("top-right")).toEqual({ inline: "rtl", block: "down" });
      expect(flowFromAnchor("bottom-left")).toEqual({ inline: "ltr", block: "up" });
      expect(flowFromAnchor("bottom-right")).toEqual({ inline: "rtl", block: "up" });
      expect(flowFromAnchor("top-middle")).toEqual({ inline: "ltr", block: "down" });
      expect(flowFromAnchor("bottom-middle")).toEqual({ inline: "ltr", block: "up" });
    });

    it("Tree direction=up renders a group's content before its own header row, with reversed sibling order", () => {
      const sections: TreeDataSection[] = [
        { id: "sec-a", label: "Section A", defaultOpen: true, items: [{ id: "item-a", label: "Item A" }] },
        { id: "sec-b", label: "Section B", defaultOpen: true, items: [{ id: "item-b", label: "Item B" }] },
      ];
      const downMarkup = renderToStaticMarkup(<Tree sections={sections} />);
      const upMarkup = renderToStaticMarkup(<Tree sections={sections} direction="up" />);

      expect(downMarkup.indexOf("Section A")).toBeLessThan(downMarkup.indexOf("Section B"));
      expect(downMarkup.indexOf("Section A")).toBeLessThan(downMarkup.indexOf("Item A"));

      expect(upMarkup.indexOf("Section B")).toBeLessThan(upMarkup.indexOf("Section A"));
      expect(upMarkup.indexOf("Item A")).toBeLessThan(upMarkup.indexOf("Section A"));
    });

    it("Tree direction=up uses chevron-left (collapsed) / chevron-up (expanded); direction=down keeps chevron-right / chevron-down", () => {
      const sections: TreeDataSection[] = [{ id: "sec", label: "Section", defaultOpen: true, items: [{ id: "item", label: "Item" }] }];
      const downMarkup = renderToStaticMarkup(<Tree sections={sections} />);
      const upMarkup = renderToStaticMarkup(<Tree sections={sections} direction="up" />);
      expect(downMarkup).toContain('data-icon="chevron-down"');
      expect(downMarkup).not.toContain('data-icon="chevron-up"');
      expect(upMarkup).toContain('data-icon="chevron-up"');
      expect(upMarkup).not.toContain('data-icon="chevron-down"');
    });

    it("Tree direction=up collapses to chevron-left instead of chevron-right", () => {
      const sections: TreeDataSection[] = [{ id: "sec", label: "Section", defaultOpen: false, items: [{ id: "item", label: "Item" }] }];
      const upMarkup = renderToStaticMarkup(<Tree sections={sections} direction="up" />);
      expect(upMarkup).toContain('data-icon="chevron-left"');
      expect(upMarkup).not.toContain('data-icon="chevron-right"');
    });

    it("Tree direction=up extends open-group branch stems toward children (top) instead of toward the bottom", () => {
      const sections: TreeDataSection[] = [
        {
          id: "sec",
          label: "Section",
          defaultOpen: true,
          items: [
            {
              id: "group",
              label: "Group",
              defaultOpen: true,
              items: [
                { id: "leaf-a", label: "Leaf A" },
                { id: "leaf-b", label: "Leaf B" },
              ],
            },
          ],
        },
      ];
      const downMarkup = renderToStaticMarkup(<Tree sections={sections} />);
      const upMarkup = renderToStaticMarkup(<Tree sections={sections} direction="up" />);
      const stemStyle = (markup: string) => markup.match(/data-slot="tree-branch-stem"[^>]*style="([^"]*)"/)?.[1] ?? "";
      const downStem = stemStyle(downMarkup);
      const upStem = stemStyle(upMarkup);
      expect(downStem).toContain("top:var(--tree-gutter-center, calc(var(--tree-row-height) / 2))");
      expect(downStem).toContain("bottom:0");
      expect(downStem).not.toContain("top:0");
      expect(upStem).toContain("top:0");
      expect(upStem).toContain("bottom:var(--tree-gutter-center, calc(var(--tree-row-height) / 2))");
      expect(upStem).not.toContain("bottom:0");
    });

    it("progressPanelTabSelection reveals one level per press, records/restores drill-down memory, collapses, and prunes stale memory", () => {
      const StubIcon = (): null => null;
      const leaf = (id: string, name: string) => singleTreeLeaf({ id, icon: StubIcon, name, tree: { sections: [] } });
      const tabs: PanelTabNode[] = [
        {
          kind: "branch",
          id: "root-a",
          icon: StubIcon,
          name: "Root A",
          children: [{ kind: "branch", id: "child-a", icon: StubIcon, name: "Child A", children: [leaf("grandchild-a", "Grandchild A")] }, leaf("child-b", "Child B")],
        },
        { kind: "branch", id: "root-b", icon: StubIcon, name: "Root B", children: [leaf("child-c", "Child C")] },
      ];

      const revealed = progressPanelTabSelection(tabs, [], ["root-a"], {});
      expect(revealed).toEqual({ path: ["root-a"], memory: {}, fold: false });

      const leafSelected = progressPanelTabSelection(tabs, ["root-a"], ["root-a", "child-b"], {});
      expect(leafSelected.path).toEqual(["root-a", "child-b"]);
      expect(leafSelected.memory).toEqual({ "root-a": "child-b" });

      const withDeepMemory = { "root-a": "child-a", "child-a": "grandchild-a" };
      const restored = progressPanelTabSelection(tabs, [], ["root-a"], withDeepMemory);
      expect(restored.path).toEqual(["root-a", "child-a", "grandchild-a"]);
      expect(restored.memory).toEqual(withDeepMemory);

      const collapsed = progressPanelTabSelection(tabs, ["root-a", "child-a", "grandchild-a"], ["root-a", "child-a"], withDeepMemory);
      expect(collapsed).toEqual({ path: ["root-a"], memory: { "root-a": "child-a" }, fold: false });

      const folded = progressPanelTabSelection(tabs, ["root-a", "child-a", "grandchild-a"], ["root-a"], withDeepMemory);
      expect(folded).toEqual({ path: ["root-a", "child-a", "grandchild-a"], memory: withDeepMemory, fold: true });

      const staleMemory = { "root-a": "no-longer-exists" };
      const pruned = progressPanelTabSelection(tabs, [], ["root-a"], staleMemory);
      expect(pruned.path).toEqual(["root-a"]);
      expect(pruned.memory).toEqual({});

      const siblingPick = progressPanelTabSelection(tabs, ["root-a", "child-a"], ["root-a", "child-b"], {});
      expect(siblingPick.path).toEqual(["root-a", "child-b"]);
    });

    it("Panel: unfolding restores the stored path, and re-pressing a nested active leaf deselects instead of folding", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [
        {
          kind: "branch",
          id: "root-a",
          icon: StubIcon,
          name: "Root A",
          children: [singleTreeLeaf({ id: "leaf-a", icon: StubIcon, name: "Leaf A", tree: { sections: [{ id: "sec", label: "Section", defaultOpen: true, items: [{ id: "row", label: "Row" }] }] } })],
        },
      ];
      const onVisibleChange = vi.fn();
      let path: readonly string[] = ["root-a", "leaf-a"];
      const onActiveTabPathChange = vi.fn((next: readonly string[]) => {
        path = next;
      });
      const { container, rerender } = render(<Panel anchor="top-left" visible={false} onVisibleChange={onVisibleChange} tabs={tabs} activeTabPath={path} onActiveTabPathChange={onActiveTabPathChange} />);
      fireEvent.click(container.querySelector('button[id="root-a"]') as HTMLElement);
      expect(onVisibleChange).toHaveBeenCalledWith(true);
      expect(onActiveTabPathChange).not.toHaveBeenCalled();

      onVisibleChange.mockClear();
      rerender(<Panel anchor="top-left" visible onVisibleChange={onVisibleChange} tabs={tabs} activeTabPath={path} onActiveTabPathChange={onActiveTabPathChange} />);
      expect(screen.getByText("Row")).toBeTruthy();
      fireEvent.click(container.querySelector('button[id="leaf-a"]') as HTMLElement);
      expect(path).toEqual(["root-a"]);
      expect(onVisibleChange).not.toHaveBeenCalled();
    });

    it("Panel forwards drill-down memory through pathMemory/onPathMemoryChange when controlled", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [{ kind: "branch", id: "root-a", icon: StubIcon, name: "Root A", children: [singleTreeLeaf({ id: "leaf-a", icon: StubIcon, name: "Leaf A", tree: { sections: [] } })] }];
      let memory: Readonly<Record<string, string>> = {};
      const onPathMemoryChange = vi.fn((next: Readonly<Record<string, string>>) => {
        memory = next;
      });
      const { container } = render(<Panel anchor="top-left" visible tabs={tabs} pathMemory={memory} onPathMemoryChange={onPathMemoryChange} />);
      fireEvent.click(container.querySelector('button[id="root-a"]') as HTMLElement);
      fireEvent.click(container.querySelector('button[id="leaf-a"]') as HTMLElement);
      expect(memory).toEqual({ "root-a": "leaf-a" });
    });

    it("Tree's controlled openStates/onOpenStateChange survives what would otherwise be a fresh remount", () => {
      const sections: TreeDataSection[] = [{ id: "sec", label: "Section", defaultOpen: false, items: [{ id: "item", label: "Item" }] }];
      let openStates: Readonly<Record<string, boolean>> = {};
      const onOpenStateChange = vi.fn((id: string, open: boolean) => {
        openStates = { ...openStates, [id]: open };
      });
      const { container, unmount } = render(<Tree sections={sections} openStates={openStates} onOpenStateChange={onOpenStateChange} />);
      fireEvent.click(container.querySelector('[data-slot="tree-section-row"]') as HTMLElement);
      expect(openStates["tree-section-sec"]).toBe(true);
      unmount();
      const { container: remounted } = render(<Tree sections={sections} openStates={openStates} onOpenStateChange={onOpenStateChange} />);
      expect(remounted.querySelector('[data-slot="tree-section-row"]')?.getAttribute("data-state")).toBe("open");
    });

    it("PanelTreeUnitsPane namespaces controlled tree open-state per unit so keys never collide across units", () => {
      const StubIcon = (): null => null;
      const units: PanelTreeUnit[] = [
        { id: "unit-a", tree: { sections: [{ id: "sec", label: "Section A", defaultOpen: false, items: [{ id: "item", label: "Item A" }] }] } },
        { id: "unit-b", tree: { sections: [{ id: "sec", label: "Section B", defaultOpen: false, items: [{ id: "item", label: "Item B" }] }] } },
      ];
      const tabs: PanelTabNode[] = [{ kind: "leaf", id: "tab-a", icon: StubIcon, name: "Tab A", trees: units }];
      let treeOpenStates: Readonly<Record<string, boolean>> = {};
      const onTreeOpenStateChange = vi.fn((id: string, open: boolean) => {
        treeOpenStates = { ...treeOpenStates, [id]: open };
      });
      const { container } = render(<Panel anchor="top-left" visible tabs={tabs} activeTabPath={["tab-a"]} treeOpenStates={treeOpenStates} onTreeOpenStateChange={onTreeOpenStateChange} />);
      const rows = container.querySelectorAll('[data-slot="tree-section-row"]');
      fireEvent.click(rows[0]);
      expect(treeOpenStates).toEqual({ "unit-a:tree-section-sec": true });
      expect(treeOpenStates["unit-b:tree-section-sec"]).toBeUndefined();
    });

    it("PanelTreeUnitsPane re-resolves lazy trees when treeContentRevision changes", () => {
      const StubIcon = (): null => null;
      let sectionLabel = "before";
      const resolveTree = vi.fn(() => ({
        sections: [{ id: "sec", label: sectionLabel, items: [{ id: "item", label: "Item" }] }],
        sortableSections: false as const,
      }));
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tool.fill", icon: StubIcon, name: "Fill", tree: { resolveTree } })];
      const { container, rerender } = render(<Panel anchor="bottom-middle" visible tabs={tabs} activeTabPath={["tool.fill"]} treeContentRevision={0} />);
      expect(container.textContent).toContain("before");
      expect(resolveTree).toHaveBeenCalledTimes(1);
      sectionLabel = "after";
      rerender(<Panel anchor="bottom-middle" visible tabs={tabs} activeTabPath={["tool.fill"]} treeContentRevision={0} />);
      expect(resolveTree).toHaveBeenCalledTimes(1);
      expect(container.textContent).toContain("before");
      rerender(<Panel anchor="bottom-middle" visible tabs={tabs} activeTabPath={["tool.fill"]} treeContentRevision={1} />);
      expect(resolveTree).toHaveBeenCalledTimes(2);
      expect(container.textContent).toContain("after");
    });

    it("PanelTreeUnitsPane omits unlabeled single-unit headers under PanelDockProvider (no lonely top grip)", () => {
      const StubIcon = (): null => null;
      const emptyDock: PanelDock = {
        anchors: {
          "top-left": [],
          "top-middle": [],
          "top-right": [],
          "right-middle": [],
          "bottom-right": [],
          "bottom-middle": [],
          "bottom-left": [],
          "left-middle": [],
        },
      };
      const tabs: PanelTabNode[] = [
        singleTreeLeaf({
          id: "framework.panel.artifact",
          icon: StubIcon,
          name: "Artifact",
          tree: {
            sections: [
              { id: "objects", label: "Building Components", defaultOpen: false, items: [] },
              { id: "references", label: "References", defaultOpen: false, items: [] },
            ],
          },
        }),
      ];
      const { container } = render(
        <PanelDockProvider dock={emptyDock} onTabDockDrop={() => undefined} onTreeUnitDockDrop={() => undefined}>
          <Panel anchor="top-left" visible tabs={tabs} activeTabPath={["framework.panel.artifact"]} />
        </PanelDockProvider>,
      );
      expect(container.querySelector('[data-slot="panel-tree-unit-header"]')).toBeNull();
      const sectionHandles = container.querySelectorAll('[data-slot="tree-section-row"] [data-slot="drag-handle"]');
      expect(sectionHandles.length).toBe(2);
    });

    it("Tree section reorder grips fire onSectionsReorder with the new id order", () => {
      const onSectionsReorder = vi.fn();
      const sections: TreeDataSection[] = [
        { id: "a", label: "A", defaultOpen: false, items: [] },
        { id: "b", label: "B", defaultOpen: false, items: [] },
        { id: "c", label: "C", defaultOpen: false, items: [] },
      ];
      const { container } = render(<Tree sections={sections} sortableSections onSectionsReorder={onSectionsReorder} />);
      const rows = container.querySelectorAll('[data-slot="tree-section-row"]');
      expect(rows.length).toBe(3);
      expect(container.querySelectorAll('[data-slot="tree-section-row"] [data-slot="drag-handle"]').length).toBe(3);
      const source = rows[0] as HTMLElement;
      const target = rows[2] as HTMLElement;
      const dataTransfer = {
        effectAllowed: "none",
        dropEffect: "none",
        types: [TREE_SECTION_REORDER_MIME],
        setData: vi.fn(),
        getData: vi.fn(() => "a"),
      };
      fireEvent.dragStart(source, { dataTransfer });
      fireEvent.dragOver(target, { dataTransfer });
      fireEvent.drop(target, { dataTransfer });
      expect(onSectionsReorder).toHaveBeenCalledWith(["b", "c", "a"]);
    });

    it("mergeTreeSectionOrder keeps remembered order and inserts new sections where the source puts them", () => {
      const sections: TreeDataSection[] = [
        { id: "a", label: "A" },
        { id: "b", label: "B" },
        { id: "c", label: "C" },
      ];
      expect(mergeTreeSectionOrder(["c", "a"], sections).map((section) => section.id)).toEqual(["c", "a", "b"]);
      const history = ["band", "editor", "inputs", "alternatives", "actions", "commands"].map((id): TreeDataSection => ({ id, label: id }));
      expect(mergeTreeSectionOrder(["actions", "commands"], history).map((section) => section.id)).toEqual(["band", "editor", "inputs", "alternatives", "actions", "commands"]);
      expect(mergeTreeSectionOrder(["commands", "actions"], history.slice(0, 1).concat(history.slice(4))).map((section) => section.id)).toEqual(["band", "commands", "actions"]);
      expect(mergeTreeSectionOrder(["band", "editor", "actions", "commands"], history.slice(4)).map((section) => section.id)).toEqual(["actions", "commands"]);
    });

    it("stretches every data-tree wrapper and nested row to the full host width", () => {
      const { container } = render(
        <Tree
          sections={[
            {
              id: "section",
              label: "Section",
              defaultOpen: true,
              items: [
                {
                  id: "group",
                  label: "Group",
                  defaultOpen: true,
                  items: [{ id: "leaf", label: "Leaf" }],
                },
              ],
            },
          ]}
        />,
      );
      const sectionWrapper = container.querySelector<HTMLElement>('[data-slot="tree-section-wrapper"]');
      const sectionBranch = container.querySelector<HTMLElement>('[data-slot="tree-section-content"]');
      const itemBranch = container.querySelector<HTMLElement>('[data-slot="tree-item-content"]');
      const rows = container.querySelectorAll<HTMLElement>('[data-slot="tree-section-row"], [data-slot="tree-item-row"]');

      expect(sectionWrapper?.classList.contains("w-full")).toBe(true);
      expect(sectionBranch?.classList.contains("w-full")).toBe(true);
      expect(sectionBranch?.parentElement?.classList.contains("w-full")).toBe(true);
      expect(itemBranch?.classList.contains("w-full")).toBe(true);
      expect(rows.length).toBe(3);
      expect(Array.from(rows).every((row) => row.classList.contains("w-full"))).toBe(true);
    });

    it("sortable TreeItem always renders a drag handle", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <SortableTreeItems items={[{ id: "row-1" }]} onReorder={() => undefined}>
            {(item) => <TreeItem id={item.id} label="Row" sortable sortableId={item.id} />}
          </SortableTreeItems>
        </TreeContext.Provider>,
      );
      expect(markup).toContain('data-slot="drag-handle"');
    });

    it("Panel wires bottom corners' trees to direction=up (content above header) while top corners stay direction=down", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [
        singleTreeLeaf({
          id: "tab-a",
          icon: StubIcon,
          name: "Tab A",
          tree: {
            sections: [
              { id: "sec-a", label: "Section A", defaultOpen: true, items: [{ id: "item-a", label: "Item A" }] },
              { id: "sec-b", label: "Section B", defaultOpen: true, items: [{ id: "item-b", label: "Item B" }] },
            ],
          },
        }),
      ];
      const { container: topContainer } = render(<Panel anchor="top-left" visible tabs={tabs} activeTabPath={["tab-a"]} />);
      const topMarkup = topContainer.querySelector('[data-slot="panel-content"]')!.innerHTML;
      expect(topMarkup.indexOf("Section A")).toBeLessThan(topMarkup.indexOf("Section B"));
      expect(topContainer.querySelector('[data-slot="scroll-area-viewport"]')?.classList.contains("justify-end")).toBe(false);

      const { container: bottomContainer } = render(<Panel anchor="bottom-left" visible tabs={tabs} activeTabPath={["tab-a"]} />);
      const bottomMarkup = bottomContainer.querySelector('[data-slot="panel-content"]')!.innerHTML;
      expect(bottomMarkup.indexOf("Section B")).toBeLessThan(bottomMarkup.indexOf("Section A"));
      const bottomTree = bottomContainer.querySelector('[data-slot="tree"]');
      const bottomViewport = bottomContainer.querySelector('[data-slot="scroll-area-viewport"]');
      expect(bottomTree?.getAttribute("dir")).toBe("auto");
      expect(bottomTree?.getAttribute("role")).toBe("tree");
      expect(bottomTree?.classList.contains("overflow-y-auto")).toBe(false);
      expect(bottomViewport?.classList.contains("min-h-full")).toBe(true);
      expect(bottomViewport?.classList.contains("flex-col")).toBe(true);
      expect(bottomViewport?.classList.contains("justify-end")).toBe(true);
    });

    it("keeps right-anchored panel tree rows left-to-right inside mirrored panel chrome", () => {
      const StubIcon = (): null => null;
      const tab = singleTreeLeaf({
        id: "history",
        icon: StubIcon,
        name: "History",
        tree: {
          sections: [
            {
              id: "commands",
              label: "Commands",
              defaultOpen: true,
              items: [{ id: "group", label: "Toggle Panel", defaultOpen: false, items: [{ id: "child", label: "Child" }] }],
            },
          ],
        },
      });
      const { container } = render(<Panel anchor="bottom-right" visible tabs={[tab]} activeTabPath={[tab.id]} />);
      expect(container.querySelector('[data-slot="panel"]')?.getAttribute("dir")).toBe("rtl");
      expect(container.querySelector('[data-slot="tree"]')?.getAttribute("dir")).toBe("auto");
      expect(container.querySelector('[data-slot="tree-item-row"]')?.closest('[data-slot="tree"]')?.getAttribute("dir")).toBe("auto");
      const closedChevron = container.querySelector('[data-slot="tree-item-row"] [data-icon]')?.getAttribute("data-icon");
      expect(closedChevron).toBe("chevron-left");
    });

    it("FlowProvider defaults to ltr/down and lets nested providers override only what they pass", () => {
      const Probe: React.FC = () => {
        const flow = useFlow();
        return (
          <span>
            {flow.inline}/{flow.block}
          </span>
        );
      };
      expect(renderToStaticMarkup(<Probe />)).toContain("ltr/down");
      expect(
        renderToStaticMarkup(
          <FlowProvider inline="rtl" block="up">
            <Probe />
          </FlowProvider>,
        ),
      ).toContain("rtl/up");
      expect(
        renderToStaticMarkup(
          <FlowProvider inline="rtl" block="up">
            <FlowProvider block="down">
              <Probe />
            </FlowProvider>
          </FlowProvider>,
        ),
      ).toContain("rtl/down");
    });

    it("Panel sets dir=rtl only for right anchors (corners and right-middle), never for left or middle anchors", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } })];
      const dirOf = (anchor: Anchor) =>
        render(<Panel anchor={anchor} visible tabs={tabs} />)
          .container.querySelector('[data-slot="panel"]')
          ?.getAttribute("dir");
      expect(dirOf("top-left")).toBeNull();
      expect(dirOf("bottom-left")).toBeNull();
      expect(dirOf("left-middle")).toBeNull();
      expect(dirOf("top-middle")).toBeNull();
      expect(dirOf("bottom-middle")).toBeNull();
      expect(dirOf("top-right")).toBe("rtl");
      expect(dirOf("bottom-right")).toBe("rtl");
      expect(dirOf("right-middle")).toBe("rtl");
    });
});
}
