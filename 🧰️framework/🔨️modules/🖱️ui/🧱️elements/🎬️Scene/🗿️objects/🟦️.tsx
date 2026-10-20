import * as React from "react";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { resolveSemanticColorHex, resolveColorHex, themeColorVar } from "@semio-tech/ui-styling";
import { ephemeralBox } from "@semio-tech/framework-transient";
import type { Plane, Point, Vector } from "../📐️geometry/🟦️.ts";
import * as THREE from "three";
import type { ThreeEvent } from "@react-three/fiber";
import { Edges } from "@react-three/drei/core/Edges.js";
import { useGLTF } from "@react-three/drei/core/Gltf.js";
import { useThree } from "../🔌️three-hooks/🟦️.ts";

// 3D scene viewer built on React Three Fiber.
// Consumers MUST provide SceneGeometry data.

export const sceneFrameControlRef: { current: { pause: () => void; resume: () => void } | null } = { current: null };
export const SceneFrameControl: React.FC = () => {
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

/** 📐️ Scene floor grid — element gray strokes, not emphasized foreground. */
export const readSceneGridColors = (): { sectionColor: string; cellColor: string } => ({
  sectionColor: resolveColorHex(themeColorVar("element"), "gray"),
  cellColor: resolveColorHex(themeColorVar("muted-foreground"), "gray"),
});

/**
 * 🖱️ selectableCursorUsageCount.current holds the data fields for a selectableCursorUsageCount.current record.
 **/
const selectableCursorUsageCount = ephemeralBox("framework.modules.ui.elements.Scene.component.tsx.selectableCursorUsageCount", 0);

/**
 * 🎬️ Interface for a geometry entry in a 3D scene.
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
 * 🧊️ Extended SceneGeometry with transform delta support.
 **/
export interface TransformableGeometry extends SceneGeometry {
  isTransformable?: boolean;
}

/**
 * 🔲️ Interface for an incremental plane transformation delta.
 **/
export interface PlaneTransformDelta {
  translation?: { x: number; y: number; z: number };
  rotation?: { x: number; y: number; z: number; w: number };
  scale?: number;
}

/**
 * 🟦️ Callback type for a single plane update.
 **/
export type OnPlaneUpdate = (geometryGuid: string, newPlane: Plane) => void;

/**
 * 📄️ Callback type for batch plane updates.
 **/
export type OnMultiPlaneUpdate = (updates: Array<{ geometryGuid: string; newPlane: Plane }>) => void;

/**
 * 🪟️ Constructs a Plane from a point and direction vector.
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
 * 📍️ Extracts the THREE.Vector3 position from a Plane.
 **/
export const getPlanePosition = (plane: Plane): THREE.Vector3 => {
  return new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z);
};

/**
 * ✅️ Checks whether a geometry has a non-null plane.
 **/
export const hasValidPlane = (geometry: SceneGeometry): boolean => {
  return geometry.plane !== undefined && geometry.plane !== null;
};

/**
 * 🔷️ Checks whether a geometry has a valid plane for camera focus.
 **/
export const isGeometryFocusable = (geometry: SceneGeometry): boolean => {
  return hasValidPlane(geometry) && (geometry.isFocusable === undefined || geometry.isFocusable === true);
};

/**
 * 🧱️ GeometryProps holds the data fields for a GeometryProps record.
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
 * 🗿️ 3D geometry mesh component with selection, hover, and edge rendering.
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
    selectableCursorUsageCount.current += 1;
    document.body.classList.add("cursor-selectable");
    return () => {
      selectableCursorUsageCount.current = Math.max(0, selectableCursorUsageCount.current - 1);
      if (selectableCursorUsageCount.current === 0) {
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
 * 🕸️ GltfProps holds the data fields for a GltfProps record.
 **/
interface GltfProps {
  src: string;
  roughness?: number;
  metalness?: number;
}

const getComputedColorForGltf = (variable: string): string => resolveSemanticColorHex(variable, "gray");

/**
 * 🔺️ Gltf holds the data fields for a Gltf record.
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
 * 📦️ GeometryFileProps holds the data fields for a GeometryFileProps record.
 **/
interface GeometryFileProps {
  src: string;
  environment?: string;
  roughness?: number;
  metalness?: number;
}
/** 📃️ GeometryFile holds the data fields for a GeometryFile record.
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
