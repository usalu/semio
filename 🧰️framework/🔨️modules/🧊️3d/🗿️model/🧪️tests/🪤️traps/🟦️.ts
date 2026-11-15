/** 🪤️ The silent geometry traps of declarative solids: an open revolve profile is closed through the axis (a shell needs its first point repeated), a `taperTo` is [scale at from, scale at to] (equal factors are no taper), and lofts whose consecutive sections overlap fold. Classified by the kernel, reported as validation warnings, and the revolve closure measured with shapely (areas) and trimesh (the volume of the tessellation equals Pappus' volume of the axis-closed polygon, so the filled body is real). */
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { loftFault, revolveClosure, taperFault, tessellateSolid, validateModel } from "../../🟦️.ts";
import type { ModelDocument, Quality, Solid, Vec2 } from "../../🧬️schema/🟦️.ts";
import { fixture } from "../🧰️support/🟦️.ts";
import { pythonOracle } from "../🧰️support/🐍️oracle/🟦️.ts";

type Closure = { name: string; profile: Vec2[]; kind: string; filledAreaM2?: number; loopAreaM2?: number };
const traps = fixture<{ revolveClosure: Closure[]; taper: { name: string; taperTo: Vec2 | null; fault: string | null }[]; loftFold: { name: string; sections: any[]; fold: [number, number] | null }[] }>("🪤️traps");
const ORACLE = fileURLToPath(new URL("../🩻️integrity/🐍️.py", import.meta.url));
const QUALITY: Quality = { chordM: 0.0005, minSegments: 64, maxSegments: 128 };

const solid = (extra: Record<string, unknown>): Solid => ({ id: "s", node: "root", material: "m", ...extra }) as Solid;
const document = (s: Solid): ModelDocument => ({
  version: "semio.model/v1", id: "d", title: { en: "d", de: "d" },
  nodes: [{ id: "root", parent: null, translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1] }],
  solids: [s], meshes: [], skins: [],
  materials: [{ id: "m", title: { en: "m", de: "m" }, baseColor: [1, 1, 1, 1], metallic: 0, roughness: 1, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: false, unlit: false }],
  variants: [], poses: [], animations: [],
});
const codes = (s: Solid): string[] => validateModel(document(s)).map((i) => i.code);

const axisPolygon = (profile: Vec2[]): (readonly number[])[] => [[profile[0]![0], 0], ...profile, [profile[profile.length - 1]![0], 0]];

describe("🌀️ the revolve closing rule", () => {
  const measured = traps.revolveClosure.map((c) => ({ name: c.name, polygon: axisPolygon(c.profile), loop: c.profile.length >= 3 ? c.profile : undefined }));
  const meshes = traps.revolveClosure.filter((c) => c.kind !== "loop").map((c) => {
    const g = tessellateSolid(solid({ kind: "revolve", profile: c.profile, axis: [0, 1, 0], origin: [0, 0, 0] }), QUALITY);
    return { name: `mesh-${c.name}`, positions: g.positions, indices: g.indices, polygon: axisPolygon(c.profile) };
  });
  const oracle = pythonOracle<Record<string, { polygonArea?: number; pappusVolume?: number; loopArea?: number; volume?: number }>>(ORACLE, { cases: [...measured, ...meshes] });
  for (const c of traps.revolveClosure) {
    it(`${c.name}: ${c.kind}`, () => {
      const r = revolveClosure(c.profile);
      expect(r.kind).toBe(c.kind);
      if (c.filledAreaM2 !== undefined) expect(r.filledAreaM2).toBeCloseTo(c.filledAreaM2, 9);
      if (c.loopAreaM2 !== undefined) expect(r.loopAreaM2).toBeCloseTo(c.loopAreaM2, 9);
      if (c.kind !== "loop") expect(r.filledAreaM2, "shapely area of the polygon closed through the axis").toBeCloseTo(oracle[c.name]!.polygonArea!, 9);
      if (c.kind === "shell" || c.kind === "axis") {
        const volume = oracle[`mesh-${c.name}`]!.volume!;
        expect(Math.abs(volume - oracle[`mesh-${c.name}`]!.pappusVolume!) / volume, "trimesh volume of the tessellation equals Pappus of the axis-closed polygon").toBeLessThan(0.01);
      }
      expect(codes(solid({ kind: "revolve", profile: c.profile, axis: [0, 1, 0], origin: [0, 0, 0] })).includes("revolve-implied-closure")).toBe(c.kind === "shell");
    });
  }
  it("repeating the first point turns the filled body into the hollow one (trimesh volumes)", () => {
    const open = traps.revolveClosure.find((c) => c.name === "wall-without-repeated-first-point")!;
    const looped = [...open.profile, open.profile[0]!];
    const volume = (profile: Vec2[]) => {
      const g = tessellateSolid(solid({ kind: "revolve", profile, axis: [0, 1, 0], origin: [0, 0, 0] }), QUALITY);
      return pythonOracle<Record<string, { volume: number }>>(ORACLE, { cases: [{ name: "v", positions: g.positions, indices: g.indices }] }).v!.volume;
    };
    const [filled, hollow] = [volume(open.profile), volume(looped)];
    expect(hollow / filled).toBeCloseTo(0.19, 1);
    expect(codes(solid({ kind: "revolve", profile: looped, axis: [0, 1, 0], origin: [0, 0, 0] }))).not.toContain("revolve-implied-closure");
  });
});

describe("🏗️ the extrude taper", () => {
  for (const c of traps.taper) {
    it(`${c.name}: ${c.fault ?? "fine"}`, () => {
      expect(taperFault(c.taperTo ?? undefined)).toBe(c.fault);
      const found = codes(solid({ kind: "extrude", profile: [[-0.2, -0.2], [0.2, -0.2], [0.2, 0.2], [-0.2, 0.2]], plane: "xy", from: 0, to: 0.3, ...(c.taperTo ? { taperTo: c.taperTo } : {}) }));
      expect(found.includes("taper-uniform")).toBe(c.fault === "uniform");
      expect(found.includes("taper-degenerate")).toBe(c.fault === "collapsed" || c.fault === "needle");
    });
  }
  it("taperTo[0] is the scale at from and taperTo[1] the scale at to, also when from is the larger coordinate", () => {
    const wide = (from: number, to: number) => {
      const g = tessellateSolid(solid({ kind: "extrude", profile: [[-0.2, -0.2], [0.2, -0.2], [0.2, 0.2], [-0.2, 0.2]], plane: "xy", from, to, taperTo: [1, 0.5] }), QUALITY);
      const at = (z: number) => Math.max(...Array.from({ length: g.positions.length / 3 }, (_, i) => (Math.abs(g.positions[i * 3 + 2]! - z) < 1e-9 ? Math.abs(g.positions[i * 3]!) : 0)));
      return [at(from), at(to)];
    };
    expect(wide(0, 0.3)).toEqual([0.2, 0.1]);
    expect(wide(0.3, 0)).toEqual([0.2, 0.1]);
  });
});

describe("🧵️ the loft fold", () => {
  for (const c of traps.loftFold) {
    it(`${c.name}: ${c.fold ? `folds at sections ${c.fold.join(" and ")}` : "is clean"}`, () => {
      expect(loftFault(c.sections)).toEqual(c.fold);
      expect(codes(solid({ kind: "loft", sections: c.sections, capStart: true, capEnd: true })).includes("loft-fold")).toBe(c.fold !== null);
    });
  }
});
