/** 🧭️ The orbit view gizmo: hit visual state and palette, the gizmo viewport, hit dispatch, the snap driver, view snap driver and view controls.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | r3f}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { reactHostPort, resolveSceneGizmoViewportPlacement, useCanvasAppearanceSync } from "@semio-tech/ui-react";
import {
  clearColorResolveCache,
  resolveColorHex,
  resolveSpatialAxisColors,
  semanticVar,
  themeColorVar,
  tokenVar
} from "@semio-tech/ui-styling";
import { type ReactElement } from "react";
import {
  type Camera,
  CanvasTexture,
  Color,
  GizmoHelper,
  Matrix4,
  Quaternion,
  useFrame,
  useThree,
  Vector3
} from "../🔌️adapters/🟦️.ts";
import { type ProjectionGizmoHit, type WorldProjectionSpec } from "../🔭️scene-types/🟦️.ts";
import {
  cadVec3ToThree,
  type OrbitCameraProjection,
  threeVec3ToCad,
  type Vec3,
  type WorldCameraState
} from "../🧷️r3f-foundation/🟦️.ts";
import {
  ORBIT_CAMERA_Z_UP,
  type OrbitCameraViewId,
  type OrbitControlsTarget,
  useWorldOrbitViewSnapGate,
  WORLD_ORBIT_CAMERA_MIN_FAR,
  worldCameraFov,
  worldCameraIsOrthographic,
  worldCameraZoom
} from "../📷️camera-view/🟦️.tsx";
import {
  orbitViewToWorldProjectionSpec,
  resolveProjectionGizmoSpec,
  worldProjectionGoalMatrix,
  worldProjectionMorphMatrix,
  worldProjectionPerspectiveFov,
  worldProjectionSpecToOrbitView,
  worldProjectionTransitionPose
} from "../📐️projection/🟦️.tsx";

// #region 📷️OrbitCameraView
const WORLD_ORBIT_VIEW_SNAP_DURATION_MS = 280;

function easeInOutCubic(t: number): number {
  return t < 0.5 ? 4 * t * t * t : 1 - (-2 * t + 2) ** 3 / 2;
}

function lerpVec3(a: Vec3, b: Vec3, t: number): Vec3 {
  return [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];
}

export interface WorldOrbitViewGizmoProps {
  readonly show?: boolean;
  readonly projectionSpec?: WorldProjectionSpec;
  readonly onViewSelect?: (view: OrbitCameraViewId) => void;
  readonly onSpecSelect: (spec: WorldProjectionSpec) => void;
}

function resolveWorldGizmoNeutralColor(): string {
  return resolveColorHex(semanticVar("muted-foreground"), "gray");
}

type ProjectionGizmoVisualState = "idle" | "hover" | "dimmed";

interface ProjectionGizmoVisualPalette {
  readonly neutralHover: string;
  readonly brighten: string;
  readonly idleOpacity: number;
  readonly hoverOpacity: number;
  readonly dimmedOpacity: number;
}

export function resolveProjectionGizmoVisualPalette(): ProjectionGizmoVisualPalette {
  return {
    neutralHover: resolveColorHex(themeColorVar("emphasized"), "dark"),
    brighten: resolveColorHex(tokenVar("light"), "light"),
    idleOpacity: 0.88,
    hoverOpacity: 1,
    dimmedOpacity: 0.4,
  };
}

function projectionGizmoHitsEqual(a: ProjectionGizmoHit, b: ProjectionGizmoHit): boolean {
  return JSON.stringify(a) === JSON.stringify(b);
}

export function projectionGizmoHitVisualState(hit: ProjectionGizmoHit, hovered: ProjectionGizmoHit | null): ProjectionGizmoVisualState {
  if (!hovered) {
    return "idle";
  }
  return projectionGizmoHitsEqual(hit, hovered) ? "hover" : "dimmed";
}

function brightenProjectionGizmoColor(baseColor: string, accentColor: string, amount: number): string {
  const base = new Color(baseColor);
  const accent = new Color(accentColor);
  base.lerp(accent, amount);
  return `#${base.getHexString()}`;
}

export function projectionGizmoHeadFillColor(baseColor: string, state: ProjectionGizmoVisualState, palette: ProjectionGizmoVisualPalette, neutral: boolean): string {
  if (state !== "hover") {
    return baseColor;
  }
  if (neutral) {
    return palette.neutralHover;
  }
  return brightenProjectionGizmoColor(baseColor, palette.brighten, 0.45);
}

function projectionGizmoHeadOpacity(state: ProjectionGizmoVisualState, palette: ProjectionGizmoVisualPalette): number {
  if (state === "hover") {
    return palette.hoverOpacity;
  }
  if (state === "dimmed") {
    return palette.dimmedOpacity;
  }
  return palette.idleOpacity;
}

//#region 🧭️WorldOrbitViewGizmoViewport
interface WorldProjectionGizmoViewportProps {
  readonly axisColors: { readonly x: string; readonly y: string; readonly z: string };
  readonly neutralColor: string;
  readonly axisScale: [number, number, number];
  readonly onHitSelect: (hit: ProjectionGizmoHit) => void;
}

function WorldOrbitViewGizmoViewportAxis(props: { readonly scale: [number, number, number]; readonly color: string; readonly rotation: [number, number, number]; readonly opacity?: number }): ReactElement {
  return (
    <group rotation={props.rotation}>
      <mesh position={[0.4, 0, 0]}>
        <boxGeometry args={props.scale} />
        <meshBasicMaterial color={props.color} toneMapped={false} transparent opacity={props.opacity ?? 0.72} />
      </mesh>
      <mesh position={[-0.4, 0, 0]}>
        <boxGeometry args={props.scale} />
        <meshBasicMaterial color={props.color} toneMapped={false} transparent opacity={(props.opacity ?? 0.72) * 0.92} />
      </mesh>
    </group>
  );
}

/** 📏️ Thin shaft from the gizmo origin to a corner hit (matches axis shafts for the eight diagonals). */
function WorldProjectionGizmoCornerShaft(props: { readonly to: readonly [number, number, number]; readonly color: string; readonly opacity?: number }): ReactElement {
  const quaternion = reactHostPort.useMemo(() => {
    const dir = new Vector3(props.to[0], props.to[1], props.to[2]);
    const length = dir.length();
    if (length < 1e-6) {
      return new Quaternion();
    }
    dir.multiplyScalar(1 / length);
    return new Quaternion().setFromUnitVectors(new Vector3(1, 0, 0), dir);
  }, [props.to]);
  const length = Math.hypot(props.to[0], props.to[1], props.to[2]);
  return (
    <mesh position={[props.to[0] * 0.5, props.to[1] * 0.5, props.to[2] * 0.5]} quaternion={quaternion}>
      <boxGeometry args={[length * 0.88, 0.022, 0.022]} />
      <meshBasicMaterial color={props.color} toneMapped={false} transparent opacity={props.opacity ?? 0.72} />
    </mesh>
  );
}

