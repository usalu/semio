import { ciBaselineEnvironment } from "../🌿️environment/🟦️.ts";
import { selectCiBaseline, type CiBaseline } from "../🟦️.ts";
import { githubValidationHistory, type GithubJson, type GithubHistory } from "../../🐙️github/🟦️.ts";

/** 🔗️ Resolves a checkout's validation boundary without inventing a baseline when history cannot be trusted. */
export async function resolveCiValidation(input: { environment: Readonly<Record<string, string | undefined>>; event: unknown; head: string; full: boolean }, transport: GithubJson, isAncestor: (base: string, head: string) => Promise<boolean>, signal: AbortSignal): Promise<{ baseline: CiBaseline; historyIssue: GithubHistory["issue"] }> {
  signal.throwIfAborted();
  const selected = ciBaselineEnvironment(input.environment, input.event, input.head, input.full);
  if (!selected.history) return { baseline: { mode: "all", head: selected.head, base: null, runId: null, reason: "full-requested" }, historyIssue: null };
  const history = await githubValidationHistory(selected.history, transport, signal);
  signal.throwIfAborted();
  if (!history.workflowId) return { baseline: { mode: "all", head: selected.head, base: null, runId: null, reason: "no-successful-ancestor" }, historyIssue: history.issue };
  const baseline = await selectCiBaseline({ ...selected.history, workflowId: history.workflowId, head: selected.head, full: false }, history.runs, isAncestor, signal);
  return { baseline, historyIssue: history.issue };
}
