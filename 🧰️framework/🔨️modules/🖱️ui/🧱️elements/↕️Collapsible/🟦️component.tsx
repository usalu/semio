// #region 🧲️Header
// 💻️ framework/ui/elements/↕️Collapsible/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as CollapsiblePrimitive from "@radix-ui/react-collapsible";
import { cn } from "../🫀️core/🏷️ClassNames/🟦️component.tsx";
import { interactiveControlTransitionClass, interactiveHoverClass } from "../🫀️core/🏷️ClassNames/🟦️component.tsx";

// #endregion 🔌️Adapters

// #region 🖥️Collapsible
// Collapsible section built on Radix primitives.
// Consumers MUST use CollapsibleTrigger.

/**
 * Collapsible holds the data fields for a Collapsible record.
 **/
function Collapsible({ ...props }: React.ComponentProps<typeof CollapsiblePrimitive.Root>) {
  return <CollapsiblePrimitive.Root data-slot="collapsible" {...props} />;
}

/**
 * CollapsibleTrigger holds the data fields for a CollapsibleTrigger record.
 **/
function CollapsibleTrigger({ className, ...props }: React.ComponentProps<typeof CollapsiblePrimitive.CollapsibleTrigger>) {
  return <CollapsiblePrimitive.CollapsibleTrigger data-slot="collapsible-trigger" className={cn("cursor-selectable", interactiveControlTransitionClass, interactiveHoverClass, className)} {...props} />;
}

/**
 **/
function CollapsibleContent({ ...props }: React.ComponentProps<typeof CollapsiblePrimitive.CollapsibleContent>) {
  return <CollapsiblePrimitive.CollapsibleContent data-slot="collapsible-content" {...props} />;
}

export { Collapsible, CollapsibleContent, CollapsibleTrigger };

// #endregion 🖥️Collapsible
