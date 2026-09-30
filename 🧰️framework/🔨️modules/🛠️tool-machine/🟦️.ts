/**
 * 🛠️ TypeScript mirror of the domain-neutral tool machines (`🦀️.rs`): tool yields, the tool transaction
 * reducer and the runner that drives a `@semio-tech/machine` statechart whose effects are yields, owning
 * at most one open transaction that commits one edit stamped with a `TransactionRef` or aborts with zero
 * trace; the host can always cancel (`abort`, `reset`). Schema of record: `🧬️schema/🔣️.json`; contract:
 * `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5.
 */
import { ActorId, init, macrostep, NullInspector, ROOT, routeCommand, timerElapsed, type Command, type Host, type Machine, type MachineSpec, type Snapshot, type TimerId } from "@semio-tech/machine";
import { mintTransactionRef, type TransactionRef } from "@semio-tech/framework-replication";

//#region 🔖️Yield
/** 🎇️ What a tool machine proposes to its transaction. */
export type ToolYield<M> = { readonly kind: "upsert"; readonly key: string; readonly mutation: M } | { readonly kind: "retract"; readonly key: string } | { readonly kind: "commit" } | { readonly kind: "abort" };

export const TOOL_YIELD_KINDS = ["upsert", "retract", "commit", "abort"] as const;
export type ToolYieldKind = (typeof TOOL_YIELD_KINDS)[number];
//#endregion 🔖️Yield

//#region 🔖️Transaction
export const TOOL_TRANSACTION_STATES = ["open", "committed", "aborted"] as const;
export type ToolTransactionState = (typeof TOOL_TRANSACTION_STATES)[number];

/** 🔒️ Refusal codes, each leaving no trace: a yield on a closed transaction (or while entering), and an event that leaves the tool at rest with its transaction open. */
export const TOOL_REFUSALS = ["toolTransaction.closed", "toolTransaction.unclosed"] as const;
export type ToolRefusal = (typeof TOOL_REFUSALS)[number];
export type ToolTransactionResult = { readonly ok: true } | { readonly ok: false; readonly refusal: ToolRefusal };

/** 🛑️ Who ended a transaction without an edit: the tool's own abort yield, or a host cancel. */
export const TOOL_ABORT_REASONS = ["tool", "blur", "captureLost", "baseMoved", "frozen", "retired"] as const;
export type ToolAbortReason = (typeof TOOL_ABORT_REASONS)[number];

const ACCEPTED: ToolTransactionResult = { ok: true };
const CLOSED: ToolTransactionResult = { ok: false, refusal: "toolTransaction.closed" };

/** 🧾️ Keyed provisional mutations in first-insertion order that commit as ONE edit or abort leaving nothing. */
export class ToolTransaction<M> {
  #state: ToolTransactionState = "open";
  readonly #entries: Array<[string, M]> = [];

  constructor(readonly reference: TransactionRef) {}

  /** ↩️ An open transaction restored from persisted tool state: `entries` are upserted in order, so their first-insertion order is kept and a repeated key keeps its first slot with its last mutation. */
  static resume<M>(reference: TransactionRef, entries: ReadonlyArray<readonly [string, M]>): ToolTransaction<M> {
    const transaction = new ToolTransaction<M>(reference);
    for (const [key, mutation] of entries) transaction.#upsert(key, mutation);
    return transaction;
  }

  #upsert(key: string, mutation: M): void {
    const entry = this.#entries.find(([existing]) => existing === key);
    if (entry) entry[1] = mutation;
    else this.#entries.push([key, mutation]);
  }

  get state(): ToolTransactionState {
    return this.#state;
  }

  /** 📋️ Provisional `[key, mutation]` entries in first-insertion order (the preview overlay). */
  entries(): ReadonlyArray<readonly [string, M]> {
    return this.#entries;
  }

  isEmpty(): boolean {
    return this.#entries.length === 0;
  }

  /** ⚖️ Upsert replaces by key keeping the first-insertion slot, retract removes, commit closes keeping the entries, abort closes discarding them; a closed transaction refuses every yield. */
  apply(yielded: ToolYield<M>): ToolTransactionResult {
    if (this.#state !== "open") return CLOSED;
    switch (yielded.kind) {
      case "upsert":
        this.#upsert(yielded.key, yielded.mutation);
        break;
      case "retract": {
        const index = this.#entries.findIndex(([key]) => key === yielded.key);
        if (index >= 0) this.#entries.splice(index, 1);
        break;
      }
      case "commit":
        this.#state = "committed";
        break;
      case "abort":
        this.#state = "aborted";
        this.#entries.length = 0;
        break;
    }
    return ACCEPTED;
  }

  /** 📦️ The batch in entry order. */
  mutations(): M[] {
    return this.#entries.map(([, mutation]) => mutation);
  }
}
//#endregion 🔖️Transaction

