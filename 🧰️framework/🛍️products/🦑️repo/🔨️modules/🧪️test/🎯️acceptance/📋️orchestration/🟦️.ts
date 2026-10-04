//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🎯️ Repository acceptance: the check-result record every acceptance harness writes, the goal plan (the ordered, grouped
// verification of the repository goal) and the goal gate that runs the plan against one hub + one `s` serve and writes a
// schema-bound summary plus an English and a German human summary.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { spawn, spawnSync, type ChildProcess } from "node:child_process";
import { createWriteStream, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, resolve } from "node:path";
import { Script } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { DEPENDENCY_INTERFACE_OWNERS, dependencyTestDomain } from "../../../📚️library/🕸️dependencies/📇️inventory/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️Schema
/** 🧬️ Repo-relative path of the acceptance JSON Schema — the one authority for every record below.
 * @see ../🧬️schema/🔣️.json */
export const ACCEPTANCE_SCHEMA_REL_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🧬️schema/🔣️.json";

/** 🗺️ Repo-relative path of the repository goal plan (acceptance ledger §8, session 13). */
export const GOAL_PLAN_REL_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🎚️config/🔣️.json";

/** 🗂️ Repo-relative, gitignored root of every goal-gate run's records and logs. */
export const GOAL_GATE_OUT_REL_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🤖️generated/🎯️acceptance";

/** 🌐️ One human sentence in both shipped languages. */
export type LocalizedText = Readonly<{ en: string; de: string }>;

/** 🚦️ The outcome of one check, step or run. */
export type AcceptanceStatus = "pass" | "fail" | "blocked" | "skipped";

/** 🧾️ `semio.acceptance.check-result/v1` — what one acceptance harness measured. */
export type AcceptanceCheckResult = Readonly<{
  schema: "semio.acceptance.check-result/v1";
  check: string;
  status: AcceptanceStatus;
  startedAt: string;
  finishedAt: string;
  durationMs: number;
  measured: Readonly<Record<string, number | string | boolean>>;
  summary: LocalizedText;
  evidence: readonly string[];
}>;

/** 🧩️ What a plan check needs from the gate's environment. */
export type AcceptanceRequirement = "hub" | "serve" | "localServe" | "backends" | "hubAdmin";

/** 🧪️ One check of the goal plan: an Nx target plus its arguments; `{hub}` and `{serve}` are substituted. */
export type GoalPlanCheck = Readonly<{
  id: string;
  title: LocalizedText;
  criteria: readonly string[];
  project: string;
  target: string;
  args: readonly string[];
  env?: Readonly<Record<string, string>>;
  requires: readonly AcceptanceRequirement[];
  browsers: number;
}>;

/** 🪜️ One step of the goal plan: its checks run serially or in parallel; a failed gating step blocks the rest. */
export type GoalPlanStep = Readonly<{ id: string; title: LocalizedText; mode: "serial" | "parallel"; gatesRest: boolean; optional?: boolean; checks: readonly GoalPlanCheck[] }>;

/** 🏁️ One outcome of the repository goal; a check counts towards outcome `N` when one of its criteria is `N.x`
 * (acceptance ledger numbering: 1 frontend, 2 hub, 3 collaboration, 4 AI over the semio MCP, 5 AGENTS.md). */
export type GoalPlanOutcome = Readonly<{ id: "1" | "2" | "3" | "4" | "5"; title: LocalizedText }>;

/** 🧩️ The requirements a provider can stand up when the command line does not name them. */
export type ProvidedRequirement = "hub" | "serve" | "localServe";

/** 🏗️ How the gate stands up one requirement zero-touch: an Nx target it holds for the whole run, answering at
 * `url` + `readyPath` within `readyBoundMs`; `{hub}` and `{runDir}` are substituted in `args` and `env`. A process that
 * exits while the url already answers is a reuse of a running server, not a failure. A hub provider's `adminCapability`
 * (repo-relative or absolute) names the admin-relay capability file its launcher keeps; the gate hands it to `hubAdmin`
 * checks when the command line names none. */
export type GoalPlanProvider = Readonly<{ url: string; readyPath: string; readyBoundMs: number; project: string; target: string; args: readonly string[]; env?: Readonly<Record<string, string>>; requires: readonly "hub"[]; adminCapability?: string }>;

/** 🗺️ `semio.acceptance.goal-plan/v1`. */
export type GoalPlan = Readonly<{ schema: "semio.acceptance.goal-plan/v1"; id: string; title: LocalizedText; outcomes: readonly GoalPlanOutcome[]; providers?: Readonly<Partial<Record<ProvidedRequirement, GoalPlanProvider>>>; steps: readonly GoalPlanStep[] }>;

/** 🏁️ The verdict of one outcome over every selected check that serves it. */
export type GoalSummaryOutcome = Readonly<{ id: GoalPlanOutcome["id"]; status: AcceptanceStatus; totals: Readonly<Record<AcceptanceStatus, number>>; checks: readonly string[] }>;

/** 📊️ `semio.acceptance.goal-summary/v1`. */
export type GoalSummary = Readonly<{
  schema: "semio.acceptance.goal-summary/v1";
  plan: string;
  hub: string | null;
  serve: string | null;
  localServe: string | null;
  startedAt: string;
  finishedAt: string;
  verdict: AcceptanceStatus;
  totals: Readonly<Record<AcceptanceStatus, number>>;
  outcomes: readonly GoalSummaryOutcome[];
  steps: readonly Readonly<{ id: string; status: AcceptanceStatus; checks: readonly AcceptanceCheckResult[] }>[];
}>;

type SchemaNode = Readonly<Record<string, unknown>>;

function schemaDocument(repoRoot: string): SchemaNode {
  return JSON.parse(readFileSync(join(repoRoot, ACCEPTANCE_SCHEMA_REL_PATH), "utf8")) as SchemaNode;
}

function typeOf(value: unknown): string {
  if (value === null) return "null";
  if (Array.isArray(value)) return "array";
  if (typeof value === "number") return Number.isInteger(value) ? "integer" : "number";
  return typeof value;
}

function typeMatches(value: unknown, wanted: string): boolean {
  const actual = typeOf(value);
  return actual === wanted || (wanted === "number" && actual === "integer");
}

/** 🔎️ Validates `value` against one `$defs` entry of the acceptance schema and returns every violation as
 * `<json pointer>: <reason>`. Interprets exactly the keywords the schema uses, so the schema file stays the authority. */
export function acceptanceSchemaViolations(repoRoot: string, definition: "checkResult" | "goalPlan" | "goalSummary", value: unknown): string[] {
  const root = schemaDocument(repoRoot);
  const defs = root.$defs as Readonly<Record<string, SchemaNode>>;
  const violations: string[] = [];
  const visit = (node: SchemaNode, current: unknown, pointer: string): void => {
    if (typeof node.$ref === "string") return visit(defs[node.$ref.replace("#/$defs/", "")]!, current, pointer);
    const at = pointer || "/";
    if ("const" in node && current !== node.const) violations.push(`${at}: expected ${JSON.stringify(node.const)}`);
    if (Array.isArray(node.enum) && !node.enum.includes(current)) violations.push(`${at}: not one of ${node.enum.join("|")}`);
    if (node.type !== undefined) {
      const types = Array.isArray(node.type) ? (node.type as string[]) : [node.type as string];
      if (!types.some((type) => typeMatches(current, type))) return void violations.push(`${at}: expected ${types.join("|")}, got ${typeOf(current)}`);
    }
    if (typeof current === "string") {
      if (typeof node.minLength === "number" && [...current].length < node.minLength) violations.push(`${at}: shorter than ${node.minLength}`);
      if (typeof node.maxLength === "number" && [...current].length > node.maxLength) violations.push(`${at}: longer than ${node.maxLength}`);
      if (typeof node.pattern === "string" && !new RegExp(node.pattern, "u").test(current)) violations.push(`${at}: does not match ${node.pattern}`);
    }
    if (typeof current === "number") {
      if (typeof node.minimum === "number" && current < node.minimum) violations.push(`${at}: below ${node.minimum}`);
      if (typeof node.maximum === "number" && current > node.maximum) violations.push(`${at}: above ${node.maximum}`);
    }
    if (Array.isArray(current)) {
      if (typeof node.minItems === "number" && current.length < node.minItems) violations.push(`${at}: fewer than ${node.minItems} items`);
      if (typeof node.maxItems === "number" && current.length > node.maxItems) violations.push(`${at}: more than ${node.maxItems} items`);
      if (node.uniqueItems === true && new Set(current.map((item) => JSON.stringify(item))).size !== current.length) violations.push(`${at}: items not unique`);
      if (node.items) current.forEach((item, index) => visit(node.items as SchemaNode, item, `${pointer}/${index}`));
    }
    if (typeOf(current) === "object") {
      const record = current as Record<string, unknown>;
      const properties = (node.properties ?? {}) as Readonly<Record<string, SchemaNode>>;
      for (const key of (node.required as string[] | undefined) ?? []) if (!(key in record)) violations.push(`${at}: missing ${key}`);
      if (typeof node.maxProperties === "number" && Object.keys(record).length > node.maxProperties) violations.push(`${at}: more than ${node.maxProperties} properties`);
      for (const [key, item] of Object.entries(record)) {
        if (properties[key]) visit(properties[key]!, item, `${pointer}/${key}`);
        else if (node.additionalProperties === false) violations.push(`${at}: unexpected ${key}`);
        else if (typeof node.additionalProperties === "object") visit(node.additionalProperties as SchemaNode, item, `${pointer}/${key}`);
      }
    }
  };
  visit(defs[definition]!, value, "");
  return violations;
}

