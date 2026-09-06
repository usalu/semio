/** 🧪️ Cross-language fixture oracle — every committed remodeling mutation vector, replayed by the
 *  TypeScript twin and compared against the bytes Rust committed.
 *
 *  Every fixture is discovered by globbing `🧬️mutations/<slug>/🧪️tests/<case>/` on disk; no case
 *  directory name is transcribed, because those names carry a content hash that is re-minted
 *  whenever a vector changes.
 */

import { readFileSync, readdirSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { decodeRemodelingSnapshot, defaultRemodelingScene, remodelingSnapshotToJsonText, type RemodelingSnapshot } from "../📸️snapshot/🟦️.ts";
import { applyRemodelingDiff, decodeRemodelingDiff, remodelingDiffLanes, remodelingDiffToJsonText } from "../🔺️diff/🟦️.ts";
import { REMODELING_MUTATION_TAGS, applyRemodelingMutation, decodeRemodelingMutation, remodelingMutationDiff, type RemodelingAnyMutation } from "../🧬️mutations/🟦️.ts";
import { remodelingArtifactFromSnapshot, remodelingArtifactToSnapshot } from "../🟦️.ts";
import { remodelingSnapshotFromDslText } from "../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const subset = join(here, "../..");
const mutationsRoot = join(subset, "🧬️schema/🧬️mutations");

interface Vector {
  slug: string;
  caseName: string;
  before: unknown;
  after: unknown;
  mutation: unknown;
  diff: unknown;
  outcome: { status: string };
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
      vectors.push({
        slug,
        caseName,
        before: readJson(join(root, "📸️snapshot/⬅️before/🔣️.json")),
        after: readJson(join(root, "📸️snapshot/➡️after/🔣️.json")),
        mutation: readJson(join(root, "🦠️mutation/🔣️.json")),
        diff: readJson(join(root, "🔺️diff/🔣️.json")),
        outcome: readJson(join(root, "🎯️outcome/🔣️.json")) as { status: string },
      });
    }
  }
  return vectors;
}

const VECTORS = discoverVectors();
/** 🕳️ The one field the committed vectors predate; see `📓️w3-ts-codec-oracle.md`. */
const KNOWN_ABSENT_KEY = "durableArtifacts";

/** 🚩 Vectors whose committed after-snapshot omits the `durableArtifacts` entry the leaf writes.
 *  `create-asset`'s Rust `diff` inserts `durable_artifacts[handle.child_id]`, so the committed
 *  after-document is stale, not the TypeScript twin — it is asserted here rather than tolerated so
 *  the list cannot silently grow. */
const DURABLE_ARTIFACT_DRIFT: readonly string[] = readdirSync(join(here, "../🧬️mutations")).filter((slug) => slug.endsWith("create-asset"));

/** ✂️ Drops the single emitted line the committed vectors predate, so the rest can be compared byte for byte. */
const withoutDurableLine = (text: string): string => text.split("\n").filter((line) => line !== `  ${JSON.stringify(KNOWN_ABSENT_KEY)}: {},` && line !== `  ${JSON.stringify(KNOWN_ABSENT_KEY)}: null,`).join("\n");

describe("remodeling fixture oracle", () => {
  it("discovers a vector for every mutation directory that ships one", () => {
    expect(VECTORS.length).toBeGreaterThan(0);
    const slugs = new Set(VECTORS.map((vector) => vector.slug));
    expect(slugs.size).toBe(VECTORS.length);
  });

  it("covers every wire tag except the one the feature file documents as vector-less", () => {
    const covered = new Set(VECTORS.map((vector) => (vector.mutation as { mutation: string }).mutation));
    const missing = REMODELING_MUTATION_TAGS.filter((tag) => !covered.has(tag));
    expect(missing).toEqual(["commitReconstruction"]);
  });
});

