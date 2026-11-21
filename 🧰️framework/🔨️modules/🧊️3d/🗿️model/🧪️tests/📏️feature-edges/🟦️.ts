/** 📏️ Feature edges against three's EdgesGeometry on the same triangles and the fixture's exact counts. */
import { BufferGeometry, Float32BufferAttribute, EdgesGeometry } from "three";
import { describe, expect, it } from "vitest";
import { featureEdges, tessellate } from "../../🟦️.ts";
import { fixture, mergedGeometry } from "../🧰️support/🟦️.ts";

const data = fixture("📏️feature-edges");

describe("featureEdges", () => {
  for (const c of data.skinSeams.cases) {
    it(c.name, async () => {
      const { positions, indices, jointDisplacements } = data.skinSeams;
      const joints = [0, 0, 0, 1, 1, 1].flatMap((half) => c.joints[half]);
      const weights = [0, 0, 0, 1, 1, 1].flatMap((half) => c.weights[half]);
      const primitive = { mode: "triangles" as const, material: "test", positions, indices, joints, weights };
      const source = { ...data.cases[0].document, meshes: [{ id: "sheet", primitives: [primitive] }] };
      const displaced = (p: number[], j: number[], w: number[]) => p.map((v, i) => {
        if (i % 3 !== 2) return v;
        const k = 4 * Math.floor(i / 3);
        return v + w.slice(k, k + 4).reduce((sum, weight, slot) => sum + weight * jointDisplacements[j[k + slot]!], 0);
      });
      const geometry = new BufferGeometry();
      geometry.setAttribute("position", new Float32BufferAttribute(displaced(positions, joints, weights), 3));
      geometry.setIndex(indices);
      const expected = new EdgesGeometry(geometry, 30).getAttribute("position");
      expect(expected.count / 2).toBe(c.segments);
      for (const shareVertices of [false, true]) {
        const edged = await featureEdges(source, Math.PI / 6, { shareVertices });
        const lines = edged.meshes[0]!.primitives[1]!;
        expect(lines.indices.length).toBe(expected.count);
        const posed = displaced([...lines.positions], [...lines.joints!], [...lines.weights!]);
        const actualZ = lines.indices.map((v) => posed[v * 3 + 2]!).sort();
        expect(actualZ).toEqual(Array.from({ length: expected.count }, (_, i) => expected.getZ(i)).sort());
      }
    });
  }

  for (const c of data.cases) {
    it(c.name, async () => {
      const resolved = await tessellate(c.document, data.quality);
      const edged = await featureEdges(resolved, (c.angleDeg * Math.PI) / 180);
      const lines = edged.meshes.flatMap((m) => m.primitives).filter((p) => p.mode === "lines");
      const segments = lines.reduce((n, p) => n + p.indices.length / 2, 0);
      const oracle = new EdgesGeometry(mergedGeometry(resolved), c.angleDeg).getAttribute("position").count / 2;
      expect(segments).toBe(oracle);
      if (c.expectedSegments !== null) expect(segments).toBe(c.expectedSegments);
      expect(edged.meshes.flatMap((m) => m.primitives).filter((p) => p.mode === "triangles")).toEqual(resolved.meshes.flatMap((m) => m.primitives));
    });
  }

  it("names the material of each edge primitive by a function of its source primitive (same segments as a single material)", async () => {
    const c = data.cases[0];
    const resolved = await tessellate(c.document, data.quality);
    const shared = await featureEdges(resolved, (c.angleDeg * Math.PI) / 180, { material: "edge" });
    const named = await featureEdges(resolved, (c.angleDeg * Math.PI) / 180, { material: (source) => `${source.material}-edge` });
    const lines = (document: typeof shared) => document.meshes.flatMap((m) => m.primitives).filter((p) => p.mode === "lines");
    expect(lines(named).map((p) => p.indices.length)).toEqual(lines(shared).map((p) => p.indices.length));
    expect(lines(shared).every((p) => p.material === "edge")).toBe(true);
    const sources = resolved.meshes.flatMap((m) => m.primitives).map((p) => `${p.material}-edge`);
    expect(lines(named).map((p) => p.material)).toEqual(sources.slice(0, lines(named).length));
  });
});