/** 🗺️ Reads and validates a goal plan. */
export function readGoalPlan(repoRoot: string, path = join(repoRoot, GOAL_PLAN_REL_PATH)): GoalPlan {
  const plan = JSON.parse(readFileSync(path, "utf8")) as unknown;
  const violations = acceptanceSchemaViolations(repoRoot, "goalPlan", plan);
  if (violations.length) throw new Error(`goal plan ${path} violates the acceptance schema:\n${violations.join("\n")}`);
  const checks = (plan as GoalPlan).steps.flatMap((step) => step.checks);
  const leaking = checks.find((check) => check.args.some((arg) => /\{user\d(Email|Password)\}/u.test(arg)));
  if (leaking) throw new Error(`goal plan check ${leaking.id} passes a credential token as an argument; credentials belong in env`);
  const ids = checks.map((check) => check.id);
  const duplicate = ids.find((id, index) => ids.indexOf(id) !== index);
  if (duplicate) throw new Error(`goal plan check id ${duplicate} is not unique`);
  const outcomes = new Set((plan as GoalPlan).outcomes.map((outcome) => outcome.id));
  const orphan = checks.find((check) => check.criteria.some((criterion) => !outcomes.has(criterion.split(".")[0] as GoalPlanOutcome["id"])));
  if (orphan) throw new Error(`goal plan check ${orphan.id} names a criterion of an outcome the plan does not declare`);
  return plan as GoalPlan;
}
//#endregion 🧬️Schema

//#region 🧾️CheckResult
/** 🧭️ The environment variable through which the goal gate hands a harness the path its result record belongs at. */
export const ACCEPTANCE_RESULT_ENV = "SEMIO_ACCEPTANCE_RESULT";

/** 🧾️ Builds a check-result record; `finishedAt`/`durationMs` are taken now. */
export function acceptanceCheckResult(input: Readonly<{ check: string; status: AcceptanceStatus; startedAt: Date; measured: Readonly<Record<string, number | string | boolean>>; summary: LocalizedText; evidence?: readonly string[] }>): AcceptanceCheckResult {
  const finished = new Date();
  return {
    schema: "semio.acceptance.check-result/v1",
    check: input.check,
    status: input.status,
    startedAt: input.startedAt.toISOString(),
    finishedAt: finished.toISOString(),
    durationMs: Math.max(0, finished.getTime() - input.startedAt.getTime()),
    measured: input.measured,
    summary: input.summary,
    evidence: input.evidence ?? [],
  };
}

/** 📤️ Validates one harness's record and writes it where the goal gate asked for it (`SEMIO_ACCEPTANCE_RESULT`), and
 * always prints the English and German summary lines. A record the schema refuses is an error, never silently dropped. */
