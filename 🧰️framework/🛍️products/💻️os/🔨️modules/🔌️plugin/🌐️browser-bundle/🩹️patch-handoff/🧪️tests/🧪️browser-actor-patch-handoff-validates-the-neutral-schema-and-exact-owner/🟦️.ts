type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "browserActorUiPatchOwnerMatchesV1" | "captureBrowserActorUiPatchV1" | "parseBrowserActorUiPatchOfferV1" | "parseBrowserActorUiPatchResultV1">, source: TestSource): Promise<void> {
  const { browserActorUiPatchOwnerMatchesV1, captureBrowserActorUiPatchV1, parseBrowserActorUiPatchOfferV1, parseBrowserActorUiPatchResultV1 } = dependencies;

  const { expect, it } = vitest;
  it("browser actor patch handoff validates the neutral schema and exact owner with an independent oracle", async () => {
    const { readFileSync } = await import("node:fs");
    const Ajv = (await import("ajv")).default;
    const equal = (await import("fast-deep-equal")).default;
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    const offer = parseBrowserActorUiPatchOfferV1(fixture.offer);
    const result = parseBrowserActorUiPatchResultV1(fixture.acknowledged);
    expect(browserActorUiPatchOwnerMatchesV1(offer, result)).toBe(true);
    expect(equal(offer.receipt, result.receipt)).toBe(true);
    expect(browserActorUiPatchOwnerMatchesV1(offer, parseBrowserActorUiPatchResultV1(fixture.rejected))).toBe(true);
    expect(offer.patches.map((patch) => patch.surface)).toEqual(result.verdicts.map((verdict) => verdict.surface));
    for (const hostile of fixture.hostileResults) expect(browserActorUiPatchOwnerMatchesV1(offer, parseBrowserActorUiPatchResultV1(hostile))).toBe(false);
    for (const hostile of fixture.hostileOffers) expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patches: hostile.patches }), hostile.name).toThrow(/patches/u);
    const [windowPatch] = fixture.offer.patches;
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patches: [{ ...windowPatch, baseRevision: "0" }] })).toThrow(/baseRevision/u);
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patches: [{ ...windowPatch, ops: [{ type: "remove", id: 1, extra: true }] }] })).toThrow(/invalid fields/u);
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patches: [{ ...windowPatch, ops: [{ type: "unknown", id: 1 }] }] })).toThrow(/type: invalid/u);
    expect(() => parseBrowserActorUiPatchResultV1({ ...fixture.rejected, verdicts: [{ surface: "map", outcome: "rejected", revision: 0 }] })).toThrow(/invalid fields/u);
    const { decodeActorUiPatchReceipt } = await import("../../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts");
    const receipt = decodeActorUiPatchReceipt(Uint8Array.from(fixture.offer.receipt));
    const port = { decodePack: () => { throw new Error("no pack in a set-children op"); }, natural: (value: unknown) => Number(value as bigint) };
    const { lift, surface, node, children } = fixture.wireSetChildren;
    const wire = (list: unknown) => [{ surface: { instance: receipt.lifetime.instanceId, surface }, baseRevision: 3n, revision: 4n, ops: [{ tag: "set-children", val: { node: BigInt(node), children: list } }] }];
    const lifted = lift === "BigUint64Array" ? BigUint64Array.from(children.map(BigInt)) : null;
    const captured = captureBrowserActorUiPatchV1(wire(lifted), receipt, receipt.lifetime, new Set([surface]), port);
    expect(captured?.patches).toEqual([{ surface, baseRevision: 3, revision: 4, ops: [{ type: "setChildren", id: node, children }] }]);
    expect(captureBrowserActorUiPatchV1(wire(children.map(BigInt)), receipt, receipt.lifetime, new Set([surface]), port)?.patches).toEqual(captured?.patches);
    expect(() => captureBrowserActorUiPatchV1(wire(Uint32Array.from(children)), receipt, receipt.lifetime, new Set([surface]), port)).toThrow(/children: invalid list/u);
  });

}
