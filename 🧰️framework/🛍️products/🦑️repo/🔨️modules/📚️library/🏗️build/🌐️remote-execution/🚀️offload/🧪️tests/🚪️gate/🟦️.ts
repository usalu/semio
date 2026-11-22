import { expect, test } from "bun:test";
import { OFFLOAD_POLICY_V1 } from "../../📐️policy/🟦️.ts";
import { offloadGateV1 } from "../../🚦️admission/🟦️.ts";
import vectors from "./🔣️.json" with { type: "json" };

test("the offload gate follows the language-neutral vectors, asking only the signals it needs", async () => {
  expect(vectors.queueThresholdMilliseconds).toBe(OFFLOAD_POLICY_V1.limits.queueThresholdMilliseconds);
  for (const row of vectors.cases) {
    const asked: string[] = [];
    const answer = await offloadGateV1({
      mode: row.mode as "distribute" | "pin" | "never", door: row.door,
      worth: () => { asked.push("worth"); return row.worth; },
      slotQueueMilliseconds: () => { asked.push("slot"); return row.slotQueueMilliseconds; },
      host: async () => { asked.push("host"); return row.host as "admit" | "wait"; },
      broker: async () => { asked.push("broker"); return row.broker as "admit" | "wait" | "unknown"; },
    });
    expect(answer, row.name).toEqual({ offload: row.offload, queue: row.queue, reason: row.reason as never });
    expect(asked, row.name).toEqual(row.asked);
  }
});
