/** 🧊️ The axonometric camera: the one projection of the 2D helpers, the depiction and the CAD scene, against hand-computed vectors, three.js orthographic camera matrices (test-only) and the cameras of the planar views and of the kernel viewer's `axonometric` preset.
 *
 * THE REFERENCES are `🧫️fixtures/🧊️axonometric-projection` (hand-computed), three's `OrthographicCamera` with `lookAt` (the neutral frame is three's frame: right-handed, +Y up), `CAMERAS` of the planar views and the viewer's `viewMatrix`.
 * Tolerance: 1e-12 against the hand-computed floats, 1e-9 against three (float64 matrix products).
 *
 * @see ./🥒️.feature — the scenarios
 */
import { OrthographicCamera, Vector3 } from "three";
import { describe, expect, it } from "vitest";
import { cameraOf as viewerCamera, viewMatrix } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/📷️camera/🟦️.ts";
import type { AxonometricSpec, Vec3 } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { CAMERAS, axonometricCamera, nearness, project, projectAxonometric } from "../../🔨️modules/🎨️depiction/🔭️views/🟦️.ts";
import { ModelRequestError } from "../../🔨️modules/🧊️volume/🚫️refusal/🟦️.ts";
import { fixture } from "../🧰️support/🧰️.ts";

type Case = { name: string; spec: AxonometricSpec; azimuthDeg: number; elevationDeg: number; right: number[]; up: number[]; toward: number[]; normalisation: number; axisScale: { x: number; y: number; z: number } };
type Vectors = { cameras: Case[]; points: { camera: string; world: number[]; paper: number[]; depth: number }[] };
const vectors = fixture<Vectors>("🧊️axonometric-projection");
const v3 = (a: readonly number[]): Vec3 => ({ x: a[0]!, y: a[1]!, z: a[2]! });
const near = (a: readonly number[], b: readonly number[], eps: number, what: string): void => void a.forEach((x, i) => expect(Math.abs(x - b[i]!), `${what}: ${a} vs ${b}`).toBeLessThanOrEqual(eps));
const scatter: Vec3[] = Array.from({ length: 40 }, (_, i) => ({ x: Math.sin(i * 1.7) * 3, y: Math.cos(i * 2.3) * 2 + 1, z: Math.sin(i * 0.9 + 1) * 1.5 }));

