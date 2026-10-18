import * as ui from "@semio-tech/ui-react/source";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerShellComponents4Tests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { Layout, MODE_CANVAS_INSET_CLASS, Mode, Pane, reactHostPort, uiDataLabel } = dependencies;
  const { describe, expect, it, vi } = vitest;
  const { render, screen, fireEvent, waitFor } = await import("@testing-library/react");

describe("Shell components", () => {

    it("Mode dock-stack celebrated silhouette paints a spinning conic fill, not a solid stroke cycle", async () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[{ id: "main", title: uiDataLabel("Main"), iconId: "app-window", children: <div>Main Body</div> }]}
            layout={{ kind: "stack", children: [{ kind: "window", id: "main" }], activeId: "main" }}
            activeWindowId="main"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const stack = container.querySelector('[data-slot="mode-dock-stack"]') as HTMLElement;
      const windowEl = stack.querySelector('[data-slot="window"]') as HTMLElement;
      windowEl.setAttribute("data-celebrated", "true");
      const mockRect = (el: Element | null, rect: Partial<DOMRect>) => {
        if (!(el instanceof HTMLElement)) return;
        vi.spyOn(el, "getBoundingClientRect").mockReturnValue({
          x: 0,
          y: 0,
          top: 0,
          left: 0,
          bottom: 0,
          right: 0,
          width: 0,
          height: 0,
          toJSON: () => ({}),
          ...rect,
        } as DOMRect);
      };
      mockRect(stack, { width: 200, height: 100, right: 200, bottom: 100 });
      mockRect(stack.querySelector('[data-slot="mode-dock-tab-cap"]'), { left: 0, right: 60, width: 60, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="mode-dock-tab-gap"]'), { left: 60, right: 160, width: 100, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="mode-dock-controls-cap"]'), { left: 160, right: 200, width: 40, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="mode-dock-tabbar"]'), { height: 24, bottom: 24, width: 200, right: 200 });
      stack.setAttribute("data-silhouette-remeasure", "celebrate");
      await waitFor(() => {
        const border = container.querySelector('[data-slot="mode-dock-silhouette-border"]');
        expect(border?.getAttribute("data-kind")).toBe("celebrated");
        expect(border?.querySelector(".window-silhouette-border-celebrated-fill")).toBeTruthy();
        expect(border?.querySelector(".window-silhouette-border-celebrated-mask")).toBeTruthy();
        expect(border?.querySelector("path")?.getAttribute("stroke")).toBe("white");
        expect(border?.querySelector("path.window-silhouette-border-celebrated")).toBeNull();
      });
    });

    it("Mode lays out all windows and marks the active one", async () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "left", title: uiDataLabel("Left"), iconId: "app-window", children: <div>Left Pane</div> },
              { id: "right", title: uiDataLabel("Right"), iconId: "app-window", children: <div>Right Pane</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                { kind: "stack", children: [{ kind: "window", id: "left" }], activeId: "left" },
                { kind: "stack", children: [{ kind: "window", id: "right" }], activeId: "right" },
              ],
            }}
            activeWindowId="right"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      expect(screen.getByText("Left Pane")).toBeTruthy();
      expect(screen.getByText("Right Pane")).toBeTruthy();
      expect(container.querySelector('[data-slot="window"][data-active="true"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window"][data-active="true"]')?.className).not.toContain("border-active-base");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="right"][data-active="true"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="left"] [data-icon-kind]')).toBeTruthy();
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="left"][data-active="true"]')).toBeNull();
      expect(screen.getByText("Left")).toBeTruthy();
      expect(screen.getByText("Right")).toBeTruthy();
      expect(container.querySelector('[data-slot="mode-body"]')?.getAttribute("data-level")).toBe("base");
      expect(container.querySelector('[data-slot="mode-body"]')?.className).toContain("ui-surface");
      expect(container.querySelector('[data-slot="mode-dock-canvas-label"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).toContain("flex-1");
      expect(container.querySelector('[data-slot="mode-dock-tabbar"]')?.className).not.toContain("ui-glass");
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("ui-glass");
      expect(container.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).not.toContain("ui-glass");
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).not.toContain("ui-glass-chrome");
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).not.toContain("ui-surface");
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).not.toContain("ml-auto");
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).not.toContain("justify-end");
      const tabbar = container.querySelector('[data-slot="mode-dock-tabbar"]');
      expect(tabbar?.querySelector('[data-slot="mode-dock-tab-cap"]')).toBeTruthy();
      expect(tabbar?.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect([...(tabbar?.children ?? [])].map((child) => child.getAttribute("data-slot")).filter(Boolean)).toEqual(["mode-dock-tab-cap", "mode-dock-tab-gap"]);
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).not.toContain("ui-glass-chrome");
      expect(container.querySelector('[data-slot="mode-dock-tab-focus"]')?.className).not.toContain("text-muted-foreground");
      expect(container.querySelector('[data-slot="mode-dock-tab-close"]')?.className).not.toContain("text-muted-foreground");
      expect(container.querySelector('[data-slot="mode-dock-tab"]')?.className).toContain("text-element");
      const layoutActiveStack = container.querySelector('[data-slot="window"][data-active="true"]')?.closest('[data-slot="mode-dock-stack"]') as HTMLElement;
      const layoutInactiveStack = [...container.querySelectorAll('[data-slot="mode-dock-stack"]')].find((stack) => !stack.querySelector('[data-slot="window"][data-active="true"]')) as HTMLElement;
      expect(layoutActiveStack?.className).toContain("z-window");
      expect(layoutInactiveStack?.className).toContain("z-window");
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
      expect(layoutInactiveStack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
      fireEvent.pointerDown(layoutActiveStack.querySelector('[data-slot="mode-dock-stack-body"]')!);
      await waitFor(() => {
        expect(layoutActiveStack.getAttribute("data-active")).toBe("true");
        expect(layoutActiveStack.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("active");
      });
      expect(layoutActiveStack.querySelector('[data-slot="mode-dock-tab"][data-active="true"] [data-slot="mode-dock-tab-close"]')).toBeTruthy();
      expect(layoutInactiveStack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.className).toContain("z-[40]");
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("border-0");
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-tab-focus"]')?.className).toContain("border-0");
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-stack-body"]')?.className).toContain("border-0");
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).toContain("border-0");
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-tab-cap-corner"]')).toBeNull();
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-stack"]')?.className).not.toContain("border-emphasized");
    });

    it("Mode exposes its top cutout to the canvas background from pointer down", async () => {
      const onCanvasPointerDown = vi.fn();
      const ControlledMode = () => {
        const [activeWindowId, setActiveWindowId] = reactHostPort.useState<string | null>("right");
        return (
          <Mode
            windows={[
              { id: "left", title: uiDataLabel("Left"), iconId: "app-window", children: <div>Left Body</div> },
              {
                id: "right",
                title: uiDataLabel("Right"),
                iconId: "app-window",
                children: (
                  <div data-testid="under-cutout-canvas" onPointerDown={onCanvasPointerDown}>
                    Right Body
                  </div>
                ),
              },
            ]}
            layout={{
              kind: "stack",
              children: [
                { kind: "window", id: "left" },
                { kind: "window", id: "right" },
              ],
              activeId: "right",
            }}
            activeWindowId={activeWindowId}
            onActiveWindowChange={setActiveWindowId}
          />
        );
      };
      const { container } = render(<ControlledMode />);
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-active="true"]')?.getAttribute("data-window-id")).toBe("right");
      fireEvent.pointerDown(container.querySelector('[data-slot="mode-dock-stack-body"]')!);
      await waitFor(() => expect(container.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("active"));
      const gap = container.querySelector('[data-slot="mode-dock-tab-gap"]')!;
      expect(container.querySelector('[data-slot="mode-dock-stack"]')?.className).toContain("pointer-events-none");
      expect(container.querySelector('[data-slot="mode-dock-stack-body"]')?.className).toContain("pointer-events-auto");
      expect(container.querySelector('[data-slot="mode-dock-tab"]')?.className).toContain("pointer-events-auto");
      expect(container.querySelector('[data-slot="mode-dock-tab"]')?.className).toContain("pointer-events-auto");
      expect(gap.className).toContain("pointer-events-none");
      expect(gap.className).not.toContain("cursor-grab");
      vi.spyOn(gap, "getBoundingClientRect").mockReturnValue({ left: 60, right: 160, top: 0, bottom: 24, width: 100, height: 24, x: 60, y: 0, toJSON: () => ({}) } as DOMRect);
      fireEvent.pointerDown(screen.getByTestId("under-cutout-canvas"), { clientX: 100, clientY: 12 });
      expect(onCanvasPointerDown).toHaveBeenCalledOnce();
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-active="true"]')).toBeNull();
      expect(container.querySelector('[data-slot="window"][data-active="true"]')).toBeNull();
      expect(screen.getByText("Right Body")).toBeTruthy();
      await waitFor(() => expect(container.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal"));
    });

    it("Mode dock-stack silhouette stays below Layout panels via z-window stacking context", () => {
      const StubIcon = (): null => null;
      const { container } = render(
        <div className="h-layout-story w-layout-story-lg">
          <Layout
            canvas={
              <Mode
                windows={[{ id: "main", title: uiDataLabel("Main"), iconId: "app-window", children: <div>Main Body</div> }]}
                layout={{ kind: "stack", children: [{ kind: "window", id: "main" }], activeId: "main" }}
                activeWindowId="main"
                onActiveWindowChange={() => {}}
              />
            }
            panels={{
              "top-left": {
                visible: true,
                size: 240,
                tabs: [{ kind: "leaf", id: "workbench", icon: StubIcon, name: "Workbench", trees: [] }],
              },
            }}
          />
        </div>,
      );
      const stack = container.querySelector('[data-slot="mode-dock-stack"]');
      const panel = container.querySelector('[data-slot="panel"]') as HTMLElement | null;
      expect(stack?.className).toContain("z-window");
      expect(stack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.className).toContain("z-[40]");
      expect(panel?.className).toContain("z-panel");
      expect(panel?.className).toContain("absolute");
      expect(panel?.style.zIndex).toBe("");
      const column = container.querySelector('[data-slot="layout-canvas-column"]') as HTMLElement | null;
      expect(column?.style.paddingLeft ?? "").toBe("");
      expect(column?.style.paddingRight ?? "").toBe("");
    });

    it("Mode clears multi-tab active chrome on inactive stacks", async () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-lg">
          <Mode
            windows={[
              { id: "a1", title: uiDataLabel("A1"), iconId: "app-window", children: <div>A1 Body</div> },
              { id: "a2", title: uiDataLabel("A2"), iconId: "app-window", children: <div>A2 Body</div> },
              { id: "b1", title: uiDataLabel("B1"), iconId: "app-window", children: <div>B1 Body</div> },
              { id: "b2", title: uiDataLabel("B2"), iconId: "app-window", children: <div>B2 Body</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                {
                  kind: "stack",
                  children: [
                    { kind: "window", id: "a1" },
                    { kind: "window", id: "a2" },
                  ],
                  activeId: "a1",
                },
                {
                  kind: "stack",
                  children: [
                    { kind: "window", id: "b1" },
                    { kind: "window", id: "b2" },
                  ],
                  activeId: "b2",
                },
              ],
            }}
            activeWindowId="b2"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const activeStack = container.querySelector('[data-slot="window"][data-active="true"]')?.closest('[data-slot="mode-dock-stack"]') as HTMLElement;
      const inactiveStack = [...container.querySelectorAll('[data-slot="mode-dock-stack"]')].find((stack) => !stack.querySelector('[data-slot="window"][data-active="true"]')) as HTMLElement;
      const inactiveStackTab = inactiveStack?.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]');
      const activeStackTab = activeStack?.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]');
      expect(inactiveStackTab?.className).not.toContain("border-normal");
      expect(inactiveStackTab?.className).not.toContain("!border-normal");
      expect(inactiveStackTab?.className).not.toContain("border-emphasized");
      expect(inactiveStackTab?.className).not.toContain("border-active-base");
      expect(activeStackTab?.className).toContain("bg-active-base");
      expect(activeStackTab?.className).toContain("text-active-foreground");
      expect(activeStackTab?.className).toContain("border-0");
      expect(inactiveStackTab?.className).toContain("text-element");
      expect(inactiveStackTab?.className).not.toContain("text-foreground");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-tabbar"]')?.className).not.toContain("ui-glass");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("ui-glass");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-tab-close"]')?.className).toContain("border-0");
      expect(activeStack?.querySelector('[data-slot="mode-dock-tabbar"]')?.className).not.toContain("ui-glass");
      expect(activeStack?.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("ui-glass");
      expect(activeStack?.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect(activeStack?.querySelector('[data-slot="mode-dock-tab-focus"]')).toBeTruthy();
      expect(activeStack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
      fireEvent.pointerDown(activeStack.querySelector('[data-slot="mode-dock-stack-body"]')!);
      await waitFor(() => {
        expect(activeStack.getAttribute("data-active")).toBe("true");
        expect(activeStack.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("active");
      });
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-tab-active-cell"]')).toBeNull();
      expect(activeStack?.querySelector('[data-slot="mode-dock-tab-active-cell"]')).toBeNull();
      expect(activeStack?.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')).toBeTruthy();
    });

    it("Mode keeps one canvas inset and one gutter between adjacent stacks", () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "left", title: uiDataLabel("Left"), iconId: "app-window", children: <div>Left Pane</div> },
              { id: "right", title: uiDataLabel("Right"), iconId: "app-window", children: <div>Right Pane</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                { kind: "stack", children: [{ kind: "window", id: "left" }], activeId: "left" },
                { kind: "stack", children: [{ kind: "window", id: "right" }], activeId: "right" },
              ],
            }}
            activeWindowId="left"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const modeBody = container.querySelector('[data-slot="mode-body"]');
      expect(modeBody?.className).toContain(MODE_CANVAS_INSET_CLASS);
      const panelGroup = container.querySelector('[data-slot="resizable-panel-group"]');
      expect(panelGroup?.getAttribute("data-panel-group-direction")).toBe("horizontal");
      const panels = [...container.querySelectorAll('[data-slot="resizable-panel"]')];
      expect(panels.length).toBeGreaterThanOrEqual(2);
      for (const panel of panels) {
        expect(panel.className).not.toContain("p-single");
        expect(panel.className).not.toContain("p-double");
      }
      const horizontalHandle = container.querySelector('[data-slot="resizable-handle"]');
      expect(horizontalHandle).toBeTruthy();
      expect(horizontalHandle!.className).toContain("w-single");
      expect(horizontalHandle!.className).not.toContain("data-[panel-group-direction=horizontal]:w-single");
      expect((horizontalHandle as HTMLElement).style.width).toBe("var(--spacing-single)");
    });

    it("Mode uses the same gutter for vertical splits as canvas inset", () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "top", title: uiDataLabel("Top"), iconId: "app-window", children: <div>Top Pane</div> },
              { id: "bottom", title: uiDataLabel("Bottom"), iconId: "app-window", children: <div>Bottom Pane</div> },
            ]}
            layout={{
              kind: "column",
              children: [
                { kind: "stack", children: [{ kind: "window", id: "top" }], activeId: "top" },
                { kind: "stack", children: [{ kind: "window", id: "bottom" }], activeId: "bottom" },
              ],
            }}
            activeWindowId="top"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const modeBody = container.querySelector('[data-slot="mode-body"]');
      expect(modeBody?.className).toContain(MODE_CANVAS_INSET_CLASS);
      const panelGroup = container.querySelector('[data-slot="resizable-panel-group"]');
      expect(panelGroup?.getAttribute("data-panel-group-direction")).toBe("vertical");
      const verticalHandle = container.querySelector('[data-slot="resizable-handle"]') as HTMLElement | null;
      expect(verticalHandle).toBeTruthy();
      expect(verticalHandle!.getAttribute("data-resize-orientation")).toBe("vertical");
      expect(verticalHandle!.className).toContain("h-single");
      expect(verticalHandle!.style.height).toBe("var(--spacing-single)");
    });

    it("Mode renders corner grabs at perpendicular split intersections", () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "lt", title: uiDataLabel("Left Top"), iconId: "app-window", children: <div>Left Top</div> },
              { id: "lb", title: uiDataLabel("Left Bottom"), iconId: "app-window", children: <div>Left Bottom</div> },
              { id: "rt", title: uiDataLabel("Right Top"), iconId: "app-window", children: <div>Right Top</div> },
              { id: "rb", title: uiDataLabel("Right Bottom"), iconId: "app-window", children: <div>Right Bottom</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                {
                  kind: "column",
                  children: [
                    { kind: "stack", children: [{ kind: "window", id: "lt" }], activeId: "lt" },
                    { kind: "stack", children: [{ kind: "window", id: "lb" }], activeId: "lb" },
                  ],
                },
                {
                  kind: "column",
                  children: [
                    { kind: "stack", children: [{ kind: "window", id: "rt" }], activeId: "rt" },
                    { kind: "stack", children: [{ kind: "window", id: "rb" }], activeId: "rb" },
                  ],
                },
              ],
            }}
            activeWindowId="lt"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const corners = [...container.querySelectorAll('[data-slot="resizable-corner"]')];
      expect(corners.length).toBeGreaterThanOrEqual(4);
      expect(corners.every((node) => node.className.includes("cursor-move"))).toBe(true);
    });

    it("Mode tab stack shows only the active window body", async () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "a", title: uiDataLabel("Alpha"), iconId: "app-window", children: <div>Alpha Body</div> },
              { id: "b", title: uiDataLabel("Beta"), iconId: "app-window", children: <div>Beta Body</div> },
            ]}
            layout={{
              kind: "stack",
              children: [
                { kind: "window", id: "a" },
                { kind: "window", id: "b" },
              ],
              activeId: "a",
            }}
            activeWindowId="a"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      expect(container.querySelector('[data-slot="mode-dock-stack-body"]')?.getAttribute("data-level")).toBe("base");
      expect(container.querySelector('[data-slot="mode-dock-stack-body"]')?.className).toContain("ui-surface");
      expect(container.querySelector('[data-slot="mode-dock-stack-body"]')?.className).toContain("p-[var(--padding-standard)]");
      expect(screen.getByText("Alpha Body")).toBeTruthy();
      expect(screen.queryByText("Beta Body")).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-chrome-column"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab-active-cell"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("ui-glass");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).toContain("bg-active-base");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).toContain("text-active-foreground");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).toContain("border-0");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).toContain("z-20");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="b"]')?.className).toContain("z-30");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="b"]')?.className).not.toContain("border-normal");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="b"]')?.className).not.toContain("!border-normal");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="b"]')?.className).not.toContain("border-emphasized");
      expect(container.querySelector('[data-slot="mode-dock-chrome-column"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-stack"] [data-slot="mode-dock-stack-body"]')).toBeTruthy();
      const multiTabBar = container.querySelector('[data-slot="mode-dock-tabbar"]');
      expect(multiTabBar?.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect(multiTabBar?.querySelectorAll('[data-slot="mode-dock-tab-focus"]')).toHaveLength(2);
      expect(container.querySelector('[data-slot="mode-dock-stack"]')?.className).not.toContain("grid");
      expect(container.querySelector('[data-slot="mode-dock-tabbar"]')?.className).not.toContain("ui-glass");
      expect(container.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("ui-glass");
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).not.toContain("ui-glass");
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).not.toContain("ui-glass-chrome");
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).toContain("border-0");
      expect(container.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
      fireEvent.pointerDown(container.querySelector('[data-slot="mode-dock-stack-body"]')!);
      await waitFor(() => {
        expect(container.querySelector('[data-slot="mode-dock-stack"]')?.getAttribute("data-active")).toBe("true");
        expect(container.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("active");
      });
      const tabOrder = () => [...container.querySelectorAll('[data-slot="mode-dock-tab"]')].map((tab) => tab.getAttribute("data-window-id"));
      expect(tabOrder()).toEqual(["a", "b"]);
      fireEvent.click(screen.getByText("Beta"));
      expect(screen.getByText("Beta Body")).toBeTruthy();
      expect(screen.queryByText("Alpha Body")).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.getAttribute("data-window-id")).toBe("b");
      expect(tabOrder()).toEqual(["a", "b"]);
    });
});
}
