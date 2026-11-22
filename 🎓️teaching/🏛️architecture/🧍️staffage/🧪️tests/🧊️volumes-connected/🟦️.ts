/** 🔗️ `volumes-connected` and the per-scale, sheet-aware `volumes-feature`: the exact distance between triangles, the groups of touching parts, the load paths of a structure, the rule on the sample figure and the judgement and the declaration of thin sheets, against hand values and an independent contact graph.
 *
 * A member solid that spans several bones (`members`) is measured here too: the distance of a bone from the solid that names it, the relation of the named bones in the rig and the three rules on the sample figure.
 *
 * THE REFERENCES are the hand-derived distances, groups and gaps of `🧫️fixtures/🧊️volumes-connected` and trimesh (closest points, containment) with scipy (connected components) in `🐍️.py`, fed with the same triangles.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🔨️modules/🗂️catalog/🧊️volumes/🔗️connected/🟦️.ts — the contact measure
 * @see ../🧊️volumes-rules/🟦️.ts — the other rules */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, ScaleDenominator } from "../../🧬️schema/🟦️.ts";
import { VOLUME_LIMITS, checkVolumes, connectedRule, featureOf, looseGroups, measureVolumes, partsOf, touchingGroups, triangleDistance2, type Loose, type Part } from "../../🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { collector, featureRule, isSheet, spanGap } from "../../🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { insideAny } from "../../🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts";
import { Mask, SMALL_OBJECT_CELLS, comparisonCell, type World } from "../../🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Cuboid = { id: string; part?: "member" | "joint" | "support" | "mass"; centre?: number[]; size: number[]; turnZDeg?: number; copies?: number[][] };
type Expected = { reason: Loose["reason"]; ids: string[]; parts: number; gapM: number; nearest?: string };
type Vectors = {
  featureM: number; toleranceM: number; apartM: number; capM: number; limits: Record<string, number>; distanceToleranceM: number; oracleSpacingM: number; oracleToleranceM: number;
  triangles: { name: string; hand: string; a: number[][]; b: number[][]; distanceM: number }[];
  cases: { name: string; hand: string; solids: Cuboid[]; groups: string[][]; loose: Expected[] }[];
  structures: { name: string; hand: string; solids: Cuboid[]; groups: number; loose: Expected[]; en?: string[]; de?: string[]; absent?: string[] }[];
  spans: { gapToleranceM: number; rod: Cuboid; segments: { name: string; hand: string; from: [number, number, number]; to: [number, number, number]; gapM: number }[]; cases: { name: string; hand: string; scale: ScaleDenominator; operations: Operation[]; coverage: number; feature: number; slender: number; gaps?: Record<string, number>; en: string[]; de: string[] }[] };
  sheets: { limitM: number; cases: { name: string; solid: Record<string, unknown>; gaugeM?: number; sheet: boolean }[]; declarations: { cases: { name: string; scale: ScaleDenominator; solid: Record<string, unknown>; bound?: { region: string; layer: string }; thin: number; hint: boolean; declaration: number; both: number; ratio?: number }[] } };
};

const vectors = fixture<Vectors>("🧊️volumes-connected");
const sample: Asset = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const ORACLE = join(STAFFAGE_ROOT, "🧪️tests/🧊️volumes-connected/🐍️.py");
const SCALES: readonly ScaleDenominator[] = [50, 100, 200, 500];
const QUADS = [[0, 2, 6, 4], [1, 5, 7, 3], [0, 1, 3, 2], [4, 6, 7, 5], [0, 4, 5, 1], [2, 3, 7, 6]];

