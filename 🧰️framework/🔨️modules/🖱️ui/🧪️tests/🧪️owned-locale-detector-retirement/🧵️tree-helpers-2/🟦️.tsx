import type { TreeDataSection } from "@semio-tech/ui-react/source";
import * as ui from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests2Vitest, Tests2Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerTreeHelpers2Tests(vitest: Tests2Vitest, dependencies: Tests2Dependencies, testSource: TestSource): Promise<void> {
  const { createTreeSelectionStore, getElementById, getTreeItemOrderedIds, getTreeNextSelectionState, interactionMergeFromModifiers, Label, normalizeTreeSelectedIds, renderToStaticMarkup, Slider, Stepper, Textarea, Tree, TreeContent, TreeContext, TreeItem, treeItemSecondaryTextClassName, TreeRow, TreeSection, TreeStateProvider } = dependencies;
  const { describe, expect, it } = vitest;

describe("tree helpers", () => {

  
      it("renders selected tree item rows with emphasized text instead of hover gray", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeItem id="tooltip.alpha" label="Alpha" isSelected />
          </TreeContext.Provider>,
        );
        expect(markup).toContain("bg-active-base");
        expect(markup).toContain("text-emphasized");
        expect(markup).toContain("text-active-foreground");
        expect(markup).not.toMatch(/data-slot="tree-label"[^>]*text-element/);
      });
  
      it("tree selection store notifies subscribers only when selection changes", () => {
        const store = createTreeSelectionStore();
        let calls = 0;
        const unsub = store.subscribe(() => {
          calls++;
        });
        store.setSelectedIds(["a"]);
        expect(calls).toBe(1);
        expect(store.isSelected("a")).toBe(true);
        store.setSelectedIds(["a"]);
        expect(calls).toBe(1);
        store.setSelectedIds([]);
        expect(calls).toBe(2);
        expect(store.isSelected("a")).toBe(false);
        unsub();
      });
  
      it("computes additive and range multi selection", () => {
        expect(
          getTreeNextSelectionState({
            selectionMode: "multiple",
            selectedIds: ["a"],
            orderedIds: ["a", "b", "c", "d"],
            targetId: "c",
            anchorId: "a",
            additiveKey: false,
            rangeKey: true,
          }),
        ).toEqual({ selectedIds: ["a", "b", "c"], anchorId: "a" });
  
        expect(
          getTreeNextSelectionState({
            selectionMode: "multiple",
            selectedIds: ["a"],
            orderedIds: ["a", "b", "c", "d"],
            targetId: "c",
            anchorId: "a",
            additiveKey: true,
            rangeKey: false,
          }),
        ).toEqual({ selectedIds: ["a", "c"], anchorId: "c" });
      });
  
      // 🕹️wave-2b: `getTreeNextSelectionState`/`normalizeTreeSelectedIds` now delegate to `🕹️interaction`'s
      // `nextSelection`/`validateState` (wave 0) — these extend the same two `it` blocks above rather than
      // adding new files, asserting the delegation preserves exact prior behavior for single mode (LAST
      // target wins, ignores modifiers), additive toggle-off, and `normalizeTreeSelectedIds`'s FIRST-id
      // clamp for externally-supplied ids (the two deliberately different clamps — see the functions' docs).
      it("single-selection-mode picks ignore additive/range keys, keeping the LAST target (nextSelection delegation)", () => {
        expect(
          getTreeNextSelectionState({
            selectionMode: "single",
            selectedIds: ["a"],
            orderedIds: ["a", "b", "c", "d"],
            targetId: "c",
            anchorId: "a",
            additiveKey: true,
            rangeKey: true,
          }),
        ).toEqual({ selectedIds: ["c"], anchorId: "c" });
      });
  
      it("additive key toggles an already-selected id back off (nextSelection invertive delegation)", () => {
        expect(
          getTreeNextSelectionState({
            selectionMode: "multiple",
            selectedIds: ["a", "c"],
            orderedIds: ["a", "b", "c", "d"],
            targetId: "c",
            anchorId: "c",
            additiveKey: true,
            rangeKey: false,
          }),
        ).toEqual({ selectedIds: ["a"], anchorId: "c" });
      });
  
      it("normalizeTreeSelectedIds clamps to the FIRST id in single mode (validateState delegation, distinct from the picks above's LAST-target clamp)", () => {
        expect(normalizeTreeSelectedIds(["c", "a", "c", "", "b"], "single")).toEqual(["c"]);
        expect(normalizeTreeSelectedIds(["c", "a", "c", "", "b"], "multiple")).toEqual(["c", "a", "b"]);
      });
  
      it("interactionMergeFromModifiers: shift wins Range even with ctrl/meta also held; alt is Subtractive; no modifier is Replace", () => {
        expect(interactionMergeFromModifiers({})).toBe("replace");
        expect(interactionMergeFromModifiers({ shiftKey: true })).toBe("range");
        expect(interactionMergeFromModifiers({ ctrlKey: true })).toBe("invertive");
        expect(interactionMergeFromModifiers({ metaKey: true })).toBe("invertive");
        expect(interactionMergeFromModifiers({ altKey: true })).toBe("subtractive");
        expect(interactionMergeFromModifiers({ shiftKey: true, ctrlKey: true })).toBe("range");
        expect(interactionMergeFromModifiers({ ctrlKey: true, altKey: true })).toBe("invertive");
      });
  
      it("orders nested tree items across sections", () => {
        const sections: TreeDataSection[] = [
          {
            id: "section-a",
            label: "Section A",
            items: [
              { id: "item-a", label: "Item A", items: [{ id: "item-a-1", label: "Item A1" }] },
              { id: "item-b", label: "Item B" },
            ],
          },
          {
            id: "section-b",
            label: "Section B",
            items: [{ id: "item-c", label: "Item C" }],
          },
        ];
  
        expect(getTreeItemOrderedIds(sections, {}, {})).toEqual(["item-a", "item-a-1", "item-b", "item-c"]);
      });
  
      it("does not render an extra placeholder gap for tree property labels", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 1, isLastAtLevel: [true], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <Label id="tooltip.manual">
              <span>Control</span>
            </Label>
          </TreeContext.Provider>,
        );
  
        expect(markup).toContain('data-slot="property-label-tree"');
        expect(markup).toContain('data-slot="tree-row-layout"');
        expect(markup).toContain('data-slot="tree-gutter"');
        expect(markup).toContain("grid-template-columns:calc(");
        expect(markup).toContain("minmax(0, 1fr)");
        expect(markup).toContain('data-slot="property-label-tree" class="min-w-0"');
        expect(markup).toContain('data-slot="property-row"');
        expect(markup).toContain("margin-inline-start:calc(-1 *");
        expect(markup).toContain("var(--layout-label)");
        expect(markup).toContain('data-slot="property-control"');
        expect(markup).toContain("justify-end");
        expect(markup).toContain("self-start");
        expect(markup).toContain("data-detail-panel-control");
        expect(markup).toContain("var(--spacing-double)");
        expect(markup).not.toContain("margin-left:13px");
        expect(markup).toContain('data-slot="tree-branch-elbow"');
        expect(markup).toContain("var(--tree-gutter-center, calc(var(--tree-row-height) / 2))");
        expect(markup).not.toContain('style="top:50%;left:7px;width:10px"');
      });
  
      it("renders explicit property labels on the shared property-row wrapper", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 1, isLastAtLevel: [true], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <Label id="tooltip.manual" rowId="custom-row" label="piece">
              <span>Control</span>
            </Label>
          </TreeContext.Provider>,
        );
  
        expect(markup).toContain('id="custom-row"');
        expect(markup).toContain('data-slot="property-control"');
        expect(markup).toContain('data-slot="tree-row-layout"');
        expect(markup).toContain('data-slot="property-control"');
        expect(markup).toContain("justify-end");
        expect(markup).toContain("self-start");
        expect(markup).toContain("data-detail-panel-control");
        expect(markup).toContain(">piece<");
      });
  
      it("anchors TreeRow property-control children to the fixed header line", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 1, isLastAtLevel: [true], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeRow>
              <Textarea id="tooltip.manual" value="Long value" showLabel />
            </TreeRow>
          </TreeContext.Provider>,
        );
  
        expect(markup).toContain('data-slot="tree-row"');
        expect(markup).toContain('data-tree-row-kind="property"');
        expect(markup).toContain('data-slot="property-row"');
        expect(markup).toContain("var(--tree-gutter-center, calc(var(--tree-row-height) / 2))");
        expect(markup).not.toContain('style="top:50%;left:7px;width:10px"');
      });
  
      it("marks unlabeled non-property TreeRow wrappers as content rows", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 1, isLastAtLevel: [true], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeRow>
              <span>Note</span>
            </TreeRow>
          </TreeContext.Provider>,
        );
  
        expect(markup).toContain('data-slot="tree-row"');
        expect(markup).toContain('data-tree-row-kind="content"');
      });
  
      it("renders Tree data rows with inline controls in the same row as the label", () => {
        const markup = renderToStaticMarkup(
          <TreeStateProvider>
            <Tree
              sections={[
                {
                  id: "inspector.objects",
                  label: "Objects (1)",
                  defaultOpen: true,
                  items: [
                    {
                      id: "inspector.object.id",
                      label: "Id",
                      control: <input id="inspector.object.id.input" value="seed-left-001" readOnly />,
                    },
                  ],
                },
              ]}
            />
          </TreeStateProvider>,
        );
  
        expect(markup).toContain('data-slot="tree-item-control"');
        expect(markup).not.toContain('data-slot="tree-property-content"');
        expect(markup).toContain('id="inspector.object.id.input"');
        expect(markup).toContain("seed-left-001");
      });
  
      it("gives tree property sliders enough width for the track, not just the thumb", () => {
        const markup = renderToStaticMarkup(
          <TreeStateProvider>
            <Tree
              sections={[
                {
                  id: "tool.fill.options",
                  label: "",
                  defaultOpen: true,
                  items: [{ id: "puzzle3d-fill-count", label: "Count", control: <Slider id="puzzle3d-fill-count" value={[3]} min={0} max={100} /> }],
                },
              ]}
            />
          </TreeStateProvider>,
        );
  
        expect(markup).toContain('data-slot="tree-item-control"');
        expect(markup).toContain('data-slot="slider-track"');
        expect(markup).toContain("grid-template-columns:minmax(0, 1fr) var(--tree-value-column, calc(50 * var(--ui-spacing)))");
      });
  
      it("keeps leaf inspector controls visible without manual expand", () => {
        const markup = renderToStaticMarkup(
          <TreeStateProvider>
            <Tree
              sections={[
                {
                  id: "procedural-play-inspector.widget",
                  label: "Widget",
                  defaultOpen: true,
                  items: [
                    {
                      id: "procedural-play-inspector.value",
                      label: "Value",
                      control: <input id="procedural-play-inspector.value.input" defaultValue="2.2" readOnly />,
                    },
                  ],
                },
              ]}
            />
          </TreeStateProvider>,
        );
        expect(markup).toContain('id="procedural-play-inspector.value.input"');
        expect(markup).toContain("2.2");
      });
  
      it("renders property-layout tree items with a dedicated control column", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeItem id="tooltip.manual" layoutKind="property" defaultOpen={true}>
              <Label id="tooltip.manual">
                <span>Control</span>
              </Label>
            </TreeItem>
          </TreeContext.Provider>,
        );
  
        expect(markup).toContain('data-slot="tree-property-item"');
        expect(markup).toContain('data-slot="tree-row-content"');
        expect(markup).toContain('data-slot="tree-item-control"');
        expect(markup).toContain('data-slot="tree-row-layout"');
        expect(markup).toContain('data-slot="tree-gutter"');
        expect(markup).toContain("grid-template-columns:calc(");
        expect(markup).toContain("var(--spacing-double)");
        expect(markup).toContain('data-slot="tree-property-content"');
        expect(markup).not.toContain('data-slot="tree-header-actions"');
        expect(markup).toContain('data-slot="property-row"');
        expect(markup.indexOf('data-slot="tree-property-item"')).toBeLessThan(markup.indexOf('data-slot="tree-property-content"'));
        expect(markup).toMatch(/data-slot="tree-property-item"[\s\S]*?<\/div><div[^>]*data-slot="tree-property-content"/);
      });
  
      it("keeps nested property-layout children outside the parent hover group so sibling rows do not share group-hover fill", () => {
        const markup = renderToStaticMarkup(
          <TreeStateProvider>
            <Tree
              sections={[
                {
                  id: "tool.fill.options",
                  label: "",
                  defaultOpen: true,
                  items: [
                    {
                      id: "puzzle3d-fill-object-capsule",
                      label: "Capsule",
                      defaultOpen: true,
                      control: <Slider id="puzzle3d-fill-object-capsule-slider" value={[0.5]} min={0} max={1} step={0.01} />,
                      items: [
                        { id: "puzzle3d-fill-vortex-a", label: "Joint A", control: <Slider id="puzzle3d-fill-vortex-a-slider" value={[0.25]} min={0} max={0.5} step={0.01} /> },
                        { id: "puzzle3d-fill-vortex-b", label: "Joint B", control: <Slider id="puzzle3d-fill-vortex-b-slider" value={[0.25]} min={0} max={0.5} step={0.01} /> },
                      ],
                    },
                  ],
                },
              ]}
            />
          </TreeStateProvider>,
        );
  
        document.body.innerHTML = markup;
        const parentRow = document.getElementById("puzzle3d-fill-object-capsule");
        const branch = document.querySelector('[data-slot="tree-property-content"]');
        const childA = document.getElementById("puzzle3d-fill-vortex-a");
        const childB = document.getElementById("puzzle3d-fill-vortex-b");
        expect(parentRow?.getAttribute("data-slot")).toBe("tree-property-item");
        expect(parentRow?.className).toContain("group/tree-row");
        expect(branch).not.toBeNull();
        expect(parentRow?.contains(branch)).toBe(false);
        expect(branch?.contains(childA)).toBe(true);
        expect(branch?.contains(childB)).toBe(true);
        expect(childA?.closest('[data-slot="tree-property-item"]')?.className).toContain("group/tree-row");
        expect(childA?.closest('[data-slot="tree-property-item"]')).not.toBe(parentRow);
        expect(childB?.closest('[data-slot="tree-property-item"]')).not.toBe(parentRow);
        expect(childA?.closest('[data-slot="tree-property-item"]')).not.toBe(childB?.closest('[data-slot="tree-property-item"]'));
      });
  
      it("keeps leaf and expandable sibling rows on the same gutter rhythm", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeSection id="tooltip.manual" defaultOpen={true}>
              <TreeItem id="tooltip.tutorial" defaultOpen={true}>
                <TreeContent>
                  <span>Nested content</span>
                </TreeContent>
              </TreeItem>
              <TreeItem id="tooltip.docs" />
            </TreeSection>
          </TreeContext.Provider>,
        );
  
        expect(markup.match(/grid-template-columns:calc\(/g)?.length ?? 0).toBeGreaterThanOrEqual(1);
        expect(markup).not.toContain("margin-left:-10px");
        expect(markup).not.toContain("padding-left:10px");
      });
  
      it("renders default icons before tree section and item labels when icon is omitted", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeSection id="tooltip.manual" label="Section" defaultOpen={true}>
              <TreeItem id="tooltip.tutorial" label="Folder" defaultOpen>
                <TreeItem id="tooltip.docs" label="Leaf" />
              </TreeItem>
            </TreeSection>
          </TreeContext.Provider>,
        );
  
        expect(markup.match(/data-slot="tree-icon"/g)?.length ?? 0).toBe(3);
        expect(markup).toContain('data-slot="tree-section-row"');
        expect(markup).toContain('data-tree-row-kind="group"');
        expect(markup).toContain('data-tree-row-kind="leaf"');
      });
  
      it("renders unlabeled data-tree sections without a folder header row", () => {
        const markup = renderToStaticMarkup(
          <Tree
            sections={[
              {
                id: "command.category.general.form",
                items: [{ id: "command.os.setDriver.arg.driver", label: "Driver", control: <span>compact</span> }],
              },
            ]}
          />,
        );
  
        expect(markup).not.toContain('data-slot="tree-section-row"');
        expect(markup).toContain("Driver");
      });
  
      it("renders tree item descriptions inline on one row with muted secondary text", () => {
        const markup = renderToStaticMarkup(
          <Tree
            sections={[
              {
                id: "tree.inline-description",
                items: [{ id: "tree.inline-description.item", label: "Capsule J", description: "capsule-j" }],
              },
            ]}
          />,
        );
  
        expect(markup).not.toContain("flex-col gap-0.5");
        expect(markup).not.toContain(" - ");
        expect(markup).toContain(treeItemSecondaryTextClassName);
        expect(markup).toContain("Capsule J");
        expect(markup).toContain("capsule-j");
        expect(markup).toContain('data-slot="tree-item-row"');
        expect(markup).toContain("h-[var(--tree-row-height,var(--size-workbench))]");
        expect(markup).toContain('data-slot="tree-row-layout"');
        expect(markup).toMatch(/data-slot="tree-row-layout"[^>]*class="[^"]*\bh-full\b[^"]*\bw-full\b/);
        expect(markup).toMatch(/data-slot="tree-row-content"[^>]*class="[^"]*\bh-full\b[^"]*\bflex\b[^"]*\bitems-center\b/);
      });
  
      it("renders parent-level vertical guides through expanded last-sibling branches", () => {
        const markup = renderToStaticMarkup(
          <TreeStateProvider>
            <Tree
              sections={[
                {
                  id: "objects",
                  label: "Objects",
                  defaultOpen: true,
                  items: [
                    {
                      id: "object-only",
                      label: "Hexagonal Cut Concrete",
                      defaultOpen: true,
                      items: [
                        { id: "vortex-a", label: "a" },
                        { id: "vortex-b", label: "b" },
                      ],
                    },
                  ],
                },
                {
                  id: "references",
                  label: "References",
                  defaultOpen: false,
                  items: [],
                },
              ]}
            />
          </TreeStateProvider>,
        );
  
        const objectBranch = markup.split('data-slot="tree-item-content"')[1]?.slice(0, 1_500) ?? "";
        expect(objectBranch.match(/data-tree-guide-line/g)?.length ?? 0).toBe(2);
        expect(objectBranch.match(/inset-inline-start:/g)?.length ?? 0).toBeGreaterThanOrEqual(2);
        expect(objectBranch).not.toMatch(/(?:^|;)left:/);
      });
  
      it("renders steppers at full control width with the current numeric value visible", () => {
        const markup = renderToStaticMarkup(<Stepper id="ui.stepper.demo" value={12.5} />);
  
        expect(markup).toContain('data-slot="stepper-group"');
        expect(markup).toContain('data-detail-panel-control="fill"');
        expect(markup).toContain('data-stepper-input="true"');
        expect(markup).toContain("w-full");
        expect(markup).toContain("min-w-0");
        expect(markup).toContain('value="12.5"');
      });
});
}
