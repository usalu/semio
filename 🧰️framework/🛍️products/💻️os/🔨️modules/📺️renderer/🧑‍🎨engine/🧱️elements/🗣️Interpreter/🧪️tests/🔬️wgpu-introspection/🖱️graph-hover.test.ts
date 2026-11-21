import { expect, test } from "bun:test";
import { z } from "zod";
import fixture from "../🧫️fixtures/🖱️wgpu-graph-hover.json";

const portHover = z.object({
  nodeId: z.string(),
  portId: z.string(),
  portIds: z.array(z.string()).optional(),
  hoverSourceWindowId: z.string(),
});

test("Zod preserves canonical graph hover and omits its cleared wire field", () => {
  for (const row of fixture.surfaces) {
    const graphHover = portHover.nullable().parse(row.hover) ?? undefined;
    const projected = JSON.parse(JSON.stringify({ graphHover }));
    expect(projected.graphHover ?? null, row.name).toEqual(row.hover);
    expect(Object.hasOwn(projected, "graphHover"), row.name).toBe(row.hover !== null);
  }
});
