/** 🎭️ The end-to-end gate of the architecture course: it boots throw-away stacks and drives the real site in a real browser
 * (Playwright, Chromium) through everything a learner does.
 *
 * Two topologies run the same specs:
 * - `dev` — the site's dev server and a development proctor, one origin through the dev proxy, as `dev` starts them;
 * - `rehearsal` — the release build of the site with the proctor origin baked in, served as static files from one
 *   origin as a CDN does, and a production-mode proctor on another origin that grants exactly that site origin: the
 *   cross-origin topology that is deployed.
 *
 * Every stack has its own ports, proctor data directory and logs under the selected ticket's generated directory
 * (or `.🧬semio/🎓️teaching/architecture-course-e2e/<run>/`), and a loopback control endpoint through which a spec takes its proctor
 * away and brings it back. Everything is stopped and deleted afterwards — on success, failure, Ctrl+C or termination; a
 * failed run keeps its directory (logs, traces, screenshots) and says where. Chromium comes from `PLAYWRIGHT_BROWSERS_PATH`,
 * else from the repo's tool cache, and is downloaded there once when it is missing.
 * The compiler and its output stay in bounded repository storage. The rehearsal retains an immutable artifact reader
 * through the complete browser journey, closes its static listener, and then releases that reader.
 *
 * The specs are sharded (`./🔒️sessions/🔣️.json`): the `shared` shard's projects share one proctor and run on several Playwright
 * workers, the `exclusive` shard's projects take the proctor away or count who is online and run on one worker of a proctor
 * of their own. The dev topology therefore boots two stacks at once (one dev site and proctor per shard); the rehearsal
 * stack, whose release build bakes its proctor origin in, runs its shards one after the other. Every stack first runs the
 * `boot` spec once, so a cold dev server has transformed everything before a wave starts, and the stacks boot one after the
 * other, so the second dev server finds the optimized dependencies of the first. How many stacks and workers run at once
 * comes from the worker policy of the admitted memory and cores (`SEMIO_E2E_STACKS`, `SEMIO_PLAYWRIGHT_WORKERS`).
 *
 * `test-e2e [--topology <both|dev|rehearsal>] [--serial] [--keep] [--proctor <executable>] [-- <Playwright arguments…>]`: no topology means
 * both, at once unless `--serial`; `--proctor` runs that executable (a release build, say) instead of building the
 * proctor; words after `--` go to `playwright test` (`--grep`, `--headed`, …); `--project` selects projects of every shard
 * and `--workers` sets the workers of the shared shard.
 * @see ./🎚️config/🟦️.ts — the Playwright projects
 * @see ./🚶️learner/🟦️.ts — the learner the specs drive
 * @see ../🧱️stack/🟦️.ts — the parts the stacks are booted from
 * @see ../🧪️tests — the specs
 * https://playwright.dev/docs/test-cli */
import { buildModeEnvironment } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";

import { existsSync, mkdirSync, readFileSync, readdirSync, realpathSync, rmSync } from "node:fs";
import { createServer } from "node:http";
import { createRequire } from "node:module";
import { createServer as createListener, type AddressInfo } from "node:net";
import { dirname, isAbsolute, join, posix, resolve } from "node:path";
import { repoToolCacheEnv, runRepositoryCommand as runOwnedCommand } from "@semio-tech/process/execution";
import { terminateOwnedChildTree } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import type { ScriptInvocation } from "@semio-tech/process/routing";
import { scriptInvocationBudget, scriptInvocationEnvironment } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "@semio-tech/process/compiler-capacity";
import { buildViteArtifact } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/⚡️caching/🌐️vite/🟦️.ts";
import { withViteArtifactV1 } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/⚡️caching/🌐️vite/💾️storage/🟦️.ts";
import { ticketOutput } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚡️caching/🎫️output/🟦️.ts";
import { readNativeCallerAuthorityV1 } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import { LAUNCHER_SIGNALS, PROCTOR_DEV_COURSE, launchProctor, proctorReady, type ProctorArtifact, type ProctorProcess } from "../../../🛂️proctor/🏗️bootstrap/🟦️.ts";
import { acquireProctor } from "../../../🛂️proctor/🏗️bootstrap/🧾️product/🟦️.ts";
import { siteArtifactProblems } from "../🚀️deploy/🌐️site/🟦️.ts";
import { SITE_FIRST_ANSWER_MS, awaitReady, httpAnswers, launchOwned, serveStaticSite } from "../🧱️stack/🟦️.ts";
import {courseOriginalBuildModeV1} from "../🏗️builder/🎚️mode/🟦️.ts";
import { SHARD_TABLE, allocateWorkersV1, runTopologySessionsV1, scheduleStacksV1, splitProjectArgumentsV1, splitWorkerArgumentsV1, waveArgumentsV1, workerBudgetV1, type StackPlanV1 } from "./🔒️sessions/🟦️.ts";
import { startToolV1 } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { plannedTestWorkersV1, withPlannedTestWorkersV1 } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/👷️workers/🟦️.ts";

