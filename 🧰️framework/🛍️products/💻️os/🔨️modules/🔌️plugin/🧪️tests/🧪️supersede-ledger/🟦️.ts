/** ✏️ TypeScript twin of the plugin runtime's supersede ledger (`⏪️time-travel/🦀️.rs` region `🔖️Supersessions`), checked
 * against the language-agnostic fixture `🧫️fixtures/🧫️supersede-ledger/🔣️.json` that `🧪️tests/🧪️supersede-ledger/🦀️.rs`
 * runs through the Rust ledger. Independent where it counts: every case is classified by this file's own implementation of the ownership law — so the roles, applied history edits,
 * undo restores and redos both runtimes reach are the fixture's. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";

type Input = string | null;
type SupersededInput = Readonly<{ target: string; input: Input }>;
type Transition = Readonly<{ id: string; actor: string; at: number; scope: string | null; inputs: readonly SupersededInput[] }>;
type Role = "edit" | "undo" | "redo";
type Authored = Readonly<{ scope: string | null; inputs: readonly SupersededInput[] }>;
type Case = Readonly<{
  id: string;
  finalAlternative: string | null;
  originals: Readonly<Record<string, string>>;
  transitions: readonly Transition[];
  records: readonly Readonly<{ id: string; role: Role; entry: string }>[];
  applied: readonly string[];
  rowsApplied: Readonly<Record<string, boolean>>;
  undo: readonly Readonly<{ actor: string; entry: string; restore: Authored }>[];
  redo: Readonly<Record<string, Readonly<{ entry: string; undo: string; redo: Authored }> | null>>;
}>;
type Fixture = Readonly<{ cases: readonly Case[] }>;

const FIXTURE_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧫️supersede-ledger";

/** 📚️ One case's transitions in fold order with their roles, entries and each author's undo and redo stacks. */
class Ledger {
  readonly records: (Transition & { role: Role; entry: number })[];
  readonly undoStacks = new Map<string, number[]>();
  readonly redoStacks = new Map<string, [number, number][]>();

  constructor(transitions: readonly Transition[], readonly originals: Readonly<Record<string, string>>) {
    for (const transition of transitions) assert.ok(transition.inputs.length > 0, "a supersession names at least one input");
    this.records = [...transitions].sort((a, b) => a.at - b.at || (a.id < b.id ? -1 : a.id > b.id ? 1 : 0)).map((transition) => ({ ...transition, role: "edit" as Role, entry: 0 }));
    this.records.forEach((_, index) => this.classify(index));
  }

  effective(before: number, scope: string | null, target: string): [number, Input] | undefined {
    for (let index = before - 1; index >= 0; index -= 1) {
      const record = this.records[index]!;
      if (record.scope !== null && record.scope !== scope) continue;
      const input = record.inputs.find((candidate) => candidate.target === target);
      if (input !== undefined) return [index, input.input];
    }
    return undefined;
  }

  owner(index: number, target: string): number | undefined {
    const record = this.records[index]!;
    if (record.role === "edit") return index;
    if (record.role === "redo") return record.entry;
    const installed = this.effective(record.entry, this.records[record.entry]!.scope, target);
    return installed === undefined ? undefined : this.owner(installed[0], target);
  }

  ownerAt(before: number, scope: string | null, target: string): number | undefined {
    const installed = this.effective(before, scope, target);
    return installed === undefined ? undefined : this.owner(installed[0], target);
  }

  held(entry: number, before: number): string[] {
    const record = this.records[entry]!;
    return record.inputs.map((input) => input.target).filter((target) => this.ownerAt(before, record.scope, target) === entry);
  }

  before(entry: number, target: string): Input | undefined {
    const installed = this.effective(entry, this.records[entry]!.scope, target);
    return installed === undefined ? this.originals[target] : installed[1];
  }

  undoStands(undo: number, before: number): boolean {
    const record = this.records[undo]!;
    return record.inputs.every((input) => this.ownerAt(before, record.scope, input.target) === this.owner(undo, input.target));
  }

