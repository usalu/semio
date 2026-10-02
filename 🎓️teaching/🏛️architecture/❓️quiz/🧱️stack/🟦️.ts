/** 🧱️ The local stack of the architecture quiz: the proctor (backend) and the site (frontend) started, awaited and stopped
 * as one.
 *
 * `dev` is the whole stack in one terminal: it reuses a proctor that already answers on `PROCTOR_PORT` (and never stops
 * what it did not start), else builds and launches one with the dev defaults of `@teaching/proctor:dev` — after setting
 * the launcher's own development data aside when it is of a storage format this proctor does not read —, waits until it
 * answers ready, then starts the site's dev server, which proxies the gateway routes to that proctor. One Ctrl+C, a
 * termination or a closed terminal stops both with their process trees; either one exiting stops the other. The parts are
 * exported for the end-to-end gate, which boots throw-away stacks from them — the static origin included, which answers
 * like a static CDN (files as they are, `index.html` for a directory, `404.html` with status 404 for anything else).
 *
 * Every process is one argv without a shell and every stop goes through the repo library's process-tree termination, so
 * the stack behaves the same on Windows, macOS, Linux and in the devcontainer.
 * @see ../../../🛂️proctor/🏗️bootstrap/🟦️.ts — building, launching and probing the proctor
 * @see ../🏗️builder/🌐️vite/🟦️.ts — the dev server and its proxy
 * @see ../🎭️e2e/🟦️.ts — the end-to-end gate
 * @see ../README.md — the launch rows */
import { spawn } from "node:child_process";
import { closeSync, openSync, readFileSync, statSync } from "node:fs";
import { createServer } from "node:http";
import type { AddressInfo } from "node:net";
import { extname, join, sep } from "node:path";
import { terminateOwnedChildTree } from "../../../../🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts";
import { LAUNCHER_SIGNALS, PROCTOR_STOP_GRACE_MS, buildProctor, developmentDataSettled, launchProctor, proctorDevelopmentEnvironment, proctorOrigin, proctorReady, type ProctorProcess } from "../../../🛂️proctor/🏗️bootstrap/🟦️.ts";

//#region ⏳️Readiness
/** 🗣️ Where a progress line of the stack goes. */
export type Say = (line: string) => void;

const announce: Say = (line) => console.log(`[stack] ${line}`);

/** ⏳️ How long a launched server may take to answer once its build is done. */
export const STACK_READY_TIMEOUT_MS = 180_000;

/** 🐢️ How long one answer of the dev server may take: its first page is transformed on demand, slowly on a busy machine. */
export const SITE_FIRST_ANSWER_MS = 15_000;

/** 💤️ Resolves after `ms`, or at once when `signal` aborts. */
function pause(ms: number, signal?: AbortSignal): Promise<void> {
  return new Promise((accept) => {
    if (signal?.aborted) return accept();
    const done = (): void => {
      clearTimeout(timer);
      signal?.removeEventListener("abort", done);
      accept();
    };
    const timer = setTimeout(done, ms);
    signal?.addEventListener("abort", done, { once: true });
  });
}

/** 🩺️ Whether `url` answers a successful HTTP status within `timeoutMs`. */
export async function httpAnswers(url: string, timeoutMs = 2_000): Promise<boolean> {
  try {
    const answer = await fetch(url, { signal: AbortSignal.timeout(timeoutMs) });
    await answer.arrayBuffer();
    return answer.ok;
  } catch {
    return false;
  }
}

/** ⏳️ Waits until `probe` answers and returns the milliseconds it took, saying a progress line every `progressMs`. It
 * fails with a message naming `label` when the server's process (`exited`) ends first, when `timeoutMs` elapses, or when
 * `signal` cancels the wait. */
