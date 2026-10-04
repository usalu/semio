import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { runRepositoryExactCargoLaws } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** 🎨️ Proves the closed mutation verb ontology with Ajv, Rust source and native membership laws. */
export async function checkMutationVerbVocabulary(repoRoot: string, segments: string[]): Promise<void> {
  const root = resolve(import.meta.dir, "../.."), schema = JSON.parse(readFileSync(resolve(root, "🧬️schema/🔣️.json"), "utf8"));
  const ontology = schema.$defs.MutationVerbPairV1.oneOf.map((row: { const: { verb: string; record: string } }) => row.const);
  const fixture = JSON.parse(readFileSync(resolve(root, "🧫️fixtures/🗣️verb-vocabulary/🔣️.json"), "utf8"));
  const rust = readFileSync(resolve(root, "🦀️.rs"), "utf8").split("pub const APPROVED_VERBS:")[1]!.split("];", 1)[0]!;
  const nativePairs = [...rust.matchAll(/\("([a-z]+)", "([A-Za-z]+)"\)/gu)].map((row) => ({ verb: row[1], record: row[2] }));
  assert.deepEqual(nativePairs, ontology);
  const Ajv = (await import("ajv")).default, ajv = new Ajv({ strict: false });
  const validate = ajv.compile({ ...schema, $ref: "#/$defs/MutationVerbPairV1" });
  for (const row of fixture.cases) {
    const expected = ontology.find((pair: { verb: string }) => pair.verb === row.verb)?.record ?? null;
    assert.equal(expected, row.record);
    assert.equal(validate(row), row.record !== null);
  }
  assert(!validate({ verb: "paint", record: "Changed" }));
  assert(!validate({ verb: "paint", record: "Painted", authority: true }));
  console.log(`mutation-verb-vocabulary: pairs=${ontology.length} vectors=${fixture.cases.length} Ajv/source parity=green`);
  if (segments.includes("--oracle-only")) return;
  const laws = await runRepositoryExactCargoLaws({ cwd: repoRoot, progress: (event) => console.log(`mutation-verb-native ${event.stage} ${event.law ?? event.package}`), groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib", name: "semio_framework_os_kernel" }, laws: ["os_spr::command::tests::space_history_verbs_match_the_language_neutral_contract", "os_spr::command::tests::approved_verbs_match_closed_schema"] }] });
  console.log(`mutation-verb-native: exact=${laws.length} passed`);
}
