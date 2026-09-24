// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

/** 🎪️ Adapter for `showcase-families`: the nine capability families of `semio-viz-showcase`.
 *
 * Subject: the LaTeX families, probed through `semio-viz-probe` with one scenario per catalogue
 * kind. Oracle: the catalogue itself — no third-party library draws a taxonomy of print
 * visualization capabilities, so the promise a catalogue entry makes is the specification.
 */

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { loadVizCatalog } from "../../🔨️modules/📊️visualization-gallery/🟦️.ts";
import { compileVizProbe, roundProbeNumbers, type ProbeProjection, type ProbeRecord } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "showcase-families";
const DECIMALS = 6;

/** 🗂️ The eight families a probe document can draw from the showcase package alone. */
const CAPABILITY_FAMILIES = [
  "encoding", "grammar", "figure", "scale",
  "shape", "layout-algorithm", "transform-data", "transform-statistical",
] as const;

/** 🗂️ The family that runs every other namespace's canonical family, and therefore the whole library. */
const NAMESPACE_FAMILIES = ["namespace"] as const;

/** 🗂️ The catalogue variants of one showcase family, sorted. */
function catalogueVariants(family: string): readonly string[] {
  const variants = loadVizCatalog()
    .kinds.filter((entry) => entry.family === family)
    .map((entry) => String(entry.options.variant ?? entry.slug));
  return [...variants].sort();
}

/** 🎯️ The catalogue's own promise: every kind present, every kind drawing something of its own. */
export function catalogueProjection(families: readonly string[]): ProbeProjection {
  const projection: Record<string, readonly (number | string)[]> = {};
  for (const family of families) {
    const variants = catalogueVariants(family);
    projection[`showcase/${family}/kinds`] = variants;
    projection[`showcase/${family}/drawing`] = [variants.length];
    projection[`showcase/${family}/distinct`] = [variants.length];
  }
  return projection;
}

/** 🧮️ Folds the probe records into the same three answers, per family. */
export function showcaseProjection(records: readonly ProbeRecord[], families: readonly string[]): ProbeProjection {
  const outlines = new Map<string, Map<string, string>>();
  for (const record of records) {
    const match = /^([a-z-]+)\/([a-z0-9-]+)$/.exec(record.scenario);
    if (match === null) continue;
    const family = match[1]!;
    const variant = match[2]!;
    if (!families.includes(family)) continue;
    const perFamily = outlines.get(family) ?? new Map<string, string>();
    perFamily.set(variant, `${perFamily.get(variant) ?? ""}|${record.key}=${record.values.join(",")}`);
    outlines.set(family, perFamily);
  }
  const projection: Record<string, readonly (number | string)[]> = {};
  for (const family of families) {
    const perFamily = outlines.get(family) ?? new Map<string, string>();
    const drawn = [...perFamily.keys()].filter((variant) => perFamily.get(variant) !== "").sort();
    projection[`showcase/${family}/kinds`] = drawn;
    projection[`showcase/${family}/drawing`] = [drawn.length];
    projection[`showcase/${family}/distinct`] = [new Set(drawn.map((variant) => perFamily.get(variant)!)).size];
  }
  return projection;
}

/** 🎯️ Compiles one committed probe document and reduces it to the projection of its families.
 *
 * The compile is narrowed by case and not by scenario: this document's probe scenario is the
 * catalogue kind being drawn (`<family>/<variant>`), which is what makes one kind's geometry
 * comparable with another's, and the harness scenario names the half of the taxonomy under test.
 */
async function subject(ctx: AdapterContext, fixture: string, families: readonly string[]): Promise<{ projection: ProbeProjection }> {
  const records = roundProbeNumbers(
    await compileVizProbe(ctx.fixture(fixture), { workDir: ctx.workDir, caseName: CASE }),
    DECIMALS,
  );
  return { projection: showcaseProjection(records, families) };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    capabilities: {
      /** 🗂️ The catalogue itself: every registered kind of the eight capability families. */
      oracle: () => ({ projection: catalogueProjection(CAPABILITY_FAMILIES) }),
      /** 🎯️ Every capability kind drawn once, reduced to presence and pairwise distinctness. */
      subject: async (ctx: AdapterContext) => await subject(ctx, "shared://🎪️showcase-families/showcase-capabilities.tex", CAPABILITY_FAMILIES),
    },
    namespaces: {
      /** 🗂️ The catalogue itself: every registered §76 namespace. */
      oracle: () => ({ projection: catalogueProjection(NAMESPACE_FAMILIES) }),
      /** 🎯️ Every namespace rendered by its own canonical family, reduced the same way. */
      subject: async (ctx: AdapterContext) => await subject(ctx, "shared://🎪️showcase-families/showcase-namespaces.tex", NAMESPACE_FAMILIES),
    },
  },
});
// #endregion 🧭️Adapter