function WorldProjectionGizmoHitHead(props: {
  readonly position: [number, number, number];
  readonly hit: ProjectionGizmoHit;
  readonly color: string;
  readonly prominent?: boolean;
  readonly palette: ProjectionGizmoVisualPalette;
  readonly visualState: ProjectionGizmoVisualState;
  readonly neutral: boolean;
  readonly axisHeadScale: number;
  readonly onHitSelect: (hit: ProjectionGizmoHit) => void;
  readonly onPointerOver: (hit: ProjectionGizmoHit) => void;
  readonly onPointerOut: () => void;
}): ReactElement {
  const fillColor = projectionGizmoHeadFillColor(props.color, props.visualState, props.palette, props.neutral);
  const headOpacity = projectionGizmoHeadOpacity(props.visualState, props.palette);
  const texture = reactHostPort.useMemo(() => {
    const canvas = document.createElement("canvas");
    canvas.width = 64;
    canvas.height = 64;
    const context = canvas.getContext("2d");
    if (!context) {
      return new CanvasTexture(canvas);
    }
    context.beginPath();
    context.arc(32, 32, props.prominent ? 16 : 12, 0, 2 * Math.PI);
    context.closePath();
    context.fillStyle = fillColor;
    context.fill();
    return new CanvasTexture(canvas);
  }, [fillColor, props.prominent]);
  reactHostPort.useEffect(() => () => texture.dispose(), [texture]);
  const scale = (props.prominent ? 1 : 0.65) * props.axisHeadScale * (props.visualState === "hover" ? 1.1 : 1);
  return (
    <sprite
      scale={scale}
      position={props.position}
      userData={{ projectionGizmoHit: props.hit }}
      onPointerOver={(event) => {
        event.stopPropagation();
        props.onPointerOver(props.hit);
      }}
      onPointerOut={(event) => {
        event.stopPropagation();
        props.onPointerOut();
      }}
      onPointerDown={(event) => {
        event.stopPropagation();
      }}
      onClick={(event) => {
        event.stopPropagation();
        const hit = (event.object.userData?.projectionGizmoHit as ProjectionGizmoHit | undefined) ?? props.hit;
        props.onHitSelect(hit);
      }}
    >
      <spriteMaterial map={texture} alphaTest={0.3} opacity={headOpacity} toneMapped={false} />
    </sprite>
  );
}

