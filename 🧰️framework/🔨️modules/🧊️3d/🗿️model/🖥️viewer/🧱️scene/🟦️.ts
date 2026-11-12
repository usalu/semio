/** 🧱️ `sceneOf`: a resolved `ModelDocument` becomes one interleaved vertex buffer (position, normal, colour), one skinning buffer, one global index buffer, primitives per node instance, skins with inverse bind matrices and variant material maps.
 *
 * Geometry shared by several nodes (one mesh, many nodes) is stored once. Missing normals are computed area-weighted, missing colours are white, skin weights are renormalised. Procedural documents (solids) must be tessellated first.
 * @see ../🧬️contract/🟦️.ts */
import { Pace, silentOperation } from "../../⏱️operation/🟦️.ts";
import type { ModelDocument, Operation, Primitive } from "../../🧬️schema/🟦️.ts";
import { SKIN_STRIDE, VERTEX_STRIDE, type Bounds, type Scene, type SceneMaterial, type ScenePrimitive, type SceneSkin } from "../🧬️contract/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

/** 🛑️ Thrown when a document cannot be drawn: unresolved solids, dangling references, malformed buffers. */
export class SceneError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "SceneError";
  }
}

type Geometry = { vertexStart: number; vertexCount: number; indexStart: number; indexCount: number; bounds: Bounds; skinned: boolean };

const fail = (message: string): never => {
  throw new SceneError(message);
};

function checkPrimitive(p: Primitive, where: string, pace: Pace): number {
  const n = p.positions.length / 3;
  if (!Number.isInteger(n)) fail(`${where}: positions is not a multiple of 3`);
  const per = p.mode === "lines" ? 2 : 3;
  if (p.indices.length % per !== 0) fail(`${where}: indices is not a multiple of ${per}`);
  for (const i of p.indices) {
    pace.tick();
    if (!Number.isInteger(i) || i < 0 || i >= n) fail(`${where}: index ${i} out of range`);
  }
  if (p.normals && p.normals.length !== n * 3) fail(`${where}: normals length`);
  if (p.colors && p.colors.length !== n * 4) fail(`${where}: colors length`);
  if ((p.joints && p.joints.length !== n * 4) || (p.weights && p.weights.length !== n * 4)) fail(`${where}: joints/weights length`);
  return n;
}

function computeNormals(p: Primitive, out: Float32Array, offset: number, pace: Pace): void {
  const n = p.positions.length / 3;
  const acc = new Float64Array(n * 3);
  if (p.mode === "triangles") {
    for (let t = 0; t < p.indices.length; t += 3) {
      pace.tick();
      const a = p.indices[t]! * 3, b = p.indices[t + 1]! * 3, c = p.indices[t + 2]! * 3;
      const ux = p.positions[b]! - p.positions[a]!, uy = p.positions[b + 1]! - p.positions[a + 1]!, uz = p.positions[b + 2]! - p.positions[a + 2]!;
      const vx = p.positions[c]! - p.positions[a]!, vy = p.positions[c + 1]! - p.positions[a + 1]!, vz = p.positions[c + 2]! - p.positions[a + 2]!;
      const nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
      for (const i of [a, b, c]) {
        acc[i] += nx;
        acc[i + 1] += ny;
        acc[i + 2] += nz;
      }
    }
  }
  for (let v = 0; v < n; v++) {
    pace.tick();
    const l = portable.hypot(acc[v * 3]!, acc[v * 3 + 1]!, acc[v * 3 + 2]!);
    const o = offset + v * VERTEX_STRIDE + 3;
    if (l > 1e-20) {
      out[o] = acc[v * 3]! / l;
      out[o + 1] = acc[v * 3 + 1]! / l;
      out[o + 2] = acc[v * 3 + 2]! / l;
    } else {
      out[o] = 0;
      out[o + 1] = 0;
      out[o + 2] = 1;
    }
  }
}

