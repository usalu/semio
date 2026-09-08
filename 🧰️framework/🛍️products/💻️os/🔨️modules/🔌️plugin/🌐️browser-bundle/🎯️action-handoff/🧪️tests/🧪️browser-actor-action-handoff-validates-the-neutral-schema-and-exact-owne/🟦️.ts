type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES, browserActorActionOwnerMatchesV1, parseBrowserActorActionInvocationV1, parseBrowserActorActionRequestV1, parseBrowserActorActionResultV1 } = dependencies;

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
    expect(equal(parseBrowserActorActionInvocationV1(fixture.invocation), fixture.invocation)).toBe(true);
    expect(parseBrowserActorActionResultV1(fixture.rejected).reason).toBe("action-refused");
    for (const hostile of fixture.hostileResults) expect(browserActorActionOwnerMatchesV1(request, parseBrowserActorActionResultV1(hostile))).toBe(false);
    expect(() => parseBrowserActorActionRequestV1({ ...fixture.request, command: new Array(BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES + 1).fill(0) })).toThrow(/bounded bytes/u);
    expect(() => parseBrowserActorActionInvocationV1({ ...fixture.invocation, address: { ...fixture.invocation.address, extra: true } })).toThrow(/exact fields/u);
  });

}
