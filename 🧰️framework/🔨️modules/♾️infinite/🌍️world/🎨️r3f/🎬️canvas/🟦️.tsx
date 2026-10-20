/** 🎬️ The world canvas: orbit mouse bindings, the demand frameloop kick, the gated orbit controls, the camera invalidator and the canvas component.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | r3f}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { reactHostPort, ThreeOrbitControls } from "@semio-tech/ui-react";
import { GestureRecognizer, applyPinchToOrbit, type PinchStep } from "@semio-tech/framework";
import React, { type CSSProperties, type ReactElement, type ReactNode } from "react";
import {
  type Camera,
  Canvas,
  MOUSE,
  ThreeOrthographicCamera,
  ThreePerspectiveCamera,
  useFrame,
  useThree,
  Vector3
} from "../🔌️adapters/🟦️.ts";
import {
  classifyWorldNavigationGestures,
  type OrbitCameraProjection,
  threeVec3ToCad,
  type Vec3,
  type WorldCameraState,
  WorldEventBindingController,
  type WorldNavigationGesture,
  type WorldNavigationSnapshot
} from "../🧷️r3f-foundation/🟦️.ts";
import { useWorldOrbitViewSnapGate } from "../📷️camera-view/🟦️.tsx";
import { WorldLayerStack } from "../🗂️layers/🟦️.tsx";

// #region 🖱️OrbitMouseBindings
/** 🖱️ Orbit-controls instance with mutable mouse button map. */
export type WorldOrbitControlsBinding = {
  readonly mouseButtons: { LEFT?: number | null; MIDDLE?: number | null; RIGHT?: number | null };
  readonly target?: Vector3;
  readonly enabled?: boolean;
  readonly update?: () => void;
};

/**
 * 🎯️ The target a camera report is allowed to use — the orbit controls' OWN, or nothing.
 *
 * A camera report says where the USER left the camera, so it may only ever carry a target some
 * controls object actually holds. `WorldOrbitGated.reportCamera` used to answer
 * `controls?.target ?? new Vector3().set(0, 0, 0)`: with `useThree().controls` momentarily null —
 * between a rig remount and r3f re-registering the controls — it published the world origin as the
 * user's target and overwrote the pose the boot framing had just computed
 * (`📓️boot-camera-framing-2026-09-15.md` §6). A non-finite component is refused for the same reason:
 * an invented number is not a reading.
 **/
export function worldCameraReportTargetV1(controls: WorldOrbitControlsBinding | null | undefined): { readonly x: number; readonly y: number; readonly z: number } | null {
  const target = controls?.target;
  if (!target) return null;
  if (!Number.isFinite(target.x) || !Number.isFinite(target.y) || !Number.isFinite(target.z)) return null;
  return { x: target.x, y: target.y, z: target.z };
}

/** 🖱️ Default orbit mouse map: middle always pans; Alt+right orbits when rotation is enabled. */
export function resolveWorldOrbitMouseButtonsIdle(_projection: OrbitCameraProjection = "perspective", _rotateEnabled = true): {
  readonly LEFT: number | null;
  readonly MIDDLE: number;
  readonly RIGHT: number | null;
} {
  return { LEFT: null, MIDDLE: MOUSE.PAN, RIGHT: null };
}

/** 🖱️ Resets orbit mouse buttons to {@link resolveWorldOrbitMouseButtonsIdle}. */
export function applyWorldOrbitMouseButtonsIdle(controls: WorldOrbitControlsBinding, projection: OrbitCameraProjection = "perspective", rotateEnabled = true): void {
  const idle = resolveWorldOrbitMouseButtonsIdle(projection, rotateEnabled);
  controls.mouseButtons.LEFT = idle.LEFT;
  controls.mouseButtons.MIDDLE = idle.MIDDLE;
  controls.mouseButtons.RIGHT = idle.RIGHT;
  controls.update?.();
}

/** 🖱️ Maps right-button modifiers: plain right → context menu, Shift+right → pan, Alt+right → orbit. */
export function resolveWorldOrbitRightMouseAction(event: Pick<PointerEvent, "button" | "altKey" | "shiftKey">, _projection: OrbitCameraProjection = "perspective"): number | null {
  if (event.button !== 2) {
    return null;
  }
  if (event.shiftKey) {
    return MOUSE.PAN;
  }
  if (event.altKey) {
    return MOUSE.ROTATE;
  }
  return null;
}

