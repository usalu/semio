import type { IconName } from "../../🔣️Icons/🟦️.tsx";
import * as React from "react";
import type { LayeredDirection, LayeredCell, LayeredLifecycle } from "../../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";

/** 🧭️ How the one strip under the one glass is read. `strip`: the app's card grid stays put above it and the mouse pans the pages behind
 * (desktop, tablet); `swipe`: the view is one cell of the same grid, each card rides on its own page, and a swipe along either axis — or the
 * wheel — carries the strip to the neighbouring cell, across the edges too: the grid wraps like an endless canvas (touch phones, windows too
 * small for the card grid). */
export type LayeredMode = "strip" | "swipe";

/** 🥞️ One backdrop page, the real page its card opens: `id` is its hash route, `data-layered-pane` and key, `label` names the opened page and its
 * placeholder, `render` is called only while the pane is mounted, `poster` is shown while it is not, and `capturePoster` (for example
 * {@link capturePosterFromCanvases}) turns the mounted page into a poster right before it is released. */
export interface LayeredPane {
  readonly id: string;
  readonly label: string;
  /** 📱️ What swipe hints and other tight chrome show instead of {@link label} when space is scarce; defaults to `label`. */
  readonly shortLabel?: string;
  readonly icon?: IconName;
  readonly render: (state: LayeredPaneState) => React.ReactNode;
  readonly poster?: string | null;
  readonly capturePoster?: (container: HTMLElement) => string | null;
}

/** 🥞️ What a mounted page knows: `opened` (full size and interactive, otherwise inert backdrop), `revealed` (its card is hovered or focused, so it
 * shows clear) and `dirty` (the page reports user work, so it is never released). */
export interface LayeredPaneState {
  readonly opened: boolean;
  readonly revealed: boolean;
  readonly dirty: () => void;
}

/** 🃏️ What a card knows: the mode, whether its page is revealed (lift the card) or opened, and `open` (hash route plus glide). */
export interface LayeredCardState {
  readonly mode: LayeredMode;
  readonly revealed: boolean;
  readonly opened: boolean;
  readonly open: () => void;
}

/** 🧢️ What the app's chrome above the element knows, so it can hide while a page is revealed or opened. */
export interface LayeredChromeState {
  readonly revealed: boolean;
  readonly opened: boolean;
}

/** 🌐️ The element's own words, always the app's (no default language): the card group, the Overview button, a waiting and a failed page, and
 * the hint of a page one swipe away in `direction` (its accessible name, such as "Go right to Physics"; the hint shows the page's label). An
 * app whose own chrome leads back to the overview names no `overview`, and the element shows no button of its own over the opened page. */
export interface LayeredLabels {
  readonly grid: string;
  readonly overview?: string;
  readonly waiting: (pane: LayeredPane) => string;
  readonly failed: (pane: LayeredPane) => string;
  readonly neighbour: (pane: LayeredPane, direction: LayeredDirection) => string;
}

/** 📏️ How far the app's own chrome covers the `top` and the `bottom` of the overview, as CSS lengths (`0px` where not given). */
export interface LayeredInsets {
  readonly top?: string;
  readonly bottom?: string;
}

/** 🥞️ Props of {@link LayeredOverview}. `cells` places every pane on the strip for the current breakpoint; in `strip` mode the card grid is the
 * app's CSS (`overlayClassName`/`overlayStyle`, each card wrapped in a `display: contents` host), in `swipe` mode the element centres each card
 * in its page's cell itself, inside `insets`, and the hints of its neighbours stand inside them too; `pan` lets the mouse pan the strip under the glass while it is between the cards; `reducedMotion` says whether
 * pan, glide and settle give way to stillness — `"never"` (the default), `"auto"` (when the device asks for it) or `"always"`; `openedId`
 * makes the open page controlled. */
export interface LayeredOverviewProps {
  readonly panes: readonly LayeredPane[];
  readonly cells: Readonly<Record<string, LayeredCell>>;
  readonly renderCard: (pane: LayeredPane, state: LayeredCardState) => React.ReactNode;
  readonly overlayClassName?: string;
  readonly overlayStyle?: React.CSSProperties;
  readonly renderChrome?: (state: LayeredChromeState) => React.ReactNode;
  readonly mode?: LayeredMode;
  readonly insets?: LayeredInsets;
  readonly pan?: "pointer" | "none";
  readonly routing?: "hash" | "none";
  readonly openedId?: string | null;
  readonly onOpenedIdChange?: (id: string | null) => void;
  readonly onRevealedIdChange?: (id: string | null) => void;
  readonly lifecycle?: Partial<LayeredLifecycle>;
  readonly windowing?: { readonly radius: number };
  readonly reducedMotion?: "auto" | "always" | "never";
  readonly labels: LayeredLabels;
}
