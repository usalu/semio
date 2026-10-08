/** 🧱️ The local stack of the architecture quiz: the proctor (backend) and the site (frontend) started, awaited and stopped
 * as one.
 *
 * `dev` is the whole stack in one terminal: it reuses a proctor that already answers on `PROCTOR_PORT` (and never stops
 * what it did not start — it says so when that proctor serves another quiz contract than the site), else builds and
 * launches one with the dev defaults of `@teaching/proctor:dev` — after setting the launcher's own development data aside
 * when it is of a storage format this proctor does not read —, waits until it answers ready, then starts the site's dev
 * server, which proxies the gateway routes to that proctor. The proctor it launched is supervised: built and launched
 * anew when its Rust sources or its catalog change, launched again when it ends by itself; the site tolerates every such
 * absence. One Ctrl+C, a termination or a closed terminal stops both with their process trees; the site exiting stops
 * the proctor. The parts are exported for the end-to-end gate, which boots throw-away stacks from them — the static
 * origin included, which answers like a static CDN (files as they are, `index.html` for a directory, `404.html` with
 * status 404 for anything else).
 *
 * Every process is one argv without a shell and every stop goes through the repo library's process-tree termination, so
 * the stack behaves the same on Windows, macOS, Linux and in the devcontainer.
 * @see ../../../🛂️proctor/🏗️bootstrap/🟦️.ts — building, launching and probing the proctor
 * @see ../🏗️builder/🌐️vite/🟦️.ts — the dev server and its proxy
 * @see ../🎭️e2e/🟦️.ts — the end-to-end gate
 * @see ../README.md — the dashboard commands */
import { spawn } from "node:child_process";
import { closeSync, openSync, readFileSync, statSync, watch, type FSWatcher } from "node:fs";
import { createServer } from "node:http";
import type { AddressInfo } from "node:net";
import { dirname, extname, join, resolve, sep } from "node:path";
import { decodeServerInstanceDefinition } from "@semio-tech/framework-server";
import { WIRE_VERSION } from "@semio-tech/quiz";
import { terminateOwnedChildTree } from "../../../../🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts";
import { disagreement, type ProctorContract } from "../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts";
import { LAUNCHER_SIGNALS, PROCTOR_STOP_GRACE_MS, buildProctor, developmentDataSettled, launchProctor, proctorDevelopmentEnvironment, proctorOrigin, proctorReady, proctorSourceChanged, proctorSourceDirectories, type ProctorProcess } from "../../../🛂️proctor/🏗️bootstrap/🟦️.ts";

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

//#region 🧑‍🔧️Supervision
/** ⏳️ How long a change of the sources or the catalog settles before the proctor is built or launched anew. */
export const PROCTOR_CHANGE_SETTLE_MS = 400;

/** ⏳️ The pause before a proctor that ended by itself is launched again after `crashes` such ends in a row: one second,
 * doubling, at most half a minute. */
export function relaunchDelay(crashes: number): number {
  return Math.min(30_000, 1_000 * 2 ** Math.max(0, crashes - 1));
}

/** 📚️ The files a proctor reads its catalog from: the catalog `catalog` and every quiz it lists; the catalog alone when it
 * cannot be read. */
export function catalogFiles(catalog: string): string[] {
  try {
    const listed = (JSON.parse(readFileSync(catalog, "utf8")) as { readonly quizzes?: unknown }).quizzes;
    return [catalog, ...(Array.isArray(listed) ? listed.filter((quiz): quiz is string => typeof quiz === "string").map((quiz) => resolve(dirname(catalog), quiz)) : [])];
  } catch {
    return [catalog];
  }
}

/** 🤝️ Whether the proctor that answers at `origin` serves the quiz contract the site's client speaks — `undefined` when
 * it does or cannot be asked, else how its contract stands to the site's, as the client in the browser concludes it. */
