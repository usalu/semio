/** 📚️ The course application shell, without the package's task and view reexports. */
import { Component, StrictMode, useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState, useSyncExternalStore, type ReactElement, type ReactNode } from "react";
import { createRoot, type Root } from "react-dom/client";
import { presenceProfile, type PresencePreferences, type PresencePresentation } from "@semio-tech/course";
import { retryWithJitteredBackoff } from "@semio-tech/framework/async";
import {
  DEFAULT_UI_DRIVER,
  Navbar,
  ResponsiveLabel,
  UI_AVAILABLE_HEIGHT,
  UI_MOBILE_MEDIA_QUERY,
  UI_TABLET_MEDIA_QUERY,
  bootstrapElementsSurfaceChromeDocument,
  elementsSurfaceDeviceForLayout,
  elementsSurfaceDeviceForMatches,
  navbarFillItem,
  useElementsSurfaceChrome,
  useMediaQuery,
  type NavbarItem,
} from "@semio-tech/ui-react/chrome";
import { COURSE_LOCALES, applyLocale, courseLocaleLoaded, loadCourseLocales, localized, preferredLocale, courseText, titled, type CourseLabelKey, type CourseLocale, type CourseText } from "../🌐️i18n/🟦️.ts";
import { browserStorageArea, localStore, type LocalStore, type StorageArea } from "../💾️persistence/🟦️.ts";
import { ProctorClient, RETRY_TIMING, proctorTransport, type ProctorConnect, type RetryTiming } from "../🛂️proctor/🟦️.ts";
import { Deputy, type CourseMaterial, type CourseMaterialSource } from "../🫡️deputy/🟦️.ts";
import { CourseSession } from "../🧭️session/🟦️.ts";
import { overallLeaderboard, runAwards, shownLeaderboard, type CourseNotice, type CourseState, type CourseStep } from "../🧭️session/🧮️state/🟦️.ts";
import { type CourseConnection } from "../🧭️session/📋️session-model/🟦️.ts";
import { LanguageChoice, LanguageSwitch, effectiveIconMotion, everyLanguage, readPreferences, textScale, withPets, writePreferences, type CoursePreferences } from "../🎛️preferences/🟦️.tsx";
import { COURSE_LANGUAGE_ANCHOR, CourseIntroduction } from "../👋️introduction/🟦️.tsx";
import { IdentityScreen, learnerName, noticeProblem } from "../🪪️identity/🟦️.tsx";
import type { HomeScreen } from "../🏠️home/🟦️.tsx";
import { pageLabel } from "../🏠️home/🗺️pages/🟦️.ts";
import { NavigationControls, placeName, useAddress } from "../🚏️navigation/🟦️.tsx";
import type { RunScreen } from "../▶️run/🟦️.tsx";
import type { ResultsScreen } from "../🏁️results/🟦️.tsx";
import { LegalFooter, type CourseLegal } from "../⚖️legal/🟦️.tsx";
import { BodyButton, CardAction, CardIcon, ProblemNote, CourseCard, cn, playTappedIcons } from "../🪟️chrome/🟦️.tsx";
import { PresenceFooter, PresenceOverlay, PresenceProvider, useDocumentVisible, usePresencePointer } from "../👥️presence/🟦️.tsx";
import { CoursePresence, presencePlace, presenceDrafts, sheetItemLabels } from "../👥️presence/🧭️rooms/🟦️.ts";
import { presenceSelf, presenceView } from "../👥️presence/👁️view/🟦️.ts";
import { type PresenceConnect } from "../👥️presence/🔌️socket/🟦️.ts";
import { FollowProvider, followedPreferences, followedCourseState, observerSession, publicPresenceRun, useFollow, usePresenceCapture, usePresenceViewport } from "../👀️follow/🟦️.tsx";
import { PetsSwitch, CoursePets, CoursePetsProvider, effectivePetMode, switchedPets, type CoursePetsSource } from "../🐾️pets/🟦️.tsx";
import "../../🎨️.css";

//#region 📚️App
/** ⚙️ What a course site passes in: the proctor base URL (`""` is the site's own origin), the course id as tenant, the
 * site's material (the course and its quizzes: with it the course shows at once and the device decides by itself
 * while the proctor is away; handed in as a {@link CourseMaterialSource} it is fetched while the client already shows,
 * and the device decides by itself from the moment it has arrived), the site's logo URL for the navbar, the site's legal pages for the footer
 * and the site's pets (where its menagerie comes from; fetched only for a learner who wants pets). The optional seams
 * replace the network, the presence sockets, the storage, the browser's language list and the retry timing. */
export interface CourseOptions {
  readonly proctor: string;
  readonly tenant: string;
  readonly material?: CourseMaterial | CourseMaterialSource;
  readonly logo?: string;
  readonly legal?: CourseLegal;
  readonly pets?: CoursePetsSource;
  readonly transport?: ProctorConnect;
  readonly presence?: PresenceConnect;
  readonly storage?: StorageArea;
  readonly languages?: readonly string[];
  readonly timing?: RetryTiming;
}

