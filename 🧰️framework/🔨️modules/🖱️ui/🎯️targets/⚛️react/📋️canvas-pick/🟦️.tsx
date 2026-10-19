import { type CanvasPickRequest, type CanvasPickTarget, type CanvasHoverFocus, CANVAS_HOVER_SOURCE_CANVAS, CANVAS_HOVER_SOURCE_PICK_MENU } from "../../../../🛂️manifest/🏷️ui-contract-and-canvas-types/🟦️.ts";
import { sortCanvasPickTargetsGeneralFirst, canvasPickTargetKey, canvasHoverFocusFromTarget, pickMostSpecificCanvasTarget } from "../../../🔨️modules/🎯️canvas-pick-targets/🟦️.ts";
import * as React from "react";
import type { UiLabel } from "../../../🧱️elements/🎗️UiLabel/🟦️.tsx";
import { useLabel } from "../../../🧱️elements/🏷️Label/🟦️.tsx";
import { useShellScopeOptional } from "../../../🧱️elements/🐚️ShellScope/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { floatingMenuItemClass } from "../../../🔨️modules/🎨️chrome-class-names/🟦️.ts";
import { createPortal } from "react-dom";
import { floatingMenuSurfaceClass } from "../🎨️chrome-classes/🟦️.ts";

export type CanvasPickMenuProps = {
  readonly request: CanvasPickRequest | null;
  readonly hoveredKey: string | null;
  readonly onHoverKey: (key: string | null) => void;
  readonly onPick: (target: CanvasPickTarget) => void;
  readonly onDismiss: () => void;
  readonly renderRow?: (target: CanvasPickTarget, active: boolean) => React.ReactNode;
  readonly title?: UiLabel;
};

/** @emoji 🎯️ Fixed DOM pick list for overlapping canvas targets (not painted on the infinite canvas). */
export function CanvasPickMenu({ request, hoveredKey, onHoverKey, onPick, onDismiss, renderRow, title }: CanvasPickMenuProps): React.ReactNode {
  const menuRef = React.useRef<HTMLDivElement | null>(null);
  const selectTargetLabel = useLabel("ui.common.selectTarget");
  const resolvedTitle = title ?? selectTargetLabel;
  // 🐚️ Falls back to `document.body` outside any shell — inside one, portals into that shell's own
  // overlay layer instead so this menu never visually escapes into another mounted shell's stacking context.
  const shellScope = useShellScopeOptional();

  React.useEffect(() => {
    if (!request) return;
    const onPointerDown = (event: PointerEvent) => {
      const menu = menuRef.current;
      if (event.target instanceof globalThis.Node && menu?.contains(event.target)) return;
      onDismiss();
    };
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") onDismiss();
    };
    window.addEventListener("pointerdown", onPointerDown, true);
    window.addEventListener("keydown", onKeyDown, true);
    return () => {
      window.removeEventListener("pointerdown", onPointerDown, true);
      window.removeEventListener("keydown", onKeyDown, true);
    };
  }, [onDismiss, request]);

  if (!request || request.targets.length === 0) return null;
  const sorted = sortCanvasPickTargetsGeneralFirst(request.targets);
  const body = sorted.map((target) => {
    const key = canvasPickTargetKey(target);
    const active = hoveredKey === key;
    return (
      <button
        key={key}
        type="button"
        role="menuitem"
        aria-selected={active}
        className={cn(floatingMenuItemClass, active && "bg-active-base text-emphasized")}
        onPointerEnter={() => onHoverKey(key)}
        onPointerLeave={() => onHoverKey(null)}
        onPointerDown={(event) => {
          event.preventDefault();
          event.stopPropagation();
          onPick(target);
        }}
      >
        {renderRow ? (
          renderRow(target, active)
        ) : (
          <>
            <span className="text-muted-foreground">{target.domain}</span> <code className="text-foreground">{target.label ?? target.id}</code>
          </>
        )}
      </button>
    );
  });
  if (typeof document === "undefined") return null;
  return createPortal(
    <div
      ref={menuRef}
      role="menu"
      className={cn("fixed z-tutorial w-layout-floating-menu-sm max-h-layout-preview-md overflow-y-auto p-single", floatingMenuSurfaceClass)}
      style={{
        left: Math.min(request.client.x + 8, window.innerWidth - 230),
        top: Math.min(request.client.y + 8, window.innerHeight - 220),
      }}
      onPointerDown={(event) => event.stopPropagation()}
    >
      <div className="text-muted-foreground px-single py-half text-2xs">{resolvedTitle}</div>
      {body}
    </div>,
    shellScope?.portalLayerRef.current ?? document.body,
  );
}

export type UseCanvasPickInteractionOptions = {
  readonly resolveTargetsAtClient: (client: { readonly x: number; readonly y: number }) => readonly CanvasPickTarget[];
  readonly onHoverFocus: (focus: CanvasHoverFocus) => void;
  readonly onSelectTarget: (target: CanvasPickTarget, request: CanvasPickRequest) => void;
  readonly clickThresholdPx?: number;
};

