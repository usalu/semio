/** 🎞️ Tool-transaction corpus (design §15, transaction-scoped amend): Ajv validates its shape; an independent TS twin of the
 * store's law — plain edits, appends into one open edit, commit, abort, undo and remote edits lifted under the open edit,
 * folded with fast-json-patch — reproduces every step the Rust store is checked against (`🦀️.rs` beside this file); an xstate
 * lifecycle machine agrees with the twin's refusals and open transaction on the corpus and on fast-check generated runs,
 * where an abort always restores the state and persisted value of before the transaction and a commit adds exactly one edit. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import fc from "fast-check";
import { applyPatch, type Operation as Patch } from "fast-json-patch";
import { initialTransition, setup, transition, assign, type AnyMachineSnapshot } from "xstate";

//#region 🧮️Twin
type Snapshot = { n: number | null };
type DemoOperation = { operation: "setN"; n: number } | { operation: "addN"; delta: number };
type Refusal = "transactionOpen" | "unknownTransaction" | "emptyApply";
type Step =
  | { apply: DemoOperation[] }
  | { append: { transaction: string; operations: DemoOperation[] } }
  | { commit: string }
  | { abort: string }
  | { undo: true }
  | { remote: { clock: "after" | "before"; operation: DemoOperation } };
type Expect = { refused: Refusal | null; n: number | null; edits: number; open: string | null; openOperations: number; announced: number; persisted: number | null };
type Case = { name: string; initial: Snapshot; steps: (Step & { expect: Expect })[] };
type Edit = { operations: DemoOperation[]; clock: number; local: boolean; undone: boolean };

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const saturate = (value: number) => Math.min(2147483647, Math.max(-2147483648, value));

/** 🩹️ One demo operation as the JSON Patch it writes against `state` (a write onto an absent value writes nothing). */
function patch(operation: DemoOperation, state: Snapshot): Patch[] {
  if (state.n === null) return [];
  return [{ op: "replace", path: "/n", value: operation.operation === "setN" ? operation.n : saturate(state.n + operation.delta) }];
}

/** 🧺️ The TS twin of the store under transaction-scoped amend: committed edits fold in clock order, the open edit always last. */
class TransactionTwin {
  readonly #initial: Snapshot;
  readonly #edits: Edit[] = [];
  #open: { transaction: string; operations: DemoOperation[] } | null = null;
  #clock = 0;
  #earliest = 0;

  constructor(initial: Snapshot) {
    this.#initial = initial;
  }

  #fold(edits: readonly Edit[], tail: readonly DemoOperation[]): number | null {
    const ordered = [...edits].filter((edit) => !edit.undone).sort((left, right) => left.clock - right.clock);
    let state: Snapshot = structuredClone(this.#initial);
    for (const operation of [...ordered.flatMap((edit) => edit.operations), ...tail]) state = applyPatch(state, patch(operation, state), true, false).newDocument;
    return state.n;
  }

  /** 📸️ What the store must show after the last step. */
  observe(refused: Refusal | null, announced: number): Expect {
    return {
      refused,
      n: this.#fold(this.#edits, this.#open?.operations ?? []),
      edits: this.#edits.length + (this.#open ? 1 : 0),
      open: this.#open?.transaction ?? null,
      openOperations: this.#open?.operations.length ?? 0,
      announced,
      persisted: this.#fold(this.#edits, []),
    };
  }

