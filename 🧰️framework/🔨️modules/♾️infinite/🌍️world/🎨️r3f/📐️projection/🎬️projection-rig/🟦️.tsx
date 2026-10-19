/** 🎬️ The world projection rig: the rig, applier and matrix driver, the curvilinear unprojection, its shaders and capture target, and the curvilinear pass. */
import { reactHostPort, sceneHostPort } from "@semio-tech/ui-react";
import { type ReactElement } from "react";
import { type Camera, HalfFloatType, LinearFilter, LinearSRGBColorSpace, Mesh, OrthographicCamera, PerspectiveCamera, PlaneGeometry, RGBAFormat, Scene, ShaderMaterial, ThreeOrthographicCamera, ThreePerspectiveCamera, useFrame, useThree, Vector3, WebGLRenderTarget } from "../../🔌️adapters/🟦️.ts";
import { type WorldProjectionMode, type WorldProjectionSpec } from "../../🔭️scene-types/🟦️.ts";
import { cadVec3ToThree, threeVec3ToCad, type WorldCameraState } from "../../🧷️r3f-foundation/🟦️.ts";
import { ORBIT_CAMERA_Z_UP, type OrbitControlsTarget, useWorldOrbitViewSnapGate, WORLD_ORBIT_CAMERA_MIN_FAR, WorldOrbitCameraViewRigSeed } from "../../📷️camera-view/🟦️.tsx";
import { computeWorldProjectionPose, WORLD_PROJECTION_DEFAULT_DISTANCE, worldProjectionFamily, worldProjectionPerspectiveFov } from "../🧮️projection-pose/🟦️.ts";
import { worldObliqueShearMatrix } from "../🔄️projection-matrices/🟦️.ts";

// #region 📐️WorldProjection
/** 📷️ Mounts the camera for a {@link WorldProjectionSpec} and the matrix/curvilinear post-processing it needs. */
export function WorldProjectionRig(props: { readonly spec: WorldProjectionSpec; readonly state: WorldCameraState; readonly seedKey: string | number; readonly onCamera?: (camera: Camera | null) => void; readonly pendingSpec?: WorldProjectionSpec | null }): ReactElement {
  const family = worldProjectionFamily(props.spec);
  const up = cadVec3ToThree(props.state.up ?? ORBIT_CAMERA_Z_UP);
  const cameraKey = `${props.spec.mode.kind}:${props.seedKey}`;
  const perspectiveFov = worldProjectionPerspectiveFov(props.spec);
  const curvilinearMode = props.spec.mode.kind === "curvilinear" ? props.spec.mode : props.pendingSpec?.mode.kind === "curvilinear" ? props.pendingSpec.mode : null;
  return (
    <>
      {family === "parallel" ? (
        <OrthographicCamera key={cameraKey} ref={props.onCamera} makeDefault up={up} near={0.2} far={WORLD_ORBIT_CAMERA_MIN_FAR} zoom={props.state.zoom} />
      ) : (
        <PerspectiveCamera key={cameraKey} ref={props.onCamera} makeDefault up={up} near={0.2} far={WORLD_ORBIT_CAMERA_MIN_FAR} fov={perspectiveFov} zoom={props.state.zoom} />
      )}
      <WorldOrbitCameraViewRigSeed state={props.state} seedKey={`${cameraKey}`} />
      {props.spec.mode.kind === "oblique" || props.spec.mode.kind === "twoPoint" ? <WorldProjectionMatrixDriver mode={props.spec.mode} /> : null}
      {curvilinearMode ? <WorldCurvilinearPass mode={curvilinearMode} /> : null}
    </>
  );
}

/** 📷️ Applies a projection preset when `seedKey` changes (owned-camera canvases) — mirrors {@link WorldOrbitCameraViewApplier}. */
export function WorldProjectionApplier(props: { readonly spec: WorldProjectionSpec; readonly seedKey: string | number }): ReactElement {
  const { camera } = useThree();
  const controls = useThree((s) => s.controls as OrbitControlsTarget | null);
  const targetScratch = reactHostPort.useMemo(() => new Vector3(), []);
  const state = reactHostPort.useMemo(() => {
    const target = controls?.target ?? targetScratch.set(0, 0, 0);
    const targetCad = threeVec3ToCad(target);
    const distance = camera ? Math.hypot(camera.position.x - target.x, camera.position.y - target.y, camera.position.z - target.z) || WORLD_PROJECTION_DEFAULT_DISTANCE : WORLD_PROJECTION_DEFAULT_DISTANCE;
    return computeWorldProjectionPose(props.spec, { target: targetCad, distance });
  }, [camera, controls, props.seedKey, props.spec, targetScratch]);
  return <WorldProjectionRig spec={props.spec} state={state} seedKey={props.seedKey} />;
}