export async function awaitReady(wait: {
  readonly label: string;
  readonly probe: () => Promise<boolean>;
  readonly exited?: Promise<number>;
  readonly signal?: AbortSignal;
  readonly timeoutMs?: number;
  readonly pollMs?: number;
  readonly progressMs?: number;
  readonly say?: Say;
  readonly now?: () => number;
}): Promise<number> {
  const { label, probe, signal, timeoutMs = STACK_READY_TIMEOUT_MS, pollMs = 250, progressMs = 5_000, say = announce, now = Date.now } = wait;
  let ended: number | undefined;
  void wait.exited?.then((status) => (ended = status));
  const started = now();
  for (let reported = started; ; await pause(pollMs, signal)) {
    if (signal?.aborted) throw new Error(`waiting for ${label} was cancelled`);
    if (ended !== undefined) throw new Error(`${label} exited with status ${ended} before it became ready`);
    if (await probe()) return now() - started;
    const at = now();
    if (at - started >= timeoutMs) throw new Error(`${label} did not become ready within ${Math.round(timeoutMs / 1_000)} s`);
    if (at - reported >= progressMs) {
      reported = at;
      say(`waiting for ${label} (${Math.round((at - started) / 1_000)} s)`);
    }
  }
}
//#endregion ⏳️Readiness

//#region 🧵️Processes
/** 🧵️ A command this process launched: `exited` settles with its exit status, `stop` ends it with every process it
 * started and settles the same way. */
export interface OwnedProcess {
  readonly pid: number | undefined;
  readonly exited: Promise<number>;
  stop(): Promise<number>;
}

/** 🚀️ Launches one argv in `cwd` with `env`; its output shares the terminal, or is appended to the file `log`. */
export function launchOwned(command: string, args: readonly string[], cwd: string, env: NodeJS.ProcessEnv = process.env, log?: string): OwnedProcess {
  const sink = log === undefined ? undefined : openSync(log, "a");
  const child = spawn(command, [...args], { cwd, env, stdio: sink === undefined ? "inherit" : ["ignore", sink, sink], windowsHide: true });
  const exited = new Promise<number>((accept) => {
    child.once("error", (error) => {
      console.error(`[stack] ${command}: ${error.message}`);
      accept(1);
    });
    child.once("close", (code, signal) => accept(code ?? (signal ? 128 : 1)));
  }).finally(() => {
    if (sink !== undefined) closeSync(sink);
  });
  return {
    pid: child.pid,
    exited,
    stop: () => {
      terminateOwnedChildTree(child);
      return exited;
    },
  };
}
//#endregion 🧵️Processes

//#region 🌍️StaticOrigin
/** 🗂️ The media type of every kind of file a site artifact holds. */
export const SITE_MEDIA_TYPES: Readonly<Record<string, string>> = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".mjs": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json",
  ".map": "application/json",
  ".webmanifest": "application/manifest+json",
  ".svg": "image/svg+xml",
  ".ico": "image/x-icon",
  ".png": "image/png",
  ".woff2": "font/woff2",
  ".txt": "text/plain; charset=utf-8",
};

/** 📄️ The file of `directory` a static CDN serves for a request path: the file itself, or `index.html` of the directory a
 * trailing slash names; `undefined` for a path that names no such file or leaves the directory. */
export function staticSiteFile(directory: string, pathname: string): string | undefined {
  let decoded: string;
  try {
    decoded = decodeURIComponent(pathname);
  } catch {
    return undefined;
  }
  if (decoded.includes("\0")) return undefined;
  const target = join(directory, decoded.endsWith("/") ? `${decoded}index.html` : decoded);
  if (!target.startsWith(`${join(directory, sep)}`)) return undefined;
  try {
    return statSync(target).isFile() ? target : undefined;
  } catch {
    return undefined;
  }
}

/** 🌍️ Serves `directory` on `127.0.0.1:port` (`0` picks a free port) as a static CDN does and nothing else — no proxy, no
 * fallback to the application: a request that names no file answers `404.html` with status 404. */
export function serveStaticSite(directory: string, port: number): Promise<{ readonly origin: string; close(): Promise<void> }> {
  const server = createServer((request, answer) => {
    const file = staticSiteFile(directory, new URL(request.url ?? "/", "http://site").pathname);
    const missing = file === undefined ? staticSiteFile(directory, "/404.html") : undefined;
    const served = file ?? missing;
    answer.statusCode = file === undefined ? 404 : 200;
    answer.setHeader("cache-control", "no-cache");
    if (served === undefined) return void answer.end();
    answer.setHeader("content-type", SITE_MEDIA_TYPES[extname(served).toLowerCase()] ?? "application/octet-stream");
    answer.end(request.method === "HEAD" ? undefined : readFileSync(served));
  });
  return new Promise((accept, reject) => {
    server.once("error", reject);
    server.listen(port, "127.0.0.1", () =>
      accept({
        origin: `http://127.0.0.1:${(server.address() as AddressInfo).port}`,
        close: () =>
          new Promise<void>((closed) => {
            server.close(() => closed());
            server.closeAllConnections();
          }),
      }),
    );
  });
}
//#endregion 🌍️StaticOrigin

