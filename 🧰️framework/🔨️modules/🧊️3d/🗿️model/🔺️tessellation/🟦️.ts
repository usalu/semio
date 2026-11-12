/** 🔺️ Tessellation of procedural solids into resolved meshes: transform, repeat copies, rigid or weighted binding, deterministic output, CCW outward winding.
 *
 * Per solid: raw geometry in the node frame, then `transform` (scale, rotation, translation), then the k-th repeat copy applies the step `translate(translation) * rotateAbout(origin, rotationAxis, rotationRad)` k times.
 * Rigid solids (no `weights`) become primitives of one mesh per node (mesh id = node id, merged per material) in node-local coordinates. Weighted solids are brought into model space with the rest world matrix of their node, blended uniformly
 * over at most four joints, and gathered in one skin (`skin`) on one mesh (`skinned`) carried by a new identity root node (`skinned`), with inverse bind matrices of the rest pose.
 * Every primitive lists in `solids` which solid produced which contiguous index range (solid order, covering all indices).
 * @see ../🧬️schema/🟦️.ts */
import type { Mesh, ModelDocument, Node, Operation, Primitive, PrimitiveSolid, Quality, Skin, Slug, Solid } from "../🧬️schema/🟦️.ts";
import { compose, determinant3, identity4, invert4, multiply4, normalMatrix, qAxisAngle, transformNormal, transformPoint, worldMatrices, type M4, type V3 } from "../🧮️math/🟦️.ts";
import { CancelledError, Pace, silentOperation } from "../⏱️operation/🟦️.ts";
import { validateModel } from "../✅️validation/🟦️.ts";
import { type Geometry } from "./🧱️builder/🟦️.ts";
import { tessellateBox } from "./📦️box/🟦️.ts";
import { tessellateExtrude } from "./🏗️extrude/🟦️.ts";
import { tessellateLoft } from "./🧵️loft/🟦️.ts";
import { tessellateCapsule, tessellateEllipsoid, tessellateRevolve } from "./🌀️revolved/🟦️.ts";

/** 🩺️ Thrown by `tessellate` when `validateModel` reports errors. */
export class ModelInvalidError extends Error {
  constructor(readonly issues: readonly import("../🧬️schema/🟦️.ts").Issue[]) {
    super(`invalid model: ${issues.filter((i) => i.severity === "error").map((i) => `${i.code} at ${i.path}`).join("; ")}`);
    this.name = "ModelInvalidError";
  }
}

export { CancelledError, silentOperation };

/** 🌍️ World matrices of the nodes at their rest transforms, keyed by node id. */
export function restWorldMatrices(nodes: readonly Node[]): Map<Slug, M4> {
  return worldMatrices(nodes, (n) => compose(n.translation, n.rotation, n.scale));
}

/** 🧊️ Geometry of one solid in its node frame after `transform` and `repeat`; triangles stay CCW outward. */
export function tessellateSolid(solid: Solid, quality: Quality, extra: M4 = identity4(), operation: Operation = silentOperation): Geometry {
  const pace = new Pace(operation);
  const raw = ((): Geometry => {
    switch (solid.kind) {
      case "box": return tessellateBox(solid, quality, operation);
      case "ellipsoid": return tessellateEllipsoid(solid, quality, operation);
      case "capsule": return tessellateCapsule(solid, quality, operation);
      case "revolve": return tessellateRevolve(solid, quality, operation);
      case "extrude": return tessellateExtrude(solid, quality, operation);
      case "loft": return tessellateLoft(solid, quality, operation);
    }
  })();
  const t = solid.transform;
  const base = multiply4(extra, compose(t?.translation ?? [0, 0, 0], t?.rotation ?? [0, 0, 0, 1], t?.scale ?? [1, 1, 1]));
  const repeat = solid.repeat;
  const count = repeat?.count ?? 1;
  const origin: V3 = (repeat?.origin as V3 | undefined) ?? [0, 0, 0];
  const turn = compose(origin, qAxisAngle(repeat?.rotationAxis ?? [0, 1, 0], repeat?.rotationRad ?? 0), [1, 1, 1]);
  const unturn = compose([-origin[0], -origin[1], -origin[2]], [0, 0, 0, 1], [1, 1, 1]);
  const step = multiply4(compose(repeat?.translation ?? [0, 0, 0], [0, 0, 0, 1], [1, 1, 1]), multiply4(turn, unturn));
  const out: Geometry = { positions: [], normals: [], indices: [] };
  let copy = identity4();
  for (let k = 0; k < count; k++) {
    const m = multiply4(copy, base);
    const nm = normalMatrix(m);
    const flip = determinant3(m) < 0;
    const offset = out.positions.length / 3;
    for (let i = 0; i < raw.positions.length; i += 3) {
      pace.tick();
      out.positions.push(...transformPoint(m, [raw.positions[i]!, raw.positions[i + 1]!, raw.positions[i + 2]!]));
      out.normals.push(...transformNormal(nm, [raw.normals[i]!, raw.normals[i + 1]!, raw.normals[i + 2]!]));
    }
    for (let i = 0; i < raw.indices.length; i += 3) {
      pace.tick();
      const [a, b, c] = [raw.indices[i]! + offset, raw.indices[i + 1]! + offset, raw.indices[i + 2]! + offset];
      out.indices.push(a, flip ? c : b, flip ? b : c);
    }
    copy = multiply4(step, copy);
  }
  return out;
}

type Accumulator = { material: Slug; positions: number[]; normals: number[]; joints?: number[]; weights?: number[]; indices: number[]; solids: PrimitiveSolid[] };

