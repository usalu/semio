import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import request from "../../🏠️workspace/📨️plugin-artifact-channel/💰️budgets-and-limits/implicit-editor-request.json";
import schema from "../../🏠️workspace/📨️plugin-artifact-channel/💰️budgets-and-limits/implicit-editor-request.schema.json";
import operation from "../../🎚️config/💡️host-operation-policy.json";
import driver from "../../🎚️config/🏛️host-driver-policy.json";

test("implicit editor requests leave host work, copy, storage and item currency available during guest execution", () => {
  expect(new Ajv().validate(schema, request)).toBe(true);
  for (const axis of ["maximumItems", "maximumCopyBytes", "maximumCapacityBytes", "maximumReleaseBytes"] as const) {
    const held = request.retained[axis] + operation.cleanup.retained[axis] + operation.transit.retained[axis] + operation.infrastructure.retained[axis];
    expect(held + driver.drive[axis]).toBeLessThan(driver.operation[axis]);
  }
  expect(request.workUnits + operation.cleanup.workFuel + operation.transit.workFuel + operation.infrastructure.workFuel + driver.root.turnFuel).toBeLessThan(driver.root.operationFuel);
  const channel = join(import.meta.dir, "../../🏠️workspace/📨️plugin-artifact-channel");
  for (const path of ["🔁️native-poll", "🧫️initial-document"]) {
    expect(readFileSync(join(channel, path, "🦀️.rs"), "utf8")).toContain("implicit_editor_request(");
  }
});
