// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { scaleLinear, scalePow } from "d3-scale";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, compileVizProbeDocument, probeProjection, roundProbeNumbers, type ProbeProjection, type VizProbeStatement } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "probe-protocol";
const DECIMALS = 9;

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 🔢️ One numeric column of the data table. */
function column(ctx: AdapterContext, name: string): number[] {
  return rows(ctx).map((row) => {
    const value = Number(row[name]);
    if (!Number.isFinite(value)) throw new Error(`scenario ${ctx.scenario.id} column ${name} carries the non-numeric value ${JSON.stringify(row[name])}`);
    return value;
  });
}

/** 🔢️ Rounds an oracle's own numbers onto the same emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => (value === 0 ? 0 : Math.round(value * factor) / factor));
}
// #endregion 🧫️Vectors

// #region 🧪️Probes
/** 🧪️ The affine mapping, written as the fixed-point expression the probe document evaluates. */
function affineStatements(ctx: AdapterContext): VizProbeStatement[] {
  const expressions = rows(ctx).map((row) => `(${row.rangeMin})+((${row.rangeMax})-(${row.rangeMin}))*((${row.input})-(${row.domainMin}))/((${row.domainMax})-(${row.domainMin}))`);
  return [{ precision: DECIMALS }, { evaluate: { key: "scale/linear", expressions } }];
}

async function affineSubject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbeDocument({ case: CASE, scenario: ctx.scenario.id, packages: [], body: affineStatements(ctx) }, { workDir: ctx.workDir });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS)) };
}

async function powerSubject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture("local://power-mapping.tex"), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS)) };
}
// #endregion 🧪️Probes

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "affine-mapping": {
      /** 🔮️ d3-scale, the published reference for the affine domain-to-range mapping. */
      oracle: (ctx: AdapterContext) => ({ projection: { "scale/linear": grid(rows(ctx).map((row) => scaleLinear().domain([Number(row.domainMin), Number(row.domainMax)]).range([Number(row.rangeMin), Number(row.rangeMax)])(Number(row.input)))) } }),
      /** 🎯️ The probe document rendered from the same vectors, compiled with the repository tectonic. */
      subject: async (ctx: AdapterContext) => (await affineSubject(ctx)),
    },
    "power-mapping": {
      /** 🔮️ d3-scale's square-root scale over the committed fixture's domain and range. */
      oracle: (ctx: AdapterContext) => ({ projection: { "scale/pow": grid(column(ctx, "input").map(scalePow().exponent(0.5).domain([0, 16]).range([0, 100]))) } }),
      /** 🎯️ The committed probe fixture, compiled with the repository tectonic. */
      subject: async (ctx: AdapterContext) => (await powerSubject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
