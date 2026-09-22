/** 🏷️ Typed-wire vocabulary the suite annotates against; the destructured `OwnedUiPayload` value shadows its class name inside the body. */
import type { OwnedUiPayload as OwnedUiPayloadOf, Profile, RetainedUiTypedValues } from "../../🟦️.ts";

type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "Builder" | "OwnedUiPayload" | "activity" | "ownPayload" | "readers" | "saturationProbe">, source: TestSource): Promise<void> {
  const { Builder, OwnedUiPayload, activity, ownPayload, readers, saturationProbe } = dependencies;

  const { it, expect } = vitest;
      const { default: fixture } = await import("../../../🧫️fixtures/🏷️fields/🔣️.json");
  const { default: rowExtentFixture } = await import("../../../../../../🧫️fixtures/🌳️tree-window-row-extent/🔣️.json");

  function prepared<P extends Profile>(kind: P, value: unknown): OwnedUiPayloadOf<RetainedUiTypedValues[P]> {
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

  it("requires and owns every closed Tree window row extent", () => {
    const wireExtent = { Standard: "standard", CompactText: "compactText", CompactSmallControl: "compactSmallControl", CompactControl: "compactControl" } as const;
    for (const extent of rowExtentFixture.extents) {
      const rowExtent = wireExtent[extent.name as keyof typeof wireExtent];
      const owner = prepared("component", { type: "treeSection", label: null, defaultOpen: null, window: { total: 10, offset: 4, rowExtent } });
      expect(owner.value).toMatchObject({ type: "treeSection", window: { total: 10, offset: 4, rowExtent } });
      const retirement = owner.beginClose();
      while (!retirement.terminalIsEmpty()) retirement.advance({ maxItems: 1, maxBytes: 4096 });
    }
    expect(() => prepared("component", { type: "treeSection", label: null, defaultOpen: null, window: { total: 10, offset: 4 } })).toThrow("Unknown UI schema discriminator");
    expect(() => prepared("component", { type: "treeSection", label: null, defaultOpen: null, window: { total: 10, offset: 4, rowExtent: "compactCheckbox" } })).toThrow("Unknown UI schema discriminator");
  });

}