  classify(index: number): void {
    const record = this.records[index]!;
    const targets = new Set(record.inputs.map((input) => input.target));
    const sameTargets = (other: readonly string[]) => other.length === targets.size && other.every((target) => targets.has(target));
    const undoStack = this.undoStacks.get(record.actor) ?? [];
    const undone = [...undoStack].reverse().find((entry) => this.records[entry]!.scope === record.scope && sameTargets(this.held(entry, index)) && record.inputs.every((input) => this.before(entry, input.target) === input.input));
    const redoTop = this.redoStacks.get(record.actor)?.at(-1);
    const redone =
      undone === undefined &&
      redoTop !== undefined &&
      this.records[redoTop[0]]!.scope === record.scope &&
      sameTargets(this.records[redoTop[1]]!.inputs.map((input) => input.target)) &&
      this.undoStands(redoTop[1], index) &&
      record.inputs.every((input) => this.records[redoTop[0]]!.inputs.some((edited) => edited.target === input.target && edited.input === input.input));
    if (undone !== undefined) {
      this.undoStacks.set(record.actor, undoStack.filter((entry) => entry !== undone));
      this.redoStacks.set(record.actor, [...(this.redoStacks.get(record.actor) ?? []), [undone, index]]);
      Object.assign(record, { role: "undo", entry: undone });
    } else if (redone && redoTop !== undefined) {
      this.redoStacks.set(record.actor, (this.redoStacks.get(record.actor) ?? []).slice(0, -1));
      this.undoStacks.set(record.actor, [...undoStack, redoTop[0]]);
      Object.assign(record, { role: "redo", entry: redoTop[0] });
    } else {
      this.redoStacks.delete(record.actor);
      this.undoStacks.set(record.actor, [...undoStack, index]);
      Object.assign(record, { role: "edit", entry: index });
    }
  }

  applied(finalAlternative: string | null): Set<number> {
    const targets = new Set(this.records.flatMap((record) => record.inputs.map((input) => input.target)));
    const owners = [...targets].map((target) => this.ownerAt(this.records.length, finalAlternative, target));
    return new Set(owners.filter((owner): owner is number => owner !== undefined));
  }

  rowApplied(index: number, applied: ReadonlySet<number>): boolean {
    const record = this.records[index]!;
    return record.role === "undo" ? !(this.undoStacks.get(record.actor) ?? []).includes(record.entry) : applied.has(record.entry);
  }

  restore(entry: number): Authored {
    return { scope: this.records[entry]!.scope, inputs: this.held(entry, this.records.length).map((target) => ({ target, input: this.before(entry, target) ?? null })) };
  }

  redoTop(actor: string): [number, number] | undefined {
    const top = this.redoStacks.get(actor)?.at(-1);
    return top !== undefined && this.undoStands(top[1], this.records.length) ? top : undefined;
  }

  redo(entry: number, undo: number): Authored {
    const edited = this.records[entry]!;
    return { scope: edited.scope, inputs: this.records[undo]!.inputs.flatMap((restored) => edited.inputs.filter((input) => input.target === restored.target)) };
  }
}

/** ⚖️ Rejects empty supersessions and classifies every case through the twin; answers the case count. */
export function supersedeLedgerOracle(repoRoot: string): number {
  const fixture: Fixture = JSON.parse(readFileSync(join(repoRoot, FIXTURE_ROOT, "🔣️.json"), "utf8"));
  assert.throws(() => new Ledger([{ ...fixture.cases[0]!.transitions[0]!, inputs: [] }], fixture.cases[0]!.originals), /at least one input/);
  for (const testCase of fixture.cases) {
    const ledger = new Ledger(testCase.transitions, testCase.originals);
    const name = (index: number) => ledger.records[index]!.id;
    assert.deepEqual(ledger.records.map((record) => ({ id: record.id, role: record.role, entry: name(record.entry) })), testCase.records, `${testCase.id}: records`);
    const applied = ledger.applied(testCase.finalAlternative);
    assert.deepEqual([...applied].map(name).sort(), testCase.applied, `${testCase.id}: applied history edits`);
    assert.deepEqual(Object.fromEntries(ledger.records.map((record, index) => [record.id, ledger.rowApplied(index, applied)])), testCase.rowsApplied, `${testCase.id}: applied rows`);
    const undo = ledger.records.flatMap((record, index) => (record.role === "edit" && applied.has(index) && (ledger.undoStacks.get(record.actor) ?? []).includes(index) ? [{ actor: record.actor, entry: name(index), restore: ledger.restore(index) }] : []));
    assert.deepEqual(undo, testCase.undo, `${testCase.id}: undo`);
    for (const [actor, expected] of Object.entries(testCase.redo)) {
      const top = ledger.redoTop(actor);
      assert.deepEqual(top === undefined ? null : { entry: name(top[0]), undo: name(top[1]), redo: ledger.redo(top[0], top[1]) }, expected, `${testCase.id}: redo of ${actor}`);
    }
  }
  return fixture.cases.length;
}
