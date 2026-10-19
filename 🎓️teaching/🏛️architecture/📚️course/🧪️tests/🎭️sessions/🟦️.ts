import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import schema from "../../🎭️e2e/🔒️sessions/🧬️schema/🔣️.json" with { type: "json" };
import fixtures from "../../🎭️e2e/🔒️sessions/🧫️fixtures/🔣️.json" with { type: "json" };
import { runTopologySessionsV1 } from "../../🎭️e2e/🔒️sessions/🟦️.ts";
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
