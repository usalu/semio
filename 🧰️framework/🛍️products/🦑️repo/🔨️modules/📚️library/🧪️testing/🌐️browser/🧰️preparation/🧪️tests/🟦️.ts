import { expect, test } from "bun:test";
import Ajv from "ajv";
import { declareLibraryBuildTestEnvelopeV1 } from "../../../🪪️native-birth/🟦️.ts";
import { withBuildTestInvocationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import { withHandledScriptInvocationSignalV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withPreparedBrowserV1 } from "../🚀️startup/🟦️.ts";
import schema from "../🧬️schema/🚀️startup.json" with { type: "json" };
import vectors from "../🧫️fixtures/🚀️startup.json" with { type: "json" };

declareLibraryBuildTestEnvelopeV1({ suite: "build-browser-preparation" });

test("browser preparation preserves invocation, refusal, ordering and cancellation", async () => {
  const actual: { name: string; events: string[]; outcome: string }[] = [];
  for (const row of vectors) await withBuildTestInvocationV1(async parent => {
    const controller = new AbortController(), reason = new Error("parent stop"), refusal = new Error("preparation refused"), events: string[] = [];
    let outcome = "served";
    await withHandledScriptInvocationSignalV1(parent, controller.signal, async invocation => {
      if (row.name === "pre-abort") controller.abort(reason);
      try {
        await withPreparedBrowserV1(invocation, async received => {
          expect(received).toBe(invocation);
          expect(received.control.signal).toBe(invocation.control.signal);
          events.push("prepare");
          if (row.name === "refusal") throw refusal;
          if (row.name === "during-preparation") controller.abort(reason);
        }, async signal => {
          expect(signal).toBe(invocation.control.signal);
          events.push("serve");
        });
      } catch (error) {
        if (row.name === "refusal") { expect(error).toBe(refusal); outcome = "refused"; }
        else { expect(error).toBe(reason); outcome = "cancelled"; }
      }
    });
    actual.push({ name: row.name, events, outcome });
  });
  expect(new Ajv({ strict: true }).validate(schema, actual)).toBe(true);
  expect(actual).toEqual(vectors);
});
