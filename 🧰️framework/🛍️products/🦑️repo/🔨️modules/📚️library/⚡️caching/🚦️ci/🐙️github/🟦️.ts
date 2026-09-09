import type { CiValidationRun } from "../🧭️baseline/🟦️.ts";

export type GithubHistoryRequest = { readonly repository: string; readonly repositoryId: number; readonly runId: number; readonly branch: string };
export type GithubJson = (path: string, signal: AbortSignal) => Promise<{ status: number; body: unknown }>;
export type GithubHistory = { readonly workflowId: number | null; readonly runs: readonly CiValidationRun[]; readonly pages: number; readonly issue: "history-limit" | "history-unavailable" | null };
const identity = (value: unknown): value is number => Number.isSafeInteger(value) && (value as number) > 0;
const text = (value: unknown): value is string => typeof value === "string" && value.length > 0 && !/[\x00-\x1f]/.test(value);
const object = (value: unknown): Record<string, unknown> => { if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Invalid GitHub response"); return value as Record<string, unknown>; };

/** 🌐️ Reads bounded JSON without following redirects or forwarding credentials beyond the configured origin. */
export function githubJsonTransport(options: { origin?: string; token?: string } = {}): GithubJson {
  const origin = new URL(options.origin ?? "https://api.github.com");
  if (origin.username || origin.password || origin.search || origin.hash || origin.pathname !== "/" || origin.protocol !== "https:" && !(origin.protocol === "http:" && ["127.0.0.1", "[::1]"].includes(origin.hostname))) throw new Error("Invalid GitHub API origin");
  return async (path, signal) => {
    signal.throwIfAborted();
    if (!path.startsWith("/") || path.startsWith("//") || /[\x00-\x20\\]/.test(path)) throw new Error("Invalid GitHub API path");
    const url = new URL(path, origin), requestSignal = AbortSignal.any([signal, AbortSignal.timeout(30000)]);
    if (url.origin !== origin.origin) throw new Error("Invalid GitHub API path");
    const response = await fetch(url, { signal: requestSignal, redirect: "manual", headers: { accept: "application/vnd.github+json", "x-github-api-version": "2026-03-10", ...(options.token ? { authorization: `Bearer ${options.token}` } : {}) } });
    if (response.status !== 200) { await response.body?.cancel(); return { status: response.status, body: null }; }
    const reader = response.body?.getReader();
    if (!reader) throw new Error("Empty GitHub API response");
    const chunks: Uint8Array[] = [];
    let size = 0;
    try {
      for (;;) {
        requestSignal.throwIfAborted();
        const chunk = await reader.read();
        if (chunk.done) break;
        size += chunk.value.byteLength;
        if (size > 8 * 1024 * 1024) throw new Error("GitHub API response exceeds its limit");
        chunks.push(chunk.value);
      }
      requestSignal.throwIfAborted();
      return { status: response.status, body: JSON.parse(Buffer.concat(chunks).toString("utf8")) };
    } finally { await reader.cancel().catch(() => {}); reader.releaseLock(); }
  };
}

/** 📜️ Converts provider fields into the repository's validation-run contract. */
function validationRun(value: unknown): CiValidationRun | null {
  const run = object(value);
  if (run.head_repository === null || run.head_branch === null) return null;
  const repository = object(run.head_repository);
  if (![run.id, run.run_number, run.workflow_id, repository.id].every(identity) || ![run.head_branch, run.event, run.status].every(text) || typeof run.head_sha !== "string" || !/^[a-f0-9]{40}$/.test(run.head_sha) || run.conclusion !== null && !text(run.conclusion)) throw new Error("Invalid GitHub validation run");
  return { id: run.id as number, number: run.run_number as number, repositoryId: repository.id as number, workflowId: run.workflow_id as number, branch: run.head_branch as string, head: run.head_sha, event: run.event as string, status: run.status as string, conclusion: run.conclusion as string | null };
}

/** 🧾️ Obtains the current workflow identity and up to 1000 successful branch runs; unavailable history cannot authorize affected-only validation. */
export async function githubValidationHistory(request: GithubHistoryRequest, transport: GithubJson, signal: AbortSignal): Promise<GithubHistory> {
  signal.throwIfAborted();
  if (typeof request.repository !== "string" || !/^(?!\.\.?\/)[A-Za-z0-9_.-]+\/(?!\.\.?$)[A-Za-z0-9_.-]+$/.test(request.repository) || !identity(request.repositoryId) || !identity(request.runId) || !text(request.branch)) throw new Error("Invalid GitHub history request");
  let pages = 0;
  try {
    const currentResponse = await transport(`/repos/${request.repository}/actions/runs/${request.runId}`, signal);
    signal.throwIfAborted();
    if (currentResponse.status !== 200) throw new Error("GitHub workflow identity unavailable");
    const current = object(currentResponse.body);
    if (current.id !== request.runId || !identity(current.workflow_id) || object(current.repository).id !== request.repositoryId) throw new Error("GitHub workflow identity mismatch");
    const workflowId = current.workflow_id, runs: CiValidationRun[] = [];
    for (let page = 1; page <= 10; page++) {
      signal.throwIfAborted();
      const query = new URLSearchParams({ page: String(page), per_page: "100", branch: request.branch, status: "success" });
      const response = await transport(`/repos/${request.repository}/actions/workflows/${workflowId}/runs?${query}`, signal);
      signal.throwIfAborted();
      if (response.status !== 200) throw new Error("GitHub workflow history unavailable");
      const body = object(response.body);
      if (!Number.isSafeInteger(body.total_count) || (body.total_count as number) < 0 || !Array.isArray(body.workflow_runs) || body.workflow_runs.length > 100) throw new Error("Invalid GitHub workflow page");
      runs.push(...body.workflow_runs.map(validationRun).filter((run): run is CiValidationRun => run !== null));
      pages++;
      if (body.workflow_runs.length < 100 || page * 100 >= (body.total_count as number)) return { workflowId, runs, pages, issue: null };
    }
    return { workflowId, runs, pages, issue: "history-limit" };
  } catch {
    signal.throwIfAborted();
    return { workflowId: null, runs: [], pages, issue: "history-unavailable" };
  }
}
