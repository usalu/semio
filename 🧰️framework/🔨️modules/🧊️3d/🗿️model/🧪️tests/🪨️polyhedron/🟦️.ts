/** 🪨️ Authored polyhedra preserve indexed topology, signed volume and area-weighted normals. */
import { readFileSync } from "node:fs";
import { BufferGeometry, Euler, Float32BufferAttribute, Matrix4, Vector3 } from "three";
import { describe, expect, it } from "vitest";
import { meshIntegrity, polyhedronExtents, polyhedronFault, tessellateSolid, type PolyhedronSolid } from "../../🟦️.ts";
import { signedVolume } from "../🧰️support/🟦️.ts";

const data = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🪨️polyhedron/🔣️.json", import.meta.url), "utf8"));
const solid: PolyhedronSolid = { id: "fold", node: "stem", material: "leaf", kind: "polyhedron", vertices: data.vertices, triangles: data.triangles };

describe("indexed polyhedra", () => {
  it("retains the authored winding and agrees with Three.js vertex normals", () => {
    expect(polyhedronFault(solid.vertices, solid.triangles)).toBeNull();
    const mesh = tessellateSolid(solid, { chordM: .1, minSegments: 4, maxSegments: 8 });
    expect(mesh.indices).toEqual(data.triangles.flat());
    expect(meshIntegrity(mesh.positions, mesh.indices).signedVolume).toBeCloseTo(data.signedVolume, 10);
    const reference = new BufferGeometry();
    reference.setAttribute("position", new Float32BufferAttribute(data.vertices.flat(), 3));
    reference.setIndex(data.triangles.flat());
    expect(signedVolume(reference)).toBeCloseTo(data.signedVolume, 10);
    reference.computeVertexNormals();
    Array.from(reference.getAttribute("normal").array).forEach((n, i) => expect(mesh.normals[i]).toBeCloseTo(n, 6));
  });
  it("retains outward winding when the bone frame mirrors and repeats an indexed solid", () => {
    const pair = data.mirroredPair;
    const mesh = tessellateSolid({ ...solid, transform: { scale: pair.scale, translation: pair.translation }, repeat: { count: 2, translation: pair.repeatOffset } }, { chordM: .1, minSegments: 4, maxSegments: 8 });
    const measured = meshIntegrity(mesh.positions, mesh.indices);
    expect(measured.components).toBe(pair.expectedComponents);
    expect(measured.signedVolume).toBeCloseTo(pair.expectedVolume, 10);
    const geometry = new BufferGeometry();
    geometry.setAttribute("position", new Float32BufferAttribute(data.vertices.flat(), 3));
    geometry.setIndex(data.triangles.flat());
    geometry.applyMatrix4(new Matrix4().makeScale(pair.scale[0], pair.scale[1], pair.scale[2]));
    expect(2 * Math.abs(signedVolume(geometry))).toBeCloseTo(measured.signedVolume, 8);
  });

  it("measures a thin panel in its own frame after a compound Three.js rotation", () => {
    const { size, translation, eulerRadians, extentsAscending } = data.thinPanel;
    const rotation = new Euler(eulerRadians[0], eulerRadians[1], eulerRadians[2]);
    const vertices = Array.from({ length: 8 }, (_, i) =>
      new Vector3(i & 1 ? size[0] : 0, i & 2 ? size[1] : 0, i & 4 ? size[2] : 0)
        .applyEuler(rotation).add(new Vector3(translation[0], translation[1], translation[2])).toArray() as [number, number, number],
    );
    const result = polyhedronExtents(vertices, [0, 2, 3]);
    result.forEach((v, i) => expect(v).toBeCloseTo(extentsAscending[i], 11));
  });
  it("preserves the signed volume of an inward dimple", () => {
    const { vertices, triangles, signedVolume: expected } = data.concave;
    expect(polyhedronFault(vertices, triangles)).toBeNull();
    const geometry = new BufferGeometry();
    geometry.setAttribute("position", new Float32BufferAttribute(vertices.flat(), 3));
    geometry.setIndex(triangles.flat());
    expect(signedVolume(geometry)).toBeCloseTo(expected, 7);
    expect(meshIntegrity(vertices.flat(), triangles.flat()).signedVolume).toBeCloseTo(expected, 10);
  });
  for (const c of data.invalid) it(c.name, () => expect(polyhedronFault(c.vertices ?? data.vertices, c.triangles)).not.toBeNull());
});
