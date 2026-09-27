/** ⏳️ LAW: the client's reading of a hub refusal walks the hub's own language-agnostic fixture
 * (`🌎️hub/🚧️refusal/🧫️fixtures/⏳️transient-apply-refusal-v1`), admitted by the hub's schema through Ajv (third party): every transient
 * cause's answer is the transient refusal the client resends, every permanent cause and every near miss is not — the client and
 * Ajv agree on each (ticket 26/09/23 C12, run `c12short3`: a transient `DB I/O aggregate admission exhausted` was read as a
 * rejection and every keystroke typed during a connection shortage was rolled back). */
export async function registerTransientApplyRefusalTests(
  vitest: NonNullable<ImportMeta["vitest"]>,
  twin: { readonly HUB_TRANSIENT_APPLY_REFUSAL_CODE: string; readonly hubTransientApplyRefusalV1: (messages: ArrayLike<number>) => boolean },
): Promise<void> {
  const { describe, it, expect } = vitest;
  const { default: fixture } = await import("../../../../../🌎️hub/🚧️refusal/🧫️fixtures/⏳️transient-apply-refusal-v1/🔣️.json");
  const { default: schema } = await import("../../../../../🌎️hub/🚧️refusal/🧬️schema/🔣️.json");
  const encode = (messages: unknown[]): Uint8Array => new TextEncoder().encode(JSON.stringify(messages));

  describe("TransientApplyRefusal", () => {
    it("names the hub schema's code", () => {
      expect(twin.HUB_TRANSIENT_APPLY_REFUSAL_CODE).toBe((schema as { $defs: Record<string, { const?: unknown }> }).$defs.HubTransientApplyRefusalCodeV1!.const);
      expect(fixture.answer.code).toBe(twin.HUB_TRANSIENT_APPLY_REFUSAL_CODE);
    });

    it("reads every transient cause as a resend and every permanent one as a refusal, agreeing with Ajv", async () => {
      const { default: Ajv } = await import("ajv");
      const ajv = new Ajv({ strict: false, allErrors: true });
      ajv.addSchema(schema, "refusal");
      const validMessage = ajv.getSchema("refusal#/$defs/HubTransientApplyRefusalMessageV1")!;
      for (const cause of fixture.causes) {
        const message = { ...fixture.answer, message: cause.reason };
        expect(validMessage(message), `${cause.cause} ${JSON.stringify(validMessage.errors)}`).toBe(true);
        expect(twin.hubTransientApplyRefusalV1(encode([message])), cause.cause).toBe(true);
      }
      for (const cause of fixture.permanentCauses) expect(twin.hubTransientApplyRefusalV1(new Uint8Array()), cause.cause).toBe(false);
      for (const invalid of fixture.invalidMessages) {
        expect(validMessage(invalid), JSON.stringify(invalid)).toBe(false);
        expect(twin.hubTransientApplyRefusalV1(encode([invalid])), JSON.stringify(invalid)).toBe(false);
      }
      expect(twin.hubTransientApplyRefusalV1(encode([{ ...fixture.answer, message: "x" }, { ...fixture.answer, message: "y" }]))).toBe(false);
      expect(twin.hubTransientApplyRefusalV1(new TextEncoder().encode("hub.unavailable"))).toBe(false);
    });
  });
}
