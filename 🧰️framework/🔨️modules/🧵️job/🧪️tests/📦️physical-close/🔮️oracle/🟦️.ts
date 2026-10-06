import { test } from "bun:test";
import { testJobPayloadPhysicalClose } from "../🟦️.ts";

test("retains every granted byte before physical page retirement using independent JSON Patch", () => {
  testJobPayloadPhysicalClose();
  console.log("[DEBUG] physical page retirement: four example lengths, staged grant charged before release");
});