export interface WorldOrbitRightMouseBindingsOptions {
  readonly projection?: OrbitCameraProjection;
  readonly rotateEnabled?: boolean;
  readonly onRightPointerDown?: (event: PointerEvent) => boolean;
  readonly onRightPointerDrag?: (event: PointerEvent, distancePx: number) => void;
  readonly onRightPointerUp?: (event: PointerEvent) => void;
  readonly dragThresholdPx?: number;
}

/** 🖱️ False from `onRightPointerDown` fully suppresses orbit's own button assignment for that gesture (e.g. Alt+right-click over a vortex opens a suggestion popup instead of orbiting). */
export function shouldAssignWorldOrbitRightMouse(event: PointerEvent, onRightPointerDown?: (event: PointerEvent) => boolean): boolean {
  return onRightPointerDown?.(event) !== false;
}

/** 🖱️ Binds projection-aware Alt+right and Shift+right actions while leaving plain right click for context menus. */
export function useWorldOrbitRightMouseBindings(controls: WorldOrbitControlsBinding | null, domElement: HTMLElement | undefined, options?: WorldOrbitRightMouseBindingsOptions): void {
  const projection = options?.projection ?? "perspective";
  const rotateEnabled = options?.rotateEnabled ?? true;
  const optionsRef = reactHostPort.useRef(options);
  optionsRef.current = options;
  reactHostPort.useLayoutEffect(() => {
    if (controls) {
      applyWorldOrbitMouseButtonsIdle(controls, projection, rotateEnabled);
    }
  }, [controls, projection, rotateEnabled]);
  reactHostPort.useEffect(() => {
    if (!controls || !domElement) {
      return;
    }
    const dragThresholdPx = options?.dragThresholdPx ?? 0;
    let rightPointer: { readonly pointerId: number; readonly x: number; readonly y: number } | null = null;
    const assignRightMouse = (event: PointerEvent) => {
      if (event.button !== 2) {
        return;
      }
      controls.mouseButtons.RIGHT = (optionsRef.current?.rotateEnabled ?? true) ? resolveWorldOrbitRightMouseAction(event, optionsRef.current?.projection ?? "perspective") : event.shiftKey ? MOUSE.PAN : null;
      controls.update?.();
    };
    const resetRightMouse = () => {
      controls.mouseButtons.RIGHT = resolveWorldOrbitMouseButtonsIdle(optionsRef.current?.projection ?? "perspective", optionsRef.current?.rotateEnabled ?? true).RIGHT;
      controls.update?.();
    };
    const onPointerDown = (event: PointerEvent) => {
      if (event.button !== 2) {
        return;
      }
      if (!shouldAssignWorldOrbitRightMouse(event, optionsRef.current?.onRightPointerDown)) {
        return;
      }
      rightPointer = { pointerId: event.pointerId, x: event.clientX, y: event.clientY };
      assignRightMouse(event);
    };
    const onPointerMove = (event: PointerEvent) => {
      const start = rightPointer;
      if (!start || start.pointerId !== event.pointerId) {
        return;
      }
      const distancePx = Math.hypot(event.clientX - start.x, event.clientY - start.y);
      if (distancePx >= dragThresholdPx) {
        optionsRef.current?.onRightPointerDrag?.(event, distancePx);
      }
    };
    const onPointerUp = (event: PointerEvent) => {
      const start = rightPointer;
      if (!start || start.pointerId !== event.pointerId) {
        return;
      }
      rightPointer = null;
      optionsRef.current?.onRightPointerUp?.(event);
      resetRightMouse();
    };
    const bindings = new WorldEventBindingController();
    bindings.listen(domElement, "pointerdown", onPointerDown as EventListener, true);
    bindings.listen(window, "pointermove", onPointerMove as EventListener);
    bindings.listen(window, "pointerup", onPointerUp as EventListener, true);
    return () => bindings.dispose();
  }, [controls, domElement, options?.dragThresholdPx, projection]);
}
// #endregion 🖱️OrbitMouseBindings

