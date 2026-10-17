/** 🧠️ Statechart kernel: the SCXML-style microstep and macrostep algorithm over the compiled tables (domains, entry and exit sets, conflict resolution, run to completion). TypeScript twin of `🧠️kernel/🦀️.rs`. */

import { BitSet, type Command, type CommandSink, type Configuration, EventId, type Machine, type MachineDefinition, type MachineSpec, type NodeDef, NodeId, ROOT, Snapshot, type StepReport, TimerId, type TransitionDef, type TransitionKind } from "../🟦️.ts";
import { type Inspector, NullInspector } from "../🔎️inspect/🟦️.ts";

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
