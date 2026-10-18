import type { IconName } from "@semio-tech/assets";
import * as React from "react";
import { useControlInlineText, useControlTooltipText } from "../../🏷️Label/🟦️.tsx";
import { useUiDriverDragSurface } from "../../🚗️UiDriver/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { windowPaneChromeToggleClass } from "../../../🔨️modules/🎛️chrome-control-presentation/🟦️.ts";
import { panelTabIconSlotClass, panelTabLabelClass } from "../../../🔨️modules/🎨️chrome-class-names/🟦️.ts";
import { renderControlIcon } from "../../🔣️Icons/🟦️.tsx";
import { DragHandle } from "../../🧱️DragHandle/🟦️.tsx";

/** 🪟️ Props for {@link WindowPaneChromeToggle} — the panel-toggle twin for every window pane header. */
export interface WindowPaneChromeToggleProps {
  readonly id: string;
  readonly icon: IconName;
  readonly label: string;
  readonly onClick?: () => void;
  readonly disabled?: boolean;
  readonly className?: string;
  /** 🫳️ Pointer-drag props forwarded to the trailing {@link DragHandle} (omit when the pane is not re-anchorable yet). */
  readonly dragPointerProps?: Pick<React.HTMLAttributes<HTMLSpanElement>, "onPointerCancel" | "onPointerDown" | "onPointerMove" | "onPointerUp">;
  readonly showDragHandle?: boolean;
  readonly emphasized?: boolean;
}

/** 🪟️ Pane chrome toggle matching panel toggles: leading semantic icon, label, trailing {@link DragHandle} — never a fold-direction chevron. */
export const WindowPaneChromeToggle: React.FC<WindowPaneChromeToggleProps> = ({ id, icon, label, onClick, disabled, className, dragPointerProps, showDragHandle = true, emphasized = false }) => {
  const inlineText = useControlInlineText(id, label);
  const tooltipText = useControlTooltipText(id, label);
  const surfaceDrag = useUiDriverDragSurface();
  const canDrag = showDragHandle && Boolean(dragPointerProps);
  return (
    <button
      type="button"
      id={id}
      data-slot="window-pane-chrome-toggle"
      data-hover-scope
      title={tooltipText}
      disabled={disabled}
      onClick={onClick}
      className={cn(windowPaneChromeToggleClass, className)}
      {...(canDrag && surfaceDrag ? dragPointerProps : {})}
    >
      <span className={panelTabIconSlotClass}>{renderControlIcon(icon, "tiny")}</span>
      {inlineText !== undefined ? (
        <span data-slot="inline-label" className={panelTabLabelClass}>
          {inlineText}
        </span>
      ) : null}
      {showDragHandle && !surfaceDrag ? <DragHandle labelId="ui.tree.drag.sort" subject={label} {...dragPointerProps} onClick={(event) => event.stopPropagation()} emphasized={emphasized} /> : null}
    </button>
  );
};
