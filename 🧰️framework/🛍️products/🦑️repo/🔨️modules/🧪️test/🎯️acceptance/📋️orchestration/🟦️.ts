//#region 🔌️Adapters
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🎯️ Repository acceptance: the check-result record every acceptance harness writes, the goal plan (the ordered, grouped
// verification of the repository goal) and the goal gate that runs the plan against one hub + one `s` serve and writes a
// schema-bound summary plus an English and a German human summary.
//
// This file keeps the schema vocabulary, the check-result record and the process runner; the children hold the rest:
// 🚦️goal-gate (the plan runner and the `AcceptanceScript` command), 🧮️placeholder-and-job-census, 📝️docstring-and-debug-census,
// 🔌️interface-import-census and 🪢️artifact-neutrality (the source censuses).

//#endregion 🧲️Header
import { spawn } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

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

/** 🧪️ One check of the goal plan: a build target plus its arguments; `{hub}` and `{serve}` are substituted. */
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

/** 🏗️ How the gate stands up one requirement zero-touch: a build target it holds for the whole run, answering at
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
