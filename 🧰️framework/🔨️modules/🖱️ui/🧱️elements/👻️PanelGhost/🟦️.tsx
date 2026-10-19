/** 👻 Owns the panel ghost and interaction command contexts that drag surfaces, trees and layouts share. */
import * as React from "react";
import { type MergeMode } from "../../../🕹️interaction/🟦️.ts";
import { reactHostPort } from "../🔌️Ports/⚛️react/🟦️.ts";
import { createDOMEventBinding } from "../../🔨️modules/👂️dom-event-binding/🟦️.ts";
import { useLevel } from "../🌈️Surface/🟦️.tsx";
import { markGhostTreeInteraction, resolveHoverRow } from "../🌳️Tree/🧭️hover-path/🟦️.ts";

/**
 * 🎯️ The ONE modifier→merge policy for `🕹️interaction`'s `nextSelection`/`nextHover` picks — every
 * surface (Tree rows, the Shell, canvas/world hosts) routes its raw pointer/keyboard modifiers through
 * this single mapping instead of hand-rolling shift/ctrl/alt checks per surface: shift → `"range"`,
 * mod/ctrl/cmd → `"invertive"`, alt → `"subtractive"`, no modifier → `"replace"`. Priority is shift, then
 * ctrl/meta, then alt — a chord that holds more than one of these picks the first that matches, so shift
 * always wins a range pick even with ctrl also held. Shares its `MergeMode` vocabulary with
 * `marqueeModeFromModifiers` above (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a
 * collapsed the old, differently-prioritized `SelectionMergeMode` duplicate into this one) — the two
 * functions differ only in priority/never-range, matching their different gestures (marquee drag vs.
 * click-select).
 */
export function interactionMergeFromModifiers(modifiers: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean; readonly altKey?: boolean }): MergeMode {
  if (modifiers.shiftKey === true) return "range";
  if (modifiers.ctrlKey === true || modifiers.metaKey === true) return "invertive";
  if (modifiers.altKey === true) return "subtractive";
  return "replace";
}

// Global ghost mode hides open panel/pane chrome during interactions; navbar/footer panel toggles stay visible.

const PANEL_GHOST_MOVE_THRESHOLD_PX = 4;

/** 👻️ Global ghost session API (begin/end while dragging or editing). */
export interface PanelGhostValue {
  readonly active: boolean;
  readonly begin: (target: EventTarget | null) => void;
  readonly end: () => void;
}

const PanelGhostContext = reactHostPort.createContext<PanelGhostValue | undefined>(undefined);

/** 👻️ Returns the global ghost controller when inside {@link GhostProvider}. */
export const usePanelGhost = (): PanelGhostValue | undefined => reactHostPort.useContext(PanelGhostContext);

export interface InteractionCommands {
  setActiveInteraction: (elementId?: string, interactionId?: string) => void;
}

export const InteractionContext = reactHostPort.createContext<InteractionCommands | undefined>(undefined);

export const ActiveInteractionContext = reactHostPort.createContext<string | undefined>(undefined);

export const useInteractionCommands = () => reactHostPort.useContext(InteractionContext);

type GhostController = PanelGhostValue & {
  readonly commands: InteractionCommands;
  readonly activeInteraction?: string;
};

function findGhostRegionAncestor(target: Element): Element | null {
  let node: Element | null = target;
  while (node) {
    if (node.hasAttribute("data-ghost-region")) return node;
    node = node.parentElement;
  }
  return null;
}

/** 📐️ Pane/panel/mode edge resize is layout chrome, not a canvas ghost interaction. */
function isChromeResizeHandleTarget(target: EventTarget | null): boolean {
  if (!(target instanceof Element)) return false;
  const slotted = target.closest("[data-slot]");
  if (!slotted) return false;
  const slot = slotted.getAttribute("data-slot") ?? "";
  return slot === "pane-resize-handle" || slot === "panel-resize-handle" || slot === "resizable-handle" || slot === "resizable-corner" || slot.startsWith("window-measures-resize");
}

/** 👻️ Keeps automatic ghosting on interaction surfaces and direct tree rows, not nested UI controls. */
export function shouldBeginAutomaticGhostInteraction(target: EventTarget | null): boolean {
  if (!(target instanceof Element)) return false;
  if (isChromeResizeHandleTarget(target)) return false;
  const region = findGhostRegionAncestor(target);
  const dimmed = target.closest("[data-dim]");
  if (!dimmed || (region && !region.contains(dimmed))) return true;
  if (!region) return false;
  return resolveHoverRow(target as HTMLElement, region as HTMLElement) === dimmed;
}

