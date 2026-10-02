/** 🎭️ The end-to-end gate of the architecture quiz: it boots throw-away stacks and drives the real site in a real browser
 * (Playwright, Chromium) through everything a learner does.
 *
 * Two topologies run the same specs:
 * - `dev` — the site's dev server and a development proctor, one origin through the dev proxy, as `dev` starts them;
 * - `rehearsal` — the release build of the site with the proctor origin baked in, served as static files from one
 *   origin as a CDN does, and a production-mode proctor on another origin that grants exactly that site origin: the
 *   cross-origin topology that is deployed.
 *
 * Every stack has its own ports, proctor data directory, dependency cache and logs under the git-ignored
 * `.🧬semio/🎓️teaching/architecture-quiz-e2e/<run>/`, and a loopback control endpoint through which a spec takes its proctor
 * away and brings it back. Everything is stopped and deleted afterwards — on success, failure, Ctrl+C or termination; a
 * failed run keeps its directory (logs, traces, screenshots) and says where. Chromium comes from `PLAYWRIGHT_BROWSERS_PATH`,
 * else from the repo's tool cache, and is downloaded there once when it is missing.
 *
 * `test-e2e [dev] [rehearsal] [--serial] [--keep] [--proctor <executable>] [Playwright arguments…]`: no topology means
 * both, at once unless `--serial`; `--proctor` runs that executable (a release build, say) instead of building the
 * proctor; anything else goes to `playwright test` (`--grep`, `--project`, `--headed`, `--workers`, …).
 * @see ./🎚️config/🟦️.ts — the Playwright projects
 * @see ./🚶️learner/🟦️.ts — the learner the specs drive
 * @see ../🧱️stack/🟦️.ts — the parts the stacks are booted from
 * @see ../🧪️tests — the specs
 * https://playwright.dev/docs/test-cli */
import { spawn } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { createServer } from "node:http";
import { createServer as createListener, type AddressInfo } from "node:net";
import { join } from "node:path";
import { repoToolCacheEnv } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
import { terminateOwnedChildTree } from "../../../../🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts";
import { runRepositoryCommand as runOwnedCommand } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import { startNativeProgress } from "../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { LAUNCHER_SIGNALS, PROCTOR_DEV_CATALOG, buildProctor, launchProctor, proctorReady, type ProctorProcess } from "../../../🛂️proctor/🏗️bootstrap/🟦️.ts";
import { siteArtifactProblems } from "../🚀️deploy/🟦️.ts";
import { SITE_FIRST_ANSWER_MS, awaitReady, httpAnswers, launchOwned, serveStaticSite } from "../🧱️stack/🟦️.ts";

//#region 🗺️Topologies
/** 🗺️ The topologies the gate boots, in the order they are named. */
export const QUIZ_E2E_TOPOLOGIES = ["dev", "rehearsal"] as const;

/** 🗺️ One topology of the gate. */
export type QuizTopology = (typeof QUIZ_E2E_TOPOLOGIES)[number];

/** 🔌️ The throw-away ports of each topology — never the dev ports, so a developer's own stack keeps running. */
export const QUIZ_E2E_PORTS: Readonly<Record<QuizTopology, { readonly site: number; readonly proctor: number }>> = { dev: { site: 6161, proctor: 8891 }, rehearsal: { site: 6162, proctor: 8892 } };

/** 🧾️ The variables a spec reads: the topology it runs in, the proctor origin of its stack and the stack's control
 * endpoint (`PLAYWRIGHT_BASE_URL` is the site origin). */
export const QUIZ_E2E_ENVIRONMENT = { topology: "TEACHING_ARCHITECTURE_QUIZ_E2E_TOPOLOGY", proctor: "TEACHING_ARCHITECTURE_QUIZ_E2E_PROCTOR", control: "TEACHING_ARCHITECTURE_QUIZ_E2E_CONTROL" } as const;

