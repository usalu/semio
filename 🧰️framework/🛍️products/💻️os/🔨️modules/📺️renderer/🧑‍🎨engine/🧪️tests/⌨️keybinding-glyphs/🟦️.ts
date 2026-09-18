/** ⌨️ The chord-glyph law, React's half. Both renderers format a keybinding for the same badge, and a
 * badge that names a different physical key than the one that fires is a real defect — a macOS browser
 * read `Ctrl+Alt+E` from the wgpu shell where React read `⌘️⌥️E` on the same navbar role chip
 * (`📓️audit-visual-parity-puzzle3d.md` §7 item 7).
 *
 * 🧫️ Read from `🐚️Shell/🧫️fixtures/⌨️keybinding-glyphs/🔣️.json` — the SAME file the wgpu law reads
 * (`🐚️Shell/🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs`), so the two tables cannot drift apart.
 */
import { describe, expect, it } from "vitest";
import { formatKeybindingShortcut, keybindingPlatformUsesMetaV1 } from "../../../../../../../🔨️modules/🖱️ui/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts";
import fixture from "../../🧱️elements/🐚️Shell/🧫️fixtures/⌨️keybinding-glyphs/🔣️.json" with { type: "json" };

describe("⌨️ keybinding glyphs", () => {
  it("reads every Apple platform spelling as Command, and nothing else as Command", () => {
    for (const platform of fixture.platforms.apple) expect(keybindingPlatformUsesMetaV1(platform), platform).toBe(true);
    for (const platform of fixture.platforms.other) expect(keybindingPlatformUsesMetaV1(platform), platform).toBe(false);
  });

  it("formats every row of the shared table on both platforms", () => {
    for (const row of fixture.rows) {
      expect(formatKeybindingShortcut(row.keys, "MacIntel"), `apple: ${row.keys}`).toBe(row.apple);
      expect(formatKeybindingShortcut(row.keys, "Win32"), `other: ${row.keys}`).toBe(row.other);
    }
  });
});
