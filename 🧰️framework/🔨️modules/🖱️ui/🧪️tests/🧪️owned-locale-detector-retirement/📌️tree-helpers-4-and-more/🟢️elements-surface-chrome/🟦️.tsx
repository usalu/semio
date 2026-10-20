/** 🟢️ The elements surface chrome phase of registerTreeHelpers4AndMoreTests. */
import type { registerTreeHelpersTests } from "../🔶️tree-helpers/🟦️.tsx";

/** 🔵 Registers the elements surface chrome tests of registerTreeHelpers4AndMoreTests. */
export function registerElementsSurfaceChromeTests({ describe, it, resetElementsSurfaceChromeForTests, applyElementsSurfaceChrome, DEFAULT_UI_DRIVER, expect, COMPACT_UI_DRIVER, vi, bootstrapElementsSurfaceChromeDocument, applyChromeRevealAtPoint, useCanvasAppearanceSync }: Pick<ReturnType<typeof registerTreeHelpersTests>, "describe" | "it" | "resetElementsSurfaceChromeForTests" | "applyElementsSurfaceChrome" | "DEFAULT_UI_DRIVER" | "expect" | "COMPACT_UI_DRIVER" | "vi" | "bootstrapElementsSurfaceChromeDocument" | "applyChromeRevealAtPoint" | "useCanvasAppearanceSync">) {
    describe("ElementsSurfaceChrome", () => {
      it("keeps dark class while nested leases are active", () => {
        resetElementsSurfaceChromeForTests();
        const outer = applyElementsSurfaceChrome({ appearance: "dark", device: "desktop", driver: DEFAULT_UI_DRIVER });
        const inner = applyElementsSurfaceChrome({ appearance: "light", device: "desktop", driver: DEFAULT_UI_DRIVER });
        expect(document.documentElement.classList.contains("dark")).toBe(false);
        inner();
        expect(document.documentElement.classList.contains("dark")).toBe(true);
        outer();
        expect(document.documentElement.classList.contains("dark")).toBe(true);
        resetElementsSurfaceChromeForTests();
        expect(document.documentElement.classList.contains("dark")).toBe(false);
      });
  
      it("reapplies the previous lease after the top lease releases", () => {
        resetElementsSurfaceChromeForTests();
        const outer = applyElementsSurfaceChrome({ appearance: "dark", device: "desktop", driver: DEFAULT_UI_DRIVER });
        const inner = applyElementsSurfaceChrome({ appearance: "dark", device: "tablet", driver: COMPACT_UI_DRIVER });
        expect(document.documentElement.dataset.uiDevice).toBe("tablet");
        expect(document.documentElement.dataset.uiDriver).toBe("compact");
        inner();
        expect(document.documentElement.dataset.uiDevice).toBe("desktop");
        expect(document.documentElement.dataset.uiDriver).toBe("default");
        outer();
        resetElementsSurfaceChromeForTests();
      });
  
      it("defers clearing dark until the next animation frame when the last lease releases", () => {
        resetElementsSurfaceChromeForTests();
        let scheduled: FrameRequestCallback | undefined;
        const raf = vi.spyOn(globalThis, "requestAnimationFrame").mockImplementation((callback: FrameRequestCallback) => {
          scheduled = callback;
          return 1;
        });
        const release = applyElementsSurfaceChrome({ appearance: "dark", device: "desktop", driver: DEFAULT_UI_DRIVER });
        release();
        expect(document.documentElement.classList.contains("dark")).toBe(true);
        expect(raf).toHaveBeenCalled();
        scheduled?.(0);
        expect(document.documentElement.classList.contains("dark")).toBe(false);
        raf.mockRestore();
        resetElementsSurfaceChromeForTests();
      });
  
      it("bootstrapElementsSurfaceChromeDocument applies explicit dark before leases exist", () => {
        resetElementsSurfaceChromeForTests();
        bootstrapElementsSurfaceChromeDocument("dark");
        expect(document.documentElement.classList.contains("dark")).toBe(true);
        expect(document.documentElement.dataset.uiAppearance).toBe("dark");
        resetElementsSurfaceChromeForTests();
      });
  
      it("applies touch chrome and the mobile device attribute for the mobile device", () => {
        resetElementsSurfaceChromeForTests();
        const release = applyElementsSurfaceChrome({ appearance: "light", device: "mobile", driver: DEFAULT_UI_DRIVER });
        expect(document.documentElement.classList.contains("touch")).toBe(true);
        expect(document.documentElement.dataset.uiDevice).toBe("mobile");
        release();
        resetElementsSurfaceChromeForTests();
      });
  
      it("clears touch chrome for the desktop device", () => {
        resetElementsSurfaceChromeForTests();
        const release = applyElementsSurfaceChrome({ appearance: "light", device: "desktop", driver: DEFAULT_UI_DRIVER });
        expect(document.documentElement.classList.contains("touch")).toBe(false);
        expect(document.documentElement.dataset.uiDevice).toBe("desktop");
        release();
        resetElementsSurfaceChromeForTests();
      });
  
      it("applies every driver axis attribute and clears them on release", async () => {
        resetElementsSurfaceChromeForTests();
        const release = applyElementsSurfaceChrome({ appearance: "light", device: "desktop", driver: COMPACT_UI_DRIVER });
        const root = document.documentElement;
        expect(root.dataset.uiDriver).toBe("compact");
        expect(root.dataset.uiLabels).toBe("icons");
        expect(root.dataset.uiDrag).toBe("surface");
        expect(root.dataset.uiChromeReveal).toBe("hover");
        expect(root.dataset.uiGumballReveal).toBe("hover");
        expect(root.dataset.uiTooltips).toBe("none");
        release();
        await new Promise((resolve) => requestAnimationFrame(resolve));
        expect(root.dataset.uiDriver).toBeUndefined();
        expect(root.dataset.uiLabels).toBeUndefined();
        expect(root.dataset.uiDrag).toBeUndefined();
        expect(root.dataset.uiChromeReveal).toBeUndefined();
        expect(root.dataset.uiGumballReveal).toBeUndefined();
        expect(root.dataset.uiTooltips).toBeUndefined();
        resetElementsSurfaceChromeForTests();
      });
  
      it("reveals a hover-reveal chrome region only while the pointer is near its edge band", () => {
        resetElementsSurfaceChromeForTests();
        const release = applyElementsSurfaceChrome({ appearance: "light", device: "desktop", driver: COMPACT_UI_DRIVER });
        const region = document.createElement("nav");
        region.dataset.uiRevealRegion = "navbar";
        region.getBoundingClientRect = () => ({ left: 0, right: 100, top: 200, bottom: 240, width: 100, height: 40 }) as DOMRect;
        document.body.appendChild(region);
        applyChromeRevealAtPoint(document.documentElement, 50, 2);
        expect(region.dataset.uiRevealed).toBe("true");
        applyChromeRevealAtPoint(document.documentElement, 50, 400);
        expect(region.dataset.uiRevealed).toBeUndefined();
        applyChromeRevealAtPoint(document.documentElement, 50, 220);
        expect(region.dataset.uiRevealed).toBe("true");
        document.body.removeChild(region);
        release();
        resetElementsSurfaceChromeForTests();
      });
  
      it("reveals a footer hover-reveal region near the bottom screen edge", () => {
        resetElementsSurfaceChromeForTests();
        const release = applyElementsSurfaceChrome({ appearance: "light", device: "desktop", driver: COMPACT_UI_DRIVER });
        const innerHeight = window.innerHeight;
        Object.defineProperty(window, "innerHeight", { value: 800, configurable: true });
        const region = document.createElement("footer");
        region.dataset.uiRevealRegion = "footer";
        region.getBoundingClientRect = () => ({ left: 0, right: 100, top: 760, bottom: 800, width: 100, height: 40 }) as DOMRect;
        document.body.appendChild(region);
        applyChromeRevealAtPoint(document.documentElement, 50, 795);
        expect(region.dataset.uiRevealed).toBe("true");
        applyChromeRevealAtPoint(document.documentElement, 50, 400);
        expect(region.dataset.uiRevealed).toBeUndefined();
        Object.defineProperty(window, "innerHeight", { value: innerHeight, configurable: true });
        document.body.removeChild(region);
        release();
        resetElementsSurfaceChromeForTests();
      });
  
      it("reveals a window-cap region when the pointer is near the stack top edge", () => {
        resetElementsSurfaceChromeForTests();
        const release = applyElementsSurfaceChrome({ appearance: "light", device: "desktop", driver: COMPACT_UI_DRIVER });
        const stack = document.createElement("div");
        stack.setAttribute("data-slot", "window-chrome-stack");
        stack.getBoundingClientRect = () => ({ left: 100, right: 400, top: 50, bottom: 350, width: 300, height: 300 }) as DOMRect;
        const cap = document.createElement("div");
        cap.dataset.uiRevealRegion = "window-cap";
        cap.getBoundingClientRect = () => ({ left: 100, right: 400, top: 50, bottom: 50, width: 300, height: 0 }) as DOMRect;
        stack.appendChild(cap);
        document.body.appendChild(stack);
        applyChromeRevealAtPoint(document.documentElement, 250, 55);
        expect(cap.dataset.uiRevealed).toBe("true");
        applyChromeRevealAtPoint(document.documentElement, 250, 100);
        expect(cap.dataset.uiRevealed).toBeUndefined();
        applyChromeRevealAtPoint(document.documentElement, 250, 58);
        expect(cap.dataset.uiRevealed).toBe("true");
        document.body.removeChild(stack);
        release();
        resetElementsSurfaceChromeForTests();
      });
  
      it("reveals a hover-reveal region on focus and clears on blur", () => {
        resetElementsSurfaceChromeForTests();
        const release = applyElementsSurfaceChrome({ appearance: "light", device: "desktop", driver: COMPACT_UI_DRIVER });
        const region = document.createElement("nav");
        region.dataset.uiRevealRegion = "navbar";
        const button = document.createElement("button");
        region.appendChild(button);
        document.body.appendChild(region);
        button.dispatchEvent(new FocusEvent("focusin", { bubbles: true }));
        expect(region.dataset.uiRevealed).toBe("true");
        button.dispatchEvent(new FocusEvent("focusout", { bubbles: true, relatedTarget: null }));
        expect(region.dataset.uiRevealed).toBeUndefined();
        document.body.removeChild(region);
        release();
        resetElementsSurfaceChromeForTests();
      });
  
      it("useCanvasAppearanceSync does not re-run sync when the callback identity changes each render", async () => {
        const { render } = await import("@testing-library/react");
        const sync = vi.fn();
        function Harness({ n }: { readonly n: number }) {
          useCanvasAppearanceSync(() => {
            sync(n);
          });
          return <span data-testid="harness">{n}</span>;
        }
        const { rerender } = render(<Harness n={1} />);
        expect(sync).toHaveBeenCalledTimes(1);
        rerender(<Harness n={2} />);
        rerender(<Harness n={3} />);
        expect(sync).toHaveBeenCalledTimes(1);
        document.documentElement.dataset.uiAppearance = "dark";
        await vi.waitFor(() => expect(sync.mock.calls.length).toBeGreaterThan(1));
        delete document.documentElement.dataset.uiAppearance;
      });
    });
}
