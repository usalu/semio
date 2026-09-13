/** 📨️ Who owns a `send-message` effect at the wire→friendly boundary, read from
 * `🧫️fixtures/📨️effect-wire-routes/🔣️.json`. */
type TestSource = { readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { shellFrameBytes, wireEffectToFriendly, isRoutedWireSendMessage, wireSendMessageTargetTag, WIRE_SEND_MESSAGE_ROUTED_TARGETS } = dependencies;
  const { afterEach, describe, expect, it, vi } = vitest;
  const { readFileSync } = await import("node:fs");
  const { fileURLToPath } = await import("node:url");
  const { dirname, join } = await import("node:path");

  type Case = {
    readonly id: string;
    readonly effect: { readonly tag: string; readonly val?: Record<string, unknown> };
    readonly friendly: unknown;
    readonly warns: boolean;
    readonly throws?: boolean;
    readonly shellFramePayload: readonly number[] | null;
  };
  const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "../🧫️fixtures/📨️effect-wire-routes/🔣️.json"), "utf8")) as {
    readonly instanceId: number;
    readonly routedTargets: readonly string[];
    readonly u64Fields: readonly string[];
    readonly cases: readonly Case[];
  };
  const decodePackValue = (bytes: Uint8Array): unknown => [...bytes];

  /** 🧬️ Rehydrates the two shapes JSON cannot carry but the wire really does: a WIT `u64` (declared as
   * decimal TEXT under `u64Fields`, restored to a `bigint`) and a `pack`/`list<u8>` (declared as
   * `{base64}`, restored to a `Uint8Array`). Narrowing either would make the fixture assert a shape no
   * renderer door ever sees. */
  const rehydrate = (value: unknown, key?: string): unknown => {
    if (typeof value === "string" && key !== undefined && fixture.u64Fields.includes(key)) return BigInt(value);
    if (Array.isArray(value)) return value.map((entry) => rehydrate(entry));
    if (value !== null && typeof value === "object") {
      const record = value as Record<string, unknown>;
      if (typeof record.base64 === "string" && Object.keys(record).length === 1) return Uint8Array.from(atob(record.base64), (character) => character.charCodeAt(0));
      return Object.fromEntries(Object.entries(record).map(([field, entry]) => [field, rehydrate(entry, field)]));
    }
    return value;
  };
  const wireOf = (row: Case): { readonly tag: string; readonly val?: Record<string, unknown> } => rehydrate(row.effect) as { readonly tag: string; readonly val?: Record<string, unknown> };

  describe("📨️ send-message effects at the wire→friendly boundary", () => {
    afterEach(() => vi.restoreAllMocks());

    it("declares exactly the endpoints a host route consumes", () => {
      expect([...WIRE_SEND_MESSAGE_ROUTED_TARGETS]).toEqual([...fixture.routedTargets]);
    });

    for (const row of fixture.cases) {
      it(`${row.id} projects and warns exactly as the fixture declares`, () => {
        const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
        const effect = wireOf(row);
        if (row.throws === true) expect(() => wireEffectToFriendly(effect, decodePackValue), `${row.id} must refuse`).toThrow();
        else expect(wireEffectToFriendly(effect, decodePackValue)).toEqual(rehydrate(row.friendly));
        expect(warn.mock.calls.length > 0, `${row.id} warn expected=${row.warns} got=${JSON.stringify(warn.mock.calls)}`).toBe(row.warns);
        const frame = shellFrameBytes(effect, fixture.instanceId);
        expect(frame === null ? null : [...frame]).toEqual(row.shellFramePayload);
      });
    }

    it("routes every shell/backbone send-message and nothing else", () => {
      for (const row of fixture.cases) {
        const target = wireSendMessageTargetTag(wireOf(row));
        expect(isRoutedWireSendMessage(wireOf(row)).valueOf()).toBe(target !== null && fixture.routedTargets.includes(target));
      }
    });

    // 🧯️ The law the console regression was: a routed send-message must not be reported as an
    // unmapped effect. `unmapped` is reserved for a tag with no case at all.
    it("never calls a routed send-message unmapped", () => {
      const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
      for (const row of fixture.cases.filter((entry) => isRoutedWireSendMessage(wireOf(entry)))) wireEffectToFriendly(wireOf(row), decodePackValue);
      expect(warn.mock.calls.flat().join(" ")).not.toContain("unmapped");
    });
  });
}
