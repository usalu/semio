/** 💾️ Statechart persistence: persisted snapshots, migrations, `persist` and `restore`. TypeScript twin of `💾️persist/🦀️.rs`. */

import { BitSet, type Machine, type MachineSpec, type NodeDef, NodeId, Snapshot } from "../🟦️.ts";

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
  /** 🔗️ No {@link Migration} bridges the persisted fingerprint to the current machine. */
  | { readonly kind: "fingerprintMismatch" }
  /** 📸️ A persisted stable id no longer exists in the current machine. */
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
