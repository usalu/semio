// #region 🧲️Header
// 💻️ framework/ui/modules/🏷️class-name-composition/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { clsx } from "clsx";
import { extendTailwindMerge } from "tailwind-merge";
// #endregion 🔌️Adapters

// #region 🎨️ClassNameComposition
/** @emoji 🧬️ Repository-owned recursive input for CSS class composition. */
export type ClassNameInput = string | number | bigint | boolean | null | undefined | { readonly [className: string]: unknown } | ClassNameInput[];

const twMergeUi = extendTailwindMerge({
  extend: {
    classGroups: {
      "bg-color": ["ui-surface", "ui-glass", "ui-veil"],
    },
  },
});

/** @emoji 🪢️ Merges CSS classes with the UI Tailwind conflict policy. */
export function cn(...inputs: ClassNameInput[]): string {
  return twMergeUi(clsx(inputs));
}
// #endregion 🎨️ClassNameComposition
