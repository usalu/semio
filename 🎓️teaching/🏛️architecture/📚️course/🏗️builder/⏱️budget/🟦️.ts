import { buildBudgetMs } from "@semio-tech/process/budget";
import { scriptInvocationBudget, type ScriptInvocation } from "@semio-tech/process/routing";

/** ⏱️ The time a course compiler step requests: the neutral build budget of the environment (`SEMIO_BUILD_BUDGET_MS`, 0 meaning the caller's whole remaining deadline), never a constant of its own.
 * @see ./🧬️schema/🔣️.json — the fixture contract */
export function courseBudgetRequestMs(environment: Readonly<Record<string, string | undefined>> = process.env): number {
  return buildBudgetMs(environment);
}

/** ⏳️ The budget of one course compiler step: the request intersected with the original operation's remaining time. */
export function courseBudgetMs(invocation: ScriptInvocation, environment: Readonly<Record<string, string | undefined>> = process.env): number {
  return scriptInvocationBudget(invocation, courseBudgetRequestMs(environment));
}