/** 🎯️ Navigation cube with face, corner, and center hit targets for projection snapping. */
function WorldProjectionGizmoViewport(props: WorldProjectionGizmoViewportProps): ReactElement {
  const invalidate = useThree((state) => state.invalidate);
  const [hoveredHit, setHoveredHit] = reactHostPort.useState<ProjectionGizmoHit | null>(null);
  const [palette, setPalette] = reactHostPort.useState(() => resolveProjectionGizmoVisualPalette());
  useCanvasAppearanceSync(
    reactHostPort.useCallback(() => {
      clearColorResolveCache();
      setPalette(resolveProjectionGizmoVisualPalette());
    }, []),
  );
  reactHostPort.useEffect(() => {
    invalidate();
  }, [hoveredHit, invalidate, palette]);
  const axisHeadScale = 0.62;
  const headBase = {
    palette,
    axisHeadScale,
    onHitSelect: props.onHitSelect,
    onPointerOver: setHoveredHit,
    onPointerOut: () => setHoveredHit(null),
  };
  const visualState = (hit: ProjectionGizmoHit) => projectionGizmoHitVisualState(hit, hoveredHit);
  const shaftOpacity = hoveredHit ? 0.38 : 0.72;
  const axisOpacity = (axis: "x" | "y" | "z") => {
    if (!hoveredHit || hoveredHit.type !== "face" || hoveredHit.axis !== axis) {
      return shaftOpacity;
    }
    return 1;
  };
  const faceHit = (axis: "x" | "y" | "z", sign: 1 | -1): ProjectionGizmoHit => ({ type: "face", axis, sign });
  // 🧭️ Upper corners keep prominent heads; lower corners match negative-axis ends — less relevant under-views.
  const cornerHits: readonly { readonly position: [number, number, number]; readonly hit: Extract<ProjectionGizmoHit, { type: "corner" }>; readonly prominent?: boolean }[] = [
    { position: [0.72, 0.72, 0.72], hit: { type: "corner", quadrant: "ne", hemisphere: "upper" }, prominent: true },
    { position: [-0.72, 0.72, 0.72], hit: { type: "corner", quadrant: "nw", hemisphere: "upper" }, prominent: true },
    { position: [0.72, -0.72, 0.72], hit: { type: "corner", quadrant: "se", hemisphere: "upper" }, prominent: true },
    { position: [-0.72, -0.72, 0.72], hit: { type: "corner", quadrant: "sw", hemisphere: "upper" }, prominent: true },
    { position: [0.72, 0.72, -0.72], hit: { type: "corner", quadrant: "ne", hemisphere: "lower" } },
    { position: [-0.72, 0.72, -0.72], hit: { type: "corner", quadrant: "nw", hemisphere: "lower" } },
    { position: [0.72, -0.72, -0.72], hit: { type: "corner", quadrant: "se", hemisphere: "lower" } },
    { position: [-0.72, -0.72, -0.72], hit: { type: "corner", quadrant: "sw", hemisphere: "lower" } },
  ];
  return (
    <group scale={22}>
      <WorldOrbitViewGizmoViewportAxis color={props.axisColors.x} rotation={[0, 0, 0]} scale={props.axisScale} opacity={axisOpacity("x")} />
      <WorldOrbitViewGizmoViewportAxis color={props.axisColors.y} rotation={[0, 0, Math.PI / 2]} scale={props.axisScale} opacity={axisOpacity("y")} />
      <WorldOrbitViewGizmoViewportAxis color={props.axisColors.z} rotation={[0, -Math.PI / 2, 0]} scale={props.axisScale} opacity={axisOpacity("z")} />
      {cornerHits.map((corner) => (
        <WorldProjectionGizmoCornerShaft key={`shaft-${corner.hit.quadrant}-${corner.hit.hemisphere}`} to={corner.position} color={props.neutralColor} opacity={shaftOpacity} />
      ))}
      <WorldProjectionGizmoHitHead position={[1, 0, 0]} hit={faceHit("x", 1)} prominent color={props.axisColors.x} neutral={false} visualState={visualState(faceHit("x", 1))} {...headBase} />
      <WorldProjectionGizmoHitHead position={[-1, 0, 0]} hit={faceHit("x", -1)} color={props.axisColors.x} neutral={false} visualState={visualState(faceHit("x", -1))} {...headBase} />
      <WorldProjectionGizmoHitHead position={[0, 1, 0]} hit={faceHit("y", 1)} prominent color={props.axisColors.y} neutral={false} visualState={visualState(faceHit("y", 1))} {...headBase} />
      <WorldProjectionGizmoHitHead position={[0, -1, 0]} hit={faceHit("y", -1)} color={props.axisColors.y} neutral={false} visualState={visualState(faceHit("y", -1))} {...headBase} />
      <WorldProjectionGizmoHitHead position={[0, 0, 1]} hit={faceHit("z", 1)} prominent color={props.axisColors.z} neutral={false} visualState={visualState(faceHit("z", 1))} {...headBase} />
      <WorldProjectionGizmoHitHead position={[0, 0, -1]} hit={faceHit("z", -1)} color={props.axisColors.z} neutral={false} visualState={visualState(faceHit("z", -1))} {...headBase} />
      {cornerHits.map((corner) => (
        <WorldProjectionGizmoHitHead key={`${corner.hit.type}-${corner.hit.quadrant}-${corner.hit.hemisphere}`} position={corner.position} hit={corner.hit} prominent={corner.prominent} color={props.neutralColor} neutral visualState={visualState(corner.hit)} {...headBase} />
      ))}
      <WorldProjectionGizmoHitHead position={[0, 0, 0]} hit={{ type: "center" }} prominent color={props.neutralColor} neutral visualState={visualState({ type: "center" })} {...headBase} />
    </group>
  );
}
//#endregion 🧭️WorldOrbitViewGizmoViewport