/** 🗃️ Where the runs of the gate keep their stacks, relative to the repository root. */
export const QUIZ_E2E_DIRECTORY = [".🧬semio", "🎓️teaching", "architecture-quiz-e2e"] as const;

/** 👷️ The Playwright workers of each topology while two run at once; one alone takes the configuration's own. The pages
 * of the quiz are what the processors are busy with, so two topologies share what one would use. */
export const SHARED_WORKERS = 2;

const PLAYWRIGHT_CLI = ["node_modules", "playwright", "cli.js"] as const;
const PLAYWRIGHT_CONFIG = ["🎓️teaching", "🏛️architecture", "❓️quiz", "🎭️e2e", "🎚️config", "🟦️.ts"] as const;

/** 🧭️ What a command line asks for: the topologies (all when none is named), whether they run one after the other,
 * whether the run directory is kept, a proctor executable to run instead of building one, and the arguments of
 * `playwright test`. */
export function endToEndPlan(segments: readonly string[]): { readonly topologies: readonly QuizTopology[]; readonly serial: boolean; readonly keep: boolean; readonly proctor: string | undefined; readonly playwright: readonly string[] } {
  const at = segments.indexOf("--proctor");
  const proctor = at < 0 ? undefined : segments[at + 1];
  if (at >= 0 && proctor === undefined) throw new Error("--proctor needs the path of a proctor executable");
  const rest = segments.filter((_, index) => at < 0 || (index !== at && index !== at + 1));
  const named = QUIZ_E2E_TOPOLOGIES.filter((topology) => rest.includes(topology));
  const own = new Set<string>([...QUIZ_E2E_TOPOLOGIES, "--serial", "--keep"]);
  return { topologies: named.length === 0 ? QUIZ_E2E_TOPOLOGIES : named, serial: rest.includes("--serial"), keep: rest.includes("--keep"), proctor, playwright: rest.filter((segment) => !own.has(segment)) };
}
//#endregion 🗺️Topologies

//#region 🧱️Stacks
/** 🗣️ One progress line of the gate. */
function say(line: string): void {
  console.log(`[e2e] ${line}`);
}

/** 🧰️ What every stack of one run shares. */
interface Run {
  readonly repoRoot: string;
  readonly bundleRoot: string;
  readonly script: string;
  readonly portEnv: string;
  readonly work: string;
  readonly proctor: string;
  readonly signal: AbortSignal;
}

/** 🧱️ A booted stack: its origins, its control endpoint and the way to stop it. */
interface Stack {
  readonly topology: QuizTopology;
  readonly site: string;
  readonly proctor: string;
  readonly control: string;
  close(): Promise<void>;
}

/** 🔌️ Whether nothing listens on loopback `port`. */
function portFree(port: number): Promise<boolean> {
  return new Promise((accept) => {
    const probe = createListener();
    probe.once("error", () => accept(false));
    probe.listen(port, "127.0.0.1", () => probe.close(() => accept(true)));
  });
}

/** 🌿️ The environment of a stack's proctor: nothing of the caller's `PROCTOR_*`, the stack's port and data directory,
 * the site's catalog, and `posture` (the mode and what it needs). */
function proctorEnvironment(repoRoot: string, port: number, data: string, posture: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  const inherited = Object.fromEntries(Object.entries(process.env).filter(([name]) => !name.startsWith("PROCTOR_")));
  return { ...inherited, PROCTOR_PORT: String(port), PROCTOR_DATA: data, PROCTOR_CATALOG: join(repoRoot, ...PROCTOR_DEV_CATALOG), ...posture };
}

/** 🎛️ The control endpoint of one stack on a free loopback port: `POST /proctor/stop` ends the stack's proctor at once
 * (a crash, as far as the site can tell), `POST /proctor/start` launches it again over the same data and answers once
 * it is ready. */
