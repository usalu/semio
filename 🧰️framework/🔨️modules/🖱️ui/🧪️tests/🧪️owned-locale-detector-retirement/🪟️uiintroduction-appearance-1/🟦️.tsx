import type { IntroductionStepDefinition } from "@semio-tech/ui-react/source";
import type * as Three from "three";
import * as ui from "@semio-tech/ui-react/source";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerUiintroductionAppearance1Tests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { Footer, GLASS_OVERLAY_BOX_CLASS, React, UIIntroduction, WindowChrome, reactHostPort, splitIntroductionBodyParagraphs, windowSilhouettePath } = dependencies;
  type IntroductionStepDefinition = any;
  const { describe, expect, it, vi } = vitest;
  const { render, screen, fireEvent, waitFor } = await import("@testing-library/react");

describe("UIIntroduction appearance", () => {

    it("Done on the last step dismisses as completed; Skip dismisses as not completed", async () => {
      const onDismiss = vi.fn();
      const steps: IntroductionStepDefinition[] = [{ id: "only", title: "Only", body: "Last step.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
      const { unmount } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={onDismiss} />);
      fireEvent.click(screen.getByRole("button", { name: /done|fertig/i }));
      expect(onDismiss).toHaveBeenCalledWith(true);
      unmount();
      onDismiss.mockClear();
      render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={onDismiss} />);
      fireEvent.click(screen.getByRole("button", { name: /skip|überspringen/i }));
      expect(onDismiss).toHaveBeenCalledWith(false);
    });

    // 🪜️ D2 regression: the title chip, close cell, body, and footer chips of one introduction step
    // are all `data-level="dialog"` and must render the exact same fill without piecewise borders —
    // the WindowChrome silhouette owns their one continuous outline.
    it("renders title, close, body, and footer chips as one borderless dialog-level glass silhouette", () => {
      const steps: IntroductionStepDefinition[] = [
        { id: "first", title: "First", body: "Step one.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
        { id: "second", title: "Second", body: "Step two.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
      ];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={1} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const stack = container.querySelector('[data-slot="introduction-info-box"]') as HTMLElement;
      expect(stack.getAttribute("data-level")).toBe("dialog");

      const titleChip = container.querySelector('[data-slot="introduction-info-box-chip"]') as HTMLElement;
      const stepChip = container.querySelector('[data-slot="introduction-step-chip"]') as HTMLElement;
      expect(titleChip.className).not.toContain("ui-surface");
      expect(stepChip.className).not.toContain("ui-surface");
      expect(titleChip.className).toContain("border-0");
      expect(stepChip.className).toContain("border-0");

      const chipCap = container.querySelector('[data-slot="window-chrome-chip-cap"]') as HTMLElement;
      const controls = container.querySelector('[data-slot="window-chrome-controls"]') as HTMLElement;
      const body = container.querySelector('[data-slot="window-chrome-body"]') as HTMLElement;
      const footerLeft = container.querySelector('[data-slot="window-chrome-footer-left"]') as HTMLElement;
      const footerCenterChip = container.querySelector('[data-slot="window-chrome-footer-center-chip"]') as HTMLElement;
      const footerRight = container.querySelector('[data-slot="window-chrome-footer-right"]') as HTMLElement;
      for (const cell of [chipCap, controls, footerLeft, footerCenterChip, footerRight]) {
        expect(cell).toBeTruthy();
        expect(cell.className).toContain("ui-glass");
        expect(cell.className).not.toContain("ui-surface");
        expect(cell.className).not.toContain("bg-transparent");
      }
      expect(body.hasAttribute("data-window-silhouette-content")).toBe(true);
      expect(body.className).not.toContain("ui-glass");

      const backButtonGroup = container.querySelector('[id="ui.introduction.back"]')?.closest('[data-slot="button-group"]') as HTMLElement;
      const nextButtonGroup = container.querySelector('[id="ui.introduction.next"]')?.closest('[data-slot="button-group"]') as HTMLElement;
      const closeButton = container.querySelector('[data-slot="introduction-close"]') as HTMLElement;
      for (const flowingChip of [titleChip, stepChip, backButtonGroup, nextButtonGroup, closeButton]) {
        expect(flowingChip).toBeTruthy();
        expect(flowingChip.parentElement?.hasAttribute("data-window-silhouette-chip")).toBe(true);
        expect(flowingChip.getAttribute("data-level") ?? flowingChip.closest("[data-level]")?.getAttribute("data-level")).toBe("dialog");
      }

      const gap = container.querySelector('[data-slot="window-chrome-gap"]') as HTMLElement;
      expect(gap.className).not.toContain("ui-glass");
      expect(gap.className).not.toContain("ui-surface");
    });

    it("stamps an introduced element and clears it on step change", async () => {
      const steps: IntroductionStepDefinition[] = [
        { id: "footer", title: "Footer", body: "This is the footer.", introduce: "ui.footer", show: [], placement: "auto", interactions: [], ordered: false, logos: [], demonstrations: [] },
        { id: "welcome", title: "Welcome", body: "No introduce.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
      ];
      const Harness: React.FC = () => {
        const [stepIndex, setStepIndex] = reactHostPort.useState(0);
        return (
          <div>
            <footer id="ui.footer" />
            <UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={stepIndex} onStepIndexChange={setStepIndex} onDismiss={vi.fn()} />
          </div>
        );
      };
      const { container } = render(<Harness />);
      const footer = container.querySelector("#ui\\.footer")!;
      await waitFor(() => {
        expect(footer.getAttribute("data-introduced")).toBe("true");
      });
      fireEvent.click(screen.getByRole("button", { name: /next|weiter/i }));
      await waitFor(() => {
        expect(footer.getAttribute("data-introduced")).toBeNull();
      });
    });

    it("unmount clears an introduced stamp", async () => {
      const { container, unmount } = render(
        <div>
          <footer id="ui.footer" />
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [{ id: "footer", title: "Footer", body: "Cutout.", introduce: "ui.footer", show: [], placement: "auto", interactions: [], ordered: false, logos: [], demonstrations: [] }],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      const footer = container.querySelector("#ui\\.footer")!;
      await waitFor(() => {
        expect(footer.getAttribute("data-introduced")).toBe("true");
      });
      unmount();
      expect(footer.getAttribute("data-introduced")).toBeNull();
    });

    it("a screen-style step (`introduce: null`) never stamps anything", () => {
      const { container } = render(
        <div>
          <footer id="ui.footer" />
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [{ id: "footer", title: "Footer", body: "Veiled only.", introduce: null, show: [], placement: "auto", interactions: [], ordered: false, logos: [], demonstrations: [] }],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      const footer = container.querySelector("#ui\\.footer")!;
      expect(footer.getAttribute("data-introduced")).toBeNull();
    });

    it("uses adaptive foreground tokens for primary and secondary text", () => {
      const { container } = render(
        <UIIntroduction
          introduction={{
            title: "Welcome",
            steps: [
              {
                id: "welcome",
                title: "Welcome",
                body: "Introduction body",
                introduce: null,
                show: [],
                placement: "center",
                interactions: [],
                ordered: false,
                logos: [],
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
      expect(box?.className).toContain("text-foreground");
      expect(box?.className).not.toMatch(/(?:^|\s)border(?:\s|$)/);
      expect(box?.className).not.toContain("border-emphasized");
      expect(box?.querySelector("p")?.className).toContain("text-muted-foreground");
      expect(box?.querySelector("p")?.className).not.toMatch(/(?:^|\s)text-muted(?:\s|$)/);
    });

    it("pulses the introduction info-box silhouette with the introduced border effect", () => {
      const rect = {
        x: 0,
        y: 0,
        top: 0,
        left: 0,
        bottom: 200,
        right: 320,
        width: 320,
        height: 200,
        toJSON() {
          return this;
        },
      };
      const rectSpy = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(rect as DOMRect);
      try {
        const { container } = render(
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [
                {
                  id: "welcome",
                  title: "Welcome",
                  body: "Introduction body",
                  introduce: null,
                  show: [],
                  placement: "center",
                  interactions: [],
                  ordered: false,
                  logos: [],
                  demonstrations: [],
                },
              ],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />,
        );
        const border = container.querySelector('[data-slot="introduction-info-box"] [data-slot="window-chrome-silhouette-border"]');
        expect(border?.getAttribute("data-kind")).toBe("introduced");
        expect(border?.hasAttribute("data-pending")).toBe(false);
        const path = border?.querySelector("path");
        expect(path).toBeTruthy();
        expect(path?.getAttribute("stroke")).toBe("var(--introduced-border-color, var(--color-secondary))");
        expect(path?.getAttribute("class") || "").toContain("window-silhouette-border-introduced");
      } finally {
        rectSpy.mockRestore();
      }
    });

    it("introduction step activates on click then returns to normal after a background click", async () => {
      const rectSpy = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue({
        x: 0,
        y: 0,
        top: 40,
        left: 40,
        bottom: 200,
        right: 360,
        width: 320,
        height: 160,
        toJSON: () => ({}),
      } as DOMRect);
      try {
        const { container } = render(
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [
                {
                  id: "welcome",
                  title: "Welcome",
                  body: "Introduction body",
                  introduce: null,
                  show: [],
                  placement: "center",
                  interactions: [],
                  ordered: false,
                  logos: [],
                  demonstrations: [],
                },
              ],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />,
        );
        const stack = container.querySelector('[data-slot="introduction-info-box"]') as HTMLElement;
        const border = () => stack.querySelector('[data-slot="window-chrome-silhouette-border"]');
        expect(border()?.getAttribute("data-kind")).toBe("introduced");
        fireEvent.pointerDown(stack.querySelector('[data-slot="window-chrome-body"]')!);
        await waitFor(() => {
          expect(stack.getAttribute("data-active")).toBe("true");
          expect(border()?.getAttribute("data-kind")).toBe("active");
        });
        fireEvent.pointerDown(document.body);
        await waitFor(() => {
          expect(stack.getAttribute("data-active")).toBeNull();
          expect(border()?.getAttribute("data-kind")).toBe("normal");
        });
      } finally {
        rectSpy.mockRestore();
      }
    });

    it("introduction and dialog glass boxes emphasize their silhouette only while the pointer is inside", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, resolve } = await import("node:path");
      const css = readFileSync(resolve(dirname(fileURLToPath(testSource.url)), "../../🎨️styling/🖌️ui/🎨️.css"), "utf8");
      expect(css).toContain('[data-slot="introduction-info-box"]');
      expect(css).toContain('[data-slot="dialog-content"]');
      expect(css).toContain("data-window-silhouette-border");
      expect(css).toContain("[data-window-silhouette]");
      expect(css).toContain(':not([data-active="true"]):hover');
      expect(css).toContain("stroke: var(--border-emphasized-color)");
      expect(css).toContain("window-silhouette-border-introduced");
      expect(css).toContain("@keyframes introduced-border-pulse");
      expect(css).not.toContain("@keyframes window-silhouette-border-introduced-pulse");
      expect(css).not.toMatch(/\[data-slot="introduction-info-box"\]:focus-within/);
      expect(css).not.toMatch(/\[data-slot="dialog-content"\]:focus-within/);
      expect(GLASS_OVERLAY_BOX_CLASS).not.toMatch(/(?:^|\s)border(?:\s|$)/);
      expect(GLASS_OVERLAY_BOX_CLASS).not.toContain("border-emphasized");
      expect(GLASS_OVERLAY_BOX_CLASS).not.toContain("border-normal");
    });

    it("wraps the introduction silhouette around the step chip and Next when Back is absent", async () => {
      const steps: IntroductionStepDefinition[] = [{ id: "only", title: "Welcome", body: "Step one.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
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
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const stack = container.querySelector('[data-slot="introduction-info-box"]') as HTMLElement;
      expect(stack.querySelector('[data-slot="window-chrome-footer-left"]')).toBeNull();
      expect(stack.querySelector('[data-slot="window-chrome-footer-center-chip"]')).toBeTruthy();
      expect(stack.querySelector('[data-slot="window-chrome-footer-right"]')).toBeTruthy();
      mockRect(stack, { left: 0, top: 0, width: 200, height: 100, right: 200, bottom: 100 });
      mockRect(stack.querySelector('[data-slot="window-chrome-chip-cap"]'), { left: 0, right: 60, width: 60, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="window-chrome-gap"]'), { left: 60, right: 160, width: 100, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="window-chrome-controls"]'), { left: 160, right: 200, width: 40, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="window-chrome-cap"]'), { left: 0, height: 24, bottom: 24, width: 200, right: 200 });
      mockRect(stack.querySelector('[data-slot="window-chrome-footer-center-chip"]'), { left: 80, right: 120, width: 40, height: 24, bottom: 100 });
      mockRect(stack.querySelector('[data-slot="window-chrome-footer-right"]'), { left: 140, right: 200, width: 60, height: 24, bottom: 100 });
      stack.setAttribute("data-silhouette-remeasure", "intro-center-right");
      await waitFor(() => {
        const path = stack.querySelector('[data-slot="window-chrome-silhouette-border"] path')?.getAttribute("d");
        expect(path).toBe(
          windowSilhouettePath({
            width: 200,
            height: 100,
            top: {
              depth: 24,
              chips: [
                { left: 0, right: 60 },
                { left: 160, right: 200 },
              ],
            },
            bottom: {
              depth: 24,
              chips: [
                { left: 80, right: 120 },
                { left: 140, right: 200 },
              ],
            },
          }),
        );
        expect(path).toContain("V99 H140 V75 H120 V99 H80 V75");
        expect(path).not.toMatch(/V99 V\d+/);
      });
    });

    it("skips the bottom-right silhouette notch when the introduction next chip is absent", async () => {
      const steps: IntroductionStepDefinition[] = [
        { id: "first", title: "First", body: "Step one.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
        {
          id: "second",
          title: "Second",
          body: "Complete the step.",
          introduce: null,
          show: [],
          placement: "center",
          interactions: [],
          ordered: false,
          logos: [],
          demonstrations: [],
        },
      ];
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
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={1} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const stack = container.querySelector('[data-slot="introduction-info-box"]') as HTMLElement;
      mockRect(stack, { left: 0, top: 0, width: 200, height: 100, right: 200, bottom: 100 });
      mockRect(stack.querySelector('[data-slot="window-chrome-chip-cap"]'), { left: 0, right: 60, width: 60, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="window-chrome-gap"]'), { left: 60, right: 160, width: 100, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="window-chrome-controls"]'), { left: 160, right: 200, width: 40, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="window-chrome-cap"]'), { left: 0, height: 24, bottom: 24, width: 200, right: 200 });
      mockRect(stack.querySelector('[data-slot="window-chrome-footer-left"]'), { left: 0, right: 50, width: 50, height: 24, bottom: 100 });
      mockRect(stack.querySelector('[data-slot="window-chrome-footer-center-chip"]'), { left: 80, right: 120, width: 40, height: 24, bottom: 100 });
      stack.setAttribute("data-silhouette-remeasure", "intro");
      await waitFor(() => {
        const path = stack.querySelector('[data-slot="window-chrome-silhouette-border"] path')?.getAttribute("d");
        expect(path).toBe(
          windowSilhouettePath({
            width: 200,
            height: 100,
            top: {
              depth: 24,
              chips: [
                { left: 0, right: 60 },
                { left: 160, right: 200 },
              ],
            },
            bottom: {
              depth: 24,
              chips: [
                { left: 0, right: 50 },
                { left: 80, right: 120 },
              ],
            },
          }),
        );
        expect(path).toContain("V75 H120 V99 H80 V75 H50 V99");
        expect(path).not.toMatch(/V99 V\d+/);
      });
    });

    it("emphasizes only the introduction body paragraph under the pointer", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, resolve } = await import("node:path");
      const css = readFileSync(resolve(dirname(fileURLToPath(testSource.url)), "../../🎨️styling/🖌️ui/🎨️.css"), "utf8");
      expect(css).toMatch(/\[data-slot="introduction-info-box"\]\s*\[data-slot="introduction-body-paragraph"\]:hover\s*\{\s*color:\s*var\(--border-emphasized-color\);/);
      expect(css).not.toMatch(/\[data-slot="introduction-info-box"\]\s*\[data-slot="window-chrome-body"\]:hover/);
      expect(css).toMatch(
        /\[data-slot="introduction-info-box"\]:is\(\s*:has\(\[data-slot="window-chrome-body"\]:hover\)[\s\S]*?\)\s*\[data-slot="introduction-info-box-chip"\]:not\(\[data-handle-hovered="true"\]\)\s*\{\s*color:\s*var\(--border-emphasized-color\);/,
      );
      expect(css).toMatch(
        /\[data-slot="introduction-info-box"\]:is\([\s\S]*?:has\(\[data-slot="introduction-info-box-chip"\]:hover\)[\s\S]*?\)\s*\[data-slot="introduction-info-box-chip"\]\s*\[data-slot="drag-handle"\]\s*\{\s*color:\s*var\(--border-emphasized-color\);/,
      );
      expect(css).not.toMatch(/\[data-slot="introduction-info-box"\]:hover\s*\[data-slot="introduction-step-chip"\]/);
      expect(css).not.toMatch(/\[data-slot="introduction-body-paragraph"\]:focus-within/);
      expect(splitIntroductionBodyParagraphs("One.\n\nTwo.\n\n\nThree.")).toEqual(["One.", "Two.", "Three."]);
      expect(splitIntroductionBodyParagraphs("  single  ")).toEqual(["single"]);
      expect(splitIntroductionBodyParagraphs("")).toEqual([]);
      const steps: IntroductionStepDefinition[] = [
        {
          id: "body-hover",
          title: "Body Hover",
          body: "First paragraph.\n\nSecond paragraph.",
          introduce: null,
          show: [],
          placement: "center",
          interactions: [{ on: { kind: "action", id: "ui.footer" }, label: "Click footer" }],
          ordered: true,
          logos: [],
          demonstrations: [],
        },
      ];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const paragraphs = container.querySelectorAll('[data-slot="introduction-body-paragraph"]');
      expect(paragraphs).toHaveLength(2);
      expect(paragraphs[0]?.textContent).toBe("First paragraph.");
      expect(paragraphs[1]?.textContent).toBe("Second paragraph.");
      expect(paragraphs[0]?.className).toContain("text-muted-foreground");
      expect(container.querySelector('[data-slot="introduction-interaction-label"]')?.textContent).toBe("Click footer");
    });
});
}
