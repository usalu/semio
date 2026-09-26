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
 * @see ../../../../../../../🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts */

import { spawn } from "node:child_process";
import { existsSync, linkSync, mkdirSync, openSync, readFileSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { BundleScript, isDevPortInUse } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { protectOwnerOnly } from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🔐️owner-only/🟦️.ts";
import { requestLocalBrokerSession, startLocalSessionBroker } from "../../../../../../../🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import {
  finishLocalHub,
  hubDevBinaryPath,
  LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES,
  LOCAL_HUB_DEVELOPMENT_PROFILES,
  startLocalHub,
  TRUSTED_CATALOG_READINESS_STALL_BOUND_MS,
  waitForReadiness,
} from "../../../../../../../🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts";

export const DEV_LOCAL_HUB_DEFAULT_URL = "http://127.0.0.1:8787";
/** 🗄️ Env naming the data root whose session broker a serve's `/_semio/dev/local-session` asks
 * (`📇️directory/🎫️local-session`). */
export const DEV_LOCAL_HUB_DATA_ENV = "SEMIO_DEV_LOCAL_HUB_DATA";
/** 👤️ Env naming the local profile a serve signs in as (`developer`, `user-1`, `user-2`). */
export const DEV_LOCAL_HUB_PROFILE_ENV = "SEMIO_DEV_LOCAL_HUB_PROFILE";
const DEV_LOCAL_HUB_DEFAULT_PROFILE = "developer";
const DEV_LOCAL_HUB_OWNER_FILE = "local-hub-owner.json";
const DEV_LOCAL_HUB_LOG_FILE = "local-hub.log";
const DEV_LOCAL_HUB_OWNER_SCRIPT = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts";
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

/** 🛑️ Ends a child and everything it started (the publisher's `nx` → `cargo` → `rustc` chain): its process group on POSIX,
 * `taskkill /T` on Windows. Killing only the wrapper would orphan the builds under it. */
function terminateProcessTree(pid: number | undefined): void {
  if (pid === undefined) return;
  try {
    if (process.platform === "win32") spawn("taskkill", ["/pid", String(pid), "/T", "/F"], { stdio: "ignore", shell: false });
    else process.kill(-pid, "SIGTERM");
  } catch {
    return;
  }
}

/** 📤️ The hub's own product verb (`os-hub:trusted-catalog-bootstrap`), as a child the signal terminates; the publisher
 * writes a new immutable generation and moves `current.json` only after a candidate hub loaded it, so a cancelled run
 * leaves the previous generation in place.
 * @see ../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts TrustedCatalogBootstrapScript */
export function devHubCatalogBootstrapPublisherV1(repoRoot: string): DevHubCatalogPublisherV1 {
  return (dataDir, packages, signal, onLine) =>
    new Promise<void>((resolvePublished, rejectPublished) => {
      const child = spawn("bun", ["nx", "run", "os-hub:trusted-catalog-bootstrap", "--packages", packages, "--outputStyle=stream"], { cwd: repoRoot, env: { ...process.env, OS_HUB_DATA: dataDir }, stdio: ["ignore", "pipe", "pipe"], shell: false, detached: process.platform !== "win32" });
      const abort = (): void => terminateProcessTree(child.pid);
      signal.addEventListener("abort", abort, { once: true });
      let pending = "";
      const lines = (chunk: Buffer): void => {
        pending += chunk.toString("utf8");
        const parts = pending.split("\n");
        pending = parts.pop() ?? "";
        for (const line of parts) if (line.trim().length > 0) onLine(line);
      };
      child.stdout.on("data", lines);
      child.stderr.on("data", lines);
      child.once("error", rejectPublished);
      child.once("exit", (code, exitSignal) => {
        signal.removeEventListener("abort", abort);
        if (signal.aborted) rejectPublished(new DOMException("catalog publication cancelled", "AbortError"));
        else if (code === 0) resolvePublished();
        else rejectPublished(new Error(`os-hub:trusted-catalog-bootstrap exited ${code ?? exitSignal}`));
      });
    });
}

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
    await publish(dataDir, options.packages ?? LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES, signal, (line) => report({ kind: "catalog-publishing", line }));
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
function parseHubPort(hubUrl: string): number {
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
  spawnOwner: (hubUrl: string, dataDir: string) => number | null;
  alive: (pid: number) => boolean;
  logBytes: (dataDir: string) => number;
}>;

/** 🤝️ Waits for a hub this serve does not own: never starts one, says it is waiting every `intervalMs`, and ends with a
 * typed `joined` or `gave-up` after `boundMs`. */
