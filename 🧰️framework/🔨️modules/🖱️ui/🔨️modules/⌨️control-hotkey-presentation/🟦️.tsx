// #region 🧲️Header
// 💻️ framework/ui/modules/⌨️control-hotkey-presentation/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { useControlHotkey } from "../⌨️control-keybinding-context/🟦️.tsx";
import { useUiDriver } from "../../🧱️elements/🚗️UiDriver/🟦️.tsx";
// #endregion 🔌️Adapters

// #region ⌨️ControlHotkeyPresentation
/** @emoji ⌨️ Props for an inline control hotkey badge. */
export interface ControlHotkeyBadgeProps {
  readonly id?: string;
  readonly allowInline: boolean;
}

/** @emoji ⌨️ Whether an inline hotkey badge should paint for the active driver. */
function useControlHotkeyInlineVisible(allowInline: boolean): boolean {
  const driver = useUiDriver();
  if (driver.hotkeys !== "inline") return false;
  if (allowInline) return true;
  return false;
}

/** @emoji ⌨️ Shared inline control-hotkey presentation. */
const controlHotkeyShortcutClassName = "ms-auto shrink-0 text-xs tracking-widest text-muted-foreground font-mono";

/** @emoji ⌨️ Inline kbd badge for a chrome control. */
export function ControlHotkeyBadge({ id, allowInline }: ControlHotkeyBadgeProps) {
  const hotkey = useControlHotkey(id);
  const show = useControlHotkeyInlineVisible(allowInline);
  if (!show || !hotkey) return null;
  return (
    <span data-slot="control-hotkey" aria-hidden className={controlHotkeyShortcutClassName}>
      {hotkey}
    </span>
  );
}
// #endregion ⌨️ControlHotkeyPresentation