//#region 🗺️Topologies
/** 🗺️ The topologies the gate boots, in the order they are named. */
export const COURSE_E2E_TOPOLOGIES = ["dev", "rehearsal"] as const;

/** 🕸️ One topology of the gate. */
export type CourseTopology = (typeof COURSE_E2E_TOPOLOGIES)[number];

/** 🧾️ The variables a spec reads: the topology it runs in, the proctor origin of its stack and the stack's control
 * endpoint (`PLAYWRIGHT_BASE_URL` is the site origin). */
export const COURSE_E2E_ENVIRONMENT = { topology: "TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY", proctor: "TEACHING_ARCHITECTURE_COURSE_E2E_PROCTOR", control: "TEACHING_ARCHITECTURE_COURSE_E2E_CONTROL" } as const;

/** 🗃️ Where the runs of the gate keep their stacks, relative to the repository root. */
export const COURSE_E2E_DIRECTORY = [".🧬semio", "🎓️teaching", "architecture-course-e2e"] as const;

/** 🎫️ Keeps browser diagnostics within the selected ticket, or the development cache when no ticket is selected. */
export function endToEndWorkDirectory(repoRoot: string, environment: NodeJS.ProcessEnv = process.env): string {
  return environment.SEMIO_TICKET_DIR ? join(ticketOutput(repoRoot, [], environment), "e2e") : join(repoRoot, ...COURSE_E2E_DIRECTORY);
}

/** 🏭️ Carries the rehearsal proctor origin into its isolated site build child. */
export function rehearsalBuildEnvironment(proctor: string, environment: Readonly<Record<string, string | undefined>> = process.env): Record<string, string | undefined> {
  return buildModeEnvironment("release", { ...environment, PROCTOR_URL: proctor });
}

/** 🎼️ Authors a bounded rehearsal product without changing production output or passing compiler escape arguments. */
export function rehearsalBuildPlan(bundleRoot: string, proctor: string, environment: Readonly<Record<string, string | undefined>> = process.env): { root: string; config: string; mode: "release"; owner: string; environment: Record<string, string | undefined> } {
  return { root: bundleRoot, config: resolve(bundleRoot, "../../🏗️builder/🌐️vite/🟦️.ts"), mode: "release", owner: "architecture-course:rehearsal", environment: rehearsalBuildEnvironment(proctor, environment) };
}
/** 👷️ The class the worker policy learns the sizes of this gate's stacks and Playwright workers under: the stacks that run at once come from the `e2e-stacks` runner, the Playwright workers of the shared shard from the `playwright` runner of the share that is left after them; multi-user scenarios open their own learner contexts. */
export const COURSE_E2E_CLASS = "architecture-course";

/** 👥️ How many Playwright workers in all the gate asks the worker policy for (the policy still bounds them by the admitted memory and cores and by its maximum, and `SEMIO_PLAYWRIGHT_WORKERS` overrides them). */
const PLAYWRIGHT_WORKERS_WANTED = 4;

const PLAYWRIGHT_CLI = ["node_modules", "playwright", "cli.js"] as const;
const PLAYWRIGHT_CONFIG = ["🎓️teaching", "🏛️architecture", "📚️course", "🎭️e2e", "🎚️config", "🟦️.ts"] as const;

/** 📋️ What a command line asks for: the topologies (all when none is named), whether they run one after the other,
 * whether the run directory is kept, a proctor executable to run instead of building one, and the arguments of
 * `playwright test`. */
