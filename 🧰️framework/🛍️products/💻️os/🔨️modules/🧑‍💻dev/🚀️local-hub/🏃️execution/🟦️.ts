import { terminateOwnedProcessTree } from "../../../../../../🔨️modules/🏃️process/🪓️termination/🟦️.ts";
import { DEV_LOCAL_HUB_DATA_ENV, DEV_LOCAL_HUB_PROFILE_ENV, DEV_LOCAL_HUB_PROVIDER_ENV, parseDevLocalHubProviderV1, type DevLocalHubProviderV1 } from "../🧬️schema/🟦️.ts";

export const DEV_LOCAL_HUB_READINESS_STALL_BOUND_MS = 300_000;

import { requestLocalBrokerSession } from "../../../📇️directory/🎫️local-session/🗄️broker/🟦️.ts";
/** 🚀️ Zero-touch loopback development hub for `dev s`: ONE detached owner per hub port and data root holds the hub's
 * local-bootstrap pipe and its session broker, and every `s` serve — single-user rows and both two-user rows — signs in
 * through that broker. Which launch row reaches a clean data root first (`▶️start`, a `dev s` row, the compound rows) no
 * longer decides who can sign in, and stopping one UI never takes the hub away from another.
 *
 * Only the DEFAULT hub is ever started here. A hub named explicitly (`S_HUB_URL`, or a hub url a caller passes) belongs to
 * whoever runs it, so a serve only JOINS it: it waits, saying so, and gives up after {@link DEV_HUB_JOIN_BOUND_MS} with a
 * typed outcome. Two serves pointed at a W2 hub that was still booting each started a hub of their own on its port (26/09/26
 * 15:03, both bound 7800 at 15:41 and forced the real one to restart). Starting is guarded by an owner lease per port and
 * per data root ({@link claimDevHubLeaseV1}), so two serves never start two hubs and a hub that has not bound yet is owned.
 * The owner republishes a catalog the current hub cannot load ({@link devHubCatalogFreshnessV1}) instead of crashing on it.
 * Contracts: `🧑‍💻dev/🧬️schema/🔣️.json` `DevHubLeaseV1`, `DevHubCatalogHeaderV1`, `DevHubCatalogPointerV1`; laws
 * `🧑‍💻dev/🧪️tests/🚀️local-hub` over `🧑‍💻dev/🧫️fixtures/🚀️local-hub.json`.
 * @see ../../../📇️directory/🎫️local-session/🗄️broker/🟦️.ts */