const append = (acc: Accumulator, id: Slug, g: Geometry, pace: Pace, joints?: number[], weights?: number[]) => {
  pace.tick(g.positions.length + g.indices.length);
  const offset = acc.positions.length / 3;
  if (g.indices.length > 0) acc.solids.push({ id, first: acc.indices.length, count: g.indices.length });
  for (const v of g.positions) acc.positions.push(v);
  for (const v of g.normals) acc.normals.push(v);
  for (const i of g.indices) acc.indices.push(i + offset);
  if (joints && weights) {
    const vertices = g.positions.length / 3;
    for (let i = 0; i < vertices; i++) {
      acc.joints!.push(...joints);
      acc.weights!.push(...weights);
    }
  }
};

const primitiveOf = (acc: Accumulator): Primitive => ({
  mode: "triangles",
  material: acc.material,
  positions: acc.positions,
  normals: acc.normals,
  ...(acc.joints ? { joints: acc.joints, weights: acc.weights! } : {}),
  indices: acc.indices,
  solids: acc.solids,
});

const unique = (base: string, used: Set<string>): string => {
  let id = base, n = 2;
  while (used.has(id)) id = `${base}-${n++}`;
  used.add(id);
  return id;
};

const VALIDATION_SHARE = 0.05;
const yieldToLoop = () => new Promise<void>((resolve) => setTimeout(resolve, 0));

/** 🏭️ Turns every solid into primitives (see module docs); the result carries no solids. Rejects with {@link CancelledError} when cancelled (checked between solids, inside validation and inside every solid's tessellation) and {@link ModelInvalidError} when the document has errors. */
export async function tessellate(document: ModelDocument, quality: Quality, operation: Operation = silentOperation): Promise<ModelDocument> {
  const pace = new Pace(operation);
  const issues = validateModel(document, { report: (f) => operation.report(f * VALIDATION_SHARE), cancelled: () => operation.cancelled() });
  if (issues.some((i) => i.severity === "error")) throw new ModelInvalidError(issues);
  if (document.solids.length === 0) return document;
  const world = restWorldMatrices(document.nodes);
  const nodeIndex = new Map(document.nodes.map((n, i) => [n.id, i]));
  const rigid = new Map<Slug, Map<Slug, Accumulator>>();
  const skinned = new Map<Slug, Accumulator>();
  const jointOrder: Slug[] = [];
  let clock = Date.now();
  for (let i = 0; i < document.solids.length; i++) {
    if (operation.cancelled()) throw new CancelledError();
    operation.report(VALIDATION_SHARE + ((1 - VALIDATION_SHARE) * i) / document.solids.length);
    if (Date.now() - clock > 16) {
      await yieldToLoop();
      clock = Date.now();
      if (operation.cancelled()) throw new CancelledError();
    }
    const solid = document.solids[i]!;
    if (solid.weights && solid.weights.length > 0) {
      const live = solid.weights.filter((w) => w.weight > 0).slice(0, 4);
      const total = live.reduce((s, w) => s + w.weight, 0);
      const joints = live.map((w) => {
        let at = jointOrder.indexOf(w.node);
        if (at < 0) at = jointOrder.push(w.node) - 1;
        return at;
      });
      const weights = live.map((w) => w.weight / total);
      while (joints.length < 4) {
        joints.push(0);
        weights.push(0);
      }
      const acc = skinned.get(solid.material) ?? { material: solid.material, positions: [], normals: [], joints: [], weights: [], indices: [], solids: [] };
      skinned.set(solid.material, acc);
      append(acc, solid.id, tessellateSolid(solid, quality, world.get(solid.node)!, operation), pace, joints, weights);
    } else {
      const perNode = rigid.get(solid.node) ?? new Map<Slug, Accumulator>();
      rigid.set(solid.node, perNode);
      const acc = perNode.get(solid.material) ?? { material: solid.material, positions: [], normals: [], indices: [], solids: [] };
      perNode.set(solid.material, acc);
      append(acc, solid.id, tessellateSolid(solid, quality, identity4(), operation), pace);
    }
  }
  const usedMeshes = new Set(document.meshes.map((m) => m.id));
  const meshes: Mesh[] = [...document.meshes];
  const nodes: Node[] = document.nodes.map((n) => n);
  for (const [nodeId, perMaterial] of rigid) {
    const at = nodeIndex.get(nodeId)!;
    const node = nodes[at]!;
    const primitives = [...perMaterial.values()].map(primitiveOf);
    const existing = node.mesh ? meshes.findIndex((m) => m.id === node.mesh) : -1;
    if (existing >= 0) meshes[existing] = { ...meshes[existing]!, primitives: [...meshes[existing]!.primitives, ...primitives] };
    else {
      const id = unique(nodeId, usedMeshes);
      meshes.push({ id, primitives });
      nodes[at] = { ...node, mesh: id };
    }
  }
  const skins: Skin[] = [...document.skins];
  if (skinned.size > 0) {
    const usedSkins = new Set(skins.map((s) => s.id));
    const usedNodes = new Set(nodes.map((n) => n.id));
    const meshId = unique("skinned", usedMeshes), skinId = unique("skinned", usedSkins), nodeId = unique("skinned", usedNodes);
    const joints = jointOrder.slice();
    skins.push({ id: skinId, joints, inverseBind: joints.map((j) => invert4(world.get(j)!) as unknown as Skin["inverseBind"][number]) });
    meshes.push({ id: meshId, primitives: [...skinned.values()].map(primitiveOf) });
    nodes.push({ id: nodeId, parent: null, translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], mesh: meshId, skin: skinId });
  }
  operation.report(1);
  return { ...document, nodes, solids: [], meshes, skins };
}