export function endToEndPlan(segments: readonly string[]): { readonly topologies: readonly CourseTopology[]; readonly serial: boolean; readonly keep: boolean; readonly proctor: string | undefined; readonly playwright: readonly string[] } {
  const plan = { topologies: [...COURSE_E2E_TOPOLOGIES] as CourseTopology[], serial: false, keep: false, proctor: undefined as string | undefined, playwright: [] as string[] };
  const seen = new Set<string>();
  for (let index = 0; index < segments.length; index++) {
    const option = segments[index]!;
    if (option === "--") { plan.playwright = segments.slice(index + 1); break; }
    if (!["--topology", "--serial", "--keep", "--proctor"].includes(option)) throw new Error(`Unknown end-to-end option ${JSON.stringify(option)}; put Playwright arguments after --`);
    if (seen.has(option)) throw new Error(`Duplicate end-to-end option ${option}`);
    seen.add(option);
    if (option === "--serial") plan.serial = true;
    else if (option === "--keep") plan.keep = true;
    else {
      const value = segments[++index];
      if (option === "--topology") {
        if (value !== "both" && value !== "dev" && value !== "rehearsal") throw new Error("--topology needs both, dev or rehearsal");
        plan.topologies = value === "both" ? [...COURSE_E2E_TOPOLOGIES] : [value];
      } else {
        if (!value || value.startsWith("--")) throw new Error("--proctor needs the path of a proctor executable");
        plan.proctor = value;
      }
    }
  }
  return plan;
}
/** 🛤️ Where a `--proctor` executable is: absolute values stay, relative ones resolve against the repository root, never against the package directory the build tool runs the script in. */
export function endToEndProctorPath(repoRoot: string, value: string, paths: Pick<typeof posix, "isAbsolute" | "resolve"> = { isAbsolute, resolve }): string {
  return paths.isAbsolute(value) ? value : paths.resolve(repoRoot, value);
}
/** 🧭️ The order the topologies boot in: the rehearsal first, because its release build needs the whole host and the dev
 * site, once up, holds the whole host for as long as it runs. */
export function bootOrder(topologies: readonly CourseTopology[]): readonly CourseTopology[] {
  return [...topologies].sort((left, right) => Number(right === "rehearsal") - Number(left === "rehearsal"));
}

//#endregion 🗺️Topologies

//#region 🧱️Stacks
/** 🗣️ One progress line of the gate. */
function say(line: string): void {
  console.log(`[e2e] ${line}`);
}

/** 🧰️ What every stack of one run shares. */
interface Run {
  readonly invocation: ScriptInvocation;
  readonly repoRoot: string;
  readonly bundleRoot: string;
  readonly script: string;
  readonly portEnv: string;
  readonly work: string;
  readonly proctor: string | ProctorArtifact;
  readonly signal: AbortSignal;
  readonly browsers: string;
  readonly report: string;
}

/** 🍰️ One stack to boot: what it hosts and the Playwright workers of every wave it runs. */
interface Slot {
  readonly plan: StackPlanV1;
  readonly workers: readonly number[];
}

/** 🧱️ A booted stack: its origins, its control endpoint and the way to stop it. */
interface Stack {
  readonly topology: CourseTopology;
  readonly label: string;
  readonly slot: Slot;
  readonly site: string;
  readonly proctor: string;
  readonly control: string;
}

/** 🧯️ Ends a bounded Node port probe before its caller may bind the selected ports. */
function probePorts(source: string, args: readonly string[], signal?: AbortSignal): Promise<string> {
  const child = startToolV1("node", ["--input-type=module", "-e", source, ...args], { stdio: ["ignore", "pipe", "pipe"], windowsHide: true, signal: signal ? AbortSignal.any([signal, AbortSignal.timeout(10_000)]) : AbortSignal.timeout(10_000) });
  let output = "";
  let failure = "";
  child.stdout!.on("data", chunk => { output += chunk; });
  child.stderr!.on("data", chunk => { failure += chunk; });
  return new Promise((accept, reject) => {
    child.once("error", reject);
    child.once("close", code => {
      const answer = output.trim();
      if (code !== 0) reject(Error(`port probe failed (${code}): ${failure || output}`));
      else accept(answer);
    });
  });
}