function browserLanguages(): readonly string[] {
  if (typeof navigator === "undefined") return [];
  return navigator.languages !== undefined && navigator.languages.length > 0 ? navigator.languages : [navigator.language];
}

/** 🚥️ How urgent the state of the connection is: all saved, something under way, or answers cannot leave the device. */
export type ConnectionTone = "calm" | "busy" | "alert";

/** 📶️ One sentence about the connection before it is put into a language: its key and the values it carries. */
interface ConnectionSentence {
  readonly key: CourseLabelKey;
  readonly values?: Readonly<Record<string, number>>;
}

/** 🚧️ Whether the proctor cannot take anything from this client: it does not answer, or it speaks another contract. */
function away(connection: CourseConnection): boolean {
  return connection.reachability === "unreachable" || connection.reachability === "incompatible";
}

/** 🔌️ The state of the connection as the indicator shows it: connecting until the proctor first answers, then saved,
 * saving, a busy proctor (it answers, but asks to slow down), a page older than the proctor's contract (kept on the
 * device until it is reloaded), or answers held on the device while the proctor does not answer or speaks an older or
 * foreign contract — where a deputy decides meanwhile, everything is saved there. */
export function connectionState(connection: CourseConnection): ConnectionSentence & { readonly tone: ConnectionTone } {
  if (!connection.online) return { key: "course.connection.offline", tone: "alert" };
  if (connection.reachability === "incompatible" && connection.contract === "newer") return { key: "course.connection.outdated", tone: "alert" };
  if (away(connection) && connection.deputy) return connection.pending > 0 ? { key: "course.connection.localWaiting", values: { waiting: connection.pending }, tone: "alert" } : { key: "course.connection.local", tone: "alert" };
  if (away(connection)) return connection.pending > 0 ? { key: "course.connection.reconnectingWaiting", values: { waiting: connection.pending }, tone: "alert" } : { key: "course.connection.reconnecting", tone: "alert" };
  if (connection.reachability === "throttled") return { key: "course.connection.throttled", tone: "busy" };
  if (connection.pending > 0) return { key: "course.connection.saving", values: { waiting: connection.pending }, tone: "busy" };
  if (connection.reachability === "unknown") return { key: "course.connection.connecting", tone: "busy" };
  return { key: "course.connection.saved", tone: "calm" };
}

/** 📨️ What the connection indicator says and how urgently. */
export function connectionMessage(connection: CourseConnection, text: CourseText): { readonly message: string; readonly tone: ConnectionTone } {
  const state = connectionState(connection);
  return { message: text(state.key, state.values), tone: state.tone };
}

/** ⏳️ What a screen whose data has not arrived yet says: connecting, unreachable (with a retry), out of date (with a
 * reload), busy (the client retries by itself and offers no button to ask a busy proctor even more often), refused, or
 * loading. */
export function waitingMessage(connection: CourseConnection, notice: CourseNotice | undefined, text: CourseText): { readonly message: string; readonly retry: boolean; readonly reload?: true } {
  if (!connection.online) return { message: text("course.connection.offline"), retry: true };
  if (connection.reachability === "incompatible" && connection.contract === "newer") return { message: text("course.app.outdated"), retry: false, reload: true };
  if (away(connection)) return { message: text("course.app.unreachable"), retry: true };
  if (connection.reachability === "throttled") return { message: text("course.app.busy"), retry: false };
  if (connection.reachability === "unknown") return { message: text("course.connection.connecting"), retry: false };
  if (notice?.kind === "refused") return { message: text("course.app.failed"), retry: true };
  return { message: text("course.app.fetching"), retry: false };
}

function Waiting(props: { readonly connection: CourseConnection; readonly notice: CourseNotice | undefined; readonly text: CourseText; readonly onRetry: () => void }): ReactElement {
  const { connection, notice, text, onRetry } = props;
  const waiting = waitingMessage(connection, notice, text);
  return (
    <div aria-busy="true" className="min-w-0">
      <CourseCard id="course-waiting" card="waiting" headingLevel={1} focusableHeading icon={<CardIcon icon="info" />} title={text("course.app.loading")}>
        <p role="status" className="m-0 text-sm">
          {waiting.message}
        </p>
        {waiting.retry ? <BodyButton onClick={onRetry}>{text("course.app.retry")}</BodyButton> : null}
        {waiting.reload ? <BodyButton onClick={reloadPage}>{text("course.app.reload")}</BodyButton> : null}
      </CourseCard>
    </div>
  );
}

/** 🔄️ Loads the page anew: a page older than the proctor's contract gets the current one. */
function reloadPage(): void {
  window.location.reload();
}

/** 🧯️ One screen that may fail to render: a screen that throws shows what happened instead — the rest of the client
 * stays — with a way to show it again and a reload. Everything the learner did lives in the session, so nothing is lost;
 * the next screen starts afresh. */
export class ScreenBoundary extends Component<{ readonly text: CourseText; readonly children: ReactNode }, { readonly error: Error | undefined }> {
  override state: { readonly error: Error | undefined } = { error: undefined };

