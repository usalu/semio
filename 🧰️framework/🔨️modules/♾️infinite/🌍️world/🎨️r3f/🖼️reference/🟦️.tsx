/** 🖼️ World reference planes: props, appearance, pose application, euler and quaternion conversion, patching, the gumball and the reference layer.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | r3f}, which re-exports every public name below so import specifiers stay unchanged.
 */

import {
  reactHostPort,
  referenceMediaPort,
  UnifiedGumball,
  gumballConfigVisible,
  gumballHandleKindToTransformMode,
  type GumballConfig,
  type GumballPose,
  type ThreeTexture
} from "@semio-tech/ui-react";
import { resolveColorHex, semanticVar, tokenVar } from "@semio-tech/ui-styling";
import { type ReactElement } from "react";
import {
  DoubleSide,
  EdgesGeometry,
  Group,
  PlaneGeometry,
  useThree
} from "../🔌️adapters/🟦️.ts";
import {
  cadVec3ToThree,
  type Quat,
  type Vec3,
  WORLD_LOCKED_OPACITY_SCALE,
  type WorldEntityFlags,
  worldEntityInspectable,
  worldEntityRenderMode,
  type WorldEntityRenderMode,
  worldEntitySelectable,
  worldRaycastNone
} from "../🧷️r3f-foundation/🟦️.ts";

// #region 📐️WorldProjection
/** 🖼️ Authored width for a reference plane that omits `widthWorld`. */
export const WORLD_REFERENCE_DEFAULT_WIDTH = 10;
// #endregion 📐️WorldProjection

// #region 🖼️Reference
/** 🖼️ Persisted media source for a world reference plane. */
export interface WorldReferenceSource {
  readonly url: string;
  readonly mediaKind: import("@semio-tech/ui-react").ReferenceMediaKind;
  readonly page?: number;
}

/** 🖼️ Serializable world reference plane on the infinite grid. */
export interface WorldReferenceProps extends WorldEntityFlags {
  readonly id: string;
  readonly source: WorldReferenceSource;
  readonly origin: Vec3;
  readonly orientation?: Quat;
  readonly scale?: number | Vec3;
  readonly widthWorld?: number;
  readonly opacity?: number;
  readonly relocate?: GumballConfig | false;
  readonly relocateActive?: boolean;
}

/** 🖼️ Gumball relocate commit for a world reference plane. */
export interface WorldReferenceRelocatePayload {
  readonly referenceId: string;
  readonly mode: "translate" | "rotate" | "scale";
  readonly before: GumballPose;
  readonly after: GumballPose;
}

export const WORLD_REFERENCE_SELECTED_BACKGROUND_CSS = semanticVar("active-base");
export const WORLD_REFERENCE_SELECTED_OUTLINE_CSS = tokenVar("primary");
export const WORLD_REFERENCE_HOVER_BACKGROUND_CSS = semanticVar("hover-base");
export const WORLD_REFERENCE_SELECTED_CONTENT_OPACITY = 0.5;

export interface WorldReferenceAppearance {
  readonly backgroundColor: string | null;
  readonly contentOpacity: number;
  readonly outlineColor: string | null;
}

/** 🖼️ Resolves hover/selection fill, content opacity, and border for reference planes. */
export function worldReferenceAppearance(renderMode: Pick<WorldEntityRenderMode, "asHover" | "showSelectedOutline">, baseOpacity: number): WorldReferenceAppearance {
  if (renderMode.showSelectedOutline) {
    return {
      backgroundColor: resolveColorHex(WORLD_REFERENCE_SELECTED_BACKGROUND_CSS, "primary"),
      contentOpacity: baseOpacity * WORLD_REFERENCE_SELECTED_CONTENT_OPACITY,
      outlineColor: resolveColorHex(WORLD_REFERENCE_SELECTED_OUTLINE_CSS, "primary"),
    };
  }
  if (renderMode.asHover) {
    return {
      backgroundColor: resolveColorHex(WORLD_REFERENCE_HOVER_BACKGROUND_CSS, "gray"),
      contentOpacity: baseOpacity,
      outlineColor: resolveColorHex(semanticVar("accent-secondary"), "gray"),
    };
  }
  return {
    backgroundColor: null,
    contentOpacity: baseOpacity,
    outlineColor: null,
  };
}

