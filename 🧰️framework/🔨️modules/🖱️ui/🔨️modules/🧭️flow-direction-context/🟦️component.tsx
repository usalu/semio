// #region 🧲️Header
// 💻️ framework/ui/modules/🧭️flow-direction-context/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { reactHostPort } from "../../🧱️elements/🔌️Ports/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🧭️FlowDirectionContext
/** @emoji 🧭️ Horizontal reading direction — `"rtl"` mirrors inline chrome. */
export type FlowInline = "ltr" | "rtl";

/** @emoji 🧭️ Vertical stacking direction — `"up"` grows content toward the display center. */
export type FlowBlock = "down" | "up";

/** @emoji 🧭️ Logical flow inherited by descendant chrome. */
export interface Flow {
  readonly inline: FlowInline;
  readonly block: FlowBlock;
}

/** @emoji 🧭️ Opaque provider children rendered by the React adapter. */
type FlowProviderChildren = unknown;

/** @emoji 🧭️ Partial logical-flow override supplied to a descendant subtree. */
interface FlowProviderProps {
  readonly inline?: FlowInline;
  readonly block?: FlowBlock;
  readonly children: FlowProviderChildren;
}

const DEFAULT_FLOW: Flow = { inline: "ltr", block: "down" };

const FlowContext = reactHostPort.createContext<Flow>(DEFAULT_FLOW);

/** @emoji 🧭️ Sets the flow for descendant chrome, merging partial overrides onto the parent flow. */
export function FlowProvider({ inline, block, children }: FlowProviderProps) {
  const parent = reactHostPort.useContext(FlowContext);
  const value = reactHostPort.useMemo((): Flow => ({ inline: inline ?? parent.inline, block: block ?? parent.block }), [inline, block, parent]);
  const content = children as React.ReactNode;
  return <FlowContext.Provider value={value}>{content}</FlowContext.Provider>;
}

/** @emoji 🪝️ Returns the nearest {@link FlowProvider} flow, defaulting to LTR/down. */
export function useFlow(): Flow {
  return reactHostPort.useContext(FlowContext);
}
// #endregion 🧭️FlowDirectionContext
