/** 🎨️ The mesh render components of the world: the auto-fit, the accepted-frame diagnostics, the projection content frame, the celebrating conic material and the paint textured mesh, with the r3f hooks the world hosts share.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | World3dHost}; the mesh style palette, the instance celebration store, the mesh parsing and residency, the mesh geometry and appearance and the GLB bounds and styling live in its child folders, and every importer names the owner of the item it uses.
 */

import React, { useCallback, useEffect, useLayoutEffect, useMemo, useRef, useSyncExternalStore, type ComponentProps } from "react";
import { clearColorResolveCache, resolveColorHex, tokenVar } from "@semio-tech/ui-styling";

// #region 🔌️Adapters
import { sceneHostPort, Box3, BufferGeometry, DoubleSide, Group, OrthographicCamera, ShaderMaterial, TextureLoader, Vector3 } from "@semio-tech/ui-react";
import { DEFAULT_LOD_GRID_FACTOR, DEFAULT_MANUAL_LOD, frameWorldProjectionPose, lodFromCameraDistance, lodGridStepWorld, lodOrbitDistanceForCamera, WORLD_REFERENCE_DEFAULT_WIDTH, worldProjectionModeFov, worldSceneContentBoundsKey, type WorldCameraState, type WorldProjectionSpec, type WorldSceneContentBounds } from "@semio-tech/framework/infinite/world/r3f";

// #endregion 🔌️Adapters
import { runtimeDiagnosticsEnabled } from "../../🏛️ShellHost/🔌️extension-runtime/🟦️.ts";
import { fitCameraFromBounds, world3dAutoFitOwed, world3dBoundsRadius, type WorldLodRecord, type WorldParsedCameraState, type WorldReferenceRecord } from "../🔭️scene-schema/🟦️.ts";
import { type MeshStyleColors, type MeshStyleKind, worldMeshMaterialRevision } from "./🖌️mesh-style-palette/🟦️.ts";
import { CELEBRATE_CONIC_FRAGMENT_SHADER, CELEBRATE_CONIC_VERTEX_SHADER, celebrateConicAngleRadians, hexToRgb01 } from "./🎊️instance-celebration/🟦️.ts";
import type { WorldMeshAppearance } from "./🔺️mesh-geometry-and-appearance/🟦️.ts";
import { paintTextureUrl, WORLD_HEATMAP_TINT_SHARE, WORLD_SOLID_MESH_USER_DATA } from "./🪟️glb-bounds-and-styling/🟦️.ts";

export const { useFrame, useLoader, useThree } = sceneHostPort.fiber;

//#region 🔖️World3dHost
//#region WorldSceneParsing
//#region WorldMeshPaint
//#region 🎉️WorldInstanceCelebrate
/** 🎉️ Spinning primary/secondary/tertiary conic fill — the 3D counterpart of `[data-celebrated="true"]`'s CSS ring. */
function CelebratingConicMaterial({ opacity = 1 }: { readonly opacity?: number }) {
  const invalidate = useThree((state) => state.invalidate);
  const colors = useMemo(() => {
    clearColorResolveCache();
    return {
      a: hexToRgb01(resolveColorHex(tokenVar("primary"))),
      b: hexToRgb01(resolveColorHex(tokenVar("secondary"))),
      c: hexToRgb01(resolveColorHex(tokenVar("tertiary"))),
    };
  }, []);
  const material = useMemo(
    () =>
      new ShaderMaterial({
        vertexShader: CELEBRATE_CONIC_VERTEX_SHADER,
        fragmentShader: CELEBRATE_CONIC_FRAGMENT_SHADER,
        uniforms: {
          uColorA: { value: colors.a },
          uColorB: { value: colors.b },
          uColorC: { value: colors.c },
          uAngle: { value: 0 },
          uOpacity: { value: opacity },
        },
        transparent: opacity < 1,
        side: DoubleSide,
        depthWrite: opacity >= 1,
      }),
    [colors, opacity],
  );
  useEffect(() => () => material.dispose(), [material]);
  useFrame(() => {
    const angleUniform=material.uniforms.uAngle;if(!angleUniform)throw Error("Owned rotating texture uniform required");
    angleUniform.value = celebrateConicAngleRadians();
    invalidate();
  });
  return <primitive object={material} attach="material" />;
}