function serveControl(proctor: { stop(): Promise<unknown>; start(): Promise<unknown> }): Promise<{ readonly url: string; close(): Promise<void> }> {
  const server = createServer((request, answer) => {
    const act = request.method !== "POST" ? undefined : request.url === "/proctor/stop" ? proctor.stop : request.url === "/proctor/start" ? proctor.start : undefined;
    const done = (status: number, body = ""): void => {
      answer.statusCode = status;
      answer.end(body);
    };
    if (act === undefined) return done(404, "POST /proctor/stop or /proctor/start");
    act().then(
      () => done(204),
      (error: unknown) => done(500, error instanceof Error ? error.message : String(error)),
    );
  });
  return new Promise((accept, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", () =>
      accept({
        url: `http://127.0.0.1:${(server.address() as AddressInfo).port}`,
        close: () =>
          new Promise<void>((closed) => {
            server.close(() => closed());
            server.closeAllConnections();
          }),
      }),
    );
  });
}

/** 🧱️ Boots one topology: its proctor (answering ready), its site (answering), and its control endpoint. Whatever was
 * started is stopped again when the boot fails. */
async function bootStack(topology: QuizTopology, run: Run): Promise<Stack> {
  const ports = QUIZ_E2E_PORTS[topology];
  const taken = (await Promise.all([ports.site, ports.proctor].map(async (port) => ((await portFree(port)) ? [] : [port])))).flat();
  if (taken.length > 0) throw new Error(`the ${topology} stack needs port ${taken.join(" and ")}, which something else listens on (another run of this gate?)`);
  const directory = join(run.work, topology);
  const site = `http://127.0.0.1:${ports.site}`;
  const proctor = `http://127.0.0.1:${ports.proctor}`;
  mkdirSync(join(directory, "proctor-data"), { recursive: true });
  const env = proctorEnvironment(run.repoRoot, ports.proctor, join(directory, "proctor-data"), topology === "dev" ? { PROCTOR_MODE: "development" } : { PROCTOR_MODE: "production", PROCTOR_ALLOWED_ORIGINS: site });
  const closing: (() => Promise<unknown>)[] = [];
  const close = async (): Promise<void> => {
    for (const end of closing.splice(0).reverse()) await end().catch((error: unknown) => say(`${topology}: ${error instanceof Error ? error.message : String(error)}`));
  };
  try {
    let running: ProctorProcess | undefined;
    const start = async (): Promise<void> => {
      if (running !== undefined) return;
      const launched = launchProctor(run.proctor, run.repoRoot, ["serve"], env, join(directory, "proctor.log"));
      running = launched;
      await awaitReady({ label: `the ${topology} proctor at ${proctor}`, probe: () => proctorReady(proctor), exited: launched.exited, signal: run.signal, say });
    };
    const stop = async (): Promise<void> => {
      const ending = running;
      running = undefined;
      await ending?.stop(0);
    };
    closing.push(stop);
    await start();
    say(`${topology}: proctor ready at ${proctor} (${env.PROCTOR_MODE})`);
    if (topology === "dev") {
      const server = launchOwned(process.execPath, [run.script, "dev-site"], run.bundleRoot, { ...process.env, [run.portEnv]: String(ports.site), PROCTOR_PORT: String(ports.proctor), TEACHING_ARCHITECTURE_QUIZ_CACHE: join(directory, "node_modules", ".vite"), TEACHING_ARCHITECTURE_QUIZ_WATCH: "off" }, join(directory, "site.log"));
      closing.push(() => server.stop());
      await awaitReady({ label: `the dev site at ${site}`, probe: () => httpAnswers(site, SITE_FIRST_ANSWER_MS), exited: server.exited, signal: run.signal, say });
      say(`dev: site ready at ${site}, proxying to ${proctor}`);
    } else {
      const artifact = join(directory, "site");
      say(`rehearsal: building the release site for ${proctor} (log: site-build.log)`);
      const build = launchOwned(process.execPath, [run.script, "build", "--outDir", artifact, "--emptyOutDir"], run.bundleRoot, { ...process.env, PROCTOR_URL: proctor }, join(directory, "site-build.log"));
      const cancel = (): void => void build.stop();
      run.signal.addEventListener("abort", cancel, { once: true });
      const stopProgress = startNativeProgress("rehearsal site build");
      const status = await build.exited.finally(() => {
        stopProgress();
        run.signal.removeEventListener("abort", cancel);
      });
      if (status !== 0) throw new Error(`the release build of the site failed (${status}); see ${join(directory, "site-build.log")}`);
      const problems = siteArtifactProblems(artifact, proctor);
      if (problems.length > 0) throw new Error(`the release build is not a site artifact:\n- ${problems.join("\n- ")}`);
      const served = await serveStaticSite(artifact, ports.site);
      closing.push(() => served.close());
      say(`rehearsal: static site at ${served.origin}, calling ${proctor} cross-origin`);
    }
    const control = await serveControl({ stop, start });
    closing.push(() => control.close());
    return { topology, site, proctor, control: control.url, close };
  } catch (error) {
    await close();
    throw error;
  }
}
//#endregion 🧱️Stacks

