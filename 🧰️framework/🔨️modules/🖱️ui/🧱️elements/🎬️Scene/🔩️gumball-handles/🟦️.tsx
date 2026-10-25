import { themeColorVar, tokenVar } from "@semio-tech/ui-styling";
import * as THREE from "three";
import * as React from "react";
import { type GumballHandleKind, type GumballVec3, gumballAxisScaleFactor, gumballV3 } from "../🧿️gumball-model/🟦️.ts";

export const GUMBALL_PLANE_COLOR_REF = themeColorVar("muted-foreground");
export const GUMBALL_UNIFORM_COLOR_REF = tokenVar("light");
export const GUMBALL_HANDLE_LENGTH = 0.98;
export const GUMBALL_ARROW_RADIUS = 0.022;
export const GUMBALL_ARROW_HEAD = 0.12;
export const GUMBALL_ARROW_HEAD_RADIUS_SCALE = 1.75;
export const GUMBALL_PLANE_OFFSET = 0.3;
export const GUMBALL_PLANE_SIZE = 0.145;
const GUMBALL_PLANE_SCALE_INSET = 0.04;
const GUMBALL_PLANE_SCALE_ARM = 0.085;
const GUMBALL_PLANE_SCALE_THICK = 0.012;
const GUMBALL_PLANE_SCALE_DEPTH = 0.012;
const GUMBALL_PLANE_SCALE_PICK = 0.12;
export const GUMBALL_RING_RADIUS = 1.06;
export const GUMBALL_RING_TUBE = 0.01;
const GUMBALL_SCALE_OFFSET = 1.45;
const GUMBALL_SCALE_SHAFT_RADIUS = 0.007;
const GUMBALL_SCALE_BOX = 0.055;
export const GUMBALL_SCALE_PICK = 0.14;
export const GUMBALL_UNIFORM_BOX = 0.068;
export const GUMBALL_SCALE_RENDER_ORDER = 1001;

export const GUMBALL_AXIS_BY_KIND: Readonly<Record<GumballHandleKind, GumballVec3 | null>> = {
  moveX: gumballV3(1, 0, 0),
  moveY: gumballV3(0, 1, 0),
  moveZ: gumballV3(0, 0, 1),
  moveXY: gumballV3(0, 0, 1),
  moveYZ: gumballV3(1, 0, 0),
  moveXZ: gumballV3(0, 1, 0),
  rotateX: gumballV3(1, 0, 0),
  rotateY: gumballV3(0, 1, 0),
  rotateZ: gumballV3(0, 0, 1),
  scaleX: gumballV3(1, 0, 0),
  scaleY: gumballV3(0, 1, 0),
  scaleZ: gumballV3(0, 0, 1),
  scaleXY: gumballV3(0, 0, 1),
  scaleYZ: gumballV3(1, 0, 0),
  scaleXZ: gumballV3(0, 1, 0),
  scaleUniform: null,
};

export type GumballScalePlane = "xy" | "yz" | "xz";

/** 🥢️ Local scale axis pair for a 2D plane scale handle. */
export function gumballScalePlaneAxisIndices(kind: GumballHandleKind): readonly [0 | 1 | 2, 0 | 1 | 2] | null {
  if (kind === "scaleXY") return [0, 1];
  if (kind === "scaleYZ") return [1, 2];
  if (kind === "scaleXZ") return [0, 2];
  return null;
}

/** 📐️ Inner corner of an L-shaped plane scale bracket (beyond the move plane square). */
export function gumballPlaneScaleCorner(plane: GumballScalePlane): GumballVec3 {
  const c = GUMBALL_PLANE_OFFSET + GUMBALL_PLANE_SIZE * 0.5 + GUMBALL_PLANE_SCALE_INSET;
  if (plane === "xy") return [c, c, 0];
  if (plane === "yz") return [0, c, c];
  return [c, 0, c];
}

