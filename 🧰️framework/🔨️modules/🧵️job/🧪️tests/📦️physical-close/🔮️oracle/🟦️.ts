import { test } from "bun:test";
import { testJobPayloadPhysicalClose } from "../🟦️.ts";

test("refuses sub-page release and retires the exact physical extent using independent JSON Patch", () => {
  testJobPayloadPhysicalClose();
  console.log("[DEBUG] physical page retirement: four logical lengths, repeated short grants retain the exact page, full release costs16384");
});
