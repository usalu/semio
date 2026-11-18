/** 🧩️ Composition of placed models into one scene document: the neutral 3D twin of the playground's composition.
 *
 * Every part is a resolved `semio.model/v1` document of one placed asset (one scale, one pose or clip). The composition gives each part a placement node (`slot-<index>`) that carries its position and its turn, hangs the part's root nodes under it, prefixes every identifier of the part with its index (`p0-`, `p1-`, …) so equal assets
 * stay apart, merges the scenes and the variants of the parts by name (so a switch of representation or mode is a switch in the one document, never a rebuild), adds a floor with a metre grid, and lays every part's clip (or pose) on one master animation `timeline` through {@link retimeChannel}.
 *
 * Frames: the staffage world (x ahead, y up, z left) maps by `(x, y, z) → (z, y, x)` to the neutral frame (README of the kernel); a positive yaw turns counter-clockwise seen from above (forward goes to the left), which is a positive rotation about +Y of the neutral frame.
 * A part with a `plantNode` stands where it is placed: the ahead travel of that root is removed from every root-level translation of the part (the 3D counterpart of the 2D figure that is planted on its ground point), so a walking figure walks in place exactly like the drawing.
 *
 * @see ../🎞️retime/🟦️.ts — the clip on the timeline
 * @see ../../🎞️timeline/🟦️.ts — the rules the master animation reproduces
 * @see ../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧬️schema/🟦️.ts — the document
 */
import type { Animation, Channel, Material, Mesh, ModelDocument, Node, Pose, Primitive, Scene, Skin, Slug, TransformPatch, Variant, Vec3 } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { constant, retimeChannel, sampleChannel, type Window } from "../🎞️retime/🟦️.ts";

/** 🧱️ One placed model. `animation` is the id of its clip in `document`, `pose` the id of its pose; both `null` leaves it at rest. `window` is where its clip plays on the timeline. */
export type Part = { readonly id: string; readonly document: ModelDocument; readonly xM: number; readonly zM: number; readonly yawDeg: number; readonly animation: Slug | null; readonly pose: Slug | null; readonly window: Window | null; readonly plantNode: Slug | null };

/** 🟫️ The floor under the parts: staffage metres, a grid line every `stepM`. */
export type Floor = { readonly minXM: number; readonly maxXM: number; readonly minZM: number; readonly maxZM: number; readonly stepM: number };

/** ⚙️ What a composition needs besides its parts. */
export type ComposeOptions = { readonly id: string; readonly timelineS: number; readonly floor: Floor | null };

/** 🏷️ The id of the animation that carries every clip. */
export const TIMELINE_ANIMATION = "timeline";

/** 🏷️ The identifier prefix of the part at `index`: `p0-`, `p1-`, … (kebab-case like every identifier, and unique whatever the placement ids are). */
export const prefixOf = (index: number): string => `p${index}-`;

/** 🧭️ The id of the node that carries the position and turn of the part at `index`. */
export const slotOf = (index: number): string => `slot-${index}`;

/** 🔎️ The placement an identifier of the composition belongs to (`ids` in the order of the parts), or null (the floor, a slot). */
export function placementOf(identifier: string, ids: readonly string[]): string | null {
  const found = /^p(\d+)-/u.exec(identifier);
  return found === null ? null : (ids[Number(found[1])] ?? null);
}

const FLOOR_NODE = "floor";
const FLOOR_MESH = "floor";
const FLOOR_FACE = "floor-face";
const FLOOR_GRID = "floor-grid";

const IDENTITY_SCALE: Vec3 = [1, 1, 1];

/** 🧭️ The position and turn of a placement as the transform of its node. */
export function placeNode(index: number, xM: number, zM: number, yawDeg: number): Node {
  const half = (yawDeg * Math.PI) / 360;
  return { id: slotOf(index), parent: null, translation: [zM, 0, xM], rotation: [0, Math.sin(half), 0, Math.cos(half)], scale: IDENTITY_SCALE };
}

