/** 🏷️ TypeScript twin of the plugin runtime's history row label rule (`build_history_view` and `backfilled_edit_label` in
 * `🦀️.rs`), checked against the language-agnostic fixture `🧫️fixtures/🧫️history-label-reload/🔣️.json` that
 * `🧪️tests/🧪️history-label-reload/🦀️.rs` drives through the runtime's publish seam (`dispatch_emit`) and two reloads. Independent where it
 * counts: Ajv (2020-12) validates the fixture against its schema, and the rule is derived here from the fixture's facts
 * alone, never from a Rust projection. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020";

type Label = Readonly<{ en: string; de: string }>;
type Case = Readonly<{ id: string; command: Readonly<{ kind: string; value: string | number }>; verb: string; leaves: readonly Label[]; expected: Label }>;
type Fixture = Readonly<{ registry: readonly Readonly<{ verb: string; label: Label }>[]; cases: readonly Case[] }>;

const FIXTURE_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧫️history-label-reload";

/** 🏷️ A single leaf keeps its label; else the declared verb's label, else the first leaf (+N). An edit's description is never a
 * label (design §20.6). */
export function historyRowLabel(registry: Fixture["registry"], testCase: Case): Label {
  const [first, ...rest] = testCase.leaves;
  const verbLabel = registry.find((row) => row.verb === testCase.verb)?.label ?? null;
  if (rest.length === 0) return first!;
  if (verbLabel !== null) return verbLabel;
  const more = rest.length > 0 ? ` (+${rest.length})` : "";
  return { en: `${first!.en}${more}`, de: `${first!.de}${more}` };
}

/** ⚖️ Validates the fixture, rejects hostile rows and derives every case through the twin; answers the case count. */
export function historyLabelReloadOracle(repoRoot: string): number {
  const fixture: Fixture = JSON.parse(readFileSync(join(repoRoot, FIXTURE_ROOT, "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, FIXTURE_ROOT, "🧬️schema/🔣️.json"), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  const first = fixture.cases[0]!;
  assert.equal(validate({ ...fixture, cases: [{ ...first, verb: "Rename described" }] }), false, "a verb is a locale-neutral id, never display text");
  assert.equal(validate({ ...fixture, cases: [{ ...first, expected: { en: first.expected.en } }] }), false, "every label names English and German");
  for (const testCase of fixture.cases) assert.deepEqual(historyRowLabel(fixture.registry, testCase), testCase.expected, testCase.id);
  return fixture.cases.length;
}