/** 🎯️ Emits every navigation-cube selection so an unchanged spec can still restore a camera moved by orbit controls. */
export function dispatchProjectionGizmoHit(hit: ProjectionGizmoHit, currentSpec: WorldProjectionSpec | undefined, onSpecSelect: (spec: WorldProjectionSpec) => void, onViewSelect?: (view: OrbitCameraViewId) => void): void {
  const spec = resolveProjectionGizmoSpec(hit, currentSpec);
  onSpecSelect(spec);
  const view = worldProjectionSpecToOrbitView(spec);
  if (view) {
    onViewSelect?.(view);
  }
}

/** 🧭️ CAD Z-up viewport navigation cube (faces / corners / center) anchored bottom-right above the folded projection pane. */
export function WorldOrbitViewGizmo(props: WorldOrbitViewGizmoProps): ReactElement | null {
  const { size } = useThree();
  const [axisColors, setAxisColors] = reactHostPort.useState(() => resolveSpatialAxisColors());
  const [neutralColor, setNeutralColor] = reactHostPort.useState(() => resolveWorldGizmoNeutralColor());
  const placement = reactHostPort.useMemo(() => resolveSceneGizmoViewportPlacement(size), [size]);
  const axisScale = reactHostPort.useMemo(() => [0.88, 0.036, 0.036] as [number, number, number], []);

  useCanvasAppearanceSync(
    reactHostPort.useCallback(() => {
      setAxisColors(resolveSpatialAxisColors());
      setNeutralColor(resolveWorldGizmoNeutralColor());
    }, []),
  );

  if (props.show === false) {
    return null;
  }
  return (
    <GizmoHelper alignment={placement.alignment} margin={placement.margin}>
      <WorldProjectionGizmoViewport
        axisColors={axisColors}
        neutralColor={neutralColor}
        axisScale={axisScale}
        onHitSelect={(hit) => dispatchProjectionGizmoHit(hit, props.projectionSpec, props.onSpecSelect, props.onViewSelect)}
      />
    </GizmoHelper>
  );
}

