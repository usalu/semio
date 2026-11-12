/** 🔭️ Multiview coverage: the matrix asset × scale × viewpoint × mode × pose, clip instant and transition, and the cross-cutting reference cases of the top, front and side requirement.
 *
 * THE REFERENCES are third-party: svgelements recomputes the bounding box of exported SVG paths, scipy the Hausdorff distance of the derived-view detection, shapely the gap of the printability vector.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ./🧰️.ts — the matrix engine
 * @see ./🪞️derived.ts — derived-view detection
 * @see ../../🧫️fixtures/🔭️multiview-coverage/🔣️.json — the vectors
 */
import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, PlanarViewpoint } from "../../🧬️schema/🟦️.ts";
import { VIEWS, type DrawingScene, type EvaluatedShape, type EvaluationQuery, type View } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { modelScene } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { resolveViews, type DrawOptions, type MultiView } from "../../🔨️modules/🔭️multiview/🟦️.ts";
import { writeDxf } from "../../🔨️modules/📤️export/📐️dxf/🟦️.ts";
import { writePdf } from "../../🔨️modules/📤️export/📕️pdf/🟦️.ts";
import { parseArguments, runExport } from "../../🔨️modules/📤️export/🎬️cli/🟦️.ts";
import { exportFilename } from "../../🎯️targets/⚛️react/🔨️modules/📤️downloads/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { resolveRepresentation } from "../../🔨️modules/🖨️representation/🟦️.ts";
import { solveStructure } from "../../🔨️modules/🏗️structure/🟦️.ts";
import { solveTransport } from "../../🔨️modules/💨️transport/🟦️.ts";
import { UNCHECKED_PROFILE, rolesAsset, rolesAssetWith, sampleAsset } from "../../🧫️fixtures/🧱️synthetic-figure/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";
import { MODE_LIST, PRESET_KEYS, UNIFORM_INSTANTS, VIEWPOINT_LIST, assemble, buildMatrix, cellKey, clipInstants, drawOptions, extentOf, hash, loadCatalog, scaleEntries, sceneBox, seeded, subjects, toleranceM, transitionPairs, universe, LONG_CLIP_S, LONG_UNIFORM_INSTANTS } from "./🧰️.ts";
import { TRANSFORMS, bestTransform, derivedFlags, pointSet } from "./🪞️derived.ts";

type Vectors = {
  viewpoints: PlanarViewpoint[];
  modes: View[];
  presetDenominators: number[];
  sampling: { uniformInstants: number; longClipSeconds: number; longUniformInstants: number; transitionFractions: number[]; transitionRandomPairs: number };
  paper: { toleranceMm: number; catalogAssets: string[]; scales: { denominator: number; mmPerMetre: number; expectedMm: { width: number; height: number } }[] };
  handedness: { heldObjects: { asset: string; pose: string; anchor: string; shapePattern?: string; zSign: number }[] };
  motion: { rootMotion: { asset: string; clip: string; durationS: number; advanceM: number; toleranceM: number }; loop: { asset: string; clip: string; cycles: number[] } };
  thermal: { movingEmitter: { asset: string; clip: string; times: number[]; followToleranceM: number } };
  printability: { gap: { denominator: number; barWidthM: number; gapM: number; bodyPenMm: number; expectedInkGapMm: number; minGapMm: number }; coarseAssets: string[] };
  determinism: { asset: string; clip: string; timeS: number; scale: number };
  scrubbing: { asset: string; clip: string; samples: number; seed: number; switches: PlanarViewpoint[] };
  derived: { cases: { name: string; elevation: number[][]; other: number[][]; flagged: boolean }[] };
};

const vectors = fixture<Vectors>("🔭️multiview-coverage");
const oracle = join(dirname(fileURLToPath(import.meta.url)), "🐍️.py");
const catalog = loadCatalog();
const byId = new Map(catalog.map((asset) => [asset.id, asset]));
const asset = (id: string): Asset => byId.get(id) ?? (() => { throw new Error(`the catalog has no asset "${id}"`); })();
const sample = sampleAsset();
const oracleUp = requireTool("uv");

const draw = (target: Asset, query: EvaluationQuery, denominator: number, mode: View = "light-line", extra: Partial<DrawOptions> = {}): MultiView =>
  resolveViews(target, query, { kind: "scale", denominator }, mode, { ...drawOptions(mode, denominator), ...extra } as DrawOptions);
const drawingOf = (multi: MultiView, view: PlanarViewpoint) => {
  const found = multi.views[view];
  if (found.kind !== "drawing") throw new Error(`${view} is unavailable: ${found.code} ${found.reason.en}`);
  return found;
};
const sceneJson = (scene: DrawingScene): string => JSON.stringify(scene);
const sha = (text: string | Uint8Array): string => createHash("sha256").update(text).digest("hex");
const uncheckedProfile = { profile: UNCHECKED_PROFILE } as Partial<DrawOptions>;
const idsOf = (shapes: readonly EvaluatedShape[], pattern?: string): Set<string> => new Set(shapes.filter((e) => e.shape.role !== "ground" && (pattern === undefined || e.shape.id.includes(pattern))).map((e) => `shape:${e.shape.id}`));

