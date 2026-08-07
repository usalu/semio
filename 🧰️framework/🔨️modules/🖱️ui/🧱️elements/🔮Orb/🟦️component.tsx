// #region 🧲️Header
// 💻️ framework/ui/elements/🔮Orb/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🎄️Orb
// Circular position indicator on a Ring. t ∈ [0,1[ maps to an angle on the ring.

interface OrbProps {
  id: string;
  t: number;
  disabled?: boolean;
  selected?: boolean;
  hovered?: boolean;
  dragging?: boolean;
  radius?: number;
  onPointerDown?: (e: React.PointerEvent<SVGCircleElement>) => void;
  onPointerMove?: (e: React.PointerEvent<SVGCircleElement>) => void;
  onPointerUp?: (e: React.PointerEvent<SVGCircleElement>) => void;
  onPointerEnter?: (e: React.PointerEvent<SVGCircleElement>) => void;
  onPointerLeave?: (e: React.PointerEvent<SVGCircleElement>) => void;
}

function Orb({ id, t, disabled = false, selected = false, hovered = false, radius = 40, dragging = false, onPointerDown, onPointerMove, onPointerUp, onPointerEnter, onPointerLeave }: OrbProps) {
  const angle = t * 2 * Math.PI - Math.PI / 2;
  const cx = Math.cos(angle) * radius;
  const cy = Math.sin(angle) * radius;
  const orbRadius = selected ? 7 : 5;
  return (
    <circle
      data-slot="orb"
      data-orb-id={id}
      cx={cx}
      cy={cy}
      r={orbRadius}
      className={cn(
        dragging ? "" : "transition-all duration-150",
        disabled ? "fill-muted-foreground/40 cursor-not-allowed" : "fill-foreground cursor-grab active:cursor-grabbing",
        selected && !disabled && "fill-accent stroke-accent-foreground stroke-1",
        hovered && !disabled && !selected && "fill-accent-foreground",
      )}
      style={{ pointerEvents: disabled ? "none" : "auto" }}
      onPointerDown={disabled ? undefined : onPointerDown}
      onPointerMove={disabled ? undefined : onPointerMove}
      onPointerUp={disabled ? undefined : onPointerUp}
      onPointerEnter={disabled ? undefined : onPointerEnter}
      onPointerLeave={disabled ? undefined : onPointerLeave}
    />
  );
}

export { Orb };
export type { OrbProps };

// #endregion 🎄️Orb
