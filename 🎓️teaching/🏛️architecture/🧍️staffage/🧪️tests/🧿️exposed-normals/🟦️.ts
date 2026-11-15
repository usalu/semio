/** 🧿️ The exposed area of a solid judged with outward normals: an inside-out closed solid, a loft with its caps off and a solid that touches another exactly give the area of the body that is really exposed, whatever the winding of their triangles, and `isClosed` is the kernel's closedness.
 *
 * THE REFERENCE is trimesh (`🐍️.py`): the area of the target's faces whose probe along trimesh's own normal of the outward-wound twin lies in no other mesh (ray parity `contains`), plus hand-computed areas (cubes, tube).
 *
 * @see ../../🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts — `exposedArea`, `orientationOf`, `isClosed` */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { tessellateSolid } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { exposedArea, isClosed } from "../../🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts";
import type { World } from "../../🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { STAFFAGE_ROOT, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

const QUALITY = { chordM: 0.0005, minSegments: 64, maxSegments: 64 };
const solid = (extra: Record<string, unknown>) => ({ id: "s", node: "n", material: "m", ...extra }) as never;
const IDENTITY = { u: { x: 1, y: 0, z: 0 }, v: { x: 0, y: 1, z: 0 }, w: { x: 0, y: 0, z: 1 } };
const world = (id: string, g: { positions: number[]; indices: number[] }, options: { inside?: boolean; mirror?: boolean } = {}): World => {
  const positions = Float64Array.from(g.positions);
  const indices = Uint32Array.from(g.indices);
  if (options.inside) for (let i = 0; i < indices.length; i += 3) [indices[i + 1], indices[i + 2]] = [indices[i + 2]!, indices[i + 1]!];
  return { id, positions, indices, ...(options.mirror ? { frame: { u: { x: -1, y: 0, z: 0 }, v: IDENTITY.v, w: IDENTITY.w } } : {}) };
};
const box = (centre: number[], size: number[]) => tessellateSolid(solid({ kind: "box", size, transform: { translation: centre } }), QUALITY);
const tube = (z0: number, z1: number, caps: boolean) => tessellateSolid(solid({ kind: "loft", sections: [{ at: [0, 0, z0], rx: 0.1, ry: 0.1 }, { at: [0, 0, z1], rx: 0.1, ry: 0.1 }], capStart: caps, capEnd: caps }), QUALITY);
const cylinder = () => tessellateSolid(solid({ kind: "revolve", profile: [[0, 0], [0, 0.1], [1, 0.1], [1, 0]], axis: [0, 0, 1], origin: [0, 0, 0] }), QUALITY);
const mirrored = (g: { positions: number[]; indices: number[] }) => ({ positions: g.positions.map((v, i) => (i % 3 === 0 ? -v : v)), indices: g.indices });

type Case = { name: string; target: { positions: number[]; indices: number[] }; reference: { positions: number[]; indices: number[] }; against: { positions: number[]; indices: number[] }[]; targets: World[]; againstWorlds: World[]; expected: number };
const lateral = 2 * Math.PI * 0.1 * 1;
const cases: Case[] = [];
const cubeA = box([0.5, 0.5, 0.5], [1, 1, 1]);
const cubeB = box([1, 0.5, 0.5], [1, 1, 1]);
cases.push({ name: "inside-out closed cube half covered by an outward cube", target: cubeA, reference: cubeA, against: [cubeB], targets: [world("a", cubeA, { inside: true })], againstWorlds: [world("b", cubeB)], expected: 5 });
cases.push({ name: "two cubes touching exactly, one inside-out", target: cubeA, reference: cubeA, against: [box([1.5, 0.5, 0.5], [1, 1, 1])], targets: [world("a", cubeA, { inside: true })], againstWorlds: [world("b", box([1.5, 0.5, 0.5], [1, 1, 1]))], expected: 5 });
cases.push({ name: "open loft skin lying exactly on a solid cylinder", target: tube(0, 1, false), reference: tube(0, 1, false), against: [cylinder()], targets: [world("t", tube(0, 1, false))], againstWorlds: [world("c", cylinder())], expected: lateral });
cases.push({ name: "open loft skin in a left-handed frame (mirrored mesh, frame says so)", target: tube(0, 1, false), reference: tube(0, 1, false), against: [cylinder()], targets: [world("t", mirrored(tube(0, 1, false)), { mirror: true })], againstWorlds: [world("c", mirrored(cylinder()), { mirror: true })], expected: lateral });
cases.push({ name: "open loft skin half inside a box", target: tube(0, 1, false), reference: tube(0, 1, false), against: [box([0, 0, 0.25], [1, 1, 0.5])], targets: [world("t", tube(0, 1, false))], againstWorlds: [world("b", box([0, 0, 0.25], [1, 1, 0.5]))], expected: lateral / 2 });

describe("🧿️ exposedArea with outward normals", () => {
  const directory = mkdtempSync(join(tmpdir(), "staffage-exposed-"));
  let reference: Record<string, { exposedArea: number; totalArea: number; outwardVolume: number }>;
  it("trimesh gives the exposed area of the outward-wound twins", { timeout: 300_000 }, () => {
    requireTool("uv");
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ cases: cases.map((c) => ({ name: c.name, target: c.reference, against: c.against, eps: 1e-3 })) }));
      reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🧿️exposed-normals/🐍️.py"), "geometry", [input]));
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
    for (const c of cases) expect(reference[c.name]!.exposedArea, c.name).toBeCloseTo(c.expected, 2);
  });
  for (const c of cases) {
    it(`${c.name}: ${c.expected.toFixed(4)} m2, as trimesh and by hand`, () => {
      expect(exposedArea(c.targets, c.againstWorlds, [], () => false, 1e-3)).toBeCloseTo(c.expected, 2);
    });
  }
  it("isClosed is true for solids and false for an open skin", () => {
    expect(isClosed(world("c", cylinder()))).toBe(true);
    expect(isClosed(world("t", tube(0, 1, false)))).toBe(false);
    expect(isClosed(world("t", tube(0, 1, true)))).toBe(true);
  });
});