  static getDerivedStateFromError(error: unknown): { readonly error: Error } {
    return { error: error instanceof Error ? error : new Error(String(error)) };
  }

  override render(): ReactNode {
    const { text, children } = this.props;
    const { error } = this.state;
    if (error === undefined) return children;
    return (
      <Page>
        <ProblemNote problem={{ message: text("course.app.crashed"), detail: error.message }} text={text} id="course-crashed">
          <BodyButton onClick={() => this.setState({ error: undefined })}>{text("course.app.retry")}</BodyButton>
          <BodyButton onClick={reloadPage}>{text("course.app.reload")}</BodyButton>
        </ProblemNote>
      </Page>
    );
  }
}

function stepKey(step: CourseStep): string {
  return step.screen === "run" || step.screen === "results" ? `${step.screen}:${step.run}` : "home";
}

function screenTitle(state: CourseState, locale: CourseLocale, text: CourseText): string {
  const { step, course } = state;
  if (step.screen === "introduction") return course === undefined ? text("course.app.loading") : localized(course.introduction.title, locale);
  if (step.screen === "identity") return text("course.identity.title");
  if (step.screen !== "home") return placeName(step, state, locale, text) ?? text("course.app.loading");
  if (course === undefined) return text("course.app.loading");
  return (step.page === undefined ? undefined : pageLabel(step.page, state, locale, text)) ?? text("course.home.title");
}

/** 🏷️ The title of the document: what the screen shows, then the course's title, in `locale` — the loading text
 * until the course arrived; without a locale the word for language in every offered language, as the chooser shows. */
export function documentTitle(state: CourseState, locale: CourseLocale | undefined): string {
  if (locale === undefined) return everyLanguage("course.preferences.language");
  const screen = screenTitle(state, locale, courseText(locale));
  const site = state.course === undefined ? undefined : titled(state.course, locale).full;
  return site === undefined || site === screen ? screen : `${screen} · ${site}`;
}

/** 🖼️ A screen other than home: it scrolls on its own, centred at a comfortable reading width — a `wide` one (a run,
 * its results) takes the room of a large screen for its panes side by side. */
function Page(props: { readonly wide?: boolean; readonly children: ReactNode }): ReactElement {
  return (
    <div className="relative min-h-0 flex-1 overflow-auto p-double">
      <div className={cn("course-page mx-auto w-full", props.wide ? "max-w-[100rem]" : "max-w-6xl")}>
        {props.children}
      </div>
    </div>
  );
}

/** 💤️ Independently loaded home, run and results screens. */
interface CourseScreens {
  readonly HomeScreen?: typeof HomeScreen;
  readonly RunScreen?: typeof RunScreen;
  readonly ResultsScreen?: typeof ResultsScreen;
}

/** 🗺️ The independently loaded module needed for the current screen. */
export function courseScreen(screen: CourseStep["screen"]): keyof CourseScreens {
  return screen === "run" ? "RunScreen" : screen === "results" ? "ResultsScreen" : "HomeScreen";
}

function loadCourseScreen(screen: keyof CourseScreens): Promise<CourseScreens> {
  switch (screen) {
    case "HomeScreen": return import("../🏠️home/🟦️.tsx").then(module => ({ HomeScreen: module.HomeScreen }));
    case "RunScreen": return import("../▶️run/🟦️.tsx").then(module => ({ RunScreen: module.RunScreen }));
    case "ResultsScreen": return import("../🏁️results/🟦️.tsx").then(module => ({ ResultsScreen: module.ResultsScreen }));
  }
}

/** 📥️ Loads the current screen with cancellation and retry during brief connection shortages. */
function useCourseScreens(screen: keyof CourseScreens): CourseScreens {
  const [screens, setScreens] = useState<CourseScreens>({});
  const missing = screens[screen] === undefined;
  useEffect(() => {
    if (!missing) return;
    const abort = new AbortController();
    retryWithJitteredBackoff(() => loadCourseScreen(screen), { ...RETRY_TIMING, signal: abort.signal }).then(loaded => {
      if (!abort.signal.aborted) setScreens(current => ({ ...current, ...loaded }));
    }, () => undefined);
    return () => abort.abort();
  }, [screen, missing]);
  return screens;
}

