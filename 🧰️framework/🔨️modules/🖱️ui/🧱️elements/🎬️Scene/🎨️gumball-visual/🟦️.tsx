import * as THREE from "three";
import { resolveSpatialAxisColors, resolveColorHex } from "@semio-tech/ui-styling";
import { reactHostPort, sceneHostPort } from "../../🔌️Ports/🟦️.tsx";
import * as React from "react";
import { GUMBALL_PLANE_COLOR_REF, GUMBALL_PREVIEW_MIN_EXTENT, GUMBALL_RING_RADIUS, GUMBALL_UNIFORM_COLOR_REF, type GumballPreviewAxis } from "../🔩️gumball-handles/🟦️.tsx";
import type { GumballHandleKind, GumballVec3 } from "../🧿️gumball-model/🟦️.ts";
import { useFrame, useThree } from "../🔌️three-hooks/🟦️.ts";

const GUMBALL_PREVIEW_EXTENT_MARGIN = 2.75;
export const GUMBALL_PREVIEW_RING_RADIUS = 1.22;
const GUMBALL_PREVIEW_RING_TUBE = 0.014;
export const GUMBALL_PREVIEW_DISK_RADIUS = GUMBALL_RING_RADIUS;
const GUMBALL_PREVIEW_DISK_SEGMENTS = 64;

const _gumballPreviewCamPos = new THREE.Vector3();
const _gumballPreviewPivot = new THREE.Vector3();

/** 👁️ World half-extent for gumball axis line previews (fills the viewport at the pivot).
 * Duck-types `isOrthographicCamera` / `isPerspectiveCamera` (not `instanceof`) so R3F cameras from a
 * duplicate `three` copy still resolve the correct frustum. */
export function gumballPreviewWorldExtent(camera: THREE.Camera, pivotWorld: THREE.Vector3): number {
  _gumballPreviewCamPos.copy(camera.position);
  const dist = Math.max(_gumballPreviewCamPos.distanceTo(pivotWorld), 1e-3);
  const ortho = camera as THREE.OrthographicCamera & { readonly isOrthographicCamera?: boolean };
  if (ortho.isOrthographicCamera) {
    const halfW = ((ortho.right - ortho.left) * 0.5) / Math.max(ortho.zoom, 1e-3);
    const halfH = ((ortho.top - ortho.bottom) * 0.5) / Math.max(ortho.zoom, 1e-3);
    return Math.max(Math.max(halfW, halfH) * GUMBALL_PREVIEW_EXTENT_MARGIN, GUMBALL_PREVIEW_MIN_EXTENT);
  }
  const perspective = camera as THREE.PerspectiveCamera & { readonly isPerspectiveCamera?: boolean };
  if (perspective.isPerspectiveCamera) {
    const vFovRad = (perspective.fov * Math.PI) / 180;
    const hFovRad = 2 * Math.atan(Math.tan(vFovRad / 2) * perspective.aspect);
    const span = dist * Math.max(Math.tan(vFovRad / 2), Math.tan(hFovRad / 2)) * GUMBALL_PREVIEW_EXTENT_MARGIN;
    return Math.max(span, GUMBALL_PREVIEW_MIN_EXTENT);
  }
  return Math.max(dist * 4, GUMBALL_PREVIEW_MIN_EXTENT);
}

function gumballAxisPreviewPoints(axis: GumballPreviewAxis, half: number): [number, number, number][] {
  if (axis === "x")
    return [
      [-half, 0, 0],
      [half, 0, 0],
    ];
  if (axis === "y")
    return [
      [0, -half, 0],
      [0, half, 0],
    ];
  return [
    [0, 0, -half],
    [0, 0, half],
  ];
}

/** 🎨️ Resolved gumball chrome from direction accents; hover/active only raise opacity. */
export interface GumballVisualPalette {
  readonly axisX: string;
  readonly axisY: string;
  readonly axisZ: string;
  readonly plane: string;
  readonly uniform: string;
  readonly idleOpacity: number;
  readonly dimmedOpacity: number;
  readonly hoverOpacity: number;
  readonly activeOpacity: number;
  readonly previewHoverOpacity: number;
  readonly previewActiveOpacity: number;
}

