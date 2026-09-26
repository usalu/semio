#!/usr/bin/env bun
/** 🌎️ `os-hub-ts` (nx `os-hub-ts`) router: `bun ./📜️script.ts <test [quick|long|exhaustive] [args…]|two-client-e2e <sqlite|postgres|neo4j>|document-growth-e2e <sqlite|postgres|neo4j>|backend <up|down|status> <postgres|neo4j|all>|backend run <postgres|neo4j> -- <command…>|backup-restore-drill|residency-watch|hub-freshness|typecheck>`.
 * Bun integration-test harness that boots the REAL `os-hub` binary and drives it with two
 * independent clients to prove the hub's collaboration contract end-to-end (ticket
 * 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS, lane 3-E). Gated behind
 * `HUB_E2E=1` (see `🤝️index.test.ts`'s own doc) — the default `test` run never touches cargo and
 * reports the whole e2e suite as skipped in well under a second. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, resolveTestLevel, runBunxStatus, runBundleScriptMain, runCargo, runVitest, type TestLevel } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { HUB_BACKEND_ENGINE, HUB_BACKENDS, claimHubBackend, ensureHubBackend, freeLoopbackPort, hubBackendEngineVersion, hubBackendIdentity, hubBackendName, hubBackendStatus, hubDevBinaryPath, hubDevPostgresBinaryPath, stopHubBackend, type HubBackendName, type HubBackendProgress } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

const HUB_RUST_DIR = "🌎️hub/📦️packages/🦀️rust";

/** 🏗️ Builds the real `os-hub` debug binary the test spawns directly (never `cargo run` — no
 * wrapper-process tree to chase on teardown). Default cargo features only (sqlite): contract-freeze
 * Amendment 2 says `--all-features` is red repo-wide and pre-existing (`🛢️db`'s postgres/neo4j
 * features have no wired driver deps), so this never passes it. Only runs when `HUB_E2E=1`, so the
 * default `test` target never pays a cargo build. */
