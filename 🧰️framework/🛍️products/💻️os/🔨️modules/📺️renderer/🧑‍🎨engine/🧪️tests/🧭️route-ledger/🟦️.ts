// #region 🧲️Header
/** @emoji 🧭️ When the shell's route effect applies a URI (🎫️ 26/09/23 C10): the neutral scenario corpus drives
 * `createShellRouteLedgerV1` and an independent XState machine of the same rule step by step, a strict Ajv schema is the
 * oracle for the corpus' shape, and the shell's source is held to routing every application through the ledger. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { assign, createActor, setup } from "xstate";
import { createShellRouteLedgerV1 } from "../../🧱️elements/🏛️ShellHost/🧭️route-ledger/🟦️.ts";
import corpus from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🧭️route-ledger/🔣️.json";
import shellSource from "../../🧱️elements/🏛️ShellHost/🟦️.tsx?raw";
// #endregion 🔌️Adapters

//#region 🧪️Laws
type StepV1 =
  | { readonly op: "fire"; readonly uri: string; readonly apply: boolean }
  | { readonly op: "begin"; readonly as: string }
  | { readonly op: "settle"; readonly of: string; readonly uri: string; readonly tookEffect: boolean }
  | { readonly op: "invalidate" };
type ScenarioV1 = { readonly name: string; readonly steps: readonly StepV1[] };
const scenarios = (corpus as { readonly scenarios: readonly ScenarioV1[] }).scenarios;

const oracleMachine = setup({
  types: {
    context: {} as { applied: string | null; generation: number },
    events: {} as { type: "settle"; started: number; uri: string; tookEffect: boolean } | { type: "invalidate" },
  },
}).createMachine({
  context: { applied: null, generation: 0 },
  on: {
    settle: { guard: ({ context, event }) => event.tookEffect && event.started === context.generation, actions: assign({ applied: ({ event }) => event.uri }) },
    invalidate: { actions: assign({ applied: null, generation: ({ context }) => context.generation + 1 }) },
  },
});

describe("shell route ledger", () => {
  it("accepts the neutral corpus under a strict schema oracle and rejects adversarial shapes", () => {
    const step = {
      oneOf: [
        { type: "object", additionalProperties: false, required: ["op", "uri", "apply"], properties: { op: { const: "fire" }, uri: { type: "string", pattern: "^/" }, apply: { type: "boolean" } } },
        { type: "object", additionalProperties: false, required: ["op", "as"], properties: { op: { const: "begin" }, as: { type: "string", minLength: 1 } } },
        { type: "object", additionalProperties: false, required: ["op", "of", "uri", "tookEffect"], properties: { op: { const: "settle" }, of: { type: "string", minLength: 1 }, uri: { type: "string", pattern: "^/" }, tookEffect: { type: "boolean" } } },
        { type: "object", additionalProperties: false, required: ["op"], properties: { op: { const: "invalidate" } } },
      ],
    };
    const validate = new Ajv({ strict: true, allErrors: true }).compile({
      type: "object",
      additionalProperties: false,
      required: ["description", "scenarios"],
      properties: {
        description: { type: "string" },
        scenarios: { type: "array", minItems: 1, items: { type: "object", additionalProperties: false, required: ["name", "steps"], properties: { name: { type: "string" }, steps: { type: "array", minItems: 1, items: step } } } },
      },
    });
    expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
    for (const hostile of [{ ...corpus, scenarios: [] }, { ...corpus, scenarios: [{ name: "x", steps: [{ op: "fire", uri: "spaces/a", apply: true }] }] }, { ...corpus, scenarios: [{ name: "x", steps: [{ op: "settle", of: "a", uri: "/", tookEffect: "yes" }] }] }]) expect(validate(hostile)).toBe(false);
  });

  it("applies a URI only when it is not in effect, exactly as the independent machine does, for every scenario", () => {
    for (const scenario of scenarios) {
      const ledger = createShellRouteLedgerV1();
      const oracle = createActor(oracleMachine).start();
      const started = new Map<string, { readonly ledger: number; readonly oracle: number }>();
      for (const [index, step] of scenario.steps.entries()) {
        const where = `${scenario.name} #${index}`;
        if (step.op === "fire") {
          expect(ledger.shouldApply(step.uri), where).toBe(step.apply);
          expect(oracle.getSnapshot().context.applied !== step.uri, where).toBe(step.apply);
        } else if (step.op === "begin") {
          started.set(step.as, { ledger: ledger.begin(), oracle: oracle.getSnapshot().context.generation });
        } else if (step.op === "settle") {
          const token = started.get(step.of);
          expect(token, where).toBeDefined();
          ledger.settle(token!.ledger, step.uri, step.tookEffect);
          oracle.send({ type: "settle", started: token!.oracle, uri: step.uri, tookEffect: step.tookEffect });
        } else {
          ledger.invalidate();
          oracle.send({ type: "invalidate" });
        }
        expect(ledger.applied(), where).toBe(oracle.getSnapshot().context.applied);
      }
      oracle.stop();
    }
  });

  it("routes every shell route application through the ledger", () => {
    expect(shellSource).toContain("if (hubOverlay || !shellRouteLedgerRef.current.shouldApply(shellUri)) return;");
    expect(shellSource).toContain("shellRouteLedgerRef.current.settle(generation, request.uri, await applyShellUriRef.current(request.uri, request.preservedViewState));");
    expect(shellSource.match(/shellRouteLedgerRef\.current\.invalidate\(\)/gu)?.length).toBe(2);
    expect(shellSource).toMatch(/rememberOpenSpaceId\(null\);\n\s+openInstanceIdRef\.current = null;\n\s+shellRouteLedgerRef\.current\.invalidate\(\);/u);
    expect(shellSource).not.toMatch(/navigateHistory\(`\/spaces\//u);
    expect(shellSource).not.toContain("navigateHistory(effect.navigate.uri)");
  });
});
//#endregion 🧪️Laws
