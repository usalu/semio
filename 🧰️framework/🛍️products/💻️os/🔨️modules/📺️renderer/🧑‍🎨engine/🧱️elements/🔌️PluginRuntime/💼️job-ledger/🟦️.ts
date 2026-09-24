// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/PluginRuntime/job-ledger/module.ts
/** @emoji 💼️ The browser plugin runtime's ledger of LIVE spawned jobs — what `🧵️TaskManager` lists with its
 * progress and cancel control. `driveSpawnedJob` (`🔌️PluginRuntime/🟦️.tsx`) opens one entry per job it drives,
 * advances it once per admitted step batch and closes it when the drive ends, so the ledger holds exactly the
 * jobs this tab is stepping right now: ephemeral local state that never crosses a wire.
 *
 * A cancel is COOPERATIVE and goes through the job's own owner: the entry carries the drive's `cancel`, which
 * sends `cancelJob` to the guest; the guest releases the id and its next `step-job` answers `job.unknown`, which
 * the drive delivers as the job's `job-completed` failure. The ledger only records that a human asked
 * (`cancelling`), never that the job stopped — the entry disappears when the drive itself ends.
 * React-free, so its laws are tested without a worker (`🧑‍🎨engine/🧪️tests/💼️job-ledger`).
 */
// #endregion 🧲️Header

//#region 🔖️JobLedger
/** 💼️ One live spawned job as the task manager shows it. */
export interface SpawnedJobRowV1 {
  /** 🔑️ `${actorId}#${job}` — the drive's own dedupe key, unique while the job runs. */
  readonly key: string;
  readonly pluginId: string;
  readonly actorId: string;
  readonly instanceId: number;
  readonly job: bigint;
  /** 🏷️ The guest-declared job kind (`semio.io-run`, `semio.puzzle3d.fill`, …). */
  readonly kind: string;
  /** 🔢️ Step slices the drive has admitted so far — the one progress measure every job has. */
  readonly steps: number;
  /** 📈️ Whether the guest has reported progress bytes at least once. */
  readonly reportsProgress: boolean;
  readonly startedAtMs: number;
  readonly progressAtMs: number;
  readonly cancelling: boolean;
}

/** 🎛️ The drive's handle on its own ledger entry. */
export interface SpawnedJobLedgerEntryV1 {
  readonly step: (steps: number, progressed: boolean, atMs?: number) => void;
  readonly end: () => void;
}

type LedgerEntry = { row: SpawnedJobRowV1; readonly cancel: () => void };

/** ⏱️ Step updates are coalesced to at most one publish per this many milliseconds: a job admits step batches
 * far faster than a table can usefully repaint, while opening, cancelling and closing publish at once. */
export const SPAWNED_JOB_STEP_PUBLISH_MS = 250;

const ledger = new Map<string, LedgerEntry>();
const listeners = new Set<() => void>();
let snapshot: readonly SpawnedJobRowV1[] = [];
let pendingStepPublish: ReturnType<typeof setTimeout> | null = null;

function publish(): void {
  if (pendingStepPublish !== null) clearTimeout(pendingStepPublish);
  pendingStepPublish = null;
  snapshot = [...ledger.values()].map((entry) => entry.row).sort((left, right) => left.startedAtMs - right.startedAtMs || left.key.localeCompare(right.key));
  for (const listener of [...listeners]) listener();
}

function publishSoon(): void {
  if (pendingStepPublish === null) pendingStepPublish = setTimeout(publish, SPAWNED_JOB_STEP_PUBLISH_MS);
}

/** 💼️ Opens the ledger entry for one job drive. Re-opening a key that is still live replaces nothing and
 * returns a handle whose `end` is inert, so a duplicated `spawn-job` can never close the real drive's row. */
export function beginSpawnedJobV1(entry: Omit<SpawnedJobRowV1, "steps" | "reportsProgress" | "progressAtMs" | "cancelling"> & { readonly cancel: () => void }): SpawnedJobLedgerEntryV1 {
  if (ledger.has(entry.key)) return { step: () => undefined, end: () => undefined };
  const { cancel, ...row } = entry;
  const owned: LedgerEntry = { row: { ...row, steps: 0, reportsProgress: false, progressAtMs: row.startedAtMs, cancelling: false }, cancel };
  ledger.set(entry.key, owned);
  publish();
  return {
    step: (steps, progressed, atMs = Date.now()) => {
      if (ledger.get(entry.key) !== owned) return;
      owned.row = { ...owned.row, steps, reportsProgress: owned.row.reportsProgress || progressed, progressAtMs: atMs };
      publishSoon();
    },
    end: () => {
      if (ledger.get(entry.key) !== owned) return;
      ledger.delete(entry.key);
      publish();
    },
  };
}

/** 🛑️ Asks the job's own owner to stop it. `false` when no such job is live or it was already asked. */
export function cancelSpawnedJobV1(key: string): boolean {
  const entry = ledger.get(key);
  if (!entry || entry.row.cancelling) return false;
  entry.row = { ...entry.row, cancelling: true };
  publish();
  entry.cancel();
  return true;
}

/** 📸️ The live rows, oldest first. The same array identity until the ledger changes. */
export function spawnedJobsSnapshotV1(): readonly SpawnedJobRowV1[] {
  return snapshot;
}

/** 👂️ Called after every ledger change; returns its own unsubscribe. */
export function subscribeSpawnedJobsV1(listener: () => void): () => void {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

/** 🧪️ Test-only reset of the module-level ledger. */
export function resetSpawnedJobsForTestsV1(): void {
  if (pendingStepPublish !== null) clearTimeout(pendingStepPublish);
  pendingStepPublish = null;
  ledger.clear();
  listeners.clear();
  snapshot = [];
}
//#endregion 🔖️JobLedger
