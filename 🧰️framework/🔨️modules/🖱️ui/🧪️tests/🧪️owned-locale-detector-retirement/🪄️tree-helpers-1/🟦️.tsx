import type { PanelTabNode } from "@semio-tech/ui-react/source";
import * as ui from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests2Vitest, Tests2Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerTreeHelpers1Tests(vitest: Tests2Vitest, dependencies: Tests2Dependencies, testSource: TestSource): Promise<void> {
  const { catalogueTreeDragController, ChevronDownIcon, ChevronLeftIcon, ChevronRightIcon, ChevronUpIcon, Command, createTreeHighlightStore, flowChevronIconName, formatControlTooltipText, formatKeybindingShortcut, getElementById, getTreeSiblingGapPx, GhostProvider, GhostRegionShell, isTreeReorderDragEvent, loadingBorderActiveClass, loadingBorderClass, loadingBorderStateClass, markGhostTreeInteraction, normalizeTreeSelectedIds, Pane, PaneHost, Panel, PanelChromeTabBar, React, renderToStaticMarkup, resolveTreeDropPosition, shouldBeginAutomaticGhostInteraction, shouldDispatchTreeRowPointerLeave, singleTreeLeaf, syncTreeSelectionPath, treeCompactSiblingGapPx, TreeContext, treeFoldChevronIcon, TreeItem, treeRowChromeClasses, treeRowChromeContentFillClasses, treeRowChromeShellClasses, TreeSection, uiDataLabel, waitingBorderActiveClass, waitingBorderClass, waitingBorderStateClass, Window, WindowChrome } = dependencies;
  const { describe, expect, it, vi } = vitest;

describe("tree helpers", () => {

      it("resolves tree drop positions from pointer location", () => {
        const event = {
          clientY: 10,
          currentTarget: { getBoundingClientRect: () => ({ top: 0, height: 100 }) },
        } as unknown as React.DragEvent<HTMLElement>;
        expect(resolveTreeDropPosition(event)).toBe("before");
      });
  
      it("detects internal tree reorder drags without palette mime", () => {
        const reorder = { dataTransfer: { types: ["application/vnd.code.tree.item"] } } as unknown as React.DragEvent;
        const palette = { dataTransfer: { types: ["application/vnd.code.tree.item", "application/x-semio-forms-question"] } } as unknown as React.DragEvent;
        expect(isTreeReorderDragEvent(reorder)).toBe(true);
        expect(isTreeReorderDragEvent(palette)).toBe(false);
      });
  
      it("builds catalogue drag payloads", () => {
        const controller = catalogueTreeDragController("application/x-test");
        const payload = controller.pointerPaletteDrag?.readEncodedDragPayload({ "application/x-test": '{"kind":"text"}' });
        expect(payload).toBe('{"kind":"text"}');
      });
  
      it("uses a single compact sibling gap for every row-kind transition", () => {
        expect(getTreeSiblingGapPx("leaf", "group")).toBe(treeCompactSiblingGapPx);
        expect(getTreeSiblingGapPx("property", "group")).toBe(treeCompactSiblingGapPx);
        expect(getTreeSiblingGapPx("property", "property")).toBe(treeCompactSiblingGapPx);
        expect(getTreeSiblingGapPx("group", "group")).toBe(treeCompactSiblingGapPx);
        expect(getTreeSiblingGapPx("content", "group")).toBe(treeCompactSiblingGapPx);
      });
  
      it("treeFoldChevronIcon mirrors the closed-state chevron for rtl and keeps the open-state chevron on the block axis", () => {
        expect(treeFoldChevronIcon("down", "ltr", false)).toBe(ChevronRightIcon);
        expect(treeFoldChevronIcon("down", "rtl", false)).toBe(ChevronLeftIcon);
        expect(treeFoldChevronIcon("up", "ltr", false)).toBe(ChevronLeftIcon);
        expect(treeFoldChevronIcon("up", "rtl", false)).toBe(ChevronRightIcon);
        expect(treeFoldChevronIcon("down", "ltr", true)).toBe(ChevronDownIcon);
        expect(treeFoldChevronIcon("down", "rtl", true)).toBe(ChevronDownIcon);
        expect(treeFoldChevronIcon("up", "ltr", true)).toBe(ChevronUpIcon);
        expect(treeFoldChevronIcon("up", "rtl", true)).toBe(ChevronUpIcon);
      });
  
      it("flowChevronIconName mirrors a fold affordance's chevron for rtl", () => {
        expect(flowChevronIconName("ltr", true)).toBe("chevron-left");
        expect(flowChevronIconName("ltr", false)).toBe("chevron-right");
        expect(flowChevronIconName("rtl", true)).toBe("chevron-right");
        expect(flowChevronIconName("rtl", false)).toBe("chevron-left");
      });
  
      it("normalizes selected ids for single and multiple selection", () => {
        expect(normalizeTreeSelectedIds(["a", "a", "b"], "single")).toEqual(["a"]);
        expect(normalizeTreeSelectedIds(["a", "a", "b"], "multiple")).toEqual(["a", "b"]);
      });
  
      it("formats keybinding shortcuts for display", () => {
        expect(formatKeybindingShortcut("mod+p")).toMatch(/p/i);
        expect(formatKeybindingShortcut("escape")).toBeTruthy();
      });
  
      it("formats control tooltip text with optional hotkeys", () => {
        expect(formatControlTooltipText({ label: "Find" })).toBe("Find");
        expect(formatControlTooltipText({ label: "Find", hotkey: "⌘P" })).toBe("Find (⌘P)");
      });
  
      it("tree highlight store notifies subscribers only when highlighted ids change", () => {
        const store = createTreeHighlightStore();
        let calls = 0;
        const unsub = store.subscribe(() => {
          calls++;
        });
        store.setHighlightedIds(["a"]);
        expect(calls).toBe(1);
        expect(store.isHighlighted("a")).toBe(true);
        store.setHighlightedIds(["a"]);
        expect(calls).toBe(1);
        store.setHighlightedIds([]);
        expect(calls).toBe(2);
        expect(store.isHighlighted("a")).toBe(false);
        unsub();
      });
  
      it("shouldDispatchTreeRowPointerLeave skips leave when moving between tree rows", () => {
        document.body.innerHTML = `
          <div data-slot="tree-item-row" id="row-a"></div>
          <div data-slot="tree-item-row" id="row-b"></div>
        `;
        const rowA = document.getElementById("row-a")!;
        const rowB = document.getElementById("row-b")!;
        expect(shouldDispatchTreeRowPointerLeave(rowB)).toBe(false);
        expect(shouldDispatchTreeRowPointerLeave(rowA)).toBe(false);
        expect(shouldDispatchTreeRowPointerLeave(null)).toBe(true);
        expect(shouldDispatchTreeRowPointerLeave(document.body)).toBe(true);
      });
  
      it("shouldDispatchTreeRowPointerLeave skips leave when moving into nested tree branch content", () => {
        document.body.innerHTML = `
          <div data-slot="tree-item-row" id="row-a"></div>
          <div data-slot="tree-item-content" id="branch-a"><span id="gap"></span></div>
        `;
        const branch = document.getElementById("branch-a")!;
        const gap = document.getElementById("gap")!;
        expect(shouldDispatchTreeRowPointerLeave(branch)).toBe(false);
        expect(shouldDispatchTreeRowPointerLeave(gap)).toBe(false);
      });
  
      it("syncTreeSelectionPath marks ancestor section rows for selected tree items", () => {
        document.body.innerHTML = `
          <div id="tree-root">
            <div data-slot="tree-section-row" id="section-a"></div>
            <div data-slot="collapsible-content">
              <div data-slot="tree-section-content">
                <div data-slot="tree-item-row" id="item-a"></div>
              </div>
            </div>
          </div>
        `;
        const root = document.getElementById("tree-root")!;
        syncTreeSelectionPath(root, ["item-a"]);
        expect(document.getElementById("item-a")?.getAttribute("data-tree-selection-path")).toBe("row");
        expect(document.getElementById("section-a")?.getAttribute("data-tree-selection-path")).toBe("row");
        syncTreeSelectionPath(root, []);
        expect(document.getElementById("item-a")?.hasAttribute("data-tree-selection-path")).toBe(false);
        expect(document.getElementById("section-a")?.hasAttribute("data-tree-selection-path")).toBe(false);
      });
  
      it("syncTreeSelectionPath marks ancestor rows when the branch renders above the parent (direction=up)", () => {
        document.body.innerHTML = `
          <div id="tree-root">
            <div data-slot="collapsible-content">
              <div data-slot="tree-section-content" id="section-branch">
                <div data-slot="tree-item-content" id="group-branch">
                  <div data-slot="tree-item-row" id="item-a"></div>
                </div>
                <div data-slot="tree-item-row" id="group-a"></div>
              </div>
            </div>
            <div data-slot="tree-section-row" id="section-a"></div>
          </div>
        `;
        const root = document.getElementById("tree-root")!;
        syncTreeSelectionPath(root, ["item-a"]);
        expect(document.getElementById("item-a")?.getAttribute("data-tree-selection-path")).toBe("row");
        expect(document.getElementById("group-a")?.getAttribute("data-tree-selection-path")).toBe("row");
        expect(document.getElementById("group-branch")?.getAttribute("data-tree-selection-path")).toBe("branch");
        expect(document.getElementById("section-a")?.getAttribute("data-tree-selection-path")).toBe("row");
        expect(document.getElementById("section-branch")?.getAttribute("data-tree-selection-path")).toBe("branch");
      });
  
      it("syncTreeSelectionPath marks property-layout parent rows when nested content is a sibling branch", () => {
        document.body.innerHTML = `
          <div id="tree-root">
            <div data-slot="tree-property-item" id="object-a"></div>
            <div data-slot="tree-property-content" id="object-branch">
              <div data-slot="tree-property-item" id="vortex-a"></div>
              <div data-slot="tree-property-item" id="vortex-b"></div>
            </div>
          </div>
        `;
        const root = document.getElementById("tree-root")!;
        syncTreeSelectionPath(root, ["vortex-a"]);
        expect(document.getElementById("vortex-a")?.getAttribute("data-tree-selection-path")).toBe("row");
        expect(document.getElementById("object-a")?.getAttribute("data-tree-selection-path")).toBe("row");
        expect(document.getElementById("object-branch")?.getAttribute("data-tree-selection-path")).toBe("branch");
        expect(document.getElementById("vortex-b")?.hasAttribute("data-tree-selection-path")).toBe(false);
      });
  
      it("markGhostTreeInteraction keeps only the active tree ancestry and guides visible", () => {
        document.body.innerHTML = `
          <div data-ghost-region id="region">
            <div data-slot="tree-section-row" data-dim id="section-a">
              <div data-slot="tree-gutter" data-dim id="section-gutter"></div>
            </div>
            <div data-slot="collapsible-content">
              <div data-slot="tree-section-content" id="section-branch">
                <div data-slot="tree-guide" data-dim id="section-guide"></div>
                <div data-slot="tree-item-row" data-dim id="item-a">
                  <div data-slot="tree-gutter" data-dim id="item-gutter"></div>
                  <span id="item-target"></span>
                </div>
                <div data-slot="tree-item-row" data-dim id="item-b"></div>
              </div>
            </div>
          </div>
        `;
        const region = document.getElementById("region")!;
        const target = document.getElementById("item-target")!;
        const marked = markGhostTreeInteraction(target, region);
  
        expect(document.getElementById("item-a")?.hasAttribute("data-active-interaction")).toBe(true);
        expect(document.getElementById("section-a")?.hasAttribute("data-active-ancestor")).toBe(true);
        expect(document.getElementById("item-gutter")?.hasAttribute("data-active-ancestor")).toBe(true);
        expect(document.getElementById("section-gutter")?.hasAttribute("data-active-ancestor")).toBe(true);
        expect(document.getElementById("section-guide")?.hasAttribute("data-active-ancestor")).toBe(true);
        expect(document.getElementById("item-b")?.hasAttribute("data-active-ancestor")).toBe(false);
        expect(marked.length).toBe(6);
      });
  
      it("markGhostTreeInteraction leaves non-tree controls hidden", () => {
        document.body.innerHTML = `<div data-ghost-region id="region"><button data-dim id="command">Command</button></div>`;
        const region = document.getElementById("region")!;
        const command = document.getElementById("command")!;
  
        expect(markGhostTreeInteraction(command, region)).toEqual([]);
        expect(command.hasAttribute("data-active-interaction")).toBe(false);
      });
  
      it("automatic ghosting ignores nested controls while preserving canvas and direct tree-row drags", () => {
        document.body.innerHTML = `
          <div data-ghost-region id="region">
            <div id="canvas"></div>
            <div data-slot="control-tree-row" data-dim id="row">
              <span id="row-label">Size</span>
              <div data-slot="slider-content" data-dim>
                <div data-slot="slider" id="slider"></div>
              </div>
            </div>
            <div data-slot="window-measures-stack" data-dim>
              <button id="window-option">Option</button>
            </div>
            <div data-slot="panel-resize-handle" id="panel-resize"></div>
            <div data-slot="pane-resize-handle" id="pane-resize"></div>
            <div data-slot="resizable-handle" id="mode-resize"></div>
            <div data-slot="resizable-corner" id="mode-corner-resize"></div>
            <div data-slot="window-measures-resize-left" id="measures-resize"></div>
          </div>
        `;
  
        expect(shouldBeginAutomaticGhostInteraction(document.getElementById("canvas"))).toBe(true);
        expect(shouldBeginAutomaticGhostInteraction(document.getElementById("row-label"))).toBe(true);
        expect(shouldBeginAutomaticGhostInteraction(document.getElementById("slider"))).toBe(false);
        expect(shouldBeginAutomaticGhostInteraction(document.getElementById("window-option"))).toBe(false);
        expect(shouldBeginAutomaticGhostInteraction(document.getElementById("panel-resize"))).toBe(false);
        expect(shouldBeginAutomaticGhostInteraction(document.getElementById("pane-resize"))).toBe(false);
        expect(shouldBeginAutomaticGhostInteraction(document.getElementById("mode-resize"))).toBe(false);
        expect(shouldBeginAutomaticGhostInteraction(document.getElementById("mode-corner-resize"))).toBe(false);
        expect(shouldBeginAutomaticGhostInteraction(document.getElementById("measures-resize"))).toBe(false);
      });
  
      it("GhostProvider keeps panel chrome visible while a panel resize handle is dragged", async () => {
        const { fireEvent, render } = await import("@testing-library/react");
        const StubIcon = (): null => null;
        const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } })];
        const onSizeChange = vi.fn();
        const { container } = render(
          <GhostProvider>
            <Panel anchor="top-left" visible tabs={tabs} size={300} minSize={100} maxSize={1000} onSizeChange={onSizeChange} />
          </GhostProvider>,
        );
        const panel = container.querySelector('[data-slot="panel"]') as HTMLElement;
        const handle = container.querySelector('[data-slot="panel-resize-handle"]') as HTMLElement;
        expect(handle).toBeTruthy();
        expect(panel.querySelectorAll("[data-dim]").length).toBeGreaterThan(0);
  
        fireEvent.pointerDown(handle, { button: 0, clientX: 100, clientY: 10 });
        fireEvent.pointerMove(document, { clientX: 160, clientY: 10 });
        fireEvent.pointerMove(handle, { clientX: 160, clientY: 10 });
  
        expect(panel.hasAttribute("data-ghost")).toBe(false);
        expect(onSizeChange).toHaveBeenCalled();
        expect(panel.querySelectorAll("[data-dim]").length).toBeGreaterThan(0);
        fireEvent.pointerUp(document);
      });
  
      it("GhostProvider keeps pane chrome visible while a pane resize handle is dragged", async () => {
        const { fireEvent, render } = await import("@testing-library/react");
        const onSizeChange = vi.fn();
        const { container } = render(
          <GhostProvider>
            <GhostRegionShell>
              <PaneHost>
                <Pane id="resize-pane" anchor="top-left" icon="box" label={uiDataLabel("Resize")} resizable size={300} onSizeChange={onSizeChange} minSize={200} maxSize={600} folded={false}>
                  <div>Content</div>
                </Pane>
              </PaneHost>
            </GhostRegionShell>
          </GhostProvider>,
        );
        const region = container.querySelector("[data-ghost-region]") as HTMLElement;
        const handle = container.querySelector('[data-slot="pane-resize-handle"]') as HTMLElement;
        expect(handle).toBeTruthy();
        expect(container.querySelector('[data-slot="pane"] [data-dim]')).toBeTruthy();
  
        fireEvent.pointerDown(handle, { button: 0, pointerId: 3, clientX: 100, clientY: 0 });
        fireEvent.pointerMove(document, { clientX: 160, clientY: 0 });
        fireEvent.pointerMove(handle, { pointerId: 3, clientX: 160, clientY: 0 });
  
        expect(region.hasAttribute("data-ghost")).toBe(false);
        expect(container.querySelector('[data-slot="pane"] [data-dim]')).toBeTruthy();
        expect(onSizeChange).toHaveBeenCalledWith(360);
        fireEvent.pointerUp(document);
      });
  
      it("GhostProvider keeps sibling window UI visible while a nested slider moves", async () => {
        const { fireEvent, render } = await import("@testing-library/react");
        render(
          <GhostProvider>
            <GhostRegionShell>
              <div data-dim id="window-ui">
                Window UI
              </div>
              <div data-slot="control-tree-row" data-dim>
                <div data-slot="slider-content" data-dim>
                  <div data-slot="slider" id="nested-slider"></div>
                </div>
              </div>
            </GhostRegionShell>
          </GhostProvider>,
        );
        const slider = document.getElementById("nested-slider")!;
        const region = slider.closest("[data-ghost-region]")!;
  
        fireEvent.pointerDown(slider, { button: 0, clientX: 10, clientY: 10 });
        fireEvent.pointerMove(document, { clientX: 20, clientY: 10 });
  
        expect(region.hasAttribute("data-ghost")).toBe(false);
        expect(document.getElementById("window-ui")).not.toBeNull();
        fireEvent.pointerUp(document);
      });
  
      it("GhostProvider dims navbar/footer PanelChromeTabBar toggles and open panels including borders on interaction", async () => {
        const { fireEvent, render } = await import("@testing-library/react");
        const StubIcon = (): null => null;
        const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } })];
        const { container } = render(
          <GhostProvider>
            <PanelChromeTabBar anchor="top-left" tabs={tabs} visible={false} />
            <PanelChromeTabBar anchor="bottom-middle" tabs={tabs} visible />
            <Panel anchor="bottom-middle" tabBarHost="chrome" visible tabs={tabs} size={300} minSize={100} maxSize={1000} />
            <GhostRegionShell>
              <div id="canvas" />
            </GhostRegionShell>
          </GhostProvider>,
        );
        const foldedChromeBar = container.querySelector('[data-slot="panel-chrome-tab-bar"][data-anchor="top-left"]') as HTMLElement;
        const openChromeBar = container.querySelector('[data-slot="panel-chrome-tab-bar"][data-anchor="bottom-middle"]');
        const panel = container.querySelector('[data-slot="panel"]') as HTMLElement;
        const canvas = container.querySelector("#canvas") as HTMLElement;
        expect(foldedChromeBar).toBeTruthy();
        expect(openChromeBar).toBeTruthy();
        expect(openChromeBar?.getAttribute("data-panel-chrome-tab-bar-placeholder")).toBe("true");
        expect(panel).toBeTruthy();
        expect(panel.querySelector('[data-slot="panel-content"]')?.hasAttribute("data-dim")).toBe(true);
        expect(panel.querySelector('[data-slot="window-chrome-cap"]')?.hasAttribute("data-dim")).toBe(true);
        expect(panel.querySelector("[data-window-silhouette-border]")?.hasAttribute("data-dim")).toBe(true);
  
        fireEvent.pointerDown(canvas, { button: 0, clientX: 10, clientY: 10 });
        fireEvent.pointerMove(document, { clientX: 40, clientY: 10 });
  
        expect(panel.getAttribute("data-ghost")).toBe("true");
        expect(foldedChromeBar.hasAttribute("data-ghost")).toBe(false);
        expect(foldedChromeBar.querySelector('[data-slot="panel-tabs"]')?.hasAttribute("data-dim")).toBe(true);
        fireEvent.pointerUp(document);
      });

      it("WindowChrome stamps data-dim on the body glass surface for ghost dimming", async () => {
        const { render, waitFor } = await import("@testing-library/react");
        const rectSpy = vi.spyOn(Element.prototype, "getBoundingClientRect").mockImplementation(function (this: Element) {
          const stack = this.closest('[data-slot="ghost-body-surface-stack"]');
          if (this.getAttribute("data-slot") === "ghost-body-surface-stack") {
            return { width: 200, height: 100, top: 0, left: 0, right: 200, bottom: 100, x: 0, y: 0, toJSON: () => ({}) } as DOMRect;
          }
          if (stack && this.hasAttribute("data-window-silhouette-chip")) {
            return { width: 60, height: 24, top: 0, left: 0, right: 60, bottom: 24, x: 0, y: 0, toJSON: () => ({}) } as DOMRect;
          }
          if (stack && (this.getAttribute("data-slot") === "window-chrome-cap" || this.getAttribute("data-slot") === "window-chrome-gap")) {
            return { width: 140, height: 24, top: 0, left: 60, right: 200, bottom: 24, x: 60, y: 0, toJSON: () => ({}) } as DOMRect;
          }
          return { width: 0, height: 0, top: 0, left: 0, right: 0, bottom: 0, x: 0, y: 0, toJSON: () => ({}) } as DOMRect;
        });
        const { container } = render(
          <GhostProvider>
            <GhostRegionShell>
              <div id="canvas" />
              <WindowChrome level="panel" stackSlot="ghost-body-surface-stack" titleChips={<span>Tab</span>} body={<div>Body</div>} />
            </GhostRegionShell>
          </GhostProvider>,
        );
        await waitFor(() => {
          const bodySurface = container.querySelector('[data-slot="window-chrome-body-surface"]');
          expect(bodySurface).toBeTruthy();
          expect(bodySurface?.hasAttribute("data-dim")).toBe(true);
        });
        rectSpy.mockRestore();
      });
  
      it("GhostProvider dims open and folded pane toggles, borders, chrome, and body on interaction", async () => {
        const { fireEvent, render } = await import("@testing-library/react");
        const { container } = render(
          <GhostProvider>
            <GhostRegionShell>
              <div id="canvas" />
              <PaneHost>
                <Pane id="open-pane" anchor="top-left" icon="box" label={uiDataLabel("Open")} folded={false}>
                  <div>Open body</div>
                </Pane>
                <Pane id="folded-pane" anchor="top-right" icon="box" label={uiDataLabel("Folded")} folded />
              </PaneHost>
            </GhostRegionShell>
          </GhostProvider>,
        );
        const panes = container.querySelectorAll('[data-slot="pane"]');
        expect(panes.length).toBe(2);
        const open = Array.from(panes).find((pane) => pane.querySelector('[data-slot="pane-body"]')) as HTMLElement;
        const folded = Array.from(panes).find((pane) => !pane.querySelector('[data-slot="pane-body"]')) as HTMLElement;
        expect(open.querySelector('[data-slot="window-chrome-cap"]')?.hasAttribute("data-dim")).toBe(true);
        expect(open.querySelector('[data-slot="pane-body"]')?.hasAttribute("data-dim")).toBe(true);
        expect(folded.querySelector('[data-slot="window-chrome-chip-cap"]')?.hasAttribute("data-dim")).toBe(true);
  
        const canvas = container.querySelector("#canvas") as HTMLElement;
        fireEvent.pointerDown(canvas, { button: 0, clientX: 10, clientY: 10 });
        fireEvent.pointerMove(document, { clientX: 40, clientY: 10 });
        expect(open.closest("[data-ghost-region]")?.getAttribute("data-ghost")).toBe("true");
        expect(folded.querySelector('[data-slot="window-pane-chrome-toggle"]')).toBeTruthy();
        fireEvent.pointerUp(document);
      });
  
      it("treeRowChromeClasses uses hover tokens for highlight and active tokens for selection", () => {
        expect(treeRowChromeClasses(false, false)).toContain("text-element");
        expect(treeRowChromeClasses(false, true)).toContain("bg-hover-interactive-fill");
        expect(treeRowChromeClasses(false, true)).toContain("text-emphasized");
        expect(treeRowChromeClasses(true, true)).toContain("bg-active-base");
        expect(treeRowChromeClasses(true, false)).toContain("bg-active-base");
        expect(treeRowChromeClasses(true, false)).toContain("text-active-foreground");
        expect(treeRowChromeClasses(true, false)).not.toContain("bg-hover-interactive-fill");
        expect(treeRowChromeClasses(false, false, true)).toContain("opacity-50");
        expect(treeRowChromeClasses(true, false, true)).toContain("opacity-50");
      });
  
      it("loadingBorderStateClass returns the active ring only when both loading and active are true", () => {
        expect(loadingBorderStateClass(false)).toBe("");
        expect(loadingBorderStateClass(false, true)).toBe("");
        expect(loadingBorderStateClass(true, false)).toBe(loadingBorderClass);
        expect(loadingBorderStateClass(true, true)).toBe(loadingBorderActiveClass);
      });
  
      it("waitingBorderStateClass returns the active ring only when both waiting and active are true", () => {
        expect(waitingBorderStateClass(false)).toBe("");
        expect(waitingBorderStateClass(false, true)).toBe("");
        expect(waitingBorderStateClass(true, false)).toBe(waitingBorderClass);
        expect(waitingBorderStateClass(true, true)).toBe(waitingBorderActiveClass);
      });
  
      it("treeRowChromeContentFillClasses adds the loading ring in the row's own color", () => {
        expect(treeRowChromeContentFillClasses(true, false, true)).toContain("border-loading-active");
        expect(treeRowChromeContentFillClasses(false, false, true)).toContain("border-loading");
        expect(treeRowChromeContentFillClasses(false, false, true)).not.toContain("border-loading-active");
        expect(treeRowChromeContentFillClasses(false, false, false)).not.toContain("border-loading");
      });
  
      it("treeRowChromeContentFillClasses adds the waiting ring in the row's own color", () => {
        expect(treeRowChromeContentFillClasses(true, false, false, true)).toContain("border-waiting-active");
        expect(treeRowChromeContentFillClasses(false, false, false, true)).toContain("border-waiting");
        expect(treeRowChromeContentFillClasses(false, false, false, true)).not.toContain("border-waiting-active");
        expect(treeRowChromeContentFillClasses(false, false, false, false)).not.toContain("border-waiting");
      });
  
      it("treeRowChromeContentFillClasses prefers the loading ring over waiting when both are set", () => {
        expect(treeRowChromeContentFillClasses(false, false, true, true)).toContain("border-loading");
        expect(treeRowChromeContentFillClasses(false, false, true, true)).not.toContain("border-waiting");
      });
  
      it("scopes tree hover chrome to the row that owns the content", () => {
        expect(treeRowChromeShellClasses(false, false).split(" ")).toContain("group/tree-row");
        expect(treeRowChromeShellClasses(false, false).split(" ")).not.toContain("group");
        expect(treeRowChromeContentFillClasses(false, false)).toContain("group-hover/tree-row:");
      });
  
      it("applies tree row fill on tree-row-content so gutter guides stay visible", () => {
        const markup = renderToStaticMarkup(
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
            <TreeSection id="tooltip.manual" label="Section" defaultOpen>
              <TreeItem id="tooltip.group" label="Group" defaultOpen>
                <TreeItem id="tooltip.first" label="First" />
                <TreeItem id="tooltip.second" label="Second" isSelected />
              </TreeItem>
            </TreeSection>
          </TreeContext.Provider>,
        );
        expect(markup.match(/data-tree-guide-line/g)?.length ?? 0).toBeGreaterThanOrEqual(2);
        expect(markup).toContain('data-slot="tree-row-content"');
        expect(markup).toMatch(/data-slot="tree-row-content"[^>]*bg-active-base/);
        expect(markup).not.toMatch(/data-slot="tree-item-row"[^>]*bg-active-base/);
      });
});
}
