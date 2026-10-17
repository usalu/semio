/** 🌪️ World3d instance layers, vortex markers: the vortex material revision, the direction arrow layout and the markers that show every vortex in the scene. */
import { Quaternion, Vector3 } from "@semio-tech/ui-react";
import { type WorldVortexRecord } from "../../🔭️scene-schema/🟦️.ts";
import { type MeshStylePalette } from "../../🎨️mesh-render/🖌️mesh-style-palette/🟦️.ts";
import { resolveVortexPointerDownIntent, worldVortexHitProxy } from "../../🔗️instance-selection/🟦️.ts";

//#region 🔖️World3dHost
//#region WorldVortexMarkers
export function worldVortexMaterialRevision(selected?: boolean, hovered?: boolean): "selected" | "hovered" | "neutral" {
  return selected ? "selected" : hovered ? "hovered" : "neutral";
}

const WORLD_VORTEX_Y_AXIS = new Vector3(0, 1, 0);
const WORLD_VORTEX_DIRECTION_FALLBACK: readonly [number, number, number] = [0, 0, -1];

function worldVortexUnitDirection(direction?: readonly [number, number, number]): Vector3 {
  const vector = new Vector3(...(direction ?? WORLD_VORTEX_DIRECTION_FALLBACK));
  if (vector.lengthSq() < 1e-12) {
    return new Vector3(...WORLD_VORTEX_DIRECTION_FALLBACK);
  }
  return vector.normalize();
}

function worldVortexArrowLayout(
  position: readonly [number, number, number],
  direction: readonly [number, number, number] | undefined,
  radius: number,
  displayDirection: "outwards" | "inwards",
): {
  readonly pointRadius: number;
  readonly shaftRadius: number;
  readonly shaftLength: number;
  readonly headLength: number;
  readonly shaftCenter: [number, number, number];
  readonly headCenter: [number, number, number];
  readonly quaternion: Quaternion;
} {
  const dir = worldVortexUnitDirection(direction);
  const arrowLength = radius;
  const headLength = radius * 0.28;
  const shaftLength = Math.max(arrowLength - headLength, radius * 0.2);
  const shaftRadius = radius * 0.055;
  const pointRadius = radius * 0.18;
  const outward = displayDirection !== "inwards";
  const shaftCenter = new Vector3(...position).addScaledVector(dir, outward ? shaftLength * 0.5 : -(headLength + shaftLength * 0.5));
  const headCenter = new Vector3(...position).addScaledVector(dir, outward ? arrowLength - headLength * 0.5 : -headLength * 0.5);
  return {
    pointRadius,
    shaftRadius,
    shaftLength,
    headLength,
    shaftCenter: shaftCenter.toArray() as [number, number, number],
    headCenter: headCenter.toArray() as [number, number, number],
    quaternion: new Quaternion().setFromUnitVectors(WORLD_VORTEX_Y_AXIS, dir),
  };
}

