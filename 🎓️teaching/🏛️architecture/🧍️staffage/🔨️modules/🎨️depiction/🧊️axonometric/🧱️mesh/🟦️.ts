/** 🧱️ The posed triangle soup of one evaluated instant: the volume solids of the asset at the scale (`maxDenominator`), tessellated by the neutral kernel at a chord tolerance of the paper, carried by the evaluated bone frames (rigid per bone, uniformly blended for weighted solids exactly as the kernel skins them), plus the glyph solids of the analytic view.
 *
 * Nothing about posing is solved here: the bone frames come from `evaluate` (placements as in `modelOf`, see ../../../🧊️volume/🧷️frames), the node hierarchy from `skeletonOf`, the geometry from the kernel `tessellateSolid` (the same tessellator the model, GLB and FBX use), the glyphs from `glyphOf`. The soup is in the neutral frame (right-handed, +Y up, +Z ahead, +X the asset's left, metres) with counter-clockwise outward triangles.
 * The rest geometry of an asset at a scale is tessellated once and cached, so the frames of a clip only pay for posing.
 *
 * @see ../../../🧊️volume/🟦️.ts — `modelOf`, the same pipeline as a model document
 * @see ../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts — `tessellateSolid`
 */
import type { Asset, ShapeRole, Slug, Volume, VolumeSolid } from "../../../../🧬️schema/🟦️.ts";
import { invert4, multiply4, restWorldMatrices, tessellateSolid, transformPoint, type M4, type Node, type Quality } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { type FrameData, type Slot, glyphOf } from "../../../🧊️volume/🧷️frames/🟦️.ts";
import { type Surface, surfacesOf } from "../../../🧊️volume/🎨️looks/🟦️.ts";
import { skeletonOf } from "../../../🧊️volume/🦴️skeleton/🟦️.ts";
import { shownSolids, solidOf } from "../../../🧊️volume/🧊️solids/🟦️.ts";

/** 🧩️ One drawn body: a volume solid or a glyph solid, its triangle range in the soup and what the views paint it by. */
export type Part = {
  readonly id: Slug;
  readonly kind: "solid" | "glyph";
  readonly bone: Slug | null;
  readonly role: ShapeRole;
  readonly tone: number;
  readonly regions: readonly Slug[];
  readonly surface: Surface | null;
  readonly material: Slug;
  readonly solid: VolumeSolid | null;
  readonly first: number;
  readonly count: number;
};

/** 🦄 The posed triangles: `vertices` are `x, y, z` in the neutral frame, `indices` three per triangle, `part` the owner of every triangle. */
export type Soup = { readonly vertices: Float64Array; readonly indices: Uint32Array; readonly part: Uint32Array; readonly parts: readonly Part[] };

/** 🎚️ The paper deviation in millimetres the tessellation of a drawing may have from the true surface. */
export const CHORD_PAPER_MM = 0.04;

/** 🐙 The tessellation quality at a scale denominator: a chord deviation of {@link CHORD_PAPER_MM} paper millimetres (so `0.04 · denominator / 1000` metres, between 0.5 mm and 50 mm) and 12 to 64 segments per circle; deterministic and the same for every frame of a drawing. */
export const qualityAt = (denominator: number): Quality => ({ chordM: Math.min(0.05, Math.max(0.0005, (CHORD_PAPER_MM * denominator) / 1000)), minSegments: 12, maxSegments: 64 });

type Rest = { readonly solid: VolumeSolid; readonly surface: Surface; readonly positions: Float64Array; readonly indices: Uint32Array; readonly joints: readonly { readonly bone: Slug; readonly weight: number }[] };
type Cache = { readonly solids: readonly Rest[]; readonly surfaces: readonly Surface[]; readonly world: Map<Slug, M4>; readonly inverse: Map<Slug, M4>; readonly nodes: readonly Node[] };

const RESTS = new WeakMap<Volume, Map<number, Cache>>();
const GLYPHS = new Map<string, { readonly positions: Float64Array; readonly indices: Uint32Array }>();
const GLYPH_CACHE_LIMIT = 1024;

const IDENTITY: M4 = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];

function jointsOf(solid: VolumeSolid): readonly { readonly bone: Slug; readonly weight: number }[] {
  const live = (solid.weights ?? []).filter((w) => w.weight > 0).slice(0, 4);
  const total = live.reduce((sum, w) => sum + w.weight, 0);
  return live.map((w) => ({ bone: w.bone, weight: w.weight / total }));
}

/** 🦋 The rest tessellation of an asset's volume at a denominator, cached per volume. */
function restOf(asset: Asset, denominator: number): Cache {
  const volume = asset.volume!;
  const byScale = RESTS.get(volume) ?? RESTS.set(volume, new Map()).get(volume)!;
  const known = byScale.get(denominator);
  if (known !== undefined) return known;
  const nodes = skeletonOf(asset);
  const world = restWorldMatrices(nodes);
  const inverse = new Map([...world].map(([id, m]) => [id, invert4(m)]));
  const shown = shownSolids(volume, denominator);
  const { surfaces, of } = surfacesOf(shown);
  const quality = qualityAt(denominator);
  const solids = shown.map((solid): Rest => {
    const joints = jointsOf(solid);
    const geometry = tessellateSolid(solidOf(solid, of.get(solid.id)!.id), quality, joints.length > 0 ? world.get(solid.bone)! : IDENTITY);
    return { solid, surface: of.get(solid.id)!, positions: Float64Array.from(geometry.positions), indices: Uint32Array.from(geometry.indices), joints };
  });
  const made: Cache = { solids, surfaces, world, inverse, nodes };
  byScale.set(denominator, made);
  return made;
}

