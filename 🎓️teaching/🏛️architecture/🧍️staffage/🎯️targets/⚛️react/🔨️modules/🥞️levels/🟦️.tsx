/** 🥞️ One level of the nested overview: the design system's `LayeredOverview` fed with the archive's pages, so a deeper page opens while the grid before it stays visible and live behind the glass.
 *
 * The reference experience of the course site is reused as is — glass veil, pointer parallax, hover reveal, swipe on phones, Escape to the parent. This module only supplies what the archive decides:
 * the cards, the grid they stand on, the layout mode (a scrolling card grid on desktop and tablet, swipe on a phone), the one pointer shared by every nested grid, and the address as the single source of the opened page.
 * Only the deepest grid handles input; the grids behind it stay live and inert.
 *
 * @see ../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx — the element
 * @see ../../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🏠️home/🟦️.tsx — the reference use
 */
import { createContext, useCallback, useContext, useEffect, useLayoutEffect, useRef, useState, type CSSProperties, type ReactElement, type ReactNode } from "react";
import { LayeredOverview, useMediaQuery, UI_MOBILE_MEDIA_QUERY, UI_TABLET_MEDIA_QUERY, type IconName, type LayeredCardState, type LayeredCell, type LayeredMode, type LayeredPane } from "@semio-tech/ui-react/chrome";
import { useArchive } from "../🧭️context/🟦️.tsx";

/** 👁️ How a page behind a card is shown: opened full size, or revealed clear while its card is hovered or focused (else blurred behind the glass, inert). */
export type PaneView = { readonly opened: boolean; readonly revealed: boolean };

/** 📄️ A page of a level. */
export type LevelPage = { readonly id: string; readonly label: string; readonly shortLabel?: string; readonly icon?: IconName; readonly render: (view: PaneView) => ReactNode };

type Camera = { readonly x: number; readonly y: number };
type PointerBus = { current: Camera | undefined; publish: (camera: Camera | undefined) => void; subscribe: (listener: (camera: Camera | undefined) => void) => () => void };

function createBus(): PointerBus {
  const listeners = new Set<(camera: Camera | undefined) => void>();
  const bus: PointerBus = {
    current: undefined,
    publish(camera) {
      if (camera?.x === bus.current?.x && camera?.y === bus.current?.y) return;
      bus.current = camera;
      for (const listener of listeners) listener(camera);
    },
    subscribe(listener) {
      listeners.add(listener);
      return () => void listeners.delete(listener);
    },
  };
  return bus;
}

const PointerContext = createContext<PointerBus | null>(null);

/** 🖱️ Provides the one pointer every nested grid shares. */
export function PointerProvider(props: { readonly children: ReactNode }): ReactElement {
  const bus = useRef<PointerBus | null>(null);
  bus.current ??= createBus();
  return <PointerContext.Provider value={bus.current}>{props.children}</PointerContext.Provider>;
}

function useInheritedCamera(ignore: boolean): Camera | undefined {
  const bus = useContext(PointerContext);
  const [camera, setCamera] = useState<Camera | undefined>(undefined);
  useLayoutEffect(() => {
    if (bus === null || ignore) return;
    setCamera(bus.current);
    return bus.subscribe(setCamera);
  }, [bus, ignore]);
  return ignore ? undefined : camera;
}

/** 📏️ The content box size of an element, measured; a default stands in where nothing can be measured (tests). */
export function useSize(fallback: { readonly width: number; readonly height: number }): [(element: HTMLElement | null) => void, { readonly width: number; readonly height: number }] {
  const [size, setSize] = useState(fallback);
  const observer = useRef<ResizeObserver | null>(null);
  const attach = useCallback((element: HTMLElement | null): void => {
    observer.current?.disconnect();
    if (element === null || typeof ResizeObserver !== "function") return;
    observer.current = new ResizeObserver((entries) => {
      const box = entries.at(-1)?.contentRect;
      if (box !== undefined && box.width > 0) setSize((previous) => (Math.abs(previous.width - box.width) < 1 && Math.abs(previous.height - box.height) < 1 ? previous : { width: box.width, height: box.height }));
    });
    observer.current.observe(element);
  }, []);
  useEffect(() => () => observer.current?.disconnect(), []);
  return [attach, size];
}

/** 🧭️ The breakpoint class of the viewport. */
export type Breakpoint = "desktop" | "tablet" | "mobile";