function Screen(props: {
  readonly session: CourseSession;
  readonly state: CourseState;
  readonly connection: CourseConnection;
  readonly text: CourseText;
  readonly locale: CourseLocale;
  readonly preferences: CoursePreferences;
  readonly onPreferences: (preferences: CoursePreferences) => void;
  readonly readOnly?: boolean;
}): ReactElement {
  const { session, state, connection, text, locale, preferences, onPreferences } = props;
  const step = state.step;
  const screens = useCourseScreens(courseScreen(step.screen));
  const [introductionPage, setIntroductionPage] = useState<string | undefined>(undefined);
  const waiting = (
    <Page>
      <Waiting connection={connection} notice={state.notice} text={text} onRetry={() => session.reconnect()} />
    </Page>
  );
  if (step.screen === "home" || step.screen === "identity" || step.screen === "introduction") {
    if (state.course === undefined || screens.HomeScreen === undefined) return waiting;
    const homeState = step.screen === "home" ? state : { ...state, step: step.screen === "identity" ? state.trail.back.findLast(previous => previous.screen === "home") ?? { screen: "home" as const } : { screen: "home" as const } };
    return (
      <>
        <screens.HomeScreen session={session} state={homeState} text={text} locale={locale} preferences={preferences} onPreferences={onPreferences} introductionPage={introductionPage} />
        {step.screen === "identity" ? <IdentityScreen session={session} text={text} observed={props.readOnly} onDismiss={() => session.open(homeState.step, true)} /> : !props.readOnly && ((!state.introduced && homeState.step.screen === "home" && homeState.step.page === undefined) || step.screen === "introduction") ? <CourseIntroduction course={state.course} locale={locale} onDismiss={() => session.readIntroduction()} onPageChange={setIntroductionPage} /> : null}
      </>
    );
  }
  switch (step.screen) {
    case "run":
      return state.runs[step.run] === undefined || screens.RunScreen === undefined ? (
        waiting
      ) : (
        <Page wide>
          <screens.RunScreen key={step.run} session={session} state={state} run={step.run} text={text} locale={locale} others={preferences.others} readOnly={props.readOnly} />
        </Page>
      );
    case "results":
      return state.runs[step.run] === undefined || screens.ResultsScreen === undefined ? (
        waiting
      ) : (
        <Page wide>
          <screens.ResultsScreen session={session} state={state} run={step.run} text={text} locale={locale} others={preferences.others} />
        </Page>
      );
  }
}

function useRootTextScale(scale: number): void {
  useLayoutEffect(() => {
    const root = document.documentElement;
    root.style.setProperty("--course-text-scale", String(scale));
    return () => {
      root.style.removeProperty("--course-text-scale");
    };
  }, [scale]);
}

const TONE_GLYPHS: { readonly [T in ConnectionTone]: string } = { alert: "⚠", busy: "↻", calm: "✓" };

/** 🔗️ The connection indicator: text to see — never a live region, because saving and saved alternate with every
 * answer — and beside it one polite status that speaks only when the connection enters an alert state (once, however
 * the waiting count or the kind of outage changes meanwhile) and when it has recovered. */
export function ConnectionStatus(props: { readonly connection: CourseConnection; readonly text: CourseText }): ReactElement {
  const { text } = props;
  const shown = connectionState(props.connection);
  const alert = shown.tone === "alert";
  const [spoken, setSpoken] = useState<{ readonly alert: boolean; readonly said?: ConnectionSentence }>({ alert: false });
  if (alert !== spoken.alert) setSpoken({ alert, said: alert ? { key: shown.key, values: shown.values } : { key: "course.connection.restored" } });
  return (
    <>
      <p data-tone={shown.tone} className="course-connection m-0 flex min-w-0 items-center gap-single text-xs text-muted-foreground md:px-single">
        <span aria-hidden="true">{TONE_GLYPHS[shown.tone]}</span>
        <span className="sr-only">{text("course.connection.label")}: </span>
        <span className="truncate max-md:sr-only">{text(shown.key, shown.values)}</span>
      </p>
      <p role="status" data-connection-announcer="" className="sr-only">
        {spoken.said === undefined ? "" : text(spoken.said.key, spoken.said.values)}
      </p>
    </>
  );
}

interface ObservedScene { readonly state: CourseState; readonly locale: CourseLocale; }

interface ClientProps {
  readonly options: CourseOptions;
  readonly session: CourseSession;
  readonly presence: CoursePresence;
  readonly state: CourseState;
  readonly connection: CourseConnection;
  readonly locale: CourseLocale;
  readonly preferences: CoursePreferences;
  readonly onPreferences: (preferences: CoursePreferences) => void;
  readonly onObservedPreferences: (preferences: PresencePreferences | undefined) => void;
  readonly onObservedScene: (scene: ObservedScene | undefined) => void;
}

/** 🖥️ The client once a language is known: the navbar — navigation on its left (overview, back,
 * forward, up), what the quizzes are about with the site's logo in its middle, the connection, who is online and the
 * language on its right — the screen of the step, the footer, the others' cursors and the pets. */
