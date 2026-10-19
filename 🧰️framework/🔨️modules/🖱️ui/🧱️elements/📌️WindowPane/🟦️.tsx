/** 📑 Owns the window pane host and the resizable pane that window chrome stacks above the body. */
import { uiSpacingPx } from "@semio-tech/ui-styling";
import * as React from "react";
import { type IconName } from "@semio-tech/assets";
import { reactHostPort } from "../🔌️Ports/⚛️react/🟦️.ts";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { type UiLabel } from "../🎗️UiLabel/🟦️.tsx";
import { LevelProvider, getLevelZClass, useSurfaceActive } from "../🌈️Surface/🟦️.tsx";
import { FlowProvider } from "../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import { WindowChrome } from "../🗂️WindowChrome/🟦️.tsx";
import { childElementId } from "../🆔️ElementId/🟦️.tsx";
import { WindowPaneChromeToggle } from "../🌳️Tree/🔀️pane-chrome-toggle/🟦️.tsx";
import { usePointerDrag } from "../🖼️Panel/🟦️.tsx";
import { useUiMobile } from "../../🔨️modules/📱️ui-mobile-context/🟦️.tsx";
import { type Anchor, anchorHorizontal, anchorPositionStyle, flowFromAnchor, nearestAnchor } from "../../🔨️modules/⚓️panel-anchor/🟦️.ts";
import { chromePanelSafeAreaStyle, useChromePanelSafeArea } from "../../🔨️modules/🛟️chrome-safe-area/🟦️.tsx";

interface PaneHostContextValue {
  /** 🎯️ Imperative bounds source for drag math (read on demand, well after mount — never during another component's render). */
  readonly containerRef: React.RefObject<HTMLDivElement | null>;
  /** 🌱️ Reactive mirror of the same node, `null` until mount — {@link usePaneSlot} portals need a render-time value, and a ref's `.current` isn't populated yet during the same pass a child first renders alongside its ref owner. */
  readonly container: HTMLDivElement | null;
}

const PaneHostContext = reactHostPort.createContext<PaneHostContextValue | undefined>(undefined);

/** 🪟️ The nearest {@link PaneHost}, or `undefined` outside one — {@link Pane} drag-to-reanchor and {@link usePaneSlot} both need it. */
export function usePaneHostContext(): PaneHostContextValue | undefined {
  return reactHostPort.useContext(PaneHostContext);
}

/**
 * 🖥️ PaneHostProps holds the data fields for a PaneHostProps record.
 **/
export interface PaneHostProps {
  readonly className?: string;
  readonly children?: React.ReactNode;
}

/** 🪟️ Bounds box for floating panes: mount one inside a window body (or any floating-chrome host) so every
 * {@link Pane} inside — direct JSX children or portaled in via {@link usePaneSlot} — shares one anchor coordinate
 * space to drag between and one DOM parent to measure drag drops against.
 *
 * Children live as siblings of the portal mount (not inside the `pointer-events-none` overlay) so window canvas
 * content keeps normal hit-testing while still receiving {@link PaneHostContext}.
 *
 * The root is its own stacking context (`isolate`): body content's layer numbers stay inside it and can never
 * out-stack the window chrome panes (`Actions`, `Utilities`, `Window Options`, search) that sit beside it at
 * `--z-pane`. A canvas host's full-bleed pointer overlay at `z-30` (raster's `Paint2dHost`) otherwise hit-tested
 * above both `Actions` chips of every raster window inside `s`, so the rail could not be opened by a pointer
 * (ticket 26/09/23 S15; same law as `NODE_GRAPH_HOST_CLASS`). */
export const PaneHost: React.FC<PaneHostProps> = ({ className, children }) => {
  const containerRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [container, setContainer] = reactHostPort.useState<HTMLDivElement | null>(null);
  const setRef = reactHostPort.useCallback((element: HTMLDivElement | null) => {
    containerRef.current = element;
    setContainer(element);
  }, []);
  const contextValue = reactHostPort.useMemo((): PaneHostContextValue => ({ containerRef, container }), [container]);
  return (
    <PaneHostContext.Provider value={contextValue}>
      <div data-slot="pane-host-root" className={cn("relative isolate h-full min-h-0 min-w-0 w-full", className)}>
        {children}
        <div ref={setRef} data-slot="pane-host" className="pointer-events-none absolute inset-0" />
      </div>
    </PaneHostContext.Provider>
  );
};

