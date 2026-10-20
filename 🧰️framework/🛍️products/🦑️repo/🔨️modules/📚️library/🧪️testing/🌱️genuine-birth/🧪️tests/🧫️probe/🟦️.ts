import { expect, test } from "bun:test";
import { readIssuedScriptProcessEnvelopeV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withBuildTestInvocationV1 } from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import { readGenuineBirthPolicyV1, selectGenuineBirthV1 } from "../../🟦️.ts";

const live = process.env.SEMIO_GENUINE_PROBE === "1";

/** 🧫️ Observes the genuine birth only when the harness law launched this file; plain runs skip it. */
test.skipIf(!live)("probe birth carries the declared repository request", () => withBuildTestInvocationV1(async original => {
  const birth = selectGenuineBirthV1(process.env.SEMIO_GENUINE_BIRTH), capabilities = readIssuedScriptProcessEnvelopeV1(original).capabilities as { arguments: string[]; tool: string };
  expect(capabilities.arguments).toEqual(["repo", ...birth.arguments]);
  expect(readGenuineBirthPolicyV1().births.map(row => row.id)).toContain(birth.id);
  expect(birth.sdk).toBe(process.env.SEMIO_NX_TOOLING_IDENTITY !== undefined);
}));

test.skipIf(!live)("probe a.b(c)+[d]", () => { expect(1).toBe(1); });

test.skipIf(!live)("probe a.b(c)+[d]x", () => { expect(1).toBe(1); });
