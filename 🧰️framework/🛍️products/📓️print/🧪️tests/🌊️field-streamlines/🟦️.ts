// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "field-streamlines";
const DECIMALS = 9;
const SAMPLES = 16;

type Field = (x: number, y: number) => [number, number];

/** 🧲️ The five analytic plane fields `spatial-vector-field` offers, written as their own definition. */
const FIELDS: Record<string, Field> = {
  shear: (x, y) => [Math.sin(y), Math.sin(x)],
  source: (x, y) => [x, y],
  vortex: (x, y) => [-y, x],
  saddle: (x, y) => [x, -y],
  dipole: (x, y) => {
    const denominator = Math.max(0.05, (x * x + y * y) ** 2);
    return [(x * x - y * y) / denominator, (2 * x * y) / denominator];
  },
};

function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

function grid(value: number): number {
  return value === 0 ? 0 : Math.round(value * 10 ** DECIMALS) / 10 ** DECIMALS;
}

/** 🚧 The window clamp of the integrator: a divergent field would otherwise leave TeX's dimension range. */
function clamp(value: number, bound: number): number {
  return Math.min(bound, Math.max(-bound, value));
}

/** 🌀 The classical fourth-order Runge-Kutta trajectory, clamped to the window, sampled before every step. */
function trajectory(field: Field, x0: number, y0: number, h: number): number[] {
  const out: number[] = [];
  let x = x0;
  let y = y0;
  for (let step = 0; step < SAMPLES; step += 1) {
    out.push(grid(x), grid(y));
    const [k1x, k1y] = field(x, y);
    const [k2x, k2y] = field(x + (h * k1x) / 2, y + (h * k1y) / 2);
    const [k3x, k3y] = field(x + (h * k2x) / 2, y + (h * k2y) / 2);
    const [k4x, k4y] = field(x + h * k3x, y + h * k3y);
    x = clamp((h / 6) * (k1x + 2 * k2x + 2 * k3x + k4x) + x, 3.4);
    y = clamp((h / 6) * (k1y + 2 * k2y + 2 * k3y + k4y) + y, 2.4);
  }
  return out;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "rk-integration": {
      /** 🔮️ The substitute of the `rk-field-integration` decision: an independent RK4 over the same fields. */
      oracle: (ctx: AdapterContext) => ({
        projection: {
          "field/streamline": rows(ctx).flatMap((row) => {
            const field = FIELDS[row.field!];
            if (field === undefined) throw new Error(`no analytic field is registered under ${JSON.stringify(row.field)}`);
            return trajectory(field, Number(row.x), Number(row.y), Number(row.dt));
          }),
        },
      }),
      /** 🎯️ `\semio_viz_georte_rk_step:` over the same seeds and step sizes. */
      subject: async (ctx: AdapterContext): Promise<{ projection: ProbeProjection }> => {
        const records = await compileVizProbe(ctx.fixture("local://streamlines.tex"), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
        return { projection: probeProjection(roundProbeNumbers(records, DECIMALS)) };
      },
    },
  },
});
// #endregion 🧭️Adapter
