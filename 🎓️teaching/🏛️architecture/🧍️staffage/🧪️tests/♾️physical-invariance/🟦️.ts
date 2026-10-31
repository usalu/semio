/** ♾️ Invariance of the physical scenario under the representation selector and the zoom; oracle: vitest deep equality (`toEqual`) of the JSON of the physical parts.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/👤️sample-figure/🔣️.json — the figure
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { AssetDocument } from "../../🧬️schema/🟦️.ts";
import type { EvaluationQuery, RepresentationSelector } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { evaluate, rigBounds } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../🔨️modules/🏗️structure/🟦️.ts";
import { solveThermal } from "../../🔨️modules/🌡️thermal/🟦️.ts";
import { solveTransport } from "../../🔨️modules/💨️transport/🟦️.ts";

const asset = (JSON.parse(readFileSync(new URL("../../🧫️fixtures/👤️sample-figure/🔣️.json", import.meta.url), "utf8")) as AssetDocument).asset;
const selectors: RepresentationSelector[] = [{ kind: "scale", scale: { denominator: 200 } }, { kind: "scale", scale: { denominator: 100 } }, { kind: "scale", scale: { denominator: 120 } }, { kind: "scale", scale: { denominator: 50 } }, { kind: "symbol", symbol: "position-dot" }];
const queries: EvaluationQuery[] = [{ kind: "pose", pose: "stand" }, { kind: "pose", pose: "carry-bag", timeS: 1.3 }, { kind: "clip", clip: "walk", timeS: 0.71 }, { kind: "clip", clip: "idle-breathing", timeS: 2.2 }];
const physical = (state: ReturnType<typeof evaluate>) => JSON.parse(JSON.stringify({ timeS: state.timeS, frame: state.frame, phase: state.phase, bones: state.bones, anchors: state.anchors, contacts: state.contacts, loads: state.loads, thermal: state.thermal, respiration: state.respiration, cycle: state.cycle, kinematics: state.kinematics, rig: rigBounds(state) }));
const geometry = (state: ReturnType<typeof evaluate>) => JSON.stringify(state.shapes.elevation.map((e) => [e.shape.id, e.rings]));

describe("♾️ physical invariance", () => {
  it("keeps bones, anchors, loads, thermal inputs, respiration and kinematics identical for every selector", () => {
    for (const query of queries) {
      const reference = physical(evaluate(asset, query));
      for (const selector of selectors) expect(physical(evaluate(asset, query, selector))).toEqual(reference);
    }
  });

  it("changes the drawn geometry between scales", () => {
    const query: EvaluationQuery = { kind: "pose", pose: "stand" };
    const drawn = [200, 100, 50].map((denominator) => geometry(evaluate(asset, query, { kind: "scale", scale: { denominator } })));
    expect(new Set(drawn).size).toBe(3);
    expect(geometry(evaluate(asset, query, { kind: "scale", scale: { denominator: 120 } }))).toBe(drawn[1]);
  });

  it("yields the same thermal field and structure from every selector", () => {
    const query: EvaluationQuery = { kind: "pose", pose: "carry-bag" };
    const thermal = JSON.stringify(solveThermal(evaluate(asset, query)));
    const structure = JSON.stringify(solveStructure(evaluate(asset, query)));
    for (const selector of selectors) {
      const state = evaluate(asset, query, selector);
      expect(JSON.stringify(solveThermal(state))).toBe(thermal);
      expect(JSON.stringify(solveStructure(state))).toBe(structure);
    }
  });

  it("yields the same transport at every time regardless of the selector", () => {
    const query: EvaluationQuery = { kind: "pose", pose: "stand", timeS: 3 };
    const field = JSON.stringify(solveTransport(asset, query, asset.thermal.environment));
    expect(JSON.stringify(solveTransport(asset, query, asset.thermal.environment))).toBe(field);
    expect(JSON.parse(field).emitters.length).toBeGreaterThan(0);
  });

  it("reproduces the nominal dimensions in the default pose at every authored scale", () => {
    for (const denominator of [200, 100, 50]) {
      const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator } });
      expect(state.bounds.maxY - state.bounds.minY).toBeCloseTo(asset.dimensions.heightM, 1);
    }
  });
});