//#region 🎬️Specs
/** 🌐️ Makes sure the Chromium builds this Playwright version drives exist in `browsers`, downloading them once. */
async function ensureBrowser(repoRoot: string, browsers: string): Promise<void> {
  const manifest = JSON.parse(readFileSync(join(repoRoot, "node_modules", "playwright-core", "browsers.json"), "utf8")) as { readonly browsers: readonly { readonly name: string; readonly revision: string }[] };
  const installed = ["chromium", "chromium-headless-shell"].every((name) => existsSync(join(browsers, `${name.replaceAll("-", "_")}-${manifest.browsers.find((browser) => browser.name === name)?.revision}`, "INSTALLATION_COMPLETE")));
  if (installed) return;
  say(`installing Chromium for Playwright into ${browsers}`);
  await runOwnedCommand(process.execPath, [join(repoRoot, ...PLAYWRIGHT_CLI), "install", "chromium"], repoRoot, "architecture-quiz:test-e2e browser", undefined, { env: { ...process.env, PLAYWRIGHT_BROWSERS_PATH: browsers } });
}

/** 🎬️ Runs the specs against one stack, every output line prefixed with its topology; resolves with the exit status. */
function runSpecs(stack: Stack, run: Run, browsers: string, output: string, forwarded: readonly string[]): Promise<number> {
  const env = { ...process.env, PLAYWRIGHT_BROWSERS_PATH: browsers, PLAYWRIGHT_BASE_URL: stack.site, [QUIZ_E2E_ENVIRONMENT.topology]: stack.topology, [QUIZ_E2E_ENVIRONMENT.proctor]: stack.proctor, [QUIZ_E2E_ENVIRONMENT.control]: stack.control };
  const child = spawn(process.execPath, [join(run.repoRoot, ...PLAYWRIGHT_CLI), "test", "--config", join(run.repoRoot, ...PLAYWRIGHT_CONFIG), "--output", join(output, stack.topology), ...forwarded], { cwd: run.repoRoot, env, stdio: ["ignore", "pipe", "pipe"], windowsHide: true });
  const cancel = (): void => terminateOwnedChildTree(child);
  run.signal.addEventListener("abort", cancel, { once: true });
  for (const stream of [child.stdout!, child.stderr!]) {
    let rest = "";
    stream.setEncoding("utf8").on("data", (chunk: string) => {
      const lines = `${rest}${chunk}`.split(/\r?\n/u);
      rest = lines.pop() ?? "";
      for (const line of lines) console.log(`[${stack.topology}] ${line}`);
    });
    stream.on("end", () => rest !== "" && console.log(`[${stack.topology}] ${rest}`));
  }
  return new Promise((accept) => {
    child.once("error", (error) => {
      say(`${stack.topology}: ${error.message}`);
      accept(1);
    });
    child.once("close", (code) => {
      run.signal.removeEventListener("abort", cancel);
      accept(code ?? 1);
    });
  });
}

