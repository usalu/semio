import type { IconName } from "@semio-tech/assets";
import * as React from "react";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { modeDockInactiveTabClass, modeDockTabLabelClassName, windowBodyFrameClass } from "../../../🔨️modules/🎨️chrome-class-names/🟦️.ts";
import { glassClass, surfaceClass } from "../../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { Icon } from "../../🔣️Icons/🟦️.tsx";
import { windowCapFrameClass } from "../../🗂️WindowChrome/🟦️.tsx";

export const modeDockTabInsertPreviewClass = "mx-half my-half flex h-[calc(100%-var(--spacing-single))] min-w-[5.5rem] max-w-[12rem] shrink-0 items-center rounded-sm border-2 border-accent bg-accent/20 px-single text-xs text-foreground/80 select-none";

export const MODE_DRAG_CURSOR_OFFSET_X = 8;
export const MODE_DRAG_CURSOR_OFFSET_Y = 10;

interface ModeDockDragPreviewProps {
  title: string;
  iconId?: IconName;
  content?: React.ReactNode;
  className?: string;
  style?: React.CSSProperties;
  tabOnly?: boolean;
}

/** 🪟️ Floating tab or window preview shown while docking. */
export const ModeDockDragPreview: React.FC<ModeDockDragPreviewProps> = ({ title, iconId, content, className, style, tabOnly = false }) =>
  tabOnly ? (
    <div data-slot="mode-dock-drag-preview" data-level="window" className={cn(modeDockInactiveTabClass, "pointer-events-none flex max-w-[12rem] shrink-0 items-center px-single text-xs text-element shadow-md select-none", glassClass, className)} style={style}>
      <div className={modeDockTabLabelClassName}>
        {iconId ? <Icon icon={iconId} size="small" className="shrink-0" /> : null}
        <span data-slot="inline-label" className="truncate">
          {title}
        </span>
      </div>
    </div>
  ) : (
    <div data-slot="mode-dock-drag-preview" className={cn("pointer-events-none flex flex-col overflow-hidden rounded shadow-lg", className)} style={style}>
      <div data-slot="mode-dock-drag-preview-cap" data-level="window" className={cn(windowCapFrameClass, "relative z-[2] flex h-medium shrink-0 items-stretch px-single", glassClass)}>
        <div className={modeDockTabLabelClassName}>
          {iconId ? <Icon icon={iconId} size="small" className="shrink-0" /> : null}
          <span data-slot="inline-label" className="truncate">
            {title}
          </span>
        </div>
      </div>
      <div data-slot="mode-dock-drag-preview-body" data-level="base" className={cn("relative min-h-0 flex-1 overflow-hidden p-[var(--padding-standard)] opacity-95", windowBodyFrameClass)}>
        {content ? (
          <div data-level="window" className={cn("h-full w-full overflow-hidden [&_*]:pointer-events-none", surfaceClass)}>
            {content}
          </div>
        ) : null}
      </div>
    </div>
  );
