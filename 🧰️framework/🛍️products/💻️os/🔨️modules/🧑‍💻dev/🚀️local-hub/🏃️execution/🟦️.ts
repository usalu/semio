/** 🚀️ Ensures a loopback `os-hub` with local-bootstrap credentials for zero-touch `dev s`. */

import { spawnSync } from "node:child_process";
import { chmodSync, existsSync, mkdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { isDevPortInUse } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { issueLocalCredential } from "../../../../../../../🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import type { LocalProfile } from "../../../../../../../🌎️hub/🚀️local-bootstrap/🛂authentication/🟦️.ts";
import {
  finishLocalHub,
  hubDevBinaryPath,
  startLocalHub,
  waitForReadiness,
  type LocalHubRun,
} from "../../../../../../../🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts";

export const DEV_LOCAL_HUB_DEFAULT_URL = "http://127.0.0.1:8787";
export const DEV_LOCAL_HUB_SESSION_PATH = "/_semio/dev/local-session";
export const DEV_LOCAL_HUB_TOKEN_ENV = "SEMIO_DEV_LOCAL_HUB_TOKEN";
export const DEV_LOCAL_HUB_USER_ENV = "SEMIO_DEV_LOCAL_HUB_USER_ID";

const DEVELOPER_PROFILE: LocalProfile = {
  profileId: "developer",
  subject: "local-developer-01",
  displayName: "Local Developer",
  allowedClientClasses: ["native", "mcp", "react-relay"],
};

export type DevLocalHubSession = {
  readonly hubUrl: string;
  readonly token: string;
  readonly userId: string;
  readonly owned: boolean;
  readonly stop: () => Promise<void>;
};

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

/** 🔏️ The packages a zero-touch dev hub publishes: the hub's own bootstrap closure plus the collaboration editors. */
export const DEV_LOCAL_HUB_CATALOG_PACKAGES: readonly string[] = ["stdio", "gis", "note", "writer", "draw", "puzzle"];

/** 🔏️ Publishes the canonical trusted catalog into `dataDir` through the hub's own product verb
 * (`os-hub:trusted-catalog-bootstrap`) when that root holds none — never a copy of another run's data root.
 * @see ../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts TrustedCatalogBootstrapScript */
export function ensureTrustedCatalog(repoRoot: string, dataDir: string, packages: readonly string[] = DEV_LOCAL_HUB_CATALOG_PACKAGES): void {
  const current = join(dataDir, "trusted-catalog", "current.json");
  if (existsSync(current)) return;
  mkdirSync(dataDir, { recursive: true });
  chmodSync(dataDir, 0o700);
  console.log(`[dev-local-hub] publishing trusted catalog (${packages.join(",")}) into ${dataDir}`);
  const status = spawnSync("bun", ["nx", "run", "os-hub:trusted-catalog-bootstrap", "--packages", packages.join(","), "--outputStyle=stream"], { cwd: repoRoot, env: { ...process.env, OS_HUB_DATA: dataDir }, stdio: "inherit", shell: false }).status;
  if (status !== 0 || !existsSync(current)) throw new Error(`dev local hub: os-hub:trusted-catalog-bootstrap exited ${status} without publishing ${current}`);
}

async function resolveSessionAuthority(hubUrl: string, token: string): Promise<string> {
  const response = await fetch(`${hubUrl}/auth/sessions/me`, {
    method: "GET",
    headers: { authorization: `Bearer ${token}` },
    signal: AbortSignal.timeout(10_000),
  });
  if (!response.ok) throw new Error(`dev local hub: session me failed with ${response.status}`);
  const body = (await response.json()) as { readonly user_id?: string; readonly userId?: string };
  const userId = body.userId ?? body.user_id;
  if (typeof userId !== "string" || userId.length === 0) throw new Error("dev local hub: session me omitted user id");
  return userId;
}

/** 🌐️ Starts (or attaches to) the loopback hub and returns a one-shot local-bootstrap session when this process owns the hub. */
export async function ensureDevLocalHub(repoRoot: string, options: { readonly hubUrl?: string; readonly dataDir?: string } = {}): Promise<DevLocalHubSession | null> {
  if (process.env.S_LOCAL_ONLY === "1" || process.env.S_LOCAL_ONLY === "true") return null;
  const hubUrl = options.hubUrl ?? process.env.S_HUB_URL ?? DEV_LOCAL_HUB_DEFAULT_URL;
  const port = parseHubPort(hubUrl);
  process.env.S_HUB_URL = hubUrl;
  if (await hubReady(hubUrl)) {
    console.log(`[dev-local-hub] reusing healthy hub at ${hubUrl}`);
    const existingToken = process.env[DEV_LOCAL_HUB_TOKEN_ENV] ?? "";
    if (/^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(existingToken)) {
      try {
        const userId = await resolveSessionAuthority(hubUrl, existingToken);
        process.env[DEV_LOCAL_HUB_USER_ENV] = userId;
        return { hubUrl, token: existingToken, userId, owned: false, stop: async () => undefined };
      } catch {
        /* token rejected — continue without a minted session */
      }
    }
    return { hubUrl, token: "", userId: "", owned: false, stop: async () => undefined };
  }

  if (isDevPortInUse("127.0.0.1", port)) {
    console.log(`[dev-local-hub] port ${port} occupied without hub readiness — continuing local-first`);
    return null;
  }
  const hubPkg = join(repoRoot, '🌎️hub', "📦️packages", "🦀️rust");
  const dataDir = resolve(options.dataDir ?? process.env.OS_HUB_DATA ?? join(repoRoot, ".🧬semio", "🌐hub", "hub-dev"));
  ensureTrustedCatalog(repoRoot, dataDir);
  const binaryPath = hubDevBinaryPath(hubPkg);
  let run: LocalHubRun | undefined;
  const stop = async (): Promise<void> => {
    if (!run) return;
    const owned = run;
    run = undefined;
    await finishLocalHub(owned);
  };
  process.once("SIGINT", () => void stop());
  process.once("SIGTERM", () => void stop());
  try {
    process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
    process.env.OS_HUB_ADMIN_TOKEN = process.env.OS_HUB_ADMIN_TOKEN ?? "dev-local-hub-admin";
    run = await startLocalHub(repoRoot, hubPkg, [DEVELOPER_PROFILE], { port, dataDir, binaryPath, adminToken: process.env.OS_HUB_ADMIN_TOKEN, capture: true });
    // Match hub trusted-catalog startup stall (300s): pre-bind catalog verify can be silent for >30s on a warm hub-dev root.
    await waitForReadiness(run, false, 300_000);
    const envelope = await issueLocalCredential(run, "developer", "react-relay");
    const token = String(envelope.capability ?? "");
    if (!/^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(token)) throw new Error("dev local hub: local-bootstrap capability missing");
    const userId = await resolveSessionAuthority(hubUrl, token);
    process.env[DEV_LOCAL_HUB_TOKEN_ENV] = token;
    process.env[DEV_LOCAL_HUB_USER_ENV] = userId;
    console.log(`[dev-local-hub] ready at ${hubUrl} as ${userId}`);
    return { hubUrl, token, userId, owned: true, stop };
  } catch (error) {
    await stop();
    console.warn(`[dev-local-hub] hub unavailable (${error instanceof Error ? error.message : String(error)}) — continuing local-first`);
    return null;
  }
}
