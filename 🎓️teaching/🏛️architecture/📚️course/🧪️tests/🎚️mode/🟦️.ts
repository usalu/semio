import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { courseCompilerModeV1, courseModeRefusalMessageV1, courseOriginalBuildModeV1 } from "../../🏗️builder/🎚️mode/🟦️.ts";
import messages from "../../🏗️builder/🎚️mode/🔣️.json" with { type: "json" };
import schema from "../../🏗️builder/🎚️mode/🧬️schema/🔣️.json" with { type: "json" };
import fixtures from "../../🏗️builder/🎚️mode/🧫️fixtures/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { createNativeBuildCallerCapabilitiesV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts";
import { createScriptProcessEnvelope, withScriptProcessEnvelope } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const ajv = new Ajv({ strict: true, allErrors: true });
const german = /erfordern|wähle|Kurs-Absicht|erforderlich/u;

function outcome(intent: unknown, originalMode: unknown): string {
  try { return `allowed:${courseCompilerModeV1(intent, originalMode)}`; } catch (error) {
    const message = (error as Error).message;
    const code = (Object.keys(messages.refusals) as (keyof typeof messages.refusals)[]).find(name => courseModeRefusalMessageV1(name) === message);
    if (!code) throw error;
    return `refused:${code}`;
  }
}

describe("course compiler mode: one global invocation axis, honored identically under Nx and Bazel", () => {
  it("keeps the fixture, the message table and the request closed under two independent schema validators", () => {
    expect(ajv.validate({ ...schema.definitions.Fixture, definitions: schema.definitions }, fixtures)).toBe(true);
    expect(ajv.validate({ ...schema.definitions.Messages, definitions: schema.definitions }, messages)).toBe(true);
    expect(validateJsonSchemaSubset(schema.definitions.Messages, messages, schema)).toEqual([]);
    for (const row of fixtures.decisions) {
      expect(ajv.validate(schema, { intent: row.intent, originalMode: row.originalMode })).toBe(true);
      expect(validateJsonSchemaSubset(schema, { intent: row.intent, originalMode: row.originalMode })).toEqual([]);
    }
    for (const row of fixtures.malformed) {
      expect(ajv.validate(schema, row)).toBe(false);
      expect(validateJsonSchemaSubset(schema, row).length).toBeGreaterThan(0);
    }
  });

  for (const row of fixtures.decisions) it(`decides ${row.id} as the fixture and as the declarative oracle`, () => {
    const actual = outcome(row.intent, row.originalMode);
    expect(actual).toBe(row.expected);
    expect(ajv.validate(fixtures.oracle, { intent: row.intent, originalMode: row.originalMode, outcome: actual })).toBe(true);
    expect(ajv.validate(fixtures.oracle, { intent: row.intent, originalMode: row.originalMode, outcome: row.expected === "refused:release-required" ? "allowed:release" : "refused:release-required" })).toBe(false);
  });

  for (const [index, row] of fixtures.malformed.entries()) it(`refuses malformed request ${index} in English and German`, () => {
    expect(() => courseCompilerModeV1((row as { intent?: unknown }).intent, (row as { originalMode?: unknown }).originalMode)).toThrow(courseModeRefusalMessageV1("closed-request"));
  });

  for (const code of Object.keys(messages.refusals) as (keyof typeof messages.refusals)[]) it(`says ${code} in English first and German second`, () => {
    const text = courseModeRefusalMessageV1(code), [english, deutsch, ...rest] = text.split("\n");
    expect(rest).toEqual([]);
    expect(english).toBe(messages.refusals[code].en);
    expect(deutsch).toBe(messages.refusals[code].de);
    expect(english).not.toBe(deutsch);
    expect(deutsch).toMatch(german);
    expect(english).not.toMatch(german);
  });

  it("names every route that selects release, for both build tools", () => {
    const text = courseModeRefusalMessageV1("release-required");
    for (const route of ["--configuration=release", "-c release", ":release", "SEMIO_BUILD_MODE=release", "--tool=nx", "--tool=bazel"]) expect(text).toContain(route);
    expect(messages.refusals["release-required"].en).toContain("require the original release build mode");
  });

  for (const tool of ["nx", "bazel"] as const) for (const mode of ["prototype", "release"] as const) for (const intent of ["dev", "release"] as const) it(`reads the original ${mode} mode of a ${tool} caller for the ${intent} intent`, async () => {
    const root = process.cwd();
    const capabilities = createNativeBuildCallerCapabilitiesV1(root, ["run", "@teaching/architecture-course:publish", `--configuration=${mode}`], tool, { SEMIO_BUILD_MODE: mode });
    await withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "course:mode-axis-test", maximumElapsedMilliseconds: 10000 }, capabilities, Date.now()), async invocation => {
      const expected = fixtures.decisions.find(row => row.intent === intent && row.originalMode === mode)!.expected;
      if (expected.startsWith("refused:")) expect(() => courseOriginalBuildModeV1(invocation, intent)).toThrow(courseModeRefusalMessageV1("release-required"));
      else expect(courseOriginalBuildModeV1(invocation, intent)).toBe(expected.slice("allowed:".length));
    });
  });
});
