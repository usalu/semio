export type CiBaselineContext = { readonly repositoryId: number; readonly workflowId: number; readonly runId: number; readonly branch: string; readonly head: string; readonly full: boolean };
export type CiValidationRun = { readonly id: number; readonly number: number; readonly repositoryId: number; readonly workflowId: number; readonly branch: string; readonly head: string; readonly event: string; readonly status: string; readonly conclusion: string | null };
export type CiBaseline =
  | { readonly mode: "affected"; readonly head: string; readonly base: string; readonly runId: number; readonly reason: "last-successful-ancestor" }
  | { readonly mode: "all"; readonly head: string; readonly base: null; readonly runId: null; readonly reason: "full-requested" | "no-successful-ancestor" };
const sha = (value: unknown): value is string => typeof value === "string" && /^[a-f0-9]{40}$/.test(value);
const identity = (value: unknown): value is number => Number.isSafeInteger(value) && (value as number) > 0;
const text = (value: unknown): value is string => typeof value === "string" && value.length > 0 && !/[\x00-\x1f]/.test(value);

/** 🧭️ Includes changes since a successful trusted ancestor, selecting full validation when none is proven. */
export async function selectCiBaseline(context: CiBaselineContext, runs: readonly CiValidationRun[], isAncestor: (base: string, head: string) => Promise<boolean>, signal?: AbortSignal): Promise<CiBaseline> {
  signal?.throwIfAborted();
  if (![context.repositoryId, context.workflowId, context.runId].every(identity) || !text(context.branch) || !sha(context.head) || typeof context.full !== "boolean") throw new Error("Invalid CI baseline context");
  if (context.full) return { mode: "all", head: context.head, base: null, runId: null, reason: "full-requested" };
  for (const run of runs) {
    if (![run.id, run.number, run.repositoryId, run.workflowId].every(identity) || !sha(run.head) || !text(run.branch) || !text(run.event) || !text(run.status) || run.conclusion !== null && !text(run.conclusion)) throw new Error("Invalid CI validation run");
  }
  const candidates = runs.filter(run => run.id !== context.runId && run.repositoryId === context.repositoryId && run.workflowId === context.workflowId && run.branch === context.branch && run.status === "completed" && run.conclusion === "success" && ["push", "schedule", "workflow_dispatch"].includes(run.event)).sort((left, right) => right.number - left.number || right.id - left.id);
  for (const run of candidates) {
    signal?.throwIfAborted();
    const ancestor = await isAncestor(run.head, context.head);
    signal?.throwIfAborted();
    if (ancestor) return { mode: "affected", head: context.head, base: run.head, runId: run.id, reason: "last-successful-ancestor" };
  }
  return { mode: "all", head: context.head, base: null, runId: null, reason: "no-successful-ancestor" };
}
