// #region 🧲️Header
// 💻️ framework/ui/elements/🎀️Ribbon/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🎀️Ribbon
// #region RibbonZone
interface RibbonZoneProps extends React.ComponentProps<"div"> {
  children: React.ReactNode;
  /** @emoji 📏️ Lets intrinsically tall ribbon content define the row height while retaining the standard minimum. */
  variableHeight?: boolean;
}

function RibbonZone({ className, children, variableHeight = false, ...props }: RibbonZoneProps) {
  return (
    <div data-slot="ribbon-zone" data-variable-height={variableHeight ? "true" : undefined} className={cn("flex shrink-0 items-center gap-single min-w-0", variableHeight ? "h-auto min-h-medium" : "h-medium", className)} {...props}>
      {children}
    </div>
  );
}

interface RibbonGroupProps extends React.ComponentProps<"div"> {
  children: React.ReactNode;
}

function RibbonGroup({ className, children, ...props }: RibbonGroupProps) {
  return (
    <div data-slot="ribbon-group" role="group" className={cn("flex shrink-0 items-center gap-single h-full", className)} {...props}>
      {children}
    </div>
  );
}

function RibbonDivider({ className, ...props }: React.ComponentProps<"div">) {
  return <div data-slot="ribbon-divider" className={cn("w-px h-small bg-border my-auto shrink-0", className)} {...props} />;
}

interface RibbonItemProps extends React.ComponentProps<"div"> {
  children: React.ReactNode;
}

function RibbonItem({ className, children, ...props }: RibbonItemProps) {
  return (
    <div data-slot="ribbon-item" className={cn("shrink-0 flex items-center h-full min-w-0", className)} {...props}>
      {children}
    </div>
  );
}

export { RibbonDivider, RibbonGroup, RibbonItem, RibbonZone };
// #endregion RibbonZone

// A ribbon is a chrome strip that grows by stacked rows — one per tree level — instead of staying a single line.

/** @emoji 🎀️ `inline` keeps every row on one horizontal line (footer drill-down); `up`/`down` stack rows vertically, growing away from the base row. */
export type RibbonDirection = "inline" | "up" | "down";

/** @emoji 🎀️ One row of a {@link Ribbon}, ordered base-first (index 0 = root/base level). */
export interface RibbonRow {
  readonly key: React.Key;
  readonly content: React.ReactNode;
}

/** @emoji 🎀️ Props for {@link Ribbon}. */
export interface RibbonProps {
  readonly id?: string;
  readonly direction: RibbonDirection;
  readonly rows: readonly RibbonRow[];
  readonly className?: string;
}

/** @emoji 🎀️ Chrome strip that grows by stacked rows — one per tree level. `up` stacks rows above the base (window utility bar); `down` stacks rows below the base (nested panel tabs); `inline` keeps the current horizontal drill-down (footer). */
function Ribbon({ id, direction, rows, className }: RibbonProps) {
  if (direction === "inline") {
    return (
      <div role="toolbar" id={id} data-slot="ribbon" data-direction={direction} className={cn("pointer-events-auto flex w-fit max-w-full shrink-0 items-center justify-start gap-single", className)}>
        {rows.map((row) => (
          <RibbonZone key={row.key}>{row.content}</RibbonZone>
        ))}
      </div>
    );
  }
  return (
    <div id={id} data-slot="ribbon" data-direction={direction} className={cn("flex w-fit max-w-full shrink-0", direction === "up" ? "flex-col-reverse items-start" : "w-full flex-col items-stretch", className)}>
      {rows.map((row, depth) => (
        <div key={row.key} data-slot="ribbon-row" data-depth={depth} className="flex min-w-0 w-full shrink-0 items-stretch">
          {row.content}
        </div>
      ))}
    </div>
  );
}

export { Ribbon };

// #endregion 🎀️Ribbon
