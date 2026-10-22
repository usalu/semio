/** 🚦️ The goal gate: running the repository goal plan against one hub and one serve, provisioning their requirements, the summary tables and the `AcceptanceScript` command. */
import { type ChildProcess } from "node:child_process";
import { createWriteStream, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { isAbsolute, join, resolve } from "node:path";
import { Script } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ACCEPTANCE_RESULT_ENV, acceptanceCheckResult, type AcceptanceCheckResult, type AcceptanceRequirement, acceptanceSchemaViolations, type AcceptanceStatus, GOAL_GATE_OUT_REL_PATH, GOAL_PLAN_REL_PATH, type GoalPlan, type GoalPlanCheck, type GoalPlanProvider, type GoalPlanStep, type GoalSummary, type GoalSummaryOutcome, type LocalizedText, type ProvidedRequirement, readGoalPlan } from "../🟦️.ts";
import { startToolV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";

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

/** 🏗️ Stands up one provider (the default spawns `repo run <project>:<target> -- <args>` in its own process group and
 * waits for its url), reporting progress lines; resolves once the url answers, rejects on the bound, an early exit with
 * nothing answering, or cancellation, and hands back `stop()` that ends exactly what it started. */
export type GoalGateProvisioner = (repoRoot: string, requirement: ProvidedRequirement, provider: GoalPlanProvider, args: readonly string[], env: NodeJS.ProcessEnv, logPath: string, signal: AbortSignal, onProgress: (line: string) => void) => Promise<Readonly<{ stop: () => Promise<void> }>>;

/** 🚀️ Runs one check's command with its environment and log file and resolves its exit code. The default spawns
 * `repo run <project>:<target> -- <args>` in its own process group with the shared build invocation (every invocation of the
 * gate computes its own project graph, so a wedged or foreign daemon cannot stall a check); laws substitute a scripted
 * executor. */
export type GoalGateExecutor = (repoRoot: string, check: GoalPlanCheck, args: readonly string[], env: NodeJS.ProcessEnv, logPath: string, children: Set<ChildProcess>) => Promise<number>;

const spawnGoalGateCheck: GoalGateExecutor = (repoRoot, check, args, env, logPath, children) => {
  const log = createWriteStream(logPath);
  const started = Date.now();
  return new Promise<number>((resolveExit) => {
    const child = startToolV1("repo", ["run", `${check.project}:${check.target}`, ...(args.length ? ["--", ...args] : [])], { cwd: repoRoot, env, stdio: ["ignore", "pipe", "pipe"], detached: process.platform !== "win32" });
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
  const child = startToolV1("repo", ["run", `${provider.project}:${provider.target}`, ...(args.length ? ["--", ...args] : [])], { cwd: repoRoot, env, stdio: ["ignore", "pipe", "pipe"], detached: process.platform !== "win32" });
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
  const env = { ...process.env, ...Object.fromEntries(planned), [ACCEPTANCE_RESULT_ENV]: resultPath, SEMIO_BUILD_TERMINAL: "false" };
  console.log(`[goal-gate] ▶ ${check.id}: repo run ${check.project}:${check.target}${args.length ? ` -- ${args.join(" ")}` : ""}`);
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
    const env = { ...process.env, ...Object.fromEntries(Object.entries(provider.env ?? {}).map(([key, value]) => [key, substitute(value, current)])), SEMIO_BUILD_TERMINAL: "false" };
    const logPath = join(options.outDir, `provider-${requirement}.log`);
    console.log(`[goal-gate] ▶ provider ${requirement}: repo run ${provider.project}:${provider.target}${args.length ? ` -- ${args.join(" ")}` : ""} → ${provider.url}`);
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
      console.log("[goal-gate] ▶ backends: repo run os-hub-ts:backend-up -- all (the shared postgres + neo4j servers, zero-touch)");
      backendsReady = (await (options.execute ?? spawnGoalGateCheck)(repoRoot, GOAL_GATE_BACKENDS_CHECK, ["all"], { ...process.env, SEMIO_BUILD_TERMINAL: "false" }, backendsLog, children)) === 0;
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
        for (const check of step.checks) console.log(`  ${check.id}: repo run ${check.project}:${check.target}${check.args.length ? ` -- ${check.args.join(" ")}` : ""} [criteria ${check.criteria.join(",")}; needs ${check.requires.join(",") || "nothing"}; browsers ${check.browsers}]`);
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
