/** 👀️ Following is an ephemeral local projection of a public peer view; every local command stays with its owner. */
import { createContext, useCallback, useContext, useEffect, useLayoutEffect, useReducer, useRef, type ReactElement, type ReactNode, type RefObject } from "react";
import { presenceRun, type Place, type PresencePreferences, type PresencePresentation, type PresenceState, type RunView } from "@semio-tech/course";
import { type CourseSession } from "../🧭️session/🟦️.ts";
import { DEFAULT_BOARD, HOME_PAGES, boardKey, type CourseState, type CourseStep } from "../🧭️session/🧮️state/🟦️.ts";
import { PRESENCE_FRAME_INTERVAL_MS } from "../👥️presence/🔌️socket/🟦️.ts";
import { type CoursePresenceSnapshot } from "../👥️presence/🧭️rooms/🟦️.ts";
import type { CoursePreferences } from "../🎛️preferences/🟦️.tsx";

/** ⚡️ The learner starts watching a public tag, or returns to their own screen. */
export type FollowEvent = { readonly type: "follow-started"; readonly tag: string } | { readonly type: "follow-stopped"; readonly reason: "interaction" | "escape" | "stop" | "departed" };

/** 🎞️ Applies one follow intent without changing the quiz session or its persisted history. */
export function evolveFollow(current: string | undefined, event: FollowEvent): string | undefined {
  return event.type === "follow-started" ? event.tag : undefined;
}

/** 🧭️ Maps a public peer place onto an observer step with an ephemeral run key. */
export function followStep(place: Place, home?: string): CourseStep {
  if (place.screen === "run" || place.screen === "results") return { screen: place.screen, run: "presence-run" };
  if (place.screen === "quiz") return { screen: "home", page: place.quiz };
  if (place.screen === "home") return { screen: "home", ...(home === undefined ? {} : { page: home }) };
  if (place.screen === "identity") return { screen: "identity" };
  if (place.screen === "introduction") return { screen: "home" };
  const page = place.screen === "leaderboard" ? HOME_PAGES.leaderboard : place.screen === "preferences" ? HOME_PAGES.preferences : place.screen === "learner" ? HOME_PAGES.learner : HOME_PAGES.badges;
  return { screen: "home", page };
}

/** 🔓️ The public run shown to peers: randomized sheet, current answers and results, with private identifiers removed. */
export function publicPresenceRun(run: RunView | undefined): PresencePresentation["run"] {
  if (run === undefined) return undefined;
  return presenceRun(run);
}

/** 🪞️ A peer screen projected over shared course reads, never over the learner's private runs or achievements. */
export function followedCourseState(own: CourseState, peer: PresenceState): CourseState {
  const run = peer.view?.run;
  const profile = peer.view?.profile;
  const board = peer.view?.board ?? DEFAULT_BOARD;
  const leaderboards = { ...(peer.view?.overall === undefined ? {} : { [boardKey(DEFAULT_BOARD)]: { board: peer.view.overall, at: peer.view.overallAt ?? 0 } }), ...(peer.view?.leaderboard === undefined ? {} : { [boardKey(board)]: { board: peer.view.leaderboard, at: peer.view.leaderboardAt ?? 0 } }) };
  return {
    ...own,
    step: followStep(peer.place, peer.view?.home),
    learner: { id: `presence-${peer.tag}`, identity: peer.identity },
    learnerView: profile === undefined ? undefined : { ...profile, identity: peer.identity, learner: `presence-${peer.tag}`, runs: profile.runs.map((entry, index) => ({ ...entry, run: `presence-history-${index}` })), badges: profile.badges.map((entry, index) => ({ ...entry, run: `presence-award-${index}` })) },
    introduced: true,
    runs: run === undefined ? {} : { "presence-run": { ...run, run: "presence-run", learner: `presence-${peer.tag}` } },
    awards: peer.view?.awards === undefined ? {} : { "presence-run": peer.view.awards },
    asked: peer.view?.asked ?? [],
    board,
    leaderboards,
    crowds: peer.view?.crowd === undefined ? {} : { [peer.view.crowd.quiz]: peer.view.crowd },
    notice: undefined,
  };
}

/** 🎛️ Uses the peer's choices during observation, with an absent device choice restoring measured layout. */
export function followedPreferences(own: CoursePreferences, peer: PresencePreferences | undefined): CoursePreferences {
  return peer === undefined ? own : { ...own, ...peer, device: peer.device };
}

/** 🔒️ Gives observer screens only read capabilities; commands and automatic refreshes have no side effects. */
export function observerSession(session: CourseSession, read: () => CourseState): CourseSession {
  const idle = (): void => undefined;
  const refresh = async (): Promise<{ readonly answered: boolean }> => ({ answered: false });
  const blocked = async (): Promise<undefined> => undefined;
  return new Proxy(session, {
    get(target, key) {
      if (key === "now") return target.now.bind(target);
      if (key === "getSnapshot") return () => ({ ...target.getSnapshot(), state: read() });
      if (key === "subscribe") return () => idle;
      if (key === "refreshLeaderboard") return refresh;
      if (typeof Reflect.get(target, key) === "function") return blocked;
      return Reflect.get(target, key);
    },
  });
}

