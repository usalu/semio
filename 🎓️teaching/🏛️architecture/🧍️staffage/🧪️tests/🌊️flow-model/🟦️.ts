/** 🌊️ The air flow inside the 3D model: the thermal scenes of every 3D file carry the developed flow field of the instant as stream tubes with arrow heads and as vapour and fog puffs, generalised per scale by the marks of the drawings, hidden in every variant but `thermal`; no emitter cone is left.
 *
 * THE REFERENCES are the numpy and trimesh oracle of `./🐍️.py` (the fixture's `expected`: bins, tube stations, radii, arrow heads, puff classes and the icosphere volumes for a synthetic field), the Khronos `gltf-validator`, three's `GLTFLoader` (the scenes, nodes and vertices a viewer reads), three's `Line3` (distances to the streams) and three's colour management (sRGB to linear).
 *
 * @see ./🥒️.feature — the scenarios
 */
import { Box3, Color, Line3, Vector3, type BufferGeometry, type Object3D } from "three";
import { describe, expect, it } from "vitest";
import type { Asset, Environment } from "../../🧬️schema/🟦️.ts";
import { CancelledError, loftFault, validateModel, type ModelDocument, type Solid } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { DisplayRange, FlowField, Vec3 } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { FLOW_DEVELOP_S } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { FLOW_STYLE, SHARED_FLOW_EXCESS_K, SHARED_FLOW_FOG_GKG, SHARED_FLOW_VAPOUR_GKG, excessOfBin, flowPaint } from "../../🔨️modules/🎨️depiction/🌊️flow-view/🟦️.ts";
import { developedTimeS, solveTransport } from "../../🔨️modules/💨️transport/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { flowDetailOf, flowMarks } from "../../🔨️modules/💨️transport/🌊️flow/🟦️.ts";
import { writeGlb } from "../../🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { FLOW_MODEL, FLOW_TAG, authoredScales, flowParts, flowUnitOf, modelOf, modelOfAsset, tubesOf } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { type GlbJson, auditGeometry, visibleGlyphs } from "../🧊️model-catalog/🔍️oracles/🟦️.ts";
import { loadGlb, validateGlb } from "../🧰️support/🧊️.ts";
import { fixture } from "../🧰️support/🧰️.ts";

type Tube = { start: number; bin: number; stations: number[]; radiiM: number[] };
type Puff = { puff: number; bin: number; radiusM: number; triangles: number };
type Scale = { scale: number; unitMPerMm: number; headLengthM: number; headRadiusM: number; streams: { stream: number; tubes: Tube[]; arrows: { index: number; bin: number; direction: number[] }[] }[]; vapour: Puff[]; fog: Puff[] };
type Case = { id: string; fixture: string; pose: string; environment: Environment | null; glbScales: number[]; fog: boolean };
type Vectors = { rules: Record<string, number | number[]>; extentM: number; theme: "light"; range: DisplayRange; tolerances: { lengthM: number; sphereRelative: number; hugM: number }; assets: Case[]; field: FlowField; expected: { scales: Scale[]; unitSpheres: Record<string, { vertices: number; volume: number; area: number }> } };

const vectors = fixture<Vectors>("🌊️flow-model");
const neutral = (p: Vec3): [number, number, number] => [p.z, p.y, p.x];
const lofts = (solids: readonly Solid[], node: string) => solids.filter((s) => s.node === node && s.kind === "loft") as Extract<Solid, { kind: "loft" }>[];
const cones = (solids: readonly Solid[], node: string) => solids.filter((s) => s.node === node && s.kind === "revolve") as Extract<Solid, { kind: "revolve" }>[];
const balls = (solids: readonly Solid[], node: string) => solids.filter((s) => s.node === node && s.kind === "polyhedron") as Extract<Solid, { kind: "polyhedron" }>[];
const linear = (hex: string): number[] => new Color().setStyle(hex).toArray();
const near = (a: readonly number[], b: readonly number[], eps: number, label: string): void => a.forEach((v, i) => expect(Math.abs(v - b[i]!), `${label}[${i}]`).toBeLessThanOrEqual(eps));

