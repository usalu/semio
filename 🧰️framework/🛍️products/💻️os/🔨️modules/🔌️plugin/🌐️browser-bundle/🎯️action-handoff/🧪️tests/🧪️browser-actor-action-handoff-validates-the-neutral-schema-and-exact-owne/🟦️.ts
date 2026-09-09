type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES, browserActorActionOwnerMatchesV1, createBrowserActorAppCommandRequestV1, createBrowserActorUiIntentRequestV1, decodeAppCommand, decodePackValue, parseBrowserActorActionRequestV1, parseBrowserActorActionResultV1 } = dependencies;

  const { expect, it } = vitest;
  it("browser actor action handoff validates the neutral schema and exact owner with an independent oracle", async () => {
    const { readFileSync } = await import("node:fs");
    const Ajv = (await import("ajv")).default;
    const equal = (await import("fast-deep-equal")).default;
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixture/🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
    expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    const request = parseBrowserActorActionRequestV1(fixture.request);
    const acknowledged = parseBrowserActorActionResultV1(fixture.acknowledged);
    expect(browserActorActionOwnerMatchesV1(request, acknowledged)).toBe(true);
    expect(equal(request, fixture.request)).toBe(true);
    expect(parseBrowserActorActionRequestV1(fixture.commandRequest).payload.kind).toBe("app-command");
    expect(parseBrowserActorActionResultV1(fixture.rejected).reason).toBe("action-refused");
    for (const hostile of fixture.hostileResults) expect(browserActorActionOwnerMatchesV1(request, parseBrowserActorActionResultV1(hostile))).toBe(false);
    expect(() => parseBrowserActorActionRequestV1({ ...fixture.request, payload: { ...fixture.request.payload, bytes: new Array(BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES + 1).fill(0) } })).toThrow(/bounded bytes/u);
    expect(() => parseBrowserActorActionRequestV1({ ...fixture.request, intent: fixture.request.payload.bytes })).toThrow(/exact fields/u);

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