/** ⏱️ How long the latest peer view remains available through a temporary roster connection shortage. */
export const FOLLOW_REJOIN_GRACE_MS = 10_000;
/** 🚪️ A missing open-room peer is confirmed after the reconnect welcome and its next batch can arrive. */
export const FOLLOW_DEPARTURE_GRACE_MS = 500;

/** 🧍️ Keeps the latest peer view across bounded connection shortages without publishing the observed state. */
export function useFollow(snapshot: CoursePresenceSnapshot): { readonly peer?: PresenceState; readonly tag?: string; readonly waiting: boolean; readonly start: (tag: string) => void; readonly stop: (reason?: Extract<FollowEvent, { type: "follow-stopped" }>["reason"]) => void } {
  const [tag, dispatch] = useReducer(evolveFollow, undefined);
  const cache = useRef<PresenceState | undefined>(undefined);
  const missingSince = useRef<number | undefined>(undefined);
  const open = snapshot.roster?.status === "open";
  const member = open ? snapshot.roster?.members.filter((entry) => entry.state.tag === tag && entry.state.tag !== snapshot.tag).sort((a, b) => Number(b.state.active) - Number(a.state.active) || a.session.localeCompare(b.session))[0] : undefined;
  if (cache.current?.tag !== tag) { cache.current = undefined; missingSince.current = undefined; }
  if (member?.state.view !== undefined) { cache.current = member.state; missingSince.current = undefined; }
  else if (tag !== undefined && missingSince.current === undefined) missingSince.current = Date.now();
  const peer = cache.current;
  useEffect(() => {
    if (tag === undefined || member?.state.view !== undefined) return;
    if (member !== undefined) { dispatch({ type: "follow-stopped", reason: "departed" }); return; }
    const remaining = Math.max(0, FOLLOW_REJOIN_GRACE_MS - (Date.now() - (missingSince.current ?? Date.now())));
    const timer = setTimeout(() => dispatch({ type: "follow-stopped", reason: "departed" }), open ? Math.min(FOLLOW_DEPARTURE_GRACE_MS, remaining) : remaining);
    return () => clearTimeout(timer);
  }, [tag, open, member]);
  return { peer, tag: peer === undefined ? undefined : tag, waiting: tag !== undefined && member === undefined, start: (tag) => dispatch({ type: "follow-started", tag }), stop: (reason = "stop") => dispatch({ type: "follow-stopped", reason }) };
}

interface FollowContextValue {
  readonly following?: string;
  readonly presentation?: PresencePresentation;
  readonly task?: string;
  readonly onCamera: (camera: PresencePresentation["camera"]) => void;
  readonly onRevealed: (reveal: string | undefined, source?: "focus" | "pointer") => void;
  readonly onSubmission: (submission: PresencePresentation["submission"]) => void;
  readonly onSort: (sort: PresencePresentation["sort"]) => void;
  readonly onTaskPage: (page: number | undefined) => void;
  readonly onSwipe?: (page: string) => void;
  readonly onLayout?: (layout: PresencePresentation["layout"]) => void;
}
const FollowContext = createContext<FollowContextValue>({ onCamera: () => undefined, onRevealed: () => undefined, onSubmission: () => undefined, onSort: () => undefined, onTaskPage: () => undefined });

/** 📺️ Shares the remote presentation and disables observed native controls while preserving their accessible content. */
export function FollowProvider(props: FollowContextValue & { readonly children: ReactNode }): ReactElement {
  return <FollowContext.Provider value={props}>{props.following === undefined ? props.children : <fieldset disabled className="contents">{props.children}</fieldset>}</FollowContext.Provider>;
}

/** 🎥️ The observer's controlled camera and selected task, absent on the owner's interactive screen. */
export function usePresenceFollow(): FollowContextValue {
  return useContext(FollowContext);
}

