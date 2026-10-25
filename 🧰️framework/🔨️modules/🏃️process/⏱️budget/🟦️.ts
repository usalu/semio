import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🌿️ Supplies caller-owned process budget overrides through first-party text values. */
export type ProcessBudgetEnvironment = Readonly<Record<string, string | undefined>>;

export const BUILD_BUDGET_MS = 0;
export const BUILD_PREPARATION_BUDGET_MS = schema.$defs.BuildPreparationDefault.const;
export const CMD_BUDGET_MS = 0;
export const ORCHESTRATOR_BUDGET_MS = 0;
export const DAEMON_BUDGET_MS = 0;

function readBudget(name: string, fallback: number, environment: ProcessBudgetEnvironment): number {
  const budget = Number(environment[name] ?? fallback);
  if (!Number.isSafeInteger(budget) || budget < schema.$defs.Milliseconds.minimum || budget > schema.$defs.Milliseconds.maximum) throw refuseBudgetV1("invalidBudget", { name });
  return budget;
}

/** 🏗️ Resolves a build budget without selecting a build tool or repository policy. */
export function buildBudgetMs(environment: ProcessBudgetEnvironment = process.env): number {
  return readBudget("SEMIO_BUILD_BUDGET_MS", BUILD_BUDGET_MS, environment);
}

/** 🧱️ Bounds expensive build preparation independently of an ordinary long-lived session. */
export function buildPreparationBudgetMs(environment: ProcessBudgetEnvironment = {}): number {
  return buildBudgetMs(environment) || BUILD_PREPARATION_BUDGET_MS;
}

/** 🛠️ Resolves a command budget supplied by its caller's environment. */
export function cmdBudgetMs(environment: ProcessBudgetEnvironment = process.env): number {
  return readBudget("SEMIO_CMD_BUDGET_MS", CMD_BUDGET_MS, environment);
}

/** 🎛️ Resolves an orchestrator budget supplied by its caller's environment. */
export function orchestratorBudgetMs(environment: ProcessBudgetEnvironment = process.env): number {
  return readBudget("SEMIO_ORCHESTRATOR_BUDGET_MS", ORCHESTRATOR_BUDGET_MS, environment);
}

/** 🖥️ Resolves a long-lived process budget supplied by its caller's environment. */
export function daemonBudgetMs(environment: ProcessBudgetEnvironment = process.env): number {
  return readBudget("SEMIO_DAEMON_BUDGET_MS", DAEMON_BUDGET_MS, environment);
}

const progressPolicy = schema.$defs.ProgressDeadlinePolicy.const, progressNotices = schema.$defs.ProgressDeadlineNotice.const;
const renderProgressNoticeV1 = (key: keyof typeof progressNotices, values: Readonly<Record<string, string | number>>, languages: readonly ("en" | "de")[]): string => languages.map((language) => Object.entries(values).reduce((text, [name, value]) => text.replaceAll("{" + name + "}", String(value)), progressNotices[key][language])).join("\n");
const refuseBudgetV1 = (key: keyof typeof progressNotices, values: Readonly<Record<string, string | number>> = {}): Error => new Error(renderProgressNoticeV1(key, values, ["en", "de"]));
/** ⏳️ Names the closed classes of long native work whose deadline is renewed by observable progress. @see ./🧬️schema/🔣️.json */
export type ProgressDeadlineKindV1 = keyof typeof progressPolicy.classes;
/** 💤️ The idle window and the nonrenewable maximum of one long action. */
export type ProgressDeadlinePolicyV1 = Readonly<{ idleMilliseconds: number; maximumMilliseconds: number }>;
/** 🏁️ Why a progress deadline ended: nothing progressed for the idle window, or the declared maximum is spent. */
export type ProgressDeadlineExpiryV1 = "idle" | "maximum";
/** 📶️ What a progress deadline reads: the clock, the milliseconds since the last observable progress (0 inside a declared wait) and the declared waits to credit against the maximum. */
export type ProgressDeadlineSignalsV1 = Readonly<{ now: () => number; idle: () => number; credit?: () => number }>;
/** ⏰️ A live deadline: the milliseconds left (<= 0 when over), and why it is over. */
export type ProgressDeadlineV1 = Readonly<{ policy: ProgressDeadlinePolicyV1; remaining: () => number; expiry: () => ProgressDeadlineExpiryV1 | null }>;

/** 🧮️ Derives the idle window and the generous maximum of an action from its class and declared cost: `base + perUnit * units` capped by the ceiling. An explicit grant (`declared`) replaces the derived values; the environment may only move the idle window. */
export function progressDeadlinePolicyV1(kind: ProgressDeadlineKindV1, units: number = 0, environment: ProcessBudgetEnvironment = process.env, declared: Readonly<{ idleMilliseconds?: number; maximumMilliseconds?: number }> = {}): ProgressDeadlinePolicyV1 {
  const row = (progressPolicy.classes as Record<string, { idleMilliseconds: number; baseMilliseconds: number; perUnitMilliseconds: number } | undefined>)[kind];
  if (row === undefined) throw refuseBudgetV1("unknownDeadlineClass", { kind: String(kind) });
  if (!Number.isSafeInteger(units) || units < 0) throw refuseBudgetV1("deadlineUnitsInvalid");
  const maximumMilliseconds = declared.maximumMilliseconds ?? Math.min(progressPolicy.ceilingMilliseconds, row.baseMilliseconds + row.perUnitMilliseconds * units), override = environment[progressPolicy.idleEnvironment];
  let idleMilliseconds = declared.idleMilliseconds ?? row.idleMilliseconds;
  if (override !== undefined && override !== "") idleMilliseconds = Number(override);
  if (!Number.isSafeInteger(idleMilliseconds) || idleMilliseconds < progressPolicy.minimumIdleMilliseconds || idleMilliseconds > progressPolicy.ceilingMilliseconds) throw refuseBudgetV1("deadlineIdleWindowInvalid", { minimum: progressPolicy.minimumIdleMilliseconds, ceiling: progressPolicy.ceilingMilliseconds, environment: progressPolicy.idleEnvironment });
  if (!Number.isSafeInteger(maximumMilliseconds) || maximumMilliseconds < 1) throw refuseBudgetV1("deadlineMaximumInvalid");
  return Object.freeze({ idleMilliseconds: Math.min(idleMilliseconds, maximumMilliseconds), maximumMilliseconds });
}

