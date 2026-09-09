import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🐙️ Verifies paginated GitHub history, bounded transport and fail-closed fallback against schema and lodash. */
export async function testGithubHistory(): Promise<void> {
  const require = createRequire(import.meta.url), ajv = new (require("ajv"))(), lodash = require("lodash");
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")), schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🔣️.json"), "utf8"));
  const requestValid = ajv.compile({ ...schema, $ref: "#/definitions/request" }), historyValid = ajv.compile({ ...schema, $ref: "#/definitions/history" });
  assert.equal(requestValid(fixture.request), true);
  const { githubValidationHistory, githubJsonTransport } = await import("../../🟦️.ts");
  const first = Array.from({ length: 100 }, (_, index) => ({ ...fixture.run, id: 300 - index, run_number: 200 - index, head_repository: { id: fixture.foreignRepositoryId } }));
  const calls: string[] = [];
  const transport = async (path: string) => { calls.push(path); return { status: 200, body: path.includes("/actions/runs/") ? fixture.current : { total_count: 101, workflow_runs: path.includes("page=1&") ? first : [fixture.run] } }; };
  const controller = new AbortController(), result = await githubValidationHistory(fixture.request, transport, controller.signal);
  assert.equal(historyValid(result), true); assert.equal(result.workflowId, 73); assert.equal(result.pages, 2); assert.equal(result.issue, null);
  const oracle = lodash.map([...first, fixture.run], (run: any) => ({ id: run.id, number: run.run_number, workflowId: run.workflow_id, repositoryId: run.head_repository.id, branch: run.head_branch, head: run.head_sha, event: run.event, status: run.status, conclusion: run.conclusion }));
  assert.deepEqual(result.runs, oracle);
  assert.equal(calls[0], "/repos/semio/fixture/actions/runs/500");
  for (const [index, path] of calls.slice(1).entries()) {
    const url = new URL(path, "https://api.github.com");
    assert.equal(url.pathname, "/repos/semio/fixture/actions/workflows/73/runs");
    assert.equal(url.searchParams.get("branch"), fixture.request.branch); assert.equal(url.searchParams.get("status"), "success");
    assert.equal(url.searchParams.get("page"), String(index + 1)); assert.equal(url.searchParams.get("per_page"), "100");
  }
  for (const status of fixture.unavailableStatuses) {
    const unavailable = await githubValidationHistory(fixture.request, async () => ({ status, body: { message: "PRIVATE RESPONSE" } }), controller.signal);
    assert.deepEqual(unavailable, { workflowId: null, runs: [], pages: 0, issue: "history-unavailable" });
  }
  for (const change of fixture.invalidRequests) {
    assert.equal(requestValid({ ...fixture.request, ...change }), false);
    await assert.rejects(githubValidationHistory({ ...fixture.request, ...change }, async () => { throw new Error("Invalid request sent"); }, controller.signal), /Invalid GitHub history request/);
  }
  for (const current of [{ ...fixture.current, id: 1 }, { ...fixture.current, workflow_id: null }, { ...fixture.current, repository: { id: 999 } }]) {
    const history = await githubValidationHistory(fixture.request, async () => ({ status: 200, body: current }), controller.signal);
    assert.equal(history.issue, "history-unavailable"); assert.equal(history.runs.length, 0);
  }
  const brokenPage = await githubValidationHistory(fixture.request, async path => ({ status: 200, body: path.includes("/actions/runs/") ? fixture.current : { total_count: 1, workflow_runs: [{ ...fixture.run, head_sha: "--invalid" }] } }), controller.signal);
  assert.equal(brokenPage.issue, "history-unavailable"); assert.deepEqual(brokenPage.runs, []);
  const partial = await githubValidationHistory(fixture.request, async path => path.includes("page=2&") ? { status: 403, body: null } : { status: 200, body: path.includes("/actions/runs/") ? fixture.current : { total_count: 101, workflow_runs: first } }, controller.signal);
  assert.deepEqual(partial, { workflowId: null, runs: [], pages: 1, issue: "history-unavailable" });
  let limitedCalls = 0;
  const limited = await githubValidationHistory(fixture.request, async path => { limitedCalls++; return { status: 200, body: path.includes("/actions/runs/") ? fixture.current : { total_count: 1200, workflow_runs: first.map((run, index) => ({ ...run, id: limitedCalls * 1000 + index })) } }; }, controller.signal);
  assert.equal(limitedCalls, 11); assert.equal(limited.pages, 10); assert.equal(limited.runs.length, 1000); assert.equal(limited.issue, "history-limit");
  const aborted = new AbortController(); aborted.abort();
  await assert.rejects(githubValidationHistory(fixture.request, async () => { throw new Error("Cancelled request sent"); }, aborted.signal), /abort/i);
  const interrupted = new AbortController();
  await assert.rejects(githubValidationHistory(fixture.request, async () => { interrupted.abort(); return { status: 200, body: fixture.current }; }, interrupted.signal), /abort/i);
  const server = Bun.serve({ hostname: "127.0.0.1", port: 0, fetch(request) {
    const url = new URL(request.url);
    assert.equal(request.headers.get("authorization"), "Bearer fixture-token");
    assert.equal(request.headers.get("x-github-api-version"), "2026-03-10");
    if (url.pathname === "/redirect") return new Response(null, { status: 302, headers: { location: "https://example.org/never-forward-token" } });
    if (url.pathname === "/oversized") return new Response("x".repeat(8 * 1024 * 1024 + 1));
    if (url.pathname === "/stream") return new Response(new ReadableStream({ start(stream) { stream.enqueue(new TextEncoder().encode("{")); } }));
    return Response.json(fixture.current);
  } });
  try {
    const api = githubJsonTransport({ origin: server.url.origin, token: "fixture-token" });
    assert.deepEqual(await api("/repos/semio/fixture/actions/runs/500", controller.signal), { status: 200, body: fixture.current });
    await assert.rejects(api("//example.org/escape", controller.signal), /path/i);
    await assert.rejects(api("/oversized", controller.signal), /limit/i);
    assert.deepEqual(await api("/redirect", controller.signal), { status: 302, body: null });
    await assert.rejects(api("/cancelled", aborted.signal), /abort/i);
    const reading = new AbortController(), timer = setTimeout(() => reading.abort(), 100), started = Date.now();
    try { await assert.rejects(api("/stream", reading.signal), /abort/i); assert.ok(Date.now() - started < 2000); }
    finally { clearTimeout(timer); }
  } finally { await server.stop(true); }
  console.log("[DEBUG] GitHub history pagination, identity, schema/lodash projection, failure fallback, cancellation and real bounded HTTP transport PASS");
}
