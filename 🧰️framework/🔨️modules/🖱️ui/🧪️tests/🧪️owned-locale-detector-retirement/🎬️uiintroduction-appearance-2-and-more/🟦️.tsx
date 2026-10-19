import type { IntroductionStepDefinition } from "@semio-tech/ui-react/source";
import * as ui from "@semio-tech/ui-react/source";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerUiintroductionAppearance2AndMoreTests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { CELEBRATE_STAMP_DURATION_MS, INTRODUCTION_INFO_BOX_GAP_PX, UIIntroduction, clampIntroductionInfoBoxPosition, introductionDemoResolveVisual, introductionPointRelativeToHost, introductionRectRelativeToHost, ndcToViewportPoint, polylinePointAt, registerIntroductionSurfaceResolver, resolveIntroductionPlacement, resolveIntroductionPoint, sampleBezierSegments } = dependencies;
  type Anchor = any;
  type IntroductionStepDefinition = any;
  const { describe, expect, it, vi } = vitest;
  const { render, screen, fireEvent, act } = await import("@testing-library/react");

describe("UIIntroduction appearance", () => {

    it("renders step logos, wrapping only those with an href, and swaps light/dark srcs", () => {
      const { container } = render(
        <UIIntroduction
          introduction={{
            title: "Welcome",
            steps: [
              {
                id: "funding",
                title: "Funding",
                body: "Funded by",
                introduce: null,
                show: [],
                placement: "center",
                interactions: [],
                ordered: false,
                logos: [
                  { src: "/🖼️assets/logo/🖼️bbsr.png", darkSrc: "/🖼️assets/logo/🖼️bbsr-dark.png", alt: "BBSR", href: "https://www.bbsr.bund.de" },
                  { src: "/🖼️assets/logo/🖼️zukunft-bau.png", darkSrc: null, alt: "Zukunft Bau", href: null },
                ],
                demonstrations: [],
              },
            ],
          }}
          stepIndex={0}
          onStepIndexChange={vi.fn()}
          onDismiss={vi.fn()}
        />,
      );
      const box = container.querySelector('[data-slot="introduction-info-box"]');
      const links = box?.querySelectorAll("a[href]");
      expect(links).toHaveLength(1);
      expect(links?.[0].getAttribute("href")).toBe("https://www.bbsr.bund.de");
      const images = box?.querySelectorAll("img");
      expect(images).toHaveLength(3);
      expect(Array.from(images ?? []).map((img) => img.getAttribute("src"))).toEqual(["/🖼️assets/logo/🖼️bbsr.png", "/🖼️assets/logo/🖼️bbsr-dark.png", "/🖼️assets/logo/🖼️zukunft-bau.png"]);
    });

    it("renders an interaction checklist instead of the Next button, ticking off completed rows", () => {
      const steps: IntroductionStepDefinition[] = [
        {
          id: "viewport",
          title: "Viewport",
          body: "Navigate.",
          introduce: null,
          show: [],
          placement: "center",
          interactions: [
            { on: { kind: "zoom", id: "puzzle3d-main" }, label: "Zoom" },
            { on: { kind: "pan", id: "puzzle3d-main" }, label: "Pan" },
            { on: { kind: "orbit", id: "puzzle3d-main" }, label: "Orbit" },
          ],
          ordered: false,
          logos: [],
          demonstrations: [],
        },
      ];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} completedInteractionIndices={[1]} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const rows = container.querySelectorAll('[data-slot="introduction-interactions"] li');
      expect(rows).toHaveLength(3);
      expect(Array.from(rows).map((row) => row.querySelector("[data-slot='introduction-interaction-label']")?.textContent)).toEqual(["Zoom", "Pan", "Orbit"]);
      expect(rows[0].getAttribute("data-completed")).toBeNull();
      expect(rows[1].getAttribute("data-completed")).toBe("true");
      expect(rows[2].getAttribute("data-completed")).toBeNull();
      expect(screen.queryByRole("button", { name: /next|done/i })).toBeNull();
    });

    it("celebrates a checklist row's own label text the instant its interaction flips to done, and only that row", () => {
      vi.useFakeTimers();
      try {
        const steps: IntroductionStepDefinition[] = [
          {
            id: "viewport",
            title: "Viewport",
            body: "Navigate.",
            introduce: null,
            show: [],
            placement: "center",
            interactions: [
              { on: { kind: "zoom", id: "puzzle3d-main" }, label: "Zoom" },
              { on: { kind: "pan", id: "puzzle3d-main" }, label: "Pan" },
            ],
            ordered: false,
            logos: [],
            demonstrations: [],
          },
        ];
        const { container, rerender } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} completedInteractionIndices={[]} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
        const labels = () => container.querySelectorAll('[data-slot="introduction-interaction-label"]');
        expect(labels()[0]?.getAttribute("data-celebrated")).toBeNull();
        expect(labels()[1]?.getAttribute("data-celebrated")).toBeNull();

        rerender(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} completedInteractionIndices={[0]} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
        expect(labels()[0]?.getAttribute("data-celebrated")).toBe("true");
        expect(labels()[1]?.getAttribute("data-celebrated")).toBeNull();

        act(() => {
          vi.advanceTimersByTime(CELEBRATE_STAMP_DURATION_MS);
        });
        expect(labels()[0]?.getAttribute("data-celebrated")).toBeNull();
      } finally {
        vi.useRealTimers();
      }
    });

    it("numbers checklist rows when the step is ordered, and omits numbers when it isn't", () => {
      const orderedSteps: IntroductionStepDefinition[] = [
        {
          id: "viewport",
          title: "Viewport",
          body: "Navigate in order.",
          introduce: null,
          show: [],
          placement: "center",
          interactions: [
            { on: { kind: "zoom", id: "puzzle3d-main" }, label: "Zoom" },
            { on: { kind: "pan", id: "puzzle3d-main" }, label: "Pan" },
          ],
          ordered: true,
          logos: [],
          demonstrations: [],
        },
      ];
      const { container, rerender } = render(<UIIntroduction introduction={{ title: "Welcome", steps: orderedSteps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const orderedRows = container.querySelectorAll('[data-slot="introduction-interactions"] li');
      expect(orderedRows[0].querySelector("[data-slot='introduction-interaction-index']")?.textContent).toBe("1.");
      expect(orderedRows[0].querySelector("[data-slot='introduction-interaction-label']")?.textContent).toBe("Zoom");
      expect(orderedRows[1].querySelector("[data-slot='introduction-interaction-index']")?.textContent).toBe("2.");
      expect(orderedRows[1].querySelector("[data-slot='introduction-interaction-label']")?.textContent).toBe("Pan");

      const unorderedSteps: IntroductionStepDefinition[] = [{ ...orderedSteps[0], ordered: false }];
      rerender(<UIIntroduction introduction={{ title: "Welcome", steps: unorderedSteps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const unorderedRows = container.querySelectorAll('[data-slot="introduction-interactions"] li');
      expect(unorderedRows[0].querySelector("[data-slot='introduction-interaction-index']")).toBeNull();
      expect(unorderedRows[0].querySelector("[data-slot='introduction-interaction-label']")?.textContent).toBe("Zoom");
      expect(unorderedRows[1].querySelector("[data-slot='introduction-interaction-label']")?.textContent).toBe("Pan");
    });

    it("renders the Next button and no checklist when the step has no interactions", () => {
      const steps: IntroductionStepDefinition[] = [{ id: "welcome", title: "Welcome", body: "Hi.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      expect(container.querySelector('[data-slot="introduction-interactions"]')).toBeNull();
      expect(screen.getByRole("button", { name: /next|done|weiter|fertig/i })).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-footer-right"]')).toBeTruthy();
    });

    it("grows the introduction window for complete step titles and wraps only at the viewport boundary", () => {
      const title = "Willkommen bei Entwerfen mit Bestand";
      const steps: IntroductionStepDefinition[] = [{ id: "welcome", title, body: "Hi.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const box = container.querySelector('[data-slot="introduction-info-box"]') as HTMLElement;
      const chip = container.querySelector('[data-slot="introduction-info-box-chip"]') as HTMLElement;
      const stepTitle = container.querySelector('[data-slot="introduction-info-box-title"]') as HTMLElement;
      const content = container.querySelector('[data-slot="introduction-info-box-content"]') as HTMLElement;
      expect(stepTitle.textContent).toBe(title);
      expect(stepTitle.className).not.toContain("truncate");
      expect(stepTitle.className).toContain("break-words");
      expect(stepTitle.className).toContain("whitespace-normal");
      expect(chip.className).toContain("max-w-none");
      expect(chip.className).not.toContain("max-w-[12rem]");
      expect(chip.className.split(" ")).toContain("shrink");
      expect(chip.className.split(" ")).not.toContain("shrink-0");
      expect(chip.className.split(" ")).toContain("!h-auto");
      expect(chip.className.split(" ")).toContain("overflow-visible");
      expect(chip.className.split(" ")).not.toContain("overflow-hidden");
      expect(box.className).toContain("w-fit");
      expect(box.className).toContain("max-w-[calc(100vw-2rem)]");
      expect(content.className).toContain("max-w-sm");
    });

    it("renders a header drag handle in the title chip and moves the info box", () => {
      const steps: IntroductionStepDefinition[] = [{ id: "welcome", title: "Welcome", body: "Hi.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const box = container.querySelector('[data-slot="introduction-info-box"]') as HTMLElement;
      const chip = container.querySelector('[data-slot="introduction-info-box-chip"]') as HTMLElement;
      const dragRail = container.querySelector('[data-slot="introduction-info-box-drag"]');
      const handle = dragRail?.querySelector('[data-slot="drag-handle"]') as HTMLElement;
      expect(chip).toBeTruthy();
      expect(dragRail).toBeTruthy();
      expect(handle).toBeTruthy();
      expect(chip.contains(dragRail)).toBe(true);
      expect(chip.textContent).toContain("Welcome");
      expect(container.querySelector('[data-slot="introduction-close"]')).toBeTruthy();
      expect(container.textContent).toMatch(/1\s*\/\s*1/);
      const startTop = Number.parseFloat(box.style.top);
      const startLeft = Number.parseFloat(box.style.left);
      fireEvent.pointerDown(handle, { pointerId: 1, pointerType: "mouse", clientX: 100, clientY: 100 });
      fireEvent.pointerMove(handle, { pointerId: 1, pointerType: "mouse", clientX: 140, clientY: 160 });
      expect(box.getAttribute("data-dragging")).toBe("true");
      expect(Number.parseFloat(box.style.top)).toBe(startTop + 60);
      expect(Number.parseFloat(box.style.left)).toBe(startLeft + 40);
      fireEvent.pointerMove(handle, { pointerId: 1, pointerType: "mouse", clientX: -10_000, clientY: -10_000 });
      expect(Number.parseFloat(box.style.top)).toBe(0);
      expect(Number.parseFloat(box.style.left)).toBe(0);
      fireEvent.pointerMove(handle, { pointerId: 1, pointerType: "mouse", clientX: 10_000, clientY: 10_000 });
      expect(Number.parseFloat(box.style.top)).toBe(window.innerHeight - box.offsetHeight);
      expect(Number.parseFloat(box.style.left)).toBe(window.innerWidth - box.offsetWidth);
      fireEvent.pointerUp(handle, { pointerId: 1, pointerType: "mouse", clientX: 10_000, clientY: 10_000 });
      expect(box.getAttribute("data-dragging")).toBeNull();
      expect(Number.parseFloat(box.style.top)).toBe(window.innerHeight - box.offsetHeight);
      expect(Number.parseFloat(box.style.left)).toBe(window.innerWidth - box.offsetWidth);
    });

    it("places pressable Back and Next controls outside the transparent side gaps' hit-testing and accessibility suppression", () => {
      const steps: IntroductionStepDefinition[] = [
        { id: "first", title: "First", body: "Step one.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
        { id: "second", title: "Second", body: "Step two.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
        { id: "third", title: "Third", body: "Step three.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
      ];
      const onStepIndexChange = vi.fn();
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={1} onStepIndexChange={onStepIndexChange} onDismiss={vi.fn()} />);
      const footer = container.querySelector('[data-slot="window-chrome-footer"]');
      const footerGapLeft = container.querySelector('[data-slot="window-chrome-footer-gap-left"]');
      const footerGapRight = container.querySelector('[data-slot="window-chrome-footer-gap-right"]');
      const footerCenter = container.querySelector('[data-slot="window-chrome-footer-center"]');
      const footerLeft = container.querySelector('[data-slot="window-chrome-footer-left"]');
      const footerRight = container.querySelector('[data-slot="window-chrome-footer-right"]');
      expect(footer).toBeTruthy();
      expect(footerGapLeft).toBeTruthy();
      expect(footerGapRight).toBeTruthy();
      expect(footerCenter).toBeTruthy();
      expect(footerLeft).toBeTruthy();
      expect(footerRight).toBeTruthy();
      expect(footerCenter?.querySelector('[data-slot="window-chrome-footer-center-chip"]')?.textContent).toMatch(/2\s*\/\s*3/);
      expect(footerGapLeft?.getAttribute("aria-hidden")).toBeNull();
      expect(footerGapRight?.getAttribute("aria-hidden")).toBeNull();
      expect(footerLeft?.classList.contains("pointer-events-auto")).toBe(true);
      expect(footerRight?.classList.contains("pointer-events-auto")).toBe(true);
      expect(footer?.className).toContain("grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]");
      const backButton = screen.getByRole("button", { name: /back|zurück/i });
      const nextButton = screen.getByRole("button", { name: /next|weiter/i });
      expect(backButton.className).toContain("hover:bg-hover-interactive-fill");
      expect(nextButton.className).toContain("hover:bg-hover-interactive-fill");
      fireEvent.click(backButton);
      fireEvent.click(nextButton);
      expect(onStepIndexChange).toHaveBeenNthCalledWith(1, 0);
      expect(onStepIndexChange).toHaveBeenNthCalledWith(2, 2);
      expect(container.querySelector('[data-slot="introduction-info-box"] [data-slot="window-chrome-body"]')?.textContent).not.toMatch(/next|done|back|zurück|\d\s*\/\s*\d/i);
    });

    it("centers the step chip when navigation occupies either side alone", () => {
      const rightOnlySteps: IntroductionStepDefinition[] = [{ id: "only", title: "Only", body: "Step one.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
      const { container, rerender } = render(<UIIntroduction introduction={{ title: "Welcome", steps: rightOnlySteps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      expect(container.querySelector('[data-slot="window-chrome-footer-gap-left"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-footer-gap-right"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-footer-center-chip"]')?.textContent).toMatch(/1\s*\/\s*1/);
      expect(container.querySelector('[data-slot="window-chrome-footer-left"]')).toBeNull();
      expect(container.querySelector('[data-slot="window-chrome-footer-right"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-footer"]')?.className).toContain("grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]");

      const leftOnlySteps: IntroductionStepDefinition[] = [
        { id: "first", title: "First", body: "Step one.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
        {
          id: "second",
          title: "Second",
          body: "Complete the step.",
          introduce: null,
          show: [],
          placement: "center",
          interactions: [{ on: { kind: "action", id: "ui.footer" }, label: "Click" }],
          ordered: false,
          logos: [],
          demonstrations: [],
        },
      ];
      rerender(<UIIntroduction introduction={{ title: "Welcome", steps: leftOnlySteps }} stepIndex={1} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      expect(container.querySelector('[data-slot="window-chrome-footer-center-chip"]')?.textContent).toMatch(/2\s*\/\s*2/);
      expect(container.querySelector('[data-slot="window-chrome-footer-left"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-footer-right"]')).toBeNull();
      expect(container.querySelector('[data-slot="window-chrome-footer"]')?.className).toContain("grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]");
    });

    it("does not paint glass on the footer center rail when the next chip is absent", () => {
      const steps: IntroductionStepDefinition[] = [
        {
          id: "interact",
          title: "Interact",
          body: "Complete the step.",
          introduce: null,
          show: [],
          placement: "center",
          interactions: [{ on: { kind: "action", id: "ui.footer" }, label: "Click" }],
          ordered: false,
          logos: [],
          demonstrations: [],
        },
      ];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const centerRail = container.querySelector('[data-slot="window-chrome-footer-center"]');
      const centerChip = container.querySelector('[data-slot="window-chrome-footer-center-chip"]');
      expect(centerRail?.className).not.toMatch(/ui-glass-chrome/);
      expect(centerChip?.className).toMatch(/ui-glass/);
      expect(container.querySelector('[data-slot="window-chrome-footer-right"]')).toBeNull();
      expect(container.querySelector('[data-slot="window-chrome-footer"]')?.className).toContain("grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]");
    });
});

  describe("introductionRectRelativeToHost", () => {
    it("subtracts the host origin from a viewport rect so absolute overlays survive CSS-transformed shells", () => {
      expect(introductionRectRelativeToHost({ top: 120, left: 80, width: 40, height: 20 }, { top: 100, left: 50, width: 400, height: 300 })).toEqual({
        top: 20,
        left: 30,
        width: 40,
        height: 20,
      });
    });
  });

  describe("introductionPointRelativeToHost", () => {
    it("subtracts the host origin from a viewport point", () => {
      expect(introductionPointRelativeToHost({ x: 250, y: 180 }, { top: 100, left: 50, width: 400, height: 300 })).toEqual({ x: 200, y: 80 });
    });
  });

  describe("clampIntroductionInfoBoxPosition", () => {
    const viewport = { width: 800, height: 600 };
    const boxSize = { width: 100, height: 50 };

    it("lets direct manipulation meet every viewport border exactly", () => {
      expect(clampIntroductionInfoBoxPosition({ top: -100, left: -100 }, boxSize, viewport, 0)).toEqual({ top: 0, left: 0 });
      expect(clampIntroductionInfoBoxPosition({ top: 1_000, left: 1_000 }, boxSize, viewport, 0)).toEqual({ top: 550, left: 700 });
    });

    it("preserves the authored placement inset", () => {
      expect(clampIntroductionInfoBoxPosition({ top: -100, left: -100 }, boxSize, viewport, INTRODUCTION_INFO_BOX_GAP_PX)).toEqual({ top: 16, left: 16 });
      expect(clampIntroductionInfoBoxPosition({ top: 1_000, left: 1_000 }, boxSize, viewport, INTRODUCTION_INFO_BOX_GAP_PX)).toEqual({ top: 534, left: 684 });
    });
  });

  describe("resolveIntroductionPlacement", () => {
    const viewport = { width: 800, height: 600 };
    const boxSize = { width: 100, height: 50 };

    it("centers when there is no anchor rect, or placement is center", () => {
      expect(resolveIntroductionPlacement("auto", null, boxSize, viewport)).toEqual({ top: 275, left: 350 });
      expect(resolveIntroductionPlacement("center", { top: 10, left: 10, width: 20, height: 20 }, boxSize, viewport)).toEqual({ top: 275, left: 350 });
    });

    it("keeps a centered box inside the viewport, so its Skip and Next controls stay clickable", () => {
      const oversized = resolveIntroductionPlacement("center", null, { width: 1585, height: 1041 }, { width: 1425, height: 885 });
      expect(oversized.top).toBeGreaterThanOrEqual(0);
      expect(oversized.left).toBeGreaterThanOrEqual(0);
      const tiny = resolveIntroductionPlacement("auto", null, boxSize, { width: 0, height: 0 });
      expect(tiny.top).toBeGreaterThanOrEqual(0);
      expect(tiny.left).toBeGreaterThanOrEqual(0);
    });

    it("auto picks the side with the most free space", () => {
      const nearTop = { top: 0, left: 350, width: 100, height: 20 };
      const placedBelow = resolveIntroductionPlacement("auto", nearTop, boxSize, viewport);
      expect(placedBelow.top).toBeGreaterThan(nearTop.top + nearTop.height);
    });

    it("explicit placements position relative to the anchor and stay within the viewport", () => {
      const anchor = { top: 275, left: 350, width: 100, height: 20 };
      const right = resolveIntroductionPlacement("right", anchor, boxSize, viewport);
      expect(right.left).toBeGreaterThan(anchor.left + anchor.width);
      const left = resolveIntroductionPlacement("left", anchor, boxSize, viewport);
      expect(left.left).toBeLessThan(anchor.left);
      const nearEdge = resolveIntroductionPlacement("right", { top: 10, left: 780, width: 10, height: 10 }, boxSize, viewport);
      expect(nearEdge.left).toBeLessThanOrEqual(viewport.width - boxSize.width);
    });
  });

  describe("resolveIntroductionPoint", () => {
    it("resolves an element point to its rect center, or a normalized offset within it", () => {
      const { container } = render(<div id="demo.target" />);
      const el = container.querySelector("#demo\\.target")!;
      vi.spyOn(el, "getBoundingClientRect").mockReturnValue({ left: 100, top: 200, width: 100, height: 50, right: 200, bottom: 250, x: 100, y: 200, toJSON: () => ({}) } as DOMRect);
      expect(resolveIntroductionPoint({ kind: "element", id: "demo.target" })).toEqual({ x: 150, y: 225 });
      expect(resolveIntroductionPoint({ kind: "element", id: "demo.target", offset: [0, 0] })).toEqual({ x: 100, y: 200 });
    });

    it("resolves absolute and normalized screen points against the viewport", () => {
      const originalWidth = window.innerWidth;
      const originalHeight = window.innerHeight;
      Object.defineProperty(window, "innerWidth", { value: 800, configurable: true });
      Object.defineProperty(window, "innerHeight", { value: 600, configurable: true });
      expect(resolveIntroductionPoint({ kind: "screen", x: 10, y: 20 })).toEqual({ x: 10, y: 20 });
      expect(resolveIntroductionPoint({ kind: "screenNormalized", x: 0.5, y: 0.5 })).toEqual({ x: 400, y: 300 });
      Object.defineProperty(window, "innerWidth", { value: originalWidth, configurable: true });
      Object.defineProperty(window, "innerHeight", { value: originalHeight, configurable: true });
    });

    it("resolves window-local absolute and normalized points against the target's rect", () => {
      const { container } = render(<div id="demo.window" />);
      const el = container.querySelector("#demo\\.window")!;
      vi.spyOn(el, "getBoundingClientRect").mockReturnValue({ left: 50, top: 60, width: 200, height: 100, right: 250, bottom: 160, x: 50, y: 60, toJSON: () => ({}) } as DOMRect);
      expect(resolveIntroductionPoint({ kind: "window", id: "demo.window", x: 10, y: 20 })).toEqual({ x: 60, y: 80 });
      expect(resolveIntroductionPoint({ kind: "windowNormalized", id: "demo.window", x: 0.5, y: 0.5 })).toEqual({ x: 150, y: 110 });
    });

    it("resolves a scene point through its registered resolver, and null once off-camera or unregistered", () => {
      const unregister = registerIntroductionSurfaceResolver("✏️demo.scene", {
        scenePoint: (position) => (position[0] > 0 ? { x: 42, y: 84, visible: true } : { x: 0, y: 0, visible: false }),
      });
      expect(resolveIntroductionPoint({ kind: "scene", id: "✏️demo.scene", position: [1, 0, 0] })).toEqual({ x: 42, y: 84 });
      expect(resolveIntroductionPoint({ kind: "scene", id: "✏️demo.scene", position: [-1, 0, 0] })).toBeNull();
      unregister();
      expect(resolveIntroductionPoint({ kind: "scene", id: "✏️demo.scene", position: [1, 0, 0] })).toBeNull();
    });

    it("resolves a canvas (2D world) point through its registered resolver", () => {
      const unregister = registerIntroductionSurfaceResolver("demo.canvas", {
        canvasPoint: (x, y) => ({ x: x * 2, y: y * 2, visible: true }),
      });
      expect(resolveIntroductionPoint({ kind: "canvas", id: "demo.canvas", x: 10, y: 20 })).toEqual({ x: 20, y: 40 });
      unregister();
      expect(resolveIntroductionPoint({ kind: "canvas", id: "demo.canvas", x: 10, y: 20 })).toBeNull();
    });

    it("resolves an entity point centered, or at a normalized offset within its rect, and a wildcard entity id", () => {
      const unregister = registerIntroductionSurfaceResolver("demo.entities", {
        entity: (domain, entityId) => {
          if (domain !== "vortex") return null;
          if (entityId === "*") return { point: { x: 5, y: 5 }, visible: true };
          if (entityId !== "obj:v0") return null;
          return { point: { x: 100, y: 100 }, rect: { x: 80, y: 90, width: 40, height: 20 }, visible: true };
        },
      });
      expect(resolveIntroductionPoint({ kind: "entity", id: "demo.entities", domain: "vortex", entity: "obj:v0" })).toEqual({ x: 100, y: 100 });
      expect(resolveIntroductionPoint({ kind: "entity", id: "demo.entities", domain: "vortex", entity: "obj:v0", offset: [0, 0] })).toEqual({ x: 80, y: 90 });
      expect(resolveIntroductionPoint({ kind: "entity", id: "demo.entities", domain: "vortex", entity: "*" })).toEqual({ x: 5, y: 5 });
      expect(resolveIntroductionPoint({ kind: "entity", id: "demo.entities", domain: "vortex", entity: "missing" })).toBeNull();
      expect(resolveIntroductionPoint({ kind: "entity", id: "demo.entities", domain: "edge", entity: "*" })).toBeNull();
      unregister();
    });

    it("resolves a curve point by arc-length t along an entity's polyline", () => {
      const unregister = registerIntroductionSurfaceResolver("demo.curve", {
        entity: (domain, entityId) => {
          if (domain !== "attraction" || entityId !== "a1") return null;
          return {
            point: { x: 50, y: 0 },
            polyline: [
              { x: 0, y: 0 },
              { x: 100, y: 0 },
            ],
            visible: true,
          };
        },
      });
      expect(resolveIntroductionPoint({ kind: "curve", id: "demo.curve", domain: "attraction", entity: "a1", t: 0 })).toEqual({ x: 0, y: 0 });
      expect(resolveIntroductionPoint({ kind: "curve", id: "demo.curve", domain: "attraction", entity: "a1", t: 0.5 })).toEqual({ x: 50, y: 0 });
      expect(resolveIntroductionPoint({ kind: "curve", id: "demo.curve", domain: "attraction", entity: "missing", t: 0.5 })).toBeNull();
      unregister();
    });

    it("resolves a domain point by mapping value onto the entity's rect along its axis", () => {
      const unregister = registerIntroductionSurfaceResolver("demo.domain", {
        entity: (domain, entityId) => {
          if (domain !== "slider" || entityId !== "fillCount") return null;
          return { point: { x: 100, y: 50 }, rect: { x: 50, y: 40, width: 100, height: 20 }, domain: { min: 0, max: 10, axis: "x" }, visible: true };
        },
      });
      expect(resolveIntroductionPoint({ kind: "domain", id: "demo.domain", domain: "slider", entity: "fillCount", value: 0 })).toEqual({ x: 50, y: 50 });
      expect(resolveIntroductionPoint({ kind: "domain", id: "demo.domain", domain: "slider", entity: "fillCount", value: 10 })).toEqual({ x: 150, y: 50 });
      expect(resolveIntroductionPoint({ kind: "domain", id: "demo.domain", domain: "slider", entity: "fillCount", value: 5 })).toEqual({ x: 100, y: 50 });
      expect(resolveIntroductionPoint({ kind: "domain", id: "demo.domain", domain: "slider", entity: "fillCount", value: 999 })).toEqual({ x: 150, y: 50 });
      unregister();
    });

    it("returns null for an element/window point that hasn't mounted", () => {
      expect(resolveIntroductionPoint({ kind: "element", id: "nothing.here" })).toBeNull();
      expect(resolveIntroductionPoint({ kind: "window", id: "nothing.here", x: 0, y: 0 })).toBeNull();
    });
  });

  describe("polylinePointAt", () => {
    it("interpolates by arc length across a multi-segment polyline", () => {
      const points = [
        { x: 0, y: 0 },
        { x: 10, y: 0 },
        { x: 10, y: 10 },
      ];
      expect(polylinePointAt(points, 0)).toEqual({ x: 0, y: 0 });
      expect(polylinePointAt(points, 1)).toEqual({ x: 10, y: 10 });
      expect(polylinePointAt(points, 0.25)).toEqual({ x: 5, y: 0 });
      expect(polylinePointAt(points, 0.75)).toEqual({ x: 10, y: 5 });
    });

    it("clamps t outside [0, 1] and degrades gracefully for degenerate polylines", () => {
      const points = [
        { x: 0, y: 0 },
        { x: 10, y: 0 },
      ];
      expect(polylinePointAt(points, -1)).toEqual({ x: 0, y: 0 });
      expect(polylinePointAt(points, 2)).toEqual({ x: 10, y: 0 });
      expect(polylinePointAt([], 0.5)).toEqual({ x: 0, y: 0 });
      expect(polylinePointAt([{ x: 3, y: 4 }], 0.5)).toEqual({ x: 3, y: 4 });
      expect(
        polylinePointAt(
          [
            { x: 1, y: 1 },
            { x: 1, y: 1 },
          ],
          0.5,
        ),
      ).toEqual({ x: 1, y: 1 });
    });
  });

  describe("sampleBezierSegments", () => {
    it("samples move/line segments verbatim and quad/cubic curves through their control points", () => {
      const linePoints = sampleBezierSegments([
        { kind: "move", to: [0, 0] },
        { kind: "line", to: [10, 0] },
      ]);
      expect(linePoints).toEqual([
        { x: 0, y: 0 },
        { x: 10, y: 0 },
      ]);

      const quadPoints = sampleBezierSegments(
        [
          { kind: "move", to: [0, 0] },
          { kind: "quad", ctrl: [5, 10], to: [10, 0] },
        ],
        2,
      );
      expect(quadPoints).toHaveLength(3);
      expect(quadPoints[1].x).toBeCloseTo(5, 5);
      expect(quadPoints[1].y).toBeCloseTo(5, 5);
      expect(quadPoints[2]).toEqual({ x: 10, y: 0 });
    });

    it("ignores unsampleable segment kinds without throwing", () => {
      expect(sampleBezierSegments([{ kind: "close" }])).toEqual([]);
      expect(sampleBezierSegments([])).toEqual([]);
    });
  });

  describe("ndcToViewportPoint", () => {
    it("maps NDC corners and center to viewport pixels", () => {
      const rect = { left: 100, top: 50, width: 200, height: 100 };
      expect(ndcToViewportPoint({ x: 0, y: 0 }, rect)).toEqual({ x: 200, y: 100 });
      expect(ndcToViewportPoint({ x: -1, y: -1 }, rect)).toEqual({ x: 100, y: 150 });
      expect(ndcToViewportPoint({ x: 1, y: 1 }, rect)).toEqual({ x: 300, y: 50 });
    });
  });

  describe("introductionDemoResolveVisual", () => {
    it("maps each gesture kind to a unique button, modifier set, and feedback family", () => {
      const at = { kind: "screenNormalized" as const, x: 0.5, y: 0.5 };
      expect(introductionDemoResolveVisual({ kind: "leftClick", at })).toEqual({ button: "left", modifiers: [], feedback: "leftClick", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "rightClick", at })).toEqual({ button: "right", modifiers: [], feedback: "rightClick", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "doubleClick", at })).toEqual({ button: "left", modifiers: [], feedback: "doubleClick", showDoubleChip: true });
      expect(introductionDemoResolveVisual({ kind: "scroll", at, deltaY: -100 })).toEqual({ button: "wheel", modifiers: [], feedback: "scroll", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "drag", from: at, to: at })).toEqual({ button: "left", modifiers: [], feedback: "dragLeft", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "drag", from: at, to: at, button: "middle" })).toEqual({ button: "middle", modifiers: [], feedback: "dragMiddle", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "orbit", from: at, to: at })).toEqual({ button: "right", modifiers: ["alt"], feedback: "orbit", showDoubleChip: false });
    });
  });
}
