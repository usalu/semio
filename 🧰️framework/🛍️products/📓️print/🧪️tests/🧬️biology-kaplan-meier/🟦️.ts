// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "biology-kaplan-meier";
const FIXTURE = "shared://🧬️biology-kaplan-meier/biology-kaplan-meier.tex";
const DECIMALS = 5;
const WIDTH = 60;
const HEIGHT = 40;
const TMAX = 24;

/** 🔢️ Rounds an oracle's numbers onto the emission grid the comparison uses. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

type Record_ = readonly [time: number, status: number];

/** ⚕️ The product-limit estimator: the risk set is what is left of the sorted record list. */
function productLimit(records: readonly Record_[]): { steps: Array<[number, number]>; final: number } {
  const total = records.length;
  let survival = 1;
  const steps: Array<[number, number]> = [[0, 1]];
  records.forEach(([time, status], index) => {
    if (status !== 1) return;
    steps.push([time, survival]);
    survival *= 1 - 1 / (total - index);
    steps.push([time, survival]);
  });
  return { steps, final: survival };
}

/** 📐️ The fixed window the fixture pins: 0..24 across 60 mm, 0..1 up 40 mm. */
const mmSurvival = (steps: ReadonlyArray<readonly [number, number]>): number[] =>
  steps.flatMap(([time, survival]) => [(WIDTH * time) / TMAX, HEIGHT * survival]);

/** 📐️ The cumulative-hazard window: the same times, −ln S drawn on a 0..2 axis. */
const mmHazard = (steps: ReadonlyArray<readonly [number, number]>): number[] =>
  steps.flatMap(([time, survival]) => [(WIDTH * time) / TMAX, (HEIGHT * -Math.log(Math.max(1e-6, survival))) / 2]);

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}

const TREATMENT: Record_[] = [[2, 1], [4, 1], [5, 0], [7, 1], [11, 1], [12, 0], [15, 1], [18, 0], [21, 1], [24, 0]];
const GROUP_A: Record_[] = [[1, 1], [2, 1], [3, 1], [4, 1]];
const GROUP_B: Record_[] = [[1, 0], [2, 1], [3, 0], [4, 1]];
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "product-limit": {
      /** 🔮️ The estimator computed independently, then placed on the fixture's own window. */
      oracle: () => {
        const { steps, final } = productLimit(TREATMENT);
        return { projection: { "geometry/path": grid(mmSurvival(steps)), "geometry/survival/final": grid([final]) } };
      },
      subject,
    },
    "two-groups": {
      /** 🔮️ Two independent estimates, emitted in the order the groups are declared. */
      oracle: () => {
        const a = productLimit(GROUP_A);
        const b = productLimit(GROUP_B);
        return {
          projection: {
            "geometry/path": [...grid(mmSurvival(a.steps)), ...grid(mmSurvival(b.steps))],
            "geometry/survival/final": grid([a.final, b.final]),
          },
        };
      },
      subject,
    },
    "cumulative-hazard": {
      /** 🔮️ The same walk, drawn on the hazard window the mode switches to. */
      oracle: () => {
        const { steps, final } = productLimit(TREATMENT);
        return { projection: { "geometry/path": grid(mmHazard(steps)), "geometry/survival/final": grid([final]) } };
      },
      subject,
    },
  },
});
// #endregion 🧭️Adapter
