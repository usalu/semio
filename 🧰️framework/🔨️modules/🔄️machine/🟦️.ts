//#region 🔢️Ids
/** 🔢 Dense index of a state node within a compiled {@link MachineDefinition}. */
export type NodeId = number & { readonly __machineId: "NodeId" };
export const NodeId = (value: number): NodeId => value as NodeId;

/** 🔢 Dense index of an event kind within a {@link StatechartEvent}'s variant space. */
export type EventId = number & { readonly __machineId: "EventId" };
export const EventId = (value: number): EventId => value as EventId;

/** 🔢 Dense index of a transition within a compiled {@link MachineDefinition}. */
export type TransitionId = number & { readonly __machineId: "TransitionId" };
export const TransitionId = (value: number): TransitionId => value as TransitionId;

/** 🔢 Dense index of a guard function within a compiled {@link MachineDefinition}. */
export type GuardId = number & { readonly __machineId: "GuardId" };
export const GuardId = (value: number): GuardId => value as GuardId;

/** 🔢 Dense index of a reducer/emitter action within a compiled {@link MachineDefinition}. */
export type ActionId = number & { readonly __machineId: "ActionId" };
export const ActionId = (value: number): ActionId => value as ActionId;

/** 🔢 Dense index of an `invoke` declaration within a compiled {@link MachineDefinition}. */
export type InvokeId = number & { readonly __machineId: "InvokeId" };
export const InvokeId = (value: number): InvokeId => value as InvokeId;

/** 🔢 Dense index of an `after` (delayed transition) timer within a compiled {@link MachineDefinition}. */
export type TimerId = number & { readonly __machineId: "TimerId" };
export const TimerId = (value: number): TimerId => value as TimerId;

/** 🔢 Runtime-assigned identity of a spawned actor. */
export type ActorId = number & { readonly __machineId: "ActorId" };
export const ActorId = (value: number): ActorId => value as ActorId;

/** 🔢 `NodeId(0)` is always the synthetic root wrapping the whole machine. */
export const ROOT: NodeId = NodeId(0);

//#endregion 🔢️Ids

//#region 🧮️Configuration
/** 🔁 Ascending iterator over the active {@link NodeId}s of a {@link Configuration}. */
export type ConfigurationIter = IterableIterator<NodeId>;

/** 🧩 Active-state configuration operations — the active configuration of a running machine is a
 * SET of atomic {@link NodeId}s, never a single value, so parallel regions can hold several
 * simultaneously-active states. TS twin of Rust's `Configuration` trait, implemented here by
 * {@link BitSet} rather than by a macro-sized `BitSet<const W: usize>` per machine (see {@link BitSet}). */
export interface Configuration {
  set(id: NodeId): void;
  clear(id: NodeId): void;
  contains(id: NodeId): boolean;
  iterOnes(): ConfigurationIter;
  clearAll(): void;
  isEmpty(): boolean;
  clone(): Configuration;
  equals(other: Configuration): boolean;
}

/** 🧮 Dynamically-sized bitset over `NodeId`s, backed by a `Set<number>`. Rust's `BitSet<const W:
 * usize>` is sized at compile time per machine by the `statechart!` macro; TS has no const generics,
 * so this twin collapses every machine's configuration to one dynamically-sized implementation. */
export class BitSet implements Configuration {
  #bits: Set<number>;

  constructor(bits: Iterable<number> = []) {
    this.#bits = new Set(bits);
  }

  set(id: NodeId): void {
    this.#bits.add(id);
  }

  clear(id: NodeId): void {
    this.#bits.delete(id);
  }

  contains(id: NodeId): boolean {
    return this.#bits.has(id);
  }

