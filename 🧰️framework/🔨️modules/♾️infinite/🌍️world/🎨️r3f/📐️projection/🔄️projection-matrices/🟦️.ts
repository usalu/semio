/** 🔄️ World projection matrices: the oblique shear, the goal and morph matrices, the transition pose and the gizmo face view. */
import { Matrix4, ThreeOrthographicCamera, ThreePerspectiveCamera } from "../../🔌️adapters/🟦️.ts";
import { type WorldProjectionMode, type WorldProjectionSpec } from "../../🔭️scene-types/🟦️.ts";
import { type Vec3, type WorldCameraState } from "../../🧷️r3f-foundation/🟦️.ts";
import { ORBIT_CAMERA_Z_UP, orbitCameraDistance, WORLD_ORBIT_CAMERA_MIN_FAR, worldProjectionMatchedOrthoZoom } from "../../📷️camera-view/🟦️.tsx";
import { applyWorldProjectionToCameraState, computeWorldProjectionPose, worldProjectionFamily, worldProjectionOrientationsEqual, worldProjectionPerspectiveFov, worldProjectionSnapZoom } from "../🧮️projection-pose/🟦️.ts";

// #region 📐️WorldProjection
/** 🪞️ Oblique receding-axis shear matrix — extracted from {@link WorldProjectionMatrixDriver} so
 * {@link worldProjectionGoalMatrix} and the live per-frame shear share one formula; `strength` ramps the shear 0→1. */
export function worldObliqueShearMatrix(mode: Extract<WorldProjectionMode, { kind: "oblique" }>, strength = 1): Matrix4 {
  const angle = mode.angle * (Math.PI / 180);
  const l = mode.variant === "military" ? 1 : mode.depthScale;
  const alpha = mode.variant === "military" ? Math.PI / 2 : angle;
  const shear = new Matrix4().identity();
  shear.elements[8] = -l * Math.cos(alpha) * strength;
  shear.elements[9] = -l * Math.sin(alpha) * strength;
  return shear;
}

/** 📐️ Destination projection matrix for `spec` at the live viewport size — the "goal" a projection-matrix morph
 * lerps toward each frame. Built from real three.js camera classes so it matches exactly what mounting that camera would
 * produce (drei's `OrthographicCamera`/`PerspectiveCamera` seed left/right/top/bottom or aspect the same way). */
export function worldProjectionGoalMatrix(spec: WorldProjectionSpec, options: { readonly zoom: number; readonly fov?: number; readonly viewport: { readonly width: number; readonly height: number }; readonly near?: number; readonly far?: number }): Matrix4 {
  const near = options.near ?? 0.2;
  const far = options.far ?? WORLD_ORBIT_CAMERA_MIN_FAR;
  const { width, height } = options.viewport;
  if (worldProjectionFamily(spec) === "parallel") {
    const camera = new ThreeOrthographicCamera(width / -2, width / 2, height / 2, height / -2, near, far);
    camera.zoom = options.zoom;
    camera.updateProjectionMatrix();
    const matrix = camera.projectionMatrix.clone();
    return spec.mode.kind === "oblique" ? matrix.multiply(worldObliqueShearMatrix(spec.mode)) : matrix;
  }
  const fov = options.fov ?? worldProjectionPerspectiveFov(spec);
  const camera = new ThreePerspectiveCamera(fov, width / Math.max(1, height), near, far);
  camera.zoom = options.zoom;
  camera.updateProjectionMatrix();
  const matrix = camera.projectionMatrix.clone();
  if (spec.mode.kind === "twoPoint") {
    matrix.elements[9] += spec.mode.verticalShift;
  }
  return matrix;
}

/** 🎞️ Element-wise projection-matrix lerp — valid across every pair this engine morphs between: persp↔ortho
 * lerps the w-row (`-z ↔ 1`, the standard projective blend), persp↔persp is cot-space FOV interpolation, and
 * oblique-shear/two-point-shift elements ramp linearly since they're already linear terms of the base matrix. */
export function worldProjectionMorphMatrix(from: Matrix4, to: Matrix4, t: number): Matrix4 {
  const result = new Matrix4();
  for (let i = 0; i < 16; i++) {
    const start=from.elements[i],end=to.elements[i];if(start===undefined||end===undefined)throw Error("Complete world projection matrix required");result.elements[i]=start+(end-start)*t;
  }
  return result;
}

/** 🎞️ Destination pose for a projection snap — preserves eye exactly when only mode changes (matches
 * {@link applyWorldProjectionToCameraState}'s "pure parameter tweaks never move the camera" invariant); re-looks only
 * when orientation changes. When `live.viewport` is supplied, a persp→ortho mode-only switch picks a zoom that matches
 * the live apparent scale via {@link worldProjectionMatchedOrthoZoom} instead of the legacy zoom-50 default — this only
 * changes *which* zoom value is chosen, never the camera position, so it cannot reposition the camera. */
export function worldProjectionTransitionPose(
  pendingSpec: WorldProjectionSpec,
  live: {
    readonly position: Vec3;
    readonly target: Vec3;
    readonly up?: Vec3;
    readonly zoom: number;
    readonly isOrthographic: boolean;
    readonly projectionSpec?: WorldProjectionSpec;
    readonly viewport?: { readonly width: number; readonly height: number };
    readonly fov?: number;
  },
): WorldCameraState {
  const previousSpec = live.projectionSpec;
  const orientationUnchanged = previousSpec !== undefined && worldProjectionOrientationsEqual(previousSpec.orientation, pendingSpec.orientation);
  if (orientationUnchanged && previousSpec && live.viewport && worldProjectionFamily(previousSpec) === "perspective" && worldProjectionFamily(pendingSpec) === "parallel") {
    const fromFov = live.fov ?? worldProjectionPerspectiveFov(previousSpec);
    const distance = orbitCameraDistance({ position: live.position, target: live.target, zoom: live.zoom });
    const zoom = worldProjectionMatchedOrthoZoom(fromFov, distance, live.viewport.height);
    return applyWorldProjectionToCameraState({ position: live.position, target: live.target, up: live.up ?? ORBIT_CAMERA_Z_UP, zoom, projection: "orthographic", projectionSpec: pendingSpec }, pendingSpec);
  }
  const zoom = worldProjectionSnapZoom(pendingSpec, live.zoom, live.isOrthographic);
  if (orientationUnchanged || !previousSpec) {
    return applyWorldProjectionToCameraState(
      {
        position: live.position,
        target: live.target,
        up: live.up ?? ORBIT_CAMERA_Z_UP,
        zoom,
        projection: live.isOrthographic ? "orthographic" : "perspective",
        projectionSpec: pendingSpec,
      },
      pendingSpec,
    );
  }
  const distance = orbitCameraDistance({ position: live.position, target: live.target, zoom: live.zoom });
  return computeWorldProjectionPose(pendingSpec, { target: live.target, distance, zoom });
}
// #endregion 📐️WorldProjection
