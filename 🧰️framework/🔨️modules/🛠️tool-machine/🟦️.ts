/**
 * 🛠️ TypeScript mirror of the domain-neutral tool machines (`🦀️.rs`): tool yields, the tool transaction
 * reducer and the runner that drives a `@semio-tech/machine` statechart whose effects are yields, owning
 * at most one open transaction that commits one edit stamped with a `TransactionRef` or aborts with zero
 * trace; the host can always cancel (`abort`, `reset`). Schema of record: `🧬️schema/🔣️.json`; contract:
 * `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5.
 */
import { ActionId, ActorId, EventId, GuardId, init, macrostep, NodeId, NullInspector, persist, restore, ROOT, routeCommand, timerElapsed, type ActionFn, type Command, type Host, type InvokeId, type Machine, type MachineSpec, type NodeDef, type Snapshot, type TimerId, type TransitionDef } from "@semio-tech/machine";
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

//#region 🔖️Scrub
/** 🎚️ The scrub protocol every continuous control speaks: `gesture` names the press (`"<control>:<ms>"`), `commit: true` marks the release, `abort: "<reason>"` a host cancel (no value). A dispatch without `gesture` is a plain one-shot edit. */
export const SCRUB_GESTURE_ARG = "gesture";
export const SCRUB_COMMIT_ARG = "commit";
export const SCRUB_ABORT_ARG = "abort";

/** 🎚️ Where one dispatch of a continuous control sits in its press. */
export type ScrubPhase = { readonly kind: "tick"; readonly gesture: string } | { readonly kind: "commit"; readonly gesture: string } | { readonly kind: "abort"; readonly gesture: string; readonly reason: ToolAbortReason };

/** 🧩️ Reads the scrub arguments: `undefined` without a non-empty `gesture` or with an unknown abort reason; `abort` wins over `commit`. */
export function parseScrubPhase(gesture: unknown, commit: unknown, abort: unknown): ScrubPhase | undefined {
  if (typeof gesture !== "string" || gesture === "") return undefined;
  if (abort !== undefined) return typeof abort === "string" && (TOOL_ABORT_REASONS as readonly string[]).includes(abort) ? { kind: "abort", gesture, reason: abort as ToolAbortReason } : undefined;
  return commit === true ? { kind: "commit", gesture } : { kind: "tick", gesture };
}

/** 📨️ What reaches a scrub: a live value's absolute leaves, the release's leaves, or a host cancel. */
export type ScrubInput<M> = { readonly kind: "tick"; readonly gesture: string; readonly leaves: readonly M[] } | { readonly kind: "commit"; readonly gesture: string; readonly leaves: readonly M[] } | { readonly kind: "abort"; readonly reason: ToolAbortReason };

/** 🧰️ A scrub's tool state: the press it follows and how many keyed leaves (`"0"`, `"1"`, …) its transaction holds. */
export type ScrubContext = { gesture: string | undefined; keys: number };

const SCRUB_EVENT_NAMES = ["Tick", "Commit"] as const;
/** 📨️ The scrub statechart's events; the host cancel is the runner's `abort`, never an event. */
export type ScrubEvent<M> = { readonly type: (typeof SCRUB_EVENT_NAMES)[number]; readonly gesture: string; readonly leaves: readonly M[]; readonly eventCount: number; eventId(): EventId; eventName(id: EventId): string };

export function scrubEvent<M>(type: ScrubEvent<M>["type"], gesture: string, leaves: readonly M[]): ScrubEvent<M> {
  return { type, gesture, leaves, eventCount: SCRUB_EVENT_NAMES.length, eventId: () => EventId(SCRUB_EVENT_NAMES.indexOf(type)), eventName: (id) => SCRUB_EVENT_NAMES[id] ?? "?" };
}

export interface ScrubSpec<M> extends MachineSpec {
  Context: ScrubContext;
  Event: ScrubEvent<M>;
  Input: ScrubContext;
  Output: never;
  Effect: ToolYield<M>;
}

/** 🔏️ Twin of Rust `SCRUB_FINGERPRINT` (the `statechart!` fingerprint of the chart). */
export const SCRUB_FINGERPRINT = 16240238296638685209n;
/** 🗺️ Twin of Rust `SCRUB_MANIFEST_JSON`. */
export const SCRUB_MANIFEST_JSON = '{"id":"scrub","states":[{"id":"root","parent":null},{"id":"idle","parent":0},{"id":"scrubbing","parent":0}],"events":["Tick","Commit"],"transitionCount":4}';

function scrubNode(stableId: string, docIndex: number): NodeDef {
  return { stableId, kind: "atomic", parent: ROOT, children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex };
}