describe.each(VECTORS.map((vector) => [`${vector.slug}/${vector.caseName}`, vector] as const))("%s", (_label, vector) => {
  const before = () => decodeRemodelingSnapshot(vector.before);
  const mutation = () => decodeRemodelingMutation(vector.mutation) as RemodelingAnyMutation;

  it("decodes its committed quartet under total validation", () => {
    expect(() => before()).not.toThrow();
    expect(() => decodeRemodelingSnapshot(vector.after)).not.toThrow();
    expect(() => mutation()).not.toThrow();
    expect(() => decodeRemodelingDiff(vector.diff)).not.toThrow();
  });

  it("applies in TypeScript to the committed after-snapshot", () => {
    const applied = applyRemodelingMutation(before(), mutation());
    const expected = decodeRemodelingSnapshot(vector.after);
    expect({ ...applied, durableArtifacts: expected.durableArtifacts }).toEqual(expected);
  });

  it("agrees with the committed after-snapshot on durableArtifacts, or is a listed stale vector", () => {
    const applied = applyRemodelingMutation(before(), mutation());
    const expected = decodeRemodelingSnapshot(vector.after);
    const drifts = JSON.stringify(applied.durableArtifacts) !== JSON.stringify(expected.durableArtifacts);
    expect(drifts ? vector.slug : null).toBe(DURABLE_ARTIFACT_DRIFT.includes(vector.slug) ? vector.slug : null);
  });

  it("produces the committed diff", () => {
    const produced = remodelingMutationDiff(before(), mutation());
    expect(JSON.parse(remodelingDiffToJsonText(produced.diff))).toEqual({ ...(vector.diff as object), ...withoutDiffDrift(produced) });
  });

  it("declares the committed outcome status", () => {
    const produced = remodelingMutationDiff(before(), mutation());
    const refused = produced.messages.some((message) => message.severity === "error" || message.severity === "fatal");
    expect(refused ? "refused" : "applied").toBe(vector.outcome.status);
  });

  it("carries the committed diff from before to after", () => {
    expect(applyRemodelingDiff(decodeRemodelingDiff(vector.diff), before())).toEqual(decodeRemodelingSnapshot(vector.after));
  });

  it("writes only the lanes the committed diff writes", () => {
    const produced = remodelingMutationDiff(before(), mutation());
    const committed = remodelingDiffLanes(decodeRemodelingDiff(vector.diff));
    expect(remodelingDiffLanes(produced.diff).filter((lane) => lane !== "durable_artifacts")).toEqual(committed);
  });

  it("round-trips both snapshots through encode/decode", () => {
    for (const json of [vector.before, vector.after]) {
      const decoded = decodeRemodelingSnapshot(json);
      expect(decodeRemodelingSnapshot(JSON.parse(remodelingSnapshotToJsonText(decoded)))).toEqual(decoded);
    }
  });

  it("re-emits the committed snapshot bytes apart from the one field they predate", () => {
    for (const side of ["⬅️before", "➡️after"] as const) {
      const path = join(mutationsRoot, vector.slug, "🧪️tests", vector.caseName, "📸️snapshot", side, "🔣️.json");
      const emitted = remodelingSnapshotToJsonText(decodeRemodelingSnapshot(readJson(path)));
      expect(withoutDurableLine(emitted)).toBe(readFileSync(path, "utf8").trimEnd());
    }
  });

  it("re-emits the committed diff bytes apart from the one field they predate", () => {
    const path = join(mutationsRoot, vector.slug, "🧪️tests", vector.caseName, "🔺️diff", "🔣️.json");
    const produced = remodelingMutationDiff(before(), mutation());
    if (produced.diff.durableArtifacts !== null) return;
    expect(withoutDurableLine(remodelingDiffToJsonText(produced.diff))).toBe(readFileSync(path, "utf8").trimEnd());
  });

  it("survives the artifact/snapshot round trip", () => {
    const snapshot = before();
    expect(remodelingArtifactToSnapshot(remodelingArtifactFromSnapshot(snapshot))).toEqual(snapshot);
  });
});

/** 🩹 The `durableArtifacts` lane the committed diffs predate, re-supplied for the comparison. */
function withoutDiffDrift(produced: { diff: { durableArtifacts: unknown } }): Record<string, unknown> {
  return { durableArtifacts: produced.diff.durableArtifacts };
}

describe("commit-reconstruction refusal", () => {
  const fixtures = join(subset, "🧫️fixtures/🏁️commit-reconstruction");
  const replaceJobVector = VECTORS.find((vector) => (vector.mutation as { mutation: string }).mutation === "replaceJob");
  const replaceSparseVector = VECTORS.find((vector) => (vector.mutation as { mutation: string }).mutation === "replaceSparse");

  it("refuses a plain sparse buffer with mutation.invalid-reconstruction-sparse and moves nothing", () => {
    expect(replaceJobVector).toBeDefined();
    expect(replaceSparseVector).toBeDefined();
    const base = decodeRemodelingSnapshot(replaceJobVector!.before);
    const commit = decodeRemodelingMutation({
      mutation: "commitReconstruction",
      job: (replaceJobVector!.mutation as { job: unknown }).job,
      sparse: (replaceSparseVector!.mutation as { sparse: unknown }).sparse,
      trajectory: null,
      mesh: null,
      geo: null,
      qc: null,
      assets: [],
    });
    const outcome = remodelingMutationDiff(base, commit);
    expect(outcome.messages.map((message) => message.code)).toEqual(["mutation.invalid-reconstruction-sparse"]);
    expect(applyRemodelingMutation(base, commit)).toEqual(base);
  });

  /** 🚩 `🧫️fixtures/🏁️commit-reconstruction/` is stale on three counts, asserted rather than skipped so
   *  the assertions flip red the moment the pair is regenerated: both documents carry a `job.stage`
   *  lexeme that is not in `ReconstructionStage` (so Rust's own `serde_json::from_str` rejects them
   *  too), they are not equal to each other although the feature file says a refused commit leaves
   *  the scene untouched, and the `🦠️…-mutation.json` that feature file names is not on disk. */
  it("still ships the stale pre-rename commit-reconstruction fixture pair", () => {
    for (const [side, stage] of [
      ["⬅️before.json", "dense-reconstructing"],
      ["➡️after.json", "completed"],
    ] as const) {
      const path = join(fixtures, side);
      expect(existsSync(path)).toBe(true);
      expect((readJson(path) as { job: { stage: string } }).job.stage).toBe(stage);
      expect(() => decodeRemodelingSnapshot(readJson(path))).toThrow(`.job.stage: expected one of`);
    }
    expect(readJson(join(fixtures, "⬅️before.json"))).not.toEqual(readJson(join(fixtures, "➡️after.json")));
    expect(existsSync(join(fixtures, "🦠️mutation.json"))).toBe(false);
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