/** 🖼️ Applies CAD pose fields onto a reference group node. */
export function applyWorldReferencePose(group: Group, reference: Pick<WorldReferenceProps, "origin" | "orientation" | "scale">): void {
  const position = cadVec3ToThree(reference.origin);
  group.position.set(position[0], position[1], position[2]);
  const quat = reference.orientation ?? ([0, 0, 0, 1] as Quat);
  group.quaternion.set(quat[0], quat[1], quat[2], quat[3]);
  const scale = worldReferenceScaleVec(reference.scale);
  group.scale.set(scale[0], scale[1], scale[2]);
}

/** 🪝️ Applies a reference pose at the moment its async-media group actually mounts. */
export function mountWorldReferencePose(group: Group | null, reference: Pick<WorldReferenceProps, "origin" | "orientation" | "scale">): Group | null {
  if (group) applyWorldReferencePose(group, reference);
  return group;
}

/** 🖼️ Writes a gumball pose back onto persisted reference props. */
export function applyWorldReferenceTransform(reference: WorldReferenceProps, after: GumballPose): WorldReferenceProps {
  return {
    ...reference,
    origin: [after.position[0], after.position[1], after.position[2]],
    orientation: [after.quaternion[0], after.quaternion[1], after.quaternion[2], after.quaternion[3]],
    scale: [after.scale[0], after.scale[1], after.scale[2]],
  };
}

const WORLD_REFERENCE_DEFAULT_QUAT: Quat = [0, 0, 0, 1];

/** 🖼️ Resolves reference orientation with CAD default identity quaternion. */
export function worldReferenceOrientation(reference: Pick<WorldReferenceProps, "orientation">): Quat {
  return reference.orientation ?? WORLD_REFERENCE_DEFAULT_QUAT;
}

/** 🖼️ Expands reference scale to a uniform XYZ tuple. */
export function worldReferenceScaleVec(scale: number | Vec3 | undefined): Vec3 {
  if (typeof scale === "number") {
    return [scale, scale, scale];
  }
  if (scale) {
    return [scale[0], scale[1], scale[2]];
  }
  return [1, 1, 1];
}

/** 🧭️ Converts a CAD quaternion to tilt Euler degrees (roll, pitch, yaw). */
export function worldQuatToEulerDegrees(quat: Quat): Vec3 {
  const [x, y, z, w] = quat;
  const sinRoll = 2 * (w * x + y * z);
  const cosRoll = 1 - 2 * (x * x + y * y);
  const roll = Math.atan2(sinRoll, cosRoll);
  const sinPitch = 2 * (w * y - z * x);
  const pitch = Math.abs(sinPitch) >= 1 ? Math.sign(sinPitch) * (Math.PI / 2) : Math.asin(sinPitch);
  const sinYaw = 2 * (w * z + x * y);
  const cosYaw = 1 - 2 * (y * y + z * z);
  const yaw = Math.atan2(sinYaw, cosYaw);
  const radToDeg = 180 / Math.PI;
  return [roll * radToDeg, pitch * radToDeg, yaw * radToDeg];
}

/** 🧭️ Converts tilt Euler degrees (roll, pitch, yaw) to a CAD quaternion. */
export function worldEulerDegreesToQuat(euler: readonly [number, number, number]): Quat {
  const degToRad = Math.PI / 180;
  const roll = euler[0] * degToRad;
  const pitch = euler[1] * degToRad;
  const yaw = euler[2] * degToRad;
  const cy = Math.cos(yaw * 0.5);
  const sy = Math.sin(yaw * 0.5);
  const cp = Math.cos(pitch * 0.5);
  const sp = Math.sin(pitch * 0.5);
  const cr = Math.cos(roll * 0.5);
  const sr = Math.sin(roll * 0.5);
  return [sr * cp * cy - cr * sp * sy, cr * sp * cy + sr * cp * sy, cr * cp * sy - sr * sp * cy, cr * cp * cy + sr * sp * sy];
}

