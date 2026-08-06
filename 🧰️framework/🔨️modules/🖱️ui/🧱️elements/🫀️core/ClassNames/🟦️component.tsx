// #region 🧲️Header
// 💻️ framework/ui/elements/core/ClassNames/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ClassValue, clsx } from "clsx";
import { extendTailwindMerge } from "tailwind-merge";
// #endregion 🔌️Adapters

//#region 🎨️ClassNames
/**
 * 🆔️ `cn`, split out of the ui-react barrel into its own `🧱️elements/🫀️core/` file (ticket
 * 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE) — NOT deferred to a later "core extraction" pass like the
 * rest of `🎼️Utilities`, because `ActionGroup`/`Toggle` call `cn(...)` at MODULE TOP LEVEL (inside a
 * top-level `cva(cn(...))` call), not inside a component body. A module-top-level read of a barrel-defined
 * `const` (here `twMergeUi`) re-exported by a barrel that in turn imports these same elements is a genuine
 * ES-module circular-import initialization-order bug: whichever module the loader reaches first in the
 * cycle sees the other's `const` still in its temporal dead zone (see `🧱️elements/🫀️core/Ports/🟦️component.tsx`'s
 * header comment for the sibling `reactHostPort` case). Elements that only call `cn(...)` inside function
 * bodies (the overwhelming majority) are unaffected — evaluation happens at render time, long after both
 * modules have finished loading — so only this symbol needed to move early.
 *
 * @emoji 🎨️ `ui-surface`/`ui-glass`/`ui-veil` are the only per-level fills — extending Tailwind's built-in
 * `bg-color` group makes them mutually exclusive with each other AND with every `bg-*` utility (same group
 * ⇒ last-in-`cn()` wins, both directions), so a fill composed after `bg-transparent` genuinely paints
 * instead of losing silently to CSS declaration-count ordering.
 */
const twMergeUi = extendTailwindMerge({
  extend: {
    classGroups: {
      "bg-color": ["ui-surface", "ui-glass", "ui-veil"],
    },
  },
});

/**
 * Merges CSS class names using Tailwind merge.
 **/
export function cn(...inputs: ClassValue[]) {
  return twMergeUi(clsx(inputs));
}
//#endregion 🎨️ClassNames