/** 🧬️ Generator form of {@link sceneOf}: yields between primitives and every few thousand vertices or indices inside one, so a caller can hand the event loop back and a cancellation is seen promptly; returns the scene. */
export function* buildScene(document: ModelDocument, operation: Operation = silentOperation): Generator<void, Scene> {
  const pace = new Pace(operation);
  if (document.solids.length > 0) fail("document carries solids; tessellate it first");
  const nodeIds = document.nodes.map((n) => n.id);
  const nodeIndex = new Map(nodeIds.map((id, i) => [id, i]));
  const materials: SceneMaterial[] = document.materials.map((m) => ({ id: m.id, baseColor: m.baseColor, emissive: m.emissive, metallic: m.metallic, roughness: m.roughness, unlit: m.unlit, doubleSided: m.doubleSided, blend: m.alphaMode === "blend" }));
  const materialIndex = new Map(materials.map((m, i) => [m.id, i]));
  const meshes = new Map(document.meshes.map((m) => [m.id, m]));
  const skinIndex = new Map(document.skins.map((s, i) => [s.id, i]));

  const skins: SceneSkin[] = [];
  let jointCount = 0;
  document.skins.forEach((s) => {
    const jointNodes = Int32Array.from(s.joints.map((j) => nodeIndex.get(j) ?? fail(`skin ${s.id}: unknown joint ${j}`)));
    if (s.inverseBind.length !== s.joints.length) fail(`skin ${s.id}: inverseBind length`);
    const inverseBind = new Float32Array(s.joints.length * 16);
    s.inverseBind.forEach((m, i) => inverseBind.set(m as readonly number[], i * 16));
    skins.push({ id: s.id, jointNodes, inverseBind, paletteOffset: jointCount });
    jointCount += s.joints.length;
  });

  let vertexTotal = 0, indexTotal = 0;
  const used = new Set<string>();
  for (const node of document.nodes) {
    if (!node.mesh) continue;
    const mesh = meshes.get(node.mesh) ?? fail(`node ${node.id}: unknown mesh ${node.mesh}`);
    if (node.skin && !skinIndex.has(node.skin)) fail(`node ${node.id}: unknown skin ${node.skin}`);
    mesh.primitives.forEach((p, i) => {
      if (!materialIndex.has(p.material)) fail(`mesh ${mesh.id}: unknown material ${p.material}`);
      const key = `${mesh.id}\u0000${i}`;
      if (used.has(key)) return;
      used.add(key);
      vertexTotal += checkPrimitive(p, `mesh ${mesh.id} primitive ${i}`, pace);
      indexTotal += p.indices.length;
    });
  }
  const anySkin = document.nodes.some((n) => n.mesh && n.skin && meshes.get(n.mesh)!.primitives.some((p) => p.joints && p.weights));
  const vertices = new Float32Array(vertexTotal * VERTEX_STRIDE);
  const skinning = anySkin ? new Float32Array(vertexTotal * SKIN_STRIDE) : null;
  const indices = new Uint32Array(indexTotal);
  const geometry = new Map<string, Geometry>();
  const primitives: ScenePrimitive[] = [];
  let vertexCursor = 0, indexCursor = 0, done = 0;
  const work = Math.max(1, document.nodes.filter((n) => n.mesh).length);

  for (const node of document.nodes) {
    if (!node.mesh) continue;
    pace.step(done++ / work);
    const mesh = meshes.get(node.mesh)!;
    const skin = node.skin ? skinIndex.get(node.skin)! : -1;
    for (let i = 0; i < mesh.primitives.length; i++) {
      const p = mesh.primitives[i]!;
      const key = `${mesh.id}\u0000${i}`;
      let g = geometry.get(key);
      if (!g) {
        const n = p.positions.length / 3;
        const min: [number, number, number] = [Infinity, Infinity, Infinity], max: [number, number, number] = [-Infinity, -Infinity, -Infinity];
        for (let v = 0; v < n; v++) {
          pace.tick();
          if ((v & 4095) === 4095) yield;
          const o = (vertexCursor + v) * VERTEX_STRIDE;
          for (let k = 0; k < 3; k++) {
            const x = p.positions[v * 3 + k]!;
            vertices[o + k] = x;
            if (x < min[k]!) min[k] = x;
            if (x > max[k]!) max[k] = x;
          }
          for (let k = 0; k < 4; k++) vertices[o + 6 + k] = p.colors ? p.colors[v * 4 + k]! : 1;
          if (p.normals) for (let k = 0; k < 3; k++) vertices[o + 3 + k] = p.normals[v * 3 + k]!;
        }
        if (!p.normals) computeNormals(p, vertices, vertexCursor * VERTEX_STRIDE, pace);
        const skinned = skinning !== null && p.joints !== undefined && p.weights !== undefined;
        if (skinned) {
          for (let v = 0; v < n; v++) {
            pace.tick();
            const o = (vertexCursor + v) * SKIN_STRIDE;
            let sum = 0;
            for (let k = 0; k < 4; k++) sum += p.weights![v * 4 + k]!;
            for (let k = 0; k < 4; k++) {
              skinning![o + k] = p.joints![v * 4 + k]!;
              skinning![o + 4 + k] = sum > 0 ? p.weights![v * 4 + k]! / sum : k === 0 ? 1 : 0;
            }
          }
        }
        for (let k = 0; k < p.indices.length; k++) {
          pace.tick();
          indices[indexCursor + k] = p.indices[k]! + vertexCursor;
        }
        g = { vertexStart: vertexCursor, vertexCount: n, indexStart: indexCursor, indexCount: p.indices.length, bounds: n > 0 ? { min, max } : { min: [0, 0, 0], max: [0, 0, 0] }, skinned };
        geometry.set(key, g);
        vertexCursor += n;
        indexCursor += p.indices.length;
      }
      primitives.push({
        index: primitives.length, node: node.id, nodeIndex: nodeIndex.get(node.id)!, mesh: mesh.id, primitive: i, mode: p.mode, material: p.material, materialIndex: materialIndex.get(p.material)!,
        skin: g.skinned ? skin : -1, vertexStart: g.vertexStart, vertexCount: g.vertexCount, indexStart: g.indexStart, indexCount: g.indexCount, bounds: g.bounds, tags: node.tags ?? [], solids: p.solids ?? [],
      });
      yield;
    }
  }

  const variants = new Map<string, Int32Array>();
  for (const v of document.variants) {
    const map = Int32Array.from(materials.map((_, i) => i));
    for (const [from, to] of Object.entries(v.bindings)) {
      const f = materialIndex.get(from), t = materialIndex.get(to);
      if (f !== undefined && t !== undefined) map[f] = t;
    }
    variants.set(v.id, map);
  }
  operation.report(1);
  return { document, nodeIds, nodeIndex, materials, materialIndex, variants, vertices, skinning, indices, primitives, skins, jointCount };
}

/** 📥️ Synchronous scene preparation; progress through `operation.report`, cancellation throws the kernel's `CancelledError`. */
export function sceneOf(document: ModelDocument, operation: Operation = silentOperation): Scene {
  const it = buildScene(document, operation);
  for (let r = it.next(); ; r = it.next()) if (r.done) return r.value;
}

/** ⏳️ Asynchronous scene preparation that hands the event loop back every ~12 ms, so large documents never freeze the page; same progress and cancellation as {@link sceneOf}. */
export async function sceneOfAsync(document: ModelDocument, operation: Operation = silentOperation): Promise<Scene> {
  const it = buildScene(document, operation);
  let clock = Date.now();
  for (let r = it.next(); ; r = it.next()) {
    if (r.done) return r.value;
    if (Date.now() - clock > 12) {
      await new Promise<void>((resolve) => setTimeout(resolve, 0));
      clock = Date.now();
    }
  }
}