export function publishAcceptanceCheckResult(repoRoot: string, result: AcceptanceCheckResult): void {
  const violations = acceptanceSchemaViolations(repoRoot, "checkResult", result);
  if (violations.length) throw new Error(`acceptance result ${result.check} violates the acceptance schema:\n${violations.join("\n")}`);
  console.log(`[acceptance] ${result.check} ${result.status.toUpperCase()} — ${result.summary.en}`);
  console.log(`[acceptance] ${result.check} ${result.status.toUpperCase()} — ${result.summary.de}`);
  const path = process.env[ACCEPTANCE_RESULT_ENV];
  if (!path) return;
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(result, null, 2)}\n`);
}
/** 🧯️ Runs one harness body and guarantees its acceptance record: an error the body throws (an unreachable hub, a
 * missing binary, a refused sign-in) is published as a `fail` record — or `blocked` when `blockedWhen` classifies it as a
 * missing precondition — carrying the error in en + de, and the process exit code is set; nothing is swallowed silently.
 * The body publishes its own record when it completes. */
export async function withAcceptanceRecord(repoRoot: string, check: string, body: () => Promise<void>, blockedWhen: (error: unknown) => boolean = () => false): Promise<void> {
  const startedAt = new Date();
  try {
    await body();
  } catch (error) {
    const message = String(error instanceof Error ? error.message : error).split("\n")[0]!.slice(0, 600);
    const blocked = blockedWhen(error);
    publishAcceptanceCheckResult(
      repoRoot,
      acceptanceCheckResult({
        check,
        status: blocked ? "blocked" : "fail",
        startedAt,
        measured: { harnessError: true },
        summary: { en: `${blocked ? "precondition missing" : "harness stopped"}: ${message}`, de: `${blocked ? "Voraussetzung fehlt" : "Prüfprogramm abgebrochen"}: ${message}` },
      }),
    );
    process.exitCode = 1;
  }
}

/** 🪵️ Runs one law process (typically `cargo test` of one exact law), streams its output through and collects it as whole
 * lines: each stream is decoded as UTF-8 across chunk boundaries and keeps its own unfinished tail, so a line split between
 * chunks or interleaved with the other stream is still one line when a harness reads its verdict. Ctrl-C calls
 * `onInterrupt` and stops the process. Resolves with the exit status (-1 when killed by a signal) and the lines. */
export function runLawProcess(command: string, args: readonly string[], options: Readonly<{ cwd: string; env: NodeJS.ProcessEnv }>, onInterrupt?: () => void): Promise<{ status: number; lines: string[] }> {
  return new Promise((resolveExit) => {
    const lines: string[] = [];
    const tails = { stdout: "", stderr: "" };
    const child = spawn(command, [...args], { cwd: options.cwd, env: options.env, stdio: ["ignore", "pipe", "pipe"] });
    const interrupt = (): void => {
      onInterrupt?.();
      child.kill("SIGINT");
    };
    process.once("SIGINT", interrupt);
    for (const stream of ["stdout", "stderr"] as const) {
      child[stream].setEncoding("utf8");
      child[stream].on("data", (text: string) => {
        process.stdout.write(text);
        const parts = (tails[stream] + text).split("\n");
        tails[stream] = parts.pop()!;
        lines.push(...parts);
      });
    }
    child.once("close", (code) => {
      process.removeListener("SIGINT", interrupt);
      lines.push(...Object.values(tails).filter((tail) => tail.length > 0));
      resolveExit({ status: code ?? -1, lines });
    });
  });
}

//#endregion 🧾️CheckResult

//#region 🎯️GoalGate
/** 🎛️ One goal-gate invocation. */
export type GoalGateOptions = Readonly<{
  hub: string | null;
  serve: string | null;
  localServe: string | null;
  hubBinary: string | null;
  hubAdminCapability: string | null;
  users: readonly Readonly<{ email: string; password: string }>[];
  planPath: string;
  outDir: string;
  only: readonly string[];
  includeOptional: boolean;
  maxBrowsers: number;
  maxParallel: number;
  signal: AbortSignal;
  execute?: GoalGateExecutor;
  provide?: GoalGateProvisioner;
}>;

/** 🏗️ Stands up one provider (the default spawns `bun nx run <project>:<target> -- <args>` in its own process group and
 * waits for its url), reporting progress lines; resolves once the url answers, rejects on the bound, an early exit with
 * nothing answering, or cancellation, and hands back `stop()` that ends exactly what it started. */
export type GoalGateProvisioner = (repoRoot: string, requirement: ProvidedRequirement, provider: GoalPlanProvider, args: readonly string[], env: NodeJS.ProcessEnv, logPath: string, signal: AbortSignal, onProgress: (line: string) => void) => Promise<Readonly<{ stop: () => Promise<void> }>>;

/** 🚀️ Runs one check's command with its environment and log file and resolves its exit code. The default spawns
 * `bun nx run <project>:<target> -- <args>` in its own process group with `NX_DAEMON=false` (every nx invocation of the
 * gate computes its own project graph, so a wedged or foreign daemon cannot stall a check); laws substitute a scripted
 * executor. */
export type GoalGateExecutor = (repoRoot: string, check: GoalPlanCheck, args: readonly string[], env: NodeJS.ProcessEnv, logPath: string, children: Set<ChildProcess>) => Promise<number>;

const spawnGoalGateCheck: GoalGateExecutor = (repoRoot, check, args, env, logPath, children) => {
  const log = createWriteStream(logPath);
  const started = Date.now();
  return new Promise<number>((resolveExit) => {
    const child = spawn("bun", ["nx", "run", `${check.project}:${check.target}`, ...(args.length ? ["--", ...args] : [])], { cwd: repoRoot, env, stdio: ["ignore", "pipe", "pipe"], detached: process.platform !== "win32" });
    children.add(child);
    child.stdout?.pipe(log, { end: false });
    child.stderr?.pipe(log, { end: false });
    const ticker = setInterval(() => console.log(`[goal-gate] … ${check.id} running ${Math.round((Date.now() - started) / 1000)} s`), 60_000);
    child.once("error", () => resolveExit(-1));
    child.once("close", (code) => {
      clearInterval(ticker);
      children.delete(child);
      log.end();
      resolveExit(code ?? -1);
    });
  });
};

async function providerAnswers(url: string): Promise<boolean> {
  try {
    const response = await fetch(url, { signal: AbortSignal.timeout(5_000) });
    await response.body?.cancel();
    return response.ok;
  } catch {
    return false;
  }
}

function terminateGroup(child: ChildProcess): void {
  if (child.pid === undefined || child.exitCode !== null || child.signalCode !== null) return;
  try {
    if (process.platform === "win32") child.kill("SIGTERM");
    else process.kill(-child.pid, "SIGTERM");
  } catch {
    child.kill("SIGTERM");
  }
}

const provisionGoalGateRequirement: GoalGateProvisioner = async (repoRoot, requirement, provider, args, env, logPath, signal, onProgress) => {
  const probe = `${provider.url.replace(/\/+$/u, "")}${provider.readyPath}`;
  if (await providerAnswers(probe)) {
    onProgress(`${requirement} already answers at ${provider.url} — reused, never stopped`);
    return { stop: async () => {} };
  }
  const log = createWriteStream(logPath);
  const child = spawn("bun", ["nx", "run", `${provider.project}:${provider.target}`, ...(args.length ? ["--", ...args] : [])], { cwd: repoRoot, env, stdio: ["ignore", "pipe", "pipe"], detached: process.platform !== "win32" });
  child.stdout?.pipe(log, { end: false });
  child.stderr?.pipe(log, { end: false });
  const exited = new Promise<void>((resolveExit) => child.once("close", () => resolveExit()));
  const stop = async (): Promise<void> => {
    terminateGroup(child);
    await Promise.race([exited, new Promise((resolveDelay) => setTimeout(resolveDelay, 30_000))]);
    log.end();
  };
  const started = Date.now();
  let reported = started;
  for (;;) {
    if (signal.aborted) {
      await stop();
      throw new Error(`${requirement} provisioning cancelled`);
    }
    if (await providerAnswers(probe)) {
      onProgress(`${requirement} ready at ${provider.url} after ${Math.round((Date.now() - started) / 1000)} s (${logPath})`);
      return { stop };
    }
    if (child.exitCode !== null || child.signalCode !== null) {
      log.end();
      throw new Error(`${provider.project}:${provider.target} exited ${child.exitCode ?? child.signalCode} before ${probe} answered — see ${logPath}`);
    }
    if (Date.now() - started >= provider.readyBoundMs) {
      await stop();
      throw new Error(`${probe} did not answer within ${Math.round(provider.readyBoundMs / 1000)} s — see ${logPath}`);
    }
    if (Date.now() - reported >= 30_000) {
      reported = Date.now();
      onProgress(`${requirement} waiting for ${probe} (${Math.round((reported - started) / 1000)} of ${Math.round(provider.readyBoundMs / 1000)} s)`);
    }
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 2_000));
  }
};

const STATUS_WORD: Readonly<Record<AcceptanceStatus, LocalizedText>> = {
  pass: { en: "pass", de: "bestanden" },
  fail: { en: "fail", de: "fehlgeschlagen" },
  blocked: { en: "blocked", de: "blockiert" },
  skipped: { en: "skipped", de: "übersprungen" },
};

/** 🔣️ Substitutes the gate's tokens: `{hub}`, `{serve}` (a serve joined to that hub), `{localServe}` (a local-only serve
 * where every plugin loads — the program matrix and idle census enumerate its catalog), `{hubBinary}`, `{hubAdminCapability}` (the hub launcher's `0600`
 * admin-capability file, a path) and `{user<N>Email}` / `{user<N>Password}`
 * (1-based, from `--users`). Credentials are only ever substituted into ENVIRONMENT values, never into arguments, so no
 * secret reaches a log line. */
function substitute(value: string, options: GoalGateOptions): string {
  return value
    .replaceAll("{runDir}", options.outDir)
    .replaceAll("{hub}", options.hub ?? "")
    .replaceAll("{serve}", options.serve ?? "")
    .replaceAll("{localServe}", options.localServe ?? "")
    .replaceAll("{hubBinary}", options.hubBinary ?? "")
    .replaceAll("{hubAdminCapability}", options.hubAdminCapability ?? "")
    .replace(/\{user(\d)(Email|Password)\}/gu, (_, index: string, field: string) => options.users[Number(index) - 1]?.[field === "Email" ? "email" : "password"] ?? "");
}

function syntheticResult(check: GoalPlanCheck, status: AcceptanceStatus, startedAt: Date, measured: Readonly<Record<string, number | string | boolean>>, summary: LocalizedText, evidence: readonly string[]): AcceptanceCheckResult {
  return acceptanceCheckResult({ check: check.id, status, startedAt, measured, summary, evidence });
}

/** 🐳️ The pre-step that stands up the shared postgres + neo4j servers once for every check requiring `backends`. */
const GOAL_GATE_BACKENDS_CHECK: GoalPlanCheck = { id: "backends-up", title: { en: "Shared postgres and neo4j servers", de: "Gemeinsame postgres- und neo4j-Server" }, criteria: ["2.4"], project: "os-hub-ts", target: "backend-up", args: ["all"], requires: [], browsers: 0 };

/** 🧯️ Why a provided requirement is absent: the provider's failure, in the gate's log language. */
type ProvisionFailures = ReadonlyMap<ProvidedRequirement | "hubAdmin", string>;

function missingRequirement(check: GoalPlanCheck, options: GoalGateOptions, backendsReady: boolean | null): AcceptanceRequirement | undefined {
  return check.requires.find((requirement) => (requirement === "hub" && !options.hub) || (requirement === "serve" && !options.serve) || (requirement === "localServe" && !options.localServe) || (requirement === "hubAdmin" && !options.hubAdminCapability) || (requirement === "backends" && backendsReady === false));
}

async function runCheck(repoRoot: string, check: GoalPlanCheck, options: GoalGateOptions, children: Set<ChildProcess>, backendsReady: boolean | null, provisionFailures: ProvisionFailures = new Map()): Promise<AcceptanceCheckResult> {
  const startedAt = new Date();
  const missing = missingRequirement(check, options, backendsReady);
  if (missing === "backends") return syntheticResult(check, "blocked", startedAt, { missing }, { en: "the shared postgres/neo4j servers did not start (os-hub-ts:backend-up)", de: "die gemeinsamen postgres/neo4j-Server starteten nicht (os-hub-ts:backend-up)" }, []);
  const provisionFailure = missing === "hub" || missing === "serve" || missing === "localServe" || missing === "hubAdmin" ? provisionFailures.get(missing) : undefined;
  if (provisionFailure && missing) return syntheticResult(check, "blocked", startedAt, { missing }, { en: `the gate could not stand up ${missing}: ${provisionFailure}`.slice(0, 1800), de: `das Tor konnte ${missing} nicht bereitstellen: ${provisionFailure}`.slice(0, 1800) }, []);
  if (missing) return syntheticResult(check, "blocked", startedAt, { missing }, { en: `needs --${missing} <url>`, de: `benötigt --${missing} <url>` }, []);
  if (options.signal.aborted) return syntheticResult(check, "skipped", startedAt, {}, { en: "cancelled before start", de: "vor dem Start abgebrochen" }, []);
  const resultPath = join(options.outDir, `${check.id}.json`);
  const logPath = join(options.outDir, `${check.id}.log`);
  const args = check.args.map((arg) => substitute(arg, options));
  const planned = Object.entries(check.env ?? {}).map(([key, value]) => [key, substitute(value, options)] as const).filter(([, value]) => value.length > 0);
  const env = { ...process.env, ...Object.fromEntries(planned), [ACCEPTANCE_RESULT_ENV]: resultPath, NX_TUI: "false", NX_DAEMON: "false" };
  console.log(`[goal-gate] ▶ ${check.id}: bun nx run ${check.project}:${check.target}${args.length ? ` -- ${args.join(" ")}` : ""}`);
  const exitCode = await (options.execute ?? spawnGoalGateCheck)(repoRoot, check, args, env, logPath, children);
  if (options.signal.aborted) return syntheticResult(check, "skipped", startedAt, { exitCode }, { en: "cancelled while running", de: "während der Ausführung abgebrochen" }, [logPath]);
  if (existsSync(resultPath)) {
    const written = JSON.parse(readFileSync(resultPath, "utf8")) as AcceptanceCheckResult;
    const record: AcceptanceCheckResult = { ...written, check: check.id };
    const violations = acceptanceSchemaViolations(repoRoot, "checkResult", record);
    if (violations.length) return syntheticResult(check, "fail", startedAt, { exitCode, schemaViolations: violations.length }, { en: `harness wrote an invalid record: ${violations[0]}`, de: `Prüfprogramm schrieb einen ungültigen Datensatz: ${violations[0]}` }, [resultPath, logPath]);
    if (record.status === "pass" && exitCode !== 0) return { ...record, status: "fail", measured: { ...record.measured, exitCode }, evidence: [...record.evidence, logPath] };
    return { ...record, evidence: [...record.evidence, logPath] };
  }
  const status: AcceptanceStatus = exitCode === 0 ? "pass" : "fail";
  return syntheticResult(check, status, startedAt, { exitCode }, exitCode === 0 ? { en: `${check.project}:${check.target} exited 0`, de: `${check.project}:${check.target} endete mit 0` } : { en: `${check.project}:${check.target} exited ${exitCode}`, de: `${check.project}:${check.target} endete mit ${exitCode}` }, [logPath]);
}

async function runParallel(repoRoot: string, checks: readonly GoalPlanCheck[], options: GoalGateOptions, children: Set<ChildProcess>, backendsReady: boolean | null, provisionFailures: ProvisionFailures): Promise<AcceptanceCheckResult[]> {
  const results = new Map<string, AcceptanceCheckResult>();
  const pending = [...checks];
  const running = new Map<string, Promise<void>>();
  let browsers = 0;
  while (pending.length || running.size) {
    const index = pending.findIndex((check) => running.size < options.maxParallel && (browsers + check.browsers <= options.maxBrowsers || (running.size === 0 && check.browsers > options.maxBrowsers)));
    if (index >= 0) {
      const check = pending.splice(index, 1)[0]!;
      browsers += check.browsers;
      running.set(check.id, runCheck(repoRoot, check, options, children, backendsReady, provisionFailures).then((result) => {
        results.set(check.id, result);
        browsers -= check.browsers;
        running.delete(check.id);
      }));
      continue;
    }
    await Promise.race(running.values());
  }
  return checks.map((check) => results.get(check.id)!);
}

function stepStatus(results: readonly AcceptanceCheckResult[]): AcceptanceStatus {
  if (results.some((result) => result.status === "fail")) return "fail";
  if (results.some((result) => result.status === "blocked")) return "blocked";
  if (results.every((result) => result.status === "skipped")) return "skipped";
  return results.some((result) => result.status === "skipped") ? "blocked" : "pass";
}

function emptyTotals(): Record<AcceptanceStatus, number> {
  return { pass: 0, fail: 0, blocked: 0, skipped: 0 };
}

/** 🏁️ The verdict of every declared outcome over the checks that serve it (a check serves outcome `N` through any
 * criterion `N.x`): fail when one of them failed, blocked when one was blocked or skipped, pass when all passed, and
 * skipped when the run selected none of them. */
export function goalOutcomes(plan: GoalPlan, steps: GoalSummary["steps"]): GoalSummaryOutcome[] {
  const criteria = new Map(plan.steps.flatMap((step) => step.checks).map((check) => [check.id, check.criteria] as const));
  return plan.outcomes.map((outcome) => {
    const results = steps.flatMap((step) => step.checks).filter((result) => (criteria.get(result.check) ?? []).some((criterion) => criterion.split(".")[0] === outcome.id));
    const totals = emptyTotals();
    for (const result of results) totals[result.status] += 1;
    const status: AcceptanceStatus = results.length === 0 ? "skipped" : totals.fail > 0 ? "fail" : totals.blocked + totals.skipped > 0 ? "blocked" : "pass";
    return { id: outcome.id, status, totals, checks: results.map((result) => result.check) };
  });
}

/** 🏁️ The per-outcome verdict table of one run in one language — the answer to "is the goal met, and where not". */
export function goalOutcomeTable(summary: GoalSummary, plan: GoalPlan, language: keyof LocalizedText): string[] {
  const heading = language === "en" ? ["Outcome", "Verdict", "Pass", "Fail", "Blocked", "Skipped"] : ["Ergebnisziel", "Urteil", "Bestanden", "Fehlgeschlagen", "Blockiert", "Übersprungen"];
  return [`| ${heading.join(" | ")} |`, `|${heading.map(() => "---").join("|")}|`, ...summary.outcomes.map((outcome) => `| ${outcome.id}. ${plan.outcomes.find((candidate) => candidate.id === outcome.id)?.title[language] ?? outcome.id} | **${STATUS_WORD[outcome.status][language]}** | ${outcome.totals.pass} | ${outcome.totals.fail} | ${outcome.totals.blocked} | ${outcome.totals.skipped} |`)];
}

/** 📝️ The human summary of one run in one language: the verdict, the per-outcome verdict table, the totals and one row
 * per check. */
export function goalSummaryMarkdown(summary: GoalSummary, plan: GoalPlan, language: keyof LocalizedText): string {
  const heading = language === "en" ? ["Step", "Check", "Status", "Seconds", "Summary"] : ["Schritt", "Prüfung", "Status", "Sekunden", "Zusammenfassung"];
  const lines = [`# ${plan.title[language]}`, "", `${language === "en" ? "Verdict" : "Ergebnis"}: **${STATUS_WORD[summary.verdict][language]}** — ${Object.entries(summary.totals).map(([status, count]) => `${STATUS_WORD[status as AcceptanceStatus][language]} ${count}`).join(", ")}`, "", ...goalOutcomeTable(summary, plan, language), "", `Hub: ${summary.hub ?? "—"} · ${language === "en" ? "Serve" : "Oberfläche"}: ${summary.serve ?? "—"} · ${language === "en" ? "Local-only serve" : "Lokale Oberfläche"}: ${summary.localServe ?? "—"} · ${summary.startedAt} → ${summary.finishedAt}`, "", `| ${heading.join(" | ")} |`, `|${heading.map(() => "---").join("|")}|`];
  for (const step of summary.steps) {
    const planned = plan.steps.find((candidate) => candidate.id === step.id);
    for (const result of step.checks) {
      const title = planned?.checks.find((check) => check.id === result.check)?.title[language] ?? result.check;
      lines.push(`| ${planned?.title[language] ?? step.id} | ${title} | ${STATUS_WORD[result.status][language]} | ${Math.round(result.durationMs / 1000)} | ${result.summary[language].replaceAll("|", "\\|")} |`);
    }
  }
  return `${lines.join("\n")}\n`;
}

