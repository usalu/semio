/** 🔭️ The projection vocabulary: families, orthographic views, axonometric and oblique variants, curvilinear mappings, orientations, modes, specs, kinds and the projection gizmo hit.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | r3f}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 📷️OrbitCameraView
//#region 🧭️WorldOrbitViewGizmoViewport
export type ProjectionGizmoHit =
  | { readonly type: "face"; readonly axis: "x" | "y" | "z"; readonly sign: 1 | -1 }
  | { readonly type: "corner"; readonly quadrant: WorldAxonometricQuadrant; readonly hemisphere: WorldAxonometricHemisphere }
  | { readonly type: "center" };
//#endregion 🧭️WorldOrbitViewGizmoViewport
// #endregion 📷️OrbitCameraView

// #region 📐️WorldProjection
/** 📐️ Parallel vs. perspective camera family a {@link WorldProjectionSpec} belongs to. */
export type WorldProjectionFamily = "parallel" | "perspective";

export type WorldOrthographicViewId = "plan" | "top" | "bottom" | "front" | "back" | "left" | "right";
export type WorldAxonometricVariant = "isometric" | "dimetric" | "trimetric";
export type WorldAxonometricQuadrant = "ne" | "nw" | "se" | "sw";
export type WorldAxonometricHemisphere = "upper" | "lower";
export type WorldObliqueVariant = "cabinet" | "cavalier" | "military";
export type WorldCurvilinearMapping = "fisheye" | "panini";

/** 🧭️ Spatial look the navigation cube owns — cardinal faces, axonometric corners, or free 3D. */
export type WorldProjectionOrientation =
  | { readonly type: "cardinal"; readonly view: WorldOrthographicViewId }
  | { readonly type: "corner"; readonly quadrant: WorldAxonometricQuadrant; readonly hemisphere?: WorldAxonometricHemisphere }
  | { readonly type: "free" };

/** 📐️ Projection *mode* the pane owns (kind + non-spatial params). Composes with {@link WorldProjectionOrientation}. */
export type WorldProjectionMode =
  | { readonly kind: "orthographic" }
  | { readonly kind: "axonometric"; readonly variant: WorldAxonometricVariant; readonly angleA: number; readonly angleB: number }
  | { readonly kind: "oblique"; readonly variant: WorldObliqueVariant; readonly angle: number; readonly depthScale: number }
  | { readonly kind: "onePoint"; readonly fov: number }
  | { readonly kind: "twoPoint"; readonly fov: number; readonly verticalShift: number }
  | { readonly kind: "threePoint"; readonly fov: number }
  | { readonly kind: "curvilinear"; readonly fov: number; readonly strength: number; readonly mapping: WorldCurvilinearMapping };

/** 📐️ Full classical projection taxonomy as mode ⊗ orientation — every mode works with every gizmo view.
 * See https://en.wikipedia.org/wiki/Axonometric_projection and https://en.wikipedia.org/wiki/Oblique_projection. */
export type WorldProjectionSpec = {
  readonly mode: WorldProjectionMode;
  readonly orientation: WorldProjectionOrientation;
};

export type WorldProjectionKind = WorldProjectionMode["kind"];
// #endregion 📐️WorldProjection
