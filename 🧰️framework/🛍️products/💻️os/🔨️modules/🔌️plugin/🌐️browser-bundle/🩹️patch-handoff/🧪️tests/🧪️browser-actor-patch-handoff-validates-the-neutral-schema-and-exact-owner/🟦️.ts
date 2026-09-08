type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { browserActorUiPatchOwnerMatchesV1, parseBrowserActorUiPatchOfferV1, parseBrowserActorUiPatchResultV1 } = dependencies;

  const { expect, it } = vitest;
  it("browser actor patch handoff validates the neutral schema and exact owner with an independent oracle", async () => {
    const { readFileSync } = await import("node:fs");
    const Ajv = (await import("ajv")).default;
    const equal = (await import("fast-deep-equal")).default;
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixture/🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    const offer = parseBrowserActorUiPatchOfferV1(fixture.offer);
    const result = parseBrowserActorUiPatchResultV1(fixture.acknowledged);
    expect(browserActorUiPatchOwnerMatchesV1(offer, result)).toBe(true);
    expect(equal(offer.receipt, result.receipt)).toBe(true);
    expect(browserActorUiPatchOwnerMatchesV1(offer, parseBrowserActorUiPatchResultV1(fixture.rejected))).toBe(true);
    for (const hostile of fixture.hostileResults) expect(browserActorUiPatchOwnerMatchesV1(offer, parseBrowserActorUiPatchResultV1(hostile))).toBe(false);
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patch: { ...fixture.offer.patch, baseRevision: "0" } })).toThrow(/baseRevision/u);
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patch: { ...fixture.offer.patch, ops: [{ type: "remove", id: 1, extra: true }] } })).toThrow(/invalid fields/u);
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patch: { ...fixture.offer.patch, ops: [{ type: "unknown", id: 1 }] } })).toThrow(/type: invalid/u);
  });

}