const PROVIDED_REQUIREMENTS: readonly ProvidedRequirement[] = ["hub", "serve", "localServe"];
const ADMIN_CAPABILITY_BOUND_MS = 60_000;

/** 🏗️ Stands up, in dependency order, every requirement the selected checks need that the command line did not name and
 * the plan provides; returns the resolved options, the holders to stop at the end and why any provider failed. */
async function provisionRequirements(repoRoot: string, plan: GoalPlan, selected: readonly GoalPlanStep[], options: GoalGateOptions): Promise<{ resolved: GoalGateOptions; holders: Readonly<{ stop: () => Promise<void> }>[]; failures: Map<ProvidedRequirement | "hubAdmin", string> }> {
  const needed = new Set(selected.flatMap((step) => step.checks.flatMap((check) => check.requires)));
  const values: Record<ProvidedRequirement, string | null> = { hub: options.hub, serve: options.serve, localServe: options.localServe };
  if (needed.has("serve") && plan.providers?.serve?.requires.includes("hub")) needed.add("hub");
  if (needed.has("hubAdmin") && !options.hubAdminCapability && plan.providers?.hub?.adminCapability) needed.add("hub");
  const holders: Readonly<{ stop: () => Promise<void> }>[] = [];
  const failures = new Map<ProvidedRequirement | "hubAdmin", string>();
  for (const requirement of PROVIDED_REQUIREMENTS) {
    const provider = plan.providers?.[requirement];
    if (!needed.has(requirement) || values[requirement] || !provider) continue;
    if (options.signal.aborted) break;
    const missing = provider.requires.find((dependency) => !values[dependency]);
    if (missing) {
      failures.set(requirement, `its provider needs ${missing}, which is absent`);
      continue;
    }
    const current: GoalGateOptions = { ...options, ...values };
    const args = provider.args.map((arg) => substitute(arg, current));
    const env = { ...process.env, ...Object.fromEntries(Object.entries(provider.env ?? {}).map(([key, value]) => [key, substitute(value, current)])), NX_TUI: "false", NX_DAEMON: "false" };
    const logPath = join(options.outDir, `provider-${requirement}.log`);
    console.log(`[goal-gate] ▶ provider ${requirement}: bun nx run ${provider.project}:${provider.target}${args.length ? ` -- ${args.join(" ")}` : ""} → ${provider.url}`);
    try {
      holders.push(await (options.provide ?? provisionGoalGateRequirement)(repoRoot, requirement, provider, args, env, logPath, options.signal, (line) => console.log(`[goal-gate] … ${line}`)));
      values[requirement] = provider.url;
    } catch (error) {
      const reason = String(error instanceof Error ? error.message : error).split("\n")[0]!.slice(0, 600);
      failures.set(requirement, reason);
      console.log(`[goal-gate] provider ${requirement} FAILED: ${reason}`);
    }
  }
  let hubAdminCapability = options.hubAdminCapability;
  const hubProvider = plan.providers?.hub;
  if (needed.has("hubAdmin") && !hubAdminCapability && values.hub) {
    if (hubProvider?.adminCapability && values.hub === hubProvider.url) {
      const path = isAbsolute(hubProvider.adminCapability) ? hubProvider.adminCapability : join(repoRoot, hubProvider.adminCapability);
      const deadline = Date.now() + Math.min(hubProvider.readyBoundMs, ADMIN_CAPABILITY_BOUND_MS);
      while (!existsSync(path) && Date.now() < deadline && !options.signal.aborted) await new Promise((resolveDelay) => setTimeout(resolveDelay, 250));
      if (existsSync(path)) {
        hubAdminCapability = path;
        console.log(`[goal-gate] hubAdmin: the hub provider's admin capability ${path}`);
      } else failures.set("hubAdmin", `the hub provider published no admin capability at ${path}`);
    } else failures.set("hubAdmin", "a hub the gate did not stand up needs --hub-admin-capability <file>");
  }
  return { resolved: { ...options, ...values, hubAdminCapability }, holders, failures };
}