function worldOf(solid: Cuboid): World {
  const centres = solid.copies ?? [solid.centre!];
  const [c, s] = [Math.cos(((solid.turnZDeg ?? 0) * Math.PI) / 180), Math.sin(((solid.turnZDeg ?? 0) * Math.PI) / 180)];
  const positions: number[] = [];
  const indices: number[] = [];
  centres.forEach((centre, copy) => {
    for (let k = 0; k < 8; k++) {
      const [x, y, z] = [((k & 4 ? 1 : -1) * solid.size[0]!) / 2, ((k & 2 ? 1 : -1) * solid.size[1]!) / 2, ((k & 1 ? 1 : -1) * solid.size[2]!) / 2];
      positions.push(centre[0]! + c * x - s * y, centre[1]! + s * x + c * y, centre[2]! + z);
    }
    for (const [p, q, r, t] of QUADS) indices.push(copy * 8 + p!, copy * 8 + q!, copy * 8 + r!, copy * 8 + p!, copy * 8 + r!, copy * 8 + t!);
  });
  return { id: solid.id, positions: Float64Array.from(positions), indices: Uint32Array.from(indices) };
}

const built = (solids: readonly Cuboid[]): readonly Part[] => partsOf(solids.map(worldOf), solids.map((solid) => solid.copies?.length ?? 1));
const options = (solids: readonly Cuboid[], structure: boolean) => ({ toleranceM: vectors.toleranceM, apartM: vectors.apartM, capM: vectors.capM, rests: (part: Part) => part.min[1] <= vectors.toleranceM, structure, kind: (part: Part) => { const kind = solids.find((solid) => solid.id === part.id)?.part; return kind === "mass" || kind === "support" ? kind : undefined; } });
const partition = (parts: readonly Part[], group: ArrayLike<number>): string[] => {
  const sets = new Map<number, number[]>();
  parts.forEach((_, i) => sets.set(group[i]!, [...(sets.get(group[i]!) ?? []), i]));
  return [...sets.values()].map((set) => set.join(",")).sort();
};
function expectLoose(found: readonly Loose[], expected: readonly Expected[], name: string): void {
  expect(found.map((x) => [x.reason, x.ids, x.parts, x.nearest]), name).toEqual(expected.map((x) => [x.reason, x.ids, x.parts, x.nearest]));
  found.forEach((x, k) => expect(Math.abs(x.gapM - expected[k]!.gapM), `${name}: gap ${x.gapM} against ${expected[k]!.gapM}`).toBeLessThanOrEqual(vectors.distanceToleranceM));
}

type Operation = { op: "replace" | "add" | "remove"; path: string; value?: unknown };
function patch(document: unknown, operations: readonly Operation[]): Asset {
  const root = structuredClone(document) as any;
  for (const { op, path, value } of operations) {
    const segments = path.split("/").slice(1);
    let node = root;
    const at = (n: any, s: string) => (Array.isArray(n) ? (s.startsWith("id=") ? n.findIndex((i: any) => i.id === s.slice(3)) : Number(s)) : s);
    for (const segment of segments.slice(0, -1)) node = node[at(node, segment)];
    const key = at(node, segments[segments.length - 1]!);
    if (op === "remove") Array.isArray(node) ? node.splice(key as number, 1) : delete node[key];
    else if (op === "add" && Array.isArray(node)) node.push(structuredClone(value));
    else node[key] = structuredClone(value);
  }
  return root as Asset;
}

describe("🔗️ the limits of the contact rule and of the sheet judgement", () => {
  it("are those of the fixture: a fifth of a feature of tolerance, one feature of clearance, six features of measured gap, a sheet twenty times as wide as thick", () => {
    for (const [name, value] of Object.entries(vectors.limits)) expect((VOLUME_LIMITS as Record<string, unknown>)[name], name).toBe(value);
    expect(VOLUME_LIMITS.connectedToleranceFeatures * featureOf(50)).toBeCloseTo(vectors.toleranceM, 12);
    expect(VOLUME_LIMITS.connectedApartFeatures * featureOf(50)).toBeCloseTo(vectors.apartM, 12);
    expect(VOLUME_LIMITS.connectedGapCapFeatures * featureOf(50)).toBeCloseTo(vectors.capM, 12);
  });
});