/** 🎨️ Reads gumball palette from design tokens (X/Y/Z → primary / secondary / tertiary permanently). */
export function resolveGumballVisualPalette(): GumballVisualPalette {
  const axes = resolveSpatialAxisColors();
  return {
    axisX: axes.x,
    axisY: axes.y,
    axisZ: axes.z,
    plane: resolveColorHex(GUMBALL_PLANE_COLOR_REF, "gray"),
    uniform: resolveColorHex(GUMBALL_UNIFORM_COLOR_REF, "light"),
    idleOpacity: 0.62,
    dimmedOpacity: 0.28,
    hoverOpacity: 0.96,
    activeOpacity: 1,
    previewHoverOpacity: 0.24,
    previewActiveOpacity: 0.38,
  };
}

/** 🎛️ Per-handle visual state for gumball hover / drag feedback. */
export type GumballHandleVisualState = "idle" | "hover" | "active" | "dimmed";

/** 🎛️ Resolves handle chrome from hover and active drag kind. */
export function gumballHandleVisualState(kind: GumballHandleKind, hovered: GumballHandleKind | null, active: GumballHandleKind | null): GumballHandleVisualState {
  if (active === kind) return "active";
  if (active !== null) return "dimmed";
  if (hovered === kind) return "hover";
  if (hovered !== null) return "dimmed";
  return "idle";
}

/** 🎨️ Handle tint + opacity for a resolved visual state. Keeps direction color; only opacity changes. */
export function gumballResolveHandleVisual(baseColor: string, state: GumballHandleVisualState, palette: GumballVisualPalette): { readonly color: string; readonly opacity: number } {
  if (state === "active") return { color: baseColor, opacity: palette.activeOpacity };
  if (state === "hover") return { color: baseColor, opacity: palette.hoverOpacity };
  if (state === "dimmed") return { color: baseColor, opacity: palette.dimmedOpacity };
  return { color: baseColor, opacity: palette.idleOpacity };
}

function gumballPreviewAxisForKind(kind: GumballHandleKind): GumballPreviewAxis | null {
  if (kind === "moveX" || kind === "rotateX" || kind === "scaleX") return "x";
  if (kind === "moveY" || kind === "rotateY" || kind === "scaleY") return "y";
  if (kind === "moveZ" || kind === "rotateZ" || kind === "scaleZ") return "z";
  return null;
}

export function gumballBaseColorForKind(kind: GumballHandleKind, palette: GumballVisualPalette): string {
  const axis = gumballPreviewAxisForKind(kind);
  if (axis === "x") return palette.axisX;
  if (axis === "y") return palette.axisY;
  if (axis === "z") return palette.axisZ;
  if (kind === "moveXY" || kind === "moveYZ" || kind === "moveXZ") return palette.plane;
  if (kind === "scaleXY" || kind === "scaleYZ" || kind === "scaleXZ") return palette.plane;
  return palette.uniform;
}

export function useGumballVisualPalette(): GumballVisualPalette {
  const [palette, setPalette] = reactHostPort.useState(() => resolveGumballVisualPalette());
  reactHostPort.useEffect(() => {
    const update = () => setPalette(resolveGumballVisualPalette());
    update();
    const observer = new MutationObserver(update);
    observer.observe(document.documentElement, { attributes: true, attributeFilter: ["class"] });
    return () => observer.disconnect();
  }, []);
  return palette;
}

const GumballLine = sceneHostPort.drei.Line;

function GumballAxisPreviewLine(props: { readonly axis: GumballPreviewAxis; readonly color: string; readonly opacity: number; readonly lineWidth: number }): React.ReactElement {
  const rootRef = reactHostPort.useRef<THREE.Group>(null);
  const camera = useThree((state) => state.camera);
  const invalidate = useThree((state) => state.invalidate);
  const [points, setPoints] = reactHostPort.useState<[number, number, number][]>(() => gumballAxisPreviewPoints(props.axis, GUMBALL_PREVIEW_MIN_EXTENT));
  useFrame(() => {
    const root = rootRef.current;
    if (!root) return;
    const half = gumballPreviewWorldExtent(camera, root.getWorldPosition(_gumballPreviewPivot));
    setPoints((prev) => {
      const next = gumballAxisPreviewPoints(props.axis, half);
      const prevHalf = Math.abs(prev[1]![props.axis === "x" ? 0 : props.axis === "y" ? 1 : 2] ?? 0);
      if (Math.abs(prevHalf - half) <= half * 0.02) return prev;
      return next;
    });
    invalidate();
  });
  return (
    <group ref={rootRef}>
      <GumballLine points={points} color={props.color} lineWidth={props.lineWidth} transparent opacity={props.opacity} depthTest={false} renderOrder={997} />
    </group>
  );
}

