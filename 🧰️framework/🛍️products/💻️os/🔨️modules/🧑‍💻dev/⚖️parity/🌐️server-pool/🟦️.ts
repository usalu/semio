import { resolveTestLevel, atTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🧩️ Semantic parity server pool owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { describeDevPortOccupant, devServerUrl, isDevPortInUse, wgpuDevPlayUrl } from "@semio-tech/repo-lib/dev";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { spawnDaemon, type SpawnDaemonHandle } from "@semio-tech/repo-lib/process";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const repoRoot = findWorkspaceRoot();

import { awaitChildExit, awaitTcpReady } from "../../♻️activation/🩺️readiness/🟦️.ts";

import { ParityRenderer } from "../🏗️structure/🟦️.ts";

import { parityOutDir } from "../📊️report/🟦️.ts";
import { runtimeExecutableV1 } from "../../../../../../🔨️modules/⚙️runtime/🏷️identity/🟦️.ts";
import { runtimeSleepV1 } from "../../../../../../🔨️modules/⚙️runtime/🚀️process/🟦️.ts";



//#endregion 🔖️ProbeCatalog

//#region 🔖️ServerPool
/** 🔌️Harness dev-server pool — clear of the catalog's per-variant 6012–6205 ports so a sweep never
 * collides with another concurrent dev's running playground. One react+wgpu port pair per shard,
 * reused (restart-between-variants) across that shard's playground list. React bakes its program
 * choice at boot via `VITE_SEMIO_PLUGIN` (no runtime `?query=` switch — see `js/index.ts`), so a
 * fresh server per variant is required on both renderers, not just wgpu. */
const PARITY_PORT_BASE = 7300;

function parityPortsForShard(shardIndex: number): { readonly react: number; readonly wgpu: number } {
  const base = PARITY_PORT_BASE + shardIndex * 2;
  return { react: base, wgpu: base + 1 };
}

const PARITY_PORT_POOL_SHARDS = 49;

// (7398 - 7300) / 2

/** 🔌️`smoke`/`triage`/`verify` are meant to be run by multiple concurrent agents/sessions — hardcoding
 * shard 0 meant every concurrent invocation collided on the same 7300/7301 pair, producing false
 * `SERVER-FAIL`/`DUMP-EMPTY` results indistinguishable from real failures (found the hard way: several
 * parallel boot-triage agents hit exactly this). Scans the shard pool for the first pair where BOTH
 * ports are actually free right now — no coordination needed between callers. */
function findFreeParityPortPair(): { readonly react: number; readonly wgpu: number } {
  for (let shard = 0; shard < PARITY_PORT_POOL_SHARDS; shard++) {
    const candidate = parityPortsForShard(shard);
    if (!isDevPortInUse("127.0.0.1", candidate.react) && !isDevPortInUse("127.0.0.1", candidate.wgpu)) return candidate;
  }
  throw new Error(`no free parity port pair in the ${PARITY_PORT_POOL_SHARDS}-shard pool (${PARITY_PORT_BASE}-${PARITY_PORT_BASE + PARITY_PORT_POOL_SHARDS * 2})`);
}

function parityDevUrl(renderer: ParityRenderer, variant: string, port: number): string {
  return renderer === "wgpu" ? wgpuDevPlayUrl("127.0.0.1", port, variant) : devServerUrl("127.0.0.1", port);
}

type ParityServerHandle = { readonly daemon: SpawnDaemonHandle; readonly port: number };

/** ⏱️A cold `repo ./📜️script.ts dev` boot can mean compiling the ENTIRE plugin crate catalog (33 crates)
 * plus, for wgpu, a from-scratch trunk/cargo build — many minutes with an empty `target/`, not the
 * ~40-60s a warm-cache boot takes. Default generously; `PARITY_BOOT_BUDGET_MS` overrides for CI/tuning. */
const PARITY_DEV_SERVER_BOOT_BUDGET_MS = Number(process.env.PARITY_BOOT_BUDGET_MS ?? 900_000);

/** 🧱️ Builds and stages one variant exactly once before either renderer starts. React's normal
 * streaming boot and WGPU's blocking boot would otherwise launch duplicate Cargo builds against the
 * shared target directory, spend most of their budget on file locks, and expose a listening port
 * before the app program exists. */
async function prebuildParityPlugin(variant: string): Promise<void> {
  const devScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts");
  const logPath = join(parityOutDir(), `prebuild-${variant}.log`);
  const lockRoot = parityOutDir();
  const lockPath = join(lockRoot, ".semio-parity-prebuild-lock");
  mkdirSync(lockRoot, { recursive: true });
  const lockDeadline = Date.now() + PARITY_DEV_SERVER_BOOT_BUDGET_MS;
  while (true) {
    try {
      mkdirSync(lockPath);
      break;
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
      if (Date.now() >= lockDeadline) throw new Error(`plugin prebuild lock for ${variant} exceeded ${PARITY_DEV_SERVER_BOOT_BUDGET_MS}ms (${lockPath})`);
      await runtimeSleepV1(500);
    }
  }
  try {
    const logStream = createWriteStream(logPath);
    const daemon = spawnDaemon(runtimeExecutableV1(), [devScript, "plugin", variant], {
      cwd: join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"),
      env: {
        ...process.env,
        SEMIO_PLUGIN: variant,
        CARGO_BUILD_JOBS: process.env.CARGO_BUILD_JOBS ?? "4",
      },
      stdio: "pipe",
    });
    daemon.child.stdout?.pipe(logStream);
    daemon.child.stderr?.pipe(logStream);
    const exited = await awaitChildExit(daemon.child, PARITY_DEV_SERVER_BOOT_BUDGET_MS);
    if (exited === "timeout") {
      daemon.kill();
      logStream.end();
      throw new Error(`plugin prebuild for ${variant} exceeded ${PARITY_DEV_SERVER_BOOT_BUDGET_MS}ms (see ${logPath})`);
    }
    logStream.end();
    if (daemon.child.exitCode !== 0) throw new Error(`plugin prebuild for ${variant} failed with code ${daemon.child.exitCode} (see ${logPath})`);
  } finally {
    rmSync(lockPath, { recursive: true, force: true });
  }
}

async function startParityDevServer(renderer: ParityRenderer, variant: string, port: number): Promise<ParityServerHandle> {
  const devScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts");
  const logPath = join(parityOutDir(), `boot-${renderer}-${variant}.log`);
  const logStream = createWriteStream(logPath);
  const daemon = spawnDaemon(runtimeExecutableV1(), [devScript, "dev"], {
    cwd: join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"),
    env: {
      ...process.env,
      SEMIO_PLUGIN: variant,
      SEMIO_RENDERER: renderer,
      SEMIO_PARITY_QUIET_CARGO: "1",
      S_OS_PORT: String(port),
      CARGO_BUILD_JOBS: process.env.CARGO_BUILD_JOBS ?? "4",
    },
    stdio: "pipe",
  });
  daemon.child.stdout?.pipe(logStream);
  daemon.child.stderr?.pipe(logStream);
  const outcome = await awaitTcpReady("127.0.0.1", port, {
    deadlineMs: PARITY_DEV_SERVER_BOOT_BUDGET_MS,
    intervalMs: 500,
    isDead: () => daemon.child.exitCode !== null,
  });
  if (outcome === "ready") return { daemon, port };
  if (outcome === "dead") throw new Error(`${renderer} dev server for ${variant} exited early (code ${daemon.child.exitCode}) — see ${logPath}`);
  daemon.kill();
  throw new Error(`${renderer} dev server for ${variant} did not open port ${port} within ${PARITY_DEV_SERVER_BOOT_BUDGET_MS}ms — see ${logPath}`);
}

/** 🧹️Best-effort: kills the spawned wrapper AND whatever ends up bound to the port, since vite/trunk
 * fork their own child processes that a plain wrapper-kill doesn't always reap. */
function stopParityDevServer(handle: ParityServerHandle): void {
  try {
    handle.daemon.kill();
  } catch {}
  const occupant = describeDevPortOccupant(handle.port);
  const pid = Number(occupant?.match(/PID (\d+)/)?.[1]);
  if (Number.isFinite(pid)) {
    try {
      process.kill(pid, "SIGTERM");
    } catch {}
  }
}

export { PARITY_DEV_SERVER_BOOT_BUDGET_MS, PARITY_PORT_BASE, PARITY_PORT_POOL_SHARDS, type ParityServerHandle, findFreeParityPortPair, parityDevUrl, parityPortsForShard, prebuildParityPlugin, startParityDevServer, stopParityDevServer };