function Client(props: ClientProps): ReactElement {
  const { options, session, presence, state, connection, locale, preferences, onPreferences, onObservedPreferences, onObservedScene } = props;
  const text = useMemo(() => courseText(locale), [locale]);
  const [task, setTask] = useState<string | undefined>(undefined);
  const [camera, setCamera] = useState<PresencePresentation["camera"]>();
  const ownScope = stepKey(state.step);
  const [swipe, setSwipe] = useState<{ readonly scope: string; readonly page: string }>();
  const [layout, setLayout] = useState<{ readonly scope: string; readonly value: PresencePresentation["layout"] }>();
  const captureSwipe = useCallback((page: string) => setSwipe({ scope: ownScope, page }), [ownScope]);
  const captureLayout = useCallback((value: PresencePresentation["layout"]) => setLayout({ scope: ownScope, value }), [ownScope]);
  const [reveal, setReveal] = useState<string | undefined>();
  const [revealedBy, setRevealedBy] = useState<"focus" | "pointer" | undefined>();
  const [submission, setSubmission] = useState<PresencePresentation["submission"]>();
  const [sort, setSort] = useState<PresencePresentation["sort"]>();
  const [taskPage, setTaskPage] = useState<number | undefined>();
  const captureReveal = useCallback((next: string | undefined, source?: "focus" | "pointer") => { setReveal(next); setRevealedBy(source); }, []);
  const [viewport, setViewport] = useState<PresencePresentation["viewport"]>();
  const shared = useSyncExternalStore(presence.subscribe, presence.getSnapshot, presence.getSnapshot);
  const follow = useFollow(shared);
  const remote = follow.peer;
  const captureCamera = usePresenceCapture(setCamera, stepKey(state.step), remote === undefined);
  const observed = useMemo(() => remote === undefined ? undefined : followedCourseState(state, remote), [state, remote]);
  const observedRef = useRef(observed);
  observedRef.current = observed;
  const observer = useMemo(() => observerSession(session, () => observedRef.current ?? state), [session]);
  const remoteLocale = useLoadedLocale(remote?.view?.locale ?? locale, locale);
  const remoteText = useMemo(() => courseText(remoteLocale), [remoteLocale]);
  const remotePreferences = { ...followedPreferences(preferences, remote?.view?.preferences), locale: remoteLocale };
  const observedScene = useMemo(() => observed === undefined ? undefined : { state: observed, locale: remoteLocale }, [observed, remoteLocale]);
  useEffect(() => onObservedScene(observedScene), [onObservedScene, observedScene]);
  useEffect(() => onObservedPreferences(remote?.view?.preferences === undefined ? undefined : { ...remote.view.preferences, device: remote.view.preferences.device }), [onObservedPreferences, remote?.view?.preferences]);
  const visible = useDocumentVisible();
  const pets = effectivePetMode(remote === undefined ? preferences.pets : remotePreferences.pets);
  const icons = effectiveIconMotion(remote === undefined ? preferences.animateIcons : remotePreferences.animateIcons, remote === undefined ? preferences.iconsChosen : remotePreferences.iconsChosen);
  const identified = state.learner !== undefined;
  const self = useMemo(() => presenceSelf(identified ? state.learner : undefined), [identified, state.learner]);
  const at = remote === undefined ? presencePlace(state.step, state, task) : { place: remote.place, room: remote.place.screen !== "identity" };
  const onScreen = state.step.screen === "run" || state.step.screen === "results" ? state.runs[state.step.run] : undefined;
  const drafts = useMemo(() => presenceDrafts(onScreen), [onScreen]);
  const remoteRun = observed?.runs["presence-run"];
  const itemLabel = useMemo(() => sheetItemLabels(remoteRun ?? onScreen, remoteRun === undefined ? locale : remoteLocale), [onScreen, locale, remoteRun, remoteLocale]);
  const quizzes = useMemo(() => state.course?.quizzes.map((quiz) => quiz.id) ?? [], [state.course]);
  const leaderboard = shownLeaderboard(state);
  const overall = overallLeaderboard(state);
  const presentation = useMemo<PresencePresentation>(() => ({ locale, home: state.step.screen === "home" ? state.step.page : undefined, camera, swipe: state.step.screen === "home" && swipe?.scope === ownScope ? swipe.page : undefined, layout: state.step.screen === "home" && layout?.scope === ownScope ? layout.value : undefined, reveal, revealedBy, viewport, taskPage: state.step.screen === "run" ? taskPage : undefined, submission: state.step.screen === "run" ? submission : undefined, sort, run: publicPresenceRun(onScreen), profile: state.learnerView === undefined ? undefined : presenceProfile(state.learnerView), asked: state.asked, board: state.board, leaderboard: leaderboard?.board, leaderboardAt: leaderboard?.at, overall: overall?.board, overallAt: overall?.at, crowd: onScreen === undefined ? undefined : state.crowds[onScreen.quiz], awards: onScreen === undefined ? undefined : runAwards(state, onScreen.run), preferences: { theme: preferences.theme, textSize: preferences.textSize, device: preferences.device, showCursors: preferences.showCursors, others: preferences.others, animateIcons: preferences.animateIcons, iconsChosen: preferences.iconsChosen, pets: preferences.pets, petsLiveliness: preferences.petsLiveliness, petsChosen: preferences.petsChosen, petsPlay: preferences.petsPlay, petsMischief: preferences.petsMischief, challenges: preferences.challenges } }), [state, locale, camera, ownScope, swipe, layout, reveal, revealedBy, viewport, taskPage, submission, sort, onScreen, preferences, leaderboard, overall]);
  useEffect(() => presence.update({ course: state.course?.id, quizzes, self, at, home: remote === undefined ? state.step.screen === "home" : observed?.step.screen === "home", homeRoute: remote === undefined ? presentation.home : remote.view?.home, drafts: remote === undefined ? drafts : undefined, active: visible, view: remote === undefined ? presentation : undefined }), [presence, state.course?.id, quizzes, self, at?.place.screen, at?.place.quiz, at?.place.task, at?.room, state.step.screen, observed?.step.screen, remote === undefined, remote?.view?.home, drafts, visible, presentation]);
  usePresencePointer(presence, remote === undefined);
  useAddress(session, state);
  const view = useMemo(() => presenceView(shared, text), [shared, text]);
  const main = useRef<HTMLElement>(null);
  const shown = useRef<string | undefined>(undefined);
  const key = stepKey(state.step);
  const observedKey = remote === undefined ? key : `${remote.tag}:${remote.place.screen}:${remote.view?.home ?? ""}:${remote.place.task ?? ""}`;
  const shownTitle = documentTitle(observed ?? state, remote === undefined ? locale : remoteLocale);
  useEffect(() => { document.title = shownTitle; }, [shownTitle]);
  usePresenceViewport(main, observedKey, remote?.view?.viewport, setViewport, remote !== undefined, remote === undefined ? presentation.swipe : remote.view?.swipe);
  useEffect(() => {
    const active = document.activeElement;
    const gridFocus = active instanceof HTMLElement && main.current?.contains(active) && active.closest("[data-course-grid][data-grid-active]") !== null && active.closest('[inert], [aria-hidden="true"]') === null;
    if (shown.current !== undefined && shown.current !== key && !gridFocus) main.current?.querySelector<HTMLElement>("h1")?.focus();
    shown.current = key;
  }, [key]);

  const courseTitle = state.course === undefined ? undefined : titled(state.course, locale);
  const brand = (
    <span data-course-brand="" className="course-brand">
      {options.logo === undefined ? null : (
        <span aria-hidden="true" className="course-brand-logo inline-flex">
          <img src={options.logo} alt="" className="size-workbench shrink-0" />
        </span>
      )}
      {courseTitle === undefined ? null : (
        <ResponsiveLabel full={courseTitle.full} short={courseTitle.short} className="course-brand-title truncate px-single text-sm font-semibold text-foreground" />
      )}
    </span>
  );
  const items: NavbarItem[] = [
    { key: "ways", content: <NavigationControls session={session} state={state} locale={locale} text={text} /> },
    { key: "introduction", content: <CardAction aria-label={text("course.introduction.replay")} title={text("course.introduction.replay")} onClick={() => session.open({ screen: "introduction" })}><CardIcon icon="info" /><span className="hidden md:inline">{text("course.introduction.replay")}</span></CardAction> },
    navbarFillItem("fill"),
    { key: "connection", className: "min-w-0 shrink", content: <ConnectionStatus connection={connection} text={text} /> },
    { key: "language", content: <span data-element-alias={COURSE_LANGUAGE_ANCHOR} className="flex"><LanguageSwitch compact locale={locale} text={text} onChange={(chosen) => onPreferences({ ...preferences, locale: chosen })} /></span> },
    { key: "brand", centered: true, className: "min-w-0 px-single", content: brand },
  ];
  return (
    <PresenceProvider view={view} showCursors={preferences.showCursors} setTask={setTask}>
      <div className="course-app flex min-h-0 flex-col overflow-hidden bg-background text-foreground" style={{ height: UI_AVAILABLE_HEIGHT }} lang={locale} data-icon-motion={icons ? "on" : "off"} onPointerDownCapture={(event) => {
        if (remote !== undefined && event.target instanceof Element && event.target.closest("[data-presence-footer]") === null) { event.preventDefault(); event.stopPropagation(); follow.stop("interaction"); return; }
        playTappedIcons(event.target, event.pointerType);
      }} onKeyDownCapture={(event) => {
        if (remote === undefined) return;
        if (event.key === "Escape") { event.preventDefault(); event.stopPropagation(); follow.stop("escape"); }
        else if (event.key !== "Tab" && !event.altKey && !event.ctrlKey && !event.metaKey && event.target instanceof Element && event.target.closest("[data-presence-footer]") === null) { event.preventDefault(); event.stopPropagation(); follow.stop("interaction"); }
      }} onWheelCapture={(event) => { if (remote !== undefined) { event.stopPropagation(); follow.stop("interaction"); } }} data-pets={pets}>
        <a
          href="#course-main"
          className="sr-only focus:not-sr-only focus:absolute focus:start-double focus:top-double focus:z-50 focus:bg-background focus:p-double focus:text-sm focus:text-foreground"
          onClick={(event) => {
            event.preventDefault();
            main.current?.focus();
          }}
        >
          {text("course.app.skip")}
        </a>
        <header className="shrink-0">
          <Navbar label={text("course.nav.label")} items={items} showFullscreenToggle={false} />
        </header>
        <main id="course-main" ref={main} tabIndex={-1} className="flex min-h-0 flex-1 flex-col outline-none">
          {connection.contract === "newer" ? (
            <ProblemNote problem={{ message: text("course.app.outdated") }} text={text} className="m-double mb-0" id="course-outdated">
              <BodyButton onClick={reloadPage}>{text("course.app.reload")}</BodyButton>
            </ProblemNote>
          ) : null}
          {state.notice === undefined ? null : (
            <ProblemNote problem={noticeProblem(state.notice, text)} text={text} className="m-double mb-0">
              <BodyButton onClick={() => session.dismissNotice()}>{text("course.app.dismiss")}</BodyButton>
            </ProblemNote>
          )}
          <div inert={remote !== undefined} hidden={remote !== undefined} className={remote === undefined ? "contents" : "hidden"}>
            <FollowProvider onCamera={captureCamera} onRevealed={captureReveal} onSubmission={setSubmission} onSort={setSort} onTaskPage={setTaskPage} onSwipe={captureSwipe} onLayout={captureLayout}>
              <ScreenBoundary key={key} text={text}>
                <Screen session={session} state={state} connection={connection} text={text} locale={locale} preferences={preferences} onPreferences={onPreferences} />
              </ScreenBoundary>
            </FollowProvider>
          </div>
          {remote === undefined || observed === undefined ? null : (
            <div data-follow-screen={remote.place.screen} lang={remoteLocale} className="flex min-h-0 flex-1 flex-col">
              <FollowProvider following={remote.tag} presentation={remote.view} task={remote.place.task} onCamera={() => undefined} onRevealed={() => undefined} onSubmission={() => undefined} onSort={() => undefined} onTaskPage={() => undefined}>
                <PresenceProvider view={view} showCursors={remotePreferences.showCursors} setTask={() => undefined}>
                  <ScreenBoundary key={`${remote.tag}:${remote.place.screen}`} text={remoteText}>
                    <Screen session={observer} state={observed} connection={connection} text={remoteText} locale={remoteLocale} preferences={remotePreferences} onPreferences={() => undefined} readOnly />
                  </ScreenBoundary>
                </PresenceProvider>
              </FollowProvider>
            </div>
          )}
        </main>
        <LegalFooter legal={options.legal} locale={locale} text={text}>
          <PresenceFooter course={state.course} text={text} locale={locale} following={follow.tag} waiting={follow.waiting} followingName={remote === undefined ? undefined : learnerName(remote.identity, remote.tag, text)} onFollow={follow.start} onStop={() => follow.stop()} />
          <PetsSwitch shown={preferences.pets !== "off"} label={text("course.preferences.petsShown")} onChange={(shown) => onPreferences(withPets(preferences, switchedPets(shown, preferences.petsLiveliness)))} />
        </LegalFooter>
        <PresenceOverlay view={view} show={remote === undefined ? preferences.showCursors : remotePreferences.showCursors} itemLabel={itemLabel} />
        <CoursePets />
      </div>
    </PresenceProvider>
  );
}