function measure(solid: Extract<Solid, { kind: "polyhedron" }>): { centre: number[]; volume: number; area: number; radii: number[] } {
  const v = solid.vertices.map((p) => new Vector3(p[0], p[1], p[2]));
  const centre = v.reduce((sum, p) => sum.add(p), new Vector3()).divideScalar(v.length);
  let volume = 0, area = 0;
  for (const [a, b, c] of solid.triangles) {
    const pa = v[a]!.clone().sub(centre), pb = v[b]!.clone().sub(centre), pc = v[c]!.clone().sub(centre);
    volume += pa.dot(pb.clone().cross(pc)) / 6;
    area += pb.clone().sub(pa).cross(pc.clone().sub(pa)).length() / 2;
  }
  return { centre: centre.toArray(), volume, area, radii: v.map((p) => p.distanceTo(centre)) };
}

const distanceTo = (polyline: readonly Vector3[], p: Vector3): number => {
  let best = Infinity;
  const hit = new Vector3();
  for (let i = 1; i < polyline.length; i++) best = Math.min(best, new Line3(polyline[i - 1]!, polyline[i]!).closestPointToPoint(p, true, hit).distanceTo(p));
  return best;
};

const ids = (document: ModelDocument): string[] => [...document.nodes.map((n) => n.id), ...document.solids.map((s) => s.id), ...document.materials.map((m) => m.id), ...document.variants.map((v) => v.id), ...(document.scenes ?? []).map((s) => s.id)];

describe("🌊️ the rules of the fixture are the rules of the library", () => {
  it("uses the paper sizes, bins and caps the oracle was given", () => {
    const r = vectors.rules;
    expect([FLOW_STYLE.streamMinMm, FLOW_STYLE.streamMaxMm, FLOW_STYLE.headLengthMm, FLOW_STYLE.headWidthMm, FLOW_STYLE.puffMinMm, FLOW_STYLE.puffMaxMm, FLOW_STYLE.vapourThresholdGKg, FLOW_STYLE.bins]).toEqual([r.streamMinMm, r.streamMaxMm, r.headLengthMm, r.headWidthMm, r.puffMinMm, r.puffMaxMm, r.vapourThresholdGKg, r.bins]);
    expect([FLOW_MODEL.chordMm, FLOW_MODEL.tubeShare, FLOW_MODEL.vapourBins, FLOW_MODEL.fogBins, [...FLOW_MODEL.puffTriangles], FLOW_MODEL.puffFineMm]).toEqual([r.chordMm, r.tubeShare, r.vapourBins, r.fogBins, r.puffTriangles, r.puffFineMm]);
    expect([SHARED_FLOW_EXCESS_K, SHARED_FLOW_VAPOUR_GKG, SHARED_FLOW_FOG_GKG, flowDetailOf(1000).spacingM]).toEqual([r.sharedExcessK, r.sharedVapourGKg, r.sharedFogGKg, r.spacingMm]);
    expect(FLOW_DEVELOP_S).toBe(14);
  });
});