/** 🎭️ `test-e2e` — builds the proctor once, boots the topologies of `segments`, runs the specs against each, and stops
 * and deletes everything again. It fails when a stack does not boot or any spec fails. */
export async function runQuizEndToEnd(repoRoot: string, bundleRoot: string, script: string, portEnv: string, segments: readonly string[]): Promise<void> {
  const plan = endToEndPlan(segments);
  const work = join(repoRoot, ...QUIZ_E2E_DIRECTORY, `${new Date().toISOString().replace(/[-:]|\.\d+Z$/gu, "")}-${process.pid}`);
  const browsers = repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH!;
  const interruption = new AbortController();
  let interrupted: NodeJS.Signals | undefined;
  const interrupt = (signal: NodeJS.Signals): void => {
    interrupted ??= signal;
    interruption.abort();
  };
  for (const signal of LAUNCHER_SIGNALS) process.on(signal, interrupt);
  const stacks: Stack[] = [];
  const started = Date.now();
  let failure: string | undefined = "the run did not finish";
  try {
    mkdirSync(work, { recursive: true });
    say(`topologies ${plan.topologies.join(", ")}${plan.serial ? " (one after the other)" : ""}; run directory ${work}`);
    await ensureBrowser(repoRoot, browsers);
    say(plan.proctor === undefined ? "building the proctor (a first build takes minutes)" : `running the proctor ${plan.proctor} as it is`);
    const run: Run = { repoRoot, bundleRoot, script, portEnv, work, proctor: plan.proctor ?? (await buildProctor(repoRoot, process.env, interruption.signal)), signal: interruption.signal };
    const statuses: number[] = [];
    const sharing = !plan.serial && plan.topologies.length > 1 && !plan.playwright.some((argument) => /^(?:--workers|-j)(?:=|$)/u.test(argument));
    const forwarded = sharing ? [...plan.playwright, "--workers", String(SHARED_WORKERS)] : plan.playwright;
    const play = async (topology: QuizTopology): Promise<void> => {
      const stack = await bootStack(topology, run);
      stacks.push(stack);
      say(`${topology}: running the specs against ${stack.site}`);
      const status = await runSpecs(stack, run, browsers, join(work, "report"), forwarded);
      say(`${topology}: specs ${status === 0 ? "passed" : `failed (${status})`}`);
      statuses.push(status);
    };
    if (plan.serial) for (const topology of plan.topologies) await play(topology);
    else {
      const outcomes = await Promise.allSettled(plan.topologies.map(play));
      for (const outcome of outcomes) if (outcome.status === "rejected") throw outcome.reason;
    }
    failure = interrupted !== undefined ? `stopped by ${interrupted}` : statuses.every((status) => status === 0) ? undefined : "specs failed";
  } catch (error) {
    failure = interrupted !== undefined ? `stopped by ${interrupted}` : error instanceof Error ? error.message : String(error);
  } finally {
    for (const signal of LAUNCHER_SIGNALS) process.off(signal, interrupt);
    interruption.abort();
    await Promise.all(stacks.map((stack) => stack.close()));
    if ((failure === undefined && !plan.keep) || readdirSync(work).length === 0) rmSync(work, { recursive: true, force: true, maxRetries: 20, retryDelay: 200 });
    else say(`kept ${work} (proctor and site logs per topology, report/ with traces and screenshots)`);
  }
  if (failure !== undefined) throw new Error(`end-to-end gate failed after ${Math.round((Date.now() - started) / 1_000)} s: ${failure}`);
  say(`passed in ${Math.round((Date.now() - started) / 1_000)} s: ${plan.topologies.join(", ")}`);
}
//#endregion 🎬️Specs