describe("📏️ the exact distance between two triangles", () => {
  it.each(vectors.triangles)("$name: $distanceM m", ({ a, b, distanceM, hand }) => {
    expect(Math.abs(Math.sqrt(triangleDistance2(a.flat(), b.flat())) - distanceM), hand).toBeLessThanOrEqual(vectors.distanceToleranceM);
    expect(Math.abs(Math.sqrt(triangleDistance2(b.flat(), a.flat())) - distanceM), hand).toBeLessThanOrEqual(vectors.distanceToleranceM);
  });
});

describe("🔗️ solids are grouped by true contact, copy by copy", () => {
  it.each(vectors.cases)("$name", ({ name, solids, groups, loose, hand }) => {
    const parts = built(solids);
    const found = touchingGroups(parts, vectors.toleranceM);
    const ids = new Map<number, string[]>();
    parts.forEach((part, i) => ids.set(found.group[i]!, [...(ids.get(found.group[i]!) ?? []), part.id]));
    expect([...ids.entries()].sort((p, q) => p[0] - q[0]).map(([, list]) => list), hand).toEqual(groups);
    expectLoose(looseGroups(parts, options(solids, false)).loose, loose, name);
  });

  it("does not depend on the order of the solids", () => {
    for (const c of vectors.cases) {
      const reversed = [...c.solids].reverse();
      expect(looseGroups(built(reversed), options(reversed, false)).loose.map((x) => [x.reason, [...x.ids].sort(), x.gapM.toFixed(9)]), c.name).toEqual(looseGroups(built(c.solids), options(c.solids, false)).loose.map((x) => [x.reason, [...x.ids].sort(), x.gapM.toFixed(9)]));
    }
  });
});

describe("🏗️ every load path of a structure ends in a support", () => {
  it.each(vectors.structures)("$name", ({ name, solids, groups, loose }) => {
    const result = looseGroups(built(solids), options(solids, true));
    expect(result.groups, name).toBe(groups);
    expectLoose(result.loose, loose, name);
  });

  it.each(vectors.structures.filter((c) => c.en !== undefined || c.absent !== undefined))("words the finding of $name for a structure and never names a mass marker", ({ solids, loose, en, de, absent }) => {
    const c = collector("volumes-connected");
    connectedRule(c, 50, "structure", built(solids), vectors.featureM, [], new Map(solids.map((solid) => [solid.id, solid.part ?? "member"] as const)));
    expect(c.findings.map((f) => f.data?.reason)).toEqual([...new Set(loose.map((x) => x.reason))]);
    const [english, german] = [c.findings.map((f) => f.message.en).join("\n"), c.findings.map((f) => f.message.de).join("\n")];
    for (const fragment of en ?? []) expect(english).toContain(fragment);
    for (const fragment of de ?? []) expect(german).toContain(fragment);
    for (const fragment of absent ?? []) expect(english + german).not.toContain(fragment);
  });
});