/** ↔ Pane resize handle — same sign convention as {@link PanelResizeHandle}: dragging the right-side handle right (or the left-side handle left) grows the pane. Middle anchors pass `deltaFactor={2}` so each edge contributes half the visual grow. */
function PaneResizeHandle({
  side,
  size,
  minSize,
  maxSize,
  onSizeChange,
  onActiveChange,
  deltaFactor = 1,
}: {
  readonly side: "left" | "right";
  readonly size: number;
  readonly minSize: number;
  readonly maxSize: number;
  readonly onSizeChange: (size: number) => void;
  readonly onActiveChange?: (active: boolean) => void;
  readonly deltaFactor?: number;
}) {
  const startRef = reactHostPort.useRef<{ pointerX: number; size: number } | null>(null);
  const pointerProps = usePointerDrag<HTMLDivElement>({
    onStart: (event) => {
      event.preventDefault();
      startRef.current = { pointerX: event.clientX, size };
      onActiveChange?.(true);
    },
    onMove: (event) => {
      const start = startRef.current;
      if (!start) return;
      const next = start.size + (side === "right" ? 1 : -1) * deltaFactor * (event.clientX - start.pointerX);
      if (next >= minSize && next <= maxSize) onSizeChange(next);
    },
    onEnd: () => {
      startRef.current = null;
      onActiveChange?.(false);
    },
    onCancel: () => {
      startRef.current = null;
      onActiveChange?.(false);
    },
  });
  return <div data-slot="pane-resize-handle" className={`absolute top-0 bottom-0 z-20 ${side === "left" ? "left-0" : "right-0"} w-single cursor-ew-resize`} {...pointerProps} />;
}

/**
 * 🔲️ Props interface for the Pane component.
 **/
export interface PaneProps {
  /** 🆔️ The pane container's own DOM id — rendered on the overlay root, and the stem every derived chrome id (`<id>.pane.fold`) falls back to. */
  readonly id: string;
  readonly anchor: Anchor;
  /** 🧭️ Fires while dragging the pane's handle, once per anchor crossed — omit to make the pane fixed (drag handle still renders as a pure affordance, matching panel toggles). */
  readonly onAnchorChange?: (anchor: Anchor) => void;
  readonly folded?: boolean;
  readonly onFoldToggle?: () => void;
  /** 🖼️ Fixed semantic icon for the pane chrome toggle — never a fold-direction chevron. */
  readonly icon: IconName;
  readonly label?: UiLabel;
  readonly size?: number;
  readonly onSizeChange?: (size: number) => void;
  readonly minSize?: number;
  readonly maxSize?: number;
  readonly resizable?: boolean;
  /** 🪣️ ⛶️ Fill the {@link PaneHost} (programmatic span across the host). */
  readonly expanded?: boolean;
  readonly overlaySlot?: string;
  readonly overlayRef?: React.Ref<HTMLDivElement>;
  readonly stackSlot?: string;
  readonly bodySlot?: string;
  readonly bodyClassName?: string;
  readonly bodyStyle?: React.CSSProperties;
  readonly stackClassName?: string;
  readonly stackDataAttrs?: Record<string, string | undefined>;
  readonly toggleId?: string;
  readonly toggleDisabled?: boolean;
  readonly dimWhenOpen?: boolean;
  readonly onResizeActiveChange?: (active: boolean) => void;
  /** 🗂️ Stacking order among panes sharing one anchor — lower first, in this anchor's flow direction. */
  readonly order?: number;
  /** 🛟️ Extra inline inset on this anchor's own edge — see {@link chromePanelSafeArea}. */
  readonly inlineEdgeReservePx?: number;
  /** 🪟️ Explicit bounds for panes mounted beside, rather than inside, their host context. */
  readonly hostRef?: React.RefObject<HTMLElement | null>;
  /** 🛟️ Minimum displacement-axis clearance from shell panels in CSS pixels. */
  readonly chromePanelGapPx?: number;
  readonly zIndex?: 10 | 20 | 30 | 40;
  readonly className?: string;
  readonly children?: React.ReactNode;
}

