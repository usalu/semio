import { expect, test } from "bun:test";
import Decimal from "decimal.js";
import { readFileSync } from "node:fs";
import { toolJobMicrosecondBudgetSelfTests } from "../🔬️tool-job-microsecond-budget/🟦️.ts";
import { toolJobTelemetryContentionSelfTests } from "../../../../⏱️trace/⏱️clock/🧪️tests/🔬️tool-job-telemetry-contention/🟦️.ts";
import { traceClockContentionFixtureUrl } from "../../../../⏱️trace/🧪️tests/🧫️fixtures/🟦️.ts";

test("microsecond deadline and worker binding observations remain executable", () => {
  expect(toolJobMicrosecondBudgetSelfTests()).toBeGreaterThan(0);
});
test("telemetry contention observations agree with independent decimal arithmetic", () => {
  expect(toolJobTelemetryContentionSelfTests()).toBeGreaterThan(0);
  const rows = JSON.parse(readFileSync(traceClockContentionFixtureUrl, "utf8"));
  for (const row of rows.verdicts) {
    const fault = row.start === null || row.end === null || new Decimal(row.end).lt(row.start) || new Decimal(row.end).minus(row.start).gte(8000);
    expect(fault).toBe(row.fault);
  }
});