/** ⚖️ The single law: the smaller of the maximum minus the elapsed time and the idle window minus the time since the last progress (never more than the elapsed time itself). */
export function progressDeadlineRemainingV1(policy: ProgressDeadlinePolicyV1, elapsedMilliseconds: number, idleMilliseconds: number): number {
  return Math.min(policy.maximumMilliseconds - elapsedMilliseconds, policy.idleMilliseconds - Math.min(Math.max(0, idleMilliseconds), elapsedMilliseconds));
}

/** 🆕️ Starts one deadline now. Progress seen before the start does not count, declared credit moves only the maximum. */
export function createProgressDeadlineV1(policy: ProgressDeadlinePolicyV1, signals: ProgressDeadlineSignalsV1): ProgressDeadlineV1 {
  if (!Number.isSafeInteger(policy.idleMilliseconds) || !Number.isSafeInteger(policy.maximumMilliseconds) || policy.idleMilliseconds < 1 || policy.maximumMilliseconds < 1) throw refuseBudgetV1("deadlinePolicyNotFinite");
  const started = signals.now();
  const measure = (): { maximum: number; idle: number } => {
    const elapsed = signals.now() - started, credit = Math.max(0, signals.credit?.() ?? 0), idle = Math.min(Math.max(0, signals.idle()), elapsed);
    return { maximum: policy.maximumMilliseconds - Math.max(0, elapsed - credit), idle: policy.idleMilliseconds - idle };
  };
  return Object.freeze({
    policy: Object.freeze({ ...policy }),
    remaining: () => { const row = measure(); return Math.min(row.maximum, row.idle); },
    expiry: () => { const row = measure(); return row.maximum <= 0 ? "maximum" : row.idle <= 0 ? "idle" : null; },
  });
}

/** ⌛️ Checks one progress deadline again every time it believes to be due, so a renewal moves the expiry and nothing else does; every delay fits the native timer range. `expire` runs once with the reason. */
export function startProgressDeadlineV1(deadline: ProgressDeadlineV1, expire: (reason: ProgressDeadlineExpiryV1) => void, unref: boolean = false): () => void {
  if (typeof expire !== "function") throw refuseBudgetV1("deadlineCallbackRequired");
  let active = true, timer: ReturnType<typeof setTimeout> | undefined;
  const check = (): void => {
    if (!active) return;
    const remaining = deadline.remaining();
    if (remaining <= 0) { active = false; expire(deadline.expiry() ?? "idle"); return; }
    timer = setTimeout(check, Math.max(1, Math.min(schema.$defs.TimerMaximumMilliseconds.const, Math.ceil(remaining))));
    if (unref) timer.unref();
  };
  check();
  return () => { active = false; if (timer !== undefined) clearTimeout(timer); };
}

/** 🛑️ The refusal of an expired progress deadline: it names the reason and carries the notice in every selected language. */
export class ProgressDeadlineExpiredV1 extends Error {
  readonly reason: ProgressDeadlineExpiryV1;
  constructor(reason: ProgressDeadlineExpiryV1, message: string) { super(message); this.name = "ProgressDeadlineExpired"; this.reason = reason; }
}

/** 🗣️ Renders one refusal in every selected language. */
export function progressDeadlineNoticeV1(reason: ProgressDeadlineExpiryV1, label: string, policy: ProgressDeadlinePolicyV1, idleMilliseconds: number, units: number, languages: readonly ("en" | "de")[]): string {
  const values: Record<string, string | number> = { label, minutes: Math.round((reason === "idle" ? idleMilliseconds : policy.maximumMilliseconds) / 60000), window: Math.round(policy.idleMilliseconds / 60000), units };
  return renderProgressNoticeV1(reason, values, languages);
}

/** 🦑️ The repository command policy of one long native command (a Cargo check, test or build): the class idle window inside the class maximum for its declared cost, so a script names its output limits and never a wall clock. */
export function nativeCommandPolicyV1<Output extends Readonly<{ maximumBytes: number; maximumLines: number }> | null>(output: Output, units: number = 0, environment: ProcessBudgetEnvironment = process.env): Readonly<{ version: 1; maximumElapsedMilliseconds: number; idleMilliseconds: number; output: Output }> {
  const policy = progressDeadlinePolicyV1("native-child", units, environment);
  return Object.freeze({ version: 1, maximumElapsedMilliseconds: policy.maximumMilliseconds, idleMilliseconds: policy.idleMilliseconds, output });
}

/** 🔎️ Whether the diagnostic trace of progress deadlines is requested. */
export function progressDeadlineTraceEnabledV1(environment: ProcessBudgetEnvironment = process.env): boolean { return environment[progressPolicy.traceEnvironment] === "1"; }

/** 🕵️ Renders one trace line in every selected language: a renewable deadline was created, or child CPU work renewed the idle windows. */
export function progressDeadlineTraceV1(kind: "traceCreated" | "traceWork", values: Readonly<Record<string, string | number>>, languages: readonly ("en" | "de")[]): string {
  return renderProgressNoticeV1(kind, values, languages) + "\n";
}
