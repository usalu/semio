/** 🧩️ TypeScript twin of the plugin runtime's composed-child history backfill (`member_backfill` in
 * `⏪️time-travel/🦀️.rs`, design §12), checked against the language-agnostic fixture
 * `🧫️fixtures/🧫️composed-child-history/🔣️.json` that `🧪️tests/🧪️composed-child-history/🦀️.rs` runs through the Rust law.
 * Independent where it counts: Ajv (2020-12) validates the fixture against its schema, and every case is derived here from
 * the fixture's facts alone, never from a Rust projection. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020";

type Label = Readonly<{ en: string; de: string }>;
type History = Readonly<{ store: string; editId: string; transaction: string | null; at: number; startedAt: string; opCount: number; opLines: readonly string[]; firstLabel?: Label }>;
type Group = Readonly<{ at: number; transaction: string | null; editIds: readonly string[]; label: Label; startedAt: string }>;
type Backfill = Readonly<{ attached: Readonly<Record<string, readonly string[]>>; groups: readonly Group[] }>;
type Case = Readonly<{ id: string; histories: readonly History[]; logged: readonly string[]; parentTransactions: Readonly<Record<string, string>>; expected: Backfill }>;
type Fixture = Readonly<{ cases: readonly Case[] }>;

const FIXTURE_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧫️composed-child-history";

/** 🏷️ A row's label: its first leaf with `(+N)` for the operations after it, else its first printed operation, else its edit id. */
function rowLabel(history: History): Label {
  if (history.firstLabel !== undefined) {
    const more = history.opCount > 1 ? ` (+${history.opCount - 1})` : "";
    return { en: `${history.firstLabel.en}${more}`, de: `${history.firstLabel.de}${more}` };
  }
  const line = history.opLines[0] ?? history.editId;
  return { en: line, de: line };
}

/** 🧾️ The backfill of `histories`: unlogged edits in moment order (ties by edit id); an edit whose transaction also landed a parent edit joins that parent's row, an edit of a transaction already grouped joins that group, every other edit opens a row of its own. */
export function memberBackfill(histories: readonly History[], logged: readonly string[], parentTransactions: Readonly<Record<string, string>>): Backfill {
  const unlogged = histories.filter((history) => !logged.includes(history.editId)).sort((a, b) => a.at - b.at || (a.editId < b.editId ? -1 : a.editId > b.editId ? 1 : 0));
  const attached: Record<string, string[]> = {};
  const groups: { at: number; transaction: string | null; editIds: string[]; label: Label; startedAt: string }[] = [];
  for (const history of unlogged) {
    const parent = history.transaction === null ? undefined : parentTransactions[history.transaction];
    if (parent !== undefined) {
      (attached[parent] ??= []).push(history.editId);
      continue;
    }
    const group = history.transaction === null ? undefined : groups.find((candidate) => candidate.transaction === history.transaction);
    if (group !== undefined) {
      group.editIds.push(history.editId);
      continue;
    }
    groups.push({ at: history.at, transaction: history.transaction, editIds: [history.editId], label: rowLabel(history), startedAt: history.startedAt });
  }
  return { attached, groups };
}

/** ⚖️ Validates the fixture, rejects hostile rows and derives every case through the twin; answers the case count. */
export function composedChildHistoryOracle(repoRoot: string): number {
  const fixture: Fixture = JSON.parse(readFileSync(join(repoRoot, FIXTURE_ROOT, "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, FIXTURE_ROOT, "🧬️schema/🔣️.json"), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  const first = fixture.cases[0]!;
  assert.equal(validate({ ...fixture, cases: [{ ...first, histories: [{ ...first.histories[0]!, store: "content" }] }] }), false, "a member store is `<slot>/<childId>`");
  assert.equal(validate({ ...fixture, cases: [{ ...first, histories: [{ ...first.histories[0]!, transaction: "nodeGraphEdit" }] }] }), false, "a transaction is its minted `tx-<hex16>` id");
  for (const testCase of fixture.cases) assert.deepEqual(memberBackfill(testCase.histories, testCase.logged, testCase.parentTransactions), testCase.expected, testCase.id);
  return fixture.cases.length;
}