describe("🌊️ the flow parts of a synthetic field match the numpy and trimesh oracle", () => {
  it.each(vectors.expected.scales.map((s) => [s.scale, s] as const))("1:%d: tubes, arrow heads and puffs", (scale, want) => {
    const { field, tolerances: tol, extentM } = vectors;
    const parts = flowParts(field, scale, extentM, vectors.theme, vectors.range);
    const paint = flowPaint(vectors.theme, field, vectors.range);
    expect(flowUnitOf(scale, extentM)).toBeCloseTo(want.unitMPerMm, 15);
    expect(parts.nodes[0]).toMatchObject({ id: "flow", parent: null, tags: ["glyph", FLOW_TAG] });
    expect(parts.nodes.slice(1).map((n) => [n.id, n.parent])).toEqual([...want.streams.map((s) => [`flow-stream-${s.stream}`, "flow"]), ["flow-vapour", "flow"], ["flow-fog", "flow"]]);
    for (const solid of parts.solids) expect(solid.tags).toEqual(["glyph", FLOW_TAG]);
    let counted = 0;
    for (const stream of want.streams) {
      const points = field.streams[stream.stream]!.points;
      const node = `flow-stream-${stream.stream}`;
      const tubes = lofts(parts.solids, node);
      expect(tubes.map((t) => [t.id, t.material])).toEqual(stream.tubes.map((t) => [`${node}-${t.start}`, `flow-excess-${t.bin}`]));
      tubes.forEach((tube, i) => {
        const expected = stream.tubes[i]!;
        expect([tube.capStart, tube.capEnd]).toEqual([true, true]);
        expect(tube.sections.map((s) => s.at)).toEqual(expected.stations.map((index) => neutral(points[index]!)));
        near(tube.sections.map((s) => s.rx), expected.radiiM, tol.lengthM, `${tube.id} radius`);
        for (const s of tube.sections) expect(s.ry).toBe(s.rx);
      });
      const heads = cones(parts.solids, node);
      expect(heads.map((h) => [h.id, h.material])).toEqual(stream.arrows.map((a, j) => [`flow-arrow-${stream.stream}-${j}`, `flow-excess-${a.bin}`]));
      heads.forEach((head, j) => {
        const arrow = stream.arrows[j]!;
        expect(head.origin).toEqual(neutral(points[arrow.index]!));
        near(head.axis, [arrow.direction[2]!, arrow.direction[1]!, arrow.direction[0]!], tol.lengthM, `${head.id} axis`);
        near(head.profile.flat(), [-want.headLengthM, 0, -want.headLengthM, want.headRadiusM, 0, 0], tol.lengthM, `${head.id} profile`);
      });
      counted += tubes.length + heads.length;
    }
    for (const [kind, list] of [["vapour", want.vapour], ["fog", want.fog]] as const) {
      const made = balls(parts.solids, `flow-${kind}`).map((solid) => ({ solid, ...measure(solid) }));
      expect(made.length).toBe(list.length);
      for (const expected of list) {
        const centre = neutral(field.puffs[expected.puff]!.at);
        const ball = made.find((m) => Math.hypot(m.centre[0]! - centre[0], m.centre[1]! - centre[1], m.centre[2]! - centre[2]) < tol.lengthM);
        expect(ball, `${kind} puff ${expected.puff}`).toBeDefined();
        const unit = vectors.expected.unitSpheres[`${expected.triangles}`]!;
        expect(ball!.solid.material).toBe(`flow-${kind}-${expected.bin}`);
        expect([ball!.solid.triangles.length, ball!.solid.vertices.length]).toEqual([expected.triangles, unit.vertices]);
        for (const radius of ball!.radii) expect(Math.abs(radius - expected.radiusM)).toBeLessThanOrEqual(tol.lengthM);
        expect(Math.abs(ball!.volume / (unit.volume * expected.radiusM ** 3) - 1)).toBeLessThanOrEqual(tol.sphereRelative);
        expect(Math.abs(ball!.area / (unit.area * expected.radiusM ** 2) - 1)).toBeLessThanOrEqual(tol.sphereRelative);
      }
      counted += made.length;
    }
    expect(parts.solids.length).toBe(counted);
    expect(new Set(parts.materials.map((m) => m.id))).toEqual(new Set(parts.solids.map((s) => s.material)));
    for (const material of parts.materials) {
      const [, kind, bin] = /^flow-(excess|vapour|fog)-(\d+)$/u.exec(material.id)!;
      const middle = kind === "excess" ? excessOfBin(paint, Number(bin)) : ((Number(bin) + 0.5) * (kind === "vapour" ? paint.vapourMaxGKg : paint.fogMaxGKg)) / (kind === "vapour" ? FLOW_MODEL.vapourBins : FLOW_MODEL.fogBins);
      expect(material.paint).toEqual(kind === "excess" ? { hex: paint.excessColour(middle), alpha: 1 } : kind === "vapour" ? { hex: paint.vapourColour(middle), alpha: paint.vapourOpacity(middle) } : { hex: paint.fogColour, alpha: paint.fogOpacity(middle) });
      expect(material.title.en).not.toBe(material.title.de);
    }
  });

  it("ends a tube where the stream bends tighter than the tube is wide and starts the next at the same station", () => {
    const hairpin = [[0, 0, 0], [0, 0.5, 0], [0, 1, 0], [0.004, 1.004, 0], [0.008, 1, 0], [0.008, 0.5, 0], [0.008, 0, 0]].map((at) => ({ at: at as [number, number, number], rx: 0.02, ry: 0.02 }));
    expect(loftFault(hairpin)).not.toBeNull();
    const tubes = tubesOf(hairpin);
    expect(tubes.length).toBeGreaterThan(1);
    for (const tube of tubes) expect(loftFault(tube)).toBeNull();
    expect(tubes.flatMap((tube, i) => tube.slice(i === 0 ? 0 : 1))).toEqual(hairpin);
    const smooth = hairpin.slice(0, 3);
    expect(tubesOf(smooth)).toEqual([smooth]);
  });

  it("gives a field without marks no part at all", () => {
    expect(flowParts({ ...vectors.field, streams: [], puffs: [] }, 50, vectors.extentM, "light", vectors.range)).toEqual({ nodes: [], solids: [], materials: [] });
  });
});

