// #region 🧲️Header
// 💻️ framework/ui/elements/Scene/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as THREE from "three";
import { ThreeEvent, useFrame, useThree } from "@react-three/fiber";
import { Edges, GizmoHelper, GizmoViewport, Grid, OrbitControls, useGLTF } from "@react-three/drei";
// 🧱️core: sceneHostPort imported directly from 🫀️core/Ports, NOT via the barrel — this component calls
// sceneHostPort.drei.Line at module top level, which requires a non-circular import (see
// 🧱️elements/🫀️core/Ports/🟦️component.tsx's header comment for why the barrel import caused a real bug).
import { sceneHostPort } from "../🫀️core/Ports/🟦️component.tsx";
import { reactHostPort } from "../🫀️core/Ports/🟦️component.tsx";
import { cn } from "../🫀️core/ClassNames/🟦️component.tsx";
import { ActionDropdown } from "../ActionGroup/🟦️component.tsx";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { surfaceClass, loadingBorderClass } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { HostThreeCanvas, uiSpacingPx, resolveSemanticColorHex, resolveColorHex, themeColorVar, tokenVar, resolveSpatialAxisColors, useLabel, type ActionDropdownOption, CameraIcon, GripVerticalIcon, type Point, type Vector, type Plane, type Camera } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters
// #region 📍️Scene
// 3D scene viewer built on React Three Fiber.
// Consumers MUST provide SceneGeometry data.

export const sceneFrameControlRef: { current: { pause: () => void; resume: () => void } | null } = { current: null };
const SceneFrameControl: React.FC = () => {
  const gl = useThree((s) => s.gl);
  const setFrameloop = useThree((s) => s.setFrameloop);
  const invalidate = useThree((s) => s.invalidate);
  reactHostPort.useEffect(() => {
    sceneFrameControlRef.current = {
      pause: () => setFrameloop("never"),
      resume: () => {
        setFrameloop("demand");
        invalidate();
      },
    };
    return () => {
      sceneFrameControlRef.current = null;
    };
  }, [gl, setFrameloop, invalidate]);
  return null;
};

const getComputedColor = (variable: string): string => resolveSemanticColorHex(variable, "gray");

/** @emoji 📐️ Scene floor grid — element gray strokes, not emphasized foreground. */
const readSceneGridColors = (): { sectionColor: string; cellColor: string } => ({
  sectionColor: resolveColorHex(themeColorVar("element"), "gray"),
  cellColor: resolveColorHex(themeColorVar("muted-foreground"), "gray"),
});

/**
 * selectableCursorUsageCount holds the data fields for a selectableCursorUsageCount record.
 **/
let selectableCursorUsageCount = 0;

/**
 * Interface for a geometry entry in a 3D scene.
 **/
export interface SceneGeometry {
  guid: string;
  plane?: Plane;
  isSelected?: boolean;
  isHovered?: boolean;
  isFocusable?: boolean;
  onClick?: () => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
}

/**
 * Extended SceneGeometry with transform delta support.
 **/
export interface TransformableGeometry extends SceneGeometry {
  isTransformable?: boolean;
}

/**
 * Interface for an incremental plane transformation delta.
 **/
export interface PlaneTransformDelta {
  translation?: { x: number; y: number; z: number };
  rotation?: { x: number; y: number; z: number; w: number };
  scale?: number;
}

/**
 * Callback type for a single plane update.
 **/
export type OnPlaneUpdate = (geometryGuid: string, newPlane: Plane) => void;

/**
 * Callback type for batch plane updates.
 **/
export type OnMultiPlaneUpdate = (updates: Array<{ geometryGuid: string; newPlane: Plane }>) => void;

/**
 * Constructs a Plane from a point and direction vector.
 **/
export const planeFromPointAndDirection = (point: Point, direction: Vector): Plane => {
  const dir = new THREE.Vector3(direction.x, direction.y, direction.z).normalize();

  const tempVec = Math.abs(dir.z) < 0.9 ? new THREE.Vector3(0, 0, 1) : new THREE.Vector3(1, 0, 0);

  const xAxis = new THREE.Vector3().crossVectors(tempVec, dir).normalize();
  const yAxis = new THREE.Vector3().crossVectors(dir, xAxis).normalize();

  return {
    origin: { x: point.x, y: point.y, z: point.z },
    xAxis: { x: xAxis.x, y: xAxis.y, z: xAxis.z },
    yAxis: { x: yAxis.x, y: yAxis.y, z: yAxis.z },
  };
};

/**
 * Extracts the THREE.Vector3 position from a Plane.
 **/
export const getPlanePosition = (plane: Plane): THREE.Vector3 => {
  return new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z);
};

/**
 * Checks whether a geometry has a non-null plane.
 **/
export const hasValidPlane = (geometry: SceneGeometry): boolean => {
  return geometry.plane !== undefined && geometry.plane !== null;
};

/**
 * Checks whether a geometry has a valid plane for camera focus.
 **/
export const isGeometryFocusable = (geometry: SceneGeometry): boolean => {
  return hasValidPlane(geometry) && (geometry.isFocusable === undefined || geometry.isFocusable === true);
};

/**
 * GeometryProps holds the data fields for a GeometryProps record.
 **/
interface GeometryProps {
  children?: React.ReactNode;
  selected?: boolean;
  hovered?: boolean;
  onClick?: (event: ThreeEvent<MouseEvent>) => void;
  onDoubleClick?: (event: ThreeEvent<MouseEvent>) => void;
  onPointerEnter?: (event: ThreeEvent<PointerEvent>) => void;
  onPointerLeave?: (event: ThreeEvent<PointerEvent>) => void;
  color?: string;
  emissiveColor?: string;
  emissiveIntensity?: number;
  showEdges?: boolean;
  edgeColor?: string;
  userData?: any;
}

/**
 * 3D geometry mesh component with selection, hover, and edge rendering.
 **/
export const Geometry: React.FC<GeometryProps> = ({ children, selected = false, hovered = false, onClick, onDoubleClick, onPointerEnter, onPointerLeave, color, emissiveColor, emissiveIntensity = 0.45, showEdges = true, edgeColor, userData }) => {
  const foregroundColor = reactHostPort.useMemo(() => getComputedColor("--foreground"), []);
  const activeBaseColor = reactHostPort.useMemo(() => getComputedColor("--active-base"), []);
  const hoverBaseColor = reactHostPort.useMemo(() => getComputedColor("--hover-base"), []);
  const [isPointerOver, setIsPointerOver] = reactHostPort.useState(false);
  const isInteractive = Boolean(onClick || onDoubleClick);

  const resolvedColor = reactHostPort.useMemo(() => {
    if (selected) return activeBaseColor;
    if (hovered) return hoverBaseColor;
    if (color) return color;
    return foregroundColor;
  }, [color, selected, hovered, activeBaseColor, hoverBaseColor, foregroundColor]);

  const resolvedEmissiveColor = reactHostPort.useMemo(() => {
    if (selected) return activeBaseColor;
    if (hovered) return hoverBaseColor;
    if (emissiveColor) return emissiveColor;
    return resolvedColor;
  }, [selected, hovered, activeBaseColor, hoverBaseColor, emissiveColor, resolvedColor]);
  const resolvedEdgeColor = reactHostPort.useMemo(() => {
    if (edgeColor) return edgeColor;
    if (selected) return activeBaseColor;
    if (hovered) return hoverBaseColor;
    return foregroundColor;
  }, [edgeColor, selected, hovered, activeBaseColor, hoverBaseColor, foregroundColor]);
  const handlePointerEnter = reactHostPort.useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (isInteractive) {
        setIsPointerOver(true);
      }
      onPointerEnter?.(event);
    },
    [isInteractive, onPointerEnter],
  );

  const handlePointerLeave = reactHostPort.useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (isInteractive) {
        setIsPointerOver(false);
      }
      onPointerLeave?.(event);
    },
    [isInteractive, onPointerLeave],
  );

  reactHostPort.useEffect(() => {
    if (!isInteractive || !isPointerOver) return;
    selectableCursorUsageCount += 1;
    document.body.classList.add("cursor-selectable");
    return () => {
      selectableCursorUsageCount = Math.max(0, selectableCursorUsageCount - 1);
      if (selectableCursorUsageCount === 0) {
        document.body.classList.remove("cursor-selectable");
      }
    };
  }, [isInteractive, isPointerOver]);

  return (
    <group userData={userData} onClick={onClick} onDoubleClick={onDoubleClick} onPointerEnter={handlePointerEnter} onPointerLeave={handlePointerLeave}>
      {children ? (
        children
      ) : (
        <mesh>
          <boxGeometry args={[1, 1, 1]} />
          <meshStandardMaterial color={resolvedColor} emissive={resolvedEmissiveColor} emissiveIntensity={emissiveIntensity} />
          {showEdges && <Edges scale={1.001} color={resolvedEdgeColor} />}
        </mesh>
      )}
    </group>
  );
};

/**
 * GltfProps holds the data fields for a GltfProps record.
 **/
interface GltfProps {
  src: string;
  roughness?: number;
  metalness?: number;
}

const getComputedColorForGltf = (variable: string): string => resolveSemanticColorHex(variable, "gray");

/**
 * Gltf holds the data fields for a Gltf record.
 **/
const Gltf: React.FC<GltfProps> = ({ src, roughness = 0.8, metalness = 0 }) => {
  const { scene } = useGLTF(src);
  const plasterColor = reactHostPort.useMemo(() => new THREE.Color(getComputedColorForGltf("--plaster")), []);
  const plasterEdgeColor = reactHostPort.useMemo(() => new THREE.Color(getComputedColorForGltf("--plaster-edge")), []);

  const clonedScene = reactHostPort.useMemo(() => {
    const cloned = scene.clone();
    const plasterMaterial = new THREE.MeshStandardMaterial({
      color: plasterColor,
      flatShading: false,
      metalness,
      roughness,
    });
    const edgeMaterial = new THREE.LineBasicMaterial({ color: plasterEdgeColor });

    cloned.traverse((child) => {
      if ((child as any).isMesh) {
        (child as any).raycast = THREE.Mesh.prototype.raycast;
        if (Array.isArray((child as any).material)) {
          (child as any).material = (child as any).material.map(() => plasterMaterial.clone());
        } else {
          (child as any).material = plasterMaterial.clone();
        }
      } else if (child instanceof THREE.Line || child instanceof THREE.LineSegments || child instanceof THREE.Points) {
        (child as any).material = edgeMaterial.clone();
      }
    });
    return cloned;
  }, [scene, plasterColor, plasterEdgeColor, roughness, metalness]);

  return <primitive object={clonedScene} />;
};

