/** 🏷️ The cast provenance contract: every missing combination is refused, a revision is the hash of the catalog text, and every exported cast says where it came from (checked by ajv against the schema).
 *
 * @see ../../🧫️fixtures/🏷️cast-provenance/🔣️.json — the language-agnostic vectors, shared with the Python bridge
 * @see ../../🔨️modules/📤️export/🎭️cast/🟦️.ts — the exporter
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { type CastIndex, type CastManifest, castManifestIssues } from "../../🔨️modules/📤️export/🎭️cast/🧬️schema/🟦️.ts";
import { canonicalJson, exportCast, presetRevision, revisionOfText } from "../../🔨️modules/📤️export/🎭️cast/🟦️.ts";
import { loadCatalogFromFiles } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { STAFFAGE_ROOT, fixture, withScratch } from "../🧰️support/🧰️.ts";

type Entry = { name: string; cast: Record<string, unknown>; issue?: string };
type Edit = { name: string; remove?: string; set?: { path: string; value: unknown } };
type Vectors = {
  revisions: { name: string; text: string; revision: string }[];
  canonical: { name: string; source: string; text: string }[];
  preset: { source: string; text: string; revision: string };
  validCasts: Entry[];
  invalidCasts: Entry[];
  indexes: { valid: { name: string; index: Record<string, unknown> }[]; invalid: Edit[] };
};

const vectors = fixture<Vectors>("🏷️cast-provenance");
const exporting = fixture<{ manifest: CastManifest }>("🎭️cast-export");
const schema = JSON.parse(readFileSync(join(STAFFAGE_ROOT, "🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json"), "utf8"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const manifestOf = ajv.getSchema(schema.$id)!;
const indexOf = ajv.getSchema(`${schema.$id}#/$defs/Index`)!;
const wrapped = (cast: unknown) => ({ version: 1, casts: { vector: cast } });

function edited(index: Record<string, unknown>, change: Edit): unknown {
  const copy = JSON.parse(JSON.stringify(index)) as Record<string, unknown>;
  const path = (change.remove ?? change.set!.path).split("/");
  const last = path.pop()!;
  const holder = path.reduce<Record<string, unknown>>((node, key) => node[key] as Record<string, unknown>, copy);
  if (change.remove !== undefined) delete holder[last];
  else holder[last] = change.set!.value;
  return copy;
}

describe("🏷️ cast contract vectors in two implementations", () => {
  for (const { name, cast } of vectors.validCasts)
    it(`ajv and the twin accept ${name}`, () => {
      expect(manifestOf(wrapped(cast)), JSON.stringify(manifestOf.errors?.slice(0, 3))).toBe(true);
      expect(castManifestIssues(wrapped(cast))).toEqual([]);
    });

  for (const { name, cast, issue } of vectors.invalidCasts)
    it(`ajv and the twin refuse ${name}`, () => {
      expect(manifestOf(wrapped(cast))).toBe(false);
      expect(castManifestIssues(wrapped(cast)).join("\n")).toContain(issue);
    });

  for (const { name, text, revision } of vectors.revisions) it(`the revision of ${name} text is ${revision.slice(0, 15)}…`, () => expect(revisionOfText(text)).toBe(revision));

  for (const { name, source, text } of vectors.canonical) it(`canonical JSON: ${name}`, () => expect(canonicalJson(JSON.parse(source))).toBe(text));

  it("a preset revision hashes its canonical JSON", () => {
    expect(canonicalJson(JSON.parse(vectors.preset.source))).toBe(vectors.preset.text);
    expect(presetRevision(JSON.parse(vectors.preset.source))).toBe(vectors.preset.revision);
  });

  for (const { name, index } of vectors.indexes.valid) it(`ajv accepts the index with ${name}`, () => expect(indexOf(index), JSON.stringify(indexOf.errors?.slice(0, 3))).toBe(true));

  const sample = vectors.indexes.valid[0]!.index;
  for (const change of vectors.indexes.invalid) it(`ajv refuses an index with ${change.name}`, () => expect(indexOf(edited(sample, change))).toBe(false));
});

describe("🏷️ cast provenance written by the exporter", { timeout: 60_000 }, () => {
  const pick = (...ids: string[]): CastManifest => ({ version: 1, casts: Object.fromEntries(ids.map((id) => [id, exporting.manifest.casts[id]!])) });

  it("describes asset revision, view, scale, state, clock and scenario of a clip, a still pose and a transient, and the index passes the schema", async () => {
    await withScratch(async (directory) => {
      const ids = ["walker-line", "radiator-rated", "radiator-transient", "walker-thermal"];
      const indexes = (await exportCast(pick(...ids), ids, directory)) as readonly CastIndex[];
      const catalog = await loadCatalogFromFiles(join(STAFFAGE_ROOT, "🗂️catalog"));
      for (const index of indexes) {
        expect(indexOf(JSON.parse(readFileSync(join(directory, index.cast, "🔣️.json"), "utf8"))), JSON.stringify(indexOf.errors?.slice(0, 3))).toBe(true);
        const cast = exporting.manifest.casts[index.cast]!;
        const file = catalog.loaded.find((entry) => entry.asset.id === cast.asset)!.path;
        const { provenance } = index;
        expect(provenance.asset).toEqual({ id: cast.asset, category: catalog.assets.find((asset) => asset.id === cast.asset)!.category, revision: revisionOfText(readFileSync(file, "utf8")) });
        expect(provenance.view).toEqual({ style: cast.view, viewpoint: cast.viewpoint, theme: cast.theme, mirror: cast.mirror === true });
        expect(provenance.scale).toEqual({ denominator: cast.scale });
        expect(provenance.state).toEqual(cast.clip === undefined ? { kind: "pose", id: cast.pose } : { kind: "clip", id: cast.clip });
        expect(provenance.clock).toEqual({ fps: index.fps, durationS: index.durationS, loop: index.loop, offsetS: 0, frames: index.frames.length });
      }
      const [line, still, transient, thermal] = indexes;
      expect(line!.provenance.scenario).toEqual({ kind: "geometry" });
      expect(still!.provenance.scenario).toEqual({ kind: "geometry" });
      expect(thermal!.provenance.scenario).toMatchObject({ kind: "thermal", environment: { id: "winter-overcast" }, spanK: null, transport: [] });
      const timeline = (transient!.provenance.scenario as { timeline: { t: number; id: string; ease: string }[] }).timeline;
      expect(timeline.map(({ t, id, ease }) => ({ t, id, ease }))).toEqual(exporting.manifest.casts["radiator-transient"]!.timeline!.map((key) => ({ t: key.t, id: key.environment, ease: key.ease })));
    });
  });

  it("changes the asset revision with one byte of the catalog file and ignores line endings", () => {
    const text = readFileSync(join(STAFFAGE_ROOT, "🗂️catalog/🪑️objects/chalkboard/🔣️.json"), "utf8");
    expect(revisionOfText(text)).toBe(revisionOfText(text.replace(/\r?\n/gu, "\r\n")));
    expect(revisionOfText(`${text} `)).not.toBe(revisionOfText(text));
  });
});