describe("🌊️ the model of a flowing asset carries the developed flow of its instant", () => {
  const assetOf = (c: Case): Asset => {
    const file = fixture<Asset | { asset: Asset }>(c.fixture);
    return "asset" in file ? file.asset : file;
  };
  const requestOf = (c: Case) => ({ at: { kind: "pose", pose: c.pose } as const, poses: [c.pose], clips: [], ...(c.environment === null ? {} : { environment: c.environment }) });

  it.each(vectors.assets.map((c) => [c.id, c] as const))("%s: tubes on the streams of the drawings' field at every scale, in the thermal scenes and the thermal variant only", { timeout: 600_000 }, async (_id, c) => {
    const asset = assetOf(c);
    const field = solveTransport(asset, { kind: "pose", pose: c.pose, timeS: developedTimeS(0, FLOW_DEVELOP_S) }, c.environment ?? asset.thermal.environment, { maxAgeS: FLOW_DEVELOP_S, flow: true }).flow!;
    expect(field.streams.length).toBeGreaterThan(4);
    const reports: number[] = [];
    const document = await modelOfAsset(asset, requestOf(c), { report: (fraction) => void reports.push(fraction), cancelled: () => false });
    expect(validateModel(document)).toEqual([]);
    expect(reports.length).toBeGreaterThan(20);
    for (let i = 1; i < reports.length; i++) expect(reports[i]!).toBeGreaterThanOrEqual(reports[i - 1]!);
    expect(reports[reports.length - 1]).toBe(1);
    for (const id of ids(document)) expect(id).not.toMatch(/emitter|cone/u);
    const flowSolids = document.solids.filter((s) => s.tags?.includes(FLOW_TAG));
    for (const solid of flowSolids) expect(solid.id).toMatch(/^s\d+-thermal-flow-/u);
    for (const scene of document.scenes!) expect(scene.nodes.some((n) => n.includes("-flow-")), scene.id).toBe(scene.id.endsWith("-thermal"));
    const extent = Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM);
    for (const scale of authoredScales(asset.volumes)) {
      const scene = `s${scale}-thermal`;
      const marks = flowMarks(field, field.timeS, flowDetailOf(scale));
      const chord = FLOW_MODEL.chordMm * (scale / 1000);
      expect(marks.ribbons.length, `streams at 1:${scale}`).toBeGreaterThan(0);
      expect(document.nodes.filter((n) => n.id.startsWith(`${scene}-flow-stream-`)).map((n) => n.id)).toEqual(marks.ribbons.map((r) => `${scene}-flow-stream-${r.stream}`));
      for (const ribbon of marks.ribbons) {
        const node = `${scene}-flow-stream-${ribbon.stream}`;
        const axis = lofts(document.solids, node).flatMap((tube, i) => tube.sections.slice(i === 0 ? 0 : 1).map((s) => [...s.at]));
        const points = field.streams[ribbon.stream]!.points.map(neutral);
        expect(axis[0]).toEqual(points[0]);
        expect(axis[axis.length - 1]).toEqual(points[points.length - 1]);
        let cursor = 0;
        for (const at of axis) {
          const found = points.findIndex((p, i) => i >= cursor && p[0] === at[0] && p[1] === at[1] && p[2] === at[2]);
          expect(found, `${node}: a tube section off the stream points`).toBeGreaterThanOrEqual(cursor);
          cursor = found + 1;
        }
        const polyline = axis.map((p) => new Vector3(p[0], p[1], p[2]));
        for (const p of points) expect(distanceTo(polyline, new Vector3(...p)), `${node}: stream point off the tube axis`).toBeLessThanOrEqual(chord + vectors.tolerances.hugM);
        expect(cones(document.solids, node).map((h) => h.origin)).toEqual(ribbon.arrows.map((a) => neutral(a.at)));
        for (const tube of lofts(document.solids, node)) for (const s of tube.sections) expect(s.rx).toBeLessThanOrEqual(Math.min((FLOW_STYLE.streamMaxMm / 2) * (scale / 1000), FLOW_MODEL.tubeShare * extent) * (1 + 1e-12));
      }
      expect(balls(document.solids, `${scene}-flow-vapour`).length).toBe(marks.puffs.filter((p) => p.vapourExcessKgPerKg * 1000 > FLOW_STYLE.vapourThresholdGKg).length);
      expect(balls(document.solids, `${scene}-flow-fog`).length).toBe(marks.puffs.filter((p) => p.condensateKgPerKg > 0).length);
      if (c.fog) expect(balls(document.solids, `${scene}-flow-fog`).length, `fog at 1:${scale}`).toBeGreaterThan(0);
    }
    const paint = flowPaint("light", field, { mode: "shared" });
    for (const material of new Set(flowSolids.map((s) => s.material))) {
      expect(document.materials.find((m) => m.id === material)!.baseColor[3], `${material} base`).toBe(0);
      const shown = document.variants.filter((v) => v.bindings[material] !== undefined);
      expect(shown.map((v) => v.id), material).toEqual(["thermal"]);
      const bound = document.materials.find((m) => m.id === shown[0]!.bindings[material])!;
      expect(bound.baseColor[3]).toBeGreaterThan(0);
      const bin = /-flow-excess-(\d+)$/u.exec(material);
      if (bin !== null) near(bound.baseColor.slice(0, 3), linear(paint.excessColour(excessOfBin(paint, Number(bin[1])))), 1e-6, material);
    }
    const posed = await modelOfAsset(asset, { at: { kind: "pose", pose: c.pose }, poses: "all", clips: [], scales: [200], flowGeometry: true, ...(c.environment === null ? {} : { environment: c.environment }) });
    for (const pose of posed.poses) expect(pose.nodes["s200-thermal-flow"]?.scale, pose.id).toEqual(pose.id === c.pose ? undefined : [1e-5, 1e-5, 1e-5]);
  });

  it("keeps the flow solids watertight, outward, free of degenerate triangles and falling with the scale (the geometry audit of the model tests)", { timeout: 600_000 }, async () => {
    const audit = await auditGeometry(assetOf(vectors.assets[0]!));
    expect(audit.findings.filter((finding) => finding.item.includes("-flow-") || finding.message.startsWith("thermal") || finding.check === "model" || finding.check === "geometry")).toEqual([]);
    const flow = audit.solids.filter((s) => s.id.includes("-flow-"));
    expect(flow.length).toBeGreaterThan(50);
    for (const solid of flow) expect([solid.closedExpected, solid.boundary, solid.overlapped, solid.degenerate, solid.volume > 0], solid.id).toEqual([true, 0, 0, 0, true]);
    const thermal = audit.facts.scenes.filter((s) => s.mode === "thermal").sort((a, b) => a.scale - b.scale).map((s) => s.triangles);
    expect(thermal).toEqual([...thermal].sort((a, b) => b - a));
  });

  it("carries the flow of a walking clip's first instant on the figure of that instant, as static geometry", { timeout: 300_000 }, async () => {
    const asset = assetOf(vectors.assets[0]!);
    const clip = asset.clips.find((c) => c.id === "walk")!;
    const developed = { kind: "clip", clip: clip.id, timeS: developedTimeS(0, FLOW_DEVELOP_S, clip.durationS) } as const;
    expect(developed.timeS).toBe(14);
    const field = solveTransport(asset, developed, asset.thermal.environment, { maxAgeS: FLOW_DEVELOP_S, periodS: clip.durationS, flow: true }).flow!;
    const root = (timeS: number) => evaluate(asset, { kind: "clip", clip: clip.id, timeS }).bones[asset.rig.root]!.frame.o;
    const travel = { x: root(14).x - root(0).x, y: root(14).y - root(0).y, z: root(14).z - root(0).z };
    expect(travel.x).toBeGreaterThan(5);
    const document = await modelOf(asset, { scale: 200, view: "thermal", at: { kind: "clip", clip: clip.id, timeS: 0 }, poses: [], clips: [clip.id] });
    expect(validateModel(document)).toEqual([]);
    const marks = flowMarks(field, field.timeS, flowDetailOf(200));
    expect(marks.ribbons.length).toBeGreaterThan(0);
    for (const ribbon of marks.ribbons) {
      const first = field.streams[ribbon.stream]!.points[0]!;
      expect(lofts(document.solids, `flow-stream-${ribbon.stream}`)[0]!.sections[0]!.at).toEqual([first.z - travel.z, first.y - travel.y, first.x - travel.x]);
    }
    const body = evaluate(asset, { kind: "clip", clip: clip.id, timeS: 0 }).bounds;
    const ahead = document.solids.filter((s) => s.tags?.includes(FLOW_TAG)).flatMap((s) => (s.kind === "loft" ? s.sections.map((section) => section.at[2]!) : []));
    expect(Math.min(...ahead)).toBeLessThan(body.maxX);
    expect(Math.max(...ahead)).toBeGreaterThan(body.minX);
    expect(document.animations[0]!.channels.filter((channel) => "node" in channel.target && channel.target.node.startsWith("flow"))).toEqual([]);
  });

  it("builds no flow where the caller animates it itself, and stops developing when cancelled", { timeout: 300_000 }, async () => {
    const c = vectors.assets[0]!;
    const asset = assetOf(c);
    const bare = await modelOfAsset(asset, { ...requestOf(c), flowGeometry: false });
    expect(validateModel(bare)).toEqual([]);
    expect(ids(bare).filter((id) => /flow|emitter/u.test(id))).toEqual([]);
    expect(bare.nodes.some((n) => n.tags?.includes(FLOW_TAG)) || bare.solids.some((s) => s.tags?.includes(FLOW_TAG))).toBe(false);
    let reports = 0;
    await expect(modelOfAsset(asset, { ...requestOf(c), environment: { ...asset.thermal.environment, airTempC: 11 } }, { report: () => void reports++, cancelled: () => reports >= 6 })).rejects.toBeInstanceOf(CancelledError);
    expect(reports).toBeLessThan(12);
  });

  it.each(vectors.assets.filter((c) => c.glbScales.length > 0).map((c) => [c.id, c] as const))("%s: the GLB is validator-clean and three reads the flow in its thermal scenes, hugging the streams, shown by the thermal variant only", { timeout: 600_000 }, async (_id, c) => {
    const asset = assetOf(c);
    const field = solveTransport(asset, { kind: "pose", pose: c.pose, timeS: developedTimeS(0, FLOW_DEVELOP_S) }, c.environment ?? asset.thermal.environment, { maxAgeS: FLOW_DEVELOP_S, flow: true }).flow!;
    const bytes = await writeGlb(asset, { pose: c.pose, scales: c.glbScales, ...(c.environment === null ? {} : { environment: c.environment }) });
    expect((await validateGlb(bytes, `${asset.id}.glb`)).messages).toEqual([]);
    const { scenes, parser } = await loadGlb(bytes);
    const json = parser.json as GlbJson & { scenes: { name: string }[] };
    for (const named of [...json.nodes, ...json.meshes, ...json.materials]) expect(named.name ?? "").not.toMatch(/emitter|cone/u);
    const visible = visibleGlyphs(json);
    const flowOf = (variant: string): string[] => [...visible[variant]!.triangles].filter((solid) => solid.includes("-flow-"));
    expect(flowOf("thermal").length).toBeGreaterThan(0);
    for (const variant of Object.keys(visible)) if (variant !== "thermal") expect([...flowOf(variant), ...[...visible[variant]!.lines].filter((solid) => solid.includes("-flow-"))], variant).toEqual([]);
    const tagged = (object: Object3D): boolean => Array.isArray(object.userData?.tags) && (object.userData.tags as string[]).includes(FLOW_TAG);
    const extent = Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM);
    expect(scenes.map((s) => s.name)).toEqual(c.glbScales.flatMap((scale) => ["body", "thermal", "structure"].map((mode) => `s${scale}-${mode}`)));
    for (const scene of scenes) {
      scene.updateMatrixWorld(true);
      const nodes: Object3D[] = [];
      scene.traverse((object) => void (tagged(object) && json.nodes.some((n) => n.name === object.name && n.mesh !== undefined) ? nodes.push(object) : undefined));
      if (!scene.name.endsWith("-thermal")) {
        expect(nodes, scene.name).toEqual([]);
        continue;
      }
      const scale = Number(/^s(\d+)-/u.exec(scene.name)![1]);
      const unit = flowUnitOf(scale, extent);
      const marks = flowMarks(field, field.timeS, flowDetailOf(scale));
      const reach = Math.max((FLOW_STYLE.streamMaxMm / 2) * unit, Math.hypot(FLOW_STYLE.headLengthMm, FLOW_STYLE.headWidthMm / 2) * unit) + FLOW_MODEL.chordMm * (scale / 1000) + vectors.tolerances.hugM;
      const streams = nodes.filter((n) => n.name.includes("-flow-stream-"));
      expect(streams.map((n) => n.name)).toEqual(marks.ribbons.map((r) => `${scene.name}-flow-stream-${r.stream}`));
      for (const [index, node] of streams.entries()) {
        const polyline = marks.ribbons[index]!.points.map((p) => new Vector3(...neutral(p)));
        let vertices = 0;
        node.traverse((object) => {
          const mesh = object as Object3D & { isMesh?: boolean; geometry: BufferGeometry };
          if (mesh.isMesh !== true) return;
          const position = mesh.geometry.getAttribute("position");
          for (let i = 0; i < position.count; i++) {
            const p = new Vector3().fromBufferAttribute(position, i).applyMatrix4(mesh.matrixWorld);
            expect(distanceTo(polyline, p), `${node.name}: a vertex off its stream`).toBeLessThanOrEqual(reach);
            vertices += 1;
          }
        });
        expect(vertices).toBeGreaterThan(20);
        const box = new Box3().setFromObject(node, true);
        const points = new Box3().setFromPoints(polyline);
        near(box.min.toArray(), points.min.toArray(), reach, `${node.name} lower corner`);
        near(box.max.toArray(), points.max.toArray(), reach, `${node.name} upper corner`);
      }
      expect(nodes.some((n) => n.name.endsWith("-flow-vapour"))).toBe(marks.puffs.some((p) => p.vapourExcessKgPerKg * 1000 > FLOW_STYLE.vapourThresholdGKg));
    }
  });
});
