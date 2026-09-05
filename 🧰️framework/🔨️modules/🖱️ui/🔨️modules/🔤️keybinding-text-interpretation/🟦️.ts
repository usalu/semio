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
// #endregion ⌨️KeybindingTextInterpretation
