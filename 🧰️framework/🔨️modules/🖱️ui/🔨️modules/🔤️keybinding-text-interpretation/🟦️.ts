// #region 🧲️Header
// 💻️ framework/ui/modules/⌨️keybinding-text-interpretation/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region ⌨️KeybindingTextInterpretation
/** @emoji ⌨️ Splits a declared `keys` binding into normalized chord alternatives. */
export function parseKeybindingChords(keys: string): string[] {
  return keys
    .split(",")
    .map((key) => key.trim().toLowerCase())
    .filter(Boolean);
}

/**
 * @emoji 🍎 The ONE rule deciding what `mod` means on this machine — Command on Apple, Control
 * everywhere else.
 *
 * Every spelling of a chord has to agree with the chord that actually FIRES, and there are three:
 * the dispatcher (`parseOwnedHotkeyChords`), the visual badge ({@link formatKeybindingShortcut}) and
 * the assistive-technology text ({@link ariaKeyshortcutsText}). While `aria-keyshortcuts` resolved
 * `mod` to `Control` on its own, the navbar role buttons rendered `⌘️⌥️V` and published
 * `aria-keyshortcuts="Control+Alt+V"` on the same button — a screen-reader user on macOS was told a
 * chord that does nothing (`📓️role-switch-regression-2026-09-14.md` §6).
 *
 * `platform` is the caller's own reading (`navigator.platform`) where one is already in hand; with no
 * argument it reads the live navigator, preferring `userAgentData.platform`.
 **/
export function keybindingPlatformUsesMetaV1(platform?: string): boolean {
  if (platform !== undefined) return /mac|iphone|ipad|ipod/i.test(platform);
  if (typeof navigator === "undefined") return false;
  if ("userAgentData" in navigator && navigator.userAgentData && typeof navigator.userAgentData === "object" && "platform" in navigator.userAgentData) {
    return (navigator.userAgentData as { readonly platform?: string }).platform === "macOS";
  }
  return /Mac|iPhone|iPod|iPad/i.test(navigator.platform);
}

/** @emoji ⌨️ Formats the first chord of a keybinding for inline or menu shortcut labels. `platform` is the caller's own reading where one is in hand, so the badge and {@link ariaKeyshortcutsText} can be proved to name the same physical key off a real machine. */
export function formatKeybindingShortcut(keys: string, platform?: string): string {
  const chord = parseKeybindingChords(keys)[0];
  if (!chord) return "";
  const apple = keybindingPlatformUsesMetaV1(platform);
  const glyph = (part: string): string => {
    switch (part) {
      case "mod":
        return apple ? "⌘️" : "Ctrl";
      case "ctrl":
        return apple ? "⌃️" : "Ctrl";
      case "meta":
        return "⌘️";
      case "alt":
        return apple ? "⌥️" : "Alt";
      case "shift":
        return apple ? "⇧️" : "Shift";
      case "backspace":
        return "⌫️";
      case "delete":
        return "⌦️";
      case "enter":
        return apple ? "↵️" : "Enter";
      case "escape":
        return apple ? "⎋️" : "Esc";
      case "up":
      case "arrowup":
        return "↑";
      case "down":
      case "arrowdown":
        return "↓";
      case "left":
      case "arrowleft":
        return "←";
      case "right":
      case "arrowright":
        return "→";
      default:
        if (part.length === 1) return part.toUpperCase();
        return part.charAt(0).toUpperCase() + part.slice(1);
    }
  };
  const parts = chord.split("+").map((part) => part.trim()).filter(Boolean);
  const labels = parts.map(glyph);
  return apple ? labels.join("") : labels.join("+");
}

const ARIA_MODIFIER_NAMES: Readonly<Record<string, string>> = {
  ctrl: "Control",
  control: "Control",
  meta: "Meta",
  cmd: "Meta",
  command: "Meta",
  alt: "Alt",
  option: "Alt",
  shift: "Shift",
};

const ARIA_NAMED_KEYS: Readonly<Record<string, string>> = {
  arrowleft: "ArrowLeft",
  arrowright: "ArrowRight",
  arrowup: "ArrowUp",
  arrowdown: "ArrowDown",
  left: "ArrowLeft",
  right: "ArrowRight",
  up: "ArrowUp",
  down: "ArrowDown",
  escape: "Escape",
  esc: "Escape",
  enter: "Enter",
  space: "Space",
  tab: "Tab",
};

function ariaKeyToken(token: string, usesMeta: boolean): string {
  if (token === "mod") return usesMeta ? "Meta" : "Control";
  const modifier = ARIA_MODIFIER_NAMES[token];
  if (modifier !== undefined) return modifier;
  if (token.length === 1) return token.toUpperCase();
  return ARIA_NAMED_KEYS[token] ?? token;
}

/** @emoji ⌨️ Rewrites this codebase's chord grammar (`"mod+alt+arrowright,ctrl+k"` — comma-separated
 * alternatives, lowercase tokens) into the `aria-keyshortcuts` grammar: space-separated chords,
 * `+`-joined, DOM `KeyboardEvent.key` spelling with capitalized modifier names. `mod` is resolved by
 * {@link keybindingPlatformUsesMetaV1}, the same predicate the dispatcher and the visual badge use,
 * so the chord a screen reader announces is the chord that fires. */
export function ariaKeyshortcutsText(keys: string | undefined, platform?: string): string | undefined {
  if (!keys) return undefined;
  const usesMeta = keybindingPlatformUsesMetaV1(platform);
  const chords = parseKeybindingChords(keys)
    .map((chord) =>
      chord
        .split("+")
        .map((token) => token.trim())
        .filter(Boolean)
        .map((token) => ariaKeyToken(token, usesMeta))
        .join("+"),
    )
    .filter(Boolean);
  return chords.length === 0 ? undefined : chords.join(" ");
}
// #endregion ⌨️KeybindingTextInterpretation
