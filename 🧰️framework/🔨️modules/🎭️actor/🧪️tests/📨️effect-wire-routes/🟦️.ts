/** 📨️ Who owns a `send-message` effect at the wire→friendly boundary, read from
 * `📦️packages/🟦️typescript/🧫️fixtures/📨️effect-wire-routes.json`. */
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
    readonly shellFramePayload: readonly number[] | null;
  };
  const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "🧫️fixtures/📨️effect-wire-routes.json"), "utf8")) as {
    readonly instanceId: number;
    readonly routedTargets: readonly string[];
    readonly cases: readonly Case[];
  };
  const decodePackValue = (bytes: Uint8Array): unknown => [...bytes];

  describe("📨️ send-message effects at the wire→friendly boundary", () => {
    afterEach(() => vi.restoreAllMocks());

    it("declares exactly the endpoints a host route consumes", () => {
      expect([...WIRE_SEND_MESSAGE_ROUTED_TARGETS]).toEqual([...fixture.routedTargets]);
    });

    for (const row of fixture.cases) {
      it(`${row.id} projects and warns exactly as the fixture declares`, () => {
        const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
        expect(wireEffectToFriendly(row.effect, decodePackValue)).toEqual(row.friendly);
        expect(warn.mock.calls.length > 0, `${row.id} warn expected=${row.warns} got=${JSON.stringify(warn.mock.calls)}`).toBe(row.warns);
        const frame = shellFrameBytes(row.effect, fixture.instanceId);
        expect(frame === null ? null : [...frame]).toEqual(row.shellFramePayload);
      });
    }

    it("routes every shell/backbone send-message and nothing else", () => {
      for (const row of fixture.cases) {
        const target = wireSendMessageTargetTag(row.effect);
        expect(isRoutedWireSendMessage(row.effect)).toBe(target !== null && fixture.routedTargets.includes(target));
      }
    });

    // 🧯️ The law the console regression was: a routed send-message must not be reported as an
    // unmapped effect. `unmapped` is reserved for a tag with no case at all.
    it("never calls a routed send-message unmapped", () => {
      const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
      for (const row of fixture.cases.filter((entry) => isRoutedWireSendMessage(entry.effect))) wireEffectToFriendly(row.effect, decodePackValue);
      expect(warn.mock.calls.flat().join(" ")).not.toContain("unmapped");
    });
  });
}
