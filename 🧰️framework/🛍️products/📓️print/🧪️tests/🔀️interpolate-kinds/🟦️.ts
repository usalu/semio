/** 🔀️ Adapter for `interpolate-kinds`: `\SemioVizInterpolate`, the public interpolator surface.
 *
 * Subject: `semio-viz-scale`'s interpolators, probed through `semio-viz-probe`.
 * Oracle: `d3-interpolate`, one function per kind the command dispatches on.
 */
import { rgb } from "d3-color";
import { interpolateArray, interpolateHcl, interpolateLab, interpolateNumber, interpolateRgb, interpolateRound } from "d3-interpolate";
import { type AdapterContext, defineTestAdapter } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { type ProbeProjection, compileVizProbe, probeProjection, roundProbeNumbers } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";

//#region 🔖️Vectors
const CASE = "interpolate-kinds";
const FIXTURE = "local://interpolate-kinds.tex";
const DECIMALS = 6;

/** 🥒️ The vector table of the running scenario, as one record per row. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, (row[index] ?? "").trim()])));
}

/** ✂️ A `;`-separated cell as a list. */
function list(cell: string): number[] {
  return cell.split(";").map((item) => Number(item.trim()));
}

/** 🔢️ Rounds an oracle's numbers onto the probe's emission grid. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🌈️ The colour interpolators the command's colour kinds must land on. */
const SPACES: Readonly<Record<string, (a: string, b: string) => (t: number) => string>> = {
  rgb: interpolateRgb,
  lab: interpolateLab,
  hcl: interpolateHcl,
};

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
//#endregion 🔖️Vectors

//#region 🔖️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    number: {
      /** 🔮️ d3-interpolate's `interpolateNumber` on the declared pairs. */
      oracle: (ctx: AdapterContext) => {
        const projection: Record<string, readonly (number | string)[]> = {};
        rows(ctx).forEach((row, index) => {
          projection[`interpolate/number/${index}`] = grid([interpolateNumber(Number(row.a), Number(row.b))(Number(row.t))]);
        });
        return { projection };
      },
      /** 🎯️ The same pairs through the kernel's `number` interpolator. */
      subject: async (ctx: AdapterContext) => await subject(ctx),
    },
    round: {
      /** 🔮️ d3-interpolate's `interpolateRound` on the declared pairs. */
      oracle: (ctx: AdapterContext) => {
        const projection: Record<string, readonly (number | string)[]> = {};
        rows(ctx).forEach((row, index) => {
          projection[`interpolate/round/${index}`] = [interpolateRound(Number(row.a), Number(row.b))(Number(row.t))];
        });
        return { projection };
      },
      /** 🎯️ The same pairs through the kernel's `round` interpolator. */
      subject: async (ctx: AdapterContext) => await subject(ctx),
    },
    colour: {
      /** 🔮️ One d3-interpolate colour interpolator per declared space, on the rendered hexadecimal. */
      oracle: (ctx: AdapterContext) => {
        const projection: Record<string, readonly (number | string)[]> = {};
        for (const row of rows(ctx)) {
          const interpolator = SPACES[row.space!];
          if (interpolator === undefined) throw new Error(`no d3 interpolator for the space ${row.space}`);
          list(row.t!).forEach((position, index) => {
            projection[`interpolate/${row.space}/${index}`] = [`|${rgb(interpolator(`#${row.a}`, `#${row.b}`)(position)).formatHex()}`];
          });
        }
        return { projection };
      },
      /** 🎯️ The same pairs through the kernel's colour kinds. */
      subject: async (ctx: AdapterContext) => await subject(ctx),
    },
    array: {
      /** 🔮️ d3-interpolate's `interpolateArray` over the declared lists. */
      oracle: (ctx: AdapterContext) => {
        const projection: Record<string, readonly (number | string)[]> = {};
        rows(ctx).forEach((row, index) => {
          projection[`interpolate/array/${index}`] = grid(interpolateArray(list(row.a!), list(row.b!))(Number(row.t)) as number[]);
        });
        return { projection };
      },
      /** 🎯️ The same lists through the kernel's `array` interpolator. */
      subject: async (ctx: AdapterContext) => await subject(ctx),
    },
  },
});
//#endregion 🔖️Adapter
