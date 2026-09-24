// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "mark-geometry";
const FIXTURE = "shared://✒️mark-geometry/mark-geometry.tex";
const ROTATION_FIXTURE = "shared://✒️mark-geometry/mark-rotation.tex";
const DECIMALS = 6;

/** 🌀 The four rotated draws of the rotation fixture, in emission order: kind and angle. */
const ROTATIONS: ReadonlyArray<readonly [string, number]> = [
  ["triangle", 0],
  ["triangle", 30],
  ["rectangle", 0],
  ["rectangle", 45],
];

/** 🗂️ The fifty-six primitives taxonomy section 0 lists, in taxonomy order. */
const KINDS = [
  "dot", "circle", "square", "rectangle", "triangle",
  "diamond", "cross", "plus", "star", "custom-glyph",
  "icon-mark", "image-mark", "text-mark", "straight-line", "polyline",
  "step-line", "curved-line", "bezier-curve", "spline", "catmull-rom-spline",
  "basis-spline", "cardinal-spline", "monotone-spline", "closed-curve", "polygon",
  "filled-path", "ribbon", "band", "envelope", "circular-arc",
  "elliptical-arc", "annular-arc", "sector", "wedge", "straight-connector",
  "orthogonal-connector", "curved-connector", "elbow-connector", "bundled-connector", "arrow",
  "bidirectional-arrow", "rectangular-region", "circular-region", "polygonal-region", "voronoi-region",
  "convex-hull", "concave-hull", "label", "callout", "leader-line",
  "bracket", "brace", "highlight-region", "reference-line", "reference-band",
  "reference-point",
] as const;

/** 🎯️ Compiles the fixture and reduces it to the two answers the scenario asks for. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = roundProbeNumbers(
    await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id }),
    DECIMALS,
  );
  const drawn = new Set<string>();
  const outlines = new Map<string, string>();
  for (const record of records) {
    const match = /^geometry\/mark\/([a-z0-9-]+)(\/.*)?$/.exec(record.key);
    if (match === null) continue;
    const kind = match[1]!;
    drawn.add(kind);
    outlines.set(kind, `${outlines.get(kind) ?? ""}|${record.key}=${record.values.join(",")}`);
  }
  return {
    projection: {
      "mark/kinds": [...drawn].sort(),
      "mark/outlines": [outlines.size],
      "mark/distinct": [new Set(outlines.values()).size],
    },
  };
}
/** 🌀 Compiles the rotation fixture and reduces the `…/at` records to angle and placement. */
async function rotationSubject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = roundProbeNumbers(
    await compileVizProbe(ctx.fixture(ROTATION_FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id }),
    DECIMALS,
  );
  const placed = records.filter((record) => /^geometry\/mark\/[a-z0-9-]+\/at$/.test(record.key));
  return {
    projection: {
      "mark/rotation/angles": placed.map((record) => Number(record.values[2])),
      "mark/rotation/placements": [new Set(placed.map((record) => `${record.values[0]},${record.values[1]},${record.values[3]}`)).size],
      "mark/rotation/distinct": [new Set(placed.map((record) => `${record.key}|${record.values.join(",")}`)).size],
    },
  };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    primitives: {
      /** 🗂️ The taxonomy itself: every section 0 slug, each with geometry of its own. */
      oracle: () => ({
        projection: {
          "mark/kinds": [...KINDS].sort(),
          "mark/outlines": [KINDS.length],
          "mark/distinct": [KINDS.length],
        },
      }),
      /** 🎯️ Every primitive drawn with no ink, reduced to presence and distinctness. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    rotation: {
      /** 🌀 The rotation the option asked for reaches the record, and moves nothing else. */
      oracle: () => ({
        projection: {
          "mark/rotation/angles": ROTATIONS.map(([, angle]) => angle),
          "mark/rotation/placements": [1],
          "mark/rotation/distinct": [ROTATIONS.length],
        },
      }),
      /** 🎯️ The same two marks drawn twice each, unrotated and rotated. */
      subject: async (ctx: AdapterContext) => (await rotationSubject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
