/** 🌅️ Boot watch: how long a hub takes to serve a published trusted catalog, cold and warm, and how its codec rows are
 * pinned after it serves.
 *
 * A fresh data root receives a copy of the catalog; the real `os-hub` boots on it (cold: no verification memory) while
 * `/readyz` is read every `intervalMs` — its `startup.catalog` (`TrustedCatalogLoadProgressV1`: per-package phase,
 * component bytes read, codec rows pinned) is recorded until the hub answers ready; then `GET /admin/api/observability`
 * `catalog` is read until every package is `ready` or `refused` (the background verification). The hub is stopped with
 * SIGTERM and booted again on the same root `restarts` times (warm: the verification memory the first boot wrote). Counts
 * only — the watch reports elapsed times, never estimates.
 *
 * Promoted from the session-12 ticket harnesses `wp-h10/readyz-watch.sh` + `h10-hub.sh` (ticket 26/09/23; there: cold
 * 153.9 s, warm 14.5 s on catalog B2 at load ~28, when every row was interpreted before the hub served).
 */
import { randomBytes } from "node:crypto";
import { spawnSync } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import type { TrustedCatalogLoadProgressV1 } from "../../📊️observability/🟦️.ts";
import { findFreePort, hubProbeCall, hubProbeSignIn, hubSeedTrustedCatalog, startHub, type HubHandle } from "../../🤝️integration-harness/🟦️.ts";

/** 🎛️ One watch. */
export type BootWatchOptions = Readonly<{
  repoRoot: string;
  binaryPath: string;
  catalogRoot: string;
  port: number | null;
  restarts: number;
  residencyBytes: number | null;
  intervalMs: number;
  readyTimeoutMs: number;
  keepRoot: boolean;
  signal: AbortSignal;
  onProgress: (line: string) => void;
}>;

/** 📏️ One reading of a booting or verifying hub. */
export type BootWatchSample = Readonly<{ atMs: number; http: number; packagesReady: number; packagesTotal: number; packagesRefused: number; componentBytesRead: number; rowsPinned: number; rowsVerified: number; rowsTotal: number }>;

/** 🌅️ One boot: time to serve, time until every package's rows were pinned, and the readings in between. */
export type BootWatchBoot = { boot: number; kind: "cold" | "warm"; readyMs?: number; verifiedMs?: number; refused?: number; samples: BootWatchSample[]; sigtermToExitMs?: number; error?: string };

/** 📊️ The watch's report. */
export type BootWatchReport = { generation: string; root: string; boots: BootWatchBoot[]; cancelled: boolean };

const EMAIL = "boot-watch@semio.dev";

function reading(atMs: number, http: number, catalog: TrustedCatalogLoadProgressV1 | undefined): BootWatchSample {
  return {
    atMs,
    http,
    packagesReady: catalog?.packagesReady ?? 0,
    packagesTotal: catalog?.packagesTotal ?? 0,
    packagesRefused: catalog?.packagesRefused ?? 0,
    componentBytesRead: catalog?.componentBytesRead ?? 0,
    rowsPinned: catalog?.rowsPinned ?? 0,
    rowsVerified: catalog?.rowsVerified ?? 0,
    rowsTotal: catalog?.rowsTotal ?? 0,
  };
}

const pause = (ms: number): Promise<void> => new Promise((resolveDelay) => setTimeout(resolveDelay, ms));

/** 🌅️ Boots once and follows the hub until it serves and until its background verification settled. */
async function watchBoot(options: BootWatchOptions, dataDir: string, password: string, boot: number): Promise<{ hub: HubHandle | undefined; result: BootWatchBoot }> {
  const result: BootWatchBoot = { boot, kind: boot === 0 ? "cold" : "warm", samples: [] };
  const port = options.port ?? (await findFreePort());
  const started = Date.now();
  const env: Record<string, string> = {
    OS_HUB_MODE: "production",
    OS_HUB_BIND: "127.0.0.1",
    OS_HUB_CREDENTIAL_SIGN_IN: "true",
    OS_HUB_ADMIN_SUBJECTS: `credential.password.v1:${EMAIL}`,
    ...(options.residencyBytes === null ? {} : { OS_HUB_GUEST_RESIDENCY_BYTES: String(options.residencyBytes) }),
  };
  let hub: HubHandle | undefined;
  try {
    hub = await startHub({ repoRoot: options.repoRoot, dataDir, adminToken: "", binaryPath: options.binaryPath, port, readyTimeoutMs: options.readyTimeoutMs, env });
    while (!options.signal.aborted && Date.now() - started < options.readyTimeoutMs) {
      const answer = await hubProbeCall(hub.baseUrl, "GET", "/readyz");
      result.samples.push(reading(Date.now() - started, answer.status, answer.json?.startup?.catalog));
      if (answer.status === 200) {
        result.readyMs = Date.now() - started;
        break;
      }
      await pause(options.intervalMs);
    }
    if (result.readyMs === undefined) throw new Error(`not ready within ${options.readyTimeoutMs} ms`);
    options.onProgress(`boot ${boot} (${result.kind}) ready in ${result.readyMs} ms`);
    const token = await hubProbeSignIn(hub.baseUrl, EMAIL, password, "bootwatch");
    while (!options.signal.aborted && Date.now() - started < options.readyTimeoutMs) {
      const answer = await hubProbeCall(hub.baseUrl, "GET", "/admin/api/observability", token);
      const catalog = answer.json?.catalog as TrustedCatalogLoadProgressV1 | undefined;
      result.samples.push(reading(Date.now() - started, answer.status, catalog));
      if (catalog && catalog.packagesReady + catalog.packagesRefused === catalog.packagesTotal) {
        result.verifiedMs = Date.now() - started;
        result.refused = catalog.packagesRefused;
        break;
      }
      await pause(options.intervalMs);
    }
    options.onProgress(`boot ${boot} (${result.kind}) every package settled in ${result.verifiedMs ?? "-"} ms, ${result.refused ?? "-"} refused`);
  } catch (error) {
    result.error = String(error instanceof Error ? error.message : error).slice(0, 400);
  }
  return { hub, result };
}

/** 🌅️ Runs the watch: one cold boot and `restarts` warm boots on the same root. */
export async function runBootWatch(options: BootWatchOptions): Promise<BootWatchReport> {
  const root = mkdtempSync(join(tmpdir(), "semio-boot-watch-"));
  const generation = hubSeedTrustedCatalog(options.catalogRoot, root);
  const password = randomBytes(18).toString("hex");
  const provisioned = spawnSync(options.binaryPath, ["credential", "set", "--email", EMAIL, "--display-name", "Boot Watch"], { env: { ...process.env, OS_HUB_DATA: root }, input: password, encoding: "utf8" });
  if (provisioned.status !== 0) throw new Error(`credential set failed: ${provisioned.stderr}`);
  const boots: BootWatchBoot[] = [];
  try {
    for (let boot = 0; boot <= options.restarts && !options.signal.aborted; boot += 1) {
      options.onProgress(`boot ${boot} on ${root} (catalog ${generation.slice(0, 12)})`);
      const { hub, result } = await watchBoot(options, root, password, boot);
      if (hub) {
        const stopping = Date.now();
        await hub.stop();
        result.sigtermToExitMs = Date.now() - stopping;
      }
      boots.push(result);
      if (result.error) break;
    }
  } finally {
    if (!options.keepRoot && boots.every((boot) => !boot.error)) rmSync(root, { recursive: true, force: true });
  }
  return { generation, root, boots, cancelled: options.signal.aborted };
}