/** 🎯️ Runs the goal plan's steps in order — each step's checks serially or in parallel within the browser and process
 * budget — after standing up (zero-touch) every hub/serve the selected checks need and the command line did not name,
 * writes every check's record, `summary.json` (`semio.acceptance.goal-summary/v1`) and `summary.en.md` / `summary.de.md`
 * (per-outcome verdict table first) into `outDir`, stops what it stood up, and returns the summary. A failed gating step
 * blocks every later step; cancellation terminates the running checks' process groups and records the rest as skipped. */
export async function runGoalGate(repoRoot: string, requested: GoalGateOptions): Promise<GoalSummary> {
  const plan = readGoalPlan(repoRoot, requested.planPath);
  mkdirSync(requested.outDir, { recursive: true });
  const children = new Set<ChildProcess>();
  const terminate = (): void => {
    for (const child of children) terminateGroup(child);
  };
  requested.signal.addEventListener("abort", terminate, { once: true });
  const startedAt = new Date().toISOString();
  const steps: { id: string; status: AcceptanceStatus; checks: AcceptanceCheckResult[] }[] = [];
  let gatedBy: string | null = null;
  const selected = plan.steps.filter((step) => (requested.only.length ? requested.only.includes(step.id) : !step.optional || requested.includeOptional));
  const provisioned = await provisionRequirements(repoRoot, plan, selected, requested);
  const options = provisioned.resolved;
  try {
    let backendsReady: boolean | null = null;
    if (selected.some((step) => step.checks.some((check) => check.requires.includes("backends")))) {
      const backendsLog = join(options.outDir, "backends-up.log");
      console.log("[goal-gate] ▶ backends: bun nx run os-hub-ts:backend-up -- all (the shared postgres + neo4j servers, zero-touch)");
      backendsReady = (await (options.execute ?? spawnGoalGateCheck)(repoRoot, GOAL_GATE_BACKENDS_CHECK, ["all"], { ...process.env, NX_TUI: "false", NX_DAEMON: "false" }, backendsLog, children)) === 0;
      console.log(`[goal-gate] backends ${backendsReady ? "ready" : "NOT ready"} (${backendsLog})`);
    }
    for (const [index, step] of selected.entries()) {
      console.log(`[goal-gate] step ${index + 1}/${selected.length} ${step.id} (${step.mode}, ${step.checks.length} checks): ${step.title.en}`);
      let checks: AcceptanceCheckResult[];
      if (gatedBy !== null) {
        const now = new Date();
        const gate = gatedBy;
        checks = step.checks.map((check) => syntheticResult(check, "blocked", now, { gatedBy: gate }, { en: `blocked: gating step ${gate} did not pass`, de: `blockiert: der vorausgehende Schritt ${gate} ist nicht bestanden` }, []));
      } else if (step.mode === "parallel") {
        checks = await runParallel(repoRoot, step.checks, options, children, backendsReady, provisioned.failures);
      } else {
        checks = [];
        for (const check of step.checks) checks.push(await runCheck(repoRoot, check, options, children, backendsReady, provisioned.failures));
      }
      const status = stepStatus(checks);
      steps.push({ id: step.id, status, checks });
      for (const check of checks) console.log(`[goal-gate] ${check.status.toUpperCase().padEnd(7)} ${check.check} (${Math.round(check.durationMs / 1000)} s) — ${check.summary.en}`);
      if (step.gatesRest && status !== "pass" && gatedBy === null) gatedBy = step.id;
    }
  } finally {
    requested.signal.removeEventListener("abort", terminate);
    for (const holder of provisioned.holders.reverse()) await holder.stop();
  }
  const totals = emptyTotals();
  for (const step of steps) for (const check of step.checks) totals[check.status] += 1;
  const verdict: AcceptanceStatus = totals.fail > 0 ? "fail" : totals.blocked > 0 || totals.skipped > 0 ? "blocked" : "pass";
  const summary: GoalSummary = { schema: "semio.acceptance.goal-summary/v1", plan: plan.id, hub: options.hub, serve: options.serve, localServe: options.localServe, startedAt, finishedAt: new Date().toISOString(), verdict, totals, outcomes: goalOutcomes(plan, steps), steps };
  const violations = acceptanceSchemaViolations(repoRoot, "goalSummary", summary);
  if (violations.length) throw new Error(`goal summary violates the acceptance schema:\n${violations.join("\n")}`);
  writeFileSync(join(options.outDir, "summary.json"), `${JSON.stringify(summary, null, 2)}\n`);
  for (const language of ["en", "de"] as const) writeFileSync(join(options.outDir, `summary.${language}.md`), goalSummaryMarkdown(summary, plan, language));
  return summary;
}

/** 🔑️ The test principals of the hub under verification, read from a JSON file `{"users":[{"email","password"}…]}`
 * (for example a slice's hub state directory), so no credential is ever typed on a command line. No file (an empty
 * `--users`) leaves every check on its own documented default principals. */
function readGoalGateUsers(path: string | undefined): readonly Readonly<{ email: string; password: string }>[] {
  if (!path) return [];
  const parsed = JSON.parse(readFileSync(resolve(path), "utf8")) as { users?: unknown };
  if (!Array.isArray(parsed.users) || !parsed.users.every((user) => typeof user?.email === "string" && typeof user?.password === "string")) throw new Error(`--users ${path} must hold {"users":[{"email","password"}…]}`);
  return parsed.users as { email: string; password: string }[];
}

