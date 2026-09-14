// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { linkHorizontal, linkVertical, linkRadial } from "d3-shape";
import { ribbon } from "d3-chord";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "shape-links-ribbons";
const FIXTURE = "local://shape-links-ribbons.tex";

const DECIMALS = 6;

/** 🔢️ Rounds an oracle's numbers onto the emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🔤️ Splits an SVG path into the token stream the probe records: command letters as
 *  strings, coordinates as numbers, in the order d3 wrote them. */
function tokens(path: string | null): (number | string)[] {
  const out: (number | string)[] = [];
  const pattern = /[MLCQAZhv]|-?\d*\.?\d+(?:e[-+]?\d+)?/gi;
  let match: RegExpExecArray | null;
  while ((match = pattern.exec(path ?? "")) !== null) {
    const raw = match[0];
    const value = Number(raw);
    out.push(Number.isNaN(value) ? raw : grid([value])[0]!);
  }
  return out;
}

type Pair = readonly [number, number];

/** 🔗️ One d3 link of the named flavour between two points. */
function link(kind: string, source: Pair, target: Pair): (number | string)[] {
  if (kind === "radial") {
    const generator = linkRadial<unknown, Pair>().source((d) => (d as Pair[])[0]!).target((d) => (d as Pair[])[1]!)
      .angle((d) => (d as Pair)[0]).radius((d) => (d as Pair)[1]).digits(15);
    return tokens(generator([source, target]));
  }
  const factory = kind === "vertical" ? linkVertical : linkHorizontal;
  const generator = factory<unknown, Pair>().source((d) => (d as Pair[])[0]!).target((d) => (d as Pair[])[1]!)
    .x((d) => (d as Pair)[0]).y((d) => (d as Pair)[1]).digits(15);
  return tokens(generator([source, target]));
}

type RibbonSpec = Readonly<{ radius: number; startAngle: number; endAngle: number; targetRadius: number; targetStartAngle: number; targetEndAngle: number; padAngle: number }>;

/** 🎀️ One d3-chord ribbon between a source and a target span. */
function chord(spec: RibbonSpec): (number | string)[] {
  type Side = { radius: number; startAngle: number; endAngle: number };
  const generator = ribbon<unknown, { source: Side; target: Side }, Side>()
    .source((d) => d.source).target((d) => d.target)
    .sourceRadius((d) => d.radius).targetRadius((d) => d.radius)
    .startAngle((d) => d.startAngle).endAngle((d) => d.endAngle)
    .padAngle(spec.padAngle);
  return tokens(generator({
    source: { radius: spec.radius, startAngle: spec.startAngle, endAngle: spec.endAngle },
    target: { radius: spec.targetRadius, startAngle: spec.targetStartAngle, endAngle: spec.targetEndAngle },
  }) as unknown as string);
}

const RIBBONS: Record<string, RibbonSpec> = {
  plain: { radius: 100, startAngle: 0.2, endAngle: 0.9, targetRadius: 100, targetStartAngle: 2.1, targetEndAngle: 2.9, padAngle: 0 },
  padded: { radius: 100, startAngle: 0.2, endAngle: 0.9, targetRadius: 100, targetStartAngle: 2.1, targetEndAngle: 2.9, padAngle: 0.06 },
  radii: { radius: 120, startAngle: 0.2, endAngle: 0.9, targetRadius: 80, targetStartAngle: 2.1, targetEndAngle: 2.9, padAngle: 0 },
  self: { radius: 100, startAngle: 0.2, endAngle: 0.9, targetRadius: 100, targetStartAngle: 0.2, targetEndAngle: 0.9, padAngle: 0 },
};

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    links: {
      /** 🔮️ d3's three link generators, including the coincident-endpoint case. */
      oracle: () => ({
        projection: {
          "link/horizontal": link("horizontal", [10, 20], [80, 60]),
          "link/vertical": link("vertical", [10, 20], [80, 60]),
          "link/radial": link("radial", [0.5, 30], [2.5, 90]),
          "link/flat": link("horizontal", [10, 20], [10, 20]),
        },
      }),
      /** 🎯️ \SemioVizLink over the same endpoints. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    ribbons: {
      /** 🔮️ d3-chord's ribbon, padded, across two radii, and folded onto itself. */
      oracle: () => ({
        projection: Object.fromEntries(Object.entries(RIBBONS).map(([tag, spec]) => [`ribbon/${tag}`, chord(spec)])),
      }),
      /** 🎯️ \SemioVizRibbon over the same spans. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