/** 🔌️ Whether a loopback port is free after the probe has released it. */
export async function portFree(port: number, signal?: AbortSignal): Promise<boolean> {
  const source = "import {createServer} from 'node:net';const probe=createServer();probe.once('error',()=>process.stdout.write('false'));probe.listen(Number(process.argv[1]),'127.0.0.1',()=>probe.close(()=>process.stdout.write('true')));";
  const answer = await probePorts(source, [String(port)], signal);
  if (answer !== "true" && answer !== "false") throw Error(`port ${port} probe returned ${answer}`);
  return answer === "true";
}

/** 🫧️ Fresh OS-selected site and proctor ports prevent old browser tabs from rejoining a later isolated stack. */
export async function endToEndPorts(signal?: AbortSignal): Promise<{ readonly site: number; readonly proctor: number }> {
  const source = "import {createServer} from 'node:net';const servers=[createServer(),createServer()];const ports=await Promise.all(servers.map(server=>new Promise((resolve,reject)=>{server.once('error',reject);server.listen(0,'127.0.0.1',()=>resolve(server.address().port));})));await Promise.all(servers.map(server=>new Promise(resolve=>server.close(resolve))));process.stdout.write(JSON.stringify(ports));";
  const ports: unknown = JSON.parse(await probePorts(source, [], signal));
  if (!Array.isArray(ports) || ports.length !== 2 || ports.some(port => !Number.isInteger(port) || port < 1 || port > 65535) || ports[0] === ports[1]) throw Error("port probe returned invalid stack ports");
  return { site: ports[0], proctor: ports[1] };
}

/** 🌿️ The environment of a stack's proctor: nothing of the caller's `PROCTOR_*`, the stack's port and data directory,
 * the site's course, and `posture` (the mode and what it needs). */
