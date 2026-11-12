/** 🧰️ Test helpers: loading fixtures and measuring meshes with three.js as the independent oracle (never imported by the kernel). */
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { Box3, BufferAttribute, BufferGeometry, Triangle, Vector3 } from "three";
import { mergeVertices } from "three/examples/jsm/utils/BufferGeometryUtils.js";
import type { ModelDocument, Primitive } from "../../🧬️schema/🟦️.ts";
import { directoryNear, readFixture } from "./🧫️reader/🟦️.ts";

export const fixture = <T = any>(name: string): T => readFixture<T>(directoryNear(import.meta.url, "../../🧫️fixtures"), name);

export const schema = (): any => JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));

/** 🧬️ The compiled validator of the model schema (Ajv is the independent oracle for the schema twin). */
export const validateSchema = new Ajv({ strict: false }).compile(schema());

export function geometryOf(primitive: Pick<Primitive, "positions" | "indices">): BufferGeometry {
  const g = new BufferGeometry();
  g.setAttribute("position", new BufferAttribute(Float64Array.from(primitive.positions), 3));
  g.setIndex([...primitive.indices]);
  return g;
}

export function trianglesOf(document: ModelDocument): Primitive[] {
  return document.meshes.flatMap((m) => m.primitives).filter((p) => p.mode === "triangles");
}

export function mergedGeometry(document: ModelDocument): BufferGeometry {
  const positions: number[] = [], indices: number[] = [];
  for (const p of trianglesOf(document)) {
    const offset = positions.length / 3;
    positions.push(...p.positions);
    indices.push(...p.indices.map((i) => i + offset));
  }
  return geometryOf({ positions, indices });
}

export function weldedGeometry(g: BufferGeometry, tolerance = 1e-6): BufferGeometry {
  const copy = g.clone();
  for (const name of Object.keys(copy.attributes)) if (name !== "position") copy.deleteAttribute(name);
  return mergeVertices(copy, tolerance);
}

export function signedVolume(g: BufferGeometry): number {
  const position = g.getAttribute("position");
  const index = g.getIndex();
  const at = (i: number) => (index ? index.getX(i) : i);
  const count = index ? index.count : position.count;
  const a = new Vector3(), b = new Vector3(), c = new Vector3();
  let volume = 0;
  for (let i = 0; i < count; i += 3) {
    a.fromBufferAttribute(position, at(i));
    b.fromBufferAttribute(position, at(i + 1));
    c.fromBufferAttribute(position, at(i + 2));
    volume += a.dot(b.clone().cross(c)) / 6;
  }
  return volume;
}

export function surfaceArea(g: BufferGeometry): number {
  const position = g.getAttribute("position");
  const index = g.getIndex();
  const at = (i: number) => (index ? index.getX(i) : i);
  const count = index ? index.count : position.count;
  const t = new Triangle();
  let area = 0;
  for (let i = 0; i < count; i += 3) {
    t.setFromAttributeAndIndices(position, at(i), at(i + 1), at(i + 2));
    area += t.getArea();
  }
  return area;
}

export function bounds(g: BufferGeometry): { min: number[]; max: number[] } {
  const box = new Box3().setFromBufferAttribute(g.getAttribute("position") as BufferAttribute);
  return { min: box.min.toArray(), max: box.max.toArray() };
}

/** 🕸️ Every undirected edge of the welded mesh is used exactly twice, once in each direction. */
export function isClosedOrientedManifold(welded: BufferGeometry): boolean {
  const index = welded.getIndex()!;
  const directed = new Map<string, number>();
  for (let i = 0; i < index.count; i += 3) {
    for (let k = 0; k < 3; k++) {
      const key = `${index.getX(i + k)}>${index.getX(i + (k + 1) % 3)}`;
      directed.set(key, (directed.get(key) ?? 0) + 1);
    }
  }
  for (const [key, count] of directed) {
    const [a, b] = key.split(">");
    if (count !== 1 || directed.get(`${b}>${a}`) !== 1) return false;
  }
  return true;
}

/** 🧭️ Number of triangles whose vertex normals disagree with the geometric face normal (dot below 0.0). */
export function badNormals(primitive: Primitive): number {
  const position = geometryOf(primitive).getAttribute("position");
  const t = new Triangle();
  const n = new Vector3(), face = new Vector3();
  let bad = 0;
  for (let i = 0; i < primitive.indices.length; i += 3) {
    const [a, b, c] = [primitive.indices[i]!, primitive.indices[i + 1]!, primitive.indices[i + 2]!];
    t.setFromAttributeAndIndices(position, a, b, c);
    t.getNormal(face);
    for (const v of [a, b, c]) {
      n.set(primitive.normals![v * 3]!, primitive.normals![v * 3 + 1]!, primitive.normals![v * 3 + 2]!);
      if (Math.abs(n.length() - 1) > 1e-6 || n.dot(face) <= 0) bad++;
    }
  }
  return bad;
}
