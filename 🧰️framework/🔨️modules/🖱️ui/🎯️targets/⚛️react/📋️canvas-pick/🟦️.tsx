import { type CanvasPickRequest, type CanvasPickTarget, type CanvasHoverFocus, CANVAS_HOVER_SOURCE_CANVAS, CANVAS_HOVER_SOURCE_PICK_MENU } from "@semio-tech/framework-manifest/ui-contract-and-canvas-types";
import { sortCanvasPickTargetsGeneralFirst, canvasPickTargetKey, canvasHoverFocusFromTarget, pickMostSpecificCanvasTarget } from "@semio-tech/ui/modules/canvas-pick-targets";
import * as React from "react";
import type { UiLabel } from "@semio-tech/ui/elements/UiLabel";
import { useLabel } from "@semio-tech/ui/elements/Label";
import { useShellScopeOptional } from "@semio-tech/ui/elements/ShellScope";
import { cn } from "@semio-tech/ui/modules/class-name-composition";
import { floatingMenuItemClass } from "@semio-tech/ui/modules/chrome-class-names";
import { useSharedBorderFrameRef } from "@semio-tech/ui/elements/SharedBorderRow";
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

/** 🎯️ Fixed DOM pick list for overlapping canvas targets (not painted on the infinite canvas). */
export function CanvasPickMenu({ request, hoveredKey, onHoverKey, onPick, onDismiss, renderRow, title }: CanvasPickMenuProps): React.ReactNode {
  const menuRef = React.useRef<HTMLDivElement | null>(null);
  const frameRef = useSharedBorderFrameRef<HTMLDivElement>(menuRef);
  const selectTargetLabel = useLabel("ui.common.selectTarget");
  const resolvedTitle = title ?? selectTargetLabel;
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
      ref={frameRef}
      role="menu"
      data-level="menu"
      data-shared-border-frame=""
      data-shared-border-edges="top right bottom left"
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

type CanvasPoint = { readonly x: number; readonly y: number };
type CanvasPickKind = "hover" | "click";

export type UseCanvasPickInteractionOptions = {
  readonly onHoverFocus: (focus: CanvasHoverFocus) => void;
  readonly onSelectTarget: (target: CanvasPickTarget, request: CanvasPickRequest) => void;
  readonly clickThresholdPx?: number;
} & (
  | {
      readonly resolveTargetsAtClient: (client: CanvasPoint) => readonly CanvasPickTarget[];
      readonly resolveTargetsAtClientAsync?: never;
    }
  | {
      readonly resolveTargetsAtClient?: never;
      readonly resolveTargetsAtClientAsync: (client: CanvasPoint, signal: AbortSignal) => Promise<readonly CanvasPickTarget[]>;
    }
);

export type CanvasPickInteraction = {
  readonly pickMenu: CanvasPickRequest | null;
  readonly menuHoveredKey: string | null;
  readonly pickMenuOpen: boolean;
  readonly onCanvasPointerDown: (client: { readonly x: number; readonly y: number }) => void;
  readonly onCanvasPointerMove: (client: { readonly x: number; readonly y: number }) => void;
  readonly onCanvasPointerLeave: () => void;
  readonly onCanvasPointerCancel: () => void;
  readonly onCanvasPointerUp: (client: { readonly x: number; readonly y: number }, modifiers?: Readonly<Record<string, boolean>>) => void;
  readonly dismissPickMenu: () => void;
  readonly onMenuHoverKey: (key: string | null) => void;
  readonly onMenuPick: (target: CanvasPickTarget) => void;
};

/** 🧭️ Shared pointer routing for canvas hover (most-specific) and click disambiguation menus. */
export function useCanvasPickInteraction({ resolveTargetsAtClient, resolveTargetsAtClientAsync, onHoverFocus, onSelectTarget, clickThresholdPx = 4 }: UseCanvasPickInteractionOptions): CanvasPickInteraction {
  const [pickMenu, setPickMenu] = React.useState<CanvasPickRequest | null>(null);
  const [menuHoveredKey, setMenuHoveredKey] = React.useState<string | null>(null);
  const pointerDownRef = React.useRef<CanvasPoint | null>(null);
  const pendingRef = React.useRef<{ readonly kind: CanvasPickKind; readonly controller: AbortController } | null>(null);

  const cancelPending = React.useCallback(() => {
    const previous = pendingRef.current;
    pendingRef.current = null;
    previous?.controller.abort();
  }, []);
  React.useEffect(() => cancelPending, [cancelPending]);

  const resolvePick = React.useCallback((client: CanvasPoint, kind: CanvasPickKind, consume: (targets: readonly CanvasPickTarget[]) => void) => {
    cancelPending();
    if (resolveTargetsAtClient) {
      consume(resolveTargetsAtClient(client));
      return;
    }
    if (!resolveTargetsAtClientAsync) return;
    const request = { kind, controller: new AbortController() };
    pendingRef.current = request;
    void resolveTargetsAtClientAsync(client, request.controller.signal).then((targets) => {
      if (pendingRef.current !== request) return;
      pendingRef.current = null;
      consume(targets);
    }, () => {
      if (pendingRef.current === request) pendingRef.current = null;
    });
  }, [cancelPending, resolveTargetsAtClient, resolveTargetsAtClientAsync]);

  const dismissPickMenu = React.useCallback(() => {
    cancelPending();
    setPickMenu(null);
    setMenuHoveredKey(null);
    onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_CANVAS, null));
  }, [cancelPending, onHoverFocus]);

  /** 🖱️ A move with a button held is a DRAG, not a hover: the surface under the pointer is being
   * orbited, panned, marquee'd or wired, and the board that owns the gesture already tracks whatever
   * chrome the drag needs. Resolving hover here instead published a hover target per pointer move —
   * on the flow node-graph one guest `interactionHover` round trip per move, 16–18 of them in a 1 s
   * drag-pan (`📓️flow-scroll-render-perf-2026-09-15.md` §2). */
  const onCanvasPointerMove = React.useCallback((client: CanvasPoint) => {
    if (pickMenu || pointerDownRef.current || pendingRef.current?.kind === "click") return;
    resolvePick(client, "hover", (targets) => {
      onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_CANVAS, pickMostSpecificCanvasTarget(targets)));
    });
  }, [onHoverFocus, pickMenu, resolvePick]);

  const onCanvasPointerLeave = React.useCallback(() => {
    if (pickMenu) return;
    if (pendingRef.current?.kind === "hover") cancelPending();
    onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_CANVAS, null));
  }, [cancelPending, onHoverFocus, pickMenu]);

  const onCanvasPointerCancel = React.useCallback(() => {
    pointerDownRef.current = null;
    cancelPending();
    if (!pickMenu) onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_CANVAS, null));
  }, [cancelPending, onHoverFocus, pickMenu]);

  const onCanvasPointerDown = React.useCallback((client: CanvasPoint) => {
    cancelPending();
    pointerDownRef.current = client;
  }, [cancelPending]);

  const onCanvasPointerUp = React.useCallback((client: CanvasPoint, modifiers: Readonly<Record<string, boolean>> = {}) => {
    const start = pointerDownRef.current;
    pointerDownRef.current = null;
    if (start && Math.hypot(client.x - start.x, client.y - start.y) > clickThresholdPx) return;

    resolvePick(client, "click", (targets) => {
      if (targets.length === 0) {
        dismissPickMenu();
        return;
      }
      const request: CanvasPickRequest = { targets, client, modifiers };
      if (targets.length === 1) {
        dismissPickMenu();
        onSelectTarget(targets[0]!, request);
        return;
      }
      setPickMenu(request);
      const first = sortCanvasPickTargetsGeneralFirst(targets)[0]!;
      const key = canvasPickTargetKey(first);
      setMenuHoveredKey(key);
      onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_PICK_MENU, first));
    });
  }, [clickThresholdPx, dismissPickMenu, onHoverFocus, onSelectTarget, resolvePick]);

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
    onCanvasPointerCancel,
    onCanvasPointerUp,
    dismissPickMenu,
    onMenuHoverKey,
    onMenuPick,
    onCanvasPointerDown,
  };
}
