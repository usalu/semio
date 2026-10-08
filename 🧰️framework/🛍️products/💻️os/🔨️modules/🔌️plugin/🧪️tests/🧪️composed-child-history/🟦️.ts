/** 🧩️ TypeScript twin of the plugin runtime's composed-child history backfill (`member_backfill` in
 * `⏪️time-travel/🦀️.rs`, design §12), checked against the language-agnostic fixture
 * `🧫️fixtures/🧫️composed-child-history/🔣️.json` that `🧪️tests/🧪️composed-child-history/🦀️.rs` runs through the Rust law.
 * Independent where it counts: every case is derived here from the fixture's facts alone, never from a Rust projection. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";

type Label = Readonly<{ en: string; de: string }>;
type History = Readonly<{ store: string; editId: string; transaction: string | null; at: number; startedAt: string; opCount: number; opLines: readonly string[]; firstLabel?: Label; intentLabel?: Label }>;
type Group = Readonly<{ at: number; transaction: string | null; editIds: readonly string[]; label: Label; startedAt: string }>;
type Backfill = Readonly<{ attached: Readonly<Record<string, readonly string[]>>; groups: readonly Group[] }>;
type Case = Readonly<{ id: string; histories: readonly History[]; logged: readonly string[]; parentTransactions: Readonly<Record<string, string>>; expected: Backfill }>;
type Fixture = Readonly<{ cases: readonly Case[] }>;

const FIXTURE_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧫️composed-child-history";

/** 🏷️ Uses declared intent or the first leaf with the remaining operation count, then printed operation or edit id. */
function rowLabel(history: History): Label {
  const label = history.intentLabel ?? history.firstLabel;
  if (label !== undefined) {
    const more = history.opCount > 1 ? ` (+${history.opCount - 1})` : "";
    return { en: `${label.en}${more}`, de: `${label.de}${more}` };
  }
  const line = history.opLines[0] ?? history.editId;
  return { en: line, de: line };
}

/** 🧾️ The backfill of `histories`: unlogged edits in moment order (ties by edit id); an edit whose transaction also landed a parent edit joins that parent's row, an edit of a transaction already grouped joins that group, every other edit opens a row of its own. */
export function memberBackfill(histories: readonly History[], logged: readonly string[], parentTransactions: Readonly<Record<string, string>>): Backfill {
  for (const history of histories) {
    assert.match(history.store, /^(?:[^/]+\/[^/]+)(?:\/[^/]+\/[^/]+)*$/u, "a member store is `<slot>/<childId>`");
    if (history.transaction !== null) assert.match(history.transaction, /^tx-[0-9a-f]{16}$/u, "a transaction is its minted `tx-<hex16>` id");
  }
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

/** ⚖️ Rejects hostile domain records and derives every case through the twin; answers the case count. */
export function composedChildHistoryOracle(repoRoot: string): number {
  const fixture: Fixture = JSON.parse(readFileSync(join(repoRoot, FIXTURE_ROOT, "🔣️.json"), "utf8"));
  const first = fixture.cases[0]!;
  assert.throws(() => memberBackfill([{ ...first.histories[0]!, store: "content" }], [], {}), /member store/);
  assert.throws(() => memberBackfill([{ ...first.histories[0]!, transaction: "nodeGraphEdit" }], [], {}), /minted/);
  for (const testCase of fixture.cases) assert.deepEqual(memberBackfill(testCase.histories, testCase.logged, testCase.parentTransactions), testCase.expected, testCase.id);
  return fixture.cases.length;
}
