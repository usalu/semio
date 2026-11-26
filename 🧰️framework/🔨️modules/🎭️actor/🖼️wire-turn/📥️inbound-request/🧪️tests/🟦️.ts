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

test("preserves a paid request until its answer and bounds stalled, replayed, and explicitly capped turns", async () => {
  const laws = (await import("./🔣️paid-progress.json")).default;
  for (const law of laws) {
    let calls = 0;
    const answer = await driveInboundRequest({
      req: 9n, capability: "mesh", payload: new Uint8Array(), originInstanceId: 1,
      turnBudget: law.case === "explicit" ? 64 : undefined,
      submit: async () => {
        calls++;
        const fuel = BigInt(1000 - (law.case === "replayed" ? 1 : calls));
        const paid = law.case !== "parked" && (law.case !== "ends" || calls <= 6);
        const retainedReceipt = {input:{operation:1n,generation:1n},remainingFuel:fuel,workUnitsSpent:1n,spent:{copiedItems:paid ? 1n : 0n}};
        const effects = calls === law.answer ? [{tag:"respond",val:{req:9n,outcome:{tag:"ok",val:new Uint8Array([61])}}}] : [];
        return { effects, status:{tag:"more-work"}, original:{retainedReceipt} };
      },
    });
    expect([answer.status,answer.turns,calls], law.case).toEqual([law.status,law.turns,law.turns]);
  }
});
