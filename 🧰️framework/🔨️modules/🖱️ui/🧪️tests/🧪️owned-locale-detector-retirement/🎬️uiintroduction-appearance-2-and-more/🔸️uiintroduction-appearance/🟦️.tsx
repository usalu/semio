/** 🔸️ The uiintroduction appearance phase of registerUiintroductionAppearance2AndMoreTests. */
import type { IntroductionStepDefinition } from "@semio-tech/ui-react/source";
import type { buildCelebrateStampDurationState } from "../🔶️celebrate-stamp-duration/🟦️.tsx";

/** 🔹 Registers the uiintroduction appearance tests of registerUiintroductionAppearance2AndMoreTests. */
export function registerUiintroductionAppearanceTests({ describe, it, render, UIIntroduction, vi, expect, screen, act, CELEBRATE_STAMP_DURATION_MS, fireEvent }: Pick<Awaited<ReturnType<typeof buildCelebrateStampDurationState>>, "describe" | "it" | "render" | "UIIntroduction" | "vi" | "expect" | "screen" | "act" | "CELEBRATE_STAMP_DURATION_MS" | "fireEvent">) {
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
}
