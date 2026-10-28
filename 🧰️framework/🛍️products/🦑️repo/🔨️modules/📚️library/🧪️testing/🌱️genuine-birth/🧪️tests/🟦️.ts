import { expect, test } from "bun:test";
import { relative, resolve } from "node:path";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import policy from "../🔣️.json";
import { genuineTestNamePatternV1, nextGenuineBirthV1, readGenuineBirthPolicyV1 } from "../🟦️.ts";
import { createBuildBootstrapCapabilitiesV1 } from "../../../🏗️build/📥️caller/📤️child/🟦️.ts";

const root = process.cwd(), probe = relative(root, resolve(import.meta.dir, "🧫️probe/🟦️.ts")).replaceAll("\\", "/"), script = resolve(import.meta.dir, "../📜️script.ts");

const bun = (args: readonly string[], extra: Record<string, string | undefined> = {}) => {
  const environment: Record<string, string | undefined> = { ...process.env, SEMIO_GENUINE_PROBE: "1", ...extra };
  delete environment.SEMIO_SCRIPT_PROCESS_INVOCATION;
  const result = Bun.spawnSync([process.execPath, ...args], { cwd: root, env: environment as Record<string, string> });
  return { status: result.exitCode, output: result.stdout.toString() + result.stderr.toString() };
};

/** 📐️ The authored ladder is closed by an independent strict schema validator. */
test("genuine birth ladder is schema-closed and refuses duplicate or conditional first births", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(policy)).toBe(true);
  expect(validate({ ...policy, unknown: true })).toBe(false);
  expect(validate({ ...policy, births: [] })).toBe(false);
  expect(validate({ ...policy, births: [{ ...policy.births[0], preparation: "unknown" }] })).toBe(false);
  expect(policy.births.filter(row => "preparation" in row).map(row => row.id)).toEqual(["target-execution"]);
  expect(readGenuineBirthPolicyV1().births.map(row => row.id)).toEqual(policy.births.map(row => row.id));
  expect(() => readGenuineBirthPolicyV1({ ...policy, births: [policy.births[0], policy.births[0]] })).toThrow();
  expect(() => readGenuineBirthPolicyV1({ ...policy, births: [{ ...policy.births[1] }, policy.births[0]] })).toThrow();
});

/** 🪜️ Failure signatures select the next declared birth, never an undeclared one. */
test("genuine birth escalation follows the declared failure signatures", () => {
  for (const row of fixture.escalation) expect(nextGenuineBirthV1(row.previous, row.failure)?.id ?? null).toBe(row.next);
});

/** 🌱️ Every declared birth is a real repository request that the build bootstrap admits. */
test("every declared birth resolves to genuine build bootstrap capabilities", async () => {
  for (const birth of policy.births) {
    const capabilities = await createBuildBootstrapCapabilitiesV1(root, birth.arguments, { ...process.env, SEMIO_BUILD_MODE: policy.buildMode, ...birth.environment });
    expect(capabilities.arguments, birth.id).toEqual(["repo", ...birth.arguments]);
    expect(capabilities.selection, birth.id).toEqual(["repo", ...birth.arguments]);
  }
});

/** 🔤️ The name escape agrees with the fixture, a regular expression oracle and bun's own test selection. */
test("test name escape selects exactly one test in bun", () => {
  for (const row of fixture.namePatterns) {
    expect(genuineTestNamePatternV1(row.name)).toBe(row.pattern);
    expect(new RegExp(row.pattern, "u").test(row.name)).toBe(true);
    expect(new RegExp(row.pattern, "u").test(row.name + "x")).toBe(false);
  }
  const selected = bun(["test", "./" + probe, "--test-name-pattern", genuineTestNamePatternV1("probe a.b(c)+[d]")]);
  expect(selected.status).toBe(0);
  expect(selected.output).toMatch(/^\s*1 pass$/mu);
  expect(selected.output).not.toMatch(/^\s*[1-9]\d* fail$/mu);
}, 60000);

/** 🧪️ The harness gives a test file the genuine envelope with the declared request. */
test("harness runs a file beneath the declared genuine birth", () => {
  for (const row of fixture.executed) {
    const result = bun([script, "--file=" + probe, "--birth=" + row.birth, "--pattern=" + row.pattern], { SEMIO_WORKSPACE_ROOT: root });
    expect(result.status, result.output.slice(-3000)).toBe(0);
    expect(result.output).toMatch(new RegExp("^\\s*" + row.pass + " pass$", "mu"));
  }
}, 240000);