/** 🐝 The surfaces of the solids shown at a denominator, in order of first use (the base surfaces of the looks). */
export const surfacesAt = (asset: Asset, denominator: number): readonly Surface[] => restOf(asset, denominator).surfaces;

/** 🧊️ The solids shown at a denominator. */
export const solidsAt = (asset: Asset, denominator: number): readonly VolumeSolid[] => restOf(asset, denominator).solids.map((r) => r.solid);

const GLYPH_HIDDEN = 1e-4;

/** 🦉 Poses the volume of an asset by the bone placements of one frame (and the glyph slots of that frame) into one soup. `asset` must have a volume; `denominator` filters the solids and sets the tessellation; `mm` is the metres per paper millimetre of the scale. */
export function poseSoup(asset: Asset, frame: FrameData, slots: ReadonlyMap<Slug, Slot>, denominator: number): Soup {
  const cache = restOf(asset, denominator);
  const posed: Node[] = cache.nodes.map((node) => {
    const local = frame.locals[node.id]!;
    return { ...node, translation: [...local.translation], rotation: [...local.rotation] };
  });
  const world = restWorldMatrices(posed);
  const mm = denominator / 1000;
  const pieces: { id: Slug; part: Omit<Part, "first" | "count">; positions: Float64Array; indices: Uint32Array; move: (out: Float64Array) => void }[] = [];
  for (const rest of cache.solids) {
    const { solid, surface, positions, indices, joints } = rest;
    let blend: M4;
    if (joints.length > 0) {
      blend = new Array<number>(16).fill(0);
      for (const joint of joints) {
        const m = multiply4(world.get(joint.bone)!, cache.inverse.get(joint.bone)!);
        for (let i = 0; i < 16; i++) blend[i]! += joint.weight * m[i]!;
      }
    } else blend = world.get(solid.bone)!;
    const matrix = blend;
    pieces.push({
      id: solid.id,
      part: { id: solid.id, kind: "solid", bone: solid.bone, role: solid.role, tone: solid.tone, regions: surface.regions, surface, material: surface.id, solid },
      positions,
      indices,
      move: (out) => {
        for (let i = 0; i < positions.length; i += 3) {
          const p = transformPoint(matrix, [positions[i]!, positions[i + 1]!, positions[i + 2]!]);
          out[i] = p[0];
          out[i + 1] = p[1];
          out[i + 2] = p[2];
        }
      },
    });
  }
  const quality = qualityAt(denominator);
  for (const [root, slot] of slots) {
    const made = glyphOf(root, slot, mm, frame.glyphs);
    const rootNode = made.nodes.find((n) => n.id === root)!;
    if (rootNode.scale[0]! < GLYPH_HIDDEN) continue;
    const glyphWorld = restWorldMatrices(made.nodes);
    for (const solid of made.solids) {
      const node = made.nodes.find((n) => n.id === solid.node)!;
      if (node.scale[0]! < GLYPH_HIDDEN || node.scale[1]! < GLYPH_HIDDEN || node.scale[2]! < GLYPH_HIDDEN) continue;
      const key = `${solid.id}|${mm}`;
      let geometry = GLYPHS.get(key);
      if (geometry === undefined) {
        const g = tessellateSolid(solid, quality);
        geometry = { positions: Float64Array.from(g.positions), indices: Uint32Array.from(g.indices) };
        if (GLYPHS.size >= GLYPH_CACHE_LIMIT) GLYPHS.delete(GLYPHS.keys().next().value!);
        GLYPHS.set(key, geometry);
      }
      const matrix = glyphWorld.get(solid.node)!;
      const positions = geometry.positions;
      pieces.push({
        id: solid.id,
        part: { id: solid.id, kind: "glyph", bone: null, role: "detail", tone: 0, regions: [], surface: null, material: solid.material, solid: null },
        positions,
        indices: geometry.indices,
        move: (out) => {
          for (let i = 0; i < positions.length; i += 3) {
            const p = transformPoint(matrix, [positions[i]!, positions[i + 1]!, positions[i + 2]!]);
            out[i] = p[0];
            out[i + 1] = p[1];
            out[i + 2] = p[2];
          }
        },
      });
    }
  }
  let vertexCount = 0;
  let indexCount = 0;
  for (const p of pieces) {
    vertexCount += p.positions.length / 3;
    indexCount += p.indices.length;
  }
  const vertices = new Float64Array(vertexCount * 3);
  const indices = new Uint32Array(indexCount);
  const part = new Uint32Array(indexCount / 3);
  const parts: Part[] = [];
  let vertexAt = 0;
  let indexAt = 0;
  for (const piece of pieces) {
    const count = piece.indices.length;
    const out = new Float64Array(piece.positions.length);
    piece.move(out);
    vertices.set(out, vertexAt * 3);
    for (let i = 0; i < count; i++) indices[indexAt + i] = piece.indices[i]! + vertexAt;
    part.fill(parts.length, indexAt / 3, (indexAt + count) / 3);
    parts.push({ ...piece.part, first: indexAt / 3, count: count / 3 });
    vertexAt += piece.positions.length / 3;
    indexAt += count;
  }
  return { vertices, indices, part, parts };
}