describe("🧊️ the axonometric camera", () => {
  for (const c of vectors.cameras)
    it(`${c.name}: the axes, the normalisation and the axis scales equal the hand-computed vectors`, () => {
      const camera = axonometricCamera(c.spec);
      expect(camera.azimuthDeg).toBeCloseTo(c.azimuthDeg, 12);
      expect(camera.elevationDeg).toBeCloseTo(c.elevationDeg, 12);
      near([camera.right.x, camera.right.y, camera.right.z], c.right, 1e-12, "right");
      near([camera.up.x, camera.up.y, camera.up.z], c.up, 1e-12, "up");
      near([camera.toward.x, camera.toward.y, camera.toward.z], c.toward, 1e-12, "toward");
      expect(camera.normalisation).toBeCloseTo(c.normalisation, 12);
      near([camera.axisScale.x, camera.axisScale.y, camera.axisScale.z], [c.axisScale.x, c.axisScale.y, c.axisScale.z], 1e-12, "axis scale");
      near([camera.looking.x, camera.looking.y, camera.looking.z], c.toward.map((x) => -x), 1e-12, "looking");
    });

  it("projects the hand-computed points, the depth towards the viewer included", () => {
    for (const p of vectors.points) {
      const c = vectors.cameras.find((candidate) => candidate.name === p.camera)!;
      const q = projectAxonometric(axonometricCamera(c.spec), v3(p.world));
      near([q.x, q.y, q.depth], [...p.paper, p.depth], 1e-12, `${p.camera} ${p.world}`);
    }
  });

  it("is to scale along the axes it says it is: isometric 1:1:1, dimetric 1:1:1/2 (ahead, up, left), the vertical axis always", () => {
    const iso = axonometricCamera({ kind: "isometric" }).axisScale;
    near([iso.x, iso.y, iso.z], [1, 1, 1], 1e-12, "isometric");
    const dim = axonometricCamera({ kind: "dimetric" }).axisScale;
    near([dim.x, dim.y, dim.z], [1, 1, 0.5], 1e-12, "dimetric");
    for (const [azimuthDeg, elevationDeg] of [[10, 5], [-120, 40], [75, 75], [0, 0]]) expect(axonometricCamera({ kind: "free", azimuthDeg, elevationDeg }).axisScale.y).toBeCloseTo(1, 12);
    const unit = (axis: Vec3): number => Math.hypot(project("axonometric", axis).x, project("axonometric", axis).y);
    expect(unit({ x: 1, y: 0, z: 0 })).toBeCloseTo(1, 12);
    expect(unit({ x: 0, y: 1, z: 0 })).toBeCloseTo(1, 12);
    const iso30 = project("axonometric", { x: 1, y: 0, z: 0 });
    expect(Math.atan2(iso30.y, iso30.x) * (180 / Math.PI)).toBeCloseTo(-30, 10);
    const lengthOfDiagonal = Math.hypot(project("axonometric", { x: 1, y: 0, z: 1 }).x, project("axonometric", { x: 1, y: 0, z: 1 }).y);
    expect(lengthOfDiagonal, "a diagonal is not to scale").not.toBeCloseTo(Math.SQRT2, 3);
  });

  for (const c of vectors.cameras)
    it(`${c.name}: equals three's OrthographicCamera view (lookAt in the right-handed neutral frame) on ${scatter.length} points`, () => {
      const spec = axonometricCamera(c.spec);
      const eye = new Vector3(spec.toward.z, spec.toward.y, spec.toward.x).multiplyScalar(10);
      const three = new OrthographicCamera(-5, 5, 5, -5, 0.1, 100);
      three.position.copy(eye);
      three.up.set(0, 1, 0);
      three.lookAt(0, 0, 0);
      three.updateMatrixWorld(true);
      for (const p of scatter) {
        const inCamera = new Vector3(p.z, p.y, p.x).applyMatrix4(three.matrixWorldInverse);
        const ours = projectAxonometric(spec, p);
        near([ours.x, ours.y, ours.depth], [spec.normalisation * inCamera.x, spec.normalisation * inCamera.y, spec.normalisation * (inCamera.z + 10)], 1e-9, `${c.name} ${JSON.stringify(p)}`);
      }
    });

  it("contains the planar cameras: the side elevation at azimuth 90 / elevation 0, the front at 0 / 0, with the nearness of their looking direction", () => {
    for (const [viewpoint, azimuthDeg] of [["elevation", 90], ["front", 0]] as const) {
      const camera = axonometricCamera({ kind: "free", azimuthDeg, elevationDeg: 0 });
      for (const p of scatter) {
        const planar = project(viewpoint, p);
        const q = project("axonometric", p, camera);
        near([q.x, q.y], [planar.x, planar.y], 1e-12, viewpoint);
        expect(nearness("axonometric", p, camera)).toBeCloseTo(nearness(viewpoint, p), 12);
      }
      near([camera.looking.x, camera.looking.y, camera.looking.z], [CAMERAS[viewpoint].looking.x, CAMERAS[viewpoint].looking.y, CAMERAS[viewpoint].looking.z], 1e-12, `${viewpoint} looking`);
    }
  });

  it("is the isometric preset of the kernel viewer: its view matrix maps every point to the same picture up to the normalisation", () => {
    const preset = viewMatrix(viewerCamera("axonometric")) as number[];
    const spec = axonometricCamera({ kind: "isometric" });
    const eye = viewerCamera("axonometric");
    expect(eye.yaw).toBeCloseTo(-spec.azimuthDeg * (Math.PI / 180), 12);
    expect(eye.pitch).toBeCloseTo(spec.elevationDeg * (Math.PI / 180), 12);
    for (const p of scatter) {
      const n = [p.z, p.y, p.x];
      const x = preset[0]! * n[0]! + preset[4]! * n[1]! + preset[8]! * n[2]!;
      const y = preset[1]! * n[0]! + preset[5]! * n[1]! + preset[9]! * n[2]!;
      const ours = projectAxonometric(spec, p);
      near([ours.x, ours.y], [spec.normalisation * x, spec.normalisation * y], 1e-9, "viewer isometric");
    }
  });

  it("refuses angles outside the documented range with the bilingual camera refusals", () => {
    expect(() => axonometricCamera({ kind: "free", azimuthDeg: 181, elevationDeg: 10 })).toThrow(/camera-/u);
    expect(() => axonometricCamera({ kind: "free", azimuthDeg: 0, elevationDeg: -1 })).toThrow(/camera-/u);
    expect(() => axonometricCamera({ kind: "free", azimuthDeg: 0, elevationDeg: 76 })).toThrow(/camera-/u);
    expect(() => axonometricCamera({ kind: "free", azimuthDeg: Number.NaN, elevationDeg: 10 })).toThrow(/camera-/u);
    const refusal = (() => { try { axonometricCamera({ kind: "free", azimuthDeg: 0, elevationDeg: 99 }); } catch (error) { return error as ModelRequestError; } throw new Error("no refusal"); })();
    expect(refusal.reason.en.length * refusal.reason.de.length).toBeGreaterThan(0);
  });
});
