/** 🧵️ The pane of LayeredOverview: mounting, error boundary, placeholder and the card host. */
import { type LayeredLifecycle, type LayeredRevealSource, type LayeredZoomHold, layeredZoomOut, layeredZoomOver, panesOverBudget, nextWarmBoot, warmDelay, scheduleIdle, panesToRelease } from "../../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
import * as React from "react";
import { type IconName, Icon } from "../../🔣️Icons/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { loadingBorderClass } from "../../../🔨️modules/🌀️status-border-presentation/🟦️.ts";
import type { LayeredMode, LayeredOverviewProps, LayeredPane, LayeredPaneState } from "../📋️types/🟦️.ts";
import { focusVisible } from "../🧰️pane-helpers/🟦️.ts";

/** 🧮️ Mounts, releases and posters of the panes: boots on reveal/open/swipe, a warm queue in cell order (while `warming`: the strip mode, never
 * the swipe mode, which warms the neighbours of its cell instead; a pace of zero boots at once instead of waiting for an idle moment), a live `budget` (least recently touched pristine pane first), time-based release, posters from `capturePoster`. Dirty, opened and
 * revealed panes are never released; the most recently opened pane (initially the first) is never released by time while the overview shows. */
export function usePaneLifecycle(panes: readonly LayeredPane[], ids: readonly string[], opened: string | null, revealed: string | null, lifecycle: LayeredLifecycle, warming: boolean) {
  const initial = opened ? [opened] : [];
  const [booted, setBooted] = React.useState<ReadonlySet<string>>(() => new Set(initial));
  const [suspended, setSuspended] = React.useState<ReadonlySet<string>>(() => new Set());
  const [dirty, setDirty] = React.useState<ReadonlySet<string>>(() => new Set());
  const [posters, setPosters] = React.useState<ReadonlyMap<string, string>>(() => new Map());
  const live = React.useMemo<ReadonlySet<string>>(() => new Set([...booted].filter((id) => !suspended.has(id))), [booted, suspended]);
  const mountedAt = React.useRef(new Map<string, number>(initial.map((id) => [id, Date.now()])));
  const touchedAt = React.useRef(new Map<string, number>(opened ? [[opened, Date.now()]] : []));
  const containers = React.useRef(new Map<string, HTMLElement>());
  const lastOpened = React.useRef<string | null>(opened ?? panes[0]?.id ?? null);
  const previousOpened = React.useRef(opened);
  const warmed = React.useRef(false);
  const state = React.useRef({ panes, live, dirty, opened, revealed, lifecycle });
  state.current = { panes, live, dirty, opened, revealed, lifecycle };

  const boot = React.useCallback((id: string) => {
    if (!state.current.live.has(id)) mountedAt.current.set(id, Date.now());
    setBooted((previous) => (previous.has(id) ? previous : new Set(previous).add(id)));
    setSuspended((previous) => (previous.has(id) ? new Set([...previous].filter((other) => other !== id)) : previous));
  }, []);
  const touch = React.useCallback(
    (id: string) => {
      touchedAt.current.set(id, Date.now());
      boot(id);
    },
    [boot],
  );
  const warm = React.useCallback(
    (id: string) => {
      warmed.current = true;
      boot(id);
    },
    [boot],
  );
  const markDirty = React.useCallback((id: string) => setDirty((previous) => (previous.has(id) ? previous : new Set(previous).add(id))), []);
  const registerContainer = React.useCallback((id: string, element: HTMLElement | null) => {
    if (element) containers.current.set(id, element);
    else containers.current.delete(id);
  }, []);
  const container = React.useCallback((id: string) => containers.current.get(id) ?? null, []);
  const keep = React.useCallback((byTime: boolean): ReadonlySet<string> => {
    const { dirty: kept, opened: open, revealed: shown } = state.current;
    return new Set([...kept, ...[open, shown, byTime && open === null ? lastOpened.current : null].filter((id): id is string => id !== null)]);
  }, []);
  const release = React.useCallback((released: readonly string[]) => {
    if (released.length === 0) return;
    const captured = new Map<string, string>();
    for (const id of released) {
      const element = containers.current.get(id);
      const poster = element ? state.current.panes.find((pane) => pane.id === id)?.capturePoster?.(element) : null;
      if (poster) captured.set(id, poster);
    }
    if (captured.size > 0) setPosters((previous) => new Map([...previous, ...captured]));
    setSuspended((previous) => new Set([...previous, ...released]));
  }, []);
  const byRecency = React.useCallback(
    (set: ReadonlySet<string>) => [...set].sort((a, b) => (touchedAt.current.get(a) ?? -Infinity) - (touchedAt.current.get(b) ?? -Infinity) || (mountedAt.current.get(a) ?? 0) - (mountedAt.current.get(b) ?? 0)),
    [],
  );

  React.useEffect(() => {
    const previous = previousOpened.current;
    previousOpened.current = opened;
    if (previous !== null && previous !== opened) touchedAt.current.set(previous, Date.now());
    if (opened === null) return;
    lastOpened.current = opened;
    touch(opened);
  }, [opened, touch]);

  React.useEffect(() => {
    release(panesOverBudget(byRecency(live), keep(false), lifecycle.budget));
  }, [live, dirty, opened, revealed, lifecycle.budget, byRecency, keep, release]);

  React.useEffect(() => {
    if (!warming) return;
    const step = nextWarmBoot(ids, booted, live.size, opened, lifecycle.budget);
    if (step.kind !== "boot") return;
    const delay = warmDelay(warmed.current, lifecycle);
    if (delay <= 0) return warm(step.id);
    return scheduleIdle(() => warm(step.id), delay, window);
  }, [warming, ids, booted, live.size, opened, lifecycle.budget, lifecycle.warmStartMs, lifecycle.warmIntervalMs, warm]);

  const timed = [lifecycle.suspendIdleMs, lifecycle.suspendOffscreenMs, lifecycle.suspendHiddenMs].some(Number.isFinite);
  React.useEffect(() => {
    if (!timed) return;
    const timer = window.setInterval(() => {
      const { live: mounted, opened: open, lifecycle: policy } = state.current;
      const since = (id: string) => Math.max(touchedAt.current.get(id) ?? 0, mountedAt.current.get(id) ?? 0);
      release(panesToRelease([...mounted].map((id) => ({ id, since: since(id) })), keep(true), Date.now(), { opened: open !== null, hidden: document.hidden }, policy));
    }, lifecycle.sweepMs);
    return () => window.clearInterval(timer);
  }, [timed, lifecycle.sweepMs, keep, release]);

  return { booted, live, posters, touch, warm, markDirty, registerContainer, container };
}

