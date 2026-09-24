/** 🚀️ Zero-touch loopback development hub for `dev s`: ONE detached owner per data root holds the hub's local-bootstrap
 * pipe and its session broker, and every `s` serve — single-user rows and both two-user rows — signs in through that
 * broker. Which launch row reaches a clean data root first (`▶️start`, a `dev s` row, the compound rows) no longer decides
 * who can sign in, and stopping one UI never takes the hub away from another.
 * @see ../../../../../../../🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts */

import { spawn, spawnSync } from "node:child_process";
import { chmodSync, existsSync, mkdirSync, openSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { BundleScript, isDevPortInUse } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
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
export const DEV_LOCAL_HUB_SESSION_PATH = "/_semio/dev/local-session";
/** 🗄️ Env naming the data root whose session broker a serve's `/_semio/dev/local-session` asks. */
export const DEV_LOCAL_HUB_DATA_ENV = "SEMIO_DEV_LOCAL_HUB_DATA";
/** 👤️ Env naming the local profile a serve signs in as (`developer`, `user-1`, `user-2`). */
export const DEV_LOCAL_HUB_PROFILE_ENV = "SEMIO_DEV_LOCAL_HUB_PROFILE";
const DEV_LOCAL_HUB_DEFAULT_PROFILE = "developer";
const DEV_LOCAL_HUB_OWNER_FILE = "local-hub-owner.json";
const DEV_LOCAL_HUB_LOG_FILE = "local-hub.log";
const DEV_LOCAL_HUB_OWNER_SCRIPT = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts";

export type DevLocalHubSession = Readonly<{ hubUrl: string; dataDir: string; profileId: string; userId: string }>;

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

/** 🗄️ The development hub data root: `OS_HUB_DATA`, else the repository's `hub-dev` root. */
export function devLocalHubDataDir(repoRoot: string, env: NodeJS.ProcessEnv = process.env): string {
  return resolve(env.OS_HUB_DATA ?? join(repoRoot, ".🧬semio", "🌐hub", "hub-dev"));
}

/** 🔏️ Publishes the development trusted catalog ({@link LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES}) into `dataDir` through the
 * hub's own product verb (`os-hub:trusted-catalog-bootstrap`) when that root holds none — never a copy of another root.
 * @see ../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts TrustedCatalogBootstrapScript */
export function ensureTrustedCatalog(repoRoot: string, dataDir: string, packages: string = LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES): void {
  const current = join(dataDir, "trusted-catalog", "current.json");
  if (existsSync(current)) return;
  mkdirSync(dataDir, { recursive: true });
  chmodSync(dataDir, 0o700);
  console.log(`[dev-local-hub] publishing trusted catalog (${packages}) into ${dataDir}`);
  const status = spawnSync("bun", ["nx", "run", "os-hub:trusted-catalog-bootstrap", "--packages", packages, "--outputStyle=stream"], { cwd: repoRoot, env: { ...process.env, OS_HUB_DATA: dataDir }, stdio: "inherit", shell: false }).status;
  if (status !== 0 || !existsSync(current)) throw new Error(`dev local hub: os-hub:trusted-catalog-bootstrap exited ${status} without publishing ${current}`);
}

/** 🪪️ The pid of the live owner of `dataDir`, or `null` (no owner record, or its process is gone). */
function liveOwnerPid(dataDir: string): number | null {
  try {
    const record = JSON.parse(readFileSync(join(dataDir, DEV_LOCAL_HUB_OWNER_FILE), "utf8")) as { readonly pid?: unknown };
    if (!Number.isSafeInteger(record.pid)) return null;
    process.kill(record.pid as number, 0);
    return record.pid as number;
  } catch {
    return null;
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
function spawnDevLocalHubOwner(repoRoot: string, hubUrl: string, dataDir: string): void {
  mkdirSync(dataDir, { recursive: true });
  chmodSync(dataDir, 0o700);
  const log = openSync(join(dataDir, DEV_LOCAL_HUB_LOG_FILE), "a", 0o600);
  const child = spawn("bun", [join(repoRoot, DEV_LOCAL_HUB_OWNER_SCRIPT), "local-hub", hubUrl, dataDir], { cwd: repoRoot, env: { ...process.env, OS_HUB_DATA: dataDir }, stdio: ["ignore", log, log], detached: true, shell: false });
  child.unref();
  console.log(`[dev-local-hub] started the local hub owner (pid ${child.pid}) for ${hubUrl}; log ${join(dataDir, DEV_LOCAL_HUB_LOG_FILE)}`);
}

/** ⏳️ Waits for the hub at `hubUrl`, bounded by how long nothing advances (owner log growth or readiness) rather than a
 * total wall budget: a clean data root first publishes its catalog, which takes as long as the component builds take. */
async function awaitDevLocalHub(hubUrl: string, dataDir: string): Promise<boolean> {
  let observation = "";
  let observedAt = Date.now();
  for (;;) {
    if (await hubReady(hubUrl)) return true;
    const owner = liveOwnerPid(dataDir);
    const next = `${owner ?? "none"}:${ownerLogBytes(dataDir)}`;
    const now = Date.now();
    if (next !== observation) {
      observation = next;
      observedAt = now;
    } else if (now - observedAt >= TRUSTED_CATALOG_READINESS_STALL_BOUND_MS || owner === null) return false;
    await new Promise<void>((resolveDelay) => setTimeout(resolveDelay, 1_000));
  }
}

/** 🌐️ Brings up (or joins) the loopback development hub and proves a session for this serve's profile through the
 * owner's broker. `null` only when local-only was asked for or no hub came up; a hub someone else operates (no broker
 * for this data root) is joined without a session, and the shell's own sign-in stays available. */
export async function ensureDevLocalHub(repoRoot: string, options: { readonly hubUrl?: string; readonly dataDir?: string; readonly profileId?: string } = {}): Promise<DevLocalHubSession | null> {
  if (process.env.S_LOCAL_ONLY === "1" || process.env.S_LOCAL_ONLY === "true") return null;
  const hubUrl = (options.hubUrl ?? process.env.S_HUB_URL ?? DEV_LOCAL_HUB_DEFAULT_URL).replace(/\/+$/u, "");
  const dataDir = options.dataDir ?? devLocalHubDataDir(repoRoot);
  const profileId = options.profileId ?? process.env[DEV_LOCAL_HUB_PROFILE_ENV] ?? DEV_LOCAL_HUB_DEFAULT_PROFILE;
  process.env.S_HUB_URL = hubUrl;
  if (!(await hubReady(hubUrl)) && liveOwnerPid(dataDir) === null && !isDevPortInUse("127.0.0.1", parseHubPort(hubUrl))) spawnDevLocalHubOwner(repoRoot, hubUrl, dataDir);
  if (!(await awaitDevLocalHub(hubUrl, dataDir))) {
    console.warn(`[dev-local-hub] no hub became ready at ${hubUrl} (see ${join(dataDir, DEV_LOCAL_HUB_LOG_FILE)}) — continuing local-first`);
    return null;
  }
  const session = await requestLocalBrokerSession(dataDir, hubUrl, profileId).catch(() => null);
  if (session === null) {
    console.log(`[dev-local-hub] joined ${hubUrl}; it has no session broker for ${dataDir}, so sign in through the shell`);
    return { hubUrl, dataDir, profileId, userId: "" };
  }
  console.log(`[dev-local-hub] ready at ${hubUrl} as ${session.userId} (${profileId})`);
  return { hubUrl, dataDir, profileId, userId: session.userId };
}

/** 🗄️ `local-hub [hubUrl] [dataDir]`: the owner process — publishes the catalog when the root has none, stages the hub
 * binary, boots the hub with the development profiles, runs the session broker, and holds until signalled. A second
 * owner for a data root with a live owner exits at once. */
export class DevLocalHubScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const [hubArg, dataArg] = segments;
    const hubUrl = (hubArg ?? process.env.S_HUB_URL ?? DEV_LOCAL_HUB_DEFAULT_URL).replace(/\/+$/u, "");
    const dataDir = dataArg ? resolve(dataArg) : devLocalHubDataDir(this.repoRoot);
    const existing = liveOwnerPid(dataDir);
    if (existing !== null && existing !== process.pid) {
      console.log(`[dev-local-hub] ${dataDir} already has a live owner (pid ${existing})`);
      return;
    }
    mkdirSync(dataDir, { recursive: true });
    chmodSync(dataDir, 0o700);
    const ownerPath = join(dataDir, DEV_LOCAL_HUB_OWNER_FILE);
    writeFileSync(ownerPath, `${JSON.stringify({ pid: process.pid, hubUrl })}\n`, { mode: 0o600 });
    try {
      ensureTrustedCatalog(this.repoRoot, dataDir);
      const hubPkg = join(this.repoRoot, "🌎️hub", "📦️packages", "🦀️rust");
      const binaryPath = hubDevBinaryPath(hubPkg);
      process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
      const adminToken = process.env.OS_HUB_ADMIN_TOKEN ?? "dev-local-hub-admin";
      const run = await startLocalHub(this.repoRoot, hubPkg, LOCAL_HUB_DEVELOPMENT_PROFILES, { port: parseHubPort(hubUrl), dataDir, binaryPath, adminToken, capture: false });
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
      if (liveOwnerPid(dataDir) === process.pid) rmSync(ownerPath, { force: true });
    }
  }
}
