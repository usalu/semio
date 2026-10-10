import { expect, test } from "bun:test";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import corpus from "../🧫️fixtures/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/🟦️.ts";
import * as owned from "../../🟦️.ts";

test("actual owned command failures retain canonical terminal outcomes against independent Node close", async () => {
  const started = performance.now(), controller = new AbortController(), deadline = setTimeout(() => controller.abort(), 15000);
  const admits = new Ajv({ strict: true }).compile(schema);
  for (const row of corpus.cases) { expect(admits(row.value), row.id).toBe(row.accepted); expect(validateJsonSchemaSubset(schema, row.value).length === 0, row.id).toBe(row.accepted); }
  const child = 'const row=JSON.parse(process.argv[1]);console.log("[DEBUG] original "+row.id);console.error("[DEBUG] original stderr "+row.id);row.signal?process.kill(process.pid,row.signal):process.exit(row.status)';
  const oracleCode = 'const{spawn}=require("node:child_process"),row=JSON.parse(process.argv[1]);const child=spawn(process.execPath,["--eval",process.argv[2],JSON.stringify(row)],{stdio:["ignore","ignore","ignore"]});child.once("close",(status,signal)=>console.log(JSON.stringify({version:1,status,signal})))';
  const observations: { id: string; outcome: unknown; failed: boolean; failure: string; expected: unknown; lines: string[] }[] = [];
  try {
    for (const row of corpus.receivers) {
      const oracle = Bun.spawn(["node", "--eval", oracleCode, JSON.stringify(row), child], { stdout: "pipe", stderr: "pipe" });
      const lines: string[] = []; let failed = false, failure = "", outcome: unknown;
      try { await owned.runOwnedCommand("node", ["--eval", child, JSON.stringify(row)], process.cwd(), "original-outcome", 15000 - (performance.now() - started), { signal: controller.signal, onLine: line => { lines.push(line); } }); }
      catch (error) { failed = true; failure = String(error); outcome = typeof error === "object" && error !== null && "outcome" in error ? error.outcome : undefined; }
      const [stdout, stderr, status] = await Promise.all([new Response(oracle.stdout).text(), new Response(oracle.stderr).text(), oracle.exited]);
      expect(status, stderr).toBe(0); const expected = JSON.parse(stdout);
      expect(admits(expected)).toBe(true); expect(expected).toEqual({ version: 1, status: row.status, signal: row.signal });
      observations.push({ id: row.id, outcome, failed, failure, expected, lines });
    }
    console.log("[DEBUG] " + JSON.stringify(observations));
    const reader = Reflect.get(owned, "readOwnedCommandOutcome");
    expect(typeof reader, "Canonical first-party outcome reader required").toBe("function");
    for (const row of corpus.cases) { if (row.accepted) expect(reader(row.value)).toBe(row.value); else expect(() => reader(row.value)).toThrow(); }
    for (const row of observations) {
      expect(row.lines).toContain("[DEBUG] original " + row.id); expect(row.lines).toContain("[DEBUG] original stderr " + row.id);
      expect(row.failed).toBe(row.id !== "complete");
      if (row.failed) { expect(admits(row.outcome), row.id).toBe(true); expect(row.outcome).toEqual(row.expected); }
    }
  } finally { clearTimeout(deadline); }
}, 15000);