/** 🛟️ One crashing page never takes down the others or the landing around them. */
class LayeredPaneBoundary extends React.Component<{ readonly failed: string; readonly children: React.ReactNode }, { readonly crashed: boolean }> {
  override state = { crashed: false };
  static getDerivedStateFromError(): { readonly crashed: boolean } {
    return { crashed: true };
  }
  override render(): React.ReactNode {
    return this.state.crashed ? (
      <div role="alert" data-layered-pane-error="" className="flex h-full w-full items-center justify-center bg-background p-double text-center text-sm text-muted-foreground">
        {this.props.failed}
      </div>
    ) : (
      this.props.children
    );
  }
}

/** 📖️ Renders a page as a child of its boundary, so a throwing page is caught there. */
function LayeredPage({ render, state }: { readonly render: LayeredPane["render"]; readonly state: LayeredPaneState }): React.ReactNode {
  return render(state);
}

/** ⏳️ What a pane shows before it first mounts: its glyph at 40 % and a pulsing page behind a loading border. */
function LayeredPanePlaceholder({ icon, busy, label }: { readonly icon?: IconName; readonly busy: boolean; readonly label: string }): React.ReactElement {
  return (
    <div role="status" aria-busy={busy} aria-label={label} className={cn("flex h-full w-full flex-col items-center justify-center gap-double bg-background p-double", loadingBorderClass)}>
      {icon ? <Icon icon={icon} size="large" className="text-foreground opacity-40" /> : null}
      <div aria-hidden className="min-h-0 w-full max-w-4xl flex-1 animate-pulse bg-muted-foreground/10 motion-reduce:animate-none" />
    </div>
  );
}

interface LayeredPaneViewProps {
  readonly pane: LayeredPane;
  readonly style?: React.CSSProperties;
  readonly opened: boolean;
  readonly revealed: boolean;
  readonly live: boolean;
  readonly shown: boolean;
  readonly busy: boolean;
  readonly poster: string | null;
  readonly waiting: string;
  readonly failed: string;
  readonly onContainer: (id: string, element: HTMLElement | null) => void;
  readonly onDirty: (id: string) => void;
}

/** 🥞️ One pane: its live page, else its poster, else its placeholder (the last two only within the window). Inert and hidden from assistive
 * technology unless opened; opened, it is a labelled, focusable region. Pointer or key input inside marks it dirty. */