describe("🔭️ multiview coverage engine", () => {
  it("enumerates the universe from the declarations: four presets, band edges, alternatives, three viewpoints, seven modes", () => {
    expect(VIEWPOINT_LIST).toEqual(vectors.viewpoints);
    expect(MODE_LIST).toEqual(vectors.modes);
    expect(MODE_LIST).toEqual(VIEWS);
    expect(PRESET_KEYS).toEqual(vectors.presetDenominators.map(String));
    for (const target of [sample, asset("walking-commuter"), asset("solar-thermal-collector")]) {
      const entries = scaleEntries(target);
      expect(universe([target]).length).toBe(entries.length * vectors.viewpoints.length * vectors.modes.length);
      for (const key of vectors.presetDenominators) expect(entries.some((entry) => entry.scale === `${key}` && entry.kind === "preset")).toBe(true);
      for (const entry of entries.filter((candidate) => candidate.kind === "band-min" || candidate.kind === "band-max")) {
        const rep = target.representations[entry.representation as keyof Asset["representations"]] as AuthoredRepresentation;
        expect([rep.validRange.minDenominator, rep.validRange.maxDenominator]).toContain(entry.requested);
      }
    }
  });

  it("reports a cell no runner produced as the failure cell-missing", () => {
    const cells = assemble([sample], []);
    expect(cells.length).toBe(universe([sample]).length);
    expect(cells.every((cell) => cell.status === "fail" && cell.failures["cell-missing"]?.count === 1)).toBe(true);
  });

  it("samples clips at keyframes, mid-transitions, extremes, start, end and at least eight uniform instants, sixteen for long clips", () => {
    const walker = asset("walking-commuter");
    const clip = walker.clips.find((candidate) => candidate.id === vectors.motion.rootMotion.clip)!;
    const found = clipInstants(walker, clip.id, (t) => [Math.sin(t * 3), Math.cos(t * 5)]);
    const tags = new Set(found.map((instant) => instant.tag));
    for (const tag of ["start", "key", "mid", "extreme", "uniform"]) expect(tags.has(tag as never)).toBe(true);
    expect(found.filter((instant) => instant.tag === "uniform").length).toBeGreaterThanOrEqual(vectors.sampling.uniformInstants);
    expect(UNIFORM_INSTANTS).toBe(vectors.sampling.uniformInstants);
    expect(LONG_CLIP_S).toBe(vectors.sampling.longClipSeconds);
    expect(LONG_UNIFORM_INSTANTS).toBe(vectors.sampling.longUniformInstants);
    expect(found.every((instant) => instant.t >= 0 && instant.t <= clip.durationS)).toBe(true);
    const long = catalog.find((candidate) => candidate.clips.some((c) => c.durationS > LONG_CLIP_S));
    if (long !== undefined) {
      const longClip = long.clips.find((c) => c.durationS > LONG_CLIP_S)!;
      expect(clipInstants(long, longClip.id).filter((instant) => instant.tag === "uniform").length).toBeGreaterThanOrEqual(vectors.sampling.longUniformInstants);
    }
  });

  it("pairs every pose with the rest pose and three seeded random pairs, identically in every run", () => {
    for (const target of [sample, asset("walking-commuter")]) {
      const pairs = transitionPairs(target);
      const rest = target.presentation.defaultPose;
      for (const pose of target.poses.filter((p) => p.id !== rest)) expect(pairs.some(([from, to]) => from === pose.id && to === rest)).toBe(true);
      expect(pairs.length).toBe(target.poses.length - 1 + vectors.sampling.transitionRandomPairs);
      expect(transitionPairs(target)).toEqual(pairs);
      const fractions = new Set(subjects(target, 100).filter((subject) => subject.query.kind === "transition").map((subject) => (subject.query as { timeS: number }).timeS));
      expect([...fractions].sort()).toEqual([...vectors.sampling.transitionFractions].sort());
    }
    const a = seeded(hash("x"));
    const b = seeded(hash("x"));
    expect([a(), a(), a()]).toEqual([b(), b(), b()]);
  });

  it("classifies a mutated sample figure: declared unavailable, undecided scale, missing front view", { timeout: 900_000 }, () => {
    const rep50 = sample.representations["50"] as AuthoredRepresentation;
    const mutant: Asset = {
      ...sample,
      clips: sample.clips.filter((clip) => clip.id === "walk"),
      representations: {
        ...sample.representations,
        "500": { status: "unavailable", denominator: 500, reason: { en: "Too small to print.", de: "Zu klein zum Drucken." }, limitingFeature: { en: "Fingers", de: "Finger" }, limitingSizeM: 0.02, alternatives: [{ kind: "scale", denominator: 100 }], cause: "below-printable-feature" } as never,
        "50": { ...rep50, geometry: { ...rep50.geometry, front: [] } },
        "100": undefined as never,
      },
    };
    delete (mutant.representations as Record<string, unknown>)["100"];
    const cells = buildMatrix([mutant]);
    expect(cells.length).toBe(universe([mutant]).length);
    const at = (scale: string, view: PlanarViewpoint, mode: View) => cells.find((cell) => cell.scale === scale && cell.view === view && cell.mode === mode)!;
    for (const view of VIEWPOINT_LIST) for (const mode of MODE_LIST) expect(at("100", view, mode).failures["representation-missing"]).toBeDefined();
    for (const mode of MODE_LIST) {
      expect(at("50", "front", mode).status).toBe("fail");
      expect(Object.keys(at("50", "front", mode).failures).some((check) => check === "unavailable-view-missing")).toBe(true);
      expect(at("50", "elevation", mode).failures["unavailable-view-missing"]).toBeUndefined();
      expect(at("50", "plan", mode).failures["unavailable-view-missing"]).toBeUndefined();
    }
    for (const cell of cells.filter((c) => c.scale === "500")) expect(cell.status === "unavailable" || cell.failures["unavailable-alternatives"] !== undefined || cell.failures["alternative-scale"] !== undefined || cell.failures["unavailable-cause"] !== undefined).toBe(true);
  });
});