function floorMesh(floor: Floor): { readonly mesh: Mesh; readonly materials: readonly Material[] } {
  const { minXM, maxXM, minZM, maxZM, stepM } = floor;
  const face: Primitive = { mode: "triangles", material: FLOOR_FACE, positions: [minZM, -0.004, minXM, minZM, -0.004, maxXM, maxZM, -0.004, maxXM, maxZM, -0.004, minXM], normals: [0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0], indices: [0, 1, 2, 0, 2, 3] };
  const positions: number[] = [];
  const indices: number[] = [];
  const line = (a: readonly [number, number], b: readonly [number, number]): void => {
    const at = positions.length / 3;
    positions.push(a[0], -0.002, a[1], b[0], -0.002, b[1]);
    indices.push(at, at + 1);
  };
  for (let x = Math.ceil(minXM / stepM) * stepM; x <= maxXM + 1e-9; x += stepM) line([minZM, x], [maxZM, x]);
  for (let z = Math.ceil(minZM / stepM) * stepM; z <= maxZM + 1e-9; z += stepM) line([z, minXM], [z, maxXM]);
  const grid: Primitive = { mode: "lines", material: FLOOR_GRID, positions, indices };
  const material = (id: string, title: { readonly en: string; readonly de: string }, baseColor: readonly [number, number, number, number]): Material => ({ id, title, baseColor, metallic: 0, roughness: 1, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: true, unlit: true });
  return { mesh: { id: FLOOR_MESH, primitives: positions.length === 0 ? [face] : [face, grid] }, materials: [material(FLOOR_FACE, { en: "Floor", de: "Boden" }, [0.82, 0.8, 0.74, 1]), material(FLOOR_GRID, { en: "Floor grid", de: "Bodenraster" }, [0.45, 0.45, 0.42, 1])] };
}

function planted(part: Part, rootLevel: ReadonlySet<Slug>, channels: readonly Channel[]): readonly Channel[] {
  if (part.plantNode === null) return channels;
  const root = channels.find((channel) => "node" in channel.target && channel.target.node === part.plantNode && channel.target.path === "translation");
  if (root === undefined) return channels;
  return channels.map((channel) => {
    if (!("node" in channel.target) || channel.target.path !== "translation" || !rootLevel.has(channel.target.node)) return channel;
    const values = channel.times.flatMap((timeS, index) => {
      const travel = sampleChannel(root, timeS)[2] ?? 0;
      return [channel.values[index * 3]!, channel.values[index * 3 + 1]!, channel.values[index * 3 + 2]! - travel];
    });
    return { ...channel, values };
  });
}

function poseChannels(part: Part, pose: Pose, rootLevel: ReadonlySet<Slug>): readonly Channel[] {
  const nodes = new Map(part.document.nodes.map((node) => [node.id, node]));
  const travel = part.plantNode === null ? 0 : (pose.nodes[part.plantNode]?.translation?.[2] ?? nodes.get(part.plantNode)?.translation[2] ?? 0);
  const out: Channel[] = [];
  for (const [node, patch] of Object.entries(pose.nodes) as [Slug, TransformPatch][]) {
    const rest = nodes.get(node);
    if (rest === undefined) continue;
    const entries: [("translation" | "rotation" | "scale"), readonly number[]][] = [];
    if (patch.translation !== undefined) entries.push(["translation", rootLevel.has(node) && part.plantNode !== null ? [patch.translation[0], patch.translation[1], patch.translation[2] - travel] : patch.translation]);
    if (patch.rotation !== undefined) entries.push(["rotation", patch.rotation]);
    if (patch.scale !== undefined) entries.push(["scale", patch.scale]);
    for (const [path, value] of entries) out.push({ target: { node, path }, interpolation: "linear", times: [0, 1], values: [...value, ...value] });
  }
  return out;
}

