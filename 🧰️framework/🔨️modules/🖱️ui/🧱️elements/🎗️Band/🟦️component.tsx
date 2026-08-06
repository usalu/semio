// #region 🧲️Header
// 💻️ framework/ui/elements/🎗️Band/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Scrollable } from "../../🧱️elements/📜Scrollable/🟦️component.tsx";
import { cn } from "../🫀️core/🏷️ClassNames/🟦️component.tsx";
import { borderNormalClass } from "../🫀️core/🏷️ClassNames/🟦️component.tsx";

// #endregion 🔌️Adapters

// #region 🥁️Band
// Horizontal band of navigation items with labels and icons.
// Consumers MUST provide BandItem entries.

/**
 * Configuration interface for a single band item.
 **/
export interface BandItem {
  content: React.ReactNode;
  className?: string;
  key?: React.Key;
}

/**
 * Props interface for the Band component.
 **/
export interface BandProps {
  id?: string;
  items: BandItem[];
  scrollable?: boolean;
  className?: string;
}

/**
 * Band holds the data fields for a Band record.
 **/
function Band({ items, scrollable = true, className, id }: BandProps) {
  // 🎨️ Transparent — Band is chrome inside whatever painted surface hosts it, never a level root.
  const bgClass = "bg-transparent";
  const borderClass = borderNormalClass;
  const itemsElement = (
    <div id={id} data-slot="band" className={cn("p-single flex gap-single items-center min-w-0", scrollable ? "w-fit" : "w-full")}>
      {items.map((item, index) => (
        <div key={item.key ?? index} className={cn("h-medium flex items-center min-w-0", item.className)}>
          {item.content}
        </div>
      ))}
    </div>
  );

  if (scrollable)
    return (
      <Scrollable orientation="horizontal" className={cn("border-b h-large", borderClass, bgClass, className)}>
        {itemsElement}
      </Scrollable>
    );
  return <div className={cn("border-b h-large", borderClass, bgClass, className)}>{itemsElement}</div>;
}

export { Band as Band };

// #endregion 🥁️Band
