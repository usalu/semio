type TestSource = { readonly directory: string; readonly url: string };

type RenderBounds = { readonly min: number; readonly max: number };
type Step = { readonly do: string; readonly advanceMs: number; readonly renders: RenderBounds };

/** @emoji 🪶️ Replays `🧫️fixtures/🪶️demand-frames/🔣️.json` against the real {@link GraphWasmCanvas}, mounted by React DOM
 * (the third-party renderer) on a faked frame + wall clock — an idle canvas paints nothing, owner calls before a frame
 * paint it once, `renderFrame` paints at once and satisfies them — and against the one shared scheduler directly (an
 * explicit trailing window and a continuous reason are bounded animations). */
export async function registerTests2(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.tsx"), "GraphWasmCanvas" | "React" | "createDemandFrameScheduler">, source: TestSource): Promise<void> {
  const { GraphWasmCanvas, React, createDemandFrameScheduler } = dependencies;
  const { describe, expect, it, vi, beforeEach, afterEach } = vitest;
  const { createRoot } = await import("react-dom/client");
  const fixture = (await import("../../../🧫️fixtures/🪶️demand-frames/🔣️.json", { with: { type: "json" } })).default as {
    readonly cases: readonly { readonly id: string; readonly steps: readonly Step[] }[];
    readonly scheduler: readonly { readonly id: string; readonly trailingWindowMs: number; readonly steps: readonly Step[] }[];
  };

  type Handle = Record<string, (...args: unknown[]) => unknown>;
  const restore: (() => void)[] = [];
  beforeEach(() => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "requestAnimationFrame", "cancelAnimationFrame", "Date"] });
    (globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
    const rect = Object.getOwnPropertyDescriptor(HTMLElement.prototype, "getBoundingClientRect");
    HTMLElement.prototype.getBoundingClientRect = () => ({ x: 0, y: 0, left: 0, top: 0, right: 400, bottom: 300, width: 400, height: 300, toJSON: () => ({}) }) as DOMRect;
    const observer = (globalThis as { ResizeObserver?: unknown }).ResizeObserver;
    (globalThis as { ResizeObserver?: unknown }).ResizeObserver = class {
      observe(): void {}
      disconnect(): void {}
    };
    restore.push(() => {
      if (rect) Object.defineProperty(HTMLElement.prototype, "getBoundingClientRect", rect);
      (globalThis as { ResizeObserver?: unknown }).ResizeObserver = observer;
    });
  });
  afterEach(() => {
    for (const undo of restore.splice(0)) undo();
    vi.useRealTimers();
  });

  const expectRenders = (label: string, painted: number, bounds: RenderBounds) => {
    expect(painted, `${label}: ${painted} renders`).toBeGreaterThanOrEqual(bounds.min);
    expect(painted, `${label}: ${painted} renders`).toBeLessThanOrEqual(bounds.max);
  };

  describe("GraphWasmCanvas paints on demand, once per change (demand-frames fixture)", () => {
    for (const testCase of fixture.cases) {
      it(testCase.id, async () => {
        let renders = 0;
        let handle: Handle | null = null;
        const session = {
          attachCanvas: async () => undefined,
          setSize: () => undefined,
          renderFrame: () => {
            renders += 1;
          },
          syncFromScenePack: () => undefined,
          setCanvasThemeJson: () => undefined,
        };
        const container = document.createElement("div");
        document.body.appendChild(container);
        const root = createRoot(container);
        for (const step of testCase.steps) {
          const before = renders;
          await React.act(async () => {
            if (step.do === "mount") root.render(React.createElement(GraphWasmCanvas, { sessionFactory: () => session, onSessionReady: (ready: unknown) => { handle = ready as Handle; }, enablePointer: false }));
            else if (step.do === "unmount") root.unmount();
            else if (step.do.startsWith("calls:")) for (const method of step.do.slice("calls:".length).split(",")) handle![method]!("{}");
          });
          await React.act(async () => {
            await vi.advanceTimersByTimeAsync(step.advanceMs);
          });
          expectRenders(`${testCase.id} / ${step.do}`, renders - before, step.renders);
        }
        if (!testCase.steps.some((step) => step.do === "unmount")) await React.act(async () => root.unmount());
        container.remove();
      });
    }
  });

  describe("the one demand-frame scheduler bounds every animation (demand-frames fixture)", () => {
    for (const testCase of fixture.scheduler) {
      it(testCase.id, async () => {
        let renders = 0;
        const scheduler = createDemandFrameScheduler(() => {
          renders += 1;
        }, { trailingWindowMs: testCase.trailingWindowMs });
        for (const step of testCase.steps) {
          const before = renders;
          if (step.do === "invalidate") scheduler.invalidate();
          else if (step.do === "paintNow") scheduler.paintNow();
          else if (step.do.startsWith("begin:")) scheduler.beginContinuous(step.do.slice("begin:".length));
          else if (step.do.startsWith("end:")) scheduler.endContinuous(step.do.slice("end:".length));
          await vi.advanceTimersByTimeAsync(step.advanceMs);
          expectRenders(`${testCase.id} / ${step.do}`, renders - before, step.renders);
        }
        scheduler.dispose();
      });
    }
  });
}
