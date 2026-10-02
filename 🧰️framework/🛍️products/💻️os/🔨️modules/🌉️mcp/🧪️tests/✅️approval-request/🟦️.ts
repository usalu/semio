/** 🧱️ Retains the fixture law at its concrete product contract owner. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";

const root = resolve(import.meta.dir, "../../../../../../..");
const fixture = <T = Record<string, unknown>>(path: string): T => JSON.parse(readFileSync(resolve(root, path), "utf8"));

test("approval intents are closed under the framework-owned contract", () => {
  const schema = fixture("🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/✅️approval-request/🔣️.json");
  const validate = new Ajv().compile(schema);
  const intent = { schema: "semio.hub.inference-approval/v1", version: 1, jobId: "ab".repeat(16), proposalHash: "cd".repeat(32) };
  expect(validate(intent)).toBe(true);
  expect(validate({ ...intent, proposal: "private bytes" })).toBe(false);
  expect(validate({ ...intent, jobId: "invalid" })).toBe(false);
});
