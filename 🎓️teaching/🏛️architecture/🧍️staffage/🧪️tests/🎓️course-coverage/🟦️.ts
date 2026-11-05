/** 🎓️ Course-wide authored staffage coverage, sharing vectors with the Python gate.
 * @see ../../🧫️fixtures/🎓️course-coverage/🔣️.json
 */
import { readFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";
import { isDeepStrictEqual } from "node:util";
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { STAFFAGE_ROOT, fixture } from "../🧰️support/🧰️.ts";

type Data = Record<string, any>;
type Entry = { category: string; row: Data | null; asset: Data };
type Source = { path: string; text: string };
const cases = fixture<Data>("🎓️course-coverage");
const architecture = join(STAFFAGE_ROOT, "..");
const enums = cases.enums;
const finite = (value: unknown, minimum = 0, strict = false): value is number => typeof value === "number" && Number.isFinite(value) && (strict ? value > minimum : value >= minimum);
const same = (a: unknown, b: unknown): boolean => isDeepStrictEqual(a, b);
const read = (path: string): Data => JSON.parse(readFileSync(path, "utf8"));
const children = (path: string): string[] => readdirSync(path, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => entry.name);
const manifest = (): Data => read(join(architecture, cases.manifest));
const catalog = (): Entry[] => {
  const root = join(architecture, cases.catalog), index = read(join(root, "🔣️.json"));
  return children(root).flatMap((category) => children(join(root, category)).map((name) => {
    const asset = read(join(root, category, name, "🔣️.json")).asset;
    return { category: category.replace(/^[^a-z0-9]+/u, ""), row: index.entries.find((row: Data) => row.id === asset.id) ?? null, asset };
  }));
};
const presets = (): Set<string> => {
  const root = join(architecture, cases.catalog);
  return new Set(children(root).filter((name) => name.includes("environments")).flatMap((name) => read(join(root, name, "🔣️.json")).presets.map((preset: Data) => preset.id)));
};

/** 📦️ Resolve every cast against the catalog rather than accepting derived representations. */
function issues(manifest: Data, entries: Entry[], environments: Set<string>): string[] {
  const found: string[] = [];
  const issue = (code: string): void => { found.push(code); };
  if (manifest.version !== 1) issue("manifest-version");
  for (const cast of Object.values(manifest.casts) as Data[]) {
    for (const key of Object.keys(cast)) if (!cases.cast_fields.includes(key)) issue("field-unknown");
    const matches = entries.filter(({ asset }) => asset.id === cast.asset);
    if (!matches.length) { issue("asset-unknown"); continue; }
    if (matches.length > 1) { issue("asset-ambiguous"); continue; }
    const { asset, row, category } = matches[0]!;
    if (category !== asset.category) issue("category-mismatch");
    if (!row) issue("row-missing");
    else {
      const facts: Data = { category: asset.category, heightM: asset.dimensions?.heightM, poses: (asset.poses ?? []).filter((pose: Data) => !pose.key).map((pose: Data) => pose.id), clips: (asset.clips ?? []).map((clip: Data) => clip.id), representations: Object.fromEntries(Object.entries(asset.representations ?? {}).map(([key, value]) => [key, (value as Data).status])) };
      for (const [key, value] of Object.entries(facts)) if (!same(row[key], value)) issue("row-mismatch");
    }
    for (const key of ["heightM", "widthM", "depthM"]) if (!finite(asset.dimensions?.[key], 0, true)) issue("units");
    if (!finite(asset.dimensions?.massKg)) issue("units");
    const representation = asset.representations?.[cast.scale];
    if (!("scale" in cast)) issue("scale-missing");
    else if (!enums.scales.includes(cast.scale)) issue("scale-preset");
    else if (representation?.status !== "authored") issue("scale-unauthored");
    else {
      if (representation.denominator !== cast.scale) issue("scale-denominator");
      if (enums.viewpoints.includes(cast.viewpoint) && !representation.geometry?.[cast.viewpoint]?.length) issue("viewpoint-empty");
    }
    if (!enums.styles.includes(cast.view)) issue("view-style");
    if (!enums.viewpoints.includes(cast.viewpoint)) issue("viewpoint-unknown");
    if (!enums.themes.includes(cast.theme)) issue("theme-unknown");
    if ("mirror" in cast && (typeof cast.mirror !== "boolean" || (cast.mirror && cast.viewpoint !== "elevation"))) issue("mirror-viewpoint");
    if (Number("pose" in cast) + Number("clip" in cast) !== 1) issue("state-count");
    if ("pose" in cast) {
      const pose = asset.poses?.find((pose: Data) => pose.id === cast.pose);
      if (!pose) issue("pose-unknown");
      else if (pose.key) issue("pose-key");
    }
    if ("clip" in cast && !asset.clips?.some((clip: Data) => clip.id === cast.clip)) issue("clip-unknown");
    const timed = "clip" in cast || "timeline" in cast;
    if (timed && !("fps" in cast)) issue("fps-missing");
    else if ("fps" in cast && !timed) issue("fps-forbidden");
    else if ("fps" in cast && (!finite(cast.fps, 1) || !Number.isInteger(cast.fps) || cast.fps > 120)) issue("fps-range");
    if (cast.view === "thermal") {
      if (("environment" in cast) === ("timeline" in cast)) issue("thermal-scenario");
      const raw = cast.timeline, keys = Array.isArray(raw) ? raw.filter((key) => key && typeof key === "object") : [];
      if ("timeline" in cast && (!Array.isArray(raw) || !raw.length || keys.length !== raw.length)) issue("timeline-key");
      const slugs = [...("environment" in cast ? [cast.environment] : []), ...keys.map((key: Data) => key.environment)];
      if (slugs.some((slug) => !environments.has(slug))) issue("preset-unknown");
      if (keys.some((key: Data) => !enums.eases.includes(key.ease))) issue("ease-missing");
      if (keys.some((key: Data) => Object.keys(key).some((name) => !["t", "environment", "ease"].includes(name)))) issue("timeline-key");
      const times = keys.map((key: Data) => key.t);
      if (times.some((time: unknown) => !finite(time)) || new Set(times).size !== times.length) issue("timeline-key");
      if (!asset.thermal?.regions?.length) issue("thermal-regions");
      if ("thermalSpanK" in cast && !finite(cast.thermalSpanK, 0, true)) issue("span-range");
      if ("transport" in cast && (!Array.isArray(cast.transport) || cast.transport.some((layer: unknown) => !enums.transport.includes(layer)) || new Set(cast.transport).size !== cast.transport.length)) issue("transport-layer");
      if ("transport" in cast && !("transportAgeS" in cast)) issue("transport-age");
      if (["transportAgeS", "transportStrokeMm", "transportTrailS"].some((key) => key in cast) && (!cast.transport?.length || ("transportAgeS" in cast && (!("clip" in cast) || !finite(cast.transportAgeS, 0, true))) || ["transportStrokeMm", "transportTrailS"].some((key) => key in cast && !finite(cast[key], 0, true)))) issue("transport-age");
    } else for (const key of ["environment", "timeline", "thermalSpanK", "transport", "transportAgeS", "transportStrokeMm", "transportTrailS"]) if (key in cast) issue("scenario-forbidden");
    if (["forces", "moments"].includes(cast.view) && (!enums.dynamics.includes(asset.structure?.dynamics) || !finite(asset.structure?.gravityMS2, 0, true))) issue("structural-model");
    for (const [name, required] of [["thermal", cast.view === "thermal"], ["transport", !!cast.transport?.length], ["structural", ["forces", "moments"].includes(cast.view)]]) if (required && row && !row.analytics?.includes(name)) issue("analytics");
  }
  return found;
}

/** 🔎️ Track literal names, dynamic cast prefixes, unknown direct references and unused entries. */
function scan(sources: Source[], manifest: Data): Data {
  const casts = Object.keys(manifest.casts), references = new Set<string>(), prefixes = new Set<string>(), broken = new Set<string>();
  const patterns = [new RegExp("\\b(?:" + cases.sources.cast_calls.join("|") + ")\\(\\s*([\"'])([^\"'\\n\\\\]*)\\1", "gu"), new RegExp("\\b(?:" + cases.sources.cast_keywords.join("|") + ")\\s*=\\s*([\"'])([^\"'\\n\\\\]*)\\1", "gu")];
  for (const source of sources) {
    for (const match of source.text.matchAll(/(["'])([^"'\n\\]*)\1/gu)) {
      const name = match[2]!;
      if (casts.includes(name)) references.add(name);
      else if (name.endsWith("-") && name.length > 1 && casts.some((cast) => cast.startsWith(name))) prefixes.add(name);
    }
    for (const pattern of patterns) for (const match of source.text.matchAll(pattern)) if (!casts.includes(match[2]!)) broken.add(match[2]!);
  }
  return { references: [...references].sort(), prefixes: [...prefixes].sort(), orphans: casts.filter((name) => !references.has(name) && ![...prefixes].some((prefix) => name.startsWith(prefix))).sort(), broken: [...broken].sort() };
}

describe("🎓️ course staffage coverage", () => {
  const synthetic = cases.synthetic;
  it("accepts shared valid vectors and independently validates them with Ajv", () => {
    expect(issues(synthetic.valid, synthetic.catalog, new Set(synthetic.presets))).toEqual([]);
    const schema = read(join(STAFFAGE_ROOT, "🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json"));
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(synthetic.valid), JSON.stringify(validate.errors)).toBe(true);
  });
  for (const vector of synthetic.invalid) it("rejects " + vector.name, () => {
    const entries = structuredClone(synthetic.catalog) as Entry[], cast = { ...structuredClone(synthetic.valid.casts[vector.cast]), ...vector.patch };
    const entry = entries.find(({ asset }) => asset.id === synthetic.valid.casts[vector.cast].asset)!;
    for (const key of vector.remove ?? []) delete cast[key];
    Object.assign(entry.asset, vector.asset_patch);
    if (vector.row_remove) entry.row = null;
    else if (entry.row) Object.assign(entry.row, vector.row_patch);
    if (vector.catalog_duplicate) entries.push(structuredClone(entry));
    expect(issues({ version: 1, casts: { bad: cast } }, entries, new Set(synthetic.presets))).toContain(vector.code);
  });
  it("agrees with the independent schema oracle on invalid cast syntax", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(read(join(STAFFAGE_ROOT, "🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json")));
    const codes = new Set(["scale-missing", "scale-preset", "viewpoint-unknown", "view-style", "theme-unknown", "mirror-viewpoint", "state-count", "fps-missing", "fps-forbidden", "fps-range", "thermal-scenario", "scenario-forbidden", "ease-missing", "span-range", "transport-layer", "transport-age", "field-unknown"]);
    for (const vector of synthetic.invalid.filter((vector: Data) => codes.has(vector.code))) {
      const cast = { ...structuredClone(synthetic.valid.casts[vector.cast]), ...vector.patch };
      for (const key of vector.remove ?? []) delete cast[key];
      expect(validate({ version: 1, casts: { bad: cast } }), vector.name).toBe(false);
    }
  });
  it("covers every declared diagnostic", () => {
    expect([...new Set([...synthetic.invalid.map((vector: Data) => vector.code), ...synthetic.invalid_sources.map((vector: Data) => vector.code), "manifest-version", "series-missing"])].sort()).toEqual([...cases.codes].sort());
    for (const version of synthetic.invalid_versions) expect(issues({ version, casts: {} }, [], new Set())).toEqual(["manifest-version"]);
  });
  it("resolves shared source names, prefixes, unknown names and orphans", () => {
    expect(scan(synthetic.valid_sources, synthetic.valid)).toEqual(synthetic.valid_expect);
    expect(scan(synthetic.prefix_sources, synthetic.valid)).toEqual(synthetic.prefix_expect);
    for (const vector of synthetic.invalid_sources) expect(scan(vector.sources, synthetic.valid)[vector.code === "ref-unknown" ? "broken" : "orphans"]).toContain(vector.mention);
  });
  it("resolves every course cast to its authored catalog scale and analytical scenario", () => {
    const course = manifest();
    const validate = new Ajv({ strict: true, allErrors: true }).compile(read(join(STAFFAGE_ROOT, "🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json")));
    expect(validate(course), JSON.stringify(validate.errors)).toBe(true);
    expect(issues(course, catalog(), presets())).toEqual([]);
  });
  it("scans every course series and rejects unknown or unreferenced casts", () => {
    const sources: Source[] = [];
    const walk = (root: string): void => {
      for (const entry of readdirSync(root, { withFileTypes: true })) {
        const path = join(root, entry.name);
        if (entry.isDirectory() && !cases.sources.skip_folders.includes(entry.name)) walk(path);
        else if (entry.isFile() && entry.name.endsWith(cases.sources.extension)) sources.push({ path: relative(architecture, path), text: readFileSync(path, "utf8") });
      }
    };
    for (const series of cases.series) walk(join(architecture, series));
    expect(sources.length).toBeGreaterThan(0);
    const report = scan(sources, manifest());
    expect(report.broken).toEqual([]);
    expect(report.orphans).toEqual([]);
  });
});