export function WorldVortexMarkers({
  vortices,
  palette,
  brushMode,
  selectionMode,
  connectSourceFullId,
  onHover,
  onVortexSelect,
  onBrushPlace,
  onVortexPointerArm,
  onVortexPointerMove,
  onVortexPointerUp,
  onConnectDragHover,
  onConnectDragDrop,
}: {
  readonly vortices: readonly WorldVortexRecord[];
  readonly palette: MeshStylePalette;
  readonly brushMode: boolean;
  readonly selectionMode?: string;
  readonly connectSourceFullId?: string;
  readonly onHover: (fullId: string | null) => void;
  readonly onVortexSelect: (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => void;
  readonly onBrushPlace: () => void;
  readonly onVortexPointerArm: (args: {
    readonly fullId: string;
    readonly position: readonly [number, number, number];
    readonly clientX: number;
    readonly clientY: number;
    readonly event: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean };
  }) => void;
  readonly onVortexPointerMove: (fullId: string, clientX: number, clientY: number) => void;
  readonly onVortexPointerUp: (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => void;
  readonly onConnectDragHover: (position: readonly [number, number, number]) => void;
  readonly onConnectDragDrop: (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => void;
}) {
  if (!vortices.length) return null;
  return (
    <group>
      {vortices.map((vortex) => {
        const radius = vortex.radius ?? 0.36;
        const displayDirection = vortex.displayDirection ?? "outwards";
        const layout = worldVortexArrowLayout(vortex.position, vortex.direction, radius, displayDirection);
        const isConnectSource = connectSourceFullId === vortex.fullId;
        const style = vortex.selected ? palette.selected : vortex.hovered ? palette.hovered : null;
        const color = isConnectSource ? "#f59e0b" : (style?.meshColor ?? vortex.color ?? "#38bdf8");
        const materialKey = worldVortexMaterialRevision(vortex.selected, vortex.hovered);
        const materialProps = {
          color,
          emissive: style?.meshColor ?? "#000000",
          emissiveIntensity: style?.emissiveIntensity ?? 0,
          transparent: true,
          opacity: 0.88,
        };
        const pointerHandlers = {
          onPointerOver: (event: { stopPropagation: () => void }) => {
            event.stopPropagation();
            onHover(vortex.fullId);
            if (connectSourceFullId) onConnectDragHover(vortex.position);
          },
          onPointerOut: (event: { stopPropagation: () => void }) => {
            event.stopPropagation();
            onHover(null);
          },
          onPointerDown: (event: { stopPropagation: () => void; button?: number; clientX: number; clientY: number; shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
            // 🖱️ Ignore right/middle — otherwise opening a context/suggestion menu arms connect-drag
            // and the portaled menu click never delivers host pointer-up to cancel it.
            if (event.button != null && event.button !== 0) return;
            event.stopPropagation();
            if (resolveVortexPointerDownIntent(brushMode, selectionMode) === "select") {
              onVortexSelect(vortex.fullId, event);
              if (brushMode) onBrushPlace();
              return;
            }
            onVortexPointerArm({
              fullId: vortex.fullId,
              position: vortex.position,
              clientX: event.clientX,
              clientY: event.clientY,
              event,
            });
          },
          onPointerMove: (event: { stopPropagation: () => void; clientX: number; clientY: number }) => {
            if (brushMode) return;
            event.stopPropagation();
            onVortexPointerMove(vortex.fullId, event.clientX, event.clientY);
            if (connectSourceFullId) onConnectDragHover(vortex.position);
          },
          onPointerUp: (event: { stopPropagation: () => void; shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
            if (brushMode) return;
            if (connectSourceFullId) {
              event.stopPropagation();
              onConnectDragDrop(vortex.fullId, event);
              return;
            }
            event.stopPropagation();
            onVortexPointerUp(vortex.fullId, event);
          },
          onClick: (event: { stopPropagation: () => void }) => {
            event.stopPropagation();
            if (brushMode) onBrushPlace();
          },
        };
        return (
          <group key={vortex.fullId}>
            <mesh position={vortex.position as [number, number, number]} visible={worldVortexHitProxy(radius).visible} {...pointerHandlers}>
              <sphereGeometry args={[worldVortexHitProxy(radius).radius, 16, 16]} />
              <meshBasicMaterial transparent opacity={0} depthWrite={false} depthTest={false} />
            </mesh>
            <mesh position={vortex.position as [number, number, number]}>
              <sphereGeometry args={[layout.pointRadius, 12, 12]} />
              <meshStandardMaterial key={materialKey} {...materialProps} />
            </mesh>
            <mesh position={layout.shaftCenter} quaternion={layout.quaternion}>
              <cylinderGeometry args={[layout.shaftRadius, layout.shaftRadius, layout.shaftLength, 10]} />
              <meshStandardMaterial key={materialKey} {...materialProps} />
            </mesh>
            <mesh position={layout.headCenter} quaternion={layout.quaternion}>
              <coneGeometry args={[layout.shaftRadius * 1.8, layout.headLength, 12]} />
              <meshStandardMaterial key={materialKey} {...materialProps} />
            </mesh>
          </group>
        );
      })}
    </group>
  );
}
//#endregion WorldVortexMarkers
//#endregion 🔖️World3dHost