/**
 * GeometryFileProps holds the data fields for a GeometryFileProps record.
 **/
interface GeometryFileProps {
  src: string;
  environment?: string;
  roughness?: number;
  metalness?: number;
}
/** GeometryFile holds the data fields for a GeometryFile record.
 **/
/**
 **/
const GeometryFile: React.FC<GeometryFileProps> = ({ src, environment, roughness, metalness }) => {
  return (
    <div className="w-full h-full">
      <Geometry>
        <reactHostPort.Suspense fallback={null}>
          <Gltf src={src} roughness={roughness} metalness={metalness} />
        </reactHostPort.Suspense>
      </Geometry>
    </div>
  );
};

/**
 * GizmoProps holds the data fields for a GizmoProps record.
 **/
interface GizmoProps {
  show?: boolean;
  onAxisClick?: (direction: THREE.Vector3) => void;
}

export const SCENE_GIZMO_LABELS: [string, string, string] = ["", "", ""];

type SceneProjectionKind = "camera" | "orthographic";

type SceneSnapViewKind = "front" | "back" | "side" | "opposite-side" | "top" | "bottom";

interface SceneGizmoSnapTarget {
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
 * resolveSceneGizmoSnapTarget holds the data fields for a resolveSceneGizmoSnapTarget record.
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
 * resolveSceneGizmoViewportPlacement holds the data fields for a resolveSceneGizmoViewportPlacement record.
 **/
export const resolveSceneGizmoViewportPlacement = (viewport: { width: number; height: number }): SceneGizmoViewportPlacement => {
  // 🧭️ GizmoHelper `margin` is the offset to the widget CENTER. Match pane chrome (`anchorPositionStyle`
  // uses `--spacing-single` from the edge to the widget's outer edge) by adding half the navigation-cube
  // screen extent so face/corner tips clear the edge by the same inset as panes/windows. Bottom margin
  // additionally clears the folded projection pane chrome (`h-medium`) plus one spacing gap so the cube
  // sits directly above it; when that pane unfolds it grows upward over the cube (DOM overlay on canvas).
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

// #region 🔖️UnifiedGumball
/** @emoji 🎛️ World-space pose snapshot for gumball drag commits. */
export type GumballPose = {
  readonly position: readonly [number, number, number];
  readonly quaternion: readonly [number, number, number, number];
  readonly scale: readonly [number, number, number];
};

/** @emoji 🎛️ Per-handle drag kinds for the unified gumball. */
export type GumballHandleKind = "moveX" | "moveY" | "moveZ" | "moveXY" | "moveYZ" | "moveXZ" | "rotateX" | "rotateY" | "rotateZ" | "scaleX" | "scaleY" | "scaleZ" | "scaleXY" | "scaleYZ" | "scaleXZ" | "scaleUniform";

/** @emoji 🎛️ Drafting plane that restricts the gumball to the in-plane handle subset (two axes, view plane, normal rotation). */
export type GumballPlaneId = "xy" | "yz" | "xz";

/** @emoji 🎛️ Visibility and snap settings for {@link UnifiedGumball}. */
export interface GumballConfig {
  readonly moveAxes?: boolean;
  readonly movePlanes?: boolean;
  readonly rotate?: boolean;
  readonly scaleAxes?: boolean;
  readonly scalePlanes?: boolean;
  readonly scaleUniform?: boolean;
  /** When set, only the planar subset for this drafting plane is shown (e.g. Top → `xy`). */
  readonly plane?: GumballPlaneId;
  readonly translationSnap?: number;
  readonly rotationSnap?: number;
  readonly scaleSnap?: number;
  readonly shiftTranslationSnap?: number;
  readonly shiftRotationSnap?: number;
  readonly shiftScaleSnap?: number;
  readonly size?: number;
}

/** @emoji 🎛️ Handles that remain visible for each drafting plane (in-plane move/scale + normal-axis rotate + uniform). */
export const GUMBALL_PLANE_HANDLES: Readonly<Record<GumballPlaneId, ReadonlySet<GumballHandleKind>>> = {
  xy: new Set(["moveX", "moveY", "moveXY", "rotateZ", "scaleX", "scaleY", "scaleXY", "scaleUniform"]),
  yz: new Set(["moveY", "moveZ", "moveYZ", "rotateX", "scaleY", "scaleZ", "scaleYZ", "scaleUniform"]),
  xz: new Set(["moveX", "moveZ", "moveXZ", "rotateY", "scaleX", "scaleZ", "scaleXZ", "scaleUniform"]),
};

/** @emoji 🎛️ Default unified gumball: every handle group visible. */
export const DEFAULT_GUMBALL_CONFIG: Readonly<Required<Pick<GumballConfig, "moveAxes" | "movePlanes" | "rotate" | "scaleAxes" | "scalePlanes" | "scaleUniform">>> = {
  moveAxes: true,
  movePlanes: true,
  rotate: true,
  scaleAxes: true,
  scalePlanes: true,
  scaleUniform: true,
};

export type GumballVec3 = readonly [number, number, number];

const gumballV3 = (x: number, y: number, z: number): GumballVec3 => [x, y, z];
const gumballDot = (a: GumballVec3, b: GumballVec3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const gumballSub = (a: GumballVec3, b: GumballVec3): GumballVec3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const gumballAdd = (a: GumballVec3, b: GumballVec3): GumballVec3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
const gumballScale = (a: GumballVec3, s: number): GumballVec3 => [a[0] * s, a[1] * s, a[2] * s];
const gumballCross = (a: GumballVec3, b: GumballVec3): GumballVec3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const gumballLen = (a: GumballVec3): number => Math.hypot(a[0], a[1], a[2]);
const gumballNorm = (a: GumballVec3): GumballVec3 => {
  const l = gumballLen(a) || 1;
  return [a[0] / l, a[1] / l, a[2] / l];
};

/** @emoji 🎛️ Resolves partial {@link GumballConfig} with defaults. */
export function resolveGumballConfig(config?: GumballConfig): Required<Pick<GumballConfig, "moveAxes" | "movePlanes" | "rotate" | "scaleAxes" | "scalePlanes" | "scaleUniform">> & GumballConfig {
  return {
    ...DEFAULT_GUMBALL_CONFIG,
    ...config,
    moveAxes: config?.moveAxes !== false,
    movePlanes: config?.movePlanes !== false,
    rotate: config?.rotate !== false,
    scaleAxes: config?.scaleAxes !== false,
    scalePlanes: config?.scalePlanes !== false,
    scaleUniform: config?.scaleUniform !== false,
    plane: config?.plane,
  };
}

/** @emoji 🎛️ True when a handle is allowed by an optional drafting-plane subset. */
export function gumballHandleAllowedByPlane(kind: GumballHandleKind, plane: GumballPlaneId | undefined): boolean {
  if (!plane) return true;
  return GUMBALL_PLANE_HANDLES[plane].has(kind);
}

/** @emoji 🎛️ True when a handle's visibility group is enabled in the resolved config. */
export function gumballHandleGroupEnabled(kind: GumballHandleKind, config: ReturnType<typeof resolveGumballConfig>): boolean {
  if (kind === "moveX" || kind === "moveY" || kind === "moveZ") return config.moveAxes;
  if (kind === "moveXY" || kind === "moveYZ" || kind === "moveXZ") return config.movePlanes;
  if (kind === "rotateX" || kind === "rotateY" || kind === "rotateZ") return config.rotate;
  if (kind === "scaleX" || kind === "scaleY" || kind === "scaleZ") return config.scaleAxes;
  if (kind === "scaleXY" || kind === "scaleYZ" || kind === "scaleXZ") return config.scalePlanes;
  return config.scaleUniform;
}

/** @emoji 🎛️ True when a handle should render for the resolved config (group flags ∩ plane subset). */
export function gumballHandleEnabled(kind: GumballHandleKind, config: ReturnType<typeof resolveGumballConfig>): boolean {
  return gumballHandleGroupEnabled(kind, config) && gumballHandleAllowedByPlane(kind, config.plane);
}

/** @emoji 🎛️ True when at least one gumball handle group is enabled. */
export function gumballConfigVisible(config?: GumballConfig): boolean {
  const resolved = resolveGumballConfig(config);
  if (!(resolved.moveAxes || resolved.movePlanes || resolved.rotate || resolved.scaleAxes || resolved.scalePlanes || resolved.scaleUniform)) return false;
  if (!resolved.plane) return true;
  const kinds: readonly GumballHandleKind[] = ["moveX", "moveY", "moveZ", "moveXY", "moveYZ", "moveXZ", "rotateX", "rotateY", "rotateZ", "scaleX", "scaleY", "scaleZ", "scaleXY", "scaleYZ", "scaleXZ", "scaleUniform"];
  return kinds.some((kind) => gumballHandleEnabled(kind, resolved));
}

/** @emoji 🎛️ Maps a handle drag to translate / rotate / scale for host commit payloads. */
export function gumballHandleKindToTransformMode(kind: GumballHandleKind): "translate" | "rotate" | "scale" {
  if (kind.startsWith("move")) return "translate";
  if (kind.startsWith("rotate")) return "rotate";
  return "scale";
}

/** @emoji 🎛️ Reads pose from a three.js object. */
export function gumballPoseFromObject3D(object: THREE.Object3D): GumballPose {
  return {
    position: [object.position.x, object.position.y, object.position.z],
    quaternion: [object.quaternion.x, object.quaternion.y, object.quaternion.z, object.quaternion.w],
    scale: [object.scale.x, object.scale.y, object.scale.z],
  };
}

/** @emoji 🎛️ Writes pose to a three.js object. */
export function applyGumballPose(object: THREE.Object3D, pose: GumballPose): void {
  object.position.set(pose.position[0], pose.position[1], pose.position[2]);
  object.quaternion.set(pose.quaternion[0], pose.quaternion[1], pose.quaternion[2], pose.quaternion[3]);
  object.scale.set(pose.scale[0], pose.scale[1], pose.scale[2]);
}

/** @emoji 🎛️ Closest-axis parameter for a world ray (axis translate drag). */
export function gumballRayAxisParameter(rayOrigin: GumballVec3, rayDir: GumballVec3, axisPoint: GumballVec3, axisDir: GumballVec3): number {
  const w0 = gumballSub(rayOrigin, axisPoint);
  const a = gumballDot(rayDir, rayDir);
  const b = gumballDot(rayDir, axisDir);
  const c = gumballDot(axisDir, axisDir);
  const d = gumballDot(rayDir, w0);
  const e = gumballDot(axisDir, w0);
  const denom = a * c - b * b;
  if (Math.abs(denom) < 1e-10) return 0;
  return (b * d - a * e) / denom;
}

/** @emoji 👁️ Unit vector from a pivot toward the active camera (TransformControls-style eye). */
export function gumballEyeFromPivot(camera: THREE.Camera, pivot: GumballVec3): GumballVec3 {
  const cam = new THREE.Vector3();
  camera.getWorldPosition(cam);
  return gumballNorm(gumballSub([cam.x, cam.y, cam.z], pivot));
}

/** @emoji 🎛️ Drag plane normal for constrained axis move/scale (matches three.js TransformControls). */
export function gumballAxisDragPlaneNormal(axisDir: GumballVec3, eye: GumballVec3): GumballVec3 {
  const axis = gumballNorm(axisDir);
  const align = gumballCross(eye, axis);
  if (gumballLen(align) > 1e-6) {
    return gumballNorm(gumballCross(axis, align));
  }
  return eye;
}

/** @emoji 🎛️ Projects a screen ray onto an axis using a camera-aligned plane through the pivot. */
export function gumballProjectRayOntoAxis(rayOrigin: GumballVec3, rayDir: GumballVec3, pivot: GumballVec3, axisDir: GumballVec3, eye: GumballVec3): number | null {
  const planeNormal = gumballAxisDragPlaneNormal(axisDir, eye);
  const hit = gumballRayPlanePoint(rayOrigin, rayDir, pivot, planeNormal);
  if (!hit) return null;
  return gumballDot(gumballSub(hit, pivot), gumballNorm(axisDir));
}

/** @emoji 🎛️ Ray-plane intersection for plane translate drags. */
export function gumballRayPlanePoint(rayOrigin: GumballVec3, rayDir: GumballVec3, planePoint: GumballVec3, planeNormal: GumballVec3): GumballVec3 | null {
  const denom = gumballDot(planeNormal, rayDir);
  if (Math.abs(denom) < 1e-10) return null;
  const t = gumballDot(planeNormal, gumballSub(planePoint, rayOrigin)) / denom;
  return gumballAdd(rayOrigin, gumballScale(rayDir, t));
}

/** @emoji 🎛️ Signed rotation angle between two vectors around an axis. */
export function gumballAxisRotateAngle(startVec: GumballVec3, currentVec: GumballVec3, axisDir: GumballVec3): number {
  const axis = gumballNorm(axisDir);
  const project = (v: GumballVec3): GumballVec3 => gumballSub(v, gumballScale(axis, gumballDot(v, axis)));
  const a = gumballNorm(project(startVec));
  const b = gumballNorm(project(currentVec));
  let angle = Math.acos(Math.max(-1, Math.min(1, gumballDot(a, b))));
  if (gumballDot(gumballCross(a, b), axis) < 0) angle = -angle;
  return angle;
}

/** @emoji 🎛️ Scale factor from projected distances along an axis. */
export function gumballAxisScaleFactor(startProj: number, currentProj: number): number {
  if (Math.abs(startProj) < 1e-10) return 1;
  return currentProj / startProj;
}

/** @emoji 🎛️ Snaps a scalar when snap step is positive. */
export function gumballSnapScalar(value: number, snap: number | undefined): number {
  if (!snap || snap <= 0) return value;
  return Math.round(value / snap) * snap;
}

/** @emoji 🎛️ Default rotation snap while Shift is held (15°). */
export const GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP = Math.PI / 12;

/** @emoji 🎛️ Default uniform scale snap while Shift is held (10%). */
export const GUMBALL_DEFAULT_SHIFT_SCALE_SNAP = 0.1;

/** @emoji 🎛️ Resolves an active snap step from config and Shift modifier. */
export function gumballEffectiveSnapValue(configSnap: number | undefined, shiftKey: boolean, shiftFallback: number): number | undefined {
  if (configSnap != null && configSnap > 0) return configSnap;
  if (shiftKey && shiftFallback > 0) return shiftFallback;
  return undefined;
}

/** @emoji 🎛️ Active gumball drag snap steps for translate / rotate / scale. */
export function gumballResolveDragSnaps(config: GumballConfig, shiftKey: boolean): { readonly translationSnap: number | undefined; readonly rotationSnap: number | undefined; readonly scaleSnap: number | undefined } {
  return {
    translationSnap: gumballEffectiveSnapValue(config.translationSnap, shiftKey, config.shiftTranslationSnap ?? 0),
    rotationSnap: gumballEffectiveSnapValue(config.rotationSnap, shiftKey, config.shiftRotationSnap ?? GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP),
    scaleSnap: gumballEffectiveSnapValue(config.scaleSnap, shiftKey, config.shiftScaleSnap ?? GUMBALL_DEFAULT_SHIFT_SCALE_SNAP),
  };
}

const GUMBALL_PLANE_COLOR_REF = themeColorVar("muted-foreground");
const GUMBALL_UNIFORM_COLOR_REF = tokenVar("light");
const GUMBALL_HANDLE_LENGTH = 0.98;
const GUMBALL_ARROW_RADIUS = 0.022;
const GUMBALL_ARROW_HEAD = 0.12;
const GUMBALL_ARROW_HEAD_RADIUS_SCALE = 1.75;
export const GUMBALL_PLANE_OFFSET = 0.3;
export const GUMBALL_PLANE_SIZE = 0.145;
const GUMBALL_PLANE_SCALE_INSET = 0.04;
const GUMBALL_PLANE_SCALE_ARM = 0.085;
const GUMBALL_PLANE_SCALE_THICK = 0.012;
const GUMBALL_PLANE_SCALE_DEPTH = 0.012;
const GUMBALL_PLANE_SCALE_PICK = 0.12;
export const GUMBALL_RING_RADIUS = 1.06;
const GUMBALL_RING_TUBE = 0.01;
const GUMBALL_SCALE_OFFSET = 1.45;
const GUMBALL_SCALE_SHAFT_RADIUS = 0.007;
const GUMBALL_SCALE_BOX = 0.055;
const GUMBALL_SCALE_PICK = 0.14;
const GUMBALL_UNIFORM_BOX = 0.068;
const GUMBALL_SCALE_RENDER_ORDER = 1001;

const GUMBALL_AXIS_BY_KIND: Readonly<Record<GumballHandleKind, GumballVec3 | null>> = {
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

type GumballScalePlane = "xy" | "yz" | "xz";

/** @emoji 📐️ Local scale axis pair for a 2D plane scale handle. */
export function gumballScalePlaneAxisIndices(kind: GumballHandleKind): readonly [0 | 1 | 2, 0 | 1 | 2] | null {
  if (kind === "scaleXY") return [0, 1];
  if (kind === "scaleYZ") return [1, 2];
  if (kind === "scaleXZ") return [0, 2];
  return null;
}

/** @emoji 📐️ Inner corner of an L-shaped plane scale bracket (beyond the move plane square). */
export function gumballPlaneScaleCorner(plane: GumballScalePlane): GumballVec3 {
  const c = GUMBALL_PLANE_OFFSET + GUMBALL_PLANE_SIZE * 0.5 + GUMBALL_PLANE_SCALE_INSET;
  if (plane === "xy") return [c, c, 0];
  if (plane === "yz") return [0, c, c];
  return [c, 0, c];
}

function gumballWorldPointToLocalOffset(point: GumballVec3, pivot: GumballVec3, quat: THREE.Quaternion): GumballVec3 {
  const v = new THREE.Vector3(point[0] - pivot[0], point[1] - pivot[1], point[2] - pivot[2]);
  v.applyQuaternion(quat.clone().invert());
  return [v.x, v.y, v.z];
}

/** @emoji 📐️ Scale factors for a 2D plane scale drag; uniform uses radial distance in the plane. */
export function gumballPlaneScaleFactors(startLocal: readonly [number, number], currentLocal: readonly [number, number], uniform = false): readonly [number, number] {
  if (uniform) {
    const factor = gumballAxisScaleFactor(Math.hypot(startLocal[0], startLocal[1]), Math.hypot(currentLocal[0], currentLocal[1]));
    return [factor, factor];
  }
  return [gumballAxisScaleFactor(startLocal[0], currentLocal[0]), gumballAxisScaleFactor(startLocal[1], currentLocal[1])];
}

function GumballPlaneScaleLHandle(props: { readonly plane: GumballScalePlane }): React.ReactElement {
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

type GumballPreviewAxis = "x" | "y" | "z";

function gumballScaleShaftMidpoint(): number {
  return (GUMBALL_RING_RADIUS + GUMBALL_SCALE_OFFSET) / 2;
}

function gumballScaleShaftLength(): number {
  return GUMBALL_SCALE_OFFSET - GUMBALL_RING_RADIUS;
}

/** @emoji 📏️ Outer scale reach along an axis (beyond move arrows and rotate rings). */
export function gumballScaleAxisOffset(): number {
  return GUMBALL_SCALE_OFFSET;
}

function GumballScaleAxisHandle(props: { readonly axis: GumballPreviewAxis }): React.ReactElement {
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
const GUMBALL_PREVIEW_EXTENT_MARGIN = 2.75;
export const GUMBALL_PREVIEW_RING_RADIUS = 1.22;
const GUMBALL_PREVIEW_RING_TUBE = 0.014;
export const GUMBALL_PREVIEW_DISK_RADIUS = GUMBALL_RING_RADIUS;
const GUMBALL_PREVIEW_DISK_SEGMENTS = 64;

const _gumballPreviewCamPos = new THREE.Vector3();
const _gumballPreviewPivot = new THREE.Vector3();

/** @emoji 👁️ World half-extent for gumball axis line previews (fills the viewport at the pivot).
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

/** @emoji 🎨️ Resolved gumball chrome from direction accents; hover/active only raise opacity. */
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

/** @emoji 🎨️ Reads gumball palette from design tokens (X/Y/Z → primary / secondary / tertiary permanently). */
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

/** @emoji 🎛️ Per-handle visual state for gumball hover / drag feedback. */
export type GumballHandleVisualState = "idle" | "hover" | "active" | "dimmed";

/** @emoji 🎛️ Resolves handle chrome from hover and active drag kind. */
export function gumballHandleVisualState(kind: GumballHandleKind, hovered: GumballHandleKind | null, active: GumballHandleKind | null): GumballHandleVisualState {
  if (active === kind) return "active";
  if (active !== null) return "dimmed";
  if (hovered === kind) return "hover";
  if (hovered !== null) return "dimmed";
  return "idle";
}

/** @emoji 🎨️ Handle tint + opacity for a resolved visual state. Keeps direction color; only opacity changes. */
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

function gumballBaseColorForKind(kind: GumballHandleKind, palette: GumballVisualPalette): string {
  const axis = gumballPreviewAxisForKind(kind);
  if (axis === "x") return palette.axisX;
  if (axis === "y") return palette.axisY;
  if (axis === "z") return palette.axisZ;
  if (kind === "moveXY" || kind === "moveYZ" || kind === "moveXZ") return palette.plane;
  if (kind === "scaleXY" || kind === "scaleYZ" || kind === "scaleXZ") return palette.plane;
  return palette.uniform;
}

function useGumballVisualPalette(): GumballVisualPalette {
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

function GumballInteractionPreview(props: { readonly kind: GumballHandleKind; readonly palette: GumballVisualPalette; readonly phase: "hover" | "active" }): React.ReactElement | null {
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

function gumballWorldAxis(localAxis: GumballVec3, quat: THREE.Quaternion): GumballVec3 {
  const v = new THREE.Vector3(localAxis[0], localAxis[1], localAxis[2]);
  v.applyQuaternion(quat);
  return [v.x, v.y, v.z];
}

/** @emoji 📡️ World-space camera ray through an NDC point — orthographic uses parallel near→far unproject rays;
 * perspective uses the pinhole from `camera.position`. Duck-types `isOrthographicCamera` (not `instanceof`) so
 * R3F-swapped cameras from a duplicate `three` copy stay correct (otherwise ortho drags fly away from the cursor). */
export function gumballRayFromNdc(ndcX: number, ndcY: number, camera: THREE.Camera): { origin: GumballVec3; dir: GumballVec3 } {
  camera.updateMatrixWorld(true);
  const ortho = camera as THREE.OrthographicCamera & { readonly isOrthographicCamera?: boolean };
  if (ortho.isOrthographicCamera) {
    const origin = new THREE.Vector3(ndcX, ndcY, -1).unproject(camera);
    const far = new THREE.Vector3(ndcX, ndcY, 1).unproject(camera);
    const dir = far.sub(origin);
    if (dir.lengthSq() < 1e-12) return { origin: [origin.x, origin.y, origin.z], dir: [0, 0, -1] };
    dir.normalize();
    return { origin: [origin.x, origin.y, origin.z], dir: [dir.x, dir.y, dir.z] };
  }
  const origin = camera.position.clone();
  const point = new THREE.Vector3(ndcX, ndcY, 0.5).unproject(camera);
  const dir = point.sub(origin);
  if (dir.lengthSq() < 1e-12) return { origin: [origin.x, origin.y, origin.z], dir: [0, 0, -1] };
  dir.normalize();
  return { origin: [origin.x, origin.y, origin.z], dir: [dir.x, dir.y, dir.z] };
}

function gumballNdcFromPointer(clientX: number, clientY: number, rect: DOMRect): { x: number; y: number } {
  return {
    x: ((clientX - rect.left) / rect.width) * 2 - 1,
    y: -(((clientY - rect.top) / rect.height) * 2 - 1),
  };
}

const _gumballPickRaycaster = new THREE.Raycaster();
const _gumballPickNdc = new THREE.Vector2();

/** @emoji 🎯️ World-space depth bias so gumball handles win picking over occluding scene geometry (see puzzle vortex pick priority). */
const GUMBALL_PICK_DEPTH_BIAS = 1.5;

/** @emoji 🎯️ Mesh raycast biasing gumball hits closer so occluding meshes do not swallow handle interaction. */
export function gumballHandleRaycast(this: THREE.Mesh, raycaster: THREE.Raycaster, intersects: THREE.Intersection[]): void {
  const local: THREE.Intersection[] = [];
  THREE.Mesh.prototype.raycast.call(this, raycaster, local);
  for (const hit of local) {
    hit.distance = Math.max(hit.distance - GUMBALL_PICK_DEPTH_BIAS, hit.distance * 0.01);
    intersects.push(hit);
  }
}

function gumballBindHandleRaycast(root: THREE.Object3D): void {
  root.traverse((node) => {
    if (node instanceof THREE.Mesh) {
      node.raycast = gumballHandleRaycast;
    }
  });
}

/** @emoji 🎛️ Raycasts a gumball root subtree for a handle at a client-space pointer. */
export function gumballRaycastOwnedAtClientPoint(camera: THREE.Camera, canvas: HTMLElement, clientX: number, clientY: number, root: THREE.Object3D): { readonly kind: GumballHandleKind; readonly object: THREE.Object3D } | null {
  const rect = canvas.getBoundingClientRect();
  if (rect.width <= 0 || rect.height <= 0) return null;
  root.updateWorldMatrix(true, false);
  const ndc = gumballNdcFromPointer(clientX, clientY, rect);
  _gumballPickNdc.set(ndc.x, ndc.y);
  _gumballPickRaycaster.setFromCamera(_gumballPickNdc, camera);
  const hits = _gumballPickRaycaster.intersectObjects([root], true);
  for (const hit of hits) {
    const kind = gumballKindFromRaycastObject(hit.object);
    if (kind) return { kind, object: hit.object };
  }
  return null;
}

/** @emoji 🎛️ True while a gumball handle owns the active canvas pointer (blocks marquee / background pick). */
export const gumballPointerConsumesCanvasEventRef = { current: false };

/** @emoji 🎛️ Reads a gumball handle kind from a raycast hit object or its parents. */
export function gumballKindFromRaycastObject(object: THREE.Object3D | null): GumballHandleKind | null {
  let node: THREE.Object3D | null = object;
  while (node) {
    const kind = node.userData?.gumballHandleKind;
    if (typeof kind === "string") return kind as GumballHandleKind;
    node = node.parent;
  }
  return null;
}

/** @emoji 🎛️ Raycasts the scene for a gumball handle at a client-space pointer (any depth; not limited to the closest scene hit). */
export function gumballRaycastAtClientPoint(scene: THREE.Scene, camera: THREE.Camera, canvas: HTMLElement, clientX: number, clientY: number): { readonly kind: GumballHandleKind; readonly object: THREE.Object3D } | null {
  const rect = canvas.getBoundingClientRect();
  if (rect.width <= 0 || rect.height <= 0) return null;
  const ndc = gumballNdcFromPointer(clientX, clientY, rect);
  _gumballPickNdc.set(ndc.x, ndc.y);
  _gumballPickRaycaster.setFromCamera(_gumballPickNdc, camera);
  const hits = _gumballPickRaycaster.intersectObjects(scene.children, true);
  for (const hit of hits) {
    const kind = gumballKindFromRaycastObject(hit.object);
    if (kind) return { kind, object: hit.object };
  }
  return null;
}

/** @emoji 🎛️ Raycasts the scene for a gumball handle kind at a client-space pointer. */
export function gumballRaycastKindAtClientPoint(scene: THREE.Scene, camera: THREE.Camera, canvas: HTMLElement, clientX: number, clientY: number): GumballHandleKind | null {
  return gumballRaycastAtClientPoint(scene, camera, canvas, clientX, clientY)?.kind ?? null;
}

interface GumballDragState {
  readonly kind: GumballHandleKind;
  readonly before: GumballPose;
  readonly startAxisParam: number;
  readonly startPlanePoint: GumballVec3;
  readonly startRotateVec: GumballVec3;
  readonly startScaleProj: number;
  readonly startUniformScale: GumballVec3;
  readonly startPlaneScaleLocal: readonly [number, number];
  readonly scalePlaneAxes: readonly [0 | 1 | 2, 0 | 1 | 2] | null;
}

/** @emoji 🎛️ Props for {@link UnifiedGumball}. */
export interface UnifiedGumballProps {
  readonly target: THREE.Object3D;
  readonly config?: GumballConfig;
  readonly onDragStart?: (kind: GumballHandleKind, pose: GumballPose) => void;
  readonly onDrag?: (kind: GumballHandleKind, pose: GumballPose) => void;
  readonly onDragEnd?: (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => void;
  readonly onDraggingChanged?: (active: boolean) => void;
}

export function gumballApplyHandleVisualMaterial(root: THREE.Object3D, material: THREE.MeshBasicMaterial): void {
  root.traverse((node) => {
    if (!(node instanceof THREE.Mesh)) return;
    if (node.visible === false || node.userData.gumballHandlePick === true) return;
    node.material = material;
  });
}

function GumballHandleMesh(props: {
  readonly kind: GumballHandleKind;
  readonly baseColor: string;
  readonly visualState: GumballHandleVisualState;
  readonly palette: GumballVisualPalette;
  readonly onPointerDown: (kind: GumballHandleKind, event: ThreeEvent<PointerEvent>) => void;
  readonly onPointerOver: (kind: GumballHandleKind) => void;
  readonly onPointerOut: () => void;
  readonly children: React.ReactNode;
}): React.ReactElement {
  const rootRef = reactHostPort.useRef<THREE.Group>(null);
  const doubleSided = props.kind === "moveXY" || props.kind === "moveYZ" || props.kind === "moveXZ" || props.kind === "scaleXY" || props.kind === "scaleYZ" || props.kind === "scaleXZ";
  const visual = gumballResolveHandleVisual(props.baseColor, props.visualState, props.palette);
  const material = reactHostPort.useMemo(
    () =>
      new THREE.MeshBasicMaterial({
        transparent: true,
        depthTest: false,
        depthWrite: false,
        side: doubleSided ? THREE.DoubleSide : THREE.FrontSide,
      }),
    [doubleSided],
  );
  reactHostPort.useEffect(() => () => material.dispose(), [material]);
  reactHostPort.useLayoutEffect(() => {
    material.color.set(visual.color);
    material.opacity = visual.opacity;
  }, [material, visual.color, visual.opacity]);
  reactHostPort.useLayoutEffect(() => {
    const root = rootRef.current;
    if (!root) return;
    gumballApplyHandleVisualMaterial(root, material);
    gumballBindHandleRaycast(root);
  }, [material, props.children, visual.color, visual.opacity]);
  return (
    <group
      ref={rootRef}
      frustumCulled={false}
      userData={{ gumballHandleKind: props.kind }}
      onPointerDown={(event) => {
        gumballPointerConsumesCanvasEventRef.current = true;
        event.stopPropagation();
        props.onPointerDown(props.kind, event);
      }}
      onPointerOver={(event) => {
        event.stopPropagation();
        props.onPointerOver(props.kind);
      }}
      onPointerOut={(event) => {
        event.stopPropagation();
        props.onPointerOut();
      }}
    >
      {props.children}
    </group>
  );
}

function GumballHandles(props: {
  readonly config: ReturnType<typeof resolveGumballConfig>;
  readonly palette: GumballVisualPalette;
  readonly hovered: GumballHandleKind | null;
  readonly active: GumballHandleKind | null;
  readonly onPointerDown: (kind: GumballHandleKind, event: ThreeEvent<PointerEvent>) => void;
  readonly onPointerOver: (kind: GumballHandleKind) => void;
  readonly onPointerOut: () => void;
}): React.ReactElement {
  const highlightKind = props.active ?? props.hovered;
  const handleProps = (kind: GumballHandleKind, children: React.ReactNode) => (
    <GumballHandleMesh
      kind={kind}
      baseColor={gumballBaseColorForKind(kind, props.palette)}
      visualState={gumballHandleVisualState(kind, props.hovered, props.active)}
      palette={props.palette}
      onPointerDown={props.onPointerDown}
      onPointerOver={props.onPointerOver}
      onPointerOut={props.onPointerOut}
    >
      {children}
    </GumballHandleMesh>
  );
  const scaleHandleProps = (kind: "scaleX" | "scaleY" | "scaleZ", axis: GumballPreviewAxis) => handleProps(kind, <GumballScaleAxisHandle axis={axis} />);
  const scalePlaneHandleProps = (kind: "scaleXY" | "scaleYZ" | "scaleXZ", plane: GumballScalePlane) => handleProps(kind, <GumballPlaneScaleLHandle plane={plane} />);
  const show = (kind: GumballHandleKind) => gumballHandleEnabled(kind, props.config);
  return (
    <group renderOrder={999}>
      {highlightKind && show(highlightKind) ? <GumballInteractionPreview kind={highlightKind} palette={props.palette} phase={props.active === highlightKind ? "active" : "hover"} /> : null}
      {show("moveX")
        ? handleProps(
            "moveX",
            <>
              <mesh rotation={[0, 0, -Math.PI / 2]}>
                <cylinderGeometry args={[GUMBALL_ARROW_RADIUS, GUMBALL_ARROW_RADIUS, GUMBALL_HANDLE_LENGTH, 8]} />
              </mesh>
              <mesh position={[GUMBALL_HANDLE_LENGTH / 2 + GUMBALL_ARROW_HEAD / 2, 0, 0]} rotation={[0, 0, -Math.PI / 2]}>
                <coneGeometry args={[GUMBALL_ARROW_RADIUS * GUMBALL_ARROW_HEAD_RADIUS_SCALE, GUMBALL_ARROW_HEAD, 8]} />
              </mesh>
            </>,
          )
        : null}
      {show("moveY")
        ? handleProps(
            "moveY",
            <>
              <mesh>
                <cylinderGeometry args={[GUMBALL_ARROW_RADIUS, GUMBALL_ARROW_RADIUS, GUMBALL_HANDLE_LENGTH, 8]} />
              </mesh>
              <mesh position={[0, GUMBALL_HANDLE_LENGTH / 2 + GUMBALL_ARROW_HEAD / 2, 0]}>
                <coneGeometry args={[GUMBALL_ARROW_RADIUS * GUMBALL_ARROW_HEAD_RADIUS_SCALE, GUMBALL_ARROW_HEAD, 8]} />
              </mesh>
            </>,
          )
        : null}
      {show("moveZ")
        ? handleProps(
            "moveZ",
            <>
              <mesh rotation={[Math.PI / 2, 0, 0]}>
                <cylinderGeometry args={[GUMBALL_ARROW_RADIUS, GUMBALL_ARROW_RADIUS, GUMBALL_HANDLE_LENGTH, 8]} />
              </mesh>
              <mesh position={[0, 0, GUMBALL_HANDLE_LENGTH / 2 + GUMBALL_ARROW_HEAD / 2]} rotation={[Math.PI / 2, 0, 0]}>
                <coneGeometry args={[GUMBALL_ARROW_RADIUS * GUMBALL_ARROW_HEAD_RADIUS_SCALE, GUMBALL_ARROW_HEAD, 8]} />
              </mesh>
            </>,
          )
        : null}
      {show("moveXY")
        ? handleProps(
            "moveXY",
            <mesh position={[GUMBALL_PLANE_OFFSET, GUMBALL_PLANE_OFFSET, 0]}>
              <planeGeometry args={[GUMBALL_PLANE_SIZE, GUMBALL_PLANE_SIZE]} />
            </mesh>,
          )
        : null}
      {show("moveYZ")
        ? handleProps(
            "moveYZ",
            <mesh position={[0, GUMBALL_PLANE_OFFSET, GUMBALL_PLANE_OFFSET]} rotation={[0, Math.PI / 2, 0]}>
              <planeGeometry args={[GUMBALL_PLANE_SIZE, GUMBALL_PLANE_SIZE]} />
            </mesh>,
          )
        : null}
      {show("moveXZ")
        ? handleProps(
            "moveXZ",
            <mesh position={[GUMBALL_PLANE_OFFSET, 0, GUMBALL_PLANE_OFFSET]} rotation={[-Math.PI / 2, 0, 0]}>
              <planeGeometry args={[GUMBALL_PLANE_SIZE, GUMBALL_PLANE_SIZE]} />
            </mesh>,
          )
        : null}
      {show("rotateX")
        ? handleProps(
            "rotateX",
            <mesh rotation={[0, Math.PI / 2, 0]}>
              <torusGeometry args={[GUMBALL_RING_RADIUS, GUMBALL_RING_TUBE, 8, 48]} />
            </mesh>,
          )
        : null}
      {show("rotateY")
        ? handleProps(
            "rotateY",
            <mesh rotation={[Math.PI / 2, 0, 0]}>
              <torusGeometry args={[GUMBALL_RING_RADIUS, GUMBALL_RING_TUBE, 8, 48]} />
            </mesh>,
          )
        : null}
      {show("rotateZ")
        ? handleProps(
            "rotateZ",
            <mesh>
              <torusGeometry args={[GUMBALL_RING_RADIUS, GUMBALL_RING_TUBE, 8, 48]} />
            </mesh>,
          )
        : null}
      {show("scaleX") ? scaleHandleProps("scaleX", "x") : null}
      {show("scaleY") ? scaleHandleProps("scaleY", "y") : null}
      {show("scaleZ") ? scaleHandleProps("scaleZ", "z") : null}
      {show("scaleXY") ? scalePlaneHandleProps("scaleXY", "xy") : null}
      {show("scaleYZ") ? scalePlaneHandleProps("scaleYZ", "yz") : null}
      {show("scaleXZ") ? scalePlaneHandleProps("scaleXZ", "xz") : null}
      {show("scaleUniform")
        ? handleProps(
            "scaleUniform",
            <>
              <mesh renderOrder={GUMBALL_SCALE_RENDER_ORDER}>
                <boxGeometry args={[GUMBALL_UNIFORM_BOX, GUMBALL_UNIFORM_BOX, GUMBALL_UNIFORM_BOX]} />
              </mesh>
              <mesh visible={false} userData={{ gumballHandlePick: true }}>
                <boxGeometry args={[GUMBALL_SCALE_PICK, GUMBALL_SCALE_PICK, GUMBALL_SCALE_PICK]} />
              </mesh>
            </>,
          )
        : null}
    </group>
  );
}

/** @emoji 🎛️ Rhino-style unified move / rotate / scale gumball for R3F scenes. */
export function UnifiedGumball(props: UnifiedGumballProps): React.ReactElement | null {
  const config = reactHostPort.useMemo(() => resolveGumballConfig(props.config), [props.config]);
  const palette = useGumballVisualPalette();
  const groupRef = reactHostPort.useRef<THREE.Group>(null);
  const dragRef = reactHostPort.useRef<GumballDragState | null>(null);
  const [hovered, setHovered] = reactHostPort.useState<GumballHandleKind | null>(null);
  const [activeKind, setActiveKind] = reactHostPort.useState<GumballHandleKind | null>(null);
  const scene = useThree((state) => state.scene);
  const camera = useThree((state) => state.camera);
  const gl = useThree((state) => state.gl);
  const invalidate = useThree((state) => state.invalidate);
  const controls = useThree((state) => state.controls as { enabled?: boolean } | null);
  const pointerGuardEnabled = gumballConfigVisible(config);

  reactHostPort.useEffect(() => {
    invalidate();
  }, [hovered, activeKind, palette, invalidate]);

  useFrame(() => {
    const group = groupRef.current;
    const target = props.target;
    if (!group || !target) return;
    target.updateMatrixWorld(true);
    target.getWorldPosition(group.position);
    target.getWorldQuaternion(group.quaternion);
    const dist = camera.position.distanceTo(group.position);
    const scale = (dist * (config.size ?? 1)) / 8;
    group.scale.setScalar(Math.max(scale, 1e-4));
  });

  const applyDrag = reactHostPort.useCallback(
    (state: GumballDragState, ndcX: number, ndcY: number, shiftKey: boolean) => {
      const target = props.target;
      const ray = gumballRayFromNdc(ndcX, ndcY, camera);
      const pivot = gumballV3(state.before.position[0], state.before.position[1], state.before.position[2]);
      const quat = new THREE.Quaternion(state.before.quaternion[0], state.before.quaternion[1], state.before.quaternion[2], state.before.quaternion[3]);
      const kind = state.kind;
      const { translationSnap, rotationSnap, scaleSnap } = gumballResolveDragSnaps(config, shiftKey);
      if (kind === "moveX" || kind === "moveY" || kind === "moveZ") {
        const localAxis = GUMBALL_AXIS_BY_KIND[kind]!;
        const axisDir = gumballWorldAxis(localAxis, quat);
        const eye = gumballEyeFromPivot(camera, pivot);
        const param = gumballProjectRayOntoAxis(ray.origin, ray.dir, pivot, axisDir, eye);
        if (param === null) return;
        const delta = gumballSnapScalar(param - state.startAxisParam, translationSnap);
        target.position.set(state.before.position[0] + axisDir[0] * delta, state.before.position[1] + axisDir[1] * delta, state.before.position[2] + axisDir[2] * delta);
      } else if (kind === "moveXY" || kind === "moveYZ" || kind === "moveXZ") {
        const planeNormal = gumballWorldAxis(GUMBALL_AXIS_BY_KIND[kind]!, quat);
        const hit = gumballRayPlanePoint(ray.origin, ray.dir, pivot, planeNormal);
        if (!hit) return;
        let delta = gumballSub(hit, state.startPlanePoint);
        if (translationSnap && translationSnap > 0) {
          delta = [gumballSnapScalar(delta[0], translationSnap), gumballSnapScalar(delta[1], translationSnap), gumballSnapScalar(delta[2], translationSnap)];
        }
        target.position.set(state.before.position[0] + delta[0], state.before.position[1] + delta[1], state.before.position[2] + delta[2]);
      } else if (kind === "rotateX" || kind === "rotateY" || kind === "rotateZ") {
        const axisDir = gumballWorldAxis(GUMBALL_AXIS_BY_KIND[kind]!, quat);
        const hit = gumballRayPlanePoint(ray.origin, ray.dir, pivot, axisDir);
        if (!hit) return;
        const currentVec = gumballSub(hit, pivot);
        let angle = gumballAxisRotateAngle(state.startRotateVec, currentVec, axisDir);
        angle = gumballSnapScalar(angle, rotationSnap);
        const deltaQuat = new THREE.Quaternion().setFromAxisAngle(new THREE.Vector3(axisDir[0], axisDir[1], axisDir[2]), angle);
        target.quaternion.copy(quat).multiply(deltaQuat);
      } else if (kind === "scaleX" || kind === "scaleY" || kind === "scaleZ") {
        const localAxis = GUMBALL_AXIS_BY_KIND[kind]!;
        const axisDir = gumballWorldAxis(localAxis, quat);
        const eye = gumballEyeFromPivot(camera, pivot);
        const param = gumballProjectRayOntoAxis(ray.origin, ray.dir, pivot, axisDir, eye);
        if (param === null) return;
        let factor = gumballAxisScaleFactor(state.startScaleProj, param);
        if (scaleSnap && scaleSnap > 0) factor = gumballSnapScalar(factor, scaleSnap);
        const axisIndex = kind === "scaleX" ? 0 : kind === "scaleY" ? 1 : 2;
        const next = [...state.before.scale] as [number, number, number];
        next[axisIndex] = state.before.scale[axisIndex] * factor;
        target.scale.set(next[0], next[1], next[2]);
      } else if (kind === "scaleXY" || kind === "scaleYZ" || kind === "scaleXZ") {
        const planeNormal = gumballWorldAxis(GUMBALL_AXIS_BY_KIND[kind]!, quat);
        const hit = gumballRayPlanePoint(ray.origin, ray.dir, pivot, planeNormal);
        if (!hit || !state.scalePlaneAxes) return;
        const local = gumballWorldPointToLocalOffset(hit, pivot, quat);
        const [ia, ib] = state.scalePlaneAxes;
        const factors = gumballPlaneScaleFactors(state.startPlaneScaleLocal, [local[ia]!, local[ib]!], shiftKey);
        let fa = factors[0];
        let fb = factors[1];
        if (scaleSnap && scaleSnap > 0) {
          fa = gumballSnapScalar(fa, scaleSnap);
          fb = gumballSnapScalar(fb, scaleSnap);
        }
        const next = [...state.before.scale] as [number, number, number];
        next[ia] = state.before.scale[ia] * fa;
        next[ib] = state.before.scale[ib] * fb;
        target.scale.set(next[0], next[1], next[2]);
      } else if (kind === "scaleUniform") {
        const hit = gumballRayPlanePoint(ray.origin, ray.dir, pivot, gumballWorldAxis(gumballV3(0, 0, 1), quat));
        if (!hit) return;
        const current = gumballSub(hit, pivot);
        const startLen = gumballLen(state.startUniformScale);
        const currentLen = gumballLen(current);
        let factor = gumballAxisScaleFactor(startLen, currentLen);
        if (scaleSnap && scaleSnap > 0) factor = gumballSnapScalar(factor, scaleSnap);
        target.scale.set(state.before.scale[0] * factor, state.before.scale[1] * factor, state.before.scale[2] * factor);
      }
      target.updateMatrixWorld(true);
      props.onDrag?.(kind, gumballPoseFromObject3D(target));
      invalidate();
    },
    [camera, config, invalidate, props],
  );

  const onWindowMoveRef = reactHostPort.useRef<(event: PointerEvent) => void>(() => {});
  const onWindowUpRef = reactHostPort.useRef<() => void>(() => {});
  const onWindowKeyRef = reactHostPort.useRef<(event: KeyboardEvent) => void>(() => {});
  const dragPointerRef = reactHostPort.useRef({ ndcX: 0, ndcY: 0, shiftKey: false });

  const endDrag = reactHostPort.useCallback(() => {
    const state = dragRef.current;
    dragRef.current = null;
    setActiveKind(null);
    window.removeEventListener("pointermove", onWindowMoveRef.current);
    window.removeEventListener("pointerup", onWindowUpRef.current);
    window.removeEventListener("keydown", onWindowKeyRef.current, true);
    window.removeEventListener("keyup", onWindowKeyRef.current, true);
    if (controls) controls.enabled = true;
    props.onDraggingChanged?.(false);
    if (!state) return;
    props.onDragEnd?.(state.kind, state.before, gumballPoseFromObject3D(props.target));
    invalidate();
  }, [controls, invalidate, props]);

  const onWindowMove = reactHostPort.useCallback(
    (event: PointerEvent) => {
      const state = dragRef.current;
      if (!state) return;
      const rect = gl.domElement.getBoundingClientRect();
      const ndc = gumballNdcFromPointer(event.clientX, event.clientY, rect);
      dragPointerRef.current = { ndcX: ndc.x, ndcY: ndc.y, shiftKey: event.shiftKey };
      applyDrag(state, ndc.x, ndc.y, event.shiftKey);
    },
    [applyDrag, gl.domElement],
  );

  const onWindowKey = reactHostPort.useCallback(
    (event: KeyboardEvent) => {
      if (event.key !== "Shift") return;
      const state = dragRef.current;
      if (!state) return;
      const pointer = dragPointerRef.current;
      applyDrag(state, pointer.ndcX, pointer.ndcY, event.type === "keydown");
      invalidate();
    },
    [applyDrag, invalidate],
  );

  const onWindowUp = reactHostPort.useCallback(() => {
    endDrag();
  }, [endDrag]);

  onWindowMoveRef.current = onWindowMove;
  onWindowUpRef.current = onWindowUp;
  onWindowKeyRef.current = onWindowKey;

  reactHostPort.useEffect(
    () => () => {
      window.removeEventListener("pointermove", onWindowMoveRef.current);
      window.removeEventListener("pointerup", onWindowUpRef.current);
      window.removeEventListener("keydown", onWindowKeyRef.current, true);
      window.removeEventListener("keyup", onWindowKeyRef.current, true);
    },
    [],
  );

  const beginDrag = reactHostPort.useCallback(
    (kind: GumballHandleKind, event: ThreeEvent<PointerEvent>) => {
      if (event.nativeEvent.button !== 0 || dragRef.current) return;
      const target = props.target;
      const before = gumballPoseFromObject3D(target);
      const rect = gl.domElement.getBoundingClientRect();
      const ndc = gumballNdcFromPointer(event.nativeEvent.clientX, event.nativeEvent.clientY, rect);
      const ray = gumballRayFromNdc(ndc.x, ndc.y, camera);
      const pivot = gumballV3(before.position[0], before.position[1], before.position[2]);
      const quat = new THREE.Quaternion(before.quaternion[0], before.quaternion[1], before.quaternion[2], before.quaternion[3]);
      let startAxisParam = 0;
      let startPlanePoint: GumballVec3 = pivot;
      let startRotateVec: GumballVec3 = gumballV3(1, 0, 0);
      let startScaleProj = 1;
      let startUniformScale: GumballVec3 = gumballV3(1, 0, 0);
      let startPlaneScaleLocal: readonly [number, number] = [1, 1];
      let scalePlaneAxes: readonly [0 | 1 | 2, 0 | 1 | 2] | null = null;
      if (kind === "moveX" || kind === "moveY" || kind === "moveZ" || kind === "scaleX" || kind === "scaleY" || kind === "scaleZ") {
        const axisDir = gumballWorldAxis(GUMBALL_AXIS_BY_KIND[kind]!, quat);
        const eye = gumballEyeFromPivot(camera, pivot);
        startAxisParam = gumballProjectRayOntoAxis(ray.origin, ray.dir, pivot, axisDir, eye) ?? 0;
        if (kind.startsWith("scale")) startScaleProj = startAxisParam;
      } else if (kind.startsWith("move")) {
        const planeNormal = gumballWorldAxis(GUMBALL_AXIS_BY_KIND[kind]!, quat);
        startPlanePoint = gumballRayPlanePoint(ray.origin, ray.dir, pivot, planeNormal) ?? pivot;
      } else if (kind.startsWith("rotate")) {
        const axisDir = gumballWorldAxis(GUMBALL_AXIS_BY_KIND[kind]!, quat);
        const hit = gumballRayPlanePoint(ray.origin, ray.dir, pivot, axisDir) ?? pivot;
        startRotateVec = gumballSub(hit, pivot);
      } else if (kind === "scaleXY" || kind === "scaleYZ" || kind === "scaleXZ") {
        const planeNormal = gumballWorldAxis(GUMBALL_AXIS_BY_KIND[kind]!, quat);
        const hit = gumballRayPlanePoint(ray.origin, ray.dir, pivot, planeNormal) ?? pivot;
        const local = gumballWorldPointToLocalOffset(hit, pivot, quat);
        const axes = gumballScalePlaneAxisIndices(kind)!;
        scalePlaneAxes = axes;
        startPlaneScaleLocal = [local[axes[0]], local[axes[1]]];
      } else {
        const hit = gumballRayPlanePoint(ray.origin, ray.dir, pivot, gumballWorldAxis(gumballV3(0, 0, 1), quat)) ?? gumballV3(1, 0, 0);
        startUniformScale = gumballSub(hit, pivot);
      }
      dragRef.current = { kind, before, startAxisParam, startPlanePoint, startRotateVec, startScaleProj, startUniformScale, startPlaneScaleLocal, scalePlaneAxes };
      dragPointerRef.current = { ndcX: ndc.x, ndcY: ndc.y, shiftKey: event.nativeEvent.shiftKey };
      setActiveKind(kind);
      setHovered(kind);
      if (controls) controls.enabled = false;
      props.onDraggingChanged?.(true);
      props.onDragStart?.(kind, before);
      window.addEventListener("pointermove", onWindowMove);
      window.addEventListener("pointerup", onWindowUp);
      window.addEventListener("keydown", onWindowKey, true);
      window.addEventListener("keyup", onWindowKey, true);
    },
    [camera, controls, gl.domElement, onWindowKey, onWindowMove, onWindowUp, props],
  );

  const beginDragRef = reactHostPort.useRef(beginDrag);
  beginDragRef.current = beginDrag;

  reactHostPort.useEffect(() => {
    if (!pointerGuardEnabled) return;
    const canvas = gl.domElement;
    const onPointerDownCapture = (event: PointerEvent) => {
      if (event.button !== 0) return;
      const group = groupRef.current;
      const ownedHit = group ? gumballRaycastOwnedAtClientPoint(camera, canvas, event.clientX, event.clientY, group) : null;
      const anyHit = gumballRaycastKindAtClientPoint(scene, camera, canvas, event.clientX, event.clientY);
      gumballPointerConsumesCanvasEventRef.current = ownedHit !== null || anyHit !== null;
      if (!ownedHit) return;
      event.preventDefault();
      event.stopImmediatePropagation();
      gumballPointerConsumesCanvasEventRef.current = true;
      beginDragRef.current(ownedHit.kind, { nativeEvent: event, stopPropagation: () => event.stopPropagation() } as ThreeEvent<PointerEvent>);
    };
    const onPointerMoveCapture = (event: PointerEvent) => {
      if (dragRef.current) return;
      const group = groupRef.current;
      if (!group) return;
      const ownedHit = gumballRaycastOwnedAtClientPoint(camera, canvas, event.clientX, event.clientY, group);
      if (ownedHit) {
        setHovered((prev) => (prev === ownedHit.kind ? prev : ownedHit.kind));
        invalidate();
      } else {
        setHovered((prev) => (prev === null ? prev : null));
      }
    };
    const clearPointerCapture = () => {
      gumballPointerConsumesCanvasEventRef.current = false;
    };
    canvas.addEventListener("pointerdown", onPointerDownCapture, true);
    canvas.addEventListener("pointermove", onPointerMoveCapture, true);
    canvas.addEventListener("pointerup", clearPointerCapture, true);
    canvas.addEventListener("pointercancel", clearPointerCapture, true);
    return () => {
      canvas.removeEventListener("pointerdown", onPointerDownCapture, true);
      canvas.removeEventListener("pointermove", onPointerMoveCapture, true);
      canvas.removeEventListener("pointerup", clearPointerCapture, true);
      canvas.removeEventListener("pointercancel", clearPointerCapture, true);
      clearPointerCapture();
    };
  }, [camera, gl.domElement, invalidate, pointerGuardEnabled, scene]);

  const handlePointerOver = reactHostPort.useCallback(
    (kind: GumballHandleKind) => {
      setHovered(kind);
      invalidate();
    },
    [invalidate],
  );

  const handlePointerOut = reactHostPort.useCallback(() => {
    if (!dragRef.current) setHovered(null);
    invalidate();
  }, [invalidate]);

  if (!gumballConfigVisible(config)) return null;

  return sceneHostPort.fiber.createPortal(
    <group ref={groupRef} frustumCulled={false}>
      <GumballHandles config={config} palette={palette} hovered={hovered} active={activeKind} onPointerDown={beginDrag} onPointerOver={handlePointerOver} onPointerOut={handlePointerOut} />
    </group>,
    scene,
  );
}
// #endregion 🔖️UnifiedGumball

const updateSceneCameraProjection = (camera: THREE.Camera): void => {
  if (camera instanceof THREE.OrthographicCamera || camera instanceof THREE.PerspectiveCamera) {
    camera.updateProjectionMatrix();
  }
};

/**
 * Gizmo holds the data fields for a Gizmo record.
 **/
const Gizmo: React.FC<GizmoProps> = ({ show = true, onAxisClick }) => {
  const { size } = useThree();
  // 🧭️ drei GizmoViewport is Y-up; Scene is Z-up so colors remap Y↔Z while staying CAD-axis (X primary, Y secondary, Z tertiary).
  const [colors, setColors] = reactHostPort.useState<[string, string, string]>(() => {
    const axes = resolveSpatialAxisColors();
    return [axes.x, axes.z, axes.y];
  });
  const placement = reactHostPort.useMemo(() => resolveSceneGizmoViewportPlacement(size), [size]);
  // GizmoViewport axis box uses boxGeometry args [length, thickness, thickness]; uniform scale yields a chunky cube.
  const axisScale = reactHostPort.useMemo(() => [0.88, 0.036, 0.036] as [number, number, number], []);

  reactHostPort.useEffect(() => {
    const updateColors = () => {
      const axes = resolveSpatialAxisColors();
      setColors([axes.x, axes.z, axes.y]);
    };
    updateColors();
    const observer = new MutationObserver(updateColors);
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["class"],
    });
    return () => observer.disconnect();
  }, []);

  if (!show) return null;
  return (
    <GizmoHelper alignment={placement.alignment} margin={placement.margin}>
      <GizmoViewport
        labels={SCENE_GIZMO_LABELS}
        axisColors={colors}
        axisScale={axisScale}
        axisHeadScale={0.92}
        hideNegativeAxes
        onClick={
          onAxisClick
            ? (e: ThreeEvent<MouseEvent>) => {
                onAxisClick(e.object.position.clone());
                return null;
              }
            : undefined
        }
      />
    </GizmoHelper>
  );
};

/**
 * SceneInnerProps holds the data fields for a SceneInnerProps record.
 **/
interface SceneInnerProps {
  children?: React.ReactNode;
  showGrid?: boolean;
  showGizmo?: boolean;
  projection: SceneProjectionKind;
  camera?: Camera;
  onCameraChange?: (camera: Camera) => void;
  onProjectionChange?: (projection: SceneProjectionKind) => void;
  focusedItemId?: string;
  onFocusComplete?: () => void;
  selectionOnDrag?: boolean;
  onOrbitEnd?: () => void;
}

/**
 * SceneInner holds the data fields for a SceneInner record.
 **/
const SceneInner: React.FC<SceneInnerProps> = ({ children, showGrid = true, showGizmo = true, projection, camera: initialCamera, onCameraChange, onProjectionChange, focusedItemId, onFocusComplete, selectionOnDrag = false, onOrbitEnd }) => {
  const [gridColors, setGridColors] = reactHostPort.useState(readSceneGridColors);

  reactHostPort.useEffect(() => {
    const updateColors = () => setGridColors(readSceneGridColors());
    updateColors();
    const observer = new MutationObserver(updateColors);
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["class"],
    });
    return () => observer.disconnect();
  }, []);

  const { camera: threeCamera, gl, size, scene: threeScene } = useThree();
  const controlsRef = reactHostPort.useRef<any>(null);
  const isUpdatingCameraRef = reactHostPort.useRef(false);
  const prevCameraStringRef = reactHostPort.useRef<string | undefined>(initialCamera ? JSON.stringify(initialCamera) : undefined);
  const cameraRestoredRef = reactHostPort.useRef(false);
  const restoredCameraStringRef = reactHostPort.useRef<string | undefined>(undefined);
  const previousProjectionRef = reactHostPort.useRef<SceneProjectionKind>(projection);
  const cameraRef = reactHostPort.useRef<THREE.Camera>(threeCamera as THREE.Camera);
  const [pendingSnapTarget, setPendingSnapTarget] = reactHostPort.useState<SceneGizmoSnapTarget | null>(null);

  reactHostPort.useEffect(() => {
    cameraRef.current = threeCamera as THREE.Camera;
    const currentCamera = cameraRef.current;
    if (projection === "orthographic" && currentCamera instanceof THREE.OrthographicCamera) {
      currentCamera.zoom = 50;
    }
    updateSceneCameraProjection(currentCamera);
  }, [projection, threeCamera]);

  const emitCameraChange = reactHostPort.useCallback(() => {
    if (!cameraRef.current || !controlsRef.current || !onCameraChange) return;
    const position = cameraRef.current.position;
    const target = controlsRef.current.target;
    const forwardVector = new THREE.Vector3().subVectors(target, position);
    if (forwardVector.lengthSq() < 0.001) return;
    const forward = forwardVector.normalize();
    const up = cameraRef.current.up.clone().normalize();
    onCameraChange({
      position: { x: position.x, y: position.y, z: position.z },
      forward: { x: forward.x, y: forward.y, z: forward.z },
      up: { x: up.x, y: up.y, z: up.z },
    });
  }, [onCameraChange]);

  reactHostPort.useEffect(() => {
    if (!cameraRef.current || !controlsRef.current) return;

    const currentCameraString = initialCamera ? JSON.stringify(initialCamera) : undefined;

    if (previousProjectionRef.current !== projection) {
      previousProjectionRef.current = projection;
      cameraRestoredRef.current = false;
      restoredCameraStringRef.current = undefined;
    }

    if (prevCameraStringRef.current !== currentCameraString) {
      cameraRestoredRef.current = false;
      prevCameraStringRef.current = currentCameraString;
    }
    if (restoredCameraStringRef.current !== currentCameraString) {
      cameraRestoredRef.current = false;
    }

    if (cameraRestoredRef.current) return;

    isUpdatingCameraRef.current = true;

    if (initialCamera) {
      const forwardLength = Math.sqrt(initialCamera.forward.x * initialCamera.forward.x + initialCamera.forward.y * initialCamera.forward.y + initialCamera.forward.z * initialCamera.forward.z);

      if (forwardLength < 0.01) {
        cameraRestoredRef.current = true;
        isUpdatingCameraRef.current = false;
        return;
      }

      requestAnimationFrame(() => {
        if (!cameraRef.current || !controlsRef.current) return;

        cameraRef.current.position.set(initialCamera.position.x, initialCamera.position.y, initialCamera.position.z);
        cameraRef.current.up.set(initialCamera.up.x, initialCamera.up.y, initialCamera.up.z);
        const target = new THREE.Vector3(initialCamera.position.x + initialCamera.forward.x, initialCamera.position.y + initialCamera.forward.y, initialCamera.position.z + initialCamera.forward.z);
        controlsRef.current.target.copy(target);
        if (projection === "orthographic" && cameraRef.current instanceof THREE.OrthographicCamera) {
          cameraRef.current.zoom = 50;
        }
        updateSceneCameraProjection(cameraRef.current);
        controlsRef.current.update();

        setTimeout(() => {
          isUpdatingCameraRef.current = false;
        }, 300);
      });

      cameraRestoredRef.current = true;
      restoredCameraStringRef.current = currentCameraString;
    } else {
      requestAnimationFrame(() => {
        if (!cameraRef.current || !controlsRef.current) return;

        cameraRef.current.position.set(10, 10, 10);
        cameraRef.current.up.set(0, 1, 0);
        controlsRef.current.target.set(0, 0, 0);
        if (projection === "orthographic" && cameraRef.current instanceof THREE.OrthographicCamera) {
          cameraRef.current.zoom = 50;
        }
        updateSceneCameraProjection(cameraRef.current);
        controlsRef.current.update();

        setTimeout(() => {
          isUpdatingCameraRef.current = false;
        }, 300);
      });

      cameraRestoredRef.current = true;
      restoredCameraStringRef.current = currentCameraString;
    }
  }, [initialCamera, projection]);

  reactHostPort.useEffect(() => {
    if (!pendingSnapTarget || !cameraRef.current || !controlsRef.current) return;

    const currentCamera = cameraRef.current;
    const controls = controlsRef.current;
    const currentTarget = controls.target.clone();
    const currentPosition = currentCamera.position.clone();
    const currentUp = currentCamera.up.clone().normalize();
    const nextDirection = new THREE.Vector3(pendingSnapTarget.cameraDirection.x, pendingSnapTarget.cameraDirection.y, pendingSnapTarget.cameraDirection.z).normalize();
    const nextUp = new THREE.Vector3(pendingSnapTarget.up.x, pendingSnapTarget.up.y, pendingSnapTarget.up.z).normalize();
    const nextPosition = currentTarget.clone().add(nextDirection.multiplyScalar(Math.max(currentPosition.distanceTo(currentTarget), 1)));
    const animationDurationMs = 280;

    isUpdatingCameraRef.current = true;

    const animateSnap = (startTime: number) => {
      const frame = (now: number) => {
        if (!cameraRef.current || !controlsRef.current) {
          setPendingSnapTarget(null);
          isUpdatingCameraRef.current = false;
          return;
        }

        const progress = Math.min(1, (now - startTime) / animationDurationMs);
        const easedProgress = progress < 0.5 ? 4 * progress * progress * progress : 1 - Math.pow(-2 * progress + 2, 3) / 2;

        cameraRef.current.position.lerpVectors(currentPosition, nextPosition, easedProgress);
        cameraRef.current.up.lerpVectors(currentUp, nextUp, easedProgress).normalize();
        controlsRef.current.target.copy(currentTarget);

        if (projection === "orthographic" && cameraRef.current instanceof THREE.OrthographicCamera) {
          cameraRef.current.zoom = 50;
        }
        updateSceneCameraProjection(cameraRef.current);
        controlsRef.current.update();

        if (progress < 1) {
          requestAnimationFrame(frame);
          return;
        }

        emitCameraChange();
        onProjectionChange?.("orthographic");
        setPendingSnapTarget(null);
        isUpdatingCameraRef.current = false;
      };

      requestAnimationFrame(frame);
    };

    requestAnimationFrame(animateSnap);
  }, [emitCameraChange, onProjectionChange, pendingSnapTarget, projection]);

  const handleGizmoAxisClick = reactHostPort.useCallback((direction: THREE.Vector3) => {
    setPendingSnapTarget(resolveSceneGizmoSnapTarget(direction));
  }, []);

  const handleStart = reactHostPort.useCallback(() => {
    if (isUpdatingCameraRef.current || projection !== "orthographic") return;
    emitCameraChange();
    onProjectionChange?.("camera");
  }, [emitCameraChange, onProjectionChange, projection]);

  const handleEnd = reactHostPort.useCallback(() => {
    if (isUpdatingCameraRef.current) return;
    onOrbitEnd?.();
    emitCameraChange();
  }, [emitCameraChange, onOrbitEnd]);

  reactHostPort.useEffect(() => {
    if (!focusedItemId || !cameraRef.current || !controlsRef.current) return;

    let retryCount = 0;
    const maxRetries = 20;

    const findAndFocusObject = () => {
      if (!cameraRef.current || !controlsRef.current) return;

      let targetObject: THREE.Object3D | null = null;

      threeScene.traverse((obj: THREE.Object3D) => {
        if (obj.userData?.id === focusedItemId || obj.name === focusedItemId) {
          targetObject = obj;
        }
      });

      if (!targetObject) {
        retryCount++;
        if (retryCount < maxRetries) {
          setTimeout(findAndFocusObject, 50);
        } else {
          console.warn(`Focus: Object ${focusedItemId} not found after ${maxRetries} retries`);
          if (onFocusComplete) onFocusComplete();
        }
        return;
      }

      const box = new THREE.Box3().setFromObject(targetObject);
      const center = box.getCenter(new THREE.Vector3());
      const size = box.getSize(new THREE.Vector3());
      const maxDim = Math.max(size.x, size.y, size.z);
      const distance = maxDim * 2;

      const camera = cameraRef.current;
      const currentPos = camera.position.clone();
      const direction = new THREE.Vector3().subVectors(currentPos, controlsRef.current.target).normalize();
      const newPosition = center.clone().add(direction.multiplyScalar(distance));

      isUpdatingCameraRef.current = true;

      const animate = () => {
        if (!cameraRef.current || !controlsRef.current) return;

        const t = 0.1;
        camera.position.lerp(newPosition, t);
        controlsRef.current.target.lerp(center, t);
        updateSceneCameraProjection(camera);
        controlsRef.current.update();

        const distanceToTarget = camera.position.distanceTo(newPosition);
        const targetDistanceToCenter = controlsRef.current.target.distanceTo(center);

        if (distanceToTarget > 0.01 || targetDistanceToCenter > 0.01) {
          requestAnimationFrame(animate);
        } else {
          isUpdatingCameraRef.current = false;
          if (onFocusComplete) onFocusComplete();
        }
      };

      requestAnimationFrame(animate);
    };

    findAndFocusObject();
  }, [focusedItemId, threeScene, onFocusComplete]);

  return (
    <>
      <OrbitControls
        ref={controlsRef}
        enableDamping={false}
        onStart={handleStart}
        mouseButtons={
          selectionOnDrag
            ? {
                LEFT: undefined,
                MIDDLE: THREE.MOUSE.ROTATE,
                RIGHT: THREE.MOUSE.ROTATE,
              }
            : {
                LEFT: THREE.MOUSE.ROTATE,
                MIDDLE: undefined,
                RIGHT: undefined,
              }
        }
        onEnd={handleEnd}
      />
      <ambientLight intensity={1} />
      {children}
      {showGrid && <Grid infiniteGrid={true} sectionColor={gridColors.sectionColor} cellColor={gridColors.cellColor} />}
      {showGizmo && <Gizmo onAxisClick={handleGizmoAxisClick} />}
    </>
  );
};

/**
 * SceneProps holds the data fields for a SceneProps record.
 **/
interface SceneProps {
  children?: React.ReactNode;
  showGrid?: boolean;
  showGizmo?: boolean;
  camera?: Camera;
  onCameraChange?: (camera: Camera) => void;
  onDoubleClickCapture?: (e: React.MouseEvent) => void;
  onPointerMissed?: (e: MouseEvent) => void;
  orthographic?: boolean;
  shadows?: boolean;
  className?: string;
  focusedItemId?: string;
  onFocusComplete?: () => void;
  projection?: SceneProjectionKind;
  onProjectionChange?: (projection: SceneProjectionKind) => void;
  selectionOnDrag?: boolean;
}

/**
 * 3D scene viewer with orbit controls, grid, and geometry rendering.
 **/
export const Scene: React.FC<SceneProps> = ({
  children,
  showGrid = true,
  showGizmo = true,
  camera,
  onCameraChange,
  onDoubleClickCapture,
  onPointerMissed,
  orthographic = false,
  shadows = false,
  className = "",
  focusedItemId,
  onFocusComplete,
  projection = "camera",
  onProjectionChange,
  selectionOnDrag = false,
}) => {
  const perspectiveLabel = useLabel("ui.host.perspective");
  const orthographicLabel = useLabel("ui.host.orthographic");
  const [resolvedProjection, setResolvedProjection] = reactHostPort.useState<SceneProjectionKind>(projection ?? (orthographic ? "orthographic" : "camera"));

  reactHostPort.useEffect(() => {
    setResolvedProjection(projection ?? (orthographic ? "orthographic" : "camera"));
  }, [orthographic, projection]);

  const handleProjectionChange = reactHostPort.useCallback(
    (nextProjection: SceneProjectionKind) => {
      setResolvedProjection(nextProjection);
      onProjectionChange?.(nextProjection);
    },
    [onProjectionChange],
  );

  const projectionOptions: ActionDropdownOption[] = [
    {
      value: "camera",
      icon: <CameraIcon className="size-3" />,
      label: perspectiveLabel,
    },
    {
      value: "orthographic",
      icon: <GripVerticalIcon className="size-3" />,
      label: orthographicLabel,
    },
  ];

  return (
    <div className={`relative h-full w-full ${className}`} style={{ minHeight: "100%", minWidth: "100%" }} onDoubleClick={onDoubleClickCapture}>
      <div className="absolute top-1 right-1 z-panel">
        <ActionDropdown id="ui.scene.projection" options={projectionOptions} value={resolvedProjection} onValueChange={(value) => handleProjectionChange(value as SceneProjectionKind)} />
      </div>
      <HostThreeCanvas
        onPointerMissed={onPointerMissed}
        orthographic={resolvedProjection === "orthographic"}
        shadows={shadows}
        frameloop="demand"
        camera={resolvedProjection === "orthographic" ? { zoom: 50, position: [10, 10, 10], near: -10000, far: 10000 } : { fov: 75, position: [10, 10, 10], near: 0.1, far: 10000 }}
        style={{ width: "100%", height: "100%" }}
      >
        <SceneFrameControl />
        <SceneInner
          showGrid={showGrid}
          showGizmo={showGizmo}
          projection={resolvedProjection}
          camera={camera}
          onCameraChange={onCameraChange}
          onProjectionChange={handleProjectionChange}
          focusedItemId={focusedItemId}
          onFocusComplete={onFocusComplete}
          selectionOnDrag={selectionOnDrag}
        >
          {children}
        </SceneInner>
      </HostThreeCanvas>
    </div>
  );
};

/**
 * Skeleton loading placeholder for a 3D scene.
 *
 **/
export const SceneSkeleton: React.FC = () => (
  <div className={cn("h-full w-full flex items-center justify-center", surfaceClass, loadingBorderClass)}>
    <div className="relative w-32 h-32 animate-pulse">
      <div className="absolute inset-0 border-4 border-muted-foreground/20 rounded-lg" />
      <div className="absolute inset-2 border-2 border-muted-foreground/20 rounded-lg" />
      <div className="absolute inset-4 border border-muted-foreground/20 rounded-lg" />
    </div>
  </div>
);

// #endregion 📍️Scene