import { spawn, type ChildProcess } from "node:child_process";
import { closeSync, existsSync, linkSync, mkdirSync, openSync, readFileSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { createConnection } from "node:net";
import { dirname, join, resolve } from "node:path";
import { isDevPortInUse } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { protectOwnerOnly } from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🔐️owner-only/🟦️.ts";



export const DEV_LOCAL_HUB_DEFAULT_URL = "http://127.0.0.1:8787";
const DEV_LOCAL_HUB_OWNER_FILE = "local-hub-owner.json";
const DEV_LOCAL_HUB_LOG_FILE = "local-hub.log";
/** ⏳️ How long a serve waits for a hub it only joins before it continues local-first. The shell's own session refresh
 * reconnects whenever that hub comes up later, so the bound only decides how long the terminal waits, not whether the app
 * ever reaches the hub. */
export const DEV_HUB_JOIN_BOUND_MS = 120_000;
/** 📣️ How often a waiting serve says it is still waiting. */
export const DEV_HUB_STATUS_INTERVAL_MS = 10_000;

export type DevLocalHubSession = Readonly<{ hubUrl: string; dataDir: string; profileId: string; userId: string }>;

//#region 🔖️Status
/** 🌐️ How a serve relates to a hub: `own` — the zero-touch default hub this checkout may start; `join` — a hub named
 * explicitly, which a serve only ever waits for. */
export type DevHubRoleV1 = "own" | "join";

export function devHubRoleV1(explicitHubUrl: string | null | undefined): DevHubRoleV1 {
  return typeof explicitHubUrl === "string" && explicitHubUrl.trim().length > 0 ? "join" : "own";
}

/** 📣️ Everything the dev hub path tells the developer, as data: the terminal line is {@link devHubStatusTextV1}. */
export type DevHubStatusV1 =
  | { readonly kind: "waiting"; readonly hubUrl: string; readonly waitedMs: number; readonly boundMs: number }
  | { readonly kind: "joined"; readonly hubUrl: string }
  | { readonly kind: "gave-up"; readonly hubUrl: string; readonly waitedMs: number }
  | { readonly kind: "owned"; readonly hubUrl: string; readonly pid: number }
  | { readonly kind: "port-taken"; readonly hubUrl: string }
  | { readonly kind: "starting"; readonly hubUrl: string; readonly pid: number }
  | { readonly kind: "catalog-stale"; readonly dataDir: string; readonly reason: string }
  | { readonly kind: "catalog-publishing"; readonly line: string }
  | { readonly kind: "catalog-published"; readonly dataDir: string }
  | { readonly kind: "catalog-cancelled"; readonly dataDir: string }
  | { readonly kind: "no-broker"; readonly hubUrl: string; readonly dataDir: string }
  | { readonly kind: "session"; readonly hubUrl: string; readonly userId: string; readonly profileId: string };

export type DevHubLocaleV1 = "en" | "de";

/** 🌐️ The terminal's language, from the POSIX locale variables in their precedence order; only an unset or unknown
 * locale reads English. */
export function devHubLocaleV1(env: Readonly<Record<string, string | undefined>> = process.env): DevHubLocaleV1 {
  const tag = env.LC_ALL || env.LC_MESSAGES || env.LANG || "";
  return /^de(?:[_.-]|$)/iu.test(tag) ? "de" : "en";
}

const seconds = (ms: number): number => Math.round(ms / 1000);

export function devHubStatusTextV1(status: DevHubStatusV1, locale: DevHubLocaleV1): string {
  const de = locale === "de";
  switch (status.kind) {
    case "waiting":
      return de
        ? `[dev-local-hub] warte auf den Hub unter ${status.hubUrl} (${seconds(status.waitedMs)} von ${seconds(status.boundMs)} s) — dieser Serve startet dort keinen eigenen Hub`
        : `[dev-local-hub] waiting for the hub at ${status.hubUrl} (${seconds(status.waitedMs)} of ${seconds(status.boundMs)} s) — this serve never starts a hub there`;
    case "joined":
      return de ? `[dev-local-hub] mit dem Hub unter ${status.hubUrl} verbunden` : `[dev-local-hub] joined the hub at ${status.hubUrl}`;
    case "gave-up":
      return de
        ? `[dev-local-hub] der Hub unter ${status.hubUrl} hat nach ${seconds(status.waitedMs)} s nicht geantwortet — es geht lokal weiter; die Shell verbindet sich, sobald er antwortet`
        : `[dev-local-hub] the hub at ${status.hubUrl} did not answer within ${seconds(status.waitedMs)} s — continuing local-first; the shell connects once it answers`;
    case "owned":
      return de ? `[dev-local-hub] der Hub unter ${status.hubUrl} gehört bereits Prozess ${status.pid} — warte auf ihn` : `[dev-local-hub] the hub at ${status.hubUrl} is already owned by process ${status.pid} — waiting for it`;
    case "port-taken":
      return de ? `[dev-local-hub] ${status.hubUrl} ist von einem anderen Prozess belegt — kein zweiter Hub` : `[dev-local-hub] ${status.hubUrl} is held by another process — not starting a second hub`;
    case "starting":
      return de ? `[dev-local-hub] Hub-Besitzer (Prozess ${status.pid}) für ${status.hubUrl} gestartet` : `[dev-local-hub] started the hub owner (process ${status.pid}) for ${status.hubUrl}`;
    case "catalog-stale":
      return de ? `[dev-local-hub] der Katalog in ${status.dataDir} ist veraltet (${status.reason}) — wird neu veröffentlicht` : `[dev-local-hub] the catalog in ${status.dataDir} is out of date (${status.reason}) — republishing it`;
    case "catalog-publishing":
      return de ? `[dev-local-hub] Katalog wird veröffentlicht: ${status.line}` : `[dev-local-hub] publishing the catalog: ${status.line}`;
    case "catalog-published":
      return de ? `[dev-local-hub] Katalog in ${status.dataDir} veröffentlicht` : `[dev-local-hub] published the catalog in ${status.dataDir}`;
    case "catalog-cancelled":
      return de ? `[dev-local-hub] Katalogveröffentlichung abgebrochen — ${status.dataDir} behält seine bisherige Generation` : `[dev-local-hub] catalog publication cancelled — ${status.dataDir} keeps its previous generation`;
    case "no-broker":
      return de
        ? `[dev-local-hub] ${status.hubUrl} hat keinen Sitzungsvermittler für ${status.dataDir} — melde dich in der Shell an`
        : `[dev-local-hub] ${status.hubUrl} has no session broker for ${status.dataDir} — sign in through the shell`;
    case "session":
      return de ? `[dev-local-hub] bereit unter ${status.hubUrl} als ${status.userId} (${status.profileId})` : `[dev-local-hub] ready at ${status.hubUrl} as ${status.userId} (${status.profileId})`;
  }
}
//#endregion 🔖️Status

//#region 🔖️Lease
/** 🔐️ One claim on a development hub (`DevHubLeaseV1`). */
export type DevHubLeaseV1 = Readonly<{ pid: number; port: number; dataDir: string; hubUrl: string; acquiredAt: number }>;

export type DevHubLeaseClaimV1 = { readonly kind: "claimed" } | { readonly kind: "held"; readonly lease: DevHubLeaseV1 };

/** 🗂️ Where port leases live: one file per hub port, shared by every data root of this checkout. */
export function devHubLeaseRootV1(repoRoot: string): string {
  return join(repoRoot, ".🧬semio", "🌐hub", "dev-hub-leases");
}

/** 🗂️ The two claims one owner holds: its hub port and its data root. */
export function devHubLeasePathsV1(leaseRoot: string, port: number, dataDir: string): readonly [string, string] {
  return [join(leaseRoot, `port-${port}.json`), join(dataDir, DEV_LOCAL_HUB_OWNER_FILE)];
}

export function isPidAliveV1(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch {
    return false;
  }
}

export function readDevHubLeaseV1(path: string): DevHubLeaseV1 | null {
  try {
    const value = JSON.parse(readFileSync(path, "utf8")) as Partial<DevHubLeaseV1>;
    if (!Number.isSafeInteger(value.pid) || (value.pid as number) < 1 || !Number.isSafeInteger(value.port) || typeof value.dataDir !== "string" || typeof value.hubUrl !== "string" || !Number.isSafeInteger(value.acquiredAt)) return null;
    return value as DevHubLeaseV1;
  } catch {
    return null;
  }
}

/** 🔐️ The live claim at `path`, or `null` (no claim, an unreadable one, or one whose process is gone). */
export function liveDevHubLeaseV1(path: string, alive: (pid: number) => boolean = isPidAliveV1): DevHubLeaseV1 | null {
  const lease = readDevHubLeaseV1(path);
  return lease !== null && alive(lease.pid) ? lease : null;
}

/** 🔗️ Creates `path` holding `lease` only if it does not exist, and never visibly empty: the record is written to a private
 * file first and hard-linked into place, which fails with `EEXIST` atomically. A plain exclusive create exposes the empty
 * file to a racer between creation and write, and a racer reading that as a dead claim moved a live one aside. */
function createDevHubLeaseFile(path: string, lease: DevHubLeaseV1): boolean {
  const staged = `${path}.${process.pid}.${Math.random().toString(36).slice(2)}.tmp`;
  writeFileSync(staged, `${JSON.stringify(lease)}\n`, { flag: "wx", mode: 0o600 });
  try {
    linkSync(staged, path);
    return true;
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "EEXIST") return false;
    throw error;
  } finally {
    rmSync(staged, { force: true });
  }
}