function option(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

/** 🎯️ `acceptance goal [--hub <url>] [--serve <url>] [--local-serve <url>] [--hub-binary <path>] [--hub-admin-capability <file>] [--users <json>] [--plan <path>] [--only <step,…>]
 * [--out <dir>] [--include-optional] [--max-browsers <n>] [--max-parallel <n>]` and `acceptance plan` (prints the plan).
 * A hub or serve the command line does not name is stood up by the plan's provider and stopped after the run. */
export class AcceptanceScript extends Script {
  async run(segments: string[]): Promise<void> {
    const [verb, ...rest] = segments;
    const planPath = resolve(option(rest, "--plan") ?? join(this.repoRoot, GOAL_PLAN_REL_PATH));
    if (verb === "plan") {
      const plan = readGoalPlan(this.repoRoot, planPath);
      for (const step of plan.steps) {
        console.log(`${step.id} (${step.mode}${step.gatesRest ? ", gates the rest" : ""}${step.optional ? ", optional" : ""}) — ${step.title.en} / ${step.title.de}`);
        for (const check of step.checks) console.log(`  ${check.id}: bun nx run ${check.project}:${check.target}${check.args.length ? ` -- ${check.args.join(" ")}` : ""} [criteria ${check.criteria.join(",")}; needs ${check.requires.join(",") || "nothing"}; browsers ${check.browsers}]`);
      }
      return;
    }
    if (verb !== "goal") throw new Error("usage: acceptance <goal|plan> [--hub <url>] [--serve <url>] [--local-serve <url>] [--hub-binary <path>] [--hub-admin-capability <file>] [--users <json>] [--plan <path>] [--only <step,…>] [--out <dir>] [--include-optional] [--max-browsers <n>] [--max-parallel <n>]");
    const controller = new AbortController();
    const cancel = (): void => controller.abort();
    process.once("SIGINT", cancel);
    process.once("SIGTERM", cancel);
    try {
      const outDir = resolve(option(rest, "--out") ?? join(this.repoRoot, GOAL_GATE_OUT_REL_PATH, new Date().toISOString().replaceAll(":", "-").replace(/\.\d+Z$/u, "Z")));
      const summary = await runGoalGate(this.repoRoot, {
        hub: option(rest, "--hub") ?? null,
        serve: option(rest, "--serve") ?? null,
        localServe: option(rest, "--local-serve") ?? null,
        hubBinary: option(rest, "--hub-binary") ?? null,
        hubAdminCapability: option(rest, "--hub-admin-capability") ?? null,
        users: readGoalGateUsers(option(rest, "--users")),
        planPath,
        outDir,
        only: (option(rest, "--only") ?? "").split(",").filter(Boolean),
        includeOptional: rest.includes("--include-optional"),
        maxBrowsers: Number(option(rest, "--max-browsers") ?? 1),
        maxParallel: Number(option(rest, "--max-parallel") ?? 4),
        signal: controller.signal,
      });
      const plan = readGoalPlan(this.repoRoot, planPath);
      console.log(goalSummaryMarkdown(summary, plan, "en"));
      console.log(goalSummaryMarkdown(summary, plan, "de"));
      for (const outcome of summary.outcomes) console.log(`[goal-gate] outcome ${outcome.id} ${outcome.status.toUpperCase()} (pass ${outcome.totals.pass}, fail ${outcome.totals.fail}, blocked ${outcome.totals.blocked}, skipped ${outcome.totals.skipped}) — ${plan.outcomes.find((candidate) => candidate.id === outcome.id)?.title.en ?? outcome.id}`);
      console.log(`[goal-gate] records: ${outDir}`);
      if (summary.verdict !== "pass") process.exitCode = 1;
    } finally {
      process.removeListener("SIGINT", cancel);
      process.removeListener("SIGTERM", cancel);
    }
  }
}
//#endregion 🎯️GoalGate

//#region 🧮️SourceCensus
/** 🗂️ One tracked source file: repo-relative path and whether it is test-only by its path (a `🧪️tests`, `tests`,
 * `🧫️fixtures`, `benches` or `examples` segment). */
export type CensusSource = Readonly<{ path: string; testOnly: boolean }>;

const TEST_SEGMENT = /(^|\/)(🧪️tests|tests|🧫️fixtures|benches|examples)\//u;

/** 🗂️ Every git-tracked `*.rs` outside the ticket tree, classified. */
export function trackedRustSources(repoRoot: string): CensusSource[] {
  const listed = spawnSync("git", ["ls-files", "-z", "--", "*.rs", ":!.🧬semio"], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 });
  if (listed.status !== 0) throw new Error(`git ls-files failed: ${listed.stderr}`);
  return listed.stdout.split("\0").filter(Boolean).map((path) => ({ path, testOnly: TEST_SEGMENT.test(path) }));
}

/** ✂️ The code of one Rust line with line comments and the parts of block comments removed; `inBlock` carries a block
 * comment across lines. String literals are honoured, so a `//` inside a string is code. */
export function rustCodeOfLine(line: string, inBlock: { open: boolean }): string {
  let code = "";
  let inString = false;
  for (let index = 0; index < line.length; index += 1) {
    const char = line[index]!;
    const next = line[index + 1];
    if (inBlock.open) {
      if (char === "*" && next === "/") {
        inBlock.open = false;
        index += 1;
      }
      continue;
    }
    if (inString) {
      code += char;
      if (char === "\\") {
        code += next ?? "";
        index += 1;
      } else if (char === '"') inString = false;
      continue;
    }
    if (char === "/" && next === "/") break;
    if (char === "/" && next === "*") {
      inBlock.open = true;
      index += 1;
      continue;
    }
    if (char === '"') inString = true;
    code += char;
  }
  return code;
}

/** 🚧️ One `unimplemented!(` / `todo!(` placeholder. */
export type PlaceholderHit = Readonly<{ path: string; line: number; macro: "unimplemented" | "todo"; testOnly: boolean; commented: boolean }>;

const PLACEHOLDER = /(unimplemented|todo)!\s*\(/gu;

/** 🚧️ Scans Rust text for placeholder macros, separating code from commented occurrences. */
export function placeholderHitsOfText(path: string, text: string, testOnly: boolean): PlaceholderHit[] {
  const hits: PlaceholderHit[] = [];
  const inBlock = { open: false };
  text.split("\n").forEach((line, index) => {
    const code = rustCodeOfLine(line, inBlock);
    for (const match of line.matchAll(PLACEHOLDER)) hits.push({ path, line: index + 1, macro: match[1] as "unimplemented" | "todo", testOnly, commented: !new RegExp(PLACEHOLDER.source, "u").test(code) });
  });
  return hits;
}

/** 🎛️ One `.action_interactive_job(<"command" | CONST_PATH>, [path::]InteractiveJobClassification::<class>)` declaration. */
export type InteractiveJobDeclaration = Readonly<{ path: string; line: number; command: string; classification: string; testOnly: boolean }>;

const INTERACTIVE_JOB = /\.action_interactive_job\(\s*(?:"([^"]+)"|\*?([A-Za-z_][\w:]*))\s*,\s*(?:[A-Za-z_][\w]*::)*InteractiveJobClassification::(\w+)/gu;

/** 🎛️ Every declared interactive job of Rust text, and how many `.action_interactive_job(` calls it holds. */
export function interactiveJobsOfText(path: string, text: string, testOnly: boolean): { declarations: InteractiveJobDeclaration[]; calls: number; codeCalls: number } {
  const inBlock = { open: false };
  const code = text.split("\n").map((line) => rustCodeOfLine(line, inBlock)).join("\n");
  const declarations = [...code.matchAll(INTERACTIVE_JOB)].map((match) => ({ path, line: code.slice(0, match.index).split("\n").length, command: match[1] ?? match[2]!, classification: match[3]!, testOnly }));
  return { declarations, calls: text.split(".action_interactive_job(").length - 1, codeCalls: code.split(".action_interactive_job(").length - 1 };
}

/** 📊️ The two production censuses over every tracked Rust source, each cross-checked against `git grep` (the oracle):
 * every placeholder line git's own regex engine finds must be one the scanner found, and every file's count of
 * `action_interactive_job(` calls must equal the scanner's. Progress every 2000 files; the signal stops the walk. */
export function runSourceCensus(repoRoot: string, signal: AbortSignal, onProgress: (line: string) => void) {
  const sources = trackedRustSources(repoRoot);
  const placeholders: PlaceholderHit[] = [];
  const jobs: InteractiveJobDeclaration[] = [];
  const callsPerFile = new Map<string, number>();
  const codeCallsPerFile = new Map<string, { calls: number; testOnly: boolean }>();
  for (const [index, source] of sources.entries()) {
    if (signal.aborted) throw new Error("source census cancelled");
    if (index % 2000 === 0) onProgress(`${index}/${sources.length} Rust sources scanned`);
    let text: string;
    try {
      text = readFileSync(join(repoRoot, source.path), "utf8");
    } catch {
      continue;
    }
    if (text.includes("!")) placeholders.push(...placeholderHitsOfText(source.path, text, source.testOnly));
    if (text.includes(".action_interactive_job(")) {
      const found = interactiveJobsOfText(source.path, text, source.testOnly);
      jobs.push(...found.declarations);
      callsPerFile.set(source.path, found.calls);
      codeCallsPerFile.set(source.path, { calls: found.codeCalls, testOnly: source.testOnly });
    }
  }
  const grep = (args: string[]): string => spawnSync("git", ["grep", ...args, "--", "*.rs", ":!.🧬semio"], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 }).stdout;
  const oraclePlaceholders = new Set(grep(["-n", "-E", "(unimplemented|todo)![[:space:]]*\\("]).split("\n").filter(Boolean).map((row) => row.split(":").slice(0, 2).join(":")));
  const scannerPlaceholders = new Set(placeholders.map((hit) => `${hit.path}:${hit.line}`));
  const oracleCalls = new Map(grep(["-c", "-F", ".action_interactive_job("]).split("\n").filter(Boolean).map((row) => [row.slice(0, row.lastIndexOf(":")), Number(row.slice(row.lastIndexOf(":") + 1))] as const));
  const placeholderDisagreements = [...oraclePlaceholders].filter((key) => !scannerPlaceholders.has(key)).concat([...scannerPlaceholders].filter((key) => !oraclePlaceholders.has(key)));
  const callDisagreements = [...new Set([...oracleCalls.keys(), ...callsPerFile.keys()])].filter((path) => (oracleCalls.get(path) ?? 0) !== (callsPerFile.get(path) ?? 0));
  const unparsed = [...codeCallsPerFile].filter(([path, entry]) => !entry.testOnly && entry.calls !== jobs.filter((job) => job.path === path).length).map(([path, entry]) => `${path}: ${entry.calls} calls in code, ${jobs.filter((job) => job.path === path).length} classified declarations`);
  onProgress(`${sources.length}/${sources.length} Rust sources scanned`);
  return { files: sources.length, placeholders, jobs, placeholderDisagreements, callDisagreements, unparsed };
}