function scroller(root: HTMLElement, following: boolean, swipe: string | undefined): HTMLElement {
  const visible = (element: HTMLElement, allowed?: HTMLElement): boolean => {
    if (element.closest('[hidden], [aria-hidden="true"]') !== null) return false;
    for (let current: HTMLElement | null = element; current !== null && root.contains(current); current = current.parentElement) if (current.hasAttribute("inert") && current !== allowed) return false;
    return true;
  };
  const grid = [...root.querySelectorAll<HTMLElement>("[data-course-grid][data-grid-active]")].find((element) => visible(element));
  const overview = grid?.querySelector<HTMLElement>(':scope > [data-layered-overview][data-mode="swipe"]');
  const overlay = overview?.querySelector<HTMLElement>(":scope > [data-layered-overlay]");
  const cell = overlay === undefined || overlay === null ? undefined : [...overlay.children].find((element) => element instanceof HTMLElement && element.dataset.layeredCell === (swipe ?? overview?.dataset.layeredResting));
  if (cell instanceof HTMLElement && visible(cell, following ? overlay ?? undefined : undefined)) return cell;
  return [root, ...root.querySelectorAll<HTMLElement>("*")].filter((element) => visible(element)).reduce((largest, element) => element.scrollHeight - element.clientHeight > largest.scrollHeight - largest.clientHeight ? element : largest, root);
}

/** 📸️ Captures the latest presentation at the presence frame budget and cancels queued frames on scope changes. */
export function usePresenceCapture<Value>(changed: (value: Value) => void, scope: string, active = true): (value: Value) => void {
  const latest = useRef(changed);
  const enabled = useRef(active);
  latest.current = changed;
  enabled.current = active;
  const frame = useRef<{ at: number; called?: number; pending: boolean; value?: Value; timer?: ReturnType<typeof setTimeout> }>({ at: -Infinity, pending: false });
  useEffect(() => () => {
    if (frame.current.timer !== undefined) clearTimeout(frame.current.timer);
    frame.current = { at: -Infinity, pending: false };
  }, [scope, active]);
  return useCallback((value: Value) => {
    if (!enabled.current) return;
    const next = frame.current;
    const now = Date.now();
    const invoke = next.called === undefined || now - next.at >= PRESENCE_FRAME_INTERVAL_MS;
    next.called = now;
    next.value = value;
    next.pending = true;
    const flush = (): void => {
      if (!next.pending || !enabled.current) return;
      next.pending = false;
      next.at = Date.now();
      latest.current(next.value as Value);
    };
    const expire = (): void => {
      const wait = Math.min((next.called ?? 0) + PRESENCE_FRAME_INTERVAL_MS - Date.now(), next.at + PRESENCE_FRAME_INTERVAL_MS - Date.now());
      if (wait > 0) { next.timer = setTimeout(expire, wait); return; }
      next.timer = undefined;
      flush();
    };
    if (invoke) {
      if (next.timer !== undefined) clearTimeout(next.timer);
      next.timer = setTimeout(expire, PRESENCE_FRAME_INTERVAL_MS);
      flush();
    } else if (next.timer === undefined) next.timer = setTimeout(expire, PRESENCE_FRAME_INTERVAL_MS);
  }, []);
}

/** 🧻️ Shares normalized scroll and applies remote scroll after layout changes; observer scroll never feeds back. */
export function usePresenceViewport(root: RefObject<HTMLElement | null>, key: string, remote: PresencePresentation["viewport"], changed: (viewport: PresencePresentation["viewport"]) => void, following: boolean, swipe?: string): void {
  const latest = useRef(changed);
  latest.current = changed;
  const share = usePresenceCapture(() => {
    const element = root.current;
    if (element === null) return;
    const target = scroller(element, false, swipe);
    const fraction = (value: number, maximum: number): number => Math.round(Math.max(0, Math.min(1, value / Math.max(1, maximum))) * 10_000) / 10_000;
    latest.current({ x: fraction(target.scrollLeft, target.scrollWidth - target.clientWidth), y: fraction(target.scrollTop, target.scrollHeight - target.clientHeight) });
  }, `${key}:${swipe ?? ""}`, !following);
  useEffect(() => {
    const element = root.current;
    if (element === null || following) return;
    const capture = (): void => share(undefined);
    element.addEventListener("scroll", capture, true);
    capture();
    return () => element.removeEventListener("scroll", capture, true);
  }, [root, key, swipe, following, share]);
  useLayoutEffect(() => {
    const element = root.current;
    if (element === null || !following || remote === undefined) return;
    const apply = (): void => {
      const target = scroller(element, true, swipe);
      target.scrollLeft = remote.x * Math.max(0, target.scrollWidth - target.clientWidth);
      target.scrollTop = remote.y * Math.max(0, target.scrollHeight - target.clientHeight);
    };
    apply();
    let frame = requestAnimationFrame(apply);
    const changes = new MutationObserver(() => { cancelAnimationFrame(frame); frame = requestAnimationFrame(apply); });
    changes.observe(element, { childList: true, subtree: true, attributes: true, attributeFilter: ["data-layered-resting", "data-grid-active", "data-mode"] });
    const observer = typeof ResizeObserver === "undefined" ? undefined : new ResizeObserver(apply);
    observer?.observe(scroller(element, true, swipe));
    return () => { cancelAnimationFrame(frame); observer?.disconnect(); changes.disconnect(); };
  }, [root, key, swipe, remote?.x, remote?.y, following]);
}
