/** 🧪️ Recomputes plain inference cases with independent selection, crypto and state validators. */
import { expect, test } from "vitest";
import { resolve } from "node:path";
import { proveInferenceServiceLaw } from "../../../🌉️mcp/💡️inference/🧪️tests/💼️inference-service-law/🟦️.ts";

test("plain inference cases match independent selection, SHA-256 and page-state oracles", () => {
  const result = proveInferenceServiceLaw(resolve(import.meta.dirname, "../../../../../../.."));
  expect(result.ajv).toBe(2);
  expect(result.selection).toBeGreaterThan(0);
  expect(result.proposals).toBeGreaterThan(0);
  expect(result.lifecycles).toBeGreaterThan(0);
});
