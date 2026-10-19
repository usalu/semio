/** 👀️ Follow events and transient views use shared vectors and Immer as an independent event reducer oracle. */
import { produce } from "immer";
import omit from "lodash/omit";
import orderBy from "lodash/orderBy";
import throttle from "lodash/throttle";
import defaults from "lodash/defaults";
import clamp from "lodash/clamp";
import round from "lodash/round";
import { act, cleanup, fireEvent, render, renderHook, screen, within } from "@testing-library/react";
import { computeAccessibleName } from "dom-accessibility-api";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import { evolveFollow, followStep, publicPresenceRun, followedCourseState, usePresenceCapture, usePresenceViewport, type FollowEvent } from "../../🎯️targets/⚛️react/🔨️modules/👀️follow/🟦️.tsx";
import { EMPTY_PRESENCE_VIEW, FOLLOW_DEPARTURE_GRACE_MS, FOLLOW_REJOIN_GRACE_MS, Dialog, FollowProvider, LeaderboardCard, PresenceFooter, PresenceProvider, followedPreferences, initialCourseState, localStore, memoryStorageOrigin, observerSession, courseText, readPreferences, useFollow, type CoursePresenceSnapshot, type CourseSession, type RoomStatus } from "@semio-tech/course-react";
import { RunScreen } from "@semio-tech/course-react/modules/run/.tsx";
import type { Leaderboard, LeaderboardRow, Place, PresencePresentation, RunView, Sheet } from "@semio-tech/course";
import vectors from "../../🧫️fixtures/👀️follow/🔣️.json";
import sheets from "../../🧫️fixtures/🃏️sheet-assembly/🔣️.json";

afterEach(() => { cleanup(); vi.useRealTimers(); });
const callbacks = { onCamera: () => undefined, onRevealed: () => undefined, onSubmission: () => undefined, onSort: () => undefined, onTaskPage: () => undefined };

function swipeViewport(following: boolean): { readonly root: HTMLElement; readonly cells: ReadonlyMap<string, HTMLElement>; readonly ignored: readonly HTMLElement[] } {
  const root = document.createElement("main");
  root.innerHTML = '<div hidden inert><div data-course-grid="root" data-grid-active><div data-layered-overview data-mode="swipe"><div data-layered-overlay><div data-layered-cell="learner"></div></div></div></div></div><fieldset disabled><div data-course-grid="root" data-grid-active><div data-layered-overview data-mode="swipe" data-layered-resting="learner"><div data-layered-pane inert><div data-course-grid="preview" data-grid-active></div></div><div data-layered-overlay><div data-layered-cell="learner"><button>Observed Learner Action</button></div><div data-layered-cell="board"><button>Observed Board Action</button></div><div data-layered-cell="offscreen"></div></div></div></div></fieldset>';
  const overlay = root.querySelector<HTMLElement>("fieldset [data-layered-overlay]")!;
  if (following) overlay.setAttribute("inert", "");
  const cells = new Map([...overlay.querySelectorAll<HTMLElement>("[data-layered-cell]")].map((cell) => [cell.dataset.layeredCell!, cell]));
  const ignored = [root.querySelector<HTMLElement>("[hidden] [data-layered-cell]")!, root.querySelector<HTMLElement>("[data-layered-pane]")!, cells.get("offscreen")!];
  for (const cell of ignored) for (const [name, value] of Object.entries({ scrollHeight: 10_000, clientHeight: 100, scrollWidth: 10_000, clientWidth: 100, scrollTop: 9900, scrollLeft: 9900 })) Object.defineProperty(cell, name, { configurable: true, writable: true, value });
  return { root, cells, ignored };
}