//#region 🛠️Dev
/** 🛠️ `dev` — the proctor, then the site, in this terminal until one of them exits or the terminal interrupts, terminates
 * or closes. `script` is the site's `📜️script.ts`, whose `dev-site` is the dev server alone; `site` names the variable
 * the site's port is read from and the port it has otherwise; `segments` go to the dev server. */
export async function runDevStack(repoRoot: string, bundleRoot: string, script: string, site: { readonly portEnv: string; readonly defaultPort: string }, segments: readonly string[] = []): Promise<void> {
  const env = proctorDevelopmentEnvironment(repoRoot);
  const proctorUrl = proctorOrigin(env);
  const port = process.env[site.portEnv] ?? site.defaultPort;
  const siteUrl = `http://127.0.0.1:${port}`;
  const interruption = new AbortController();
  let interrupted: NodeJS.Signals | undefined;
  let proctor: ProctorProcess | undefined;
  let server: OwnedProcess | undefined;
  const interrupt = (signal: NodeJS.Signals): void => {
    interrupted ??= signal;
    proctor?.interrupt(signal);
    interruption.abort();
  };
  for (const signal of LAUNCHER_SIGNALS) process.on(signal, interrupt);
  try {
    if (await proctorReady(proctorUrl)) announce(`reusing the proctor that already answers at ${proctorUrl}; this command will not stop it`);
    else {
      if (!(await developmentDataSettled(repoRoot, env, announce))) return;
      announce(`building the proctor for ${proctorUrl} (a first build takes minutes)`);
      proctor = launchProctor(await buildProctor(repoRoot, env, interruption.signal), repoRoot, ["serve"], env);
      const took = await awaitReady({ label: `the proctor at ${proctorUrl}`, probe: () => proctorReady(proctorUrl), exited: proctor.exited, signal: interruption.signal });
      announce(`proctor ready at ${proctorUrl} after ${took} ms`);
    }
    if (await httpAnswers(siteUrl)) announce(`reusing the site that already answers at ${siteUrl}; this command will not stop it`);
    else {
      server = launchOwned(process.execPath, [script, "dev-site", ...segments], bundleRoot, { ...process.env, [site.portEnv]: port, PROCTOR_PORT: env.PROCTOR_PORT });
      await awaitReady({ label: `the site at ${siteUrl}`, probe: () => httpAnswers(siteUrl, SITE_FIRST_ANSWER_MS), exited: server.exited, signal: interruption.signal });
    }
    announce(`site ready at ${siteUrl}, proctor at ${proctorUrl}`);
    if (proctor === undefined && server === undefined) return;
    announce(`Ctrl+C stops ${[proctor && "the proctor", server && "the site"].filter(Boolean).join(" and ")}`);
    const ended = await Promise.race([
      new Promise<undefined>((accept) => interruption.signal.addEventListener("abort", () => accept(undefined), { once: true })),
      ...(proctor === undefined ? [] : [proctor.exited.then((status) => `the proctor exited with status ${status}`)]),
      ...(server === undefined ? [] : [server.exited.then((status) => `the site exited with status ${status}`)]),
    ]);
    if (ended !== undefined && interrupted === undefined) throw new Error(`${ended}; the stack stops`);
  } catch (error) {
    if (interrupted === undefined) throw error;
  } finally {
    if (interrupted !== undefined) announce(`${interrupted} received; stopping the stack`);
    await Promise.all([server?.stop(), proctor?.stop(interrupted === undefined ? undefined : PROCTOR_STOP_GRACE_MS)]);
    for (const signal of LAUNCHER_SIGNALS) process.off(signal, interrupt);
    if (interrupted !== undefined) process.exitCode = interrupted === "SIGINT" ? 130 : 143;
  }
}
//#endregion 🛠️Dev
