/** 🧊️ World volumes: props, pose and transform application, containment tests, the gumball, the volume box item and the volume layer.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | r3f}, which re-exports every public name below so import specifiers stay unchanged.
 */

import {
  reactHostPort,
  UnifiedGumball,
  gumballConfigVisible,
  gumballHandleKindToTransformMode,
  type GumballConfig,
  type GumballPose
} from "@semio-tech/ui-react";
import { type ReactElement } from "react";
import {
  BoxGeometry,
  DoubleSide,
  EdgesGeometry,
  Group,
  Matrix4,
  Vector3
} from "../🔌️adapters/🟦️.ts";
import {
  cadVec3ToThree,
  type Quat,
  type Vec3,
  WORLD_LOCKED_OPACITY_SCALE,
  type WorldEntityFlags,
  worldEntityRenderMode,
  worldEntitySelectable,
  worldRaycastNone
} from "../🧷️r3f-foundation/🟦️.ts";

// #region 🧊️Volume
/** 🧊️ Serializable oriented target volume box for fill constraints. */
export interface WorldVolumeProps extends WorldEntityFlags {
  readonly id: string;
  readonly origin: Vec3;
  readonly orientation?: Quat;
  readonly scale?: number | Vec3;
  readonly color?: string;
  readonly opacity?: number;
  readonly relocate?: GumballConfig | false;
  readonly relocateActive?: boolean;
}

/** 🧊️ Gumball relocate commit for a world target volume. */
export interface WorldVolumeRelocatePayload {
  readonly volumeId: string;
  readonly mode: "translate" | "rotate" | "scale";
  readonly before: GumballPose;
  readonly after: GumballPose;
}

export const WORLD_VOLUME_DEFAULT_COLOR = "#22c55e";
export const WORLD_VOLUME_DEFAULT_OPACITY = 0.22;

function worldVolumeScaleVec(scale: number | Vec3 | undefined): Vec3 {
  if (typeof scale === "number") {
    return [scale, scale, scale];
  }
  if (scale) {
    return [scale[0], scale[1], scale[2]];
  }
  return [1, 1, 1];
}

/** 🧊️ Applies CAD pose fields onto a volume group node. */
export function applyWorldVolumePose(group: Group, volume: Pick<WorldVolumeProps, "origin" | "orientation" | "scale">): void {
  const position = cadVec3ToThree(volume.origin);
  group.position.set(position[0], position[1], position[2]);
  const quat = volume.orientation ?? ([0, 0, 0, 1] as Quat);
  group.quaternion.set(quat[0], quat[1], quat[2], quat[3]);
  const scale = worldVolumeScaleVec(volume.scale);
  group.scale.set(scale[0], scale[1], scale[2]);
}

/** 🧊️ Writes a gumball pose back onto persisted volume props. */
export function applyWorldVolumeTransform(volume: WorldVolumeProps, after: GumballPose): WorldVolumeProps {
  return {
    ...volume,
    origin: [after.position[0], after.position[1], after.position[2]],
    orientation: [after.quaternion[0], after.quaternion[1], after.quaternion[2], after.quaternion[3]],
    scale: [after.scale[0], after.scale[1], after.scale[2]],
  };
}

const _worldVolumeMatrix = new Matrix4();
const _worldVolumeInverse = new Matrix4();
const _worldVolumeCorner = new Vector3();
const _worldVolumeLocal = new Vector3();

/** 🧊️ True when every corner of a world AABB lies inside any oriented volume (union). */
export function worldVolumesContainAabb(volumes: readonly WorldVolumeProps[], aabbMin: Vec3, aabbMax: Vec3, epsilon = 1e-3): boolean {
  if (!volumes.length) {
    return true;
  }
  const corners: Vec3[] = [
    [aabbMin[0], aabbMin[1], aabbMin[2]],
    [aabbMax[0], aabbMin[1], aabbMin[2]],
    [aabbMin[0], aabbMax[1], aabbMin[2]],
    [aabbMax[0], aabbMax[1], aabbMin[2]],
    [aabbMin[0], aabbMin[1], aabbMax[2]],
    [aabbMax[0], aabbMin[1], aabbMax[2]],
    [aabbMin[0], aabbMax[1], aabbMax[2]],
    [aabbMax[0], aabbMax[1], aabbMax[2]],
  ];
  for (const volume of volumes) {
    const group = new Group();
    applyWorldVolumePose(group, volume);
    group.updateMatrixWorld(true);
    _worldVolumeInverse.copy(group.matrixWorld).invert();
    const hx = 0.5 + epsilon;
    const hy = 0.5 + epsilon;
    const hz = 0.5 + epsilon;
    let inside = true;
    for (const corner of corners) {
      _worldVolumeCorner.set(corner[0], corner[1], corner[2]);
      _worldVolumeLocal.copy(_worldVolumeCorner).applyMatrix4(_worldVolumeInverse);
      if (Math.abs(_worldVolumeLocal.x) > hx || Math.abs(_worldVolumeLocal.y) > hy || Math.abs(_worldVolumeLocal.z) > hz) {
        inside = false;
        break;
      }
    }
    if (inside) {
      return true;
    }
  }
  return false;
}

