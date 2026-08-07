// #region 🧲️Header
// 💻️ framework/ui/elements/📑️Tabs/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as TabsPrimitive from "@radix-ui/react-tabs";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { interactiveTabActiveClass, interactiveHoverClass } from "../🏷️ClassNames/🟦️component.tsx";

// #endregion 🔌️Adapters

// #region 🏷️Tabs
// Tab container built on Radix primitives.
// Consumers MUST use TabsTrigger and TabsContent.

/**
 * Tabs holds the data fields for a Tabs record.
 **/
function Tabs({ className, ...props }: React.ComponentProps<typeof TabsPrimitive.Root>) {
  return <TabsPrimitive.Root data-slot="tabs" className={cn("flex flex-col gap-single", className)} {...props} />;
}

/**
 * TabsList holds the data fields for a TabsList record.
 **/
// 🎨️ Transparent — TabsList is chrome inside whatever painted surface (Panel/Pane/Dialog body) hosts
// it, never a level root of its own; a consumer placing it outside any painted surface should wrap
// it in <Surface>.
function TabsList({ className, ...props }: React.ComponentProps<typeof TabsPrimitive.List>) {
  return <TabsPrimitive.List data-slot="tabs-list" className={cn("text-muted-foreground inline-flex h-large w-fit items-center justify-center p-single bg-transparent", className)} {...props} />;
}

/** TabsTrigger holds the data fields for a TabsTrigger record.
 **/
/**
 **/
function TabsTrigger({ className, ...props }: React.ComponentProps<typeof TabsPrimitive.Trigger>) {
  return (
    <TabsPrimitive.Trigger
      data-slot="tabs-trigger"
      className={cn(
        "focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:outline-ring text-element inline-flex h-[calc(100%-var(--stroke-hairline))] flex-1 items-center justify-center gap-single border border-transparent p-single text-sm font-medium whitespace-nowrap transition-[color,box-shadow] focus-visible:ring-[length:var(--stroke-focus)] focus-visible:outline-1 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:shadow-sm [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        interactiveTabActiveClass,
        interactiveHoverClass,
        className,
      )}
      {...props}
    />
  );
}

/**
 **/
function TabsContent({ className, ...props }: React.ComponentProps<typeof TabsPrimitive.Content>) {
  return <TabsPrimitive.Content data-slot="tabs-content" className={cn("flex-1 outline-none", className)} {...props} />;
}

export { Tabs, TabsContent, TabsList, TabsTrigger };

// #endregion 🏷️Tabs
