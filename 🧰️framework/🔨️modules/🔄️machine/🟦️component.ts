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

//#region 🔎️Inspection
/** 🔎 One structured observation emitted while a macrostep runs to completion. */
export type InspectionEvent<M extends MachineSpec> =
  | { readonly kind: "macrostepStart" }
  | { readonly kind: "microstep"; readonly exited: readonly NodeId[]; readonly entered: readonly NodeId[] }
  | { readonly kind: "commandIssued"; readonly command: Command<M> }
  | { readonly kind: "settled"; readonly microsteps: number };

/** 🔎 Observer of {@link InspectionEvent}s — implemented by hosts/tooling that need microstep visibility. */
export interface Inspector<M extends MachineSpec> {
  observe(event: InspectionEvent<M>): void;
}

/** 🔎 An {@link Inspector} that discards every event — the default for callers that don't need tracing. */
export class NullInspector<M extends MachineSpec> implements Inspector<M> {
  observe(): void {}
}

/** 🔎 One recorded microstep — the exited/entered node sets, in kernel-execution order. */
export interface MicrostepTrace {
  readonly exited: readonly NodeId[];
  readonly entered: readonly NodeId[];
}

/** 🔎 An {@link Inspector} that records every microstep for later assertion/replay. */
export class TraceInspector<M extends MachineSpec> implements Inspector<M> {
  readonly entries: MicrostepTrace[] = [];

  observe(event: InspectionEvent<M>): void {
    if (event.kind === "microstep") this.entries.push({ exited: event.exited, entered: event.entered });
  }
}
//#endregion 🔎️Inspection

//#region 🧠️Kernel
/** 🧯 Safety cap against unguarded eventless transition cycles — a malformed machine hits this
 * instead of looping forever. */
export const MICROSTEP_LIMIT = 1000;

function isDescendant(nodes: readonly NodeDef[], a: NodeId, ancestor: NodeId): boolean {
  if (a === ancestor) return false;
  let cur = nodes[a]!.parent;
  while (cur !== undefined) {
    if (cur === ancestor) return true;
    cur = nodes[cur]!.parent;
  }
  return false;
}

function isDescendantOrSelf(nodes: readonly NodeDef[], a: NodeId, ancestor: NodeId): boolean {
  return a === ancestor || isDescendant(nodes, a, ancestor);
}

function depthOf(nodes: readonly NodeDef[], id: NodeId): number {
  let depth = 0;
  let cur = nodes[id]!.parent;
  while (cur !== undefined) {
    depth += 1;
    cur = nodes[cur]!.parent;
  }
  return depth;
}

function isCompoundOrParallel(nodes: readonly NodeDef[], id: NodeId): boolean {
  const kind = nodes[id]!.kind;
  return kind === "compound" || kind === "parallel";
}

function isLeafish(nodes: readonly NodeDef[], id: NodeId): boolean {
  const kind = nodes[id]!.kind;
  return kind === "atomic" || kind === "final";
}

/** 🌳 The transition domain per SCXML `getTransitionDomain` — the innermost compound/parallel
 * ancestor whose descendants fully cover source+targets. Always terminates at {@link ROOT}. */
function computeDomain(nodes: readonly NodeDef[], source: NodeId, targets: readonly NodeId[], kind: TransitionKind): NodeId {
  if (targets.length === 0) return source;
  if (kind === "internal" && isCompoundOrParallel(nodes, source) && targets.every((t) => isDescendant(nodes, t, source))) return source;
  let anc = nodes[source]!.parent;
  while (anc !== undefined) {
    if (isCompoundOrParallel(nodes, anc) && targets.every((t) => isDescendantOrSelf(nodes, t, anc!))) return anc;
    anc = nodes[anc]!.parent;
  }
  return ROOT;
}

function resolveEffectiveTargets<M extends MachineSpec>(nodes: readonly NodeDef[], targets: readonly NodeId[], snapshot: Snapshot<M>): NodeId[] {
  const out: NodeId[] = [];
  for (const t of targets) {
    const kind = nodes[t]!.kind;
    if (kind === "historyShallow" || kind === "historyDeep") {
      const recorded = snapshot.historyFor(t);
      if (recorded) {
        for (const r of recorded) if (!out.includes(r)) out.push(r);
      } else {
        const fallback = nodes[t]!.initial;
        if (fallback !== undefined && !out.includes(fallback)) out.push(fallback);
      }
    } else if (!out.includes(t)) {
      out.push(t);
    }
  }
  return out;
}