export function gumballWorldPointToLocalOffset(point: GumballVec3, pivot: GumballVec3, quat: THREE.Quaternion): GumballVec3 {
  const v = new THREE.Vector3(point[0] - pivot[0], point[1] - pivot[1], point[2] - pivot[2]);
  v.applyQuaternion(quat.clone().invert());
  return [v.x, v.y, v.z];
}

/** ✖️ Scale factors for a 2D plane scale drag; uniform uses radial distance in the plane. */
export function gumballPlaneScaleFactors(startLocal: readonly [number, number], currentLocal: readonly [number, number], uniform = false): readonly [number, number] {
  if (uniform) {
    const factor = gumballAxisScaleFactor(Math.hypot(startLocal[0], startLocal[1]), Math.hypot(currentLocal[0], currentLocal[1]));
    return [factor, factor];
  }
  return [gumballAxisScaleFactor(startLocal[0], currentLocal[0]), gumballAxisScaleFactor(startLocal[1], currentLocal[1])];
}

export function GumballPlaneScaleLHandle(props: { readonly plane: GumballScalePlane }): React.ReactElement {
  const [cx, cy, cz] = gumballPlaneScaleCorner(props.plane);
  const arm = GUMBALL_PLANE_SCALE_ARM;
  const thick = GUMBALL_PLANE_SCALE_THICK;
  const depth = GUMBALL_PLANE_SCALE_DEPTH;
  const pick = GUMBALL_PLANE_SCALE_PICK;
  if (props.plane === "xy") {
    return (
      <>
        <mesh position={[cx + arm * 0.5, cy, cz]} renderOrder={GUMBALL_SCALE_RENDER_ORDER}>
          <boxGeometry args={[arm, thick, depth]} />
        </mesh>
        <mesh position={[cx, cy + arm * 0.5, cz]} renderOrder={GUMBALL_SCALE_RENDER_ORDER}>
          <boxGeometry args={[thick, arm, depth]} />
        </mesh>
        <mesh position={[cx + arm * 0.5, cy + arm * 0.5, cz]} visible={false} userData={{ gumballHandlePick: true }}>
          <boxGeometry args={[pick, pick, depth * 2]} />
        </mesh>
      </>
    );
  }
  if (props.plane === "yz") {
    return (
      <>
        <mesh position={[cx, cy + arm * 0.5, cz]} renderOrder={GUMBALL_SCALE_RENDER_ORDER}>
          <boxGeometry args={[depth, arm, thick]} />
        </mesh>
        <mesh position={[cx, cy, cz + arm * 0.5]} renderOrder={GUMBALL_SCALE_RENDER_ORDER}>
          <boxGeometry args={[depth, thick, arm]} />
        </mesh>
        <mesh position={[cx, cy + arm * 0.5, cz + arm * 0.5]} visible={false} userData={{ gumballHandlePick: true }}>
          <boxGeometry args={[depth * 2, pick, pick]} />
        </mesh>
      </>
    );
  }
  return (
    <>
      <mesh position={[cx + arm * 0.5, cy, cz]} renderOrder={GUMBALL_SCALE_RENDER_ORDER}>
        <boxGeometry args={[arm, depth, thick]} />
      </mesh>
      <mesh position={[cx, cy, cz + arm * 0.5]} renderOrder={GUMBALL_SCALE_RENDER_ORDER}>
        <boxGeometry args={[thick, depth, arm]} />
      </mesh>
      <mesh position={[cx + arm * 0.5, cy, cz + arm * 0.5]} visible={false} userData={{ gumballHandlePick: true }}>
        <boxGeometry args={[pick, depth * 2, pick]} />
      </mesh>
    </>
  );
}

export type GumballPreviewAxis = "x" | "y" | "z";

function gumballScaleShaftMidpoint(): number {
  return (GUMBALL_RING_RADIUS + GUMBALL_SCALE_OFFSET) / 2;
}

