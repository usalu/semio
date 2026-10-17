/** 👥️ Shared presence, cursors and what the others think (design §15–§17): every learner sees who else is online and
 * where, the pointers, keyboard focus and drags of the learners on the same page — on the home page also inside every
 * page behind the cards — and, in a quiz, the draft answers of everyone thinking along.
 *
 * Presence sockets of the proctor carry it — ephemeral shared state, never persisted, one socket per room kind: the
 * catalog's roster room with each learner's {@link PresenceState} (tag, identity, place, whether the tab is visible);
 * the room of the current place with each {@link CursorState}, which on home also *watches* the rooms of every page
 * behind the cards and every quiz's thinking room read-only at about 4 Hz; and in a run the quiz's thinking room, where
 * the learner's drafts go as a {@link ThinkingState} at most twice a second. A cursor is relative (0…1) to an anchor
 * every learner at that place renders (`data-presence-anchor`: cards, the leaderboard, a task card, an item
 * `item:<id>`, a category `category:<id>`), so it survives other viewports and randomized item orders; a drag names the
 * item it carries. State frames go out at most {@link PRESENCE_FRAME_HZ} times a second, latest state wins; a lost socket
 * reconnects with jittered backoff — ever more slowly while sessions keep ending right after they began — and sends its
 * latest state and watch again. Others' cursors are decorative (`aria-hidden`); their name labels are written in the ink
 * that reads on their palette colour and step aside from the control that has keyboard focus. The roster is text for
 * everyone: the online count in the navbar, who is where on the learner card, learners in a quiz on its card and online
 * marks in the leaderboard.
 *
 * @see ../../../../🔨️modules/👥️presence/🟦️.ts — rooms, admission checks, the roster and the thinking crowd of the core
 * @see ../../../../../🖥️server/🛣️routes-and-sockets/🟦️.ts — `presenceSocketUrl`, the `semio.presence.v1` frames including `watch`
 */

