// #region 🧲️Header
// 💻️ framework/ui/elements/🫀️core/🧭️Flow/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header


// #region 🔌️Adapters
import * as React from "react";
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🧭️Flow
export type FlowInline = "ltr" | "rtl";

/** @emoji 🧭️ Vertical stacking direction — `"up"` grows content toward the display center (see {@link Ribbon}). */
export type FlowBlock = "down" | "up";

/** @emoji 🧭️ The flow descendant chrome mirrors against. */
export interface Flow {
  readonly inline: FlowInline;
  readonly block: FlowBlock;
}

const DEFAULT_FLOW: Flow = { inline: "ltr", block: "down" };

const FlowContext = reactHostPort.createContext<Flow>(DEFAULT_FLOW);

/** @emoji 🧭️ Sets the flow for descendant chrome, merging partial overrides onto the parent flow (nesting overrides). */
export const FlowProvider: React.FC<{
  readonly inline?: FlowInline;
  readonly block?: FlowBlock;
  readonly children: React.ReactNode;
}> = ({ inline, block, children }) => {
  const parent = reactHostPort.useContext(FlowContext);
  const value = reactHostPort.useMemo((): Flow => ({ inline: inline ?? parent.inline, block: block ?? parent.block }), [inline, block, parent]);
  return <FlowContext.Provider value={value}>{children}</FlowContext.Provider>;
};

/** @emoji 🪝️ Returns the nearest {@link FlowProvider} flow (defaults to LTR/down). */
export function useFlow(): Flow {
  return reactHostPort.useContext(FlowContext);
}
// #endregion 🧭️Flow
