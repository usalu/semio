// #region Header
/**
 * 🌎️ `os-hub-ts` — a Bun integration-test harness for the real `os-hub` binary (ticket
 * 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS, lane 3-E). Boots the compiled
 * binary on a scanned free port against a temp `OS_HUB_DATA`, polls a real endpoint for
 * readiness, and hands back a handle `🧪️index.test.ts` drives with two independent
 * `@semio-tech/framework-os` `DirectoryClient`s plus raw document-WS wire frames. Never `cargo
 * run`s (a wrapper process would complicate teardown) — spawns the prebuilt debug binary
 * directly, so `stop()` is a plain signal to one process, no process tree to chase.
 */
// #endregion Header

import { type ChildProcessByStdio, spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { createServer } from "node:net";
import { join } from "node:path";
import type { Readable } from "node:stream";

export { getWorkspaceRoot } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

//#region 🔖️Port
/** 🔌️ Binds an ephemeral port (`:0`), reads back what the OS assigned, then releases it — the
 * standard "scan, don't hardcode" free-port trick (contract-freeze §C0's own e2e-port section
 * describes the same idea for a sibling lane's fixed pool; this harness has no fixed pool to
 * share, so it scans instead). A small bind-then-release race is possible (another process could
 * grab the port before the hub does); acceptable for a local dev/CI integration test. */
export async function findFreePort(host = "127.0.0.1"): Promise<number> {
  return new Promise((resolvePort, rejectPort) => {
    const server = createServer();
    server.unref();
    server.once("error", rejectPort);
    server.listen(0, host, () => {
      const address = server.address();
      if (address === null || typeof address === "string") {
        server.close(() => rejectPort(new Error("findFreePort: no ephemeral port assigned")));
        return;
      }
      const { port } = address;
      server.close(() => resolvePort(port));
    });
  });
}
//#endregion 🔖️Port

//#region 🔖️Readiness
/** ⏳️ Polls `url` with `fetch` until it answers (any HTTP status counts — the point is proving
 * the listener is actually accepting connections and axum is routing, not that this particular
 * route accepts us), or throws once `timeoutMs` elapses. */
export async function waitForHttpReady(url: string, headers: Record<string, string> = {}, timeoutMs = 60_000): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  let lastError: unknown = new Error("waitForHttpReady: never attempted");
  while (Date.now() < deadline) {
    try {
      await fetch(url, { headers });
      return;
    } catch (error) {
      lastError = error;
      await new Promise((resolveDelay) => setTimeout(resolveDelay, 150));
    }
  }
  throw new Error(`waitForHttpReady: ${url} unreachable after ${timeoutMs}ms: ${String(lastError)}`);
}
//#endregion 🔖️Readiness

//#region 🔖️Hub
/** 📁️ `<repoRoot>/target/debug/os-hub[.exe]` — the plain `cargo build` (default features)
 * output; never the `--release` path, matching `📜️script.ts`'s own build step. */
export function resolveHubBinaryPath(repoRoot: string): string {
  const name = process.platform === "win32" ? "os-hub.exe" : "os-hub";
  return join(repoRoot, "target", "debug", name);
}

export type HubOptions = {
  readonly repoRoot: string;
  readonly dataDir: string;
  readonly adminToken: string;
  readonly port?: number;
  readonly readyTimeoutMs?: number;
};

export type HubHandle = {
  readonly port: number;
  readonly baseUrl: string;
  readonly wsBaseUrl: string;
  readonly stdout: () => string;
  readonly stderr: () => string;
  stop(): Promise<void>;
};

/** 🚀️ Spawns the prebuilt `os-hub` binary directly (no `cargo run` wrapper) on a scanned free
 * port with a temp `OS_HUB_DATA`/`OS_HUB_ADMIN_TOKEN`, waits for a real HTTP response before
 * returning, and hands back a handle whose `stop()` reliably tears the process down (`SIGTERM`,
 * escalating to `SIGKILL` after a grace period) even if the caller never reads the rest of the
 * handle. */
export async function startHub(options: HubOptions): Promise<HubHandle> {
  const bin = resolveHubBinaryPath(options.repoRoot);
  if (!existsSync(bin)) {
    throw new Error(`startHub: ${bin} does not exist — build it first: cargo build --manifest-path 🌎️hub/📦️packages/🦀️rust/Cargo.toml`);
  }
  const port = options.port ?? (await findFreePort());
  const env: NodeJS.ProcessEnv = { ...process.env, OS_HUB_PORT: String(port), OS_HUB_DATA: options.dataDir, OS_HUB_ADMIN_TOKEN: options.adminToken };
  const child: ChildProcessByStdio<null, Readable, Readable> = spawn(bin, [], { env, stdio: ["ignore", "pipe", "pipe"] });
  let stdout = "";
  let stderr = "";
  child.stdout.on("data", (chunk: Buffer) => {
    stdout += chunk.toString("utf8");
  });
  child.stderr.on("data", (chunk: Buffer) => {
    stderr += chunk.toString("utf8");
  });
  let exited = false;
  child.once("exit", () => {
    exited = true;
  });
  const baseUrl = `http://127.0.0.1:${port}`;
  const wsBaseUrl = `ws://127.0.0.1:${port}`;

  const stop = async (): Promise<void> => {
    if (exited) return;
    await new Promise<void>((resolveStop) => {
      child.once("exit", () => resolveStop());
      child.kill("SIGTERM");
      setTimeout(() => {
        if (!exited) child.kill("SIGKILL");
      }, 5_000);
    });
  };

  try {
    await waitForHttpReady(`${baseUrl}/admin/api/overview`, { authorization: `Bearer ${options.adminToken}` }, options.readyTimeoutMs ?? 60_000);
  } catch (error) {
    await stop();
    throw new Error(`startHub: hub never became ready — stderr:\n${stderr}\n${String(error)}`);
  }
  if (exited) {
    throw new Error(`startHub: hub process exited before becoming ready — stderr:\n${stderr}`);
  }

  return { port, baseUrl, wsBaseUrl, stdout: () => stdout, stderr: () => stderr, stop };
}
//#endregion 🔖️Hub
