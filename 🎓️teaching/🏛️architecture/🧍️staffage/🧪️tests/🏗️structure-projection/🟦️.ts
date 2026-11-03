/** 🏗️ Three-dimensional statics and the projection of forces and moments: the solver works in physical metres and newtons, plan, front and side elevation only project its vectors.
 *
 * THE REFERENCE is numpy (🐍️.py) in a right-handed frame with explicit conversion: a balcony with a lateral load, a crane slewed 90 degrees, a person leaning sideways on two feet, and the three cameras derived from the physical viewer position.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🔨️modules/🏗️structure/📽️projection/🟦️.ts — the projection under test
 */
import { describe, expect, it } from "vitest";
import type { Asset, Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { DrawingItem, DrawingScene, EvaluatedState, StructuralResult, Vec3 } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { buildState, loadCase, near } from "../../🔨️modules/🏗️structure/🧰️harness/🟦️.ts";
import { solveStructure, worldForce } from "../../🔨️modules/🏗️structure/🟦️.ts";
import { magnitude, planeOf, projectForce, projectMoment, towardViewer } from "../../🔨️modules/🏗️structure/📽️projection/🟦️.ts";
import { catalog } from "../🧊️three-views/🧰️.ts";

type Row = Record<string, number>;
type Oracle = Record<string, any>;
const VIEWPOINTS: readonly Viewpoint[] = ["elevation", "front", "plan"];
const values = (name: string): Oracle => loadCase(name).oracle.values as Oracle;
const close = (a: number, b: number, scale = 1): boolean => Math.abs(a - b) <= 1e-9 * Math.max(1, scale, Math.abs(b));

function expectReaction(actual: { fxN: number; fyN: number; fzN: number; mxNm: number; myNm: number; mzNm: number }, expected: Row, scale: number): void {
  for (const key of ["fxN", "fyN", "fzN", "mxNm", "myNm", "mzNm"] as const) expect(close(actual[key], expected[key]!, scale), `${key}: ${actual[key]} vs ${expected[key]}`).toBe(true);
}

function expectBalanced(result: StructuralResult, scale: number): void {
  for (const [key, v] of Object.entries(result.residual)) expect(Math.abs(v) / scale, `residual ${key}`).toBeLessThan(1e-9);
}

describe("balcony with a lateral load against numpy", () => {
  const doc = loadCase("🏗️balcony-lateral");
  const oracle = values("🏗️balcony-lateral");
  const cases: readonly (readonly [string, ReturnType<typeof buildState>])[] = [["both", buildState(doc)], ...doc.scenarios.map((s) => [s.id, buildState(doc, s)] as const)];
  for (const [name, state] of cases) {
    const result = solveStructure(state);
    it(`${name}: reacts all six components and balances all six residual components`, () => {
      expectReaction(result.reactions[0]!, oracle[name].reaction, 3000);
      expectBalanced(result, 3000);
    });
    it(`${name}: cuts the member with the lateral shear, the lateral bending and no torsion`, () => {
      const stations = result.members["beam"]!.stations;
      for (const e of oracle[name].stations as Row[]) {
        const s = stations.filter((x) => Math.abs(x.s - e.s!) < 1e-9).at(-1)!;
        for (const key of ["axialN", "shearN", "shearZN", "torsionNm", "momentNm", "momentLateralNm"] as const) expect(close(s[key], e[key]!, 3000), `${key} at ${e.s}`).toBe(true);
      }
    });
  }
  it("puts the out-of-plane moment about the wall's vertical axis, which the side elevation cannot show as a curl", () => {
    const wall = solveStructure(cases[0]![1]).reactions[0]!;
    expect(near(wall.myNm, 600, 1e-12)).toBe(true);
    expect(near(wall.fzN, -300, 1e-12)).toBe(true);
    const curl = (viewpoint: Viewpoint): number => projectMoment(viewpoint, { x: wall.mxNm, y: wall.myNm, z: wall.mzNm }).curl;
    expect(near(curl("elevation"), 2000, 1e-12)).toBe(true);
    expect(near(curl("plan"), -600, 1e-12)).toBe(true);
    expect(Math.abs(curl("front"))).toBeLessThan(1e-9);
  });
});

describe("a crane slewed 90 degrees against numpy", () => {
  const doc = loadCase("🏗️crane-slewed");
  const oracle = values("🏗️crane-slewed");
  const slewed = buildState(doc);
  const result = solveStructure(slewed);
  it("points the jib along the lateral axis, as the hand geometry says", () => {
    const jib = slewed.bones["jib"]!.frame.u;
    oracle["slewed"].jibDirection.forEach((e: number, i: number) => expect(Math.abs([jib.x, jib.y, jib.z][i]! - e)).toBeLessThan(1e-9));
  });
  it("takes the base moment about the forward axis, not about the lateral axis a foreshortened side view would give", () => {
    expectReaction(result.reactions[0]!, oracle["slewed"].reaction, 4.1e5);
    expectBalanced(result, 4.1e5);
    const flat = solveStructure(buildState(doc, doc.scenarios[0]!)).reactions[0]!;
    expectReaction(flat, oracle["unslewed"].reaction, 4.1e5);
    expect(Math.abs(flat.mxNm)).toBeLessThan(1e-6);
    expect(Math.abs(result.reactions[0]!.mzNm)).toBeLessThan(1e-6);
  });
  it("bends the jib by the full lever arm although its side projection is a point", () => {
    const stations = result.members["jib"]!.stations;
    const root = stations.filter((s) => s.s < 1e-9).at(-1)!;
    expect(near(Math.hypot(root.momentNm, root.momentLateralNm), oracle["slewed"].jibRootBendingNm, 1e-9)).toBe(true);
    oracle["slewed"].jibRootMomentVector.forEach((e: number, i: number) => expect(close([root.momentVector.x, root.momentVector.y, root.momentVector.z][i]!, e, 4.1e5)).toBe(true));
    expect(Math.abs(root.torsionNm)).toBeLessThan(1e-6);
    expect(near(Math.hypot(root.shearN, root.shearZN), Math.hypot(oracle["slewed"].jibRootForce[0], oracle["slewed"].jibRootForce[1], oracle["slewed"].jibRootForce[2]), 1e-9)).toBe(true);
  });
});

describe("a person leaning sideways on two feet against numpy", () => {
  const doc = loadCase("🏗️lean-lateral");
  const oracle = values("🏗️lean-lateral");
  for (const [name, state] of [["lean", buildState(doc)], ["upright", buildState(doc, doc.scenarios[0]!)]] as const) {
    const result = solveStructure(state);
    it(`${name}: shares the weight over the feet by the lever rule in the lateral direction`, () => {
      const left = result.reactions.find((r) => r.anchor === "foot-left")!;
      const right = result.reactions.find((r) => r.anchor === "foot-right")!;
      expect(close(left.fyN, oracle[name].leftNormalN, 540)).toBe(true);
      expect(close(right.fyN, oracle[name].rightNormalN, 540)).toBe(true);
      expectBalanced(result, 540);
    });
    it(`${name}: carries the lean in the shoulder joint as a moment about the forward axis`, () => {
      const joint = result.joints["torso"]!;
      expect(close(joint.mxNm, oracle[name].torsoJointMxNm, 100)).toBe(true);
      expect(Math.abs(joint.myNm)).toBeLessThan(1e-6);
      expect(Math.abs(joint.mzNm)).toBeLessThan(1e-6);
    });
  }
  it("moves the centre of mass sideways and loads the leaning side more", () => {
    const result = solveStructure(buildState(doc));
    expect(near(result.centreOfMass.z, (30 * oracle["lean"].torsoComZ) / 55, 1e-9, 1e-12)).toBe(true);
    expect(result.reactions.find((r) => r.anchor === "foot-left")!.fyN).toBeGreaterThan(result.reactions.find((r) => r.anchor === "foot-right")!.fyN);
  });
});

describe("a tied canopy under a lateral wind load", () => {
  const doc = loadCase("🪢️tied-canopy");
  const wind = { id: "wind", bone: "canopy", at: { x: 1, y: 0, z: 0 }, fxN: 0, fyN: -500, fzN: 200 };
  const result = solveStructure(buildState(doc, { id: "lateral", loads: [wind] }));
  it("lets the wall pin take the lateral force it can and states honestly that a pin plus a tie cannot hold the lateral moment", () => {
    const pin = result.reactions.find((r) => r.anchor === "wall-pin")!;
    const rod = result.reactions.find((r) => r.anchor === "tie-rod")!;
    expect(pin.fzN).toBeLessThan(0);
    expect(Math.abs(rod.fzN)).toBeLessThan(1e-12);
    expect(near(result.residual.fzN, 200 + pin.fzN, 1e-9, 1e-9)).toBe(true);
    expect(Math.abs(result.residual.myNm)).toBeGreaterThan(1);
    expect(result.notes.some((n) => n.en.includes("Unbalanced") && n.de.includes("Nicht im Gleichgewicht"))).toBe(true);
  });
  it("keeps the in-plane tie force of the load case unchanged by the lateral component", () => {
    const base = solveStructure(buildState(doc));
    const rod = (r: StructuralResult): number => Math.hypot(r.reactions.find((x) => x.anchor === "tie-rod")!.fxN, r.reactions.find((x) => x.anchor === "tie-rod")!.fyN);
    expect(near(rod(result), rod(base), 1e-9)).toBe(true);
  });
});

describe("a curtain-wall bay under wind suction against numpy and the hand values of the buildings worker", () => {
  const doc = loadCase("🏗️curtain-wall-bay");
  const oracle = values("🏗️curtain-wall-bay");
  const result = solveStructure(buildState(doc));
  const head = result.reactions.find((r) => r.anchor === "head")!;
  const base = result.reactions.find((r) => r.anchor === "base")!;
  it("restrains the head in x, y and z and the base laterally only, so only z reactions arise", () => {
    expectReaction(head, oracle["head"], 5000);
    expectReaction(base, oracle["base"], 5000);
    expect(Math.abs(base.fxN) + Math.abs(base.fyN)).toBeLessThan(1e-9);
    expectBalanced(result, 5000);
  });
  it("reproduces the hand values: reactions +2467.3 N at the head and +2522.3 N at the base, moment 1135.0 and 3022.5 N·m, shears +2522.3, +1161.5, -2467.3 N", () => {
    const hand = oracle["hand"];
    expect(Math.abs(head.fzN - hand.headFzN)).toBeLessThan(0.05);
    expect(Math.abs(base.fzN - hand.baseFzN)).toBeLessThan(0.05);
    const stations = result.members["mullion"]!.stations;
    const at = (y: number) => stations.filter((x) => Math.abs(x.s - y) < 1e-9).at(-1)!;
    expect(Math.abs(at(0.45).momentVector.x - hand.momentAtSpandrelNm)).toBeLessThan(0.05);
    expect(Math.abs(at(2.075).momentVector.x - hand.peakMomentNm)).toBeLessThan(0.05);
    [at(0.275), at(0.9625), at(2.8875)].forEach((s, i) => expect(Math.abs(s.shearZN - [hand.shears[0], hand.shears[1], hand.shears[2]][i]!)).toBeLessThan(0.05));
  });
  it("signs the suction moment +mx: the cut moment about the forward axis is positive for outward (-z) wind, and the lateral bending about the bone normal is -mx", () => {
    for (const e of oracle["stations"] as Row[]) {
      const s = result.members["mullion"]!.stations.filter((x) => Math.abs(x.s - e.s!) < 1e-9).at(-1)!;
      for (const [key, value] of [["mxNm", s.momentVector.x], ["myNm", s.momentVector.y], ["mzNm", s.momentVector.z], ["shearZN", s.shearZN], ["momentLateralNm", s.momentLateralNm], ["momentNm", s.momentNm]] as const) expect(close(value, e[key]!, 5000), `${key} at ${e.s}`).toBe(true);
      expect(e.mxNm).toBeGreaterThan(0);
    }
  });
  it("shows the suction moment as a curl in the front elevation and a double arrow in the side elevation and plan", () => {
    const m = { x: 3022.45, y: 0, z: 0 };
    expect(Math.abs(projectMoment("front", m).curl - -3022.45)).toBeLessThan(1e-9);
    expect(Math.hypot(projectMoment("elevation", m).vector.x, projectMoment("elevation", m).vector.y)).toBeCloseTo(3022.45, 9);
    expect(Math.hypot(projectMoment("plan", m).vector.x, projectMoment("plan", m).vector.y)).toBeCloseTo(3022.45, 9);
  });
});

describe("the projection of vectors against numpy cameras", () => {
  const doc = loadCase("🏗️projection-vectors") as unknown as { vectors: number[][]; oracle: { values: Oracle } };
  const oracle = doc.oracle.values;
  for (const viewpoint of VIEWPOINTS)
    it(`${viewpoint}: places forces and moments like the camera derived from the viewer position`, () => {
      doc.vectors.forEach((v, i) => {
        const vec: Vec3 = { x: v[0]!, y: v[1]!, z: v[2]! };
        const plane = planeOf(viewpoint, vec);
        const force = oracle[viewpoint].forces[i];
        expect(close(plane.x, force.x, 1e3) && close(plane.y, force.y, 1e3)).toBe(true);
        expect(close(towardViewer(viewpoint, vec), force.towardViewer, 1e3)).toBe(true);
        const moment = projectMoment(viewpoint, vec);
        const expected = oracle[viewpoint].moments[i];
        expect(close(moment.vector.x, expected.x, 1e3) && close(moment.vector.y, expected.y, 1e3)).toBe(true);
        expect(close(moment.curl, expected.curl, 1e3)).toBe(true);
      });
    });
  it("keeps the magnitude of every vector identical in the three views", () => {
    for (const v of doc.vectors) {
      const vec: Vec3 = { x: v[0]!, y: v[1]!, z: v[2]! };
      for (const viewpoint of VIEWPOINTS) {
        const p = projectForce(viewpoint, vec);
        expect(close(Math.hypot(p.arrow.x, p.arrow.y, p.outOfPlane), magnitude(vec), 1e3)).toBe(true);
        expect(p.total).toBe(magnitude(vec));
        const m = projectMoment(viewpoint, vec);
        expect(close(Math.hypot(m.vector.x, m.vector.y, m.curl), m.total, 1e3)).toBe(true);
      }
    }
  });
});

describe("supports split per physical contact", () => {
  const assets = catalog();
  const select = { kind: "scale", scale: { denominator: 50 } } as const;
  const solved = (id: string, pose: string) => {
    const asset = assets.find((a) => a.id === id)!;
    const state = evaluate(asset, { kind: "pose", pose, timeS: 0 }, select);
    return { state, result: solveStructure(state) };
  };
  const SPLIT: readonly (readonly [string, string])[] = [["tram", ""], ["forklift", ""], ["excavator", ""], ["diesel-generator", ""], ["oil-pumpjack", ""], ["concrete-mixer-truck", ""], ["dining-table", ""], ["bar-stool", ""], ["cafe-chair", ""], ["operating-table", ""], ["city-car", ""], ["wheel-loader", ""]];
  for (const [id, wanted] of SPLIT)
    it(`${id}: the reactions of every physical contact sum to the weight and the load, in all six components`, () => {
      const asset = assets.find((a) => a.id === id);
      if (asset === undefined) return;
      const pose = wanted === "" ? asset.presentation.defaultPose : wanted;
      const { result } = solved(id, pose);
      const active = result.reactions.filter((r) => r.active);
      expect(active.length).toBeGreaterThanOrEqual(3);
      const weight = result.segmentWeights.reduce((a, f) => a - f.fyN, 0);
      const applied = result.applied.reduce((a, f) => a + f.fyN, 0);
      expect(near(active.reduce((a, r) => a + r.fyN, 0), weight - applied, 1e-6, 1e-6 * (1 + weight))).toBe(true);
      const scale = 1 + weight + result.applied.reduce((a, f) => a + Math.hypot(f.fxN, f.fyN, f.fzN), 0);
      expect(Math.hypot(result.residual.fxN, result.residual.fyN, result.residual.fzN) / scale).toBeLessThan(1e-6);
      expect(Math.hypot(result.residual.mxNm, result.residual.myNm, result.residual.mzNm) / (scale * 10)).toBeLessThan(1e-6);
      expect(new Set(active.map((r) => r.z.toFixed(3))).size).toBeGreaterThanOrEqual(2);
    });
  it("reports convergence of a solution that has converged: kneeling-weeding, leaning on a wall and leaning back are stable and balanced", () => {
    for (const [id, pose] of [["kneeling-gardener", "kneeling-weeding"], ["leaning-person", "leaning-shoulder-wall"], ["seated-office-worker", "leaning-back"]] as const) {
      const { result } = solved(id, pose);
      expect(result.notes.some((n) => n.en.includes("did not converge") || n.de.includes("nicht konvergiert")), `${id} ${pose}`).toBe(false);
      expect(Math.hypot(result.residual.fxN, result.residual.fyN, result.residual.fzN)).toBeLessThan(1e-6);
    }
  });
  it("names the tipping axis when every active support lies on one line in plan, and not when the supports span the plan", () => {
    const doc = loadCase("🏗️lean-lateral");
    const lean = { id: "lean-40", bones: { torso: { abductionDeg: 40 } } };
    expect(solveStructure(buildState(doc, lean)).notes.some((n) => n.en.startsWith("Collinear supports"))).toBe(false);
    const onOneLine = { ...doc, anchors: doc.anchors.map((a) => ({ ...a, z: 0, y: a.id === "foot-left" ? -0.2 : 0.2 })) };
    const result = solveStructure(buildState(onOneLine, lean));
    expect(result.notes.some((n) => n.en.startsWith("Collinear supports") && n.de.startsWith("Kollineare Auflager"))).toBe(true);
    expect(Math.abs(result.residual.mxNm)).toBeGreaterThan(1);
  });
});

describe("member scalars are the same in every view", () => {
  const assets = catalog();
  const select = { kind: "scale", scale: { denominator: 50 } } as const;
  const samples: readonly (readonly [string, string])[] = [["standing-adult", "stand"], ["tower-crane", "parked-weathervane"], ["tower-crane", "lifting-load"], ["cantilever-balcony", "point-load-tip"], ["tied-canopy", "snow-load"], ["leaning-person", "leaning-shoulder-wall"], ["jogger", "stand"]];
  const quantities = ["axial", "shear", "stress"] as const;
  const perBone = (scene: DrawingScene): Map<string, Set<number>> => {
    const out = new Map<string, Set<number>>();
    for (const layer of scene.layers) for (const item of layer.items) if (item.kind === "path" && item.bone !== undefined && item.value !== undefined && item.id.startsWith("shape:")) out.set(`${item.bone}|${item.value.quantity}`, (out.get(`${item.bone}|${item.value.quantity}`) ?? new Set()).add(item.value.value));
    return out;
  };
  for (const [id, pose] of samples)
    it(`${id} (${pose}): N, V, σ and M of every bone are identical in side elevation, front elevation and plan`, () => {
      const asset = assets.find((a) => a.id === id);
      if (asset === undefined || !asset.poses.some((p) => p.id === pose)) return;
      const state = evaluate(asset, { kind: "pose", pose, timeS: 0 }, select);
      const result = solveStructure(state);
      const runs: [string, DrawingScene][] = [];
      for (const view of ["forces", "moments"] as const)
        for (const quantity of view === "forces" ? quantities : ["axial" as const])
          for (const viewpoint of VIEWPOINTS) runs.push([`${view}:${quantity}:${viewpoint}`, depict(state, { ...defaultDepictionOptions(view, "en", 50, viewpoint, "light"), forceQuantity: quantity }, { structure: result })]);
      const reference = new Map<string, number>();
      for (const [name, scene] of runs) {
        const [view, quantity] = name.split(":") as [string, string];
        for (const [key, set] of perBone(scene)) {
          expect(set.size, `${name} ${key} is one value`).toBe(1);
          const identity = `${view}:${quantity}:${key}`;
          const v = [...set][0]!;
          if (reference.has(identity)) expect(near(v, reference.get(identity)!, 1e-12, 1e-12), `${identity}: ${v} vs ${reference.get(identity)}`).toBe(true);
          else reference.set(identity, v);
        }
      }
      expect(reference.size).toBeGreaterThan(0);
      for (const [key, value] of reference) {
        const [view, , bone, quantity] = key.split(/[:|]/u);
        if (view === "forces" && quantity === "axial") {
          const member = result.members[bone!];
          if (member !== undefined) expect(near(value, member.stations.reduce((best, s) => (Math.abs(s.axialN) > Math.abs(best) ? s.axialN : best), 0), 1e-9, 1e-9)).toBe(true);
        }
      }
    });
});

describe("one load case, three views", () => {
  const assets = catalog();
  const assetOf = (id: string): Asset => assets.find((a) => a.id === id)!;
  const select = { kind: "scale", scale: { denominator: 50 } } as const;
  const itemsOf = (scene: DrawingScene): DrawingItem[] => scene.layers.flatMap((l) => l.items);
  const lateral = (state: EvaluatedState): EvaluatedState => {
    const at = state.anchors["load-point"]!;
    return { ...state, loads: [...state.loads, worldForce("wind-side", "slab", "wind", { en: "Lateral wind", de: "Seitenwind" }, [at.at.x, at.at.y, at.z], [0, 0, 400])] };
  };
  const scenes = (state: EvaluatedState, view: "forces" | "moments") => {
    const result = solveStructure(state);
    return { result, by: Object.fromEntries(VIEWPOINTS.map((viewpoint) => [viewpoint, depict(state, { ...defaultDepictionOptions(view, "en", 50, viewpoint, "light"), diagrams: { axial: true, shear: true, moment: true, freeBody: true } }, { structure: result })])) as Record<Viewpoint, DrawingScene> };
  };
  const balcony = lateral(evaluate(assetOf("cantilever-balcony"), { kind: "pose", pose: "point-load-tip", timeS: 0 }, select));

  it("draws the lateral load as a dot or cross in the side elevation and as an arrow in plan and front, with the same value", () => {
    const { by } = scenes(balcony, "forces");
    const arrow = (viewpoint: Viewpoint, suffix = ""): DrawingItem | undefined => itemsOf(by[viewpoint]).find((i) => i.id === `force:applied:wind-side${suffix}`);
    const symbol = (viewpoint: Viewpoint): DrawingItem | undefined => arrow(viewpoint, ":out");
    expect(symbol("elevation")).toBeDefined();
    expect(arrow("elevation")).toBeUndefined();
    for (const viewpoint of ["plan", "front"] as const) {
      expect(arrow(viewpoint)).toBeDefined();
      expect(symbol(viewpoint)).toBeUndefined();
    }
    const valueOf = (item: DrawingItem | undefined): number => (item?.kind === "path" ? item.value!.value : Number.NaN);
    expect(near(valueOf(symbol("elevation")), 400, 1e-12)).toBe(true);
    expect(near(valueOf(arrow("plan")), 400, 1e-12)).toBe(true);
    expect(near(valueOf(arrow("front")), 400, 1e-12)).toBe(true);
  });

  it("gives every force arrow the same total magnitude in all three views", () => {
    const { result, by } = scenes(balcony, "forces");
    const totals = (viewpoint: Viewpoint): Map<string, number> => new Map(itemsOf(by[viewpoint]).flatMap((i) => (i.kind === "path" && i.value?.quantity === "force" && !i.id.endsWith(":out") ? [[i.id, i.value.value] as const] : [])));
    const reference = totals("elevation");
    for (const viewpoint of ["plan", "front"] as const) for (const [id, v] of totals(viewpoint)) if (reference.has(id)) expect(near(v, reference.get(id)!, 1e-12)).toBe(true);
    const wall = result.reactions.find((r) => r.anchor === "wall-fix")!;
    for (const viewpoint of VIEWPOINTS) {
      const drawn = itemsOf(by[viewpoint]).filter((i) => i.id === "force:reaction:wall-fix" || i.id === "force:reaction:wall-fix:out");
      expect(drawn.length).toBeGreaterThan(0);
      expect(near(Math.hypot(wall.fxN, wall.fyN, wall.fzN), Math.max(...drawn.map((i) => (i.kind === "path" ? i.value!.value : 0))), 1e-9) || drawn.length === 2).toBe(true);
    }
  });

  it("draws the reaction moment as a curl about the line of sight and a double arrow in the plane, in every view", () => {
    const { result, by } = scenes(balcony, "moments");
    const wall = result.reactions.find((r) => r.anchor === "wall-fix")!;
    const item = (viewpoint: Viewpoint, id: string): DrawingItem | undefined => itemsOf(by[viewpoint]).find((i) => i.id === id);
    const id = "moment:reaction:wall-fix";
    const m: Vec3 = { x: wall.mxNm, y: wall.myNm, z: wall.mzNm };
    for (const viewpoint of VIEWPOINTS) {
      const p = projectMoment(viewpoint, m);
      const total = Math.hypot(p.vector.x, p.vector.y, p.curl);
      expect(near(total, magnitude(m), 1e-9)).toBe(true);
      expect(item(viewpoint, id) !== undefined).toBe(Math.abs(p.curl) > 1e-3 * p.total);
      expect(item(viewpoint, `${id}:vector`) !== undefined).toBe(Math.hypot(p.vector.x, p.vector.y) > 1e-3 * p.total);
    }
    expect(Math.abs(wall.mxNm)).toBeLessThan(1e-6);
    expect(wall.myNm).not.toBe(0);
  });

  it("states one legend in all views, with the line-of-sight symbols and a hatch-free dash encoding", () => {
    const { by } = scenes(balcony, "forces");
    const legends = (viewpoint: Viewpoint): string => JSON.stringify(by[viewpoint].legends).replace(/ \d+ (further values in the inspector|weitere Werte im Inspektor)\./gu, "");
    expect(legends("plan")).toBe(legends("elevation"));
    expect(legends("front")).toBe(legends("elevation"));
    const arrows = by["elevation"].legends.find((l) => l.quantity === "force")!;
    expect(arrows.convention.en).toContain("dot");
    expect(arrows.convention.de).toContain("Punkt");
    const dashed = itemsOf(by["plan"]).filter((i) => i.kind === "path" && i.id.startsWith("force:") && i.stroke?.dashMm !== undefined);
    expect(dashed.length).toBeGreaterThan(0);
  });

  it("does not change the load case with the view", () => {
    const first = solveStructure(balcony);
    const before = JSON.stringify(first);
    scenes(balcony, "forces");
    scenes(balcony, "moments");
    expect(JSON.stringify(solveStructure(balcony))).toBe(before);
    expect(first.applied.map((f) => f.id)).toContain("wind-side");
  });

  it("shows the slewed crane's jib in plan with its full bending although the side view foreshortens it", () => {
    const crane = evaluate(assetOf("tower-crane"), { kind: "pose", pose: "parked-weathervane", timeS: 0 }, select);
    const { result, by } = scenes(crane, "moments");
    const peak = (viewpoint: Viewpoint): number => Math.max(0, ...itemsOf(by[viewpoint]).flatMap((i) => (i.kind === "path" && i.id.startsWith("diagram:moment:") && i.value ? [Math.abs(i.value.value)] : [])));
    for (const viewpoint of VIEWPOINTS) expect(Number.isFinite(peak(viewpoint))).toBe(true);
    const jib = result.members["jib"]!.stations.map((s) => Math.hypot(s.momentNm, s.momentLateralNm));
    expect(Math.max(...jib)).toBeGreaterThan(0);
  });

  it("draws a leaning person's contact reactions in all three views", () => {
    const person = evaluate(assetOf("leaning-person"), { kind: "pose", pose: "leaning-shoulder-wall", timeS: 0 }, select);
    const { result, by } = scenes(person, "forces");
    const active = result.reactions.filter((r) => r.active);
    expect(active.length).toBeGreaterThan(0);
    for (const viewpoint of VIEWPOINTS) for (const r of active) expect(itemsOf(by[viewpoint]).some((i) => i.id === `force:reaction:${r.anchor}` || i.id === `force:reaction:${r.anchor}:out`)).toBe(true);
  });
});
