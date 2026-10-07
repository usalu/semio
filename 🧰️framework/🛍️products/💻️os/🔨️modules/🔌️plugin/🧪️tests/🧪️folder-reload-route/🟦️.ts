/** 📁️ TypeScript twin of the folder reload route law (`🧪️tests/🧪️folder-reload-route/🦀️.rs`), checked against the
 * language-agnostic fixture `🧫️fixtures/🧫️folder-reload-route/🔣️.json`. Independent where it counts: every expectation is derived here from the fixture's steps alone with
 * this file's own fold of the count-and-label document — an unscoped history edit holds on every line, a scoped one only
 * on its alternative, a mutation that changes nothing warns, and finalizing as a new alternative moves only its author. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";

type Edit = Readonly<{ kind: "setCount"; value: number }> | Readonly<{ kind: "setLabel"; value: string }>;
type Head = Readonly<{ count: number; label: string }>;
type Label = Readonly<{ en: string; de: string }>;
type Step = Readonly<Record<string, unknown>>;
type Supersession = Readonly<{ target: number; replacement: Edit; scope: string | null }>;
type Program = Readonly<{ head: Head; line: string; alternatives: readonly string[]; superseded: readonly number[] }>;
type Author = Program & Readonly<{ warnings: readonly Readonly<{ mutation: number; code: string }>[]; historyRows: readonly Label[]; editRows: number; afterReload: Head }>;
type Merge = Readonly<{ merged: number; ahead: number }>;
type TwoPeers = Readonly<{
  author: readonly Step[];
  session: readonly Step[];
  peerEdit: Edit;
  finish: readonly Step[];
  expected: Readonly<{ previewBody: string; pendingWhileEditing: readonly number[]; authorMerge: Merge; peerMerge: Merge; settled: Readonly<{ head: Head; superseded: readonly number[]; historyRows: readonly Label[]; editRows: number }> }>;
}>;
type Fixture = Readonly<{ steps: readonly Step[]; readerEdit: Edit; afterReload: Edit; expected: Readonly<{ author: Author; reader: Program }>; twoPeers: TwoPeers }>;
type Fold = Readonly<{ head: Head; superseded: readonly number[]; unchanged: readonly number[] }>;

const FIXTURE_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧫️folder-reload-route";

/** 🌐️ A history row names both explicit locales. */
function assertLabel(label: unknown): void {
  assert.ok(label !== null && typeof label === "object", "a history row is an object");
  for (const locale of ["en", "de"] as const) assert.ok(typeof (label as Partial<Label>)[locale] === "string" && (label as Label)[locale].length > 0, `a history row names ${locale}`);
}

/** 🧮️ A document head carries its count and label. */
function assertHead(head: unknown): void {
  assert.ok(head !== null && typeof head === "object", "a head is an object");
  const value = head as Partial<Head>;
  assert.ok(Number.isInteger(value.count) && typeof value.label === "string", "a head names its integer count and its label");
}

/** 🔀️ A read-back merge answers its taken and ahead event counts. */
function assertMerge(merge: unknown): void {
  assert.ok(merge !== null && typeof merge === "object", "a merge is an object");
  const value = merge as Partial<Merge>;
  assert.ok(Number.isInteger(value.merged) && value.merged! >= 0 && Number.isInteger(value.ahead) && value.ahead! >= 0, "a merge answers its taken and ahead counts");
}

/** ✏️ Document edits carry a typed value. */
function assertEdit(edit: Edit): void {
  if (edit.kind === "setCount") assert.ok(Number.isInteger(edit.value), "a count edit carries an integer");
  else assert.ok(edit.kind === "setLabel" && typeof edit.value === "string", "a label edit carries text");
}

/** ✅️ A finalize selects an overwrite or a named alternative. */
function assertCommit(commit: unknown): asserts commit is { choice?: "overwrite"; name?: string } {
  assert.ok(commit !== null && typeof commit === "object", "a finalize selects a choice");
  const value = commit as { choice?: unknown; name?: unknown };
  assert.ok((value.choice === "overwrite" && value.name === undefined) || (value.choice === undefined && typeof value.name === "string" && value.name.length > 0), "a finalize is an overwrite or a named alternative, never both");
}