function buildHubBinary(repoRoot: string): void {
  if (process.env.HUB_E2E !== "1") return;
  const preexisting = process.env.OS_HUB_BINARY;
  if (preexisting) {
    console.log(`[os-hub-ts] HUB_E2E=1 — using OS_HUB_BINARY=${preexisting} (skip cargo build)`);
    return;
  }
  console.log("[os-hub-ts] HUB_E2E=1 — building os-hub (default features, no --all-features)…");
  runCargo(["build", "--manifest-path", "Cargo.toml"], join(repoRoot, HUB_RUST_DIR));
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    buildHubBinary(this.repoRoot);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🗄️ Runs one real-binary hub e2e scenario (`🌎️hub/🧪️tests/<scenario>`) on one storage backend: `sqlite`, or
 * `postgres`/`neo4j` on the ONE shared development server (`claimHubBackend`: a run database of its own on postgres,
 * the exclusive lease over a reset graph on neo4j), released afterwards — also when a failing run leaves through
 * `process.exit`, which skips `finally`. The hub is `OS_HUB_BINARY` when set, else the Nx-staged `build-dev-postgres`
 * executable, which links all three drivers. */
abstract class BackendE2eScript extends BundleScript {
  protected abstract readonly scenario: string;
  protected abstract readonly minimumLevel: TestLevel;

  async run(segments: string[]): Promise<void> {
    const [name, ...extra] = segments;
    const { rest } = resolveTestLevel(extra, this.minimumLevel);
    if (name !== "sqlite" && name !== "postgres" && name !== "neo4j") throw new Error(`${this.scenario} e2e needs one backend: sqlite | postgres | neo4j`);
    const binary = process.env.OS_HUB_BINARY ?? hubDevPostgresBinaryPath(join(this.repoRoot, HUB_RUST_DIR));
    const cancel = new AbortController();
    const interrupt = (): void => cancel.abort();
    process.once("SIGINT", interrupt);
    process.once("SIGTERM", interrupt);
    const claim = name === "sqlite" ? undefined : await claimHubBackend(this.repoRoot, name, `${this.scenario.replaceAll("-", "_")}_${process.pid}`, { signal: cancel.signal, onProgress: backendProgress });
    let released = !claim;
    const release = (): void => {
      if (released || !claim) return;
      released = true;
      claim.release();
    };
    process.once("exit", release);
    try {
      Object.assign(process.env, claim ? claim.env : { OS_HUB_STORAGE_BACKEND: "sqlite" }, { HUB_E2E: "1", OS_HUB_BINARY: binary, HUB_TWO_CLIENT_PORT: process.env.HUB_TWO_CLIENT_PORT ?? String(await freeLoopbackPort()) });
      if (claim) process.env.HUB_E2E_WRITER_PROBE = JSON.stringify(claim.client);
      console.log(`[os-hub-ts] ${this.scenario} e2e backend=${name} binary=${binary} hub-port=${process.env.HUB_TWO_CLIENT_PORT}${claim ? ` ${name}=${claim.server.host}:${claim.server.port} (${claim.server.identity.container})` : ""}`);
      await runVitest(this.root, [this.scenario, ...rest], "../../🧪️tests/🎚️config/🟦️.ts");
    } finally {
      process.removeListener("SIGINT", interrupt);
      process.removeListener("SIGTERM", interrupt);
      release();
    }
  }
}

/** 📈️ Prints one backend start step on stderr: `[os-hub-ts] backend postgres start 12 s`. */
function backendProgress(progress: HubBackendProgress): void {
  console.error(`[os-hub-ts] backend ${progress.name} ${progress.phase} ${progress.elapsedSeconds} s`);
}

/** 🐳️ `backend <up|down|status> <postgres|neo4j|all>` — the PERSISTENT development backend servers: `up` starts the
 * named server(s) from their `🌎️hub/compose.yaml` service (idempotent: a ready server is reused) and prints the hub
 * environment that selects it; `down` removes container and volume; `status` prints one line per backend. The same
 * shared servers serve `os-hub:dev-postgres`/`dev-neo4j` zero-touch and the pg/neo4j gates above.
 * Ctrl-C during `up` removes the half-started container.
 * `backend run <postgres|neo4j> -- <command…>` runs one command under a claim on the shared server (its own run
 * database on postgres, the exclusive lease over a reset graph on neo4j) with the hub environment that selects it
 * (`OS_HUB_DATABASE_URL`, `OS_HUB_NEO4J_URI`/`_USER`/`_PASSWORD`, …), forwards Ctrl-C to it, releases the claim when it
 * ends and exits with its status — how the db engine's live laws reach the one shared server. */
class BackendScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "run") return this.runClaimed(segments.slice(1));
    const [verb, target = "all", ...extra] = segments;
    if (extra.length || !["up", "down", "status"].includes(verb ?? "")) throw new Error("usage: backend <up|down|status> <postgres|neo4j|all> | backend run <postgres|neo4j> -- <command…>");
    const names = target === "all" ? (Object.keys(HUB_BACKENDS) as HubBackendName[]) : [hubBackendName(target)];
    const cancel = new AbortController();
    const interrupt = (): void => cancel.abort();
    process.once("SIGINT", interrupt);
    try {
      console.log(`[os-hub-ts] container engine ${HUB_BACKEND_ENGINE} ${await hubBackendEngineVersion(this.repoRoot)}`);
      for (const name of names) {
        const identity = hubBackendIdentity(name);
        if (verb === "down") {
          stopHubBackend(this.repoRoot, identity);
          console.log(`[os-hub-ts] backend ${name} down (container ${identity.container} and its volume removed)`);
          continue;
        }
        const server = verb === "up" ? await ensureHubBackend(this.repoRoot, identity, { signal: cancel.signal, onProgress: backendProgress }) : await hubBackendStatus(this.repoRoot, identity);
        if (!server) {
          console.log(`[os-hub-ts] backend ${name} absent`);
          continue;
        }
        console.log(`[os-hub-ts] backend ${name} ready ${server.host}:${server.port} container=${identity.container}`);
        for (const [key, value] of Object.entries(server.env)) console.log(`${key}=${value}`);
      }
    } finally {
      process.removeListener("SIGINT", interrupt);
    }
  }

  private async runClaimed(segments: string[]): Promise<void> {
    const [target, separator, ...command] = segments;
    if (separator !== "--" || command.length === 0) throw new Error("usage: backend run <postgres|neo4j> -- <command…>");
    const name = hubBackendName(target);
    const cancel = new AbortController();
    const interrupt = (): void => cancel.abort();
    process.once("SIGINT", interrupt);
    const claim = await claimHubBackend(this.repoRoot, name, `run_${process.pid}`, { signal: cancel.signal, onProgress: backendProgress });
    let released = false;
    const release = (): void => {
      if (released) return;
      released = true;
      claim.release();
    };
    process.once("exit", release);
    try {
      console.log(`[os-hub-ts] backend run ${name}=${claim.server.host}:${claim.server.port} (${claim.server.identity.container}): ${command.join(" ")}`);
      const { spawn } = await import("node:child_process");
      const status = await new Promise<number>((resolveExit, rejectExit) => {
        const child = spawn(command[0]!, command.slice(1), { cwd: this.repoRoot, env: { ...process.env, ...claim.env }, stdio: "inherit", shell: false });
        const forward = (): void => void child.kill("SIGINT");
        cancel.signal.addEventListener("abort", forward, { once: true });
        child.once("error", rejectExit);
        child.once("close", (code) => {
          cancel.signal.removeEventListener("abort", forward);
          resolveExit(code ?? -1);
        });
      });
      console.log(`[os-hub-ts] backend run ${name} finished with status ${status}; claim released`);
      if (status !== 0) process.exitCode = status;
    } finally {
      process.removeListener("SIGINT", interrupt);
      release();
    }
  }
}