function proctorEnvironment(repoRoot: string, port: number, data: string, posture: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  const inherited = Object.fromEntries(Object.entries(process.env).filter(([name]) => !name.startsWith("PROCTOR_")));
  return { ...inherited, PROCTOR_PORT: String(port), PROCTOR_DATA: data, PROCTOR_COURSE: join(repoRoot, ...PROCTOR_DEV_COURSE), ...posture };
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

/** 👓️ Holds the site the last build published under the artifact reader's custody until `release`, which settles once the
 * custody is returned. */
function holdSiteArtifact(run: Run): Promise<{ readonly path: string; release(): Promise<void> }> {
  return new Promise((accept, reject) => {
    let finish!: () => void;
    const finished = new Promise<void>((done) => (finish = done));
    const custody: Promise<void> = withViteArtifactV1(run.repoRoot, "release", "architecture-course:rehearsal", run.invocation, scriptCompilerAdmissionControlV1(run.invocation, "vite:rehearsal-reader", 0), async (artifact) => {
      accept({ path: artifact.path, release: () => (finish(), custody) });
      await finished;
    });
    custody.catch(reject);
  });
}

/** 🥞️ Retains one ready stack of a topology and its artifact reader through the learner journeys, then stops every owned part; the stack has run the warm-up spec before `consume` starts. */
async function withStack(slot: Slot, run: Run, consume: (stack: Stack) => Promise<void>): Promise<void> {
  const topology = slot.plan.topology as CourseTopology;
  const label = slot.plan.label;
  const ports = await endToEndPorts(run.signal);
  const taken = (await Promise.all([ports.site, ports.proctor].map(async (port) => ((await portFree(port, run.signal)) ? [] : [port])))).flat();
  if (taken.length > 0) throw new Error(`the ${label} stack needs port ${taken.join(" and ")}, which something else listens on (another run of this gate?)`);
  const directory = join(run.work, label);
  const site = `http://127.0.0.1:${ports.site}`;
  const proctor = `http://127.0.0.1:${ports.proctor}`;
  mkdirSync(join(directory, "proctor-data"), { recursive: true });
  const env = proctorEnvironment(run.repoRoot, ports.proctor, join(directory, "proctor-data"), topology === "dev" ? { PROCTOR_MODE: "development" } : { PROCTOR_MODE: "production", PROCTOR_ALLOWED_ORIGINS: site });
  const closing: (() => Promise<unknown>)[] = [];
  const close = async (): Promise<void> => {
    const failures: unknown[] = [];
    for (const end of closing.splice(0).reverse()) await end().catch((error: unknown) => { failures.push(error); say(`${label}: ${error instanceof Error ? error.message : String(error)}`); });
    if (failures.length) throw new AggregateError(failures, `The ${label} stack did not stop cleanly`);
  };
  try {
    let running: ProctorProcess | undefined;
    const start = async (): Promise<void> => {
      if (running !== undefined) return;
      const launched = typeof run.proctor === "string" ? launchProctor(run.proctor, run.repoRoot, ["serve"], env, join(directory, "proctor.log")) : run.proctor.launch(["serve"], env, join(directory, "proctor.log"));
      running = launched;
      await awaitReady({ label: `the ${label} proctor at ${proctor}`, probe: () => proctorReady(proctor), exited: launched.exited, signal: run.signal, say });
    };
    const stop = async (): Promise<void> => {
      const ending = running;
      running = undefined;
      await ending?.stop(0);
    };
    closing.push(stop);
    await start();
    say(`${label}: proctor ready at ${proctor} (${env.PROCTOR_MODE})`);
    const control = await serveControl({ stop, start });
    closing.push(() => control.close());
    const stack: Stack = { topology, label, slot, site, proctor, control: control.url };
    if (topology === "dev") {
      const server = launchOwned(process.execPath, [run.script, "dev-site"], run.bundleRoot, scriptInvocationEnvironment(run.invocation, { ...process.env, [run.portEnv]: String(ports.site), PROCTOR_PORT: String(ports.proctor), TEACHING_ARCHITECTURE_COURSE_WATCH: "off" }), join(directory, "site.log"));
      closing.push(() => server.stop());
      await awaitReady({ label: `the ${label} site at ${site}`, probe: () => httpAnswers(site, SITE_FIRST_ANSWER_MS), exited: server.exited, signal: run.signal, say });
      say(`${label}: site ready at ${site}, proxying to ${proctor}`);
      await warmUp(stack, run);
      await consume(stack);
    } else {
      courseOriginalBuildModeV1(run.invocation, "release");
      say(`${label}: building the release site for ${proctor}`);
      await buildViteArtifact({ ...rehearsalBuildPlan(run.bundleRoot, proctor), workspace: run.repoRoot, invocation: run.invocation, control: scriptCompilerAdmissionControlV1(run.invocation, "vite:rehearsal-build", 0), signal: run.signal });
      const held = await holdSiteArtifact(run);
      closing.push(() => held.release());
      const artifact = held.path;
      const problems = siteArtifactProblems(artifact, proctor);
      if (problems.length > 0) throw new Error(`the release build is not a site artifact:\n- ${problems.join("\n- ")}`);
      const served = await serveStaticSite(artifact, ports.site);
      closing.push(() => served.close());
      say(`${label}: static site at ${served.origin}, calling ${proctor} cross-origin`);
      await warmUp(stack, run);
      await consume(stack);
    }
  } finally {
    await close();
  }
}
//#endregion 🧱️Stacks

//#region 🎬️Specs
/** 🌐️ Makes sure the Chromium builds this Playwright version drives exist in `browsers`, downloading them once. The
 * build list is read from the `playwright-core` the `playwright` package itself resolves (the isolated install links no
 * `playwright-core` at the root). */
async function ensureBrowser(invocation: ScriptInvocation, repoRoot: string, browsers: string, signal: AbortSignal): Promise<void> {
  signal.throwIfAborted();
  const core = dirname(createRequire(realpathSync(join(repoRoot, ...PLAYWRIGHT_CLI))).resolve("playwright-core/package.json"));
  const manifest = JSON.parse(readFileSync(join(core, "browsers.json"), "utf8")) as { readonly browsers: readonly { readonly name: string; readonly revision: string }[] };
  const installed = ["chromium", "chromium-headless-shell"].every((name) => existsSync(join(browsers, `${name.replaceAll("-", "_")}-${manifest.browsers.find((browser) => browser.name === name)?.revision}`, "INSTALLATION_COMPLETE")));
  if (installed) return;
  say(`installing Chromium for Playwright into ${browsers}`);
  await runOwnedCommand(invocation, process.execPath, [join(repoRoot, ...PLAYWRIGHT_CLI), "install", "chromium"], repoRoot, "architecture-course:test-e2e browser", { version: 1, maximumElapsedMilliseconds: scriptInvocationBudget(invocation, 600_000), output: readNativeCallerAuthorityV1(invocation.capabilities).transport }, { env: { ...process.env, PLAYWRIGHT_BROWSERS_PATH: browsers } });
}

/** 🎬️ Runs `playwright test` against one stack, every output line prefixed with the stack's label; resolves with the exit status and whether Playwright found no test to run. */
function runSpecs(stack: Stack, run: Run, output: string, forwarded: readonly string[]): Promise<{ readonly code: number; readonly empty: boolean }> {
  run.signal.throwIfAborted();
  const env = { ...process.env, PLAYWRIGHT_BROWSERS_PATH: run.browsers, PLAYWRIGHT_BASE_URL: stack.site, [COURSE_E2E_ENVIRONMENT.topology]: stack.topology, [COURSE_E2E_ENVIRONMENT.proctor]: stack.proctor, [COURSE_E2E_ENVIRONMENT.control]: stack.control };
  const child = startToolV1("node", [join(run.repoRoot, ...PLAYWRIGHT_CLI), "test", "--config", join(run.repoRoot, ...PLAYWRIGHT_CONFIG), "--output", output, ...forwarded], { cwd: run.repoRoot, env, stdio: ["ignore", "pipe", "pipe"], windowsHide: true });
  const cancel = (): void => terminateOwnedChildTree(child);
  run.signal.addEventListener("abort", cancel, { once: true });
  let empty = false;
  for (const stream of [child.stdout!, child.stderr!]) {
    let rest = "";
    const show = (line: string): void => {
      empty ||= /No tests found/u.test(line);
      console.log(`[${stack.label}] ${line}`);
    };
    stream.setEncoding("utf8").on("data", (chunk: string) => {
      const lines = `${rest}${chunk}`.split(/\r?\n/u);
      rest = lines.pop() ?? "";
      lines.forEach(show);
    });
    stream.on("end", () => rest !== "" && show(rest));
  }
  return new Promise((accept) => {
    child.once("error", (error) => {
      say(`${stack.label}: ${error.message}`);
      accept({ code: 1, empty: false });
    });
    child.once("close", (code) => {
      run.signal.removeEventListener("abort", cancel);
      accept({ code: code ?? 1, empty: empty && code !== 0 });
    });
  });
}

/** 🔥️ Runs the warm-up project (the `boot` spec) once against a stack that is up, so its dev server has transformed and optimized everything before a wave starts and the next dev server to boot finds the optimized dependencies. */
async function warmUp(stack: Stack, run: Run): Promise<void> {
  say(`${stack.label}: warming up with the ${SHARD_TABLE.warmup} spec`);
  const { code } = await runSpecs(stack, run, join(run.report, `${stack.label}-${SHARD_TABLE.warmup}`), ["--project", SHARD_TABLE.warmup, "--workers", "1"]);
  if (code !== 0) throw new Error(`the ${stack.label} stack did not pass its ${SHARD_TABLE.warmup} spec (${code})`);
}

/** 🏁️ `test-e2e` — builds the proctor once, boots the topologies of `segments`, runs the specs against each, and stops
 * and deletes everything again. It fails when a stack does not boot or any spec fails. */
export async function runCourseEndToEnd(invocation: ScriptInvocation, repoRoot: string, bundleRoot: string, script: string, portEnv: string, segments: readonly string[]): Promise<void> {
  const plan = endToEndPlan(segments);
  courseOriginalBuildModeV1(invocation,plan.topologies.includes("rehearsal")?"release":"dev");
  const work = join(endToEndWorkDirectory(repoRoot), `${new Date().toISOString().replace(/[-:]|\.\d+Z$/gu, "")}-${process.pid}`);
  const browsers = repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH!;
  const interruption = new AbortController();
  const signal = AbortSignal.any([invocation.control.signal, interruption.signal]);
  let interrupted: NodeJS.Signals | undefined;
  const interrupt = (signal: NodeJS.Signals): void => {
    interrupted ??= signal;
    interruption.abort();
  };
  for (const signal of LAUNCHER_SIGNALS) process.on(signal, interrupt);
  const started = Date.now();
  let failure: string | undefined = "the run did not finish";
  let artifact: ProctorArtifact | undefined;
  try {
    mkdirSync(work, { recursive: true });
    say(`topologies ${plan.topologies.join(", ")}${plan.serial ? " (one after the other)" : ""}; run directory ${work}`);
    await ensureBrowser(invocation, repoRoot, browsers, signal);
    const given = plan.proctor === undefined ? undefined : endToEndProctorPath(repoRoot, plan.proctor);
    say(given === undefined ? "building the proctor (a first build takes minutes)" : `running the proctor ${given} as it is`);
    if (given === undefined) artifact = await acquireProctor(repoRoot, invocation, process.env, signal);
    const run: Run = { invocation, repoRoot, bundleRoot, script, portEnv, work, proctor: given ?? artifact!, signal, browsers, report: join(work, "report") };
    const statuses: number[] = [];
    const ran: boolean[] = [];
    const { projects: requested, rest } = splitProjectArgumentsV1(plan.playwright);
    const explicit = splitWorkerArgumentsV1(rest).workers;
    const order = bootOrder(plan.topologies);
    const wanted = plan.serial ? 1 : scheduleStacksV1(SHARD_TABLE, order, requested, Number.MAX_SAFE_INTEGER, false).stacks.length;
    const play = async (stack: Stack): Promise<void> => {
      say(`${stack.label}: running ${stack.slot.plan.waves.map((wave) => `${wave.shard} (${wave.projects.join(", ")})`).join(", then ") || "nothing but the warm-up"} against ${stack.site}`);
      for (const [index, wave] of stack.slot.plan.waves.entries()) {
        const { code, empty } = await runSpecs(stack, run, join(run.report, `${stack.label}-${wave.shard}`), waveArgumentsV1(plan.playwright, wave, stack.slot.workers[index]!, explicit));
        say(`${stack.label}: ${wave.shard} ${code === 0 ? "passed" : empty ? "found no test to run" : `failed (${code})`}`);
        statuses.push(empty ? 0 : code);
        ran.push(!empty);
      }
    };
    await withPlannedTestWorkersV1("e2e-stacks", process.env, COURSE_E2E_CLASS, (stacks) => {
      const schedule = scheduleStacksV1(SHARD_TABLE, order, requested, stacks.workers, plan.serial);
      const workers = allocateWorkersV1(schedule.stacks, workerBudgetV1(plannedTestWorkersV1("playwright", process.env, COURSE_E2E_CLASS, PLAYWRIGHT_WORKERS_WANTED, say), stacks.workerMegabytes * (schedule.concurrent ? schedule.stacks.length : 1)), schedule.concurrent);
      say(`${schedule.stacks.length} stack(s)${schedule.concurrent ? " at once" : " one after the other"}: ${schedule.stacks.map((stack, index) => `${stack.label} [${stack.waves.map((wave, position) => `${wave.shard} x${workers[index]![position]}`).join(" > ")}]`).join(", ")}`);
      const slots: readonly Slot[] = schedule.stacks.map((stack, index) => ({ plan: stack, workers: workers[index]! }));
      return runTopologySessionsV1(slots, !schedule.concurrent, (slot, consume) => withStack(slot, run, consume), play);
    }, wanted, say);
    if (ran.length > 0 && !ran.some(Boolean)) throw new Error("Playwright found no test to run in any shard");
    failure = interrupted !== undefined ? `stopped by ${interrupted}` : statuses.every((status) => status === 0) ? undefined : "specs failed";
  } catch (error) {
    failure = interrupted !== undefined ? `stopped by ${interrupted}` : error instanceof Error ? error.message : String(error);
  } finally {
    try { await artifact?.close(); } catch (error) { failure = `${failure ? failure + "; " : ""}proctor artifact close failed: ${error instanceof Error ? error.message : String(error)}`; }
    for (const signal of LAUNCHER_SIGNALS) process.off(signal, interrupt);
    interruption.abort();
    if (existsSync(work) && ((failure === undefined && !plan.keep) || readdirSync(work).length === 0)) rmSync(work, { recursive: true, force: true, maxRetries: 20, retryDelay: 200 });
    else say(`kept ${work} (proctor and site logs per topology, report/ with traces and screenshots)`);
  }
  if (failure !== undefined) throw new Error(`end-to-end gate failed after ${Math.round((Date.now() - started) / 1_000)} s: ${failure}`);
  say(`passed in ${Math.round((Date.now() - started) / 1_000)} s: ${plan.topologies.join(", ")}`);
}
//#endregion 🎬️Specs