function claimOneDevHubLease(path: string, lease: DevHubLeaseV1, alive: (pid: number) => boolean): DevHubLeaseClaimV1 {
  for (let attempt = 0; attempt < 64; attempt += 1) {
    if (createDevHubLeaseFile(path, lease)) return { kind: "claimed" };
    const held = liveDevHubLeaseV1(path, alive);
    if (held !== null) return held.pid === lease.pid ? { kind: "claimed" } : { kind: "held", lease: held };
    const aside = `${path}.${lease.pid}.${attempt}.stale`;
    try {
      renameSync(path, aside);
    } catch {
      continue;
    }
    const moved = readDevHubLeaseV1(aside);
    rmSync(aside, { force: true });
    if (moved !== null && alive(moved.pid) && moved.pid !== lease.pid) {
      if (!createDevHubLeaseFile(path, moved)) continue;
      return { kind: "held", lease: moved };
    }
  }
  throw new Error(`dev hub lease ${path}: no stable claim after 64 attempts`);
}

/** 🔐️ Claims both leases of one owner — the hub port first, then the data root — atomically per file. A stale claim
 * (its process is gone) is moved aside under a unique name, and a claimer that moved a LIVE claim a racer had just written
 * puts it back, so two claimers racing over one stale claim end with exactly one holder. When the data root is held, the
 * port claim is given back: an owner holds both or neither. */
export function claimDevHubLeaseV1(paths: readonly [string, string], lease: DevHubLeaseV1, alive: (pid: number) => boolean = isPidAliveV1): DevHubLeaseClaimV1 {
  for (const path of paths) mkdirSync(dirname(path), { recursive: true });
  const port = claimOneDevHubLease(paths[0], lease, alive);
  if (port.kind === "held") return port;
  const data = claimOneDevHubLease(paths[1], lease, alive);
  if (data.kind === "held") releaseDevHubLeaseV1([paths[0]], lease.pid);
  return data;
}

/** 🔓️ Gives back every claim `pid` holds among `paths`; another holder's claim is left alone. */
export function releaseDevHubLeaseV1(paths: readonly string[], pid: number): void {
  for (const path of paths) if (readDevHubLeaseV1(path)?.pid === pid) rmSync(path, { force: true });
}
//#endregion 🔖️Lease

//#region 🔖️Catalog
/** 🧬️ The catalog header contract (`DevHubCatalogHeaderV1`) as the owner reads it: the format version and the package
 * record keys the current hub loads. Read from the schema so the schema stays the one place they are declared. */
function devHubCatalogHeaderContract(): Readonly<{ schemaVersion: number; packageKeys: readonly string[] }> {
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8")) as { $defs: Record<string, { properties: { schemaVersion: { const: number }; packages: { items: { required: string[] } } } }> };
  const header = schema.$defs.DevHubCatalogHeaderV1!;
  return { schemaVersion: header.properties.schemaVersion.const, packageKeys: header.properties.packages.items.required };
}

export type DevHubCatalogFreshnessV1 =
  | { readonly kind: "absent" }
  | { readonly kind: "current"; readonly generationId: string }
  | { readonly kind: "stale"; readonly reason: string };

/** 🔍️ Whether the data root's current catalog generation is one the current hub loads. `absent` — no catalog yet;
 * `stale` — a pointer or generation the current publisher would not write (an older format version, a package record
 * missing a key the format now requires, or unreadable files), which the hub refuses at boot. */
export function devHubCatalogFreshnessV1(dataDir: string): DevHubCatalogFreshnessV1 {
  const pointerPath = join(dataDir, "trusted-catalog", "current.json");
  if (!existsSync(pointerPath)) return { kind: "absent" };
  let generationId: string;
  try {
    generationId = String((JSON.parse(readFileSync(pointerPath, "utf8")) as { generationId?: unknown }).generationId ?? "");
  } catch {
    return { kind: "stale", reason: "current.json is unreadable" };
  }
  if (!/^[0-9a-f]{64}$/u.test(generationId)) return { kind: "stale", reason: "current.json names no generation" };
  let header: { schemaVersion?: unknown; profiles?: unknown; packages?: unknown };
  try {
    header = JSON.parse(readFileSync(join(dataDir, "trusted-catalog", "generations", generationId, "trusted-catalog.json"), "utf8")) as typeof header;
  } catch {
    return { kind: "stale", reason: `generation ${generationId.slice(0, 12)} is unreadable` };
  }
  const contract = devHubCatalogHeaderContract();
  if (header.schemaVersion !== contract.schemaVersion) return { kind: "stale", reason: `format ${String(header.schemaVersion)} ≠ ${contract.schemaVersion}` };
  if (!Array.isArray(header.profiles) || header.profiles.length === 0 || !Array.isArray(header.packages) || header.packages.length === 0) return { kind: "stale", reason: "no profiles or packages" };
  for (const record of header.packages as unknown[]) {
    const keys = record !== null && typeof record === "object" ? Object.keys(record) : [];
    const missing = contract.packageKeys.filter((key) => !keys.includes(key));
    if (missing.length > 0) return { kind: "stale", reason: `package ${String((record as { pluginId?: unknown } | null)?.pluginId ?? "?")} has no ${missing.join(", ")}` };
  }
  return { kind: "current", generationId };
}