/** 🪞️ Post-multiplies the oblique receding-axis shear or two-point vertical lens-shift onto the active
 * camera's projection matrix every frame (after recomputing the pristine matrix, so resize/zoom never compounds
 * the shear) and refreshes `projectionMatrixInverse` — required because r3f raycasting/picking reads the inverse.
 * Defers to {@link WorldProjectionSnapDriver} while a projection morph is in flight (`morphRef.current`), since the
 * morph's goal matrix already bakes in the destination shear/shift via {@link worldObliqueShearMatrix} — running both
 * would double-apply the transform. */
function WorldProjectionMatrixDriver(props: { readonly mode: Extract<WorldProjectionMode, { kind: "oblique" | "twoPoint" }> }): null {
  const { camera, invalidate } = useThree();
  const { morphRef } = useWorldOrbitViewSnapGate();
  useFrame(() => {
    if (morphRef.current) {
      return;
    }
    camera.updateProjectionMatrix();
    if (props.mode.kind === "oblique" && camera instanceof ThreeOrthographicCamera) {
      camera.projectionMatrix.multiply(worldObliqueShearMatrix(props.mode));
    } else if (props.mode.kind === "twoPoint" && camera instanceof ThreePerspectiveCamera) {
      camera.projectionMatrix.elements[9] += props.mode.verticalShift;
    }
    camera.projectionMatrixInverse.copy(camera.projectionMatrix).invert();
    invalidate();
  });
  return null;
}

/** 🐟️ Remaps a pointer NDC coordinate through the inverse curvilinear radius mapping so raycasting against
 * the exact (undistorted) capture-space render stays pixel-accurate under fisheye/panini distortion. */
export function worldCurvilinearUnproject(ndc: readonly [number, number], mode: Extract<WorldProjectionMode, { kind: "curvilinear" }>, aspect = 1): readonly [number, number] {
  const halfFov = (Math.min(mode.fov, 160) * (Math.PI / 180)) / 2;
  const scaled: [number, number] = [ndc[0] * aspect, ndc[1]];
  const r = Math.hypot(scaled[0], scaled[1]);
  if (r < 1e-6) {
    return ndc;
  }
  const theta = r * halfFov;
  const rectilinearR = Math.tan(theta) / Math.tan(halfFov);
  const sourceR = r + (rectilinearR - r) * mode.strength;
  const scale = sourceR / r;
  return [(scaled[0] * scale) / aspect, scaled[1] * scale];
}

const WORLD_CURVILINEAR_VERTEX_SHADER = `
  varying vec2 vUv;
  void main() {
    vUv = uv;
    gl_Position = vec4(position.xy, 0.0, 1.0);
  }
`;

/** 🐟️ Fisheye/panini remap blit — samples the linear capture and applies Three's output color-space convert. */
export const WORLD_CURVILINEAR_FRAGMENT_SHADER = `
  uniform sampler2D tCapture;
  uniform float uFov;
  uniform float uStrength;
  uniform float uAspect;
  uniform float uMapping; // 0 = fisheye, 1 = panini
  varying vec2 vUv;
  void main() {
    vec2 ndc = vUv * 2.0 - 1.0;
    vec2 scaled = vec2(ndc.x * uAspect, ndc.y);
    float halfFov = uFov * 0.5;
    float r = length(scaled);
    vec2 sourceNdc = ndc;
    if (r > 1e-5) {
      float theta = r * halfFov;
      float rectR = tan(theta) / tan(halfFov);
      float sourceR = mix(r, rectR, uStrength);
      float scale = sourceR / r;
      sourceNdc = vec2((scaled.x * scale) / uAspect, scaled.y * scale);
    }
    vec2 sourceUv = sourceNdc * 0.5 + 0.5;
    if (sourceUv.x < 0.0 || sourceUv.x > 1.0 || sourceUv.y < 0.0 || sourceUv.y > 1.0) {
      gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
      #include <colorspace_fragment>
      return;
    }
    gl_FragColor = texture2D(tCapture, sourceUv);
    #include <colorspace_fragment>
  }
`;

/** 🐟️ Capture-target defaults for the curvilinear blit: linear working-space storage, linear filtering
 * (fisheye UV warp must interpolate), half-float precision (same as pmndrs/drei `RenderCubeTexture`), sized by the
 * caller at drawing-buffer resolution. Not drei `<Fisheye>` — that component portals children into a cubemap sphere
 * and has no FOV/strength/panini; our taxonomy needs a sibling planar remap pass instead. */
