import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

/** 🧪️ Compares baseline selection with lodash and native Git ancestry without modifying history. */
export async function testCiBaseline(workspace: string): Promise<void> {
  const require = createRequire(import.meta.url), lodash = require("lodash"), validator = new (require("ajv"))();
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧭️baseline-selection/🔣️.json"), "utf8")), schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🔣️.json"), "utf8"));
  const contextValid = validator.compile({ ...schema, $ref: "#/definitions/context" }), runValid = validator.compile({ ...schema, $ref: "#/definitions/run" }), selectionValid = validator.compile({ ...schema, $ref: "#/definitions/selection" });
  assert.equal(contextValid(fixture.context), true);
  const { selectCiBaseline } = await import("../../🟦️.ts");
  for (const row of fixture.cases) {
    const runs = fixture.runs.map((run: any, index: number) => index === 2 ? { ...run, ...row.change } : run);
    assert.ok(runs.every((run: any) => runValid(run)));
    const oracle = lodash.orderBy(lodash.filter(runs, (run: any) => run.id !== fixture.context.runId && run.repositoryId === fixture.context.repositoryId && run.workflowId === fixture.context.workflowId && run.branch === fixture.context.branch && run.status === "completed" && run.conclusion === "success" && ["push", "schedule", "workflow_dispatch"].includes(run.event) && fixture.ancestors.includes(run.head)), ["number", "id"], ["desc", "desc"])[0];
    assert.equal(oracle.id, row.expectedRunId, row.name);
    const selected = await selectCiBaseline(fixture.context, runs, async (base: string) => fixture.ancestors.includes(base));
    assert.equal(selectionValid(selected), true);
    assert.deepEqual(selected, { mode: "affected", head: fixture.context.head, base: oracle.head, runId: oracle.id, reason: "last-successful-ancestor" }, row.name);
  }
  for (const full of [false, true]) {
    const selection = await selectCiBaseline({ ...fixture.context, full }, full ? fixture.runs : [], async () => { throw new Error("Ancestry must not be queried"); });
    assert.deepEqual(selection, { mode: "all", head: fixture.context.head, base: null, runId: null, reason: full ? "full-requested" : "no-successful-ancestor" });
  }
  await assert.rejects(selectCiBaseline({ ...fixture.context, head: "--bad" }, [], async () => true), /Invalid/);
  const cancelled = new AbortController(); cancelled.abort();
  await assert.rejects(selectCiBaseline(fixture.context, fixture.runs, async () => { throw new Error("Cancelled selection queried ancestry"); }, cancelled.signal), /abort/i);
  const interrupted = new AbortController();
  await assert.rejects(selectCiBaseline(fixture.context, fixture.runs, async () => { interrupted.abort(); return true; }, interrupted.signal), /abort/i);
  const commits = spawnSync("git", ["rev-list", "--max-count=3", "HEAD"], { cwd: workspace, encoding: "utf8" }); assert.equal(commits.status, 0, commits.stderr);
  const [head, parent] = commits.stdout.trim().split("\n"); assert.ok(head && parent);
  const selected = await selectCiBaseline({ ...fixture.context, head }, [{ ...fixture.runs[0], head: parent }], async (base: string, target: string) => {
    const git = spawnSync("git", ["merge-base", "--is-ancestor", base, target], { cwd: workspace, encoding: "utf8" });
    assert.ok(git.status === 0 || git.status === 1, git.stderr); return git.status === 0;
  });
  assert.equal(selected.base, parent);
  console.log(`[DEBUG] CI baseline ${fixture.cases.length} selection vectors, full fallback, validation and native Git ancestry PASS`);
}