/** 🧭️ The design system's breakpoints as one value. */
export function useBreakpoint(): Breakpoint {
  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const tablet = useMediaQuery(UI_TABLET_MEDIA_QUERY);
  return mobile ? "mobile" : tablet ? "tablet" : "desktop";
}

/** 📍️ Cells of `count` pages, row by row in `columns` columns. */
export function rowMajorCells(ids: readonly string[], columns: number): Readonly<Record<string, LayeredCell>> {
  return Object.fromEntries(ids.map((id, index) => [id, { column: index % columns, row: Math.floor(index / columns) }]));
}

/** 🔳️ The CSS track list of `weights`, each at least `minPx` when that floor is set. */
export const tracks = (weights: readonly number[], minPx = 0): string => weights.map((weight) => `minmax(${minPx > 0 ? `${minPx}px` : "0"}, ${weight}fr)`).join(" ");

/** 🥞️ The props of one {@link Level}. */
export type LevelProps = {
  readonly label: string;
  readonly pages: readonly LevelPage[];
  readonly cells: Readonly<Record<string, LayeredCell>>;
  readonly openedId: string | null;
  readonly interactive: boolean;
  readonly mode: LayeredMode;
  readonly onOpen: (id: string | null) => void;
  readonly onEscape?: () => void;
  readonly renderCard: (page: LevelPage, card: LayeredCardState) => ReactNode;
  readonly columns: string;
  readonly rows: string;
  readonly budget?: number;
};

/** 🥞️ A layered overview of pages whose cards stand on a CSS grid above the glass. */
export function Level(props: LevelProps): ReactElement {
  const { label, pages, cells, openedId, interactive, mode, onOpen, onEscape, renderCard, columns, rows, budget = 4 } = props;
  const { t, reducedMotion } = useArchive();
  const bus = useContext(PointerContext);
  const inherited = useInheritedCamera(interactive || openedId !== null);
  const root = useRef<HTMLDivElement>(null);
  const panes: readonly LayeredPane[] = pages.map((page) => ({ id: page.id, label: page.label, ...(page.shortLabel === undefined ? {} : { shortLabel: page.shortLabel }), ...(page.icon === undefined ? {} : { icon: page.icon }), render: (view) => page.render({ opened: view.opened, revealed: view.revealed }) }));
  const previous = useRef({ openedId, interactive });
  useLayoutEffect(() => {
    const before = previous.current;
    previous.current = { openedId, interactive };
    if (!interactive) return;
    if (openedId === null && before.openedId !== null) root.current?.querySelector<HTMLElement>(`[data-layered-card="${before.openedId}"] a[href]`)?.focus({ preventScroll: true });
  }, [openedId, interactive]);
  useEffect(() => {
    if (!interactive || openedId !== null || onEscape === undefined) return;
    const onKey = (event: KeyboardEvent): void => {
      if (event.key !== "Escape" || event.defaultPrevented) return;
      const target = event.target;
      if (target instanceof Element && target.closest("input, select, textarea, [role=dialog]") !== null) return;
      event.preventDefault();
      onEscape();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [interactive, openedId, onEscape]);
  return (
    <div ref={root} data-level={label} data-grid-active={interactive ? "" : undefined} className="relative h-full min-h-0 w-full">
      <LayeredOverview
        panes={panes}
        cells={cells}
        renderCard={(pane, card) => {
          const page = pages.find((candidate) => candidate.id === pane.id);
          return page === undefined ? null : renderCard(page, card);
        }}
        overlayClassName="staffage-grid"
        overlayStyle={{ "--staffage-columns": columns, "--staffage-rows": rows } as CSSProperties}
        mode={mode}
        routing="none"
        interactive={interactive}
        openedId={openedId}
        camera={interactive ? undefined : inherited}
        onCameraChange={interactive ? (point) => bus?.publish(point) : undefined}
        onOpenedIdChange={onOpen}
        lifecycle={{ budget }}
        reducedMotion={reducedMotion ? "always" : "never"}
        labels={{
          grid: label,
          waiting: (pane) => t("level.waiting", { page: pane.label }),
          failed: (pane) => t("level.failed", { page: pane.label }),
          neighbour: (pane, direction) => t(`level.neighbour.${direction}`, { page: pane.label }),
        }}
      />
    </div>
  );
}
