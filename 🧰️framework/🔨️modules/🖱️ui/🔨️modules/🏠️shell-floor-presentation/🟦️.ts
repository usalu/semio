// #region 🧲️Header
// 💻️ framework/ui/modules/🏠️shell-floor-presentation/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { SurfaceScopeValue } from "../../🧱️elements/🌈️Surface/🟦️.tsx";
import { surfaceClass } from "../🌈️surface-presentation/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🏛️ShellFloorPresentation
/** @emoji 🏛️ Whether base-floor chrome must paint its own surface. */
export function shellFloorPaints(parent: SurfaceScopeValue | null): boolean {
  return !(parent?.level === "base" && parent.fill !== "none");
}

/** @emoji 🏛️ Base-floor fill without nested same-level painting. */
export function shellFloorFillClass(parent: SurfaceScopeValue | null): string {
  return shellFloorPaints(parent) ? surfaceClass : "bg-transparent";
}
// #endregion 🏛️ShellFloorPresentation
