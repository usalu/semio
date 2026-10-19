import * as ui from "@semio-tech/ui-react/source";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerShellComponents10Tests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { COMPACT_UI_DRIVER, Pane, PaneHost, Panel, UiDriverProvider, UiMobileProvider, createEvenWindowLayout, nearestAnchor, uiDataLabel, usePaneSlot } = dependencies;
  type Anchor = any;
  const { describe, expect, it, vi } = vitest;
  const { render, screen, fireEvent } = await import("@testing-library/react");

describe("Shell components", () => {

    describe("Pane", () => {
      it("nearestAnchor resolves each of the eight zones, incl. the dead-center zone by distance from the nearer midline", () => {
        const hostRect = { left: 0, top: 0, width: 300, height: 300 };
        expect(nearestAnchor(10, 10, hostRect)).toBe("top-left");
        expect(nearestAnchor(150, 10, hostRect)).toBe("top-middle");
        expect(nearestAnchor(290, 10, hostRect)).toBe("top-right");
        expect(nearestAnchor(10, 150, hostRect)).toBe("left-middle");
        expect(nearestAnchor(290, 150, hostRect)).toBe("right-middle");
        expect(nearestAnchor(10, 290, hostRect)).toBe("bottom-left");
        expect(nearestAnchor(150, 290, hostRect)).toBe("bottom-middle");
        expect(nearestAnchor(290, 290, hostRect)).toBe("bottom-right");
        expect(nearestAnchor(160, 155, hostRect)).toBe("right-middle");
        expect(nearestAnchor(155, 160, hostRect)).toBe("bottom-middle");
      });

      it("Pane defaults to a folded chip when folded is omitted", () => {
        const { container } = render(
          <PaneHost>
            <Pane id="default-folded-pane" anchor="top-left" icon="box" label={uiDataLabel("Pane")}>
              <div data-testid="default-folded-content">Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(container.querySelector('[data-slot="pane"]')?.getAttribute("data-folded")).toBe("true");
        expect(screen.queryByTestId("default-folded-content")).toBeNull();
      });

      it("Pane defaults open width to the panel default (300px) when unfolded", () => {
        const { container } = render(
          <PaneHost>
            <Pane id="default-width-pane" anchor="top-left" icon="box" label={uiDataLabel("Pane")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        const pane = container.querySelector('[data-slot="pane"]') as HTMLElement;
        expect(pane.style.width).toBe("300px");
        expect(container.querySelector('[data-slot="window-chrome-gap"]')).toBeTruthy();
      });

      it("Pane positions itself via the same anchorPositionStyle math as Panel and folds to a chip", () => {
        const { container, rerender } = render(
          <PaneHost>
            <Pane id="test-pane" anchor="top-right" icon="box" label={uiDataLabel("Test")} folded={false}>
              <div data-testid="pane-content">Content</div>
            </Pane>
          </PaneHost>,
        );
        const pane = container.querySelector('[data-slot="pane"]') as HTMLElement;
        expect(pane.getAttribute("data-anchor")).toBe("top-right");
        expect(pane.style.right).toBe("var(--spacing-single)");
        expect(pane.style.top).toBe("var(--spacing-single)");
        expect(screen.getByTestId("pane-content")).toBeTruthy();
        rerender(
          <PaneHost>
            <Pane id="test-pane" anchor="top-right" icon="box" label={uiDataLabel("Test")} folded>
              <div data-testid="pane-content">Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(screen.queryByTestId("pane-content")).toBeNull();
      });

      it("Pane opens its own pane level with chip glass above a transparent clipped payload", () => {
        const { container } = render(
          <PaneHost>
            <Pane id="level-pane" anchor="top-right" icon="box" label={uiDataLabel("Test")} folded={false}>
              <div data-testid="pane-level-content">{"placeholder"}</div>
            </Pane>
          </PaneHost>,
        );
        const pane = container.querySelector('[data-slot="pane"]') as HTMLElement;
        expect(pane.getAttribute("data-level")).toBe("pane");
        const chromeStack = container.querySelector('[data-slot="window-chrome-stack"]') as HTMLElement;
        expect(chromeStack.getAttribute("data-level")).toBe("pane");
        const chip = container.querySelector('[data-slot="window-chrome-chip-cap"]') as HTMLElement;
        expect(chip.className).toContain("ui-glass");
        const body = container.querySelector('[data-slot="pane-body"]') as HTMLElement;
        expect(body.hasAttribute("data-window-silhouette-content")).toBe(true);
        expect(body.className).not.toContain("ui-glass");
      });

      it("Pane grows down from top anchors, up from bottom anchors, and symmetrically around middle anchors within responsive bounds", () => {
        const { container, rerender } = render(
          <PaneHost>
            <Pane id="direction-pane" anchor="top-left" icon="box" label={uiDataLabel("Direction")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        const pane = () => container.querySelector('[data-slot="pane"]') as HTMLElement;
        expect(pane().className).toContain("flex-col");
        expect(pane().className).not.toContain("flex-col-reverse");
        expect(pane().style.top).toBe("var(--spacing-single)");
        expect(pane().style.maxWidth).toBe("min(100% - (var(--spacing-single) * 2), 300px)");
        expect(pane().style.maxHeight).toBe("calc(100% - (var(--panel-inset) * 2))");

        rerender(
          <PaneHost>
            <Pane id="direction-pane" anchor="bottom-left" icon="box" label={uiDataLabel("Direction")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(pane().className).toContain("flex-col-reverse");
        expect(pane().style.bottom).toBe("var(--spacing-single)");

        rerender(
          <PaneHost>
            <Pane id="direction-pane" anchor="bottom-middle" icon="box" label={uiDataLabel("Direction")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(pane().className).toContain("flex-col-reverse");
        expect(pane().className).toContain("items-center");
        expect(pane().style.bottom).toBe("var(--spacing-single)");
        expect(pane().style.left).toBe("50%");
        expect(pane().style.transform).toBe("translateX(-50%)");

        rerender(
          <PaneHost>
            <Pane id="direction-pane" anchor="left-middle" icon="box" label={uiDataLabel("Direction")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(pane().style.top).toBe("50%");
        expect(pane().style.transform).toBe("translateY(-50%)");
      });

      it("Pane chrome toggle folds and unfolds when onFoldToggle is wired", () => {
        let folded = true;
        const onFoldToggle = vi.fn(() => {
          folded = !folded;
        });
        const { container, rerender } = render(
          <PaneHost>
            <Pane id="fold-pane" anchor="bottom-right" icon="camera" label={uiDataLabel("Projection")} folded={folded} onFoldToggle={onFoldToggle}>
              <div data-testid="fold-pane-content">Modes</div>
            </Pane>
          </PaneHost>,
        );
        expect(screen.queryByTestId("fold-pane-content")).toBeNull();
        expect(container.querySelector('[data-slot="pane"]')?.getAttribute("data-anchor")).toBe("bottom-right");
        fireEvent.click(container.querySelector('[data-slot="window-pane-chrome-toggle"]')!);
        expect(onFoldToggle).toHaveBeenCalledTimes(1);
        rerender(
          <PaneHost>
            <Pane id="fold-pane" anchor="bottom-right" icon="camera" label={uiDataLabel("Projection")} folded={folded} onFoldToggle={onFoldToggle}>
              <div data-testid="fold-pane-content">Modes</div>
            </Pane>
          </PaneHost>,
        );
        expect(screen.getByTestId("fold-pane-content")).toBeTruthy();
        fireEvent.click(container.querySelector('[data-slot="window-pane-chrome-toggle"]')!);
        expect(onFoldToggle).toHaveBeenCalledTimes(2);
        rerender(
          <PaneHost>
            <Pane id="fold-pane" anchor="bottom-right" icon="camera" label={uiDataLabel("Projection")} folded={folded} onFoldToggle={onFoldToggle}>
              <div data-testid="fold-pane-content">Modes</div>
            </Pane>
          </PaneHost>,
        );
        expect(screen.queryByTestId("fold-pane-content")).toBeNull();
      });

      it("Pane isolates toggle pointer events from a pointer-capturing canvas host", () => {
        const hostPointerDown = vi.fn();
        const onFoldToggle = vi.fn();
        const { container } = render(
          <UiDriverProvider driver={COMPACT_UI_DRIVER}>
            <div onPointerDown={hostPointerDown}>
              <PaneHost>
                <Pane id="projection-pane" anchor="bottom-right" icon="camera" label={uiDataLabel("Projection")} folded onFoldToggle={onFoldToggle}>
                  <div>Modes</div>
                </Pane>
              </PaneHost>
            </div>
          </UiDriverProvider>,
        );
        const toggle = container.querySelector('[data-slot="window-pane-chrome-toggle"]') as HTMLElement;
        fireEvent.pointerDown(toggle, { pointerId: 1, clientX: 10, clientY: 10 });
        fireEvent.pointerUp(toggle, { pointerId: 1, clientX: 10, clientY: 10 });
        fireEvent.click(toggle);
        expect(hostPointerDown).not.toHaveBeenCalled();
        expect(onFoldToggle).toHaveBeenCalledTimes(1);
      });

      it("Pane drag handle calls onAnchorChange once per anchor crossed, resolved against the PaneHost's bounds", () => {
        let anchor: Anchor = "top-left";
        const onAnchorChange = vi.fn((next: Anchor) => {
          anchor = next;
        });
        const { container, rerender } = render(
          <PaneHost>
            <Pane id="drag-pane" anchor={anchor} onAnchorChange={onAnchorChange} icon="box" label={uiDataLabel("Drag")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        const host = container.querySelector('[data-slot="pane-host"]') as HTMLElement;
        host.getBoundingClientRect = () => ({ left: 0, top: 0, width: 300, height: 300, right: 300, bottom: 300, x: 0, y: 0, toJSON: () => ({}) }) as DOMRect;
        const handle = container.querySelector('[data-slot="window-pane-chrome-toggle"] [data-slot="drag-handle"]') as HTMLElement;
        fireEvent.pointerDown(handle, { pointerId: 1, clientX: 10, clientY: 10 });
        fireEvent.pointerMove(handle, { pointerId: 1, clientX: 290, clientY: 290 });
        expect(onAnchorChange).toHaveBeenCalledWith("bottom-right");
        fireEvent.pointerUp(handle, { pointerId: 1, clientX: 290, clientY: 290 });
        rerender(
          <PaneHost>
            <Pane id="drag-pane" anchor={anchor} onAnchorChange={onAnchorChange} icon="box" label={uiDataLabel("Drag")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(container.querySelector('[data-slot="pane"]')?.getAttribute("data-anchor")).toBe("bottom-right");
      });

      it("Pane without onAnchorChange still renders a drag-handle affordance like panel toggles", () => {
        const { container } = render(
          <PaneHost>
            <Pane id="fixed-pane" anchor="bottom-left" icon="box" label={uiDataLabel("Fixed")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(container.querySelector('[data-slot="window-chrome-cap"] [data-slot="drag-handle"]')).toBeTruthy();
        expect(container.querySelector('[data-slot="window-chrome-cap"] [data-icon="box"]')).toBeTruthy();
      });

      it("under the compact driver, Pane renders no drag-handle and no label, but the whole toggle stays draggable", () => {
        let anchor: Anchor = "top-left";
        const onAnchorChange = vi.fn((next: Anchor) => {
          anchor = next;
        });
        const { container } = render(
          <UiDriverProvider driver={COMPACT_UI_DRIVER}>
            <PaneHost>
              <Pane id="drag-pane" anchor={anchor} onAnchorChange={onAnchorChange} icon="box" label={uiDataLabel("Drag")} folded={false}>
                <div>Content</div>
              </Pane>
            </PaneHost>
          </UiDriverProvider>,
        );
        const host = container.querySelector('[data-slot="pane-host"]') as HTMLElement;
        host.getBoundingClientRect = () => ({ left: 0, top: 0, width: 300, height: 300, right: 300, bottom: 300, x: 0, y: 0, toJSON: () => ({}) }) as DOMRect;
        expect(container.querySelector('[data-slot="window-pane-chrome-toggle"] [data-slot="drag-handle"]')).toBeNull();
        expect(container.querySelector('[data-slot="window-pane-chrome-toggle"]')?.textContent).not.toContain("Drag");
        const toggle = container.querySelector('[data-slot="window-pane-chrome-toggle"]') as HTMLElement;
        fireEvent.pointerDown(toggle, { pointerId: 1, clientX: 10, clientY: 10 });
        fireEvent.pointerMove(toggle, { pointerId: 1, clientX: 290, clientY: 290 });
        expect(onAnchorChange).toHaveBeenCalledWith("bottom-right");
        fireEvent.pointerUp(toggle, { pointerId: 1, clientX: 290, clientY: 290 });
      });

      it("Pane resize handle grows the pane on its inner edge and respects min/max size", () => {
        let size = 300;
        const onSizeChange = vi.fn((next: number) => {
          size = next;
        });
        const { container, rerender } = render(
          <PaneHost>
            <Pane id="resize-pane" anchor="top-left" icon="box" label={uiDataLabel("Resize")} resizable size={size} onSizeChange={onSizeChange} minSize={200} maxSize={600} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        const handle = container.querySelector('[data-slot="pane-resize-handle"]') as HTMLElement;
        expect(handle.className).toContain("right-0");
        fireEvent.pointerDown(handle, { pointerId: 2, clientX: 100, clientY: 0 });
        fireEvent.pointerMove(handle, { pointerId: 2, clientX: 150, clientY: 0 });
        expect(onSizeChange).toHaveBeenCalledWith(350);
        rerender(
          <PaneHost>
            <Pane id="resize-pane" anchor="top-left" icon="box" label={uiDataLabel("Resize")} resizable size={size} onSizeChange={onSizeChange} minSize={200} maxSize={600} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(container.querySelector('[data-slot="pane"]')?.getAttribute("style")).toContain("350px");
      });

      it("usePaneSlot portals its pane into the nearest PaneHost and renders nothing outside one", () => {
        function DeepChild() {
          return usePaneSlot(
            <Pane id="slotted-pane" anchor="bottom-right" icon="box" label={uiDataLabel("Slotted")} folded={false}>
              <div data-testid="slotted-content">Slotted</div>
            </Pane>,
          );
        }
        const { container: withHost } = render(
          <PaneHost>
            <DeepChild />
          </PaneHost>,
        );
        expect(withHost.querySelector('[data-slot="pane"][data-anchor="bottom-right"]')).toBeTruthy();
        expect(screen.getByTestId("slotted-content")).toBeTruthy();

        function OrphanDeepChild() {
          return usePaneSlot(
            <Pane id="orphan-pane" anchor="top-left" icon="box">
              {null}
            </Pane>,
          );
        }
        const { container: withoutHost } = render(<OrphanDeepChild />);
        expect(withoutHost.querySelector('[data-slot="pane"]')).toBeNull();
      });

      it("on mobile respects folded state — no drag or resize handles, fold toggle still works", () => {
        const onFoldToggle = vi.fn();
        const { container } = render(
          <UiMobileProvider mobile>
            <PaneHost>
              <Pane id="mobile-pane" anchor="bottom-right" icon="box" label={uiDataLabel("Mobile")} onAnchorChange={vi.fn()} resizable size={300} onSizeChange={vi.fn()} folded onFoldToggle={onFoldToggle}>
                <div data-testid="mobile-pane-content">Content</div>
              </Pane>
            </PaneHost>
          </UiMobileProvider>,
        );
        expect(container.querySelector('[data-slot="window-chrome-cap"] [data-slot="drag-handle"]')).toBeNull();
        expect(container.querySelector('[data-slot="pane-resize-handle"]')).toBeNull();
        expect(screen.queryByTestId("mobile-pane-content")).toBeNull();
        fireEvent.click(container.querySelector('[data-slot="window-pane-chrome-toggle"]')!);
        expect(onFoldToggle).toHaveBeenCalledTimes(1);
      });

      it("mirrors dir=rtl for right anchors with a fixed semantic icon and trailing drag handle, while the body carries no dir override", () => {
        const { container: leftContainer } = render(
          <PaneHost>
            <Pane id="left-pane" anchor="top-left" icon="box" label={uiDataLabel("Left")} folded={false}>
              <div data-testid="left-pane-content">Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(leftContainer.querySelector('[data-slot="pane"]')?.getAttribute("dir")).toBeNull();
        expect(leftContainer.querySelector('[data-slot="pane-body"]')?.getAttribute("dir")).toBeNull();
        expect(leftContainer.querySelector('[data-slot="pane"]')?.className).toContain("items-start");
        expect(leftContainer.querySelector('[data-slot="pane"]')?.className).not.toContain("items-end");
        expect(leftContainer.querySelector('[data-icon="box"]')).toBeTruthy();
        expect(leftContainer.querySelector('[data-slot="window-chrome-cap"] [data-slot="drag-handle"]')).toBeTruthy();

        const { container: rightContainer } = render(
          <PaneHost>
            <Pane id="right-pane" anchor="top-right" icon="camera" label={uiDataLabel("Right")} folded={false}>
              <div data-testid="right-pane-content">Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(rightContainer.querySelector('[data-slot="pane"]')?.getAttribute("dir")).toBe("rtl");
        expect(rightContainer.querySelector('[data-slot="pane-body"]')?.getAttribute("dir")).toBeNull();
        expect(rightContainer.querySelector('[data-slot="pane"]')?.className).toContain("items-start");
        expect(rightContainer.querySelector('[data-slot="pane"]')?.className).not.toContain("items-end");
        expect(rightContainer.querySelector('[data-icon="camera"]')).toBeTruthy();
        expect(rightContainer.querySelector('[data-slot="window-chrome-cap"] [data-slot="drag-handle"]')).toBeTruthy();
      });

      it("centers a middle anchor with items-center instead of items-start", () => {
        const { container } = render(
          <PaneHost>
            <Pane id="middle-pane" anchor="top-middle" icon="box" label={uiDataLabel("Middle")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(container.querySelector('[data-slot="pane"]')?.className).toContain("items-center");
      });
    });

    it("createEvenWindowLayout builds a row of stacks", () => {
      const layout = createEvenWindowLayout(["a", "b"]);
      expect(layout.kind).toBe("row");
      if (layout.kind === "row") {
        expect(layout.children).toHaveLength(2);
      }
    });
});
}