export interface WorldProjectionSnapDriverProps {
  readonly pendingSpec: WorldProjectionSpec | null;
  readonly currentProjectionSpec?: WorldProjectionSpec;
  readonly onPendingSpecClear: () => void;
  readonly onCameraChange?: (state: WorldCameraState) => void;
  readonly onSpecSnap?: (spec: WorldProjectionSpec, state: WorldCameraState) => void;
}

interface WorldProjectionSnapAnim {
  readonly spec: WorldProjectionSpec;
  readonly fromPos: Vector3;
  readonly fromUp: Vector3;
  readonly fromMatrix: Matrix4;
  readonly to: WorldCameraState;
  readonly toFov: number;
  readonly start: number;
  holding: boolean;
}

/** 🎞️ Interpolates the orbit camera to a {@link WorldProjectionSpec} when `pendingSpec` is set — morphs pose AND the
 * projection matrix itself (via {@link worldProjectionGoalMatrix} / {@link worldProjectionMorphMatrix}) over the same 280ms
 * tween, so perspective↔orthographic, FOV, oblique shear and two-point shift all interpolate instead of popping at remount. */
export function WorldProjectionSnapDriver(props: WorldProjectionSnapDriverProps): null {
  const { camera, size } = useThree();
  const controls = useThree((state) => state.controls as OrbitControlsTarget | null);
  const invalidate = useThree((state) => state.invalidate);
  const { setSnapGate, morphRef } = useWorldOrbitViewSnapGate();
  const animRef = reactHostPort.useRef<WorldProjectionSnapAnim | null>(null);

  reactHostPort.useEffect(() => {
    if (!props.pendingSpec || !camera || !controls?.target) {
      return;
    }
    const pendingSpec = props.pendingSpec;
    const fromSpec = props.currentProjectionSpec;
    const targetCad = threeVec3ToCad(controls.target);
    const fromZoom = worldCameraZoom(camera);
    const isOrthographic = worldCameraIsOrthographic(camera);
    const fromFov = worldCameraFov(camera);
    const to = worldProjectionTransitionPose(pendingSpec, {
      position: threeVec3ToCad(camera.position),
      target: targetCad,
      up: threeVec3ToCad(camera.up),
      zoom: fromZoom,
      isOrthographic,
      projectionSpec: fromSpec,
      viewport: size,
      fov: fromFov,
    });
    const toFov = worldProjectionPerspectiveFov(pendingSpec);
    animRef.current = { spec: pendingSpec, fromPos: camera.position.clone(), fromUp: camera.up.clone(), fromMatrix: camera.projectionMatrix.clone(), to, toFov, start: performance.now(), holding: false };
    morphRef.current = { fromSpec, toSpec: pendingSpec, fromMatrix: animRef.current.fromMatrix, fromFov: fromFov ?? worldProjectionPerspectiveFov(fromSpec ?? pendingSpec), toFov, eased: 0, holding: false };
    setSnapGate(true);
    props.onPendingSpecClear();
    invalidate();
  }, [camera, controls, invalidate, morphRef, props.currentProjectionSpec, props.pendingSpec, props.onPendingSpecClear, setSnapGate, size]);

  useFrame(() => {
    const anim = animRef.current;
    if (!anim || !camera || !controls) {
      return;
    }
    if (anim.holding) {
      const goal = worldProjectionGoalMatrix(anim.spec, { zoom: anim.to.zoom, fov: anim.toFov, viewport: size, near: 0.2, far: WORLD_ORBIT_CAMERA_MIN_FAR });
      camera.projectionMatrix.copy(goal);
      camera.projectionMatrixInverse.copy(goal).invert();
      invalidate();
      return;
    }
    const progress = Math.min(1, (performance.now() - anim.start) / WORLD_ORBIT_VIEW_SNAP_DURATION_MS);
    const eased = easeInOutCubic(progress);
    const nextPos = cadVec3ToThree(lerpVec3(threeVec3ToCad(anim.fromPos), anim.to.position, eased));
    const nextUp = cadVec3ToThree(lerpVec3(threeVec3ToCad(anim.fromUp), anim.to.up ?? ORBIT_CAMERA_Z_UP, eased));
    camera.position.set(nextPos[0], nextPos[1], nextPos[2]);
    camera.up.set(nextUp[0], nextUp[1], nextUp[2]);
    const target = cadVec3ToThree(anim.to.target);
    camera.lookAt(target[0], target[1], target[2]);
    controls.target.set(target[0], target[1], target[2]);
    controls.update();
    const goal = worldProjectionGoalMatrix(anim.spec, { zoom: anim.to.zoom, fov: anim.toFov, viewport: size, near: 0.2, far: WORLD_ORBIT_CAMERA_MIN_FAR });
    const morphed = worldProjectionMorphMatrix(anim.fromMatrix, goal, eased);
    camera.projectionMatrix.copy(morphed);
    camera.projectionMatrixInverse.copy(morphed).invert();
    if (morphRef.current) {
      morphRef.current.eased = eased;
    }
    if (progress < 1) {
      invalidate();
      return;
    }
    anim.holding = true;
    if (morphRef.current) {
      morphRef.current.eased = 1;
      morphRef.current.holding = true;
    }
    // Zoom/FOV fields are synced without `updateProjectionMatrix()` — that would rebuild the matrix from the camera's
    // stale left/right/top/bottom/aspect fields and stomp the pinned morph goal above.
    const zoomable = camera as Camera & { readonly isPerspectiveCamera?: boolean; readonly isOrthographicCamera?: boolean; zoom?: number; fov?: number };
    if (zoomable.isPerspectiveCamera || zoomable.isOrthographicCamera) {
      zoomable.zoom = anim.to.zoom;
    }
    if (zoomable.isPerspectiveCamera) {
      zoomable.fov = anim.toFov;
    }
    setSnapGate(false);
    props.onCameraChange?.(anim.to);
    props.onSpecSnap?.(anim.spec, anim.to);
    invalidate();
  });

  reactHostPort.useEffect(() => {
    if (animRef.current?.holding) {
      animRef.current = null;
    }
    if (morphRef.current?.holding) {
      morphRef.current = null;
    }
  }, [camera, morphRef]);

  return null;
}

