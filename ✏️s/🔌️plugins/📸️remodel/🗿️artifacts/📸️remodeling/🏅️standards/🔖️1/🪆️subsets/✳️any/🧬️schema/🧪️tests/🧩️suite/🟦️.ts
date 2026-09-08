/** 🧪️ Cross-language fixture oracle — every committed remodeling mutation vector, replayed by the
 *  TypeScript twin and compared against the bytes Rust committed.
 *
 *  Every fixture is discovered by globbing `🧬️mutations/<slug>/🧪️tests/<case>/` on disk; no case
 *  directory name is transcribed, because those names carry a content hash that is re-minted
 *  whenever a vector changes.
 *
 *  A vector is one of three shapes, told apart by what it commits rather than by its name:
 *  an APPLIED vector ships a `🔺️diff/🔣️.json` and an `applied` outcome; a REFUSED vector ships
 *  `🔺️diff/🚫️.absent` (the repository-wide marker for a file that is deliberately not there) and a
 *  `rejected` outcome naming the code and target its guard raises; a WARNED no-op ships an all-null
 *  diff, an `applied` outcome and a `mutation.no-op` message, and its after-document is its
 *  before-document. All three are asserted here.
 */

import { readFileSync, readdirSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { decodeRemodelingSnapshot, defaultRemodelingScene, remodelingSnapshotToJsonText, type RemodelingSnapshot } from "../../📸️snapshot/🟦️.ts";
import { applyRemodelingDiff, decodeRemodelingDiff, remodelingDiffLanes, remodelingDiffToJsonText } from "../../🔺️diff/🟦️.ts";
import { REMODELING_MUTATION_TAGS, applyRemodelingMutation, decodeRemodelingMutation, remodelingMutationDiff, type RemodelingAnyMutation } from "../../🧬️mutations/🟦️.ts";
import { remodelingArtifactFromSnapshot, remodelingArtifactToSnapshot } from "../../🟦️.ts";
import { remodelingSnapshotFromDslText } from "../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const subset = join(here, "../../..");
const mutationsRoot = join(subset, "🧬️schema/🧬️mutations");

interface Outcome {
  status: "applied" | "rejected";
  code?: string;
  path?: string[];
  messages?: { level: string; code: string }[];
}

interface Vector {
  slug: string;
  caseName: string;
  before: unknown;
  after: unknown;
  mutation: unknown;
  diff: unknown | null;
  outcome: Outcome;
}

const readJson = (path: string): unknown => JSON.parse(readFileSync(path, "utf8"));

/** 🔎 Globs the committed vectors as they are on disk right now. */
function discoverVectors(): Vector[] {
  const vectors: Vector[] = [];
  for (const slug of readdirSync(mutationsRoot).sort()) {
    const testsDir = join(mutationsRoot, slug, "🧪️tests");
    if (!existsSync(testsDir)) continue;
    for (const caseName of readdirSync(testsDir).sort()) {
      const root = join(testsDir, caseName);
      if (!existsSync(join(root, "🦠️mutation/🔣️.json"))) continue;
      const diffPath = join(root, "🔺️diff/🔣️.json");
      vectors.push({
        slug,
        caseName,
        before: readJson(join(root, "📸️snapshot/⬅️before/🔣️.json")),
        after: readJson(join(root, "📸️snapshot/➡️after/🔣️.json")),
        mutation: readJson(join(root, "🦠️mutation/🔣️.json")),
        diff: existsSync(diffPath) ? readJson(diffPath) : null,
        outcome: readJson(join(root, "🎯️outcome/🔣️.json")) as Outcome,
      });
    }
  }
  return vectors;
}

const VECTORS = discoverVectors();

describe("remodeling fixture oracle", () => {
  it("discovers at least one vector for every mutation directory that ships tests", () => {
    const slugs = new Set(VECTORS.map((vector) => vector.slug));
    expect(slugs.size).toBe(readdirSync(mutationsRoot).filter((slug) => existsSync(join(mutationsRoot, slug, "🧪️tests"))).length);
    const identities = VECTORS.map((vector) => `${vector.slug}/${vector.caseName}`);
    expect(new Set(identities).size).toBe(identities.length);
  });

  it("covers every wire tag, commitReconstruction now included", () => {
    const covered = new Set(VECTORS.map((vector) => (vector.mutation as { mutation: string }).mutation));
    expect(REMODELING_MUTATION_TAGS.filter((tag) => !covered.has(tag))).toEqual([]);
  });

  it("carries all four vector roles for the vocabulary as a whole", () => {
    const refused = VECTORS.filter((vector) => vector.outcome.status === "rejected");
    const warned = VECTORS.filter((vector) => vector.diff !== null && remodelingDiffLanes(decodeRemodelingDiff(vector.diff)).length === 0);
    expect(refused.length).toBeGreaterThan(0);
    expect(warned.length).toBeGreaterThan(0);
    expect(new Set(refused.map((vector) => vector.slug)).size).toBeGreaterThan(20);
  });
});

describe.each(VECTORS.map((vector) => [`${vector.slug}/${vector.caseName}`, vector] as const))("%s", (_label, vector) => {
  const before = () => decodeRemodelingSnapshot(vector.before);
  const mutation = () => decodeRemodelingMutation(vector.mutation) as RemodelingAnyMutation;
  const refused = vector.outcome.status === "rejected";

  it("decodes its committed quartet under total validation", () => {
    expect(() => before()).not.toThrow();
    expect(() => decodeRemodelingSnapshot(vector.after)).not.toThrow();
    expect(() => mutation()).not.toThrow();
    if (vector.diff !== null) expect(() => decodeRemodelingDiff(vector.diff)).not.toThrow();
  });

  it("applies in TypeScript to the committed after-snapshot, durableArtifacts included", () => {
    expect(applyRemodelingMutation(before(), mutation())).toEqual(decodeRemodelingSnapshot(vector.after));
  });

  it("produces the committed diff, or commits no diff at all when it refuses", () => {
    const produced = remodelingMutationDiff(before(), mutation());
    if (refused) {
      expect(vector.diff).toBeNull();
      expect(remodelingDiffLanes(produced.diff)).toEqual([]);
      return;
    }
    expect(JSON.parse(remodelingDiffToJsonText(produced.diff))).toEqual(vector.diff);
  });

  it("declares the committed outcome status and every diagnostic it names", () => {
    const produced = remodelingMutationDiff(before(), mutation());
    const isRefusal = produced.messages.some((message) => message.severity === "error" || message.severity === "fatal");
    expect(isRefusal ? "rejected" : "applied").toBe(vector.outcome.status);
    if (isRefusal) {
      expect(produced.messages.map((message) => message.code)).toEqual([vector.outcome.code]);
      expect(produced.messages[0].target).toEqual(vector.outcome.path ?? []);
      return;
    }
    expect(produced.messages.map((message) => message.code)).toEqual((vector.outcome.messages ?? []).map((message) => message.code));
  });

  it("carries the committed diff from before to after", () => {
    if (refused) {
      expect(vector.after).toEqual(vector.before);
      return;
    }
    expect(applyRemodelingDiff(decodeRemodelingDiff(vector.diff), before())).toEqual(decodeRemodelingSnapshot(vector.after));
  });

  it("writes only the lanes the committed diff writes", () => {
    const produced = remodelingMutationDiff(before(), mutation());
    const committed = vector.diff === null ? [] : remodelingDiffLanes(decodeRemodelingDiff(vector.diff));
    expect(remodelingDiffLanes(produced.diff)).toEqual(committed);
  });

  it("round-trips both snapshots through encode/decode", () => {
    for (const json of [vector.before, vector.after]) {
      const decoded = decodeRemodelingSnapshot(json);
      expect(decodeRemodelingSnapshot(JSON.parse(remodelingSnapshotToJsonText(decoded)))).toEqual(decoded);
    }
  });

  it("re-emits the committed snapshot bytes exactly", () => {
    for (const side of ["⬅️before", "➡️after"] as const) {
      const path = join(mutationsRoot, vector.slug, "🧪️tests", vector.caseName, "📸️snapshot", side, "🔣️.json");
      expect(remodelingSnapshotToJsonText(decodeRemodelingSnapshot(readJson(path)))).toBe(readFileSync(path, "utf8").trimEnd());
    }
  });

  it("re-emits the committed diff bytes exactly", () => {
    if (refused) return;
    const path = join(mutationsRoot, vector.slug, "🧪️tests", vector.caseName, "🔺️diff", "🔣️.json");
    expect(remodelingDiffToJsonText(remodelingMutationDiff(before(), mutation()).diff)).toBe(readFileSync(path, "utf8").trimEnd());
  });

  it("survives the artifact/snapshot round trip", () => {
    const snapshot = before();
    expect(remodelingArtifactToSnapshot(remodelingArtifactFromSnapshot(snapshot))).toEqual(snapshot);
  });
});

describe("commit-reconstruction shared vector", () => {
  const fixtures = join(subset, "🧫️fixtures/🏁️commit-reconstruction");

  /** 🏁️ The one kind whose diff reads process-global staging state a `(before, mutation, after)`
   *  triple cannot carry, committed as the refusal its own guard raises. All three documents are
   *  schema-valid, the pair really is unchanged across the refusal, and the payload the feature file
   *  names is on disk — the three counts on which this fixture used to be stale. */
  it("ships a schema-valid, self-consistent refusal triple", () => {
    for (const name of ["⬅️before.json", "🦠️mutation.json", "➡️after.json"]) expect(existsSync(join(fixtures, name))).toBe(true);
    const base = decodeRemodelingSnapshot(readJson(join(fixtures, "⬅️before.json")));
    expect(readJson(join(fixtures, "➡️after.json"))).toEqual(readJson(join(fixtures, "⬅️before.json")));
    expect(base.job.stage).toBe("bundle-adjusting");
    expect(base.results.mesh).not.toBeNull();
  });

  it("refuses a plain sparse buffer with mutation.invalid-reconstruction-sparse and moves nothing", () => {
    const base = decodeRemodelingSnapshot(readJson(join(fixtures, "⬅️before.json")));
    const commit = decodeRemodelingMutation(readJson(join(fixtures, "🦠️mutation.json")));
    const outcome = remodelingMutationDiff(base, commit);
    expect(outcome.messages.map((message) => message.code)).toEqual(["mutation.invalid-reconstruction-sparse"]);
    expect(applyRemodelingMutation(base, commit)).toEqual(base);
  });
});

describe("committed example assets", () => {
  const dslAssets = [join(subset, "📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio")];

  it.each(dslAssets)("parses %s as a remodeling document", (path) => {
    const snapshot: RemodelingSnapshot = remodelingSnapshotFromDslText(readFileSync(path, "utf8"));
    expect(snapshot.schema).toBe("remodeling.scene");
    expect(snapshot.id).toBe("remodeling");
    expect(snapshot.streams).toEqual([]);
    expect(snapshot.assets).toEqual({});
    expect(snapshot.results.mesh.source).toBe("placeholder");
    expect(snapshot.results.mesh.mesh.target.dialect).toEqual({ artifactKind: "s.stdio.semio", standard: "v1", subset: "mesh" });
  });

  it("parses the demo document to the plugin's own default scene apart from its mesh child id", () => {
    const parsed = remodelingSnapshotFromDslText(readFileSync(dslAssets[0], "utf8"));
    const scene = defaultRemodelingScene();
    expect({ ...parsed, results: { ...parsed.results, mesh: { ...parsed.results.mesh, mesh: null } } }).toEqual({ ...scene, results: { ...scene.results, mesh: { ...scene.results.mesh, mesh: null } } });
  });

  it("ships a non-empty editor command asset", () => {
    expect(readFileSync(join(subset, "✏️editor/📚️examples/🎬️demo-session/🖼️assets/🎮️.cmd.semio"), "utf8").length).toBeGreaterThan(8);
  });
});
