import * as THREE from "three";
import { uiSpacingPx } from "@semio-tech/ui-styling";

/**
 * 🧭️ GizmoProps holds the data fields for a GizmoProps record.
 **/
export interface GizmoProps {
  show?: boolean;
  onAxisClick?: (direction: THREE.Vector3) => void;
}

export const SCENE_GIZMO_LABELS: [string, string, string] = ["", "", ""];

export type SceneProjectionKind = "camera" | "orthographic";

type SceneSnapViewKind = "front" | "back" | "side" | "opposite-side" | "top" | "bottom";

export interface SceneGizmoSnapTarget {
  axis: "x" | "y" | "z";
  sign: 1 | -1;
  view: SceneSnapViewKind;
  cameraDirection: {
    x: number;
    y: number;
    z: number;
  };
  up: {
    x: number;
    y: number;
    z: number;
  };
}

interface SceneGizmoViewportPlacement {
  alignment: "top-left" | "top-right" | "bottom-left" | "bottom-right";
  margin: [number, number];
}

/**
 * 🗃️ resolveSceneGizmoSnapTarget holds the data fields for a resolveSceneGizmoSnapTarget record.
 **/
export const resolveSceneGizmoSnapTarget = (direction: Pick<THREE.Vector3, "x" | "y" | "z">): SceneGizmoSnapTarget => {
  const dominantAxis = [
    { axis: "x" as const, magnitude: Math.abs(direction.x), raw: direction.x },
    { axis: "y" as const, magnitude: Math.abs(direction.y), raw: direction.y },
    { axis: "z" as const, magnitude: Math.abs(direction.z), raw: direction.z },
  ].sort((a, b) => b.magnitude - a.magnitude)[0] ?? { axis: "x" as const, magnitude: 1, raw: 1 };
  const sign = dominantAxis.raw >= 0 ? 1 : -1;

  if (dominantAxis.axis === "x") {
    return {
      axis: "x",
      sign,
      view: sign > 0 ? "side" : "opposite-side",
      cameraDirection: { x: sign, y: 0, z: 0 },
      up: { x: 0, y: 1, z: 0 },
    };
  }

  if (dominantAxis.axis === "y") {
    return {
      axis: "y",
      sign,
      view: sign > 0 ? "top" : "bottom",
      cameraDirection: { x: 0, y: sign, z: 0 },
      up: sign > 0 ? { x: 0, y: 0, z: -1 } : { x: 0, y: 0, z: 1 },
    };
  }

  return {
    axis: "z",
    sign,
    view: sign > 0 ? "front" : "back",
    cameraDirection: { x: 0, y: 0, z: sign },
    up: { x: 0, y: 1, z: 0 },
  };
};

/**
 * 📇️ resolveSceneGizmoViewportPlacement holds the data fields for a resolveSceneGizmoViewportPlacement record.
 **/
export const resolveSceneGizmoViewportPlacement = (viewport: { width: number; height: number }): SceneGizmoViewportPlacement => {
  const chromeInset = Math.max(4, Math.round(uiSpacingPx(1)));
  const foldedPaneChromePx = Math.round(uiSpacingPx(7));
  const gizmoHalfExtentPx = 28;
  const preferredX = chromeInset + gizmoHalfExtentPx;
  const preferredY = chromeInset + foldedPaneChromePx + chromeInset + gizmoHalfExtentPx;
  const maxFitX = Math.max(22, Math.floor(viewport.width / 3));
  const maxFitY = Math.max(22, Math.floor(viewport.height / 3));
  return {
    alignment: "bottom-right",
    margin: [Math.min(preferredX, maxFitX), Math.min(preferredY, maxFitY)],
  };
};