/** 🏷️ One docstring line that breaks AGENTS.md's emoji-first rule: the `@emoji` residue token in front of the emoji, or
 * an opener with no emoji at all (a symbol glyph such as `⊕` or `√` counts as the marker). */
export type DocstringHit = Readonly<{ path: string; line: number; rule: "at-emoji" | "no-emoji"; text: string }>;

const DOCSTRING_MARKER = /^(?:\p{Extended_Pictographic}|\p{So}|\p{Sm}|\p{Regional_Indicator}|[0-9#*]️?⃣)/u;
const AT_EMOJI_LINE = /(?:\/\/\/|\/\/!|\/\*\*)\s*@emoji|^\s*\*\s*@emoji/u;
const RUST_LINE_DOC = /^\/\/[/!](?!\/)/u;

/** 🧷️ Every docstring opener of one source, by 1-based line with its trimmed content: a Rust `///` or `//!` run's first non-empty
 * line (a run ends where its doc kind changes) and a `/** … *\/` block's first non-empty content line (Rust and TypeScript; `/**\/` and `/***` are no docstrings, and a `/**`
 * behind `//` or a quote is text), in line order. */
export function docstringOpenersOfText(path: string, text: string): { readonly line: number; readonly content: string }[] {
  const lines = text.split("\n");
  const openers: { readonly line: number; readonly content: string }[] = [];
  const opener = (index: number, content: string): void => {
    openers.push({ line: index + 1, content: content.trim() });
  };
  if (path.endsWith(".rs")) {
    let run: string | null = null;
    let opened = false;
    lines.forEach((raw, index) => {
      const line = raw.trimStart();
      if (!RUST_LINE_DOC.test(line)) {
        run = null;
        return;
      }
      if (run !== line.slice(0, 3)) opened = false;
      run = line.slice(0, 3);
      if (opened || line.slice(3).trim().length === 0) return;
      opened = true;
      opener(index, line.slice(3));
    });
  }
  let open = false;
  lines.forEach((raw, index) => {
    if (!open) {
      const start = raw.indexOf("/**");
      if (start < 0 || raw[start + 3] === "/" || raw[start + 3] === "*" || /\/\/|["'`]/u.test(raw.slice(0, start))) return;
      const rest = raw.slice(start + 3);
      const closeAt = rest.indexOf("*/");
      const content = closeAt >= 0 ? rest.slice(0, closeAt) : rest;
      if (content.trim().length > 0) {
        opener(index, content);
        return;
      }
      open = closeAt < 0;
      return;
    }
    const closeAt = raw.indexOf("*/");
    const content = (closeAt >= 0 ? raw.slice(0, closeAt) : raw).replace(/^\s*\*?/u, "");
    if (content.trim().length > 0) {
      open = false;
      opener(index, content);
      return;
    }
    if (closeAt >= 0) open = false;
  });
  return openers.sort((left, right) => left.line - right.line);
}

/** 🔖️ Every docstring finding of one source. `at-emoji`: each line where a doc opener (`///`, `//!`, `/**`) or a block
 * continuation (` * `) is followed by the `@emoji` token — wherever it stands, so an `@emoji` paragraph inside a run and a
 * generator's emitted doc line count too. `no-emoji`: each docstring opener ({@link docstringOpenersOfText}) that starts with
 * neither an emoji nor a symbol glyph. */
export function docstringHitsOfText(path: string, text: string): DocstringHit[] {
  const hits: DocstringHit[] = [];
  text.split("\n").forEach((raw, index) => {
    if (AT_EMOJI_LINE.test(raw)) hits.push({ path, line: index + 1, rule: "at-emoji", text: raw.trim().slice(0, 80) });
  });
  const flagged = new Set(hits.map((hit) => hit.line));
  for (const { line, content } of docstringOpenersOfText(path, text)) {
    if (!flagged.has(line) && !content.startsWith("@emoji") && !DOCSTRING_MARKER.test(content)) hits.push({ path, line, rule: "no-emoji", text: content.slice(0, 80) });
  }
  return hits.sort((left, right) => left.line - right.line);
}

/** 🪞️ One marker that opens more than one docstring of a source (AGENTS.md: every docstring starts with a unique emoji; design
 * §21.2 holds that per file): the marker's grapheme without its variation selector, and every opener line it starts. */
export type DocstringEmojiReuse = Readonly<{ path: string; emoji: string; lines: readonly number[] }>;

const DOCSTRING_GRAPHEMES = new Intl.Segmenter("en", { granularity: "grapheme" });

/** 🔁️ Every marker of one source that opens more than one docstring ({@link docstringOpenersOfText}; an opener without a marker or
 * behind the `@emoji` residue is the `docstrings` census's own finding), in the order of each marker's first opener. */
export function docstringEmojiReuseOfText(path: string, text: string): DocstringEmojiReuse[] {
  const openers = new Map<string, number[]>();
  for (const { line, content } of docstringOpenersOfText(path, text)) {
    if (!DOCSTRING_MARKER.test(content)) continue;
    const emoji = DOCSTRING_GRAPHEMES.segment(content)[Symbol.iterator]().next().value!.segment.replaceAll("\uFE0F", "");
    openers.set(emoji, [...(openers.get(emoji) ?? []), line]);
  }
  return [...openers].filter(([, lines]) => lines.length > 1).map(([emoji, lines]) => ({ path, emoji, lines }));
}

/** 📚️ Every tracked Rust / TypeScript source the docstring censuses read: outside the ticket tree, generated trees and declaration files. */
function docstringSources(repoRoot: string): string[] {
  const listed = spawnSync("git", ["ls-files", "-z", "--", "*.rs", "*.ts", "*.tsx", ":!.🧬semio", ":!*.d.ts", ":!**/🤖️generated/**"], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 });
  if (listed.status !== 0) throw new Error(`git ls-files failed: ${listed.stderr}`);
  return listed.stdout.split("\0").filter(Boolean);
}

/** 🗳️ The docstring census over every source {@link docstringSources} lists, with its `@emoji` findings cross-checked line for line
 * against `git grep` (the oracle). */
export function runDocstringCensus(repoRoot: string, signal: AbortSignal, onProgress: (line: string) => void) {
  const sources = docstringSources(repoRoot);
  const hits: DocstringHit[] = [];
  for (const [index, path] of sources.entries()) {
    if (signal.aborted) throw new Error("docstring census cancelled");
    if (index % 5000 === 0) onProgress(`${index}/${sources.length} sources scanned`);
    let text: string;
    try {
      text = readFileSync(join(repoRoot, path), "utf8");
    } catch {
      continue;
    }
    hits.push(...docstringHitsOfText(path, text));
  }
  const grep = (pattern: string): string[] => spawnSync("git", ["grep", "-n", "-E", pattern, "--", "*.rs", "*.ts", "*.tsx", ":!.🧬semio", ":!*.d.ts", ":!**/🤖️generated/**"], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 }).stdout.split("\n").filter(Boolean).map((row) => row.split(":").slice(0, 2).join(":"));
  const oracle = new Set([...grep("(///|//!|/\\*\\*)[[:space:]]*@emoji"), ...grep("^[[:space:]]*\\*[[:space:]]*@emoji")]);
  const scanned = new Set(hits.filter((hit) => hit.rule === "at-emoji").map((hit) => `${hit.path}:${hit.line}`));
  const disagreements = [...oracle].filter((key) => !scanned.has(key)).concat([...scanned].filter((key) => !oracle.has(key)));
  onProgress(`${sources.length}/${sources.length} sources scanned`);
  return { files: sources.length, hits, disagreements };
}

/** 🧮️ The per-file docstring-emoji census over `paths` (repository-relative Rust / TypeScript sources; every source
 * {@link docstringSources} lists when absent): each marker reused within one file, see {@link docstringEmojiReuseOfText}. */
export function runDocstringEmojiCensus(repoRoot: string, signal: AbortSignal, onProgress: (line: string) => void, paths?: readonly string[]) {
  const sources = (paths ?? docstringSources(repoRoot)).filter((path) => /\.(?:rs|tsx?)$/u.test(path) && !path.endsWith(".d.ts"));
  const reuse: DocstringEmojiReuse[] = [];
  for (const [index, path] of sources.entries()) {
    if (signal.aborted) throw new Error("docstring emoji census cancelled");
    if (index % 5000 === 0) onProgress(`${index}/${sources.length} sources scanned`);
    let text: string;
    try {
      text = readFileSync(join(repoRoot, path), "utf8");
    } catch {
      continue;
    }
    reuse.push(...docstringEmojiReuseOfText(path, text));
  }
  onProgress(`${sources.length}/${sources.length} sources scanned`);
  return { files: sources.length, reuse };
}
/** 🐞️ One line carrying the `[DEBUG]` tag, which AGENTS.md reserves for temporary logs removed before a change lands. */
export type DebugTagHit = Readonly<{ path: string; line: number; text: string }>;

/** 🐞️ Every line of one source that carries the `[DEBUG]` tag. */
export function debugTagHitsOfText(path: string, text: string): DebugTagHit[] {
  return text.split("\n").flatMap((line, index) => (line.includes("[DEBUG]") ? [{ path, line: index + 1, text: line.trim().slice(0, 120) }] : []));
}

/** 🐞️ The census's own rule vocabulary — this module and the source-census fixture name the tag they look for. */
export const DEBUG_TAG_CENSUS_EXEMPT: readonly string[] = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🧫️fixtures/🧮️source-census/🔣️.json"];

/** 🐞️ The `[DEBUG]` census over every tracked text source outside the ticket tree, Markdown prose and
 * {@link DEBUG_TAG_CENSUS_EXEMPT}, cross-checked file by file against `git grep -c` (the oracle). Progress every 5000
 * files; the signal stops the walk. */
export function runDebugTagCensus(repoRoot: string, signal: AbortSignal, onProgress: (line: string) => void) {
  const scope = ["--", ":!.🧬semio", ":!*.md", ":!.cursor", ...DEBUG_TAG_CENSUS_EXEMPT.map((path) => `:!${path}`)];
  const listed = spawnSync("git", ["grep", "-l", "-z", "-I", "-F", "[DEBUG]", ...scope], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 });
  const sources = listed.stdout.split("\0").filter(Boolean);
  const hits: DebugTagHit[] = [];
  for (const [index, path] of sources.entries()) {
    if (signal.aborted) throw new Error("debug-tag census cancelled");
    if (index % 5000 === 0) onProgress(`${index}/${sources.length} tagged sources scanned`);
    let text: string;
    try {
      text = readFileSync(join(repoRoot, path), "utf8");
    } catch {
      continue;
    }
    hits.push(...debugTagHitsOfText(path, text));
  }
  const oracle = new Map(spawnSync("git", ["grep", "-c", "-I", "-F", "[DEBUG]", ...scope], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 }).stdout.split("\n").filter(Boolean).map((row) => [row.slice(0, row.lastIndexOf(":")), Number(row.slice(row.lastIndexOf(":") + 1))] as const));
  const scanned = new Map<string, number>();
  for (const hit of hits) scanned.set(hit.path, (scanned.get(hit.path) ?? 0) + 1);
  const disagreements = [...new Set([...oracle.keys(), ...scanned.keys()])].filter((path) => (oracle.get(path) ?? 0) !== (scanned.get(path) ?? 0));
  onProgress(`${sources.length}/${sources.length} tagged sources scanned`);
  return { files: sources.length, hits, disagreements };
}
/** 🧱️ The JavaScript libraries reachable only through their interface module, package → owning directory (the dependency
 * policy's {@link DEPENDENCY_INTERFACE_OWNERS}): production code outside the owner never imports the package or its
 * subpaths; test-domain code (the taxonomy's tests, fixtures, examples, oracles, probes, generators) may use it as an oracle. */
