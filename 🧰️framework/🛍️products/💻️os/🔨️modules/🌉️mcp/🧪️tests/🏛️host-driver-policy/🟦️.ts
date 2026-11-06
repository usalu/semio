import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { MCP_HOST_DRIVER_POLICY_PATH, MCP_HOST_OPERATION_POLICY_PATH, withMcpHostDriverPolicy } from "../../🟦️.ts";

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
  expect(Number.isSafeInteger(policy.root.maximumLifetimeMicroseconds)).toBe(true);
  expect(policy.root.maximumLifetimeMicroseconds).toBeGreaterThan(policy.root.maximumTurnMicroseconds);
  expect(policy.root.maximumLifetimeMicroseconds).toBe(86400000000);
});

test("every launch gets the authored policy exactly once and a caller's own policy is never replaced", () => {
  expect(withMcpHostDriverPolicy(["stdio", "--folder", "x"])).toEqual(["stdio", "--folder", "x", "--host-driver-policy", MCP_HOST_DRIVER_POLICY_PATH, "--operation-policy", MCP_HOST_OPERATION_POLICY_PATH]);
  expect(withMcpHostDriverPolicy(["stdio", "--host-driver-policy", "mine.json"])).toEqual(["stdio", "--host-driver-policy", "mine.json", "--operation-policy", MCP_HOST_OPERATION_POLICY_PATH]);
});

test("cleanup, host transit and guest infrastructure reserve disjoint currency within the same finite operation",()=>{
 const request=JSON.parse(readFileSync(MCP_HOST_OPERATION_POLICY_PATH,"utf8")),contract=JSON.parse(readFileSync(new URL("../../🎚️config/🧬️schema/💡️host-operation-policy.json",import.meta.url),"utf8"));
 expect(new Ajv({strict:true}).validate(contract,request)).toBe(true);
 for(const axis of["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes"]as const)expect(request.cleanup.retained[axis]+request.transit.retained[axis]+request.infrastructure.retained[axis]).toBeLessThan(policy.operation[axis]);
 expect(request.cleanup.retained.maximumDepth).toBeLessThanOrEqual(policy.operation.maximumDepth);
 expect(request.cleanup.workFuel+request.transit.workFuel+request.infrastructure.workFuel).toBeLessThan(policy.root.operationFuel);
 expect(request.transit.retained.maximumDepth).toBeLessThanOrEqual(policy.operation.maximumDepth);
 expect(request.infrastructure.retained.maximumDepth).toBeLessThanOrEqual(policy.operation.maximumDepth);
 const noInfrastructure={...request};delete noInfrastructure.infrastructure;expect(new Ajv({strict:true}).validate(contract,noInfrastructure)).toBe(false);
 expect(request.cleanup.executionFuel).toBeLessThan(policy.root.operationExecutionFuel);
 expect(request.cleanup.maximumMicroseconds).toBeLessThan(policy.root.maximumTurnMicroseconds);
 const args=withMcpHostDriverPolicy(["stdio","--operation-policy","caller-cleanup.json"]);
 expect(args.filter(value=>value==="--operation-policy")).toHaveLength(1);
 expect(args[args.indexOf("--operation-policy")+1]).toBe("caller-cleanup.json");
});

test("administrative funding is required before workspace and actor construction",()=>{
 const text=(path:string)=>readFileSync(new URL("../../"+path,import.meta.url),"utf8"),port=text("🔀️dispatch/🔌️channel-port/🦀️.rs"),workspace=text("🏠️workspace/🪪️headless-workspace-state/🦀️.rs"),open=text("🏠️workspace/🛂️gateway-backend/🏗️open-and-discovery/🦀️.rs"),channel=text("🏠️workspace/📨️plugin-artifact-channel/🧱️channel-record/🦀️.rs");
 const infer=port.slice(port.indexOf("pub struct InferCommand"),port.indexOf("pub struct InferenceCancel"));
 expect(infer).not.toContain("operation_policy");
 expect(workspace).not.toContain("OnceLock<semio_os_mcp_port::channel_port::HostOperationPolicy>");
 expect(workspace).toContain("pub operation_policy: semio_os_mcp_port::channel_port::HostOperationPolicy");
 expect(open).toContain("operation_policy,");
 expect(channel).toContain("pub operation_policy:semio_os_mcp_port::channel_port::HostOperationPolicy");
});
