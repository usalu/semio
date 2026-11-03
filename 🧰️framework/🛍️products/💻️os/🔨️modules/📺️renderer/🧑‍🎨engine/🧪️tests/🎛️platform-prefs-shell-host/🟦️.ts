// @vitest-environment jsdom
/** ♿️ The React half of the platform-preferences law. The media queries the web reports become the four values of the native shell's `platform_prefs_changed` hook, are reported once per change, and are stated on the shell root where the stylesheet renders them (high contrast, reduced motion, dark scheme).
 *
 * 🧫️ Read from `🐚️Shell/🧪️tests/🧫️fixtures/🎛️platform-prefs/🔣️.json` — the SAME file the Rust driver law reads (`🛂️manifest/🧱️app-surface/📱️shell-hooks/🧪️tests`) and the wgpu browser host answers (`🎛️platform-prefs`), so queries, change rule and wire spelling cannot drift. */
import { describe, expect, it } from "vitest";
import { applyPlatformPrefsV1, createPlatformPrefsDriverV1, PLATFORM_PREFS_MEDIA_QUERIES_V1, platformPrefsAttributesV1, resolvePlatformPrefsV1, watchPlatformPrefsV1, type ShellPlatformPrefsV1 } from "../../🧱️elements/🏛️ShellHost/🎛️platform-prefs/🟦️.ts";
import fixture from "../../🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/🎛️platform-prefs/🔣️.json" with { type: "json" };

const answering = (prefs: ShellPlatformPrefsV1) => (query: string) => ({ matches: Object.entries(PLATFORM_PREFS_MEDIA_QUERIES_V1).some(([key, text]) => text === query && prefs[key as keyof ShellPlatformPrefsV1]), media: query, addEventListener: () => undefined, removeEventListener: () => undefined }) as unknown as MediaQueryList;

describe("♿️ platform preferences of the React shell", () => {
  it("asks the media queries of the fixture and nothing else", () => {
    expect(PLATFORM_PREFS_MEDIA_QUERIES_V1).toEqual(fixture.queries);
    const asked: string[] = [];
    resolvePlatformPrefsV1({ matchMedia: (query) => ({ matches: asked.push(query) < 0 }) as MediaQueryList });
    expect(asked.sort()).toEqual(Object.values(fixture.queries).sort());
    expect(resolvePlatformPrefsV1({})).toEqual(fixture.calm);
  });

  it("reports a change once, the first report always, like the Rust driver", () => {
    const driver = createPlatformPrefsDriverV1();
    for (const row of fixture.reports) expect(driver.note(row.prefs), row.id).toBe(row.changed);
    driver.forget();
    expect(driver.note(fixture.reports[0]!.prefs), "a forgotten report counts again").toBe(true);
  });

  it("spells the values on the wire like the Rust struct", () => {
    expect(JSON.stringify({ forcedColors: true, prefersContrastMore: false, prefersDark: true, prefersReducedMotion: true })).toBe(fixture.wire);
  });

  for (const row of fixture.rendering) {
    it(`states the preferences on the shell root: ${row.id}`, () => {
      expect(platformPrefsAttributesV1(row.prefs)).toEqual(row.attributes);
      const root = document.createElement("div");
      root.setAttribute("data-forced-colors", "stale");
      applyPlatformPrefsV1(root, row.prefs);
      for (const [name, value] of Object.entries(row.attributes)) expect(root.getAttribute(name), name).toBe(value);
    });
  }

  it("follows a live change of any query and stops listening when disposed", () => {
    let prefs: ShellPlatformPrefsV1 = fixture.calm;
    const listeners: Array<() => void> = [];
    const view = { matchMedia: (query: string) => ({ ...answering(prefs)(query), addEventListener: (_: string, listener: () => void) => void listeners.push(listener), removeEventListener: (_: string, listener: () => void) => void listeners.splice(listeners.indexOf(listener), 1) }) as unknown as MediaQueryList };
    const heard: ShellPlatformPrefsV1[] = [];
    const stop = watchPlatformPrefsV1(view, (next) => heard.push(next));
    expect(heard).toEqual([fixture.calm]);
    expect(listeners.length).toBe(4);
    for (const listener of [...listeners]) listener();
    expect(heard.length, "an unchanged report is silent").toBe(1);
    prefs = { ...fixture.calm, forcedColors: true };
    for (const listener of [...listeners]) listener();
    expect(heard).toEqual([fixture.calm, prefs]);
    stop();
    expect(listeners.length).toBe(0);
  });
});
