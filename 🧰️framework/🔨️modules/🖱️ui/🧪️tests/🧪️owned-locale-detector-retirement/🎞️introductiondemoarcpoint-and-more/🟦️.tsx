import type { IntroductionStepDefinition } from "@semio-tech/ui-react/source";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerIntroductiondemoarcpointAndMoreTests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { INTRODUCTION_DEMO_IDLE_THRESHOLD_MS, React, UIIntroduction, clampSliderValuesToReady, formatNumber, iconShotFrameClass, iconShotFrameStyle, introductionDemoArcPoint, referenceMediaKindFromUrl, resolveSliderDraftClear, sliderValuesMatch, sunPositionFromAzimuthElevation, useIntroductionPointerIdle } = dependencies;
  type IntroductionStepDefinition = any;
  const { describe, expect, it, vi } = vitest;
  const { render, fireEvent, act } = await import("@testing-library/react");

  describe("introductionDemoArcPoint", () => {
    it("starts and ends exactly at the endpoints, and bulges away from the straight line in between", () => {
      const from = { x: 0, y: 0 };
      const to = { x: 100, y: 0 };
      expect(introductionDemoArcPoint(from, to, 0)).toEqual(from);
      expect(introductionDemoArcPoint(from, to, 1)).toEqual(to);
      const mid = introductionDemoArcPoint(from, to, 0.5);
      // 🎬️ The straight-line midpoint is (50, 0) — an orbit arc must bulge perpendicular (off the x-axis).
      expect(mid.x).toBeCloseTo(50, 5);
      expect(Math.abs(mid.y)).toBeGreaterThan(5);
    });
  });

  describe("useIntroductionPointerIdle", () => {
    it("goes idle after the threshold, resets on coordinate or pointer-lock movement, and ignores stationary pointermoves", () => {
      vi.useFakeTimers();
      try {
        const IdleProbe: React.FC = () => {
          const { idle, lastPositionRef } = useIntroductionPointerIdle(true, 1000);
          return <div data-testid="idle-probe" data-idle={idle ? "true" : "false"} data-last={lastPositionRef.current ? `${lastPositionRef.current.x},${lastPositionRef.current.y}` : ""} />;
        };
        const { getByTestId } = render(<IdleProbe />);
        const probe = () => getByTestId("idle-probe").getAttribute("data-idle");

        expect(probe()).toBe("false");
        act(() => {
          vi.advanceTimersByTime(999);
        });
        expect(probe()).toBe("false");
        act(() => {
          vi.advanceTimersByTime(1);
        });
        expect(probe()).toBe("true");

        fireEvent(window, new MouseEvent("pointermove", { bubbles: true, clientX: 10, clientY: 20 }));
        expect(probe()).toBe("false");
        expect(getByTestId("idle-probe").getAttribute("data-last")).toBe("10,20");

        // 🎬️ Same coordinates as the last move: ignored, so the idle timer set by the move above must NOT
        // have been restarted — advancing exactly the threshold from that move flips idle back to true.
        fireEvent(window, new MouseEvent("pointermove", { bubbles: true, clientX: 10, clientY: 20 }));
        expect(probe()).toBe("false");
        act(() => {
          vi.advanceTimersByTime(1000);
        });
        expect(probe()).toBe("true");

        const pointerLockMove = new PointerEvent("pointermove", { bubbles: true, clientX: 10, clientY: 20 });
        Object.defineProperty(pointerLockMove, "movementX", { value: 4 });
        fireEvent(window, pointerLockMove);
        expect(probe()).toBe("false");
      } finally {
        vi.useRealTimers();
      }
    });
  });

  describe("UIIntroduction demonstration", () => {
    const demoSteps: IntroductionStepDefinition[] = [
      {
        id: "fill",
        title: "Füllen",
        body: "Click fill.",
        introduce: "tool.fill",
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "tool", id: "fill" }, label: "Activate Fill" }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: "tool.fill" } } }],
      },
    ];

    it("mutes the real cursor only once idle, and restores it the instant the pointer moves", () => {
      vi.useFakeTimers();
      try {
        render(
          <div>
            <button id="tool.fill">Fill</button>
            <UIIntroduction introduction={{ title: "Welcome", steps: demoSteps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />
          </div>,
        );
        expect(document.documentElement.hasAttribute("data-introduction-demonstrating")).toBe(false);

        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.getAttribute("data-introduction-demonstrating")).toBe("true");
        const overlay = document.querySelector<HTMLElement>('[data-slot="introduction-demonstration"]');
        expect(overlay?.style.visibility).toBe("visible");

        fireEvent(window, new MouseEvent("pointermove", { bubbles: true, clientX: 5, clientY: 5 }));
        expect(document.documentElement.hasAttribute("data-introduction-demonstrating")).toBe(false);
        expect(overlay?.style.visibility).toBe("hidden");
      } finally {
        vi.useRealTimers();
      }
    });

    it("unmount clears the demonstrating attribute", () => {
      vi.useFakeTimers();
      try {
        const { unmount } = render(
          <div>
            <button id="tool.fill">Fill</button>
            <UIIntroduction introduction={{ title: "Welcome", steps: demoSteps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />
          </div>,
        );
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.getAttribute("data-introduction-demonstrating")).toBe("true");
        unmount();
        expect(document.documentElement.hasAttribute("data-introduction-demonstrating")).toBe(false);
      } finally {
        vi.useRealTimers();
      }
    });

    it("a purely informational step (Next is the only way forward) auto-demonstrates clicking Next, even without a declared demonstration", () => {
      vi.useFakeTimers();
      try {
        const steps: IntroductionStepDefinition[] = [{ id: "welcome", title: "Welcome", body: "No demo declared.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
        render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.getAttribute("data-introduction-demonstrating")).toBe("true");
      } finally {
        vi.useRealTimers();
      }
    });

    it("omitted demonstrations (Rust serde default) still auto-demonstrates Next on informational steps", () => {
      vi.useFakeTimers();
      try {
        const steps = [{ id: "welcome", title: "Welcome", body: "No demo field.", introduce: null, show: [], placement: "center" as const, interactions: [], ordered: false, logos: [] }] as IntroductionStepDefinition[];
        render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
        expect(document.querySelector('[data-slot="introduction-info-box"]')).toBeTruthy();
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.getAttribute("data-introduction-demonstrating")).toBe("true");
      } finally {
        vi.useRealTimers();
      }
    });

    it("mounts the demonstration callout cluster while demonstrating", () => {
      vi.useFakeTimers();
      try {
        render(
          <div>
            <button id="tool.fill">Fill</button>
            <UIIntroduction introduction={{ title: "Welcome", steps: demoSteps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />
          </div>,
        );
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.querySelector('[data-slot="introduction-demonstration-callout"]')).toBeTruthy();
        expect(document.querySelector(".introduction-demo-mouse")).toBeTruthy();
        expect(document.querySelector('.introduction-demo-mouse-button[data-part="left"]')).toBeTruthy();
      } finally {
        vi.useRealTimers();
      }
    });

    it("an interaction-gated step with no declared demonstration never mutes the cursor", () => {
      vi.useFakeTimers();
      try {
        const steps: IntroductionStepDefinition[] = [
          {
            id: "transform-utility",
            title: "Transform",
            body: "No demo declared.",
            introduce: "transform",
            show: [],
            placement: "auto",
            interactions: [{ on: { kind: "utility", id: "transform" }, label: "Activate Transform" }],
            ordered: false,
            logos: [],
            demonstrations: [],
          },
        ];
        render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.hasAttribute("data-introduction-demonstrating")).toBe(false);
      } finally {
        vi.useRealTimers();
      }
    });

    const viewportDemoSteps: IntroductionStepDefinition[] = [
      {
        id: "viewport",
        title: "3D View",
        body: "Zoom, pan, orbit.",
        introduce: "framework.window.puzzle3dMain",
        show: [],
        placement: "auto",
        interactions: [
          { on: { kind: "zoom", id: "puzzle3d-main" }, label: "Zoom" },
          { on: { kind: "pan", id: "puzzle3d-main" }, label: "Pan" },
          { on: { kind: "orbit", id: "puzzle3d-main" }, label: "Orbit" },
        ],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "scroll", at: { kind: "element", id: "framework.window.puzzle3dMain" }, deltaY: -100 } },
          { gesture: { kind: "drag", from: { kind: "element", id: "framework.window.puzzle3dMain" }, to: { kind: "element", id: "framework.window.puzzle3dMain" }, button: "middle" } },
          { gesture: { kind: "orbit", from: { kind: "element", id: "framework.window.puzzle3dMain" }, to: { kind: "element", id: "framework.window.puzzle3dMain" } } },
        ],
      },
    ];

    it("still demonstrates remaining interactions when some are already completed", () => {
      vi.useFakeTimers();
      try {
        render(
          <>
            <div id="framework.window.puzzle3dMain" style={{ width: 400, height: 300 }} />
            <UIIntroduction introduction={{ title: "Welcome", steps: viewportDemoSteps }} stepIndex={0} completedInteractionIndices={[0]} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />
          </>,
        );
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.getAttribute("data-introduction-demonstrating")).toBe("true");
        expect(document.querySelector('[data-slot="introduction-demonstration-callout"]')).toBeTruthy();
      } finally {
        vi.useRealTimers();
      }
    });

    it("stops demonstrating when every interaction is completed", () => {
      vi.useFakeTimers();
      try {
        render(
          <>
            <div id="framework.window.puzzle3dMain" style={{ width: 400, height: 300 }} />
            <UIIntroduction introduction={{ title: "Welcome", steps: viewportDemoSteps }} stepIndex={0} completedInteractionIndices={[0, 1, 2]} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />
          </>,
        );
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.hasAttribute("data-introduction-demonstrating")).toBe(false);
        expect(document.querySelector('[data-slot="introduction-demonstration-callout"]')).toBeNull();
      } finally {
        vi.useRealTimers();
      }
    });

    it("demonstrates all gestures on gallery steps without interactions regardless of completion indices", () => {
      vi.useFakeTimers();
      try {
        const gallerySteps: IntroductionStepDefinition[] = [
          {
            id: "gallery",
            title: "Gallery",
            body: "All gestures.",
            introduce: null,
            show: [],
            placement: "center",
            interactions: [],
            ordered: false,
            logos: [],
            demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: "tool.fill" } } }, { gesture: { kind: "scroll", at: { kind: "element", id: "tool.fill" }, deltaY: -100 } }],
          },
        ];
        render(
          <div>
            <button id="tool.fill">Fill</button>
            <UIIntroduction introduction={{ title: "Welcome", steps: gallerySteps }} stepIndex={0} completedInteractionIndices={[0, 1, 2]} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />,
          </div>,
        );
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.getAttribute("data-introduction-demonstrating")).toBe("true");
      } finally {
        vi.useRealTimers();
      }
    });

    it("emphasizes the step count chip when hovered", () => {
      const steps: IntroductionStepDefinition[] = [{ id: "welcome", title: "Welcome", body: "Hello", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const stepChip = container.querySelector('[data-slot="introduction-step-chip"]');
      expect(stepChip).toBeTruthy();
      const stepText = stepChip?.querySelector("span");
      expect(stepText).toBeTruthy();
      expect(stepText?.className).toContain("group-hover:text-emphasized");
      expect(stepText?.className).toContain("transition-colors");
    });
  });

  describe("formatNumber", () => {
    it("strips IEEE-754 float artifacts for display", () => {
      expect(formatNumber(-2.5999999999999996)).toBe("-2.6");
      expect(formatNumber(0.1 + 0.2)).toBe("0.3");
      expect(formatNumber(42)).toBe("42");
      expect(formatNumber(1e-7)).toBe("1e-7");
      expect(formatNumber(NaN)).toBe("");
      expect(formatNumber(Infinity)).toBe("");
      expect(formatNumber("not-a-number")).toBe("not-a-number");
    });
  });

  describe("slider draft confirmation", () => {
    it("keeps pending draft values until the external value catches up", () => {
      expect(resolveSliderDraftClear([42], [10], 1)).toEqual([42]);
      expect(resolveSliderDraftClear([42], [42], 1)).toBeNull();
      expect(sliderValuesMatch([0.5], [0.51], 0.1)).toBe(true);
      expect(sliderValuesMatch([0.5], [0.8], 0.1)).toBe(false);
    });
  });

  describe("clampSliderValuesToReady", () => {
    it("passes values through unchanged when ready is not set", () => {
      expect(clampSliderValuesToReady([500], undefined, 0)).toEqual([500]);
    });

    it("clamps every value down to the ready extent", () => {
      expect(clampSliderValuesToReady([500], 40, 0)).toEqual([40]);
      expect(clampSliderValuesToReady([10, 500], 40, 0)).toEqual([10, 40]);
    });

    it("leaves values at or below ready untouched", () => {
      expect(clampSliderValuesToReady([40], 40, 0)).toEqual([40]);
      expect(clampSliderValuesToReady([5], 40, 0)).toEqual([5]);
    });

    it("floors the clamp ceiling at min, even when ready is below it", () => {
      expect(clampSliderValuesToReady([50], 0, 10)).toEqual([10]);
    });
  });

  describe("referenceMediaKindFromUrl", () => {
    it("infers image, svg, and pdf kinds from paths", () => {
      expect(referenceMediaKindFromUrl("/infinite-assets/✏️sketch/🖼️.png")).toBe("image");
      expect(referenceMediaKindFromUrl("/infinite-assets/icon.svg")).toBe("svg");
      expect(referenceMediaKindFromUrl("/infinite-assets/🗺️site.pdf")).toBe("pdf");
      expect(referenceMediaKindFromUrl("/unknown.bin")).toBeNull();
    });
  });

  describe("iconShotFrame", () => {
    it("sizes the frame from fixed width and height", () => {
      const landscape = iconShotFrameStyle(512, 256);
      expect(landscape.aspectRatio).toBe("512 / 256");
      expect(landscape.width).toBe("100%");
      expect(landscape.height).toBe("auto");
      const portrait = iconShotFrameStyle(256, 512);
      expect(portrait.width).toBe("auto");
      expect(portrait.height).toBe("100%");
    });

    it("masks ellipse shots with a rounded frame", () => {
      expect(iconShotFrameClass("ellipse")).toBe("rounded-[50%]");
      expect(iconShotFrameClass("rectangle")).toBe("rounded-none");
    });
  });

  describe("sunPositionFromAzimuthElevation", () => {
    it("places the sun on the azimuth/elevation sphere", () => {
      const [x, y, z] = sunPositionFromAzimuthElevation(0, 90, 100);
      expect(x).toBeCloseTo(0, 5);
      expect(y).toBeCloseTo(0, 5);
      expect(z).toBeCloseTo(100, 5);
      const [ex, ey, ez] = sunPositionFromAzimuthElevation(90, 0, 100);
      expect(ex).toBeCloseTo(0, 5);
      expect(ey).toBeCloseTo(100, 5);
      expect(ez).toBeCloseTo(0, 5);
    });
  });
}
