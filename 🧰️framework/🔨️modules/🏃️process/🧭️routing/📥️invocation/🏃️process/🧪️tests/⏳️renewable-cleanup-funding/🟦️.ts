import { expect, test } from "bun:test";
import { createIssuedResourceControlV1, createRenewableIssuedResourceControlV1, createScriptProcessEnvelope, readAcceptedPhysicalCleanupV1, withAcceptedPhysicalCleanupV1, withScriptProcessEnvelope } from "../../🟦️.ts";
import { waitingDelayV1 } from "../../🐕️watchdog/🟦️.ts";
import fixture from "./🔣️.json";

/** ⏳️ A renewable caller funds its physical cleanup body from the nonrenewable maximum, so a declared wait beyond the idle window survives. @see ./🥒️.feature */
test("renewable idle window is not frozen into the physical body cutoff", async () => {
  await withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "fixture:renewable-cleanup", maximumElapsedMilliseconds: 0 }, {}, Date.now()), async original => {
    const renewable = createRenewableIssuedResourceControlV1(createIssuedResourceControlV1(original, "fixture:renewable-cleanup", 0), { idleMilliseconds: fixture.idleMilliseconds, maximumMilliseconds: fixture.maximumMilliseconds }, 0);
    await withAcceptedPhysicalCleanupV1(renewable, { version: 1, kind: "native-image", resource: "fixture:renewable-cleanup", reserveMilliseconds: fixture.reserveMilliseconds }, async (token, body) => {
      const facts = readAcceptedPhysicalCleanupV1(token), window = facts.bodyDeadlineEpochMilliseconds! - facts.acceptedAtEpochMilliseconds;
      expect(window).toBeGreaterThanOrEqual(fixture.bodyWindowMilliseconds.minimum);
      expect(window).toBeLessThanOrEqual(fixture.bodyWindowMilliseconds.maximum);
      await waitingDelayV1(fixture.waitMilliseconds);
      expect(body.remainingMilliseconds()!).toBeGreaterThan(0);
    });
  });
});
