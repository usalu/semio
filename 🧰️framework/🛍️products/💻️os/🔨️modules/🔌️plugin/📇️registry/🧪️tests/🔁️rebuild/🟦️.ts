/** 🧪️ Portable rebuild composition and schema validation against Ajv. */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "bun:test";
import Ajv from "ajv";
import { composeRebuildPlansV1, parseRebuildPlanV1, readRebuildPlanV1, selectRebuildSteps, REBUILD_STAGES, guestFrameworkCheckArgs, RebuildPlanCheckScript, RebuildAllScript } from "../../🔁️rebuild/🟦️.ts";
import schema from "../../🔁️rebuild/🧬️schema/🔣️.json";
import fixtures from "../🧫️fixtures/🔁️rebuild/🔣️.json";
const validate = new Ajv({ strict: true, strictTuples: false, allErrors: true }).compile(schema);

describe("owner-authored rebuild composition", () => {
  for (const vector of fixtures.cases) it(vector.id, () => {
    for (const plan of vector.plans) expect(validate(plan), JSON.stringify(validate.errors)).toBe(true);
    const plans = vector.plans.map(parseRebuildPlanV1);
    const composed = composeRebuildPlansV1(plans);
    const oracle = vector.plans.flatMap(plan => plan.steps).sort((a, b) => schema.$defs.RebuildStepV1.properties.stage.enum.indexOf(a.stage) - schema.$defs.RebuildStepV1.properties.stage.enum.indexOf(b.stage));
    expect(JSON.stringify(composed.steps)).toBe(JSON.stringify(oracle));
    expect(composed.steps.map(step => step.id)).toEqual(vector.expected);
    expect(validate(composed), JSON.stringify(validate.errors)).toBe(true);
  });

  it("neutral defaults contain no concrete guest or Hub inputs", () => {
    const plan = readRebuildPlanV1();
    expect(plan.steps.map(step => step.id)).toEqual(["provenance", "mutation-authority", "schema-index", "guest-framework", "describe", "generate", "check"]);
    expect(JSON.stringify(plan)).not.toMatch(/semio-s-|✏️s|os-hub|activate-s|verify-s/);
    expect(schema.$defs.RebuildStepV1.properties.stage.enum).toEqual([...REBUILD_STAGES]);
    expect(guestFrameworkCheckArgs(plan.guestFrameworkChecks[0]!).slice(0, 6)).toEqual(["check", "--manifest-path", "Cargo.toml", "--lib", "--target", "wasm32-wasip2"]);
  });

  it("selects contiguous ranges and refuses unknown, repeated or inverted bounds", () => {
    const steps = composeRebuildPlansV1(fixtures.cases[1]!.plans.map(parseRebuildPlanV1)).steps;
    expect(selectRebuildSteps(steps, ["--from", "describe", "--to", "guest"]).map(step => step.id)).toEqual(["describe", "check", "guest"]);
    for (const args of [["--from", "check", "--to", "describe"], ["--to", "missing"], ["--to", "check", "--to", "check"], ["--from"]]) expect(() => selectRebuildSteps(steps, args)).toThrow();
    expect(() => composeRebuildPlansV1([parseRebuildPlanV1(fixtures.cases[0]!.plans[0]), parseRebuildPlanV1(fixtures.cases[0]!.plans[0])])).toThrow(/duplicated/);
  });

  it("matches an independent schema oracle on malformed plans and portable inputs", () => {
    const plan = fixtures.cases[0]!.plans[0]!;
    const valid = { target: "wasm32-wasip2", workspace: "Cargo.toml", packages: ["framework"], features: [] };
    const cases = [plan, { ...plan, extra: true }, { ...plan, steps: [] }, { ...plan, steps: [{ ...plan.steps[0], id: "UPPER" }] }, { ...plan, steps: [{ ...plan.steps[0], command: ["cargo", "nx"] }] }, { ...plan, steps: [{ ...plan.steps[0], command: ["nx"] }] }, { ...plan, steps: [{ ...plan.steps[0], command: ["nx", ""] }] }, ...["Cargo.toml", "specific/Cargo.toml", "../Cargo.toml", "/tmp/Cargo.toml", "x\\Cargo.toml", "C:/Cargo.toml", "a//Cargo.toml", "a/../Cargo.toml"].map(workspace => ({ ...plan, guestFrameworkChecks: [{ ...valid, workspace }] }))];
    for (const value of cases) {
      let accepted = true;
      try { parseRebuildPlanV1(value); } catch { accepted = false; }
      expect(accepted, JSON.stringify(value)).toBe(validate(value));
    }
    expect(() => parseRebuildPlanV1({ ...plan, steps: [...plan.steps].reverse() })).toThrow(/stage order/);
    expect(() => parseRebuildPlanV1({ ...plan, guestFrameworkChecks: [{ ...valid, features: ["other/sync"] }] })).toThrow(/crate-qualified/);
  });

  it("observes the neutral runner plan through real runtime output", () => {
    const controller = new AbortController();
    const invocation = { policy: { version: 1 as const, owner: "registry-rebuild-test", maximumElapsedMilliseconds: 0 }, capabilities: {}, control: { signal: controller.signal, remainingMilliseconds: () => null, publish: () => {}, yieldContinuation: async () => {} } };
    const script = new RebuildPlanCheckScript(process.cwd(), process.cwd(), invocation);
    script.run([]);
    expect(() => script.run(["unexpected"])).toThrow(/accepts no arguments/);
  });

  for (const vector of fixtures.execution) it(vector.id, async () => {
    const controller = new AbortController(), observed: string[] = [];
    const invocation = { policy: { version: 1 as const, owner: "registry-execution-test", maximumElapsedMilliseconds: 0 }, capabilities: {}, control: { signal: controller.signal, remainingMilliseconds: () => null, publish: () => {}, yieldContinuation: async () => {} } };
    const plan = parseRebuildPlanV1(fixtures.cases[0]!.plans[0]);
    const script = new RebuildAllScript(process.cwd(), process.cwd(), invocation, plan, {
      acquire: async signal => {
        expect(signal).toBe(controller.signal);
        observed.push("acquired");
        return { release: () => { observed.push("released"); } };
      },
      run: async (step, signal, budgetMilliseconds) => {
        expect(signal).toBe(controller.signal);
        expect(budgetMilliseconds).toBeGreaterThan(0);
        observed.push(step.id);
        if (vector.stop === step.id) {
          if (vector.cancel) controller.abort(Error("cancelled"));
          else throw Error("failed step");
        }
      },
    });
    if (vector.stop === null) await script.run([]);
    else await expect(script.run([])).rejects.toThrow(vector.cancel ? "cancelled" : "failed step");
    expect(observed).toEqual(vector.expected);
  });

  it("imports no downstream development or concrete owner from the neutral executor", () => {
    const source = readFileSync(new URL("../../🔁️rebuild/🟦️.ts", import.meta.url), "utf8");
    expect(source).not.toMatch(/🧑‍💻dev|✏️s|🌎️hub|stagedConvergence/);
  });
});
