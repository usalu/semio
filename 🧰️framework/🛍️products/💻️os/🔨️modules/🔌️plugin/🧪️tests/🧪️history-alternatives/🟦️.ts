/** 🌿️ TypeScript twin of the plugin runtime's alternatives list (`history_alternative_views` in `🦀️.rs`), checked against
 * the language-agnostic fixture `🧫️fixtures/🧫️history-alternatives/🔣️.json` that `🧪️tests/🧪️history-alternatives/🦀️.rs`
 * runs through the Rust projection. Independent where it counts: Ajv (2020-12) validates the fixture against its schema,
 * and the branch time is formatted by JavaScript's own `Date`, not a port of the Rust calendar arithmetic. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020";

type Fact = Readonly<{ kind: "branch"; alternative: string; actor: string; at: number }> | Readonly<{ kind: "supersede"; scope: string | null }>;
type Row = Readonly<{ id: string; name: string; current: boolean; author: string | null; branchedAt: string | null; edited: boolean }>;
type Case = Readonly<{ id: string; alternatives: readonly Readonly<{ id: string; name: string }>[]; active: string | null; facts: readonly Fact[]; expected: readonly Row[] }>;
type Fixture = Readonly<{ cases: readonly Case[] }>;

const FIXTURE_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧫️history-alternatives";

/** 📆️ RFC 3339 UTC with milliseconds only when nonzero, through `Date`. */
function rfc3339(at: number): string {
  return new Date(at).toISOString().replace(/\.000Z$/u, "Z");
}

/** 🌿️ The alternatives list: declaration order, active marker, first branch as author and time, edited by a scoped supersession. */
export function historyAlternativeRows(testCase: Case): readonly Row[] {
  return testCase.alternatives.map(({ id, name }) => {
    const branch = testCase.facts.find((fact) => fact.kind === "branch" && fact.alternative === id);
    return {
      id,
      name,
      current: testCase.active === id,
      author: branch?.kind === "branch" ? branch.actor : null,
      branchedAt: branch?.kind === "branch" ? rfc3339(branch.at) : null,
      edited: testCase.facts.some((fact) => fact.kind === "supersede" && fact.scope === id),
    };
  });
}

/** ⚖️ Validates the fixture, rejects hostile rows and derives every case through the twin; answers the case count. */
export function historyAlternativesOracle(repoRoot: string): number {
  const fixture: Fixture = JSON.parse(readFileSync(join(repoRoot, FIXTURE_ROOT, "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, FIXTURE_ROOT, "🧬️schema/🔣️.json"), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  const first = fixture.cases[0]!;
  assert.equal(validate({ ...fixture, cases: [{ ...first, facts: [{ kind: "branch", alternative: "alt", actor: "ada" }] }] }), false, "a branch carries its time");
  assert.equal(validate({ ...fixture, cases: [{ ...first, expected: [{ ...first.expected[0], branchedAt: "2025-09-30 12:00" }] }] }), false, "a branch time is RFC 3339 UTC");
  for (const testCase of fixture.cases) assert.deepEqual(historyAlternativeRows(testCase), testCase.expected, testCase.id);
  return fixture.cases.length;
}
