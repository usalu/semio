/** 🪜️ The preparations that are not pinned tools: the Node native core is a row whose verification asks the prebuild script, the compiler helpers are a step that runs only when the whole table is set up, an optional step never fails the setup, a required one does, and both stop when cancelled; every text exists in both languages. */
import {describe, expect, setDefaultTimeout, test} from "bun:test";
import Ajv from "ajv";
import {existsSync} from "node:fs";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {TOOL_SETUP_V1, setupToolsV1, toolRowV1, toolRowsV1, toolSetupMessageV1, toolStepSummaryV1, toolStepsV1, verifyToolV1, warmToolStepsV1, type ToolContextV1, type ToolStepV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type: "json"};

setDefaultTimeout(60000);
const root = join(import.meta.dir, "../../../../../../../../.."), signal = AbortSignal.timeout(30000), step = TOOL_SETUP_V1.steps[0]!;
const copy = (patch: Partial<Record<string, unknown>>): ToolStepV1 => ({...step, ...patch}) as ToolStepV1;
const machine = (hook: ToolContextV1["hook"]): ToolContextV1 => Object.freeze({
  root: "/repo", toolsRoot: "/repo/.cache/b/t/m", platform: "linux" as NodeJS.Platform, environment: {SEMIO_LANGUAGE: "en", PATH: "/usr/bin"}, home: "/home/dev",
  read: () => "{\"definitions\":{\"Protocol\":{\"const\":{\"version\":7}}}}", exists: () => false, list: () => [], resolvePackage: () => "/x", capture: async () => "", provisioned: () => undefined, hook,
});

describe("the Node native core row", () => {
  test("follows the Rust toolchain in the group order and asks the prebuild script that lives in the runtime", () => {
    const ids = TOOL_SETUP_V1.groups.all, row = toolRowV1("node-native-core");
    expect(ids.indexOf("node-native-core")).toBe(ids.indexOf("rust") + 1);
    expect(row.setup).toMatchObject({kind: "script", prepare: ["prepare"], verify: ["verify"]});
    expect(row.probe.mode).toBe("hook");
    if (row.setup.kind !== "script") throw new Error("script row required");
    expect(existsSync(join(root, row.setup.script))).toBe(true);
    expect(row.summary.en).not.toBe(row.summary.de);
  });

  test("is ok when the hook succeeds, missing when it does not, and setup runs the script once to get there", async () => {
    const row = toolRowV1("node-native-core");
    expect(await verifyToolV1(row, machine(async (_row, pin) => ({found: pin, location: "script"})), signal)).toMatchObject({status: "ok", pin: "7", found: "7"});
    expect(await verifyToolV1(row, machine(async () => ({found: null, location: null})), signal)).toMatchObject({status: "missing", pin: "7"});
    let built = false;
    const context = machine(async (_row, pin) => (built ? {found: pin, location: "script"} : {found: null, location: null})), lines: string[] = [];
    const reports = await setupToolsV1([row], context, async (provided, pin) => { expect([provided.id, pin]).toEqual(["node-native-core", "7"]); built = true; }, signal, line => lines.push(line));
    expect(reports.map(report => [report.id, report.status])).toEqual([["node-native-core", "ok"]]);
    expect(lines).toEqual(["Provisioning node-native-core 7.", "node-native-core 7 is ready."]);
    expect(toolRowsV1("all").some(candidate => candidate.id === "node-native-core")).toBe(true);
  });
});