// #region 🎬️WorldCanvas
type OrbitControlsBinding = WorldOrbitControlsBinding;

/** 🎞️ Kicks demand-frameloop renders across mount + orbit/grid setup frames and whenever the
 * store reports a pending invalidate (async GLB/texture commits after the initial kick). */
export function DemandFrameloopKick(): null {
  const invalidate = useThree((state) => state.invalidate);
  const gl = useThree((state) => state.gl);
  reactHostPort.useLayoutEffect(() => {
    invalidate();
    let frame = 0;
    let raf = 0;
    const tick = () => {
      invalidate();
      frame += 1;
      if (frame < 4) raf = requestAnimationFrame(tick);
    };
    raf = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(raf);
  }, [invalidate]);
  reactHostPort.useEffect(() => {
    let cancelled = false;
    let raf = 0;
    let frames = 0;
    const pump = () => {
      if (cancelled) return;
      invalidate();
      frames += 1;
      if (frames < 24) raf = requestAnimationFrame(pump);
    };
    raf = requestAnimationFrame(pump);
    const onContextRestored = () => invalidate();
    gl.domElement.addEventListener("webglcontextrestored", onContextRestored);
    return () => {
      cancelled = true;
      cancelAnimationFrame(raf);
      gl.domElement.removeEventListener("webglcontextrestored", onContextRestored);
    };
  }, [gl, invalidate]);
  return null;
}

export interface WorldOrbitGatedProps {
  readonly camera?: ThreePerspectiveCamera | null;
  readonly zoom?: number;
  readonly projection?: OrbitCameraProjection;
  /** 🔒️ Rotation/polar-angle limits implied by the active {@link WorldProjectionSpec}; see {@link worldProjectionOrbitConstraints}. */
  readonly constraints?: { readonly rotate: boolean; readonly minPolar?: number; readonly maxPolar?: number };
  readonly onCamera?: (state: WorldCameraState) => void;
  readonly controlsGate?: boolean;
  readonly onCameraNavigate?: (active: boolean) => void;
  readonly controlsKey?: string | number;
  readonly onRightPointerDown?: (event: PointerEvent) => boolean;
  /** 🧭️ Reports which gestures (pan/zoom/orbit) a drag/scroll performed, classified between `start` and `end`. */
  readonly onNavigationGestures?: (gestures: readonly WorldNavigationGesture[]) => void;
}

const WORLD_ORBIT_CONSTRAINTS_DEFAULT: NonNullable<WorldOrbitGatedProps["constraints"]> = { rotate: true };

/** 🖱️ The element a world canvas actually RECEIVES pointer and wheel events on.
 *
 * A `<Canvas eventSource={…}>` hands its own events to that source element and stamps
 * `pointer-events: none` on the canvas itself (`@react-three/fiber` `events.connected`, and
 * `pointerEvents = eventSource ? 'none' : 'auto'` in its canvas style) — so anything bound to
 * `gl.domElement` on such a canvas is bound to an element the browser never routes an event to.
 * `WorldCanvas` passes an `eventSource`, which is how camera navigation came to be dead in EVERY
 * react world3d window: orbit, pan and wheel zoom were all listening on the canvas
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END — no gesture moved `data-viewport-camera-json` off its
 * seed pose on the generation3d preview, measured for wheel, alt+right, shift+right and middle).
 */
export function useWorldPointerTarget(): HTMLElement {
  const gl = useThree((state) => state.gl);
  const connected = useThree((state) => state.events.connected);
  return (connected instanceof HTMLElement ? connected : gl.domElement) as HTMLElement;
}

/** 🤏️ Applies one shared-recognizer pinch step to a Three orbit rig through the renderer-neutral
 * {@link applyPinchToOrbit} law: target-centred dolly (perspective) or zoom factor (orthographic), plus
 * the centroid pan in the camera plane, inside the controls' own distance/zoom limits. */