function addDescendantStatesToEnter<M extends MachineSpec>(nodes: readonly NodeDef[], state: NodeId, snapshot: Snapshot<M>, out: NodeId[]): void {
  const kind = nodes[state]!.kind;
  if (kind === "historyShallow" || kind === "historyDeep") {
    for (const r of resolveEffectiveTargets(nodes, [state], snapshot)) addDescendantStatesToEnter(nodes, r, snapshot, out);
    return;
  }
  if (!out.includes(state)) out.push(state);
  if (kind === "compound") {
    const initial = nodes[state]!.initial;
    if (initial !== undefined) {
      addDescendantStatesToEnter(nodes, initial, snapshot, out);
      addAncestorStatesToEnter(nodes, initial, state, snapshot, out);
    }
  } else if (kind === "parallel") {
    for (const child of nodes[state]!.children) {
      if (!out.some((e) => isDescendantOrSelf(nodes, e, child))) addDescendantStatesToEnter(nodes, child, snapshot, out);
    }
  }
}

function addAncestorStatesToEnter<M extends MachineSpec>(nodes: readonly NodeDef[], state: NodeId, stopAt: NodeId, snapshot: Snapshot<M>, out: NodeId[]): void {
  let anc = nodes[state]!.parent;
  while (anc !== undefined && anc !== stopAt) {
    if (!out.includes(anc)) out.push(anc);
    if (nodes[anc]!.kind === "parallel") {
      for (const child of nodes[anc]!.children) {
        if (!out.some((e) => isDescendantOrSelf(nodes, e, child))) addDescendantStatesToEnter(nodes, child, snapshot, out);
      }
    }
    anc = nodes[anc]!.parent;
  }
}

function stateDone(nodes: readonly NodeDef[], config: Configuration, node: NodeId): boolean {
  const kind = nodes[node]!.kind;
  if (kind === "final") return true;
  if (kind === "compound") {
    for (const child of nodes[node]!.children) if (config.contains(child)) return stateDone(nodes, config, child);
    return false;
  }
  if (kind === "parallel") return nodes[node]!.children.every((c) => stateDone(nodes, config, c));
  return false;
}

function computeDoneNodes(nodes: readonly NodeDef[], config: Configuration): NodeId[] {
  const out: NodeId[] = [];
  for (const id of config.iterOnes()) if (isCompoundOrParallel(nodes, id) && stateDone(nodes, config, id)) out.push(id);
  return out;
}

type Selector = { readonly kind: "event"; readonly event: EventId } | { readonly kind: "spontaneous" } | { readonly kind: "timer"; readonly timer: TimerId };

function candidatesFor<M extends MachineSpec>(definition: MachineDefinition<M>, config: Configuration, context: M["Context"], event: M["Event"] | undefined, selector: Selector, done: readonly NodeId[]): number[] {
  const out: number[] = [];
  definition.transitions.forEach((t, i) => {
    if (!config.contains(t.source)) return;
    const matchesTrigger =
      (selector.kind === "event" && t.trigger.kind === "event" && t.trigger.event === selector.event) ||
      (selector.kind === "spontaneous" && (t.trigger.kind === "eventless" || (t.trigger.kind === "done" && done.includes(t.trigger.node)))) ||
      (selector.kind === "timer" && t.trigger.kind === "timer" && t.trigger.timer === selector.timer);
    if (!matchesTrigger) return;
    if (t.guard !== undefined && !definition.guards[t.guard]!(context, event)) return;
    out.push(i);
  });
  return out;
}

/** 🥊 Keeps the deepest-source transition when two candidates' exit domains overlap (child
 * preemption); ties keep document order. */
function resolveConflicts(nodes: readonly NodeDef[], transitions: readonly TransitionDef[], candidates: readonly number[]): number[] {
  const sorted = [...candidates].sort((a, b) => transitions[a]!.docIndex - transitions[b]!.docIndex);
  const selected: number[] = [];
  outer: for (const cand of sorted) {
    const candDomain = computeDomain(nodes, transitions[cand]!.source, transitions[cand]!.targets, transitions[cand]!.kind);
    const toRemove: number[] = [];
    for (let i = 0; i < selected.length; i += 1) {
      const sel = selected[i]!;
      const selDomain = computeDomain(nodes, transitions[sel]!.source, transitions[sel]!.targets, transitions[sel]!.kind);
      if (isDescendantOrSelf(nodes, candDomain, selDomain) || isDescendantOrSelf(nodes, selDomain, candDomain)) {
        if (depthOf(nodes, transitions[cand]!.source) > depthOf(nodes, transitions[sel]!.source)) toRemove.push(i);
        else continue outer;
      }
    }
    for (let i = toRemove.length - 1; i >= 0; i -= 1) selected.splice(toRemove[i]!, 1);
    selected.push(cand);
  }
  return selected;
}

