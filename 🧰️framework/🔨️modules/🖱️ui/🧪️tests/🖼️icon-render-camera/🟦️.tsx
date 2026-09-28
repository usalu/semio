import type { IconRenderRequest } from "@semio-tech/ui-styling";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🖼️icon-render-camera/🔣️.json";
import schema from "../../🧬️schema/🖼️icon-render-camera/🔣️.json";
/** 📐️ three.js itself, the oracle the icon camera is measured against (a test of the ui module, the library's interface owner). */
import * as THREE from "three";

/** 🖼️ The icon renderer's camera and SVG finishing: an orthographic shot keeps its zoom, a fitted shot recomputes it from the
 * model bounds, and a transparent shot drops the three.js SVGRenderer clear colour. */
export async function registerIconRenderCameraTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🎯️targets/⚛️react/🟦️.tsx"), "buildIconCamera" | "finalizeIconSvgMarkup" | "iconRenderCameraPose">): Promise<void> {
  const { buildIconCamera, finalizeIconSvgMarkup, iconRenderCameraPose } = dependencies;
  const { describe, expect, it } = vitest;
  describe("icon camera projection contract", () => {
    it("validates the shared language-neutral camera cases", () => {
      const validate = new Ajv({ strict: false }).compile(schema);
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    });
    for (const sample of fixture.cases) {
      it(sample.id, () => {
        const { minimum, maximum } = sample.model;
        const geometry = new THREE.BoxGeometry(...minimum.map((value, axis) => maximum[axis] - value) as [number, number, number]);
        const model = new THREE.Mesh(geometry);
        model.position.set(...minimum.map((value, axis) => (maximum[axis] + value) / 2) as [number, number, number]);
        model.updateMatrixWorld(true);
        const request: IconRenderRequest = {
          ...sample.request,
          format: sample.request.format as IconRenderRequest["format"],
          camera: { ...sample.request.camera, position: [sample.request.camera.position[0], sample.request.camera.position[1], sample.request.camera.position[2]], target: [sample.request.camera.target[0], sample.request.camera.target[1], sample.request.camera.target[2]], projection: sample.request.camera.projection === "orthographic" ? "orthographic" : "perspective" },
          lights: { ambientIntensity: 1, ambientColor: "#fff", sunAzimuth: 0, sunElevation: 45, sunIntensity: 1, sunColor: "#fff" },
        };
        const pose = iconRenderCameraPose(request, model);
        const camera = buildIconCamera(request, pose) as THREE.PerspectiveCamera | THREE.OrthographicCamera;
        camera.updateMatrixWorld(true);
        for (const field of ["position", "target"] as const) {
          pose[field].forEach((value, axis) => expect(Math.abs(value - sample.expected[field][axis])).toBeLessThan(fixture.tolerance));
        }
        expect(Math.abs(pose.zoom - sample.expected.zoom)).toBeLessThan(fixture.tolerance);
        expect(camera instanceof THREE.OrthographicCamera).toBe(sample.request.camera.projection === "orthographic");
        if (camera instanceof THREE.PerspectiveCamera) expect(Math.abs(camera.getEffectiveFOV() - sample.expected.effectiveFov)).toBeLessThan(fixture.tolerance);
        const projected = new THREE.Vector3(...pose.target).project(camera);
        expect(Math.abs(projected.x - sample.expected.targetNdc[0])).toBeLessThan(fixture.tolerance);
        expect(Math.abs(projected.y - sample.expected.targetNdc[1])).toBeLessThan(fixture.tolerance);
        if (camera instanceof THREE.OrthographicCamera) {
          const right = new THREE.Vector3().setFromMatrixColumn(camera.matrixWorld, 0);
          const next = new THREE.Vector3(...pose.target).add(right).project(camera);
          expect(Math.abs((next.x - projected.x) * sample.preview.width / 2 - sample.expected.previewZoom)).toBeLessThan(fixture.tolerance);
        }
        geometry.dispose();
      });
    }
  });
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
      expect((camera as THREE.OrthographicCamera).isOrthographicCamera).toBe(true);
      expect((camera as THREE.OrthographicCamera).zoom).toBe(42);
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
