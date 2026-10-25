import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { MCP_HOST_DRIVER_POLICY_PATH, withMcpHostDriverPolicy } from "../../🟦️.ts";

const schema = JSON.parse(readFileSync(new URL("../../../🔌️plugin/🖥️host/🔔️wake/📃️policy/🧬️schema/🔣️.json", import.meta.url), "utf8"));
const policy = JSON.parse(readFileSync(MCP_HOST_DRIVER_POLICY_PATH, "utf8"));

test("the authored host driver policy satisfies the closed policy schema of the wake driver under an independent validator", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(policy), JSON.stringify(validate.errors)).toBe(true);
});

test("the policy is finite and gives the root turn real time: currency is bounded, a turn is never shorter than a millisecond", () => {
  for (const axis of ["admission", "drive", "wake", "operation"] as const) for (const value of Object.values<number>(policy[axis])) expect(Number.isSafeInteger(value) && value >= 0, axis).toBe(true);
  expect(policy.operation.maximumItems).toBeGreaterThan(policy.wake.maximumItems);
  expect(policy.root.maximumTurnMicroseconds).toBeGreaterThanOrEqual(1000);
});

test("every launch gets the authored policy exactly once and a caller's own policy is never replaced", () => {
  expect(withMcpHostDriverPolicy(["stdio", "--folder", "x"])).toEqual(["stdio", "--folder", "x", "--host-driver-policy", MCP_HOST_DRIVER_POLICY_PATH]);
  expect(withMcpHostDriverPolicy(["stdio", "--host-driver-policy", "mine.json"])).toEqual(["stdio", "--host-driver-policy", "mine.json"]);
});
