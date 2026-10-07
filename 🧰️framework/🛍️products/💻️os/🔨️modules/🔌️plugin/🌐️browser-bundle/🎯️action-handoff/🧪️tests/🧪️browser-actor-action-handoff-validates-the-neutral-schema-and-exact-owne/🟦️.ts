type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES, browserActorActionOwnerMatchesV1, browserActorActionRefusalReasonV1, browserActorAdmissionRefusalReasonV1, browserActorGuestRefusalReasonV1, createBrowserActorAppCommandRequestV1, createBrowserActorUiIntentRequestV1, decodeAppCommand, decodePackValue, parseBrowserActorActionRequestV1, parseBrowserActorActionResultV1 } = dependencies;

  const { expect, it } = vitest;
  it("browser actor action handoff validates the neutral schema and exact owner with an independent oracle", async () => {
    const { readFileSync } = await import("node:fs");
    const Ajv = (await import("ajv")).default;
    const equal = (await import("fast-deep-equal")).default;
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
    const request = parseBrowserActorActionRequestV1(fixture.request);
    const acknowledged = parseBrowserActorActionResultV1(fixture.acknowledged);
    expect(new Ajv({ strict: true }).compile({ ...schema, $ref: "#/definitions/request" })(request)).toBe(true);
    expect(new Ajv({ strict: true }).compile({ ...schema, $ref: "#/definitions/result" })(acknowledged)).toBe(true);
    expect(browserActorActionOwnerMatchesV1(request, acknowledged)).toBe(true);
    expect(acknowledged.commit).toEqual({ operation: "18446744073709551615", revision: "18364758544493064720" });
    expect(equal(request, fixture.request)).toBe(true);
    expect(parseBrowserActorActionRequestV1(fixture.commandRequest).payload.kind).toBe("app-command");
    expect(parseBrowserActorActionResultV1(fixture.rejected).reason).toBe("action-refused");
    for (const row of fixture.guestRefusals) {
      const reason = browserActorGuestRefusalReasonV1(row.detail);
      expect(reason, JSON.stringify(row.detail)).toBe(row.reason);
      expect(parseBrowserActorActionResultV1({ ...fixture.rejected, reason }).reason).toBe(reason);
      expect(browserActorActionRefusalReasonV1(new Error(reason)), reason).toBeNull();
    }
    for (const row of fixture.admissionRefusals) {
      const reason = browserActorAdmissionRefusalReasonV1(row.detail);
      expect(reason, JSON.stringify(row.detail)).toBe(row.reason);
      expect(parseBrowserActorActionResultV1({ ...fixture.rejected, reason }).reason).toBe(reason);
      expect(browserActorActionRefusalReasonV1(new Error(reason)), reason).toBe("dispatch-failed");
    }
    for (const hostile of fixture.hostileResults) expect(browserActorActionOwnerMatchesV1(request, parseBrowserActorActionResultV1(hostile))).toBe(false);
    expect(() => parseBrowserActorActionRequestV1({ ...fixture.request, payload: { ...fixture.request.payload, bytes: new Array(BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES + 1).fill(0) } })).toThrow(/bounded bytes/u);
    expect(() => parseBrowserActorActionRequestV1({ ...fixture.request, intent: fixture.request.payload.bytes })).toThrow(/exact fields/u);
    expect(() => parseBrowserActorActionResultV1({ ...fixture.acknowledged, commit: { operation: "01", revision: "2" } })).toThrow(/decimal u64/u);
    expect(() => parseBrowserActorActionResultV1({ ...fixture.acknowledged, commit: { operation: "18446744073709551616", revision: "2" } })).toThrow(/decimal u64/u);
    expect(() => parseBrowserActorActionResultV1({ ...fixture.rejected, commit: fixture.acknowledged.commit })).toThrow(/rejected publication/u);

    const owner = { ...request, actionSequence: 11 };
    const intent = createBrowserActorUiIntentRequestV1(owner, fixture.uiIntent.surface, { ...fixture.uiIntent, seq: BigInt(fixture.uiIntent.seq) });
    expect(intent.payload.kind).toBe("ui-intent");
    const action = createBrowserActorAppCommandRequestV1(owner, fixture.actionInvocation, fixture.commandViewState);
    const command = createBrowserActorAppCommandRequestV1({ ...owner, actionSequence: 12 }, fixture.commandInvocation, fixture.commandViewState);
    for (const [encoded, expected, sequence] of [[action, fixture.actionInvocation, 11], [command, fixture.commandInvocation, 12]] as const) {
      expect(encoded.payload.kind).toBe("app-command");
      const frame = decodeAppCommand(Uint8Array.from(encoded.payload.bytes));
      expect("Command" in frame).toBe(true);
      expect(frame.Command.seq).toBe(sequence);
      expect(equal(decodePackValue(Uint8Array.from(frame.Command.command)), expected)).toBe(true);
      expect(equal(decodePackValue(Uint8Array.from(frame.Command.view_state)), fixture.commandViewState)).toBe(true);
    }
  });

}