describe("presentation capture budget", () => {
  for (const vector of vectors.capture) it(`captures latest camera under the 15 Hz presence budget: ${vector.id}`, () => {
    vi.useFakeTimers();
    vi.setSystemTime(0);
    const sent: [number, number][] = [];
    const oracle: [number, number][] = [];
    const hook = renderHook(() => usePresenceCapture((camera: { x: number; y: number; scale: number }) => sent.push([Date.now(), camera.x]), "home"));
    const reference = throttle((x: number) => oracle.push([Date.now(), x]), 67);
    for (const [at, x] of vector.calls) {
      act(() => { vi.advanceTimersByTime(at! - Date.now()); hook.result.current({ x: x!, y: 0, scale: 1 }); reference(x!); });
    }
    act(() => vi.advanceTimersByTime(1000));
    expect(sent).toEqual(vector.sent);
    expect(sent).toEqual(oracle);
    hook.unmount();
    reference.cancel();
  });
  it("keeps the capture callback stable, uses current sink and cancels pending frames on scope, observation and unmount", () => {
    vi.useFakeTimers();
    vi.setSystemTime(0);
    const first = vi.fn();
    const second = vi.fn();
    const hook = renderHook(({ sink, scope, active }) => usePresenceCapture(sink, scope, active), { initialProps: { sink: first, scope: "home", active: true } });
    const capture = hook.result.current;
    act(() => { capture(0); capture(1); });
    hook.rerender({ sink: second, scope: "home", active: true });
    expect(hook.result.current).toBe(capture);
    act(() => vi.advanceTimersByTime(67));
    expect(second).toHaveBeenLastCalledWith(1);
    act(() => capture(2));
    hook.rerender({ sink: second, scope: "run", active: true });
    act(() => vi.advanceTimersByTime(67));
    expect(second).toHaveBeenCalledOnce();
    act(() => { capture(3); capture(4); });
    hook.rerender({ sink: second, scope: "run", active: false });
    act(() => { capture(5); vi.advanceTimersByTime(67); });
    expect(second).toHaveBeenCalledTimes(2);
    hook.rerender({ sink: second, scope: "run", active: true });
    act(() => { capture(6); capture(7); });
    hook.unmount();
    act(() => vi.advanceTimersByTime(67));
    expect(second).toHaveBeenCalledTimes(3);
    expect(vi.getTimerCount()).toBe(0);
  });
  it("reads only the latest normalized scroll at the frame budget and never republishes observer scroll", () => {
    vi.useFakeTimers();
    vi.setSystemTime(0);
    const element = document.createElement("main");
    for (const [name, value] of Object.entries({ scrollHeight: 1000, clientHeight: 100, scrollWidth: 500, clientWidth: 100 })) Object.defineProperty(element, name, { value });
    const changed = vi.fn();
    const root = { current: element };
    const hook = renderHook(({ following }) => usePresenceViewport(root, "home", undefined, changed, following), { initialProps: { following: false } });
    const read = vi.spyOn(element, "querySelectorAll");
    for (let index = 1; index <= 4; index += 1) act(() => { vi.advanceTimersByTime(10); element.scrollTop = index * 100; element.scrollLeft = index * 50; element.dispatchEvent(new Event("scroll")); });
    expect(changed).toHaveBeenCalledOnce();
    expect(read).not.toHaveBeenCalled();
    act(() => vi.advanceTimersByTime(27));
    expect(changed).toHaveBeenLastCalledWith({ x: 0.5, y: 0.4444 });
    expect(read.mock.calls.filter(([selector]) => selector === "*")).toHaveLength(1);
    act(() => element.dispatchEvent(new Event("scroll")));
    hook.rerender({ following: true });
    act(() => { element.dispatchEvent(new Event("scroll")); vi.advanceTimersByTime(67); });
    expect(changed).toHaveBeenCalledTimes(2);
    hook.unmount();
    expect(vi.getTimerCount()).toBe(0);
  });
  it("captures the current settled swipe cell and recaptures another card without a scroll event", () => {
    vi.useFakeTimers();
    const { root, cells } = swipeViewport(false);
    for (const vector of vectors.scroll) for (const [name, value] of Object.entries(vector.owner)) Object.defineProperty(cells.get(vector.page)!, name, { configurable: true, writable: true, value });
    const changed = vi.fn();
    const hook = renderHook(({ swipe }) => usePresenceViewport({ current: root }, "home", undefined, changed, false, swipe), { initialProps: { swipe: vectors.scroll[0]!.page } });
    for (const vector of vectors.scroll) {
      hook.rerender({ swipe: vector.page });
      expect(changed).toHaveBeenLastCalledWith(vector.viewport);
      const oracle = { x: round(clamp(vector.owner.scrollLeft / (vector.owner.scrollWidth - vector.owner.clientWidth), 0, 1), 4), y: round(clamp(vector.owner.scrollTop / (vector.owner.scrollHeight - vector.owner.clientHeight), 0, 1), 4) };
      expect(changed.mock.calls.at(-1)?.[0]).toEqual(oracle);
    }
    hook.unmount();
    expect(vi.getTimerCount()).toBe(0);
  });
  it("applies remote scroll to the disabled observed swipe cell and skips hidden own content, inert previews and taller offscreen cards", () => {
    vi.useFakeTimers();
    const { root, cells, ignored } = swipeViewport(true);
    for (const vector of vectors.scroll) for (const [name, value] of Object.entries(vector.observed)) Object.defineProperty(cells.get(vector.page)!, name, { configurable: true, writable: true, value });
    const changed = vi.fn();
    const hook = renderHook(({ swipe, viewport }) => usePresenceViewport({ current: root }, "home", viewport, changed, true, swipe), { initialProps: { swipe: vectors.scroll[0]!.page, viewport: vectors.scroll[0]!.viewport } });
    for (const vector of vectors.scroll) {
      hook.rerender({ swipe: vector.page, viewport: vector.viewport });
      const cell = cells.get(vector.page)!;
      expect({ left: cell.scrollLeft, top: cell.scrollTop }).toEqual(vector.expected);
      expect(cell.querySelector("button")!.matches(":disabled")).toBe(true);
      act(() => cell.dispatchEvent(new Event("scroll", { bubbles: true })));
    }
    for (const cell of ignored) expect({ left: cell.scrollLeft, top: cell.scrollTop }).toEqual({ left: 9900, top: 9900 });
    expect(changed).not.toHaveBeenCalled();
    hook.unmount();
    expect(vi.getTimerCount()).toBe(0);
  });
});