async function proctorContract(origin: string): Promise<ProctorContract | undefined> {
  try {
    const answer = await fetch(`${origin}/instance`, { signal: AbortSignal.timeout(2_000) });
    return answer.ok ? disagreement(decodeServerInstanceDefinition(await answer.json())) : undefined;
  } catch {
    return undefined;
  }
}

/** 🧑‍🔧️ The dev proctor of one command: built, launched over settled development data and awaited ready; then, unless
 * `watch` is off, built anew and launched in place of the running one whenever a Rust source it is built from changes
 * (a failed build leaves the running one serving) and launched anew whenever its catalog changes; and launched again,
 * ever more patiently, whenever it ends by itself. Each step is said. While it is away the site keeps everything on the
 * device, so every swap is a short absence of the proctor and nothing more. */
class SupervisedProctor {
  private running: ProctorProcess | undefined;
  private executable = "";
  private crashes = 0;
  private stopped = false;
  private work: Promise<void> = Promise.resolve();
  private settling: ReturnType<typeof setTimeout> | undefined;
  private pending: "build" | "launch" | undefined;
  private readonly watchers: FSWatcher[] = [];

  constructor(
    private readonly repoRoot: string,
    private readonly env: NodeJS.ProcessEnv,
    private readonly signal: AbortSignal,
  ) {}

  private get origin(): string {
    return proctorOrigin(this.env);
  }

  /** ▶️ Builds and launches the proctor and, with `watch`, follows its sources and catalog; `false` when the development
   * data may not be used (said, with the exit status set). */
  async start(watch: boolean): Promise<boolean> {
    if (!(await developmentDataSettled(this.repoRoot, this.env, announce))) return false;
    announce(`building the proctor for ${this.origin} (a first build takes minutes)`);
    this.executable = await buildProctor(this.repoRoot, this.env, this.signal);
    await this.launch();
    if (watch) await this.watch();
    return true;
  }

  /** 🔀️ Forwards an interrupt or a termination of the terminal to the running proctor. */
  interrupt(signal: NodeJS.Signals): void {
    this.running?.interrupt(signal);
  }

  /** ⏹️ Stops following and ends the running proctor, within `graceMs` when given. */
  async stop(graceMs?: number): Promise<void> {
    this.stopped = true;
    clearTimeout(this.settling);
    for (const watcher of this.watchers.splice(0)) watcher.close();
    await this.work.catch(() => undefined);
    const running = this.running;
    this.running = undefined;
    await running?.stop(graceMs);
  }

  private async launch(): Promise<void> {
    const launched = launchProctor(this.executable, this.repoRoot, ["serve"], this.env);
    this.running = launched;
    void launched.exited.then((status) => {
      if (this.running !== launched || this.stopped) return;
      this.running = undefined;
      this.crashes += 1;
      const delay = relaunchDelay(this.crashes);
      announce(`the proctor ended by itself with status ${status}; launching it again in ${Math.round(delay / 1_000)} s — the site keeps everything on the device meanwhile`);
      setTimeout(() => this.schedule("launch"), delay);
    });
    const took = await awaitReady({ label: `the proctor at ${this.origin}`, probe: () => proctorReady(this.origin), exited: launched.exited, signal: this.signal });
    this.crashes = 0;
    announce(`proctor ready at ${this.origin} after ${took} ms`);
  }

  private async watch(): Promise<void> {
    let directories: string[];
    try {
      directories = await proctorSourceDirectories(this.repoRoot, this.env);
    } catch (error) {
      announce(`not following the proctor's sources: ${error instanceof Error ? error.message : String(error)}`);
      directories = [];
    }
    const catalog = resolve(this.env.PROCTOR_CATALOG!);
    const follow = (directory: string, changed: (file: string) => boolean, step: "build" | "launch"): void => {
      try {
        this.watchers.push(
          watch(directory, { recursive: true }, (_, file) => {
            if (file !== null && changed(String(file))) this.changed(step);
          }),
        );
      } catch (error) {
        announce(`not following ${directory}: ${error instanceof Error ? error.message : String(error)}`);
      }
    };
    for (const directory of directories) follow(directory, proctorSourceChanged, "build");
    follow(dirname(catalog), (file) => catalogFiles(catalog).includes(resolve(dirname(catalog), file)), "launch");
    announce(`following ${directories.length} source folders and the catalog: the proctor is built and launched anew when they change (TEACHING_ARCHITECTURE_QUIZ_WATCH=off stops that)`);
  }