/** 🤝️ The two-client document collaboration e2e (`🌎️hub/🧪️tests/🤝️two-client-document`). */
class TwoClientE2eScript extends BackendE2eScript {
  protected readonly scenario = "two-client-document";
  protected readonly minimumLevel = "long";
}

/** 📈️ The concurrent document growth e2e (`🌎️hub/🧪️tests/📈️document-growth`). */
class DocumentGrowthE2eScript extends BackendE2eScript {
  protected readonly scenario = "document-growth";
  protected readonly minimumLevel = "exhaustive";
}

function flagValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

/** 🛑️ An AbortSignal that Ctrl-C and SIGTERM abort, released by `done`. */
function interruptSignal(): { signal: AbortSignal; done: () => void } {
  const controller = new AbortController();
  const abort = (): void => controller.abort();
  process.once("SIGINT", abort);
  process.once("SIGTERM", abort);
  const done = (): void => {
    process.removeListener("SIGINT", abort);
    process.removeListener("SIGTERM", abort);
  };
  return { signal: controller.signal, done };
}

/** 💾️ `backup-restore-drill [--catalog-root <data root>] [--kind <kindId|schema prefix>] [--edits <n>] [--rounds <n>]
 * [--keep]` — the backup/restore drill on a fresh root seeded with a copy of a published trusted catalog (default: the
 * development hub's `.🧬semio/🌐hub/hub-dev`), the hub being `OS_HUB_BINARY` or the Nx-staged `build-dev` executable. */
class BackupRestoreDrillScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const startedAt = new Date();
    const interrupt = interruptSignal();
    const { runBackupRestoreDrill } = await import("../../🧪️tests/💾️backup-restore/🟦️.ts");
    try {
      const rounds = await runBackupRestoreDrill({
        repoRoot: this.repoRoot,
        binaryPath: process.env.OS_HUB_BINARY ?? hubDevBinaryPath(join(this.repoRoot, HUB_RUST_DIR)),
        catalogRoot: flagValue(segments, "--catalog-root") ?? join(this.repoRoot, ".🧬semio", "🌐hub", "hub-dev"),
        kind: flagValue(segments, "--kind") ?? "note",
        edits: Number(flagValue(segments, "--edits") ?? 20),
        rounds: Number(flagValue(segments, "--rounds") ?? 1),
        readyTimeoutMs: Number(flagValue(segments, "--ready-timeout-ms") ?? 1_800_000),
        keepRoots: segments.includes("--keep"),
        signal: interrupt.signal,
        onProgress: (line) => console.log(`[backup-drill] ${line}`),
      });
      const passed = rounds.filter((round) => round.pass).length;
      const first = rounds[0];
      publishAcceptanceCheckResult(
        this.repoRoot,
        acceptanceCheckResult({
          check: "hub-backup-restore",
          status: rounds.length > 0 && passed === rounds.length ? "pass" : "fail",
          startedAt,
          measured: { rounds: rounds.length, passed, sigtermToExitMs: first?.sigtermToExitMs ?? -1, archiveBytes: first?.archiveBytes ?? -1, archiveMs: first?.archiveMs ?? -1, restoredReadyMs: first?.restoredReadyMs ?? -1, byteIdentical: first?.byteIdentical ?? false },
          summary: {
            en: `${passed}/${rounds.length} backup/restore rounds pass${first?.error ? `; ${first.error.slice(0, 200)}` : ""}`,
            de: `${passed}/${rounds.length} Runden Sicherung/Wiederherstellung bestanden${first?.error ? `; ${first.error.slice(0, 200)}` : ""}`,
          },
          evidence: rounds.flatMap((round) => (round.roots ? [round.roots] : [])),
        }),
      );
      if (passed !== rounds.length || rounds.length === 0) process.exitCode = 1;
    } finally {
      interrupt.done();
    }
  }
}

/** 🧠️ `residency-watch --hub <url> [--pid <pid>] [--kinds <kindId,…>] [--rounds <n>] [--interval-ms <n>] [--settle-ms <n>]
 * [--budget-mib <n>]` — opens every creatable kind on a running hub (round-robin, `--rounds` times) while sampling its
 * resident set and the hub's compiled-guest residency, then watches it settle. Credentials: `OS_HUB_PROBE_EMAIL` /
 * `OS_HUB_PROBE_PASSWORD` (default the `dev s` local user; an admin subject for the residency readings). Passes when every
 * creation issues an open plan, every kind's pair agrees across rounds and, given a budget, the settled resident set is
 * within it. */
class ResidencyWatchScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const hub = flagValue(segments, "--hub");
    if (!hub) throw new Error("usage: residency-watch --hub <url> [--pid <pid>] [--kinds …] [--interval-ms <n>] [--settle-ms <n>] [--budget-mib <n>]");
    const startedAt = new Date();
    const interrupt = interruptSignal();
    const { runResidencyWatch } = await import("../../🧪️tests/🧠️residency/🟦️.ts");
    try {
      const budget = flagValue(segments, "--budget-mib");
      const report = await runResidencyWatch({
        hub: hub.replace(/\/$/u, ""),
        pid: flagValue(segments, "--pid") ? Number(flagValue(segments, "--pid")) : null,
        email: process.env.OS_HUB_PROBE_EMAIL ?? "user1@semio.dev",
        password: process.env.OS_HUB_PROBE_PASSWORD ?? "gm1-local-dev-pass-1",
        kinds: (flagValue(segments, "--kinds") ?? "").split(",").filter(Boolean),
        intervalMs: Number(flagValue(segments, "--interval-ms") ?? 5_000),
        settleMs: Number(flagValue(segments, "--settle-ms") ?? 120_000),
        budgetMiB: budget ? Number(budget) : null,
        rounds: Number(flagValue(segments, "--rounds") ?? 1),
        signal: interrupt.signal,
        onProgress: (line) => console.log(`[residency] ${line}`),
      });
      const planned = report.rows.filter((row) => row.openPlanStatus === 200).length;
      const withinBudget = report.budgetMiB === null || report.finalMiB <= report.budgetMiB;
      const status = !report.cancelled && report.rows.length > 0 && planned === report.rows.length && report.pairsAgree && withinBudget ? "pass" : "fail";
      console.log(`[residency] samples ${JSON.stringify(report.samples)}`);
      console.log(`[residency] rounds ${JSON.stringify(report.rounds)}`);
      publishAcceptanceCheckResult(
        this.repoRoot,
        acceptanceCheckResult({
          check: "hub-residency",
          status,
          startedAt,
          measured: { kinds: report.rows.length, openPlans: planned, baselineMiB: report.baselineMiB, peakMiB: report.peakMiB, finalMiB: report.finalMiB, releasedMiB: report.releasedMiB, budgetMiB: report.budgetMiB ?? -1, samples: report.samples.length, rounds: report.rounds.length, compilesPerRound: report.rounds.map((round) => round.compiles).join(","), divergentKinds: report.divergentKinds.join(",") },
          summary: {
            en: `${planned}/${report.rows.length} creations opened; resident ${report.baselineMiB} → peak ${report.peakMiB} → settled ${report.finalMiB} MiB (released ${report.releasedMiB} MiB)${report.budgetMiB === null ? "" : `, budget ${report.budgetMiB} MiB`}; compiles per round ${report.rounds.map((round) => round.compiles).join("/") || "-"}; pairs ${report.pairsAgree ? "agree" : `differ for ${report.divergentKinds.join(", ")}`}`,
            de: `${planned}/${report.rows.length} Erstellungen geöffnet; resident ${report.baselineMiB} → Spitze ${report.peakMiB} → beruhigt ${report.finalMiB} MiB (freigegeben ${report.releasedMiB} MiB)${report.budgetMiB === null ? "" : `, Budget ${report.budgetMiB} MiB`}; Kompilierungen je Runde ${report.rounds.map((round) => round.compiles).join("/") || "-"}; Paare ${report.pairsAgree ? "stimmen überein" : `weichen ab für ${report.divergentKinds.join(", ")}`}`,
          },
        }),
      );
      if (status !== "pass") process.exitCode = 1;
    } finally {
      interrupt.done();
    }
  }
}

/** 🌅️ `boot-watch [--catalog-root <data root>] [--port <n>] [--restarts <n>] [--residency-bytes <n>] [--interval-ms <n>]
 * [--ready-timeout-ms <n>] [--keep]` — boots the hub (`OS_HUB_BINARY` or the Nx-staged `build-dev` executable) on a fresh
 * root seeded with a copy of a published trusted catalog (default the development hub's `.🧬semio/🌐hub/hub-dev`), cold
 * and then `--restarts` times warm, following `/readyz` `startup.catalog` until it serves and the admin observability
 * `catalog` until every package's codec rows are pinned. Passes when every boot serves and no package is refused. */
class BootWatchScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const startedAt = new Date();
    const interrupt = interruptSignal();
    const { runBootWatch } = await import("../../🧪️tests/🌅️boot-watch/🟦️.ts");
    try {
      const residencyBytes = flagValue(segments, "--residency-bytes");
      const port = flagValue(segments, "--port");
      const report = await runBootWatch({
        repoRoot: this.repoRoot,
        binaryPath: process.env.OS_HUB_BINARY ?? hubDevBinaryPath(join(this.repoRoot, HUB_RUST_DIR)),
        catalogRoot: flagValue(segments, "--catalog-root") ?? join(this.repoRoot, ".🧬semio", "🌐hub", "hub-dev"),
        port: port ? Number(port) : null,
        restarts: Number(flagValue(segments, "--restarts") ?? 1),
        residencyBytes: residencyBytes ? Number(residencyBytes) : null,
        intervalMs: Number(flagValue(segments, "--interval-ms") ?? 1_000),
        readyTimeoutMs: Number(flagValue(segments, "--ready-timeout-ms") ?? 3_600_000),
        keepRoot: segments.includes("--keep"),
        signal: interrupt.signal,
        onProgress: (line) => console.log(`[boot-watch] ${line}`),
      });
      for (const boot of report.boots) console.log(`[boot-watch] boot ${boot.boot} ${JSON.stringify({ ...boot, samples: boot.samples.length })}`);
      console.log(`[boot-watch] samples ${JSON.stringify(report.boots.map((boot) => boot.samples))}`);
      const cold = report.boots[0];
      const warm = report.boots.slice(1);
      const served = report.boots.length > 0 && report.boots.every((boot) => boot.readyMs !== undefined && boot.verifiedMs !== undefined && !boot.error);
      const refused = report.boots.reduce((sum, boot) => sum + (boot.refused ?? 0), 0);
      const status = !report.cancelled && served && refused === 0 ? "pass" : "fail";
      const warmReady = warm.map((boot) => boot.readyMs ?? -1).join("/") || "-";
      publishAcceptanceCheckResult(
        this.repoRoot,
        acceptanceCheckResult({
          check: "hub-boot",
          status,
          startedAt,
          measured: { boots: report.boots.length, coldReadyMs: cold?.readyMs ?? -1, coldVerifiedMs: cold?.verifiedMs ?? -1, warmReadyMs: warmReady, warmVerifiedMs: warm.map((boot) => boot.verifiedMs ?? -1).join("/") || "-", refused, generation: report.generation },
          summary: {
            en: `cold boot served in ${cold?.readyMs ?? "-"} ms, every package pinned at ${cold?.verifiedMs ?? "-"} ms; warm boots served in ${warmReady} ms; ${refused} packages refused${cold?.error ? `; ${cold.error.slice(0, 200)}` : ""}`,
            de: `Kaltstart bediente nach ${cold?.readyMs ?? "-"} ms, jedes Paket nach ${cold?.verifiedMs ?? "-"} ms festgelegt; Warmstarts bedienten nach ${warmReady} ms; ${refused} Pakete abgelehnt${cold?.error ? `; ${cold.error.slice(0, 200)}` : ""}`,
          },
          evidence: [report.root],
        }),
      );
      if (status !== "pass") process.exitCode = 1;
    } finally {
      interrupt.done();
    }
  }
}

/** 🏷️ `hub-freshness --hub <url> [--binary <path>]` — asserts the running hub is built from the current tree (Cargo's
 * freshness rule over the `os-hub.sources.json` record its staging verb wrote beside the executable). */
class HubFreshnessScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const hub = flagValue(segments, "--hub");
    if (!hub) throw new Error("usage: hub-freshness --hub <url> [--binary <path>]");
    const startedAt = new Date();
    const { checkHubBuildFreshness } = await import("../../🧪️tests/🏷️build-freshness/🟦️.ts");
    const report = await checkHubBuildFreshness(hub.replace(/\/$/u, ""), flagValue(segments, "--binary") ?? null);
    console.log(`[hub-freshness] ${JSON.stringify({ ...report, changed: report.changed.slice(0, 20) })}`);
    publishAcceptanceCheckResult(
      this.repoRoot,
      acceptanceCheckResult({
        check: "hub-build-freshness",
        status: report.verdict === "fresh" ? "pass" : report.verdict === "stale" ? "fail" : "blocked",
        startedAt,
        measured: { verdict: report.verdict, runId: report.runId ?? "", sources: report.sources, changed: report.changed.length, executable: report.executable ?? "", builtAtMs: report.builtAtMs ?? -1 },
        summary: {
          en: `${report.verdict}: ${report.reason.en}`,
          de: `${report.verdict === "fresh" ? "aktuell" : report.verdict === "stale" ? "veraltet" : "nicht prüfbar"}: ${report.reason.de}`,
        },
        evidence: report.record ? [report.record] : [],
      }),
    );
    if (report.verdict !== "fresh") process.exitCode = 1;
  }
}

/** 🪁️ Type-checks every `🌎️hub/**` TypeScript source against the hub-scoped `tsconfig.json`. */
class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    const status = runBunxStatus(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root);
    if (status !== 0) process.exit(status);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("two-client-e2e", TwoClientE2eScript).register("document-growth-e2e", DocumentGrowthE2eScript).register("backend", BackendScript).register("backup-restore-drill", BackupRestoreDrillScript).register("residency-watch", ResidencyWatchScript).register("boot-watch", BootWatchScript).register("hub-freshness", HubFreshnessScript).register("typecheck", TypecheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