  /** ▶️ Runs one step: its refusal (nothing changed) or its effect, and the operations it announced. */
  step(step: Step): Expect {
    if ("apply" in step || "undo" in step) {
      if (this.#open) return this.observe("transactionOpen", 0);
      if ("undo" in step) {
        const target = [...this.#edits].filter((edit) => edit.local && !edit.undone).sort((left, right) => right.clock - left.clock)[0];
        if (target) target.undone = true;
        return this.observe(null, 0);
      }
      this.#edits.push({ operations: step.apply, clock: ++this.#clock, local: true, undone: false });
      return this.observe(null, step.apply.length);
    }
    if ("append" in step) {
      if (this.#open && this.#open.transaction !== step.append.transaction) return this.observe("transactionOpen", 0);
      if (step.append.operations.length === 0) return this.observe("emptyApply", 0);
      this.#open ??= { transaction: step.append.transaction, operations: [] };
      this.#open.operations.push(...step.append.operations);
      return this.observe(null, 0);
    }
    if ("commit" in step || "abort" in step) {
      const named = "commit" in step ? step.commit : step.abort;
      if (!this.#open || this.#open.transaction !== named) return this.observe("unknownTransaction", 0);
      const closed = this.#open;
      this.#open = null;
      if ("abort" in step) return this.observe(null, 0);
      this.#edits.push({ operations: closed.operations, clock: ++this.#clock, local: true, undone: false });
      return this.observe(null, closed.operations.length);
    }
    this.#edits.push({ operations: [step.remote.operation], clock: step.remote.clock === "after" ? ++this.#clock : --this.#earliest, local: false, undone: false });
    return this.observe(null, 0);
  }
}
//#endregion 🧮️Twin

//#region 🔄️LifecycleOracle
type LifecycleEvent = { type: "apply" } | { type: "undo" } | { type: "remote" } | { type: "append"; transaction: string; empty: boolean } | { type: "commit"; transaction: string } | { type: "abort"; transaction: string };

/** 🔄️ The open/idle lifecycle and its refusals as an xstate machine — independent of the twin's ledger. */
const lifecycle = setup({
  types: {} as { context: { transaction: string | null; refused: Refusal | null }; events: LifecycleEvent },
  guards: {
    other: ({ context, event }) => "transaction" in event && event.transaction !== context.transaction,
    empty: ({ event }) => event.type === "append" && event.empty,
  },
  actions: {
    accept: assign({ refused: () => null }),
    refuseOpen: assign({ refused: () => "transactionOpen" as const }),
    refuseUnknown: assign({ refused: () => "unknownTransaction" as const }),
    refuseEmpty: assign({ refused: () => "emptyApply" as const }),
    opens: assign({ transaction: ({ event }) => ("transaction" in event ? event.transaction : null), refused: () => null }),
    closes: assign({ transaction: () => null, refused: () => null }),
  },
}).createMachine({
  id: "toolTransaction",
  initial: "idle",
  context: { transaction: null, refused: null },
  states: {
    idle: {
      on: {
        apply: { actions: "accept" },
        undo: { actions: "accept" },
        remote: { actions: "accept" },
        append: [{ guard: "empty", actions: "refuseEmpty" }, { target: "open", actions: "opens" }],
        commit: { actions: "refuseUnknown" },
        abort: { actions: "refuseUnknown" },
      },
    },
    open: {
      on: {
        apply: { actions: "refuseOpen" },
        undo: { actions: "refuseOpen" },
        remote: { actions: "accept" },
        append: [{ guard: "other", actions: "refuseOpen" }, { guard: "empty", actions: "refuseEmpty" }, { actions: "accept" }],
        commit: [{ guard: "other", actions: "refuseUnknown" }, { target: "idle", actions: "closes" }],
        abort: [{ guard: "other", actions: "refuseUnknown" }, { target: "idle", actions: "closes" }],
      },
    },
  },
});

function lifecycleEvent(step: Step): LifecycleEvent {
  if ("apply" in step) return { type: "apply" };
  if ("undo" in step) return { type: "undo" };
  if ("remote" in step) return { type: "remote" };
  if ("append" in step) return { type: "append", transaction: step.append.transaction, empty: step.append.operations.length === 0 };
  if ("commit" in step) return { type: "commit", transaction: step.commit };
  return { type: "abort", transaction: step.abort };
}

/** 🧭️ Runs `steps` through the lifecycle machine: per step, its refusal and the transaction left open. */
function lifecycleRun(steps: readonly Step[]): { refused: Refusal | null; open: string | null }[] {
  let snapshot: AnyMachineSnapshot = initialTransition(lifecycle)[0];
  return steps.map((step) => {
    snapshot = transition(lifecycle, snapshot, lifecycleEvent(step))[0];
    return { refused: snapshot.context.refused, open: snapshot.context.transaction };
  });
}
//#endregion 🔄️LifecycleOracle

//#region 🧪️Corpus
const corpus = read("../../🧫️fixtures/🧫️tool-transaction/🔣️.json");
const schema = read("../../🧬️schema/🔣️tool-transaction/🔣️.json");
const cases = corpus.cases as Case[];

test("🧬️ the tool-transaction corpus satisfies its JSON Schema", () => {
  const validate = new Ajv({ strict: true, allErrors: true, allowUnionTypes: true }).compile(schema);
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
});

test("🧺️ the TS twin reproduces every step of every case", () => {
  for (const testCase of cases) {
    const twin = new TransactionTwin(testCase.initial);
    testCase.steps.forEach((step, index) => expect(twin.step(step), `${testCase.name} step ${index}`).toEqual(step.expect));
  }
});

test("🔄️ the xstate lifecycle agrees with every refusal and open transaction of the corpus", () => {
  for (const testCase of cases) {
    expect(lifecycleRun(testCase.steps), testCase.name).toEqual(testCase.steps.map((step) => ({ refused: step.expect.refused, open: step.expect.open })));
  }
});
//#endregion 🧪️Corpus

//#region 🔡️CommandCodec
type CodecCommand = { append: string } | { commit: string } | { abort: string };

/** 🔢️ Unsigned LEB128, the varint every command field length and ordinal uses. */
function leb128(value: number): number[] {
  const bytes: number[] = [];
  do {
    const low = value & 0x7f;
    value = Math.floor(value / 128);
    bytes.push(value > 0 ? low | 0x80 : low);
  } while (value > 0);
  return bytes;
}

const utf8 = (text: string) => [...new TextEncoder().encode(text)];
const field = (text: string) => [...leb128(utf8(text).length), ...utf8(text)];

/** 🔡️ An independent encoder of the transaction commands: format byte 1, the ordinal (19 append, 20 commit, 21 abort), then
 * the transaction id, for an append its tool and its operation count. */
function encodeCommand(command: CodecCommand, tool: string): string {
  const bytes = "append" in command ? [1, ...leb128(19), ...field(command.append), ...field(tool), ...leb128(0)] : "commit" in command ? [1, ...leb128(20), ...field(command.commit)] : [1, ...leb128(21), ...field(command.abort)];
  return bytes.map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

test("🔡️ an independent encoder reproduces every command vector", () => {
  for (const vector of corpus.codec as { command: CodecCommand; hex: string }[]) expect(encodeCommand(vector.command, corpus.tool), JSON.stringify(vector.command)).toBe(vector.hex);
});
//#endregion 🔡️CommandCodec

//#region 🎲️Properties
const operation = fc.oneof(fc.record({ operation: fc.constant("setN" as const), n: fc.integer({ min: -50, max: 50 }) }), fc.record({ operation: fc.constant("addN" as const), delta: fc.integer({ min: -5, max: 5 }) }));
const transactionId = fc.constantFrom("tx-a", "tx-b");
const step: fc.Arbitrary<Step> = fc.oneof(
  fc.record({ apply: fc.array(operation, { minLength: 1, maxLength: 3 }) }),
  fc.record({ append: fc.record({ transaction: transactionId, operations: fc.array(operation, { maxLength: 3 }) }) }),
  fc.record({ commit: transactionId }),
  fc.record({ abort: transactionId }),
  fc.constant({ undo: true as const }),
  fc.record({ remote: fc.record({ clock: fc.constantFrom("after" as const, "before" as const), operation }) }),
);

test("🎲️ on random runs the twin and the xstate lifecycle agree, an abort leaves no trace and a commit is one announced edit", () => {
  fc.assert(
    fc.property(fc.array(step, { maxLength: 40 }), (steps) => {
      const twin = new TransactionTwin({ n: 0 });
      const oracle = lifecycleRun(steps);
      let before: Expect | null = null;
      let foreign = 0;
      steps.forEach((step, index) => {
        const prior = twin.observe(null, 0);
        const observed = twin.step(step);
        expect({ refused: observed.refused, open: observed.open }).toEqual(oracle[index]);
        if (observed.refused !== null) expect({ ...observed, refused: null }).toEqual({ ...prior, refused: null });
        if (prior.open === null && observed.open !== null) {
          before = prior;
          foreign = 0;
        }
        if ("remote" in step) foreign += 1;
        if ("abort" in step && observed.refused === null && before !== null && foreign === 0) expect({ n: observed.n, edits: observed.edits, persisted: observed.persisted }).toEqual({ n: before.n, edits: before.edits, persisted: before.persisted });
        if ("commit" in step && observed.refused === null) {
          expect(observed.edits).toBe(prior.edits);
          expect(observed.announced).toBe(prior.openOperations);
          expect(observed.persisted).toBe(observed.n);
        }
      });
    }),
    { numRuns: 500 },
  );
});
//#endregion 🎲️Properties