export const INTERFACE_OWNED_PACKAGES: Readonly<Record<string, string>> = Object.freeze(Object.fromEntries(Object.entries(DEPENDENCY_INTERFACE_OWNERS).filter(([, owner]) => owner.ecosystem === "js").map(([name, owner]) => [name, owner.directory])));

/** 🧱️ One import of an interface-owned package outside its owner. */
export type InterfaceImportHit = Readonly<{ path: string; line: number; specifier: string; owner: string }>;

const MODULE_SPECIFIER = /(?:\bfrom\s*|\bimport\s*\(\s*|\brequire\s*\(\s*|^\s*import\s+)["']([^"'\n]+)["']/gmu;

/** 🧱️ Whether a path belongs to the test domain: under the domain root or below one of its directory names. */
export function isTestDomainPath(path: string, testDomain: Readonly<{ directoryNames: readonly string[]; domainPath: string }>): boolean {
  return path.startsWith(`${testDomain.domainPath}/`) || path.split("/").slice(0, -1).some((segment) => testDomain.directoryNames.includes(segment));
}

/** 🧱️ The imports of interface-owned packages in one production source outside their owners. */
export function interfaceImportHitsOfText(path: string, text: string, owners: Readonly<Record<string, string>> = INTERFACE_OWNED_PACKAGES): InterfaceImportHit[] {
  const hits: InterfaceImportHit[] = [];
  for (const match of text.matchAll(MODULE_SPECIFIER)) {
    const specifier = match[1]!;
    const owned = Object.keys(owners).find((name) => specifier === name || specifier.startsWith(`${name}/`));
    if (!owned || path.startsWith(owners[owned]!)) continue;
    const at = match.index! + match[0].length - specifier.length - 1;
    hits.push({ path, line: text.slice(0, at).split("\n").length, specifier, owner: owners[owned]! });
  }
  return hits;
}

/** 🧱️ The interface-ownership census over every tracked TypeScript / JavaScript production source (the test domain read from
 * the taxonomy), cross-checked line for line against `git grep` (the oracle). */
export function runInterfaceImportCensus(repoRoot: string, signal: AbortSignal, onProgress: (line: string) => void) {
  const taxonomy = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")) as Record<string, unknown>;
  const testDomain = dependencyTestDomain(taxonomy);
  const pathspec = ["--", "*.ts", "*.tsx", "*.js", "*.jsx", "*.mjs", "*.cjs", ":!.🧬semio"];
  const listed = spawnSync("git", ["ls-files", "-z", ...pathspec], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 });
  if (listed.status !== 0) throw new Error(`git ls-files failed: ${listed.stderr}`);
  const sources = listed.stdout.split("\0").filter((path) => path && !isTestDomainPath(path, testDomain));
  const hits: InterfaceImportHit[] = [];
  for (const [index, path] of sources.entries()) {
    if (signal.aborted) throw new Error("interface-ownership census cancelled");
    if (index % 5000 === 0) onProgress(`${index}/${sources.length} production sources scanned`);
    let text: string;
    try {
      text = readFileSync(join(repoRoot, path), "utf8");
    } catch {
      continue;
    }
    if (Object.keys(INTERFACE_OWNED_PACKAGES).some((name) => text.includes(name))) hits.push(...interfaceImportHitsOfText(path, text));
  }
  const names = Object.keys(INTERFACE_OWNED_PACKAGES).map((name) => name.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")).join("|");
  const production = new Set(sources);
  const oracle = new Set(
    spawnSync("git", ["grep", "-n", "-E", `(from|import|require)[[:space:]]*\\(?[[:space:]]*["'](${names})(/[^"']*)?["']`, ...pathspec], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 })
      .stdout.split("\n")
      .filter(Boolean)
      .map((row) => row.split(":").slice(0, 2))
      .filter(([path]) => production.has(path!) && !Object.values(INTERFACE_OWNED_PACKAGES).some((owner) => path!.startsWith(owner)))
      .map((parts) => parts.join(":")),
  );
  const scanned = new Set(hits.map((hit) => `${hit.path}:${hit.line}`));
  const disagreements = [...oracle].filter((key) => !scanned.has(key)).concat([...scanned].filter((key) => !oracle.has(key)));
  onProgress(`${sources.length}/${sources.length} production sources scanned`);
  return { files: sources.length, hits, disagreements };
}
//#endregion 🧮️SourceCensus

