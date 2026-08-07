// #region 🧲️Header
// 💻️ framework/ui/elements/🪟HoverCard/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as HoverCardPrimitive from "@radix-ui/react-hover-card";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { glassClass } from "../🏷️ClassNames/🟦️component.tsx";
import { SurfaceScope } from "../🌈️Surface/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🔧️HoverCard
// Hover-triggered card built on Radix primitives.
// Consumers MUST use HoverCardTrigger to activate.

/**
 * HoverCard holds the data fields for a HoverCard record.
 **/
function HoverCard({ ...props }: React.ComponentProps<typeof HoverCardPrimitive.Root>) {
  return <HoverCardPrimitive.Root data-slot="hover-card" {...props} />;
}

/**
 * HoverCardTrigger holds the data fields for a HoverCardTrigger record.
 **/
function HoverCardTrigger({ className, ...props }: React.ComponentProps<typeof HoverCardPrimitive.Trigger>) {
  return <HoverCardPrimitive.Trigger data-slot="hover-card-trigger" className={cn(className)} {...props} />;
}

/**
 * HoverCardContent holds the data fields for a HoverCardContent record.
 **/
function HoverCardContent({ className, align = "center", sideOffset = 4, children, ...props }: React.ComponentProps<typeof HoverCardPrimitive.Content>) {
  return (
    <HoverCardPrimitive.Portal data-slot="hover-card-portal">
      <HoverCardPrimitive.Content
        data-slot="hover-card-content"
        data-level="menu"
        align={align}
        sideOffset={sideOffset}
        className={cn(
          "text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-menu w-64 origin-(--radix-hover-card-content-transform-origin) border p-single outline-hidden",
          glassClass,
          className,
        )}
        {...props}
      >
        <SurfaceScope level="menu" fill="glass">
          {children}
        </SurfaceScope>
      </HoverCardPrimitive.Content>
    </HoverCardPrimitive.Portal>
  );
}

export { HoverCard, HoverCardContent, HoverCardTrigger };

// #endregion 🔧️HoverCard
