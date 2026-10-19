import type { DockSkeleton, PanelDock, PanelTabNode, RibbonRow } from "@semio-tech/ui-react/source";
import * as ui from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests2Vitest, Tests2Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerControlChrome2Tests(vitest: Tests2Vitest, dependencies: Tests2Dependencies, testSource: TestSource): Promise<void> {
  const { applyDockSkeleton, dockSkeletonOf, dockSkeletonsEqual, isPanelTabInSubtree, measureWindowSilhouetteMetrics, Panel, PanelRightIcon, reconcileActivePath, renderToStaticMarkup, Ribbon, RibbonItem, RibbonZone, Search, singleTreeLeaf, Stepper, ToggleGroup, WindowChrome, WINDOW_PANE_MEASURES_ICON, WindowPaneChromeToggle, windowSilhouettePath } = dependencies;
  const { describe, expect, it, vi } = vitest;

describe("control chrome", () => {

  
      it("panel chip-cap paints glass above a transparent payload without unused controls", async () => {
        const { render } = await import("@testing-library/react");
        const tabs = [singleTreeLeaf({ id: "tab-a", icon: PanelRightIcon, name: "Tab A", tree: { sections: [] } }), singleTreeLeaf({ id: "tab-b", icon: PanelRightIcon, name: "Tab B", tree: { sections: [] } })];
        const { container } = render(<Panel anchor="top-left" visible onVisibleChange={() => {}} tabs={tabs} activeTabPath={["tab-a"]} />);
        const chip = container.querySelector('[data-slot="window-chrome-chip-cap"]') as HTMLElement;
        const body = container.querySelector('[data-slot="panel-content"]') as HTMLElement;
        const controls = container.querySelector('[data-slot="window-chrome-controls"]') as HTMLElement;
        expect(chip.className).toContain("ui-glass");
        expect(body.hasAttribute("data-window-silhouette-content")).toBe(true);
        expect(body.className).not.toContain("ui-glass");
        expect(controls).toBeNull();
        expect(chip.className).not.toContain("ui-glass-chrome");
        expect(chip.hasAttribute("data-window-silhouette-chip")).toBe(true);
        expect(container.querySelector('[data-slot="window-chrome-silhouette-border"]')).toBeTruthy();
        const inactive = container.querySelector('[data-slot="panel-tab-button"]:not([data-active="true"])') as HTMLElement;
        const active = container.querySelector('[data-slot="panel-tab-button"][data-active="true"]') as HTMLElement;
        expect(inactive.className).toContain("bg-transparent");
        expect(inactive.className).not.toContain("ui-surface");
        expect(inactive.className).toContain("border-e");
        expect(inactive.className).toContain("!border-normal");
        expect(active.className).toContain("bg-active-base");
        expect(active.className).toContain("border-e");
        expect(active.className).toContain("!border-normal");
        expect(active.className).toContain("border-0");
        expect(body.style.zIndex).toBe("1");
        expect((container.querySelector('[data-slot="window-chrome-cap"]') as HTMLElement).style.zIndex).toBe("2");
      });
  
      it("measureWindowSilhouetteMetrics reads RTL top caps from painted chip spans instead of assuming LTR gap order", () => {
        const stack = document.createElement("div");
        stack.innerHTML = `
          <div data-slot="window-chrome-cap">
            <div data-slot="window-chrome-controls" data-window-silhouette-chip data-dock="top"></div>
            <div data-slot="window-chrome-gap"></div>
            <div data-slot="window-chrome-chip-cap" data-window-silhouette-chip data-dock="top"></div>
          </div>
        `;
        const mockRect = (el: Element | null, rect: Partial<DOMRect>) => {
          if (!(el instanceof HTMLElement)) return;
          vi.spyOn(el, "getBoundingClientRect").mockReturnValue({
            x: rect.left ?? 0,
            y: rect.top ?? 0,
            top: rect.top ?? 0,
            left: rect.left ?? 0,
            bottom: rect.bottom ?? 0,
            right: rect.right ?? 0,
            width: rect.width ?? 0,
            height: rect.height ?? 0,
            toJSON: () => ({}),
            ...rect,
          } as DOMRect);
        };
        mockRect(stack, { width: 200, height: 100, right: 200, bottom: 100 });
        mockRect(stack.querySelector('[data-slot="window-chrome-controls"]'), { left: 0, right: 40, width: 40, height: 24, top: 0, bottom: 24 });
        mockRect(stack.querySelector('[data-slot="window-chrome-gap"]'), { left: 40, right: 140, width: 100, height: 24, top: 0, bottom: 24 });
        mockRect(stack.querySelector('[data-slot="window-chrome-chip-cap"]'), { left: 140, right: 200, width: 60, height: 24, top: 0, bottom: 24 });
        mockRect(stack.querySelector('[data-slot="window-chrome-cap"]'), { left: 0, right: 200, width: 200, height: 24, top: 0, bottom: 24 });
        expect(measureWindowSilhouetteMetrics(stack)).toEqual({
          width: 200,
          height: 100,
          top: {
            depth: 24,
            chips: [
              { left: 0, right: 40 },
              { left: 140, right: 200 },
            ],
          },
          bottom: { depth: 0, chips: [] },
        });
      });
  
      it("measureWindowSilhouetteMetrics places bottom-docked caps on the bottom silhouette edge", () => {
        const stack = document.createElement("div");
        stack.innerHTML = `
          <div data-slot="window-chrome-body"></div>
          <div data-slot="window-chrome-cap">
            <div data-slot="window-chrome-chip-cap" data-window-silhouette-chip data-dock="bottom"></div>
            <div data-slot="window-chrome-gap"></div>
            <div data-slot="window-chrome-controls" data-window-silhouette-chip data-dock="bottom"></div>
          </div>
        `;
        const mockRect = (el: Element | null, rect: Partial<DOMRect>) => {
          if (!(el instanceof HTMLElement)) return;
          vi.spyOn(el, "getBoundingClientRect").mockReturnValue({
            x: rect.left ?? 0,
            y: rect.top ?? 0,
            top: rect.top ?? 0,
            left: rect.left ?? 0,
            bottom: rect.bottom ?? 0,
            right: rect.right ?? 0,
            width: rect.width ?? 0,
            height: rect.height ?? 0,
            toJSON: () => ({}),
            ...rect,
          } as DOMRect);
        };
        mockRect(stack, { width: 200, height: 100, right: 200, bottom: 100 });
        mockRect(stack.querySelector('[data-slot="window-chrome-chip-cap"]'), { left: 0, right: 60, width: 60, height: 24, top: 76, bottom: 100 });
        mockRect(stack.querySelector('[data-slot="window-chrome-gap"]'), { left: 60, right: 160, width: 100, height: 24, top: 76, bottom: 100 });
        mockRect(stack.querySelector('[data-slot="window-chrome-controls"]'), { left: 160, right: 200, width: 40, height: 24, top: 76, bottom: 100 });
        mockRect(stack.querySelector('[data-slot="window-chrome-cap"]'), { left: 0, right: 200, width: 200, height: 24, top: 76, bottom: 100 });
        const metrics = measureWindowSilhouetteMetrics(stack);
        expect(metrics?.top).toEqual({ depth: 0, chips: [] });
        expect(metrics?.bottom).toEqual({
          depth: 24,
          chips: [
            { left: 0, right: 60 },
            { left: 160, right: 200 },
          ],
        });
        expect(windowSilhouettePath(metrics!)).toBe(
          windowSilhouettePath({
            width: 200,
            height: 100,
            top: { depth: 0, chips: [] },
            bottom: {
              depth: 24,
              chips: [
                { left: 0, right: 60 },
                { left: 160, right: 200 },
              ],
            },
          }),
        );
      });
  
      it("measureWindowSilhouetteMetrics ignores nested pane silhouette chips so the window bottom stays rectangular", () => {
        const stack = document.createElement("div");
        stack.setAttribute("data-window-silhouette", "");
        stack.setAttribute("data-slot", "mode-dock-stack");
        stack.innerHTML = `
          <div data-slot="mode-dock-tabbar">
            <div data-slot="mode-dock-tab-cap" data-window-silhouette-chip data-dock="top"></div>
            <div data-slot="mode-dock-tab-gap"></div>
            <div data-slot="mode-dock-controls-cap" data-window-silhouette-chip data-dock="top"></div>
          </div>
          <div data-slot="mode-dock-stack-body">
            <div data-window-silhouette data-slot="window-chrome-stack">
              <div data-slot="window-chrome-chip-cap" data-window-silhouette-chip data-dock="bottom"></div>
              <div data-slot="window-chrome-gap"></div>
              <div data-slot="window-chrome-controls" data-window-silhouette-chip data-dock="bottom"></div>
            </div>
          </div>
        `;
        const mockRect = (el: Element | null, rect: Partial<DOMRect>) => {
          if (!(el instanceof HTMLElement)) return;
          vi.spyOn(el, "getBoundingClientRect").mockReturnValue({
            x: rect.left ?? 0,
            y: rect.top ?? 0,
            top: rect.top ?? 0,
            left: rect.left ?? 0,
            bottom: rect.bottom ?? 0,
            right: rect.right ?? 0,
            width: rect.width ?? 0,
            height: rect.height ?? 0,
            toJSON: () => ({}),
            ...rect,
          } as DOMRect);
        };
        mockRect(stack, { width: 200, height: 100, right: 200, bottom: 100 });
        mockRect(stack.querySelector('[data-slot="mode-dock-tab-cap"]'), { left: 0, right: 60, width: 60, height: 24, top: 0, bottom: 24 });
        mockRect(stack.querySelector('[data-slot="mode-dock-tab-gap"]'), { left: 60, right: 160, width: 100, height: 24, top: 0, bottom: 24 });
        mockRect(stack.querySelector('[data-slot="mode-dock-controls-cap"]'), { left: 160, right: 200, width: 40, height: 24, top: 0, bottom: 24 });
        mockRect(stack.querySelector('[data-slot="mode-dock-tabbar"]'), { left: 0, right: 200, width: 200, height: 24, top: 0, bottom: 24 });
        mockRect(stack.querySelector('[data-slot="window-chrome-chip-cap"]'), { left: 140, right: 200, width: 60, height: 24, top: 76, bottom: 100 });
        mockRect(stack.querySelector('[data-slot="window-chrome-gap"]'), { left: 0, right: 140, width: 140, height: 24, top: 76, bottom: 100 });
        mockRect(stack.querySelector('[data-slot="window-chrome-controls"]'), { left: 0, right: 0, width: 0, height: 24, top: 76, bottom: 100 });
        expect(measureWindowSilhouetteMetrics(stack)).toEqual({
          width: 200,
          height: 100,
          top: {
            depth: 24,
            chips: [
              { left: 0, right: 60 },
              { left: 160, right: 200 },
            ],
          },
          bottom: { depth: 0, chips: [] },
        });
      });
  
      it("bottom-anchored panels stamp capDock=bottom on WindowChrome and keep rtl on the panel root for top-right", () => {
        const StubIcon = (): null => null;
        const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } }), singleTreeLeaf({ id: "tab-b", icon: StubIcon, name: "Tab B", tree: { sections: [] } })];
        const bottomMarkup = renderToStaticMarkup(<Panel anchor="bottom-right" visible onVisibleChange={() => {}} tabs={tabs} activeTabPath={["tab-a"]} />);
        expect(bottomMarkup).toContain('data-dock="bottom"');
        expect(bottomMarkup).toContain("flex-col-reverse");
        const rightMarkup = renderToStaticMarkup(<Panel anchor="top-right" visible onVisibleChange={() => {}} tabs={tabs} activeTabPath={["tab-a"]} />);
        expect(rightMarkup).toContain('dir="rtl"');
        expect(rightMarkup).toContain('data-dock="top"');
        expect(rightMarkup).not.toMatch(/data-slot="window-chrome-stack"[^>]*flex-col-reverse/);
      });
  
      it("pane and panel chrome toggles emphasize only their own hovered content", () => {
        const paneMarkup = renderToStaticMarkup(<WindowPaneChromeToggle id="ui.pane.toggle.hover" icon={WINDOW_PANE_MEASURES_ICON} label="Options" dragPointerProps={{ onPointerDown: () => {} }} />);
        expect(paneMarkup).toContain("data-hover-scope");
        expect(paneMarkup).toContain('data-slot="drag-handle"');
        expect(paneMarkup).toMatch(/data-hover-scope[\s\S]*data-slot="drag-handle"/);
        const toggleClassName = paneMarkup.match(/data-slot="window-pane-chrome-toggle"[^>]*class="([^"]+)"/)?.[1] ?? "";
        expect(toggleClassName).toContain("hover:not-data-[handle-hovered=true]:text-emphasized");
        const handleClassName = paneMarkup.match(/data-slot="drag-handle"[^>]*class="([^"]+)"/)?.[1] ?? "";
        expect(handleClassName).toContain("text-muted-foreground");
        expect(handleClassName).toContain("hover:text-emphasized");
      });
  
      it("draws exactly one border around the Stepper group, never a second one on its buttons", () => {
        const stepperMarkup = renderToStaticMarkup(<Stepper id="ui.stepper.regression" value={5} min={0} max={10} />);
        expect(stepperMarkup).not.toContain("border-border");
        expect(stepperMarkup.match(/data-slot="stepper-(group|minus|plus)"/g)?.length).toBe(3);
        const groupMatch = stepperMarkup.match(/<div data-slot="stepper-group"[^>]*class="([^"]*)"/);
        expect(groupMatch?.[1]).toContain("border");
      });
  
      it("renders adjacent ribbon toggle group items for segmented focus borders", () => {
        const markup = renderToStaticMarkup(
          <RibbonZone>
            <RibbonItem>
              <ToggleGroup
                kind="single"
                value="save"
                items={[
                  { value: "transform", id: "ui.ribbon.group.transform", icon: "move-3d", text: "Transform" },
                  { value: "save", id: "ui.ribbon.group.save", icon: "save", text: "Save" },
                  { value: "transfer", id: "ui.ribbon.group.transfer", icon: "arrow-right-left", text: "Transfer" },
                ]}
              />
            </RibbonItem>
          </RibbonZone>,
        );
        expect(markup).toContain('data-slot="toggle-group-item"');
        expect(markup.match(/data-slot="toggle-group-item"/g)?.length).toBe(3);
        expect(markup).toContain('data-state="on"');
      });
  
      it("Ribbon stacks rows upward for direction=up and downward for direction=down, inline keeps one line", () => {
        const rows: RibbonRow[] = [
          { key: "base", content: <span>Base</span> },
          { key: "nested", content: <span>Nested</span> },
        ];
        const upMarkup = renderToStaticMarkup(<Ribbon direction="up" rows={rows} />);
        expect(upMarkup).toContain('data-direction="up"');
        expect(upMarkup).toContain("flex-col-reverse");
        expect(upMarkup.match(/data-slot="ribbon-row"/g)?.length).toBe(2);
        expect(upMarkup.match(/data-slot="ribbon-row"[^>]*shrink-0/g)?.length).toBe(2);
        expect(upMarkup.indexOf("Base")).toBeLessThan(upMarkup.indexOf("Nested"));
  
        const downMarkup = renderToStaticMarkup(<Ribbon direction="down" rows={rows} />);
        expect(downMarkup).toContain('data-direction="down"');
        expect(downMarkup).not.toContain("flex-col-reverse");
        expect(downMarkup).not.toContain("gap-single");
        const rowMatches = [...downMarkup.matchAll(/data-slot="ribbon-row"[^>]*class="([^"]*)"/g)];
        expect(rowMatches.length).toBe(2);
        for (const match of rowMatches) expect(match[1]).not.toMatch(/\bgap-single\b/);
  
        const inlineMarkup = renderToStaticMarkup(<Ribbon id="ui.ribbon" direction="inline" rows={rows} />);
        expect(inlineMarkup).toContain('role="toolbar"');
        expect(inlineMarkup).toContain('data-slot="ribbon-zone"');
        expect(inlineMarkup).not.toContain('data-slot="ribbon-row"');
  
        const variableMarkup = renderToStaticMarkup(
          <RibbonZone variableHeight>
            <div>Intrinsic content</div>
          </RibbonZone>,
        );
        expect(variableMarkup).toContain('data-variable-height="true"');
        const variableZoneClass = variableMarkup.match(/data-variable-height="true" class="([^"]*)"/)?.[1].split(" ") ?? [];
        expect(variableZoneClass).toContain("h-auto");
        expect(variableZoneClass).toContain("min-h-medium");
        expect(variableZoneClass).not.toContain("h-medium");
      });
  
      it("reconcileActivePath validates each segment and truncates at the first invalid one — no first-sibling substitution, no auto-descend", () => {
        type Node = { readonly id: string; readonly children?: readonly Node[] };
        const tree: Node[] = [
          { id: "workbench", children: [{ id: "document" }, { id: "catalogue" }] },
          { id: "display", children: [{ id: "windows" }] },
        ];
        const childrenOf = (node: Node) => node.children;
        expect(reconcileActivePath(tree, [], childrenOf)).toEqual([]);
        expect(reconcileActivePath(tree, ["display"], childrenOf)).toEqual(["display"]);
        expect(reconcileActivePath(tree, ["workbench", "catalogue"], childrenOf)).toEqual(["workbench", "catalogue"]);
        expect(reconcileActivePath(tree, ["unknown"], childrenOf)).toEqual([]);
        expect(reconcileActivePath(tree, ["workbench", "nope"], childrenOf)).toEqual(["workbench"]);
      });
  
      it("dockSkeletonOf/dockSkeletonsEqual round-trip a PanelDock (incl. a populated middle anchor) and detect structural equality", () => {
        const StubIcon = (): null => null;
        const dock: PanelDock = {
          anchors: {
            "top-left": [
              {
                kind: "branch",
                id: "workbench",
                icon: StubIcon,
                name: "Workbench",
                children: [singleTreeLeaf({ id: "document", icon: StubIcon, name: "Artifact", tree: { sections: [] } })],
              },
            ],
            "top-middle": [singleTreeLeaf({ id: "search", icon: StubIcon, name: "Search", tree: { sections: [] } })],
            "top-right": [],
            "right-middle": [],
            "bottom-right": [singleTreeLeaf({ id: "settings", icon: StubIcon, name: "Settings", tree: { sections: [] } })],
            "bottom-middle": [],
            "bottom-left": [],
            "left-middle": [],
          },
        };
        const skeleton = dockSkeletonOf(dock);
        expect(skeleton).toEqual({
          version: 3,
          anchors: {
            "top-left": [{ id: "workbench", children: [{ id: "document", trees: ["document.tree"] }] }],
            "top-middle": [{ id: "search", trees: ["search.tree"] }],
            "top-right": [],
            "right-middle": [],
            "bottom-right": [{ id: "settings", trees: ["settings.tree"] }],
            "bottom-middle": [],
            "bottom-left": [],
            "left-middle": [],
          },
        });
        expect(dockSkeletonsEqual(skeleton, dockSkeletonOf(dock))).toBe(true);
        expect(dockSkeletonsEqual(skeleton, null)).toBe(false);
        expect(dockSkeletonsEqual(null, null)).toBe(true);
      });
  
      it("applyDockSkeleton reuses default object identity, drops unknown ids, appends new defaults, and falls back on kind mismatch", () => {
        const StubIcon = (): null => null;
        const documentLeaf = singleTreeLeaf({ id: "document", icon: StubIcon, name: "Artifact", tree: { sections: [] } });
        const catalogueLeaf = singleTreeLeaf({ id: "catalogue", icon: StubIcon, name: "Catalogue", tree: { sections: [] } });
        const workbenchBranch: PanelTabNode = { kind: "branch", id: "workbench", icon: StubIcon, name: "Workbench", children: [documentLeaf, catalogueLeaf] };
        const settingsLeaf = singleTreeLeaf({ id: "settings", icon: StubIcon, name: "Settings", tree: { sections: [] } });
        const defaultDock: PanelDock = {
          anchors: { "top-left": [workbenchBranch], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [settingsLeaf], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
        };
  
        const untouchedSkeleton = dockSkeletonOf(defaultDock);
        const untouched = applyDockSkeleton(defaultDock, untouchedSkeleton);
        expect(untouched.anchors["top-left"][0]).toBe(workbenchBranch);
        expect(untouched.anchors["bottom-right"][0]).toBe(settingsLeaf);
  
        const moved: DockSkeleton = {
          version: 3,
          anchors: {
            "top-left": [{ id: "workbench", children: [{ id: "document", trees: ["document.tree"] }] }],
            "top-middle": [],
            "top-right": [],
            "right-middle": [],
            "bottom-right": [
              { id: "catalogue", trees: ["catalogue.tree"] },
              { id: "settings", trees: ["settings.tree"] },
            ],
            "bottom-middle": [],
            "bottom-left": [],
            "left-middle": [],
          },
        };
        const afterMove = applyDockSkeleton(defaultDock, moved);
        const movedWorkbench = afterMove.anchors["top-left"][0];
        expect(movedWorkbench?.kind).toBe("branch");
        expect(movedWorkbench?.kind === "branch" ? movedWorkbench.children : []).toEqual([documentLeaf]);
        expect(afterMove.anchors["bottom-right"].map((tab) => tab.id)).toEqual(["catalogue", "settings"]);
        expect(afterMove.anchors["bottom-right"][0]).toBe(catalogueLeaf);
  
        const withUnknown: DockSkeleton = {
          version: 3,
          anchors: { "top-left": [{ id: "ghost-tab" }], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
        };
        const afterUnknown = applyDockSkeleton(defaultDock, withUnknown);
        expect(afterUnknown.anchors["top-left"].map((tab) => tab.id)).toEqual(["workbench"]);
        expect(afterUnknown.anchors["top-left"][0]).toBe(workbenchBranch);
        expect(afterUnknown.anchors["bottom-right"].map((tab) => tab.id)).toEqual(["settings"]);
  
        const kindMismatch: DockSkeleton = {
          version: 3,
          anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [{ id: "settings", children: [{ id: "document" }] }], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
        };
        const afterMismatch = applyDockSkeleton(defaultDock, kindMismatch);
        expect(afterMismatch.anchors["bottom-right"][0]).toBe(settingsLeaf);
        expect(afterMismatch.anchors["top-left"].map((tab) => tab.id)).toEqual(["workbench"]);
      });
  
      it("applyDockSkeleton keeps a deliberately-emptied branch empty instead of resurrecting its default children", () => {
        const StubIcon = (): null => null;
        const documentLeaf = singleTreeLeaf({ id: "document", icon: StubIcon, name: "Artifact", tree: { sections: [] } });
        const workbenchBranch: PanelTabNode = { kind: "branch", id: "workbench", icon: StubIcon, name: "Workbench", children: [documentLeaf] };
        const defaultDock: PanelDock = {
          anchors: { "top-left": [workbenchBranch], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
        };
        const emptiedWorkbench: DockSkeleton = {
          version: 3,
          anchors: {
            "top-left": [{ id: "workbench", children: [] }],
            "top-middle": [],
            "top-right": [],
            "right-middle": [],
            "bottom-right": [{ id: "document", trees: ["document.tree"] }],
            "bottom-middle": [],
            "bottom-left": [],
            "left-middle": [],
          },
        };
        const result = applyDockSkeleton(defaultDock, emptiedWorkbench);
        const resultWorkbench = result.anchors["top-left"][0];
        expect(resultWorkbench?.kind === "branch" ? resultWorkbench.children : null).toEqual([]);
        expect(result.anchors["bottom-right"][0]).toBe(documentLeaf);
      });
  
      it("isPanelTabInSubtree matches the node itself and any descendant, but not unrelated siblings", () => {
        const StubIcon = (): null => null;
        const child = singleTreeLeaf({ id: "child", icon: StubIcon, name: "Child", tree: { sections: [] } });
        const branch: PanelTabNode = { kind: "branch", id: "parent", icon: StubIcon, name: "Parent", children: [child] };
        expect(isPanelTabInSubtree(branch, "parent")).toBe(true);
        expect(isPanelTabInSubtree(branch, "child")).toBe(true);
        expect(isPanelTabInSubtree(branch, "unrelated")).toBe(false);
        expect(isPanelTabInSubtree(child, "parent")).toBe(false);
      });
});
}