describe("steps", () => {
  test("pass the strict schema and the subset validator, apply to every platform and refuse every mutation", () => {
    const validate = new Ajv({strict: true}).compile(schema);
    expect(validate(TOOL_SETUP_V1), JSON.stringify(validate.errors)).toBe(true);
    expect(toolStepsV1("darwin").map(row => row.id)).toEqual(["compiler-helpers"]);
    expect(toolStepsV1("linux")).toEqual(toolStepsV1("win32"));
    expect(toolStepsV1("freebsd" as NodeJS.Platform)).toEqual([]);
    const mutations: Record<string, (value: any) => void> = {
      "unknown kind": value => { value.steps[0].kind = "nested-repo"; },
      "missing german summary": value => { delete value.steps[0].summary.de; },
      "unknown platform": value => { value.steps[0].platforms.push("freebsd"); },
      "no timeout": value => { delete value.steps[0].timeoutSeconds; },
      "timeout below a minute": value => { value.steps[0].timeoutSeconds = 5; },
      "missing steps": value => { delete value.steps; },
      "missing warm text": value => { delete value.messages.texts.warmFailed; },
    };
    for (const [name, mutate] of Object.entries(mutations)) {
      const mutated = structuredClone(TOOL_SETUP_V1);
      mutate(mutated);
      expect({name, ajv: validate(mutated), subset: validateJsonSchemaSubset(schema, mutated, schema).length === 0}, name).toEqual({name, ajv: false, subset: false});
    }
  });

  test("keep German next to English with the same placeholders", () => {
    for (const key of ["warm", "warmed", "warmFailed"] as const) {
      const text = TOOL_SETUP_V1.messages.texts[key], holes = (value: string) => [...value.matchAll(/\{([a-z]+)\}/gu)].map(match => match[1]).sort();
      expect(holes(text.de), key).toEqual(holes(text.en));
      expect(text.de).not.toBe(text.en);
    }
    expect(toolStepSummaryV1(step, {SEMIO_LANGUAGE: "de"})).toBe(step.summary.de);
    expect(toolStepSummaryV1(step, {LANG: "C", LC_ALL: "C", LANGUAGE: ""})).toBe(`${step.summary.en} / ${step.summary.de}`);
  });

  test("an optional step that fails is reported and the setup goes on, a required one fails the setup", async () => {
    const lines: string[] = [], second = copy({id: "second"});
    const results = await warmToolStepsV1([step, second], async row => { if (row.id === step.id) throw new Error("rustc missing\nsecond line"); }, signal, line => lines.push(line), {SEMIO_LANGUAGE: "en"});
    expect(results.map(row => [row.id, row.status])).toEqual([[step.id, "failed"], ["second", "ready"]]);
    expect(lines[0]).toBe(`Preparing compiler-helpers: ${step.summary.en}`);
    expect(lines[1]).toBe("compiler-helpers could not be prepared (rustc missing); it is built on the first compile instead.");
    expect(lines.at(-1)).toBe("second is ready.");
    const german: string[] = [];
    await warmToolStepsV1([step], async () => { throw new Error("kein rustc"); }, signal, line => german.push(line), {SEMIO_LANGUAGE: "de"});
    expect(german[1]).toContain("konnte nicht vorbereitet werden (kein rustc)");
    await expect(warmToolStepsV1([copy({optional: false})], async () => { throw new Error("hard"); }, signal, () => undefined, {SEMIO_LANGUAGE: "en"})).rejects.toThrow("hard");
  });

  test("stop when cancelled, between steps and inside a step, and give every step its own deadline", async () => {
    const controller = new AbortController(), seen: AbortSignal[] = [];
    controller.abort(new Error("cancelled"));
    await expect(warmToolStepsV1([step], async () => { throw new Error("must not run"); }, controller.signal, () => undefined)).rejects.toThrow("cancelled");
    const inside = new AbortController();
    await expect(warmToolStepsV1([step], async (_row, stepSignal) => { inside.abort(new Error("cancelled")); stepSignal.throwIfAborted(); }, inside.signal, () => undefined)).rejects.toThrow("cancelled");
    const quick = copy({timeoutSeconds: 0.05});
    const results = await warmToolStepsV1([quick], async (_row, stepSignal) => { seen.push(stepSignal); await new Promise<void>((resolve, reject) => { stepSignal.addEventListener("abort", () => reject(stepSignal.reason), {once: true}); setTimeout(resolve, 2000); }); }, signal, () => undefined, {SEMIO_LANGUAGE: "en"});
    expect(results[0]).toMatchObject({id: step.id, status: "failed"});
    expect(seen[0]!.aborted).toBe(true);
  });

  test("render their start line in every language when none resolves", async () => {
    const lines: string[] = [];
    await warmToolStepsV1([step], async () => undefined, signal, line => lines.push(line), {LANG: "C", LC_ALL: "C", LANGUAGE: ""});
    expect(lines[0]).toBe(toolSetupMessageV1("warm", {step: step.id, summary: toolStepSummaryV1(step, {LANG: "C", LC_ALL: "C", LANGUAGE: ""})}, {LANG: "C", LC_ALL: "C", LANGUAGE: ""}));
    expect(lines[0]).toContain(" / ");
  });
});
