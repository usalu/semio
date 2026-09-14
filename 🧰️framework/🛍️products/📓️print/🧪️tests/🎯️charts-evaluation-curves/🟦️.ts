// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { descending, quantileSorted } from "d3-array";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "charts-evaluation-curves";
const DECIMALS = 9;

/** 🧪️ `demo-scores` of `semio-viz-charts-distribution`: a scored, labelled sample. */
const SCORES = [0.95, 0.9, 0.86, 0.81, 0.77, 0.71, 0.66, 0.6, 0.55, 0.48, 0.42, 0.37, 0.31, 0.25, 0.18, 0.09] as const;
const LABELS = [1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0] as const;

const grid = (value: number): number => {
  const factor = 10 ** DECIMALS;
  return value === 0 ? 0 : Math.round(value * factor) / factor;
};

/** 🎯️ Compiles one committed fixture of this case and returns its whole projection. */
async function probe(ctx: AdapterContext, fixture: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(`local://${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🔮️Oracles
/** 🔮️ The threshold sweep: descending order via d3-array, then the four running counters. */
function sweep() {
  const order = SCORES.map((_, i) => i).sort((a, b) => descending(SCORES[a]!, SCORES[b]!));
  const positives = LABELS.filter((l) => l > 0.5).length;
  const negatives = LABELS.length - positives;
  const roc = { x: [0], y: [0] };
  const pr = { x: [] as number[], y: [] as number[] };
  const gain = { x: [0], y: [0] };
  let truePositives = 0;
  let falsePositives = 0;
  let area = 0;
  order.forEach((index, step) => {
    if (LABELS[index]! > 0.5) truePositives++;
    else {
      area += truePositives / (positives * negatives);
      falsePositives++;
    }
    roc.x.push(grid(falsePositives / negatives));
    roc.y.push(grid(truePositives / positives));
    pr.x.push(grid(truePositives / positives));
    pr.y.push(grid(truePositives / (truePositives + falsePositives)));
    gain.x.push(grid((step + 1) / LABELS.length));
    gain.y.push(grid(truePositives / positives));
  });
  return { roc, pr, gain, area: grid(area) };
}

/** 🔮️ The four confusion counts at the median score, in the order the family emits them. */
function confusion(): number[] {
  const sorted = [...SCORES].sort((a, b) => a - b);
  const threshold = quantileSorted(sorted, 0.5)!;
  let tp = 0;
  let fp = 0;
  let fn = 0;
  let tn = 0;
  SCORES.forEach((score, index) => {
    const positive = LABELS[index]! > 0.5;
    if (score >= threshold) positive ? tp++ : fp++;
    else positive ? fn++ : tn++;
  });
  return [tp, fp, fn, tn];
}
// #endregion 🔮️Oracles

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "roc": {
      /** 🔮️ The ROC sweep from the origin plus the rank-sum area. */
      oracle: () => {
        const s = sweep();
        return { projection: { fpr: s.roc.x, tpr: s.roc.y, auc: [s.area] } };
      },
      /** 🎯️ The rates `\semio_viz_eval_roc:NN` produced. */
      subject: async (ctx: AdapterContext) => (await probe(ctx, "roc.tex")),
    },
    "pr": {
      /** 🔮️ The precision-recall sweep, which has no origin point. */
      oracle: () => {
        const s = sweep();
        return { projection: { recall: s.pr.x, precision: s.pr.y } };
      },
      /** 🎯️ The rates `\semio_viz_eval_pr:NN` produced. */
      subject: async (ctx: AdapterContext) => (await probe(ctx, "pr.tex")),
    },
    "gain": {
      /** 🔮️ The cumulative-gains sweep over the whole population. */
      oracle: () => {
        const s = sweep();
        return { projection: { population: s.gain.x, captured: s.gain.y } };
      },
      /** 🎯️ The rates `\semio_viz_eval_gain:NN` produced. */
      subject: async (ctx: AdapterContext) => (await probe(ctx, "gain.tex")),
    },
    "confusion": {
      /** 🔮️ The four outcome counts at d3-array's median of the score column. */
      oracle: () => ({ projection: { "geometry/confusion": confusion() } }),
      /** 🎯️ The `geometry/confusion` record the evaluation family emitted. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/confusion": (await probe(ctx, "confusion.tex")).projection["geometry/confusion"] ?? [] } }),
    },
  },
});
// #endregion 🧭️Adapter
