import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import schema from "../../🎭️e2e/🔒️sessions/🧬️schema/🔣️.json" with { type: "json" };
import fixtures from "../../🎭️e2e/🔒️sessions/🧫️fixtures/🔣️.json" with { type: "json" };
import { SHARD_TABLE, allocateWorkersV1, planStacksV1, projectMatchesV1, runTopologySessionsV1, scheduleStacksV1, splitProjectArgumentsV1, splitWorkerArgumentsV1, waveArgumentsV1, workerBudgetV1 } from "../../🎭️e2e/🔒️sessions/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

describe("scoped topology sessions", () => {
  it("agrees with draft-07 on the language-neutral lifecycle cases", () => {
    const validate = new Ajv({ strict: true }).compile(schema);
    for (const value of [fixtures, { cases: [{ serial: "yes", failure: null, trace: [] }] }])
      expect(validateJsonSchemaSubset(schema, value).length === 0).toBe(validate(value));
  });

  for (const row of fixtures.cases) it(`retains every reader until its consumer drains (${row.serial ? "serial" : "concurrent"}, failure ${row.failure})`, async () => {
    const trace: string[] = [];
    const sessions = runTopologySessionsV1(["rehearsal", "dev"], row.serial, async (topology, consume) => {
      trace.push(`open ${topology}`);
      try {
        if (row.failure === `open ${topology}`) throw Error(topology);
        await consume(topology);
      }
      finally { trace.push(`close ${topology}`); }
    }, async topology => {
      trace.push(`start ${topology}`);
      await new Promise(resolve => setTimeout(resolve, topology === "dev" ? 10 : 0));
      trace.push(`${row.failure === topology ? "fail" : "end"} ${topology}`);
      if (row.failure === topology) throw Error(String(topology));
    });
    if (row.failure) await expect(sessions).rejects.toThrow();
    else await sessions;
    expect(trace).toEqual(row.trace);
  });
});

describe("sharded end-to-end stacks", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  const config = readFileSync(new URL("../../🎭️e2e/🎚️config/🟦️.ts", import.meta.url), "utf8");

  it("agrees with draft-07 on the authored shard table and every stored vector", () => {
    const value = { ...fixtures, table: SHARD_TABLE };
    expect(validate(value), JSON.stringify(validate.errors)).toBe(true);
    expect(validateJsonSchemaSubset(schema, value)).toEqual([]);
    for (const refused of [{ ...fixtures, table: { ...SHARD_TABLE, version: 2 } }, { ...fixtures, table: { ...SHARD_TABLE, extra: true } }, { ...fixtures, allocations: [{ ...fixtures.allocations[0], budget: 0 }] }, { ...fixtures, schedules: [{ ...fixtures.schedules[0], topologies: ["staging"] }] }])
      expect(validateJsonSchemaSubset(schema, refused).length === 0, JSON.stringify(refused).slice(0, 60)).toBe(validate(refused));
  });

  it("places every project of the Playwright configuration in exactly one shard, and lets a project depend only on its own shard", () => {
    const projects = [...config.matchAll(/\{ name: "(\w+)"(?:, dependencies: \[([^\]]*)\])?/gu)].map(([, name, dependencies]) => ({ name: name!, dependencies: (dependencies ?? "").split(",").map(word => word.trim().replaceAll('"', "")).filter(Boolean) }));
    expect(projects.map(project => project.name).sort()).toEqual([...SHARD_TABLE.shards.flatMap(shard => shard.projects), SHARD_TABLE.warmup].sort());
    for (const shard of SHARD_TABLE.shards) for (const name of shard.projects) for (const dependency of projects.find(project => project.name === name)!.dependencies) expect(shard.projects, `${name} depends on ${dependency}`).toContain(dependency);
    expect(projects.find(project => project.name === SHARD_TABLE.warmup)!.dependencies).toEqual([]);
    for (const layout of Object.values(SHARD_TABLE.stacks)) expect(layout.flat().sort()).toEqual(SHARD_TABLE.shards.map(shard => shard.id).sort());
    expect(SHARD_TABLE.shards.filter(shard => !shard.parallel).flatMap(shard => shard.projects).every(name => config.includes(`name: "${name}"`))).toBe(true);
  });

  it("matches project patterns the way Playwright does and cuts the project and worker selection from the forwarded arguments", () => {
    for (const row of fixtures.matches) expect(projectMatchesV1(row.pattern, row.project), `${row.pattern} ${row.project}`).toBe(row.matches);
    for (const row of fixtures.arguments) {
      expect(splitProjectArgumentsV1(row.args)).toEqual({ projects: row.projects, rest: splitProjectArgumentsV1(row.args).rest });
      expect(splitWorkerArgumentsV1(splitProjectArgumentsV1(row.args).rest)).toEqual({ workers: row.workers ?? undefined, rest: row.rest });
    }
  });

  it("lays the projects over the stacks of a topology and the stacks over a run as the independent oracle does", () => {
    for (const row of fixtures.stacks) expect(planStacksV1(SHARD_TABLE, row.topology, row.requested, row.split), JSON.stringify(row).slice(0, 80)).toEqual(row.stacks.map(stack => ({ topology: row.topology, ...stack })));
    for (const row of fixtures.schedules) {
      const schedule = scheduleStacksV1(SHARD_TABLE, row.topologies, row.requested, row.granted, row.serial);
      expect({ concurrent: schedule.concurrent, labels: schedule.stacks.map(stack => stack.label) }, JSON.stringify(row).slice(0, 80)).toEqual({ concurrent: row.concurrent, labels: row.labels });
    }
    expect(() => planStacksV1(SHARD_TABLE, "staging", [], true)).toThrow("no stacks");
  });

  it("never runs a wave that needs its proctor alone on more than one worker and never hands out more workers than the budget holds", () => {
    for (const row of fixtures.allocations) {
      const workers = allocateWorkersV1(row.parallel.map(waves => ({ waves: waves.map(parallel => ({ parallel })) })), row.budget, row.concurrent);
      expect(workers, JSON.stringify(row).slice(0, 80)).toEqual(row.workers);
      row.parallel.forEach((waves, stack) => waves.forEach((parallel, wave) => parallel || expect(workers[stack]![wave]).toBe(1)));
      if (row.concurrent && row.budget >= row.parallel.length) expect(workers.reduce((total, stack) => total + Math.max(...stack), 0)).toBeLessThanOrEqual(row.budget);
    }
    for (const row of fixtures.budgets) expect(workerBudgetV1({ workers: row.workers, memoryMegabytes: row.memoryMegabytes, workerMegabytes: row.workerMegabytes }, row.stacksMegabytes), JSON.stringify(row)).toBe(row.budget);
    for (const row of fixtures.waves) expect(waveArgumentsV1(row.forwarded, row.wave, row.workers, row.explicit ?? undefined)).toEqual(row.args);
  });

  it("boots two dev stacks beside a rehearsal stack that runs its shards in turn, each stack once", () => {
    const { stacks, concurrent } = scheduleStacksV1(SHARD_TABLE, ["rehearsal", "dev"], [], 3, false);
    expect(concurrent).toBe(true);
    expect(stacks.map(stack => [stack.label, stack.waves.map(wave => wave.shard)])).toEqual([["rehearsal", ["shared", "exclusive"]], ["dev-shared", ["shared"]], ["dev-exclusive", ["exclusive"]]]);
    expect(stacks.flatMap(stack => stack.waves).flatMap(wave => wave.projects).filter(project => project === "faults")).toHaveLength(2);
  });
});