function applyTransitions<M extends MachineSpec>(definition: MachineDefinition<M>, snapshot: Snapshot<M>, transitionsIdx: readonly number[], event: M["Event"] | undefined, sink: CommandSink<M>, inspector: Inspector<M>): void {
  const nodes = definition.nodes;
  const exitIds: NodeId[] = [];
  for (const ti of transitionsIdx) {
    const t = definition.transitions[ti]!;
    const domain = computeDomain(nodes, t.source, t.targets, t.kind);
    for (const id of snapshot.configuration.iterOnes()) if (isDescendant(nodes, id, domain) && !exitIds.includes(id)) exitIds.push(id);
  }
  exitIds.sort((a, b) => depthOf(nodes, b) - depthOf(nodes, a));

  for (const owner of exitIds) {
    for (const child of nodes[owner]!.children) {
      const childKind = nodes[child]!.kind;
      if (childKind === "historyShallow") {
        const activeChild = nodes[owner]!.children.find((c) => snapshot.configuration.contains(c) && nodes[c]!.kind !== "historyShallow" && nodes[c]!.kind !== "historyDeep");
        if (activeChild !== undefined) snapshot.recordHistory(child, [activeChild]);
      } else if (childKind === "historyDeep") {
        const leaves: NodeId[] = [];
        for (const id of snapshot.configuration.iterOnes()) if (isDescendant(nodes, id, owner) && isLeafish(nodes, id)) leaves.push(id);
        snapshot.recordHistory(child, leaves);
      }
    }
  }

  for (const id of exitIds) {
    for (const actionId of nodes[id]!.exitActions) definition.actions[actionId]!(snapshot.context, event, sink);
    for (const [timerId] of nodes[id]!.timers) sink.push({ kind: "cancelTimer", timer: timerId });
    for (const invokeId of nodes[id]!.invokes) sink.push({ kind: "stopInvoke", invoke: invokeId });
    snapshot.configuration.clear(id);
  }

  for (const ti of transitionsIdx) {
    const t = definition.transitions[ti]!;
    for (const actionId of t.actions) definition.actions[actionId]!(snapshot.context, event, sink);
  }

  const entryIds: NodeId[] = [];
  for (const ti of transitionsIdx) {
    const t = definition.transitions[ti]!;
    const domain = computeDomain(nodes, t.source, t.targets, t.kind);
    const effectiveTargets = resolveEffectiveTargets(nodes, t.targets, snapshot);
    for (const target of effectiveTargets) addDescendantStatesToEnter(nodes, target, snapshot, entryIds);
    for (const target of effectiveTargets) addAncestorStatesToEnter(nodes, target, domain, snapshot, entryIds);
  }
  entryIds.sort((a, b) => depthOf(nodes, a) - depthOf(nodes, b));

  for (const id of entryIds) {
    snapshot.configuration.set(id);
    for (const actionId of nodes[id]!.entryActions) definition.actions[actionId]!(snapshot.context, event, sink);
    for (const [timerId, delayMs] of nodes[id]!.timers) sink.push({ kind: "schedule", timer: timerId, delayMs });
    for (const invokeId of nodes[id]!.invokes) sink.push({ kind: "startInvoke", invoke: invokeId });
  }

  inspector.observe({ kind: "microstep", exited: exitIds, entered: entryIds });
}

function finalizeStatus<M extends MachineSpec>(definition: MachineDefinition<M>, snapshot: Snapshot<M>): void {
  if (snapshot.status.kind === "done") return;
  if (stateDone(definition.nodes, snapshot.configuration, ROOT) && definition.makeOutput) {
    snapshot.status = { kind: "done", output: definition.makeOutput(snapshot.context) };
  }
}

type RaisedSelector = { readonly kind: "event"; readonly event: EventId } | { readonly kind: "timer"; readonly timer: TimerId };

interface ActiveTrigger<M extends MachineSpec> {
  readonly selector: RaisedSelector;
  readonly event?: M["Event"];
}

function runToCompletion<M extends MachineSpec>(definition: MachineDefinition<M>, snapshot: Snapshot<M>, seed: ActiveTrigger<M> | undefined, sink: CommandSink<M>, inspector: Inspector<M>): StepReport {
  inspector.observe({ kind: "macrostepStart" });
  const queue: ActiveTrigger<M>[] = seed ? [seed] : [];
  let microsteps = 0;
  for (;;) {
    if (microsteps >= MICROSTEP_LIMIT) break;
    let selected: number[];
    let eventOwned: M["Event"] | undefined;
    const trigger = queue.shift();
    if (trigger) {
      const done = computeDoneNodes(definition.nodes, snapshot.configuration);
      const selector: Selector = trigger.selector.kind === "event" ? { kind: "event", event: trigger.selector.event } : { kind: "timer", timer: trigger.selector.timer };
      selected = candidatesFor(definition, snapshot.configuration, snapshot.context, trigger.event, selector, done);
      eventOwned = trigger.event;
    } else {
      const done = computeDoneNodes(definition.nodes, snapshot.configuration);
      const spontaneous = candidatesFor(definition, snapshot.configuration, snapshot.context, undefined, { kind: "spontaneous" }, done);
      if (spontaneous.length === 0) break;
      selected = spontaneous;
      eventOwned = undefined;
    }
    if (selected.length === 0) continue;
    const resolved = resolveConflicts(definition.nodes, definition.transitions, selected);
    microsteps += 1;
    const local: Command<M>[] = [];
    applyTransitions(definition, snapshot, resolved, eventOwned, local, inspector);
    for (const command of local) {
      if (command.kind === "raise") queue.push({ selector: { kind: "event", event: command.event.eventId() }, event: command.event });
      inspector.observe({ kind: "commandIssued", command });
      sink.push(command);
    }
  }
  finalizeStatus(definition, snapshot);
  inspector.observe({ kind: "settled", microsteps });
  return { microsteps };
}