  private changed(step: "build" | "launch"): void {
    this.pending = step === "build" || this.pending === "build" ? "build" : "launch";
    clearTimeout(this.settling);
    this.settling = setTimeout(() => {
      const next = this.pending;
      this.pending = undefined;
      if (next !== undefined) this.schedule(next);
    }, PROCTOR_CHANGE_SETTLE_MS);
  }

  private schedule(step: "build" | "launch"): void {
    if (this.stopped) return;
    this.work = this.work.then(() => this.renew(step)).catch((error: unknown) => announce(`the proctor was not renewed: ${error instanceof Error ? error.message : String(error)}`));
  }

  private async renew(step: "build" | "launch"): Promise<void> {
    if (this.stopped) return;
    if (step === "build") {
      announce("a source of the proctor changed: building it anew while the running one serves");
      this.executable = await buildProctor(this.repoRoot, this.env, this.signal);
    } else if (this.running !== undefined) announce("the catalog changed: launching the proctor anew");
    const running = this.running;
    this.running = undefined;
    await running?.stop();
    if (this.stopped) return;
    if (!(await developmentDataSettled(this.repoRoot, this.env, announce))) return;
    await this.launch();
  }
}
//#endregion 🧑‍🔧️Supervision

//#region 🛠️Dev
/** 🛠️ `dev` — the proctor, then the site, in this terminal until the site exits or the terminal interrupts, terminates
 * or closes. A proctor that already answers is reused — and never stopped — and said to be of another contract when it
 * is; else this command runs its own {@link SupervisedProctor}, following its sources and catalog unless
 * `TEACHING_ARCHITECTURE_QUIZ_WATCH` is `off`. `script` is the site's `📜️script.ts`, whose `dev-site` is the dev server
 * alone; `site` names the variable the site's port is read from and the port it has otherwise; `segments` go to the dev
 * server. */
export async function runDevStack(repoRoot: string, bundleRoot: string, script: string, site: { readonly portEnv: string; readonly defaultPort: string }, segments: readonly string[] = []): Promise<void> {
  const env = proctorDevelopmentEnvironment(repoRoot);
  const proctorUrl = proctorOrigin(env);
  const port = process.env[site.portEnv] ?? site.defaultPort;
  const siteUrl = `http://127.0.0.1:${port}`;
  const interruption = new AbortController();
  let interrupted: NodeJS.Signals | undefined;
  let proctor: SupervisedProctor | undefined;
  let server: OwnedProcess | undefined;
  const interrupt = (signal: NodeJS.Signals): void => {
    interrupted ??= signal;
    proctor?.interrupt(signal);
    interruption.abort();
  };
  for (const signal of LAUNCHER_SIGNALS) process.on(signal, interrupt);
  try {
    if (await proctorReady(proctorUrl)) {
      announce(`reusing the proctor that already answers at ${proctorUrl}; this command will not stop it`);
      const contract = await proctorContract(proctorUrl);
      if (contract !== undefined)
        announce(`[WARN] that proctor serves ${contract === "foreign" ? "no quiz contract the site speaks" : `an ${contract} quiz contract than the site (wire version ${WIRE_VERSION})`}: the site sends it nothing and keeps everything on the device. Stop it and run this command again to build the current one.`);
    } else {
      proctor = new SupervisedProctor(repoRoot, env, interruption.signal);
      if (!(await proctor.start(process.env.TEACHING_ARCHITECTURE_QUIZ_WATCH !== "off"))) return;
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
