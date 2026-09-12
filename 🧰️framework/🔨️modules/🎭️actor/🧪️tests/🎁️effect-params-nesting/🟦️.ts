/** 🎁️ LAW: the wire→friendly effect projection reads every `req`-bearing effect's payload out of its
 * own `…-params` record, and unwraps a WIT `option<T>` before touching it.
 *
 * The neutral oracle is `🧫️fixtures/🎁️effect-params-nesting/🔣️.json`; its schema half is the WIT
 * itself (`🔌️plugin/🧬️schema/📜️.wit`), which the fixture names and this law re-reads, so a record
 * that gains or loses the `params` extraction cannot drift past both. */
type TestSource = { readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { wireEffectToFriendly } = dependencies;
  const { describe, expect, it } = vitest;
  const { readFileSync } = await import("node:fs");
  const { fileURLToPath } = await import("node:url");
  const { dirname, join, resolve } = await import("node:path");

  const here = dirname(fileURLToPath(source.url));
  const repoRoot = resolve(here, "../../../..");
  type Case = { readonly id: string; readonly effect: { readonly tag: string; readonly val?: Record<string, unknown> }; readonly friendly: unknown };
  const fixture = JSON.parse(readFileSync(join(here, "../🧫️fixtures/🎁️effect-params-nesting/🔣️.json"), "utf8")) as {
    readonly schema: string;
    readonly nested: { readonly records: Record<string, readonly string[]> };
    readonly flat: readonly string[];
    readonly cases: readonly Case[];
  };
  // 🧬️ `[7, 7]` in the fixture is the decoded pack; the double is the identity this law measures with.
  const decodePackValue = (bytes: Uint8Array): unknown => [...bytes];

  describe("🎁️ req-bearing effects carry their payload under `params`", () => {
    it("agrees with the WIT schema about which records nest and which stay flat", () => {
      const wit = readFileSync(resolve(repoRoot, fixture.schema), "utf8");
      for (const [record, fields] of Object.entries(fixture.nested.records)) {
        const effect = wit.slice(wit.indexOf(`record ${record}-effect {`));
        expect(effect.slice(0, effect.indexOf("}")), `${record}-effect nests its payload`).toContain(`params: ${record}-params,`);
        const params = wit.slice(wit.indexOf(`record ${record}-params {`));
        const body = params.slice(0, params.indexOf("}"));
        for (const field of fields) expect(body, `${record}-params declares ${field}`).toContain(`${field}:`);
      }
      for (const record of fixture.flat) {
        const effect = wit.slice(wit.indexOf(`record ${record}-effect {`));
        expect(effect.slice(0, effect.indexOf("}")), `${record}-effect stays flat`).not.toContain("params:");
      }
    });

    for (const row of fixture.cases) {
      it(`${row.id} projects exactly as the fixture declares`, () => {
        const friendly = wireEffectToFriendly(row.effect, decodePackValue);
        expect(JSON.parse(JSON.stringify(friendly ?? null))).toEqual(row.friendly);
      });
    }

    it("never hands the host a nameless dispatch", () => {
      for (const row of fixture.cases) {
        const friendly = wireEffectToFriendly(row.effect, decodePackValue) as { readonly dispatchAction?: { readonly action: string } } | null;
        if (friendly && "dispatchAction" in friendly) expect(friendly.dispatchAction?.action.length).toBeGreaterThan(0);
      }
    });
  });
}