/** ✏️ One edit applied to a head. */
function applyEdit(head: Head, edit: Edit): Head {
  assertEdit(edit);
  return edit.kind === "setCount" ? { ...head, count: edit.value } : { ...head, label: edit.value };
}

/** 🧮️ The document a program viewing `line` folds: every mutation under its newest history edit that holds on that line. */
export function foldLine(edits: readonly Edit[], supersessions: readonly Supersession[], line: string): Fold {
  let head: Head = { count: 0, label: "" };
  const superseded: number[] = [];
  const unchanged: number[] = [];
  edits.forEach((recorded, position) => {
    const held = supersessions.filter((supersession) => supersession.target === position && (supersession.scope === null || supersession.scope === line)).at(-1);
    if (held !== undefined) superseded.push(position);
    const next = applyEdit(head, held?.replacement ?? recorded);
    if (next.count === head.count && next.label === head.label) unchanged.push(position);
    head = next;
  });
  return { head, superseded, unchanged };
}

/** 🏷️ The history row of one finalized history edit in every locale. */
export function historyEditRow(scope: string | null, mutations: number): Label {
  const count: Label = mutations === 1 ? { en: "1 mutation", de: "1 Mutation" } : { en: `${mutations} mutations`, de: `${mutations} Mutationen` };
  const where: Label = scope === null ? { en: "overwrite", de: "überschrieben" } : { en: `alternative ${scope}`, de: `Alternative ${scope}` };
  return { en: `History edited — ${where.en}: ${count.en}`, de: `Verlauf bearbeitet — ${where.de}: ${count.de}` };
}

/** 🚶️ Walks the author's steps: the edits in applied order, the finalized history edits and the alternatives they named. */
export function walkAuthor(steps: readonly Step[]): Readonly<{ edits: readonly Edit[]; supersessions: readonly Supersession[]; alternatives: readonly string[] }> {
  const edits: Edit[] = [];
  const supersessions: Supersession[] = [];
  const alternatives: string[] = [];
  let draft: { target: number; replacement: Edit } | null = null;
  for (const step of steps) {
    assert.equal(Object.keys(step).length, 1, "a step names exactly one verb");
    const [name, argument] = Object.entries(step)[0]!;
    if (name === "edit") {
      assert.equal(draft, null, "a document edit is frozen while the history is edited");
      assertEdit(argument as Edit);
      edits.push(argument as Edit);
    } else if (name === "begin") {
      const target = argument as number;
      assert.ok(target < edits.length, `begin names the applied mutation ${target}`);
      draft = { target, replacement: edits[target]! };
    } else if (name === "input") {
      assert.ok(draft !== null, "an input drafts an open session");
      draft = { target: draft.target, replacement: { ...draft.replacement, value: (argument as { value: unknown }).value } as Edit };
    } else if (name === "commit") {
      assert.ok(draft !== null, "a commit finalizes an open session");
      assertCommit(argument);
      assertEdit(draft.replacement);
      const scope = argument.name ?? null;
      if (scope !== null) alternatives.push(scope);
      supersessions.push({ ...draft, scope });
      draft = null;
    }
  }
  assert.equal(draft, null, "every session is finalized before the reload");
  return { edits, supersessions, alternatives };
}

/** 📁️ What the reload must give each program back, derived from the steps alone. */
export function deriveFolderReload(fixture: Fixture): Fixture["expected"] {
  const { edits, supersessions, alternatives } = walkAuthor(fixture.steps);
  const listed = alternatives.length === 0 ? [] : ["", ...alternatives];
  const authorLine = alternatives.at(-1) ?? "";
  const author = foldLine(edits, supersessions, authorLine);
  const reader = foldLine([...edits, fixture.readerEdit], supersessions, "");
  return {
    author: {
      head: author.head,
      line: authorLine,
      alternatives: listed,
      superseded: author.superseded,
      warnings: author.unchanged.map((mutation) => ({ mutation, code: "mutation.no-op" })),
      historyRows: supersessions.map((supersession) => historyEditRow(supersession.scope, 1)).reverse(),
      editRows: edits.length,
      afterReload: foldLine([...edits, fixture.afterReload], supersessions, authorLine).head,
    },
    reader: { head: reader.head, line: "", alternatives: listed, superseded: reader.superseded },
  };
}