export interface WorldOrbitViewSnapDriverProps {
  readonly pendingView: OrbitCameraViewId | null;
  readonly onPendingViewClear: () => void;
  readonly onCameraChange?: (state: WorldCameraState) => void;
  readonly onProjectionChange?: (projection: OrbitCameraProjection) => void;
  readonly onViewSnap?: (view: OrbitCameraViewId, state: WorldCameraState) => void;
}

/** 🎞️ Interpolates the orbit camera to a named view when `pendingView` is set. */
export function WorldOrbitViewSnapDriver(props: WorldOrbitViewSnapDriverProps): ReactElement {
  const pendingViewRef = reactHostPort.useRef<OrbitCameraViewId | null>(null);
  reactHostPort.useEffect(() => {
    if (props.pendingView) {
      pendingViewRef.current = props.pendingView;
    }
  }, [props.pendingView]);
  const pendingSpec = props.pendingView ? orbitViewToWorldProjectionSpec(props.pendingView) : null;
  return (
    <WorldProjectionSnapDriver
      pendingSpec={pendingSpec}
      onPendingSpecClear={props.onPendingViewClear}
      onCameraChange={(state) => {
        props.onCameraChange?.(state);
        const view = pendingViewRef.current;
        if (view) {
          props.onViewSnap?.(view, state);
          pendingViewRef.current = null;
        }
        if (state.projection) {
          props.onProjectionChange?.(state.projection);
        }
      }}
    />
  );
}