/** 📤️ Publishes a catalog into a data root, streaming its progress lines, and stops when `signal` aborts. */
export type DevHubCatalogPublisherV1 = (dataDir: string, packages: string, signal: AbortSignal, onLine: (line: string) => void) => Promise<void>;

/** 🔏️ Makes the data root's catalog one the current hub loads: an absent or stale one is (re)published through
 * `publish`, with every progress line reported; a stale one is never handed to the hub, which refuses it at boot.
 * `cancelled` when `signal` aborted the publication (the previous generation stays current). */
export async function ensureCurrentTrustedCatalogV1(
  dataDir: string,
  publish: DevHubCatalogPublisherV1,
  options: Readonly<{ packages?: string; signal?: AbortSignal; report?: (status: DevHubStatusV1) => void }> = {},
): Promise<"current" | "published" | "cancelled"> {
  const freshness = devHubCatalogFreshnessV1(dataDir);
  if (freshness.kind === "current") return "current";
  const report = options.report ?? (() => undefined);
  if (freshness.kind === "stale") report({ kind: "catalog-stale", dataDir, reason: freshness.reason });
  mkdirSync(dataDir, { recursive: true });
  protectOwnerOnly(dataDir, "directory");
  const signal = options.signal ?? new AbortController().signal;
  try {
    await publish(dataDir, options.packages ?? "", signal, (line) => report({ kind: "catalog-publishing", line }));
  } catch (error) {
    if (signal.aborted) {
      report({ kind: "catalog-cancelled", dataDir });
      return "cancelled";
    }
    throw error;
  }
  const after = devHubCatalogFreshnessV1(dataDir);
  if (after.kind !== "current") throw new Error(`dev local hub: the publication left ${dataDir} without a current catalog (${after.kind === "stale" ? after.reason : "absent"})`);
  report({ kind: "catalog-published", dataDir });
  return "published";
}
//#endregion 🔖️Catalog

//#region 🔖️Join
export function parseHubPort(hubUrl: string): number {
  const port = Number(new URL(hubUrl).port || (hubUrl.startsWith("https:") ? 443 : 80));
  if (!Number.isSafeInteger(port) || port <= 0) throw new Error(`dev local hub: invalid hub url ${hubUrl}`);
  return port;
}

async function hubReady(hubUrl: string): Promise<boolean> {
  try {
    const response = await fetch(`${hubUrl}/auth/sessions/me`, { method: "GET", signal: AbortSignal.timeout(1_500) });
    return response.status === 401 || response.status === 200;
  } catch {
    return false;
  }
}

/** 🔌️ The world the join and own paths act on — the real one by default, a double in the laws. */
export type DevHubWorldV1 = Readonly<{
  ready: (hubUrl: string) => Promise<boolean>;
  portInUse: (port: number) => boolean;
  now: () => number;
  sleep: (ms: number) => Promise<void>;
  report: (status: DevHubStatusV1) => void;
  spawnOwner: (hubUrl: string, dataDir: string, signal?: AbortSignal) => Promise<number | null>;
  stopOwner: (pid: number) => Promise<void>;
  alive: (pid: number) => boolean;
  logBytes: (dataDir: string) => number;
}>;

/** 🤝️ Waits for a hub this serve does not own: never starts one, says it is waiting every `intervalMs`, and ends with a
 * typed `joined` or `gave-up` after `boundMs`. */
export async function joinDevHubV1(hubUrl: string, world: DevHubWorldV1, boundMs: number = DEV_HUB_JOIN_BOUND_MS, intervalMs: number = DEV_HUB_STATUS_INTERVAL_MS, signal?: AbortSignal): Promise<Extract<DevHubStatusV1, { kind: "joined" | "gave-up" }>> {
  const started = world.now();
  let reportedAt = Number.NEGATIVE_INFINITY;
  for (;;) {
    if (signal?.aborted) return { kind: "gave-up", hubUrl, waitedMs: world.now() - started };
    if (await world.ready(hubUrl)) {
      if (signal?.aborted) return { kind: "gave-up", hubUrl, waitedMs: world.now() - started };
      const joined = { kind: "joined", hubUrl } as const;
      world.report(joined);
      return joined;
    }
    const waitedMs = world.now() - started;
    if (waitedMs >= boundMs) {
      const gaveUp = { kind: "gave-up", hubUrl, waitedMs } as const;
      world.report(gaveUp);
      return gaveUp;
    }
    if (waitedMs - reportedAt >= intervalMs) {
      reportedAt = waitedMs;
      world.report({ kind: "waiting", hubUrl, waitedMs, boundMs });
    }
    await world.sleep(Math.min(1_000, Math.max(0, boundMs - waitedMs)));
  }
}

/** 🚀️ Brings the DEFAULT hub up: joins it when it answers, waits for its live owner when one holds its port or data root
 * (a hub that has not bound yet is still owned), refuses to start a second hub on a port someone else holds, and otherwise
 * starts one owner. Readiness is bounded by no progress (the owner's log stops growing) rather than wall time, because a
 * clean data root first publishes its catalog. */
