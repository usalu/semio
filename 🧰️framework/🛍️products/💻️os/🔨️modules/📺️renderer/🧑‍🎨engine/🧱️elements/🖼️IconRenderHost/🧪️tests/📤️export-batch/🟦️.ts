/** 📤️ Ajv validates neutral icon export admission and failure isolation. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020";
import { describe, expect, it } from "vitest";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const read = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
const fixture = read("🧫️fixtures/📤️export-batch/🔣️.json");
const queueCases = fixture.queueCases as readonly { name: string; groups: readonly number[]; cancelBeforeGroup: number | null; expected: { admitted: number; failed: number; discarded: number; phase: string } }[];

describe("Icon export batch admission", () => {
  it("accepts the language-neutral lifecycle fixture", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(read("🧬️schema/📤️export-batch/🔣️.json"));
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });
  it("rejects each invalid request independently using the schema oracle", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(read("🧬️schema/📤️export-request/🔣️.json"));
    const accepted = fixture.invalid.filter((item: { request: unknown }) => validate(item.request));
    expect(accepted).toHaveLength(fixture.expected.completed);
    expect(fixture.invalid.length - accepted.length).toBe(fixture.expected.failed);
  });
  it.each(queueCases)("preserves admission and cancellation scope: $name", async (scenario) => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(read("🧬️schema/📤️export-request/🔣️.json"));
    let controller = new AbortController();
    const requests: Array<{ request: unknown; signal: AbortSignal }> = [];
    for (let index = 0; index <= scenario.groups.length; index++) {
      if (scenario.cancelBeforeGroup === index) controller.abort();
      if (index === scenario.groups.length) break;
      if (controller.signal.aborted) controller = new AbortController();
      for (let item = 0; item < scenario.groups[index]; item++) requests.push({ request: fixture.invalid[0].request, signal: controller.signal });
    }
    const outcomes = await Promise.allSettled(requests.map(async ({ request, signal }) => {
      signal.throwIfAborted();
      if (!validate(request)) throw new Error("invalid export request");
    }));
    const discarded = outcomes.filter((outcome) => outcome.status === "rejected" && outcome.reason.name === "AbortError").length;
    const failed = outcomes.filter((outcome) => outcome.status === "rejected").length - discarded;
    expect({ admitted: requests.length, failed, discarded, phase: discarded === requests.length ? "cancelled" : "failed" }).toEqual(scenario.expected);
  });
});
