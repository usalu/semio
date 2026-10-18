import * as THREE from "three";
import type { ThreeEvent } from "@react-three/fiber";
import * as React from "react";
import { reactHostPort, sceneHostPort } from "../../🔌️Ports/🟦️.tsx";
import { type GumballConfig, type GumballHandleKind, type GumballPose, type GumballVec3, gumballAxisRotateAngle, gumballAxisScaleFactor, gumballConfigVisible, gumballEyeFromPivot, gumballHandleEnabled, gumballLen, gumballPoseFromObject3D, gumballProjectRayOntoAxis, gumballRayPlanePoint, gumballResolveDragSnaps, gumballSnapScalar, gumballSub, gumballV3, resolveGumballConfig } from "../🧿️gumball-model/🟦️.ts";
import { type GumballHandleVisualState, GumballInteractionPreview, type GumballVisualPalette, gumballBaseColorForKind, gumballHandleVisualState, gumballResolveHandleVisual, gumballWorldAxis, useGumballVisualPalette } from "../🎨️gumball-visual/🟦️.tsx";
import { gumballBindHandleRaycast, gumballNdcFromPointer, gumballPointerConsumesCanvasEventRef, gumballRayFromNdc, gumballRaycastKindAtClientPoint, gumballRaycastOwnedAtClientPoint } from "../🎯️gumball-pick/🟦️.ts";
import { GUMBALL_ARROW_HEAD, GUMBALL_ARROW_HEAD_RADIUS_SCALE, GUMBALL_ARROW_RADIUS, GUMBALL_AXIS_BY_KIND, GUMBALL_HANDLE_LENGTH, GUMBALL_PLANE_OFFSET, GUMBALL_PLANE_SIZE, GUMBALL_RING_RADIUS, GUMBALL_RING_TUBE, GUMBALL_SCALE_PICK, GUMBALL_SCALE_RENDER_ORDER, GUMBALL_UNIFORM_BOX, GumballPlaneScaleLHandle, type GumballPreviewAxis, GumballScaleAxisHandle, type GumballScalePlane, gumballPlaneScaleFactors, gumballScalePlaneAxisIndices, gumballWorldPointToLocalOffset } from "../🔩️gumball-handles/🟦️.tsx";
import { useFrame, useThree } from "../🔌️three-hooks/🟦️.ts";

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

/** 🎛️ Props for {@link UnifiedGumball}. */
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

/** 🎛️ Rhino-style unified move / rotate / scale gumball for R3F scenes. */
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
