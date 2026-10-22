/** 📋️ The stdio owner-table helpers (facet vocabulary, schema children, trait-impl detection, spec-leaf names) return exactly what their language-agnostic corpus says and, where an independent Python model exists, agree with it; the shared constants are pinned. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { testPythonV1 } from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import { gitSpawnEnv } from "../../../../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import * as owner from "../../🟦️.ts";
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };

const rules: Record<string, (root: string, ...args: any[]) => unknown> = {
  policyStdioArtifactFacets: (_root, taxonomy) => owner.policyStdioArtifactFacets(taxonomy),
  policyStdioSchemaChildDirs: (_root, taxonomy) => owner.policyStdioSchemaChildDirs(taxonomy),
  policyStdioRepresentationDirs: (_root, taxonomy) => owner.policyStdioRepresentationDirs(taxonomy),
  policyStdioFormatDir: (_root, artifacts, id) => owner.policyStdioFormatDir(artifacts, id),
  policyStdioArtifactsDirName: () => owner.policyStdioArtifactsDirName(),
  policyRustFileHasRealTraitImpl: (_root, body, trait) => owner.policyRustFileHasRealTraitImpl(body, trait),
  specLeaves: () => ({ text: [...owner.POLICY_STDIO_TEXT_SPEC_LEAVES], binary: [...owner.POLICY_STDIO_BINARY_SPEC_LEAVES] }),
};
const artifacts = realpathSync(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir()));
const libraryRoot = resolve(import.meta.dir, "../../../../../.."), libraryRel = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const canon = (value: unknown): unknown => (value === undefined ? null : JSON.parse(JSON.stringify(value)));
const admit = (root: string) => {
  for (const rel of ["🔣️taxonomy.json", "🧬️schema/🔣️.json"]) { mkdirSync(dirname(join(root, libraryRel, rel)), { recursive: true }); copyFileSync(join(libraryRoot, rel), join(root, libraryRel, rel)); }
  for (const args of [["init", "--quiet"], ["add", "-A"]]) expect(spawnSync("git", args, { cwd: root, env: gitSpawnEnv(), encoding: "utf8" }).status).toBe(0);
};
const run = (row: any) => {
  const root = mkdtempSync(join(artifacts, "lawcov-"));
  try {
    for (const [path, lines] of Object.entries((row.files ?? {}) as Record<string, string[]>)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), lines.length ? lines.join("\n") + "\n" : ""); }
    if (row.git) admit(root);
    return canon(rules[row.rule]!(root, ...(row.args ?? [])));
  } finally { rmSync(root, { recursive: true, force: true }); }
};
const modelled = (corpus.cases as any[]).filter((row) => row.oracle !== false);
const model = (() => {
  const run = spawnSync(testPythonV1() ?? "python3", ["-I", resolve(import.meta.dir, "./🐍️oracle.py"), resolve(import.meta.dir, "./🔣️.json")], { encoding: "utf8" });
  expect(run.status, run.stderr).toBe(0);
  return JSON.parse(run.stdout) as Record<string, unknown>;
})();

test("the corpus satisfies the strict independent validator and names every case once", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(new Set(corpus.cases.map((row) => row.id)).size).toBe(corpus.cases.length);
  expect(Object.keys(model).sort()).toEqual(modelled.map((row) => row.id).sort());
});

for (const row of corpus.cases as any[]) {
  (row.defect ? test.failing : test)(`${row.rule}: ${row.id}`, () => {
    expect(run(row), "law versus corpus").toEqual(row.expect);
    if (row.oracle !== false) expect(model[row.id], "oracle versus corpus").toEqual(row.expect);
  }, row.git ? 120000 : 30000);
}

test("the owner table names the stdio plugin, its artifacts and the shared facet vocabulary", () => {
  expect([owner.POLICY_STDIO_OWNER_TABLE_REL, owner.POLICY_STDIO_PLUGIN_REL, owner.POLICY_STDIO_ARTIFACTS_REL]).toEqual(["✏️s/🔌️plugins/🗄️stdio/📇️registry/🔣️.json", "✏️s/🔌️plugins/🗄️stdio", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"]);
  expect([owner.POLICY_STDIO_FACET_DECOMPOSER, owner.POLICY_STDIO_FACET_TEXT, owner.POLICY_STDIO_FACET_BINARY, owner.POLICY_STDIO_FACET_DESERIALIZERS, owner.POLICY_STDIO_FACET_SERIALIZERS, owner.POLICY_STDIO_IO_IMPORT, owner.POLICY_STDIO_IO_EXPORT]).toEqual(["🪓️decomposer", "📝️text", "💾️binary", "🧩️deserializers", "🧵️serializers", "📥️import", "📤️export"]);
  expect([...owner.POLICY_STDIO_CODEC_BANNED_MARKERS]).toEqual(["SRAS", "IFCCARTOONMESH", 'b"minimal"', "stub codec", "minimal stub codec"]);
});
