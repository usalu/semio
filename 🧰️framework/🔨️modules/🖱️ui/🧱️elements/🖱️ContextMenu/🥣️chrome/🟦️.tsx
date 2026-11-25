/** 🥣 Owns the shared context menu chrome frames that menus and ghost surfaces render inside. */
import * as React from "react";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { Icon, type IconSource } from "../../🔣️Icons/🟦️.tsx";
import { WindowChrome, windowChromeTitleChipClass } from "../../🗂️WindowChrome/🟦️.tsx";

/** 🪟️ Context menu frame: one rectangle with a glass title chip overlaid on its top row, no enlarge/close; the menu rows scroll beneath the chip. Forwards its ref to the outer window-chrome stack so callers (e.g. {@link ContextMenuController}'s on-screen clamp) can measure/adjust the rendered surface. */
export const ContextMenuChrome = reactHostPort.forwardRef<HTMLDivElement, { readonly title: string; readonly icon: IconSource; readonly children: React.ReactNode; readonly className?: string; readonly style?: React.CSSProperties }>(
  ({ title, icon, children, className, style }, ref) => {
    return (
      <WindowChrome
        ref={ref}
        active={true}
        capFitContent={true}
        level="menu"
        stackSlot="context-menu-content"
        className={cn("z-menu w-fit min-w-[10rem]", className)}
        style={style}
        titleChips={
          <div data-slot="context-menu-title-chip" className={cn(windowChromeTitleChipClass, "flex min-w-0 items-center gap-single")}>
            <Icon icon={icon} size="small" className="shrink-0" />
            <span className="truncate">{title}</span>
          </div>
        }
        body={children}
        bodyClassName="pointer-events-auto px-0"
      />
    );
  },
);

/** 🪜️ Submenu wing: a chipless rectangular frame placed so its edge coincides with its parent frame's edge; the shared border compositor resolves the coincident edge to one line. */
export const ContextMenuWingFrame = reactHostPort.forwardRef<HTMLDivElement, { readonly parentKey: string; readonly children: React.ReactNode; readonly className?: string; readonly style?: React.CSSProperties }>(
  ({ parentKey, children, className, style }, ref) => (
    <WindowChrome
      ref={ref}
      active={true}
      level="menu"
      stackSlot="context-menu-submenu"
      stackDataAttrs={{ "data-context-menu-submenu-of": parentKey }}
      className={cn("z-menu w-fit min-w-[10rem]", className)}
      style={style}
      body={children}
      bodyClassName="pointer-events-auto px-0"
    />
  ),
);