//#endregion 🎉️WorldInstanceCelebrate
//#endregion WorldMeshPaint
/** 🎯️ Fits the orbit camera to the producer's published bounds (or, absent those, the scene
 * group's own AABB) once per fit key, preserving the view direction. */
export function WorldAutoFit({
  groupRef,
  fitKey,
  padding,
  camera,
  bounds,
  userMoved,
  onFitted,
}: {
  readonly groupRef: React.RefObject<Group | null>;
  readonly fitKey: string;
  readonly padding: number;
  readonly camera: WorldParsedCameraState;
  readonly bounds: readonly [readonly number[], readonly number[]] | null;
  readonly userMoved: boolean;
  readonly onFitted: (state: WorldCameraState) => void;
}): null {
  const { camera: sceneCamera, controls, invalidate } = useThree();
  const appliedKeyRef = useRef("");
  const targetScratch = useMemo(() => new Vector3(), []);
  useFrame(() => {
    if (!sceneCamera) return;
    const group = groupRef.current;
    if (!group) return;
    if (!world3dAutoFitOwed(appliedKeyRef.current, fitKey, userMoved)) return;
    let center: readonly [number, number, number];
    let radius: number;
    if (bounds) {
      const [minimum, maximum] = bounds;
      center = [((minimum[0] ?? 0) + (maximum[0] ?? 0)) * 0.5, ((minimum[1] ?? 0) + (maximum[1] ?? 0)) * 0.5, ((minimum[2] ?? 0) + (maximum[2] ?? 0)) * 0.5];
      radius = world3dBoundsRadius(minimum, maximum);
    } else {
      const box = new Box3().setFromObject(group);
      if (box.isEmpty()) return;
      const boxCenter = box.getCenter(new Vector3());
      const boxMinimum = box.min;
      const boxMaximum = box.max;
      center = [boxCenter.x, boxCenter.y, boxCenter.z];
      radius = world3dBoundsRadius([boxMinimum.x, boxMinimum.y, boxMinimum.z], [boxMaximum.x, boxMaximum.y, boxMaximum.z]);
    }
    if (radius <= 0) return;
    appliedKeyRef.current = fitKey;
    const aspect = (sceneCamera as { aspect?: number }).aspect ?? 1;
    const fitted = fitCameraFromBounds(center, radius, camera, padding, aspect);
    const orbit = controls as { target: Vector3; update?: () => void } | null;
    const target = orbit?.target ?? targetScratch;
    target.set(fitted.target[0], fitted.target[1], fitted.target[2]);
    sceneCamera.position.set(fitted.position[0], fitted.position[1], fitted.position[2]);
    if ("zoom" in sceneCamera) sceneCamera.zoom = fitted.zoom;
    sceneCamera.updateProjectionMatrix();
    if (orbit) orbit.update?.();
    else sceneCamera.lookAt(target);
    invalidate();
    onFitted({ ...camera, position: fitted.position, target: fitted.target, zoom: fitted.zoom });
  });
  return null;
}

type WorldAcceptedFrameDiagnosticsProps = {
  readonly surfaceId: string;
  readonly windowInstanceId?: string | null;
  readonly projection: WorldProjectionSpec;
  readonly cameraTarget: readonly [number, number, number];
  readonly contentBounds: WorldSceneContentBounds | null;
  readonly references: readonly WorldReferenceRecord[];
  readonly lod: WorldLodRecord;
};

/** 🩺️ Emits one deduplicated, diagnostics-gated receipt from the actual R3F camera after
 * projection/framing and LOD frame runners have accepted their inputs. */