export async function ownDevHubV1(hubUrl: string, dataDir: string, leaseRoot: string, world: DevHubWorldV1, signal?: AbortSignal): Promise<boolean> {
  if (signal?.aborted) return false;
  const paths = devHubLeasePathsV1(leaseRoot, parseHubPort(hubUrl), dataDir);
  if (await world.ready(hubUrl)) {
    if (signal?.aborted) return false;
    world.report({ kind: "joined", hubUrl });
    return true;
  }
  const holder = () => liveDevHubLeaseV1(paths[0], world.alive) ?? liveDevHubLeaseV1(paths[1], world.alive);
  const held = holder();
  let spawned: number | null = null;
  if (held !== null) world.report({ kind: "owned", hubUrl, pid: held.pid });
  else if (world.portInUse(parseHubPort(hubUrl))) {
    world.report({ kind: "port-taken", hubUrl });
    return false;
  } else {
    spawned = await world.spawnOwner(hubUrl, dataDir, signal);
    if (spawned === null) return false;
    world.report({ kind: "starting", hubUrl, pid: spawned });
  }
  let handedOff = false;
  try {
  let observation = "";
  let observedAt = world.now();
  for (;;) {
    if (signal?.aborted) return false;
    if (await world.ready(hubUrl)) {
    if (signal?.aborted) return false;
      world.report({ kind: "joined", hubUrl });
      handedOff = true;
      return true;
    }
    const owner = holder();
    if (owner === null && (spawned === null || !world.alive(spawned))) return false;
    const next = `${owner?.pid ?? spawned}:${world.logBytes(owner?.dataDir ?? dataDir)}`;
    const now = world.now();
    if (next !== observation) {
      observation = next;
      observedAt = now;
    } else if (now - observedAt >= DEV_LOCAL_HUB_READINESS_STALL_BOUND_MS) return false;
    await world.sleep(1_000);
  }
  } finally { if (!handedOff && spawned !== null) await world.stopOwner(spawned); }
}

function ownerLogBytes(dataDir: string): number {
  try {
    return statSync(join(dataDir, DEV_LOCAL_HUB_LOG_FILE)).size;
  } catch {
    return 0;
  }
}

/** 🚀️ Starts the detached owner for `dataDir` (its output goes to `<dataDir>/local-hub.log`). */
async function stopDevLocalHubOwner(child: ChildProcess): Promise<void> {
  if (child.pid === undefined || child.exitCode !== null || child.signalCode !== null) return;
  terminateOwnedProcessTree(child.pid);
  const deadline = Date.now() + 2_000;
  while (child.exitCode === null && child.signalCode === null && Date.now() < deadline) await new Promise<void>((done) => setTimeout(done, 20));
  if (child.exitCode === null && child.signalCode === null) child.kill("SIGKILL");
}

async function spawnDevLocalHubOwner(repoRoot: string, hubUrl: string, dataDir: string, provider: DevLocalHubProviderV1 | null, children: Map<number, ChildProcess>, signal?: AbortSignal): Promise<number | null> {
  if (provider === null || signal?.aborted) return null;
  mkdirSync(dataDir, { recursive: true });
  protectOwnerOnly(dataDir, "directory");
  const log = openSync(join(dataDir, DEV_LOCAL_HUB_LOG_FILE), "a", 0o600);
  let child: ChildProcess;
  try {
    child = spawn(provider.owner.program, [...provider.owner.args, hubUrl, dataDir], { cwd: repoRoot, env: { ...process.env, OS_HUB_DATA: dataDir }, stdio: ["ignore", log, log], detached: true, shell: false });
  } catch { return null; }
  finally { closeSync(log); }
  const started = await new Promise<boolean>((done) => {
    const finish = (accepted: boolean) => { clearTimeout(timer); signal?.removeEventListener("abort", cancelled); done(accepted); };
    const cancelled = () => finish(false);
    const timer = setTimeout(cancelled, 5_000);
    child.once("spawn", () => finish(!signal?.aborted));
    child.on("error", () => finish(false));
    signal?.addEventListener("abort", cancelled, { once: true });
    if (signal?.aborted) cancelled();
  });
  if (!started || child.pid === undefined || signal?.aborted) { await stopDevLocalHubOwner(child); return null; }
  const pid = child.pid;
  children.set(pid, child);
  child.once("exit", () => children.delete(pid));
  child.unref();
  return pid;
}

/** 🔌️ The real world of {@link ownDevHubV1} and {@link joinDevHubV1}, reporting in the terminal's language. */
export function devHubWorldV1(repoRoot: string, locale: DevHubLocaleV1 = devHubLocaleV1(), provider: DevLocalHubProviderV1 | null = null): DevHubWorldV1 {
  const children = new Map<number, ChildProcess>();
  return {
    ready: hubReady,
    portInUse: (port) => isDevPortInUse("127.0.0.1", port),
    now: () => Date.now(),
    sleep: (ms) => new Promise<void>((resolveDelay) => setTimeout(resolveDelay, ms)),
    report: (status) => (status.kind === "gave-up" || status.kind === "port-taken" ? console.warn : console.log)(devHubStatusTextV1(status, locale)),
    spawnOwner: (hubUrl, dataDir, signal) => spawnDevLocalHubOwner(repoRoot, hubUrl, dataDir, provider, children, signal),
    stopOwner: async (pid) => { const child = children.get(pid); if (child) await stopDevLocalHubOwner(child); },
    alive: isPidAliveV1,
    logBytes: ownerLogBytes,
  };
}
//#endregion 🔖️Join