describe("🔭️ reference cases", () => {
  it("projects known dimensions to the paper at 1:50 and 1:200 in the three views with the handed camera bases", () => {
    const figure = rolesAsset();
    for (const { denominator, mmPerMetre, expectedMm } of vectors.paper.scales) {
      expect(mmPerMetre).toBe(1000 / denominator);
      const multi = draw(figure, { kind: "pose", pose: "stand" }, denominator, "light-line", uncheckedProfile);
      const state = multi.state!;
      for (const view of VIEWPOINT_LIST) {
        const { scene } = drawingOf(multi, view);
        const box = sceneBox(scene, idsOf(state.shapes[view], "figure"))!;
        expect(Math.abs(box.maxX - box.minX - expectedMm.width), `${view} 1:${denominator} width`).toBeLessThanOrEqual(vectors.paper.toleranceMm);
        expect(Math.abs(box.maxY - box.minY - expectedMm.height), `${view} 1:${denominator} height`).toBeLessThanOrEqual(vectors.paper.toleranceMm);
        const extent = extentOf(state.shapes[view].filter((shape) => shape.shape.id === "figure"))!;
        const { xMm, yMm } = scene.metadata.origin;
        const k = scene.metadata.mmPerMetre;
        expect(Math.abs((box.minX - xMm) / k - extent.minX)).toBeLessThanOrEqual(vectors.paper.toleranceMm / k);
        expect(Math.abs((yMm - box.minY) / k - extent.maxY)).toBeLessThanOrEqual(vectors.paper.toleranceMm / k);
      }
    }
  });

  it("keeps the real dimensions of catalog assets on the paper in every view (mm = m · 1000 / denominator)", () => {
    for (const id of vectors.paper.catalogAssets) {
      const target = asset(id);
      for (const denominator of [200, 50]) {
        const rep = target.representations[`${denominator}` as keyof Asset["representations"]];
        if (rep?.status !== "authored") continue;
        const multi = draw(target, { kind: "pose", pose: target.presentation.defaultPose }, denominator, "light-line", uncheckedProfile);
        const k = 1000 / denominator;
        const { heightM, widthM, depthM } = target.dimensions;
        const expected: Record<PlanarViewpoint, [number, number]> = { elevation: [widthM, heightM], front: [depthM, heightM], plan: [widthM, depthM] };
        for (const view of VIEWPOINT_LIST) {
          const found = multi.views[view];
          if (found.kind !== "drawing") continue;
          const box = sceneBox(found.scene, idsOf(multi.state!.shapes[view]))!;
          const slack = (toleranceM(target) + 0.001) * k + 1;
          expect(Math.abs(box.maxX - box.minX - expected[view][0] * k), `${id} 1:${denominator} ${view} width`).toBeLessThanOrEqual(slack);
          expect(Math.abs(box.maxY - box.minY - expected[view][1] * k), `${id} 1:${denominator} ${view} height`).toBeLessThanOrEqual(slack);
        }
      }
    }
  });

  it("places an asymmetric held object on the side of its hand in every view (left is +z: paper right in the front, up in the plan)", () => {
    for (const held of vectors.handedness.heldObjects) {
      const target = asset(held.asset);
      const multi = draw(target, { kind: "pose", pose: held.pose }, 50, "light-line", uncheckedProfile);
      const state = multi.state!;
      expect(state.anchors[held.anchor]!.z * held.zSign, `${held.asset} ${held.anchor}`).toBeGreaterThan(0.02);
      if (held.shapePattern === undefined) continue;
      const front = drawingOf(multi, "front").scene;
      const plan = drawingOf(multi, "plan").scene;
      const frontBox = sceneBox(front, idsOf(state.shapes.front, held.shapePattern))!;
      const planBox = sceneBox(plan, idsOf(state.shapes.plan, held.shapePattern))!;
      expect(((frontBox.minX + frontBox.maxX) / 2 - front.metadata.origin.xMm) * held.zSign).toBeGreaterThan(0);
      expect((plan.metadata.origin.yMm - (planBox.minY + planBox.maxY) / 2) * held.zSign).toBeGreaterThan(0);
      const lateral = extentOf(state.shapes.front.filter((shape) => shape.shape.id.includes(held.shapePattern!)))!;
      expect(((lateral.minX + lateral.maxX) / 2) * held.zSign).toBeGreaterThan(0);
    }
  });

  it("keeps planted contacts still and advances the root as one state in the three views", () => {
    const { asset: id, clip: clipId, advanceM, toleranceM: tolerance } = vectors.motion.rootMotion;
    const target = asset(id);
    const clip = target.clips.find((candidate) => candidate.id === clipId)!;
    const at = (timeS: number) => evaluate(target, { kind: "clip", clip: clipId, timeS }, { kind: "scale", scale: { denominator: 100 } });
    const start = at(0);
    const end = at(clip.durationS);
    expect(clip.rootMotion).toBe("plant");
    expect(Math.abs(end.root.x - start.root.x - advanceM)).toBeLessThanOrEqual(tolerance);
    for (const view of ["elevation", "plan"] as const) {
      const a = extentOf(start.shapes[view])!;
      const b = extentOf(end.shapes[view])!;
      expect(Math.abs(b.minX - a.minX - (end.root.x - start.root.x))).toBeLessThanOrEqual(0.03);
    }
    const span = clip.contacts.find((contact) => contact.toS - contact.fromS > 0.1)!;
    const planted = Array.from({ length: 5 }, (_, i) => at(span.fromS + ((span.toS - span.fromS) * (i + 0.5)) / 5).anchors[span.anchor]!);
    for (const p of planted) {
      expect(Math.abs(p.at.x - planted[0]!.at.x)).toBeLessThanOrEqual(0.02);
      expect(Math.abs(p.z - planted[0]!.z)).toBeLessThanOrEqual(0.02);
    }
    for (const timeS of [0, 0.3, 0.6, 0.9]) {
      const state = at(timeS);
      const e = extentOf(state.shapes.elevation)!;
      const f = extentOf(state.shapes.front)!;
      const p = extentOf(state.shapes.plan)!;
      const tol = toleranceM(target);
      expect(Math.abs(e.minX - p.minX) + Math.abs(e.maxX - p.maxX)).toBeLessThanOrEqual(2 * tol);
      expect(Math.abs(e.minY - f.minY) + Math.abs(e.maxY - f.maxY)).toBeLessThanOrEqual(2 * tol);
      expect(Math.abs(f.minX - p.minY) + Math.abs(f.maxX - p.maxY)).toBeLessThanOrEqual(2 * tol);
    }
  });

  it("continues the loop with an absolute time and the clip phase: the same pose per cycle, the root advanced by whole cycles", () => {
    const { asset: id, clip: clipId, cycles } = vectors.motion.loop;
    const target = asset(id);
    const clip = target.clips.find((candidate) => candidate.id === clipId)!;
    const phase = 0.3;
    const states = cycles.map((n) => evaluate(target, { kind: "clip", clip: clipId, timeS: n * clip.durationS + phase }, { kind: "scale", scale: { denominator: 100 } }));
    states.forEach((state, i) => {
      expect(state.timeS).toBeCloseTo(cycles[i]! * clip.durationS + phase, 6);
      expect(state.clipTimeS).toBeCloseTo(phase, 6);
      for (const view of VIEWPOINT_LIST) {
        const a = extentOf(states[0]!.shapes[view])!;
        const b = extentOf(state.shapes[view])!;
        const shift = view === "front" ? 0 : state.root.x - states[0]!.root.x;
        expect(Math.abs(b.maxY - a.maxY)).toBeLessThanOrEqual(0.02);
        if (view !== "front") expect(Math.abs(b.minX - shift - a.minX)).toBeLessThanOrEqual(0.03);
        else expect(Math.abs(b.minX - a.minX)).toBeLessThanOrEqual(0.03);
      }
    });
    expect(states[1]!.root.x - states[0]!.root.x).toBeGreaterThan(0.5 * (clip.advanceM ?? 0));
  });

  it("moves thermal emitters with the body and persists the transport as one analysis across the three views", () => {
    const { asset: id, clip: clipId, times, followToleranceM } = vectors.thermal.movingEmitter;
    const target = asset(id);
    const query = (timeS: number): EvaluationQuery => ({ kind: "clip", clip: clipId, timeS });
    const environment = target.thermal.environment;
    const rootAt = (t: number) => evaluate(target, query(t), { kind: "scale", scale: { denominator: 100 } }).root.x;
    const fields = times.map((t) => solveTransport(target, query(t), environment));
    expect(fields[0]!.emitters.length).toBeGreaterThan(0);
    fields.forEach((field, i) => {
      field.emitters.forEach((emitter, j) => expect(Math.abs(emitter.at.x - fields[0]!.emitters[j]!.at.x - (rootAt(times[i]!) - rootAt(0)))).toBeLessThanOrEqual(followToleranceM));
    });
    const scrubbed = solveTransport(target, query(times[2]!), environment);
    expect(JSON.stringify(scrubbed)).toBe(JSON.stringify(fields[2]));
    const plain = draw(target, query(times[2]!), 100, "thermal", uncheckedProfile);
    const withTransport = draw(target, query(times[2]!), 100, "thermal", { ...uncheckedProfile, transport: ["parcels", "streamlines"] });
    expect(withTransport.analysis.transport).toBeDefined();
    const count = (scene: DrawingScene): number => scene.layers.reduce((n, layer) => n + layer.items.length, 0);
    for (const view of VIEWPOINT_LIST) {
      expect(count(drawingOf(withTransport, view).scene), `${view} transport overlay`).toBeGreaterThan(count(drawingOf(plain, view).scene));
      expect(drawingOf(withTransport, view).state).toBe(withTransport.state);
    }
    const a = drawingOf(draw(target, query(times[1]!), 100, "thermal", uncheckedProfile), "plan").scene;
    const b = drawingOf(draw(target, query(times[3]!), 100, "thermal", uncheckedProfile), "plan").scene;
    expect(sceneJson(a)).not.toBe(sceneJson(b));
  });

  it("reads one magnitude per region and member across the three views for thermal, forces and moments", () => {
    for (const id of ["standing-adult", "walking-commuter"]) {
      const target = asset(id);
      const query: EvaluationQuery = { kind: "pose", pose: target.presentation.defaultPose };
      for (const mode of ["thermal", "forces", "moments"] as const) {
        const multi = draw(target, query, 50, mode, uncheckedProfile);
        const peaks = new Map<string, Map<PlanarViewpoint, number>>();
        const legends = new Set<string>();
        for (const view of VIEWPOINT_LIST) {
          const drawn = multi.views[view];
          if (drawn.kind !== "drawing") continue;
          legends.add(JSON.stringify(drawn.scene.legends));
          for (const layer of drawn.scene.layers)
            for (const item of layer.items) {
              const owner = mode === "thermal" ? item.kind === "path" && item.region : item.kind === "path" && item.bone;
              if (item.kind !== "path" || item.value === undefined || !owner) continue;
              const key = `${owner}|${item.value.quantity}`;
              const perView = peaks.get(key) ?? new Map<PlanarViewpoint, number>();
              perView.set(view, Math.max(perView.get(view) ?? Number.NEGATIVE_INFINITY, item.value.value));
              peaks.set(key, perView);
            }
        }
        expect(legends.size, `${id} ${mode} legends`).toBe(1);
        for (const [key, perView] of peaks) {
          const values = [...perView.values()];
          if (values.length > 1) expect(Math.max(...values) - Math.min(...values), `${id} ${mode} ${key} ${JSON.stringify([...perView])}`).toBeLessThanOrEqual(1e-9 * Math.max(1, ...values.map(Math.abs)));
        }
      }
    }
  });

  it("gives every analysis mode a colour-independent cue (hatch pattern or dash) in every viewpoint under the default options", () => {
    for (const mode of ["thermal", "forces", "moments"] as const) {
      const target = asset(mode === "thermal" ? "electric-kettle" : "standing-adult");
      const multi = draw(target, { kind: "pose", pose: target.presentation.defaultPose }, 50, mode, uncheckedProfile);
      for (const view of VIEWPOINT_LIST) {
        const scene = drawingOf(multi, view).scene;
        const cue = scene.layers.some((layer) => layer.items.some((item) => item.kind === "path" && (item.fill?.pattern !== undefined || (item.stroke?.dashMm?.length ?? 0) > 0)));
        expect(cue, `${mode} ${view}`).toBe(true);
      }
    }
  });

  it("states in the thermal legend that a value within 35 % of the centre is drawn unhatched, so the unhatched state is itself the cue", () => {
    let unhatchedScenes = 0;
    for (const id of ["bookshelf", "three-seat-sofa", "glulam-beam", "steel-column", "single-bed", "standing-adult"]) {
      const target = asset(id);
      const multi = draw(target, { kind: "pose", pose: target.presentation.defaultPose }, 100, "thermal", uncheckedProfile);
      for (const view of VIEWPOINT_LIST) {
        const found = multi.views[view];
        if (found.kind !== "drawing") continue;
        const hatched = found.scene.layers.some((layer) => layer.items.some((item) => item.kind === "path" && item.fill?.pattern !== undefined));
        if (hatched) continue;
        unhatchedScenes++;
        const legend = JSON.stringify(found.scene.legends.map((l) => [l.convention, l.interpretation]));
        expect(legend, `${id} ${view} en`).toMatch(/35\s?%/u);
        expect(legend, `${id} ${view} en hatch`).toMatch(/unhatched|without hatch/iu);
        expect(legend, `${id} ${view} de`).toMatch(/schraffiert|ohne Schraffur/iu);
      }
    }
    expect(unhatchedScenes, "at least one near-ambient thermal scene is unhatched").toBeGreaterThan(0);
  });

  it("carries an out-of-plane force and moment through the state to the front and plan force views", () => {
    const base = sample.poses[0]!;
    const withLoad: Asset = {
      ...sample,
      poses: sample.poses.map((pose) => (pose.id === base.id ? { ...pose, loads: [...pose.loads, { id: "side-push", kind: "imposed", title: { en: "Side push", de: "Seitlicher Schub" }, bone: "torso", x: 0, y: 0.5, z: 0.1, fxN: 0, fyN: 0, fzN: 120, momentXNm: 40, source: "illustrative" } as never] } : pose)),
    };
    const query: EvaluationQuery = { kind: "pose", pose: base.id };
    const state = evaluate(withLoad, query, { kind: "scale", scale: { denominator: 50 } });
    const load = state.loads.find((candidate) => candidate.id === "side-push")!;
    expect(load.fzN).toBeCloseTo(120, 9);
    expect(load.mxNm).toBeCloseTo(40, 9);
    const structure = solveStructure(state);
    expect(JSON.stringify(structure)).not.toBe(JSON.stringify(solveStructure(evaluate(sample, query, { kind: "scale", scale: { denominator: 50 } }))));
    const without = draw(sample, query, 50, "forces", uncheckedProfile);
    const loaded = draw(withLoad, query, 50, "forces", uncheckedProfile);
    for (const view of ["front", "plan"] as const) expect(sceneJson(drawingOf(loaded, view).scene), `${view} forces`).not.toBe(sceneJson(drawingOf(without, view).scene));
  });

  it("measures the minimum printable gap against the print profile at 1:200 and reports it honestly", { timeout: 300_000 }, () => {
    const { denominator, barWidthM, gapM, bodyPenMm, expectedInkGapMm, minGapMm } = vectors.printability.gap;
    const shape = (id: string, x: number) => ({ id, bone: "ground", role: "body", z: 0, filled: true, tone: 0.5, kind: "rect", x, y: 0, w: barWidthM, h: 1.8 }) as never;
    const gapped = rolesAssetWith(() => [shape("bar-a", 0), shape("bar-b", barWidthM + gapM)]);
    const k = 1000 / denominator;
    const resolution = resolveRepresentation(gapped, { denominator }, { ...UNCHECKED_PROFILE, id: "office-laser", minStrokeMm: 0.1, minGapMm, minFeatureMm: 0.2, minHatchSpacingMm: 0.4 });
    expect(resolution.kind).toBe("unsuitable");
    const violation = (resolution as unknown as { suitability: { violations: { check: string; measuredMm: number }[] } }).suitability.violations.find((v) => v.check === "gap")!;
    expect(violation.measuredMm).toBeLessThan(minGapMm);
    expect(Math.abs(violation.measuredMm - expectedInkGapMm)).toBeLessThanOrEqual(0.03);
    if (oracleUp) {
      const a = [[0, 0], [barWidthM * k, 0], [barWidthM * k, 1.8 * k], [0, 1.8 * k]];
      const b = a.map(([x, y]) => [x! + (barWidthM + gapM) * k, y!]);
      const out = runOracle({ distances: [{ key: "gap", a, b }] });
      expect(Math.abs(out.distances.gap! - bodyPenMm - violation.measuredMm)).toBeLessThanOrEqual(0.03);
    }
    const wide = resolveRepresentation(gapped, { denominator: 50 }, { ...UNCHECKED_PROFILE, id: "office-laser", minStrokeMm: 0.1, minGapMm, minFeatureMm: 0.2, minHatchSpacingMm: 0.4 });
    expect(wide.kind).toBe("authored");
  });

  it("never draws a coarse view whose pen is below the profile: the resolver and an independent stroke scan agree", { timeout: 600_000 }, () => {
    const profiles = loadProfiles();
    for (const id of vectors.printability.coarseAssets) {
      const target = asset(id);
      for (const denominator of [500, 200]) {
        const rep = target.representations[`${denominator}` as keyof Asset["representations"]];
        if (rep?.status !== "authored") continue;
        const query: EvaluationQuery = { kind: "pose", pose: target.presentation.defaultPose };
        const scene = (() => {
          const found = draw(target, query, denominator, "light-line", uncheckedProfile).views.plan;
          return found.kind === "drawing" ? found.scene : undefined;
        })();
        if (scene === undefined) continue;
        const strokes = scene.layers.filter((layer) => ["body", "detail", "texture", "glazing"].includes(layer.id)).flatMap((layer) => layer.items.flatMap((item) => (item.kind === "path" && item.stroke ? [item.stroke.widthMm] : [])));
        const thinnest = Math.min(...strokes);
        for (const profile of profiles) {
          if (!(thinnest < profile.minStrokeMm - 1e-9)) continue;
          const result = draw(target, query, denominator, "light-line", { profile } as Partial<DrawOptions>).views.plan;
          expect(result.kind, `${id} 1:${denominator} ${profile.id}: pen ${thinnest} mm < ${profile.minStrokeMm} mm`).toBe("unavailable");
          if (result.kind === "unavailable") expect(result.code).toBe("scale-unsuitable");
        }
      }
    }
  });

  it("switches views repeatedly and scrubs in any order without changing a pixel of a time or a view", { timeout: 600_000 }, () => {
    const { asset: id, clip: clipId, samples, seed, switches } = vectors.scrubbing;
    const target = asset(id);
    const clip = target.clips.find((candidate) => candidate.id === clipId)!;
    const random = seeded(seed);
    const times = Array.from({ length: samples }, () => Math.round(random() * clip.durationS * 1000) / 1000);
    const render = (t: number, mode: View = "light-line") => draw(target, { kind: "clip", clip: clipId, timeS: t }, 100, mode, uncheckedProfile);
    const baseline = new Map(times.map((t) => [t, render(t)]));
    for (const t of [...times].reverse()) {
      const again = render(t);
      for (const view of VIEWPOINT_LIST) expect(sceneJson(drawingOf(again, view).scene)).toBe(sceneJson(drawingOf(baseline.get(t)!, view).scene));
    }
    const shuffled = [...times].sort(() => random() - 0.5);
    for (const t of shuffled) expect(sceneJson(drawingOf(render(t, "thermal"), "plan").scene)).toBe(sceneJson(drawingOf(render(t, "thermal"), "plan").scene));
    const t = times[0]!;
    const reference = new Map(VIEWPOINT_LIST.map((view) => [view, sceneJson(drawingOf(baseline.get(t)!, view).scene)]));
    const state = JSON.stringify(baseline.get(t)!.state!.bones);
    for (let round = 0; round < 3; round++)
      for (const view of switches) {
        const switched = render(t);
        expect(sceneJson(drawingOf(switched, view).scene)).toBe(reference.get(view));
        expect(JSON.stringify(switched.state!.bones)).toBe(state);
        for (const other of VIEWPOINT_LIST) expect(drawingOf(switched, other).state).toBe(switched.state);
      }
  });

  it("exports deterministic bytes: SVG, PDF and DXF of two independent runs are identical in every viewpoint", { timeout: 300_000 }, () => {
    const { asset: id, clip: clipId, timeS, scale } = vectors.determinism;
    const target = asset(id);
    const run = () => {
      const multi = draw(target, { kind: "clip", clip: clipId, timeS }, scale, "light-line", uncheckedProfile);
      const scenes = VIEWPOINT_LIST.map((view) => drawingOf(multi, view).scene);
      return { svg: scenes.map((scene) => writeSvg(scene)), pdf: sha(writePdf(scenes)), dxf: VIEWPOINT_LIST.map((viewpoint) => writeDxf(modelScene(multi.state!, { viewpoint }))) };
    };
    const a = run();
    const b = run();
    expect(a.svg.map(sha)).toEqual(b.svg.map(sha));
    expect(a.pdf).toBe(b.pdf);
    expect(a.dxf.map(sha)).toEqual(b.dxf.map(sha));
    expect(new Set(a.svg.map(sha)).size).toBe(3);
    expect(new Set(a.dxf.map(sha)).size).toBe(3);
  });

  it("names and writes the export files of the three viewpoints uniquely and identically in two CLI runs", { timeout: 600_000 }, async () => {
    const { asset: id, clip: clipId, timeS, scale } = vectors.determinism;
    const target = asset(id);
    const runs: Record<string, string>[] = [];
    for (let run = 0; run < 2; run++) {
      const directory = mkdtempSync(join(tmpdir(), "staffage-cli-"));
      try {
        const files: Record<string, string> = {};
        for (const viewpoint of VIEWPOINT_LIST) {
          const written = await runExport(parseArguments(["--asset", id, "--clip", clipId, "--time", String(timeS), "--view", "light-line", "--scale", String(scale), "--format", "svg", "--out", directory, "--lang", "en", "--viewpoint", viewpoint, "--catalog", join(STAFFAGE_ROOT, "🗂️catalog")]));
          for (const path of written) files[basename(path)] = sha(readFileSync(path));
        }
        runs.push(files);
      } finally {
        rmSync(directory, { recursive: true, force: true });
      }
    }
    expect(Object.keys(runs[0]!).length).toBe(3);
    expect(runs[1]).toEqual(runs[0]);
    expect(new Set(Object.values(runs[0]!)).size).toBe(3);
    const names = VIEWPOINT_LIST.flatMap((viewpoint) => (["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm"] as const).map((format) => exportFilename(target, { kind: "clip", clip: clipId, timeS }, { view: "light-line", viewpoint, scale: { denominator: scale } } as never, format)));
    expect(new Set(names).size).toBe(names.length);
  });

  it("matches an independent SVG parser: svgelements bounds equal the scene bounds of every viewpoint", { timeout: 300_000 }, () => {
    const multi = draw(sample, { kind: "pose", pose: "stand" }, 50, "light-line", uncheckedProfile);
    const svgs = VIEWPOINT_LIST.map((view) => ({ key: view, svg: writeSvg(drawingOf(multi, view).scene) }));
    const out = runOracle({ svgs });
    for (const view of VIEWPOINT_LIST) {
      const box = sceneBox(drawingOf(multi, view).scene)!;
      const [minX, minY, maxX, maxY] = out.svgs[view]!;
      for (const [got, want] of [[minX, box.minX], [minY, box.minY], [maxX, box.maxX], [maxY, box.maxY]] as const) expect(Math.abs(got! - want)).toBeLessThanOrEqual(0.01);
    }
  });
});