/** 🚀 Builds a fresh {@link Snapshot} from `input`, entering the root's default descendant chain and
 * settling any eventless/done transitions enabled immediately on init. */
export function init<M extends MachineSpec>(machine: Machine<M>, input: M["Input"], sink: CommandSink<M>): Snapshot<M> {
  const definition = machine.definition;
  const snapshot = new Snapshot<M>(definition.nodes, new BitSet(), definition.contextFromInput(input));
  const entryIds: NodeId[] = [];
  addDescendantStatesToEnter(definition.nodes, ROOT, snapshot, entryIds);
  entryIds.sort((a, b) => depthOf(definition.nodes, a) - depthOf(definition.nodes, b));
  for (const id of entryIds) {
    snapshot.configuration.set(id);
    for (const actionId of definition.nodes[id]!.entryActions) definition.actions[actionId]!(snapshot.context, undefined, sink);
    for (const [timerId, delayMs] of definition.nodes[id]!.timers) sink.push({ kind: "schedule", timer: timerId, delayMs });
    for (const invokeId of definition.nodes[id]!.invokes) sink.push({ kind: "startInvoke", invoke: invokeId });
  }
  runToCompletion(definition, snapshot, undefined, sink, new NullInspector());
  return snapshot;
}

/** 🏃 Runs one external event to completion (a "macrostep"): the triggered microstep, then every
 * enabled eventless/`on_done` microstep, until the configuration settles. */
export function macrostep<M extends MachineSpec>(machine: Machine<M>, snapshot: Snapshot<M>, event: M["Event"], sink: CommandSink<M>, inspector: Inspector<M>): StepReport {
  return runToCompletion(machine.definition, snapshot, { selector: { kind: "event", event: event.eventId() }, event }, sink, inspector);
}

/** ⏱ Runs an `after`-timer firing to completion — the entry point when a {@link Host} reports a
 * scheduled {@link TimerId} elapsed. */
export function timerElapsed<M extends MachineSpec>(machine: Machine<M>, snapshot: Snapshot<M>, timer: TimerId, sink: CommandSink<M>, inspector: Inspector<M>): StepReport {
  return runToCompletion(machine.definition, snapshot, { selector: { kind: "timer", timer } }, sink, inspector);
}
//#endregion 🧠️Kernel

//#region 🌐️Host
/** 🌐 Executes the side effects a {@link Command} describes. No async method — hosts own their own
 * tasks/timers and report completion back as ordinary events. */
export interface Host<M extends MachineSpec> {
  /** 🎇 Executes a consumer-defined effect requested by a running actor. */
  executeEffect(actor: ActorId, effect: M["Effect"]): void;
  /** ⏱ Schedules a delayed-transition timer for the given actor. */
  schedule(actor: ActorId, timer: TimerId, delayMs: number): void;
  /** ⏱ Cancels a previously scheduled timer (invoked when its owning state exits). */
  cancelTimer(actor: ActorId, timer: TimerId): void;
  /** 🚀 Starts the task/actor backing an `invoke` declaration. */
  startTask(actor: ActorId, invoke: InvokeId): void;
  /** 🛑 Stops a previously started task (invoked when its owning state exits). */
  cancelTask(actor: ActorId, invoke: InvokeId): void;
  /** 🕰 The host's current clock reading, in milliseconds. */
  nowMs(): number;
}

/** 🖥 A synchronous, wall-clock-backed {@link Host} for native (non-browser-driven) callers. Timers
 * are polled by the caller via {@link NativeHost.dueTimers} rather than firing on their own — keeping
 * the whole runtime single-threaded per actor, same as Rust's `NativeHost`. */
export class NativeHost<M extends MachineSpec> implements Host<M> {
  readonly #start = Date.now();
  readonly #effects: Array<[ActorId, M["Effect"]]> = [];
  readonly #pendingTimers: Array<[ActorId, TimerId, number]> = [];
  readonly #startedTasks: Array<[ActorId, InvokeId]> = [];

  /** 🎇 Effects recorded so far, in emission order. */
  effects(): ReadonlyArray<readonly [ActorId, M["Effect"]]> {
    return this.#effects;
  }

  /** 🎇 Drains and returns every recorded effect. */
  drainEffects(): Array<[ActorId, M["Effect"]]> {
    return this.#effects.splice(0, this.#effects.length);
  }

  /** 🚀 Tasks started via `invoke`, still pending cancellation. */
  startedTasks(): ReadonlyArray<readonly [ActorId, InvokeId]> {
    return this.#startedTasks;
  }