function GumballPlanePreviewMesh(props: { readonly orientation: "xy" | "yz" | "xz"; readonly color: string; readonly opacity: number }): React.ReactElement {
  const rotation = props.orientation === "yz" ? ([0, Math.PI / 2, 0] as const) : props.orientation === "xz" ? ([-Math.PI / 2, 0, 0] as const) : ([0, 0, 0] as const);
  return (
    <mesh rotation={rotation} renderOrder={997}>
      <circleGeometry args={[GUMBALL_PREVIEW_DISK_RADIUS, GUMBALL_PREVIEW_DISK_SEGMENTS]} />
      <meshBasicMaterial color={props.color} transparent opacity={props.opacity} depthTest={false} depthWrite={false} side={THREE.DoubleSide} />
    </mesh>
  );
}

export function GumballInteractionPreview(props: { readonly kind: GumballHandleKind; readonly palette: GumballVisualPalette; readonly phase: "hover" | "active" }): React.ReactElement | null {
  const color = gumballBaseColorForKind(props.kind, props.palette);
  const fillOpacity = props.phase === "active" ? props.palette.previewActiveOpacity : props.palette.previewHoverOpacity;
  const lineWidth = props.phase === "active" ? 3.25 : 2.5;
  const lineOpacity = props.phase === "active" ? 0.92 : 0.72;
  const axis = gumballPreviewAxisForKind(props.kind);
  if (axis) {
    return <GumballAxisPreviewLine axis={axis} color={color} opacity={lineOpacity} lineWidth={lineWidth} />;
  }
  if (props.kind === "moveXY" || props.kind === "scaleXY") {
    return <GumballPlanePreviewMesh orientation="xy" color={color} opacity={fillOpacity} />;
  }
  if (props.kind === "moveYZ" || props.kind === "scaleYZ") {
    return <GumballPlanePreviewMesh orientation="yz" color={color} opacity={fillOpacity} />;
  }
  if (props.kind === "moveXZ" || props.kind === "scaleXZ") {
    return <GumballPlanePreviewMesh orientation="xz" color={color} opacity={fillOpacity} />;
  }
  if (props.kind === "rotateX") {
    return (
      <mesh rotation={[0, Math.PI / 2, 0]} renderOrder={997}>
        <torusGeometry args={[GUMBALL_PREVIEW_RING_RADIUS, GUMBALL_PREVIEW_RING_TUBE, 8, 64]} />
        <meshBasicMaterial color={color} transparent opacity={lineOpacity} depthTest={false} depthWrite={false} />
      </mesh>
    );
  }
  if (props.kind === "rotateY") {
    return (
      <mesh rotation={[Math.PI / 2, 0, 0]} renderOrder={997}>
        <torusGeometry args={[GUMBALL_PREVIEW_RING_RADIUS, GUMBALL_PREVIEW_RING_TUBE, 8, 64]} />
        <meshBasicMaterial color={color} transparent opacity={lineOpacity} depthTest={false} depthWrite={false} />
      </mesh>
    );
  }
  if (props.kind === "rotateZ") {
    return (
      <mesh renderOrder={997}>
        <torusGeometry args={[GUMBALL_PREVIEW_RING_RADIUS, GUMBALL_PREVIEW_RING_TUBE, 8, 64]} />
        <meshBasicMaterial color={color} transparent opacity={lineOpacity} depthTest={false} depthWrite={false} />
      </mesh>
    );
  }
  if (props.kind === "scaleUniform") {
    return (
      <mesh renderOrder={997}>
        <sphereGeometry args={[0.16, 20, 20]} />
        <meshBasicMaterial color={color} transparent opacity={fillOpacity} depthTest={false} depthWrite={false} wireframe={false} />
      </mesh>
    );
  }
  return null;
}

export function gumballWorldAxis(localAxis: GumballVec3, quat: THREE.Quaternion): GumballVec3 {
  const v = new THREE.Vector3(localAxis[0], localAxis[1], localAxis[2]);
  v.applyQuaternion(quat);
  return [v.x, v.y, v.z];
}