describe("🪞️ derived-view detection", () => {
  const shapeOf = (ring: number[][]): EvaluatedShape[] => [{ shape: { id: "s", role: "body" } as never, rings: [ring.map(([x, y]) => ({ x: x!, y: y! }))], closed: true }];

  it("flags a copied, mirrored, rotated or scaled elevation as front or plan and keeps handcrafted outlines", () => {
    for (const c of vectors.derived.cases) {
      const flags = derivedFlags({ elevation: shapeOf(c.elevation), front: shapeOf(c.other), plan: [] });
      expect(flags.some((flag) => flag.view === "front" && flag.against === "elevation"), c.name).toBe(c.flagged);
    }
  });

  it("matches scipy: the Hausdorff distance under every candidate transform", () => {
    const pairs = vectors.derived.cases.flatMap((c) => {
      const a = pointSet(shapeOf(c.elevation))!;
      const b = pointSet(shapeOf(c.other))!;
      return TRANSFORMS.map(({ name, m }) => ({ key: `${c.name}|${name}`, a, b: b.map((p) => ({ x: m[0] * p.x + m[1] * p.y, y: m[2] * p.x + m[3] * p.y })) }));
    });
    const out = runOracle({ hausdorff: pairs.map((pair) => ({ key: pair.key, a: pair.a.map((p) => [p.x, p.y]), b: pair.b.map((p) => [p.x, p.y]) })) });
    for (const c of vectors.derived.cases) {
      const best = Math.min(...TRANSFORMS.map(({ name }) => out.hausdorff[`${c.name}|${name}`]!));
      expect(Math.abs(best - bestTransform(pointSet(shapeOf(c.elevation))!, pointSet(shapeOf(c.other))!).distance), c.name).toBeLessThanOrEqual(1e-9);
    }
  });
});

function loadProfiles(): { id: string; minStrokeMm: number; minGapMm: number; minFeatureMm: number; minHatchSpacingMm: number; title: never; description: never; source: never }[] {
  const document = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🗂️catalog/🖨️print-profiles/🔣️.json"), "utf8")) as { profiles: ReturnType<typeof loadProfiles> };
  return document.profiles;
}

type OracleJob = { svgs?: { key: string; svg: string }[]; hausdorff?: { key: string; a: number[][]; b: number[][] }[]; distances?: { key: string; a: number[][]; b: number[][] }[] };
type OracleOut = { svgs: Record<string, number[] | null>; hausdorff: Record<string, number>; distances: Record<string, number> };

function runOracle(job: OracleJob): OracleOut {
  const directory = mkdtempSync(join(tmpdir(), "staffage-coverage-"));
  try {
    const file = join(directory, "job.json");
    writeFileSync(file, JSON.stringify(job));
    return JSON.parse(python(oracle, "geometry", [file]).trim().split("\n").pop()!) as OracleOut;
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}