/** 🗄️ The development hub data root: `OS_HUB_DATA`, else the repository's `hub-dev` root. */
export function devLocalHubDataDir(repoRoot: string, env: NodeJS.ProcessEnv = process.env): string {
  return resolve(env.OS_HUB_DATA ?? join(repoRoot, ".🧬semio", "🌐hub", "hub-dev"));
}

/** 🌐️ Brings up the default loopback development hub, or joins the one named by `hubUrl` / `S_HUB_URL`, and proves a
 * session for this serve's profile through the owner's broker. `null` when local-only was asked for, when an explicit hub
 * did not answer within its bound, or when no default hub came up; a hub someone else operates (no broker for this data
 * root) is joined without a session, and the shell's own sign-in stays available. `S_HUB_URL` names the hub this process
 * serves with only once one answers: a serve that continues local-first hands its Vite no hub, so the shell never proxies
 * `/_semio/hub/*` (its trusted plugin catalog first) to a hub that is not there (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-
 * EDITING follow-up 3: `GET /_semio/hub/trusted-catalog/plugin-modules` answered 500 at every boot of a hub-less serve). */
export async function ensureDevLocalHub(
  repoRoot: string,
  options: { readonly hubUrl?: string; readonly dataDir?: string; readonly profileId?: string; readonly world?: DevHubWorldV1; readonly joinBoundMs?: number; readonly provider?: DevLocalHubProviderV1; readonly signal?: AbortSignal } = {},
): Promise<DevLocalHubSession | null> {
  if (process.env.S_LOCAL_ONLY === "1" || process.env.S_LOCAL_ONLY === "true") {
    delete process.env.S_HUB_URL;
    return null;
  }
  const explicit = options.hubUrl ?? process.env.S_HUB_URL;
  const role = devHubRoleV1(explicit);
  const hubUrl = (role === "join" ? explicit! : DEV_LOCAL_HUB_DEFAULT_URL).trim().replace(/\/+$/u, "");
  const dataDir = options.dataDir ?? devLocalHubDataDir(repoRoot);
  const provider = options.provider ?? (process.env[DEV_LOCAL_HUB_PROVIDER_ENV] ? parseDevLocalHubProviderV1(JSON.parse(process.env[DEV_LOCAL_HUB_PROVIDER_ENV]!)) : null);
  const profileId = options.profileId ?? process.env[DEV_LOCAL_HUB_PROFILE_ENV] ?? provider?.defaultProfileId ?? "";
  const world = options.world ?? devHubWorldV1(repoRoot, devHubLocaleV1(), provider);
  delete process.env.S_HUB_URL;
  const up = role === "join" ? (await joinDevHubV1(hubUrl, world, options.joinBoundMs ?? DEV_HUB_JOIN_BOUND_MS, DEV_HUB_STATUS_INTERVAL_MS, options.signal)).kind === "joined" : await ownDevHubV1(hubUrl, dataDir, devHubLeaseRootV1(repoRoot), world, options.signal);
  if (!up || options.signal?.aborted) return null;
  const session = await requestLocalBrokerSession(dataDir, hubUrl, profileId, options.signal).catch(() => null);
  if (options.signal?.aborted) return null;
  process.env.S_HUB_URL = hubUrl;
  if (session === null) {
    world.report({ kind: "no-broker", hubUrl, dataDir });
    return { hubUrl, dataDir, profileId, userId: "" };
  }
  world.report({ kind: "session", hubUrl, userId: session.userId, profileId });
  return { hubUrl, dataDir, profileId, userId: session.userId };
}

//#region 🔖️DevServeFixture
/** 🛎️ How long {@link ensureDevServe} waits for a serve it started to answer (a cold `s` Vite boot measured 55 s under a
 * busy fleet, ticket 26/09/23 S18). */
export const DEV_SERVE_BOOT_BOUND_MS = 300_000;
/** 📣️ How often a booting serve is reported. */
export const DEV_SERVE_STATUS_INTERVAL_MS = 5_000;
/** 🚫️ Ports a serve fixture never binds: the canonical hub's. */
export const DEV_SERVE_REFUSED_PORTS: readonly number[] = [7800];
const DEV_SERVE_STOP_BOUND_MS = 15_000;

/** 🧭️ What the fixture does with a port: `reuse` a serve that already answers (and never stops it), `spawn` one on a free
 * port, refuse the canonical hub port, refuse a port someone else holds without answering as a serve. */
export type DevServePlanV1 = "reuse" | "spawn" | "refuse-hub-port" | "refuse-occupied";

export function devServePlanV1(port: number, answers: boolean, occupied: boolean): DevServePlanV1 {
  if (DEV_SERVE_REFUSED_PORTS.includes(port)) return "refuse-hub-port";
  if (answers) return "reuse";
  return occupied ? "refuse-occupied" : "spawn";
}

/** 🔢️ The loopback port a `--serve <url>` names; a serve on another host can only be reused, never started, so it is
 * refused here. */
export function devServePortV1(serveUrl: string): number {
  const url = new URL(serveUrl);
  if (url.protocol !== "http:" || !["127.0.0.1", "localhost", "[::1]"].includes(url.hostname)) throw new Error(`dev serve: ${serveUrl} is not a loopback http url`);
  const port = Number(url.port || 80);
  if (!Number.isSafeInteger(port) || port <= 0 || port > 65_535) throw new Error(`dev serve: invalid port in ${serveUrl}`);
  return port;
}

/** 📣️ Everything the fixture reports, as data; the line is {@link devServeStatusTextV1}. */
export type DevServeStatusV1 =
  | { readonly kind: "reusing"; readonly url: string }
  | { readonly kind: "spawning"; readonly url: string; readonly pid: number; readonly logPath: string }
  | { readonly kind: "booting"; readonly url: string; readonly waitedMs: number; readonly boundMs: number }
  | { readonly kind: "ready"; readonly url: string; readonly waitedMs: number }
  | { readonly kind: "stopped"; readonly url: string };

