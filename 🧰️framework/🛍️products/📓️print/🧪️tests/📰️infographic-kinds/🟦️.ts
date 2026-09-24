/** 📰️ Adapter for `infographic-kinds`: the four families of `semio-viz-infographic`.
 *
 * Subject: the LaTeX families, probed through `semio-viz-probe` with one scenario per catalogue
 * kind. Oracle: the catalogue itself — no third-party library draws an infographic, so the promise
 * a catalogue entry makes is the specification, exactly as in `showcase-families`.
 */
import assert from "node:assert/strict";
import { type AdapterContext, defineTestAdapter } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { loadVizCatalog } from "../../🔨️modules/📊️visualization-gallery/🟦️.ts";
import { type ProbeProjection, type ProbeRecord, compileVizProbe, roundProbeNumbers } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";

//#region 🔖️Vectors
const CASE = "infographic-kinds";
const DECIMALS = 6;

/** 🗂️ The four families the §44 catalogue kinds are served from. */
const FAMILIES = ["infographic-number", "infographic-list", "infographic-icon", "infographic-illustration"] as const;

/** 🗂️ The catalogue variants of one family, sorted. */
function catalogueVariants(family: string): readonly string[] {
  return loadVizCatalog()
    .kinds.filter((entry) => entry.family === family)
    .map((entry) => String(entry.options.variant ?? entry.slug))
    .sort();
}

/** 🎯️ The catalogue's own promise: every kind present, every kind drawing something of its own. */
export function catalogueProjection(families: readonly string[]): ProbeProjection {
  const projection: Record<string, readonly (number | string)[]> = {};
  for (const family of families) {
    const variants = catalogueVariants(family);
    projection[`infographic/${family}/kinds`] = variants;
    projection[`infographic/${family}/drawing`] = [variants.length];
    projection[`infographic/${family}/distinct`] = [variants.length];
  }
  return projection;
}

/** 🧮️ Folds the probe records into the same three answers, per family: which kinds emitted an
 * outline at all, how many did, and how many of those outlines are pairwise different. */
export function infographicProjection(records: readonly ProbeRecord[], families: readonly string[]): ProbeProjection {
  const outlines = new Map<string, Map<string, string>>();
  for (const record of records) {
    const match = /^([a-z-]+)\/([a-z0-9-]+)$/.exec(record.scenario);
    if (match === null) continue;
    const family = match[1]!;
    if (!families.includes(family)) continue;
    const perFamily = outlines.get(family) ?? new Map<string, string>();
    const variant = match[2]!;
    perFamily.set(variant, `${perFamily.get(variant) ?? ""}|${record.key}=${record.values.join(",")}`);
    outlines.set(family, perFamily);
  }
  const projection: Record<string, readonly (number | string)[]> = {};
  for (const family of families) {
    const perFamily = outlines.get(family) ?? new Map<string, string>();
    const drawn = [...perFamily.keys()].filter((variant) => perFamily.get(variant) !== "").sort();
    projection[`infographic/${family}/kinds`] = drawn;
    projection[`infographic/${family}/drawing`] = [drawn.length];
    projection[`infographic/${family}/distinct`] = [new Set(drawn.map((variant) => perFamily.get(variant)!)).size];
  }
  return projection;
}

/** 🎯️ Compiles the committed probe document, reduces it to the projection of its families and
 * holds it against the catalogue. The recorded no-oracle decision rests on specification vectors,
 * which the platform discharges *inside* the scenario rather than by an oracle-versus-subject
 * comparison, so the specification has to be asserted here or the case would have no teeth. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = roundProbeNumbers(await compileVizProbe(ctx.fixture("shared://📰️infographic-kinds/infographic-kinds.tex"), { workDir: ctx.workDir, caseName: CASE }), DECIMALS);
  const projection = infographicProjection(records, FAMILIES);
  assert.deepEqual(projection, catalogueProjection(FAMILIES));
  return { projection };
}
//#endregion 🔖️Vectors

//#region 🔖️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    kinds: {
      /** 🗂️ The catalogue itself: every registered §44 kind of the four infographic families. */
      oracle: () => ({ projection: catalogueProjection(FAMILIES) }),
      /** 🎯️ Every infographic kind drawn once, reduced to presence and pairwise distinctness. */
      subject: async (ctx: AdapterContext) => await subject(ctx),
    },
  },
});
//#endregion 🔖️Adapter
