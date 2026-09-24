/** 🪶️ Adapter for `plot-grammar`: `\SemioVizPlot`, the one call of taxonomy §79.
 *
 * Subject: `semio-viz-plot`, probed through `semio-viz-probe` — the channel registry for the
 * vocabulary, the `plot/point` record for the placement.
 * Oracles: `d3-scale` for the linear and band placement, and the TypeScript twin's `planVizChart`
 * for the pipeline as a whole.
 */
import { scaleBand, scaleLinear } from "d3-scale";
import { type AdapterContext, defineTestAdapter } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { planVizChart } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import { type ProbeProjection, type ProbeRecord, compileVizProbe, roundProbeNumbers } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";

//#region 🔖️Vectors
const CASE = "plot-grammar";
const FIXTURE = "shared://🪶️plot-grammar/plot-grammar.tex";
const DECIMALS = 4;

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

/** 🔢️ Rounds a number onto the grid the `plot/point` record is written on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🎯️ Compiles the committed fixture and returns the records of one scenario. */
async function records(ctx: AdapterContext): Promise<ProbeRecord[]> {
  return roundProbeNumbers(await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id }), DECIMALS);
}

/** 📍️ The mapped rows of a probed plot, flattened as `x0, y0, x1, y1, …`. The mark kernel's own
 * records are left out: `mark-geometry` adjudicates the glyph, this case the placement. */
function points(probed: readonly ProbeRecord[]): number[] {
  return probed.filter((record) => record.key === "geometry/plot/point").flatMap((record) => record.values.map(Number));
}
//#endregion 🔖️Vectors

//#region 🔖️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "channel-vocabulary": {
      /** 🔮️ The specification: sixteen declared channels, bound exactly when an option list names
       * them, and a binding that records the column with the scale only when one is given. */
      oracle: (ctx: AdapterContext) => {
        const channels = list(rows(ctx)[0]!.channels!);
        const bound = (named: readonly string[]): number[] => channels.map((channel) => (named.includes(channel) ? 1 : 0));
        return {
          projection: {
            "channels/none": bound([]),
            "channels/positional": bound(["x", "y"]),
            "channels/all": bound(channels),
            "binding/plain": ["|cat", "|x"],
            "binding/scaled": ["|val", "|plot-y"],
            "binding/unbound": ["|", "|"],
          },
        };
      },
      /** 🎯️ What the plot's channel registry answers for each of those option lists. */
      subject: async (ctx: AdapterContext) => {
        const probed = await records(ctx);
        const projection: Record<string, readonly (number | string)[]> = {};
        for (const record of probed) projection[record.key] = record.values;
        return { projection };
      },
    },
    "linear-mapping": {
      /** 🔮️ Two d3-scale linear scales over the declared domains and the plot rectangle. */
      oracle: (ctx: AdapterContext) => {
        const row = rows(ctx)[0]!;
        const x = scaleLinear(list(row.xDomain!).map(Number), list(row.xRange!).map(Number));
        const y = scaleLinear(list(row.yDomain!).map(Number), list(row.yRange!).map(Number));
        const xs = list(row.x!).map(Number);
        const ys = list(row.y!).map(Number);
        return { projection: { "plot/point": grid(xs.flatMap((value, index) => [x(value), y(ys[index]!)])) } };
      },
      /** 🎯️ Where `\SemioVizPlot` put each row of the same table. */
      subject: async (ctx: AdapterContext) => ({ projection: { "plot/point": points(await records(ctx)) } as ProbeProjection }),
    },
    "band-mapping": {
      /** 🔮️ d3-scale's band scale for the categorical channel, read at the centre of each band. */
      oracle: (ctx: AdapterContext) => {
        const row = rows(ctx)[0]!;
        const categories = list(row.x!);
        const range = list(row.xRange!).map(Number);
        const x = scaleBand<string>().domain(categories).range([range[0]!, range[1]!]);
        const y = scaleLinear(list(row.yDomain!).map(Number), list(row.yRange!).map(Number));
        const ys = list(row.y!).map(Number);
        return { projection: { "plot/point": grid(categories.flatMap((value, index) => [x(value)! + x.bandwidth() / 2, y(ys[index]!)])) } };
      },
      /** 🎯️ Where `\SemioVizPlot` put each row of the same table. */
      subject: async (ctx: AdapterContext) => ({ projection: { "plot/point": points(await records(ctx)) } as ProbeProjection }),
    },
    "twin-plan": {
      /** 🔮️ The TypeScript twin resolving the same specification — the same table, the same channel
       * bindings, the same frame — into drawing primitives, read at the centre of each point mark. */
      oracle: (ctx: AdapterContext) => {
        const row = rows(ctx)[0]!;
        const table = {
          name: "demo",
          columns: ["cat", "t", "val", "val2"],
          rows: [
            { cat: "A", t: 1, val: 4, val2: 2 },
            { cat: "B", t: 2, val: 7, val2: 3 },
            { cat: "C", t: 3, val: 3, val2: 5 },
            { cat: "D", t: 4, val: 8, val2: 4 },
            { cat: "E", t: 5, val: 5, val2: 6 },
          ],
        };
        const plan = planVizChart({
          width: Number(row.width),
          height: Number(row.height),
          margin: { top: Number(row.marginTop), right: Number(row.marginRight), bottom: Number(row.marginBottom), left: Number(row.marginLeft) },
          tables: [table],
          scales: [
            { name: "plot-x", kind: "linear", domain: list(row.xDomain!).map(Number), range: [Number(row.marginLeft), Number(row.width) - Number(row.marginRight)] },
            { name: "plot-y", kind: "linear", domain: list(row.yDomain!).map(Number), range: [Number(row.marginTop), Number(row.height) - Number(row.marginBottom)] },
          ],
          layers: [
            {
              mark: "point",
              data: "demo",
              encodings: { x: { column: row.xColumn!, scale: "plot-x" }, y: { column: row.yColumn!, scale: "plot-y" } },
            },
          ],
        });
        const circles = plan.items.filter((item): item is Extract<typeof item, { kind: "circle" }> => item.kind === "circle");
        return { projection: { "plot/point": grid(circles.flatMap((item) => [item.cx, item.cy])) } };
      },
      /** 🎯️ Where the LaTeX pipeline put the same rows. */
      subject: async (ctx: AdapterContext) => ({ projection: { "plot/point": points(await records(ctx)) } as ProbeProjection }),
    },
  },
});
//#endregion 🔖️Adapter
