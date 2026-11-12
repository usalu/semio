/** 📷️ Camera: view/projection matrices equal three's PerspectiveCamera/OrthographicCamera (language-agnostic vectors recorded from three), preset directions, analytic orthographic fits, perspective fits that contain the box tightly (three projection), rays equal three's Raycaster, orbit/pan/zoom. */
import { Raycaster, Vector3 } from "three";
import { describe, expect, it } from "vitest";
import { PRESETS, basisOf, cameraOf, clipPlanes, fitCamera, orbit, pan, projectionMatrix, rayOf, viewMatrix, visibleHeight, withPreset, zoom, type CameraState } from "../../📷️camera/🟦️.ts";
import { fixture, threeCamera } from "../🧰️support/🟦️.ts";

const data = fixture("📷️camera");
const close = (a: ArrayLike<number>, b: ArrayLike<number>, tol: number) => {
  expect(a.length).toBe(b.length);
  for (let i = 0; i < a.length; i++) expect(Math.abs(a[i]! - b[i]!), `index ${i}`).toBeLessThan(tol);
};
const corners = (b: { min: number[]; max: number[] }) => Array.from({ length: 8 }, (_, i) => new Vector3(i & 1 ? b.max[0] : b.min[0], i & 2 ? b.max[1] : b.min[1], i & 4 ? b.max[2] : b.min[2]));

describe("matrices", () => {
  for (const c of data.cameras) {
    it(`${c.name} equals the recorded three matrices`, () => {
      close(viewMatrix(c.camera), c.view, data.tolerance);
      close(projectionMatrix(c.camera, c.aspect), c.projection, data.tolerance);
      close(basisOf(c.camera).eye, c.eye, data.tolerance);
    });
    it(`${c.name} equals live three matrices`, () => {
      const t = threeCamera(c.camera, c.aspect);
      close(viewMatrix(c.camera), t.matrixWorldInverse.toArray(), 1e-9);
      close(projectionMatrix(c.camera, c.aspect), t.projectionMatrix.toArray(), 1e-9);
    });
    it(`${c.name} ray equals three's Raycaster`, () => {
      const r = rayOf(c.camera, c.aspect, c.ray.ndc[0], c.ray.ndc[1]);
      close(r.origin, c.ray.origin, data.tolerance);
      close(r.direction, c.ray.direction, data.tolerance);
      const rc = new Raycaster();
      rc.setFromCamera({ x: c.ray.ndc[0], y: c.ray.ndc[1] } as never, threeCamera(c.camera, c.aspect));
      close(r.direction, rc.ray.direction.toArray(), 1e-9);
    });
  }
  it("writes into Float32Array and an existing array", () => {
    const out = new Float32Array(16);
    expect(viewMatrix(cameraOf("front"), out)).toBe(out);
    expect(out[15]).toBe(1);
  });
});

describe("presets", () => {
  const dir = (p: keyof typeof PRESETS) => basisOf(cameraOf(p));
  it("front looks at the asset front (-Z), +X to the right", () => {
    const b = dir("front");
    close(b.forward, [0, 0, -1], 1e-12);
    close(b.right, [1, 0, 0], 1e-12);
    close(b.up, [0, 1, 0], 1e-12);
  });
  it("elevation looks along +X from the asset's right with ahead (+Z) to the right of the picture", () => {
    const b = dir("elevation");
    close(b.forward, [1, 0, 0], 1e-12);
    close(b.right, [0, 0, 1], 1e-12);
    close(b.up, [0, 1, 0], 1e-12);
  });
  it("plan looks down with ahead to the right and the asset's left (+X) at the top", () => {
    const b = dir("plan");
    close(b.forward, [0, -1, 0], 1e-12);
    close(b.right, [0, 0, 1], 1e-12);
    close(b.up, [1, 0, 0], 1e-12);
  });
  it("axonometric is the isometric corner, orthographic", () => {
    const c = cameraOf("axonometric");
    expect(c.projection).toBe("orthographic");
    const f = basisOf(c).forward;
    expect(Math.abs(Math.abs(f[0]) - Math.abs(f[2]))).toBeLessThan(1e-12);
    expect(Math.abs(Math.abs(f[1]) - 1 / Math.sqrt(3))).toBeLessThan(1e-12);
  });
  it("is orthonormal and right-handed for all presets and a sweep of angles", () => {
    for (let yaw = -4; yaw <= 4; yaw += 0.7) for (let pitch = -1.57; pitch <= 1.571; pitch += 0.52) {
      const b = basisOf({ ...cameraOf("perspective"), yaw, pitch });
      expect(Math.abs(b.right[0] * b.up[0] + b.right[1] * b.up[1] + b.right[2] * b.up[2])).toBeLessThan(1e-12);
      expect(Math.abs(Math.hypot(...b.up) - 1)).toBeLessThan(1e-12);
      const r = new Vector3(...b.right).cross(new Vector3(...b.up));
      close(r.toArray(), [-b.forward[0], -b.forward[1], -b.forward[2]], 1e-12);
    }
  });
  it("withPreset keeps target and distance", () => {
    const c = withPreset({ ...cameraOf("front"), target: [1, 2, 3], distance: 9 }, "plan");
    expect(c.target).toEqual([1, 2, 3]);
    expect(c.distance).toBe(9);
    expect(c.pitch).toBeCloseTo(Math.PI / 2);
  });
});

