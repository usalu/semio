import { terminateOwnedProcessTree } from "../../../🏃️process/🟦️.ts";
import { appendFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { spawn, spawnSync } from "node:child_process";
import { join } from "node:path";

export const TRANSACTION_V2_DEFAULT_FILTER_WAVES: readonly (readonly string[])[] = [
  [
    "process-tree-killed|language-neutral|incomplete plans|rolls back after-regenerations|rejects stale generator",
    "rolls back after-(?:staging|embedded-root-staging|moves|relocations|symlink-retargeting|edits)|rolls back before-verify",
    "rejects forged|rejects unreachable|keeps double-plan|recovers caught",
    "committed and rolled-back|elects exactly|restores a quarantined|rejects stale (?!generator)|rejects ordinal",
    "parent-killed (?:transaction-attempt-preparation-mkdir|transaction-attempt-preparation-children|transaction-initial-lease-json-write-mkdir|transaction-initial-lease-json-candidate-written|transaction-initial-lease-json-canonical-exchanged|transaction-initial-lease-prepared|transaction-initial-wal-mkdir|transaction-initial-journal-write-mkdir|transaction-initial-journal-candidate-written|transaction-initial-journal-canonical-exchanged)$",
    "parent-killed (?:transaction-initial-journal-canonical|transaction-attempt-canonical-published|transaction-journal-write-mkdir|transaction-journal-candidate-written|transaction-journal-previous-exchanged|transaction-journal-canonical-exchanged|transaction-wal-prepared|transaction-backup-write-mkdir|transaction-backup-write-mid|transaction-backup-write-prepared)$",
    "parent-killed (?:transaction-backup-inner-exchange|transaction-backup-exchange|transaction-backup-retained|transaction-edit-write-mkdir|transaction-edit-write-mid|transaction-edit-write-prepared|transaction-edit-inner-exchange|transaction-edit-exchange|transaction-edit-canonical-exchange|transaction-restore-mkdir)$",
    "parent-killed (?:transaction-restore-prepared|transaction-restore-exchange|transaction-restore-canonical-exchange|transaction-lease-stale-quarantined|transaction-lease-preparation-mkdir|transaction-lease-json-write-mkdir|transaction-lease-json-candidate-written|transaction-lease-json-canonical-exchanged|transaction-lease-prepared|transaction-lease-canonical-published)$",
  ],
];

export type TransactionV2ShardOutcome = Readonly<{
  ordinal: number;
  filter: string;
  concurrency: number;
  milliseconds: number;
  code: number | null;
  signal: NodeJS.Signals | null;
}>;

export type TransactionV2ShardOptions = Readonly<{
  repoRoot: string;
  artifactRoot: string;
  bundle: string;
  bundleRoot: string;
  normalizationBundle: string;
  schemaSnapshot: string;
  runId: string;
  runRoot: string;
  filterWaves: readonly (readonly string[])[];
  budgetMs?: number;
}>;

export type TransactionV2ShardResult = Readonly<{
  boundaries: string[];
  failure?: Error;
  outcomes: TransactionV2ShardOutcome[];
}>;

/** 🏃️ Executes the exact transaction shard selection with bounded owned-process retirement. */
export async function runTransactionV2Shards(options: TransactionV2ShardOptions): Promise<TransactionV2ShardResult> {
  const registry = join(options.bundleRoot, `pids-${options.runId}.txt`);
  const boundaryRegistry = join(options.bundleRoot, `boundaries-${options.runId}.txt`);
  writeFileSync(registry, "");
  writeFileSync(boundaryRegistry, "");
  const children: ReturnType<typeof spawn>[] = [];
  const childOutcomes = new Map<ReturnType<typeof spawn>, Promise<void>>();
  const closedStreams: Promise<void>[] = [];
  const outcomes: TransactionV2ShardOutcome[] = [];
  const killTree = terminateOwnedProcessTree;
  const registeredPids = (): number[] => [
    ...new Set(
      readFileSync(registry, "utf8")
        .split(/\s+/u)
        .filter(Boolean)
        .map(Number)
        .filter((pid) => Number.isSafeInteger(pid) && pid > 0),
    ),
  ];
  let failure: Error | undefined;
  let stopped = false;
  const stop = (error: Error): void => {
    if (stopped) return;
    stopped = true;
    failure = error;
    for (const child of children) if (child.pid && child.exitCode === null && child.signalCode === null) killTree(child.pid);
    for (const pid of registeredPids()) killTree(pid);
  };
  const interrupt = (): void => stop(new Error("Transaction v2 aggregate stopped: SIGINT"));
  const terminate = (): void => stop(new Error("Transaction v2 aggregate stopped: SIGTERM"));
  process.stdout.on("error", stop);
  process.stderr.on("error", stop);
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", terminate);
  const spawnFilter = (filter: string): ReturnType<typeof spawn> => {
    const concurrency = 6;
    const outputRoot = join(options.runRoot, "📓️shards", `🔢️${children.length + 1}`);
    const streams = ["stdout", "stderr"].map((kind) => {
      const root = join(outputRoot, kind);
      mkdirSync(root, { recursive: true });
      const path = join(root, "🔤️.txt");
      writeFileSync(path, "", { flag: "wx" });
      return path;
    });
    const startedAt = performance.now();
    const child = spawn(process.execPath, ["test", `--max-concurrency=${concurrency}`, options.bundle, "-t", filter], {
      cwd: options.repoRoot,
      detached: process.platform !== "win32",
      env: {
        ...process.env,
        NX_DAEMON: "false",
        SEMIO_TRANSACTION_V2_BOUNDARY_REGISTRY: boundaryRegistry,
        SEMIO_TRANSACTION_V2_MODULE: options.normalizationBundle,
        SEMIO_TRANSACTION_V2_SCHEMA: options.schemaSnapshot,
        SEMIO_TRANSACTION_V2_PID_REGISTRY: registry,
        SEMIO_TRANSACTION_V2_RUN_ID: options.runId,
        SEMIO_TRANSACTION_V2_RUN_ROOT: options.runRoot,
        SEMIO_TEST_ARTIFACT_DIR: options.artifactRoot,
      },
      stdio: ["ignore", "pipe", "pipe"],
    });
    child.stdout!.on("data", (bytes) => {
      try {
        appendFileSync(streams[0]!, bytes);
        process.stdout.write(bytes);
      } catch (error) {
        stop(error instanceof Error ? error : new Error(String(error)));
      }
    });
    child.stderr!.on("data", (bytes) => {
      try {
        appendFileSync(streams[1]!, bytes);
        process.stderr.write(bytes);
      } catch (error) {
        stop(error instanceof Error ? error : new Error(String(error)));
      }
    });
    closedStreams.push(new Promise((accept) => child.once("close", () => accept())));
    const ordinal = children.push(child);
    childOutcomes.set(
      child,
      new Promise<void>((accept) => {
        child.once("error", (error) => {
          stop(error);
          accept();
        });
        child.once("exit", (code, signal) => {
          const milliseconds = performance.now() - startedAt;
          outcomes.push({ ordinal, filter, concurrency, milliseconds, code, signal });
          console.error(`[TRACE] Transaction v2 shard ${ordinal} finished in ${(milliseconds / 1_000).toFixed(2)}s`);
          if (code !== 0 || signal) stop(new Error(`Transaction v2 shard ${ordinal} failed with ${signal ?? code}`));
          accept();
        });
      }),
    );
    return child;
  };
  const timer = setTimeout(() => stop(new Error(`Transaction v2 complete aggregate exceeded ${options.budgetMs ?? 14_000} milliseconds`)), options.budgetMs ?? 14_000);
  try {
    for (const wave of options.filterWaves) {
      const waveChildren: ReturnType<typeof spawn>[] = [];
      for (const filter of wave) {
        const child = spawnFilter(filter);
        waveChildren.push(child);
        if (stopped) break;
      }
      await Promise.all(waveChildren.map((child) => childOutcomes.get(child)!));
      if (stopped) break;
    }
  } finally {
    clearTimeout(timer);
  }
  await Promise.all(closedStreams);
  process.stdout.off("error", stop);
  process.stderr.off("error", stop);
  process.off("SIGINT", interrupt);
  process.off("SIGTERM", terminate);
  for (const pid of registeredPids()) {
    try {
      process.kill(pid, 0);
      killTree(pid);
      failure ??= new Error(`Transaction v2 aggregate left child ${pid} alive`);
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ESRCH") throw error;
    }
  }
  return {
    boundaries: readFileSync(boundaryRegistry, "utf8").split("\n").filter(Boolean).sort(),
    failure,
    outcomes: outcomes.sort((left, right) => left.ordinal - right.ordinal),
  };
}
