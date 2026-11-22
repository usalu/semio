/** 🦴️ Lateral rest placement: a bone is oriented fully in its parent at rest (`Bone.angleDeg`, `Bone.abductionDeg`, `Bone.twistDeg`), offsets turn it about its own rest axes and its children hang from it; every view, anchor, node placement and validator rule reads the one chain.
 *
 * THE REFERENCE is scipy `Rotation` (the chain and the node placements), numpy (anchors) and shapely (extents of the solids) in 🐍️.py; the named cases are also hand-computed in the fixture.
 * Every consumer family (depiction and multiview, SVG, PDF, DXF, DWG, sheet, frames, cast and its levels of detail, the 3D model with GLB and FBX, structure, thermal) shows the lifted clavicle at the hand-computed place of the fixture (`consumers`); svgelements, jsdom, PDFium, ezdxf, ezdwg, three and numpy read the outputs back.
 * The interactive 3D stage has its own case: ../../🎯️targets/⚛️react/🧪️tests/🧊️model-lateral-rest/🟦️.tsx.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🦴️lateral-rest/🔣️.json — the vectors
 */
import { mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import Ajv from "ajv";
import { JSDOM } from "jsdom";
import { AnimationMixer, Box3, Vector3, type Object3D } from "three";
import { describe, expect, it } from "vitest";
import { poseModel } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { Anchor, Asset, AuthoredRepresentation, AuthoredVolume, Bone, BoneOffset, Clip, PlanarViewpoint, Pose, Shape, Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { DrawingScene, EvaluatedState, ModelScene, View } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { DEFAULT_CATALOG_ROOT } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { defaultDepictionOptions, modelScene } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { diagramOutOfPlane, diagramPolygons, peakOf } from "../../🔨️modules/🎨️depiction/🏗️structural-view/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { exportCast } from "../../🔨️modules/📤️export/🎭️cast/🟦️.ts";
import type { CastManifest } from "../../🔨️modules/📤️export/🎭️cast/🧬️schema/🟦️.ts";
import { resolveLod } from "../../🔨️modules/📤️export/🎭️cast/🧊️lod/🟦️.ts";
import { frameSchedule } from "../../🔨️modules/📤️export/🎞️frames/🟦️.ts";
import { writeFbx } from "../../🔨️modules/📤️export/🎬️fbx/🟦️.ts";
import { writeDxf } from "../../🔨️modules/📤️export/📐️dxf/🟦️.ts";
import { modelSheet, resolveSheet } from "../../🔨️modules/📤️export/📑️sheet/🟦️.ts";
import { PT_PER_MM, writePdf } from "../../🔨️modules/📤️export/📕️pdf/🟦️.ts";
import { configureDwg, writeDwg } from "../../🔨️modules/📤️export/🖊️dwg/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { writeGlb } from "../../🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { resolveViews, type DrawOptions } from "../../🔨️modules/🔭️multiview/🟦️.ts";
import { solveStructure } from "../../🔨️modules/🏗️structure/🟦️.ts";
import { modelOf, modelOfAsset } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { compose, placement, restFrames, solveChain, solveFrames, solveRig } from "../../🔨️modules/🦴️rig/🟦️.ts";
import { placementsOf, restPlacements } from "../../🔨️modules/🧊️volume/🦴️skeleton/🟦️.ts";
import { STAFFAGE_ROOT, ensureDwgWasm, fixture, pixel, requireTool, withScratch } from "../🧰️support/🧰️.ts";
import { python, renderPdf, svgShapeBoxes } from "../🧰️support/🐍️.ts";
import { readDwgBytes } from "../🧰️support/🖊️.ts";
import { loadFbx, loadGlb } from "../🧰️support/🧊️.ts";
import { extentOf, type Extent } from "../🧊️three-views/🧰️.ts";
requireTool("uv");

type Triple = [number, number, number];
type FrameVector = { origin: Triple; tip: Triple; u: Triple; v: Triple; w: Triple };
type Vectors = {
  base: string;
  scale: number;
  root: { x: number; y: number; angleDeg: number };
  frameTolerance: number;
  extentToleranceM: number;
  girdle: { bones: Bone[]; reparent: Record<string, Partial<Bone>>; sagittal: Record<string, Partial<Bone>> };
  anchors: Anchor[];
  shapes: Record<Viewpoint, Shape[]>;
  flatOnClavicle: Shape;
  cases: { name: string; pose: string; offsets: Record<string, BoneOffset>; frames: Record<string, FrameVector>; extents: Record<string, Partial<Extent>>; anchors: Record<string, Triple> }[];
  consumers: {
    pose: string;
    toleranceM: number;
    paperToleranceM: number;
    meshToleranceM: number;
    boxes: Record<PlanarViewpoint, Extent>;
    clip: Clip;
    instants: { timeS: number; liftDeg: number; front: Extent; tip: Triple }[];
    solids: { body: AuthoredVolume["body"][number]; structure: AuthoredVolume["structure"][number]; thermal: AuthoredVolume["thermal"][number] };
    capsule: { ahead: [number, number]; up: [number, number]; left: [number, number] };
    region: string;
    structure: { pose: string; bone: string; massKg: number; gravityMS2: number; leverM: number; shearN: number; momentNm: number; baseline: [number, number][] };
  };
};

const vectors = fixture<Vectors>("🦴️lateral-rest");
const sample = fixture<{ asset: Asset }>(vectors.base).asset;
const select = { kind: "scale", scale: { denominator: vectors.scale } } as const;
const stand = sample.poses.find((pose) => pose.id === "stand")!;

const dressed = (base: Asset, bones: readonly Bone[], shapes: Record<Viewpoint, Shape[]>): Asset => {
  const authored = base.representations[String(vectors.scale) as keyof Asset["representations"]] as AuthoredRepresentation;
  const poses: Pose[] = [...base.poses, ...vectors.cases.map((c) => ({ ...stand, id: c.pose, bones: Object.fromEntries(Object.entries(c.offsets).filter(([id]) => bones.some((bone) => bone.id === id))) }))];
  return { ...base, rig: { ...base.rig, bones }, anchors: [...base.anchors, ...vectors.anchors], poses, representations: { ...base.representations, [String(vectors.scale)]: { ...authored, geometry: shapes } } };
};

const girdleBones: Bone[] = [...sample.rig.bones.map((bone) => (vectors.girdle.reparent[bone.id] === undefined ? bone : { ...bone, ...vectors.girdle.reparent[bone.id] })), ...vectors.girdle.bones];
const girdled = dressed(sample, girdleBones, vectors.shapes);
const armShapes = Object.fromEntries((["elevation", "front", "plan"] as const).map((view) => [view, vectors.shapes[view].filter((shape) => shape.bone === "arm-l")])) as Record<Viewpoint, Shape[]>;
const sagittal = dressed(sample, sample.rig.bones.map((bone) => (vectors.girdle.sagittal[bone.id] === undefined ? bone : { ...bone, ...vectors.girdle.sagittal[bone.id] })), armShapes);

const extents = (state: EvaluatedState): Record<string, Extent> => {
  const out: Record<string, Extent> = {};
  for (const view of ["elevation", "front", "plan"] as const) for (const evaluated of state.shapes[view]) out[`${view}/${evaluated.shape.id}`] = extentOf([evaluated], true)!;
  return out;
};

const frameOf = (state: EvaluatedState, id: string): FrameVector => {
  const bone = state.bones[id]!;
  const triple = (p: { x: number; y: number; z: number }): Triple => [p.x, p.y, p.z];
  return { origin: triple(bone.frame.o), tip: triple(bone.tip), u: triple(bone.frame.u), v: triple(bone.frame.v), w: triple(bone.frame.w) };
};

const expectFrame = (label: string, actual: FrameVector, expected: FrameVector, tolerance: number): void => {
  for (const part of ["origin", "tip", "u", "v", "w"] as const) for (let i = 0; i < 3; i++) expect(Math.abs(actual[part][i]! - expected[part][i]!), `${label} ${part}[${i}]: ${actual[part][i]} vs ${expected[part][i]}`).toBeLessThanOrEqual(tolerance);
};

const expectExtent = (label: string, actual: Extent, expected: Partial<Extent>): void => {
  for (const [edge, value] of Object.entries(expected)) expect(Math.abs(actual[edge as keyof Extent] - value), `${label} ${edge}: ${actual[edge as keyof Extent]} vs ${value}`).toBeLessThanOrEqual(vectors.extentToleranceM);
};

const anchorOf = (state: EvaluatedState, id: string): Triple => [state.anchors[id]!.at.x, state.anchors[id]!.at.y, state.anchors[id]!.z];

describe("🦴️ the hand-computed vectors of the lateral rest placement", () => {
  for (const c of vectors.cases)
    it(c.name, () => {
      const state = evaluate(girdled, { kind: "pose", pose: c.pose }, select);
      expect(state.root.y, "the figure stands at the root height of the vectors").toBeCloseTo(vectors.root.y, 9);
      for (const [id, expected] of Object.entries(c.frames)) expectFrame(`${c.pose} ${id}`, frameOf(state, id), expected, 1e-6);
      const got = extents(state);
      for (const [key, part] of Object.entries(c.extents)) expectExtent(`${c.pose} ${key}`, got[key]!, part);
      for (const [id, at] of Object.entries(c.anchors)) for (let i = 0; i < 3; i++) expect(anchorOf(state, id)[i], `${c.pose} anchor ${id}[${i}]`).toBeCloseTo(at[i]!, 6);
    });

  it("keeps the exact right angles of the rest placement exact", () => {
    const rest = restFrames(girdled.rig);
    const clean = (v: readonly number[]): number[] => v.map((x) => x + 0);
    expect(clean(rest["clavicle-l"]!.frame.u)).toEqual([0, 0, 1]);
    expect(clean(rest["clavicle-l"]!.frame.w)).toEqual([-1, 0, 0]);
    expect(clean(rest["arm-l"]!.frame.u)).toEqual([0, -1, 0]);
    expect(clean(rest["arm-l"]!.frame.v)).toEqual([1, 0, 0]);
    expect(clean(rest["arm-l"]!.frame.w)).toEqual([0, 0, 1]);
    expect(clean(rest["arm-r"]!.frame.w)).toEqual([0, 0, 1]);
  });
});

describe("🦴️ the hanging arm equals the sagittal arm beside the symmetry plane", () => {
  const arms: Record<string, BoneOffset>[] = [{}, { "arm-l": { angleDeg: 90 } }, { "arm-l": { abductionDeg: 60 } }, { "arm-l": { angleDeg: 40, abductionDeg: 25, twistDeg: -30 }, "arm-r": { angleDeg: -20, abductionDeg: -35, twistDeg: 15 } }];
  const withArms = (asset: Asset, offsets: Record<string, BoneOffset>): Asset => ({ ...asset, poses: [...asset.poses, { ...stand, id: "probe", bones: offsets }] });

  it.each(arms.map((offsets) => ({ offsets, name: JSON.stringify(offsets) })))("arm frames and arm shapes agree for $name", ({ offsets }) => {
    const hung = evaluate(withArms(girdled, offsets), { kind: "pose", pose: "probe" }, select);
    const flat = evaluate(withArms(sagittal, offsets), { kind: "pose", pose: "probe" }, select);
    for (const id of ["arm-l", "arm-r"]) expectFrame(id, frameOf(hung, id), frameOf(flat, id), 1e-12);
    const [a, b] = [extents(hung), extents(flat)];
    for (const key of Object.keys(b)) expectExtent(key, a[key]!, b[key]!);
    for (let i = 0; i < 3; i++) expect(anchorOf(hung, "wrist-l")[i]).toBeCloseTo(anchorOf(flat, "wrist-l")[i]!, 12);
  });
});

describe("🦴️ a third-party library recomputes the chain", () => {
  it("solves every case with scipy, places the anchors with numpy and measures the solids with shapely", { timeout: 300_000 }, () => {
    const directory = mkdtempSync(join(tmpdir(), "staffage-lateral-rest-"));
    try {
      const input = join(directory, "input.json");
      const order = Object.keys(restFrames(girdled.rig));
      const rig = order.map((id) => girdled.rig.bones.find((bone) => bone.id === id)!).map((bone) => ({ id: bone.id, parent: bone.parent, x: bone.x, y: bone.y, z: bone.z ?? 0, angleDeg: bone.angleDeg, abductionDeg: bone.abductionDeg ?? 0, twistDeg: bone.twistDeg ?? 0, lengthM: bone.lengthM }));
      const solids = Object.fromEntries((["elevation", "front", "plan"] as const).map((view) => [view, vectors.shapes[view].filter((shape) => shape.solid !== undefined)]));
      writeFileSync(input, JSON.stringify({ rig, root: vectors.root, cases: Object.fromEntries(vectors.cases.map((c) => [c.pose, c.offsets])), solids, anchors: vectors.anchors }));
      type Reference = { frames: Record<string, FrameVector>; placements: Record<string, { translation: Triple; rotation: [number, number, number, number] }>; extents: Record<string, Extent>; anchors: Record<string, Triple> };
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🦴️lateral-rest/🐍️.py"), "geometry", [input])) as Record<string, Reference>;
      for (const c of vectors.cases) {
        const state = evaluate(girdled, { kind: "pose", pose: c.pose }, select);
        const expected = reference[c.pose]!;
        for (const bone of girdled.rig.bones) expectFrame(`${c.pose} ${bone.id} (scipy)`, frameOf(state, bone.id), expected.frames[bone.id]!, vectors.frameTolerance);
        const chain = solveChain(girdled.rig, c.offsets, vectors.root);
        for (const bone of girdled.rig.bones) for (let i = 0; i < 3; i++) expect(chain[bone.id]!.frame.w[i], `${c.pose} ${bone.id} chain w[${i}]`).toBeCloseTo(expected.frames[bone.id]!.w[i]!, 9);
        const placed = placementsOf(girdled, Object.fromEntries(Object.entries(state.bones).map(([id, bone]) => [id, bone.frame])));
        for (const bone of girdled.rig.bones) {
          const [ours, theirs] = [placed[bone.id]!, expected.placements[bone.id]!];
          for (let i = 0; i < 3; i++) expect(ours.translation[i], `${c.pose} ${bone.id} translation[${i}]`).toBeCloseTo(theirs.translation[i]!, 9);
          const sign = ours.rotation.reduce((sum, value, i) => sum + value * theirs.rotation[i]!, 0) < 0 ? -1 : 1;
          for (let i = 0; i < 4; i++) expect(ours.rotation[i]! * sign, `${c.pose} ${bone.id} rotation[${i}]`).toBeCloseTo(theirs.rotation[i]!, 9);
        }
        const got = extents(state);
        for (const [key, extent] of Object.entries(expected.extents)) expectExtent(`${c.pose} ${key} (shapely)`, got[key]!, extent);
        for (const [id, at] of Object.entries(expected.anchors)) for (let i = 0; i < 3; i++) expect(anchorOf(state, id)[i], `${c.pose} anchor ${id}[${i}] (numpy)`).toBeCloseTo(at[i]!, 9);
      }
      const rest = restPlacements(girdled);
      const atOrigin = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🦴️lateral-rest/🐍️.py"), "geometry", [(writeFileSync(input, JSON.stringify({ rig, root: { x: 0, y: 0, angleDeg: 0 }, cases: { rest: {} }, solids: {}, anchors: [] })), input)])) as Record<string, Reference>;
      for (const bone of girdled.rig.bones) {
        const [ours, theirs] = [rest[bone.id]!, atOrigin.rest!.placements[bone.id]!];
        for (let i = 0; i < 3; i++) expect(ours.translation[i], `rest ${bone.id} translation[${i}]`).toBeCloseTo(theirs.translation[i]!, 9);
        const sign = ours.rotation.reduce((sum, value, i) => sum + value * theirs.rotation[i]!, 0) < 0 ? -1 : 1;
        for (let i = 0; i < 4; i++) expect(ours.rotation[i]! * sign, `rest ${bone.id} rotation[${i}]`).toBeCloseTo(theirs.rotation[i]!, 9);
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

describe("🦴️ the contract and the validator know the rest placement", () => {
  const schema = JSON.parse(readFileSync(join(STAFFAGE_ROOT, "🧬️schema/🔣️.json"), "utf8")) as { $id: string };
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const bone = ajv.compile({ $ref: `${schema.$id}#/$defs/Bone` });

  it("accepts abductionDeg and twistDeg on a bone and refuses values beyond half a turn", () => {
    for (const candidate of girdleBones) expect(bone(candidate), JSON.stringify(bone.errors)).toBe(true);
    expect(bone({ ...vectors.girdle.bones[0]!, twistDeg: 181 })).toBe(false);
    expect(bone({ ...vectors.girdle.bones[0]!, abductionDeg: -181 })).toBe(false);
  });

  const added = new Set(vectors.girdle.bones.map((candidate) => candidate.id));
  const codes = (asset: Asset): string[] => validateAsset(asset).filter((issue) => !(issue.code.startsWith("volumes-") && added.has((issue.data as { bone?: string } | undefined)?.bone ?? ""))).map((issue) => `${issue.code} ${issue.path}`).sort();

  it("adds no finding for the shoulder girdle with a solid clavicle, apart from the volume rules that ask the new bones for their own solids", { timeout: 300_000 }, () => {
    const baseline = codes(dressed(sample, sample.rig.bones, armShapes));
    expect(codes(girdled).filter((line) => !baseline.includes(line))).toEqual([]);
  });

  it("reports a flat elevation shape on the clavicle, whose plane is edge-on at rest, and the flat arm that an uncompensated shrug carries out of the paper, although no pose spreads or twists a bone", { timeout: 300_000 }, () => {
    const posed = (asset: Asset, keep: readonly string[]): Asset => ({ ...asset, poses: asset.poses.filter((pose) => sample.poses.some((own) => own.id === pose.id) || keep.includes(pose.id)) });
    const ambiguous = (asset: Asset) => validateAsset(asset).filter((issue) => issue.code === "lateral-ambiguous").map((issue) => issue.data as { bone: string; shapes: string[]; bands: string[]; thetaMaxDeg: number });
    const flat = dressed(sample, girdleBones, { ...vectors.shapes, elevation: [...vectors.shapes.elevation, vectors.flatOnClavicle] });
    expect(ambiguous(posed(flat, ["rest", "arm-flexed"]))).toMatchObject([{ bone: "clavicle-l", shapes: [vectors.flatOnClavicle.id], bands: [`1:${vectors.scale} elevation`], thetaMaxDeg: 90 }]);
    expect(ambiguous(posed(girdled, ["rest", "arm-flexed"]))).toEqual([]);
    const shrugged = ambiguous(posed(girdled, ["rest", "shrug"]));
    expect(shrugged.map((data) => data.bone).sort()).toEqual(["arm-l", "arm-r"]);
    for (const data of shrugged) {
      expect(data.thetaMaxDeg).toBeCloseTo(30, 6);
      expect(data.bands.every((band) => band.endsWith("elevation"))).toBe(true);
    }
    expect(ambiguous(posed(girdled, ["rest", "shrug-hanging"])).flatMap((data) => data.bands).filter((band) => band.endsWith("elevation"))).toEqual([]);
  });
});

describe("🦴️ every consumer of the bone frames carries the lateral bone", () => {
  const consumers = vectors.consumers;
  const fitted: Asset = (() => {
    const volume = girdled.volumes!["50"] as AuthoredVolume;
    const authored = girdled.representations[String(vectors.scale) as keyof Asset["representations"]] as AuthoredRepresentation;
    const regioned = Object.fromEntries((["elevation", "front", "plan"] as const).map((view) => [view, vectors.shapes[view].map((shape): Shape => ({ ...shape, region: shape.bone === "clavicle-l" ? consumers.region : "arms" }))])) as Record<Viewpoint, Shape[]>;
    return {
      ...girdled,
      clips: [...girdled.clips, consumers.clip],
      thermal: { ...girdled.thermal, respiration: { ...girdled.thermal.respiration!, motion: [] } },
      representations: { ...girdled.representations, [String(vectors.scale)]: { ...authored, geometry: regioned } },
      volumes: { ...girdled.volumes, "50": { ...volume, body: [...volume.body, consumers.solids.body], thermal: [...volume.thermal, consumers.solids.thermal], structure: [...volume.structure, consumers.solids.structure] } },
    } as Asset;
  })();
  const VIEWPOINTS = ["elevation", "front", "plan"] as const;
  const PX_PER_MM = 96 / 25.4;
  const source = { kind: "scale", denominator: vectors.scale } as const;
  const shrug = { kind: "pose", pose: consumers.pose } as const;
  const optionsOf = (view: View): DrawOptions => {
    const { view: _view, viewpoint: _viewpoint, scale: _scale, representation: _representation, ...rest } = defaultDepictionOptions(view, "en", vectors.scale);
    return rest as DrawOptions;
  };
  const sceneOf = (view: View, viewpoint: PlanarViewpoint, query: Parameters<typeof resolveViews>[1] = shrug): DrawingScene => {
    const drawn = resolveViews(fitted, query, source, view, optionsOf(view)).views[viewpoint];
    if (drawn.kind !== "drawing") throw new Error(`${view} ${viewpoint}: ${drawn.reason.en}`);
    return drawn.scene;
  };
  const boxOf = (points: readonly { readonly x: number; readonly y: number }[]): Extent => ({ minX: Math.min(...points.map((p) => p.x)), maxX: Math.max(...points.map((p) => p.x)), minY: Math.min(...points.map((p) => p.y)), maxY: Math.max(...points.map((p) => p.y)) });
  const toModel = (scene: { readonly mmPerMetre: number; readonly origin: { readonly xMm: number; readonly yMm: number } }, box: Extent): Extent => ({ minX: (box.minX - scene.origin.xMm) / scene.mmPerMetre, maxX: (box.maxX - scene.origin.xMm) / scene.mmPerMetre, minY: (scene.origin.yMm - box.maxY) / scene.mmPerMetre, maxY: (scene.origin.yMm - box.minY) / scene.mmPerMetre });
  const clavicleItem = (scene: DrawingScene) => {
    const items = scene.layers.flatMap((layer) => layer.items).filter((item) => item.kind === "path" && item.bone === "clavicle-l" && item.id === "shape:clavicle");
    expect(items, `${scene.metadata.view} ${scene.metadata.viewpoint}: one drawn clavicle`).toHaveLength(1);
    const item = items[0]!;
    if (item.kind !== "path") throw new Error("the clavicle is a path");
    return item;
  };
  const drawnBox = (scene: DrawingScene): Extent => toModel(scene.metadata, boxOf(clavicleItem(scene).commands.flatMap((command) => (command.op === "Z" ? [] : [{ x: command.x, y: command.y }]))));
  const near = (label: string, actual: Extent, expected: Extent, tolerance: number): void => {
    for (const edge of ["minX", "maxX", "minY", "maxY"] as const) expect(Math.abs(actual[edge] - expected[edge]), `${label} ${edge}: ${actual[edge]} vs ${expected[edge]}`).toBeLessThanOrEqual(tolerance);
  };
  const pathBox = (svg: string, group: string): Extent => {
    const document = new JSDOM(svg, { contentType: "image/svg+xml" }).window.document;
    const element = document.getElementById(group);
    expect(element, `the SVG has the group ${group}`).not.toBeNull();
    expect(element!.getAttribute("data-bone")).toBe("clavicle-l");
    const numbers = [...element!.querySelector("path")!.getAttribute("d")!.matchAll(/-?\d+(?:\.\d+)?(?:e-?\d+)?/gu)].map((match) => Number(match[0]));
    return boxOf(numbers.flatMap((value, index) => (index % 2 === 0 ? [{ x: value, y: numbers[index + 1]! }] : [])));
  };
  const polylineBoxes = (lines: readonly { readonly points: readonly (readonly number[])[] }[]): Extent[] => lines.map((line) => boxOf(line.points.map(([x, y]) => ({ x: x!, y: y! }))));
  const hasBox = (label: string, boxes: readonly Extent[], expected: Extent, tolerance: number): void => {
    const distance = (box: Extent): number => Math.max(...(["minX", "maxX", "minY", "maxY"] as const).map((edge) => Math.abs(box[edge] - expected[edge])));
    expect(Math.min(...boxes.map(distance)), `${label}: a polyline with the box ${JSON.stringify(expected)} among ${JSON.stringify(boxes)}`).toBeLessThanOrEqual(tolerance);
  };
  const reopenDxf = (text: string): Promise<{ errors: string[]; lwpolylines: { points: number[][] }[] }> => withScratch((directory) => {
    const file = join(directory, "scene.dxf");
    writeFileSync(file, text);
    return JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/📐️dxf-reopen/🐍️.py"), "documents", [file])) as { errors: string[]; lwpolylines: { points: number[][] }[] };
  });
  const cadOf = (viewpoint: PlanarViewpoint): ModelScene => modelScene(evaluate(fitted, shrug, select), { viewpoint });

  it("draws the lifted clavicle at the hand-computed place in the three views of one state (depiction, multiview)", () => {
    for (const viewpoint of VIEWPOINTS) for (const view of ["light-solid", "dark-line"] as const) near(`${view} ${viewpoint}`, drawnBox(sceneOf(view, viewpoint)), consumers.boxes[viewpoint], consumers.toleranceM);
  });

  it("writes it to SVG where svgelements and jsdom find it", async () => {
    await withScratch((directory) => {
      for (const viewpoint of VIEWPOINTS) {
        const scene = sceneOf("light-solid", viewpoint);
        const svg = writeSvg(scene);
        near(`svg ${viewpoint} (jsdom)`, toModel(scene.metadata, pathBox(svg, "shape-clavicle")), consumers.boxes[viewpoint], consumers.paperToleranceM);
        const file = join(directory, `${viewpoint}.svg`);
        writeFileSync(file, svg);
        hasBox(`svg ${viewpoint} (svgelements)`, svgShapeBoxes(file).map((box) => toModel(scene.metadata, { minX: box.minX / PX_PER_MM, maxX: box.maxX / PX_PER_MM, minY: box.minY / PX_PER_MM, maxY: box.maxY / PX_PER_MM })), consumers.boxes[viewpoint], consumers.paperToleranceM);
      }
    });
  }, 300_000);

  it("writes it to PDF where PDFium paints it", async () => {
    await withScratch((directory) => {
      const scene = sceneOf("light-solid", "front");
      const file = join(directory, "front.pdf");
      writeFileSync(file, writePdf([scene]));
      const dpi = 1200;
      const image = renderPdf(file, dpi);
      const at = (lateral: number, height: number): readonly [number, number, number] => pixel(image, Math.round(((scene.metadata.origin.xMm + lateral * scene.metadata.mmPerMetre) * PT_PER_MM * dpi) / 72), Math.round(((scene.metadata.origin.yMm - height * scene.metadata.mmPerMetre) * PT_PER_MM * dpi) / 72));
      const box = consumers.boxes.front;
      const [inside, above, medial] = [at((box.minX + box.maxX) / 2, (box.minY + box.maxY) / 2), at((box.minX + box.maxX) / 2, box.maxY + 0.01), at(box.minX - 0.01, (box.minY + box.maxY) / 2)];
      expect(above, "paper above the clavicle").toEqual(medial);
      expect(inside, "the clavicle is painted at its centre").not.toEqual(above);
    });
  }, 300_000);

  it("writes it to DXF and DWG where ezdxf and ezdwg read it back", async () => {
    configureDwg({ wasm: readFileSync(ensureDwgWasm()) });
    for (const viewpoint of VIEWPOINTS) {
      const scene = cadOf(viewpoint);
      const dxf = await reopenDxf(writeDxf(scene));
      expect(dxf.errors).toEqual([]);
      hasBox(`dxf ${viewpoint}`, polylineBoxes(dxf.lwpolylines), consumers.boxes[viewpoint], consumers.toleranceM);
      const dwg = await readDwgBytes(await writeDwg(scene));
      hasBox(`dwg ${viewpoint}`, polylineBoxes(dwg.entities.filter((entity) => entity.type === "LWPOLYLINE").map((entity) => ({ points: entity.points! }))), consumers.boxes[viewpoint], consumers.toleranceM);
    }
  }, 600_000);

  it("lays it out on the multiview sheet, as drawing and as CAD model", async () => {
    const { sheet } = resolveSheet(fitted, shrug, source, "light-solid", optionsOf("light-solid"), "first", "en");
    const paths = [...new JSDOM(writeSvg(sheet), { contentType: "image/svg+xml" }).window.document.querySelectorAll('[data-bone="clavicle-l"] path')].map((path) => {
      const numbers = [...path.getAttribute("d")!.matchAll(/-?\d+(?:\.\d+)?(?:e-?\d+)?/gu)].map((match) => Number(match[0]));
      return boxOf(numbers.flatMap((value, index) => (index % 2 === 0 ? [{ x: value, y: numbers[index + 1]! }] : [])));
    });
    expect(paths).toHaveLength(3);
    for (const viewpoint of VIEWPOINTS) hasBox(`sheet ${viewpoint}`, paths.map((box) => toModel({ mmPerMetre: sheet.metadata.mmPerMetre, origin: sheet.sheet.originsMm[viewpoint] }, box)), consumers.boxes[viewpoint], consumers.paperToleranceM);
    const cad = modelSheet(evaluate(fitted, shrug, select), { projection: "first", denominator: vectors.scale, language: "en", axonometric: false });
    const size = (box: Extent): [number, number] => [box.maxX - box.minX, box.maxY - box.minY];
    const sizes = polylineBoxes((await reopenDxf(writeDxf(cad))).lwpolylines).map(size);
    for (const viewpoint of VIEWPOINTS) {
      const [width, height] = size(consumers.boxes[viewpoint]);
      expect(Math.min(...sizes.map(([w, h]) => Math.max(Math.abs(w - width), Math.abs(h - height)))), `sheet dxf ${viewpoint}: a polyline of ${width} x ${height} m`).toBeLessThanOrEqual(consumers.toleranceM);
    }
  }, 300_000);

  it("samples a clip on the clavicle frame by frame", () => {
    const schedule = frameSchedule(consumers.clip, 20);
    expect(schedule.count).toBe(20);
    for (const instant of consumers.instants) {
      expect(schedule.times.some((timeS) => Math.abs(timeS - instant.timeS) < 1e-12), `the schedule has ${instant.timeS} s`).toBe(true);
      const query = { kind: "clip", clip: consumers.clip.id, timeS: instant.timeS } as const;
      near(`clip ${instant.timeS} s`, drawnBox(sceneOf("light-solid", "front", query)), instant.front, consumers.toleranceM);
      const tip = evaluate(fitted, query, select).bones["clavicle-l"]!.tip;
      [tip.x, tip.y, tip.z].forEach((value, index) => expect(value, `tip[${index}] at ${instant.timeS} s`).toBeCloseTo(instant.tip[index]!, 9));
    }
  });

  it("exports it as a cast at every level of detail", async () => {
    const root = mkdtempSync(join(tmpdir(), "staffage-lateral-cast-"));
    try {
      writeFileSync(join(root, "🔣️.json"), readFileSync(join(DEFAULT_CATALOG_ROOT, "🔣️.json")));
      symlinkSync(join(DEFAULT_CATALOG_ROOT, "🖨️print-profiles"), join(root, "🖨️print-profiles"), "junction");
      const manifest = JSON.parse(readFileSync(join(DEFAULT_CATALOG_ROOT, "🔣️.json"), "utf8")) as { categories: { id: string; folder: string }[] };
      const folder = join(root, manifest.categories.find((category) => category.id === fitted.category)!.folder, fitted.id);
      mkdirSync(folder, { recursive: true });
      writeFileSync(join(folder, "🔣️.json"), JSON.stringify({ version: "semio.staffage/v2", asset: fitted }));
      const casts = { version: 1, casts: { still: { asset: fitted.id, pose: consumers.pose, view: "light-solid", theme: "light", viewpoint: "front", scale: vectors.scale }, moving: { asset: fitted.id, clip: consumers.clip.id, view: "light-solid", theme: "light", viewpoint: "front", scale: vectors.scale, fps: 20 } } } as unknown as CastManifest;
      const [still] = await exportCast(casts, ["still"], join(root, "out-still"), { catalog: root });
      near("cast partsM", boxOf(still!.logical.partsM["clavicle"]!.map(([x, y]) => ({ x, y }))), consumers.boxes.front, consumers.paperToleranceM);
      const half = consumers.instants.find((instant) => instant.timeS === 0.5)!;
      for (const [level, frames] of [["full", 20], ["preview", 10]] as const) {
        const [index] = await exportCast(casts, ["moving"], join(root, `out-${level}`), { catalog: root, lod: resolveLod(level, 20) });
        expect(index!.frames, level).toHaveLength(frames);
        const svg = readFileSync(join(root, `out-${level}`, index!.cast, index!.frames[Math.round(half.timeS * index!.fps)]!), "utf8");
        near(`cast ${level} at ${half.timeS} s`, toModel({ mmPerMetre: index!.mmPerMetre, origin: { xMm: index!.originMm[0], yMm: index!.originMm[1] } }, pathBox(svg, "shape-clavicle")), half.front, consumers.paperToleranceM);
      }
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 600_000);

  const neutralTip = (world: readonly number[], length: number): Triple => [world[12]! + world[0]! * length, world[13]! + world[1]! * length, world[14]! + world[2]! * length];
  const expectTip = (label: string, world: readonly number[], tip: Triple, tolerance: number): void => neutralTip(world, 0.17).forEach((value, index) => expect(Math.abs(value - tip[2 - index]!), `${label} tip[${index}]: ${value} vs ${tip[2 - index]}`).toBeLessThanOrEqual(tolerance));

  it("poses the 3D model: the node of the clavicle reaches its hand-computed tip in a pose and through a clip (kernel replay of modelOf and of the viewer's modelOfAsset)", async () => {
    const half = consumers.instants.find((instant) => instant.timeS === 0.5)!;
    const single = await modelOf(fitted, { scale: vectors.scale, view: "light-solid", poses: [consumers.pose], clips: [consumers.clip.id] });
    const whole = await modelOfAsset(fitted, { at: shrug, poses: [consumers.pose], clips: [consumers.clip.id], scales: [vectors.scale] });
    for (const [label, document] of [["modelOf", single], ["modelOfAsset", whole]] as const) {
      const posed = poseModel(document, consumers.pose);
      expect(posed.issues, label).toEqual([]);
      expectTip(`${label} pose`, posed.world["clavicle-l"]!, half.tip, 1e-9);
      for (const instant of consumers.instants) {
        const sampled = poseModel(document, { animation: consumers.clip.id, timeS: instant.timeS });
        expect(sampled.issues, `${label} ${instant.timeS}`).toEqual([]);
        expectTip(`${label} clip ${instant.timeS} s`, sampled.world["clavicle-l"]!, instant.tip, 1e-4);
        const arm = sampled.world["arm-l"]!;
        [arm[12]!, arm[13]!, arm[14]!].forEach((value, index) => expect(Math.abs(value - instant.tip[2 - index]!), `${label} the arm hangs from the tip at ${instant.timeS} s`).toBeLessThanOrEqual(1e-4));
      }
    }
  }, 300_000);

  it("exports the 3D model: three loads the GLB with the clavicle solid at its place and replays the FBX clip", async () => {
    const { scene } = await loadGlb(await writeGlb(fitted, { pose: consumers.pose, scales: [vectors.scale] }));
    scene.updateMatrixWorld(true);
    const bone = scene.getObjectByName("clavicle-l")!;
    expect(bone, "the GLB has the clavicle node").toBeDefined();
    const box = new Box3();
    const walk = (object: Object3D): void => {
      if ((object as { isMesh?: boolean }).isMesh) box.expandByObject(object, true);
      for (const child of object.children) if (!fitted.rig.bones.some((candidate) => candidate.id === child.name && candidate.id !== "clavicle-l")) walk(child);
    };
    walk(bone);
    expect(box.isEmpty(), "the clavicle node carries its solid").toBe(false);
    const got = { left: [box.min.x, box.max.x], up: [box.min.y, box.max.y], ahead: [box.min.z, box.max.z] } as const;
    for (const axis of ["ahead", "up", "left"] as const) for (const end of [0, 1] as const) expect(Math.abs(got[axis][end] - consumers.capsule[axis][end]), `glb ${axis}[${end}]: ${got[axis][end]} vs ${consumers.capsule[axis][end]}`).toBeLessThanOrEqual(consumers.meshToleranceM);
    const fbx = loadFbx(await writeFbx(fitted, { clip: consumers.clip.id, scale: vectors.scale }));
    const mixer = new AnimationMixer(fbx);
    mixer.clipAction(fbx.animations[0]!).play();
    for (const instant of consumers.instants) {
      mixer.setTime(instant.timeS);
      fbx.updateMatrixWorld(true);
      const at = fbx.getObjectByName("arm-l")!.getWorldPosition(new Vector3());
      [at.z, at.y, at.x].forEach((value, index) => expect(Math.abs(value - instant.tip[index]!), `fbx arm origin[${index}] at ${instant.timeS} s: ${value} vs ${instant.tip[index]}`).toBeLessThanOrEqual(1e-3));
    }
  }, 300_000);

  it("solves the structure through the lateral member and projects its diagram to the views it shows in", () => {
    const wanted = consumers.structure;
    const state = evaluate(fitted, { kind: "pose", pose: wanted.pose }, select);
    const result = solveStructure(state);
    const joint = result.joints[wanted.bone]!;
    expect(wanted.massKg * wanted.gravityMS2).toBeCloseTo(wanted.shearN, 9);
    expect(wanted.shearN * wanted.leverM).toBeCloseTo(wanted.momentNm, 9);
    expect(Math.abs(joint.shearN)).toBeCloseTo(wanted.shearN, 9);
    expect(Math.abs(joint.momentNm)).toBeCloseTo(wanted.momentNm, 9);
    expect(Math.abs(joint.mxNm)).toBeCloseTo(wanted.momentNm, 9);
    for (const zero of [joint.axialN, joint.shearZN, joint.torsionNm, joint.momentLateralNm, joint.myNm, joint.mzNm, joint.fxN, joint.fzN]) expect(zero).toBeCloseTo(0, 9);
    for (const station of result.members[wanted.bone]!.stations) expect(Math.abs(station.momentNm), `bending at ${station.s} m`).toBeCloseTo(wanted.shearN * (wanted.leverM - station.s), 9);
    const directory = mkdtempSync(join(tmpdir(), "staffage-lateral-structure-"));
    try {
      const input = join(directory, "input.json");
      const order = Object.keys(restFrames(fitted.rig));
      const rig = order.map((id) => fitted.rig.bones.find((bone) => bone.id === id)!).map((bone) => ({ id: bone.id, parent: bone.parent, x: bone.x, y: bone.y, z: bone.z ?? 0, angleDeg: bone.angleDeg, abductionDeg: bone.abductionDeg ?? 0, twistDeg: bone.twistDeg ?? 0, lengthM: bone.lengthM, ...(bone.mass === undefined ? {} : { mass: { kg: bone.mass.kg, comX: bone.mass.comX, comY: bone.mass.comY } }) }));
      writeFileSync(input, JSON.stringify({ rig, root: vectors.root, cases: Object.fromEntries(vectors.cases.map((c) => [c.pose, c.offsets])), solids: {}, anchors: [], joints: [wanted.bone, "arm-l"], gravityMS2: wanted.gravityMS2 }));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🦴️lateral-rest/🐍️.py"), "geometry", [input])) as Record<string, { joints: Record<string, { moment: Triple; torsionNm: number; momentNm: number; momentLateralNm: number; axialN: number; shearN: number; shearZN: number }> }>;
      for (const c of vectors.cases) {
        const solved = solveStructure(evaluate(fitted, { kind: "pose", pose: c.pose }, select));
        for (const bone of [wanted.bone, "arm-l"]) {
          const [ours, theirs] = [solved.joints[bone]!, reference[c.pose]!.joints[bone]!];
          [ours.mxNm, ours.myNm, ours.mzNm].forEach((value, index) => expect(value, `${c.pose} ${bone} moment[${index}] (numpy)`).toBeCloseTo(theirs.moment[index]!, 6));
          for (const key of ["torsionNm", "momentNm", "momentLateralNm", "axialN", "shearN", "shearZN"] as const) expect(ours[key], `${c.pose} ${bone} ${key} (numpy)`).toBeCloseTo(theirs[key], 6);
        }
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
    const peak = peakOf(result, "moment");
    const only = (bone: string): boolean => bone === wanted.bone;
    const polygons = diagramPolygons(state, result, "moment", "front", peak, (p) => p, 0.1, false, only);
    expect(polygons.length, "front: the bending diagram of the clavicle is drawn").toBeGreaterThan(0);
    const points = polygons.flatMap((polygon) => polygon.points);
    for (const [x, y] of wanted.baseline) expect(Math.min(...points.map((p) => Math.hypot(p.x - x, p.y - y))), `front: the baseline reaches (${x}, ${y})`).toBeLessThanOrEqual(1e-9);
    expect(diagramPolygons(state, result, "moment", "elevation", peak, (p) => p, 0.1, false, only), "the side elevation looks along the clavicle: no ordinate").toEqual([]);
    for (const kind of ["moment", "shear"] as const) expect(diagramOutOfPlane(state, result, kind, "plan", peakOf(result, kind), only).map((mark) => mark.bone), `the plan looks along the ${kind} ordinate of the clavicle and marks it`).toEqual([wanted.bone]);
  }, 300_000);

  it("paints its thermal region and its force and moment views at the same place", () => {
    for (const viewpoint of VIEWPOINTS) {
      const thermal = sceneOf("thermal", viewpoint);
      expect(clavicleItem(thermal).region).toBe(consumers.region);
      near(`thermal ${viewpoint}`, drawnBox(thermal), consumers.boxes[viewpoint], consumers.toleranceM);
      for (const view of ["forces", "moments"] as const) near(`${view} ${viewpoint}`, drawnBox(sceneOf(view, viewpoint)), consumers.boxes[viewpoint], consumers.toleranceM);
    }
  }, 300_000);
});

describe("🦴️ a sagittal rig is untouched", () => {
  it("needs no three-dimensional chain and equals the plain planar composition bit for bit", () => {
    expect(solveFrames(sample.rig, {}, vectors.root)).toBeNull();
    const world = solveRig(sample.rig, {}, vectors.root);
    const planar: Record<string, ReturnType<typeof placement>> = {};
    for (const id of Object.keys(restFrames(sample.rig))) {
      const b = sample.rig.bones.find((candidate) => candidate.id === id)!;
      planar[id] = compose(b.parent === null ? placement(vectors.root.x, vectors.root.y, vectors.root.angleDeg) : planar[b.parent]!, placement(b.x, b.y, b.angleDeg));
    }
    expect(world).toEqual(planar);
  });

  it("solves the three-dimensional chain of a sagittal rig to the same affines", () => {
    const chain = solveChain(sample.rig, { "arm-l": { angleDeg: 35 }, torso: { angleDeg: -10 } }, vectors.root);
    const world = solveRig(sample.rig, { "arm-l": { angleDeg: 35 }, torso: { angleDeg: -10 } }, vectors.root);
    for (const id of Object.keys(world)) world[id]!.forEach((value, i) => expect(chain[id]!.affine[i]).toBeCloseTo(value, 12));
  });
});