export const WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS = {
  minFilter: LinearFilter,
  magFilter: LinearFilter,
  format: RGBAFormat,
  type: HalfFloatType,
  colorSpace: LinearSRGBColorSpace,
} as const;

/** 🐟️ Wide-FOV planar capture (capped at 160°, exact and material-agnostic below full 180°) remapped through
 * a fullscreen fisheye/panini shader — chosen over pmndrs/drei `<Fisheye>` / a 6-face cubemap unwrap because it needs
 * one extra render target instead of six, keeps orbit/gizmo chrome undistorted as siblings, supports strength+panini,
 * and the taxonomy never promises >=180° coverage. */
function WorldCurvilinearPass(props: { readonly mode: Extract<WorldProjectionMode, { kind: "curvilinear" }> }): null {
  const { gl, camera, scene, size, invalidate } = useThree();
  const { morphRef } = useWorldOrbitViewSnapGate();
  const targetRef = reactHostPort.useRef<InstanceType<typeof WebGLRenderTarget<InstanceType<typeof sceneHostPort.three.Texture>>> | null>(null);
  const quadRef = reactHostPort.useRef<{ readonly scene: InstanceType<typeof Scene>; readonly camera: InstanceType<typeof ThreeOrthographicCamera>; readonly material: InstanceType<typeof ShaderMaterial> } | null>(null);

  useFrame(() => {
    if (!(camera instanceof ThreePerspectiveCamera)) {
      return;
    }
    const pixelRatio = gl.getPixelRatio();
    const width = Math.max(1, Math.floor(size.width * pixelRatio));
    const height = Math.max(1, Math.floor(size.height * pixelRatio));
    if (!targetRef.current || targetRef.current.width !== width || targetRef.current.height !== height) {
      targetRef.current?.dispose();
      targetRef.current = new WebGLRenderTarget(width, height, { ...WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS });
    }
    if (!quadRef.current) {
      const material = new ShaderMaterial({
        vertexShader: WORLD_CURVILINEAR_VERTEX_SHADER,
        fragmentShader: WORLD_CURVILINEAR_FRAGMENT_SHADER,
        uniforms: { tCapture: { value: targetRef.current.texture }, uFov: { value: 1 }, uStrength: { value: 1 }, uAspect: { value: 1 }, uMapping: { value: 0 } },
        depthTest: false,
        depthWrite: false,
        toneMapped: false,
      });
      const quadScene = new Scene();
      quadScene.add(new Mesh(new PlaneGeometry(2, 2), material));
      const quadCamera = new ThreeOrthographicCamera(-1, 1, 1, -1, 0, 1);
      quadRef.current = { scene: quadScene, camera: quadCamera, material };
    }
    const target = targetRef.current;
    const { material } = quadRef.current;
    const morph = morphRef.current;
    const enteringCurvilinear = morph?.toSpec.mode.kind === "curvilinear";
    const leavingCurvilinear = morph?.fromSpec?.mode.kind === "curvilinear";
    let ramp = 1;
    let fov = props.mode.fov;
    if (morph && enteringCurvilinear && !leavingCurvilinear) {
      ramp = morph.eased;
      fov = morph.fromFov + (morph.toFov - morph.fromFov) * morph.eased;
    } else if (morph && leavingCurvilinear && !enteringCurvilinear) {
      ramp = 1 - morph.eased;
      fov = morph.fromFov + (morph.toFov - morph.fromFov) * morph.eased;
    } else if (morph && !enteringCurvilinear && !leavingCurvilinear) {
      ramp = 0;
    }
    const {tCapture,uFov,uStrength,uAspect,uMapping}=material.uniforms;
    if(!tCapture||!uFov||!uStrength||!uAspect||!uMapping)throw Error("Complete curvilinear shader uniforms required");
    tCapture.value = target.texture;
    uFov.value = Math.min(fov, 160) * (Math.PI / 180);
    uStrength.value = props.mode.strength * ramp;
    uAspect.value = size.width / Math.max(1, size.height);
    uMapping.value = props.mode.mapping === "panini" ? 1 : 0;

    gl.setRenderTarget(target);
    gl.render(scene, camera);
    gl.setRenderTarget(null);
    gl.render(quadRef.current.scene, quadRef.current.camera);
    invalidate();
  }, 1);

  reactHostPort.useEffect(() => {
    return () => {
      targetRef.current?.dispose();
      quadRef.current?.material.dispose();
    };
  }, []);
  return null;
}
// #endregion 📐️WorldProjection