export const LayeredPaneView = /* @__PURE__ */ React.memo(function LayeredPaneView(props: LayeredPaneViewProps): React.ReactElement {
  const { pane, opened, revealed, onContainer, onDirty } = props;
  const dirty = React.useCallback(() => onDirty(pane.id), [onDirty, pane.id]);
  const ref = React.useCallback((element: HTMLDivElement | null) => onContainer(pane.id, element), [onContainer, pane.id]);
  const state = React.useMemo<LayeredPaneState>(() => ({ opened, revealed, dirty }), [opened, revealed, dirty]);
  return (
    <div
      ref={ref}
      data-layered-pane={pane.id}
      data-opened={opened ? "" : undefined}
      role={opened ? "region" : undefined}
      aria-label={opened ? pane.label : undefined}
      aria-hidden={opened ? undefined : true}
      tabIndex={opened ? -1 : undefined}
      inert={!opened}
      className="relative h-full w-full overflow-hidden bg-background outline-none"
      style={{ contain: "layout paint", ...props.style }}
      onPointerDownCapture={dirty}
      onKeyDownCapture={dirty}
    >
      {props.live ? (
        <LayeredPaneBoundary failed={props.failed}>
          <LayeredPage render={pane.render} state={state} />
        </LayeredPaneBoundary>
      ) : !props.shown ? null : props.poster ? (
        <img src={props.poster} alt="" aria-hidden className="h-full w-full object-cover" />
      ) : (
        <LayeredPanePlaceholder icon={pane.icon} busy={props.busy} label={props.waiting} />
      )}
    </div>
  );
});

/** 🎯 The explore control inside `host`, when `node` is that control or something inside it. */
function zoomTrigger(node: EventTarget | null, host: HTMLElement): Element | null {
  const element = node instanceof Element ? node : node instanceof Text ? node.parentElement : null;
  if (element === null) return null;
  const trigger = element.closest("[data-layered-zoom]");
  return trigger !== null && host.contains(trigger) ? trigger : null;
}

/** 🃏️ The stable host of one card: the pointer zooms its page only while it is over the card's explore control
 * (`data-layered-zoom`); keyboard focus anywhere in the card reveals the page, and leaving that focus conceals it. The card
 * itself is the app's. Memoized, so a reveal re-renders only the two cards whose state changed. A pointer hover conceals
 * the other hosts without rendering them again, and marks each one `data-concealed`: the slot and the hit target stay,
 * and a survey of the page treats the host as unseen, so a pet that stood on that card falls. The hovered card is marked `data-tinted` and wears the veil's wash. */
export const LayeredCardHost = /* @__PURE__ */ React.memo(function LayeredCardHost(props: {
  readonly pane: LayeredPane;
  readonly mode: LayeredMode;
  readonly revealed: boolean;
  readonly source: LayeredRevealSource | null;
  readonly zoomHold: React.MutableRefObject<LayeredZoomHold | null>;
  readonly onOpen: (id: string) => void;
  readonly onReveal: (id: string, source: LayeredRevealSource) => void;
  readonly onConceal: (id: string) => void;
  readonly onElement: (id: string, element: HTMLElement | null) => void;
  readonly renderCard: LayeredOverviewProps["renderCard"];
  readonly placement: { readonly onCell: React.RefCallback<HTMLDivElement> | undefined; readonly style: React.CSSProperties | undefined } | null;
}): React.ReactElement {
  const { pane, onOpen, onElement, onReveal, onConceal, zoomHold } = props;
  const open = React.useCallback(() => onOpen(pane.id), [onOpen, pane.id]);
  const onCell = props.placement?.onCell;
  const ref = React.useCallback((element: HTMLDivElement | null) => { onElement(pane.id, element); onCell?.(element); }, [onElement, pane.id, onCell]);
  return (
    <div
      ref={ref}
      data-layered-card={pane.id}
      data-revealed={props.revealed ? "" : undefined}
      data-reveal={props.source ?? undefined}
      data-layered-cell={props.placement === null ? undefined : pane.id}
      className={props.placement === null ? "contents" : "pointer-events-none flex min-h-0 min-w-0 items-center-safe justify-center overflow-y-auto px-[36px] [:where(&)_*]:touch-pinch-zoom"}
      style={props.placement?.style}
      onPointerOver={(event) => {
        if (event.pointerType !== "mouse" || zoomTrigger(event.target, event.currentTarget) === null) return;
        const decision = layeredZoomOver(zoomHold.current);
        zoomHold.current = decision.hold;
        if (decision.reveal) onReveal(pane.id, "pointer");
      }}
      onPointerOut={(event) => {
        if (event.pointerType !== "mouse") return;
        const host = event.currentTarget;
        const next = event.relatedTarget;
        const leavingHost = !(next instanceof Node) || !host.contains(next);
        const from = zoomTrigger(event.target, host);
        const to = zoomTrigger(next, host);
        if (!leavingHost && (from === null || to !== null)) return;
        const decision = layeredZoomOut(zoomHold.current);
        zoomHold.current = decision.hold;
        if (decision.conceal) onConceal(pane.id);
      }}
      onFocus={(event) => focusVisible(event.target) && onReveal(pane.id, "focus")}
      onBlur={(event) => !event.currentTarget.contains(event.relatedTarget as Node | null) && onConceal(pane.id)}
    >
      {props.renderCard(pane, { mode: props.mode, revealed: props.revealed, opened: false, open })}
    </div>
  );
});
