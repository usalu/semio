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

function isAppleUiPlatform(): boolean {
  if (typeof navigator === "undefined") return false;
  if ("userAgentData" in navigator && navigator.userAgentData && typeof navigator.userAgentData === "object" && "platform" in navigator.userAgentData) {
    return (navigator.userAgentData as { readonly platform?: string }).platform === "macOS";
  }
  return /Mac|iPhone|iPod|iPad/i.test(navigator.platform);
}

/** @emoji ⌨️ Formats the first chord of a keybinding for inline or menu shortcut labels. */
export function formatKeybindingShortcut(keys: string): string {
  const chord = parseKeybindingChords(keys)[0];
  if (!chord) return "";
  const apple = isAppleUiPlatform();
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
        return "↑";
      case "down":
        return "↓";
      case "left":
        return "←";
      case "right":
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
  mod: "Control",
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

function ariaKeyToken(token: string): string {
  const modifier = ARIA_MODIFIER_NAMES[token];
  if (modifier !== undefined) return modifier;
  if (token.length === 1) return token.toUpperCase();
  return ARIA_NAMED_KEYS[token] ?? token;
}

/** @emoji ⌨️ Rewrites this codebase's chord grammar (`"mod+alt+arrowright,ctrl+k"` — comma-separated
 * alternatives, lowercase tokens) into the `aria-keyshortcuts` grammar: space-separated chords,
 * `+`-joined, DOM `KeyboardEvent.key` spelling with capitalized modifier names. `mod` resolves to
 * `Control` because `aria-keyshortcuts` names one concrete chord that assistive tech reads verbatim —
 * {@link formatKeybindingShortcut} stays the platform-aware VISUAL spelling. */
export function ariaKeyshortcutsText(keys: string | undefined): string | undefined {
  if (!keys) return undefined;
  const chords = parseKeybindingChords(keys)
    .map((chord) =>
      chord
        .split("+")
        .map((token) => token.trim())
        .filter(Boolean)
        .map(ariaKeyToken)
        .join("+"),
    )
    .filter(Boolean);
  return chords.length === 0 ? undefined : chords.join(" ");
}
// #endregion ⌨️KeybindingTextInterpretation