/** 🖼️ Applies a declarative inspector patch to one reference plane. */
export function patchWorldReferenceProps(reference: WorldReferenceProps, field: "origin" | "rotation" | "scale" | "scaleUniform" | "widthWorld" | "opacity", value: unknown): WorldReferenceProps | null {
  const patch: { -readonly [K in keyof Omit<WorldReferenceProps, "id">]?: WorldReferenceProps[K] } = {};
  if (field === "origin" && Array.isArray(value) && value.length === 3) {
    patch.origin = [Number(value[0]), Number(value[1]), Number(value[2])];
  } else if (field === "rotation" && Array.isArray(value) && value.length === 3) {
    patch.orientation = worldEulerDegreesToQuat([Number(value[0]), Number(value[1]), Number(value[2])]);
  } else if (field === "scale" && Array.isArray(value) && value.length === 3) {
    const sx = Number(value[0]);
    const sy = Number(value[1]);
    const sz = Number(value[2]);
    patch.scale = sx === sy && sy === sz ? sx : ([sx, sy, sz] as Vec3);
  } else if (field === "scaleUniform") {
    const parsed = typeof value === "number" ? value : Number(value);
    if (Number.isFinite(parsed)) {
      patch.scale = parsed;
    }
  } else if (field === "widthWorld") {
    const parsed = typeof value === "number" ? value : Number(value);
    if (Number.isFinite(parsed)) {
      patch.widthWorld = parsed;
    }
  } else if (field === "opacity") {
    const parsed = typeof value === "number" ? value : Number(value);
    if (Number.isFinite(parsed)) {
      patch.opacity = parsed;
    }
  } else {
    return null;
  }
  return { ...reference, ...patch };
}

function worldReferencePlaneSize(reference: WorldReferenceProps, aspect: number): readonly [number, number] {
  const width = reference.widthWorld ?? WORLD_REFERENCE_DEFAULT_WIDTH;
  return [width, width / Math.max(aspect, 1e-6)];
}

const WorldReferenceGumball = reactHostPort.memo(function WorldReferenceGumball(props: { readonly referenceId: string; readonly target: Group; readonly config: GumballConfig; readonly onRelocate?: (payload: WorldReferenceRelocatePayload) => void }) {
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
          referenceId: props.referenceId,
          mode: gumballHandleKindToTransformMode(kind),
          before: beforeRef.current ?? before,
          after,
        });
        beforeRef.current = null;
      }}
    />
  );
});

