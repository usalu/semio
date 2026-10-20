/** 🟢️ The resolve introduction point phase of registerUiintroductionAppearance2AndMoreTests. */
import type { buildCelebrateStampDurationState } from "../🔶️celebrate-stamp-duration/🟦️.tsx";

/** 🔵 Registers the resolve introduction point tests of registerUiintroductionAppearance2AndMoreTests. */
export function registerResolveIntroductionPointTests({ describe, it, render, vi, expect, resolveIntroductionPoint, registerIntroductionSurfaceResolver, polylinePointAt }: Pick<Awaited<ReturnType<typeof buildCelebrateStampDurationState>>, "describe" | "it" | "render" | "vi" | "expect" | "resolveIntroductionPoint" | "registerIntroductionSurfaceResolver" | "polylinePointAt">) {
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
}