import { createContext, useContext, useEffect, useLayoutEffect, useRef, useState, type CSSProperties, type ReactElement, type ReactNode } from "react";
import { type CatalogView, type Cursor } from "@semio-tech/quiz";
import { Icon, presenceColor, presencePaint } from "@semio-tech/ui-react/chrome";
import { type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { cn } from "../🪟️chrome/🟦️.tsx";
import { QuizPresence } from "./🧭️rooms/🟦️.ts";
import { EMPTY_PRESENCE_VIEW, type PeerCursor, placeText, type PresenceView } from "./👁️view/🟦️.ts";

//#region ⚛️React
interface PresenceContextValue {
  readonly view: PresenceView;
  readonly showCursors: boolean;
  readonly setTask: (task: string | undefined) => void;
}

const PresenceContext = createContext<PresenceContextValue>({ view: EMPTY_PRESENCE_VIEW, showCursors: true, setTask: () => undefined });

/** 👥️ Makes `view` the presence every screen below shows (others' cursors only while `showCursors`), and `setTask`
 * where the run screen reports its task. */
export function PresenceProvider(props: { readonly view: PresenceView; readonly showCursors?: boolean; readonly setTask: (task: string | undefined) => void; readonly children: ReactNode }): ReactElement {
  return <PresenceContext.Provider value={{ view: props.view, showCursors: props.showCursors ?? true, setTask: props.setTask }}>{props.children}</PresenceContext.Provider>;
}

/** 👁️ The presence the screens show. */
export function usePresenceView(): PresenceView {
  return useContext(PresenceContext).view;
}

/** 📌️ Reports the task on screen for the learner's place, and none once the screen leaves. */
export function usePresenceTask(task: string | undefined): void {
  const { setTask } = useContext(PresenceContext);
  useEffect(() => {
    setTask(task);
    return () => setTask(undefined);
  }, [setTask, task]);
}

/** 👀️ Whether the page is visible (a hidden tab is away). */
export function useDocumentVisible(): boolean {
  const [visible, setVisible] = useState(() => typeof document === "undefined" || document.visibilityState !== "hidden");
  useEffect(() => {
    const change = (): void => setVisible(document.visibilityState !== "hidden");
    document.addEventListener("visibilitychange", change);
    return () => document.removeEventListener("visibilitychange", change);
  }, []);
  return visible;
}

/** 📐️ The cursor at viewport point (`x`, `y`) relative to `anchor`'s box, clamped to [0, 1] and rounded to 10⁻⁴;
 * `undefined` for an element without an anchor key or without a box. */
export function cursorAt(anchor: HTMLElement, x: number, y: number): Cursor | undefined {
  const key = anchor.dataset.presenceAnchor;
  const box = anchor.getBoundingClientRect();
  if (key === undefined || box.width <= 0 || box.height <= 0) return undefined;
  const unit = (value: number): number => Math.round(Math.min(1, Math.max(0, value)) * 10_000) / 10_000;
  return { anchor: key, x: unit((x - box.left) / box.width), y: unit((y - box.top) / box.height) };
}

function anchorOf(target: EventTarget | null): HTMLElement | null {
  return target instanceof Element ? target.closest<HTMLElement>("[data-presence-anchor]") : null;
}

function draggedItem(target: EventTarget | null): string | undefined {
  if (!(target instanceof Element) || target.closest("[data-quiz-grip]") === null) return undefined;
  return target.closest<HTMLElement>("[data-quiz-item]")?.dataset.quizItem;
}

/** 🖱️ Feeds `presence` from the document: the pointer over its nearest anchor (a card, an item, a category), also while
 * pressed, the item a grip drags until release, nothing once the pointer leaves the page or the window loses focus, and
 * the anchor holding keyboard focus while the keyboard moves it. */
export function usePresencePointer(presence: QuizPresence): void {
  useEffect(() => {
    if (typeof document === "undefined") return;
    let keyboard = false;
    const move = (event: PointerEvent): void => {
      const anchor = anchorOf(event.target);
      presence.point(anchor === null ? undefined : cursorAt(anchor, event.clientX, event.clientY));
    };
    const down = (event: PointerEvent): void => {
      keyboard = false;
      presence.drag(draggedItem(event.target));
    };
    const up = (): void => presence.drag(undefined);
    const leave = (): void => {
      presence.point(undefined);
      presence.drag(undefined);
    };
    const key = (event: KeyboardEvent): void => {
      keyboard = true;
      if (event.key === "Escape") presence.drag(undefined);
    };
    const focusIn = (event: FocusEvent): void => presence.focusOn(keyboard ? anchorOf(event.target)?.dataset.presenceAnchor : undefined);
    const focusOut = (event: FocusEvent): void => {
      if (event.relatedTarget === null) presence.focusOn(undefined);
    };
    const root = document.documentElement;
    document.addEventListener("pointermove", move, { passive: true });
    document.addEventListener("pointerdown", down, { capture: true, passive: true });
    window.addEventListener("pointerup", up, { capture: true });
    window.addEventListener("pointercancel", up, { capture: true });
    root.addEventListener("pointerleave", leave);
    window.addEventListener("blur", leave);
    document.addEventListener("keydown", key, { capture: true });
    document.addEventListener("focusin", focusIn);
    document.addEventListener("focusout", focusOut);
    return () => {
      document.removeEventListener("pointermove", move);
      document.removeEventListener("pointerdown", down, { capture: true });
      window.removeEventListener("pointerup", up, { capture: true });
      window.removeEventListener("pointercancel", up, { capture: true });
      root.removeEventListener("pointerleave", leave);
      window.removeEventListener("blur", leave);
      document.removeEventListener("keydown", key, { capture: true });
      document.removeEventListener("focusin", focusIn);
      document.removeEventListener("focusout", focusOut);
    };
  }, [presence]);
}

function appearance(): "light" | "dark" {
  return typeof document !== "undefined" && document.documentElement.classList.contains("dark") ? "dark" : "light";
}

/** 🖋️ The relative luminance above which black ink contrasts more with a colour than white ink does: where
 * `(L + 0.05) / 0.05` equals `1.05 / (L + 0.05)`. Either ink then reaches at least 4.58 : 1 on any colour. */
const INK_FLIP_LUMINANCE = Math.sqrt(1.05 * 0.05) - 0.05;

function channel(value: number): number {
  return value <= 0.03928 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
}

/** 🖋️ The ink that reads on the palette colour of roster slot `colour` in `appearance`: black or white, whichever
 * contrasts more by the WCAG relative luminance of that colour. */
export function peerInk(colour: number, appearance: "light" | "dark"): string {
  const { h, s, l } = presenceColor(colour, appearance);
  const chroma = s * Math.min(l, 1 - l);
  const part = (offset: number): number => {
    const turn = (offset + h / 30) % 12;
    return l - chroma * Math.max(-1, Math.min(turn - 3, 9 - turn, 1));
  };
  const luminance = 0.2126 * channel(part(0)) + 0.7152 * channel(part(8)) + 0.0722 * channel(part(4));
  return luminance > INK_FLIP_LUMINANCE ? "#000000" : "#ffffff";
}

/** 🎨️ The custom properties that colour a learner of roster slot `colour`: `--quiz-peer` in the current appearance, and
 * the ink of its name label for either appearance (`--quiz-peer-ink-light`, `--quiz-peer-ink-dark`), of which the
 * stylesheet takes the one the document shows. */
export function paintStyle(colour: number): CSSProperties {
  return { ["--quiz-peer" as string]: presencePaint(colour, appearance()), ["--quiz-peer-ink-light" as string]: peerInk(colour, "light"), ["--quiz-peer-ink-dark" as string]: peerInk(colour, "dark") } as CSSProperties;
}

function anchorSelector(anchor: string): string {
  return `[data-presence-anchor="${typeof CSS !== "undefined" && typeof CSS.escape === "function" ? CSS.escape(anchor) : anchor}"]`;
}

/** 🫣️ How far (px) a name label keeps from the control that has focus (WCAG 2.2 SC 2.4.11, focus not obscured). */
export const PEER_LABEL_CLEARANCE_PX = 24;

const LABEL_OFFSET = { cursor: { left: 16, top: 24 }, focus: { left: -2, top: -20 } } as const;

/** ⌨️ The box of the control that has focus in the document, if any. */
function focusedBox(): DOMRect | undefined {
  const active = typeof document === "undefined" ? null : document.activeElement;
  if (active === null || active === document.body) return undefined;
  const box = active.getBoundingClientRect();
  return box.width > 0 && box.height > 0 ? box : undefined;
}

/** 🫣️ Whether the name label of a mark placed at (`left`, `top`) would lie over `focus` or closer to it than
 * {@link PEER_LABEL_CLEARANCE_PX}. */
function covers(mark: HTMLElement, left: number, top: number, focus: DOMRect): boolean {
  const label = mark.querySelector<HTMLElement>(".quiz-peer-label");
  if (label === null) return false;
  const offset = LABEL_OFFSET[mark.dataset.peer === "focus" ? "focus" : "cursor"];
  const x = left + offset.left;
  const y = top + offset.top;
  const reach = PEER_LABEL_CLEARANCE_PX;
  return x < focus.right + reach && x + label.offsetWidth > focus.left - reach && y < focus.bottom + reach && y + label.offsetHeight > focus.top - reach;
}

/** 📍️ Moves every peer mark of `layer` onto its anchor's box as laid out now; marks whose anchor is not on screen hide.
 * Without `within`, anchors are looked up in the document outside inert pages and placed in viewport pixels, and a
 * mark whose name label would cover the control that has keyboard focus is marked `data-shy` (its label hides, the
 * pointer or frame stays); with `within`, inside it only and placed in its own (unscaled) pixels, so marks inside a
 * page scaled on the overview follow. */
export function placePeers(layer: HTMLElement, within?: HTMLElement): void {
  const frame = within?.getBoundingClientRect();
  const scale = within === undefined || frame === undefined || within.offsetWidth <= 0 ? 1 : frame.width / within.offsetWidth;
  const focus = within === undefined ? focusedBox() : undefined;
  for (const mark of layer.querySelectorAll<HTMLElement>("[data-peer]")) {
    const selector = anchorSelector(mark.dataset.anchor ?? "");
    const target = within === undefined ? ([...document.querySelectorAll<HTMLElement>(selector)].find((element) => element.closest("[inert]") === null) ?? null) : within.querySelector<HTMLElement>(selector);
    if (target === null) {
      mark.hidden = true;
      continue;
    }
    const seen = target.getBoundingClientRect();
    const box = frame === undefined ? seen : { left: (seen.left - frame.left) / scale, top: (seen.top - frame.top) / scale, width: seen.width / scale, height: seen.height / scale };
    mark.hidden = false;
    const left = mark.dataset.peer === "focus" ? box.left : box.left + Number(mark.dataset.x) * box.width;
    const top = mark.dataset.peer === "focus" ? box.top : box.top + Number(mark.dataset.y) * box.height;
    mark.style.transform = `translate(${left}px, ${top}px)`;
    if (mark.dataset.peer === "focus") {
      mark.style.width = `${box.width}px`;
      mark.style.height = `${box.height}px`;
    }
    mark.toggleAttribute("data-shy", focus !== undefined && covers(mark, left, top, focus));
  }
}

function PeerMarks(props: { readonly peers: readonly PeerCursor[]; readonly itemLabel?: (item: string) => string | undefined }): ReactElement {
  const { itemLabel } = props;
  return (
    <>
      {props.peers.map((peer) => {
        const label = peer.drag === undefined ? peer.label : `${peer.label} ▸ ${itemLabel?.(peer.drag) ?? peer.drag}`;
        return (
          <div key={peer.session} style={paintStyle(peer.colour)}>
            {peer.focus === undefined ? null : (
              <div data-peer="focus" data-anchor={peer.focus} data-tag={peer.tag} hidden className="quiz-peer-focus">
                <span className="quiz-peer-label">{peer.label}</span>
              </div>
            )}
            {peer.cursor === undefined ? null : (
              <div data-peer="cursor" data-anchor={peer.cursor.anchor} data-x={peer.cursor.x} data-y={peer.cursor.y} data-tag={peer.tag} data-drag={peer.drag} hidden className="quiz-peer">
                <svg viewBox="0 0 14.666391 23.899563" width="14.666391" height="23.899563" className="quiz-peer-arrow">
                  <g transform="translate(-219.91582,-287.95767)">
                    <path d="m 220.04082,288.26777 v 23.46446 h 5.64666 v -17.59835 z" />
                    <path d="m 226.42314,294.89902 7.98557,8.29594 -3.99279,4.14797 -3.99278,-4.14797 z" />
                  </g>
                </svg>
                <span className="quiz-peer-label">{label}</span>
              </div>
            )}
          </div>
        );
      })}
    </>
  );
}

function usePlacing(layer: { readonly current: HTMLDivElement | null }, active: boolean, within?: () => HTMLElement | null, everyMs = 500): void {
  useLayoutEffect(() => {
    if (layer.current !== null) placePeers(layer.current, within?.() ?? undefined);
  });
  useEffect(() => {
    if (!active) return;
    let frame = 0;
    const place = (): void => {
      if (layer.current !== null) placePeers(layer.current, within?.() ?? undefined);
    };
    const schedule = (): void => {
      if (typeof requestAnimationFrame !== "function") return place();
      cancelAnimationFrame(frame);
      frame = requestAnimationFrame(place);
    };
    window.addEventListener("resize", schedule);
    document.addEventListener("scroll", schedule, { capture: true, passive: true });
    document.addEventListener("focusin", schedule);
    const timer = setInterval(schedule, everyMs);
    return () => {
      window.removeEventListener("resize", schedule);
      document.removeEventListener("scroll", schedule, { capture: true });
      document.removeEventListener("focusin", schedule);
      clearInterval(timer);
      if (typeof cancelAnimationFrame === "function") cancelAnimationFrame(frame);
    };
  }, [active]);
}

/** 🖱️ Other learners' pointers, keyboard focus and drags over the page in their palette colours, labelled with their
 * names ("Anna ▸ Tea light" while dragging) — decorative (`aria-hidden`, never hit by the pointer); positions follow
 * layout on every render, scroll and resize, and glide between frames unless the learner prefers reduced motion. */
export function PresenceOverlay(props: { readonly view: PresenceView; readonly show: boolean; readonly itemLabel?: (item: string) => string | undefined }): ReactElement | null {
  const { view, show } = props;
  const layer = useRef<HTMLDivElement>(null);
  const active = show && view.peers.length > 0;
  usePlacing(layer, active);
  if (!active) return null;
  return (
    <div ref={layer} aria-hidden="true" data-presence-layer="" className="pointer-events-none fixed inset-0 z-40 overflow-hidden">
      <PeerMarks peers={view.peers} itemLabel={props.itemLabel} />
    </div>
  );
}

/** 🥞️ The others inside one page behind the overview's cards: the pointers and focus of the learners in that page's
 * room (`scope`, watched from home), placed on the page's own anchors so they scale and move with the page. Decorative;
 * nothing while the page is opened (the overlay of the page shows them then) or others' cursors are switched off. */
export function PanePeers(props: { readonly scope: string | undefined; readonly opened: boolean }): ReactElement | null {
  const { view, showCursors } = useContext(PresenceContext);
  const peers = props.scope === undefined ? [] : (view.rooms.get(props.scope) ?? []);
  const layer = useRef<HTMLDivElement>(null);
  const active = showCursors && !props.opened && peers.length > 0;
  usePlacing(layer, active, () => layer.current?.parentElement ?? null, 250);
  if (!active) return null;
  return (
    <div ref={layer} aria-hidden="true" data-pane-peers={props.scope} className="pointer-events-none absolute inset-0 z-10 overflow-hidden">
      <PeerMarks peers={peers} />
    </div>
  );
}

/** 🟢️ A learner's online mark in its palette colour, named for assistive technology. */
export function OnlineMark(props: { readonly colour: number; readonly text: QuizText }): ReactElement {
  return (
    <>
      <span aria-hidden="true" className="quiz-online" style={paintStyle(props.colour)} />
      <span className="sr-only">{props.text("quiz.presence.onlineMark")}</span>
    </>
  );
}

/** 👥️ The online count for the navbar: a people icon and the number, the words for assistive technology and wide
 * screens. Nothing until the roster room is joined. */
export function PresenceStatus(props: { readonly text: QuizText }): ReactElement | null {
  const { roster } = usePresenceView();
  if (roster === undefined) return null;
  return (
    <p data-presence-status="" className="m-0 flex min-w-0 items-center gap-single text-xs text-muted-foreground md:px-single">
      <Icon icon="users" size="small" className="shrink-0" />
      <span aria-hidden="true" className="tabular-nums md:hidden">
        {roster.online}
      </span>
      <span className="max-md:sr-only">{props.text("quiz.presence.online", { count: roster.online })}</span>
    </p>
  );
}

/** 🗒️ Who is where, as text: the online count and a disclosure listing every learner online with the place, the own
 * entry marked and learners whose tab is hidden marked away. */
export function PresenceList(props: { readonly catalog: CatalogView; readonly text: QuizText; readonly locale: QuizLocale; readonly className?: string }): ReactElement | null {
  const { catalog, text, locale } = props;
  const { roster, me, colours } = usePresenceView();
  if (roster === undefined) return null;
  return (
    <details data-presence-list="" className={cn("relative min-h-0 overflow-auto text-xs", props.className)}>
      <summary className="quiz-target cursor-pointer text-foreground">
        {text("quiz.presence.online", { count: roster.online })} · {text("quiz.presence.whoIsWhere")}
      </summary>
      <ul role="list" className="m-0 mt-single flex list-none flex-col gap-single p-0 text-muted-foreground">
        {roster.learners.map((learner) => (
          <li key={learner.tag} className="flex min-w-0 flex-wrap items-center gap-x-single">
            <OnlineMark colour={colours.get(learner.tag) ?? 0} text={text} />
            <span className="quiz-name text-foreground">
              {learner.display}
              {learner.tag === me ? ` (${text("quiz.presence.you")})` : ""}
            </span>
            <span>
              – {placeText(learner.place, catalog, text, locale)}
              {learner.active ? "" : ` (${text("quiz.presence.away")})`}
            </span>
          </li>
        ))}
      </ul>
    </details>
  );
}
//#endregion ⚛️React
