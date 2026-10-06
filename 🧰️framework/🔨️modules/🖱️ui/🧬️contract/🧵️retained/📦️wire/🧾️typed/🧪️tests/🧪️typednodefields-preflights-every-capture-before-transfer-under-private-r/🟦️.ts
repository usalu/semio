/** 🏷️ Typed-wire vocabulary the suite annotates against; the destructured `OwnedUiPayload` value shadows its class name inside the body. */
import type { OwnedUiPayload as OwnedUiPayloadOf, Profile, RetainedUiTypedValues } from "../../🟦️.ts";

type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "Builder" | "OwnedUiPayload" | "RetainedUiTypedCursor" | "activity" | "ownPayload" | "readers" | "saturationProbe">, source: TestSource): Promise<void> {
  const { Builder, OwnedUiPayload, RetainedUiTypedCursor, activity, ownPayload, readers, saturationProbe } = dependencies;

  const { it, expect } = vitest;
      const { default: fixture } = await import("../../../🧫️fixtures/🏷️fields/🔣️.json");
  const { default: rowExtentFixture } = await import("../../../../../../🧫️fixtures/🌳️tree-window-row-extent/🔣️.json");
  const { default: typedFixture } = await import("../../../🧫️fixtures/🧾️typed/🔣️.json");

  function prepared<P extends Profile>(kind: P, value: unknown): OwnedUiPayloadOf<RetainedUiTypedValues[P]> {
    const builder = new Builder();
    const program = readers[kind](builder, value);
    for (let i = 0; i < 100_000; i++) {
      const result = program.next();
      if (result.done) return ownPayload({ value: result.value, references: 1, owned: builder.owned, bytes: builder.bytes, children: builder.children, fields: builder.fields, kind });
    }
    throw new Error("Private ownership fixture did not terminate");
  }

  it("normalizes the native Input draft target through the retained browser cursor", () => {
    const row = typedFixture.components.find((candidate: { wire: { draftTarget?: string } }) => candidate.wire.draftTarget);
    expect(row).toBeDefined();
    if (!row?.nativeHex) throw new Error("Typed input fixture requires its native transport bytes");
    const input = Uint8Array.from(Buffer.from(row.nativeHex, "hex"));
    const cursor = new RetainedUiTypedCursor(input, "component");
    expect(input.byteLength).toBe(0);
    for (let i = 0; i < 100_000; i++) {
      const step = cursor.advance({ maxItems: 1, maxBytes: 4096 });
      if (step.kind === "rejected") throw new Error(cursor.failure ?? "typed normalization rejected native input");
      if (step.kind === "ready") break;
    }
    const owner = cursor.takeResult();
    expect(owner?.value).toEqual(row!.expected);
    const retirement = owner!.beginClose();
    while (!retirement.terminalIsEmpty()) retirement.advance({ maxItems: 1, maxBytes: 4096 });
    cursor.beginClose();
    while (cursor.closeStep({ maxItems: 1, maxBytes: 4096 }).kind !== "complete") {}
  });

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
      const owner = prepared("component", { type: "treeSection", label: null, defaultOpen: null, window: { total: 10, offset: 4, rowExtent }, headerToolbar: 41 });
      expect(owner.value).toMatchObject({ type: "treeSection", window: { total: 10, offset: 4, rowExtent }, headerToolbar: 41 });
      const retirement = owner.beginClose();
      while (!retirement.terminalIsEmpty()) retirement.advance({ maxItems: 1, maxBytes: 4096 });
    }
    expect(() => prepared("component", { type: "treeSection", label: null, defaultOpen: null, headerToolbar: null, window: { total: 10, offset: 4 } })).toThrow("Unknown UI schema discriminator");
    expect(() => prepared("component", { type: "treeSection", label: null, defaultOpen: null, headerToolbar: null, window: { total: 10, offset: 4, rowExtent: "compactCheckbox" } })).toThrow("Unknown UI schema discriminator");
  });

  it("requires and owns the explicit Tree inline toolbar and detail relations", () => {
    const owner = prepared("component", {
      type: "treeItem",
      label: "Conflict",
      description: null,
      icon: null,
      defaultOpen: null,
      draggable: null,
      dragData: null,
      dimmed: null,
      selected: null,
      window: null,
      granularity: null,
      inlineToolbar: 42,
      detail: 43,
      rowActions: [], target: null,
    });
    expect(owner.value).toMatchObject({ type: "treeItem", inlineToolbar: 42, detail: 43 });
    const retirement = owner.beginClose();
    while (!retirement.terminalIsEmpty()) retirement.advance({ maxItems: 1, maxBytes: 4096 });
    expect(() => prepared("component", { type: "treeItem", label: "Conflict", description: null, icon: null, defaultOpen: null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, inlineToolBar: 42, rowActions: [], target: null })).toThrow("Unknown UI field: inlineToolBar");
    expect(() => prepared("component", { type: "treeItem", label: "Conflict", description: null, icon: null, defaultOpen: null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, treeDetail: 43, rowActions: [], target: null })).toThrow("Unknown UI field: treeDetail");
  });

}