function useGhostController(): GhostController {
  const [active, setActive] = reactHostPort.useState(false);
  const [activeInteraction, setActiveInteractionState] = reactHostPort.useState<string | undefined>(undefined);
  const activeMarkedRefs = reactHostPort.useRef<HTMLElement[]>([]);
  const sessionActiveRef = reactHostPort.useRef(false);

  const clearActiveMarks = reactHostPort.useCallback(() => {
    for (const el of activeMarkedRefs.current) {
      el.removeAttribute("data-active-interaction");
      el.removeAttribute("data-active-ancestor");
    }
    activeMarkedRefs.current = [];
  }, []);

  const begin = reactHostPort.useCallback(
    (target: EventTarget | null) => {
      clearActiveMarks();
      sessionActiveRef.current = true;
      setActive(true);
      if (!target || !(target instanceof Element)) return;
      const region = findGhostRegionAncestor(target);
      if (!region) return;
      activeMarkedRefs.current = markGhostTreeInteraction(target, region);
    },
    [clearActiveMarks],
  );

  const end = reactHostPort.useCallback(() => {
    clearActiveMarks();
    sessionActiveRef.current = false;
    setActive(false);
    setActiveInteractionState(undefined);
  }, [clearActiveMarks]);

  const setActiveInteraction = reactHostPort.useCallback(
    (elementId?: string, interactionId?: string) => {
      if (interactionId) {
        setActiveInteractionState(interactionId);
        const el = elementId ? document.getElementById(elementId) : null;
        begin(el);
        return;
      }
      end();
    },
    [begin, end],
  );

  const commands = reactHostPort.useMemo<InteractionCommands>(() => ({ setActiveInteraction }), [setActiveInteraction]);

  const controllerRef = reactHostPort.useRef<GhostController | null>(null);
  controllerRef.current = { active, begin, end, commands, activeInteraction };

  reactHostPort.useEffect(() => {
    const pendingRef: { current: { readonly x: number; readonly y: number; readonly target: EventTarget | null } | null } = { current: null };
    const beganViaPointerDragRef = { current: false };
    const bindings = createDOMEventBinding();
    const capture = true;
    const onDown = (event: Event) => {
      const pointerEvent = event as PointerEvent;
      if (pointerEvent.button !== 0) return;
      pendingRef.current = { x: pointerEvent.clientX, y: pointerEvent.clientY, target: pointerEvent.target };
    };
    const onMove = (event: Event) => {
      const pointerEvent = event as PointerEvent;
      const pending = pendingRef.current;
      if (!pending || sessionActiveRef.current) return;
      const dx = pointerEvent.clientX - pending.x;
      const dy = pointerEvent.clientY - pending.y;
      if (dx * dx + dy * dy < PANEL_GHOST_MOVE_THRESHOLD_PX * PANEL_GHOST_MOVE_THRESHOLD_PX) return;
      if (!shouldBeginAutomaticGhostInteraction(pending.target)) {
        pendingRef.current = null;
        return;
      }
      controllerRef.current?.begin(pending.target);
      beganViaPointerDragRef.current = true;
      pendingRef.current = null;
    };
    const onUp = () => {
      pendingRef.current = null;
      if (beganViaPointerDragRef.current) {
        beganViaPointerDragRef.current = false;
        controllerRef.current?.end();
      }
    };
    bindings.listen(document, "pointerdown", onDown, capture);
    bindings.listen(document, "pointermove", onMove, capture);
    bindings.listen(document, "pointerup", onUp, capture);
    bindings.listen(document, "pointercancel", onUp, capture);
    return () => bindings.dispose();
  }, []);

  return reactHostPort.useMemo(() => ({ active, begin, end, commands, activeInteraction }), [active, activeInteraction, begin, commands, end]);
}

/** 👻️ Mounts global ghost detection and interaction context for layout + panels. */
export const GhostProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const ghost = useGhostController();
  reactHostPort.useEffect(() => {
    setPanelGhostSessionBridge({ begin: ghost.begin, end: ghost.end });
    return () => setPanelGhostSessionBridge(null);
  }, [ghost.begin, ghost.end]);
  const panelGhostValue = reactHostPort.useMemo<PanelGhostValue>(() => ({ active: ghost.active, begin: ghost.begin, end: ghost.end }), [ghost.active, ghost.begin, ghost.end]);
  return (
    <PanelGhostContext.Provider value={panelGhostValue}>
      <InteractionContext.Provider value={ghost.commands}>
        <ActiveInteractionContext.Provider value={ghost.activeInteraction}>{children}</ActiveInteractionContext.Provider>
      </InteractionContext.Provider>
    </PanelGhostContext.Provider>
  );
};

interface GhostRegionShellProps extends React.HTMLAttributes<HTMLDivElement> {
  children?: React.ReactNode;
  /** 🫳️ When true, the region root is click-through while ghosted (panels over canvas). */
  clickThroughWhenGhost?: boolean;
  /** 👻️ When false, this region stays undimmed during a global ghost session (navbar/footer panel toggles). Default true — open panels/panes hide. */
  sessionGhost?: boolean;
}

/** 👻️ Ghost region shell: dims {@link data-dim} children when {@link GhostProvider} is active and {@link sessionGhost} is true. */
export const GhostRegionShell = reactHostPort.forwardRef<HTMLDivElement, GhostRegionShellProps>(function GhostRegionShell({ children, className, style, clickThroughWhenGhost = false, sessionGhost = true, ...props }, ref) {
  const ghost = usePanelGhost();
  const ghostActive = Boolean(sessionGhost && ghost?.active);
  const mergedStyle = reactHostPort.useMemo(
    () => ({
      ...(style as React.CSSProperties | undefined),
      pointerEvents: clickThroughWhenGhost && ghostActive ? ("none" as const) : undefined,
    }),
    [clickThroughWhenGhost, ghostActive, style],
  );
  return (
    <div ref={ref} data-ghost-region data-ghost={ghostActive ? "true" : undefined} className={className} style={mergedStyle} {...props}>
      {children}
    </div>
  );
});

interface PanelGhostRootProps extends React.HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode;
}

/** 👻️ Panel shell marked as a ghost region; dims when {@link GhostProvider} session is active. */
export const PanelGhostRoot = reactHostPort.forwardRef<HTMLDivElement, PanelGhostRootProps>(function PanelGhostRoot({ children, className, style, ...props }, ref) {
  const level = useLevel();
  return (
    <GhostRegionShell ref={ref} clickThroughWhenGhost data-level={level} data-elevation-root="" className={className} style={style} {...props}>
      {children}
    </GhostRegionShell>
  );
});

type PanelGhostSessionBridge = Pick<PanelGhostValue, "begin" | "end">;

export let panelGhostSessionBridge: PanelGhostSessionBridge | null = null;

export function setPanelGhostSessionBridge(bridge: PanelGhostSessionBridge | null): void {
  panelGhostSessionBridge = bridge;
}
