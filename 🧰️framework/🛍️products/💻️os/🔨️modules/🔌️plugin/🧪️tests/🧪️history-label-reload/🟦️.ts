/** 🏷️ TypeScript twin of the plugin runtime's history row label rule (`build_history_view` and `backfilled_edit_label` in
 * `🦀️.rs`), checked against the language-agnostic fixture `🧫️fixtures/🧫️history-label-reload/🔣️.json` that
 * `🧪️tests/🧪️history-label-reload/🦀️.rs` drives through the runtime's publish seam (`dispatch_emit`) and two reloads. Independent where it
 * counts: the rule is derived here from the fixture's facts
 * alone, never from a Rust projection. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";

type Label = Readonly<{ en: string; de: string }>;
type Case = Readonly<{ id: string; command: Readonly<{ kind: string; value: string | number }>; verb: string; leaves: readonly Label[]; expected: Label }>;
type Fixture = Readonly<{ registry: readonly Readonly<{ verb: string; label: Label }>[]; cases: readonly Case[] }>;

const FIXTURE_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧫️history-label-reload";

/** 🌐️ A history label supplies both explicit locales. */
function assertLabel(label: unknown): asserts label is Label {
  assert.ok(label !== null && typeof label === "object", "a history label is an object");
  const value = label as Partial<Label>;
  for (const locale of ["en", "de"] as const) assert.ok(typeof value[locale] === "string" && value[locale].length > 0, `a history label names ${locale}`);
}

/** 🏷️ A single leaf keeps its label; else the declared verb's label, else the first leaf (+N). An edit's description is never a
 * label (design §20.6). */
export function historyRowLabel(registry: Fixture["registry"], testCase: Case): Label {
  assert.match(testCase.verb, /^[A-Za-z][A-Za-z0-9_-]*$/u, "a verb is a locale-neutral id");
  assert.ok(testCase.leaves.length > 0, "a history edit names a leaf label");
  testCase.leaves.forEach(assertLabel);
  const [first, ...rest] = testCase.leaves;
  const verbLabel = registry.find((row) => row.verb === testCase.verb)?.label ?? null;
  if (rest.length === 0) return first!;
  if (verbLabel !== null) return verbLabel;
  const more = rest.length > 0 ? ` (+${rest.length})` : "";
  return { en: `${first!.en}${more}`, de: `${first!.de}${more}` };
}

/** ⚖️ Rejects hostile domain records and derives every case through the twin; answers the case count. */
export function historyLabelReloadOracle(repoRoot: string): number {
  const fixture: Fixture = JSON.parse(readFileSync(join(repoRoot, FIXTURE_ROOT, "🔣️.json"), "utf8"));
  const first = fixture.cases[0]!;
  assert.throws(() => historyRowLabel(fixture.registry, { ...first, verb: "Rename described" }), /locale-neutral/);
  assert.throws(() => assertLabel({ en: first.expected.en }), /names de/);
  fixture.registry.forEach((row) => { assert.match(row.verb, /^[A-Za-z][A-Za-z0-9_-]*$/u); assertLabel(row.label); });
  for (const testCase of fixture.cases) { assertLabel(testCase.expected); assert.deepEqual(historyRowLabel(fixture.registry, testCase), testCase.expected, testCase.id); }
  return fixture.cases.length;
}
