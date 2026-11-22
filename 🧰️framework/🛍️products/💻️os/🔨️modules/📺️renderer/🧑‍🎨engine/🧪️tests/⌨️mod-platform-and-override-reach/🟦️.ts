import { describe, expect, test } from "vitest";
import { SHELL_KEYBINDINGS, composeControlKeybindings, isAppleHotkeyPlatform, keyboardEventMatchesOwnedHotkey, parseOwnedHotkeyChords } from "@semio-tech/ui/modules/control-keybinding-context";
import { readStoredUiKeybindingOverrides } from "@semio-tech/ui/modules/keybinding-persistence";
import fixture from "../🧫️fixtures/⌨️mod-platform-and-override-reach/🔣️.json";

describe("⌨️ mod resolves from the host platform", () => {
  for (const platform of fixture.platforms) {
    test(`${platform.id} reads the platform string`, () => expect(isAppleHotkeyPlatform(platform.platform)).toBe(platform.usesMeta));
    test.each(fixture.modChords)(`${platform.id}: $id`, (row) => {
      const event = { key: row.event.key, ctrlKey: row.event.ctrl, metaKey: row.event.meta, shiftKey: row.event.shift, altKey: row.event.alt } as KeyboardEvent;
      const fires = parseOwnedHotkeyChords(row.chord, isAppleHotkeyPlatform(platform.platform)).some((chord) => keyboardEventMatchesOwnedHotkey(event, chord));
      expect(fires).toBe(row.matches[platform.id as "mac" | "windows" | "linux"]);
    });
  }
});

describe("⌨️ overrides reach every control id", () => {
  const storage = { get: () => JSON.stringify(fixture.overrideReach.stored), set: () => {}, remove: () => {} };
  const overrides = readStoredUiKeybindingOverrides(storage);
  const composed = composeControlKeybindings(new Map(Object.entries(fixture.overrideReach.appDefaults)), overrides);

  test("the storage gate drops what is not an element id or carries no chord", () => {
    for (const id of fixture.overrideReach.droppedByTheStorageGate) expect(id in overrides).toBe(false);
    expect("ui.search.toggle" in overrides).toBe(false);
  });

  test.each(Object.entries(fixture.overrideReach.resolved))("%s resolves to %s", (id, keys) => expect(composed.get(id)).toBe(keys));

  test("the shell table keeps its own default where nothing overrides it", () => expect(composed.get("ui.search.toggle")).toBe(SHELL_KEYBINDINGS["ui.search.toggle"]));
});