const WorldReferencePlaneItem = reactHostPort.memo(function WorldReferencePlaneItem(props: {
  readonly reference: WorldReferenceProps;
  readonly selected: boolean;
  readonly hovered: boolean;
  readonly revealed: boolean;
  readonly gumballConfig?: GumballConfig;
  readonly relocateActive?: boolean;
  readonly translationSnap?: number;
  readonly onSelect?: (id: string, modifiers: { readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean }) => void;
  readonly onHover?: (id: string | null) => void;
  readonly onRelocate?: (payload: WorldReferenceRelocatePayload) => void;
}) {
  const groupRef = reactHostPort.useRef<Group>(null);
  const mountGroup = reactHostPort.useCallback(
    (group: Group | null) => {
      groupRef.current = mountWorldReferencePose(group, props.reference);
    },
    [props.reference.origin, props.reference.orientation, props.reference.scale],
  );
  const [pointerHovered, setPointerHovered] = reactHostPort.useState(false);
  const [media, setMedia] = reactHostPort.useState<{ readonly width: number; readonly height: number; readonly texture: ThreeTexture } | null>(null);
  const selectable = worldEntitySelectable(props.reference);
  const inspectable = worldEntityInspectable(props.reference);
  const interactionHovered = selectable && (props.hovered || pointerHovered);
  const renderMode = {
    ...worldEntityRenderMode(props.reference, {
      hovered: interactionHovered,
      selected: props.selected,
      revealed: props.revealed || (selectable && pointerHovered),
    }),
    showSelectedOutline: props.selected && inspectable,
  };
  reactHostPort.useEffect(() => {
    let cancelled = false;
    referenceMediaPort
      .loadReferenceTexture(props.reference.source)
      .then((loaded) => {
        if (cancelled) {
          loaded.texture.dispose();
          return;
        }
        setMedia((prev) => {
          prev?.texture.dispose();
          return loaded;
        });
      })
      .catch(() => {
        // texture load failure — plane stays hidden
      });
    return () => {
      cancelled = true;
    };
  }, [props.reference.id, props.reference.source.url, props.reference.source.mediaKind, props.reference.source.page]);
  const invalidate = useThree((state) => state.invalidate);
  // 🎞️ `WorldCanvas` uses `frameloop="demand"` — async texture arrival must kick a draw or panes stay transparent.
  reactHostPort.useLayoutEffect(() => {
    if (media) invalidate();
  }, [invalidate, media]);
  const mediaAspect = media ? media.width / media.height : 1;
  const [planeWidth, planeHeight] = reactHostPort.useMemo(() => worldReferencePlaneSize(props.reference, mediaAspect), [props.reference, mediaAspect]);
  const opacityBase = props.reference.opacity ?? 1;
  const opacityDimmed = renderMode.dim ? opacityBase * WORLD_LOCKED_OPACITY_SCALE : opacityBase;
  const appearance = worldReferenceAppearance(renderMode, opacityDimmed);
  const planeOutlineGeo = reactHostPort.useMemo(() => {
    const plane = new PlaneGeometry(planeWidth, planeHeight);
    const edges = new EdgesGeometry(plane);
    plane.dispose();
    return edges;
  }, [planeWidth, planeHeight]);
  reactHostPort.useEffect(() => () => planeOutlineGeo.dispose(), [planeOutlineGeo]);
  if (!renderMode.visible || !media) {
    return null;
  }
  const config = props.reference.relocate === false ? null : { ...(props.reference.relocate ?? {}), translationSnap: props.translationSnap };
  const showGumball = props.selected && selectable && props.relocateActive !== false && groupRef.current && config && gumballConfigVisible(config);
  return (
    <>
      <group ref={mountGroup} visible={renderMode.visible} userData={{ worldReferenceId: props.reference.id }}>
        {appearance.backgroundColor ? (
          <mesh raycast={worldRaycastNone} renderOrder={-11}>
            <planeGeometry args={[planeWidth, planeHeight]} />
            <meshBasicMaterial attach="material" color={appearance.backgroundColor} transparent opacity={1} depthWrite={false} side={DoubleSide} toneMapped={false} />
          </mesh>
        ) : null}
        <mesh
          renderOrder={-10}
          raycast={selectable ? undefined : worldRaycastNone}
          onPointerDown={(event) => {
            if (!selectable || event.button !== 0) {
              return;
            }
            event.stopPropagation();
            props.onSelect?.(props.reference.id, { shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
          }}
          onPointerOver={(event) => {
            if (!selectable) {
              return;
            }
            event.stopPropagation();
            setPointerHovered(true);
            props.onHover?.(props.reference.id);
          }}
          onPointerOut={(event) => {
            if (!selectable) {
              return;
            }
            event.stopPropagation();
            setPointerHovered(false);
            props.onHover?.(null);
          }}
        >
          <planeGeometry args={[planeWidth, planeHeight]} />
          <meshBasicMaterial attach="material" map={media.texture} transparent opacity={appearance.contentOpacity} depthWrite={false} side={DoubleSide} toneMapped={false} />
        </mesh>
        {appearance.outlineColor ? (
          <lineSegments raycast={worldRaycastNone} geometry={planeOutlineGeo} renderOrder={-9} scale={[1.002, 1.002, 1.002]}>
            <lineBasicMaterial attach="material" color={appearance.outlineColor} transparent opacity={renderMode.showSelectedOutline ? 1 : 0.9} depthWrite={false} />
          </lineSegments>
        ) : null}
      </group>
      {showGumball && groupRef.current ? <WorldReferenceGumball referenceId={props.reference.id} target={groupRef.current} config={config} onRelocate={props.onRelocate} /> : null}
    </>
  );
});

/** 🖼️ Renders persisted world reference planes for CAD and puzzle 3d hosts. */
export function WorldReferenceLayer(props: {
  readonly references: readonly WorldReferenceProps[];
  readonly selectedIds?: ReadonlySet<string>;
  readonly hoveredId?: string | null;
  readonly revealedIds?: ReadonlySet<string>;
  readonly gumballConfig?: GumballConfig;
  readonly relocateActive?: boolean;
  readonly translationSnap?: number;
  readonly onSelect?: (id: string, modifiers: { readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean }) => void;
  readonly onHover?: (id: string | null) => void;
  readonly onRelocate?: (payload: WorldReferenceRelocatePayload) => void;
}): ReactElement | null {
  if (!props.references.length) {
    return null;
  }
  const selected = props.selectedIds ?? new Set<string>();
  const revealed = props.revealedIds ?? new Set<string>();
  return (
    <>
      {props.references.map((reference) => (
        <WorldReferencePlaneItem
          key={reference.id}
          reference={reference}
          selected={selected.has(reference.id)}
          hovered={props.hoveredId === reference.id}
          revealed={revealed.has(reference.id)}
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
// #endregion 🖼️Reference
