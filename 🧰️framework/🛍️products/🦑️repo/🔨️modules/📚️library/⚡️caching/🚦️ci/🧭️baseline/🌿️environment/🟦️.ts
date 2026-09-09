import type { GithubHistoryRequest } from "../../🐙️github/🟦️.ts";

export type CiBaselineEnvironment = { readonly head: string; readonly full: boolean; readonly history: GithubHistoryRequest | null };
const text = (value: unknown): value is string => typeof value === "string" && value.length > 0 && !/[\x00-\x1f]/.test(value);
const object = (value: unknown): Record<string, unknown> => { if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Invalid GitHub event"); return value as Record<string, unknown>; };
const identity = (value: string | undefined): number => { if (!value || !/^[1-9][0-9]*$/.test(value) || !Number.isSafeInteger(Number(value))) throw new Error("Invalid GitHub identity"); return Number(value); };

/** 🎯️ Uses the checked-out commit and destination branch; scheduled, tagged and local checks cover the complete graph. */
export function ciBaselineEnvironment(environment: Readonly<Record<string, string | undefined>>, event: unknown, head: string, full: boolean): CiBaselineEnvironment {
  if (!/^[a-f0-9]{40}$/.test(head) || typeof full !== "boolean") throw new Error("Invalid CI checkout");
  if (environment.GITHUB_ACTIONS !== "true") return { head, full: true, history: null };
  if (!["push", "pull_request", "schedule", "workflow_dispatch"].includes(environment.GITHUB_EVENT_NAME ?? "")) throw new Error("Unsupported GitHub validation event");
  const repository = environment.GITHUB_REPOSITORY, repositoryId = identity(environment.GITHUB_REPOSITORY_ID), runId = identity(environment.GITHUB_RUN_ID);
  const payload = object(event), source = object(payload.repository);
  if (!repository || !/^(?!\.\.?\/)[A-Za-z0-9_.-]+\/(?!\.\.?$)[A-Za-z0-9_.-]+$/.test(repository) || environment.GITHUB_SHA !== head || source.id !== repositoryId || typeof source.full_name !== "string" || source.full_name.toLowerCase() !== repository.toLowerCase()) throw new Error("Invalid GitHub checkout or repository identity");
  let branch: unknown;
  if (environment.GITHUB_EVENT_NAME === "pull_request") {
    const base = object(object(payload.pull_request).base);
    if (object(base.repo).id !== repositoryId) throw new Error("Invalid pull request destination repository");
    branch = base.ref;
    if (!text(branch)) throw new Error("Invalid pull request destination branch");
  } else if (environment.GITHUB_REF?.startsWith("refs/heads/")) {
    branch = environment.GITHUB_REF.slice("refs/heads/".length);
    if (!text(branch)) throw new Error("Invalid GitHub branch");
  }
  if (full || environment.GITHUB_EVENT_NAME === "schedule" || !branch) return { head, full: true, history: null };
  return { head, full: false, history: { repository, repositoryId, runId, branch: branch as string } };
}