  *iterOnes(): ConfigurationIter {
    for (const id of [...this.#bits].sort((a, b) => a - b)) yield NodeId(id);
  }

  clearAll(): void {
    this.#bits.clear();
  }

  isEmpty(): boolean {
    return this.#bits.size === 0;
  }

  clone(): BitSet {
    return new BitSet(this.#bits);
  }

  equals(other: Configuration): boolean {
    if (!(other instanceof BitSet)) return false;
    if (other.#bits.size !== this.#bits.size) return false;
    for (const id of this.#bits) if (!other.#bits.has(id)) return false;
    return true;
  }
}

//#endregion 🧮️Configuration

//#region 🎭️Schema
/** 📨 A consumer-defined event, reflected into a dense {@link EventId} space. TS twin of Rust's
 * `#[derive(StatechartEvent)]`-generated impl — hand-implemented here since TS has no derive macros. */
export interface StatechartEvent {
  readonly eventCount: number;
  eventId(): EventId;
  eventName(id: EventId): string;
}

/** 🎭 Bundles a machine's associated types into one generic parameter — TS twin of Rust's `Machine`
 * associated types (`M::Context`, `M::Event`, …), accessed here as `M["Context"]`, `M["Event"]`, …
 * since TS has no associated-type syntax. */
export interface MachineSpec {
  readonly Context: unknown;
  readonly Event: StatechartEvent;
  readonly Input: unknown;
  readonly Output: unknown;
  readonly Effect: unknown;
}

/** 🎭 A compiled statechart: consumer-owned types bound to a static {@link MachineDefinition}. Rust's
 * `Machine` trait is implemented by a marker type whose `definition()` static method the kernel calls
 * implicitly (`M::definition()`); TS has no static dispatch over a type parameter, so every kernel
 * entry point below takes a `Machine<M>` value explicitly instead. */
export interface Machine<M extends MachineSpec> {
  readonly definition: MachineDefinition<M>;
}

/** 🧵 Reads context+event, decides whether a guarded transition may fire. Pure — no I/O, no mutation. */
export type GuardFn<M extends MachineSpec> = (context: M["Context"], event: M["Event"] | undefined) => boolean;

/** 🧵 Mutates context and/or pushes commands; used for entry/exit/transition actions alike. */
export type ActionFn<M extends MachineSpec> = (context: M["Context"], event: M["Event"] | undefined, sink: CommandSink<M>) => void;

/** 🧵 Builds the initial context from consumer-supplied input. */
export type InputFn<M extends MachineSpec> = (input: M["Input"]) => M["Context"];

/** 🧵 Builds the machine's output once the root reaches a fully-final configuration. */
export type OutputFn<M extends MachineSpec> = (context: M["Context"]) => M["Output"];

/** 📐 The compiled definition of a machine — dense tables, no string dispatch. */
export interface MachineDefinition<M extends MachineSpec> {
  readonly id: string;
  readonly nodes: readonly NodeDef[];
  readonly transitions: readonly TransitionDef[];
  readonly contextFromInput: InputFn<M>;
  readonly makeOutput?: OutputFn<M>;
  readonly guards: readonly GuardFn<M>[];
  readonly actions: readonly ActionFn<M>[];
  /** Stable hash of the compiled structure — used to gate {@link restore}. `bigint` (not `number`)
   * since Rust's `u64` fingerprint can exceed `Number.MAX_SAFE_INTEGER`. */
  readonly fingerprint: bigint;
  readonly manifestJson: string;
}

//#endregion 🎭️Schema

//#region 🌳️Tables
/** 🌳 The structural kind of a state node. */
export type NodeKind = "atomic" | "compound" | "parallel" | "final" | "historyShallow" | "historyDeep";

/** 🌳 One compiled state node. `NodeId(0)` is always the synthetic root, so domain computation
 * always terminates. */
export interface NodeDef {
  readonly stableId: string;
  readonly kind: NodeKind;
  readonly parent?: NodeId;
  /** Compound: default child to enter. History: fallback target when no history is recorded. */
  readonly initial?: NodeId;
  readonly children: readonly NodeId[];
  readonly entryActions: readonly ActionId[];
  readonly exitActions: readonly ActionId[];
  readonly invokes: readonly InvokeId[];
  /** `after` delayed transitions owned by this state: `[timer, delayMs]`. */
  readonly timers: readonly (readonly [TimerId, number])[];
  readonly docIndex: number;
}

/** 🔔 What causes a {@link TransitionDef} to become a candidate during a microstep. */
export type Trigger =
  | { readonly kind: "event"; readonly event: EventId }
  | { readonly kind: "eventless" }
  /** Fires once every descendant of `node` reaches a final state (`on_done`). */
  | { readonly kind: "done"; readonly node: NodeId }
  /** Fires when the named `after` timer elapses (delivered via {@link timerElapsed}). */
  | { readonly kind: "timer"; readonly timer: TimerId };

/** 🔀 External transitions exit+re-enter their source; internal transitions to a descendant of a
 * compound source leave the source itself active. */
export type TransitionKind = "external" | "internal";

/** 🔀 One compiled transition. Guard/action indices are dense — the tables they index into live on
 * {@link MachineDefinition}. */
export interface TransitionDef {
  readonly source: NodeId;
  readonly trigger: Trigger;
  readonly guard?: GuardId;
  readonly targets: readonly NodeId[];
  readonly kind: TransitionKind;
  readonly actions: readonly ActionId[];
  readonly docIndex: number;
}

//#endregion 🌳️Tables

//#region 🎇️Commands
/** 🎇 A declarative request the kernel produces but never executes — the {@link Host} does. */
export type Command<M extends MachineSpec> =
  | { readonly kind: "effect"; readonly effect: M["Effect"] }
  | { readonly kind: "raise"; readonly event: M["Event"] }
  | { readonly kind: "send"; readonly to: ActorId; readonly event: M["Event"] }
  | { readonly kind: "emit"; readonly output: M["Output"] }
  | { readonly kind: "startInvoke"; readonly invoke: InvokeId }
  | { readonly kind: "stopInvoke"; readonly invoke: InvokeId }
  | { readonly kind: "schedule"; readonly timer: TimerId; readonly delayMs: number }
  | { readonly kind: "cancelTimer"; readonly timer: TimerId };

/** 🎇 Where a running machine pushes the {@link Command}s it produces. A plain `Command<M>[]` already
 * satisfies this — `Array.prototype.push` structurally matches. */
export interface CommandSink<M extends MachineSpec> {
  push(command: Command<M>): void;
}

//#endregion 🎇️Commands

//#region 📸️Snapshot
/** 🏳 Whether a machine is still running, has produced an output, or was stopped by its host. */
export type Status<Output> = { readonly kind: "running" } | { readonly kind: "done"; readonly output: Output } | { readonly kind: "stopped" };

/** 📋 How many microsteps a macrostep took before settling. */
export interface StepReport {
  readonly microsteps: number;
}

/** 📸 A machine's runtime state: active configuration (never a single value — see {@link
 * Configuration}), consumer context, status, and private history slots for `history(...)` targets.
 * History is a true private field (`#history`) — Rust's `pub(crate)` has no direct TS equivalent, so
 * kernel functions in this same file mutate it only through {@link recordHistory}/{@link historyFor}. */
export class Snapshot<M extends MachineSpec> {
  configuration: Configuration;
  context: M["Context"];
  status: Status<M["Output"]>;
  readonly #nodes: readonly NodeDef[];
  readonly #history: Array<[NodeId, NodeId[]]> = [];

  constructor(nodes: readonly NodeDef[], configuration: Configuration, context: M["Context"], status: Status<M["Output"]> = { kind: "running" }) {
    this.#nodes = nodes;
    this.configuration = configuration;
    this.context = context;
    this.status = status;
  }

  /** 🔎 Whether the state with this stable id is part of the active configuration. */
  matches(stableId: string): boolean {
    for (const id of this.configuration.iterOnes()) if (this.#nodes[id]!.stableId === stableId) return true;
    return false;
  }

  /** 💾 The recorded history for a `history(...)` node, if any was ever captured. */
  historyFor(node: NodeId): readonly NodeId[] | undefined {
    return this.#history.find(([key]) => key === node)?.[1];
  }

  /** 💾 Records (or replaces) the history entry for a `history(...)` node's owner. */
  recordHistory(node: NodeId, value: readonly NodeId[]): void {
    const entry = this.#history.find(([key]) => key === node);
    if (entry) entry[1] = [...value];
    else this.#history.push([node, [...value]]);
  }

  /** 💾 Every recorded history entry, owner-node-id first. */
  historyEntries(): ReadonlyArray<readonly [NodeId, readonly NodeId[]]> {
    return this.#history;
  }

  /** 🧭 A fresh, independent copy for {@link explore}'s BFS frontier — configuration, context and
   * history are deep-copied and status always resets to `running`, matching Rust's `explore` seeding
   * every frontier entry off `Status::Running` regardless of the source snapshot's status. Requires
   * `M["Context"]` to be `structuredClone`-able (functions/class instances with private state are
   * not) — the TS analogue of Rust's `M::Context: Clone` bound, enforced at runtime here rather than
   * by the type system. */
  branchForExploration(): Snapshot<M> {
    const branch = new Snapshot<M>(this.#nodes, this.configuration.clone(), structuredClone(this.context), { kind: "running" });
    for (const [owner, ids] of this.#history) branch.recordHistory(owner, ids);
    return branch;
  }
}
//#endregion 📸️Snapshot
