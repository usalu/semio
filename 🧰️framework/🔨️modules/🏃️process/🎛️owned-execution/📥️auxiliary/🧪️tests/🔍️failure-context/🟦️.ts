/** 🔍️ Auxiliary writer failures retain descriptor, phase and native cause without launching a process. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import vectors from "./🔣️.json" with { type: "json" };
import { OwnedAuxiliaryPublicationFailureV1, publishOwnedAuxiliaryInputV1 } from "../../🟦️.ts";

const ajv = new Ajv({ strict: true });
ajv.addSchema(schema, "auxiliary");
const valid = ajv.compile({ $ref: "auxiliary#/definitions/PublicationFailure" });

for (const row of vectors.cases) test(row.phase, async () => {
  const primary = row.error ? Object.assign(new Error("native publication refused"), row.error) : undefined;
  const listeners = new Map<string, (error?: unknown) => void>();
  const observed: { original: unknown; failure: OwnedAuxiliaryPublicationFailureV1 }[] = [];
  const pipe = {
    once(event: string, listener: (error?: unknown) => void) { listeners.set(event, listener); return pipe; },
    end(_text: string, _encoding: "utf8", callback: (error?: Error | null) => void) {
      if (row.phase === "end-call") throw primary;
      if (row.phase === "end-callback") callback(primary);
      else if (row.phase === "error-event") listeners.get("error")!(primary);
      else listeners.get("close")!();
      return pipe;
    },
  };
  await publishOwnedAuxiliaryInputV1(row.phase === "pipe-missing" ? null : pipe, "source", (original, phase) => observed.push({ original, failure: new OwnedAuxiliaryPublicationFailureV1(original, phase) }));
  listeners.get("error")?.(Object.assign(new Error("later failure"), { code: "EPIPE" }));
  expect(observed).toHaveLength(1);
  const result = observed[0]!;
  expect(valid(result.failure.context)).toBe(true);
  expect(JSON.parse(JSON.stringify(result.failure.context))).toEqual(row.expected);
  expect(result.failure.cause).toBe(result.original);
  expect(Object.isFrozen(result.failure.context)).toBe(true);
  expect(result.failure.message).toContain("descriptor=3");
  expect(result.failure.message).toContain("phase=" + row.phase);
  if (row.error) expect(result.original).toBe(primary);
});

test("diagnostics ignore accessor fields and retain the original primary", () => {
  let touched = false;
  const primary = Object.create(null, { code: { get() { touched = true; throw new Error("accessor called"); } } });
  const failure = new OwnedAuxiliaryPublicationFailureV1(primary, "end-call");
  expect(touched).toBe(false);
  expect(failure.cause).toBe(primary);
  expect(failure.context.code).toBeNull();
});
