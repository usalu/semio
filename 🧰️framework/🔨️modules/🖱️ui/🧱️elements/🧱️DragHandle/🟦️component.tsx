// #region 🧲️Header
// 💻️ framework/ui/elements/🧱️DragHandle/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️component.ts";
import { ChromeControlHint } from "../💡️ChromeControlHint/🟦️component.tsx";
import { GripVerticalIcon, MoveIcon } from "../🔣️Icons/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🫳️DragAffordance
/** @emoji data-hover-scope attr for DragHandle hover exclusion. */
export const HANDLE_HOVER_SCOPE_ATTR = "data-hover-scope";


/**
 * @emoji 🫳️ Universal grip that starts a drag — pass `onPointerDown` for pointer-capture drags (and optionally the rest of {@link usePointerDrag}'s handlers), spread dnd-kit `attributes`/`listeners`, or use as a pure affordance on whole-surface draggables.
 * `emphasized` mirrors the ambient active/ready state of the element it belongs to.
 * Parent hover emphasis is CSS: `[data-hover-scope]:hover [data-slot="drag-handle"]` in `🎨️ui.css` — the grip paints its own muted color at rest and cannot inherit `hover:text-emphasized` from the label/icon beside it.
 */
export const DragHandle: React.FC<{
  readonly labelId?: string;
  readonly iconKind?: "grip-vertical" | "move";
  readonly onPointerDown?: React.PointerEventHandler<HTMLSpanElement>;
  readonly onPointerMove?: React.PointerEventHandler<HTMLSpanElement>;
  readonly onPointerUp?: React.PointerEventHandler<HTMLSpanElement>;
  readonly onPointerCancel?: React.PointerEventHandler<HTMLSpanElement>;
  readonly attributes?: object;
  readonly listeners?: Record<string, unknown>;
  readonly onClick?: React.MouseEventHandler<HTMLSpanElement>;
  readonly className?: string;
  readonly emphasized?: boolean;
}> = ({ labelId = "ui.tree.drag.sort", iconKind = "grip-vertical", onPointerDown, onPointerMove, onPointerUp, onPointerCancel, attributes, listeners, onClick, className, emphasized = false }) => (
  <ChromeControlHint id={labelId}>
    <span
      data-slot="drag-handle"
      data-drag-role={iconKind === "move" ? "transfer" : "sort"}
      className={cn("inline-flex shrink-0 cursor-grab touch-none items-center justify-center transition-colors hover:text-emphasized active:cursor-grabbing", emphasized ? "text-emphasized" : "text-muted-foreground", className)}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onPointerCancel={onPointerCancel}
      onClick={onClick}
      onPointerEnter={(event) => {
        event.currentTarget.closest(`[${HANDLE_HOVER_SCOPE_ATTR}]`)?.setAttribute("data-handle-hovered", "true");
      }}
      onPointerLeave={(event) => {
        event.currentTarget.closest(`[${HANDLE_HOVER_SCOPE_ATTR}]`)?.removeAttribute("data-handle-hovered");
      }}
      {...(attributes as React.ComponentProps<"span">)}
      {...(listeners as React.ComponentProps<"span">)}
    >
      {iconKind === "move" ? <MoveIcon size={12} /> : <GripVerticalIcon size={12} />}
    </span>
  </ChromeControlHint>
);

/** @emoji 🎯️ Passive drop-zone fill — secondary accent, kept visually distinct from the stronger primary-accent indicator on the actively hovered target. */



// #endregion 🫳️DragAffordance