//#region 🔖️Machine
/** 🎰️ A statechart whose effects are tool yields. */
export type ToolMachineSpec = MachineSpec & { readonly Effect: ToolYield<unknown> };
export type ToolMutation<S extends ToolMachineSpec> = S["Effect"] extends ToolYield<infer M> ? M : never;

/** ⏰️ The hybrid logical clock of the moment an event is sent (replication `HybridLogicalTimestamp`). */
export type ToolClock = Parameters<typeof mintTransactionRef>[1];
export type ToolActor = Parameters<typeof mintTransactionRef>[0];

/** 🧷️ The single kernel actor a runner drives. */
export const TOOL_MACHINE_ACTOR = ActorId(0);

/** 🔁️ What one event did to the runner's transaction slot. */
export type ToolStep<M> =
  | { readonly kind: "idle" }
  | { readonly kind: "open" }
  | { readonly kind: "committed"; readonly transaction: TransactionRef; readonly mutations: M[] }
  | { readonly kind: "aborted"; readonly transaction: TransactionRef; readonly reason: ToolAbortReason }
  | { readonly kind: "empty"; readonly transaction: TransactionRef };

export type ToolStepResult<M> = { readonly ok: true; readonly step: ToolStep<M> } | { readonly ok: false; readonly refusal: ToolRefusal };

/** 🏃️ Drives one tool machine and owns at most one open transaction: it opens at the first upsert while none is open (id minted from actor, the event's clock and tool). Every event is fail-closed: a yield after the close within the event (`closed`) or an event that leaves the tool at rest, its root's initial state active, with the transaction open (`unclosed`) is refused, publishes nothing and drops the transaction. Non-yield commands go to the host. */
export class ToolMachineRunner<S extends ToolMachineSpec> {
  #transaction: ToolTransaction<ToolMutation<S>> | undefined;
  #snapshot: Snapshot<S>;

  private constructor(
    readonly machine: Machine<S>,
    readonly tool: string,
    readonly actor: ToolActor,
    readonly input: S["Input"],
    readonly host: Host<S>,
    snapshot: Snapshot<S>,
  ) {
    this.#snapshot = snapshot;
  }

  /** 🚀️ Enters the initial configuration; a yield while entering is refused. */
  static start<S extends ToolMachineSpec>(machine: Machine<S>, tool: string, actor: ToolActor, input: S["Input"], host: Host<S>): { readonly ok: true; readonly runner: ToolMachineRunner<S> } | { readonly ok: false; readonly refusal: ToolRefusal } {
    const commands: Command<S>[] = [];
    const snapshot = init(machine, input, commands);
    let refused = false;
    for (const command of commands) {
      if (command.kind === "effect") refused = true;
      else routeCommand(host, snapshot, TOOL_MACHINE_ACTOR, command);
    }
    return refused ? { ok: false, refusal: "toolTransaction.closed" } : { ok: true, runner: new ToolMachineRunner(machine, tool, actor, input, host, snapshot) };
  }

  /** ⏯️ Rebuilds a runner from persisted tool state (`intoParts`) to continue its gesture: refuses a committed or aborted transaction (`closed`) and a resting snapshot that holds an open transaction (`unclosed`). Timers scheduled before the persist stay with the host that scheduled them. */
  static resume<S extends ToolMachineSpec>(machine: Machine<S>, tool: string, actor: ToolActor, input: S["Input"], snapshot: Snapshot<S>, transaction: ToolTransaction<ToolMutation<S>> | undefined, host: Host<S>): { readonly ok: true; readonly runner: ToolMachineRunner<S> } | { readonly ok: false; readonly refusal: ToolRefusal } {
    const runner = new ToolMachineRunner(machine, tool, actor, input, host, snapshot);
    runner.#transaction = transaction;
    if (transaction && transaction.state !== "open") return { ok: false, refusal: "toolTransaction.closed" };
    if (transaction && runner.atRest()) return { ok: false, refusal: "toolTransaction.unclosed" };
    return { ok: true, runner };
  }