export interface WorldOrbitViewControlsProps {
  readonly show?: boolean;
  readonly projectionSpec?: WorldProjectionSpec;
  readonly externalPendingSpec?: WorldProjectionSpec | null;
  readonly onExternalPendingSpecClear?: () => void;
  readonly onCameraChange?: (state: WorldCameraState) => void;
  readonly onProjectionChange?: (projection: OrbitCameraProjection) => void;
  readonly onViewSnap?: (view: OrbitCameraViewId, state: WorldCameraState) => void;
  readonly onSpecSnap?: (spec: WorldProjectionSpec, state: WorldCameraState) => void;
  /** 🎞️ Reports the unified pane-or-gizmo pending spec so the host can pre-mount {@link WorldProjectionRig}'s
   * curvilinear pass before the camera remount (see `pendingSpec` there). */
  readonly onPendingSpecChange?: (spec: WorldProjectionSpec | null) => void;
}

/** 🧭️ Bundles {@link WorldOrbitViewGizmo} and {@link WorldProjectionSnapDriver}. */
export function WorldOrbitViewControls(props: WorldOrbitViewControlsProps): ReactElement {
  const [internalPendingSpec, setInternalPendingSpec] = reactHostPort.useState<WorldProjectionSpec | null>(null);
  const pendingSpec = props.externalPendingSpec ?? internalPendingSpec;
  const clearPendingSpec = reactHostPort.useCallback(() => {
    if (props.externalPendingSpec) {
      props.onExternalPendingSpecClear?.();
      return;
    }
    setInternalPendingSpec(null);
  }, [props.externalPendingSpec, props.onExternalPendingSpecClear]);
  const onPendingSpecChange = props.onPendingSpecChange;
  reactHostPort.useEffect(() => {
    onPendingSpecChange?.(pendingSpec);
  }, [onPendingSpecChange, pendingSpec]);
  return (
    <>
      <WorldProjectionSnapDriver
        pendingSpec={pendingSpec}
        currentProjectionSpec={props.projectionSpec}
        onPendingSpecClear={clearPendingSpec}
        onCameraChange={props.onCameraChange}
        onSpecSnap={props.onSpecSnap}
      />
      <WorldOrbitViewGizmo show={props.show} projectionSpec={props.projectionSpec} onSpecSelect={setInternalPendingSpec} />
    </>
  );
}
// #endregion 📷️OrbitCameraView
