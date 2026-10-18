// #region 🧲️Header
// 💻️ framework/ui/elements/🎬️Scene/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as THREE from "three";
import type { ThreeEvent } from "@react-three/fiber";
import { GizmoHelper } from "@react-three/drei/core/GizmoHelper.js";
import { GizmoViewport } from "@react-three/drei/core/GizmoViewport.js";
import { Grid } from "@react-three/drei/core/Grid.js";
import { OrbitControls } from "@react-three/drei/core/OrbitControls.js";
// 🧱️core: sceneHostPort imported directly from 🫀️core/Ports, NOT via the barrel — this component calls
// sceneHostPort.drei.Line at module top level, which requires a non-circular import (see
// 🧱️elements/🔌️Ports/🟦️.tsx's header comment for why the barrel import caused a real bug).
import { HostThreeCanvas, reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { ActionDropdown, type ActionDropdownOption } from "../⚡️ActionGroup/🟦️.tsx";
import { surfaceClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { loadingBorderClass } from "../../🔨️modules/🌀️status-border-presentation/🟦️.ts";
import { useLabel } from "../🏷️Label/🟦️.tsx";
import { resolveSpatialAxisColors } from "@semio-tech/ui-styling";
import { CameraIcon, GripVerticalIcon } from "../🔣️Icons/🟦️.tsx";
import type { Camera } from "./📐️geometry/🟦️.ts";
import { type GizmoProps, SCENE_GIZMO_LABELS, type SceneGizmoSnapTarget, type SceneProjectionKind, resolveSceneGizmoSnapTarget, resolveSceneGizmoViewportPlacement } from "./🧭️gizmo-snap/🟦️.ts";
import { useThree } from "./🔌️three-hooks/🟦️.ts";
import { SceneFrameControl, readSceneGridColors } from "./🗿️objects/🟦️.tsx";
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