  /** 🧳️ The tool state to persist between dispatches (window transient): the statechart snapshot and the open transaction; `resume` continues from them. */
  intoParts(): readonly [Snapshot<S>, ToolTransaction<ToolMutation<S>> | undefined] {
    return [this.#snapshot, this.#transaction];
  }

  /** 📸️ The tool state: configuration and context (ephemeral, never history). */
  get snapshot(): Snapshot<S> {
    return this.#snapshot;
  }

  /** 🛋️ Whether the tool rests: the root's initial state is active. */
  atRest(): boolean {
    const initial = this.machine.definition.nodes[ROOT]!.initial;
    return initial !== undefined && this.#snapshot.configuration.contains(initial);
  }

  /** 🧯️ Host cancel (focus or capture lost, base moved, freeze, retirement): drops the open transaction with zero trace and returns the statechart to its initial configuration. */
  abort(reason: ToolAbortReason): ToolStep<ToolMutation<S>> {
    const transaction = this.#rest();
    return transaction ? { kind: "aborted", transaction, reason } : { kind: "idle" };
  }

  /** ♻️ Silent host reset to the freshly started runner (same input); answers the ref of the transaction it dropped. */
  reset(): TransactionRef | undefined {
    return this.#rest();
  }

  #rest(): TransactionRef | undefined {
    const { nodes } = this.machine.definition;
    for (const id of this.#snapshot.configuration.iterOnes()) {
      for (const [timer] of nodes[id]!.timers) this.host.cancelTimer(TOOL_MACHINE_ACTOR, timer);
      for (const invoke of nodes[id]!.invokes) this.host.cancelTask(TOOL_MACHINE_ACTOR, invoke);
    }
    const commands: Command<S>[] = [];
    this.#snapshot = init(this.machine, this.input, commands);
    for (const command of commands) if (command.kind !== "effect") routeCommand(this.host, this.#snapshot, TOOL_MACHINE_ACTOR, command);
    const reference = this.#transaction?.reference;
    this.#transaction = undefined;
    return reference;
  }

  /** 📝️ The open transaction, if any. */
  transaction(): ToolTransaction<ToolMutation<S>> | undefined {
    return this.#transaction;
  }

  /** 📨️ Runs `event` to completion and settles its yields. */
  send(event: S["Event"], clock: ToolClock): ToolStepResult<ToolMutation<S>> {
    const commands: Command<S>[] = [];
    macrostep(this.machine, this.#snapshot, event, commands, new NullInspector());
    return this.#settle(commands, clock);
  }

  /** ⏱️ Runs an elapsed `after` timer to completion and settles its yields. */
  timerElapsed(timer: TimerId, clock: ToolClock): ToolStepResult<ToolMutation<S>> {
    const commands: Command<S>[] = [];
    timerElapsed(this.machine, this.#snapshot, timer, commands, new NullInspector());
    return this.#settle(commands, clock);
  }

  #settle(commands: readonly Command<S>[], clock: ToolClock): ToolStepResult<ToolMutation<S>> {
    let refused = false;
    for (const command of commands) {
      if (command.kind !== "effect") {
        routeCommand(this.host, this.#snapshot, TOOL_MACHINE_ACTOR, command);
        continue;
      }
      if (refused) continue;
      const yielded = command.effect as ToolYield<ToolMutation<S>>;
      if (this.#transaction) refused = !this.#transaction.apply(yielded).ok;
      else if (yielded.kind === "upsert") {
        this.#transaction = new ToolTransaction(mintTransactionRef(this.actor, clock, this.tool));
        this.#transaction.apply(yielded);
      }
    }
    const transaction = this.#transaction;
    if (refused || (transaction?.state === "open" && this.atRest())) {
      this.#transaction = undefined;
      return { ok: false, refusal: refused ? "toolTransaction.closed" : "toolTransaction.unclosed" };
    }
    if (!transaction) return { ok: true, step: { kind: "idle" } };
    if (transaction.state === "open") return { ok: true, step: { kind: "open" } };
    this.#transaction = undefined;
    if (transaction.state === "aborted") return { ok: true, step: { kind: "aborted", transaction: transaction.reference, reason: "tool" } };
    return { ok: true, step: transaction.isEmpty() ? { kind: "empty", transaction: transaction.reference } : { kind: "committed", transaction: transaction.reference, mutations: transaction.mutations() } };
  }
}
//#endregion 🔖️Machine
