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
import { dirname, join, resolve } from "node:path";
import { Script } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
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
export type AcceptanceRequirement = "hub" | "serve" | "backends";

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

/** 🗺️ `semio.acceptance.goal-plan/v1`. */
export type GoalPlan = Readonly<{ schema: "semio.acceptance.goal-plan/v1"; id: string; title: LocalizedText; steps: readonly GoalPlanStep[] }>;

/** 📊️ `semio.acceptance.goal-summary/v1`. */
export type GoalSummary = Readonly<{
  schema: "semio.acceptance.goal-summary/v1";
  plan: string;
  hub: string | null;
  serve: string | null;
  startedAt: string;
  finishedAt: string;
  verdict: AcceptanceStatus;
  totals: Readonly<Record<AcceptanceStatus, number>>;
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
//#endregion 🧾️CheckResult

//#region 🎯️GoalGate
/** 🎛️ One goal-gate invocation. */
export type GoalGateOptions = Readonly<{
  hub: string | null;
  serve: string | null;
  hubBinary: string | null;
  users: readonly Readonly<{ email: string; password: string }>[];
  planPath: string;
  outDir: string;
  only: readonly string[];
  includeOptional: boolean;
  maxBrowsers: number;
  maxParallel: number;
  signal: AbortSignal;
  execute?: GoalGateExecutor;
}>;

/** 🚀️ Runs one check's command with its environment and log file and resolves its exit code. The default spawns
 * `bun nx run <project>:<target> -- <args>` in its own process group; laws substitute a scripted executor. */
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

const STATUS_WORD: Readonly<Record<AcceptanceStatus, LocalizedText>> = {
  pass: { en: "pass", de: "bestanden" },
  fail: { en: "fail", de: "fehlgeschlagen" },
  blocked: { en: "blocked", de: "blockiert" },
  skipped: { en: "skipped", de: "übersprungen" },
};

/** 🔣️ Substitutes the gate's tokens: `{hub}`, `{serve}`, `{hubBinary}` and `{user<N>Email}` / `{user<N>Password}`
 * (1-based, from `--users`). Credentials are only ever substituted into ENVIRONMENT values, never into arguments, so no
 * secret reaches a log line. */
function substitute(value: string, options: GoalGateOptions): string {
  return value
    .replaceAll("{hub}", options.hub ?? "")
    .replaceAll("{serve}", options.serve ?? "")
    .replaceAll("{hubBinary}", options.hubBinary ?? "")
    .replace(/\{user(\d)(Email|Password)\}/gu, (_, index: string, field: string) => options.users[Number(index) - 1]?.[field === "Email" ? "email" : "password"] ?? "");
}

function syntheticResult(check: GoalPlanCheck, status: AcceptanceStatus, startedAt: Date, measured: Readonly<Record<string, number | string | boolean>>, summary: LocalizedText, evidence: readonly string[]): AcceptanceCheckResult {
  return acceptanceCheckResult({ check: check.id, status, startedAt, measured, summary, evidence });
}

function missingRequirement(check: GoalPlanCheck, options: GoalGateOptions): AcceptanceRequirement | undefined {
  return check.requires.find((requirement) => (requirement === "hub" && !options.hub) || (requirement === "serve" && !options.serve));
}

async function runCheck(repoRoot: string, check: GoalPlanCheck, options: GoalGateOptions, children: Set<ChildProcess>): Promise<AcceptanceCheckResult> {
  const startedAt = new Date();
  const missing = missingRequirement(check, options);
  if (missing) return syntheticResult(check, "blocked", startedAt, { missing }, { en: `needs --${missing} <url>`, de: `benötigt --${missing} <url>` }, []);
  if (options.signal.aborted) return syntheticResult(check, "skipped", startedAt, {}, { en: "cancelled before start", de: "vor dem Start abgebrochen" }, []);
  const resultPath = join(options.outDir, `${check.id}.json`);
  const logPath = join(options.outDir, `${check.id}.log`);
  const args = check.args.map((arg) => substitute(arg, options));
  const planned = Object.entries(check.env ?? {}).map(([key, value]) => [key, substitute(value, options)] as const).filter(([, value]) => value.length > 0);
  const env = { ...process.env, ...Object.fromEntries(planned), [ACCEPTANCE_RESULT_ENV]: resultPath, NX_TUI: "false" };
  console.log(`[goal-gate] ▶ ${check.id}: bun nx run ${check.project}:${check.target}${args.length ? ` -- ${args.join(" ")}` : ""}`);
  const exitCode = await (options.execute ?? spawnGoalGateCheck)(repoRoot, check, args, env, logPath, children);
  if (options.signal.aborted) return syntheticResult(check, "skipped", startedAt, { exitCode }, { en: "cancelled while running", de: "während der Ausführung abgebrochen" }, [logPath]);
  if (existsSync(resultPath)) {
    const record = JSON.parse(readFileSync(resultPath, "utf8")) as AcceptanceCheckResult;
    const violations = acceptanceSchemaViolations(repoRoot, "checkResult", record);
    if (violations.length) return syntheticResult(check, "fail", startedAt, { exitCode, schemaViolations: violations.length }, { en: `harness wrote an invalid record: ${violations[0]}`, de: `Prüfprogramm schrieb einen ungültigen Datensatz: ${violations[0]}` }, [resultPath, logPath]);
    if (record.status === "pass" && exitCode !== 0) return { ...record, status: "fail", measured: { ...record.measured, exitCode }, evidence: [...record.evidence, logPath] };
    return { ...record, evidence: [...record.evidence, logPath] };
  }
  const status: AcceptanceStatus = exitCode === 0 ? "pass" : "fail";
  return syntheticResult(check, status, startedAt, { exitCode }, exitCode === 0 ? { en: `${check.project}:${check.target} exited 0`, de: `${check.project}:${check.target} endete mit 0` } : { en: `${check.project}:${check.target} exited ${exitCode}`, de: `${check.project}:${check.target} endete mit ${exitCode}` }, [logPath]);
}

async function runParallel(repoRoot: string, checks: readonly GoalPlanCheck[], options: GoalGateOptions, children: Set<ChildProcess>): Promise<AcceptanceCheckResult[]> {
  const results = new Map<string, AcceptanceCheckResult>();
  const pending = [...checks];
  const running = new Map<string, Promise<void>>();
  let browsers = 0;
  while (pending.length || running.size) {
    const index = pending.findIndex((check) => running.size < options.maxParallel && (browsers + check.browsers <= options.maxBrowsers || (running.size === 0 && check.browsers > options.maxBrowsers)));
    if (index >= 0) {
      const check = pending.splice(index, 1)[0]!;
      browsers += check.browsers;
      running.set(check.id, runCheck(repoRoot, check, options, children).then((result) => {
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

/** 📝️ The human summary of one run in one language: the verdict, the totals and one row per check. */
export function goalSummaryMarkdown(summary: GoalSummary, plan: GoalPlan, language: keyof LocalizedText): string {
  const heading = language === "en" ? ["Step", "Check", "Status", "Seconds", "Summary"] : ["Schritt", "Prüfung", "Status", "Sekunden", "Zusammenfassung"];
  const lines = [`# ${plan.title[language]}`, "", `${language === "en" ? "Verdict" : "Ergebnis"}: **${STATUS_WORD[summary.verdict][language]}** — ${Object.entries(summary.totals).map(([status, count]) => `${STATUS_WORD[status as AcceptanceStatus][language]} ${count}`).join(", ")}`, "", `Hub: ${summary.hub ?? "—"} · Serve: ${summary.serve ?? "—"} · ${summary.startedAt} → ${summary.finishedAt}`, "", `| ${heading.join(" | ")} |`, `|${heading.map(() => "---").join("|")}|`];
  for (const step of summary.steps) {
    const planned = plan.steps.find((candidate) => candidate.id === step.id);
    for (const result of step.checks) {
      const title = planned?.checks.find((check) => check.id === result.check)?.title[language] ?? result.check;
      lines.push(`| ${planned?.title[language] ?? step.id} | ${title} | ${STATUS_WORD[result.status][language]} | ${Math.round(result.durationMs / 1000)} | ${result.summary[language].replaceAll("|", "\\|")} |`);
    }
  }
  return `${lines.join("\n")}\n`;
}

/** 🎯️ Runs the goal plan's steps in order — each step's checks serially or in parallel within the browser and process
 * budget — writes every check's record, `summary.json` (`semio.acceptance.goal-summary/v1`) and `summary.en.md` /
 * `summary.de.md` into `outDir`, and returns the summary. A failed gating step blocks every later step; cancellation
 * terminates the running checks' process groups and records the rest as skipped. */
export async function runGoalGate(repoRoot: string, options: GoalGateOptions): Promise<GoalSummary> {
  const plan = readGoalPlan(repoRoot, options.planPath);
  mkdirSync(options.outDir, { recursive: true });
  const children = new Set<ChildProcess>();
  const terminate = (): void => {
    for (const child of children) {
      if (child.pid === undefined) continue;
      try {
        if (process.platform === "win32") child.kill("SIGTERM");
        else process.kill(-child.pid, "SIGTERM");
      } catch {
        child.kill("SIGTERM");
      }
    }
  };
  options.signal.addEventListener("abort", terminate, { once: true });
  const startedAt = new Date().toISOString();
  const steps: { id: string; status: AcceptanceStatus; checks: AcceptanceCheckResult[] }[] = [];
  let gatedBy: string | null = null;
  const selected = plan.steps.filter((step) => (options.only.length ? options.only.includes(step.id) : !step.optional || options.includeOptional));
  for (const [index, step] of selected.entries()) {
    console.log(`[goal-gate] step ${index + 1}/${selected.length} ${step.id} (${step.mode}, ${step.checks.length} checks): ${step.title.en}`);
    let checks: AcceptanceCheckResult[];
    if (gatedBy !== null) {
      const now = new Date();
      const gate = gatedBy;
      checks = step.checks.map((check) => syntheticResult(check, "blocked", now, { gatedBy: gate }, { en: `blocked: gating step ${gate} did not pass`, de: `blockiert: der vorausgehende Schritt ${gate} ist nicht bestanden` }, []));
    } else if (step.mode === "parallel") {
      checks = await runParallel(repoRoot, step.checks, options, children);
    } else {
      checks = [];
      for (const check of step.checks) checks.push(await runCheck(repoRoot, check, options, children));
    }
    const status = stepStatus(checks);
    steps.push({ id: step.id, status, checks });
    for (const check of checks) console.log(`[goal-gate] ${check.status.toUpperCase().padEnd(7)} ${check.check} (${Math.round(check.durationMs / 1000)} s) — ${check.summary.en}`);
    if (step.gatesRest && status !== "pass" && gatedBy === null) gatedBy = step.id;
  }
  options.signal.removeEventListener("abort", terminate);
  const totals: Record<AcceptanceStatus, number> = { pass: 0, fail: 0, blocked: 0, skipped: 0 };
  for (const step of steps) for (const check of step.checks) totals[check.status] += 1;
  const verdict: AcceptanceStatus = totals.fail > 0 ? "fail" : totals.blocked > 0 || totals.skipped > 0 ? "blocked" : "pass";
  const summary: GoalSummary = { schema: "semio.acceptance.goal-summary/v1", plan: plan.id, hub: options.hub, serve: options.serve, startedAt, finishedAt: new Date().toISOString(), verdict, totals, steps };
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

/** 🎯️ `acceptance goal [--hub <url>] [--serve <url>] [--hub-binary <path>] [--users <json>] [--plan <path>] [--only <step,…>]
 * [--out <dir>] [--include-optional] [--max-browsers <n>] [--max-parallel <n>]` and `acceptance plan` (prints the plan). */
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
    if (verb !== "goal") throw new Error("usage: acceptance <goal|plan> [--hub <url>] [--serve <url>] [--hub-binary <path>] [--users <json>] [--plan <path>] [--only <step,…>] [--out <dir>] [--include-optional] [--max-browsers <n>] [--max-parallel <n>]");
    const controller = new AbortController();
    const cancel = (): void => controller.abort();
    process.once("SIGINT", cancel);
    process.once("SIGTERM", cancel);
    try {
      const outDir = resolve(option(rest, "--out") ?? join(this.repoRoot, GOAL_GATE_OUT_REL_PATH, new Date().toISOString().replaceAll(":", "-").replace(/\.\d+Z$/u, "Z")));
      const summary = await runGoalGate(this.repoRoot, {
        hub: option(rest, "--hub") ?? null,
        serve: option(rest, "--serve") ?? null,
        hubBinary: option(rest, "--hub-binary") ?? null,
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
//#endregion 🧮️SourceCensus