function applyWorldOrbitPinch(controls: ThreeOrbitControls, camera: Camera, step: PinchStep, viewportHeight: number): void {
  const orthographic = camera instanceof ThreeOrthographicCamera;
  const perspective = camera instanceof ThreePerspectiveCamera;
  if (!orthographic && !perspective) return;
  const next = applyPinchToOrbit(
    { position: [camera.position.x, camera.position.y, camera.position.z], target: [controls.target.x, controls.target.y, controls.target.z], up: [camera.up.x, camera.up.y, camera.up.z], zoom: orthographic ? camera.zoom : 1 },
    step,
    orthographic ? { kind: "orthographic", frustumHeight: camera.top - camera.bottom } : { kind: "perspective", fovYRadians: ((camera as ThreePerspectiveCamera).fov * Math.PI) / 180 },
    viewportHeight,
    { distance: { min: controls.minDistance, max: controls.maxDistance }, zoom: { min: controls.minZoom, max: controls.maxZoom } },
  );
  camera.position.set(next.position[0], next.position[1], next.position[2]);
  controls.target.set(next.target[0], next.target[1], next.target[2]);
  if (orthographic) {
    camera.zoom = next.zoom;
    camera.updateProjectionMatrix();
  }
  controls.update();
}

/**
 * 🛰️ Canvas-local Three orbit-control binding that never crosses the optional Drei runtime boundary.
 *
 * Two-finger touch is NOT Three's: the shared `👆️gesture` {@link GestureRecognizer} listens in the capture
 * phase on the same pointer target, so it sees the second contact before `OrbitControls` does, suspends
 * the controls for the whole latched gesture (the finger left down never resumes orbiting), and drives
 * dolly/zoom + pan itself through {@link applyWorldOrbitPinch}. The same recognizer runs in
 * `🖥️Board2dHost` and the dag `🕸️NodeGraph` surface, so a pinch means one thing on every viewport.
 * `OrbitControls` keeps mouse input and one-finger orbit, and still emits the gesture's `start`/`end`
 * (it tracked the first finger), which is what commits the camera once.
 */
function WorldOrbitControlsBridge({
  camera,
  enabled,
  mouseButtons,
  constraints,
  controlsKey,
  onChange,
  onStart,
  onEnd,
}: {
  readonly camera: Camera;
  readonly enabled: boolean;
  readonly mouseButtons: ReturnType<typeof resolveWorldOrbitMouseButtonsIdle>;
  readonly constraints?: { readonly rotate: boolean; readonly minPolar?: number; readonly maxPolar?: number };
  readonly controlsKey?: string | number;
  readonly onChange: () => void;
  readonly onStart: (controls: ThreeOrbitControls) => void;
  readonly onEnd: (controls: ThreeOrbitControls) => void;
}): null {
  const pointerTarget = useWorldPointerTarget();
  const set = useThree((state) => state.set);
  const get = useThree((state) => state.get);
  const controlsRef = reactHostPort.useRef<ThreeOrbitControls | null>(null);
  const gestureRecognizerRef = reactHostPort.useRef<GestureRecognizer | null>(null);
  const enabledRef = reactHostPort.useRef(enabled);
  enabledRef.current = enabled;
  const callbacksRef = reactHostPort.useRef({ onChange, onStart, onEnd });
  callbacksRef.current = { onChange, onStart, onEnd };
  const resolvedConstraints = constraints ?? WORLD_ORBIT_CONSTRAINTS_DEFAULT;
  reactHostPort.useEffect(() => {
    const controls = new ThreeOrbitControls(camera, pointerTarget);
    controls.enableDamping = false;
    controls.enablePan = true;
    controls.enableZoom = true;
    controls.enabled = enabledRef.current;
    controls.mouseButtons = { ...mouseButtons };
    const change = () => callbacksRef.current.onChange();
    const start = () => callbacksRef.current.onStart(controls);
    const end = () => callbacksRef.current.onEnd(controls);
    controls.addEventListener("change", change);
    controls.addEventListener("start", start);
    controls.addEventListener("end", end);
    controlsRef.current = controls;
    const gestures = new GestureRecognizer();
    gestureRecognizerRef.current = gestures;
    const gesturePoint = (event: PointerEvent) => {
      const rect = pointerTarget.getBoundingClientRect();
      return { pointerId: event.pointerId, x: event.clientX - rect.left, y: event.clientY - rect.top };
    };
    const onGestureDown = (event: PointerEvent) => {
      if (gestures.down(gesturePoint(event)).kind === "pinchBegin") controls.enabled = false;
    };
    const onGestureMove = (event: PointerEvent) => {
      const verdict = gestures.move(gesturePoint(event));
      if (verdict.kind === "pinch" && enabledRef.current) applyWorldOrbitPinch(controls, camera, verdict.step, pointerTarget.clientHeight);
    };
    const onGestureUp = (event: PointerEvent) => {
      if (gestures.up(event.pointerId).kind === "pinchEnd") controls.enabled = enabledRef.current;
    };
    const gestureDocument = pointerTarget.ownerDocument;
    pointerTarget.addEventListener("pointerdown", onGestureDown, { capture: true });
    gestureDocument.addEventListener("pointermove", onGestureMove, { capture: true });
    gestureDocument.addEventListener("pointerup", onGestureUp, { capture: true });
    gestureDocument.addEventListener("pointercancel", onGestureUp, { capture: true });
    set({ controls });
    controls.update();
    return () => {
      pointerTarget.removeEventListener("pointerdown", onGestureDown, { capture: true });
      gestureDocument.removeEventListener("pointermove", onGestureMove, { capture: true });
      gestureDocument.removeEventListener("pointerup", onGestureUp, { capture: true });
      gestureDocument.removeEventListener("pointercancel", onGestureUp, { capture: true });
      if (gestureRecognizerRef.current === gestures) gestureRecognizerRef.current = null;
      controls.removeEventListener("change", change);
      controls.removeEventListener("start", start);
      controls.removeEventListener("end", end);
      controls.dispose();
      controlsRef.current = null;
      if (get().controls === controls) set({ controls: null });
    };
  }, [camera, controlsKey, get, pointerTarget, set]);
  reactHostPort.useEffect(() => {
    const controls = controlsRef.current;
    if (!controls) return;
    controls.enabled = enabled && !gestureRecognizerRef.current?.latched;
    controls.mouseButtons = { ...mouseButtons };
    controls.enableRotate = resolvedConstraints.rotate;
    controls.minPolarAngle = resolvedConstraints.minPolar ?? 0;
    controls.maxPolarAngle = resolvedConstraints.maxPolar ?? Math.PI;
    controls.update();
  }, [enabled, mouseButtons, resolvedConstraints]);
  return null;
}