  /** ⏱ Removes and returns every timer whose deadline has passed. */
  dueTimers(): Array<[ActorId, TimerId]> {
    const now = this.nowMs();
    const due: Array<[ActorId, TimerId]> = [];
    const remaining = this.#pendingTimers.filter(([actor, timer, at]) => {
      if (at > now) return true;
      due.push([actor, timer]);
      return false;
    });
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...remaining);
    return due;
  }

  executeEffect(actor: ActorId, effect: M["Effect"]): void {
    this.#effects.push([actor, effect]);
  }

  schedule(actor: ActorId, timer: TimerId, delayMs: number): void {
    this.#pendingTimers.push([actor, timer, this.nowMs() + delayMs]);
  }

  cancelTimer(actor: ActorId, timer: TimerId): void {
    const kept = this.#pendingTimers.filter(([a, t]) => !(a === actor && t === timer));
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...kept);
  }

  startTask(actor: ActorId, invoke: InvokeId): void {
    this.#startedTasks.push([actor, invoke]);
  }

  cancelTask(actor: ActorId, invoke: InvokeId): void {
    const kept = this.#startedTasks.filter(([a, i]) => !(a === actor && i === invoke));
    this.#startedTasks.length = 0;
    this.#startedTasks.push(...kept);
  }

  nowMs(): number {
    return Date.now() - this.#start;
  }
}

/** 🧪 A {@link Host} with a caller-driven simulated clock — never sleeps in real time. */
export class TestHost<M extends MachineSpec> implements Host<M> {
  #clockMs = 0;
  readonly #effects: Array<[ActorId, M["Effect"]]> = [];
  readonly #pendingTimers: Array<[ActorId, TimerId, number]> = [];
  readonly #startedTasks: Array<[ActorId, InvokeId]> = [];
  readonly #cancelledTasks: Array<[ActorId, InvokeId]> = [];

  /** 🎇 Effects recorded so far, in emission order. */
  effects(): ReadonlyArray<readonly [ActorId, M["Effect"]]> {
    return this.#effects;
  }

  /** 🚀 Tasks currently started (not yet cancelled), for invoke-lifecycle assertions. */
  startedTasks(): ReadonlyArray<readonly [ActorId, InvokeId]> {
    return this.#startedTasks;
  }

  /** 🛑 Tasks that have been cancelled, for invoke-lifecycle assertions. */
  cancelledTasks(): ReadonlyArray<readonly [ActorId, InvokeId]> {
    return this.#cancelledTasks;
  }

  /** ⏱ Advances the simulated clock and returns timers that became due, removing them. */
  advance(delayMs: number): Array<[ActorId, TimerId]> {
    this.#clockMs += delayMs;
    const now = this.#clockMs;
    const due: Array<[ActorId, TimerId]> = [];
    const remaining = this.#pendingTimers.filter(([actor, timer, at]) => {
      if (at > now) return true;
      due.push([actor, timer]);
      return false;
    });
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...remaining);
    return due;
  }

  executeEffect(actor: ActorId, effect: M["Effect"]): void {
    this.#effects.push([actor, effect]);
  }

  schedule(actor: ActorId, timer: TimerId, delayMs: number): void {
    this.#pendingTimers.push([actor, timer, this.#clockMs + delayMs]);
  }

  cancelTimer(actor: ActorId, timer: TimerId): void {
    const kept = this.#pendingTimers.filter(([a, t]) => !(a === actor && t === timer));
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...kept);
  }

  startTask(actor: ActorId, invoke: InvokeId): void {
    this.#startedTasks.push([actor, invoke]);
  }

  cancelTask(actor: ActorId, invoke: InvokeId): void {
    const kept = this.#startedTasks.filter(([a, i]) => !(a === actor && i === invoke));
    this.#startedTasks.length = 0;
    this.#startedTasks.push(...kept);
    this.#cancelledTasks.push([actor, invoke]);
  }

  nowMs(): number {
    return this.#clockMs;
  }
}
//#endregion 🌐️Host

//#region 💾️Persist
/** 💾 A machine's logical state, addressed by stable string ids so adding/renumbering states never
 * invalidates a previously persisted snapshot. */
export interface PersistedSnapshot {
  readonly version: number;
  readonly fingerprint: bigint;
  readonly states: readonly string[];
  readonly history: readonly (readonly [string, readonly string[]])[];
  readonly done: boolean;
}

/** 💾 Why {@link restore} could not rebuild a {@link Snapshot}. */
export type RestoreError =
  /** No {@link Migration} bridges the persisted fingerprint to the current machine. */
  | { readonly kind: "fingerprintMismatch" }
  /** A persisted stable id no longer exists in the current machine. */
  | { readonly kind: "unknownStableId"; readonly stableId: string };