export type CanvasPickInteraction = {
  readonly pickMenu: CanvasPickRequest | null;
  readonly menuHoveredKey: string | null;
  readonly pickMenuOpen: boolean;
  readonly onCanvasPointerDown: (client: { readonly x: number; readonly y: number }) => void;
  readonly onCanvasPointerMove: (client: { readonly x: number; readonly y: number }) => void;
  readonly onCanvasPointerLeave: () => void;
  readonly onCanvasPointerUp: (client: { readonly x: number; readonly y: number }, modifiers?: Readonly<Record<string, boolean>>) => void;
  readonly dismissPickMenu: () => void;
  readonly onMenuHoverKey: (key: string | null) => void;
  readonly onMenuPick: (target: CanvasPickTarget) => void;
};

/** @emoji 🎯️ Shared pointer routing for canvas hover (most-specific) and click disambiguation menus. */
export function useCanvasPickInteraction({ resolveTargetsAtClient, onHoverFocus, onSelectTarget, clickThresholdPx = 4 }: UseCanvasPickInteractionOptions): CanvasPickInteraction {
  const [pickMenu, setPickMenu] = React.useState<CanvasPickRequest | null>(null);
  const [menuHoveredKey, setMenuHoveredKey] = React.useState<string | null>(null);
  const pointerDownRef = React.useRef<{ readonly x: number; readonly y: number } | null>(null);

  const dismissPickMenu = React.useCallback(() => {
    setPickMenu(null);
    setMenuHoveredKey(null);
    onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_CANVAS, null));
  }, [onHoverFocus]);

  /** 🖱️ A move with a button held is a DRAG, not a hover: the surface under the pointer is being
   * orbited, panned, marquee'd or wired, and the board that owns the gesture already tracks whatever
   * chrome the drag needs. Resolving hover here instead published a hover target per pointer move —
   * on the flow node-graph one guest `interactionHover` round trip per move, 16–18 of them in a 1 s
   * drag-pan (`📓️flow-scroll-render-perf-2026-09-15.md` §2). */
  const onCanvasPointerMove = React.useCallback(
    (client: { readonly x: number; readonly y: number }) => {
      if (pickMenu) return;
      if (pointerDownRef.current) return;
      const targets = resolveTargetsAtClient(client);
      const specific = pickMostSpecificCanvasTarget(targets);
      onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_CANVAS, specific));
    },
    [onHoverFocus, pickMenu, resolveTargetsAtClient],
  );

  const onCanvasPointerLeave = React.useCallback(() => {
    if (pickMenu) return;
    onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_CANVAS, null));
  }, [onHoverFocus, pickMenu]);

  const onCanvasPointerDown = React.useCallback((client: { readonly x: number; readonly y: number }) => {
    pointerDownRef.current = client;
  }, []);

  const onCanvasPointerUp = React.useCallback(
    (client: { readonly x: number; readonly y: number }, modifiers: Readonly<Record<string, boolean>> = {}) => {
      const start = pointerDownRef.current;
      pointerDownRef.current = null;
      if (start) {
        const dx = client.x - start.x;
        const dy = client.y - start.y;
        if (Math.hypot(dx, dy) > clickThresholdPx) return;
      }
      const targets = resolveTargetsAtClient(client);
      if (targets.length === 0) {
        dismissPickMenu();
        return;
      }
      if (targets.length === 1) {
        dismissPickMenu();
        onSelectTarget(targets[0]!, { targets, client, modifiers });
        return;
      }
      const request: CanvasPickRequest = { targets, client, modifiers };
      setPickMenu(request);
      const first = sortCanvasPickTargetsGeneralFirst(targets)[0]!;
      const key = canvasPickTargetKey(first);
      setMenuHoveredKey(key);
      onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_PICK_MENU, first));
    },
    [clickThresholdPx, dismissPickMenu, onHoverFocus, onSelectTarget, resolveTargetsAtClient],
  );

  const onMenuHoverKey = React.useCallback(
    (key: string | null) => {
      setMenuHoveredKey(key);
      if (!pickMenu) return;
      const target = key ? (pickMenu.targets.find((row) => canvasPickTargetKey(row) === key) ?? null) : null;
      onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_PICK_MENU, target));
    },
    [onHoverFocus, pickMenu],
  );

  const onMenuPick = React.useCallback(
    (target: CanvasPickTarget) => {
      if (!pickMenu) return;
      onSelectTarget(target, pickMenu);
      dismissPickMenu();
    },
    [dismissPickMenu, onSelectTarget, pickMenu],
  );

  return {
    pickMenu,
    menuHoveredKey,
    pickMenuOpen: pickMenu !== null,
    onCanvasPointerMove,
    onCanvasPointerLeave,
    onCanvasPointerUp,
    dismissPickMenu,
    onMenuHoverKey,
    onMenuPick,
    onCanvasPointerDown,
  };
}