/** 🛰️ Orbit controls with injectable gate (specializations disable during drag/tools). */
export function WorldOrbitGated(props: WorldOrbitGatedProps): ReactElement | null {
  const sceneCamera = useThree((state) => state.camera);
  const pointerTarget = useWorldPointerTarget();
  const camera = props.camera === undefined ? sceneCamera : props.camera;
  const controls = useThree((s) => s.controls as OrbitControlsBinding | null);
  const gate = props.controlsGate ?? false;
  const { snapGate } = useWorldOrbitViewSnapGate();
  const invalidate = useThree((s) => s.invalidate);
  const projection = props.projection ?? (camera instanceof ThreeOrthographicCamera ? "orthographic" : "perspective");
  const rotateEnabled = props.constraints?.rotate ?? true;
  const mouseButtonsIdle = reactHostPort.useMemo(() => resolveWorldOrbitMouseButtonsIdle(projection, rotateEnabled), [projection, rotateEnabled]);
  useWorldOrbitRightMouseBindings(controls, pointerTarget, { projection, rotateEnabled, onRightPointerDown: props.onRightPointerDown });
  const navigationSnapshotRef = reactHostPort.useRef<WorldNavigationSnapshot | null>(null);
  reactHostPort.useEffect(() => {
    invalidate();
  }, [gate, invalidate]);
  if (!camera || props.camera === null) return null;
  /** 🎯️ The gesture's OWN controls are the authority, never `useThree().controls`.
   *
   * `reportCamera` used to read the r3f store and fall back to `targetScratch.set(0, 0, 0)` — so a
   * report taken while the store's `controls` slot was momentarily null published the WORLD ORIGIN as
   * the user's camera target and overwrote a framed pose with it. A camera report must never invent a
   * target: the `start`/`end` events are fired BY the controls object, so that object is handed in and
   * a report with no target is not made at all (`📓️boot-camera-framing-2026-09-15.md` §6). */
  const reportCamera = (source: OrbitControlsBinding | null) => {
    if (!props.onCamera || !camera) return;
    const tgt = worldCameraReportTargetV1(source ?? controls);
    if (!tgt) return;
    props.onCamera({
      position: threeVec3ToCad(camera.position),
      target: threeVec3ToCad(tgt),
      zoom: props.zoom ?? 1,
      up: threeVec3ToCad(camera.up),
      projection,
    });
  };
  const captureNavigationSnapshot = (source: OrbitControlsBinding | null): WorldNavigationSnapshot | null => {
    if (!camera) return null;
    const tgt = worldCameraReportTargetV1(source ?? controls);
    if (!tgt) return null;
    return {
      position: threeVec3ToCad(camera.position),
      target: threeVec3ToCad(tgt),
      zoom: camera instanceof ThreeOrthographicCamera ? camera.zoom : 1,
      projection,
    };
  };
  return (
    <WorldOrbitControlsBridge
      camera={camera}
      enabled={!gate && !snapGate}
      mouseButtons={mouseButtonsIdle}
      constraints={props.constraints}
      controlsKey={props.controlsKey}
      onChange={() => invalidate()}
      onStart={(source) => {
        invalidate();
        navigationSnapshotRef.current = captureNavigationSnapshot(source);
        props.onCameraNavigate?.(true);
      }}
      onEnd={(source) => {
        invalidate();
        props.onCameraNavigate?.(false);
        reportCamera(source);
        const before = navigationSnapshotRef.current;
        navigationSnapshotRef.current = null;
        const after = captureNavigationSnapshot(source);
        if (before && after && props.onNavigationGestures) {
          const gestures = classifyWorldNavigationGestures(before, after);
          if (gestures.length > 0) props.onNavigationGestures(gestures);
        }
      }}
    />
  );
}