/** 💾 Migrates a {@link PersistedSnapshot} captured under an older machine fingerprint. */
export interface Migration {
  readonly sourceFingerprint: bigint;
  migrate(snapshot: PersistedSnapshot): PersistedSnapshot;
}

/** 💾 Captures a running {@link Snapshot} as a portable, stable-id-addressed value. */
export function persist<M extends MachineSpec>(machine: Machine<M>, snapshot: Snapshot<M>): PersistedSnapshot {
  const nodes = machine.definition.nodes;
  const states = [...snapshot.configuration.iterOnes()].map((id) => nodes[id]!.stableId);
  const history = snapshot.historyEntries().map(([owner, ids]) => [nodes[owner]!.stableId, ids.map((id) => nodes[id]!.stableId)] as const);
  return { version: 1, fingerprint: machine.definition.fingerprint, states, history, done: snapshot.status.kind === "done" };
}

function stableIdToNode(nodes: readonly NodeDef[], stableId: string): NodeId | undefined {
  const idx = nodes.findIndex((n) => n.stableId === stableId);
  return idx === -1 ? undefined : NodeId(idx);
}

/** 💾 Rebuilds a {@link Snapshot} from a {@link PersistedSnapshot}, applying `migrations` in sequence
 * until the fingerprint matches the current machine, then re-resolving stable ids back to dense
 * {@link NodeId}s. `context` is supplied by the caller since the consumer's context may itself need
 * domain-specific deserialization. */
export function restore<M extends MachineSpec>(
  machine: Machine<M>,
  persisted: PersistedSnapshot,
  context: M["Context"],
  migrations: readonly Migration[],
): { readonly ok: true; readonly snapshot: Snapshot<M> } | { readonly ok: false; readonly error: RestoreError } {
  const definition = machine.definition;
  let current = persisted;
  while (current.fingerprint !== definition.fingerprint) {
    const next = migrations.find((m) => m.sourceFingerprint === current.fingerprint);
    if (!next) return { ok: false, error: { kind: "fingerprintMismatch" } };
    current = next.migrate(current);
  }
  const configuration = new BitSet();
  for (const stableId of current.states) {
    const id = stableIdToNode(definition.nodes, stableId);
    if (id === undefined) return { ok: false, error: { kind: "unknownStableId", stableId } };
    configuration.set(id);
  }
  const snapshot = new Snapshot<M>(definition.nodes, configuration, context, { kind: "running" });
  for (const [ownerStableId, ids] of current.history) {
    const owner = stableIdToNode(definition.nodes, ownerStableId);
    if (owner === undefined) return { ok: false, error: { kind: "unknownStableId", stableId: ownerStableId } };
    const resolved: NodeId[] = [];
    for (const stableId of ids) {
      const id = stableIdToNode(definition.nodes, stableId);
      if (id === undefined) return { ok: false, error: { kind: "unknownStableId", stableId } };
      resolved.push(id);
    }
    snapshot.recordHistory(owner, resolved);
  }
  return { ok: true, snapshot };
}
//#endregion 💾️Persist

//#region 🎬️Runtime
class Actor<M extends MachineSpec> {
  readonly id: ActorId;
  snapshot: Snapshot<M>;
  readonly mailbox: M["Event"][] = [];

  constructor(id: ActorId, snapshot: Snapshot<M>) {
    this.id = id;
    this.snapshot = snapshot;
  }
}

/** 🎬 Owns every spawned actor for one machine type and routes their {@link Command}s to a {@link
 * Host}. Mailboxes drain in round-robin order until quiescent. Rust's `ActorLogic`/`MachineLogic`
 * (a marker trait + its blanket impl, abstracting the actor's associated types) has no TS
 * counterpart here — {@link MachineSpec} already supplies those types structurally, so the marker
 * layer collapses to nothing. */
export class ActorSystem<M extends MachineSpec> {
  readonly host: Host<M>;
  readonly #machine: Machine<M>;
  readonly #actors: Actor<M>[] = [];
  #nextId = 0;

  constructor(host: Host<M>, machine: Machine<M>) {
    this.host = host;
    this.#machine = machine;
  }

