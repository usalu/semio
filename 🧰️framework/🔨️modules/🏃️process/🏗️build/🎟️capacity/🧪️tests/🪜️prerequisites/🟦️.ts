import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { availableParallelism, tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { acquireCompilerCapacityLeaseV1, scriptCompilerAdmissionControlV1 } from "../../🟦️.ts";
import { compileUnitsShareV1 } from "../../🧮️units/🟦️.ts";
import { captureOwnedCommandResultV1 } from "../../../../🎛️owned-execution/📥️capture/🟦️.ts";
import { declareBuildTestEnvelopeV1, withBuildTestInvocationV1 } from "../../../../🧪️testing/📥️invocation/🟦️.ts";
import { nativeBuildIndependentJobPlanV1 } from "../../../../📦️artifacts/🏗️native-build/🧩️component-pass/📋️prerequisites/🟦️.ts";
import fixture from "./🧫️cases.json" with { type: "json" };

declareBuildTestEnvelopeV1({ owner: "test:compiler-prerequisites", maximumElapsedMilliseconds: 60_000, capabilities: {} });

test("an independent tool subprocess progresses while two compiler owners retain their physical shares", () => withBuildTestInvocationV1(async original => {
  const host = availableParallelism();
  if (host < 3) return;
  const { preferred, minimum: reserve } = compileUnitsShareV1(host);
  const state = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();
  mkdirSync(state, { recursive: true });
  const directory = mkdtempSync(join(state, "cp-"));
  const left = Math.min(preferred, host - reserve - 1);
  const acquire = (owner: string, max: number) => acquireCompilerCapacityLeaseV1({
    directory, control: scriptCompilerAdmissionControlV1(original, owner, 25_000), environment: {},
    request: { kind: "available", maximumParallelism: max }, independentJobs: true,
  });
  const oracle = new Database(":memory:");
  try {
    const normal = await acquire("test:ordinary-parent", preferred);
    try { expect(normal.parallelism).toBe(preferred); } finally { normal.release(); }
    const first = await acquire("test:first-parent", left);
    try {
      const second = await acquire("test:second-parent", host - left);
      try {
        const actual = scriptCompilerAdmissionControlV1(original, "test:parent-prerequisites", 15_000);
        const child = await captureOwnedCommandResultV1(process.execPath,
          [fileURLToPath(new URL("./📜️script.ts", import.meta.url)), directory, String(reserve)],
          process.cwd(), { control: actual, output: { maximumBytes: 65_536, maximumLines: 256 } });
        expect(child.status, child.stderr).toBe(0);
        const slots = JSON.parse(child.stdout).slots;
        const observation = oracle.query<{ parents: number; total: number }, [number, number, number]>(
          "SELECT ?1 + ?2 AS parents, ?1 + ?2 + ?3 AS total"
        ).get(first.parallelism, second.parallelism, slots);
        expect(observation).toEqual({ parents: host - reserve, total: host });
      } finally { second.release(); }
    } finally { first.release(); }
    for (const model of fixture.models) {
      const share = compileUnitsShareV1(model.host);
      expect([share.preferred, share.minimum, model.host - share.minimum]).toEqual([model.preferred, model.reserve, model.coreLimit]);
    }
  } finally {
    oracle.close();
    rmSync(directory, { recursive: true, force: true });
  }
}), 30_000);

test("only a one-core release settles repository prerequisites ahead of the compiler", () => {
  const fixturePlan = fixture.plan;
  const plan = {
    version: 1 as const,
    passes: [],
    jobs: fixturePlan.jobs.map(row => ({
      ...row, cwd: ".", profile: "release", target: "wasm32-wasip2", workerProtocol: "jobserver" as const,
    })),
    stageAfter: fixturePlan.stageAfter,
  };
  const oracle = new Database(":memory:");
  try {
    oracle.exec("CREATE TABLE owners(id TEXT PRIMARY KEY, position INTEGER, command TEXT); CREATE TABLE edges(job TEXT, prerequisite TEXT)");
    const job = oracle.query("INSERT INTO owners VALUES (?1,?2,?3)");
    const edge = oracle.query("INSERT INTO edges VALUES (?1,?2)");
    for (const [index, item] of plan.jobs.entries()) {
      job.run(item.id, index, item.command[0]);
      for (const dependency of item.after) edge.run(item.id, dependency.slice(4));
    }
    const expected = (oracle.query(`
      WITH RECURSIVE depths(id, depth) AS (
        SELECT id, 0 FROM owners WHERE command='repo'
        UNION ALL SELECT edges.job, depths.depth + 1 FROM depths JOIN edges ON edges.prerequisite = depths.id
      )
      SELECT owners.id FROM owners JOIN depths USING(id) WHERE command='repo' GROUP BY owners.id
      ORDER BY max(depth), position
    `).all() as {id:string}[]).map(row => row.id);
    expect(expected).toEqual(fixturePlan.before);
    const single = nativeBuildIndependentJobPlanV1(plan, 1);
    expect(single.before.map(row => row.id)).toEqual(expected);
    expect(single.independentJobs).toBe(false);
    expect(single.plan.jobs?.map(({id,after}) => ({id,after}))).toEqual(fixturePlan.remaining);
    expect(single.plan.stageAfter).toEqual(fixturePlan.remainingStages);
    expect(nativeBuildIndependentJobPlanV1(plan, 12)).toEqual({plan, before: [], independentJobs: true});
    expect(nativeBuildIndependentJobPlanV1(plan, 12).plan).toBe(plan);
  } finally {
    oracle.close();
  }
});