export function WorldAcceptedFrameDiagnostics(props: WorldAcceptedFrameDiagnosticsProps): null {
  const { camera, controls, size } = useThree();
  const fallbackTarget = useMemo(() => new Vector3(), []);
  const lastReceiptRef = useRef("");
  useFrame(() => {
    if (!runtimeDiagnosticsEnabled()) return;
    const target = (controls as { readonly target?: Vector3 } | null)?.target ?? fallbackTarget.set(...props.cameraTarget);
    const orbitDistance = camera.position.distanceTo(target);
    const lodDistance = lodOrbitDistanceForCamera(camera, orbitDistance, size.height);
    const automaticLod = props.lod.automaticLod ?? true;
    const depthVariableLod = props.lod.depthVariableLod ?? false;
    const manualLod = props.lod.manualLod ?? DEFAULT_MANUAL_LOD;
    const sceneLod = automaticLod || depthVariableLod ? lodFromCameraDistance(lodDistance, 100) : manualLod;
    const gridFactor = props.lod.gridFactor ?? DEFAULT_LOD_GRID_FACTOR;
    const receipt = {
      surfaceId: props.surfaceId,
      windowInstanceId: props.windowInstanceId ?? null,
      projection: props.projection,
      camera: {
        kind: camera.type,
        position: camera.position.toArray(),
        target: target.toArray(),
        up: camera.up.toArray(),
        quaternion: camera.quaternion.toArray(),
        fov: "fov" in camera && typeof camera.fov === "number" ? camera.fov : null,
        zoom: "zoom" in camera && typeof camera.zoom === "number" ? camera.zoom : 1,
        near: "near" in camera && typeof camera.near === "number" ? camera.near : null,
        far: "far" in camera && typeof camera.far === "number" ? camera.far : null,
      },
      viewport: { width: size.width, height: size.height, aspect: size.height > 0 ? size.width / size.height : null },
      contentBounds: props.contentBounds,
      references: props.references.filter((reference) => !reference.hidden).map((reference) => ({ id: reference.id, origin: reference.origin, widthWorld: reference.widthWorld ?? WORLD_REFERENCE_DEFAULT_WIDTH })),
      grid: { automaticLod, depthVariableLod, distanceReference: 100, orbitDistance, lodDistance, sceneLod, gridFactor, gridStepWorld: lodGridStepWorld(sceneLod, gridFactor) },
    };
    const encoded = JSON.stringify(receipt);
    if (encoded === lastReceiptRef.current) return;
    lastReceiptRef.current = encoded;
    console.info(`[TRACE] react-world-frame ${encoded}`);
  });
  return null;
}

/** 📷️ Viewport-aware reframe for a projection seed so orthographic panes fit content — re-runs whenever
 * {@link worldSceneContentBoundsKey} changes (a running tool grows the scene) until the host stops enabling it
 * (user-owned orbit/pan/zoom). */
export function WorldProjectionContentFrame(props: {
  readonly enabled: boolean;
  readonly spec: WorldProjectionSpec | undefined;
  readonly bounds: WorldSceneContentBounds | null;
  readonly fov: number;
  readonly onFramed: (state: WorldParsedCameraState) => void;
}): null {
  const size = useThree((state) => state.size);
  const getThree = useThree((state) => state.get);
  const invalidate = useThree((state) => state.invalidate);
  const framedBoundsKeyRef = useRef<string | null>(null);
  const onFramedRef = useRef(props.onFramed);
  onFramedRef.current = props.onFramed;
  const boundsKey = worldSceneContentBoundsKey(props.bounds);
  useLayoutEffect(() => {
    if (!props.enabled || !props.spec || !props.bounds || !boundsKey) return;
    if (size.width < 1 || size.height < 1) return;
    if (framedBoundsKeyRef.current === boundsKey) return;
    framedBoundsKeyRef.current = boundsKey;
    const { camera, controls: rawControls } = getThree();
    const controls = rawControls as { target: Vector3; update?: () => void } | null;
    const pose = frameWorldProjectionPose(props.spec, props.bounds, { viewportWidth: size.width, viewportHeight: size.height });
    const framed: WorldParsedCameraState = { ...pose, fov: worldProjectionModeFov(props.spec) ?? props.fov, explicitProjection: true, projectionFrame: "content" };
    const ortho = camera as OrthographicCamera & { readonly isOrthographicCamera?: boolean };
    if (ortho.isOrthographicCamera) {
      ortho.left = size.width / -2;
      ortho.right = size.width / 2;
      ortho.top = size.height / 2;
      ortho.bottom = size.height / -2;
      ortho.zoom = framed.zoom;
      ortho.position.set(framed.position[0], framed.position[1], framed.position[2]);
      ortho.up.set(framed.up?.[0] ?? 0, framed.up?.[1] ?? 1, framed.up?.[2] ?? 0);
      const target = controls?.target;
      if (target) {
        target.set(framed.target[0], framed.target[1], framed.target[2]);
        controls?.update?.();
      } else {
        ortho.lookAt(framed.target[0], framed.target[1], framed.target[2]);
      }
      ortho.updateProjectionMatrix();
    } else if (camera) {
      camera.position.set(framed.position[0], framed.position[1], framed.position[2]);
      camera.up.set(framed.up?.[0] ?? 0, framed.up?.[1] ?? 1, framed.up?.[2] ?? 0);
      const target = controls?.target;
      if (target) {
        target.set(framed.target[0], framed.target[1], framed.target[2]);
        controls?.update?.();
      } else {
        camera.lookAt(framed.target[0], framed.target[1], framed.target[2]);
      }
      if ("zoom" in camera) (camera as OrthographicCamera).zoom = framed.zoom;
      if ("updateProjectionMatrix" in camera) (camera as OrthographicCamera).updateProjectionMatrix();
    }
    invalidate();
    onFramedRef.current(framed);
  }, [boundsKey, getThree, invalidate, props.bounds, props.enabled, props.fov, props.spec, size.height, size.width]);
  return null;
}

