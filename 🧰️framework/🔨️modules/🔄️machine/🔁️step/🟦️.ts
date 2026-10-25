/** 🔁️ Statechart stepping: the step inspector and the `MachineStep` driver with `start` and `step`. TypeScript twin of `🔁️step/🦀️.rs`. */

import { type Command, type Machine, type MachineSpec, type NodeDef, NodeId, Snapshot, type StepReport } from "../🟦️.ts";
import type { InspectionEvent, Inspector } from "../🔎️inspect/🟦️.ts";
import { init, macrostep } from "../🧠️kernel/🟦️.ts";
import { type Migration, persist, type PersistedSnapshot, restore, type RestoreError } from "../💾️persist/🟦️.ts";

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

/** 🖥️ Everything a host needs to project one settled transition into its own state lanes.
 * `entered`/`exited` are the union across the macrostep's microsteps, in execution order — a node
 * touched twice within one macrostep appears twice, because that is what happened. `active` is the
 * settled configuration and is the field to project from. */
export class MachineStep<M extends MachineSpec> {
  readonly entered: readonly string[];
  readonly exited: readonly string[];
  readonly active: readonly string[];
  readonly commands: readonly Command<M>[];
  readonly report: StepReport;
  readonly persisted: PersistedSnapshot;

  constructor(
    entered: readonly string[],
    exited: readonly string[],
    active: readonly string[],
    commands: readonly Command<M>[],
    report: StepReport,
    persisted: PersistedSnapshot,
  ) {
    this.entered = entered;
    this.exited = exited;
    this.active = active;
    this.commands = commands;
    this.report = report;
    this.persisted = persisted;
  }

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

/** 🪜️ Restores from `prior`, runs one macrostep, and persists the result — the whole
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
