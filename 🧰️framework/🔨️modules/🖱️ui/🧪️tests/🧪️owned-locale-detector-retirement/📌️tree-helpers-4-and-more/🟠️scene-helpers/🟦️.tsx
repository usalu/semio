/** 🟠️ The scene helpers phase of registerTreeHelpers4AndMoreTests. */
import type { registerTreeHelpersTests } from "../🔶️tree-helpers/🟦️.tsx";

/** 🟡 Registers the scene helpers tests of registerTreeHelpers4AndMoreTests. */
export function registerSceneHelpersTests({ describe, it, expect, SCENE_GIZMO_LABELS, resolveSceneGizmoSnapTarget, THREE, resolveSceneGizmoViewportPlacement }: Pick<ReturnType<typeof registerTreeHelpersTests>, "describe" | "it" | "expect" | "SCENE_GIZMO_LABELS" | "resolveSceneGizmoSnapTarget" | "THREE" | "resolveSceneGizmoViewportPlacement">) {
    describe("scene helpers", () => {
      it("keeps the 3d gizmo label-free", () => {
        expect(SCENE_GIZMO_LABELS).toEqual(["", "", ""]);
      });
  
      it("maps dominant gizmo axes to blender-style orthographic snap targets", () => {
        expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(1, 0.2, 0.1))).toEqual({
          axis: "x",
          sign: 1,
          view: "side",
          cameraDirection: { x: 1, y: 0, z: 0 },
          up: { x: 0, y: 1, z: 0 },
        });
  
        expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(0.1, 1, 0.2))).toEqual({
          axis: "y",
          sign: 1,
          view: "top",
          cameraDirection: { x: 0, y: 1, z: 0 },
          up: { x: 0, y: 0, z: -1 },
        });
  
        expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(0.1, 0.2, -1))).toEqual({
          axis: "z",
          sign: -1,
          view: "back",
          cameraDirection: { x: 0, y: 0, z: -1 },
          up: { x: 0, y: 1, z: 0 },
        });
      });
  
      it("preserves the complementary blender views for negative axis clicks", () => {
        expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(-1, 0, 0))).toEqual({
          axis: "x",
          sign: -1,
          view: "opposite-side",
          cameraDirection: { x: -1, y: 0, z: 0 },
          up: { x: 0, y: 1, z: 0 },
        });
  
        expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(0, -1, 0))).toEqual({
          axis: "y",
          sign: -1,
          view: "bottom",
          cameraDirection: { x: 0, y: -1, z: 0 },
          up: { x: 0, y: 0, z: 1 },
        });
  
        expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(0, 0, 1))).toEqual({
          axis: "z",
          sign: 1,
          view: "front",
          cameraDirection: { x: 0, y: 0, z: 1 },
          up: { x: 0, y: 1, z: 0 },
        });
      });
  
      it("keeps the gizmo in the bottom-right corner above the folded projection pane chrome", () => {
        expect(resolveSceneGizmoViewportPlacement({ width: 1280, height: 720 })).toEqual({
          alignment: "bottom-right",
          margin: [32, 58],
        });
  
        expect(resolveSceneGizmoViewportPlacement({ width: 120, height: 160 })).toEqual({
          alignment: "bottom-right",
          margin: [32, 53],
        });
  
        expect(resolveSceneGizmoViewportPlacement({ width: 40, height: 48 })).toEqual({
          alignment: "bottom-right",
          margin: [22, 22],
        });
      });
    });
}