/** 🔄️ Keeps demand frameloop alive while the camera moves. */
export function WorldCameraInvalidator(): null {
  const { controls, camera } = useThree();
  const lastPos = reactHostPort.useRef(new Vector3());
  const lastTarget = reactHostPort.useRef(new Vector3());
  useFrame(({ invalidate }) => {
    const ctrl = controls as { readonly target?: Vector3 } | null;
    if (!ctrl) return;
    const target = ctrl.target;
    const moved = !camera.position.equals(lastPos.current) || (target ? !target.equals(lastTarget.current) : false);
    if (moved) {
      lastPos.current.copy(camera.position);
      if (target) lastTarget.current.copy(target);
      invalidate();
    }
  });
  return null;
}

export interface WorldCanvasProps {
  readonly children?: ReactNode;
  readonly className?: string;
  readonly style?: CSSProperties;
  readonly overlay?: ReactNode;
  readonly dataRootAttr?: string;
  readonly dataLod?: string;
  readonly rootRef?: React.RefObject<HTMLDivElement | null>;
  readonly onContextMenu?: (event: React.MouseEvent) => void;
  readonly extraRootProps?: Record<string, string | undefined>;
  readonly frameloop?: "always" | "demand";
  readonly background?: string;
  readonly cameraUp?: Vec3;
  readonly cameraPosition?: Vec3;
  readonly cameraFov?: number;
  readonly cameraNear?: number;
  readonly cameraFar?: number;
  readonly dpr?: number | [number, number];
  readonly shadows?: boolean | "basic" | "percentage" | "soft" | "variance";
  readonly gl?: React.ComponentProps<typeof Canvas>["gl"];
  readonly onCanvasReady?: (binding: { readonly camera: Camera; readonly domElement: HTMLCanvasElement }) => void;
  readonly onPointerDown?: (event: PointerEvent) => void;
  readonly onPointerMove?: (event: PointerEvent) => void;
  readonly onPointerUp?: (event: PointerEvent) => void;
  readonly onPointerLeave?: (event: PointerEvent) => void;
  readonly onPointerCancel?: (event: PointerEvent) => void;
  readonly onWheel?: (event: WheelEvent) => void;
  readonly onDoubleClick?: (event: MouseEvent) => void;
  readonly onLostPointerCapture?: (event: PointerEvent) => void;
  /** 👆️ Fires only when a click/pointer event hits no interactive object (r3f raycast miss) — the correct signal for "background click deselects", since r3f's per-object `stopPropagation()` does not stop the underlying native DOM event from bubbling to a host-level `onClick`. */
  readonly onPointerMissed?: (event: MouseEvent) => void;
}

