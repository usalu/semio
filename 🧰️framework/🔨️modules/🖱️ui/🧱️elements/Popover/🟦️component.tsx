// #region 🧲️Header
// 💻️ framework/ui/elements/Popover/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as PopoverPrimitive from "@radix-ui/react-popover";
// 🚧️W3-interim: these still live in the ui-react barrel (not yet extracted to their own
// 🧱️elements/<Element>/ or 🧱️elements/🫀️core/ dirs) — W3 rewires this import per-symbol as each
// dependency's own element/core file lands. Do not import the barrel from any OTHER new leaf file
// without the same marker; grep for `🚧️W3-interim` must be empty before W6 closes.
import { cn, useFlow, glassClass, SurfaceScope } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 🌐️Popover
// Floating popover component built on Radix primitives.

/**
 * Popover holds the data fields for a Popover record.
/**
 * Popover holds the data fields for a Popover record.
 **/
function Popover({ ...props }: React.ComponentProps<typeof PopoverPrimitive.Root>) {
  return <PopoverPrimitive.Root data-slot="popover" {...props} />;
}

/**
 * PopoverTrigger holds the data fields for a PopoverTrigger record.
 **/
function PopoverTrigger({ className, ...props }: React.ComponentProps<typeof PopoverPrimitive.Trigger>) {
  return <PopoverPrimitive.Trigger data-slot="popover-trigger" className={cn(className)} {...props} />;
}

/**
 * PopoverContent holds the data fields for a PopoverContent record.
 **/
function PopoverContent({ className, align = "center", sideOffset = 4, children, ...props }: React.ComponentProps<typeof PopoverPrimitive.Content>) {
  const flow = useFlow();
  return (
    <PopoverPrimitive.Portal>
      <PopoverPrimitive.Content
        data-slot="popover-content"
        data-level="menu"
        dir={flow.inline === "rtl" ? "rtl" : undefined}
        align={align}
        sideOffset={sideOffset}
        className={cn(
          "text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-menu w-72 origin-(--radix-popover-content-transform-origin) border p-1 outline-hidden",
          glassClass,
          className,
        )}
        {...props}
      >
        <SurfaceScope level="menu" fill="glass">
          {children}
        </SurfaceScope>
      </PopoverPrimitive.Content>
    </PopoverPrimitive.Portal>
  );
}

/**
 * PopoverAnchor holds the data fields for a PopoverAnchor record.
 **/
function PopoverAnchor({ ...props }: React.ComponentProps<typeof PopoverPrimitive.Anchor>) {
  return <PopoverPrimitive.Anchor data-slot="popover-anchor" {...props} />;
}

export { Popover, PopoverAnchor, PopoverContent, PopoverTrigger };

// #endregion 🌐️Popover