describe("fit", () => {
  it("fits orthographic axis views analytically", () => {
    for (const f of data.fits.filter((f: any) => ["front", "elevation", "plan"].includes(f.preset))) {
      const half = { front: [1, 1], elevation: [1, 1], plan: [1, 1] }[f.preset as "front"]!;
      const expected = 2 * Math.max(half[1]!, half[0]! / f.aspect) * f.margin;
      expect(f.fitted.orthoHeight).toBeCloseTo(expected, 5);
      close(f.fitted.target, [0, 1, 0], 1e-9);
      const c = fitCamera(cameraOf(f.preset), f.bounds, f.aspect, f.margin);
      close([c.orthoHeight, c.distance, c.extent, ...c.target], [f.fitted.orthoHeight, f.fitted.distance, f.fitted.extent, ...f.fitted.target], 1e-5);
    }
  });
  it("fits every preset and aspect completely and tightly (three projection)", () => {
    for (const f of data.fits) {
      const cam = fitCamera(cameraOf(f.preset), f.bounds, f.aspect, 1.1);
      const t = threeCamera(cam, f.aspect);
      let peak = 0;
      for (const p of corners(f.bounds)) {
        const n = p.clone().project(t);
        peak = Math.max(peak, Math.abs(n.x), Math.abs(n.y));
        expect(Math.abs(n.x)).toBeLessThanOrEqual(1 / 1.1 + 1e-9 + 0.1);
        expect(Math.abs(n.y)).toBeLessThanOrEqual(1 / 1.1 + 1e-9 + 0.1);
        expect(n.z).toBeGreaterThan(-1);
        expect(n.z).toBeLessThan(1);
      }
      expect(peak, `${f.name} tight`).toBeGreaterThan(0.8);
    }
  });
  it("falls back to the unit box for empty or invalid bounds", () => {
    const c = fitCamera(cameraOf("front"), { min: [Infinity, Infinity, Infinity], max: [-Infinity, -Infinity, -Infinity] }, 1);
    expect(Number.isFinite(c.orthoHeight)).toBe(true);
    expect(c.target).toEqual([0, 0, 0]);
  });
  it("keeps clip planes around the model", () => {
    const c = fitCamera(cameraOf("perspective"), { min: [-1, 0, -1], max: [1, 2, 1] }, 1.3);
    const { near, far } = clipPlanes(c);
    expect(near).toBeGreaterThan(0);
    expect(near).toBeLessThan(c.distance - c.extent);
    expect(far).toBeGreaterThan(c.distance + c.extent);
  });
});

describe("orbit, pan, zoom", () => {
  const base: CameraState = { ...cameraOf("perspective"), target: [0, 1, 0], distance: 5 };
  it("orbits and clamps the pitch", () => {
    expect(orbit(base, 0.3, 0).yaw).toBeCloseTo(base.yaw + 0.3);
    expect(orbit(base, 0, 10).pitch).toBeCloseTo(Math.PI / 2);
    expect(orbit(base, 0, -10).pitch).toBeCloseTo(-Math.PI / 2);
  });
  it("pans in the picture plane by fractions of the visible height", () => {
    const p = pan(base, 0.1, 0.2, 2);
    const b = basisOf(base);
    const h = visibleHeight(base);
    const d = [p.target[0] - 0, p.target[1] - 1, p.target[2] - 0];
    close(d, [b.right[0] * 0.1 * h * 2 + b.up[0] * 0.2 * h, b.right[1] * 0.1 * h * 2 + b.up[1] * 0.2 * h, b.right[2] * 0.1 * h * 2 + b.up[2] * 0.2 * h], 1e-12);
  });
  it("zooms perspective by distance and orthographic by height", () => {
    expect(zoom(base, 2).distance).toBeCloseTo(2.5);
    const o = { ...cameraOf("front"), orthoHeight: 4 };
    expect(zoom(o, 2).orthoHeight).toBeCloseTo(2);
    expect(zoom(o, 2).distance).toBe(o.distance);
  });
});
