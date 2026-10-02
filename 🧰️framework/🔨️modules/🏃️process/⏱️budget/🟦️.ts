import schema from "./🧬️schema/🔣️.json";

/** 🌿️ Supplies caller-owned process budget overrides through first-party text values. */
export type ProcessBudgetEnvironment = Readonly<Record<string, string | undefined>>;

export const BUILD_BUDGET_MS = 0;
export const CMD_BUDGET_MS = 0;
export const ORCHESTRATOR_BUDGET_MS = 0;
export const DAEMON_BUDGET_MS = 0;

function readBudget(name: string, fallback: number, environment: ProcessBudgetEnvironment): number {
  const budget = Number(environment[name] ?? fallback);
  if (!Number.isSafeInteger(budget) || budget < schema.$defs.Milliseconds.minimum || budget > schema.$defs.Milliseconds.maximum) throw Error(`Invalid process budget ${name}`);
  return budget;
}

/** 🏗️ Resolves a build budget without selecting a build tool or repository policy. */
export function buildBudgetMs(environment: ProcessBudgetEnvironment = process.env): number {
  return readBudget("SEMIO_BUILD_BUDGET_MS", BUILD_BUDGET_MS, environment);
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
