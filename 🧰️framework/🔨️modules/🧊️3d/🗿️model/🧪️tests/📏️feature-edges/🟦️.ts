/** 📏️ Feature edges against three's EdgesGeometry on the same triangles and the fixture's exact counts. */
import { EdgesGeometry } from "three";
import { describe, expect, it } from "vitest";
import { featureEdges, tessellate } from "../../🟦️.ts";
import { fixture, mergedGeometry } from "../🧰️support/🟦️.ts";

const data = fixture("📏️feature-edges");

describe("featureEdges", () => {
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
