import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { driveInboundRequest } from "../🟦️.ts";
import cases from "./🔣️.json";

test("a cancelled geometry request never enters another guest turn", async () => {
  const db = new Database(":memory:");
  try {
    const visits = db.query("WITH RECURSIVE visits(n) AS (VALUES(0) UNION ALL SELECT n + 1 FROM visits WHERE n < ?) SELECT MAX(n) AS turns FROM visits");
    for (const law of cases) {
      const control = new AbortController();
      let calls = 0;
      if (law.cancelAfterTurns === 0) control.abort();
      const result = await driveInboundRequest({
        req:1n, capability:"evaluate", payload:new Uint8Array(), originInstanceId:1, signal:control.signal,
        submit:async () => { calls++; return {effects:[],status:{tag:"more-work"}}; },
        onProgress:({turns}) => { if (turns === law.cancelAfterTurns) control.abort(); }
      });
      expect(visits.get(law.cancelAfterTurns)).toEqual({turns:law.expectedTurns});
      expect(result).toEqual({status:"cancelled",turns:law.expectedTurns});
      expect(calls).toBe(law.expectedTurns);
    }
  } finally { db.close(); }
});
