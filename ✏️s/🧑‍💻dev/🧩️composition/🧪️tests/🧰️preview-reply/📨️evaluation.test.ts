import { expect, test } from "bun:test";
import { z } from "zod";
import fixture from "./📨️evaluation.json";

const reply = z.strictObject({
  done: z.boolean(),
  cancellable: z.boolean(),
  phase: z.string().min(1).max(64),
  unitsDone: z.number().int().nonnegative(),
  unitsTotal: z.number().int().nonnegative(),
  output: z.nullable(z.record(z.string(), z.unknown())),
}).refine(row => {
  const complete = row.phase === "complete";
  const terminal = complete || row.phase === "cancelled";
  return row.done === terminal && row.cancellable !== terminal && row.unitsDone <= row.unitsTotal &&
    (complete ? row.unitsDone === row.unitsTotal && row.output !== null : row.output === null);
});

test("the named evaluation rows satisfy the independent typed response schema", () => {
  for (const row of fixture.rows) {
    const item = reply.parse(row.reply);
    expect(item.output !== null).toBe(item.phase === "complete");
    if (!item.done) {
      expect(row.status.inFlight).toBe(1);
      expect(row.status.ratio).toBe(item.unitsDone / item.unitsTotal);
    }
    expect(row.owesHop).toBe(item.phase !== "cancelled");
  }
});

test("bare dictionaries, textual output and inconsistent headers fail the typed response schema", () => {
  for (const row of fixture.refused) expect(reply.safeParse(row).success).toBe(false);
});