describe("ephemeral following", () => {
  for (const vector of vectors.controls) it(`exposes the observed ${vector.task} controls as disabled without hiding their answers (${vector.sheet})`, async () => {
    const sheet = sheets.sheets.find((entry) => entry.id === vector.sheet)!.sheet as unknown as Sheet;
    const run: RunView = { run: "presence-run", learner: "presence-alice", quiz: "architecture", status: "open", startedAt: 1, sheet, answers: {} };
    const state = initialCourseState({ introduced: true, learner: { id: "own", identity: { kind: "anonymous" } }, runs: { "presence-run": run } });
    const session = { now: () => 42, answer: vi.fn(), openTask: vi.fn(), submit: vi.fn(), askCrowd: vi.fn(), unaskCrowd: vi.fn() } as unknown as CourseSession;
    const result = render(<><FollowProvider {...callbacks} following="alice" task={vector.task}><RunScreen session={session} state={state} run="presence-run" locale="en" text={courseText("en")} readOnly /></FollowProvider><button>Stop Following</button></>);
    const task = result.container.querySelector<HTMLElement>(`[data-presence-anchor="task:${vector.task}"]`)!;
    expect(task).not.toBeNull();
    const controls = within(task).getAllByRole(vector.role as "button" | "combobox" | "textbox");
    const observed = [...result.container.querySelectorAll<HTMLButtonElement | HTMLInputElement | HTMLSelectElement>(".course-run button, .course-run input, .course-run select")];
    expect(controls.length).toBeGreaterThan(0);
    expect(observed.length).toBeGreaterThan(0);
    for (const control of observed) {
      expect(control.matches(":disabled")).toBe(vector.disabled);
      expect(computeAccessibleName(control).length).toBeGreaterThan(0);
    }
    const user = userEvent.setup();
    await user.tab();
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Stop Following" }));
    for (const control of observed) if (control instanceof HTMLButtonElement) await user.click(control);
    expect(session.answer).not.toHaveBeenCalled();
    expect(session.openTask).not.toHaveBeenCalled();
    expect(session.submit).not.toHaveBeenCalled();
    expect(session.askCrowd).not.toHaveBeenCalled();
    expect(session.unaskCrowd).not.toHaveBeenCalled();
    expect(screen.queryByRole("alertdialog")).toBeNull();
  });
  it("replays shared follow events like the independent reducer", () => {
    let actual: string | undefined;
    let oracle: { tag?: string } = {};
    for (const vector of vectors.events) {
      actual = evolveFollow(actual, vector as FollowEvent);
      oracle = produce(oracle, (draft) => {
        if (vector.type === "follow-started") draft.tag = vector.tag;
        else delete draft.tag;
      });
      expect(actual).toBe(vector.expected);
      expect(actual).toBe(oracle.tag);
    }
  });
  it("maps public places without private run identifiers", () => {
    for (const vector of vectors.places) expect(followStep(vector.place as Place, "home" in vector ? vector.home : undefined)).toEqual(vector.expected);
  });
  it("projects randomized remote sheets without changing saved runs", () => {
    const own = initialCourseState({ introduced: false, runs: {} });
    const run = { run: "private-run", learner: "private-learner", quiz: "architecture", status: "open", answers: {}, startedAt: 42, sheet: { tasks: [{ id: "order", items: [{ id: "second" }, { id: "first" }] }] } } as unknown as RunView;
    const publicRun = publicPresenceRun(run);
    expect(publicRun).not.toHaveProperty("learner");
    expect(publicRun).not.toHaveProperty("run");
    expect(publicRun).toEqual(omit(run, ["run", "learner"]));
    const shown = followedCourseState(own, { tag: "alice", identity: { kind: "anonymous" }, place: { screen: "run", quiz: "architecture" }, active: true, view: { run: publicRun } });
    expect(shown.runs["presence-run"]?.sheet.tasks[0]?.items.map((item) => item.id)).toEqual(["second", "first"]);
    expect(own.runs).toEqual({});
    expect(own.step).toEqual({ screen: "home" });
    expect(shown.learner?.id).not.toBe("private-learner");
  });
  it("blocks commands and automatic observer refreshes", async () => {
    const own = initialCourseState({ introduced: true, runs: {} });
    const answer = vi.fn();
    const refreshCrowd = vi.fn();
    const session = { now: () => 42, answer, refreshCrowd, getSnapshot: () => ({ state: own }) } as unknown as CourseSession;
    const observing = observerSession(session, () => own);
    observing.answer("presence-run", "task", { kind: "sorting", order: [] });
    await observing.refreshCrowd("architecture");
    expect(answer).not.toHaveBeenCalled();
    expect(refreshCrowd).not.toHaveBeenCalled();
    expect(observing.now()).toBe(42);
  });
  it("uses the peer's measured layout when no explicit device choice was published", () => {
    const own = { ...readPreferences(localStore(memoryStorageOrigin().tab(), "follow")), device: "tablet" as const };
    for (const vector of vectors.preferences) {
      const shown = followedPreferences(own, vector.peer as PresencePresentation["preferences"]);
      expect(shown.device).toBe(vector.expectedDevice);
      expect(shown.theme).toBe(vector.expectedTheme);
      expect(JSON.parse(JSON.stringify(shown))).toEqual(defaults({}, vector.peer, omit(own, ["device"])));
      expect(own.device).toBe("tablet");
    }
    expect(followedPreferences(own, undefined)).toBe(own);
  });
  it("ends observation when the person leaves or is themselves following", () => {
    vi.useFakeTimers();
    const member = { session: "peer", surface: "course", colour: 1, state: { tag: "alice", identity: { kind: "anonymous" as const }, place: { screen: "home" as const }, active: true, view: { locale: "en" as const } } };
    const snapshot: CoursePresenceSnapshot = { tag: "own", roster: { status: "open", members: [member], self: "mine", colour: undefined, refused: undefined, watched: new Map() }, room: undefined, thinking: undefined, scopes: { room: undefined, thinking: undefined } };
    const hook = renderHook(({ snapshot }) => useFollow(snapshot), { initialProps: { snapshot } });
    act(() => hook.result.current.start("alice"));
    expect(hook.result.current.peer?.tag).toBe("alice");
    hook.rerender({ snapshot: { ...snapshot, roster: { ...snapshot.roster!, members: [{ ...member, state: { ...member.state, view: undefined } }] } } });
    expect(hook.result.current.tag).toBeUndefined();
    act(() => hook.result.current.start("alice"));
    hook.rerender({ snapshot: { ...snapshot, roster: { ...snapshot.roster!, members: [] } } });
    act(() => vi.advanceTimersByTime(FOLLOW_DEPARTURE_GRACE_MS));
    expect(hook.result.current.tag).toBeUndefined();
  });
  it("keeps the peer projection through a short roster reconnect and stops after a bounded long outage", () => {
    vi.useFakeTimers();
    const member = { session: "peer", surface: "course", colour: 1, state: { tag: "alice", identity: { kind: "anonymous" as const }, place: { screen: "home" as const }, active: true, view: { locale: "en" as const } } };
    const snapshot: CoursePresenceSnapshot = { tag: "own", roster: { status: "open", members: [member], self: "mine", colour: undefined, refused: undefined, watched: new Map() }, room: undefined, thinking: undefined, scopes: { room: undefined, thinking: undefined } };
    const hook = renderHook(({ snapshot }) => useFollow(snapshot), { initialProps: { snapshot } });
    act(() => hook.result.current.start("alice"));
    hook.rerender({ snapshot: { ...snapshot, roster: { ...snapshot.roster!, status: "waiting", members: [] } } });
    act(() => vi.advanceTimersByTime(FOLLOW_REJOIN_GRACE_MS - FOLLOW_DEPARTURE_GRACE_MS - 1));
    expect(hook.result.current.tag).toBe("alice");
    expect(hook.result.current.peer?.view?.locale).toBe("en");
    expect(hook.result.current.waiting).toBe(true);
    hook.rerender({ snapshot: { ...snapshot, roster: { ...snapshot.roster!, status: "open", members: [] } } });
    act(() => vi.advanceTimersByTime(FOLLOW_DEPARTURE_GRACE_MS - 1));
    hook.rerender({ snapshot });
    expect(hook.result.current.waiting).toBe(false);
    expect(hook.result.current.tag).toBe("alice");
    for (const step of vectors.reconnect) {
      hook.rerender({ snapshot: { ...snapshot, roster: { ...snapshot.roster!, status: step.status as RoomStatus, members: [] } } });
      act(() => vi.advanceTimersByTime(step.elapsed));
      expect(hook.result.current.tag).toBe(step.following);
    }
    expect(hook.result.current.tag).toBeUndefined();
  });
  it("keeps a stop control available while the roster is reconnecting", () => {
    const stop = vi.fn();
    render(<PresenceProvider view={EMPTY_PRESENCE_VIEW} setTask={() => undefined}><PresenceFooter text={courseText("en")} locale="en" following="alice" followingName="Alice" waiting onFollow={() => undefined} onStop={stop} /></PresenceProvider>);
    expect(screen.getByText("Waiting for Alice to reconnect…")).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "Stop following" }));
    expect(stop).toHaveBeenCalledOnce();
  });
  it("keeps the observed dialog language outside the owner's language subtree", () => {
    const shown = (locale: "en" | "de") => <div className="course-app" lang="en"><section lang={locale}><Dialog id="follow-lang" role="dialog" title="Remote" describedBy="follow-desc" icon={null} observed locale={locale} onEscape={() => undefined}><p id="follow-desc">Remote copy</p></Dialog></section></div>;
    const result = render(shown("de"));
    expect(screen.getByRole("dialog").getAttribute("lang")).toBe("de");
    expect(screen.getByRole("dialog").getAttribute("aria-modal")).toBeNull();
    result.rerender(shown("en"));
    expect(screen.getByRole("dialog").getAttribute("lang")).toBe("en");
  });
  it("renders the peer's published leaderboard and sort independently of the owner's cached board", () => {
    const rows: readonly LeaderboardRow[] = [{ tag: "alice", identity: { kind: "pseudonym", handle: "alice" }, rank: 1, total: 30, best: {}, badges: [], runs: 2, reachedAt: 1, lastActivity: 1 }, { tag: "bob", identity: { kind: "pseudonym", handle: "bob" }, rank: 2, total: 10, best: {}, badges: [], runs: 1, reachedAt: 2, lastActivity: 2 }];
    const leaderboard: Leaderboard = { period: "all-time", rows, learners: 2, submissions: 3 };
    const own = initialCourseState({ introduced: true, learner: { id: "own", identity: { kind: "anonymous" } }, runs: {} });
    const published = vi.fn();
    const session = {} as CourseSession;
    const shown = (sort: PresencePresentation["sort"]) => {
      const presentation: PresencePresentation = { leaderboard, leaderboardAt: 42, board: { period: "all-time" }, sort };
      const state = followedCourseState(own, { tag: "alice", identity: { kind: "pseudonym", handle: "alice" }, place: { screen: "leaderboard" }, active: true, view: presentation });
      return <FollowProvider {...callbacks} following="alice" presentation={presentation} onSort={published}><LeaderboardCard session={session} state={state} text={courseText("en")} locale="en" revealed onOpen={() => undefined} /></FollowProvider>;
    };
    const result = render(shown(vectors.sort[0] as PresencePresentation["sort"]));
    for (const vector of vectors.sort) {
      result.rerender(shown(vector as PresencePresentation["sort"]));
      const names = [...result.container.querySelectorAll("tbody [data-cell='name']")].map((cell) => cell.textContent?.replace(" (you)", ""));
      const oracle = orderBy(rows, ["total"], [vector.direction === "ascending" ? "asc" : "desc"]).map((row) => row.tag);
      expect(names).toEqual(vector.expected);
      expect(names).toEqual(oracle);
      expect(result.container.querySelector("[aria-sort]")?.getAttribute("aria-sort")).toBe(vector.direction);
    }
    expect(own.leaderboards).toEqual({});
    expect(published).not.toHaveBeenCalled();
  });
  for (const locale of ["en", "de"] as const) it(`offers accessible public peer follow in the footer (${locale})`, () => {
    const follow = vi.fn();
    const view = { ...EMPTY_PRESENCE_VIEW, me: "own", roster: { online: 2, active: 2, quizzes: {}, learners: [{ tag: "own", identity: { kind: "anonymous" as const }, display: "Me", place: { screen: "home" as const }, active: true, session: "own", sessions: ["own"] }, { tag: "alice", identity: { kind: "anonymous" as const }, display: "Alice", place: { screen: "home" as const }, active: true, session: "peer", sessions: ["peer"], view: { locale } }] } };
    render(<PresenceProvider view={view} setTask={() => undefined}><PresenceFooter text={courseText(locale)} locale={locale} onFollow={follow} onStop={() => undefined} /></PresenceProvider>);
    const button = screen.getByRole("button", { name: vectors.labels[locale] });
    expect(computeAccessibleName(button)).toBe(vectors.labels[locale]);
    fireEvent.click(button);
    expect(follow).toHaveBeenCalledWith("alice");
    expect(screen.getByText(locale === "en" ? "Me (you)" : "Me (du)")).toBeTruthy();
  });
  it("mirrors the leader's confirmation and progress without exposing command actions or trapping the footer", () => {
    const sheet = sheets.sheets.find((entry) => entry.id === "icons-1")!.sheet as unknown as Sheet;
    const run: RunView = { run: "presence-run", learner: "presence-alice", quiz: "architecture", status: "open", startedAt: 1, sheet, answers: {} };
    const own = initialCourseState({ introduced: true, learner: { id: "own", identity: { kind: "anonymous" } }, runs: { "presence-run": run } });
    const session = { now: () => 42, answer: vi.fn(), openTask: vi.fn(), submit: vi.fn() } as unknown as CourseSession;
    const shown = (submission?: PresencePresentation["submission"]) => <div className="course-app"><footer><button>Footer Stop</button></footer><FollowProvider {...callbacks} following="alice" presentation={{ submission }}><RunScreen session={session} state={own} run="presence-run" locale="en" text={courseText("en")} readOnly /></FollowProvider></div>;
    const result = render(shown());
    for (const vector of vectors.dialogs) {
      result.rerender(shown(vector as PresencePresentation["submission"]));
      const dialog = screen.getByRole("alertdialog");
      expect(dialog.getAttribute("aria-modal")).toBeNull();
      expect(screen.getByRole("button", { name: "Footer Stop" }).closest("[inert]")).toBeNull();
      expect([...dialog.querySelectorAll("button")].every((button) => button.disabled)).toBe(true);
      if (vector.phase === "saving") expect(dialog.querySelector("progress")?.value).toBe(vector.done);
    }
    result.rerender(shown());
    expect(screen.queryByRole("alertdialog")).toBeNull();
    expect(session.submit).not.toHaveBeenCalled();
  });
});