const WorldVolumeGumball = reactHostPort.memo(function WorldVolumeGumball(props: { readonly volumeId: string; readonly target: Group; readonly config: GumballConfig; readonly onRelocate?: (payload: WorldVolumeRelocatePayload) => void }) {
  const beforeRef = reactHostPort.useRef<GumballPose | null>(null);
  return (
    <UnifiedGumball
      target={props.target}
      config={props.config}
      onDragStart={(_kind, before) => {
        beforeRef.current = before;
      }}
      onDragEnd={(kind, before, after) => {
        props.onRelocate?.({
          volumeId: props.volumeId,
          mode: gumballHandleKindToTransformMode(kind),
          before: beforeRef.current ?? before,
          after,
        });
        beforeRef.current = null;
      }}
    />
  );
});

const _worldVolumeEdgeGeo = new EdgesGeometry(new BoxGeometry(1, 1, 1));

const WorldVolumeBoxItem = reactHostPort.memo(function WorldVolumeBoxItem(props: {
  readonly volume: WorldVolumeProps;
  readonly selected: boolean;
  readonly hovered: boolean;
  readonly revealed: boolean;
  readonly interactive?: boolean;
  readonly gumballConfig?: GumballConfig;
  readonly relocateActive?: boolean;
  readonly translationSnap?: number;
  readonly onSelect?: (id: string, modifiers: { readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean }) => void;
  readonly onHover?: (id: string | null) => void;
  readonly onRelocate?: (payload: WorldVolumeRelocatePayload) => void;
}) {
  const groupRef = reactHostPort.useRef<Group>(null);
  const renderMode = worldEntityRenderMode(props.volume, { hovered: props.hovered, selected: props.selected, revealed: props.revealed });
  const selectable = props.interactive !== false && worldEntitySelectable(props.volume);
  reactHostPort.useLayoutEffect(() => {
    if (groupRef.current) {
      applyWorldVolumePose(groupRef.current, props.volume);
    }
  }, [props.volume.origin, props.volume.orientation, props.volume.scale]);
  if (!renderMode.visible) {
    return null;
  }
  const opacityBase = props.volume.opacity ?? WORLD_VOLUME_DEFAULT_OPACITY;
  const opacity = renderMode.dim ? opacityBase * WORLD_LOCKED_OPACITY_SCALE : opacityBase;
  const color = props.volume.color ?? WORLD_VOLUME_DEFAULT_COLOR;
  const config = props.volume.relocate === false ? null : { ...(props.volume.relocate ?? {}), translationSnap: props.translationSnap };
  const showGumball = props.interactive !== false && props.selected && selectable && props.relocateActive !== false && groupRef.current && config && gumballConfigVisible(config);
  return (
    <>
      <group
        ref={groupRef}
        visible={renderMode.visible}
        userData={{ worldVolumeId: props.volume.id }}
        onPointerDown={(event) => {
          if (!selectable) {
            return;
          }
          event.stopPropagation();
          props.onSelect?.(props.volume.id, { shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
        }}
        onPointerOver={(event) => {
          if (!selectable) {
            return;
          }
          event.stopPropagation();
          props.onHover?.(props.volume.id);
        }}
        onPointerOut={(event) => {
          if (!selectable) {
            return;
          }
          event.stopPropagation();
          props.onHover?.(null);
        }}
      >
        <mesh raycast={selectable ? undefined : worldRaycastNone}>
          <boxGeometry args={[1, 1, 1]} />
          <meshStandardMaterial color={color} transparent opacity={opacity} depthWrite={false} side={DoubleSide} />
        </mesh>
        <lineSegments raycast={worldRaycastNone} geometry={_worldVolumeEdgeGeo}>
          <lineBasicMaterial color={color} transparent opacity={Math.min(1, opacity + 0.35)} depthWrite={false} />
        </lineSegments>
      </group>
      {showGumball && groupRef.current ? <WorldVolumeGumball volumeId={props.volume.id} target={groupRef.current} config={config} onRelocate={props.onRelocate} /> : null}
    </>
  );
});

/** 🧊️ Renders persisted target volume boxes for puzzle 3d fill constraints. */
export function WorldVolumeLayer(props: {
  readonly volumes: readonly WorldVolumeProps[];
  readonly selectedIds?: ReadonlySet<string>;
  readonly hoveredId?: string | null;
  readonly revealedIds?: ReadonlySet<string>;
  readonly interactive?: boolean;
  readonly gumballConfig?: GumballConfig;
  readonly relocateActive?: boolean;
  readonly translationSnap?: number;
  readonly onSelect?: (id: string, modifiers: { readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean }) => void;
  readonly onHover?: (id: string | null) => void;
  readonly onRelocate?: (payload: WorldVolumeRelocatePayload) => void;
}): ReactElement | null {
  if (!props.volumes.length) {
    return null;
  }
  const selected = props.selectedIds ?? new Set<string>();
  const revealed = props.revealedIds ?? new Set<string>();
  return (
    <>
      {props.volumes.map((volume) => (
        <WorldVolumeBoxItem
          key={volume.id}
          volume={volume}
          selected={selected.has(volume.id)}
          hovered={props.hoveredId === volume.id}
          revealed={revealed.has(volume.id)}
          interactive={props.interactive}
          gumballConfig={props.gumballConfig}
          relocateActive={props.relocateActive}
          translationSnap={props.translationSnap}
          onSelect={props.onSelect}
          onHover={props.onHover}
          onRelocate={props.onRelocate}
        />
      ))}
    </>
  );
}
// #endregion 🧊️Volume