export async function joinDevHubV1(hubUrl: string, world: DevHubWorldV1, boundMs: number = DEV_HUB_JOIN_BOUND_MS, intervalMs: number = DEV_HUB_STATUS_INTERVAL_MS): Promise<Extract<DevHubStatusV1, { kind: "joined" | "gave-up" }>> {
  const started = world.now();
  let reportedAt = Number.NEGATIVE_INFINITY;
  for (;;) {
    if (await world.ready(hubUrl)) {
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
export async function ownDevHubV1(hubUrl: string, dataDir: string, leaseRoot: string, world: DevHubWorldV1): Promise<boolean> {
  const paths = devHubLeasePathsV1(leaseRoot, parseHubPort(hubUrl), dataDir);
  if (await world.ready(hubUrl)) {
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
    spawned = world.spawnOwner(hubUrl, dataDir);
    if (spawned === null) return false;
    world.report({ kind: "starting", hubUrl, pid: spawned });
  }
  let observation = "";
  let observedAt = world.now();
  for (;;) {
    if (await world.ready(hubUrl)) {
      world.report({ kind: "joined", hubUrl });
      return true;
    }
    const owner = holder();
    if (owner === null && (spawned === null || !world.alive(spawned))) return false;
    const next = `${owner?.pid ?? spawned}:${world.logBytes(owner?.dataDir ?? dataDir)}`;
    const now = world.now();
    if (next !== observation) {
      observation = next;
      observedAt = now;
    } else if (now - observedAt >= TRUSTED_CATALOG_READINESS_STALL_BOUND_MS) return false;
    await world.sleep(1_000);
  }
}

function ownerLogBytes(dataDir: string): number {
  try {
    return statSync(join(dataDir, DEV_LOCAL_HUB_LOG_FILE)).size;
  } catch {
    return 0;
  }
}

/** 🚀️ Starts the detached owner for `dataDir` (its output goes to `<dataDir>/local-hub.log`). */
function spawnDevLocalHubOwner(repoRoot: string, hubUrl: string, dataDir: string): number | null {
  mkdirSync(dataDir, { recursive: true });
  protectOwnerOnly(dataDir, "directory");
  const log = openSync(join(dataDir, DEV_LOCAL_HUB_LOG_FILE), "a", 0o600);
  const child = spawn("bun", [join(repoRoot, DEV_LOCAL_HUB_OWNER_SCRIPT), "local-hub", hubUrl, dataDir], { cwd: repoRoot, env: { ...process.env, OS_HUB_DATA: dataDir }, stdio: ["ignore", log, log], detached: true, shell: false });
  child.unref();
  return child.pid ?? null;
}

/** 🔌️ The real world of {@link ownDevHubV1} and {@link joinDevHubV1}, reporting in the terminal's language. */
export function devHubWorldV1(repoRoot: string, locale: DevHubLocaleV1 = devHubLocaleV1()): DevHubWorldV1 {
  return {
    ready: hubReady,
    portInUse: (port) => isDevPortInUse("127.0.0.1", port),
    now: () => Date.now(),
    sleep: (ms) => new Promise<void>((resolveDelay) => setTimeout(resolveDelay, ms)),
    report: (status) => (status.kind === "gave-up" || status.kind === "port-taken" ? console.warn : console.log)(devHubStatusTextV1(status, locale)),
    spawnOwner: (hubUrl, dataDir) => spawnDevLocalHubOwner(repoRoot, hubUrl, dataDir),
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
 * root) is joined without a session, and the shell's own sign-in stays available. */
export async function ensureDevLocalHub(
  repoRoot: string,
  options: { readonly hubUrl?: string; readonly dataDir?: string; readonly profileId?: string; readonly world?: DevHubWorldV1; readonly joinBoundMs?: number } = {},
): Promise<DevLocalHubSession | null> {
  if (process.env.S_LOCAL_ONLY === "1" || process.env.S_LOCAL_ONLY === "true") return null;
  const explicit = options.hubUrl ?? process.env.S_HUB_URL;
  const role = devHubRoleV1(explicit);
  const hubUrl = (role === "join" ? explicit! : DEV_LOCAL_HUB_DEFAULT_URL).trim().replace(/\/+$/u, "");
  const dataDir = options.dataDir ?? devLocalHubDataDir(repoRoot);
  const profileId = options.profileId ?? process.env[DEV_LOCAL_HUB_PROFILE_ENV] ?? DEV_LOCAL_HUB_DEFAULT_PROFILE;
  const world = options.world ?? devHubWorldV1(repoRoot);
  process.env.S_HUB_URL = hubUrl;
  const up = role === "join" ? (await joinDevHubV1(hubUrl, world, options.joinBoundMs ?? DEV_HUB_JOIN_BOUND_MS)).kind === "joined" : await ownDevHubV1(hubUrl, dataDir, devHubLeaseRootV1(repoRoot), world);
  if (!up) return null;
  const session = await requestLocalBrokerSession(dataDir, hubUrl, profileId).catch(() => null);
  if (session === null) {
    world.report({ kind: "no-broker", hubUrl, dataDir });
    return { hubUrl, dataDir, profileId, userId: "" };
  }
  world.report({ kind: "session", hubUrl, userId: session.userId, profileId });
  return { hubUrl, dataDir, profileId, userId: session.userId };
}

/** 🗄️ `local-hub [hubUrl] [dataDir]`: the owner process — claims the hub port and the data root, makes the catalog one the
 * current hub loads (republishing a stale one, cancelled by SIGINT/SIGTERM), stages the hub binary, boots the hub with the
 * development profiles, runs the session broker, and holds until signalled. An owner that finds either claim held by a
 * live owner, or the port bound by anyone, exits at once. */
export class DevLocalHubScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const [hubArg, dataArg] = segments;
    const hubUrl = (hubArg ?? DEV_LOCAL_HUB_DEFAULT_URL).replace(/\/+$/u, "");
    const dataDir = dataArg ? resolve(dataArg) : devLocalHubDataDir(this.repoRoot);
    const locale = devHubLocaleV1();
    const report = (status: DevHubStatusV1): void => console.log(devHubStatusTextV1(status, locale));
    const port = parseHubPort(hubUrl);
    const paths = devHubLeasePathsV1(devHubLeaseRootV1(this.repoRoot), port, dataDir);
    mkdirSync(dataDir, { recursive: true });
    protectOwnerOnly(dataDir, "directory");
    const claim = claimDevHubLeaseV1(paths, { pid: process.pid, port, dataDir, hubUrl, acquiredAt: Date.now() });
    if (claim.kind === "held") {
      report({ kind: "owned", hubUrl, pid: claim.lease.pid });
      return;
    }
    const cancel = new AbortController();
    const stopPublishing = (): void => cancel.abort();
    process.once("SIGINT", stopPublishing);
    process.once("SIGTERM", stopPublishing);
    try {
      if (isDevPortInUse("127.0.0.1", port)) {
        report({ kind: "port-taken", hubUrl });
        return;
      }
      const catalog = await ensureCurrentTrustedCatalogV1(dataDir, devHubCatalogBootstrapPublisherV1(this.repoRoot), { signal: cancel.signal, report });
      process.off("SIGINT", stopPublishing);
      process.off("SIGTERM", stopPublishing);
      if (catalog === "cancelled") return;
      const hubPkg = join(this.repoRoot, "🌎️hub", "📦️packages", "🦀️rust");
      const binaryPath = hubDevBinaryPath(hubPkg);
      process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
      const adminToken = process.env.OS_HUB_ADMIN_TOKEN ?? "dev-local-hub-admin";
      const run = await startLocalHub(this.repoRoot, hubPkg, LOCAL_HUB_DEVELOPMENT_PROFILES, { port, dataDir, binaryPath, adminToken, capture: false });
      let broker: ReturnType<typeof startLocalSessionBroker> | null = null;
      const stop = (): void => {
        broker?.stop();
        void finishLocalHub(run);
      };
      process.once("SIGINT", stop);
      process.once("SIGTERM", stop);
      try {
        await waitForReadiness(run, false, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS);
        broker = startLocalSessionBroker(run, dataDir, LOCAL_HUB_DEVELOPMENT_PROFILES, 2);
        console.log(`[dev-local-hub] ready at ${hubUrl}; session broker for ${broker.record.profiles.join(",")}`);
        await new Promise<void>((resolveExit) => (run.child.exitCode !== null ? resolveExit() : run.child.once("exit", () => resolveExit())));
      } finally {
        process.off("SIGINT", stop);
        process.off("SIGTERM", stop);
        broker?.stop();
        await finishLocalHub(run);
      }
    } finally {
      process.off("SIGINT", stopPublishing);
      process.off("SIGTERM", stopPublishing);
      releaseDevHubLeaseV1(paths, process.pid);
    }
  }
}
