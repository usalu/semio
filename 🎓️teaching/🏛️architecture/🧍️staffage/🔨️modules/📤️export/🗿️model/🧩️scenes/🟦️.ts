/** 🧩️ A multi-scene model tessellated scene by scene, each at the quality of its own scale, and joined again into one resolved document.
 *
 * The kernel tessellates a document at one quality, or one scene of it (`tessellate(document, quality, operation, { scene })`, which returns that scene as a self-contained document). A model of one asset holds a scene per scale and mode (`s50-body`, `s100-thermal`, …) and a coarser scale is tessellated more coarsely (see `qualityOf`): {@link resolveScenes} tessellates every scene on its own and {@link joinScenes} puts the results back together:
 * rigid meshes keep their node ids (the scene-specific mesh nodes are unique), the skinned mesh, skin and node a scene gets are renamed `<scene>-<id>` so no two scenes collide, materials, variants, poses and clips of the source are kept, and every scene lists its resolved nodes.
 *
 * @see ../../../🧊️volume/⏱️job/🟦️.ts — `qualityOf`
 * @see ../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🎞️scenes/🟦️.ts — scenes
 */
import { type ModelDocument, type Node, type Operation, type Quality, type Scene, type Skin, type Mesh, type Slug, scenesOf, tessellate } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";

/** 🔢️ The denominator of a scene id `s<scale>-<mode>`; `undefined` for any other id. */
export function scaleOfScene(sceneId: Slug): number | undefined {
  const found = /^s(\d+)-/u.exec(sceneId);
  return found === null ? undefined : Number(found[1]);
}

/** 🧵️ Joins single-scene resolved parts of the procedural `source` into one resolved multi-scene document. */
export function joinScenes(source: ModelDocument, parts: readonly { readonly scene: Slug; readonly document: ModelDocument }[]): ModelDocument {
  const known = new Set(source.nodes.map((n) => n.id));
  const nodes = new Map<Slug, Node>(source.nodes.map((n) => [n.id, n]));
  const meshes: Mesh[] = [];
  const skins: Skin[] = [];
  const scenes: Scene[] = [];
  for (const { scene, document } of parts) {
    const renamed = new Map<Slug, Slug>();
    const added = document.nodes.filter((n) => !known.has(n.id));
    for (const node of added) renamed.set(node.id, `${scene}-${node.id}`);
    const meshRename = new Map<Slug, Slug>();
    const skinRename = new Map<Slug, Slug>();
    for (const node of added) {
      if (node.mesh !== undefined) meshRename.set(node.mesh, `${scene}-${node.mesh}`);
      if (node.skin !== undefined) skinRename.set(node.skin, `${scene}-${node.skin}`);
    }
    for (const node of document.nodes) {
      if (known.has(node.id)) {
        if (node.mesh !== undefined && !nodes.get(node.id)!.mesh) nodes.set(node.id, { ...nodes.get(node.id)!, mesh: node.mesh });
        continue;
      }
      nodes.set(renamed.get(node.id)!, { ...node, id: renamed.get(node.id)!, parent: node.parent === null ? null : (renamed.get(node.parent) ?? node.parent), ...(node.mesh === undefined ? {} : { mesh: meshRename.get(node.mesh) ?? node.mesh }), ...(node.skin === undefined ? {} : { skin: skinRename.get(node.skin) ?? node.skin }) });
    }
    for (const mesh of document.meshes) meshes.push({ ...mesh, id: meshRename.get(mesh.id) ?? mesh.id });
    for (const skin of document.skins) skins.push({ ...skin, id: skinRename.get(skin.id) ?? skin.id });
    const own = document.scenes![0]!;
    scenes.push({ ...own, nodes: own.nodes.map((n) => renamed.get(n) ?? n) });
  }
  const order = [...source.nodes.map((n) => n.id), ...[...nodes.keys()].filter((id) => !known.has(id))];
  return { ...source, nodes: order.map((id) => nodes.get(id)!), solids: [], meshes, skins, scenes };
}

/** 🏭️ The resolved document of a procedural multi-scene model: every scene tessellated at the quality `qualityOf(scale)` of its own scale, joined by {@link joinScenes}. Progress runs over the scenes; rejects with the kernel's `CancelledError` when cancelled. */
export async function resolveScenes(source: ModelDocument, qualityOf: (scale: number) => Quality, operation: Operation): Promise<ModelDocument> {
  const scenes = scenesOf(source);
  const parts: { scene: Slug; document: ModelDocument }[] = [];
  for (const [index, scene] of scenes.entries()) {
    const scale = scaleOfScene(scene.id);
    if (scale === undefined) throw new Error(`scene ${scene.id} has no scale`);
    const share = { report: (f: number) => operation.report((index + f) / scenes.length), cancelled: () => operation.cancelled() };
    parts.push({ scene: scene.id, document: await tessellate(source, qualityOf(scale), share, { scene: scene.id }) });
  }
  operation.report(1);
  return joinScenes(source, parts);
}