function useSetup(options: CourseOptions): { readonly store: LocalStore; readonly presence: CoursePresence; readonly session: CourseSession; readonly arriving: Promise<CourseMaterial | undefined> | undefined } {
  const [setup] = useState(() => {
    const store = localStore(options.storage ?? browserStorageArea(), options.tenant);
    const proctor = new ProctorClient(options.transport ?? proctorTransport(options.proctor), options.tenant);
    const presence = new CoursePresence({ proctor: options.proctor, connect: options.presence, timing: options.timing });
    const { material } = options;
    const deputy = material === undefined || typeof material === "function" ? undefined : new Deputy(material);
    const arriving = typeof material === "function" ? material().catch(() => undefined) : undefined;
    return { store, presence, arriving, session: new CourseSession({ proctor, store, deputy, timing: options.timing }) };
  });
  return setup;
}

/** 🌐️ The language the client shows in, once its texts have arrived: `wanted` — every offered language while none is
 * chosen — is fetched as chunks of its own; until they are here the language shown stays the one before, and at the very start
 * nothing is ready. */
function useCourseLocale(wanted: CourseLocale | undefined): { readonly ready: boolean; readonly locale: CourseLocale | undefined } {
  const needed = wanted === undefined ? COURSE_LOCALES : [wanted];
  const arrived = needed.every(courseLocaleLoaded);
  const [, rerender] = useState(0);
  const shown = useRef<{ readonly locale: CourseLocale | undefined } | undefined>(undefined);
  if (arrived) shown.current = { locale: wanted };
  useEffect(() => {
    if (arrived) return;
    let live = true;
    void loadCourseLocales(needed).then(() => live && rerender((count) => count + 1));
    return () => {
      live = false;
    };
  }, [arrived, wanted]);
  return shown.current === undefined ? { ready: false, locale: undefined } : { ready: true, locale: shown.current.locale };
}

