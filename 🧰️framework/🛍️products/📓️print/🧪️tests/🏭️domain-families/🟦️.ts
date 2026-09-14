// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { descending, extent, sort } from "d3-array";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbeDocument, roundProbeNumbers, type ProbeProjection, type ProbeRecord, type VizProbeStatement } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "domain-families";
const DECIMALS = 3;
const PACKAGES = ["tikz", "semio-viz-domain", "semio-viz-text"] as const;
const CATALOG = join(import.meta.dir, "..", "..", "🖼️assets", "🔣️viz-catalog.json");

/** 🗂️ One catalogue entry, reduced to what a family run needs. */
type CatalogKind = Readonly<{ id: string; slug: string; family: string; options: Readonly<Record<string, unknown>> }>;

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 🗂️ The catalogue entries of the families the table names, ordered by family and then by kind id. */
function kinds(families: readonly string[]): CatalogKind[] {
  const catalog = JSON.parse(readFileSync(CATALOG, "utf8")) as { kinds: CatalogKind[] };
  const wanted = new Set(families);
  return catalog.kinds
    .filter((kind) => wanted.has(kind.family))
    .sort((a, b) => (a.family === b.family ? a.id.localeCompare(b.id) : a.family.localeCompare(b.family)));
}

/** 🔑 One catalogue option set as the `l3keys` list `\SemioVizRunFamily` expects. */
function optionList(options: Readonly<Record<string, unknown>>): string {
  return Object.entries(options)
    .map(([key, value]) => {
      if (typeof value === "boolean") return `${key}=${value ? "true" : "false"}`;
      const text = String(value);
      return `${key}=${text.includes(",") ? `{${text}}` : text}`;
    })
    .join(",");
}
// #endregion 🧫️Vectors

// #region 🧪️Probe
/** 🧪️ One probe document that runs every catalogue kind of the named families, each preceded by
 * a string record naming the kind, so the record stream can be cut into one stream per kind. */
function coverageStatements(entries: readonly CatalogKind[]): VizProbeStatement[] {
  const body: VizProbeStatement[] = [{ precision: DECIMALS }, { raw: "\\SemioVizProbeOn" }];
  for (const kind of entries) {
    body.push({ text: { key: "kind", value: `${kind.family}/${kind.slug}` } });
    body.push({ raw: `\\begin{tikzpicture}[x=1mm,y=1mm]\\SemioVizRunFamily{${kind.family}}[${optionList(kind.options)}]\\end{tikzpicture}` });
  }
  return body;
}

/** ✂️ Cuts the record stream at every `kind` marker into one signature per catalogue kind. */
function signatures(records: readonly ProbeRecord[]): Map<string, string[]> {
  const streams = new Map<string, string[]>();
  let current: string[] | undefined;
  for (const record of records) {
    if (record.key === "kind") {
      current = [];
      streams.set(String(record.values[0]), current);
      continue;
    }
    current?.push(`${record.key}:${record.values.join(",")}`);
  }
  return streams;
}

/** 🎯️ Kind count, distinct-signature count and empty-stream count of every family, in table order. */
async function coverage(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const families = rows(ctx).map((row) => row.family!);
  const entries = kinds(families);
  const records = await compileVizProbeDocument(
    { case: CASE, scenario: ctx.scenario.id, packages: PACKAGES, body: coverageStatements(entries) },
    { workDir: ctx.workDir },
  );
  const streams = signatures(roundProbeNumbers(records, DECIMALS));
  const measured: number[] = [];
  for (const family of families) {
    const own = entries.filter((kind) => kind.family === family);
    const own_streams = own.map((kind) => (streams.get(`${family}/${kind.slug}`) ?? []).join("|"));
    measured.push(own_streams.length, new Set(own_streams).size, own_streams.filter((stream) => stream === "").length);
  }
  return { projection: { "catalog/domain-families": measured } };
}

/** 🧪️ The §13 reading-order grid over the table's own term frequencies, one column wide. */
function termStatements(ctx: AdapterContext): VizProbeStatement[] {
  const table = rows(ctx);
  const first = table[0]!;
  const body: VizProbeStatement[] = [
    { precision: DECIMALS },
    { raw: "\\SemioVizTable{terms}{term,count}" },
    ...table.map((row) => ({ raw: `\\SemioVizRow{terms}{${row.term},${row.count}}` })),
    { raw: "\\SemioVizProbeOn" },
    {
      raw:
        `\\begin{tikzpicture}[x=1mm,y=1mm]\\SemioVizRunFamily{text-viz}` +
        `[data=terms,term=term,count=count,layout=grid,sort=count,columns=1,` +
        `sizemin=${first.sizemin},sizemax=${first.sizemax}]\\end{tikzpicture}`,
    },
  ];
  return body;
}

/** 🎯️ The glyph body height of every placed word, in placement order. */
async function termSizes(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbeDocument(
    { case: CASE, scenario: ctx.scenario.id, packages: PACKAGES, body: termStatements(ctx) },
    { workDir: ctx.workDir },
  );
  const values = roundProbeNumbers(records, DECIMALS)
    .filter((record) => record.key === "geometry/word")
    .map((record) => Number(record.values[2]));
  return { projection: { "geometry/word/size": values } };
}
// #endregion 🧪️Probe

// #region 🔮️Oracle
/** 📋️ The specification: a family that owns n kinds yields n streams, n signatures and no empty one. */
function specifiedCoverage(ctx: AdapterContext): ProbeProjection {
  const flat: number[] = [];
  for (const row of rows(ctx)) flat.push(Number(row.kinds), Number(row.kinds), 0);
  return { "catalog/domain-families": flat };
}

/** 🔮️ d3-array: the frequency extent is the scale domain, `descending` is the placement order. */
function d3TermSizes(ctx: AdapterContext): ProbeProjection {
  const table = rows(ctx);
  const first = table[0]!;
  const min = Number(first.sizemin);
  const max = Number(first.sizemax);
  const counts = table.map((row) => Number(row.count));
  const [lo, hi] = extent(counts) as [number, number];
  const ordered = sort(counts, (a, b) => descending(a, b));
  const scaled = ordered.map((count) => Number((min + ((max - min) * (count - lo)) / Math.max(1e-9, hi - lo)).toFixed(DECIMALS)));
  return { "geometry/word/size": scaled };
}
// #endregion 🔮️Oracle

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "every-domain-kind-draws-its-own-geometry": {
      /** 📋️ The counts the feature specifies for every family it names. */
      oracle: (ctx: AdapterContext) => ({ projection: specifiedCoverage(ctx) }),
      /** 🎯️ The same counts measured from the compiled catalogue kinds. */
      subject: async (ctx: AdapterContext) => await coverage(ctx),
    },
    "term-glyph-size-follows-the-frequency-extent": {
      /** 🔮️ d3-array's extent and descending order over the same frequencies. */
      oracle: (ctx: AdapterContext) => ({ projection: d3TermSizes(ctx) }),
      /** 🎯️ The glyph body heights the §13 grid layout writes. */
      subject: async (ctx: AdapterContext) => await termSizes(ctx),
    },
  },
});
// #endregion 🧭️Adapter
