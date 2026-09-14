// #region Header
// framework/ui/elements/💡️ChromeControlHint/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// Licensed under LGPL-3.0-or-later.
// #endregion Header

// #region 🔌️Adapters
import * as React from "react";
import { createPortal } from "react-dom";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { glassClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { useFlow } from "../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import { useControlAccessibleLabel, useControlTooltipText } from "../🏷️Label/🟦️.tsx";
import { useShellScopeOptional } from "../🐚️ShellScope/🟦️.tsx";
import { SurfaceScope } from "../🌈️Surface/🟦️.tsx";
import { resolvePopoverPlacement } from "../🗨️Popover/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 💡️ChromeControlHint
/** @emoji ⏱️ Hover delay before a chrome control tooltip opens — matches dissolved Radix provider default. */
export const CHROME_CONTROL_TOOLTIP_DELAY_MS = 400;

const chromeControlTooltipSurfaceClass = cn(
  "pointer-events-none fixed z-menu w-max max-w-sm border p-single text-xs text-balance text-popover-foreground shadow-lg",
  glassClass,
);

/** @emoji 🏷️ Glass menu-tier hover tooltip for chrome controls (outer wrapper avoids Radix-style ref merge loops). */
export function ChromeControlHint({ id, text, always = false, children }: { readonly id?: string; readonly text?: string; readonly always?: boolean; readonly children: React.ReactElement }): React.ReactNode {
  const accessibleLabel = useControlAccessibleLabel(id, text);
  const tooltipText = useControlTooltipText(id, text, { always });
  const flow = useFlow();
  const shellScope = useShellScopeOptional();
  const tooltipId = React.useId().replace(/[^A-Za-z0-9_-]/g, "");
  const triggerRef = React.useRef<HTMLSpanElement>(null);
  const contentRef = React.useRef<HTMLDivElement>(null);
  const [open, setOpen] = React.useState(false);
  const [placement, setPlacement] = React.useState<{ left: number; top: number; transformOrigin: string } | null>(null);
  const openTimerRef = React.useRef<ReturnType<typeof setTimeout> | null>(null);

  const clearOpenTimer = React.useCallback(() => {
    if (openTimerRef.current !== null) {
      clearTimeout(openTimerRef.current);
      openTimerRef.current = null;
    }
  }, []);

  const scheduleOpen = React.useCallback(() => {
    if (!tooltipText) return;
    clearOpenTimer();
    openTimerRef.current = setTimeout(() => setOpen(true), CHROME_CONTROL_TOOLTIP_DELAY_MS);
  }, [clearOpenTimer, tooltipText]);

  const close = React.useCallback(() => {
    clearOpenTimer();
    setOpen(false);
  }, [clearOpenTimer]);

  React.useLayoutEffect(() => {
    if (!open || !tooltipText) {
      setPlacement(null);
      return;
    }
    const trigger = triggerRef.current;
    const content = contentRef.current;
    if (!trigger || !content) return;
    const anchor = trigger.getBoundingClientRect();
    const measured = content.getBoundingClientRect();
    const resolved = resolvePopoverPlacement(anchor, measured, { width: window.innerWidth, height: window.innerHeight }, "top", "center", 8, 0, 8, flow.inline === "rtl", true);
    setPlacement({ left: resolved.left, top: resolved.top, transformOrigin: resolved.transformOrigin });
  }, [flow.inline, open, tooltipText]);

  React.useEffect(() => () => clearOpenTimer(), [clearOpenTimer]);

  if (!React.isValidElement(children)) return children;
  if (!accessibleLabel && !tooltipText) return children;

  const childProps = children.props as { readonly title?: string; readonly "aria-label"?: string; readonly "aria-describedby"?: string };
  const describedBy = open && tooltipText ? tooltipId : childProps["aria-describedby"];
  const hintedChild = React.cloneElement(children, {
    title: tooltipText ? undefined : childProps.title,
    "aria-label": childProps["aria-label"] ?? accessibleLabel,
    "aria-describedby": describedBy,
  } as Record<string, unknown>);

  const portalTarget = shellScope?.portalLayerRef.current ?? (typeof document !== "undefined" ? document.body : null);
  const tooltipPortal =
    open && tooltipText && portalTarget
      ? createPortal(
          <div
            ref={contentRef}
            id={tooltipId}
            role="tooltip"
            data-slot="tooltip-content"
            data-level="menu"
            dir={flow.inline === "rtl" ? "rtl" : undefined}
            className={chromeControlTooltipSurfaceClass}
            style={
              placement
                ? { left: placement.left, top: placement.top, transformOrigin: placement.transformOrigin, visibility: "visible" }
                : { left: 0, top: 0, visibility: "hidden" }
            }
          >
            <SurfaceScope level="menu" fill="glass">
              {tooltipText}
            </SurfaceScope>
          </div>,
          portalTarget,
        )
      : null;

  return (
    <>
      <span
        ref={triggerRef}
        data-slot="chrome-control-hint"
        className="inline-flex max-w-full"
        onPointerEnter={scheduleOpen}
        onPointerLeave={close}
        onPointerCancel={close}
        onFocusCapture={scheduleOpen}
        onBlurCapture={close}
      >
        {hintedChild}
      </span>
      {tooltipPortal}
    </>
  );
}
// #endregion 💡️ChromeControlHint
