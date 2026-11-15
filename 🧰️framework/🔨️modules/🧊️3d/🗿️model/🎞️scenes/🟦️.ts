/** 🎞️ Scenes of a model: named sets of mesh-carrying nodes over one shared skeleton.
 *
 * Rule: a document without `scenes` (or with `[]`) is one implicit scene holding every node that carries a solid or a mesh. A node belongs to a scene by being listed in it; a solid or a mesh belongs to the scenes of its node. A bone that needs different solids per scene has one identity child node per scene.
 * {@link selectScene} cuts one scene out as a self-contained single-scene document: its mesh-carrying nodes, the skeleton they need (ancestors, weight joints, skin joints) without the solids and meshes of foreign scenes, the matching animation channels, poses and skins.
 * @see ../🧬️schema/🟦️.ts#Scene */
import type { Animation, ModelDocument, Node, Pose, Scene, Slug } from "../🧬️schema/🟦️.ts";

/** 🛑️ Thrown when a scene id does not exist in the document. */
export class UnknownSceneError extends Error {
  constructor(readonly sceneId: Slug) {
    super(`unknown scene ${sceneId}`);
    this.name = "UnknownSceneError";
  }
}

/** 🎛️ The scene half of every scene-aware call: omitted means the whole document. */
export type SceneOption = { readonly scene?: Slug };

/** 🧱️ The nodes that carry a solid or a mesh, in document order. */
function carriers(document: ModelDocument): Slug[] {
  const solid = new Set(document.solids.map((s) => s.node));
  return document.nodes.filter((n) => n.mesh !== undefined || solid.has(n.id)).map((n) => n.id);
}

/** 📚️ The explicit scenes, or the one implicit scene `{ id: document.id, title: document.title, nodes: every carrier }`. */
export function scenesOf(document: ModelDocument): readonly Scene[] {
  if (document.scenes !== undefined && document.scenes.length > 0) return document.scenes;
  return [{ id: document.id, title: document.title, nodes: carriers(document) }];
}

const find = (document: ModelDocument, sceneId: Slug): Scene => scenesOf(document).find((s) => s.id === sceneId) ?? ((): never => { throw new UnknownSceneError(sceneId); })();

/** 🔗️ The scene ids of every node, in scene order; empty for a document without explicit scenes. */
export function sceneIdsOfNodes(document: ModelDocument): Map<Slug, Slug[]> {
  const out = new Map<Slug, Slug[]>();
  for (const scene of document.scenes ?? []) for (const node of scene.nodes) {
    const list = out.get(node) ?? [];
    if (!list.includes(scene.id)) list.push(scene.id);
    out.set(node, list);
  }
  return out;
}

/** 🦴️ The nodes a scene needs: its own, their ancestors, the joints of its solids' weights and of its skins, and the ancestors of all of those. */
export function nodesOfScene(document: ModelDocument, sceneId: Slug): ReadonlySet<Slug> {
  const scene = find(document, sceneId);
  const byId = new Map<Slug, Node>(document.nodes.map((n) => [n.id, n]));
  const skins = new Map(document.skins.map((s) => [s.id, s]));
  const members = new Set(scene.nodes);
  const needed = new Set<Slug>();
  const add = (id: Slug): void => {
    for (let at: Slug | null = id; at !== null && !needed.has(at); at = byId.get(at)?.parent ?? null) needed.add(at);
  };
  for (const id of members) add(id);
  for (const solid of document.solids) if (members.has(solid.node)) for (const w of solid.weights ?? []) add(w.node);
  for (const id of members) {
    const skin = byId.get(id)?.skin;
    if (skin !== undefined) for (const joint of skins.get(skin)?.joints ?? []) add(joint);
  }
  return needed;
}

/** 🧬️ The ids of the solids of a scene: the procedural solids of its nodes, then the provenance ranges of the meshes of its nodes; first occurrence order, no repeats. */
export function solidsOfScene(document: ModelDocument, sceneId: Slug): readonly Slug[] {
  const scene = find(document, sceneId);
  const members = new Set(scene.nodes);
  const out: Slug[] = [];
  const seen = new Set<Slug>();
  const push = (id: Slug): void => { if (!seen.has(id)) { seen.add(id); out.push(id); } };
  for (const solid of document.solids) if (members.has(solid.node)) push(solid.id);
  const meshes = new Map(document.meshes.map((m) => [m.id, m]));
  for (const node of document.nodes) {
    if (!members.has(node.id) || node.mesh === undefined) continue;
    for (const p of meshes.get(node.mesh)?.primitives ?? []) for (const range of p.solids ?? []) push(range.id);
  }
  return out;
}

/** 🎨️ The ids of the materials a scene draws: those of its solids and primitives, each with the targets of the variant bindings that start there. */
export function materialsOfScene(document: ModelDocument, sceneId: Slug): ReadonlySet<Slug> {
  const scene = find(document, sceneId);
  const members = new Set(scene.nodes);
  const used = new Set<Slug>();
  for (const solid of document.solids) if (members.has(solid.node)) used.add(solid.material);
  const meshes = new Map(document.meshes.map((m) => [m.id, m]));
  for (const node of document.nodes) if (members.has(node.id) && node.mesh !== undefined) for (const p of meshes.get(node.mesh)?.primitives ?? []) used.add(p.material);
  for (const variant of document.variants) for (const [from, to] of Object.entries(variant.bindings)) if (used.has(from)) used.add(to);
  return used;
}

/** ✂️ One scene as a self-contained document: `scenes` is that scene alone (an implicit scene stays implicit and returns the document itself); foreign solids, meshes and skins are dropped, shared skeleton, materials, variants and clips stay. Throws {@link UnknownSceneError}. */
export function selectScene(document: ModelDocument, sceneId: Slug): ModelDocument {
  const scene = find(document, sceneId);
  if (document.scenes === undefined || document.scenes.length === 0) return document;
  const needed = nodesOfScene(document, sceneId);
  const members = new Set(scene.nodes);
  const nodes = document.nodes.filter((n) => needed.has(n.id)).map((n): Node => {
    if (members.has(n.id) || (n.mesh === undefined && n.skin === undefined)) return n;
    const { mesh: _mesh, skin: _skin, ...rest } = n;
    return rest;
  });
  const meshIds = new Set(nodes.flatMap((n) => (n.mesh === undefined ? [] : [n.mesh])));
  const skinIds = new Set(nodes.flatMap((n) => (n.skin === undefined ? [] : [n.skin])));
  const animations = document.animations.map((a): Animation => ({ ...a, channels: a.channels.filter((c) => !("node" in c.target) || needed.has(c.target.node)) }));
  const poses = document.poses.map((p): Pose => ({ ...p, nodes: Object.fromEntries(Object.entries(p.nodes).filter(([id]) => needed.has(id))) }));
  return {
    ...document,
    nodes,
    scenes: [scene],
    solids: document.solids.filter((s) => members.has(s.node)),
    meshes: document.meshes.filter((m) => meshIds.has(m.id)),
    skins: document.skins.filter((s) => skinIds.has(s.id)),
    animations,
    poses,
  };
}