describe("🦴️ one member solid may span several bones that it names in `members`", () => {
  const spans = vectors.spans;
  const rod = worldOf(spans.rod);

  it("has the limits of the fixture: one smallest feature, nine points along the bone", () => {
    expect([VOLUME_LIMITS.memberSpanFeatures, VOLUME_LIMITS.memberSpanSamples]).toEqual([vectors.limits.memberSpanFeatures, vectors.limits.memberSpanSamples]);
  });

  it.each(spans.segments)("measures how far $name leaves it: $gapM m", ({ from, to, gapM, hand }) => {
    expect(Math.abs(spanGap(rod, from, to, (point) => insideAny([rod], point)) - gapM), hand).toBeLessThanOrEqual(vectors.distanceToleranceM);
    expect(Math.abs(spanGap(rod, to, from, (point) => insideAny([rod], point)) - gapM), hand).toBeLessThanOrEqual(vectors.distanceToleranceM);
  });

  it.each(spans.cases)("$name: $coverage coverage, $feature feature, $slender slender findings", ({ scale, operations, coverage, feature, slender, gaps, en, de, hand }) => {
    const asset = patch(sample, operations);
    const rules = ["volumes-structure-coverage", "volumes-feature", "volumes-slender"] as const;
    const found = checkVolumes(asset, { scale, rules }).filter((f) => !f.measurements.some((m) => m.name.startsWith("outsideBodyM:")));
    const of = (rule: string) => found.filter((f) => f.rule === rule);
    expect(rules.map((rule) => of(rule).length), `${hand}\n${found.map((f) => f.message.en).join("\n")}`).toEqual([coverage, feature, slender]);
    const [english, german] = [of(rules[0]).map((f) => f.message.en).join("\n"), of(rules[0]).map((f) => f.message.de).join("\n")];
    for (const fragment of en) expect(english).toContain(fragment);
    for (const fragment of de) expect(german).toContain(fragment);
    if (gaps === undefined) return;
    const measured = measureVolumes(asset, { scale, rules: [rules[0]] }).filter((m) => m.name.startsWith("spanGapM:"));
    expect(measured.map((m) => m.name).sort()).toEqual(Object.keys(gaps).map((key) => `spanGapM:${key}`).sort());
    for (const m of measured) {
      expect(Math.abs(m.value - gaps[m.name.slice("spanGapM:".length)]!), `${m.name} ${m.value}`).toBeLessThanOrEqual(spans.gapToleranceM);
      expect([m.limit, m.bound, m.ok]).toEqual([VOLUME_LIMITS.memberSpanFeatures * featureOf(scale), "max", m.value <= VOLUME_LIMITS.memberSpanFeatures * featureOf(scale)]);
    }
  });

  it("leaves the sample figure without any spanning measurement", () => {
    expect(measureVolumes(sample, { rules: ["volumes-structure-coverage"] }).filter((m) => m.name.startsWith("spanGapM:"))).toEqual([]);
  });
});