const SCRUB_NODES: readonly NodeDef[] = [{ stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(2)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 }, scrubNode("idle", 1), scrubNode("scrubbing", 2)];

const SCRUB_TRANSITIONS: readonly TransitionDef[] = [
  { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [ActionId(0)], docIndex: 0 },
  { source: NodeId(1), trigger: { kind: "event", event: EventId(1) }, targets: [NodeId(1)], kind: "external", actions: [ActionId(1)], docIndex: 1 },
  { source: NodeId(2), trigger: { kind: "event", event: EventId(0) }, guard: GuardId(0), targets: [NodeId(2)], kind: "external", actions: [ActionId(0)], docIndex: 2 },
  { source: NodeId(2), trigger: { kind: "event", event: EventId(1) }, guard: GuardId(0), targets: [NodeId(1)], kind: "external", actions: [ActionId(1)], docIndex: 3 },
];

function scrubReplace<M>(context: ScrubContext, leaves: readonly M[], sink: Command<ScrubSpec<M>>[]): void {
  leaves.forEach((leaf, index) => sink.push({ kind: "effect", effect: { kind: "upsert", key: String(index), mutation: leaf } }));
  for (let index = leaves.length; index < context.keys; index += 1) sink.push({ kind: "effect", effect: { kind: "retract", key: String(index) } });
  context.keys = leaves.length;
}

/** 🎚️ The ONE continuous-control tool (twin of Rust `ScrubMachine<M>`): `idle → scrubbing` on a tick, ticks of the same press stay, the release of the same press returns to `idle` (a release from `idle` is a one-shot press); every tick replaces the entries with its absolute leaves, the release commits ONE edit, a host abort leaves zero trace. */
export function scrubMachine<M>(): Machine<ScrubSpec<M>> {
  const follow: ActionFn<ScrubSpec<M>> = (context, event, sink) => {
    if (event?.type !== "Tick") return;
    context.gesture = event.gesture;
    scrubReplace(context, event.leaves, sink as Command<ScrubSpec<M>>[]);
  };
  const settle: ActionFn<ScrubSpec<M>> = (context, event, sink) => {
    if (event?.type !== "Commit") return;
    scrubReplace(context, event.leaves, sink as Command<ScrubSpec<M>>[]);
    sink.push({ kind: "effect", effect: { kind: "commit" } });
    context.gesture = undefined;
    context.keys = 0;
  };
  return {
    definition: {
      id: "scrub",
      nodes: SCRUB_NODES,
      transitions: SCRUB_TRANSITIONS,
      contextFromInput: (input) => ({ ...input }),
      guards: [(context, event) => event !== undefined && context.gesture === event.gesture],
      actions: [follow, settle],
      fingerprint: SCRUB_FINGERPRINT,
      manifestJson: SCRUB_MANIFEST_JSON,
    },
  };
}

/** 🧷️ The scrub's host: no timer, no invoke, no foreign effect. */
export class ScrubHost<M> implements Host<ScrubSpec<M>> {
  executeEffect(): void {}
  schedule(): void {}
  cancelTimer(): void {}
  startTask(_actor: ActorId, _invoke: InvokeId): void {}
  cancelTask(_actor: ActorId, _invoke: InvokeId): void {}
  nowMs(): number {
    return 0;
  }
}

/** 💾️ One window's open scrub between dispatches (window transient, never history). */
export type ScrubState<M> = { readonly states: readonly string[]; readonly tool: string; readonly actor: ToolActor; readonly gesture: string; readonly baseRevision: string; readonly transaction: TransactionRef; readonly entries: ReadonlyArray<readonly [string, M]> };

/** 🎚️ One press of a continuous control on one document revision (twin of Rust `Scrub<M>`); a tick or release of another press first host-aborts the open one (`captureLost`). */
export class Scrub<M> {
  private constructor(
    readonly runner: ToolMachineRunner<ScrubSpec<M>>,
    readonly baseRevision: string,
  ) {}

  static start<M>(tool: string, actor: ToolActor, baseRevision: string): Scrub<M> {
    const started = ToolMachineRunner.start(scrubMachine<M>(), tool, actor, { gesture: undefined, keys: 0 }, new ScrubHost<M>());
    if (!started.ok) throw new Error(started.refusal);
    return new Scrub(started.runner, baseRevision);
  }

