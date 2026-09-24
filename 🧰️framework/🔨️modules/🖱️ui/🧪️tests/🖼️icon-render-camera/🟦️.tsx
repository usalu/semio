import type { IconRenderRequest } from "@semio-tech/ui-styling";
/** 📐️ Type-only three.js namespace; the runtime `THREE` value arrives through the dependency bag and would shadow this name. */
import type * as Three from "three";

/** 🖼️ The icon renderer's camera and SVG finishing: an orthographic shot keeps its zoom, a fitted shot recomputes it from the
 * model bounds, and a transparent shot drops the three.js SVGRenderer clear colour. */
export async function registerIconRenderCameraTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🎯️targets/⚛️react/🟦️.tsx"), "THREE" | "buildIconCamera" | "finalizeIconSvgMarkup" | "iconRenderCameraPose">): Promise<void> {
  const { THREE, buildIconCamera, finalizeIconSvgMarkup, iconRenderCameraPose } = dependencies;
  const { describe, expect, it } = vitest;
  describe("finalizeIconSvgMarkup", () => {
    it("removes three.js SVGRenderer white clear color when the shot background is transparent", () => {
      const input = '<svg xmlns="http://www.w3.org/2000/svg" style="background-color: rgb(255, 255, 255);"><path d="M0 0"/></svg>';
      expect(finalizeIconSvgMarkup(input, { background: undefined })).not.toMatch(/background-color/i);
      expect(finalizeIconSvgMarkup(input, { background: "#101014" })).toMatch(/background-color/i);
    });
  });
  describe("iconRenderCameraPose", () => {
    it("builds an orthographic camera for orthographic shots", () => {
      const request: IconRenderRequest = {
        assetUrl: "mesh://x",
        width: 256,
        height: 256,
        format: "svg" as const,
        lights: { ambientIntensity: 1, ambientColor: "#fff", sunAzimuth: 0, sunElevation: 45, sunIntensity: 1, sunColor: "#fff" },
        camera: { position: [10, -10, 8], target: [0, 0, 0], zoom: 42, projection: "orthographic" as const },
      };
      const camera = buildIconCamera(request);
      expect((camera as Three.OrthographicCamera).isOrthographicCamera).toBe(true);
      expect((camera as Three.OrthographicCamera).zoom).toBe(42);
    });
    it("recomputes orthographic zoom when fit is enabled", () => {
      const mesh = new THREE.Mesh(new THREE.BoxGeometry(4, 4, 4));
      const request: IconRenderRequest = {
        assetUrl: "mesh://x",
        width: 256,
        height: 256,
        format: "svg" as const,
        fit: { enabled: true, padding: 1.25 },
        lights: { ambientIntensity: 1, ambientColor: "#fff", sunAzimuth: 0, sunElevation: 45, sunIntensity: 1, sunColor: "#fff" },
        camera: { position: [10, -10, 8], target: [0, 0, 0], zoom: 200, projection: "orthographic" as const },
      };
      const pose = iconRenderCameraPose(request, mesh);
      expect(pose.projection).toBe("orthographic");
      expect(pose.zoom).toBeLessThan(50);
      expect(pose.zoom).toBeGreaterThan(1);
    });
  });
}