export function devServeStatusTextV1(status: DevServeStatusV1, locale: DevHubLocaleV1): string {
  const de = locale === "de";
  switch (status.kind) {
    case "reusing":
      return de ? `[dev-serve] ${status.url} antwortet bereits — wird mitbenutzt und nicht beendet` : `[dev-serve] ${status.url} already answers — reusing it, never stopping it`;
    case "spawning":
      return de ? `[dev-serve] starte die Shell unter ${status.url} (Prozess ${status.pid}, Protokoll ${status.logPath})` : `[dev-serve] starting the shell at ${status.url} (process ${status.pid}, log ${status.logPath})`;
    case "booting":
      return de ? `[dev-serve] warte auf ${status.url} (${seconds(status.waitedMs)} von ${seconds(status.boundMs)} s)` : `[dev-serve] waiting for ${status.url} (${seconds(status.waitedMs)} of ${seconds(status.boundMs)} s)`;
    case "ready":
      return de ? `[dev-serve] ${status.url} bereit nach ${seconds(status.waitedMs)} s` : `[dev-serve] ${status.url} ready after ${seconds(status.waitedMs)} s`;
    case "stopped":
      return de ? `[dev-serve] ${status.url} beendet` : `[dev-serve] stopped ${status.url}`;
  }
}

/** 🖥️ Which shell a serve hosts: the React shell (`🧑‍💻dev` `serve <variant> react <profile>`) or the wgpu browser shell
 * (`🧊️wgpu/🌐️server` `serve <variant> <profile> --port <port>`). */
export type DevServeRendererV1 = "react" | "wgpu";
export type DevServeProfileV1 = "dev" | "release";

/** 🧾️ What the fixture asks its world to start (`DevServeSpawnRequestV1`, `🧑‍💻dev/🧬️schema/🔣️.json`): one shell serve on
 * `port`, local-only unless `hubUrl` names the hub it joins, output in `logPath`. */
export type DevServeSpawnRequestV1 = Readonly<{ port: number; variant: string; renderer: DevServeRendererV1; profile: DevServeProfileV1; hubUrl: string | null; logPath: string }>;

/** 🧾️ The process a spawn request becomes: `bun <script> …args` in `cwd` (all repo-relative), with `env` over the
 * caller's environment — a `null` value removes the variable, so a local-only serve never inherits a hub. */
export type DevServeCommandV1 = Readonly<{ script: string; args: readonly string[]; cwd: string; env: Readonly<Record<string, string | null>> }>;

const DEV_SERVE_REACT_SCRIPT = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts";
const DEV_SERVE_WGPU_SCRIPT = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts";

/** 🧾️ Both spawn shapes, as data: HMR is off for either shell so no peer's edit reloads a page mid-run. */
export function devServeCommandV1(request: DevServeSpawnRequestV1): DevServeCommandV1 {
  const link = request.hubUrl === null ? { S_LOCAL_ONLY: "1", S_HUB_URL: null } : { S_LOCAL_ONLY: null, S_HUB_URL: request.hubUrl };
  const env = { SEMIO_PLUGIN: request.variant, SEMIO_RENDERER: request.renderer, SEMIO_VITE_HMR: "0", S_OS_PORT: String(request.port), ...link };
  return request.renderer === "react"
    ? { script: DEV_SERVE_REACT_SCRIPT, args: ["serve", request.variant, "react", request.profile], cwd: dirname(DEV_SERVE_REACT_SCRIPT), env }
    : { script: DEV_SERVE_WGPU_SCRIPT, args: ["serve", request.variant, request.profile, "--port", String(request.port)], cwd: dirname(DEV_SERVE_WGPU_SCRIPT), env };
}

/** 🔌️ The world the fixture acts on — the real one by default, a double in the laws. */
export type DevServeWorldV1 = Readonly<{
  answers: (url: string) => Promise<boolean>;
  portInUse: (port: number) => Promise<boolean>;
  spawnServe: (request: DevServeSpawnRequestV1) => Readonly<{ pid: number; exited: () => boolean }>;
  terminate: (pid: number) => void;
  now: () => number;
  sleep: (ms: number) => Promise<void>;
}>;

/** 🛎️ A serve a harness runs against: `url` to drive, `reused` when it was already there, `stop()` ends only what the
 * fixture itself started. */
export type DevServerV1 = Readonly<{ url: string; reused: boolean; stop: () => Promise<void> }>;

export type DevServeOptionsV1 = Readonly<{
  repoRoot: string;
  port: number;
  variant?: string;
  renderer?: DevServeRendererV1;
  profile?: DevServeProfileV1;
  locale?: DevHubLocaleV1;
  hubUrl?: string;
  signal?: AbortSignal;
  onProgress?: (status: DevServeStatusV1, line: string) => void;
  beforeSpawn?: () => Promise<void>;
  bootBoundMs?: number;
  intervalMs?: number;
  logPath?: string;
  world?: DevServeWorldV1;
}>;

async function devServePortHeld(port: number): Promise<boolean> {
  return await new Promise<boolean>((resolveHeld) => {
    const socket = createConnection({ host: "127.0.0.1", port });
    const settle = (held: boolean): void => {
      socket.destroy();
      resolveHeld(held);
    };
    socket.setTimeout(2_000);
    socket.once("connect", () => settle(true));
    socket.once("timeout", () => settle(false));
    socket.once("error", () => settle(false));
  });
}