  /** ⏯️ The scrub a window persisted; a state the chart cannot restore is refused (`closed`). */
  static resume<M>(state: ScrubState<M>): { readonly ok: true; readonly scrub: Scrub<M> } | { readonly ok: false; readonly refusal: ToolRefusal } {
    const machine = scrubMachine<M>();
    const restored = restore(machine, { version: 1, fingerprint: SCRUB_FINGERPRINT, states: state.states, history: [], done: false }, { gesture: state.gesture, keys: state.entries.length }, []);
    if (!restored.ok) return { ok: false, refusal: "toolTransaction.closed" };
    const resumed = ToolMachineRunner.resume(machine, state.tool, state.actor, { gesture: undefined, keys: 0 }, restored.snapshot, ToolTransaction.resume(state.transaction, state.entries), new ScrubHost<M>());
    return resumed.ok ? { ok: true, scrub: new Scrub(resumed.runner, state.baseRevision) } : resumed;
  }

  get gesture(): string | undefined {
    return this.runner.snapshot.context.gesture;
  }

  transaction(): ToolTransaction<M> | undefined {
    return this.runner.transaction();
  }

  send(input: ScrubInput<M>, clock: ToolClock): ToolStepResult<M> {
    if (input.kind === "abort") return { ok: true, step: this.runner.abort(input.reason) };
    if (this.gesture !== undefined && this.gesture !== input.gesture) this.runner.abort("captureLost");
    return this.runner.send(scrubEvent<M>(input.kind === "tick" ? "Tick" : "Commit", input.gesture, input.leaves), clock);
  }

  /** 💾️ The state to persist: defined only while a transaction is open. */
  persist(): ScrubState<M> | undefined {
    const [snapshot, transaction] = this.runner.intoParts();
    const gesture = snapshot.context.gesture;
    if (transaction?.state !== "open" || gesture === undefined) return undefined;
    return { states: persist(scrubMachine<M>(), snapshot).states, tool: this.runner.tool, actor: this.runner.actor, gesture, baseRevision: this.baseRevision, transaction: transaction.reference, entries: transaction.entries().map(([key, mutation]) => [key, mutation] as const) };
  }
}

/** 🗂️ Every window's open scrub plus the press each window last closed (twin of Rust `ScrubLedger<M>`). */
export class ScrubLedger<M> {
  readonly #windows = new Map<string, ScrubState<M>>();
  readonly #closed = new Map<string, string>();

  isEmpty(): boolean {
    return this.#windows.size === 0;
  }

  open(window: string): ScrubState<M> | undefined {
    return this.#windows.get(window);
  }

  /** 🪟️ The windows holding an open scrub, in window id order. */
  windows(): string[] {
    return [...this.#windows.keys()].sort();
  }

  /** 👁️ Every open scrub's provisional leaves, window by window in window id order — the render overlay. */
  provisional(): M[] {
    return this.windows().flatMap((window) => this.#windows.get(window)!.entries.map(([, leaf]) => leaf));
  }

  /** 📨️ Runs one input of `window`'s press: a late tick of the closed press is silent; another tool or document revision reopens on the current one. */
  send(window: string, tool: string, actor: ToolActor, baseRevision: string, input: ScrubInput<M>, clock: ToolClock): ToolStepResult<M> {
    if (input.kind === "abort") return { ok: true, step: this.abort(window, undefined, input.reason) };
    if (this.#closed.get(window) === input.gesture) return { ok: true, step: { kind: "idle" } };
    const state = this.#windows.get(window);
    this.#windows.delete(window);
    const resumed = state && state.tool === tool && state.baseRevision === baseRevision ? Scrub.resume(state) : undefined;
    const scrub = resumed?.ok ? resumed.scrub : Scrub.start<M>(tool, actor, baseRevision);
    const step = scrub.send(input, clock);
    if (input.kind === "commit") this.#closed.set(window, input.gesture);
    const persisted = scrub.persist();
    if (persisted) this.#windows.set(window, persisted);
    return step;
  }

  /** 🧯️ Host cancel of `window`'s open scrub (only of `gesture` when named): zero trace; the press is closed. */
  abort(window: string, gesture: string | undefined, reason: ToolAbortReason): ToolStep<M> {
    if (gesture !== undefined) this.#closed.set(window, gesture);
    const state = this.#windows.get(window);
    if (!state || (gesture !== undefined && gesture !== state.gesture)) return { kind: "idle" };
    this.#windows.delete(window);
    this.#closed.set(window, state.gesture);
    return { kind: "aborted", transaction: state.transaction, reason };
  }

  abortAll(reason: ToolAbortReason): ToolStep<M>[] {
    return this.windows().map((window) => this.abort(window, undefined, reason));
  }

  /** 🪦️ Host cancel (`retired`) of every window `keep` refuses; their closed presses are forgotten. */
  retainWindows(keep: (window: string) => boolean): ToolStep<M>[] {
    const dropped = this.windows().filter((window) => !keep(window)).map((window) => this.abort(window, undefined, "retired"));
    for (const window of [...this.#closed.keys()]) if (!keep(window)) this.#closed.delete(window);
    return dropped;
  }
}
//#endregion 🔖️Scrub
