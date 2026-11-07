import { expect, test } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { probeBuildPreflightFilesystemV1, runBuildPreflightPhasesV1 } from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/📋️phases.json" with { type: "json" };

test("filesystem probe failures retain identity and cannot enter managed setup", async () => {
  const validate = new Ajv({ strict: true }).compile({ ...schema, $ref: "#/definitions/FilesystemCase" });
  const database = new Database(":memory:");
  try { for (const row of fixture.filesystem) {
    expect(validate(row), row.id).toBe(true);
    const trace: string[] = [], failure = Error(row.id);
    const run = (step: string) => { trace.push(step); if (row.failure === step) throw failure; };
    let caught: unknown, allocated = false;
    try {
      const observed = probeBuildPreflightFilesystemV1("win32", "journaled", { exchange: () => run("exchange"), caseSensitive: () => { run("case"); return true; }, check: () => run("check") });
      await runBuildPreflightPhasesV1(fixture.requirements, { ...fixture.cases[0]!.facts, ...observed }, async observe => { allocated = true; observe(fixture.cases[0]!.observedTools); }, () => {});
    } catch (error) { caught = error; }
    expect(trace, row.id).toEqual(row.trace);
    const oracle = database.query("WITH events(position,event) AS (VALUES(0,'check'),(1,'exchange'),(2,'check'),(3,'case'),(4,'check')) SELECT event FROM events WHERE position<=CASE ?1 WHEN 'check' THEN 0 WHEN 'exchange' THEN 1 WHEN 'case' THEN 3 ELSE 4 END ORDER BY position").all(row.failure).map(value => (value as { event: string }).event);
    expect(trace, row.id).toEqual(oracle);
    if (row.failure === "none") expect(allocated, row.id).toBe(true);
    else { expect(caught, row.id).toBe(failure); expect(allocated, row.id).toBe(false); }
  } } finally { database.close(); }
});

test("physical admission precedes managed tool observation and completion", async () => {
  const validate = new Ajv({ strict: true }).compile({ ...schema, $ref: "#/definitions/PhaseCase" });
  const database = new Database(":memory:");
  try {
    for (const row of fixture.cases) {
      expect(validate(row), row.id).toBe(true);
      let allocations = 0, stage = "before", accepted = false, held = false, failure: unknown;
      const trace: string[] = [], cancelled = Error("cancelled");
      const check = () => { if (stage === "observation") expect(held, row.id).toBe(true); if (row.cancelAt === stage) throw cancelled; };
      try {
        await runBuildPreflightPhasesV1(fixture.requirements, row.facts, async observe => {
          allocations++;
          trace.push("acquire");
          held = true;
          stage = "observation";
          try {
            for (let index = 0; index < row.scopeCalls; index++) { trace.push("observe"); observe(row.observedTools); }
            stage = "after";
          } finally { held = false; trace.push("release"); }
        }, check);
        accepted = true;
        trace.push("complete");
      } catch (error) { failure = error; }
      const physical = database.query("SELECT json_extract(?1,'$.architecture') IN (SELECT value FROM json_each(?2,'$.architectures')) AND json_extract(?1,'$.availableBytes')>=json_extract(?2,'$.minimumAvailableBytes') AND (NOT json_extract(?2,'$.exchange') OR (json_extract(?1,'$.exchange')='supported' AND (json_extract(?1,'$.caseSensitive')='yes' OR (json_extract(?1,'$.platform')='win32' AND json_extract(?1,'$.caseSensitive')='fold-safe')))) AS ok").get(JSON.stringify(row.facts), JSON.stringify(fixture.requirements)) as { ok: number };
      const allTools = database.query("SELECT NOT EXISTS(SELECT value FROM json_each(?1,'$.tools') EXCEPT SELECT value FROM json_each(?2)) AS ok").get(JSON.stringify(fixture.requirements), JSON.stringify(row.observedTools)) as { ok: number };
      expect(allocations, row.id).toBe(row.allocations);
      expect(allocations, row.id).toBe(Number(Boolean(physical.ok) && row.cancelAt !== "before"));
      expect(accepted, row.id).toBe(row.accepted);
      expect(accepted, row.id).toBe(Boolean(physical.ok && allTools.ok && row.scopeCalls === 1 && row.cancelAt === "none"));
      expect(held, row.id).toBe(false);
      if (row.cancelAt !== "none") expect(failure, row.id).toBe(cancelled);
      const observations = allocations ? row.cancelAt === "observation" ? 1 : row.scopeCalls : 0;
      const traceOracle = database.query("WITH RECURSIVE observed(n) AS (SELECT 1 WHERE ?1>0 UNION ALL SELECT n+1 FROM observed WHERE n<?1), events(position,event) AS (SELECT 0,'acquire' WHERE ?2 UNION ALL SELECT n,'observe' FROM observed UNION ALL SELECT ?1+1,'release' WHERE ?2 UNION ALL SELECT ?1+2,'complete' WHERE ?3) SELECT event FROM events ORDER BY position").all(observations, allocations, Number(accepted)).map(row => (row as { event: string }).event);
      expect(trace, row.id).toEqual(traceOracle);
    }
  } finally { database.close(); }
});

test("tool-scope failures retain their identity and escaped observations are refused", async () => {
  const facts = fixture.cases[0]!.facts, tools = fixture.cases[0]!.observedTools;
  for (const afterObservation of [false, true]) {
    const failure = Error("tool scope failed");
    let released = false, actual: unknown;
    try {
      await runBuildPreflightPhasesV1(fixture.requirements, facts, async observe => {
        try { if (afterObservation) observe(tools); throw failure; } finally { released = true; }
      }, () => {});
    } catch (error) { actual = error; }
    expect(actual).toBe(failure);
    expect(released).toBe(true);
  }
  let escaped: ((tools: readonly string[]) => void) | undefined;
  await runBuildPreflightPhasesV1(fixture.requirements, facts, async observe => { escaped = observe; observe(tools); }, () => {});
  expect(() => escaped!(tools)).toThrow("One live preflight tool observation required");
  for (const deliveries of [[tools, ["rust"]], [["rust"], tools]]) {
    await expect(runBuildPreflightPhasesV1(fixture.requirements, facts, async observe => { for (const offered of deliveries) try { observe(offered); } catch {} }, () => {})).rejects.toThrow();
  }
});