function gumballScaleShaftLength(): number {
  return GUMBALL_SCALE_OFFSET - GUMBALL_RING_RADIUS;
}

/** 📏️ Outer scale reach along an axis (beyond move arrows and rotate rings). */
export function gumballScaleAxisOffset(): number {
  return GUMBALL_SCALE_OFFSET;
}

export function GumballScaleAxisHandle(props: { readonly axis: GumballPreviewAxis }): React.ReactElement {
  const shaftMid = gumballScaleShaftMidpoint();
  const shaftLen = gumballScaleShaftLength();
  if (props.axis === "x") {
    return (
      <>
        <mesh rotation={[0, 0, -Math.PI / 2]} position={[shaftMid, 0, 0]} renderOrder={GUMBALL_SCALE_RENDER_ORDER}>
          <cylinderGeometry args={[GUMBALL_SCALE_SHAFT_RADIUS, GUMBALL_SCALE_SHAFT_RADIUS, shaftLen, 6]} />
        </mesh>
        <mesh position={[GUMBALL_SCALE_OFFSET, 0, 0]} renderOrder={GUMBALL_SCALE_RENDER_ORDER}>
          <boxGeometry args={[GUMBALL_SCALE_BOX, GUMBALL_SCALE_BOX, GUMBALL_SCALE_BOX]} />
        </mesh>
        <mesh position={[GUMBALL_SCALE_OFFSET, 0, 0]} visible={false} userData={{ gumballHandlePick: true }}>
          <boxGeometry args={[GUMBALL_SCALE_PICK, GUMBALL_SCALE_PICK, GUMBALL_SCALE_PICK]} />
        </mesh>
      </>
    );
  }
  if (props.axis === "y") {
    return (
      <>
        <mesh position={[0, shaftMid, 0]} renderOrder={GUMBALL_SCALE_RENDER_ORDER}>
          <cylinderGeometry args={[GUMBALL_SCALE_SHAFT_RADIUS, GUMBALL_SCALE_SHAFT_RADIUS, shaftLen, 6]} />
        </mesh>
        <mesh position={[0, GUMBALL_SCALE_OFFSET, 0]} renderOrder={GUMBALL_SCALE_RENDER_ORDER}>
          <boxGeometry args={[GUMBALL_SCALE_BOX, GUMBALL_SCALE_BOX, GUMBALL_SCALE_BOX]} />
        </mesh>
        <mesh position={[0, GUMBALL_SCALE_OFFSET, 0]} visible={false} userData={{ gumballHandlePick: true }}>
          <boxGeometry args={[GUMBALL_SCALE_PICK, GUMBALL_SCALE_PICK, GUMBALL_SCALE_PICK]} />
        </mesh>
      </>
    );
  }
  return (
    <>
      <mesh rotation={[Math.PI / 2, 0, 0]} position={[0, 0, shaftMid]} renderOrder={GUMBALL_SCALE_RENDER_ORDER}>
        <cylinderGeometry args={[GUMBALL_SCALE_SHAFT_RADIUS, GUMBALL_SCALE_SHAFT_RADIUS, shaftLen, 6]} />
      </mesh>
      <mesh position={[0, 0, GUMBALL_SCALE_OFFSET]} renderOrder={GUMBALL_SCALE_RENDER_ORDER}>
        <boxGeometry args={[GUMBALL_SCALE_BOX, GUMBALL_SCALE_BOX, GUMBALL_SCALE_BOX]} />
      </mesh>
      <mesh position={[0, 0, GUMBALL_SCALE_OFFSET]} visible={false} userData={{ gumballHandlePick: true }}>
        <boxGeometry args={[GUMBALL_SCALE_PICK, GUMBALL_SCALE_PICK, GUMBALL_SCALE_PICK]} />
      </mesh>
    </>
  );
}
export const GUMBALL_PREVIEW_MIN_EXTENT = 12;