  /** 🎬 Initializes and registers a root actor, routing its initial commands immediately. */
  spawnRoot(input: M["Input"]): ActorId {
    const id = ActorId(this.#nextId);
    this.#nextId += 1;
    const buffer: Command<M>[] = [];
    const snapshot = init(this.#machine, input, buffer);
    this.#actors.push(new Actor(id, snapshot));
    this.#routeCommands(id, buffer);
    return id;
  }

  /** 🎬 The current {@link Snapshot} of an actor, if it exists. */
  snapshot(id: ActorId): Snapshot<M> | undefined {
    return this.#actors.find((a) => a.id === id)?.snapshot;
  }

  /** 🎬 Enqueues an event for delivery on the next {@link ActorSystem.drain}. */
  send(to: ActorId, event: M["Event"]): void {
    this.#actors.find((a) => a.id === to)?.mailbox.push(event);
  }

  /** 🎬 Delivers a {@link TimerId} elapsed notification straight to `macrostep`'s timer entry point for `to`. */
  timerElapsed(to: ActorId, timer: TimerId): StepReport | undefined {
    const actor = this.#actors.find((a) => a.id === to);
    if (!actor) return undefined;
    const buffer: Command<M>[] = [];
    const report = timerElapsed(this.#machine, actor.snapshot, timer, buffer, new NullInspector());
    this.#routeCommands(to, buffer);
    return report;
  }

  /** 🎬 Drains every actor's mailbox to quiescence, running one macrostep per delivered event. */
  drain(): StepReport[] {
    const reports: StepReport[] = [];
    for (;;) {
      let progressed = false;
      for (const actor of this.#actors) {
        const event = actor.mailbox.shift();
        if (event === undefined) continue;
        progressed = true;
        const buffer: Command<M>[] = [];
        const report = macrostep(this.#machine, actor.snapshot, event, buffer, new NullInspector());
        this.#routeCommands(actor.id, buffer);
        reports.push(report);
      }
      if (!progressed) break;
    }
    return reports;
  }

  #routeCommands(actor: ActorId, commands: readonly Command<M>[]): void {
    const sends: Array<readonly [ActorId, M["Event"]]> = [];
    const found = this.#actors.find((a) => a.id === actor);
    if (found) {
      for (const command of commands) {
        const pair = routeCommand(this.host, found.snapshot, actor, command);
        if (pair) sends.push(pair);
      }
    }
    for (const [to, event] of sends) this.send(to, event);
  }
}

/** 🎬 Applies one {@link Command} to `host`/`snapshot`; returns a `send` command's `[to, event]` pair
 * for the caller to route on, since a lone {@link Host}+{@link Snapshot} pair (no {@link ActorSystem})
 * has no other actor to deliver it to. */
export function routeCommand<M extends MachineSpec>(host: Host<M>, snapshot: Snapshot<M>, actor: ActorId, command: Command<M>): readonly [ActorId, M["Event"]] | undefined {
  switch (command.kind) {
    case "effect":
      host.executeEffect(actor, command.effect);
      return undefined;
    case "raise":
      return undefined;
    case "send":
      return [command.to, command.event];
    case "emit":
      snapshot.status = { kind: "done", output: command.output };
      return undefined;
    case "startInvoke":
      host.startTask(actor, command.invoke);
      return undefined;
    case "stopInvoke":
      host.cancelTask(actor, command.invoke);
      return undefined;
    case "schedule":
      host.schedule(actor, command.timer, command.delayMs);
      return undefined;
    case "cancelTimer":
      host.cancelTimer(actor, command.timer);
      return undefined;
  }
}
//#endregion 🎬️Runtime

//#region 🔁️Step
class StepInspector<M extends MachineSpec> implements Inspector<M> {
  readonly entered: NodeId[] = [];
  readonly exited: NodeId[] = [];

  observe(event: InspectionEvent<M>): void {
    if (event.kind === "microstep") {
      this.exited.push(...event.exited);
      this.entered.push(...event.entered);
    }
  }
}

/** 🔁 Everything a host needs to project one settled transition into its own state lanes.
 * `entered`/`exited` are the union across the macrostep's microsteps, in execution order — a node
 * touched twice within one macrostep appears twice, because that is what happened. `active` is the
 * settled configuration and is the field to project from. */
export class MachineStep<M extends MachineSpec> {
  constructor(
    readonly entered: readonly string[],
    readonly exited: readonly string[],
    readonly active: readonly string[],
    readonly commands: readonly Command<M>[],
    readonly report: StepReport,
    readonly persisted: PersistedSnapshot,
  ) {}

  /** 🔎 Whether the settled configuration contains the state with this stable id. */
  isActive(stableId: string): boolean {
    return this.active.includes(stableId);
  }
}

function stableIds(nodes: readonly NodeDef[], ids: readonly NodeId[]): string[] {
  return ids.map((id) => nodes[id]!.stableId);
}

function machineStepOf<M extends MachineSpec>(machine: Machine<M>, snapshot: Snapshot<M>, entered: readonly NodeId[], exited: readonly NodeId[], commands: readonly Command<M>[], report: StepReport): MachineStep<M> {
  const nodes = machine.definition.nodes;
  return new MachineStep(stableIds(nodes, entered), stableIds(nodes, exited), stableIds(nodes, [...snapshot.configuration.iterOnes()]), commands, report, persist(machine, snapshot));
}

/** 🌱 Builds the initial configuration — the host's first call, when its lane holds no {@link
 * PersistedSnapshot} yet. */
export function start<M extends MachineSpec>(machine: Machine<M>, input: M["Input"]): MachineStep<M> {
  const commands: Command<M>[] = [];
  const snapshot = init(machine, input, commands);
  return machineStepOf(machine, snapshot, [], [], commands, { microsteps: 0 });
}

/** 🔁 Restores from `prior`, runs one macrostep, and persists the result — the whole
 * read-transition-write cycle, with the live {@link Snapshot} confined to this call. */
export function step<M extends MachineSpec>(
  machine: Machine<M>,
  prior: PersistedSnapshot,
  context: M["Context"],
  event: M["Event"],
  migrations: readonly Migration[],
): { readonly ok: true; readonly step: MachineStep<M> } | { readonly ok: false; readonly error: RestoreError } {
  const restored = restore(machine, prior, context, migrations);
  if (!restored.ok) return restored;
  const commands: Command<M>[] = [];
  const inspector = new StepInspector<M>();
  const report = macrostep(machine, restored.snapshot, event, commands, inspector);
  return { ok: true, step: machineStepOf(machine, restored.snapshot, inspector.entered, inspector.exited, commands, report) };
}
//#endregion 🔁️Step

//#region 🧭️Testing
/** 🧭 A set of representative events tried from every reachable configuration. */
export class Model<M extends MachineSpec> {
  constructor(readonly events: readonly M["Event"][]) {}
}

/** 🧭 What a BFS {@link explore} found: distinct configurations visited and every stable state id
 * reached across them. */
export interface Coverage {
  readonly visitedConfigurations: number;
  readonly reachedStableIds: readonly string[];
}

function activeStableIds<M extends MachineSpec>(nodes: readonly NodeDef[], snapshot: Snapshot<M>): string[] {
  return [...snapshot.configuration.iterOnes()].map((id) => nodes[id]!.stableId);
}

/** 🧭 Breadth-first walk over reachable configurations, trying every event in `model` from each
 * newly-discovered configuration. Approximates reachability by configuration only — guard outcomes
 * that depend on context may under-approximate. */
export function explore<M extends MachineSpec>(machine: Machine<M>, model: Model<M>, input: M["Input"]): Coverage {
  const nodes = machine.definition.nodes;
  const root = init(machine, input, []);
  const visited: Configuration[] = [];
  const frontier: Snapshot<M>[] = [root];
  const reachedIds: string[] = [];

  let snapshot = frontier.pop();
  while (snapshot) {
    if (visited.some((v) => v.equals(snapshot!.configuration))) {
      snapshot = frontier.pop();
      continue;
    }
    for (const stable of activeStableIds(nodes, snapshot)) if (!reachedIds.includes(stable)) reachedIds.push(stable);
    visited.push(snapshot.configuration.clone());

    for (const event of model.events) {
      const next = snapshot.branchForExploration();
      macrostep(machine, next, event, [], new NullInspector());
      frontier.push(next);
    }
    snapshot = frontier.pop();
  }

  return { visitedConfigurations: visited.length, reachedStableIds: reachedIds };
}

/** ⚠️ Why an {@link Invariant} check or {@link runConformance} fixture failed. */
export type FsmError = { readonly kind: "violation"; readonly message: string };

/** 🧭 A named property that must hold of every {@link Snapshot} visited during exploration. */
export interface Invariant<M extends MachineSpec> {
  readonly name: string;
  readonly check: (snapshot: Snapshot<M>) => { readonly ok: true } | { readonly ok: false; readonly error: FsmError };
}

/** 🧭 Runs every invariant against `snapshot`, returning one formatted message per violation. */
export function checkInvariants<M extends MachineSpec>(snapshot: Snapshot<M>, invariants: readonly Invariant<M>[]): readonly string[] {
  const violations: string[] = [];
  for (const invariant of invariants) {
    const result = invariant.check(snapshot);
    if (!result.ok) violations.push(`${invariant.name}: ${result.error.message}`);
  }
  return violations;
}

/** 🧭 One step of an inline conformance fixture: send `event`, then assert every stable id in
 * `expectActive` is part of the settled configuration. */
export interface ConformanceStep<M extends MachineSpec> {
  readonly event: M["Event"];
  readonly expectActive: readonly string[];
}

/** 🧭 Runs `steps` against a freshly-initialized machine, failing fast with a descriptive message
 * naming the offending step and the actual active configuration. */
export function runConformance<M extends MachineSpec>(machine: Machine<M>, input: M["Input"], steps: readonly ConformanceStep<M>[]): { readonly ok: true } | { readonly ok: false; readonly error: FsmError } {
  const nodes = machine.definition.nodes;
  const sink: Command<M>[] = [];
  const snapshot = init(machine, input, sink);
  for (let index = 0; index < steps.length; index += 1) {
    const step = steps[index]!;
    macrostep(machine, snapshot, step.event, sink, new NullInspector());
    for (const expected of step.expectActive) {
      if (!snapshot.matches(expected)) {
        return { ok: false, error: { kind: "violation", message: `conformance step ${index}: expected active state '${expected}', got ${JSON.stringify(activeStableIds(nodes, snapshot))}` } };
      }
    }
  }
  return { ok: true };
}
//#endregion 🧭️Testing
