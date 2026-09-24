// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { csvParse, dsvFormat } from "d3-dsv";
import { readFileSync } from "node:fs";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "data-csv";
const FIXTURE = "shared://🗃️data-csv/data-csv.tex";

/** 📄️ The committed file the probe reads, as d3-dsv sees it. */
function source(ctx: AdapterContext, name: string): string {
  return readFileSync(ctx.fixture(`shared://🗃️data-csv/${name}`), "utf8");
}

/** 🧵️ One parsed column in the transport encoding the probe uses, empty cells dropped. */
function column(rows: readonly Record<string, string | undefined>[], name: string): string[] {
  return rows.map((row) => row[name] ?? "").filter((value) => value !== "").map((value) => `|${value}`);
}

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id, extraSources: ["cities.csv", "places.tsv"] });
  return { projection: probeProjection(records, ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "header-row": {
      /** 🔮️ d3-dsv's csvParse of the same committed file. */
      oracle: (ctx: AdapterContext) => {
        const rows = csvParse(source(ctx, "cities.csv"));
        return { projection: { "csv/columns": [...rows.columns], "csv/city": column(rows, "city"), "csv/population": column(rows, "population") } };
      },
      /** 🎯️ The same file loaded by \SemioVizTableFromCSV inside a compiled document. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "quoted-fields": {
      /** 🔮️ d3-dsv's handling of a delimiter inside quotes and of doubled quotes. */
      oracle: (ctx: AdapterContext) => ({ projection: { "csv/note": column(csvParse(source(ctx, "cities.csv")), "note") } }),
      /** 🎯️ The same quoted fields from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "custom-delimiter": {
      /** 🔮️ d3-dsv with a semicolon delimiter over a headerless file, named positionally. */
      oracle: (ctx: AdapterContext) => {
        const records = dsvFormat(";").parseRows(source(ctx, "places.tsv"));
        const names = records[0]!.map((_, index) => `c${index + 1}`);
        const rows = records.map((record) => Object.fromEntries(record.map((value, index) => [names[index]!, value])));
        return { projection: { "tsv/columns": names, "tsv/c1": column(rows, "c1"), "tsv/c3": column(rows, "c3") } };
      },
      /** 🎯️ The same file loaded with the delimiter and header options from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