/** 🏗️ Composes the parts into one resolved document with the master animation {@link TIMELINE_ANIMATION}; throws for a duplicate placement id. */
export function composePlayground(parts: readonly Part[], options: ComposeOptions): ModelDocument {
  const ids = new Set<string>();
  for (const part of parts) {
    if (ids.has(part.id)) throw new Error(`duplicate placement ${part.id}`);
    ids.add(part.id);
  }
  const nodes: Node[] = [];
  const meshes: Mesh[] = [];
  const skins: Skin[] = [];
  const materials: Material[] = [];
  const variants = new Map<Slug, { title: Variant["title"]; bindings: Record<Slug, Slug> }>();
  const scenes = new Map<Slug, { title: Scene["title"]; tags: readonly Slug[] | undefined; nodes: Slug[] }>();
  const channels: { target: Channel["target"]; interpolation: Channel["interpolation"]; times: number[]; values: number[] }[] = [];
  const push = (channel: Channel, keyed: { readonly times: readonly number[]; readonly values: readonly number[] }): void => {
    if (keyed.times.length > 0) channels.push({ target: channel.target, interpolation: channel.interpolation, times: [...keyed.times], values: [...keyed.values] });
  };
  for (const [index, part] of parts.entries()) {
    const p = prefixOf(index);
    const q = (id: Slug): Slug => `${p}${id}`;
    const { document } = part;
    const place = placeNode(index, part.xM, part.zM, part.yawDeg);
    nodes.push(place);
    const rootLevel = new Set(document.nodes.filter((node) => node.parent === null).map((node) => node.id));
    for (const node of document.nodes) nodes.push({ ...node, id: q(node.id), parent: node.parent === null ? place.id : q(node.parent), ...(node.mesh === undefined ? {} : { mesh: q(node.mesh) }), ...(node.skin === undefined ? {} : { skin: q(node.skin) }) });
    for (const mesh of document.meshes) meshes.push({ id: q(mesh.id), primitives: mesh.primitives.map((primitive) => ({ ...primitive, material: q(primitive.material), ...(primitive.solids === undefined ? {} : { solids: primitive.solids.map((solid) => ({ ...solid, id: q(solid.id) })) }) })) });
    for (const skin of document.skins) skins.push({ ...skin, id: q(skin.id), joints: skin.joints.map(q) });
    for (const material of document.materials) materials.push({ ...material, id: q(material.id) });
    for (const variant of document.variants) {
      const merged = variants.get(variant.id) ?? { title: variant.title, bindings: {} };
      for (const [from, to] of Object.entries(variant.bindings)) merged.bindings[q(from)] = q(to);
      variants.set(variant.id, merged);
    }
    for (const scene of document.scenes ?? []) {
      const merged = scenes.get(scene.id) ?? { title: scene.title, tags: scene.tags, nodes: [] };
      merged.nodes.push(...scene.nodes.map(q));
      scenes.set(scene.id, merged);
    }
    const retarget = (channel: Channel): Channel => ({ ...channel, target: "node" in channel.target ? { node: q(channel.target.node), path: channel.target.path } : { material: q(channel.target.material), path: channel.target.path } });
    const animation: Animation | undefined = part.animation === null ? undefined : document.animations.find((candidate) => candidate.id === part.animation);
    const pose = part.pose === null ? undefined : document.poses.find((candidate) => candidate.id === part.pose);
    if (animation !== undefined && part.window !== null) for (const channel of planted(part, rootLevel, animation.channels)) push(retarget(channel), retimeChannel(channel, part.window, options.timelineS));
    else if (pose !== undefined) for (const channel of poseChannels(part, pose, rootLevel)) push(retarget(channel), constant(channel.values.slice(0, channel.values.length / 2), options.timelineS));
  }
  if (options.floor !== null) {
    const { mesh, materials: floorMaterials } = floorMesh(options.floor);
    nodes.push({ id: FLOOR_NODE, parent: null, translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: IDENTITY_SCALE, mesh: mesh.id });
    meshes.push(mesh);
    materials.push(...floorMaterials);
    for (const scene of scenes.values()) scene.nodes.push(FLOOR_NODE);
  }
  const animation: Animation = { id: TIMELINE_ANIMATION, title: { en: "Timeline", de: "Zeitleiste" }, durationS: options.timelineS, loop: false, channels, events: [] };
  return {
    version: "semio.model/v1",
    id: options.id,
    title: { en: "Playground", de: "Spielplatz" },
    nodes,
    scenes: [...scenes].map(([id, scene]) => ({ id, title: scene.title, nodes: scene.nodes, ...(scene.tags === undefined ? {} : { tags: scene.tags }) })),
    solids: [],
    meshes,
    skins,
    materials,
    variants: [...variants].map(([id, variant]) => ({ id, title: variant.title, bindings: variant.bindings })),
    poses: [],
    animations: [animation],
  };
}
