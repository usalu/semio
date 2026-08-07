// #region 🧲️Header
// 💻️ framework/ui/elements/📜Scrollable/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
// 🧱️core: reactHostPort imported directly from 🫀️core/Ports, NOT via the barrel — this component calls
// reactHostPort.forwardRef at module top level, which requires a non-circular import (see
// 🧱️elements/🔌️Ports/🟦️component.tsx's header comment for why the barrel import caused a real bug).
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { useWindowContentDeadLineScroll, windowContentDeadLineScrollClass } from "../🎛️Chrome/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🎮️Scrollable
/** @emoji 📜️ Native overflow scroll host (avoids Radix ScrollArea `setViewport` / `setScrollbar*Enabled` ref update loops). */
const Scrollable = reactHostPort.forwardRef<HTMLDivElement, React.ComponentPropsWithoutRef<"div"> & { orientation?: "vertical" | "horizontal" | "both" }>(({ className, children, orientation = "vertical", ...props }, ref) => {
  const scrollerRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const setScrollerRef = reactHostPort.useCallback(
    (node: HTMLDivElement | null) => {
      scrollerRef.current = node;
      if (typeof ref === "function") ref(node);
      else if (ref) ref.current = node;
    },
    [ref],
  );
  useWindowContentDeadLineScroll(scrollerRef);
  return (
    <div
      ref={setScrollerRef}
      data-slot="scroll-area"
      className={cn(
        "relative min-h-0 min-w-0 size-full focus-visible:ring-ring/50 transition-[color,box-shadow] outline-none focus-visible:ring-[length:var(--stroke-focus)] focus-visible:outline-1",
        orientation === "horizontal" ? "overflow-x-auto overflow-y-hidden" : orientation === "vertical" ? "overflow-y-auto overflow-x-hidden" : "overflow-auto",
        windowContentDeadLineScrollClass,
        className,
      )}
      {...props}
    >
      <div data-slot="scroll-area-viewport" className="min-h-0 min-w-0 w-full">
        {children}
      </div>
    </div>
  );
});
Scrollable.displayName = "Scrollable";

export { Scrollable };

// #endregion 🎮️Scrollable