describe("🐍️ the contact graph equals trimesh and scipy on the same triangles", () => {
  it("has the same groups, the same gaps and the same distances of spanned bones", { timeout: 600_000 }, () => {
    requireTool("uv");
    const directory = mkdtempSync(join(tmpdir(), "staffage-connected-"));
    try {
      const all = [...vectors.cases, ...vectors.structures];
      const cases = all.map((c) => ({ name: c.name, parts: built(c.solids).map((part) => ({ positions: [...part.positions], indices: [...part.indices] })) }));
      const rod = worldOf(vectors.spans.rod);
      const spans = { mesh: { positions: [...rod.positions], indices: [...rod.indices] }, samples: VOLUME_LIMITS.memberSpanSamples, segments: vectors.spans.segments.map(({ name, from, to }) => ({ name, from, to })) };
      writeFileSync(join(directory, "input.json"), JSON.stringify({ tolerance: vectors.toleranceM, cap: vectors.capM, spacing: vectors.oracleSpacingM, cases, spans }));
      const output = JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, { labels: number[]; gaps: number[][] }> & { spans: Record<string, number> };
      const reference: Record<string, { labels: number[]; gaps: number[][] }> = output;
      for (const { name, from, to } of vectors.spans.segments) expect(Math.abs(spanGap(rod, from, to, (point) => insideAny([rod], point)) - output.spans[name]!), `span of ${name}`).toBeLessThanOrEqual(vectors.oracleToleranceM);
      for (const c of all) {
        const parts = built(c.solids);
        const owned = touchingGroups(parts, vectors.toleranceM);
        expect(partition(parts, owned.group), `${c.name}: groups`).toEqual(partition(parts, reference[c.name]!.labels));
        const judged = looseGroups(parts, options(c.solids, "groups" in c && typeof c.groups === "number"));
        for (const x of judged.loose) {
          if (owned.count === 1) continue;
          const group = Array.from({ length: owned.count }, (_, g) => g).find((g) => {
            const own = parts.filter((_, i) => owned.group[i] === g);
            return own.length === x.parts && [...new Set(own.map((part) => part.id))].join() === x.ids.join();
          })!;
          let least = vectors.capM;
          const structure = "groups" in c && typeof c.groups === "number";
          const counts = (part: Part): boolean => !structure || (c.solids as readonly Cuboid[]).find((solid) => solid.id === part.id)?.part !== "mass";
          parts.forEach((own, i) => parts.forEach((other, j) => { if (owned.group[i] === group && owned.group[j] !== group && counts(own) && counts(other)) least = Math.min(least, reference[c.name]!.gaps[i]![j]!); }));
          expect(Math.abs(x.gapM - least), `${c.name}: gap of "${x.ids[0]}" ${x.gapM} against ${least}`).toBeLessThanOrEqual(vectors.oracleToleranceM);
        }
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

describe("📐️ a thin sheet is judged by its two large extents", () => {
  it.each(vectors.sheets.cases)("$name: sheet $sheet", ({ solid, gaugeM, sheet }) => {
    expect(isSheet({ id: "s", bone: "b", material: "m", role: "body", tone: 0.5, ...solid } as never, vectors.sheets.limitM, gaugeM)).toBe(sheet);
  });

  it.each(vectors.sheets.declarations.cases)("a thin sheet passes only when declared: $name", ({ scale, solid, bound, thin, hint, declaration, both, ratio }) => {
    const first = (sample.volumes![`${scale}`] as unknown as { body: { bone: string; material: string }[] }).body[0]!;
    const sheet = { id: "sheet-test", bone: first.bone, material: first.material, role: "body", tone: 0.5, ...solid };
    const asset = patch(sample, [{ op: "add", path: `/volumes/${scale}/body/-`, value: sheet }, ...(bound === undefined ? [] : [{ op: "add" as const, path: `/volumes/${scale}/thermal/-`, value: { id: "sheet-test-surface", of: sheet.id, ...bound } }])]);
    const measured = featureRule(asset, [scale]);
    const found = measured.findings.filter((f) => f.data?.solid === "sheet-test");
    for (const f of found) expect([f.rule, f.code, f.message.de.includes('"sheet-test"')]).toEqual(["volumes-feature", "volumes-feature", true]);
    const of = (text: string) => found.filter((f) => f.message.en.includes(text));
    expect(of("mm thin, below").length).toBe(thin);
    expect(of("declare `sheet: true`").length).toBe(hint ? 1 : 0);
    expect(of("declares `sheet` but is no thin sheet").length).toBe(declaration);
    expect(of("declares both `sheet` and `slender`").length).toBe(both);
    expect(found.length).toBe(thin + declaration + both);
    const m = measured.measurements.find((x) => x.name === "sheetRatio:sheet-test");
    if (ratio === undefined) expect(m).toBeUndefined();
    else {
      expect(m!.value).toBeCloseTo(ratio, 9);
      expect([m!.limit, m!.bound, m!.ok]).toEqual([VOLUME_LIMITS.sheetRatio, "min", ratio >= VOLUME_LIMITS.sheetRatio]);
    }
  });
});

describe("🔬️ every scale is judged for its own smallest feature", () => {
  it("passes a 20 mm cube at 1:50 and reports the identical cube at 1:500", () => {
    const first = (sample.volumes!["50"] as unknown as { body: { bone: string; material: string }[] }).body[0]!;
    const cube = { id: "carried-test", kind: "box", bone: first.bone, material: first.material, role: "body", tone: 0.5, size: [0.02, 0.02, 0.02] };
    const asset = patch(sample, [{ op: "add", path: "/volumes/50/body/-", value: cube }, { op: "add", path: "/volumes/500/body/-", value: cube }]);
    expect(checkVolumes(asset, { scale: 50, rules: ["volumes-feature"] }).filter((f) => f.message.en.includes('"carried-test"'))).toEqual([]);
    const coarse = checkVolumes(asset, { scale: 500, rules: ["volumes-feature"] }).filter((f) => f.message.en.includes('"carried-test"'));
    expect(coarse.length).toBe(1);
    expect(coarse[0]!.measurements[0]!.value).toBeCloseTo(0.02, 12);
    expect(coarse[0]!.measurements[0]!.limit).toBeLessThanOrEqual(featureOf(500));
    expect(coarse[0]!.data).toEqual({ solid: "carried-test" });
  });
});

describe("👤️ volumes-connected on the sample figure", () => {
  const rule = { rules: ["volumes-connected"] as const };

  it("reports nothing at any scale: one body, a load path that ends on the ground", () => {
    expect(checkVolumes(sample, rule)).toEqual([]);
    for (const scale of SCALES) {
      const measured = measureVolumes(sample, { scale, ...rule });
      expect(measured.filter((m) => m.name === "info:groups").map((m) => [m.list, m.value])).toEqual([["body", 1], ["structure", 1]]);
      expect(measured.filter((m) => m.name.startsWith("looseGroups:")).every((m) => m.value === 0 && m.limit === 0 && m.ok)).toBe(true);
    }
  });

  it("finds a head lifted off the neck: floating, named, in English and German", () => {
    const found = checkVolumes(patch(sample, [{ op: "replace", path: "/volumes/500/body/id=head/transform/translation", value: [1.103, 0, 0] }]), { scale: 500, ...rule });
    expect(found.map((f) => [f.rule, f.scale, f.list, f.data?.reason, f.data?.solids])).toEqual([["volumes-connected", 500, "body", "floating", ["head"]]]);
    expect(found[0]!.message.en).toContain('"head" at least 450.0 mm');
    expect(found[0]!.message.de).toContain('"head" mindestens 450.0 mm');
    expect(found[0]!.measurements.map((m) => [m.name, m.value, m.limit, m.ok])).toEqual([["looseGroups:floating", 1, 0, false]]);
    expect(checkVolumes(patch(sample, [{ op: "replace", path: "/volumes/500/body/id=head/transform/translation", value: [1.103, 0, 0] }]), { scale: 200, ...rule })).toEqual([]);
  });

  it("finds a shoe set beside its foot: it stands on the ground, but the ankle of the rig ties it to the shank", () => {
    const found = checkVolumes(patch(sample, [{ op: "replace", path: "/volumes/500/body/id=shoe-l/transform/translation", value: [0.075, -0.0725, 0.39] }]), { scale: 500, ...rule });
    expect(found.map((f) => [f.list, f.data?.reason, f.data?.solids])).toEqual([["body", "detached", ["shoe-l"]]]);
    expect(found[0]!.message.en).toContain("joint foot-l — shank-l");
    expect(found[0]!.message.de).toContain("Gelenk foot-l — shank-l");
    const gap = Number((found[0]!.data!.gapsM as readonly string[])[0]);
    expect(gap).toBeGreaterThan(featureOf(500));
    expect(gap).toBeLessThan(0.3 - 0.045);
  });

  it("finds a member pulled off its joint: an open load path", () => {
    const found = checkVolumes(patch(sample, [{ op: "replace", path: "/volumes/100/structure/id=member-arm-l/p0", value: [0, 0, 0.5] }, { op: "replace", path: "/volumes/100/structure/id=member-arm-l/p1", value: [0.6, 0, 0.5] }]), { scale: 100, ...rule });
    expect(found.map((f) => [f.list, f.data?.reason, f.data?.solids])).toEqual([["structure", "unsupported", ["member-arm-l"]]]);
    expect(found[0]!.message.en).toContain('"member-arm-l"');
    expect(found[0]!.message.de).toContain("offenen Lastpfad");
  });

  it("is deterministic and stops when cancelled", () => {
    const text = (asset: Asset): string => JSON.stringify(measureVolumes(asset, rule));
    expect(text(sample)).toBe(text(structuredClone(sample)));
    const controller = new AbortController();
    controller.abort();
    expect(() => checkVolumes(sample, rule, { signal: controller.signal })).toThrow();
  });
});

describe("🫙️ a hollow vessel is judged by its overall extents, a lump by its own", () => {
  const vessels = fixture<{ vessels: { cases: { name: string; solid: Record<string, unknown>; finding: boolean }[] } }>("🧊️volumes-connected").vessels;
  it.each(vessels.cases)("$name: finding $finding", ({ solid, finding }) => {
    const first = (sample.volumes!["50"] as unknown as { body: { bone: string; material: string }[] }).body[0]!;
    const found = checkVolumes(patch(sample, [{ op: "add", path: "/volumes/50/body/-", value: { id: "vessel-test", bone: first.bone, material: first.material, role: "body", tone: 0.5, ...solid } }]), { scale: 50, rules: ["volumes-feature"] });
    expect(found.some((f) => f.message.en.includes('"vessel-test"'))).toBe(finding);
  });
});

describe("🔎️ the rule raster scales with the object", () => {
  const raster = fixture<{ raster: { cells: number; floorM: number; cases: { featureM: number; longM: number; cellM: number }[]; rectangles: { a: number[]; b: number[]; featureM: number; iou: number; toleranceIou: number } } }>("🧊️volumes-connected").raster;
  const ring = (r: readonly number[]) => [[{ x: r[0]!, y: r[1]! }, { x: r[2]!, y: r[1]! }, { x: r[2]!, y: r[3]! }, { x: r[0]!, y: r[3]! }]];
  const iouAt = (cell: number): number => {
    const { a, b } = raster.rectangles;
    const box = { minX: a[0]! - 2 * cell, minY: a[1]! - 2 * cell, maxX: a[2]! + 2 * cell, maxY: a[3]! + 2 * cell };
    const [first, second] = [new Mask(box, cell), new Mask(box, cell)];
    first.fill(ring(a), true);
    second.fill(ring(b), true);
    let [both, either] = [0, 0];
    for (let k = 0; k < first.cells.length; k++) {
      both += first.cells[k]! & second.cells[k]!;
      either += first.cells[k]! | second.cells[k]!;
    }
    return both / either;
  };

  it.each(raster.cases)("feature $featureM m, long side $longM m: cell $cellM m", ({ featureM, longM, cellM }) => {
    expect(SMALL_OBJECT_CELLS).toBe(raster.cells);
    expect(comparisonCell(featureM, longM)).toBeCloseTo(cellM, 12);
  });

  it("measures the IoU of a 60 mm part to its hand value, which a quarter-feature raster misses", () => {
    const { a, featureM, iou, toleranceIou } = raster.rectangles;
    const long = Math.max(a[2]! - a[0]!, a[3]! - a[1]!);
    expect(Math.abs(iouAt(comparisonCell(featureM, long)) - iou)).toBeLessThanOrEqual(toleranceIou);
    expect(Math.abs(iouAt(featureM / 4) - iou)).toBeGreaterThan(toleranceIou);
  });
});

describe("🦴️ a long member keeps its true section at every scale when it is declared slender", () => {
  type Long = { scale: ScaleDenominator; femur: Record<string, unknown>; radius: Record<string, unknown>; finger: Record<string, unknown>; hoopWide: Record<string, unknown>; hoopNarrow: Record<string, unknown>; cases: { name: string; list: "body" | "structure"; solid: "femur" | "radius" | "finger" | "hoopWide" | "hoopNarrow"; slender: boolean; feature: boolean; slenderFinding: boolean }[] };
  const long = fixture<{ longMembers: Long }>("🧊️volumes-connected").longMembers;
  it.each(long.cases)("$name: feature finding $feature, slender finding $slenderFinding", ({ list, solid, slender, feature, slenderFinding }) => {
    const member = (sample.volumes![`${long.scale}`] as unknown as { structure: { bone: string; material: string; member: string }[] }).structure[0]!;
    const value = { id: "long-test", bone: member.bone, material: member.material, role: "body", tone: 0.5, ...long[solid], ...(slender ? { slender: true } : {}), ...(list === "structure" ? { member: member.member, part: "member" } : {}) };
    const found = checkVolumes(patch(sample, [{ op: "add", path: `/volumes/${long.scale}/${list}/-`, value }]), { scale: long.scale, rules: ["volumes-feature", "volumes-slender"] }).filter((f) => f.message.en.includes('"long-test"'));
    expect(found.some((f) => f.rule === "volumes-feature"), found.map((f) => f.message.en).join("\n")).toBe(feature);
    expect(found.some((f) => f.rule === "volumes-slender"), found.map((f) => f.message.en).join("\n")).toBe(slenderFinding);
    for (const f of found) expect(f.message.de).toContain('"long-test"');
  });
});
