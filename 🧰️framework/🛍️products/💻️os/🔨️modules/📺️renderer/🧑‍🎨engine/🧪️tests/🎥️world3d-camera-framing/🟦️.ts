/** @emoji 🎥️ Actual Three Box3 oracle for the neutral retained World3d camera-framing fixture. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { Box3, BufferGeometry, EdgesGeometry, Float32BufferAttribute, Group, LineBasicMaterial, LineSegments, Mesh, MeshBasicMaterial, OrthographicCamera, PerspectiveCamera, Quaternion, Vector3 } from "three";
import { describe, expect, it } from "vitest";
import { frameWorldProjectionPose, worldSceneContentBounds } from "../../../../♾️infinite/🌍️world/🎨️r3f/🟦️.tsx";
import { world3dAutoFitKey, world3dAutoFitOwed, world3dBoundsRadius, world3dFrameCameraFromBounds } from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧪️fixtures/🎥️world3d-camera-framing/🔣️.json"), "utf8")) as any;
const closeArray = (actual: readonly number[], expected: readonly number[], precision = 5) => actual.forEach((value, index) => expect(value).toBeCloseTo(expected[index]!, precision));

type FixtureCamera = { position: [number, number, number]; target: [number, number, number]; up: [number, number, number]; fov: number; zoom: number; projection: unknown };

function actualThreeBounds(): Box3 {
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new Float32BufferAttribute(fixture.mesh.data.positions, 3));
  geometry.setIndex(fixture.mesh.data.indices);
  const group = new Group();
  for (const record of fixture.instances) {
    const instance = new Group();
    const mesh = new Mesh(geometry, new MeshBasicMaterial());
    const outline = new LineSegments(new EdgesGeometry(geometry), new LineBasicMaterial());
    outline.scale.setScalar(fixture.renderedOutlineScale);
    mesh.add(outline);
    instance.position.fromArray(record.position);
    instance.quaternion.copy(new Quaternion(...(record.rotation as [number, number, number, number])));
    instance.scale.fromArray(record.scale);
    instance.add(mesh);
    group.add(instance);
  }
  group.updateMatrixWorld(true);
  return new Box3().setFromObject(group);
}

function framed(camera: FixtureCamera) {
  const bounds = actualThreeBounds();
  const center = bounds.getCenter(new Vector3()).toArray() as [number, number, number];
  const radius = world3dBoundsRadius(bounds.min.toArray(), bounds.max.toArray());
  const projection = camera.projection as string | { mode?: { kind?: string } };
  const parallel = projection === "orthographic" || (typeof projection === "object" && projection.mode?.kind === "orthographic");
  const sceneCamera = parallel ? new OrthographicCamera() : new PerspectiveCamera(camera.fov, fixture.viewport[0] / fixture.viewport[1]);
  const aspect = "aspect" in sceneCamera ? sceneCamera.aspect : 1;
  // 📐️ The fixture's `projection` is the full projection SPEC; a camera state's own `projection`
  // is the orbit mode the spec resolves to, which is the same `parallel` decision the scene camera
  // above is built from.
  return world3dFrameCameraFromBounds(center, radius, { ...camera, projection: parallel ? "orthographic" : "perspective", explicitProjection: camera.projection !== undefined }, fixture.fit.padding, aspect);
}

describe("🎥️ world3d current React camera framing", () => {
  it("measures transformed resident meshes with actual Three Box3", () => {
    const bounds = actualThreeBounds();
    closeArray(bounds.min.toArray(), fixture.expect.boundsMin);
    closeArray(bounds.max.toArray(), fixture.expect.boundsMax);
    closeArray(bounds.getCenter(new Vector3()).toArray(), fixture.expect.center);
    expect(world3dBoundsRadius(bounds.min.toArray(), bounds.max.toArray())).toBeCloseTo(fixture.expect.radius, 6);
  });

  it("frames delivered, three-point-template, and orthographic cameras while preserving projection zoom", () => {
    const perspective = framed(fixture.cameras.perspective);
    const threePoint = framed(fixture.cameras.perspectiveThreePoint);
    const top = framed(fixture.cameras.orthographicTop);
    closeArray(perspective.position, fixture.expect.perspective.position);
    closeArray(perspective.target, fixture.expect.perspective.target);
    closeArray(threePoint.position, fixture.expect.perspectiveThreePoint.position);
    closeArray(threePoint.target, fixture.expect.perspectiveThreePoint.target);
    closeArray(top.position, fixture.expect.orthographicTop.position);
    closeArray(top.target, fixture.expect.orthographicTop.target);
    expect(top.zoom).toBe(fixture.cameras.orthographicTop.zoom);
    expect(Math.hypot(...top.position.map((value, axis) => value - top.target[axis]!))).toBeCloseTo(fixture.expect.orthographicTop.distance, 5);
  });

  it("reframes a Top projection when raw instances arrive before loaded-mesh auto-fit", () => {
    const rawBounds = worldSceneContentBounds(fixture.instances);
    expect(rawBounds).not.toBeNull();
    const contentPose = frameWorldProjectionPose(fixture.cameras.orthographicTop.projection, rawBounds!, {
      viewportWidth: fixture.viewport[0],
      viewportHeight: fixture.viewport[1],
    });
    const loaded = framed({ ...fixture.cameras.orthographicTop, ...contentPose });
    closeArray(loaded.position, fixture.expect.orthographicTopAfterRawContent.position);
    closeArray(loaded.target, fixture.expect.orthographicTopAfterRawContent.target);
    expect(loaded.zoom).toBeCloseTo(fixture.expect.orthographicTopAfterRawContent.zoom, 6);
  });

  it("keys ownership by revision and external camera seed, and never re-arms for mesh roster churn", () => {
    const seed = JSON.stringify(fixture.cameras.perspective);
    const key = world3dAutoFitKey(fixture.fit.revision, seed, null);
    expect(world3dAutoFitOwed("", key, false)).toBe(true);
    expect(world3dAutoFitOwed(key, key, false)).toBe(false);
    expect(world3dAutoFitOwed("", key, true)).toBe(false);
    expect(world3dAutoFitKey(fixture.fit.revision, seed, null)).toBe(key);
    expect(world3dAutoFitKey(fixture.fit.revision + 1, seed, null)).not.toBe(key);
    expect(world3dAutoFitKey(fixture.fit.revision, JSON.stringify(fixture.cameras.orthographicTop), null)).not.toBe(key);
  });
});