async function devServeAnswers(url: string): Promise<boolean> {
  try {
    const response = await fetch(url, { method: "GET", signal: AbortSignal.timeout(3_000) });
    await response.body?.cancel();
    return response.ok;
  } catch {
    return false;
  }
}

/** 🌍️ The real world: {@link devServeCommandV1} under `bun` in its own process group, output in `logPath`. */
export function devServeWorldV1(repoRoot: string): DevServeWorldV1 {
  return {
    answers: devServeAnswers,
    portInUse: devServePortHeld,
    spawnServe: (request) => {
      mkdirSync(dirname(request.logPath), { recursive: true });
      const log = openSync(request.logPath, "a");
      const command = devServeCommandV1(request);
      const env: NodeJS.ProcessEnv = { ...process.env };
      for (const [name, value] of Object.entries(command.env)) {
        if (value === null) delete env[name];
        else env[name] = value;
      }
      const child = spawn(process.execPath, [join(repoRoot, command.script), ...command.args], { cwd: join(repoRoot, command.cwd), env, stdio: ["ignore", log, log], detached: process.platform !== "win32", windowsHide: true });
      closeSync(log);
      child.unref();
      if (!child.pid) throw new Error(`dev serve: could not start the serve for port ${request.port}`);
      return { pid: child.pid, exited: () => child.exitCode !== null || child.signalCode !== null };
    },
    terminate: terminateOwnedProcessTree,
    now: () => Date.now(),
    sleep: (ms) => new Promise<void>((resolveDelay) => setTimeout(resolveDelay, ms)),
  };
}

/** 🛎️ The ONE shared serve fixture every browser harness runs against (ticket 26/09/23 S18, R10's productization
 * contract): reuses a serve that already answers on `port` (and never stops it), otherwise starts the shell serve there
 * ({@link devServeCommandV1}: `renderer` react or wgpu, `profile` dev or release; default the `s` React dev serve) —
 * local-only, or joined to `hubUrl` — reports its boot with progress, honours `signal` (a cancelled boot stops what
 * it started), and hands back `stop()` for exactly what it started. The canonical hub port and a port held by something
 * that is not a serve are refused, never shared. `locale` is the language of the progress lines only; a harness seats the
 * shell's own language itself. */
export async function ensureDevServe(options: DevServeOptionsV1): Promise<DevServerV1> {
  const world = options.world ?? devServeWorldV1(options.repoRoot);
  const locale = options.locale ?? devHubLocaleV1();
  const url = `http://127.0.0.1:${options.port}/`;
  const report = (status: DevServeStatusV1): void => options.onProgress?.(status, devServeStatusTextV1(status, locale));
  options.signal?.throwIfAborted();
  const plan = devServePlanV1(options.port, await world.answers(url), await world.portInUse(options.port));
  if (plan === "refuse-hub-port") throw new Error(`dev serve: port ${options.port} is the canonical hub's, never a serve's`);
  if (plan === "refuse-occupied") throw new Error(`dev serve: port ${options.port} is held by a process that does not answer as a serve`);
  if (plan === "reuse") {
    report({ kind: "reusing", url });
    return { url, reused: true, stop: async () => {} };
  }
  await options.beforeSpawn?.();
  options.signal?.throwIfAborted();
  if (options.beforeSpawn) {
    const preparedPlan = devServePlanV1(options.port, await world.answers(url), await world.portInUse(options.port));
    if (preparedPlan === "reuse") {
      report({ kind: "reusing", url });
      return { url, reused: true, stop: async () => {} };
    }
    if (preparedPlan !== "spawn") throw new Error(`dev serve: port ${options.port} was occupied during preparation`);
  }
  options.signal?.throwIfAborted();
  const logPath = options.logPath ?? join(options.repoRoot, ".🧬semio", "🌐hub", "dev-serves", `serve-${options.port}.log`);
  const child = world.spawnServe({ port: options.port, variant: options.variant ?? "s", renderer: options.renderer ?? "react", profile: options.profile ?? "dev", hubUrl: options.hubUrl?.trim().replace(/\/+$/u, "") || null, logPath });
  report({ kind: "spawning", url, pid: child.pid, logPath });
  let stopped: Promise<void> | null = null;
  const stop = (): Promise<void> =>
    (stopped ??= (async () => {
      world.terminate(child.pid);
      const started = world.now();
      while ((await world.portInUse(options.port)) && world.now() - started < DEV_SERVE_STOP_BOUND_MS) await world.sleep(200);
      report({ kind: "stopped", url });
    })());
  const boundMs = options.bootBoundMs ?? DEV_SERVE_BOOT_BOUND_MS;
  const intervalMs = options.intervalMs ?? DEV_SERVE_STATUS_INTERVAL_MS;
  const started = world.now();
  let reportedAt = started;
  try {
    for (;;) {
      options.signal?.throwIfAborted();
      if (await world.answers(url)) {
        report({ kind: "ready", url, waitedMs: world.now() - started });
        return { url, reused: false, stop };
      }
      if (child.exited()) throw new Error(`dev serve: the serve for ${url} exited before it answered — see ${logPath}`);
      const waitedMs = world.now() - started;
      if (waitedMs >= boundMs) throw new Error(`dev serve: ${url} did not answer within ${seconds(boundMs)} s — see ${logPath}`);
      if (world.now() - reportedAt >= intervalMs) {
        reportedAt = world.now();
        report({ kind: "booting", url, waitedMs, boundMs });
      }
      await world.sleep(500);
    }
  } catch (error) {
    await stop();
    throw error;
  }
}
//#endregion 🔖️DevServeFixture