/** 📖️ The language of another learner's screen, once its texts have arrived; the client's own language until then. */
function useLoadedLocale(wanted: CourseLocale, fallback: CourseLocale): CourseLocale {
  const arrived = courseLocaleLoaded(wanted);
  const [, rerender] = useState(0);
  useEffect(() => {
    if (arrived) return;
    let live = true;
    void loadCourseLocales([wanted]).then(() => live && rerender((count) => count + 1));
    return () => {
      live = false;
    };
  }, [arrived, wanted]);
  return arrived ? wanted : fallback;
}

/** ❓️ The whole course client for one proctor tenant. It speaks the language the learner stored, else the one the
 * browser's list preselects among the offered ones; while neither exists it shows only the language chooser. */
export function CourseApp(options: CourseOptions): ReactElement {
  const { session, store, presence, arriving } = useSetup(options);
  useLayoutEffect(() => {
    session.start();
    return () => session.stop();
  }, [session]);
  useEffect(() => {
    let live = true;
    void arriving?.then((material) => {
      if (live && material !== undefined) session.attachDeputy(new Deputy(material));
    });
    return () => {
      live = false;
    };
  }, [arriving, session]);
  useEffect(() => () => presence.stop(), [presence]);
  const { state, connection } = useSyncExternalStore(session.subscribe, session.getSnapshot, session.getSnapshot);
  const [preferences, setPreferences] = useState<CoursePreferences>(() => readPreferences(store));
  const [observedPreferences, setObservedPreferences] = useState<PresencePreferences | undefined>();
  const [observedScene, setObservedScene] = useState<ObservedScene | undefined>();
  const shownPreferences = followedPreferences(preferences, observedPreferences);
  const wanted = preferences.locale ?? preferredLocale(options.languages ?? browserLanguages());
  const { ready, locale } = useCourseLocale(wanted);
  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const tablet = useMediaQuery(UI_TABLET_MEDIA_QUERY);
  useElementsSurfaceChrome({ appearance: shownPreferences.theme, device: elementsSurfaceDeviceForLayout(elementsSurfaceDeviceForMatches({ mobile, tablet }), shownPreferences.device), driver: DEFAULT_UI_DRIVER, browserDefaults: "native" });
  useRootTextScale(textScale(shownPreferences.textSize));
  useLayoutEffect(() => {
    void applyLocale(locale ?? wanted);
  }, [locale, wanted]);
  const title = ready ? documentTitle(state, locale) : "";
  useEffect(() => {
    if (ready && locale === undefined) document.title = title;
  }, [ready, locale, title]);
  useEffect(() => store.watch((change) => (change.kind === "cleared" || (change.kind === "slice" && change.slice === "preferences")) && setPreferences(readPreferences(store))), [store]);

  const change = (next: CoursePreferences): void => {
    setPreferences(next);
    writePreferences(store, next);
  };
  if (!ready) return <></>;
  if (locale !== undefined)
    return (
      <CoursePetsProvider source={options.pets} choice={shownPreferences.pets} chosen={shownPreferences.petsChosen} play={shownPreferences.petsPlay} mischief={shownPreferences.petsMischief} state={observedScene?.state ?? state} locale={observedScene?.locale ?? locale}>
        <Client options={options} session={session} presence={presence} state={state} connection={connection} locale={locale} preferences={preferences} onPreferences={change} onObservedPreferences={setObservedPreferences} onObservedScene={setObservedScene} />
      </CoursePetsProvider>
    );
  return (
    <div className="course-app flex min-h-0 flex-col overflow-hidden bg-background text-foreground" style={{ height: UI_AVAILABLE_HEIGHT }}>
      <main id="course-main" className="relative flex min-h-0 flex-1 flex-col overflow-auto p-double">
        <div className="mx-auto w-full max-w-xl">
          <LanguageChoice onChoose={(chosen) => change({ ...preferences, locale: chosen })} />
        </div>
      </main>
      <LegalFooter legal={options.legal} locale={undefined} text={undefined} />
    </div>
  );
}

/** 🚀️ Mounts the course client into `root` — with the stored theme and the known language applied to the document
 * before the first paint, the page's own loading text standing until the texts of that language (every offered one while
 * none is known) have arrived — and returns the function that unmounts it again. */
export function mountCourse(root: HTMLElement, options: CourseOptions): () => void {
  const preferences = readPreferences(localStore(options.storage ?? browserStorageArea(), options.tenant));
  bootstrapElementsSurfaceChromeDocument(preferences.theme);
  const wanted = preferences.locale ?? preferredLocale(options.languages ?? browserLanguages());
  void applyLocale(wanted);
  let reactRoot: Root | undefined;
  let live = true;
  const render = (): void => {
    if (!live) return;
    reactRoot = createRoot(root);
    reactRoot.render(
      <StrictMode>
        <CourseApp {...options} />
      </StrictMode>,
    );
  };
  const needed = wanted === undefined ? COURSE_LOCALES : [wanted];
  if (needed.every(courseLocaleLoaded)) render();
  else void loadCourseLocales(needed).then(render, render);
  return () => {
    live = false;
    reactRoot?.unmount();
  };
}
//#endregion 📚️App
