/** 🧬️ Solid provenance: every resolved primitive lists which solid produced which index range (contiguous, in solid order, covering all indices); the counts come from three's BoxGeometry, the feature-edge ranges from three's EdgesGeometry of each solid's own triangles. */
import { BoxGeometry, BufferAttribute, BufferGeometry, EdgesGeometry } from "three";
import { describe, expect, it } from "vitest";
import { featureEdges, tessellate, validateModel } from "../../🟦️.ts";
import type { ModelDocument, Primitive } from "../../🧬️schema/🟦️.ts";
import { fixture, validateSchema } from "../🧰️support/🟦️.ts";

const data = fixture("🧬️provenance");
const boxIndices = new BoxGeometry(1, 1, 1).getIndex()!.count;

const summary = (document: ModelDocument, mode: "triangles" | "lines") =>
  Object.fromEntries(document.meshes.map((m) => [m.id, m.primitives.filter((p) => p.mode === mode).map((p) => ({ material: p.material, solids: p.solids }))]));

describe("tessellate provenance", () => {
  it("lists the solid of every index range, per node mesh, material and the skinned mesh", async () => {
    expect(boxIndices).toBe(data.expected.boxIndices);
    const resolved = await tessellate(data.document, data.quality);
    expect(summary(resolved, "triangles")).toEqual(data.expected.meshes);
    expect(validateModel(resolved).filter((i) => i.severity === "error")).toEqual([]);
    expect(validateSchema(resolved), JSON.stringify(validateSchema.errors)).toBe(true);
  });

  it("covers every primitive contiguously in solid order", async () => {
    const resolved = await tessellate(data.document, data.quality);
    const order = data.document.solids.map((s: { id: string }) => s.id);
    for (const primitive of resolved.meshes.flatMap((m) => m.primitives)) {
      let cursor = 0, last = -1;
      for (const range of primitive.solids!) {
        expect(range.first).toBe(cursor);
        expect(range.count % 3).toBe(0);
        expect(order.indexOf(range.id)).toBeGreaterThan(last);
        last = order.indexOf(range.id);
        cursor += range.count;
      }
      expect(cursor).toBe(primitive.indices.length);
    }
  });

  it("is deterministic", async () => {
    expect(JSON.stringify(await tessellate(data.document, data.quality))).toBe(JSON.stringify(await tessellate(data.document, data.quality)));
  });
});

describe("featureEdges provenance", () => {
  it("keeps the solid ranges of the line primitives", async () => {
    const edged = await featureEdges(await tessellate(data.document, data.quality), (data.angleDeg * Math.PI) / 180);
    expect(summary(edged, "lines")).toEqual(data.expected.edges);
    expect(validateModel(edged).filter((i) => i.severity === "error")).toEqual([]);
  });

  it("agrees with three's EdgesGeometry on each solid's own triangles", async () => {
    const resolved = await tessellate(data.document, data.quality);
    const edged = await featureEdges(resolved, (data.angleDeg * Math.PI) / 180);
    for (const mesh of edged.meshes) for (const lines of mesh.primitives.filter((p) => p.mode === "lines")) {
      const source = resolved.meshes.find((m) => m.id === mesh.id)!.primitives.find((p) => p.mode === "triangles" && p.material === lines.material && p.joints === undefined === (lines.joints === undefined))!;
      for (const range of lines.solids!) {
        const own = source.solids!.find((s) => s.id === range.id)!;
        const g = new BufferGeometry();
        g.setAttribute("position", new BufferAttribute(Float64Array.from(source.positions), 3));
        g.setIndex(source.indices.slice(own.first, own.first + own.count) as number[]);
        expect(range.count / 2).toBe(new EdgesGeometry(g, data.angleDeg).getAttribute("position").count / 2);
        expect(range.count / 2).toBe(data.expected.boxEdgeSegments * (own.count / data.expected.boxIndices));
      }
    }
  });

  it("gives primitives without provenance lines without provenance", async () => {
    const resolved = await tessellate(data.document, data.quality);
    const stripped: ModelDocument = { ...resolved, meshes: resolved.meshes.map((m) => ({ ...m, primitives: m.primitives.map(({ solids: _, ...rest }: Primitive) => rest) })) };
    const edged = await featureEdges(stripped, (data.angleDeg * Math.PI) / 180);
    for (const line of edged.meshes.flatMap((m) => m.primitives).filter((p) => p.mode === "lines")) expect(line.solids).toBeUndefined();
  });
});