const PANE_DEFAULT_SIZE = 300;

const PANE_DEFAULT_MIN_SIZE = 200;

const PANE_DEFAULT_MAX_SIZE = 600;

/** 🪟️ One floating pane inside a {@link PaneHost} — anchored via the exact same {@link anchorPositionStyle}/
 * {@link flowFromAnchor} math as {@link Panel}, foldable to a chip, optionally width-resizable (inner edge for
 * corners / side-middle; both edges for top/bottom-middle, matching panel grow), and (given `onAnchorChange`)
 * draggable by its handle to any of the eight anchors via {@link nearestAnchor} — dropped anywhere inside the
 * enclosing {@link PaneHost}, not just on the target slot, since the box is the only drop target there is.
 * Chrome matches panel toggles: semantic {@link icon} + label + trailing {@link DragHandle}; fold/unfold is the title chip only. */
export const Pane: React.FC<PaneProps> = ({
  id,
  anchor,
  onAnchorChange,
  folded = true,
  onFoldToggle,
  icon,
  label,
  size = PANE_DEFAULT_SIZE,
  onSizeChange,
  minSize = PANE_DEFAULT_MIN_SIZE,
  maxSize = PANE_DEFAULT_MAX_SIZE,
  resizable: resizableProp,
  expanded = false,
  overlaySlot,
  overlayRef,
  stackSlot = "window-chrome-stack",
  bodySlot = "pane-body",
  bodyClassName = "overflow-y-auto p-single",
  bodyStyle,
  stackClassName,
  stackDataAttrs,
  toggleId,
  toggleDisabled,
  dimWhenOpen = false,
  onResizeActiveChange,
  order = 0,
  inlineEdgeReservePx = 0,
  hostRef,
  chromePanelGapPx = uiSpacingPx(1),
  zIndex,
  className = "",
  children,
}) => {
  const host = usePaneHostContext();
  const mobile = useUiMobile();
  const paneRootRef = reactHostPort.useRef<HTMLDivElement>(null);
  const fallbackHostRef = reactHostPort.useRef<HTMLElement | null>(null);
  const paneHostRef = hostRef ?? host?.containerRef ?? fallbackHostRef;
  const safeArea = useChromePanelSafeArea({ hostRef: paneHostRef, affordanceRef: paneRootRef, anchor, yieldAxis: "either", gapPx: chromePanelGapPx, enabled: !expanded && !mobile });
  const setPaneRootRef = reactHostPort.useCallback(
    (element: HTMLDivElement | null) => {
      paneRootRef.current = element;
      if (typeof overlayRef === "function") overlayRef(element);
      else if (overlayRef) (overlayRef as React.MutableRefObject<HTMLDivElement | null>).current = element;
    },
    [overlayRef],
  );
  const [surfaceActive, surfaceActiveProps] = useSurfaceActive(paneRootRef);
  const effectiveFolded = folded;
  const flow = flowFromAnchor(anchor);
  const horizontal = anchorHorizontal(anchor);
  const [dragging, setDragging] = reactHostPort.useState(false);
  const lastAnchorRef = reactHostPort.useRef(anchor);
  lastAnchorRef.current = anchor;
  const resizable = resizableProp ?? Boolean(onSizeChange);
  const stopPointerPropagation = reactHostPort.useCallback((event: React.PointerEvent<HTMLDivElement>) => event.stopPropagation(), []);

  const dragPointerProps = usePointerDrag<HTMLSpanElement>({
    onStart: () => {
      if (!onAnchorChange) return;
      setDragging(true);
    },
    onMove: (event) => {
      if (!onAnchorChange) return;
      const hostRect = paneHostRef.current?.getBoundingClientRect();
      if (!hostRect) return;
      const next = nearestAnchor(event.clientX, event.clientY, hostRect);
      if (next !== lastAnchorRef.current) {
        lastAnchorRef.current = next;
        onAnchorChange(next);
      }
    },
    onEnd: () => setDragging(false),
    onCancel: () => setDragging(false),
  });

  const positionStyle: React.CSSProperties = expanded
    ? { position: "absolute", inset: 0, zIndex: zIndex, width: "100%", height: "100%", maxWidth: "100%", maxHeight: "100%" }
    : {
        ...anchorPositionStyle(anchor),
        order,
        ...(zIndex !== undefined ? { zIndex } : {}),
        width: !mobile && !effectiveFolded ? `${size}px` : undefined,
        maxWidth: !mobile && !effectiveFolded ? `min(100% - (var(--spacing-single) * 2)${inlineEdgeReservePx > 0 ? ` - ${inlineEdgeReservePx}px` : ""}, ${size}px)` : undefined,
        ...chromePanelSafeAreaStyle(anchor, { inlinePx: inlineEdgeReservePx + safeArea.inlinePx, blockPx: safeArea.blockPx }),
      };
  const resizeSides: readonly ("left" | "right")[] = horizontal === "middle" ? ["left", "right"] : [horizontal === "left" ? "right" : "left"];
  const resizeDeltaFactor = horizontal === "middle" ? 2 : 1;
  const chromeToggleId = toggleId ?? childElementId(id, "pane", "fold");

  return (
    <LevelProvider level="pane">
      <div
        ref={setPaneRootRef}
        id={id}
        {...surfaceActiveProps}
        data-slot={overlaySlot ?? "pane"}
        data-level="pane"
        data-anchor={anchor}
        data-safe-area-inline={safeArea.inlinePx || undefined}
        data-safe-area-block={safeArea.blockPx || undefined}
        data-folded={effectiveFolded ? "true" : undefined}
        data-expanded={expanded ? "true" : undefined}
        data-dragging={dragging ? "true" : undefined}
        {...(dimWhenOpen ? { "data-dim": true } : {})}
        dir={flow.inline === "rtl" ? "rtl" : undefined}
        onPointerDown={stopPointerPropagation}
        onPointerMove={stopPointerPropagation}
        onPointerUp={stopPointerPropagation}
        onPointerCancel={stopPointerPropagation}
        className={cn(
          "pointer-events-auto absolute flex min-h-0 min-w-0 box-border overflow-visible",
          getLevelZClass("pane"),
          expanded && "z-panel",
          flow.block === "up" ? "flex-col-reverse" : "flex-col",
          horizontal === "middle" ? "items-center" : "items-start",
          effectiveFolded && "w-fit",
          !effectiveFolded && !expanded && "w-full max-w-full",
          expanded && "h-full max-h-full",
          className,
        )}
        style={positionStyle}
      >
        <FlowProvider inline={flow.inline} block={flow.block}>
          <WindowChrome
            stackSlot={stackSlot}
            chipOnly={effectiveFolded}
            active={!effectiveFolded && surfaceActive}
            level="pane"
            capDock={flow.block === "up" ? "bottom" : "top"}
            stackClassName={cn("bg-transparent", !effectiveFolded && "w-full min-h-0 flex-1", expanded && "h-full max-h-full min-h-0", stackClassName)}
            bodyClassName={bodyClassName}
            bodySlot={bodySlot}
            bodyStyle={bodyStyle}
            stackDataAttrs={stackDataAttrs}
            titleChips={
              <WindowPaneChromeToggle
                id={chromeToggleId}
                icon={icon}
                label={label ?? id}
                disabled={toggleDisabled}
                onClick={onFoldToggle}
                dragPointerProps={mobile || !onAnchorChange ? undefined : dragPointerProps}
                showDragHandle={!mobile}
                emphasized={dragging}
              />
            }
            body={!effectiveFolded ? children : undefined}
          />
          {resizable && !mobile && !effectiveFolded && !expanded && onSizeChange
            ? resizeSides.map((side) => <PaneResizeHandle key={side} side={side} size={size} minSize={minSize} maxSize={maxSize} onSizeChange={onSizeChange} onActiveChange={onResizeActiveChange} deltaFactor={resizeDeltaFactor} />)
            : null}
        </FlowProvider>
      </div>
    </LevelProvider>
  );
};