/** 🧺️ How a {@link WorldCanvas}'s children reach its r3f root without re-rendering `<Canvas>`: the latest children and the
 * setters of the mounted {@link WorldCanvasChildren}. */
type WorldCanvasChildrenSlotV1 = { current: ReactNode; readonly listeners: Set<React.Dispatch<React.SetStateAction<ReactNode>>> };

/** 🪣️ Renders the latest children of its {@link WorldCanvasChildrenSlotV1} inside the r3f root — the only part of the canvas a
 * parent render reaches. */
function WorldCanvasChildren(props: { readonly slot: WorldCanvasChildrenSlotV1 }): ReactNode {
  const [children, setChildren] = reactHostPort.useState<ReactNode>(() => props.slot.current);
  reactHostPort.useLayoutEffect(() => {
    const slot = props.slot;
    slot.listeners.add(setChildren);
    setChildren(() => slot.current);
    return () => {
      slot.listeners.delete(setChildren);
    };
  }, [props.slot]);
  return children;
}

const WORLD_CANVAS_STYLE: CSSProperties = { height: "100%", width: "100%" };
const WORLD_CANVAS_DEFAULT_DPR: [number, number] = [1, 2];
/** 🖨️ `stencil` is on because the world section cap (`World3dSection.cap`) closes a cut with a stencil pass. */
const WORLD_CANVAS_DEFAULT_GL = { antialias: true, stencil: true };

/**
 * 🌍️ Generic infinite-world r3f canvas shell (`frameloop="demand"`).
 *
 * 🎯️ The canvas is mounted into — and binds its DOM events to — a wrapper element this shell names
 * itself, never r3f's private inner div. Left to itself, `<Canvas>` connects its handlers to that
 * inner div inside the r3f root's OWN commit, which lands a turn after the shell has already swapped
 * the window's surfaces away on a mode or role switch: the div is gone by then and the connect
 * raises `Cannot read properties of null (reading 'addEventListener')` as an uncaught page error,
 * one per switch, followed by the `Context Lost` of the canvas nobody kept
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, measured on `http://127.0.0.1:6018/?plugin=generation3d`).
 * Naming the source removes the null rather than swallowing it — the canvas exists only once the
 * wrapper does, and a detached element still accepts a listener. The wrapper holds the canvas and
 * nothing else, so the event scope is exactly the one r3f's inner div had and `props.overlay` stays
 * outside it.
 *
 * 📐️ `<Canvas>` renders only when its own configuration changes: a Canvas render re-runs r3f's `configure`, which publishes a
 * fresh `size` + `viewport` on EVERY call (it compares the 8-key measured rect with the 4-key stored size), re-rendering every
 * `useThree` size subscriber — twice per canvas per world hover (ticket 26/09/23 F3). Handlers read the latest props through
 * a ref and the children travel through {@link WorldCanvasChildren}.
 *
 * 🖱️ The wheel hook belongs on the EVENT SOURCE, not on the canvas: a `<Canvas eventSource>`
 * stamps `pointer-events: none` on its canvas, so a wheel listener bound there is never called.
 */
