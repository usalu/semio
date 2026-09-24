/** 🌈️ Adapter for `theme-palettes`: the colour kernel every family reads its hues from.
 *
 * Subject: `semio-viz-theme`, probed through `semio-viz-probe`.
 * Oracles: `d3-scale`'s ordinal scale for the palette wrap, `d3-interpolate`'s `piecewise` over
 * `d3-color`'s Lab and HCL for the ramps, and `d3-color` itself for the declared stops.
 */
import { rgb } from "d3-color";
import { interpolateHcl, interpolateLab, piecewise } from "d3-interpolate";
import { scaleOrdinal } from "d3-scale";
import { type AdapterContext, defineTestAdapter } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { type ProbeProjection, compileVizProbe, probeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";

//#region 🔖️Vectors
const CASE = "theme-palettes";
const FIXTURE = "shared://🌈️theme-palettes/theme-palettes.tex";

/** 🥒️ The vector table of the running scenario, as one record per row. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, (row[index] ?? "").trim()])));
}

/** ✂️ A `;`-separated cell as a list. */
function list(cell: string): string[] {
  return cell.split(";").map((item) => item.trim()).filter((item) => item.length > 0);
}

/** 🔤 The transport encoding the probe writes names and colours in. */
function bar(value: string): string {
  return `|${value}`;
}

/** 🌈️ The hexadecimal `d3-color` normalises a declared stop into, with the probe's `#` prefix. */
function hex(stop: string): string {
  return bar(rgb(`#${stop}`).formatHex());
}

/** 🎨️ Every slot index the fixture asks the theme about, for a palette of `size` colours: enough
 * to wrap twice and land past the second wrap, so an off-by-one modulus cannot hide. */
function slots(count: number): number[] {
  return Array.from({ length: count }, (_, index) => index);
}

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(records, ctx.scenario.id) };
}

/** 🔮️ One ordinal scale over the declared range, read at every slot the fixture probes. */
function wrapped(range: readonly string[], count: number): string[] {
  const scale = scaleOrdinal<number, string>().domain(range.map((_, index) => index)).range([...range]);
  return slots(count).map((index) => bar(scale(index % range.length)));
}

/** 🔮️ `d3-interpolate`'s piecewise ramp over the declared stops, in the given colour space. */
function ramp(space: (a: string, b: string) => (t: number) => string, stops: readonly string[], positions: readonly number[]): string[] {
  const interpolator = piecewise(space, stops.map((stop) => `#${stop}`));
  return positions.map((position) => bar(rgb(interpolator(position)).formatHex()));
}
//#endregion 🔖️Vectors

//#region 🔖️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "categorical-wrap": {
      /** 🔮️ d3-scale's ordinal scale over each declared palette, plus the palette's own length. */
      oracle: (ctx: AdapterContext) => {
        const projection: Record<string, readonly (number | string)[]> = {};
        const probed: Record<string, number> = { presence: 18, brand: 14, gray: 14 };
        for (const row of rows(ctx)) {
          const colours = list(row.colours!);
          projection[`slot/${row.palette}`] = wrapped(colours, probed[row.palette!]!);
          projection[`count/${row.palette}`] = [colours.length];
        }
        return { projection };
      },
      /** 🎯️ The colour name `semio-viz-theme` answers for each of those slots. */
      subject: async (ctx: AdapterContext) => await subject(ctx),
    },
    "pattern-wrap": {
      /** 🔮️ The same ordinal wrap over the grayscale theme's hatch set. */
      oracle: (ctx: AdapterContext) => ({ projection: { "pattern/hatch": wrapped(list(rows(ctx)[0]!.patterns!), 17) } }),
      /** 🎯️ The hatch `semio-viz-theme` answers for each of those slots. */
      subject: async (ctx: AdapterContext) => await subject(ctx),
    },
    "scheme-stops": {
      /** 🔮️ Every declared stop as d3-color reads and re-formats it. */
      oracle: (ctx: AdapterContext) => {
        const projection: Record<string, readonly (number | string)[]> = {};
        for (const row of rows(ctx)) {
          for (const appearance of ["light", "dark"] as const) {
            projection[`stops/${appearance}/${row.scheme}`] = list(row[appearance]!).map((stop) => bar(rgb(`#${stop}`).formatHex().slice(1)));
          }
        }
        return { projection };
      },
      /** 🎯️ The stop lists `semio-viz-theme` reports for both appearances. */
      subject: async (ctx: AdapterContext) => await subject(ctx),
    },
    "scheme-ramp-lab": {
      /** 🔮️ d3-interpolate's piecewise ramp over the same stops, in d3-color's CIE Lab. */
      oracle: (ctx: AdapterContext) => {
        const projection: Record<string, readonly (number | string)[]> = {};
        for (const row of rows(ctx)) {
          const samples = ramp(interpolateLab, list(row.stops!), list(row.positions!).map(Number));
          samples.forEach((colour, index) => {
            projection[`ramp/lab/${row.scheme}/${index}`] = [colour];
          });
        }
        return { projection };
      },
      /** 🎯️ The same ramp sampled through the theme with `interpolator=lab`. */
      subject: async (ctx: AdapterContext) => await subject(ctx),
    },
    "scheme-ramp-hcl": {
      /** 🔮️ d3-interpolate's piecewise ramp over the same stops, in d3-color's HCL. */
      oracle: (ctx: AdapterContext) => {
        const projection: Record<string, readonly (number | string)[]> = {};
        for (const row of rows(ctx)) {
          const samples = ramp(interpolateHcl, list(row.stops!), list(row.positions!).map(Number));
          samples.forEach((colour, index) => {
            projection[`ramp/hcl/${row.scheme}/${index}`] = [colour];
          });
        }
        return { projection };
      },
      /** 🎯️ The same ramp sampled through the theme with `interpolator=hcl`. */
      subject: async (ctx: AdapterContext) => await subject(ctx),
    },
    "scheme-ramp-oklab": {
      /** 🔮️ The stops themselves, as d3-color normalises them: an interpolation space owes its own
       * knots back unchanged, whatever it does between them, and OKLab has no d3 reference to
       * adjudicate the space in between. */
      oracle: (ctx: AdapterContext) => {
        const projection: Record<string, readonly (number | string)[]> = {};
        for (const row of rows(ctx)) {
          list(row.stops!).forEach((stop, index) => {
            projection[`ramp/oklab/${row.scheme}/${index}`] = [hex(stop)];
          });
        }
        return { projection };
      },
      /** 🎯️ The default OKLab ramp sampled at its own stop positions. */
      subject: async (ctx: AdapterContext) => await subject(ctx),
    },
  },
});
//#endregion 🔖️Adapter
