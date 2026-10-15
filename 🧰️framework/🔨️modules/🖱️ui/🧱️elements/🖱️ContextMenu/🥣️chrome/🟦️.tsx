/** 🥣 Owns the shared context menu chrome frame that menus and ghost surfaces render inside. */
import * as React from "react";
import { reactHostPort } from "../../🔌️Ports/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { Icon, type IconSource } from "../../🔣️Icons/🟦️.tsx";
import { WindowChrome, windowChromeTitleChipClass } from "../../🗂️WindowChrome/🟦️.tsx";

/** @emoji 🪟️ Context menu with U-cutout chrome — title chip only, no enlarge/close; gap punches through. Forwards its ref to the outer window-chrome stack so callers (e.g. {@link ContextMenuController}'s on-screen clamp) can measure/adjust the rendered surface. */
export const ContextMenuChrome = reactHostPort.forwardRef<HTMLDivElement, { readonly title: string; readonly icon: IconSource; readonly children: React.ReactNode; readonly className?: string; readonly style?: React.CSSProperties }>(
  ({ title, icon, children, className, style }, ref) => {
    return (
      <WindowChrome
        ref={ref}
        active={true}
        capFitContent={true}
        level="menu"
        stackSlot="context-menu-content"
        className={cn("z-menu w-fit min-w-[10rem] max-h-layout-command overflow-y-auto", className)}
        style={style}
        titleChips={
          <div data-slot="context-menu-title-chip" className={cn(windowChromeTitleChipClass, "flex min-w-0 items-center gap-single")}>
            <Icon icon={icon} size="small" className="shrink-0" />
            <span className="truncate">{title}</span>
          </div>
        }
        body={children}
        bodyClassName="pointer-events-auto w-fit overflow-visible p-single"
      />
    );
  },
);
