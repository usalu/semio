import { BufferAttribute, DoubleSide, type ThreeEvent } from "@semio-tech/ui-react";
import { useMemo } from "../../⚛️react-port/🟦️.ts";

type Point = [number, number, number];
type Segment = [Point, Point];
type TerrainFeature = { readonly id: string; readonly name: string };
type TerrainFeatureGroups<T extends TerrainFeature> = {
  readonly positions: readonly T[];
  readonly routes: readonly T[];
  readonly regions: readonly T[];
};

export type TerrainOverlayTile = {
  readonly positions: readonly (TerrainFeature & { readonly point: Point })[];
  readonly routes: readonly TerrainPath[];
  readonly regions: readonly TerrainPath[];
};
type TerrainPath = TerrainFeature & { readonly segments: readonly Segment[] };
type Attribution = { readonly url: string; readonly label: string };
type TerrainOverlayDescriptor = { readonly overlays?: TerrainFeatureGroups<TerrainFeature>; readonly attribution?: Attribution };

export function parseTerrainOverlayDescriptor(json: string | undefined): TerrainOverlayDescriptor {
  try {
    const parsed = JSON.parse(json ?? "null") as TerrainOverlayDescriptor | null;
    const attribution = parsed?.attribution;
    const overlays = parsed?.overlays;
    return {
      overlays: overlays && Array.isArray(overlays.positions) && Array.isArray(overlays.routes) && Array.isArray(overlays.regions) ? overlays : undefined,
      attribution: attribution && typeof attribution.label === "string" && typeof attribution.url === "string" && /^https:\/\//i.test(attribution.url) ? attribution : undefined,
    };
  } catch {
    return {};
  }
}

export function TerrainMapFeatures({ tile, tileWidth, onFeatureHover, onPositionDown, onPositionMove, onPositionUp }: {
  readonly tile: TerrainOverlayTile;
  readonly tileWidth: number;
  readonly onFeatureHover?: (name: string | null) => void;
  readonly onPositionDown?: (id: string, event: ThreeEvent<PointerEvent>) => void;
  readonly onPositionMove?: (event: ThreeEvent<PointerEvent>) => void;
  readonly onPositionUp?: (event: ThreeEvent<PointerEvent>) => void;
}) {
  const lift = tileWidth * 0.003;
  const radius = tileWidth * 0.004;
  const show = (feature: TerrainFeature) => () => onFeatureHover?.(feature.name || null);
  const hide = () => onFeatureHover?.(null);
  const positions = useMemo(() => [...tile.positions].sort((a, b) => a.id < b.id ? -1 : Number(a.id > b.id)), [tile.positions]);
  return (
    <group>
      {positions.map((feature) => {
        const [x, y, z] = feature.point;
        return (
          <mesh key={feature.id} name={feature.name} position={[x, y, z + radius * 2]} scale={[radius, radius, radius * 2]} onPointerOver={show(feature)} onPointerOut={hide} onPointerDown={onPositionDown && ((event) => onPositionDown(feature.id, event))} onPointerMove={onPositionMove} onPointerUp={onPositionUp}>
            <octahedronGeometry args={[1]} />
            <meshBasicMaterial color="#f8d94e" toneMapped={false} />
          </mesh>
        );
      })}
      {tile.routes.map((feature) => <TerrainPathMesh key={feature.id} path={feature} color="#f7f4e8" halfWidth={tileWidth * 0.0011} lift={lift} onEnter={show(feature)} onLeave={hide} />)}
      {tile.regions.map((feature) => <TerrainPathMesh key={feature.id} path={feature} color="#4fd4e8" halfWidth={tileWidth * 0.0009} lift={lift} onEnter={show(feature)} onLeave={hide} />)}
    </group>
  );
}

function TerrainPathMesh({ path, color, halfWidth, lift, onEnter, onLeave }: { readonly path: TerrainPath; readonly color: string; readonly halfWidth: number; readonly lift: number; readonly onEnter: () => void; readonly onLeave: () => void }) {
  const vertices = useMemo(() => {
    const positions: number[] = [];
    for (const [a, b] of path.segments) {
      const length = Math.hypot(b[0] - a[0], b[1] - a[1]);
      if (length === 0) continue;
      const dx = ((a[1] - b[1]) / length) * halfWidth;
      const dy = ((b[0] - a[0]) / length) * halfWidth;
      const leftA = [a[0] + dx, a[1] + dy, a[2] + lift];
      const rightA = [a[0] - dx, a[1] - dy, a[2] + lift];
      const leftB = [b[0] + dx, b[1] + dy, b[2] + lift];
      const rightB = [b[0] - dx, b[1] - dy, b[2] + lift];
      positions.push(...leftA, ...rightA, ...leftB, ...leftB, ...rightA, ...rightB);
    }
    return new BufferAttribute(new Float32Array(positions), 3);
  }, [path, halfWidth, lift]);
  return (
    <mesh name={path.name} onPointerOver={onEnter} onPointerOut={onLeave}>
      <bufferGeometry attributes-position={vertices} />
      <meshBasicMaterial color={color} side={DoubleSide} toneMapped={false} />
    </mesh>
  );
}