export function PaintTexturedMesh({
  geometry,
  style,
  styleKind,
  textureBase64,
  appearance,
  flatShading,
  children,
  ...meshProps
}: {
  readonly geometry: BufferGeometry;
  readonly style: MeshStyleColors;
  readonly styleKind: MeshStyleKind;
  readonly textureBase64?: string;
  readonly appearance?: WorldMeshAppearance | null;
  readonly flatShading?: boolean;
  readonly children?: React.ReactNode;
} & ComponentProps<"mesh">) {
  const paintMap = textureBase64 ? useLoader(TextureLoader, paintTextureUrl(textureBase64)) : null;
  const invalidate=useThree(state=>state.invalidate);
  useSyncExternalStore(useCallback(listener=>{appearance?.listeners.add(listener);return ()=>appearance?.listeners.delete(listener);},[appearance]),()=>`${appearance?.pending ?? 0}:${appearance?.fault ?? ""}:${appearance?.closed ?? false}`,()=>"0::false");
  useEffect(()=>{if(!appearance)return;const listener=()=>invalidate();appearance.listeners.add(listener);return ()=>{appearance.listeners.delete(listener);};},[appearance,invalidate]);
  const authored=appearance && !appearance.closed && (styleKind === "neutral" || styleKind === "disabled");
  const hasVertexColors = geometry.hasAttribute("color");
  const heatmap = geometry.userData.world3dHeatmap === true && styleKind !== "provisional" && styleKind !== "celebrated";
  const preserveVertexColors = hasVertexColors && (styleKind === "neutral" || styleKind === "disabled" || heatmap);
  const heatmapTint = preserveVertexColors && styleKind !== "neutral" && styleKind !== "disabled";
  const celebrating = styleKind === "celebrated" && !preserveVertexColors;
  return (
    <mesh geometry={geometry} userData={WORLD_SOLID_MESH_USER_DATA} {...meshProps}>
      {authored ? <primitive object={appearance.materials} attach="material" dispose={null} /> : celebrating ? (
        <CelebratingConicMaterial opacity={style.opacity} />
      ) : (
        <meshStandardMaterial
          key={worldMeshMaterialRevision(styleKind)}
          color={preserveVertexColors ? "#ffffff" : style.meshColor}
          vertexColors={preserveVertexColors}
          map={paintMap ?? undefined}
          side={DoubleSide}
          flatShading={flatShading}
          metalness={0}
          roughness={1}
          emissive={preserveVertexColors && !heatmapTint ? "#000000" : style.meshColor}
          emissiveIntensity={preserveVertexColors && !heatmapTint ? 0 : style.emissiveIntensity * (heatmapTint ? WORLD_HEATMAP_TINT_SHARE : 1)}
          transparent={style.opacity < 1}
          opacity={style.opacity}
        />
      )}
      {children}
    </mesh>
  );
}
//#endregion WorldSceneParsing
//#endregion 🔖️World3dHost
