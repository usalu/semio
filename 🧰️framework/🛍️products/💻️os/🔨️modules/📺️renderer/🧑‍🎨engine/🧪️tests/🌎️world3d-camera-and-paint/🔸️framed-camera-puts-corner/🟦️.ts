/** 🔸️ The framed camera puts corner tests of the framework renderer hosts suite. */
import { expect, it } from "vitest";
import { worldVortexHitProxy, worldInstanceMeshRaycast, applyWorldInstanceMeshRaycast } from "../../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts";
import { world3dFrameCameraFromBounds, world3dBoundsRadius } from "../../../🧱️elements/🌐️World3dHost/🔭️scene-schema/🟦️.ts";
import { mergeWorldViewportCamera, resolveVortexPointerDownIntent, shouldReattachWorldViewportCamera, worldCameraPoseApproxEqual, buildWorldCameraDispatchArgs, worldCameraSetCameraDispatchArgs, worldInstancePickBlocked, parseWorldTerrainStyle } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🔹 Registers the framed camera puts corner tests of the framework renderer hosts suite. */
export function registerFramedCameraPutsCornerTests(): void {
  it("a framed camera puts every corner of the delivered box inside the viewport, for every bundled generation3d example", () => {
    const examples: readonly (readonly [string, readonly [number, number, number], readonly [number, number, number]])[] = [
      ["hexagonal-mushroom-column", [-0.5, -0.43301, 0], [0.5, 0.43301, 6]],
      ["rectangle-extrude-volume", [0, 0, 0], [2, 2, 3]],
      ["face-sweep-extrude", [0, 0, 0], [2, 1.5, 4]],
      ["box-shell-preview", [0, 0, 0], [2, 2, 2]],
      ["box-fillet-preview", [0, 0, 0], [2, 2, 2]],
      ["rectangle-wire-preview", [0, 0, 0], [2, 1.5, 0]],
      ["sphere-box-fuse", [-1.2, -1.2, -1.2], [1.5, 1.5, 1.5]],
      ["sphere-cut-with-torus", [-2.2, -2.2, -2.2], [2.2, 2.2, 2.2]],
    ];
    const seed = { position: [4, -4, 3] as [number, number, number], target: [0, 0, 0] as [number, number, number], zoom: 1, projection: "perspective" as const, fov: 45, explicitProjection: false, projectionFrame: "content" as const };
    for (const [name, minimum, maximum] of examples) {
      for (const aspect of [1.7, 1.0, 0.7]) {
        const center: [number, number, number] = [(minimum[0] + maximum[0]) / 2, (minimum[1] + maximum[1]) / 2, (minimum[2] + maximum[2]) / 2];
        const framed = world3dFrameCameraFromBounds(center, world3dBoundsRadius(minimum, maximum), seed, undefined, aspect);
        const forward = [framed.target[0] - framed.position[0], framed.target[1] - framed.position[1], framed.target[2] - framed.position[2]];
        const forwardLength = Math.hypot(...forward);
        const unitForward = forward.map((value) => value / forwardLength);
        const worldUp = [0, 0, 1];
        const right = [unitForward[1] * worldUp[2] - unitForward[2] * worldUp[1], unitForward[2] * worldUp[0] - unitForward[0] * worldUp[2], unitForward[0] * worldUp[1] - unitForward[1] * worldUp[0]];
        const rightLength = Math.hypot(...right);
        const unitRight = right.map((value) => value / rightLength);
        const unitUp = [unitRight[1] * unitForward[2] - unitRight[2] * unitForward[1], unitRight[2] * unitForward[0] - unitRight[0] * unitForward[2], unitRight[0] * unitForward[1] - unitRight[1] * unitForward[0]];
        const halfVertical = Math.tan(((seed.fov * Math.PI) / 180) * 0.5);
        for (const corner of [
          [minimum[0], minimum[1], minimum[2]],
          [minimum[0], minimum[1], maximum[2]],
          [minimum[0], maximum[1], minimum[2]],
          [minimum[0], maximum[1], maximum[2]],
          [maximum[0], minimum[1], minimum[2]],
          [maximum[0], minimum[1], maximum[2]],
          [maximum[0], maximum[1], minimum[2]],
          [maximum[0], maximum[1], maximum[2]],
        ]) {
          const relative = [corner[0] - framed.position[0], corner[1] - framed.position[1], corner[2] - framed.position[2]];
          const depth = relative[0] * unitForward[0] + relative[1] * unitForward[1] + relative[2] * unitForward[2];
          expect(depth).toBeGreaterThan(0);
          const ndcY = (relative[0] * unitUp[0] + relative[1] * unitUp[1] + relative[2] * unitUp[2]) / (depth * halfVertical);
          const ndcX = (relative[0] * unitRight[0] + relative[1] * unitRight[1] + relative[2] * unitRight[2]) / (depth * halfVertical * aspect);
          expect(`${name}@${aspect} x=${ndcX.toFixed(3)}`).toBe(`${name}@${aspect} x=${Math.max(Math.min(ndcX, 0.999), -0.999).toFixed(3)}`);
          expect(`${name}@${aspect} y=${ndcY.toFixed(3)}`).toBe(`${name}@${aspect} y=${Math.max(Math.min(ndcY, 0.999), -0.999).toFixed(3)}`);
        }
      }
    }
  });
  it("buildWorldCameraDispatchArgs carries position/target/zoom/up but never a projection field", () => {
    const withUp = buildWorldCameraDispatchArgs({ position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1], projection: "orthographic" });
    expect(withUp).toEqual({ position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1] });
    expect(withUp).not.toHaveProperty("projection");
    expect(withUp).not.toHaveProperty("projectionSpec");

    const withoutUp = buildWorldCameraDispatchArgs({ position: [1, 2, 3], target: [0, 0, 0], zoom: 1, projection: "perspective" });
    expect(withoutUp).toEqual({ position: [1, 2, 3], target: [0, 0, 0], zoom: 1 });
    expect(withoutUp).not.toHaveProperty("up");
    expect(withoutUp).not.toHaveProperty("projection");
  });
  it("worldCameraSetCameraDispatchArgs nests the camera pose under a `camera` key, never flat alongside windowId", () => {
    const args = worldCameraSetCameraDispatchArgs("puzzle.3d.play.viewport", { position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1], projection: "orthographic" });
    expect(args).toEqual({ windowId: "puzzle.3d.play.viewport", camera: { position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1] } });
    expect(args).not.toHaveProperty("position");
    expect(args).not.toHaveProperty("target");
    expect(args).not.toHaveProperty("zoom");
    expect(args).not.toHaveProperty("up");
  });
  it("worldCameraPoseApproxEqual matches exact poses and float-noise, rejects a genuinely different pose", () => {
    const base = { position: [1, 2, 3] as const, target: [0, 0, 0] as const, zoom: 1 };
    expect(worldCameraPoseApproxEqual(base, { position: [1, 2, 3], target: [0, 0, 0], zoom: 1 })).toBe(true);
    expect(worldCameraPoseApproxEqual(base, { position: [1.0000001, 2.0000001, 3], target: [0, 0, 1e-9], zoom: 1.0000001 })).toBe(true);
    expect(worldCameraPoseApproxEqual(base, { position: [9, 2, 3], target: [0, 0, 0], zoom: 1 })).toBe(false);
    expect(worldCameraPoseApproxEqual(base, { position: [1, 2, 3], target: [0, 0, 0], zoom: 5 })).toBe(false);
  });
  it("shouldReattachWorldViewportCamera suppresses a self-echo of the last dispatched camera but not a genuinely different pose", () => {
    const previous = '{"position":[1,2,3],"target":[0,0,0],"zoom":1}';
    const dispatched = { position: [4, 5, 6] as const, target: [0, 0, 0] as const, zoom: 2 };
    const echoedJson = '{"position":[4.0000001,5,6],"target":[0,0,0],"zoom":2}';
    const differentJson = '{"position":[9,9,9],"target":[0,0,0],"zoom":1}';
    expect(shouldReattachWorldViewportCamera(previous, echoedJson, dispatched)).toBe(false);
    expect(shouldReattachWorldViewportCamera(previous, differentJson, dispatched)).toBe(true);
    expect(shouldReattachWorldViewportCamera(previous, previous, dispatched)).toBe(false);
    expect(shouldReattachWorldViewportCamera(previous, differentJson, null)).toBe(true);
  });
  it("preserves projectionSpec.view from gizmo snaps instead of clobbering to top", () => {
    const merged = mergeWorldViewportCamera(
      { position: [0, 0, 10], target: [0, 0, 0], zoom: 50, projection: "orthographic", fov: 45, explicitProjection: true, projectionFrame: "content", projectionSpec: { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } } },
      { position: [0, -600, 0], target: [0, 0, 0], zoom: 50, projection: "orthographic", projectionSpec: { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } } },
    );
    expect(merged.projectionSpec).toEqual({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } });
  });
  it("accepts extended world 3d scene fields", () => {
    const node = {
      type: "componentScene",
      surfaceId: "puzzle.3d.play.viewport",
      controllerId: "puzzle3d-play",
      componentKind: "world-3d",
      world3d: {
        cameraJson: "{}",
        meshesJson: "[]",
        instancesJson: "[]",
        selectionJson: "{}",
        vorticesJson: "[]",
        attractionsJson: "[]",
        targetVolumesJson: "[]",
        referencesJson: "[]",
        brushPreviewJson: undefined,
        interactionJson: '{"activeUtility":"select"}',
        engagementPreviewJson: '[{"kind":"point","role":"origin","position":[0,0,0]},{"kind":"box-preview","role":"preview","cornerA":[0,0,0],"cornerB":[2,2,0]}]',
        statusJson: '{"computing":true,"label":"Evaluating"}',
        terrainJson: '{"tileUrlTemplate":"/dem/{z}/{x}/{y}.png","projectOriginLon":9.7382,"projectOriginLat":52.3759,"exaggeration":1.5,"colorRamp":"hypsometric","minZoom":6,"maxZoom":14}',
      },
    };
    expect(node.world3d?.meshesJson).toBe("[]");
    expect(node.world3d?.vorticesJson).toBe("[]");
    expect(node.world3d?.interactionJson).toContain("select");
    expect(node.world3d?.engagementPreviewJson).toContain("box-preview");
    expect(node.world3d?.statusJson).toContain("computing");
    expect(node.world3d?.terrainJson).toContain("hypsometric");
  });
  it("parses GIS 3D terrain style JSON, defaulting missing fields, and rejects a missing tileUrlTemplate", () => {
    expect(parseWorldTerrainStyle(undefined)).toBeNull();
    expect(parseWorldTerrainStyle("not json")).toBeNull();
    expect(parseWorldTerrainStyle('{"projectOriginLon":1}')).toBeNull();
    const style = parseWorldTerrainStyle('{"tileUrlTemplate":"/dem/{z}/{x}/{y}.png","projectOriginLon":9.7382,"projectOriginLat":52.3759,"exaggeration":2}');
    expect(style).toMatchObject({
      tileUrlTemplate: "/dem/{z}/{x}/{y}.png",
      projectOriginLon: 9.7382,
      projectOriginLat: 52.3759,
      exaggeration: 2,
      colorRamp: "hypsometric",
      minZoom: 6,
      maxZoom: 14,
    });
  });
  it("blocks instance picking for brush and volume brush engagements but not move", () => {
    expect(worldInstancePickBlocked("brush")).toBe(true);
    expect(worldInstancePickBlocked("volumeBrush")).toBe(true);
    expect(worldInstancePickBlocked("surfaceBrush")).toBe(true);
    expect(worldInstancePickBlocked("move")).toBe(false);
    expect(worldInstancePickBlocked(undefined)).toBe(false);
  });
  it("keeps the vortex hit proxy visible so Three's raycaster does not skip it", () => {
    expect(worldVortexHitProxy(0.36)).toEqual({ visible: true, radius: 0.36 });
    expect(worldVortexHitProxy(0.1).radius).toBeGreaterThanOrEqual(0.36);
    expect(worldVortexHitProxy().visible).toBe(true);
  });
  it("clears instance mesh raycasts when pick is blocked so sibling vortex markers stay hittable", () => {
    expect(worldInstanceMeshRaycast(true)).toBeUndefined();
    expect(worldInstanceMeshRaycast(false)).toEqual(expect.any(Function));
    const mesh = { isMesh: true, raycast: "keep" as unknown };
    const root = { traverse: (fn: (object: typeof mesh) => void) => fn(mesh) };
    applyWorldInstanceMeshRaycast(root, false, "keep");
    expect(mesh.raycast).toEqual(expect.any(Function));
    applyWorldInstanceMeshRaycast(root, true, "keep");
    expect(mesh.raycast).toBe("keep");
  });
  it("resolves vortex pointer-down to select in brush or vertex mode and click-or-drag otherwise", () => {
    expect(resolveVortexPointerDownIntent(true)).toBe("select");
    expect(resolveVortexPointerDownIntent(false, "vertex")).toBe("select");
    expect(resolveVortexPointerDownIntent(false)).toBe("click-or-drag");
    expect(resolveVortexPointerDownIntent(false, "mesh")).toBe("click-or-drag");
  });
}
