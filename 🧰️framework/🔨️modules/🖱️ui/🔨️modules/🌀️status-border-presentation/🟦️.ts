// #region 🧲️Header
// 💻️ framework/ui/modules/🌀️status-border-presentation/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { UiStatus } from "@semio-tech/ui-styling";
import { cn } from "../🏷️class-name-composition/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🌀️StatusBorderPresentation
/** @emoji 🌀️ Dashed waiting border. */
export const waitingBorderClass = "border-waiting";

/** @emoji 🌀️ Active waiting border. */
export const waitingBorderActiveClass = cn(waitingBorderClass, "border-waiting-active");

/** @emoji 🌀️ Spinning loading border. */
export const loadingBorderClass = "border-loading";

/** @emoji 🌀️ Active loading border. */
export const loadingBorderActiveClass = cn(loadingBorderClass, "border-loading-active");

/** @emoji 🌀️ Waiting border selected from state. */
export function waitingBorderStateClass(waiting: boolean, active = false): string {
  return waiting ? (active ? waitingBorderActiveClass : waitingBorderClass) : "";
}

/** @emoji 🌀️ Loading border selected from state. */
export function loadingBorderStateClass(loading: boolean, active = false): string {
  return loading ? (active ? loadingBorderActiveClass : loadingBorderClass) : "";
}

/** @emoji 🌀️ Chrome status mapped to its border presentation. */
export function chromeStatusBorderClass(status: UiStatus | undefined, active = false): string {
  if (status === "loading") return loadingBorderStateClass(true, active);
  if (status === "waiting") return waitingBorderStateClass(true, active);
  return "";
}

/** @emoji 🌀️ Loading border in the level-aware element color. */
export const loadingBorderElementClass = cn(loadingBorderClass, "border-loading-element");

/** @emoji 🌀️ Waiting border in the level-aware element color. */
export const waitingBorderElementClass = cn(waitingBorderClass, "border-waiting-element");
// #endregion 🌀️StatusBorderPresentation