/** 👥️ What two programs on one folder must show (live fault F4), derived from the steps alone: the author's preview is the
 * state before its edited mutation with the draft, every later mutation — the peer's included — is not applied while it
 * edits, a read-back merge takes exactly the events the reader lacks, nobody is ahead of the folder it just read, and once
 * settled both fold the same log under the author's history edit. */
export function deriveTwoPeers(scenario: TwoPeers): TwoPeers["expected"] {
  const before = walkAuthor(scenario.author);
  assert.deepEqual([before.supersessions, before.alternatives], [[], []], "the author only edits before the peer attaches");
  const begin = scenario.session.find((step) => "begin" in step) as { begin: number } | undefined;
  const input = scenario.session.find((step) => "input" in step) as { input: { value: unknown } } | undefined;
  assert.ok(begin !== undefined && input !== undefined, "the session begins a mutation and drafts it");
  const draft = { ...before.edits[begin.begin]!, value: input.input.value } as Edit;
  const preview = [...before.edits.slice(0, begin.begin), draft].reduce(applyEdit, { count: 0, label: "" } as Head);
  const log = [...before.edits, scenario.peerEdit];
  const finalized = walkAuthor([...scenario.author, ...scenario.session, ...scenario.finish]);
  const settled = foldLine(log, finalized.supersessions, "");
  return {
    previewBody: `count=${preview.count} label=${preview.label}`,
    pendingWhileEditing: log.map((_, position) => position).filter((position) => position > begin.begin),
    authorMerge: { merged: 1, ahead: 0 },
    peerMerge: { merged: finalized.supersessions.length, ahead: 0 },
    settled: { head: settled.head, superseded: settled.superseded, historyRows: finalized.supersessions.map((supersession) => historyEditRow(supersession.scope, 1)).reverse(), editRows: log.length },
  };
}

/** ⚖️ Rejects hostile domain records and derives the expectation through the twin; answers the step count. */
export function folderReloadRouteOracle(repoRoot: string): number {
  const fixture: Fixture = JSON.parse(readFileSync(join(repoRoot, FIXTURE_ROOT, "🔣️.json"), "utf8"));
  assert.throws(() => assertCommit({ choice: "overwrite", name: "Both" }), /never both/);
  assert.throws(() => assertEdit({ kind: "setCount", value: "seven" } as unknown as Edit), /integer/);
  assert.throws(() => assertHead({ count: 9 }), /count and its label/);
  assert.throws(() => assertLabel({ en: "History edited" }), /names de/);
  assert.throws(() => assertMerge({ merged: 1 }), /taken and ahead/);
  for (const head of [fixture.expected.author.head, fixture.expected.author.afterReload, fixture.expected.reader.head, fixture.twoPeers.expected.settled.head]) assertHead(head);
  for (const row of [...fixture.expected.author.historyRows, ...fixture.twoPeers.expected.settled.historyRows]) assertLabel(row);
  assertMerge(fixture.twoPeers.expected.authorMerge);
  assertMerge(fixture.twoPeers.expected.peerMerge);
  assert.deepEqual(deriveFolderReload(fixture), fixture.expected);
  assert.notDeepEqual(fixture.expected.author.head, fixture.expected.reader.head, "the two programs view different lines, so a reload that lost a viewed alternative is visible in the head");
  assert.deepEqual(deriveTwoPeers(fixture.twoPeers), fixture.twoPeers.expected);
  assert.notEqual(fixture.twoPeers.expected.previewBody, `count=${fixture.twoPeers.expected.settled.head.count} label=${fixture.twoPeers.expected.settled.head.label}`, "the preview while editing differs from the settled head, so a read-back that replaced the session's document is visible");
  return fixture.steps.length + fixture.twoPeers.author.length + fixture.twoPeers.session.length + fixture.twoPeers.finish.length;
}