export function WorldCanvas(props: WorldCanvasProps): ReactElement {
  const extra = props.extraRootProps ?? {};
  const frameloop = props.frameloop ?? "demand";
  const cameraUp = props.cameraUp ?? ([0, 0, 1] as Vec3);
  const ownedCamera = props.cameraPosition !== undefined;
  const onWheelRef = reactHostPort.useRef(props.onWheel);
  const [canvasEventSource, setCanvasEventSource] = reactHostPort.useState<HTMLDivElement | null>(null);
  onWheelRef.current = props.onWheel;
  const propsRef = reactHostPort.useRef(props);
  propsRef.current = props;
  const [childrenSlot] = reactHostPort.useState<WorldCanvasChildrenSlotV1>(() => ({ current: props.children, listeners: new Set() }));
  reactHostPort.useLayoutEffect(() => {
    childrenSlot.current = props.children;
    for (const listener of childrenSlot.listeners) listener(() => props.children);
  }, [childrenSlot, props.children]);
  const cameraOptions = ownedCamera
    ? {
        up: [...cameraUp] as [number, number, number],
        position: [...props.cameraPosition!] as [number, number, number],
        fov: props.cameraFov ?? 45,
        ...(props.cameraNear !== undefined ? { near: props.cameraNear } : {}),
        ...(props.cameraFar !== undefined ? { far: props.cameraFar } : {}),
      }
    : undefined;
  const cameraKey = JSON.stringify(cameraOptions ?? null);
  const dpr = props.dpr ?? WORLD_CANVAS_DEFAULT_DPR;
  const dprKey = JSON.stringify(dpr);
  const gl = props.gl ?? WORLD_CANVAS_DEFAULT_GL;
  const glKey = typeof gl === "function" ? gl : JSON.stringify(gl);
  const canvas = reactHostPort.useMemo(
    () =>
      canvasEventSource ? (
        <Canvas
          eventSource={canvasEventSource}
          frameloop={frameloop}
          style={WORLD_CANVAS_STYLE}
          dpr={dpr}
          shadows={props.shadows}
          camera={cameraOptions}
          gl={gl}
          onPointerDown={(event) => propsRef.current.onPointerDown?.(event.nativeEvent)}
          onPointerMove={(event) => propsRef.current.onPointerMove?.(event.nativeEvent)}
          onPointerUp={(event) => propsRef.current.onPointerUp?.(event.nativeEvent)}
          onPointerLeave={(event) => propsRef.current.onPointerLeave?.(event.nativeEvent)}
          onPointerCancel={(event) => propsRef.current.onPointerCancel?.(event.nativeEvent)}
          onWheel={(event) => propsRef.current.onWheel?.(event.nativeEvent)}
          onContextMenu={(event) => propsRef.current.onContextMenu?.(event)}
          onDoubleClick={(event) => propsRef.current.onDoubleClick?.(event.nativeEvent)}
          onLostPointerCapture={(event) => propsRef.current.onLostPointerCapture?.(event.nativeEvent)}
          onPointerMissed={(event) => propsRef.current.onPointerMissed?.(event)}
          onCreated={({ camera, gl: renderer }) => {
            propsRef.current.onCanvasReady?.({ camera, domElement: renderer.domElement });
          }}
        >
          {frameloop === "demand" ? <DemandFrameloopKick /> : null}
          {props.background ? <color attach="background" args={[props.background]} /> : null}
          <WorldLayerStack>
            <WorldCanvasChildren slot={childrenSlot} />
          </WorldLayerStack>
        </Canvas>
      ) : null,
    [canvasEventSource, frameloop, dprKey, props.shadows, cameraKey, glKey, props.background, childrenSlot],
  );
  reactHostPort.useEffect(() => {
    if (!canvasEventSource) return;
    const onWheel = (event: WheelEvent) => {
      event.preventDefault();
      onWheelRef.current?.(event);
    };
    canvasEventSource.addEventListener("wheel", onWheel, { passive: false });
    return () => canvasEventSource.removeEventListener("wheel", onWheel);
  }, [canvasEventSource]);
  const setRootRef = reactHostPort.useCallback(
    (element: HTMLDivElement | null) => {
      const external = props.rootRef;
      if (!external) return;
      external.current = element;
    },
    [props.rootRef],
  );
  return (
    <div
      ref={setRootRef}
      className={props.className ?? "relative h-full w-full"}
      style={{ width: "100%", height: "100%", touchAction: "none", overscrollBehavior: "contain", ...props.style }}
      onContextMenu={props.onContextMenu}
      {...(props.dataRootAttr ? { [props.dataRootAttr]: true } : {})}
      {...(props.dataLod ? { "data-world-lod": props.dataLod } : {})}
      {...extra}
    >
      <div ref={setCanvasEventSource} style={{ width: "100%", height: "100%" }}>
        {canvas}
      </div>
      {props.overlay}
    </div>
  );
}
// #endregion 🎬️WorldCanvas
