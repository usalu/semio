// #region 🧲️Header
// 💻️ framework/ui/modules/👥️presence-presentation/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { STYLING_PRESENCE_PALETTES } from "@semio-tech/ui-styling";
// #endregion 🔌️Adapters

// #region 🎨️PresencePalette
/** 🌓️ Selects which of `STYLING_PRESENCE_PALETTES`'s `light`/`dark` base `{s, l}` {@link presenceColor} resolves against. */
export type PresenceAppearance = "light" | "dark";

/** 🎨️ Resolved HSL triple — `h` in degrees `[0, 360)`, `s`/`l` in `[0, 1]`. */
export interface PresenceHsl {
  readonly h: number;
  readonly s: number;
  readonly l: number;
}

/** 🎨️ Deterministic per-session palette color for a hub-assigned index (contract freeze §C7.5):
 * `index % 12` selects one of the 12 base hues (`STYLING_PRESENCE_PALETTES.hues`); `Math.floor(index / 12)`
 * (`k`) desaturates by `0.25` once the roster wraps past two full cycles and alternates lightness by
 * `±0.14` every other cycle (lighter in `"light"`, darker in `"dark"`). Byte-identical to the Rust twin
 * `presence_color` in `🧊️component.rs`. Replaces the deleted FNV-hash `presenceHueForActor`. */
export function presenceColor(index: number, appearance: PresenceAppearance): PresenceHsl {
  const base = index % 12;
  const k = Math.floor(index / 12);
  const h = STYLING_PRESENCE_PALETTES.hues[base]!;
  const baseAppearance = STYLING_PRESENCE_PALETTES[appearance];
  const s = baseAppearance.s - (k >= 2 ? 0.25 : 0);
  const lShift = k % 2 === 1 ? 0.14 : 0;
  const l = appearance === "light" ? baseAppearance.l + lShift : baseAppearance.l - lShift;
  return { h, s, l };
}

/** 🎨️ CSS custom-property reference for a peer's base-cycle palette index (`index % 12`) — only
 * meaningful when `Math.floor(index / 12) === 0`; callers past the first cycle render {@link presenceColor}'s
 * HSL inline instead (contract freeze §C7.5). */
export function presenceCssVar(index: number): string {
  return `var(--presence-${index % 12})`;
}

/** 🖌️ The paint of a session's palette slot: the `--presence-N` CSS var for the base cycle (it follows `.dark` by
 * itself), or an inline `hsl()` literal of {@link presenceColor} past it — the {@link presenceColor}/{@link presenceCssVar}
 * split from contract freeze §C7.5. A session without a slot paints as slot 0. */
export function presencePaint(color: number | undefined, appearance: PresenceAppearance): string {
  const index = color ?? 0;
  if (Math.floor(index / 12) === 0) return presenceCssVar(index);
  const { h, s, l } = presenceColor(index, appearance);
  return `hsl(${h}deg ${(s * 100).toFixed(2)}% ${(l * 100).toFixed(2)}%)`;
}
// #endregion 🎨️PresencePalette
