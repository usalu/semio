type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { Builder, OwnedUiPayload, activity, ownPayload, readers, saturationProbe } = dependencies;
  type Profile = any;
  type RetainedUiTypedValues = any;

  const { it, expect } = vitest;
      const { default: fixture } = await import("../../../🧪️fixtures/🏷️fields/🔣️.json");

  function prepared<P extends Profile>(kind: P, value: unknown): OwnedUiPayload<RetainedUiTypedValues[P]> {
    const builder = new Builder();
    const program = readers[kind](builder, value);
    for (let i = 0; i < 100_000; i++) {
      const result = program.next();
      if (result.done) return ownPayload({ value: result.value, references: 1, owned: builder.owned, bytes: builder.bytes, children: builder.children, fields: builder.fields, kind });
    }
    throw new Error("Private ownership fixture did not terminate");
  }

  it("TypedNodeFields preflights every capture before transfer under private reference saturation", () => {
    const source = prepared("node", { ...fixture.node, component: fixture.replacement });
    const replacement = prepared("component", fixture.replacement);
    const state = prepared("activity", { activity: "loading", disabled: false });
    const outcomes = saturationProbe!(source, replacement, state);
    for (const owner of [source, replacement, state]) { const retirement = owner.beginClose(); while (!retirement.terminalIsEmpty()) retirement.advance({ maxItems: 1, maxBytes: 4096 }); }
    expect(outcomes).toHaveLength(20);
    expect(outcomes.every(row => row.rejected && row.preserved), JSON.stringify(outcomes)).toBe(true);
  });

}
